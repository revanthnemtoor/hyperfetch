use std::fs;
use crate::core::module::Module;

pub struct CpuModule;

impl Module for CpuModule {
    fn name(&self) -> &'static str {
        "CPU"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        if let Ok(content) = fs::read_to_string("/proc/cpuinfo") {
            let mut model_name = String::new();
            let mut cores = 0;

            for line in content.lines() {
                if line.starts_with("model name") {
                    if model_name.is_empty() {
                        model_name = line.split(':').nth(1).unwrap_or("").trim().to_string();
                    }
                    cores += 1;
                }
            }

            if !model_name.is_empty() {
                return vec![("CPU".to_string(), format!("{} ({} cores)", model_name, cores))];
            }
        }
        
        // Fallback to sysinfo crate if /proc parsing fails or is not Linux
        use sysinfo::System;
        let mut sys = System::new();
        sys.refresh_cpu_usage();
        let cpus = sys.cpus();
        if !cpus.is_empty() {
            return vec![("CPU".to_string(), format!("{} ({} cores)", cpus[0].brand().trim(), cpus.len()))];
        }

        vec![("CPU".to_string(), "Unknown CPU".to_string())]
    }
}
