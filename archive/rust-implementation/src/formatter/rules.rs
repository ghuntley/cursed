//! Formatting rules and style guidelines for CURSED code

use crate::ast::*;
use crate::formatter::config::FormatterConfig;

/// Formatting rule that can be applied to AST nodes
pub trait FormattingRule {
    /// Apply the rule to format a piece of code
    fn apply(&self, input: &str, config: &FormatterConfig) -> String;
    
    /// Get the name of this rule
    fn name(&self) -> &'static str;
    
    /// Check if this rule should be applied
    fn should_apply(&self, config: &FormatterConfig) -> bool {
        true
    }
}

/// Rule for formatting indentation
pub struct IndentationRule;

impl FormattingRule for IndentationRule {
    fn apply(&self, input: &str, config: &FormatterConfig) -> String {
        let lines: Vec<&str> = input.lines().collect();
        let mut result = Vec::new();
        let mut indent_level = 0;
        
        for line in lines {
            let trimmed = line.trim();
            
            // Decrease indent for closing braces
            if trimmed.starts_with('}') || trimmed.starts_with(')') || trimmed.starts_with(']') {
                if indent_level > 0 {
                    indent_level -= 1;
                }
            }
            
            // Apply indentation
            if !trimmed.is_empty() {
                let indent = if config.use_tabs {
                    "\t".repeat(indent_level)
                } else {
                    " ".repeat(indent_level * config.indent_size)
                };
                result.push(format!("{}{}", indent, trimmed));
            } else {
                result.push(String::new());
            }
            
            // Increase indent for opening braces
            if trimmed.ends_with('{') || trimmed.ends_with('(') || trimmed.ends_with('[') {
                indent_level += 1;
            }
        }
        
        result.join("\n")
    }
    
    fn name(&self) -> &'static str {
        "indentation"
    }
}

/// Rule for formatting operators
pub struct OperatorSpacingRule;

impl FormattingRule for OperatorSpacingRule {
    fn apply(&self, input: &str, config: &FormatterConfig) -> String {
        if !config.spaces_around_operators {
            return input.to_string();
        }
        
        let mut result = input.to_string();
        
        // Binary operators that should have spaces
        let operators = [
            "==", "!=", "<=", ">=", "&&", "||", "<<", ">>",
            "+", "-", "*", "/", "%", "<", ">", "=", "&", "|", "^"
        ];
        
        // Sort by length (longest first) to avoid partial replacements
        let mut sorted_ops = operators.to_vec();
        sorted_ops.sort_by_key(|op| std::cmp::Reverse(op.len()));
        
        for op in sorted_ops {
            // Skip if operator is already properly spaced
            let spaced_op = format!(" {} ", op);
            if result.contains(&spaced_op) {
                continue;
            }
            
            // Add spaces around operators
            result = result.replace(op, &spaced_op);
            
            // Clean up multiple spaces
            result = result.replace(&format!("  {}  ", op), &spaced_op);
        }
        
        result
    }
    
    fn name(&self) -> &'static str {
        "operator_spacing"
    }
    
    fn should_apply(&self, config: &FormatterConfig) -> bool {
        config.spaces_around_operators
    }
}

/// Rule for formatting line length
pub struct LineLengthRule;

impl FormattingRule for LineLengthRule {
    fn apply(&self, input: &str, config: &FormatterConfig) -> String {
        let lines: Vec<&str> = input.lines().collect();
        let mut result = Vec::new();
        
        for line in lines {
            if line.len() <= config.max_line_length {
                result.push(line.to_string());
            } else {
                // Try to break long lines
                let broken_lines = self.break_line(line, config);
                result.extend(broken_lines);
            }
        }
        
        result.join("\n")
    }
    
    fn name(&self) -> &'static str {
        "line_length"
    }
}

impl LineLengthRule {
    /// Break a long line into multiple lines
    fn break_line(&self, line: &str, config: &FormatterConfig) -> Vec<String> {
        let mut result = Vec::new();
        let mut current_line = String::new();
        let mut current_length = 0;
        let indent = self.get_line_indent(line, config);
        let continuation_indent = format!("{}    ", indent); // Extra indentation for continuation
        
        // Simple word-based breaking for now
        for word in line.split_whitespace() {
            let word_length = word.len() + 1; // +1 for space
            
            if current_length + word_length > config.max_line_length && !current_line.is_empty() {
                result.push(current_line);
                current_line = format!("{}{}", continuation_indent, word);
                current_length = continuation_indent.len() + word.len();
            } else {
                if !current_line.is_empty() {
                    current_line.push(' ');
                    current_length += 1;
                }
                current_line.push_str(word);
                current_length += word.len();
            }
        }
        
        if !current_line.is_empty() {
            result.push(current_line);
        }
        
        if result.is_empty() {
            result.push(line.to_string());
        }
        
        result
    }
    
