//! CLI integration for structured error reporting

use crate::error::{ErrorReporter, ErrorCode, StructuredError};
use clap::Parser;
use std::fs;
use std::path::Path;

/// Enhanced CLI error handling and reporting options
#[derive(Parser, Debug)]
pub struct ErrorCliOptions {
    /// Disable colored output
    #[arg(long)]
    pub no_color: bool,
    
    /// Maximum number of errors to report
    #[arg(long, default_value = "100")]
    pub max_errors: usize,
    
    /// Show detailed error explanations
    #[arg(long)]
    pub explain: Option<String>,
    
    /// Show context lines around errors
    #[arg(long, default_value = "3")]
    pub context_lines: usize,
    
    /// Output errors in JSON format
    #[arg(long)]
    pub json: bool,
    
    /// Show help for all error codes
    #[arg(long)]
    pub list_error_codes: bool,
}

/// Enhanced error reporter with file reading capabilities
pub struct FileAwareErrorReporter {
    reporter: ErrorReporter,
    context_lines: usize,
    json_output: bool,
}

impl FileAwareErrorReporter {
    pub fn new(options: &ErrorCliOptions) -> Self {
        let reporter = ErrorReporter::new()
            .with_colored_output(!options.no_color && !options.json)
            .with_max_errors(options.max_errors);
            
        Self {
            reporter,
            context_lines: options.context_lines,
            json_output: options.json,
        }
    }
    
    /// Add an error with file context
    pub fn add_error_with_file(&mut self, mut error: StructuredError, file_path: &str) {
        // Read file content for context if location is provided
        if let Some(location) = &mut error.location {
            location.file = file_path.to_string();
            
            // Read source line for highlighting
            if let Ok(content) = fs::read_to_string(file_path) {
                let lines: Vec<&str> = content.lines().collect();
                if location.line > 0 && location.line <= lines.len() {
                    location.source_line = Some(lines[location.line - 1].to_string());
                    
                    // Add context lines
                    let start_line = location.line.saturating_sub(self.context_lines);
                    let end_line = (location.line + self.context_lines).min(lines.len());
                    
                    let mut context = Vec::new();
                    for i in start_line..=end_line {
                        if i != location.line && i > 0 && i <= lines.len() {
                            context.push(format!("  {}: {}", i, lines[i - 1]));
                        }
                    }
                    error = error.with_context(context);
                }
            }
        }
        
        self.reporter.add_error(error);
    }
    
    /// Print all errors
    pub fn print_all(&self) {
        if self.json_output {
            self.print_json();
        } else {
            self.reporter.print_all();
        }
    }
    
    /// Print errors in JSON format
    fn print_json(&self) {
        // For now, print a simple JSON structure
        // In a full implementation, you'd use serde to serialize
        println!("{{");
        println!("  \"errors\": [");
        
        // This is a simplified JSON output - in practice you'd implement
        // proper serialization with serde
        let errors = &self.reporter.errors;
        for (i, error) in errors.iter().enumerate() {
            println!("    {{");
            println!("      \"code\": \"{}\",", error.code.as_str());
            println!("      \"message\": \"{}\",", error.message.replace('"', "\\\""));
            println!("      \"severity\": \"{:?}\",", error.severity);
            
            if let Some(location) = &error.location {
                println!("      \"location\": {{");
                println!("        \"file\": \"{}\",", location.file.replace('"', "\\\""));
                println!("        \"line\": {},", location.line);
                println!("        \"column\": {}", location.column);
                println!("      }},");
            }
            
            if !error.suggestions.is_empty() {
                println!("      \"suggestions\": [");
                for (j, suggestion) in error.suggestions.iter().enumerate() {
                    let comma = if j < error.suggestions.len() - 1 { "," } else { "" };
                    println!("        \"{}\"{}",  suggestion.replace('"', "\\\""), comma);
                }
                println!("      ]");
            }
            
            let comma = if i < errors.len() - 1 { "," } else { "" };
            println!("    }}{}", comma);
        }
        
        println!("  ],");
        println!("  \"error_count\": {},", self.reporter.error_count());
        println!("  \"warning_count\": {}", self.reporter.warning_count());
        println!("}}");
    }
    
