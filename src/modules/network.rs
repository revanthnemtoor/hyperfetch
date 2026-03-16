use crate::core::module::Module;
use std::fs;

pub struct NetworkModule;

impl Module for NetworkModule {
    fn name(&self) -> &'static str {
        "Network"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        if let Ok(entries) = fs::read_dir("/sys/class/net") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name != "lo" {
                    let operstate_path = entry.path().join("operstate");
                    if let Ok(state) = fs::read_to_string(operstate_path) {
                        if state.trim() == "up" {
                            return vec![("Network".to_string(), name)];
                        }
                    }
                }
            }
        }

        vec![("Network".to_string(), "Disconnected".to_string())]
    }
}
