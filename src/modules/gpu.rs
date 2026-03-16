use crate::core::module::Module;
use std::fs;

/// Module for detecting installed Graphics Processing Units (GPUs).
pub struct GpuModule;

/// Resolves a PCI vendor and device ID into a human-readable name by searching local PCI databases.
fn get_pci_device_name(vendor_id: &str, device_id: &str) -> Option<String> {
    // Search common Linux PCI ID database locations
    let paths = ["/usr/share/hwdata/pci.ids", "/usr/share/misc/pci.ids", "/var/lib/pci.ids"];
    let mut content = String::new();
    for path in paths {
        if let Ok(data) = fs::read_to_string(path) {
            content = data;
            break;
        }
    }
    if content.is_empty() { return None; }
    
    // Exact mapping logic for the pci.ids file format
    let v_lower = vendor_id.to_lowercase();
    let d_lower = device_id.to_lowercase();

    let vendor_prefix = format!("{}  ", v_lower);
    let device_prefix = format!("\t{}  ", d_lower);
    
    let mut in_vendor = false;
    for line in content.lines() {
        if !in_vendor {
            if line.starts_with(&vendor_prefix) {
                in_vendor = true;
            }
        } else {
            if line.starts_with('\t') {
                if line.starts_with(&device_prefix) {
                    let mut name = line[device_prefix.len()..].trim().to_string();
                    // Clean up names like "GA107M [GeForce RTX 3050 Ti Mobile]" -> "GeForce RTX 3050 Ti Mobile"
                    if let Some(start) = name.find('[') {
                        if let Some(end) = name.find(']') {
                            name = name[start + 1..end].to_string();
                        }
                    }
                    return Some(name);
                }
            } else if !line.starts_with('#') {
                break;
            }
        }
    }
    None
}

impl Module for GpuModule {
    fn name(&self) -> &'static str {
        "GPU"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        let mut gpus = Vec::new();
        
        // Iterate through Direct Rendering Manager (DRM) nodes to find video cards
        if let Ok(entries) = fs::read_dir("/sys/class/drm") {
            for entry in entries.flatten() {
                let name = entry.file_name().to_string_lossy().to_string();
                if name.starts_with("card") && !name.contains('-') {
                    let vendor_path = entry.path().join("device/vendor");
                    let device_path = entry.path().join("device/device");
                    
                    if let (Ok(vendor_hex), Ok(device_hex)) = (fs::read_to_string(&vendor_path), fs::read_to_string(&device_path)) {
                        let v = vendor_hex.trim().trim_start_matches("0x");
                        let d = device_hex.trim().trim_start_matches("0x");
                        
                        let mut gpu_name = "".to_string();
                        if let Some(resolved_name) = get_pci_device_name(v, d) {
                            gpu_name = resolved_name;
                        } else {
                            // Hardcoded fallback for most common vendors
                            let vendor = match v {
                                "1002" => "AMD",
                                "10de" => "NVIDIA",
                                "8086" => "Intel",
                                _ => "Unknown",
                            };
                            if vendor != "Unknown" {
                                gpu_name = format!("{} GPU", vendor);
                            }
                        }

                        if !gpu_name.is_empty() && !gpus.contains(&gpu_name) {
                            gpus.push(gpu_name);
                        }
                    }
                }
            }
        }

        // Return all detected GPUs, numbering them if multiple exist
        if !gpus.is_empty() {
            let mut results = Vec::new();
            for (i, gpu) in gpus.into_iter().enumerate() {
                let key = if i == 0 { "GPU".to_string() } else { format!("GPU {}", i) };
                results.push((key, gpu));
            }
            return results;
        }

        vec![("GPU".to_string(), "Unknown GPU".to_string())]
    }
}
