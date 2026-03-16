use crate::core::module::Module;
use std::fs;

pub struct WifiModule;

impl Module for WifiModule {
    fn name(&self) -> &'static str {
        "WiFi"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        if let Ok(content) = fs::read_to_string("/proc/net/wireless") {
            for line in content.lines().skip(2) {
                let mut parts = line.split_whitespace();
                if let Some(interface) = parts.next() {
                    let interface = interface.trim_end_matches(':');
                    let _status = parts.next();
                    let link_quality = parts.next().unwrap_or("0").trim_end_matches('.');
                    let signal_level = parts.next().unwrap_or("0").trim_end_matches('.');

                    if let (Ok(q), Ok(lvl)) = (link_quality.parse::<f64>(), signal_level.parse::<i32>()) {
                        let quality_pct = (q / 70.0) * 100.0;
                        return vec![(format!("WiFi ({})", interface), format!("{:.0}% ({} dBm)", quality_pct.clamp(0.0, 100.0), lvl))];
                    }
                }
            }
        }
        
        vec![("WiFi".to_string(), "Disconnected / Ethernet".to_string())]
    }
}
