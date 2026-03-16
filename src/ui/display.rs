use colored::*;
use crate::config::ThemeConfig;
use crossterm::{cursor, terminal, execute};
use std::io;

/// Helper to wrap text in ANSI color codes based on a string name.
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

/// Renders the classic side-by-side layout with robust centering for TUI mode.
pub fn print_layout(logo: &crate::ui::ascii::AsciiLogo, sys_info: &Vec<(String, String)>, theme: &ThemeConfig, live: bool) {
    let (term_width, term_height) = terminal::size().unwrap_or((80, 24));
    let term_width = term_width as usize;
    let term_height = term_height as usize;

    let logo_lines = &logo.lines;
    let max_lines = std::cmp::max(logo_lines.len(), sys_info.len());
    
    let logo_width = if logo_lines.is_empty() { 0 } else { 
        logo_lines.iter().map(|l| l.chars().count()).max().unwrap_or(0) 
    };

    let info_width = sys_info.iter()
        .map(|(k, v)| k.chars().count() + theme.separator.chars().count() + v.chars().count() + 2)
        .max()
        .unwrap_or(40);

    let content_width = if logo_width > 0 { logo_width + 3 + info_width } else { info_width };
    
    let left_padding = if live && term_width > content_width + 2 {
        (term_width - content_width) / 2
    } else {
        2
    };

    let top_padding = if live && term_height > max_lines + 3 {
        (term_height - max_lines - 3) / 2
    } else {
        0
    };

    for line_idx in 0..max_lines {
        if live {
            let _ = execute!(io::stdout(), cursor::MoveTo(left_padding as u16, (top_padding + line_idx) as u16));
        } else {
            print!("{}", " ".repeat(left_padding));
        }

        let mut printed_width = 0;

        // Logo Column
        if logo_width > 0 {
            if line_idx < logo_lines.len() {
                let line = &logo_lines[line_idx];
                let visible_len = line.chars().count();
                let logo_pad = logo_width.saturating_sub(visible_len);
                print!("{}{}", colorize(line, &logo.color), " ".repeat(logo_pad));
            } else {
                print!("{}", " ".repeat(logo_width));
            }
            print!("   ");
            printed_width += logo_width + 3;
        }

        // Info Column
        if line_idx < sys_info.len() {
            let (key, val) = &sys_info[line_idx];
            let key_colored = colorize(key, &theme.color_key);
            
            let remaining = term_width.saturating_sub(left_padding + printed_width + key.chars().count() + theme.separator.chars().count() + 4);
            let val_str = if val.chars().count() > remaining && live && remaining > 3 {
                format!("{}...", &val[..remaining.saturating_sub(3)])
            } else {
                val.clone()
            };
            
            print!("{} {} {}", key_colored, theme.separator, colorize(&val_str, &theme.color_value));
        }

        if live {
            print!("\x1B[K"); // Clear to end of line
        } else {
            println!();
        }
    }
}

/// Renders system information in a vertical table with centering for TUI mode.
pub fn print_table(sys_info: &Vec<(String, String)>, live: bool) {
    let (term_width, term_height) = terminal::size().unwrap_or((80, 24));
    let term_width = term_width as usize;
    let term_height = term_height as usize;
    
    let max_key_len = sys_info.iter().map(|(k, _)| k.chars().count()).max().unwrap_or(10);
    let max_val_len = sys_info.iter().map(|(_, v)| v.chars().count()).max().unwrap_or(30);
    let content_width = max_key_len + 2 + max_val_len;

    let left_padding = if live && term_width > content_width + 2 {
        (term_width - content_width) / 2
    } else {
        2
    };

    let top_padding = if live && term_height > sys_info.len() + 3 {
        (term_height - sys_info.len() - 3) / 2
    } else {
        0
    };

    for (line_idx, (key, val)) in sys_info.iter().enumerate() {
        if live {
            let _ = execute!(io::stdout(), cursor::MoveTo(left_padding as u16, (top_padding + line_idx) as u16));
        } else {
            print!("{}", " ".repeat(left_padding));
        }
        
        let padding = " ".repeat(max_key_len.saturating_sub(key.chars().count()));
        print!("{}{}  {}", key.bold(), padding, val);
        
        if live {
            print!("\x1B[K");
        } else {
            println!();
        }
    }
}
