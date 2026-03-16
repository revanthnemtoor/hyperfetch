use crate::core::module::Module;
use crate::core::sys_paths::CPUINFO;

/// Module for identifying the CPU model and core count.
pub struct CpuModule;

impl Module for CpuModule {
    fn name(&self) -> &'static str {
        "CPU"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        // Use the globally cached /proc/cpuinfo to avoid redundant file I/O
        if !CPUINFO.is_empty() {
            let mut model_name = String::new();
            let mut cores = 0;

            for line in CPUINFO.lines() {
                // "model name" lines appear once per core
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
        
        // Robust fallback using the sysinfo crate if /proc parsing is unavailable
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
