use crate::core::module::Module;
use std::fs;

/// Module for detecting system temperatures and fan speeds via hwmon.
pub struct SensorsModule;

impl Module for SensorsModule {
    fn name(&self) -> &'static str {
        "Sensors"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        let mut results = Vec::new();

        // Scan all hardware monitor (hwmon) nodes in sysfs
        if let Ok(entries) = fs::read_dir("/sys/class/hwmon") {
            for entry in entries.flatten() {
                let path = entry.path();
                
                // Identify the sensor type (e.g., k10temp, coretemp for CPUs)
                let name_path = path.join("name");
                if let Ok(name) = fs::read_to_string(name_path) {
                    let name = name.trim();
                    
                    // CPU Temperature Detection
                    if name == "coretemp" || name == "k10temp" || name == "zenpower" || name == "acpitz" {
                        let temp_path = path.join("temp1_input");
                        if let Ok(temp_str) = fs::read_to_string(temp_path) {
                            if let Ok(millidegrees) = temp_str.trim().parse::<f64>() {
                                let c = millidegrees / 1000.0;
                                let label_suffix = if name == "coretemp" { "Core" } else { name };
                                results.push((format!("CPU Temp ({})", label_suffix), format!("{:.1}°C", c)));
                            }
                        }
                    }

                    // GPU Temperature Detection (AMD and Open Source NVIDIA)
                    if name == "amdgpu" || name == "nouveau" {
                        let temp_path = path.join("temp1_input");
                        if let Ok(temp_str) = fs::read_to_string(temp_path) {
                            if let Ok(millidegrees) = temp_str.trim().parse::<f64>() {
                                let c = millidegrees / 1000.0;
                                results.push(("GPU Temp".to_string(), format!("{:.1}°C", c)));
                            }
                        }
                    }

                    // Fan Speed Detection
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

        // Return results or a helpful "Unsupported" notice
        if results.is_empty() {
            vec![("Sensors".to_string(), "Unsupported".to_string())]
        } else {
            results
        }
    }
}
