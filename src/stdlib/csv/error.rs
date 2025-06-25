use crate::error::CursedError;
/// CursedError handling for CSV operations
use std::fmt;
use std::io;

/// Result type for CSV operations
pub type CsvResult<T> = std::result::Result<T, CsvError>;

/// Comprehensive error types for CSV operations
#[derive(Debug, Clone)]
pub enum CsvError {
    /// I/O error during reading or writing
    Io(String),
    
    /// Parse error with detailed location information
    Parse(ParseError),
    
    /// Invalid UTF-8 sequence encountered
    InvalidUtf8(FromUtf8Error),
    
    /// Field count mismatch
    FieldCountMismatch {
        expected: usize,
        actual: usize,
        line: usize,
    },
    
    /// Invalid configuration
    InvalidConfiguration(String),
    
    /// Schema validation error
    SchemaValidation(String),
    
    /// Type conversion error
    TypeConversion {
        field: String,
        expected_type: String,
        value: String,
        line: usize,
        column: usize,
    },
    
    /// Column not found
    ColumnNotFound(String),
    
    /// Header missing or invalid
    InvalidHeader(String),
    
    /// Buffer overflow during processing
    BufferOverflow(usize),
    
    /// Timeout during operation
    Timeout(String),
    
    /// General error with message
    General(String),
}

/// Detailed parse error with location information
#[derive(Debug, Clone)]
pub struct ParseError {
    /// Line where the record starts
    pub start_line: usize,
    
    /// Line where the error occurred
    pub line: usize,
    
    /// Column where the error occurred
    pub column: usize,
    
    /// The actual error message
    pub message: String,
    
    /// The problematic input if available
    pub input: Option<String>,
}

impl ParseError {
    /// Create a new parse error
    pub fn new(start_line: usize, line: usize, column: usize, message: String) -> Self {
        Self {
            start_line,
            line,
            column,
            message,
            input: None,
        }
    }
    
    /// Create a new parse error with input context
    pub fn with_input(start_line: usize, line: usize, column: usize, message: String, input: String) -> Self {
        Self {
            start_line,
            line,
            column,
            message,
            input: Some(input),
        }
    }
    
    /// Get the error message
    pub fn error(&self) -> &str {
        &self.message
    }
    
    /// Unwrap the underlying error (compatibility with Go-style interface)
    pub fn unwrap(&self) -> &str {
        &self.message
    }
}

// impl fmt::Display for ParseError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         if let Some(input) = &self.input {
//             write!(f, "parse error at line {}, column {}: {} (input: '{}')", 
//                    self.line, self.column, self.message, input)
//         } else {
//             write!(f, "parse error at line {}, column {}: {}", 
//                    self.line, self.column, self.message)
//         }
//     }
// }

impl StdError for ParseError {}

// impl fmt::Display for CsvError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             CsvError::Io(msg) => write!(f, "I/O error: {}", msg),
//             CsvError::Parse(err) => write!(f, "{}", err),
//             CsvError::InvalidUtf8(err) => write!(f, "invalid UTF-8: {}", err),
//             CsvError::FieldCountMismatch { expected, actual, line } => {
//                 write!(f, "field count mismatch at line {}: expected {}, got {}", line, expected, actual)
//             },
//             CsvError::InvalidConfiguration(msg) => write!(f, "invalid configuration: {}", msg),
//             CsvError::SchemaValidation(msg) => write!(f, "schema validation error: {}", msg),
//             CsvError::TypeConversion { field, expected_type, value, line, column } => {
//                 write!(f, "type conversion error at line {}, column {}: cannot convert '{}' to {} for field '{}'", 
//                        line, column, value, expected_type, field)
//             },
//             CsvError::ColumnNotFound(name) => write!(f, "column not found: '{}'", name),
//             CsvError::InvalidHeader(msg) => write!(f, "invalid header: {}", msg),
//             CsvError::BufferOverflow(size) => write!(f, "buffer overflow: size {}", size),
//             CsvError::Timeout(msg) => write!(f, "timeout: {}", msg),
//             CsvError::General(msg) => write!(f, "{}", msg),
//         }
//     }
// }

impl StdError for CsvError {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        match self {
            CsvError::Parse(err) => Some(err),
            CsvError::InvalidUtf8(err) => Some(err),
            _ => None,
        }
    }
}

// impl From<std::io::Error> for CsvError {
//     fn from(err: std::io::Error) -> Self {
//         CsvError::Io(err.to_string())
//     }
// }

impl From<FromUtf8Error> for CsvError {
    fn from(err: FromUtf8Error) -> Self {
        CsvError::InvalidUtf8(err)
    }
}

impl From<ParseError> for CsvError {
    fn from(err: ParseError) -> Self {
        CsvError::Parse(err)
    }
}

/// Helper functions for creating common errors

/// Create an I/O error
pub fn io_error(message: &str) -> CsvError {
    CsvError::Io(message.to_string())
}

/// Create a parse error
pub fn parse_error(line: usize, column: usize, message: &str) -> CsvError {
    CsvError::Parse(ParseError::new(line, line, column, message.to_string()))
}

/// Create a parse error with input context
pub fn parse_error_with_input(line: usize, column: usize, message: &str, input: &str) -> CsvError {
    CsvError::Parse(ParseError::with_input(line, line, column, message.to_string(), input.to_string()))
}

/// Create a field count mismatch error
pub fn field_count_mismatch(expected: usize, actual: usize, line: usize) -> CsvError {
    CsvError::FieldCountMismatch { expected, actual, line }
}

/// Create an invalid configuration error
pub fn invalid_configuration(message: &str) -> CsvError {
    CsvError::InvalidConfiguration(message.to_string())
}

/// Create a schema validation error
pub fn schema_validation_error(message: &str) -> CsvError {
    CsvError::SchemaValidation(message.to_string())
}

/// Create a type conversion error
pub fn type_conversion_error(field: &str, expected_type: &str, value: &str, line: usize, column: usize) -> CsvError {
    CsvError::TypeConversion {
        field: field.to_string(),
        expected_type: expected_type.to_string(),
        value: value.to_string(),
        line,
        column,
    }
}

/// Create a column not found error
pub fn column_not_found(name: &str) -> CsvError {
    CsvError::ColumnNotFound(name.to_string())
}

/// Create an invalid header error
pub fn invalid_header(message: &str) -> CsvError {
    CsvError::InvalidHeader(message.to_string())
}

/// Create a buffer overflow error
pub fn buffer_overflow(size: usize) -> CsvError {
    CsvError::BufferOverflow(size)
}

/// Create a timeout error
pub fn timeout_error(message: &str) -> CsvError {
    CsvError::Timeout(message.to_string())
}

/// Create a general error
pub fn general_error(message: &str) -> CsvError {
    CsvError::General(message.to_string())
}

