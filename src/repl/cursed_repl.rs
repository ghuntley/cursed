//! CURSED REPL implementation

use crate::error::CursedError;
use super::session_manager::SessionManager;
use std::collections::HashMap;

pub struct CursedRepl {
    session_manager: SessionManager,
    context: HashMap<String, String>,
}

impl CursedRepl {
    pub fn new() -> Result<Self, CursedError> {
        Ok(Self {
            session_manager: SessionManager::new()?,
            context: HashMap::new(),
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
}
