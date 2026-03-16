use once_cell::sync::Lazy;
use std::fs;

// This module optimizes system file access by performing a single, atomic read
// for heavily-accessed paths like /proc/cpuinfo or /proc/meminfo.
//
// Since hyperfetch uses rayon for parallel module execution, multiple threads
// might attempt to read the same file simultaneously. Using Lazy<String> ensures
// that only the first thread performs the disk/sysfs I/O, while all other threads
// wait and then receive the string from memory instantly.

/// Cached content of /proc/cpuinfo
pub static CPUINFO: Lazy<String> = Lazy::new(|| {
    fs::read_to_string("/proc/cpuinfo").unwrap_or_default()
});

/// Cached content of /proc/meminfo
pub static MEMINFO: Lazy<String> = Lazy::new(|| {
    fs::read_to_string("/proc/meminfo").unwrap_or_default()
});

/// Cached content of the OS release information from /etc/os-release or /usr/lib/os-release
pub static OS_RELEASE: Lazy<String> = Lazy::new(|| {
    fs::read_to_string("/etc/os-release").unwrap_or_else(|_| {
        fs::read_to_string("/usr/lib/os-release").unwrap_or_default()
    })
});
