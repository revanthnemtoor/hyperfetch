use crate::core::module::Module;
use std::fs;

/// Module for reporting battery charge level and health status.
pub struct BatteryModule;

impl Module for BatteryModule {
    fn name(&self) -> &'static str {
        "Battery"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        let mut results = Vec::new();

        // Read from the persistent power supply subsystem in sysfs
        if let Ok(entries) = fs::read_dir("/sys/class/power_supply") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                
                // Identify battery nodes (usually BAT0, BAT1, etc.)
                if name.starts_with("BAT") {
                    let cap_path = entry.path().join("capacity");
                    let status_path = entry.path().join("status");
                    
                    if let Ok(cap_str) = fs::read_to_string(cap_path) {
                        let cap = cap_str.trim();
                        let status = fs::read_to_string(status_path).unwrap_or("".into());
                        let status = status.trim();
                        
                        // Check for battery health/wear level if available
                        let health_path = entry.path().join("capacity_level");
                        let health = fs::read_to_string(health_path).unwrap_or("".into());
                        let health = health.trim();
                        
                        let display = if !health.is_empty() && health != "Normal" {
                            format!("{}% [{}, {}]", cap, status, health)
                        } else {
                            format!("{}% [{}]", cap, status)
                        };
                        
                        results.push((format!("Battery ({})", name), display));
                    }
                }
            }
        }

        // Return battery stats or a "No Battery" notice for desktops
        if results.is_empty() {
            vec![("Battery".to_string(), "Desktop / No Battery".to_string())]
        } else {
            results
        }
    }
}
