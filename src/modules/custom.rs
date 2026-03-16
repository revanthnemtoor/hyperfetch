use crate::core::module::Module;
use std::process::Command;
use std::time::{Duration, SystemTime};
use std::fs;
use wait_timeout::ChildExt;

/// A dynamic module that executes user-defined shell commands and parses their output.
/// Supports configurable timeouts and persistent caching to prevent UI stalling.
pub struct CustomShellModule {
    /// Display name of the module
    pub module_name: String,
    /// The shell command to execute
    pub command: String,
    /// Maximum time to wait for the command to finish
    pub timeout_ms: Option<u64>,
    /// How long to persist the command output in the cache
    pub cache_minutes: Option<u64>,
}

impl Module for CustomShellModule {
    fn name(&self) -> &str {
        &self.module_name
    }

    fn fetch(&self) -> Vec<(String, String)> {
        let cache_path = self.get_cache_path();

        // Check if a valid, non-expired cache entry exists
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

        // Execute the command via 'sh -c'
        let mut child = match Command::new("sh")
            .arg("-c")
            .arg(&self.command)
            .stdout(std::process::Stdio::piped())
            .stderr(std::process::Stdio::null())
            .spawn() {
                Ok(c) => c,
                Err(_) => return vec![],
            };

        // Enforce the execution timeout to keep the fetch blazingly fast
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
                // Command failed or timed out; kill the child process
                let _ = child.kill();
                "".to_string()
            }
            None => {
                // Severe timeout; kill the child process
                let _ = child.kill();
                "".to_string()
            }
        };

        if !result.is_empty() {
            // Persist the result to the cache for future executions
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
    /// Resolves the filesystem path for this module's cache file (~/.cache/hyperfetch/custom/...).
    fn get_cache_path(&self) -> std::path::PathBuf {
        let mut path = dirs::cache_dir().unwrap_or_else(|| std::env::temp_dir());
        path.push("hyperfetch");
        path.push("custom");
        // Create a safe filename slug from the module name
        let slug = self.module_name.to_lowercase()
            .replace(' ', "_")
            .replace(|c: char| !c.is_alphanumeric() && c != '_', "");
        path.push(format!("{}.cache", slug));
        path
    }
}
