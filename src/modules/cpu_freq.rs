use crate::core::module::Module;
use std::fs;

/// Module for reporting the CPU frequency, showing both current usage and rated maximum.
pub struct CpuFreqModule;

impl Module for CpuFreqModule {
    fn name(&self) -> &'static str {
        "CPU Speed"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        // Source paths for various frequency metrics in sysfs
        let base_path = "/sys/devices/system/cpu/cpu0/cpufreq/";
        let cur_path = format!("{}scaling_cur_freq", base_path);
        let max_path = format!("{}cpuinfo_max_freq", base_path);
        let rated_path = format!("{}base_frequency", base_path);

        let cur_khz = fs::read_to_string(cur_path).ok()
            .and_then(|s| s.trim().parse::<f64>().ok());
            
        let max_khz = fs::read_to_string(max_path).ok()
            .and_then(|s| s.trim().parse::<f64>().ok())
            .or_else(|| {
                fs::read_to_string(rated_path).ok()
                    .and_then(|s| s.trim().parse::<f64>().ok())
            });

        match (cur_khz, max_khz) {
            (Some(cur), Some(max)) => {
                let cur_ghz = cur / 1_000_000.0;
                let max_ghz = max / 1_000_000.0;
                // Display both for a complete picture: real-time fluctuation vs stable rating
                vec![("CPU Speed".to_string(), format!("{:.2} GHz (Max: {:.2} GHz)", cur_ghz, max_ghz))]
            }
            (Some(cur), None) => {
                vec![("CPU Speed".to_string(), format!("{:.2} GHz", cur / 1_000_000.0))]
            }
            (None, Some(max)) => {
                vec![("CPU Speed".to_string(), format!("{:.2} GHz (Max)", max / 1_000_000.0))]
            }
            _ => vec![("CPU Speed".to_string(), "Unknown".to_string())],
        }
    }
}
