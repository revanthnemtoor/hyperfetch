use clap::{Parser, Subcommand};

/// A blazingly fast system fetch tool in Rust.
#[derive(Parser, Debug)]
#[command(name = "hyperfetch", version = "0.1.0", author = "Revanth", about = "An extremely fast system fetch tool written in Rust")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Enable live interactive dashboard mode
    #[arg(short, long, global = true)]
    pub live: bool,

    /// Specify a configuration file path or a profile name
    #[arg(short, long, global = true)]
    pub config: Option<String>,

    /// Override the visual logo with another OS's ASCII or a file path
    #[arg(long, global = true)]
    pub logo: Option<String>,

    /// Override the modules to run, comma-separated (e.g., os,cpu,memory)
    #[arg(long, global = true)]
    pub modules: Option<String>,

    /// Apply a custom visual layout theme (e.g., minimal, neon)
    #[arg(long, global = true)]
    pub theme: Option<String>,

    /// Output raw JSON instead of the ASCII UI (perfect for piping to jq)
    #[arg(short, long, global = true)]
    pub json: bool,

    /// Output a clean tabular text format instead of the ASCII UI
    #[arg(long, global = true)]
    pub table: bool,

    /// Show detailed performance profiling metrics at the end of execution
    #[arg(long, global = true)]
    pub benchmark: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// Execute the fetch layout (Default action)
    Run,
    /// Print a list of all native and configured custom modules
    ListModules,
    /// Verify system sensors and cache capabilities
    Doctor,
    /// Generate shell completion scripts
    Completions {
        /// The shell to generate completions for
        shell: clap_complete::Shell,
    },
    /// Initialize the default configuration file
    Init,
}
