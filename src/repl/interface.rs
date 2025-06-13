//! REPL User Interface Module
//! 
//! Provides the main interface components for the CURSED REPL,
//! including output formatting, error display, and user interaction.

use std::io::{self, Write};
use colored::Colorize;

use crate::repl::{ReplOutput, ReplResult};
use crate::error::CursedError;

/// REPL interface configuration
#[derive(Debug, Clone)]
pub struct InterfaceConfig {
    pub use_colors: bool,
    pub show_line_numbers: bool,
    pub show_timing: bool,
    pub show_types: bool,
    pub verbose_errors: bool,
}

impl Default for InterfaceConfig {
    fn default() -> Self {
        Self {
            use_colors: atty::is(atty::Stream::Stdout),
            show_line_numbers: true,
            show_timing: false,
            show_types: true,
            verbose_errors: false,
        }
    }
}

/// REPL user interface manager
pub struct ReplInterface {
    config: InterfaceConfig,
    line_number: usize,
}

impl ReplInterface {
    /// Create a new REPL interface
    pub fn new(config: InterfaceConfig) -> Self {
        Self {
            config,
            line_number: 1,
        }
    }

    /// Print the welcome message
    pub fn print_welcome(&self, version: &str, working_dir: Option<&std::path::Path>) {
        if self.config.use_colors {
            println!("{}", "🔥 CURSED REPL".bright_red().bold());
            println!("{} {}", "Version:".bright_blue(), version.bright_white());
            println!("{}", "Welcome to the most fire programming language! 🚀".bright_yellow());
        } else {
            println!("🔥 CURSED REPL v{}", version);
            println!("Welcome to the most fire programming language! 🚀");
        }
        
        println!("Type :help for available commands or :exit to quit");
        
        if let Some(dir) = working_dir {
            if self.config.use_colors {
                println!("{} {}", "📁 Working directory:".bright_blue(), dir.display().to_string().bright_white());
            } else {
                println!("📁 Working directory: {}", dir.display());
            }
        }
        
        println!();
    }

    /// Print a formatted output
    pub fn print_output(&self, output: &ReplOutput) {
        if output.is_error {
            self.print_error_output(output);
        } else {
            self.print_success_output(output);
        }
    }

    /// Print successful output
    fn print_success_output(&self, output: &ReplOutput) {
        if !output.content.is_empty() {
            if self.config.show_line_numbers {
                if self.config.use_colors {
                    print!("{} ", format!("[{}]", self.line_number).bright_black());
                } else {
                    print!("[{}] ", self.line_number);
                }
            }

            if self.config.use_colors {
                print!("{}", output.content.bright_white());
            } else {
                print!("{}", output.content);
            }

            if output.show_type && self.config.show_types {
                if self.config.use_colors {
                    print!(" {}", " : <type>".bright_black());
                } else {
                    print!(" : <type>");
                }
            }

            if let Some(duration) = output.execution_time {
                if self.config.show_timing {
                    if self.config.use_colors {
                        print!(" {}", format!("({}ms)", duration.as_millis()).bright_black());
                    } else {
                        print!(" ({}ms)", duration.as_millis());
                    }
                }
            }

            println!();
        }
    }

    /// Print error output
    fn print_error_output(&self, output: &ReplOutput) {
        if self.config.use_colors {
            print!("{} ", "🔥".bright_red());
            print!("{}", "Error:".bright_red().bold());
            print!(" {}", output.content.bright_red());
        } else {
            print!("🔥 Error: {}", output.content);
        }

        if let Some(duration) = output.execution_time {
            if self.config.show_timing {
                if self.config.use_colors {
                    print!(" {}", format!("({}ms)", duration.as_millis()).bright_black());
                } else {
                    print!(" ({}ms)", duration.as_millis());
                }
            }
        }

        println!();
    }

    /// Print a general message
    pub fn print_message(&self, message: &str, message_type: MessageType) {
        match message_type {
            MessageType::Info => {
                if self.config.use_colors {
                    println!("{} {}", "ℹ️".bright_blue(), message.bright_white());
                } else {
                    println!("ℹ️  {}", message);
                }
            }
            MessageType::Warning => {
                if self.config.use_colors {
                    println!("{} {}", "⚠️".bright_yellow(), message.bright_yellow());
                } else {
                    println!("⚠️  {}", message);
                }
            }
            MessageType::Success => {
                if self.config.use_colors {
                    println!("{} {}", "✅".bright_green(), message.bright_green());
                } else {
                    println!("✅ {}", message);
                }
            }
            MessageType::Error => {
                if self.config.use_colors {
                    println!("{} {}", "❌".bright_red(), message.bright_red());
                } else {
                    println!("❌ {}", message);
                }
            }
        }
    }

