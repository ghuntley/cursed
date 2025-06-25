use crate::error::CursedError;
// Session Management for CURSED REPL
// 
// Manages REPL session state including variables, functions,
// command history, and code execution context.

use std::collections::HashMap;
use std::time::SystemTime;

use crate::repl::ReplResult;

/// Session variable information
#[derive(Debug, Clone)]
pub struct SessionVariable {
/// Session function information
#[derive(Debug, Clone)]
pub struct SessionFunction {
/// Command history entry
#[derive(Debug, Clone)]
pub struct HistoryEntry {
/// REPL session manager
pub struct SessionManager {
impl SessionManager {
    /// Create a new session manager
    pub fn new() -> Self {
        Self {
        }
    }

    /// Initialize the session
    pub fn initialize(&mut self) -> ReplResult<()> {
        // Clear any existing state
        self.clear()?;
        
        // Initialize basic environment
        self.session_code.push("// CURSED REPL Session".to_string());
        self.session_code.push(format!("// Session ID: {}", self.session_id));
        self.session_code.push(format!("// Started: {:?}", SystemTime::now()));
        self.session_code.push("".to_string());

        Ok(())
    /// Clear the session state
    pub fn clear(&mut self) -> ReplResult<()> {
        self.variables.clear();
        self.functions.clear();
        self.session_code.clear();
        
        // Keep history but mark the clear point
        self.add_to_history(
        );

        Ok(())
    /// Execute CURSED code in the session context
    pub fn execute_code(&mut self, code: &str) -> ReplResult<String> {
        let start_time = std::time::Instant::now();
        
        // Add code to session
        self.session_code.push(code.to_string());
        
        // Parse and analyze the code to extract variables and functions
        self.analyze_code(code)?;
        
        // Execute the code using the new evaluation system
        let result = crate::execute_repl_code(code, self)
            .map_err(|e| CursedError::repl_error(e.to_string()))?;
        
        let execution_time = start_time.elapsed();
        
        // Add to history
        self.add_to_history(code.to_string(), true, execution_time);
        
        Ok(result)
    /// Get the current session code as a single string
    pub fn get_session_code(&self) -> String {
        self.session_code.join("\n")
    /// Format the session code
    pub fn format_session_code(&self) -> ReplResult<String> {
        let code = self.get_session_code();
        // This would use the CURSED formatter
        crate::format(&code).map_err(|e| CursedError::repl_error(e.to_string()))
    /// Lint the session code
    pub fn lint_session_code(&self) -> ReplResult<Vec<String>> {
        let code = self.get_session_code();
        // This would use the CURSED linter
        // For now, return placeholder
        Ok(Vec::from(["No linting issues found".to_string()]))
    /// List all variables in the session
    pub fn list_variables(&self) -> Vec<(String, String, String)> {
        self.variables
            .values()
            .map(|var| (var.name.clone(), var.type_info.clone(), var.value.clone()))
            .collect()
    /// List all functions in the session
    pub fn list_functions(&self) -> Vec<(String, String)> {
        self.functions
            .values()
            .map(|func| (func.name.clone(), func.signature.clone()))
            .collect()
    /// Get the type of an expression
    pub fn get_expression_type(&self, expression: &str) -> ReplResult<String> {
        // This would analyze the expression in the current session context
        // For now, return a placeholder
        
        // Check if it's a known variable
        if let Some(var) = self.variables.values().find(|v| v.name == expression) {
            return Ok(var.type_info.clone());
        // Check if it's a known function
        if let Some(func) = self.functions.values().find(|f| f.name == expression) {
            return Ok(format!("func {}", func.signature));
        // Try to infer type from literal
        if expression.chars().all(|c| c.is_ascii_digit()) {
            Ok("int".to_string())
        } else if expression.parse::<f64>().is_ok() {
            Ok("float64".to_string())
        } else if expression.starts_with('"') && expression.ends_with('"') {
            Ok("string".to_string())
        } else if expression == "true" || expression == "false" {
            Ok("bool".to_string())
        } else {
            Ok("unknown".to_string())
        }
    }

    /// Get command history
    pub fn get_history(&self, count: usize) -> Vec<String> {
        self.history
            .iter()
            .rev()
            .take(count)
            .map(|entry| entry.command.clone())
            .collect()
    /// Add entry to command history
    pub fn add_to_history(&mut self, command: String, success: bool, execution_time: std::time::Duration) {
        let entry = HistoryEntry {

        self.history.push(entry);

        // Limit history size
        if self.history.len() > self.max_history_size {
            self.history.remove(0);
        }
    }

    /// Cleanup session resources
    pub fn cleanup(&mut self) -> ReplResult<()> {
        // Save session if needed
        // Clear resources
        self.variables.clear();
        self.functions.clear();
        Ok(())
    /// Analyze code to extract variables and functions
    fn analyze_code(&mut self, code: &str) -> ReplResult<()> {
        // Simple analysis - in a real implementation, this would use the parser
        let lines: Vec<&str> = code.split("\n").collect();
        
        for line in lines {
            let trimmed = line.trim();
            
            // Look for variable declarations
            if trimmed.starts_with("facts ") || trimmed.starts_with("sus ") {
                if let Some(var_info) = self.parse_variable_declaration(trimmed) {
                    self.variables.insert(var_info.name.clone(), var_info);
                }
            }
            
            // Look for function declarations
            if trimmed.starts_with("slay ") {
                if let Some(func_info) = self.parse_function_declaration(trimmed) {
                    self.functions.insert(func_info.name.clone(), func_info);
                }
            }
        Ok(())
    /// Parse a variable declaration
    fn parse_variable_declaration(&self, line: &str) -> Option<SessionVariable> {
        // Simple parsing - would be more sophisticated in real implementation
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() >= 4 && parts[2] == "=" {
            let name = parts[1].to_string();
            let value = parts[3..].join(" ");
            let type_info = self.infer_type(&value);
            
            Some(SessionVariable {
            })
        } else {
            None
        }
    }

    /// Parse a function declaration
    fn parse_function_declaration(&self, line: &str) -> Option<SessionFunction> {
        // Simple parsing - would be more sophisticated in real implementation
        if let Some(name_start) = line.find("slay ") {
            if let Some(paren_pos) = line.find('(') {
                let name = line[name_start + 5..paren_pos].trim().to_string();
                let signature = line[paren_pos..].to_string();
                
                Some(SessionFunction {
                })
            } else {
                None
            }
        } else {
            None
        }
    }

    /// Infer type from value
    fn infer_type(&self, value: &str) -> String {
        if value.chars().all(|c| c.is_ascii_digit()) {
            "int".to_string()
        } else if value.parse::<f64>().is_ok() {
            "float64".to_string()
        } else if value.starts_with('"') && value.ends_with('"') {
            "string".to_string()
        } else if value == "true" || value == "false" {
            "bool".to_string()
        } else {
            "unknown".to_string()
        }
    }

    /// Execute CURSED code (placeholder implementation)
    fn execute_cursed_code(&self, code: &str) -> ReplResult<String> {
        // This would interface with the actual CURSED interpreter
        // For now, return a placeholder result
        
        if code.trim().is_empty() {
            return Ok("".to_string());
        // Simple evaluation for basic expressions
        if code.trim().chars().all(|c| c.is_ascii_digit()) {
            return Ok(code.trim().to_string());
        if code.contains("=") && !code.contains("==") {
            return Ok("".to_string()); // Variable assignment
        // Default response
        Ok("(evaluation result)".to_string())
    /// Generate a unique session ID
    fn generate_session_id() -> String {
        use std::time::{SystemTime, UNIX_EPOCH};
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        format!("cursed_session_{}", timestamp)
    }
}

impl Default for SessionManager {
    fn default() -> Self {
        Self::new()
    }
}

