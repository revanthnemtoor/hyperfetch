use serde::{Deserialize, Serialize};

/// Configuration for user-defined shell modules
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomModuleConfig {
    /// Display name of the custom module
    pub name: String,
    /// Shell command or script path to execute
    pub command: String,
    /// Optional execution timeout in milliseconds
    pub timeout_ms: Option<u64>,
    /// Optional caching duration in minutes
    pub cache_minutes: Option<u64>,
}

/// Visual settings for the ASCII art layout and data presentation
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThemeConfig {
    /// Color for the property labels (e.g., "OS", "CPU")
    pub color_key: String,
    /// Color for the property values (e.g., "Arch Linux", "AMD Ryzen")
    pub color_value: String,
    /// Character sequence used to separate keys from values
    pub separator: String,
}

impl Default for ThemeConfig {
    fn default() -> Self {
        Self {
            color_key: "blue".to_string(),
            color_value: "white".to_string(),
            separator: ":".to_string(),
        }
    }
}

/// Root configuration structure for hyperfetch
#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    /// Name of the ASCII logo to display; "default" triggers auto-detection
    #[serde(default = "default_logo")]
    pub logo: String,
    /// Ordered list of modules to display in the standard fetch output
    #[serde(default = "default_modules")]
    pub modules: Vec<String>,
    /// Custom shell scripts to be integrated as modules
    #[serde(default)]
    pub custom: Vec<CustomModuleConfig>,
    /// Global UI theme settings
    #[serde(default)]
    pub theme: ThemeConfig,
}

fn default_logo() -> String { "default".to_string() }

/// Hardcoded fallback list of modules if none are specified in config.toml
fn default_modules() -> Vec<String> {
    vec![
        "os".to_string(),
        "hostname".to_string(),
        "host".to_string(),
        "kernel".to_string(),
        "uptime".to_string(),
        "packages".to_string(),
        "shell".to_string(),
        "display".to_string(),
        "monitor".to_string(),
        "de/wm".to_string(),
        "theme".to_string(),
        "terminal".to_string(),
        "terminal font".to_string(),
        "cpu".to_string(),
        "cpu freq".to_string(),
        "gpu".to_string(),
        "gpu driver".to_string(),
        "gpu vram".to_string(),
        "memory".to_string(),
        "swap".to_string(),
        "disk".to_string(),
        "network".to_string(),
        "local ip".to_string(),
        "wifi".to_string(),
        "locale".to_string(),
        "sensors".to_string(),
        "battery".to_string(),
        "environment".to_string(),
    ]
}

impl Default for Config {
    fn default() -> Self {
        Self {
            logo: default_logo(),
            custom: vec![],
            theme: ThemeConfig::default(),
            modules: default_modules(),
        }
    }
}

impl Config {
    /// Attempts to load the configuration from ~/.config/hyperfetch/config.toml.
    /// Supports profile-based loading and local file path overrides via CLI.
    pub fn load(cli_path: Option<&str>) -> Self {
        let mut path = dirs::config_dir().unwrap_or_default();
        path.push("hyperfetch");

        // Determine if cli_path is an exact path or a named profile
        if let Some(custom_path) = cli_path {
            if custom_path.contains('/') || custom_path.ends_with(".toml") {
                path = std::path::PathBuf::from(custom_path);
            } else {
                path.push("profiles");
                path.push(format!("{}.toml", custom_path));
            }
        } else {
            path.push("config.toml");
        }

        // Try reading the file from the resolved path
        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(config) = toml::from_str(&content) {
                return config;
            }
        }

        // If no config is found and no profile was requested, bootstrap a default config
        if cli_path.is_none() {
            let default_config = Self::default();
            if let Some(parent) = path.parent() {
                let _ = std::fs::create_dir_all(parent);
                if let Ok(toml_str) = toml::to_string_pretty(&default_config) {
                    let _ = std::fs::write(&path, toml_str);
                }
            }
            return default_config;
        }

        // Silent fallback in case of missing profiles
        Self::default()
    }
}
