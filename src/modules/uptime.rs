use std::fs;
use crate::core::module::Module;

pub struct UptimeModule;

impl Module for UptimeModule {
    fn name(&self) -> &'static str {
        "Uptime"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        if let Ok(content) = fs::read_to_string("/proc/uptime") {
            if let Some(uptime_str) = content.split_whitespace().next() {
                if let Ok(uptime_secs) = uptime_str.parse::<f64>() {
                    let secs = uptime_secs as u64;
                    let days = secs / 86400;
                    let hours = (secs % 86400) / 3600;
                    let mins = (secs % 3600) / 60;
                    
                    let time = if days > 0 {
                        format!("{}d {}h {}m", days, hours, mins)
                    } else if hours > 0 {
                        format!("{}h {}m", hours, mins)
                    } else {
                        format!("{}m", mins)
                    };
                    return vec![("Uptime".to_string(), time)];
                }
            }
        }
        vec![("Uptime".to_string(), "Unknown".to_string())]
    }
}
