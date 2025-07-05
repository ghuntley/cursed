//! Structured error system for CURSED compiler
//! 
//! This module provides a comprehensive error handling system with:
//! - Structured error codes (E0001, E0002, etc.)
//! - User-friendly error messages with context
//! - Colored output and source location highlighting
//! - Error recovery and multiple error reporting
//! - Contextual help and suggestions

use std::fmt;
use std::collections::HashMap;
use colored::Colorize;

/// Structured error with code, message, and context
#[derive(Debug, Clone)]
pub struct StructuredError {
    pub code: ErrorCode,
    pub message: String,
    pub location: Option<ErrorSourceLocation>,
    pub context: Vec<String>,
    pub suggestions: Vec<String>,
    pub severity: ErrorSeverity,
}

/// Error codes similar to rustc (E0001, E0002, etc.)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCode {
    // Syntax errors (E0001-E0099)
    E0001, // Unexpected token
    E0002, // Unterminated string
    E0003, // Unterminated block comment
    E0004, // Invalid escape sequence
    E0005, // Invalid character
    E0006, // Missing semicolon
    E0007, // Unexpected end of input
    E0008, // Unclosed delimiter
    E0009, // Invalid number format
    E0010, // Invalid identifier
    
    // Type errors (E0100-E0199)
    E0100, // Type mismatch
    E0101, // Unknown type
    E0102, // Cannot infer type
    E0103, // Circular type dependency
    E0104, // Generic type parameter not found
    E0105, // Wrong number of type arguments
    E0106, // Type does not implement trait
    E0107, // Mismatched function signature
    E0108, // Cannot assign to immutable variable
    E0109, // Variable not found
    E0110, // Function not found
    E0111, // Struct field not found
    E0112, // Cannot access private field
    E0113, // Cannot call private function
    E0114, // Type annotation required
    E0115, // Invalid type cast
    
    // Compilation errors (E0200-E0299)
    E0200, // Import not found
    E0201, // Circular import
    E0202, // Package not found
    E0203, // Multiple definitions
    E0204, // No main function
    E0205, // Invalid main function signature
    E0206, // LLVM error
    E0207, // Codegen error
    E0208, // Optimization error
    E0209, // Linking error
    E0210, // Invalid target
    
    // Runtime errors (E0300-E0399)
    E0300, // Null pointer dereference
    E0301, // Index out of bounds
    E0302, // Stack overflow
    E0303, // Heap overflow
    E0304, // Division by zero
    E0305, // Channel operation failed
    E0306, // Goroutine panic
    E0307, // Deadlock detected
    E0308, // Memory allocation failed
    E0309, // Invalid operation
    
    // Security errors (E0400-E0499)
    E0400, // Unsafe operation
    E0401, // Buffer overflow
    E0402, // Use after free
    E0403, // Double free
    E0404, // Invalid memory access
    E0405, // Crypto operation failed
    E0406, // Invalid certificate
    E0407, // Authentication failed
    E0408, // Authorization failed
    E0409, // Insecure configuration
    
    // I/O errors (E0500-E0599)
    E0500, // File not found
    E0501, // Permission denied
    E0502, // Network error
    E0503, // Database error
    E0504, // Serialization error
    E0505, // Deserialization error
    E0506, // Encoding error
    E0507, // Decoding error
    E0508, // Timeout error
    E0509, // Connection error
}

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ErrorSeverity {
    Error,
    Warning,
    Note,
    Help,
}

/// Enhanced source location with file content for highlighting
#[derive(Debug, Clone)]
pub struct ErrorSourceLocation {
    pub file: String,
    pub line: usize,
    pub column: usize,
    pub length: usize,
    pub source_line: Option<String>,
}

/// Error reporter with colored output and multiple error handling
pub struct ErrorReporter {
    pub errors: Vec<StructuredError>,
    pub warnings: Vec<StructuredError>,
    max_errors: usize,
    colored_output: bool,
    error_explanations: HashMap<ErrorCode, ErrorExplanation>,
}

