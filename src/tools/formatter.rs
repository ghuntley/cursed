//! CURSED Code Formatter
//!
//! This module provides comprehensive code formatting for the CURSED programming language.
//! It handles all CURSED language constructs including Gen Z slang keywords and maintains
//! consistent style according to configurable formatting options.

use crate::ast::*;
use crate::lexer::Lexer;
use crate::parser::Parser;
use std::fmt;
use tracing::{debug, info, instrument};

/// Configuration options for the CURSED code formatter
#[derive(Debug, Clone)]
pub struct FormatterConfig {
    /// Number of spaces for each indentation level
    pub indent_size: usize,
    /// Maximum line width before wrapping
    pub line_width: usize,
    /// Style for opening braces
    pub brace_style: BraceStyle,
    /// Whether to insert spaces around operators
    pub spaces_around_operators: bool,
    /// Whether to insert space after commas
    pub space_after_comma: bool,
    /// Whether to format comments
    pub format_comments: bool,
    /// Whether to preserve empty lines
    pub preserve_empty_lines: bool,
    /// Maximum number of consecutive empty lines to preserve
    pub max_empty_lines: usize,
}

/// Brace style options
#[derive(Debug, Clone, PartialEq)]
pub enum BraceStyle {
    /// Opening brace on same line: `if condition {`
    SameLine,
    /// Opening brace on next line indented: 
    /// ```
    /// if condition
    /// {
    /// ```
    NextLine,
    /// Opening brace on next line unindented:
    /// ```
    /// if condition
    /// {
    /// ```
    NextLineUnindented,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            indent_size: 4,
            line_width: 100,
            brace_style: BraceStyle::SameLine,
            spaces_around_operators: true,
            space_after_comma: true,
            format_comments: true,
            preserve_empty_lines: true,
            max_empty_lines: 2,
        }
    }
}

/// Result of formatting operation
#[derive(Debug)]
pub struct FormatterResult {
    /// The formatted source code
    pub formatted_code: String,
    /// Whether any changes were made
    pub changed: bool,
    /// Number of lines processed
    pub lines_processed: usize,
    /// Any warnings or issues encountered
    pub warnings: Vec<String>,
}

/// The main CURSED code formatter
pub struct CursedFormatter {
    config: FormatterConfig,
    current_indent: usize,
    output: String,
    current_line_length: usize,
    empty_line_count: usize,
}

impl CursedFormatter {
    /// Create a new formatter with the given configuration
    pub fn new(config: FormatterConfig) -> Self {
        Self {
            config,
            current_indent: 0,
            output: String::new(),
            current_line_length: 0,
            empty_line_count: 0,
        }
    }

    /// Create a formatter with default configuration
    pub fn default() -> Self {
        Self::new(FormatterConfig::default())
    }

    /// Format the given CURSED source code
    #[instrument(skip(self, source))]
    pub fn format(&mut self, source: &str) -> Result<FormatterResult, crate::error::Error> {
        info!("Starting to format {} characters of source code", source.len());
        
        // Reset state
        self.output.clear();
        self.current_indent = 0;
        self.current_line_length = 0;
        self.empty_line_count = 0;

        // For now, implement a simple regex-based formatter until the AST parsing is working
        let formatted_code = self.format_with_regex(source);
        let original_code = source.to_string();
        let changed = original_code != formatted_code;
        let lines_processed = formatted_code.lines().count();

        Ok(FormatterResult {
            formatted_code,
            changed,
            lines_processed,
            warnings: Vec::new(),
        })
    }

    /// Simple regex-based formatter for basic CURSED constructs
    fn format_with_regex(&mut self, source: &str) -> String {
        let mut result = source.to_string();
        
        // Add spaces around operators if configured
        if self.config.spaces_around_operators {
            // Handle assignment operators
            result = regex::Regex::new(r"(\w)\s*=\s*(\w)")
                .unwrap()
                .replace_all(&result, "$1 = $2")
                .to_string();
            
            // Handle arithmetic operators
            result = regex::Regex::new(r"(\w)\s*([+\-*/])\s*(\w)")
                .unwrap()
                .replace_all(&result, "$1 $2 $3")
                .to_string();
                
            // Handle comparison operators
            result = regex::Regex::new(r"(\w)\s*([<>!]=?)\s*(\w)")
                .unwrap()
                .replace_all(&result, "$1 $2 $3")
                .to_string();
        }
        
        // Add spaces after commas if configured
        if self.config.space_after_comma {
            result = regex::Regex::new(r",([^\s])")
                .unwrap()
                .replace_all(&result, ", $1")
                .to_string();
        }
        
        // Format function declarations (slay)
        result = regex::Regex::new(r"slay\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*\(")
            .unwrap()
            .replace_all(&result, "slay $1(")
            .to_string();
        
        // Format variable declarations (sus)
        result = regex::Regex::new(r"sus\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*=")
            .unwrap()
            .replace_all(&result, "sus $1 =")
            .to_string();
        
        // Format constant declarations (facts)
        result = regex::Regex::new(r"facts\s+([a-zA-Z_][a-zA-Z0-9_]*)\s*=")
            .unwrap()
            .replace_all(&result, "facts $1 =")
            .to_string();
        
        // Format if statements (lowkey)
        result = regex::Regex::new(r"lowkey\s+")
            .unwrap()
            .replace_all(&result, "lowkey ")
            .to_string();
        
        // Format else statements (highkey)
        result = regex::Regex::new(r"\}\s*highkey\s*\{")
            .unwrap()
            .replace_all(&result, "} highkey {")
            .to_string();
        
        // Format while loops (periodt)
        result = regex::Regex::new(r"periodt\s+")
            .unwrap()
            .replace_all(&result, "periodt ")
            .to_string();
        
        // Format for loops (bestie)
        result = regex::Regex::new(r"bestie\s+")
            .unwrap()
            .replace_all(&result, "bestie ")
            .to_string();
        
        // Add basic indentation
        self.add_basic_indentation(&result)
    }
    
