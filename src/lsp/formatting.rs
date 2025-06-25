use crate::error::CursedError;
// Formatting provider for CURSED language server
// 
// Provides document formatting integration with the CURSED formatter

use std::collections::HashMap;
use tower_lsp::lsp_types::*;
use tracing::{debug, error, instrument, warn};

use crate::tools::formatter::{FormatterConfig, BraceStyle, CursedFormatter, OperatorSpacing, CommaSpacing};
// use crate::stdlib::string::{split_join, core as string_core, transform, search};

/// Formatting provider for the LSP server
pub struct FormattingProvider {
    /// Default formatting configuration
    default_config: FormatterConfig,
}

impl FormattingProvider {
    /// Create a new formatting provider
    pub fn new() -> Self {
        let default_config = FormatterConfig::default();
        Self {
            default_config,
        }
    }

    /// Format entire document
    #[instrument(skip(self, content))]
    pub async fn format_document(&self, content: &str) -> crate::error::Result<()> {
        debug!("Formatting document");
        
        // Create formatter with default config
        let mut formatter = CursedFormatter::new(self.default_config.clone());
        
        // Format the document
        match formatter.format(content) {
            Ok(result) => {
                if !result.formatting_errors.is_empty() {
                    warn!("Formatting completed with {} errors", result.formatting_errors.len());
                    for error in &result.formatting_errors {
                        warn!("Formatting error: {}", error);
                    }
                }
                debug!("Document formatted successfully, changes_made: {}", result.changes_made);
                Ok(result.formatted_code)
            }
            Err(e) => {
                error!("Failed to format document: {}", e);
                // Return original content on formatting error
                Ok(content.to_string())
            }
        }
    }

    /// Format document and return text edits
    #[instrument(skip(self, content))]
    pub async fn format_document_edits(
        &self,
        content: &str,
        options: FormattingOptions,
    ) -> Option<Vec<TextEdit>> {
        debug!("Getting document format edits");
        
        // Convert LSP formatting options to CURSED formatter config
        let config = self.lsp_options_to_config(options);
        
        // Create formatter with LSP-converted config
        let mut formatter = CursedFormatter::new(config);
        
        // Format the document
        match formatter.format(content) {
            Ok(result) => {
                if !result.changes_made {
                    debug!("No formatting changes needed");
                    return Some(vec![]);
                }
                
                if !result.formatting_errors.is_empty() {
                    warn!("Formatting completed with {} errors", result.formatting_errors.len());
                    for error in &result.formatting_errors {
                        warn!("Formatting error: {}", error);
                    }
                }
                
                // Calculate the range that covers the entire document
                let lines = content.split("\n").collect::<Vec<_>>();
                let end_line = lines.len().saturating_sub(1);
                let end_character = lines.get(end_line).map(|line| line.len()).unwrap_or(0);
                
                let full_range = Range {
                    start: Position { line: 0, character: 0 },
                    end: Position { 
                        line: end_line as u32, 
                        character: end_character as u32 
                    },
                };
                
                debug!("Document formatted successfully, {} lines changed", result.lines_changed);
                
                // Return a single edit that replaces the entire document
                Some(vec![TextEdit {
                    range: full_range,
                    new_text: result.formatted_code,
                }])
            }
            Err(e) => {
                error!("Failed to format document: {}", e);
                None
            }
        }
    }

    /// Format range of document
    #[instrument(skip(self, content))]
    pub async fn format_range(
        &self,
        content: &str,
        range: Range,
        options: FormattingOptions,
    ) -> Option<Vec<TextEdit>> {
        debug!("Formatting range {:?}", range);
        
        // Extract the range content
        let range_content = self.extract_range_content(content, range);
        if range_content.is_empty() {
            return Some(vec![]);
        }

        // Convert LSP formatting options to CURSED formatter config
        let config = self.lsp_options_to_config(options);
        
        // For range formatting, we need to format the entire document to maintain consistency
        // CURSED's formatter works at the AST level and needs full context
        let mut formatter = CursedFormatter::new(config);
        
        match formatter.format(content) {
            Ok(result) => {
                if !result.changes_made {
                    debug!("No formatting changes needed in range");
                    return Some(vec![]);
                }
                
                if !result.formatting_errors.is_empty() {
                    warn!("Range formatting completed with {} errors", result.formatting_errors.len());
                    for error in &result.formatting_errors {
                        warn!("Formatting error: {}", error);
                    }
                }
                
                // Extract the corresponding range from the formatted content
                let formatted_range_content = self.extract_range_content(&result.formatted_code, range);
                
                // Only apply the edit if the range content actually changed
                if formatted_range_content != range_content {
                    debug!("Range formatting applied changes");
                    Some(vec![TextEdit {
                        range,
                        new_text: formatted_range_content,
                    }])
                } else {
                    debug!("Range content unchanged after formatting");
                    Some(vec![])
                }
            }
            Err(e) => {
                error!("Failed to format range: {}", e);
                None
            }
        }
    }

