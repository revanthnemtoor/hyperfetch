use crate::core::module::Module;
use std::fs;

/// Module for detecting connected displays and their native resolutions.
pub struct DisplayModule;

impl Module for DisplayModule {
    fn name(&self) -> &'static str {
        "Display"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        let mut results = Vec::new();
        
        // Scan Direct Rendering Manager (DRM) paths in sysfs for connected monitors
        // This is significantly faster than shelling out to xrandr.
        if let Ok(entries) = fs::read_dir("/sys/class/drm") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                
                // Active displays are represented by nodes with hyphens (e.g., card0-DP-1)
                if name.contains('-') {
                    let status_path = entry.path().join("status");
                    if let Ok(status) = fs::read_to_string(status_path) {
                        if status.trim() == "connected" {
                            let modes_path = entry.path().join("modes");
                            if let Ok(modes) = fs::read_to_string(modes_path) {
                                if let Some(first_mode) = modes.lines().next() {
                                    // Extract the first (usually highest/native) resolution
                                    let clean_name = name.split('-').skip(1).collect::<Vec<&str>>().join("-");
                                    results.push((format!("Display ({})", clean_name), first_mode.to_string()));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Return detected displays or "Headless" if none are found
        if results.is_empty() {
             vec![("Display".to_string(), "Headless".to_string())]
        } else {
             results
        }
    }
}
