use clap::Parser;

/// A blazingly fast system fetch tool in Rust.
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// Enable live interactive dashboard mode
    #[arg(short, long)]
    pub live: bool,

    /// Specify a configuration file path
    #[arg(short, long)]
    pub config: Option<String>,

    /// Override the output theme
    #[arg(short, long)]
    pub theme: Option<String>,

    /// Output raw JSON instead of the ASCII UI (perfect for piping to jq)
    #[arg(short, long)]
    pub json: bool,

    /// Override the visual logo with another OS's ASCII
    #[arg(long)]
    pub logo: Option<String>,
}
