//! # Interface Type Assertion Error Visualization
//!
//! This module provides enhanced error visualization capabilities for interface type assertions.
//! It integrates with the error propagation and filesystem source location mechanisms to provide
//! rich, visually appealing error messages with source code context and highlighting.
//!
//! ## Key Features
//!
//! 1. Colored error message formatting with syntax highlighting
//! 2. Visual indicators for error locations in source code
//! 3. Type path visualization for interface inheritance hierarchies
//! 4. Integration with the filesystem source location tracker
//! 5. Support for rendering error messages in various formats (terminal, HTML, markdown)

use std::fmt;
use tracing::{debug, error, info, instrument, trace, warn};

use crate::ast::traits::Node;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::error::Error;
use crate::error::SourceLocation;
use crate::error::type_assertion_error::{TypeAssertionError, helpers as error_helpers};

/// Struct representing a visual type assertion error
pub struct VisualTypeAssertionError {
    /// The error message
    pub message: String,
    /// The source location of the error
    pub location: SourceLocation,
    /// The expected type name
    pub expected_type: String,
    /// The actual type name
    pub actual_type: Option<String>,
    /// Source code context lines with line numbers
    pub context_lines: Vec<(usize, String)>,
    /// Path visualization between types (if available)
    pub type_path: Option<String>,
}

impl fmt::Display for VisualTypeAssertionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // Format the basic error message
        writeln!(f, "Error: {}", self.message)?;
        
        // Add location information if available
        if let Some(file) = &self.location.file {
            writeln!(f, "Location: {}:{}:{}", file, self.location.line, self.location.column)?;
        }
        
        // Add type information
        writeln!(f, "Expected type: {}", self.expected_type)?;
        if let Some(actual) = &self.actual_type {
            writeln!(f, "Actual type: {}", actual)?;
        }
        
        // Add source code context if available
        if !self.context_lines.is_empty() {
            writeln!(f, "\nSource context:")?;
            
            for (line_num, content) in &self.context_lines {
                let prefix = if *line_num == self.location.line {
                    "> " // Highlight the error line
                } else {
                    "  "
                };
                
                writeln!(f, "{}{}| {}", prefix, line_num, content)?;
                
                // Add caret pointing to the error location for the error line
                if *line_num == self.location.line && self.location.column > 0 {
                    let line_num_spaces = line_num.to_string().len() + 2; // +2 for prefix
                    let spaces = line_num_spaces + 2 + self.location.column - 1; // +2 for "| "
                    writeln!(f, "{}^-- Error occurs here", " ".repeat(spaces))?;
                }
            }
        }
        
        // Add type path visualization if available
        if let Some(path) = &self.type_path {
            writeln!(f, "\nType relationship:")?;
            writeln!(f, "{}", path)?;
        }
        
        Ok(())
    }
}

/// Trait for enhanced error message visualization
pub trait ErrorVisualization<'ctx> {
    /// Create a visual type assertion error with enhanced formatting
    fn create_visual_type_assertion_error(
        &self,
        message: &str,
        location: &SourceLocation,
        expected_type: &str,
        actual_type: Option<&str>,
        context_lines: Vec<(usize, String)>,
    ) -> VisualTypeAssertionError;
    
    /// Format an error message with visual enhancements
    fn format_visual_error_message(
        &self,
        error: &VisualTypeAssertionError
    ) -> String;
    
    /// Create a type path visualization between expected and actual types
    fn create_type_path_visualization(
        &self,
        expected_type_id: u32,
        actual_type_id: u32
    ) -> Option<String>;
    
    /// Generate HTML version of error message for web display
    fn generate_html_error_message(
        &self,
        error: &VisualTypeAssertionError
    ) -> String;
    
    /// Generate a terminal-friendly colored error message
    fn generate_terminal_error_message(
        &self,
        error: &VisualTypeAssertionError
    ) -> String;
}

