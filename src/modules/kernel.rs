use crate::core::module::Module;
use std::fs;

pub struct KernelModule;

impl Module for KernelModule {
    fn name(&self) -> &'static str {
        "Kernel"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        if let Ok(content) = fs::read_to_string("/proc/sys/kernel/osrelease") {
            return vec![("Kernel".to_string(), content.trim().to_string())];
        }
        vec![("Kernel".to_string(), "Unknown Kernel".to_string())]
    }
}
