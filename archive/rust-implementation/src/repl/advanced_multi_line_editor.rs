//! Advanced multi-line editor for CURSED REPL
//! Supports complex expressions, function definitions, and block statements

use crate::error::CursedError;
use std::collections::VecDeque;

/// Multi-line editor state
#[derive(Debug, Clone)]
pub enum EditorState {
    SingleLine,
    MultiLineFunction,
    MultiLineStruct,
    MultiLineIf,
    MultiLineLoop,
    MultiLineBlock,
}

/// Context for tracking multi-line input
#[derive(Debug)]
pub struct MultiLineContext {
    /// Current lines being edited
    lines: VecDeque<String>,
    /// Current editor state
    state: EditorState,
    /// Brace/bracket depth tracking
    brace_depth: i32,
    bracket_depth: i32,
    paren_depth: i32,
    /// Whether we're inside a string literal
    in_string: bool,
    /// String delimiter (single or double quote)
    string_delimiter: char,
    /// Current indentation level
    indent_level: usize,
    /// Whether the current statement is complete
    is_complete: bool,
}

impl MultiLineContext {
    pub fn new() -> Self {
        Self {
            lines: VecDeque::new(),
            state: EditorState::SingleLine,
            brace_depth: 0,
            bracket_depth: 0,
            paren_depth: 0,
            in_string: false,
            string_delimiter: '"',
            indent_level: 0,
            is_complete: true,
        }
    }
    
    /// Add a line to the current multi-line input
    pub fn add_line(&mut self, line: String) -> Result<bool, CursedError> {
        let trimmed = line.trim();
        
        // Handle empty lines
        if trimmed.is_empty() && self.state == EditorState::SingleLine {
            return Ok(true); // Complete on empty line in single-line mode
        }
        
        // Analyze the line to determine state changes
        self.analyze_line(&line)?;
        
        // Add the line with proper indentation
        let indented_line = self.apply_indentation(line);
        self.lines.push_back(indented_line);
        
        // Check if the input is complete
        self.is_complete = self.check_completion();
        
        Ok(self.is_complete)
    }
    
    /// Analyze a line to update the multi-line state
    fn analyze_line(&mut self, line: &str) -> Result<(), CursedError> {
        let trimmed = line.trim();
        
        // Update state based on keywords
        if self.state == EditorState::SingleLine {
            if trimmed.starts_with("slay ") && trimmed.contains('{') && !trimmed.ends_with('}') {
                self.state = EditorState::MultiLineFunction;
            } else if trimmed.starts_with("squad ") && trimmed.contains('{') && !trimmed.ends_with('}') {
                self.state = EditorState::MultiLineStruct;
            } else if trimmed.starts_with("ready ") && trimmed.contains('{') && !trimmed.ends_with('}') {
                self.state = EditorState::MultiLineIf;
            } else if trimmed.starts_with("bestie ") && trimmed.contains('{') && !trimmed.ends_with('}') {
                self.state = EditorState::MultiLineLoop;
            } else if trimmed.ends_with('{') && !self.is_single_line_block(trimmed) {
                self.state = EditorState::MultiLineBlock;
            }
        }
        
        // Track bracket/brace depth
        self.update_depth_tracking(line)?;
        
        Ok(())
    }
    
    /// Update depth tracking for brackets, braces, and parentheses
    fn update_depth_tracking(&mut self, line: &str) -> Result<(), CursedError> {
        let mut chars = line.chars().peekable();
        
        while let Some(ch) = chars.next() {
            match ch {
                '"' | '\'' => {
                    if !self.in_string {
                        self.in_string = true;
                        self.string_delimiter = ch;
                    } else if ch == self.string_delimiter {
                        // Check for escaped quotes
                        let mut escape_count = 0;
                        let mut pos = line.len();
                        for (i, prev_ch) in line.chars().rev().enumerate() {
                            if prev_ch == '\\' {
                                escape_count += 1;
                            } else {
                                pos = line.len() - i;
                                break;
                            }
                        }
                        
                        if escape_count % 2 == 0 {
                            self.in_string = false;
                        }
                    }
                }
                '{' if !self.in_string => {
                    self.brace_depth += 1;
                    self.indent_level += 1;
                }
                '}' if !self.in_string => {
                    self.brace_depth -= 1;
                    if self.indent_level > 0 {
                        self.indent_level -= 1;
                    }
                }
                '[' if !self.in_string => {
                    self.bracket_depth += 1;
                }
                ']' if !self.in_string => {
                    self.bracket_depth -= 1;
                }
                '(' if !self.in_string => {
                    self.paren_depth += 1;
                }
                ')' if !self.in_string => {
                    self.paren_depth -= 1;
                }
                _ => {}
            }
        }
        
        // Validate depth tracking
        if self.brace_depth < 0 || self.bracket_depth < 0 || self.paren_depth < 0 {
            return Err(CursedError::syntax_error("Unmatched closing bracket/brace/parenthesis"));
        }
        
        Ok(())
    }
    