/// Detailed error explanation with examples
#[derive(Debug, Clone)]
pub struct ErrorExplanation {
    pub title: String,
    pub description: String,
    pub examples: Vec<String>,
    pub common_causes: Vec<String>,
    pub solutions: Vec<String>,
}

impl ErrorCode {
    /// Get the error code as a string (e.g., "E0001")
    pub fn as_str(&self) -> &'static str {
        match self {
            ErrorCode::E0001 => "E0001",
            ErrorCode::E0002 => "E0002",
            ErrorCode::E0003 => "E0003",
            ErrorCode::E0004 => "E0004",
            ErrorCode::E0005 => "E0005",
            ErrorCode::E0006 => "E0006",
            ErrorCode::E0007 => "E0007",
            ErrorCode::E0008 => "E0008",
            ErrorCode::E0009 => "E0009",
            ErrorCode::E0010 => "E0010",
            ErrorCode::E0100 => "E0100",
            ErrorCode::E0101 => "E0101",
            ErrorCode::E0102 => "E0102",
            ErrorCode::E0103 => "E0103",
            ErrorCode::E0104 => "E0104",
            ErrorCode::E0105 => "E0105",
            ErrorCode::E0106 => "E0106",
            ErrorCode::E0107 => "E0107",
            ErrorCode::E0108 => "E0108",
            ErrorCode::E0109 => "E0109",
            ErrorCode::E0110 => "E0110",
            ErrorCode::E0111 => "E0111",
            ErrorCode::E0112 => "E0112",
            ErrorCode::E0113 => "E0113",
            ErrorCode::E0114 => "E0114",
            ErrorCode::E0115 => "E0115",
            ErrorCode::E0200 => "E0200",
            ErrorCode::E0201 => "E0201",
            ErrorCode::E0202 => "E0202",
            ErrorCode::E0203 => "E0203",
            ErrorCode::E0204 => "E0204",
            ErrorCode::E0205 => "E0205",
            ErrorCode::E0206 => "E0206",
            ErrorCode::E0207 => "E0207",
            ErrorCode::E0208 => "E0208",
            ErrorCode::E0209 => "E0209",
            ErrorCode::E0210 => "E0210",
            ErrorCode::E0300 => "E0300",
            ErrorCode::E0301 => "E0301",
            ErrorCode::E0302 => "E0302",
            ErrorCode::E0303 => "E0303",
            ErrorCode::E0304 => "E0304",
            ErrorCode::E0305 => "E0305",
            ErrorCode::E0306 => "E0306",
            ErrorCode::E0307 => "E0307",
            ErrorCode::E0308 => "E0308",
            ErrorCode::E0309 => "E0309",
            ErrorCode::E0400 => "E0400",
            ErrorCode::E0401 => "E0401",
            ErrorCode::E0402 => "E0402",
            ErrorCode::E0403 => "E0403",
            ErrorCode::E0404 => "E0404",
            ErrorCode::E0405 => "E0405",
            ErrorCode::E0406 => "E0406",
            ErrorCode::E0407 => "E0407",
            ErrorCode::E0408 => "E0408",
            ErrorCode::E0409 => "E0409",
            ErrorCode::E0500 => "E0500",
            ErrorCode::E0501 => "E0501",
            ErrorCode::E0502 => "E0502",
            ErrorCode::E0503 => "E0503",
            ErrorCode::E0504 => "E0504",
            ErrorCode::E0505 => "E0505",
            ErrorCode::E0506 => "E0506",
            ErrorCode::E0507 => "E0507",
            ErrorCode::E0508 => "E0508",
            ErrorCode::E0509 => "E0509",
        }
    }
    
    /// Get the error category
    pub fn category(&self) -> &'static str {
        match self {
            ErrorCode::E0001 | ErrorCode::E0002 | ErrorCode::E0003 | ErrorCode::E0004 |
            ErrorCode::E0005 | ErrorCode::E0006 | ErrorCode::E0007 | ErrorCode::E0008 |
            ErrorCode::E0009 | ErrorCode::E0010 => "Syntax",
            
            ErrorCode::E0100 | ErrorCode::E0101 | ErrorCode::E0102 | ErrorCode::E0103 |
            ErrorCode::E0104 | ErrorCode::E0105 | ErrorCode::E0106 | ErrorCode::E0107 |
            ErrorCode::E0108 | ErrorCode::E0109 | ErrorCode::E0110 | ErrorCode::E0111 |
            ErrorCode::E0112 | ErrorCode::E0113 | ErrorCode::E0114 | ErrorCode::E0115 => "Type",
            
            ErrorCode::E0200 | ErrorCode::E0201 | ErrorCode::E0202 | ErrorCode::E0203 |
            ErrorCode::E0204 | ErrorCode::E0205 | ErrorCode::E0206 | ErrorCode::E0207 |
            ErrorCode::E0208 | ErrorCode::E0209 | ErrorCode::E0210 => "Compilation",
            
            ErrorCode::E0300 | ErrorCode::E0301 | ErrorCode::E0302 | ErrorCode::E0303 |
            ErrorCode::E0304 | ErrorCode::E0305 | ErrorCode::E0306 | ErrorCode::E0307 |
            ErrorCode::E0308 | ErrorCode::E0309 => "Runtime",
            
            ErrorCode::E0400 | ErrorCode::E0401 | ErrorCode::E0402 | ErrorCode::E0403 |
            ErrorCode::E0404 | ErrorCode::E0405 | ErrorCode::E0406 | ErrorCode::E0407 |
            ErrorCode::E0408 | ErrorCode::E0409 => "Security",
            
            ErrorCode::E0500 | ErrorCode::E0501 | ErrorCode::E0502 | ErrorCode::E0503 |
            ErrorCode::E0504 | ErrorCode::E0505 | ErrorCode::E0506 | ErrorCode::E0507 |
            ErrorCode::E0508 | ErrorCode::E0509 => "I/O",
        }
    }
    
    /// Get the default severity for this error code
    pub fn default_severity(&self) -> ErrorSeverity {
        // For now, all error codes are actual errors
        ErrorSeverity::Error
    }
}

