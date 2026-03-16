use crate::core::module::Module;
use std::fs;

pub struct PackagesModule;

impl Module for PackagesModule {
    fn name(&self) -> &'static str {
        "Packages"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        let mut pkg_counts = Vec::new();

        // Arch / Pacman
        if let Ok(entries) = fs::read_dir("/var/lib/pacman/local") {
            let count = entries.count();
            if count > 0 {
                pkg_counts.push(format!("{} (pacman)", count));
            }
        }

        // Debian / APT (rough fast estimate via counting dirs/files or standard dpkg status)
        if let Ok(content) = fs::read_to_string("/var/lib/dpkg/status") {
            let count = content.lines().filter(|l| l.starts_with("Package:")).count();
            if count > 0 {
                pkg_counts.push(format!("{} (dpkg)", count));
            }
        }

        // Flatpak
        if let Ok(entries) = fs::read_dir("/var/lib/flatpak/app") {
            let count = entries.count();
            if count > 0 {
                pkg_counts.push(format!("{} (flatpak)", count));
            }
        }

        if pkg_counts.is_empty() {
            vec![("Packages".to_string(), "Unknown".to_string())]
        } else {
            vec![("Packages".to_string(), pkg_counts.join(", "))]
        }
    }
}
