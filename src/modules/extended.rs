use crate::core::module::Module;
use std::env;
use std::fs;
use std::net::UdpSocket;

// This file contains many smaller, specialized modules to avoid file-system bloat.

/// Module for detecting the system hostname.
pub struct HostnameModule;
impl Module for HostnameModule {
    fn name(&self) -> &'static str { "Hostname" }
    fn fetch(&self) -> Vec<(String, String)> {
        match fs::read_to_string("/etc/hostname") {
            Ok(content) => vec![(self.name().to_string(), content.trim().to_string())],
            Err(_) => vec![],
        }
    }
}

/// Module for identifying the hardware model/host (e.g., "ThinkPad X1 Carbon").
pub struct HardwareModelModule;
impl Module for HardwareModelModule {
    fn name(&self) -> &'static str { "Host" }
    fn fetch(&self) -> Vec<(String, String)> {
        if let Ok(name) = fs::read_to_string("/sys/devices/virtual/dmi/id/product_name") {
            let version = fs::read_to_string("/sys/devices/virtual/dmi/id/product_version")
                .unwrap_or_default()
                .trim()
                .to_string();
            let mut result = name.trim().to_string();
            if !version.is_empty() && version != "None" {
                result.push_str(" ");
                result.push_str(&version);
            }
            return vec![(self.name().to_string(), result)];
        }
        vec![]
    }
}

/// Module for identifying the Desktop Environment and Window Manager.
pub struct WmDeModule;
impl Module for WmDeModule {
    fn name(&self) -> &'static str { "DE/WM" }
    fn fetch(&self) -> Vec<(String, String)> {
        let de = env::var("XDG_CURRENT_DESKTOP")
            .or_else(|_| env::var("DESKTOP_SESSION"))
            .unwrap_or_default();
        
        let mut results = vec![];
        if !de.is_empty() {
             results.push(("DE".to_string(), de.clone()));
        }
        
        // Detection for Wayland compositors vs X11 window managers
        let wm = env::var("WAYLAND_DISPLAY").unwrap_or_else(|_| env::var("XDG_SESSION_DESKTOP").unwrap_or_default());
        if !wm.is_empty() {
             let mut wm_name = wm.clone();
             if wm.starts_with("wayland") {
                 wm_name = format!("{} (Wayland)", de.split(':').next().unwrap_or("Compositor"));
              }
             results.push(("WM".to_string(), wm_name));
        }

        results
    }
}

/// Module for detecting GTK themes, icons, and cursor settings.
pub struct ThemeModule;
impl Module for ThemeModule {
    fn name(&self) -> &'static str { "Theme" }
    fn fetch(&self) -> Vec<(String, String)> {
        let mut results = vec![];
        if let Some(mut path) = dirs::config_dir() {
            path.push("gtk-3.0");
            path.push("settings.ini");
            // Parse the GTK settings file for theme data
            if let Ok(config) = fs::read_to_string(path) {
                for line in config.lines() {
                    if line.starts_with("gtk-theme-name") {
                        results.push(("Theme".to_string(), line.split('=').nth(1).unwrap_or("").trim().to_string()));
                    } else if line.starts_with("gtk-icon-theme-name") {
                        results.push(("Icons".to_string(), line.split('=').nth(1).unwrap_or("").trim().to_string()));
                    } else if line.starts_with("gtk-font-name") {
                        results.push(("Font".to_string(), line.split('=').nth(1).unwrap_or("").trim().to_string()));
                    } else if line.starts_with("gtk-cursor-theme-name") {
                        results.push(("Cursor".to_string(), line.split('=').nth(1).unwrap_or("").trim().to_string()));
                    }
                }
            }
        }
        results
    }
}

/// Module for reporting Swap memory usage from /proc/meminfo.
pub struct SwapModule;
impl Module for SwapModule {
    fn name(&self) -> &'static str { "Swap" }
    fn fetch(&self) -> Vec<(String, String)> {
        use crate::core::sys_paths::MEMINFO;
        if !MEMINFO.is_empty() {
            let mut total = 0;
            let mut free = 0;
            for line in MEMINFO.lines() {
                if line.starts_with("SwapTotal:") {
                    total = line.split_whitespace().nth(1).and_then(|n| n.parse::<u64>().ok()).unwrap_or(0);
                } else if line.starts_with("SwapFree:") {
                    free = line.split_whitespace().nth(1).and_then(|n| n.parse::<u64>().ok()).unwrap_or(0);
                }
            }
            if total > 0 {
                let used = total.saturating_sub(free);
                return vec![("Swap".to_string(), format!("{:.2} GiB / {:.2} GiB", used as f64 / 1_048_576.0, total as f64 / 1_048_576.0))];
            }
        }
        vec![]
    }
}

