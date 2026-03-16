mod cli;
mod config;
mod core;
mod modules;
mod ui;

use clap::Parser;
use crate::cli::Cli;
use colored::Colorize;
use std::time::Instant;

use crate::core::module::Module;
use crate::core::cache::HardwareCache;
use crate::modules::{os::OsModule, kernel::KernelModule, uptime::UptimeModule, cpu::CpuModule, memory::MemoryModule, gpu::GpuModule, disk::DiskModule, network::NetworkModule, shell::ShellModule, terminal::TerminalModule, desktop::DesktopModule, sensors::SensorsModule, display::DisplayModule, packages::PackagesModule, environment::EnvironmentModule, battery::BatteryModule, cpu_freq::CpuFreqModule, vram::VramModule, wifi::WifiModule};

use crate::modules::extended::{HostnameModule, WmDeModule, ThemeModule, SwapModule, LocalIpModule, LocaleModule, HardwareModelModule, MonitorModule, GpuDriverModule, TerminalFontModule};

fn main() {
    let start_time = Instant::now();
    let args = Cli::parse();
    let conf = config::Config::load(args.config.as_deref());

    // Define all available modules
    let available_modules: Vec<Box<dyn Module>> = vec![
        Box::new(OsModule),
        Box::new(HostnameModule),
        Box::new(HardwareModelModule),
        Box::new(KernelModule),
        Box::new(UptimeModule),
        Box::new(PackagesModule),
        Box::new(ShellModule),
        Box::new(DisplayModule),
        Box::new(MonitorModule),
        Box::new(WmDeModule),
        Box::new(ThemeModule),
        Box::new(TerminalModule),
        Box::new(TerminalFontModule),
        Box::new(CpuModule),
        Box::new(CpuFreqModule),
        Box::new(GpuModule),
        Box::new(GpuDriverModule),
        Box::new(VramModule),
        Box::new(MemoryModule),
        Box::new(SwapModule),
        Box::new(DiskModule),
        Box::new(NetworkModule),
        Box::new(LocalIpModule),
        Box::new(WifiModule),
        Box::new(LocaleModule),
        Box::new(SensorsModule),
        Box::new(BatteryModule),
        Box::new(EnvironmentModule),
    ];

    let mut cache = HardwareCache::load();
    let mut cache_modified = false;

    use rayon::prelude::*;

    // Automatically spawn all fetches onto the rayon thread pool
    let results: Vec<_> = conf.modules.par_iter().filter_map(|m_name| {
        let name_lower = m_name.to_lowercase();
        
        // Cache interception for slow hardware detection
        if (name_lower == "gpu" || name_lower == "display") && cache.data.contains_key(&name_lower) {
            let cached_data = cache.data.get(&name_lower).unwrap().clone();
            return Some((name_lower, cached_data));
        }
        
        // Otherwise, fetch natively
        if let Some(module) = available_modules.iter().find(|m| m.name().to_lowercase() == name_lower) {
            Some((name_lower, module.fetch()))
        } else {
            None
        }
    }).collect();

    let mut sys_info = vec![];
    let mut os_name = "Linux".to_string(); // default for ascii

    for (name_lower, entries) in results {
        // Save newly fetched hardware maps to cache
        if (name_lower == "gpu" || name_lower == "display") && !cache.data.contains_key(&name_lower) {
             cache.data.insert(name_lower, entries.clone());
             cache_modified = true;
        }

        for (key, val) in entries {
            if key == "OS" {
                os_name = val.clone();
            }
            sys_info.push((key, val));
        }
    }

    if cache_modified {
        cache.save();
    }

    // JSON Dump Override
    if args.json {
        let json_output = serde_json::to_string_pretty(&sys_info).unwrap_or_else(|_| "[]".to_string());
        println!("{}", json_output);
        return;
    }

    // Logo resolution
    let os_target = if let Some(override_logo) = args.logo {
        override_logo
    } else {
        os_name
    };

    let logo = ui::ascii::AsciiLogo::get(&os_target);

    // Print to layout
    ui::display::print_layout(&logo, &sys_info);

    println!("{}", format!("fetch took {:?}", start_time.elapsed()).bright_black().italic());
}
