use crate::core::module::Module;
use std::env;
use std::path::Path;

pub struct ShellModule;

impl Module for ShellModule {
    fn name(&self) -> &'static str {
        "Shell"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        if let Ok(shell_path) = env::var("SHELL") {
            let shell_name = Path::new(&shell_path)
                .file_name()
                .and_then(|name| name.to_str())
                .unwrap_or(&shell_path);
            return vec![("Shell".to_string(), shell_name.to_string())];
        }
        vec![("Shell".to_string(), "Unknown".to_string())]
    }
}
