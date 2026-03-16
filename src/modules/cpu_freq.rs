use crate::core::module::Module;
use std::fs;

/// Module for reporting the current scaling frequency of the CPU.
pub struct CpuFreqModule;

impl Module for CpuFreqModule {
    fn name(&self) -> &'static str {
        "CPU Freq"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        // Read the current frequency of the first core (cpu0) for maximum speed.
        // In most balanced governors, this is representative of the whole package.
        if let Ok(freq_str) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq") {
            if let Ok(khz) = freq_str.trim().parse::<f64>() {
                let ghz = khz / 1_000_000.0;
                return vec![("CPU Freq (current)".to_string(), format!("{:.2} GHz", ghz))];
            }
        }
        
        vec![("CPU Freq".to_string(), "Unknown".to_string())]
    }
}