impl StructuredError {
    /// Create a new structured error
    pub fn new(code: ErrorCode, message: String) -> Self {
        Self {
            code,
            message,
            location: None,
            context: Vec::new(),
            suggestions: Vec::new(),
            severity: code.default_severity(),
        }
    }
    
    /// Add source location to the error
    pub fn with_location(mut self, location: ErrorSourceLocation) -> Self {
        self.location = Some(location);
        self
    }
    
    /// Add context information
    pub fn with_context(mut self, context: Vec<String>) -> Self {
        self.context = context;
        self
    }
    
    /// Add suggestions
    pub fn with_suggestions(mut self, suggestions: Vec<String>) -> Self {
        self.suggestions = suggestions;
        self
    }
    
    /// Set severity
    pub fn with_severity(mut self, severity: ErrorSeverity) -> Self {
        self.severity = severity;
        self
    }
    
    /// Convenience constructors for common error types
    pub fn syntax_error(message: &str) -> Self {
        Self::new(ErrorCode::E0001, message.to_string())
    }
    
    pub fn type_error(message: &str) -> Self {
        Self::new(ErrorCode::E0100, message.to_string())
    }
    
    pub fn compile_error(message: &str) -> Self {
        Self::new(ErrorCode::E0200, message.to_string())
    }
    
    pub fn runtime_error(message: &str) -> Self {
        Self::new(ErrorCode::E0300, message.to_string())
    }
    
    pub fn unterminated_string(line: usize, column: usize) -> Self {
        Self::new(ErrorCode::E0002, "Unterminated string literal".to_string())
            .with_location(ErrorSourceLocation {
                file: "".to_string(),
                line,
                column,
                length: 1,
                source_line: None,
            })
            .with_suggestions(vec![
                "Add a closing quote (\") to terminate the string".to_string(),
                "Check for unescaped quotes within the string".to_string(),
            ])
    }
    