/// Module for detecting the local IPv4 address using a fast UDP connection trick.
pub struct LocalIpModule;
impl Module for LocalIpModule {
    fn name(&self) -> &'static str { "Local IP" }
    fn fetch(&self) -> Vec<(String, String)> {
        // Fast UDP trick: we "connect" to a public IP to see which local interface the OS picks.
        // No packets are actually sent over the wire.
        if let Ok(socket) = UdpSocket::bind("0.0.0.0:0") {
            if socket.connect("8.8.8.8:80").is_ok() {
                if let Ok(addr) = socket.local_addr() {
                    return vec![("Local IP".to_string(), addr.ip().to_string())];
                }
            }
        }
        vec![]
    }
}

/// Module for reporting the current system locale (language).
pub struct LocaleModule;
impl Module for LocaleModule {
    fn name(&self) -> &'static str { "Locale" }
    fn fetch(&self) -> Vec<(String, String)> {
        let locale = env::var("LANG").or_else(|_| env::var("LC_ALL")).unwrap_or_default();
        if !locale.is_empty() {
             vec![("Locale".to_string(), locale)]
        } else {
             vec![]
        }
    }
}

/// Module for identifying connected monitors and their resolutions.
pub struct MonitorModule;
impl Module for MonitorModule {
    fn name(&self) -> &'static str { "Monitor" }
    fn fetch(&self) -> Vec<(String, String)> {
        let mut results = vec![];
        if let Ok(entries) = fs::read_dir("/sys/class/drm") {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                if name.starts_with("card") && name.contains('-') {
                    if let Ok(status) = fs::read_to_string(path.join("status")) {
                        if status.trim() == "connected" {
                            let modes_path = path.join("modes");
                            if let Ok(content) = fs::read_to_string(&modes_path) {
                                if let Some(first_mode) = content.lines().next() {
                                    results.push((format!("Monitor ({})", name.split('-').last().unwrap_or("")), first_mode.to_string()));
                                }
                            }
                        }
                    }
                }
            }
        }
        results
    }
}

/// Module for detecting the active GPU driver being used by the kernel.
pub struct GpuDriverModule;
impl Module for GpuDriverModule {
    fn name(&self) -> &'static str { "GPU Driver" }
    fn fetch(&self) -> Vec<(String, String)> {
        let mut results = vec![];
        if let Ok(entries) = fs::read_dir("/sys/class/drm") {
            for entry in entries.flatten() {
                let path = entry.path();
                let name = path.file_name().unwrap_or_default().to_string_lossy();
                if name.starts_with("card") && !name.contains('-') {
                    // Inspect the device uevent file for the active driver name
                    if let Ok(uevent) = fs::read_to_string(path.join("device/uevent")) {
                        for line in uevent.lines() {
                            if line.starts_with("DRIVER=") {
                                let driver = line.split('=').nth(1).unwrap_or("").trim().to_string();
                                results.push((format!("GPU Driver ({})", name.replace("card", "")), driver));
                            }
                        }
                    }
                }
            }
        }
        results
    }
}

/// Module for detecting the font configured in the terminal emulator.
pub struct TerminalFontModule;
impl Module for TerminalFontModule {
    fn name(&self) -> &'static str { "Terminal Font" }
    fn fetch(&self) -> Vec<(String, String)> {
        // Fast hardcoded detection for common terminal emulators like Alacritty
        let term = env::var("TERM").unwrap_or_default();
        if term.contains("alacritty") {
            if let Some(mut path) = dirs::config_dir() {
                path.push("alacritty/alacritty.yml");
                if let Ok(cfg) = fs::read_to_string(&path) {
                    for line in cfg.lines() {
                        if line.contains("family:") {
                            let font = line.split(':').nth(1).unwrap_or("").trim().trim_matches('\'').trim_matches('"');
                            return vec![("Terminal Font".to_string(), font.to_string())];
                        }
                    }
                }
            }
        }
        vec![]
    }
}