    /// Format on type (triggered by specific characters)
    #[instrument(skip(self, content))]
    pub async fn format_on_type(
        &self,
        content: &str,
        position: Position,
        trigger_character: &str,
        options: FormattingOptions,
    ) -> Option<Vec<TextEdit>> {
        debug!("Formatting on type: '{}' at {:?}", trigger_character, position);
        
        match trigger_character {
            "{" => self.format_on_opening_brace(content, position, options).await,
            "}" => self.format_on_closing_brace(content, position, options).await,
            ";" => self.format_on_semicolon(content, position, options).await,
            _ => None,
        }
    }

    /// Format when opening brace is typed
    pub async fn format_on_opening_brace(
        &self,
        content: &str,
        position: Position,
        options: FormattingOptions,
    ) -> Option<Vec<TextEdit>> {
        // Format the current line and potentially add proper indentation
        let lines = split_join::split(content, "\n");
        let line_index = position.line as usize;
        
        if line_index >= lines.len() {
            return None;
        }

        let line = &lines[line_index];
        let config = self.lsp_options_to_config(options);
        
        // Check if we need to add indentation for the next line
        if search::ends_with(&transform::trim(line), "{") {
            let indentation = self.calculate_indentation(&config, line);
            let next_line_indent = indentation + config.indent_size;
            
            let insert_position = Position {
                line: position.line,
                character: line.len() as u32,
            };
            
            return Some(vec![TextEdit {
                range: Range {
                    start: insert_position,
                    end: insert_position,
                },
                new_text: format!("\n{}", string_core::repeat(" ", next_line_indent)),
            }]);
        }

        None
    }

    /// Format when closing brace is typed
    pub async fn format_on_closing_brace(
        &self,
        content: &str,
        position: Position,
        options: FormattingOptions,
    ) -> Option<Vec<TextEdit>> {
        // Adjust indentation of the current line with closing brace
        let lines = split_join::split(content, "\n");
        let line_index = position.line as usize;
        
        if line_index >= lines.len() {
            return None;
        }

        let line = &lines[line_index];
        let config = self.lsp_options_to_config(options);
        
        // Calculate proper indentation for closing brace
        let proper_indent = self.calculate_closing_brace_indentation(&lines, line_index, &config);
        let current_indent = line.len() - transform::trim_start(line).len();
        
        if proper_indent != current_indent {
            return Some(vec![TextEdit {
                range: Range {
                    start: Position { line: position.line, character: 0 },
                    end: Position { line: position.line, character: current_indent as u32 },
                },
                new_text: string_core::repeat(" ", proper_indent),
            }]);
        }

        None
    }

    /// Format when semicolon is typed
    pub async fn format_on_semicolon(
        &self,
        content: &str,
        position: Position,
        options: FormattingOptions,
    ) -> Option<Vec<TextEdit>> {
        // Format the current statement
        let lines = split_join::split(content, "\n");
        let line_index = position.line as usize;
        
        if line_index >= lines.len() {
            return None;
        }

        let line = &lines[line_index];
        let config = self.lsp_options_to_config(options);
        
        // Simple formatting: ensure proper spacing around operators
        let formatted_line = self.format_line_spacing(line, &config);
        
        if formatted_line != *line {
            return Some(vec![TextEdit {
                range: Range {
                    start: Position { line: position.line, character: 0 },
                    end: Position { line: position.line, character: line.len() as u32 },
                },
                new_text: formatted_line,
            }]);
        }

        None
    }

    /// Convert LSP formatting options to CURSED formatter config
    pub fn lsp_options_to_config(&self, options: FormattingOptions) -> FormatterConfig {
        let mut config = self.default_config.clone();
        
        // Map LSP options to CURSED formatter config
        config.indent_size = options.tab_size as usize;
        
        // Handle additional options from properties
        for (key, value) in &options.properties {
            match key.as_str() {
                "lineWidth" | "line_width" => {
                    if let Ok(width) = value.to_string().parse::<usize>() {
                        config.line_width = width;
                    }
                }
                "braceStyle" | "brace_style" => {
                    match value.to_string().to_lowercase().as_str() {
                        "sameline" | "same_line" => config.brace_style = BraceStyle::SameLine,
                        "nextline" | "next_line" => config.brace_style = BraceStyle::NextLine,
                        "nextlineunindented" | "next_line_unindented" => config.brace_style = BraceStyle::NextLineUnindented,
                        _ => {} // Keep default
                    }
                }
                "operatorSpacing" | "operator_spacing" => {
                    match value.to_string().to_lowercase().as_str() {
                        "withspaces" | "with_spaces" | "true" => config.operator_spacing = OperatorSpacing::WithSpaces,
                        "withoutspaces" | "without_spaces" | "false" => config.operator_spacing = OperatorSpacing::WithoutSpaces,
                        _ => {} // Keep default
                    }
                }
                "commaSpacing" | "comma_spacing" => {
                    match value.to_string().to_lowercase().as_str() {
                        "withspaces" | "with_spaces" | "true" => config.comma_spacing = CommaSpacing::WithSpaces,
                        "withoutspaces" | "without_spaces" | "false" => config.comma_spacing = CommaSpacing::WithoutSpaces,
                        _ => {} // Keep default
                    }
                }
                "maxEmptyLines" | "max_empty_lines" => {
                    if let Ok(max_lines) = value.to_string().parse::<usize>() {
                        config.max_empty_lines = max_lines;
                    }
                }
                _ => {} // Ignore unknown properties
            }
        }
        
        config
    }

