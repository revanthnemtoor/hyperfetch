use crate::core::module::Module;
use std::fs;

/// Module for detecting the execution environment (Virtual Machines or Containers).
pub struct EnvironmentModule;

impl Module for EnvironmentModule {
    fn name(&self) -> &'static str {
        "Environment"
    }

    fn fetch(&self) -> Vec<(String, String)> {
        let mut results = Vec::new();

        // Container Detection: Check cgroups and specific environment files
        let root_cgroup = fs::read_to_string("/proc/1/cgroup").unwrap_or_default();
        let mut container = None;
        if root_cgroup.contains("docker") || fs::metadata("/.dockerenv").is_ok() {
            container = Some("Docker");
        } else if root_cgroup.contains("lxc") || fs::metadata("/dev/lxd/sock").is_ok() {
            container = Some("LXC");
        } else if fs::metadata("/run/.containerenv").is_ok() {
            container = Some("Podman");
        }

        if let Some(c) = container {
            results.push(("Container".to_string(), c.to_string()));
        }

        // VM Detection: Analyze DMI system vendor information
        if let Ok(sys_vendor) = fs::read_to_string("/sys/class/dmi/id/sys_vendor") {
            let vendor = sys_vendor.trim().to_lowercase();
            if vendor.contains("qemu") || vendor.contains("kvm") {
                results.push(("VM".to_string(), "KVM/QEMU".to_string()));
            } else if vendor.contains("vmware") {
                results.push(("VM".to_string(), "VMware".to_string()));
            } else if vendor.contains("virtualbox") || vendor.contains("innotek") {
                results.push(("VM".to_string(), "VirtualBox".to_string()));
            } else if vendor.contains("microsoft") {
                results.push(("VM".to_string(), "Hyper-V / WSL".to_string()));
            }
        }

        // Default to "Bare Metal" if no virtualization is detected
        if results.is_empty() {
             vec![("Environment".to_string(), "Bare Metal".to_string())]
        } else {
             results
        }
    }
}
