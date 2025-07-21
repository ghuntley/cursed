//! CURSED REPL implementation

use crate::error::CursedError;
use crate::execution::CursedExecutionEngine;
use super::session_manager::SessionManager;
use std::collections::HashMap;
use std::fs;
use rustyline::error::ReadlineError;
use rustyline::{Editor, DefaultEditor};
use colored::*;

pub struct CursedRepl {
    session_manager: SessionManager,
    context: HashMap<String, String>,
    execution_engine: CursedExecutionEngine,
}

impl CursedRepl {
    pub fn new() -> Result<Self, CursedError> {
        Ok(Self {
            session_manager: SessionManager::new()?,
            context: HashMap::new(),
            execution_engine: CursedExecutionEngine::new()?,
        })
    }

    pub fn evaluate(&mut self, input: &str) -> Result<String, CursedError> {
        // Store input for session management
        self.session_manager.add_to_history(input.to_string());
        
        // Simple evaluation - extend as needed
        if input.trim().is_empty() {
            return Ok(String::new());
        }

        // Basic CURSED language constructs
        if input.starts_with("let ") {
            self.handle_variable_declaration(input)
        } else if input.starts_with("print ") {
            self.handle_print_statement(input)
        } else {
            Ok(format!("Evaluated: {}", input))
        }
    }

    fn handle_variable_declaration(&mut self, input: &str) -> Result<String, CursedError> {
        // Basic variable assignment: let x = value
        let parts: Vec<&str> = input.split('=').collect();
        if parts.len() == 2 {
            let var_name = parts[0].trim().strip_prefix("let ").unwrap_or("").trim();
            let value = parts[1].trim();
            self.context.insert(var_name.to_string(), value.to_string());
            Ok(format!("Variable '{}' set to '{}'", var_name, value))
        } else {
            Err(CursedError::syntax_error("Invalid variable declaration"))
        }
    }

    fn handle_print_statement(&self, input: &str) -> Result<String, CursedError> {
        let content = input.strip_prefix("print ").unwrap_or("").trim();
        
        // Remove quotes if present
        let content = if content.starts_with('"') && content.ends_with('"') {
            &content[1..content.len()-1]
        } else if let Some(value) = self.context.get(content) {
            value
        } else {
            content
        };
        
        Ok(content.to_string())
    }

    pub fn get_session_manager(&self) -> &SessionManager {
        &self.session_manager
    }

    pub fn load_startup_file(&mut self, path: &str) -> Result<(), CursedError> {
        let content = fs::read_to_string(path)
            .map_err(|e| CursedError::Io(format!("Failed to read startup file: {}", e)))?;
        
        // Execute startup file content
        match self.execution_engine.execute(&content) {
            Ok(_) => println!("{} Startup file loaded successfully", "✓".green()),
            Err(e) => println!("{} Error in startup file: {}", "✗".red(), e),
        }
        
        Ok(())
    }