    /// Get the indentation of a line
    fn get_line_indent(&self, line: &str, config: &FormatterConfig) -> String {
        let mut indent = String::new();
        
        for ch in line.chars() {
            if ch == ' ' || ch == '\t' {
                indent.push(ch);
            } else {
                break;
            }
        }
        
        indent
    }
}

/// Rule for formatting blank lines
pub struct BlankLineRule;

impl FormattingRule for BlankLineRule {
    fn apply(&self, input: &str, config: &FormatterConfig) -> String {
        let lines: Vec<&str> = input.lines().collect();
        let mut result = Vec::new();
        let mut consecutive_empty = 0;
        
        for line in lines {
            if line.trim().is_empty() {
                consecutive_empty += 1;
                if consecutive_empty <= config.max_empty_lines {
                    result.push(String::new());
                }
            } else {
                consecutive_empty = 0;
                result.push(line.to_string());
            }
        }
        
        result.join("\n")
    }
    
    fn name(&self) -> &'static str {
        "blank_lines"
    }
    
    fn should_apply(&self, config: &FormatterConfig) -> bool {
        config.preserve_empty_lines && config.max_empty_lines > 0
    }
}

/// Rule for formatting trailing whitespace
pub struct TrailingWhitespaceRule;

impl FormattingRule for TrailingWhitespaceRule {
    fn apply(&self, input: &str, config: &FormatterConfig) -> String {
        if !config.trim_trailing_whitespace {
            return input.to_string();
        }
        
        let lines: Vec<&str> = input.lines().collect();
        let result: Vec<String> = lines.iter()
            .map(|line| line.trim_end().to_string())
            .collect();
        
        let mut formatted = result.join("\n");
        
        // Add final newline if configured
        if config.insert_final_newline && !formatted.ends_with('\n') {
            formatted.push('\n');
        }
        
        formatted
    }
    
    fn name(&self) -> &'static str {
        "trailing_whitespace"
    }
    
    fn should_apply(&self, config: &FormatterConfig) -> bool {
        config.trim_trailing_whitespace
    }
}

/// Rule for formatting import statements
pub struct ImportFormattingRule;

impl FormattingRule for ImportFormattingRule {
    fn apply(&self, input: &str, config: &FormatterConfig) -> String {
        let lines: Vec<&str> = input.lines().collect();
        let mut result = Vec::new();
        let mut import_lines = Vec::new();
        let mut in_import_block = false;
        
        for line in lines {
            let trimmed = line.trim();
            
            if trimmed.starts_with("yeet ") {
                import_lines.push(line.to_string());
                in_import_block = true;
            } else if in_import_block && trimmed.is_empty() {
                // End of import block
                if config.sort_imports {
                    import_lines.sort();
                }
                
                if config.group_imports && import_lines.len() > 1 {
                    result.push("yeet (".to_string());
                    for import in &import_lines {
                        let import_path = import.trim_start_matches("yeet ").trim();
                        result.push(format!("    {}", import_path));
                    }
                    result.push(")".to_string());
                } else {
                    result.extend(import_lines.clone());
                }
                
                import_lines.clear();
                in_import_block = false;
                result.push(line.to_string());
            } else {
                if in_import_block {
                    // Process remaining imports
                    if config.sort_imports {
                        import_lines.sort();
                    }
                    
                    if config.group_imports && import_lines.len() > 1 {
                        result.push("yeet (".to_string());
                        for import in &import_lines {
                            let import_path = import.trim_start_matches("yeet ").trim();
                            result.push(format!("    {}", import_path));
                        }
                        result.push(")".to_string());
                    } else {
                        result.extend(import_lines.clone());
                    }
                    
                    import_lines.clear();
                    in_import_block = false;
                }
                result.push(line.to_string());
            }
        }
        
        // Handle imports at end of file
        if in_import_block {
            if config.sort_imports {
                import_lines.sort();
            }
            
            if config.group_imports && import_lines.len() > 1 {
                result.push("yeet (".to_string());
                for import in &import_lines {
                    let import_path = import.trim_start_matches("yeet ").trim();
                    result.push(format!("    {}", import_path));
                }
                result.push(")".to_string());
            } else {
                result.extend(import_lines);
            }
        }
        
        result.join("\n")
    }
    
