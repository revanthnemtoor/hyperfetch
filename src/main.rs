// Main entry point for the hyperfetch utility.
// This file coordinates module execution, configuration loading, and UI rendering.
mod cli;
mod config;
mod core;
mod modules;
mod ui;

use clap::Parser;
use crate::cli::Cli;
use colored::Colorize;
use std::time::{Instant, Duration};
use std::io::{self, Write};
use crossterm::{
    cursor,
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    event::{self, Event, KeyCode},
};

use crate::core::module::Module;
use crate::core::cache::HardwareCache;
use crate::modules::{os::OsModule, kernel::KernelModule, uptime::UptimeModule, cpu::CpuModule, memory::MemoryModule, gpu::GpuModule, disk::DiskModule, network::NetworkModule, shell::ShellModule, terminal::TerminalModule, sensors::SensorsModule, display::DisplayModule, packages::PackagesModule, environment::EnvironmentModule, battery::BatteryModule, cpu_freq::CpuFreqModule, vram::VramModule, wifi::WifiModule};

use crate::modules::extended::{HostnameModule, WmDeModule, ThemeModule, SwapModule, LocalIpModule, LocaleModule, HardwareModelModule, MonitorModule, GpuDriverModule, TerminalFontModule};
use crate::modules::custom::CustomShellModule;

