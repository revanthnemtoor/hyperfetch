use std::fs;
use crate::core::module::Module;

pub struct OsModule;

impl Module for OsModule {
    fn name(&self) -> &'static str {
        "OS"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        // fastfetch reads /etc/os-release directly
        if let Ok(content) = fs::read_to_string("/etc/os-release") {
            for line in content.lines() {
                if line.starts_with("PRETTY_NAME=") {
                    return vec![("OS".to_string(), line.replace("PRETTY_NAME=", "").replace("\"", ""))];
                }
            }
        }
        vec![("OS".to_string(), "Unknown Linux".to_string())]
    }
}