    /// Extract content from a range
    pub fn extract_range_content(&self, content: &str, range: Range) -> String {
        let lines = split_join::split(content, "\n");
        let start_line = range.start.line as usize;
        let end_line = range.end.line as usize;
        
        if start_line >= lines.len() {
            return String::new();
        }
        
        if start_line == end_line {
            // Single line range
            if let Some(line) = lines.get(start_line) {
                let start_char = range.start.character as usize;
                let end_char = range.end.character as usize;
                if start_char < line.len() {
                    let end_char = end_char.min(line.len());
                    return line[start_char..end_char].to_string();
                }
            }
            return String::new();
        }
        
        // Multi-line range
        let mut result = String::new();
        
        for line_idx in start_line..=end_line.min(lines.len() - 1) {
            if let Some(line) = lines.get(line_idx) {
                if line_idx == start_line {
                    // First line: from start character to end
                    let start_char = range.start.character as usize;
                    if start_char < line.len() {
                        result.push_str(&line[start_char..]);
                    }
                } else if line_idx == end_line {
                    // Last line: from beginning to end character
                    let end_char = range.end.character as usize;
                    let end_char = end_char.min(line.len());
                    result.push_str(&line[..end_char]);
                } else {
                    // Middle lines: entire line
                    result.push_str(line);
                }
                
                // Add newline except for the last line
                if line_idx < end_line {
                    result.push('\n');
                }
            }
        }
        
        result
    }

    /// Calculate indentation level for a line
    pub fn calculate_indentation(&self, _config: &FormatterConfig, line: &str) -> usize {
        line.len() - transform::trim_start(line).len()
    }

    /// Calculate proper indentation for a closing brace
    pub fn calculate_closing_brace_indentation(
        &self,
        lines: &[String],
        current_line: usize,
        config: &FormatterConfig,
    ) -> usize {
        // Find the matching opening brace
        let mut brace_count = 0;
        
        for line_idx in (0..current_line).rev() {
            if let Some(line) = lines.get(line_idx) {
                for ch in line.chars() {
                    match ch {
                        '}' => brace_count += 1,
                        '{' => {
                            if brace_count == 0 {
                                // Found matching opening brace
                                return self.calculate_indentation(config, line);
                            }
                            brace_count -= 1;
                        }
                        _ => {}
                    }
                }
            }
        }
        
        // Default indentation if no matching brace found
        0
    }

    /// Format spacing in a line
    pub fn format_line_spacing(&self, line: &str, _config: &FormatterConfig) -> String {
        let mut result = String::new();
        let chars: Vec<char> = line.chars().collect();
        let mut i = 0;
        
        while i < chars.len() {
            let ch = chars[i];
            
            match ch {
                '=' | '+' | '-' | '*' | '/' | '%' | '&' | '|' | '^' => {
                    // Add space before operator if not present
                    if !result.ends_with(' ') && !result.is_empty() {
                        result.push(' ');
                    }
                    result.push(ch);
                    
                    // Add space after operator if not present
                    if i + 1 < chars.len() && chars[i + 1] != ' ' && chars[i + 1] != '=' {
                        result.push(' ');
                    }
                }
                ',' => {
                    result.push(ch);
                    // Add space after comma if not present
                    if i + 1 < chars.len() && chars[i + 1] != ' ' {
                        result.push(' ');
                    }
                }
                _ => {
                    result.push(ch);
                }
            }
            
            i += 1;
        }
        
        result
    }

    /// Check if formatting is available
    pub async fn is_formatter_available(&self) -> bool {
        // The CURSED formatter is built-in, so it's always available
        true
    }

    /// Get formatter version
    pub async fn get_formatter_version(&self) -> Option<String> {
        Some("0.1.0".to_string())
    }
}

impl Default for FormattingProvider {
    fn default() -> Self {
        Self::new()
    }
}