    fn name(&self) -> &'static str {
        "import_formatting"
    }
    
    fn should_apply(&self, config: &FormatterConfig) -> bool {
        config.group_imports || config.sort_imports
    }
}

/// Rule for formatting brace style
pub struct BraceStyleRule;

impl FormattingRule for BraceStyleRule {
    fn apply(&self, input: &str, config: &FormatterConfig) -> String {
        if !config.space_before_brace {
            return input.to_string();
        }
        
        let mut result = input.to_string();
        
        // Add space before opening braces
        result = result.replace("){", ") {");
        result = result.replace("]{", "] {");
        
        // Keywords that should have space before brace
        let keywords = ["nah", "lowkey", "lol", "bestie", "periodt", "ready", "flex", "vibes"];
        
        for keyword in keywords {
            let pattern = format!("{}{{", keyword);
            let replacement = format!("{} {{", keyword);
            result = result.replace(&pattern, &replacement);
        }
        
        result
    }
    
    fn name(&self) -> &'static str {
        "brace_style"
    }
    
    fn should_apply(&self, config: &FormatterConfig) -> bool {
        config.space_before_brace
    }
}

/// Rule for formatting parentheses spacing
pub struct ParenthesesSpacingRule;

impl FormattingRule for ParenthesesSpacingRule {
    fn apply(&self, input: &str, config: &FormatterConfig) -> String {
        let mut result = input.to_string();
        
        if config.space_inside_parentheses {
            // Add spaces inside parentheses
            result = result.replace("(", "( ");
            result = result.replace(")", " )");
            
            // Clean up multiple spaces
            result = result.replace("(  ", "( ");
            result = result.replace("  )", " )");
            
            // Handle empty parentheses
            result = result.replace("( )", "()");
        } else {
            // Remove spaces inside parentheses
            result = result.replace("( ", "(");
            result = result.replace(" )", ")");
        }
        
        result
    }
    
    fn name(&self) -> &'static str {
        "parentheses_spacing"
    }
}

/// Rule for formatting array/slice brackets
pub struct BracketSpacingRule;

impl FormattingRule for BracketSpacingRule {
    fn apply(&self, input: &str, config: &FormatterConfig) -> String {
        let mut result = input.to_string();
        
        if config.space_inside_brackets {
            // Add spaces inside brackets
            result = result.replace("[", "[ ");
            result = result.replace("]", " ]");
            
            // Clean up multiple spaces
            result = result.replace("[  ", "[ ");
            result = result.replace("  ]", " ]");
            
            // Handle empty brackets
            result = result.replace("[ ]", "[]");
        } else {
            // Remove spaces inside brackets
            result = result.replace("[ ", "[");
            result = result.replace(" ]", "]");
        }
        
        result
    }
    
    fn name(&self) -> &'static str {
        "bracket_spacing"
    }
}

/// Collection of all formatting rules
pub struct FormattingRules {
    rules: Vec<Box<dyn FormattingRule>>,
}

impl Default for FormattingRules {
    fn default() -> Self {
        Self {
            rules: vec![
                Box::new(TrailingWhitespaceRule),
                Box::new(ImportFormattingRule),
                Box::new(IndentationRule),
                Box::new(OperatorSpacingRule),
                Box::new(BraceStyleRule),
                Box::new(ParenthesesSpacingRule),
                Box::new(BracketSpacingRule),
                Box::new(BlankLineRule),
                Box::new(LineLengthRule),
            ],
        }
    }
}

impl FormattingRules {
    /// Apply all applicable rules to the input
    pub fn apply_all(&self, input: &str, config: &FormatterConfig) -> String {
        let mut result = input.to_string();
        
        for rule in &self.rules {
            if rule.should_apply(config) {
                result = rule.apply(&result, config);
            }
        }
        
        result
    }
    
    /// Apply a specific rule by name
    pub fn apply_rule(&self, input: &str, rule_name: &str, config: &FormatterConfig) -> Option<String> {
        for rule in &self.rules {
            if rule.name() == rule_name && rule.should_apply(config) {
                return Some(rule.apply(input, config));
            }
        }
        None
    }
    
    /// Get all rule names
    pub fn rule_names(&self) -> Vec<&str> {
        self.rules.iter().map(|rule| rule.name()).collect()
    }
}