    pub fn unexpected_token(expected: &str, found: &str, line: usize, column: usize) -> Self {
        Self::new(ErrorCode::E0001, format!("Expected {}, found {}", expected, found))
            .with_location(ErrorSourceLocation {
                file: "".to_string(),
                line,
                column,
                length: found.len(),
                source_line: None,
            })
            .with_suggestions(vec![
                format!("Replace {} with {}", found, expected),
                "Check the syntax around this location".to_string(),
            ])
    }
    
    pub fn unknown_variable(name: &str, line: usize, column: usize) -> Self {
        Self::new(ErrorCode::E0109, format!("Variable '{}' not found", name))
            .with_location(ErrorSourceLocation {
                file: "".to_string(),
                line,
                column,
                length: name.len(),
                source_line: None,
            })
            .with_suggestions(vec![
                format!("Declare the variable with 'sus {} = ...'", name),
                "Check the variable name for typos".to_string(),
                "Make sure the variable is in scope".to_string(),
            ])
    }
    
    pub fn type_mismatch(expected: &str, found: &str, line: usize, column: usize) -> Self {
        Self::new(ErrorCode::E0100, format!("Type mismatch: expected {}, found {}", expected, found))
            .with_location(ErrorSourceLocation {
                file: "".to_string(),
                line,
                column,
                length: 1,
                source_line: None,
            })
            .with_suggestions(vec![
                format!("Convert {} to {}", found, expected),
                "Check the types in this expression".to_string(),
                "Consider using explicit type casting".to_string(),
            ])
    }
    
    pub fn function_not_found(name: &str, line: usize, column: usize) -> Self {
        Self::new(ErrorCode::E0110, format!("Function '{}' not found", name))
            .with_location(ErrorSourceLocation {
                file: "".to_string(),
                line,
                column,
                length: name.len(),
                source_line: None,
            })
            .with_suggestions(vec![
                format!("Define the function with 'slay {}() {{ ... }}'", name),
                "Check the function name for typos".to_string(),
                "Make sure the function is imported or in scope".to_string(),
            ])
    }
    
    pub fn unterminated_block_comment(line: usize, column: usize) -> Self {
        Self::new(ErrorCode::E0003, "Unterminated block comment".to_string())
            .with_location(ErrorSourceLocation {
                file: "".to_string(),
                line,
                column,
                length: 1,
                source_line: None,
            })
            .with_suggestions(vec![
                "Add 'on god' to close the block comment".to_string(),
                "Check for missing 'on god' in block comments".to_string(),
            ])
    }
    
    pub fn invalid_escape_sequence(sequence: &str, line: usize, column: usize) -> Self {
        Self::new(ErrorCode::E0004, format!("Invalid escape sequence: \\{}", sequence))
            .with_location(ErrorSourceLocation {
                file: "".to_string(),
                line,
                column,
                length: sequence.len() + 1,
                source_line: None,
            })
            .with_suggestions(vec![
                "Use a valid escape sequence (\\n, \\t, \\r, \\\\, \\\", \\')".to_string(),
                "Escape the backslash if literal: \\\\".to_string(),
            ])
    }
    
    pub fn missing_main_function() -> Self {
        Self::new(ErrorCode::E0204, "No main function found".to_string())
            .with_suggestions(vec![
                "Add a main function: slay main() { ... }".to_string(),
                "Ensure the main function is properly defined".to_string(),
            ])
    }
    
    pub fn import_not_found(path: &str) -> Self {
        Self::new(ErrorCode::E0200, format!("Import '{}' not found", path))
            .with_suggestions(vec![
                "Check the import path for typos".to_string(),
                "Make sure the imported module exists".to_string(),
                "Verify the module is in the correct location".to_string(),
            ])
    }
}