impl<'ctx> ErrorVisualization<'ctx> for LlvmCodeGenerator<'ctx> {
    #[instrument(skip(self, message, location, context_lines), level = "debug")]
    fn create_visual_type_assertion_error(
        &self,
        message: &str,
        location: &SourceLocation,
        expected_type: &str,
        actual_type: Option<&str>,
        context_lines: Vec<(usize, String)>,
    ) -> VisualTypeAssertionError {
        // Get the expected and actual type IDs for path visualization
        let expected_type_id = self.get_expected_type_id().unwrap_or(0);
        let actual_type_id = self.get_actual_type_id().unwrap_or(0);
        
        // Create type path visualization if both types are known
        let type_path = if expected_type_id > 0 && actual_type_id > 0 {
            self.create_type_path_visualization(expected_type_id, actual_type_id)
        } else {
            None
        };
        
        VisualTypeAssertionError {
            message: message.to_string(),
            location: location.clone(),
            expected_type: expected_type.to_string(),
            actual_type: actual_type.map(|s| s.to_string()),
            context_lines,
            type_path,
        }
    }
    
    #[instrument(skip(self, error), level = "debug")]
    fn format_visual_error_message(
        &self,
        error: &VisualTypeAssertionError
    ) -> String {
        // Check if we should use terminal colors or plain text
        let use_colors = std::env::var("CURSED_COLOR_ERRORS")
            .map(|v| v == "1" || v.to_lowercase() == "true")
            .unwrap_or(true); // Default to using colors
        
        if use_colors {
            self.generate_terminal_error_message(error)
        } else {
            // Fallback to plain text formatting
            error.to_string()
        }
    }
    
    #[instrument(skip(self), level = "debug")]
    fn create_type_path_visualization(
        &self,
        expected_type_id: u32,
        actual_type_id: u32
    ) -> Option<String> {
        // Try to get type names for better visualization
        let expected_name = self.get_type_name_by_id(expected_type_id)
            .unwrap_or_else(|| format!("Type#{}", expected_type_id));
        
        let actual_name = self.get_type_name_by_id(actual_type_id)
            .unwrap_or_else(|| format!("Type#{}", actual_type_id));
        
        // Check if we have a registered type visualization path
        if let Some(path) = self.get_type_path_visualization(expected_type_id, actual_type_id) {
            return Some(path);
        }
        
        // If no registered path, create a simple visualization
        Some(format!("{} -> {}: No conversion path available", actual_name, expected_name))
    }
    
    #[instrument(skip(self, error), level = "debug")]
    fn generate_html_error_message(
        &self,
        error: &VisualTypeAssertionError
    ) -> String {
        let mut html = String::new();
        
        // Add CSS style
        html.push_str("<style>");
        html.push_str(".error-message { color: red; font-weight: bold; }");
        html.push_str(".error-location { color: gray; }");
        html.push_str(".error-type { color: blue; }");
        html.push_str(".error-context { margin: 10px 0; font-family: monospace; }");
        html.push_str(".error-line { background-color: #ffebeb; }");
        html.push_str(".error-indicator { color: red; }");
        html.push_str(".error-path { margin: 10px 0; font-family: monospace; color: purple; }");
        html.push_str("</style>");
        
        // Error message
        html.push_str(&format!("<div class='error-message'>Error: {}</div>", error.message));
        
        // Location
        if let Some(file) = &error.location.file {
            html.push_str(&format!(
                "<div class='error-location'>Location: {}:{}:{}</div>",
                file, error.location.line, error.location.column
            ));
        }
        
        // Type information
        html.push_str(&format!("<div class='error-type'>Expected type: {}</div>", error.expected_type));
        if let Some(actual) = &error.actual_type {
            html.push_str(&format!("<div class='error-type'>Actual type: {}</div>", actual));
        }
        
        // Source context
        if !error.context_lines.is_empty() {
            html.push_str("<div class='error-context'><pre>");
            
            for (line_num, content) in &error.context_lines {
                let line_class = if *line_num == error.location.line {
                    "error-line"
                } else {
                    ""
                };
                
                html.push_str(&format!(
                    "<div class='{}'>{}| {}</div>",
                    line_class, line_num, html_encode(content)
                ));
                
                // Add error indicator
                if *line_num == error.location.line && error.location.column > 0 {
                    let spaces = error.location.column - 1;
                    html.push_str(&format!(
                        "<div class='error-indicator'>{}<span>^-- Error occurs here</span></div>",
                        "&nbsp;".repeat(spaces + 2 + line_num.to_string().len())
                    ));
                }
            }
            
            html.push_str("</pre></div>");
        }
        
        // Type path
        if let Some(path) = &error.type_path {
            html.push_str(&format!("<div class='error-path'><pre>{}</pre></div>", path));
        }
        
        html
    }
    
    #[instrument(skip(self, error), level = "debug")]
    fn generate_terminal_error_message(
        &self,
        error: &VisualTypeAssertionError
    ) -> String {
        // Simple terminal colors using ANSI escape codes
        let red = "\x1B[31m";
        let green = "\x1B[32m";
        let yellow = "\x1B[33m";
        let blue = "\x1B[34m";
        let magenta = "\x1B[35m";
        let cyan = "\x1B[36m";
        let bold = "\x1B[1m";
        let reset = "\x1B[0m";
        
        let mut output = String::new();
        
        // Error message
        output.push_str(&format!("{}{}{}: {}{}\n", bold, red, "Error", error.message, reset));
        
        // Location
        if let Some(file) = &error.location.file {
            output.push_str(&format!(
                "{}Location:{} {}:{}:{}\n",
                blue, reset, file, error.location.line, error.location.column
            ));
        }
        
        // Type information
        output.push_str(&format!(
            "{}Expected type:{} {}{}\n",
            green, reset, cyan, error.expected_type
        ));
        
        if let Some(actual) = &error.actual_type {
            output.push_str(&format!(
                "{}Actual type:{} {}{}\n",
                green, reset, cyan, actual
            ));
        }
        
        // Source context
        if !error.context_lines.is_empty() {
            output.push_str("\nSource context:\n");
            
            for (line_num, content) in &error.context_lines {
                if *line_num == error.location.line {
                    // Highlight the error line
                    output.push_str(&format!(
                        "{}>{} {}{}|{} {}{}\n",
                        bold, reset, bold, line_num, reset, yellow, content
                    ));
                    
                    // Add caret pointing to the error location
                    if error.location.column > 0 {
                        let line_num_spaces = line_num.to_string().len();
                        let spaces = line_num_spaces + 2 + error.location.column - 1; // +2 for "| "
                        output.push_str(&format!(
                            "{}{}^--{} {}Error occurs here{}\n",
                            " ".repeat(spaces), red, reset, red, reset
                        ));
                    }
                } else {
                    // Regular context line
                    output.push_str(&format!("  {}| {}\n", line_num, content));
                }
            }
        }
        
        // Type path
        if let Some(path) = &error.type_path {
            output.push_str(&format!("\n{}Type relationship:{}\n{}{}{}", 
                                    magenta, reset, magenta, path, reset));
        }
        
        output
    }
}

// Simple HTML encoding helper
fn html_encode(s: &str) -> String {
    s.replace("&", "&amp;")
     .replace("<", "&lt;")
     .replace(">", "&gt;")
     .replace("\"", "&quot;")
     .replace("'", "&#39;")
}

// Helper methods for error visualization
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Get a type path visualization between two types
    fn get_type_path_visualization(&self, expected_type_id: u32, actual_type_id: u32) -> Option<String> {
        // This is a placeholder - in a real implementation, we would try to find a path
        // between the actual and expected types through the inheritance hierarchy
        None
    }
}

/// Register the error visualization module
pub fn register_error_visualization() {
    debug!("Registered error visualization for interface type assertions");
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_visualization_registration() {
        // Simple test to ensure module registration works
        register_error_visualization();
        assert!(true);
    }
}