    /// Handle CLI explain command
    pub fn handle_explain_command(code_str: &str) -> Result<(), String> {
        let code = parse_error_code(code_str)?;
        let reporter = ErrorReporter::new();
        reporter.print_explanation(code);
        Ok(())
    }
    
    /// List all available error codes
    pub fn list_error_codes() {
        println!("Available error codes:");
        println!();
        
        // Syntax errors
        println!("Syntax Errors (E0001-E0099):");
        println!("  E0001 - Unexpected token");
        println!("  E0002 - Unterminated string literal");
        println!("  E0003 - Unterminated block comment"); 
        println!("  E0004 - Invalid escape sequence");
        println!("  E0005 - Invalid character");
        println!("  E0006 - Missing semicolon");
        println!("  E0007 - Unexpected end of input");
        println!("  E0008 - Unclosed delimiter");
        println!("  E0009 - Invalid number format");
        println!("  E0010 - Invalid identifier");
        println!();
        
        // Type errors
        println!("Type Errors (E0100-E0199):");
        println!("  E0100 - Type mismatch");
        println!("  E0101 - Unknown type");
        println!("  E0102 - Cannot infer type");
        println!("  E0103 - Circular type dependency");
        println!("  E0104 - Generic type parameter not found");
        println!("  E0105 - Wrong number of type arguments");
        println!("  E0106 - Type does not implement trait");
        println!("  E0107 - Mismatched function signature");
        println!("  E0108 - Cannot assign to immutable variable");
        println!("  E0109 - Variable not found");
        println!("  E0110 - Function not found");
        println!("  E0111 - Struct field not found");
        println!("  E0112 - Cannot access private field");
        println!("  E0113 - Cannot call private function");
        println!("  E0114 - Type annotation required");
        println!("  E0115 - Invalid type cast");
        println!();
        
        // Compilation errors
        println!("Compilation Errors (E0200-E0299):");
        println!("  E0200 - Import not found");
        println!("  E0201 - Circular import");
        println!("  E0202 - Package not found");
        println!("  E0203 - Multiple definitions");
        println!("  E0204 - No main function");
        println!("  E0205 - Invalid main function signature");
        println!("  E0206 - LLVM error");
        println!("  E0207 - Codegen error");
        println!("  E0208 - Optimization error");
        println!("  E0209 - Linking error");
        println!("  E0210 - Invalid target");
        println!();
        
        println!("Use `cursed --explain <code>` for detailed explanations.");
    }
    
    /// Check if there are errors
    pub fn has_errors(&self) -> bool {
        self.reporter.has_errors()
    }
    
    /// Get error count
    pub fn error_count(&self) -> usize {
        self.reporter.error_count()
    }
}