impl ErrorReporter {
    /// Create a new error reporter
    pub fn new() -> Self {
        Self {
            errors: Vec::new(),
            warnings: Vec::new(),
            max_errors: 100,
            colored_output: true,
            error_explanations: Self::create_error_explanations(),
        }
    }
    
    /// Set whether to use colored output
    pub fn with_colored_output(mut self, colored: bool) -> Self {
        self.colored_output = colored;
        self
    }
    
    /// Set maximum number of errors before stopping
    pub fn with_max_errors(mut self, max: usize) -> Self {
        self.max_errors = max;
        self
    }
    
    /// Add an error
    pub fn add_error(&mut self, error: StructuredError) {
        if self.errors.len() < self.max_errors {
            self.errors.push(error);
        }
    }
    
    /// Add a warning
    pub fn add_warning(&mut self, warning: StructuredError) {
        self.warnings.push(warning);
    }
    
    /// Check if there are any errors
    pub fn has_errors(&self) -> bool {
        !self.errors.is_empty()
    }
    
    /// Get the number of errors
    pub fn error_count(&self) -> usize {
        self.errors.len()
    }
    
    /// Get the number of warnings
    pub fn warning_count(&self) -> usize {
        self.warnings.len()
    }
    
    /// Print all errors and warnings
    pub fn print_all(&self) {
        for error in &self.errors {
            self.print_error(error);
        }
        
        for warning in &self.warnings {
            self.print_error(warning);
        }
        
        if self.has_errors() {
            self.print_summary();
        }
    }
    
    /// Print a single error with formatting
    pub fn print_error(&self, error: &StructuredError) {
        let severity_str = match error.severity {
            ErrorSeverity::Error => if self.colored_output { "error".red().bold() } else { "error".normal() },
            ErrorSeverity::Warning => if self.colored_output { "warning".yellow().bold() } else { "warning".normal() },
            ErrorSeverity::Note => if self.colored_output { "note".blue().bold() } else { "note".normal() },
            ErrorSeverity::Help => if self.colored_output { "help".green().bold() } else { "help".normal() },
        };
        
        let code_str = if self.colored_output {
            error.code.as_str().bright_black()
        } else {
            error.code.as_str().normal()
        };
        
        // Print main error line
        println!("{}: {}: {}", severity_str, code_str, error.message);
        
        // Print source location if available
        if let Some(location) = &error.location {
            let location_str = if self.colored_output {
                format!("{}:{}:{}", location.file, location.line, location.column).bright_black()
            } else {
                format!("{}:{}:{}", location.file, location.line, location.column).normal()
            };
            
            println!("  {} {}", "-->".bright_blue(), location_str);
            
            // Print source line with highlighting if available
            if let Some(source_line) = &location.source_line {
                println!("   {}", "|".bright_blue());
                println!("{:3} {} {}", location.line, "|".bright_blue(), source_line);
                
                // Print caret pointing to error location
                let spaces = " ".repeat(location.column.saturating_sub(1));
                let carets = if self.colored_output {
                    "^".repeat(location.length.max(1)).red()
                } else {
                    "^".repeat(location.length.max(1)).normal()
                };
                println!("   {} {}{}", "|".bright_blue(), spaces, carets);
            }
        }
        
        // Print context if available
        for context in &error.context {
            println!("  {} {}", "note:".blue(), context);
        }
        
        // Print suggestions if available
        for suggestion in &error.suggestions {
            println!("  {} {}", "help:".green(), suggestion);
        }
        
        // Print detailed explanation if available
        if let Some(explanation) = self.error_explanations.get(&error.code) {
            println!();
            println!("  {} For more information about this error, try `cursed --explain {}`", 
                     "note:".blue(), error.code.as_str());
        }
        
        println!();
    }
    
