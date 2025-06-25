use crate::error::CursedError;
// Multi-line Input Editor for CURSED REPL
// 
// Handles multi-line input with automatic indentation detection,
// bracket matching, and intelligent continuation detection.

use std::io::{self, Write};

use crate::repl::ReplResult;

/// Multi-line input editor for CURSED REPL
pub struct MultiLineEditor {
    indent_size: usize,
    auto_indent: bool,
    bracket_pairs: Vec<(char, char)>,
}

impl MultiLineEditor {
    /// Create a new multi-line editor
    pub fn new() -> Self {
        Self {
            indent_size: 4,
            auto_indent: true,
            bracket_pairs: vec![
                ('(', ')'),
                ('[', ']'),
                ('{', '}'),
            ],
        }
    }

    /// Create multi-line editor with custom settings
    pub fn with_settings(indent_size: usize, auto_indent: bool) -> Self {
        Self {
            indent_size,
            auto_indent,
            bracket_pairs: vec![
                ('(', ')'),
                ('[', ']'),
                ('{', '}'),
            ],
        }
    }

    /// Check if the current input needs continuation
    pub fn needs_continuation(&self, input: &str) -> bool {
        let trimmed = input.trim();
        
        // Empty input doesn't need continuation
        if trimmed.is_empty() {
            return false;
        }
        
        // Check for unmatched brackets
        if self.has_unmatched_brackets(input) {
            return true;
        }
        
        // Check for incomplete structures
        if self.has_incomplete_structure(input) {
            return true;
        }
        
        // Check for line continuation indicators
        if self.has_continuation_indicator(input) {
            return true;
        }
        
        false
    }

    /// Check if input is complete (no more lines needed)
    pub fn is_complete(&self, current_input: &str, new_line: &str) -> bool {
        let combined = format!("{}\n{}", current_input, new_line);
        
        // If the new line is empty and brackets are balanced, consider complete
        if new_line.trim().is_empty() && !self.has_unmatched_brackets(&combined) {
            return true;
        }
        
        // Check for explicit completion indicators
        if new_line.trim() == ";" || new_line.trim() == "" {
            return !self.has_unmatched_brackets(&combined);
        }
        
        false
    }

    /// Read a line with the given prompt
    pub fn read_line(&self, prompt: &str) -> ReplResult<String> {
        print!("{}", prompt);
        io::stdout().flush().map_err(|e| CursedError::repl_error(e.to_string()))?;
        
        let mut input = String::new();
        io::stdin().read_line(&mut input)
            .map_err(|e| CursedError::repl_error(e.to_string()))?;
        
        // Remove trailing newline
        if input.ends_with('\n') {
            input.pop();
            if input.ends_with('\r') {
                input.pop();
            }
        }
        
        Ok(input)
    }

    /// Calculate appropriate indentation for the next line
    pub fn calculate_indentation(&self, current_input: &str) -> usize {
        if !self.auto_indent {
            return 0;
        }
        
        let lines: Vec<&str> = current_input.split("\n").collect();
        if lines.is_empty() {
            return 0;
        }
        
        let last_line = lines.last().unwrap();
        let current_indent = self.get_line_indentation(last_line);
        
        // Increase indentation after opening brackets or keywords
        if self.should_increase_indentation(last_line) {
            return current_indent + self.indent_size;
        }
        
        // Decrease indentation for closing brackets
        if self.should_decrease_indentation(last_line) {
            return current_indent.saturating_sub(self.indent_size);
        }
        
        current_indent
    }

    /// Generate indented prompt for continuation
    pub fn get_continuation_prompt(&self, base_prompt: &str, current_input: &str) -> String {
        let indent = self.calculate_indentation(current_input);
        let spaces = " ".repeat(indent);
        format!("{}{}", base_prompt, spaces)
    }

