//! Simplified CURSED formatter for immediate use

use crate::error::CursedError;
use std::fs;
use std::path::Path;

use super::config::FormatterConfig;
use super::output::{FormatCheckResult, DiffFormatter, FormattingStats};

/// Simplified formatter that works with string manipulation
#[derive(Debug, Clone)]
pub struct SimpleCursedFormatter {
    pub config: FormatterConfig,
}

impl Default for SimpleCursedFormatter {
    fn default() -> Self {
        Self {
            config: FormatterConfig::default(),
        }
    }
}

impl SimpleCursedFormatter {
    /// Create a new formatter with custom configuration
    pub fn new(config: FormatterConfig) -> Self {
        Self { config }
    }

    /// Load configuration from file
    pub fn with_config_file<P: AsRef<Path>>(path: P) -> Result<Self, CursedError> {
        let content = fs::read_to_string(path)?;
        let config: FormatterConfig = toml::from_str(&content)
            .map_err(|e| CursedError::ConfigError(format!("Failed to parse config: {}", e)))?;
        Ok(Self::new(config))
    }

    /// Format source code using string manipulation
    pub fn format(&self, source: &str) -> Result<String, CursedError> {
        let mut lines: Vec<String> = source.lines().map(|l| l.to_string()).collect();
        
        // Apply formatting rules
        self.format_indentation(&mut lines);
        self.format_spacing(&mut lines);
        self.format_imports(&mut lines);
        self.format_blank_lines(&mut lines);
        self.trim_trailing_whitespace(&mut lines);
        
        let mut result = lines.join("\n");
        
        // Add final newline if configured
        if self.config.insert_final_newline && !result.ends_with('\n') {
            result.push('\n');
        }
        
        Ok(result)
    }

    /// Format indentation
    fn format_indentation(&self, lines: &mut [String]) {
        let mut indent_level = 0;
        let mut in_switch = false;
        
        for line in lines {
            let trimmed = line.trim().to_string();
            
            if trimmed.is_empty() {
                *line = String::new();
                continue;
            }
            
            // Handle switch statements
            if trimmed.starts_with("periodt ") {
                in_switch = true;
            }
            
            // Decrease indent for closing braces/parentheses
            if trimmed.starts_with('}') || trimmed.starts_with(')') || trimmed == ")" {
                if indent_level > 0 {
                    indent_level -= 1;
                }
                if trimmed.starts_with('}') {
                    in_switch = false;
                }
            }
            
            // Special handling for switch case statements
            let mut current_indent = indent_level;
            if in_switch && (trimmed.starts_with("case ") || trimmed.starts_with("default:")) {
                // Case labels are indented one level more than the switch
                current_indent = indent_level + 1;
            } else if in_switch && indent_level > 0 && !trimmed.starts_with("case ") && !trimmed.starts_with("default:") && !trimmed.starts_with('}') {
                // Case content is indented two levels more than the switch
                current_indent = indent_level + 2;
            }
            
            // Apply indentation
            let indent = if self.config.use_tabs {
                "\t".repeat(current_indent)
            } else {
                " ".repeat(current_indent * self.config.indent_size)
            };
            
            *line = format!("{}{}", indent, trimmed);
            
            // Increase indent for opening braces/parentheses
            if trimmed.ends_with('{') || trimmed.ends_with('(') {
                indent_level += 1;
            }
        }
    }

    /// Format spacing around operators and keywords
    fn format_spacing(&self, lines: &mut [String]) {
        for line in lines {
            // Always format arrays, tuples, and function parameters
            *line = self.format_arrays_and_tuples(line);
            *line = self.format_function_parameters(line);
            *line = self.format_channel_operations(line);
            
            // Handle operator spacing based on configuration
            if self.config.spaces_around_operators {
                *line = self.add_operator_spaces(line);
            } else {
                // For compact mode, remove spaces around operators
                *line = self.remove_operator_spaces(line);
            }
            
            // Handle brace spacing based on configuration
            if self.config.space_before_brace {
                *line = self.add_brace_spaces(line);
            } else {
                // For compact mode, remove spaces before braces
                *line = self.remove_brace_spaces(line);
            }
        }
    }