    /// Print error summary
    fn print_summary(&self) {
        let error_count = self.error_count();
        let warning_count = self.warning_count();
        
        let error_str = if error_count == 1 { "error" } else { "errors" };
        let warning_str = if warning_count == 1 { "warning" } else { "warnings" };
        
        if error_count > 0 && warning_count > 0 {
            println!("Compilation failed with {} {} and {} {}", 
                     error_count, error_str, warning_count, warning_str);
        } else if error_count > 0 {
            println!("Compilation failed with {} {}", error_count, error_str);
        } else if warning_count > 0 {
            println!("Compilation succeeded with {} {}", warning_count, warning_str);
        }
    }
    
    /// Get detailed explanation for an error code
    pub fn get_explanation(&self, code: ErrorCode) -> Option<&ErrorExplanation> {
        self.error_explanations.get(&code)
    }
    
    /// Print detailed explanation for an error code
    pub fn print_explanation(&self, code: ErrorCode) {
        if let Some(explanation) = self.get_explanation(code) {
            println!("{} {}", "Error".red().bold(), code.as_str());
            println!();
            println!("{}", explanation.title.bold());
            println!();
            println!("{}", explanation.description);
            println!();
            
            if !explanation.examples.is_empty() {
                println!("{}", "Examples:".bold());
                for example in &explanation.examples {
                    println!("  {}", example);
                }
                println!();
            }
            
            if !explanation.common_causes.is_empty() {
                println!("{}", "Common causes:".bold());
                for cause in &explanation.common_causes {
                    println!("  • {}", cause);
                }
                println!();
            }
            
            if !explanation.solutions.is_empty() {
                println!("{}", "Solutions:".bold());
                for solution in &explanation.solutions {
                    println!("  • {}", solution);
                }
                println!();
            }
        } else {
            println!("No explanation available for error code {}", code.as_str());
        }
    }
    
    /// Create error explanations database
    fn create_error_explanations() -> HashMap<ErrorCode, ErrorExplanation> {
        let mut explanations = HashMap::new();
        
        explanations.insert(ErrorCode::E0001, ErrorExplanation {
            title: "Unexpected Token".to_string(),
            description: "The parser encountered a token that was not expected at this position in the code.".to_string(),
            examples: vec![
                "Expected ')' but found 'identifier'".to_string(),
                "Expected ';' but found 'slay'".to_string(),
            ],
            common_causes: vec![
                "Missing punctuation (semicolons, commas, brackets)".to_string(),
                "Typos in keywords or identifiers".to_string(),
                "Incorrect syntax structure".to_string(),
            ],
            solutions: vec![
                "Check the syntax around the error location".to_string(),
                "Verify all brackets and parentheses are properly closed".to_string(),
                "Ensure keywords are spelled correctly".to_string(),
            ],
        });
        
        explanations.insert(ErrorCode::E0002, ErrorExplanation {
            title: "Unterminated String Literal".to_string(),
            description: "A string literal was started but never closed with a matching quote.".to_string(),
            examples: vec![
                "\"This string is missing a closing quote".to_string(),
                "`This raw string is missing a closing backtick".to_string(),
            ],
            common_causes: vec![
                "Missing closing quote".to_string(),
                "Unescaped quotes within the string".to_string(),
                "Mixing quote types (\" vs ')".to_string(),
            ],
            solutions: vec![
                "Add the matching closing quote".to_string(),
                "Escape quotes within the string (\\\"".to_string(),
                "Use raw strings with backticks for complex strings".to_string(),
            ],
        });
        
        explanations.insert(ErrorCode::E0100, ErrorExplanation {
            title: "Type Mismatch".to_string(),
            description: "A value of one type was used where a different type was expected.".to_string(),
            examples: vec![
                "Expected 'normie' (integer) but found 'tea' (string)".to_string(),
                "Cannot add integer and string".to_string(),
            ],
            common_causes: vec![
                "Using wrong type in expressions".to_string(),
                "Incorrect function arguments".to_string(),
                "Missing type conversions".to_string(),
            ],
            solutions: vec![
                "Convert the value to the expected type".to_string(),
                "Check function parameter types".to_string(),
                "Use explicit type annotations".to_string(),
            ],
        });
        
        explanations.insert(ErrorCode::E0109, ErrorExplanation {
            title: "Variable Not Found".to_string(),
            description: "A variable was used but has not been declared in the current scope.".to_string(),
            examples: vec![
                "Variable 'x' not found".to_string(),
                "Function 'calculateArea' not found".to_string(),
            ],
            common_causes: vec![
                "Typo in variable name".to_string(),
                "Variable declared in different scope".to_string(),
                "Variable used before declaration".to_string(),
            ],
            solutions: vec![
                "Declare the variable with 'sus variable_name = value'".to_string(),
                "Check spelling of variable name".to_string(),
                "Ensure variable is in scope".to_string(),
            ],
        });
        
        explanations.insert(ErrorCode::E0204, ErrorExplanation {
            title: "No Main Function".to_string(),
            description: "Every CURSED program must have a main function as the entry point.".to_string(),
            examples: vec![
                "slay main() { vibez.spill(\"Hello, world!\") }".to_string(),
            ],
            common_causes: vec![
                "Missing main function definition".to_string(),
                "Main function has wrong name or signature".to_string(),
                "Main function is not public".to_string(),
            ],
            solutions: vec![
                "Add a main function: slay main() { ... }".to_string(),
                "Check the main function name spelling".to_string(),
                "Ensure main function takes no parameters".to_string(),
            ],
        });
        
        explanations
    }
}

