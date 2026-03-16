use crate::core::module::Module;
use crate::core::sys_paths::MEMINFO;

/// Module for reporting total and used system memory (RAM).
pub struct MemoryModule;

impl Module for MemoryModule {
    fn name(&self) -> &'static str {
        "Memory"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        // Use globally cached /proc/meminfo
        if !MEMINFO.is_empty() {
            let mut mem_total = 0;
            let mut mem_available = 0;
            let mut found_total = false;
            let mut found_avail = false;

            for line in MEMINFO.lines() {
                if line.starts_with("MemTotal:") {
                    if let Some(val) = parse_kb(line) {
                        mem_total = val;
                        found_total = true;
                    }
                } else if line.starts_with("MemAvailable:") {
                    if let Some(val) = parse_kb(line) {
                        mem_available = val;
                        found_avail = true;
                    }
                }

                if found_total && found_avail {
                    break;
                }
            }

            if found_total && found_avail {
                // Calculate used memory (Total - Available)
                let used = mem_total.saturating_sub(mem_available);
                return vec![("Memory".to_string(), format!("{:.2} GiB / {:.2} GiB", used as f64 / 1048576.0, mem_total as f64 / 1048576.0))];
            }
        }
        
        // Fallback using sysinfo crate
        use sysinfo::System;
        let mut sys = System::new();
        sys.refresh_memory();
        let used = sys.used_memory() as f64 / 1073741824.0;
        let total = sys.total_memory() as f64 / 1073741824.0;
        vec![("Memory".to_string(), format!("{:.2} GiB / {:.2} GiB", used, total))]
    }
}

/// Helper to parse numeric values from /proc/meminfo lines (format: "Key: Value kB")
fn parse_kb(line: &str) -> Option<u64> {
    line.split_whitespace()
        .nth(1)
        .and_then(|num| num.parse::<u64>().ok())
}
