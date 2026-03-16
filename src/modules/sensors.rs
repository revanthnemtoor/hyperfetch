use crate::core::module::Module;
use std::fs;

pub struct SensorsModule;

impl Module for SensorsModule {
    fn name(&self) -> &'static str {
        "Sensors"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        let mut results = Vec::new();

        // Check hwmon
        if let Ok(entries) = fs::read_dir("/sys/class/hwmon") {
            for entry in entries.flatten() {
                let path = entry.path();
                
                // Usually hwmon*/name tells us what it is (e.g. k10temp, coretemp)
                let name_path = path.join("name");
                if let Ok(name) = fs::read_to_string(name_path) {
                    let name = name.trim();
                    // Just read first temp1_input for CPU temps (often coretemp or k10temp)
                    if name == "coretemp" || name == "k10temp" || name == "zenpower" || name == "acpitz" {
                        let temp_path = path.join("temp1_input");
                        if let Ok(temp_str) = fs::read_to_string(temp_path) {
                            if let Ok(millidegrees) = temp_str.trim().parse::<f64>() {
                                let c = millidegrees / 1000.0;
                                results.push(("CPU Temp".to_string(), format!("{:.1}°C", c)));
                            }
                        }
                    }

                    // GPU temps
                    if name == "amdgpu" || name == "nouveau" {
                        // Usually temp1_input for AMD GPUs
                        let temp_path = path.join("temp1_input");
                        if let Ok(temp_str) = fs::read_to_string(temp_path) {
                            if let Ok(millidegrees) = temp_str.trim().parse::<f64>() {
                                let c = millidegrees / 1000.0;
                                results.push(("GPU Temp".to_string(), format!("{:.1}°C", c)));
                            }
                        }
                    }

                    // Look for fan1_input
                    let fan_path = path.join("fan1_input");
                    if let Ok(fan_str) = fs::read_to_string(fan_path) {
                        let rpm = fan_str.trim();
                        if !rpm.is_empty() && rpm != "0" {
                            results.push(("Fan Speed".to_string(), format!("{} RPM", rpm)));
                        }
                    }
                }
            }
        }

        if results.is_empty() {
            vec![("Sensors".to_string(), "Unsupported".to_string())]
        } else {
            results
        }
    }
}
