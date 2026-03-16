use clap::{Parser, Subcommand};

/// Command-line interface definition for hyperfetch.
#[derive(Parser, Debug)]
#[command(name = "hyperfetch", version = "0.2.0-next", author = "Revanth", about = "An extremely fast system fetch tool written in Rust")]
pub struct Cli {
    /// Optional subcommand to execute specific tasks instead of a standard fetch
    #[command(subcommand)]
    pub command: Option<Commands>,

    /// Future feature: Real-time dashboard mode (currently in development)
    #[arg(short, long, global = true)]
    pub live: bool,

    /// Override the default configuration with a custom file path or named profile
    #[arg(short, long, global = true)]
    pub config: Option<String>,

    /// Display a different OS's ASCII logo or provide a path to a custom logo file
    #[arg(long, global = true)]
    pub logo: Option<String>,

    /// Specify exactly which modules to execute, overriding the configuration file
    #[arg(long, global = true)]
    pub modules: Option<String>,

    /// Theme override for colors and formatting
    #[arg(long, global = true)]
    pub theme: Option<String>,

    /// Output system information in machine-readable JSON format
    #[arg(short, long, global = true)]
    pub json: bool,

    /// Output system information in a clean, non-ASCII table format
    #[arg(long, global = true)]
    pub table: bool,

    /// Display timing metrics and module execution statistics
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
    /// Generate a man page for hyperfetch
    Man,
}