fn main() -> io::Result<()> {
    // Parse command line arguments
    let args = Cli::parse();
    
    // Load configuration
    let conf = config::Config::load(args.config.as_deref());

    // Register all built-in modules.
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

    // Inject custom shell scripts
    for script in &conf.custom {
        available_modules.push(Box::new(CustomShellModule {
            module_name: script.name.clone(),
            command: script.command.clone(),
            timeout_ms: script.timeout_ms,
            cache_minutes: script.cache_minutes,
        }));
    }

    // Handle subcommands
    let command = args.command.unwrap_or(crate::cli::Commands::Run);
    match command {
        crate::cli::Commands::ListModules => {
            println!("{}", "Available Modules:".bold().green());
            for m in &available_modules {
                println!("  - {}", m.name());
            }
            return Ok(());
        }
        crate::cli::Commands::Doctor => {
            println!("{}", "--- System Doctor ---".bold().blue());
            println!("Version:         {}", env!("CARGO_PKG_VERSION"));
            println!("Config Path:     {}", dirs::config_dir().map(|mut p| { p.push("hyperfetch"); p.to_string_lossy().to_string() }).unwrap_or_else(|| "Unknown".to_string()));
            println!("Custom Scripts:  {}", conf.custom.len());
            println!("Rayon Threads:   {}", rayon::current_num_threads());
            return Ok(());
        }
        crate::cli::Commands::Completions { shell } => {
            use clap::CommandFactory;
            use clap_complete::generate;
            let mut cmd = crate::cli::Cli::command();
            generate(shell, &mut cmd, "hyperfetch", &mut io::stdout());
            return Ok(());
        }
        crate::cli::Commands::Init => {
            let path = dirs::config_dir().map(|mut p| { p.push("hyperfetch"); p.push("config.toml"); p }).unwrap();
            if path.exists() {
                println!("{}", "Configuration already exists.".yellow());
            } else {
                let _ = config::Config::load(None);
                println!("{} {}", "Initialized default configuration at".green(), path.display());
            }
            return Ok(());
        }
        crate::cli::Commands::Man => {
            use clap::CommandFactory;
            let cmd = crate::cli::Cli::command();
            let man = clap_mangen::Man::new(cmd);
            let mut buffer = Vec::new();
            man.render(&mut buffer).expect("Failed to render man page");
            print!("{}", String::from_utf8_lossy(&buffer));
            return Ok(());
        }
        crate::cli::Commands::Run => {}
    }

    let mut cache = HardwareCache::load();
    let mut cache_modified = false;

    // TUI Setup for Live Mode
    if args.live {
        terminal::enable_raw_mode()?;
        execute!(io::stdout(), EnterAlternateScreen, cursor::Hide, terminal::Clear(terminal::ClearType::All))?;
    }

    // Main execution loop
    loop {
        let loop_start = Instant::now();
        
        use rayon::prelude::*;

        let mut requested_names: Vec<String> = vec![];
        let to_parse = if let Some(cli_mods) = &args.modules {
            cli_mods.split(',').map(|s| s.trim().to_string()).collect()
        } else {
            conf.modules.clone()
        };

        for m in to_parse {
            let normalized = if conf.custom.iter().any(|c| c.name.to_lowercase() == m.to_lowercase()) {
                m 
            } else {
                m.replace('-', " ").replace('_', " ")
            };

            match normalized.as_str() {
                "system" => requested_names.extend(vec!["os".to_string(), "kernel".to_string(), "uptime".to_string(), "cpu".to_string(), "cpu speed".to_string(), "memory".to_string(), "disk".to_string()]),
                "hardware" => requested_names.extend(vec!["cpu".to_string(), "cpu speed".to_string(), "gpu".to_string(), "memory".to_string(), "disk".to_string()]),
                "network" => requested_names.extend(vec!["network".to_string(), "wifi".to_string(), "local ip".to_string()]),
                "cpu freq" => requested_names.push("cpu speed".to_string()),
                _ => requested_names.push(normalized),
            }
        }

        let results: Vec<_> = requested_names.par_iter().map(|m_name| {
            let name_lower = m_name.to_lowercase();
            if (name_lower == "gpu" || name_lower == "display") && cache.data.contains_key(&name_lower) {
                let cached_data = cache.data.get(&name_lower).unwrap().clone();
                return (name_lower, cached_data);
            }
            if let Some(module) = available_modules.iter().find(|m| m.name().to_lowercase() == name_lower) {
                (name_lower, module.fetch())
            } else {
                (name_lower, vec![])
            }
        }).collect();

        let mut sys_info = Vec::with_capacity(32);
        let mut seen_keys = std::collections::HashSet::new();
        let mut os_name = "Linux".to_string();

        sys_info.push(("Version".to_string(), env!("CARGO_PKG_VERSION").to_string()));

        for (name_lower, entries) in results {
            if (name_lower == "gpu" || name_lower == "display") && !cache.data.contains_key(&name_lower) && !entries.is_empty() {
                 cache.data.insert(name_lower.clone(), entries.clone());
                 cache_modified = true;
            }
            for (key, val) in entries {
                if seen_keys.contains(&key) || key == "Version" { continue; }
                seen_keys.insert(key.clone());
                if key == "OS" { os_name = val.clone(); }
                sys_info.push((key, val));
            }
        }

        if cache_modified {
            let _ = cache.save();
            cache_modified = false;
        }

        if args.json {
            let mut map = serde_json::Map::new();
            for (k, v) in &sys_info {
                let key = k.to_lowercase().replace(' ', "_").replace(|c: char| !c.is_alphanumeric() && c != '_', "");
                map.insert(key, serde_json::Value::String(v.clone()));
            }
            println!("{}", serde_json::to_string_pretty(&map).unwrap_or_default());
        } else {
            let os_target = if let Some(override_logo) = &args.logo {
                override_logo.clone()
            } else if conf.logo != "default" && !conf.logo.is_empty() {
                conf.logo.clone()
            } else {
                os_name
            };

            let logo = ui::ascii::AsciiLogo::get(&os_target);
            
            if args.table {
                ui::display::print_table(&sys_info, args.live);
            } else {
                ui::display::print_layout(&logo, &sys_info, &conf.theme, args.live);
            }

            if args.benchmark {
                println!("\n{}", "--- Benchmark ---".bright_blue().bold());
                println!("Modules:   {}", requested_names.len());
                println!("Time:      {:?}", loop_start.elapsed());
                println!("Version:   {}\n", env!("CARGO_PKG_VERSION"));
            } else if args.live {
                let (term_width, term_height) = terminal::size().unwrap_or((80, 24));
                let status_line = format!("Live Mode | Refresh: 1s | Version: {} | Took: {:?}", env!("CARGO_PKG_VERSION"), loop_start.elapsed());
                let exit_line = "Press 'q' or Esc to exit";
                
                let s_pad = (term_width as usize).saturating_sub(status_line.chars().count()) / 2;
                let e_pad = (term_width as usize).saturating_sub(exit_line.chars().count()) / 2;

                // Use MoveTo for status lines as well to prevent raw mode drift
                let _ = execute!(io::stdout(), 
                    cursor::MoveTo(0, (term_height - 2) as u16),
                    terminal::Clear(terminal::ClearType::CurrentLine),
                    cursor::MoveTo(s_pad as u16, (term_height - 2) as u16)
                );
                print!("{}\x1B[K", status_line.bright_black().italic());

                let _ = execute!(io::stdout(), 
                    cursor::MoveTo(0, (term_height - 1) as u16),
                    terminal::Clear(terminal::ClearType::CurrentLine),
                    cursor::MoveTo(e_pad as u16, (term_height - 1) as u16)
                );
                print!("{}\x1B[K", exit_line.bright_black().italic());
                
                let _ = io::stdout().flush();
            } else {
                println!("{}", format!("fetch took {:?}", loop_start.elapsed()).bright_black().italic());
            }
        }

        if !args.live {
            break;
        }

        if event::poll(Duration::from_secs(1))? {
            if let Event::Key(key) = event::read()? {
                if key.code == KeyCode::Char('q') || key.code == KeyCode::Esc {
                    break;
                }
            }
        }
    }

    if args.live {
        execute!(io::stdout(), cursor::Show, LeaveAlternateScreen)?;
        terminal::disable_raw_mode()?;
    }

    Ok(())
}
