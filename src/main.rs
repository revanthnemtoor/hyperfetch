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
use crate::modules::{os::OsModule, kernel::KernelModule, uptime::UptimeModule, cpu::CpuModule, memory::MemoryModule, gpu::GpuModule, disk::DiskModule, network::NetworkModule, shell::ShellModule, terminal::TerminalModule, sensors::SensorsModule, display::DisplayModule, packages::PackagesModule, environment::EnvironmentModule, battery::BatteryModule, cpu_freq::CpuFreqModule, vram::VramModule, wifi::WifiModule};

use crate::modules::extended::{HostnameModule, WmDeModule, ThemeModule, SwapModule, LocalIpModule, LocaleModule, HardwareModelModule, MonitorModule, GpuDriverModule, TerminalFontModule};
use crate::modules::custom::CustomShellModule;

fn main() {
    let start_time = Instant::now();
    let args = Cli::parse();
    
    // Evaluate if user passed --profile or --config
    let conf = config::Config::load(args.config.as_deref());

    // Define all available modules
    let mut available_modules: Vec<Box<dyn Module>> = vec![
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

    for script in &conf.custom {
        available_modules.push(Box::new(CustomShellModule {
            module_name: script.name.clone(),
            command: script.command.clone(),
            timeout_ms: script.timeout_ms,
            cache_minutes: script.cache_minutes,
        }));
    }

    let command = args.command.unwrap_or(crate::cli::Commands::Run);
    match command {
        crate::cli::Commands::ListModules => {
            println!("{}", "Available Modules:".bold().green());
            for m in &available_modules {
                println!("  - {}", m.name());
            }
            return;
        }
        crate::cli::Commands::Doctor => {
            println!("{}", "--- System Doctor ---".bold().blue());
            println!("Config Path:     {}", dirs::config_dir().map(|mut p| { p.push("hyperfetch"); p.to_string_lossy().to_string() }).unwrap_or_else(|| "Unknown".to_string()));
            println!("Custom Scripts:  {}", conf.custom.len());
            println!("Rayon Threads:   {}", rayon::current_num_threads());
            println!("Micro-caches:    Active");
            return;
        }
        crate::cli::Commands::Completions { shell } => {
            use clap::CommandFactory;
            use clap_complete::generate;
            let mut cmd = crate::cli::Cli::command();
            generate(shell, &mut cmd, "hyperfetch", &mut std::io::stdout());
            return;
        }
        crate::cli::Commands::Init => {
            let path = dirs::config_dir().map(|mut p| { p.push("hyperfetch"); p.push("config.toml"); p }).unwrap();
            if path.exists() {
                println!("{}", "Configuration already exists.".yellow());
            } else {
                let _ = config::Config::load(None);
                println!("{} {}", "Initialized default configuration at".green(), path.display());
            }
            return;
        }
        crate::cli::Commands::Run => {}
    }

    let mut cache = HardwareCache::load();
    let mut cache_modified = false;

    use rayon::prelude::*;
    use std::collections::HashSet;

    // Aliasing & Normalization logic
    let mut run_modules: Vec<String> = vec![];
    let to_parse = if let Some(cli_mods) = &args.modules {
        cli_mods.split(',').map(|s| s.trim().to_string()).collect()
    } else {
        conf.modules.clone()
    };

    for m in to_parse {
        let normalized = if conf.custom.iter().any(|c| c.name.to_lowercase() == m.to_lowercase()) {
            m // Don't normalize custom modules
        } else {
            m.replace('-', " ").replace('_', " ")
        };

        if normalized == "system" {
            run_modules.extend(vec!["os".to_string(), "kernel".to_string(), "uptime".to_string(), "cpu".to_string(), "memory".to_string(), "disk".to_string()]);
        } else if normalized == "hardware" {
            run_modules.extend(vec!["cpu".to_string(), "gpu".to_string(), "memory".to_string(), "disk".to_string()]);
        } else if normalized == "network" {
            run_modules.extend(vec!["network".to_string(), "wifi".to_string(), "local ip".to_string()]);
        } else {
            run_modules.push(normalized);
        }
    }

    // Automatically spawn all fetches onto the rayon thread pool
    let results: Vec<_> = run_modules.par_iter().filter_map(|m_name| {
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

    let mut sys_info = Vec::with_capacity(32);
    let mut seen_keys = HashSet::new();
    let mut os_name = "Linux".to_string(); // default for ascii

    for (name_lower, entries) in results {
        // Save newly fetched hardware maps to cache
        if (name_lower == "gpu" || name_lower == "display") && !cache.data.contains_key(&name_lower) {
             cache.data.insert(name_lower, entries.clone());
             cache_modified = true;
        }

        for (key, val) in entries {
            if seen_keys.contains(&key) {
                continue;
            }
            seen_keys.insert(key.clone());

            if key == "OS" {
                os_name = val.clone();
            }
            sys_info.push((key.clone(), val.clone()));
        }
    }

    if cache_modified {
        cache.save();
    }

    // JSON Dump Override
    if args.json {
        let mut map = serde_json::Map::new();
        let mut gpu_names = vec![];
        let mut gpu_drivers = vec![];
        let mut disks = serde_json::Map::new();
        
        for (k, v) in &sys_info {
            let key = k.to_lowercase().replace(' ', "_").replace(|c: char| !c.is_alphanumeric() && c != '_', "");
            
            if k.starts_with("GPU") && !k.contains("Driver") && !k.contains("VRAM") {
                gpu_names.push(v.clone());
            } else if k.starts_with("GPU Driver") {
                gpu_drivers.push(v.clone());
            } else if k.starts_with("Disk") {
                let mount = k.replace("Disk (", "").replace(")", "");
                disks.insert(mount, serde_json::Value::String(v.clone()));
            } else if key == "memory" || key == "swap" {
                let parts: Vec<&str> = v.split(" / ").collect();
                if parts.len() == 2 {
                    let mut mem_map = serde_json::Map::new();
                    mem_map.insert("used".to_string(), serde_json::Value::String(parts[0].to_string()));
                    mem_map.insert("total".to_string(), serde_json::Value::String(parts[1].to_string()));
                    map.insert(key, serde_json::Value::Object(mem_map));
                } else {
                    map.insert(key, serde_json::Value::String(v.clone()));
                }
            } else {
                map.insert(key, serde_json::Value::String(v.clone()));
            }
        }
        
        if !gpu_names.is_empty() {
            let mut gpus = vec![];
            for i in 0..gpu_names.len() {
                let mut gpu_obj = serde_json::Map::new();
                gpu_obj.insert("name".to_string(), serde_json::Value::String(gpu_names[i].clone()));
                if i < gpu_drivers.len() {
                    gpu_obj.insert("driver".to_string(), serde_json::Value::String(gpu_drivers[i].clone()));
                }
                gpus.push(serde_json::Value::Object(gpu_obj));
            }
            map.insert("gpu".to_string(), serde_json::Value::Array(gpus));
        }
        
        if !disks.is_empty() {
            map.insert("disk".to_string(), serde_json::Value::Object(disks));
        }

        let json_output = serde_json::to_string_pretty(&map).unwrap_or_else(|_| "{}".to_string());
        println!("{}", json_output);
        return;
    }

    // Logo resolution
    let os_target = if let Some(override_logo) = args.logo {
        override_logo
    } else if conf.logo != "default" && !conf.logo.is_empty() {
        conf.logo
    } else {
        os_name
    };

    let logo = ui::ascii::AsciiLogo::get(&os_target);

    // Filter layout based on config, terminal width, and explicit format hooks
    if args.table {
        ui::display::print_table(&sys_info);
    } else {
        ui::display::print_layout(&logo, &sys_info, &conf.theme);
    }

    let runtime = start_time.elapsed();
    
    if args.benchmark {
        println!("\n{}", "--- Benchmark ---".bright_blue().bold());
        println!("Modules executed: {}", run_modules.len().to_string().yellow());
        println!("Execution time:   {:?}", runtime);
        let layout_type = if args.table { "Table" } else if args.json { "JSON" } else { "ASCII Layout" };
        println!("Layout renderer:  {}\n", layout_type);
    } else {
        println!("{}", format!("fetch took {:?}", runtime).bright_black().italic());
    }
}