    /// Check for unmatched brackets in the input
    fn has_unmatched_brackets(&self, input: &str) -> bool {
        let mut stack = Vec::new();
        let mut in_string = false;
        let mut string_char = '"';
        let mut in_comment = false;
        
        let mut chars = input.chars().peekable();
        
        while let Some(ch) = chars.next() {
            match ch {
                // Handle string literals
                '"' | '\'' if !in_comment => {
                    if in_string && ch == string_char {
                        in_string = false;
                    } else if !in_string {
                        in_string = true;
                        string_char = ch;
                    }
                }
                
                // Handle comments
                '/' if !in_string && !in_comment => {
                    if let Some(&'/') = chars.peek() {
                        in_comment = true;
                        chars.next(); // consume second '/'
                    }
                }
                
                '\n' if in_comment => {
                    in_comment = false;
                }
                
                // Handle brackets (only when not in string or comment)
                ch if !in_string && !in_comment => {
                    // Check for opening brackets
                    for &(open, close) in &self.bracket_pairs {
                        if ch == open {
                            stack.push(close);
                            break;
                        } else if ch == close {
                            if stack.last() == Some(&close) {
                                stack.pop();
                            } else {
                                // Mismatched closing bracket
                                return true;
                            }
                            break;
                        }
                    }
                }
                
                _ => {}
            }
        }
        
        // If stack is not empty, we have unmatched opening brackets
        !stack.is_empty()
    }

    /// Check for incomplete language structures
    fn has_incomplete_structure(&self, input: &str) -> bool {
        let trimmed = input.trim();
        
        // Function declarations without body
        if trimmed.starts_with("slay ") && !trimmed.contains('{') {
            return true;
        }
        
        // Control structures without body
        if (trimmed.starts_with("lowkey ") || 
            trimmed.starts_with("highkey ") ||
            trimmed.starts_with("bestie ") ||
            trimmed.starts_with("for ") ||
            trimmed.starts_with("while ")) && !trimmed.contains('{') {
            return true;
        }
        
        // Struct or interface declarations
        if (trimmed.starts_with("squad ") || 
            trimmed.starts_with("collab ")) && !trimmed.contains('{') {
            return true;
        }
        
        false
    }

    /// Check for explicit continuation indicators
    fn has_continuation_indicator(&self, input: &str) -> bool {
        let trimmed = input.trim();
        
        // Lines ending with operators that suggest continuation
        trimmed.ends_with('+') ||
        trimmed.ends_with('-') ||
        trimmed.ends_with('*') ||
        trimmed.ends_with('/') ||
        trimmed.ends_with('=') ||
        trimmed.ends_with("&&") ||
        trimmed.ends_with("||") ||
        trimmed.ends_with(',')
    }

    /// Get the indentation level of a line
    fn get_line_indentation(&self, line: &str) -> usize {
        let mut indent = 0;
        for ch in line.chars() {
            if ch == ' ' {
                indent += 1;
            } else if ch == '\t' {
                indent += self.indent_size; // Assume tab = indent_size spaces
            } else {
                break;
            }
        }
        indent
    }

    /// Check if indentation should be increased after this line
    fn should_increase_indentation(&self, line: &str) -> bool {
        let trimmed = line.trim();
        
        // After opening brackets
        if trimmed.ends_with('{') || trimmed.ends_with('(') || trimmed.ends_with('[') {
            return true;
        }
        
        // After control structure keywords
        if trimmed.starts_with("lowkey ") ||
           trimmed.starts_with("highkey ") ||
           trimmed.starts_with("bestie ") ||
           trimmed.starts_with("flex ") ||
           trimmed.starts_with("for ") ||
           trimmed.starts_with("while ") {
            return true;
        }
        
        false
    }

    /// Check if indentation should be decreased for this line
    fn should_decrease_indentation(&self, line: &str) -> bool {
        let trimmed = line.trim();
        
        // Closing brackets
        trimmed.starts_with('}') || 
        trimmed.starts_with(')') || 
        trimmed.starts_with(']')
    }
}

impl Default for MultiLineEditor {
    fn default() -> Self {
        Self::new()
    }
}

