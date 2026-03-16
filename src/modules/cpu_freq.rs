use crate::core::module::Module;
use std::fs;

pub struct CpuFreqModule;

impl Module for CpuFreqModule {
    fn name(&self) -> &'static str {
        "CPU Freq"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        // Average the frequencies of all active cores, or just read cpu0 if we want it blazing fast
        if let Ok(freq_str) = fs::read_to_string("/sys/devices/system/cpu/cpu0/cpufreq/scaling_cur_freq") {
            if let Ok(khz) = freq_str.trim().parse::<f64>() {
                let ghz = khz / 1_000_000.0;
                return vec![("CPU Freq".to_string(), format!("{:.2} GHz", ghz))];
            }
        }
        
        vec![("CPU Freq".to_string(), "Unknown".to_string())]
    }
}