    /// Add spaces around operators (only if configured)
    fn add_operator_spaces(&self, line: &str) -> String {
        let mut result = line.to_string();
        
        // Binary operators that should have spaces
        let operators = [
            ("==", " == "), ("!=", " != "), ("<=", " <= "), (">=", " >= "),
            ("&&", " && "), ("||", " || "), ("<<", " << "), (">>", " >> "),
            ("+=", " += "), ("-=", " -= "), ("*=", " *= "), ("/=", " /= "),
            ("++", "++"), ("--", "--"), // Keep these without spaces
            (":=", " := "),
        ];
        
        // Handle assignment and comparison operators
        for (op, replacement) in &operators {
            if !result.contains(replacement) {
                result = result.replace(op, replacement);
            }
        }
        
        // Single character operators (handle carefully to avoid conflicts)
        if !result.contains(" = ") && !result.contains(":=") && !result.contains("==") {
            result = result.replace("=", " = ");
        }
        
        // Simple approach: use regex pattern matching for arithmetic operators
        let chars: Vec<char> = result.chars().collect();
        let mut new_result = String::new();
        let mut i = 0;
        
        while i < chars.len() {
            let ch = chars[i];
            
            // Check if this is an arithmetic operator that needs spacing
            if ['+', '-', '*', '/', '%', '<', '>'].contains(&ch) {
                let prev_char = if i > 0 { Some(chars[i-1]) } else { None };
                let next_char = if i + 1 < chars.len() { Some(chars[i+1]) } else { None };
                
                // Add space before if needed
                let needs_space_before = match prev_char {
                    Some(' ') => false, // Already spaced
                    Some('=') => false, // Part of compound operator
                    Some('+') if ch == '+' => false, // ++
                    Some('-') if ch == '-' => false, // --
                    Some('<') if ch == '-' => false, // <-
                    Some(_) => true, // Needs space
                    None => false, // Start of line
                };
                
                // Add space after if needed  
                let needs_space_after = match next_char {
                    Some(' ') => false, // Already spaced
                    Some('=') => false, // Part of compound operator
                    Some('+') if ch == '+' => false, // ++
                    Some('-') if ch == '-' => false, // --
                    Some('-') if ch == '<' => false, // <-
                    Some(_) => true, // Needs space
                    None => false, // End of line
                };
                
                if needs_space_before {
                    new_result.push(' ');
                }
                new_result.push(ch);
                if needs_space_after {
                    new_result.push(' ');
                }
            } else {
                new_result.push(ch);
            }
            
            i += 1;
        }
        result = new_result;
        
        // Clean up multiple spaces
        while result.contains("  ") {
            result = result.replace("  ", " ");
        }
        
        result
    }

    /// Add spaces before braces
    fn add_brace_spaces(&self, line: &str) -> String {
        let mut result = line.to_string();
        
        // Add space before opening braces
        result = result.replace("){", ") {");
        result = result.replace("]{", "] {");
        
        // Keywords that should have space before brace
        let keywords = ["nah", "lowkey", "lol", "bestie", "periodt", "ready", "flex", "vibes", "fam"];
        
        for keyword in keywords {
            let pattern = format!("{}{{", keyword);
            let replacement = format!("{} {{", keyword);
            result = result.replace(&pattern, &replacement);
        }
        
        result
    }

    /// Remove spaces around operators (for compact mode)
    fn remove_operator_spaces(&self, line: &str) -> String {
        let mut result = line.to_string();
        
        // Remove spaces around operators
        result = result.replace(" >= ", ">=");
        result = result.replace(" > ", ">");
        result = result.replace(" <= ", "<=");
        result = result.replace(" < ", "<");
        result = result.replace(" == ", "==");
        result = result.replace(" != ", "!=");
        result = result.replace(" && ", "&&");
        result = result.replace(" || ", "||");
        result = result.replace(" + ", "+");
        result = result.replace(" - ", "-");
        result = result.replace(" * ", "*");
        result = result.replace(" / ", "/");
        result = result.replace(" % ", "%");
        result = result.replace(" = ", "=");
        result = result.replace(" := ", ":=");
        result = result.replace(" += ", "+=");
        result = result.replace(" -= ", "-=");
        result = result.replace(" *= ", "*=");
        result = result.replace(" /= ", "/=");
        result = result.replace(" << ", "<<");
        result = result.replace(" >> ", ">>");
        
        result
    }

    /// Remove spaces before braces (for compact mode)
    fn remove_brace_spaces(&self, line: &str) -> String {
        let mut result = line.to_string();
        
        // Remove spaces before braces
        result = result.replace(" {", "{");
        result = result.replace(" (", "(");
        result = result.replace(" [", "[");
        
        result
    }

    /// Format arrays and tuples
    fn format_arrays_and_tuples(&self, line: &str) -> String {
        let mut result = line.to_string();
        
        // Handle verbose config for arrays
        if self.config.max_array_elements_single_line == 1 {
            // Convert arrays to multiline format
            result = self.format_multiline_arrays(&result);
        } else {
            // Standard array formatting: add spaces after commas
            result = self.add_comma_spaces(&result);
        }
        
        result
    }