impl Default for ErrorReporter {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for StructuredError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.code.as_str(), self.message)
    }
}

impl std::error::Error for StructuredError {}

/// Convenience functions for creating common errors
pub fn unexpected_token(expected: &str, found: &str, line: usize, column: usize) -> StructuredError {
    StructuredError::unexpected_token(expected, found, line, column)
}

pub fn unterminated_string(line: usize, column: usize) -> StructuredError {
    StructuredError::unterminated_string(line, column)
}

pub fn unknown_variable(name: &str, line: usize, column: usize) -> StructuredError {
    StructuredError::unknown_variable(name, line, column)
}

pub fn type_mismatch(expected: &str, found: &str, line: usize, column: usize) -> StructuredError {
    StructuredError::type_mismatch(expected, found, line, column)
}

pub fn function_not_found(name: &str, line: usize, column: usize) -> StructuredError {
    StructuredError::function_not_found(name, line, column)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_error_code_formatting() {
        assert_eq!(ErrorCode::E0001.as_str(), "E0001");
        assert_eq!(ErrorCode::E0100.as_str(), "E0100");
        assert_eq!(ErrorCode::E0200.as_str(), "E0200");
    }
    
    #[test]
    fn test_error_categories() {
        assert_eq!(ErrorCode::E0001.category(), "Syntax");
        assert_eq!(ErrorCode::E0100.category(), "Type");
        assert_eq!(ErrorCode::E0200.category(), "Compilation");
    }
    
    #[test]
    fn test_structured_error_creation() {
        let error = StructuredError::syntax_error("Test error message");
        assert_eq!(error.code, ErrorCode::E0001);
        assert_eq!(error.message, "Test error message");
        assert_eq!(error.severity, ErrorSeverity::Error);
    }
    
    #[test]
    fn test_error_reporter() {
        let mut reporter = ErrorReporter::new();
        assert_eq!(reporter.error_count(), 0);
        
        reporter.add_error(StructuredError::syntax_error("Test error"));
        assert_eq!(reporter.error_count(), 1);
        assert!(reporter.has_errors());
    }
    
    #[test]
    fn test_error_with_location() {
        let error = StructuredError::unexpected_token(")", "identifier", 10, 5);
        assert!(error.location.is_some());
        let location = error.location.unwrap();
        assert_eq!(location.line, 10);
        assert_eq!(location.column, 5);
    }
    
    #[test]
    fn test_error_with_suggestions() {
        let error = StructuredError::unknown_variable("x", 5, 10);
        assert!(!error.suggestions.is_empty());
        assert!(error.suggestions.iter().any(|s| s.contains("Declare the variable")));
    }
}