/// Parse error code string (E0001, 0001, etc.)
fn parse_error_code(code_str: &str) -> Result<ErrorCode, String> {
    let normalized = if code_str.starts_with('E') {
        code_str.to_uppercase()
    } else {
        format!("E{:0>4}", code_str)
    };
    
    match normalized.as_str() {
        "E0001" => Ok(ErrorCode::E0001),
        "E0002" => Ok(ErrorCode::E0002),
        "E0003" => Ok(ErrorCode::E0003),
        "E0004" => Ok(ErrorCode::E0004),
        "E0005" => Ok(ErrorCode::E0005),
        "E0006" => Ok(ErrorCode::E0006),
        "E0007" => Ok(ErrorCode::E0007),
        "E0008" => Ok(ErrorCode::E0008),
        "E0009" => Ok(ErrorCode::E0009),
        "E0010" => Ok(ErrorCode::E0010),
        "E0100" => Ok(ErrorCode::E0100),
        "E0101" => Ok(ErrorCode::E0101),
        "E0102" => Ok(ErrorCode::E0102),
        "E0103" => Ok(ErrorCode::E0103),
        "E0104" => Ok(ErrorCode::E0104),
        "E0105" => Ok(ErrorCode::E0105),
        "E0106" => Ok(ErrorCode::E0106),
        "E0107" => Ok(ErrorCode::E0107),
        "E0108" => Ok(ErrorCode::E0108),
        "E0109" => Ok(ErrorCode::E0109),
        "E0110" => Ok(ErrorCode::E0110),
        "E0111" => Ok(ErrorCode::E0111),
        "E0112" => Ok(ErrorCode::E0112),
        "E0113" => Ok(ErrorCode::E0113),
        "E0114" => Ok(ErrorCode::E0114),
        "E0115" => Ok(ErrorCode::E0115),
        "E0200" => Ok(ErrorCode::E0200),
        "E0201" => Ok(ErrorCode::E0201),
        "E0202" => Ok(ErrorCode::E0202),
        "E0203" => Ok(ErrorCode::E0203),
        "E0204" => Ok(ErrorCode::E0204),
        "E0205" => Ok(ErrorCode::E0205),
        "E0206" => Ok(ErrorCode::E0206),
        "E0207" => Ok(ErrorCode::E0207),
        "E0208" => Ok(ErrorCode::E0208),
        "E0209" => Ok(ErrorCode::E0209),
        "E0210" => Ok(ErrorCode::E0210),
        "E0300" => Ok(ErrorCode::E0300),
        "E0301" => Ok(ErrorCode::E0301),
        "E0302" => Ok(ErrorCode::E0302),
        "E0303" => Ok(ErrorCode::E0303),
        "E0304" => Ok(ErrorCode::E0304),
        "E0305" => Ok(ErrorCode::E0305),
        "E0306" => Ok(ErrorCode::E0306),
        "E0307" => Ok(ErrorCode::E0307),
        "E0308" => Ok(ErrorCode::E0308),
        "E0309" => Ok(ErrorCode::E0309),
        "E0400" => Ok(ErrorCode::E0400),
        "E0401" => Ok(ErrorCode::E0401),
        "E0402" => Ok(ErrorCode::E0402),
        "E0403" => Ok(ErrorCode::E0403),
        "E0404" => Ok(ErrorCode::E0404),
        "E0405" => Ok(ErrorCode::E0405),
        "E0406" => Ok(ErrorCode::E0406),
        "E0407" => Ok(ErrorCode::E0407),
        "E0408" => Ok(ErrorCode::E0408),
        "E0409" => Ok(ErrorCode::E0409),
        "E0500" => Ok(ErrorCode::E0500),
        "E0501" => Ok(ErrorCode::E0501),
        "E0502" => Ok(ErrorCode::E0502),
        "E0503" => Ok(ErrorCode::E0503),
        "E0504" => Ok(ErrorCode::E0504),
        "E0505" => Ok(ErrorCode::E0505),
        "E0506" => Ok(ErrorCode::E0506),
        "E0507" => Ok(ErrorCode::E0507),
        "E0508" => Ok(ErrorCode::E0508),
        "E0509" => Ok(ErrorCode::E0509),
        _ => Err(format!("Unknown error code: {}", code_str)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_parse_error_code() {
        assert!(matches!(parse_error_code("E0001"), Ok(ErrorCode::E0001)));
        assert!(matches!(parse_error_code("0001"), Ok(ErrorCode::E0001)));
        assert!(matches!(parse_error_code("1"), Ok(ErrorCode::E0001)));
        assert!(parse_error_code("E9999").is_err());
    }
    
    #[test]
    fn test_cli_options() {
        let options = ErrorCliOptions {
            no_color: false,
            max_errors: 50,
            explain: None,
            context_lines: 2,
            json: false,
            list_error_codes: false,
        };
        
        let reporter = FileAwareErrorReporter::new(&options);
        assert!(!reporter.json_output);
        assert_eq!(reporter.context_lines, 2);
    }
}
