use colored::*;
use crate::config::ThemeConfig;

/// Helper to wrap text in ANSI color codes based on a string name.
/// This matches the theme colors defined in the configuration.
fn colorize(text: &str, color: &str) -> colored::ColoredString {
    match color {
        "cyan" => text.cyan().bold(),
        "yellow" => text.yellow().bold(),
        "blue" => text.blue().bold(),
        "red" => text.red().bold(),
        "green" => text.green().bold(),
        "magenta" => text.magenta().bold(),
        "black" => text.black().bold(),
        "white" => text.white().bold(),
        _ => text.normal(),
    }
}

/// Renders the classic side-by-side layout: ASCII logo on the left, system data on the right.
pub fn print_layout(logo: &crate::ui::ascii::AsciiLogo, sys_info: &Vec<(String, String)>, theme: &ThemeConfig) {
    let logo_lines = &logo.lines;
    
    let max_lines = std::cmp::max(logo_lines.len(), sys_info.len());
    let logo_width = logo_lines.iter().map(|l| l.chars().count()).max().unwrap_or(16);

    for line_idx in 0..max_lines {
        // Print Logo Line
        let logo_str = if line_idx < logo_lines.len() {
            let line = &logo_lines[line_idx];
            let padding = " ".repeat(logo_width.saturating_sub(line.chars().count()));
            format!("{}{}", colorize(line, &logo.color), padding)
        } else {
            " ".repeat(logo_width) // Padding if logo is shorter
        };

        // Print Sys Info Line
        let info_str = if line_idx < sys_info.len() {
            let (key, val) = &sys_info[line_idx];
            let key_colored = colorize(key, &theme.color_key);
            let val_colored = colorize(val, &theme.color_value);
            format!("{} {} {}", key_colored, theme.separator, val_colored)
        } else {
            "".to_string()
        };

        println!("  {}   {}", logo_str, info_str);
    }
    println!();
}

/// Renders system information in a clean, vertical tabular format without ASCII art.
/// Auto-aligns values based on the longest property key.
pub fn print_table(sys_info: &Vec<(String, String)>) {
    let max_key_len = sys_info.iter().map(|(k, _)| k.chars().count()).max().unwrap_or(10);
    
    println!();
    for (key, val) in sys_info {
        let padding = " ".repeat(max_key_len.saturating_sub(key.chars().count()));
        println!("{}{}  {}", key.bold(), padding, val);
    }
    println!();
}
