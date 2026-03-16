use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Persistent JSON-based cache for static hardware details that are expensive to detect.
/// This prevents slow GPU or Display detection from stalling every fetch execution.
#[derive(Serialize, Deserialize, Default)]
pub struct HardwareCache {
    /// Mapping of module names to their detected property-value pairs
    pub data: HashMap<String, Vec<(String, String)>>,
    /// Unix timestamp of the last successful detection
    pub last_updated: u64,
}

impl HardwareCache {
    /// Resolves the filesystem path for the hardware cache file (~/.cache/fetch/hardware.json).
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

    /// Loads the cache from the filesystem.
    /// Automatically invalidates and returns a fresh cache if the data is older than 24 hours.
    pub fn load() -> Self {
        if let Some(path) = Self::cache_path() {
            if let Ok(content) = fs::read_to_string(&path) {
                if let Ok(cache) = serde_json::from_str::<Self>(&content) {
                    // Cache validation: Invalidate after 24 hours
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

    /// Serializes and writes the current cache state to the filesystem.
    pub fn save(&mut self) -> Result<(), std::io::Error> {
        if let Some(path) = Self::cache_path() {
            if let Ok(dur) = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH) {
                self.last_updated = dur.as_secs();
            }
            if let Ok(json) = serde_json::to_string_pretty(self) {
                return fs::write(path, json);
            }
        }
        Ok(())
    }
}
