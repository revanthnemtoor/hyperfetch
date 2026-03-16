use crate::core::module::Module;
use crate::core::sys_paths::OS_RELEASE;

/// Module for detecting the Operating System name and version.
pub struct OsModule;

impl Module for OsModule {
    fn name(&self) -> &'static str {
        "OS"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        // We prioritize reading /etc/os-release (cached in OS_RELEASE)
        // This is a standard way to get the "pretty name" of the distribution.
        if !OS_RELEASE.is_empty() {
            for line in OS_RELEASE.lines() {
                if line.starts_with("PRETTY_NAME=") {
                    let name = line.replace("PRETTY_NAME=", "").replace("\"", "");
                    return vec![("OS".to_string(), name)];
                }
            }
        }
        
        // Fallback for non-standard distributions
        vec![("OS".to_string(), "Unknown Linux".to_string())]
    }
}