    /// Print a progress indicator
    pub fn print_progress(&self, message: &str, current: usize, total: usize) {
        if self.config.use_colors {
            print!("\r{} {} [{}/{}]", "⏳".bright_yellow(), message.bright_white(), current, total);
        } else {
            print!("\r⏳ {} [{}/{}]", message, current, total);
        }
        io::stdout().flush().unwrap();
    }

    /// Clear progress indicator
    pub fn clear_progress(&self) {
        print!("\r");
        for _ in 0..80 {
            print!(" ");
        }
        print!("\r");
        io::stdout().flush().unwrap();
    }

    /// Prompt for user input
    pub fn prompt_input(&self, message: &str) -> ReplResult<String> {
        if self.config.use_colors {
            print!("{} ", message.bright_cyan());
        } else {
            print!("{} ", message);
        }
        io::stdout().flush().map_err(|e| CursedError::repl_error(e.to_string()))?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .map_err(|e| CursedError::repl_error(e.to_string()))?;

        Ok(input.trim().to_string())
    }

    /// Prompt for confirmation
    pub fn prompt_confirmation(&self, message: &str) -> ReplResult<bool> {
        loop {
            let prompt = format!("{} (y/n)", message);
            let input = self.prompt_input(&prompt)?;
            
            match input.to_lowercase().as_str() {
                "y" | "yes" => return Ok(true),
                "n" | "no" => return Ok(false),
                _ => {
                    self.print_message("Please enter 'y' or 'n'", MessageType::Warning);
                    continue;
                }
            }
        }
    }

    /// Display a table of data
    pub fn print_table(&self, headers: &[&str], rows: &[Vec<String>]) {
        if rows.is_empty() {
            return;
        }

        // Calculate column widths
        let mut widths: Vec<usize> = headers.iter().map(|h| h.len()).collect();
        
        for row in rows {
            for (i, cell) in row.iter().enumerate() {
                if i < widths.len() {
                    widths[i] = widths[i].max(cell.len());
                }
            }
        }

        // Print header
        if self.config.use_colors {
            for (i, header) in headers.iter().enumerate() {
                print!("{:<width$}", header.bright_blue().bold(), width = widths[i] + 2);
            }
        } else {
            for (i, header) in headers.iter().enumerate() {
                print!("{:<width$}", header, width = widths[i] + 2);
            }
        }
        println!();

        // Print separator
        for width in &widths {
            for _ in 0..*width + 2 {
                print!("-");
            }
        }
        println!();

        // Print rows
        for row in rows {
            for (i, cell) in row.iter().enumerate() {
                if i < widths.len() {
                    if self.config.use_colors {
                        print!("{:<width$}", cell.bright_white(), width = widths[i] + 2);
                    } else {
                        print!("{:<width$}", cell, width = widths[i] + 2);
                    }
                }
            }
            println!();
        }
    }

    /// Increment line number
    pub fn increment_line(&mut self) {
        self.line_number += 1;
    }

    /// Get current line number
    pub fn line_number(&self) -> usize {
        self.line_number
    }

    /// Clear the screen
    pub fn clear_screen(&self) {
        print!("\x1B[2J\x1B[1;1H");
        io::stdout().flush().unwrap();
    }
}

/// Message types for output formatting
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MessageType {
    Info,
    Warning,
    Success,
    Error,
}

impl Default for ReplInterface {
    fn default() -> Self {
        Self::new(InterfaceConfig::default())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn test_interface_creation() {
        let interface = ReplInterface::default();
        assert_eq!(interface.line_number(), 1);
    }

    #[test]
    fn test_output_formatting() {
        let interface = ReplInterface::new(InterfaceConfig {
            use_colors: false,
            ..Default::default()
        });

        let output = ReplOutput::success("test result".to_string())
            .with_timing(Duration::from_millis(10));

        // This would normally print to stdout, but we can't easily test that
        // The important thing is that it doesn't panic
        interface.print_output(&output);
    }

    #[test]
    fn test_message_types() {
        let interface = ReplInterface::new(InterfaceConfig {
            use_colors: false,
            ..Default::default()
        });

        interface.print_message("Info message", MessageType::Info);
        interface.print_message("Warning message", MessageType::Warning);
        interface.print_message("Success message", MessageType::Success);
        interface.print_message("Error message", MessageType::Error);
    }

    #[test]
    fn test_table_printing() {
        let interface = ReplInterface::new(InterfaceConfig {
            use_colors: false,
            ..Default::default()
        });

        let headers = vec!["Name", "Type", "Value"];
        let rows = vec![
            vec!["x".to_string(), "int".to_string(), "42".to_string()],
            vec!["y".to_string(), "string".to_string(), "hello".to_string()],
        ];

        interface.print_table(&headers, &rows);
    }
}
