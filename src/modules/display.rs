use crate::core::module::Module;
use std::fs;

pub struct DisplayModule;

impl Module for DisplayModule {
    fn name(&self) -> &'static str {
        "Display"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        let mut results = Vec::new();
        // Read directly from sysfs drm paths for displays avoiding xrandr overhead
        if let Ok(entries) = fs::read_dir("/sys/class/drm") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                // Usually cardX-DP-Y or cardX-eDP-Y
                if name.contains('-') {
                    let status_path = entry.path().join("status");
                    if let Ok(status) = fs::read_to_string(status_path) {
                        if status.trim() == "connected" {
                            let modes_path = entry.path().join("modes");
                            if let Ok(modes) = fs::read_to_string(modes_path) {
                                if let Some(first_mode) = modes.lines().next() {
                                    // Extract the first (highest) resolution
                                    let clean_name = name.split('-').skip(1).collect::<Vec<&str>>().join("-");
                                    results.push((format!("Display ({})", clean_name), first_mode.to_string()));
                                }
                            }
                        }
                    }
                }
            }
        }

        if results.is_empty() {
             vec![("Display".to_string(), "Headless".to_string())]
        } else {
             results
        }
    }
}
