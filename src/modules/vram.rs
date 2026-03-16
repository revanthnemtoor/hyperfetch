use crate::core::module::Module;
use std::fs;

/// Module for detecting Video RAM (VRAM) usage across various GPU drivers.
pub struct VramModule;

impl Module for VramModule {
    fn name(&self) -> &'static str {
        "GPU VRAM"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        let mut results = Vec::new();

        // Attempt to read VRAM info from sysfs (supported by open-source drivers like amdgpu, nouveau)
        if let Ok(entries) = fs::read_dir("/sys/class/drm") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("card") && !name.contains('-') {
                    let used_path = entry.path().join("device/mem_info_vram_used");
                    let total_path = entry.path().join("device/mem_info_vram_total");
                    
                    if let (Ok(used_str), Ok(total_str)) = (fs::read_to_string(&used_path), fs::read_to_string(&total_path)) {
                        if let (Ok(used), Ok(total)) = (used_str.trim().parse::<u64>(), total_str.trim().parse::<u64>()) {
                           if total > 0 {
                               let used_mb = used as f64 / 1_048_576.0;
                               let total_mb = total as f64 / 1_048_576.0;
                               results.push((format!("VRAM ({})", name), format!("{:.0} MiB / {:.0} MiB ({:.0}%)", used_mb, total_mb, (used_mb/total_mb)*100.0)));
                           }
                        }
                    }
                }
            }
        }

        // NVIDIA Proprietary Driver Fallback
        // Since NVIDIA doesn't expose VRAM in sysfs, we use nvidia-smi.
        if results.is_empty() {
            // First, check if the GPU is suspended to avoid the 1-2 second wake-up lag.
            let mut is_suspended = false;
            if let Ok(entries) = fs::read_dir("/sys/bus/pci/devices") {
                for entry in entries.flatten() {
                    if let Ok(vendor) = fs::read_to_string(entry.path().join("vendor")) {
                        if vendor.trim() == "0x10de" {
                            if let Ok(status) = fs::read_to_string(entry.path().join("power/runtime_status")) {
                                if status.trim() == "suspended" {
                                    is_suspended = true;
                                    break;
                                }
                            }
                        }
                    }
                }
            }

            if is_suspended {
                results.push(("GPU VRAM".to_string(), "Suspended".to_string()));
            } else if let Ok(output) = std::process::Command::new("nvidia-smi")
                .arg("--query-gpu=name,memory.used,memory.total")
                .arg("--format=csv,noheader,nounits")
                .output()
            {
                if let Ok(stdout) = String::from_utf8(output.stdout) {
                    for line in stdout.lines() {
                        let parts: Vec<&str> = line.split(',').collect();
                        if parts.len() == 3 {
                            let name = parts[0].trim();
                            if let (Ok(used), Ok(total)) = (parts[1].trim().parse::<f64>(), parts[2].trim().parse::<f64>()) {
                                if total > 0.0 {
                                    results.push((
                                        format!("VRAM ({})", name),
                                        format!("{:.0} MiB / {:.0} MiB ({:.0}%)", used, total, (used / total) * 100.0)
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Return discovered VRAM info or an unsupported notice
        if results.is_empty() {
            vec![("GPU VRAM".to_string(), "Unknown / Unsupported".to_string())]
        } else {
            results
        }
    }
}
