use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub modules: Vec<String>,
    pub theme: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            modules: vec![
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
            ],
            theme: "default".to_string(),
        }
    }
}

impl Config {
    pub fn load(cli_path: Option<&str>) -> Self {
        let mut path = dirs::config_dir().unwrap_or_default();
        path.push("fetch");
        path.push("config.toml");

        if let Some(custom_path) = cli_path {
            path = std::path::PathBuf::from(custom_path);
        }

        if let Ok(content) = std::fs::read_to_string(&path) {
            if let Ok(config) = toml::from_str(&content) {
                return config;
            }
        }

        // Generate default config file if missing
        let default_config = Self::default();
        if let Some(parent) = path.parent() {
            let _ = std::fs::create_dir_all(parent);
            if let Ok(toml_str) = toml::to_string_pretty(&default_config) {
                let _ = std::fs::write(&path, toml_str);
            }
        }

        default_config
    }
}
