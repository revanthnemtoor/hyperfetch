use colored::*;

pub fn print_layout(logo: &crate::ui::ascii::AsciiLogo, sys_info: &Vec<(String, String)>) {
    let logo_lines = &logo.lines;
    
    let max_lines = std::cmp::max(logo_lines.len(), sys_info.len());
    let logo_width = logo_lines.iter().map(|l| l.chars().count()).max().unwrap_or(16);

    for line_idx in 0..max_lines {
        // Print Logo Line
        let logo_str = if line_idx < logo_lines.len() {
            let line = logo_lines[line_idx];
            let padding = " ".repeat(logo_width.saturating_sub(line.chars().count()));
            let colored_line = match logo.color {
                "cyan" => line.cyan().bold(),
                "yellow" => line.yellow().bold(),
                "blue" => line.blue().bold(),
                "red" => line.red().bold(),
                "green" => line.green().bold(),
                "magenta" => line.magenta().bold(),
                _ => line.white().bold(),
            };
            format!("{}{}", colored_line, padding)
        } else {
            " ".repeat(logo_width) // Padding if logo is shorter
        };

        // Print Sys Info Line
        let info_str = if line_idx < sys_info.len() {
            let (key, val) = &sys_info[line_idx];
            let key_colored = match logo.color {
                "cyan" => key.cyan().bold(),
                "yellow" => key.yellow().bold(),
                "blue" => key.blue().bold(),
                "red" => key.red().bold(),
                "green" => key.green().bold(),
                "magenta" => key.magenta().bold(),
                _ => key.white().bold(),
            };
            format!("{}: {}", key_colored, val)
        } else {
            "".to_string()
        };

        println!("  {}   {}", logo_str, info_str);
    }
    println!();
}