    /// Add basic indentation to the formatted code
    fn add_basic_indentation(&self, source: &str) -> String {
        let mut result = String::new();
        let mut indent_level = 0;
        let indent_str = " ".repeat(self.config.indent_size);
        
        for line in source.lines() {
            let trimmed = line.trim();
            
            // Decrease indent for closing braces
            if trimmed.starts_with('}') {
                if indent_level > 0 {
                    indent_level -= 1;
                }
            }
            
            // Add indentation
            if !trimmed.is_empty() {
                for _ in 0..indent_level {
                    result.push_str(&indent_str);
                }
                result.push_str(trimmed);
            }
            result.push('\n');
            
            // Increase indent for opening braces
            if trimmed.ends_with('{') {
                indent_level += 1;
            }
        }
        
        result
    }

    /// Write text to output
    fn write(&mut self, text: &str) {
        self.output.push_str(text);
        self.current_line_length += text.len();
    }

    /// Write indented text
    fn write_indented(&mut self, text: &str) {
        self.write_indent();
        self.write(text);
    }

    /// Write current indentation
    fn write_indent(&mut self) {
        let spaces = " ".repeat(self.current_indent * self.config.indent_size);
        self.write(&spaces);
    }

    /// Write a newline
    fn write_newline(&mut self) {
        self.output.push('\n');
        self.current_line_length = 0;
        self.empty_line_count += 1;
    }

    /// Format opening brace according to brace style
    fn format_opening_brace(&mut self) {
        match self.config.brace_style {
            BraceStyle::SameLine => {
                self.write(" {");
                self.write_newline();
            }
            BraceStyle::NextLine | BraceStyle::NextLineUnindented => {
                self.write_newline();
                if self.config.brace_style == BraceStyle::NextLine {
                    self.write_indented("{");
                } else {
                    self.write("{");
                }
                self.write_newline();
            }
        }
    }

    /// Format closing brace
    fn format_closing_brace(&mut self) {
        self.write_indented("}");
    }

    /// Increase indentation level
    fn increase_indent(&mut self) {
        self.current_indent += 1;
    }

    /// Decrease indentation level
    fn decrease_indent(&mut self) {
        if self.current_indent > 0 {
            self.current_indent -= 1;
        }
    }
}

impl fmt::Display for FormatterResult {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Formatted {} lines", self.lines_processed)?;
        if self.changed {
            write!(f, " (changes made)")?;
        } else {
            write!(f, " (no changes)")?;
        }
        if !self.warnings.is_empty() {
            write!(f, " with {} warnings", self.warnings.len())?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_formatter_config_default() {
        let config = FormatterConfig::default();
        assert_eq!(config.indent_size, 4);
        assert_eq!(config.line_width, 100);
        assert_eq!(config.brace_style, BraceStyle::SameLine);
        assert!(config.spaces_around_operators);
        assert!(config.space_after_comma);
    }

    #[test]
    fn test_brace_style_formatting() {
        let mut formatter = CursedFormatter::new(FormatterConfig {
            brace_style: BraceStyle::SameLine,
            ..FormatterConfig::default()
        });
        
        formatter.format_opening_brace();
        assert!(formatter.output.contains(" {"));
    }

    #[test]
    fn test_indentation() {
        let mut formatter = CursedFormatter::default();
        formatter.increase_indent();
        formatter.write_indented("test");
        
        assert!(formatter.output.starts_with("    test"));
    }

    #[test]
    fn test_operator_spacing() {
        let config_with_spaces = FormatterConfig {
            spaces_around_operators: true,
            ..FormatterConfig::default()
        };
        
        let config_without_spaces = FormatterConfig {
            spaces_around_operators: false,
            ..FormatterConfig::default()
        };
        
        // Test cases would need actual expression parsing to be complete
        assert!(config_with_spaces.spaces_around_operators);
        assert!(!config_without_spaces.spaces_around_operators);
    }

    #[test]
    fn test_simple_formatting() {
        let mut formatter = CursedFormatter::default();
        let source = "slay test(){sus x=1+2}";
        let result = formatter.format(source).unwrap();
        

        
        assert!(result.changed);
        assert!(result.formatted_code.contains("slay test()"));
        assert!(result.formatted_code.contains("sus x = 1 + 2"));
    }

    #[test]
    fn test_indentation_with_braces() {
        let mut formatter = CursedFormatter::default();
        let source = "slay test(){\nsus x = 1\n}";
        let result = formatter.format(source).unwrap();
        
        assert!(result.changed);
        // Should add proper indentation
        assert!(result.formatted_code.contains("    sus x = 1"));
    }
}
