use crate::core::module::Module;
use std::env;

/// Module for identifying the terminal emulator currently in use.
pub struct TerminalModule;

impl Module for TerminalModule {
    fn name(&self) -> &'static str {
        "Terminal"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        // Check for common environment variables set by terminal emulators
        if let Ok(term) = env::var("TERM_PROGRAM") {
            return vec![("Terminal".to_string(), term)];
        }
        
        // Emulator-specific detection via unique environment hooks
        if env::var("ALACRITTY_WINDOW_ID").is_ok() { return vec![("Terminal".to_string(), "Alacritty".to_string())]; }
        if env::var("KITTY_PID").is_ok() { return vec![("Terminal".to_string(), "Kitty".to_string())]; }
        if env::var("KONSOLE_VERSION").is_ok() { return vec![("Terminal".to_string(), "Konsole".to_string())]; }
        if env::var("WT_SESSION").is_ok() { return vec![("Terminal".to_string(), "Windows Terminal".to_string())]; }

        // Last resort fallback: use the $TERM variable and clean up suffixes
        if let Ok(term) = env::var("TERM") {
            let clean = term.replace("-256color", "").replace("-color", "");
            return vec![("Terminal".to_string(), clean)];
        }

        vec![("Terminal".to_string(), "TTY".to_string())]
    }
}
