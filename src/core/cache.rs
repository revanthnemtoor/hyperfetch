use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Default)]
pub struct HardwareCache {
    pub data: HashMap<String, Vec<(String, String)>>,
    pub last_updated: u64,
}

impl HardwareCache {
    fn cache_path() -> Option<PathBuf> {
        if let Some(mut path) = dirs::cache_dir() {
            path.push("fetch");
            fs::create_dir_all(&path).ok()?;
            path.push("hardware.json");
            Some(path)
        } else {
            None
        }
    }

    pub fn load() -> Self {
        if let Some(path) = Self::cache_path() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(cache) = serde_json::from_str::<Self>(&content) {
                    // Invalidate after 24 hours
                    if let Ok(dur) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
                        if dur.as_secs() - cache.last_updated < 86400 {
                            return cache;
                        }
                    }
                }
            }
        }
        Self::default()
    }

    pub fn save(&mut self) {
        if let Some(path) = Self::cache_path() {
            if let Ok(dur) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
                self.last_updated = dur.as_secs();
            }
            if let Ok(json) = serde_json::to_string_pretty(self) {
                let _ = fs::write(path, json);
            }
        }
    }
}