    /// Format multiline arrays for verbose config
    fn format_multiline_arrays(&self, line: &str) -> String {
        let mut result = line.to_string();
        
        // Simple approach: if line contains [numbers], format as multiline
        if let Some(start) = result.find('[') {
            if let Some(end) = result.find(']') {
                if start < end {
                    let array_content = &result[start + 1..end];
                    if array_content.contains(',') {
                        // Split into multiple lines
                        let elements: Vec<&str> = array_content.split(',').collect();
                        if elements.len() > 1 {
                            let prefix = &result[..start + 1];
                            let suffix = &result[end..];
                            let mut multiline = prefix.to_string();
                            multiline.push('\n');
                            
                            for (i, elem) in elements.iter().enumerate() {
                                let trimmed = elem.trim();
                                multiline.push_str("    ");
                                multiline.push_str(trimmed);
                                if i < elements.len() - 1 {
                                    multiline.push(',');
                                }
                                multiline.push('\n');
                            }
                            
                            multiline.push_str(suffix);
                            result = multiline;
                        }
                    }
                }
            }
        }
        
        result
    }

    /// Add spaces after commas
    fn add_comma_spaces(&self, line: &str) -> String {
        let mut result = line.to_string();
        
        // Add spaces after commas in arrays, tuples, and function parameters
        let mut chars: Vec<char> = result.chars().collect();
        let mut new_result = String::new();
        let mut i = 0;
        
        while i < chars.len() {
            let ch = chars[i];
            new_result.push(ch);
            
            if ch == ',' {
                // Add space after comma if not already there
                if i + 1 < chars.len() && chars[i + 1] != ' ' && chars[i + 1] != '\n' {
                    new_result.push(' ');
                }
            }
            
            i += 1;
        }
        
        new_result
    }

    /// Format function parameters
    fn format_function_parameters(&self, line: &str) -> String {
        let mut result = line.to_string();
        
        // Add spaces after commas in function parameters
        result = self.add_comma_spaces(&result);
        
        result
    }

    /// Format channel operations
    fn format_channel_operations(&self, line: &str) -> String {
        let mut result = line.to_string();
        
        // Handle channel send: ch<-42 -> ch <- 42
        if !result.contains(" <- ") {
            result = result.replace("<-", " <- ");
        }
        
        // Handle channel receive: value := <-ch -> value := <-ch (no space before <-)
        // Keep the receive operation as is if it already has proper spacing
        
        // Clean up multiple spaces
        while result.contains("  ") {
            result = result.replace("  ", " ");
        }
        
        result
    }

    /// Format import statements
    fn format_imports(&self, lines: &mut Vec<String>) {
        if !self.config.group_imports && !self.config.sort_imports {
            return;
        }
        
        let mut import_indices = Vec::new();
        let mut import_lines = Vec::new();
        
        // Find import statements
        for (i, line) in lines.iter().enumerate() {
            let trimmed = line.trim();
            if trimmed.starts_with("yeet ") {
                import_indices.push(i);
                import_lines.push(trimmed.to_string());
            }
        }
        
        if import_lines.is_empty() {
            return;
        }
        
        // Sort imports if configured
        if self.config.sort_imports {
            import_lines.sort();
        }
        
        // Group imports if configured and there are multiple imports
        if self.config.group_imports && import_lines.len() > 1 {
            // Remove original import lines
            for &i in import_indices.iter().rev() {
                lines.remove(i);
            }
            
            // Add grouped imports at the first import position
            let first_import_pos = import_indices[0];
            lines.insert(first_import_pos, "yeet (".to_string());
            
            for (i, import) in import_lines.iter().enumerate() {
                let import_path = import.trim_start_matches("yeet ").trim();
                lines.insert(first_import_pos + 1 + i, format!("    {}", import_path));
            }
            
            lines.insert(first_import_pos + 1 + import_lines.len(), ")".to_string());
        } else {
            // Replace original imports with sorted ones
            for (i, &line_idx) in import_indices.iter().enumerate() {
                if i < import_lines.len() {
                    lines[line_idx] = import_lines[i].clone();
                }
            }
        }
    }

    /// Format blank lines
    fn format_blank_lines(&self, lines: &mut Vec<String>) {
        if !self.config.preserve_empty_lines {
            return;
        }
        
        let mut result = Vec::new();
        let mut consecutive_empty = 0;
        
        for line in lines.iter() {
            if line.trim().is_empty() {
                consecutive_empty += 1;
                if consecutive_empty <= self.config.max_empty_lines {
                    result.push(String::new());
                }
            } else {
                consecutive_empty = 0;
                result.push(line.clone());
            }
        }
        
        *lines = result;
    }