    /// Apply proper indentation to a line
    fn apply_indentation(&self, line: String) -> String {
        let trimmed = line.trim();
        
        if trimmed.is_empty() {
            return line;
        }
        
        // Calculate indentation level
        let mut indent = self.indent_level;
        
        // Reduce indentation for closing braces
        if trimmed.starts_with('}') || trimmed.starts_with("otherwise") {
            indent = indent.saturating_sub(1);
        }
        
        // Apply indentation (2 spaces per level)
        let spaces = "  ".repeat(indent);
        format!("{}{}", spaces, trimmed)
    }
    
    /// Check if the current input is complete
    fn check_completion(&self) -> bool {
        // Input is complete if all brackets/braces/parentheses are balanced
        // and we're not in a string, and not in an incomplete statement
        
        if self.in_string {
            return false;
        }
        
        if self.brace_depth > 0 || self.bracket_depth > 0 || self.paren_depth > 0 {
            return false;
        }
        
        // Check for incomplete statements
        if !self.lines.is_empty() {
            let last_line = self.lines.back().unwrap().trim();
            
            // Incomplete if line ends with continuation indicators
            if last_line.ends_with('\\') || 
               last_line.ends_with(',') ||
               last_line.ends_with("&&") ||
               last_line.ends_with("||") {
                return false;
            }
            
            // Check for incomplete function signatures
            if last_line.contains("slay ") && last_line.contains('(') && !last_line.contains(')') {
                return false;
            }
        }
        
        // Reset to single-line mode if complete
        true
    }
    
    /// Check if a line contains a complete single-line block
    fn is_single_line_block(&self, line: &str) -> bool {
        let open_braces = line.chars().filter(|&c| c == '{').count();
        let close_braces = line.chars().filter(|&c| c == '}').count();
        open_braces == close_braces && open_braces > 0
    }
    
    /// Get the current multi-line input as a single string
    pub fn get_input(&self) -> String {
        self.lines.iter().cloned().collect::<Vec<_>>().join("\n")
    }
    
    /// Get the current prompt for the multi-line editor
    pub fn get_prompt(&self) -> String {
        match self.state {
            EditorState::SingleLine => "cursed> ".to_string(),
            EditorState::MultiLineFunction => "   slay... ".to_string(),
            EditorState::MultiLineStruct => "  squad... ".to_string(),
            EditorState::MultiLineIf => "  ready... ".to_string(),
            EditorState::MultiLineLoop => " bestie... ".to_string(),
            EditorState::MultiLineBlock => "    ... ".to_string(),
        }
    }
    
    /// Check if currently in multi-line mode
    pub fn is_multi_line(&self) -> bool {
        !matches!(self.state, EditorState::SingleLine) || !self.is_complete
    }
    
    /// Reset the multi-line context
    pub fn reset(&mut self) {
        self.lines.clear();
        self.state = EditorState::SingleLine;
        self.brace_depth = 0;
        self.bracket_depth = 0;
        self.paren_depth = 0;
        self.in_string = false;
        self.indent_level = 0;
        self.is_complete = true;
    }
    
    /// Get syntax highlighting hints for the current input
    pub fn get_syntax_hints(&self) -> Vec<(usize, usize, String)> {
        let mut hints = Vec::new();
        let input = self.get_input();
        
        // This would integrate with a syntax highlighter
        // For now, just return empty hints
        // TODO: Implement full syntax highlighting
        
        hints
    }
    
    /// Get auto-completion suggestions for the current context
    pub fn get_completion_context(&self) -> String {
        match self.state {
            EditorState::MultiLineFunction => "function_body".to_string(),
            EditorState::MultiLineStruct => "struct_body".to_string(),
            EditorState::MultiLineIf => "if_body".to_string(),
            EditorState::MultiLineLoop => "loop_body".to_string(),
            EditorState::MultiLineBlock => "block_body".to_string(),
            EditorState::SingleLine => "global".to_string(),
        }
    }
    
    /// Handle special key inputs (like Tab for auto-completion)
    pub fn handle_special_key(&mut self, key: &str) -> Result<Option<String>, CursedError> {
        match key {
            "Tab" => {
                // Auto-indent or complete
                if let Some(last_line) = self.lines.back() {
                    if last_line.trim().is_empty() {
                        // Auto-indent
                        let indent = "  ".repeat(self.indent_level);
                        return Ok(Some(indent));
                    }
                }
                Ok(None)
            }
            "Ctrl+C" => {
                // Cancel multi-line input
                self.reset();
                Ok(Some("^C".to_string()))
            }
            "Ctrl+D" => {
                // Complete input if in multi-line mode
                if self.is_multi_line() {
                    self.is_complete = true;
                    Ok(Some(self.get_input()))
                } else {
                    Ok(None)
                }
            }
            _ => Ok(None)
        }
    }
    
