use std::fmt;

// Standalone test for the CURSED formatter
// This tests the formatter in isolation without depending on the rest of the codebase

#[derive(Debug)]
pub enum Error {
    Parse(String),
    Compile(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Parse(msg) => write!(f, "Parse error: {}", msg),
            Error::Compile(msg) => write!(f, "Compilation error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

// Copy of the formatter code for standalone testing
#[derive(Debug, Clone)]
pub struct CursedFormatter {
    config: FormatterConfig,
    indentation_level: usize,
    line_buffer: Vec<String>,
    current_line: String,
}

#[derive(Debug, Clone)]
pub struct FormatterConfig {
    pub options: FormattingOptions,
    pub preserve_comments: bool,
    pub sort_imports: bool,
    pub max_line_length: usize,
    pub trailing_commas: bool,
}

#[derive(Debug, Clone)]
pub struct FormattingOptions {
    pub indent_style: IndentStyle,
    pub indent_size: usize,
    pub spaces_around_operators: bool,
    pub spaces_after_commas: bool,
    pub spaces_inside_parentheses: bool,
    pub spaces_inside_braces: bool,
    pub brace_style: BraceStyle,
    pub blank_lines: BlankLinePolicy,
}

#[derive(Debug, Clone, PartialEq)]
pub enum IndentStyle {
    Spaces,
    Tabs,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BraceStyle {
    SameLine,
    NextLine,
    NextLineIndented,
}

#[derive(Debug, Clone)]
pub struct BlankLinePolicy {
    pub max_blank_lines: usize,
    pub before_functions: usize,
    pub after_imports: usize,
    pub around_control_structures: usize,
}

impl Default for FormatterConfig {
    fn default() -> Self {
        Self {
            options: FormattingOptions::default(),
            preserve_comments: true,
            sort_imports: true,
            max_line_length: 100,
            trailing_commas: false,
        }
    }
}

impl Default for FormattingOptions {
    fn default() -> Self {
        Self {
            indent_style: IndentStyle::Spaces,
            indent_size: 4,
            spaces_around_operators: true,
            spaces_after_commas: true,
            spaces_inside_parentheses: false,
            spaces_inside_braces: true,
            brace_style: BraceStyle::SameLine,
            blank_lines: BlankLinePolicy::default(),
        }
    }
}

impl Default for BlankLinePolicy {
    fn default() -> Self {
        Self {
            max_blank_lines: 2,
            before_functions: 1,
            after_imports: 1,
            around_control_structures: 0,
        }
    }
}

impl CursedFormatter {
    pub fn new() -> Self {
        Self::with_config(FormatterConfig::default())
    }

    pub fn with_config(config: FormatterConfig) -> Self {
        Self {
            config,
            indentation_level: 0,
            line_buffer: Vec::new(),
            current_line: String::new(),
        }
    }

    pub fn format(&mut self, source: &str) -> Result<String> {
        // Simple formatting for demonstration
        let mut result = String::new();
        let mut lines = source.lines();
        
        while let Some(line) = lines.next() {
            let trimmed = line.trim();
            
            // Handle Gen Z keywords
            if trimmed.starts_with("facts") {
                result.push_str(&self.format_facts_line(trimmed));
            } else if trimmed.starts_with("slay") {
                result.push_str(&self.format_slay_line(trimmed));
            } else if trimmed.starts_with("yeet") {
                result.push_str(&self.format_yeet_line(trimmed));
            } else if trimmed.starts_with("lowkey") {
                result.push_str(&self.format_lowkey_line(trimmed));
            } else if trimmed.starts_with("yolo") {
                result.push_str(&self.format_yolo_line(trimmed));
            } else if trimmed == "{" {
                result.push_str(" {\n");
                self.indentation_level += 1;
            } else if trimmed == "}" {
                if self.indentation_level > 0 {
                    self.indentation_level -= 1;
                }
                result.push_str(&self.get_indentation());
                result.push_str("}\n");
            } else if !trimmed.is_empty() {
                result.push_str(&self.get_indentation());
                result.push_str(trimmed);
                result.push('\n');
            } else {
                result.push('\n');
            }
        }
        
        Ok(result)
    }
    
    fn format_facts_line(&self, line: &str) -> String {
        let formatted = if self.config.options.spaces_around_operators {
            line.replace("=", " = ")
        } else {
            line.replace(" = ", "=")
        };
        
        format!("{}{}\n", self.get_indentation(), formatted)
    }
    
    fn format_slay_line(&self, line: &str) -> String {
        let mut formatted = line.to_string();
        
        // Add space after slay if not present
        if !formatted.contains("slay ") {
            formatted = formatted.replace("slay", "slay ");
        }
        
        // Handle parentheses spacing
        if self.config.options.spaces_inside_parentheses {
            formatted = formatted.replace("(", "( ").replace(")", " )");
        }
        
        format!("{}{}", self.get_indentation(), formatted)
    }
    
    fn format_yeet_line(&self, line: &str) -> String {
        let mut formatted = line.to_string();
        
        // Add space after yeet if not present
        if !formatted.contains("yeet ") {
            formatted = formatted.replace("yeet", "yeet ");
        }
        
        format!("{}{}\n", self.get_indentation(), formatted)
    }
    
    fn format_lowkey_line(&self, line: &str) -> String {
        let mut formatted = line.to_string();
        
        // Add space after lowkey if not present
        if !formatted.contains("lowkey ") {
            formatted = formatted.replace("lowkey", "lowkey ");
        }
        
        format!("{}{}", self.get_indentation(), formatted)
    }
    
    fn format_yolo_line(&self, line: &str) -> String {
        let mut formatted = line.to_string();
        
        // Add space after yolo if not present
        if !formatted.contains("yolo ") {
            formatted = formatted.replace("yolo", "yolo ");
        }
        
        format!("{}{}\n", self.get_indentation(), formatted)
    }
    
    fn get_indentation(&self) -> String {
        match self.config.options.indent_style {
            IndentStyle::Spaces => " ".repeat(self.config.options.indent_size * self.indentation_level),
            IndentStyle::Tabs => "\t".repeat(self.indentation_level),
        }
    }
}

fn main() {
    println!("Testing CURSED Formatter");
    
    let mut formatter = CursedFormatter::new();
    
    // Test basic facts declaration
    let test1 = "facts   x=42;";
    println!("Input: {}", test1);
    match formatter.format(test1) {
        Ok(result) => println!("Output: {}", result.trim()),
        Err(e) => println!("Error: {}", e),
    }
    println!();
    
    // Test slay function
    let test2 = "slay main(){yolo \"Hello\"}";
    println!("Input: {}", test2);
    match formatter.format(test2) {
        Ok(result) => println!("Output:\n{}", result),
        Err(e) => println!("Error: {}", e),
    }
    println!();
    
    // Test yeet import
    let test3 = "yeet\"std::io\";";
    println!("Input: {}", test3);
    match formatter.format(test3) {
        Ok(result) => println!("Output: {}", result.trim()),
        Err(e) => println!("Error: {}", e),
    }
    println!();
    
    // Test complex example
    let test4 = r#"facts greeting = "Hello from CURSED!"
facts number = 42

slay main() {
yolo greeting;
yolo number;
}

slay calculate(x, y) {
yolo x + y;
}"#;
    
    println!("Complex test input:\n{}", test4);
    match formatter.format(test4) {
        Ok(result) => println!("Complex test output:\n{}", result),
        Err(e) => println!("Error: {}", e),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_facts_declaration() {
        let mut formatter = CursedFormatter::new();
        let source = "facts   x=42;";
        
        let result = formatter.format(source).unwrap();
        assert!(result.contains("facts x = 42"));
    }

    #[test]
    fn test_format_slay_function() {
        let mut formatter = CursedFormatter::new();
        let source = "slay  main(){yolo \"Hello\"}";
        
        let result = formatter.format(source).unwrap();
        assert!(result.contains("slay main()"));
        assert!(result.contains("yolo \"Hello\""));
    }

    #[test]
    fn test_format_yeet_import() {
        let mut formatter = CursedFormatter::new();
        let source = "yeet\"std::io\";";
        
        let result = formatter.format(source).unwrap();
        assert!(result.contains("yeet \"std::io\""));
    }

    #[test]
    fn test_indentation() {
        let mut formatter = CursedFormatter::new();
        let source = "slay main() {\nyolo \"test\";\n}";
        
        let result = formatter.format(source).unwrap();
        // Should contain proper indentation
        assert!(result.contains("    yolo"));
    }
}
