use crate::core::module::Module;
use std::process::Command;

pub struct CustomShellModule {
    pub module_name: String,
    pub command: String,
    pub timeout_ms: Option<u64>,
    pub cache_minutes: Option<u64>,
}

use std::time::{Duration, SystemTime};
use std::fs;
use wait_timeout::ChildExt;

impl Module for CustomShellModule {
    fn name(&self) -> &str {
        &self.module_name
    }

    fn fetch(&self) -> Vec<(String, String)> {
        let cache_path = self.get_cache_path();

        // Check Cache
        if let Some(minutes) = self.cache_minutes {
            if let Ok(metadata) = fs::metadata(&cache_path) {
                if let Ok(modified) = metadata.modified() {
                    let elapsed = SystemTime::now().duration_since(modified).unwrap_or_default();
                    if elapsed.as_secs() < minutes * 60 {
                        if let Ok(content) = fs::read_to_string(&cache_path) {
                            return vec![(self.module_name.clone(), content.trim().to_string())];
                        }
                    }
                }
            }
        }

        // Execute Command
        let mut child = match Command::new("sh")
            .arg("-c")
            .arg(&self.command)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn() {
                Ok(c) => c,
                Err(_) => return vec![],
            };

        let timeout = Duration::from_millis(self.timeout_ms.unwrap_or(2000));
        let exit_status = child.wait_timeout(timeout).ok().flatten();

        let result = match exit_status {
            Some(status) if status.success() => {
                let mut output = String::new();
                use std::io::Read;
                if let Some(mut stdout) = child.stdout {
                    let _ = stdout.read_to_string(&mut output);
                }
                output.trim().to_string()
            }
            Some(_) => {
                // Command failed or timed out
                let _ = child.kill();
                "".to_string()
            }
            None => {
                // Timed out
                let _ = child.kill();
                "".to_string()
            }
        };

        if !result.is_empty() {
            // Save to Cache
            if let Some(_) = self.cache_minutes {
                let _ = fs::create_dir_all(cache_path.parent().unwrap());
                let _ = fs::write(&cache_path, &result);
            }
            vec![(self.module_name.clone(), result)]
        } else {
            vec![]
        }
    }
}

impl CustomShellModule {
    fn get_cache_path(&self) -> std::path::PathBuf {
        let mut path = dirs::cache_dir().unwrap_or_else(|| std::env::temp_dir());
        path.push("hyperfetch");
        path.push("custom");
        let slug = self.module_name.to_lowercase()
            .replace(' ', "_")
            .replace(|c: char| !c.is_alphanumeric() && c != '_', "");
        path.push(format!("{}.cache", slug));
        path
    }
}