    pub fn run_repl_loop(&mut self, no_history: bool) -> Result<(), Box<dyn std::error::Error>> {
        let mut rl = DefaultEditor::new()?;
        
        if !no_history {
            // Try to load history file
            let history_path = dirs::home_dir()
                .map(|mut path| {
                    path.push(".cursed_history");
                    path
                })
                .unwrap_or_else(|| std::path::PathBuf::from(".cursed_history"));
            
            if let Err(e) = rl.load_history(&history_path) {
                match e {
                    ReadlineError::Io(ref io_err) if io_err.kind() == std::io::ErrorKind::NotFound => {
                        // File not found is expected on first run
                    }
                    _ => {
                        println!("{} Failed to load history: {}", "Warning".yellow(), e);
                    }
                }
            }
        }

        self.print_welcome();
        
        loop {
            let readline = rl.readline(&format!("{} ", "cursed>".cyan().bold()));
            
            match readline {
                Ok(line) => {
                    if line.trim().is_empty() {
                        continue;
                    }
                    
                    // Handle special commands
                    if let Some(response) = self.handle_special_command(&line) {
                        if response == "exit" {
                            break;
                        }
                        println!("{}", response);
                        continue;
                    }
                    
                    // Add to history
                    if !no_history {
                        rl.add_history_entry(line.as_str())?;
                    }
                    
                    // Process the input
                    match self.process_input(&line) {
                        Ok(result) => {
                            if !result.trim().is_empty() {
                                println!("{}", result);
                            }
                        }
                        Err(e) => {
                            println!("{} {}", "Error:".red().bold(), e);
                        }
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("{}", "^C".yellow());
                    continue;
                }
                Err(ReadlineError::Eof) => {
                    println!("{}", "Goodbye!".cyan());
                    break;
                }
                Err(err) => {
                    println!("{} {}", "Error:".red().bold(), err);
                    break;
                }
            }
        }
        
        // Save history
        if !no_history {
            let history_path = dirs::home_dir()
                .map(|mut path| {
                    path.push(".cursed_history");
                    path
                })
                .unwrap_or_else(|| std::path::PathBuf::from(".cursed_history"));
            
            if let Err(e) = rl.save_history(&history_path) {
                println!("{} Failed to save history: {}", "Warning".yellow(), e);
            }
        }
        
        Ok(())
    }
    
    fn print_welcome(&self) {
        println!("{}", "CURSED REPL".cyan().bold());
        println!("{}", "Type :help for help, :quit to exit".dimmed());
        println!();
    }
    
    fn handle_special_command(&self, input: &str) -> Option<String> {
        let input = input.trim();
        
        match input {
            ":quit" | ":exit" | ":q" => Some("exit".to_string()),
            ":help" | ":h" => Some(self.get_help_text()),
            ":history" => Some(self.get_history_text()),
            ":clear" => Some(self.clear_screen()),
            ":vars" => Some(self.show_variables()),
            ":version" => Some(format!("CURSED REPL v{}", option_env!("CARGO_PKG_VERSION").unwrap_or("unknown"))),
            _ if input.starts_with(':') => Some(format!("Unknown command: {}. Type :help for available commands.", input)),
            _ => None,
        }
    }
    
    fn get_help_text(&self) -> String {
        format!("{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}",
            "Available commands:".cyan().bold(),
            "  :help, :h       - Show this help message",
            "  :quit, :exit, :q - Exit the REPL",
            "  :history        - Show command history",
            "  :clear          - Clear the screen",
            "  :vars           - Show current variables",
            "  :version        - Show version information",
            "",
            "CURSED language features:".cyan().bold(),
            "  - Variables: let x = 42",
            "  - Functions: func add(a, b) { return a + b }",
            "  - Control flow: if, while, for loops",
            "  - And much more!")
    }
    
    fn get_history_text(&self) -> String {
        let history = self.session_manager.get_history();
        if history.is_empty() {
            "No history available".dimmed().to_string()
        } else {
            let mut result = "Command History:".cyan().bold().to_string();
            for (i, entry) in history.iter().enumerate() {
                result.push_str(&format!("\n{:3}: {}", i + 1, entry));
            }
            result
        }
    }
    
    fn clear_screen(&self) -> String {
        print!("\x1B[2J\x1B[1;1H");
        "".to_string()
    }
    
    fn show_variables(&self) -> String {
        if self.context.is_empty() {
            "No variables defined".dimmed().to_string()
        } else {
            let mut result = "Variables:".cyan().bold().to_string();
            for (key, value) in &self.context {
                result.push_str(&format!("\n  {} = {}", key.green(), value));
            }
            result
        }
    }
    
    fn process_input(&mut self, input: &str) -> Result<String, CursedError> {
        // Store in session history
        self.session_manager.add_to_history(input.to_string());
        
        // Try to execute with the CURSED execution engine
        match self.execution_engine.execute_repl(input) {
            Ok(result) => Ok(result),
            Err(e) => {
                // Fall back to basic evaluation for simple cases
                self.evaluate(input)
            }
        }
    }
}
