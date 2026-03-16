use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CustomModuleConfig {
    pub name: String,
    pub command: String,
    pub timeout_ms: Option<u64>,
    pub cache_minutes: Option<u64>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ThemeConfig {
    pub color_key: String,
    pub color_value: String,
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

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    #[serde(default = "default_logo")]
    pub logo: String,
    #[serde(default = "default_modules")]
    pub modules: Vec<String>,
    #[serde(default)]
    pub custom: Vec<CustomModuleConfig>,
    #[serde(default)]
    pub theme: ThemeConfig,
}

fn default_logo() -> String { "default".to_string() }
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

        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(config) = toml::from_str(&content) {
                return config;
            }
        }

        // Generate default config file only if we are targeting the root default config
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

        // Fallback if profile doesn't exist
        Self::default()
    }
}