    /// Trim trailing whitespace
    fn trim_trailing_whitespace(&self, lines: &mut [String]) {
        if !self.config.trim_trailing_whitespace {
            return;
        }
        
        for line in lines {
            *line = line.trim_end().to_string();
        }
    }

    /// Check if source code is already formatted
    pub fn is_formatted(&self, source: &str) -> Result<bool, CursedError> {
        let formatted = self.format(source)?;
        Ok(source == formatted)
    }

    /// Format source code and compare with original
    pub fn format_diff(&self, source: &str) -> Result<String, CursedError> {
        let formatted = self.format(source)?;
        
        if source == formatted {
            Ok(String::new()) // No changes
        } else {
            let diff_formatter = DiffFormatter::default();
            Ok(diff_formatter.generate_diff(source, &formatted, "source"))
        }
    }
}

/// Format a single file
pub fn format_single_file(
    formatter: &SimpleCursedFormatter,
    diff_formatter: &DiffFormatter,
    filename: &str,
    check: bool,
    diff: bool,
) -> Result<FormatCheckResult, Box<dyn std::error::Error>> {
    let source = fs::read_to_string(filename)?;
    
    match formatter.format(&source) {
        Ok(formatted) => {
            if source != formatted {
                if check {
                    Ok(FormatCheckResult::needs_formatting(
                        filename.to_string(),
                        None,
                    ))
                } else if diff {
                    let diff_output = diff_formatter.generate_diff(&source, &formatted, filename);
                    Ok(FormatCheckResult::needs_formatting(
                        filename.to_string(),
                        Some(diff_output),
                    ))
                } else {
                    // Write formatted code to file
                    fs::write(filename, formatted)?;
                    Ok(FormatCheckResult::needs_formatting(
                        filename.to_string(),
                        None,
                    ))
                }
            } else {
                Ok(FormatCheckResult::no_change(filename.to_string()))
            }
        }
        Err(e) => Ok(FormatCheckResult::error(
            filename.to_string(),
            e.to_string(),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_formatting() {
        let formatter = SimpleCursedFormatter::default();
        let source = "sus x drip=42\nsus y tea=\"hello\"\nvibez.spill(x)";
        
        let formatted = formatter.format(source).unwrap();
        
        assert!(formatted.contains("sus x drip = 42"));
        assert!(formatted.contains("sus y tea = \"hello\""));
        assert!(formatted.contains("vibez.spill(x)"));
    }

    #[test]
    fn test_indentation() {
        let formatter = SimpleCursedFormatter::default();
        let source = "nah x > 0 {\nvibez.spill(\"positive\")\n}";
        
        let formatted = formatter.format(source).unwrap();
        
        assert!(formatted.contains("    vibez.spill(\"positive\")"));
    }

    #[test]
    fn test_import_grouping() {
        let formatter = SimpleCursedFormatter::default();
        let source = "yeet \"math\"\nyeet \"string\"\nyeet \"crypto\"";
        
        let formatted = formatter.format(source).unwrap();
        
        assert!(formatted.contains("yeet ("));
        assert!(formatted.contains("    \"math\""));
        assert!(formatted.contains("    \"string\""));
        assert!(formatted.contains("    \"crypto\""));
        assert!(formatted.contains(")"));
    }

    #[test]
    fn test_operator_spacing() {
        let formatter = SimpleCursedFormatter::default();
        let source = "sus result=x+y*z\nsus compare=a==b&&c!=d";
        
        let formatted = formatter.format(source).unwrap();
        
        assert!(formatted.contains("sus result = x + y * z"));
        assert!(formatted.contains("sus compare = a == b && c != d"));
    }

    #[test]
    fn test_short_declaration() {
        let formatter = SimpleCursedFormatter::default();
        let source = "x:=42\n(a,b,c):=(1,2,3)";
        
        let formatted = formatter.format(source).unwrap();
        
        assert!(formatted.contains("x := 42"));
        assert!(formatted.contains("(a,b,c) := (1,2,3)"));
    }

    #[test]
    fn test_brace_spacing() {
        let formatter = SimpleCursedFormatter::default();
        let source = "nah x>0{\nvibez.spill(\"ok\")\n}";
        
        let formatted = formatter.format(source).unwrap();
        
        assert!(formatted.contains("nah x > 0 {"));
    }
}