    /// Get current line count
    pub fn line_count(&self) -> usize {
        self.lines.len()
    }
    
    /// Get current state description
    pub fn get_state_description(&self) -> String {
        match self.state {
            EditorState::SingleLine => "Single line input".to_string(),
            EditorState::MultiLineFunction => "Multi-line function definition".to_string(),
            EditorState::MultiLineStruct => "Multi-line struct definition".to_string(),
            EditorState::MultiLineIf => "Multi-line if statement".to_string(),
            EditorState::MultiLineLoop => "Multi-line loop statement".to_string(),
            EditorState::MultiLineBlock => "Multi-line block statement".to_string(),
        }
    }
}

/// Advanced multi-line editor
pub struct AdvancedMultiLineEditor {
    context: MultiLineContext,
    history: Vec<String>,
    max_history: usize,
}

impl AdvancedMultiLineEditor {
    pub fn new() -> Self {
        Self {
            context: MultiLineContext::new(),
            history: Vec::new(),
            max_history: 1000,
        }
    }
    
    /// Process a line of input
    pub fn process_line(&mut self, line: String) -> Result<Option<String>, CursedError> {
        let is_complete = self.context.add_line(line)?;
        
        if is_complete {
            let input = self.context.get_input();
            
            // Add to history if non-empty
            if !input.trim().is_empty() {
                self.add_to_history(input.clone());
            }
            
            // Reset context for next input
            self.context.reset();
            
            Ok(Some(input))
        } else {
            Ok(None)
        }
    }
    
    /// Get the current prompt
    pub fn get_prompt(&self) -> String {
        self.context.get_prompt()
    }
    
    /// Check if in multi-line mode
    pub fn is_multi_line(&self) -> bool {
        self.context.is_multi_line()
    }
    
    /// Add entry to history
    fn add_to_history(&mut self, entry: String) {
        self.history.push(entry);
        
        // Limit history size
        if self.history.len() > self.max_history {
            self.history.remove(0);
        }
    }
    
    /// Get history
    pub fn get_history(&self) -> &[String] {
        &self.history
    }
    
    /// Reset the editor
    pub fn reset(&mut self) {
        self.context.reset();
    }
    
    /// Handle special keys
    pub fn handle_special_key(&mut self, key: &str) -> Result<Option<String>, CursedError> {
        self.context.handle_special_key(key)
    }
    
    /// Get current state description
    pub fn get_state_description(&self) -> String {
        self.context.get_state_description()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_line_completion() {
        let mut editor = AdvancedMultiLineEditor::new();
        let result = editor.process_line("sus x drip = 42".to_string()).unwrap();
        assert!(result.is_some());
        assert_eq!(result.unwrap(), "sus x drip = 42");
    }

    #[test]
    fn test_multi_line_function() {
        let mut editor = AdvancedMultiLineEditor::new();
        
        // Start function definition
        let result = editor.process_line("slay add(a drip, b drip) drip {".to_string()).unwrap();
        assert!(result.is_none());
        assert!(editor.is_multi_line());
        
        // Add function body
        let result = editor.process_line("    damn a + b".to_string()).unwrap();
        assert!(result.is_none());
        
        // Close function
        let result = editor.process_line("}".to_string()).unwrap();
        assert!(result.is_some());
        assert!(!editor.is_multi_line());
    }

    #[test]
    fn test_bracket_balancing() {
        let mut context = MultiLineContext::new();
        context.add_line("sus arr []drip = [1, 2, 3]".to_string()).unwrap();
        assert!(context.is_complete);
        
        context.add_line("sus nested [][]drip = [[1, 2], [3".to_string()).unwrap();
        assert!(!context.is_complete);
        
        context.add_line(", 4]]".to_string()).unwrap();
        assert!(context.is_complete);
    }

    #[test]
    fn test_string_handling() {
        let mut context = MultiLineContext::new();
        
        // Complete string
        context.add_line("sus msg tea = \"Hello, world!\"".to_string()).unwrap();
        assert!(context.is_complete);
        
        // Incomplete string
        context.reset();
        context.add_line("sus msg tea = \"Hello,".to_string()).unwrap();
        assert!(!context.is_complete);
        
        context.add_line("world!\"".to_string()).unwrap();
        assert!(context.is_complete);
    }
}
