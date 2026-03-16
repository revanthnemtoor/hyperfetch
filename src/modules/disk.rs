use crate::core::module::Module;
use sysinfo::Disks;

/// Module for detecting physical disks and reporting their usage.
pub struct DiskModule;

impl Module for DiskModule {
    fn name(&self) -> &'static str {
        "Disk"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        // Refresh the list of mounted disks
        let disks = Disks::new_with_refreshed_list();
        let mut disk_stats = Vec::new();

        for disk in disks.list() {
            let fs = disk.file_system().to_string_lossy();
            
            // Filter out virtual, overlay, and temporary filesystems to only show physical storage
            let is_physical = !fs.eq_ignore_ascii_case("tmpfs") 
                           && !fs.eq_ignore_ascii_case("overlay") 
                           && !fs.eq_ignore_ascii_case("squashfs")
                           && !fs.eq_ignore_ascii_case("devtmpfs")
                           && !fs.is_empty();
            
            if is_physical {
                let total_space = disk.total_space();
                let available_space = disk.available_space();
                
                if total_space > 0 {
                    // Prevent duplicate reporting for filesystems mounted multiple times (common in Btrfs)
                    let has_duplicate_sz = disk_stats.iter().any(|d: &(u64, u64, std::path::PathBuf)| d.0 == total_space);
                    
                    if !has_duplicate_sz {
                         disk_stats.push((total_space, available_space, disk.mount_point().to_path_buf()));
                    }
                }
            }
        }

        // Format the results for display
        if !disk_stats.is_empty() {
             let mut results = Vec::new();
             for (total_space, available_space, mount) in disk_stats {
                  let used = total_space.saturating_sub(available_space) as f64 / 1_073_741_824.0;
                  let total = total_space as f64 / 1_073_741_824.0;
                  let mount_str = mount.to_string_lossy();
                  let key = format!("Disk ({})", mount_str);
                  results.push((key, format!("{:.1} GiB / {:.1} GiB ({:.0}%)", used, total, (used/total) * 100.0)));
             }
             return results;
        }

        vec![("Disk".to_string(), "Unknown Disk".to_string())]
    }
}
