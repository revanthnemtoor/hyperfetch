use crate::core::module::Module;
use std::env;

pub struct DesktopModule;

impl Module for DesktopModule {
    fn name(&self) -> &'static str {
        "DE/WM"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        let mut de = env::var("XDG_CURRENT_DESKTOP").unwrap_or_default();
        if de.is_empty() {
            de = env::var("DESKTOP_SESSION").unwrap_or_default();
        }

        let mut protocol = "".to_string();
        if env::var("WAYLAND_DISPLAY").is_ok() {
            protocol = " (Wayland)".to_string();
        } else if env::var("DISPLAY").is_ok() {
            protocol = " (X11)".to_string();
        }

        let mut results = Vec::new();
        if !de.is_empty() {
            // Clean up XDG formatting like "ubuntu:GNOME"
            if let Some(clean) = de.split(':').last() {
                results.push(("DE/WM".to_string(), format!("{}{}", clean, protocol)));
                return results;
            }
        }

        vec![("DE/WM".to_string(), "Headless / CLI".to_string())]
    }
}
