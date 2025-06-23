//! Enhanced error hierarchy for CURSED
//!
//! This module provides a comprehensive error type system with categorization,
//! severity levels, and error chain management.

use crate::error::CursedError;
use crate::debug::source_location::SourceLocation;
use std::collections::HashMap;
use std::fmt;
use std::sync::{Arc, Mutex};
use std::time::{SystemTime, UNIX_EPOCH};

/// Base trait for all CURSED errors
pub trait CursedErrorTrait: fmt::Display + fmt::Debug + Send + Sync {
    /// Get the error category
    fn category(&self) -> ErrorCategory;
    
    /// Get the error severity
    fn severity(&self) -> ErrorSeverity;
    
    /// Get the error code
    fn error_code(&self) -> &str;
    
    /// Get the error message
    fn message(&self) -> &str;
    
    /// Get the source location if available
    fn source_location(&self) -> Option<&SourceLocation>;
    
    /// Get the underlying cause if any
    fn cause(&self) -> Option<&dyn CursedErrorTrait>;
    
    /// Convert to CursedError
    fn to_cursed_error(&self) -> CursedError;
    
    /// Clone this error into a boxed trait object
    fn clone_box(&self) -> Box<dyn CursedErrorTrait>;
    
    /// Check if this error can be recovered from
    fn is_recoverable(&self) -> bool {
        self.severity() != ErrorSeverity::Fatal
    }
    
    /// Check if this error should trigger a panic
    fn should_panic(&self) -> bool {
        matches!(self.severity(), ErrorSeverity::Fatal | ErrorSeverity::Critical)
    }
}

/// Implement Clone for boxed CursedErrorTrait trait objects
impl Clone for Box<dyn CursedErrorTrait> {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

/// Error categories for organization
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ErrorCategory {
    /// Parsing and syntax errors
    Parse,
    /// Type system errors
    Type,
    /// Runtime execution errors
    Runtime,
    /// I/O and filesystem errors
    Io,
    /// Compilation errors
    Compilation,
    /// Memory management errors
    Memory,
    /// Concurrency and goroutine errors
    Concurrency,
    /// Network and communication errors
    Network,
    /// Database and storage errors
    Database,
    /// Template and rendering errors
    Template,
    /// Package and dependency errors
    Package,
    /// REPL and interactive errors
    Repl,
    /// Panic and recovery errors
    Panic,
    /// External library errors
    External,
    /// User input errors
    User,
    /// System and environment errors
    System,
    /// Security and authentication errors
    Security,
    /// Configuration errors
    Configuration,
}

impl ErrorCategory {
    /// Get human-readable category name
    pub fn name(&self) -> &'static str {
        match self {
            ErrorCategory::Parse => "Parse",
            ErrorCategory::Type => "Type",
            ErrorCategory::Runtime => "Runtime",
            ErrorCategory::Io => "I/O",
            ErrorCategory::Compilation => "Compilation",
            ErrorCategory::Memory => "Memory",
            ErrorCategory::Concurrency => "Concurrency",
            ErrorCategory::Network => "Network",
            ErrorCategory::Database => "Database",
            ErrorCategory::Template => "Template",
            ErrorCategory::Package => "Package",
            ErrorCategory::Repl => "REPL",
            ErrorCategory::Panic => "Panic",
            ErrorCategory::External => "External",
            ErrorCategory::User => "User",
            ErrorCategory::System => "System",
            ErrorCategory::Security => "Security",
            ErrorCategory::Configuration => "Configuration",
        }
    }

    /// Get category color for display
    pub fn color_code(&self) -> &'static str {
        match self {
            ErrorCategory::Parse => "\x1b[31m",         // Red
            ErrorCategory::Type => "\x1b[35m",          // Magenta
            ErrorCategory::Runtime => "\x1b[33m",       // Yellow
            ErrorCategory::Io => "\x1b[36m",            // Cyan
            ErrorCategory::Compilation => "\x1b[31m",   // Red
            ErrorCategory::Memory => "\x1b[91m",        // Bright Red
            ErrorCategory::Concurrency => "\x1b[95m",   // Bright Magenta
            ErrorCategory::Network => "\x1b[94m",       // Bright Blue
            ErrorCategory::Database => "\x1b[92m",      // Bright Green
            ErrorCategory::Template => "\x1b[96m",      // Bright Cyan
            ErrorCategory::Package => "\x1b[93m",       // Bright Yellow
            ErrorCategory::Repl => "\x1b[97m",          // Bright White
            ErrorCategory::Panic => "\x1b[91m",         // Bright Red
            ErrorCategory::External => "\x1b[90m",      // Dark Gray
            ErrorCategory::User => "\x1b[34m",          // Blue
            ErrorCategory::System => "\x1b[37m",        // Light Gray
            ErrorCategory::Security => "\x1b[91m",      // Bright Red
            ErrorCategory::Configuration => "\x1b[93m", // Bright Yellow
        }
    }

    /// Reset color code
    pub const RESET: &'static str = "\x1b[0m";
}

/// Error severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ErrorSeverity {
    /// Informational message (not really an error)
    Info,
    /// Warning that doesn't stop execution
    Warning,
    /// Error that stops current operation but is recoverable
    Error,
    /// Critical error that requires immediate attention
    Critical,
    /// Fatal error that cannot be recovered from
    Fatal,
}

impl ErrorSeverity {
    /// Get human-readable severity name
    pub fn name(&self) -> &'static str {
        match self {
            ErrorSeverity::Info => "INFO",
            ErrorSeverity::Warning => "WARN",
            ErrorSeverity::Error => "ERROR",
            ErrorSeverity::Critical => "CRITICAL",
            ErrorSeverity::Fatal => "FATAL",
        }
    }

    /// Get severity color for display
    pub fn color_code(&self) -> &'static str {
        match self {
            ErrorSeverity::Info => "\x1b[37m",     // Light Gray
            ErrorSeverity::Warning => "\x1b[33m",  // Yellow
            ErrorSeverity::Error => "\x1b[31m",    // Red
            ErrorSeverity::Critical => "\x1b[91m", // Bright Red
            ErrorSeverity::Fatal => "\x1b[97;41m", // White on Red
        }
    }

    /// Check if this severity should trigger logging
    pub fn should_log(&self) -> bool {
        *self >= ErrorSeverity::Warning
    }

    /// Check if this severity should be reported to monitoring
    pub fn should_monitor(&self) -> bool {
        *self >= ErrorSeverity::Error
    }
}

/// I/O specific errors
#[derive(Debug, Clone)]
pub struct IoError {
    pub code: String,
    pub message: String,
    pub source_location: Option<SourceLocation>,
    pub io_kind: std::io::ErrorKind,
    pub file_path: Option<String>,
}

impl IoError {
    pub fn new(code: String, message: String, io_kind: std::io::ErrorKind) -> Self {
        Self {
            code,
            message,
            source_location: None,
            io_kind,
            file_path: None,
        }
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.source_location = Some(location);
        self
    }

    pub fn with_file_path(mut self, path: String) -> Self {
        self.file_path = Some(path);
        self
    }

    pub fn permission_denied(path: &str) -> Self {
        Self::new(
            "IO_PERMISSION_DENIED".to_string(),
            format!("Permission denied accessing: {}", path),
            std::io::ErrorKind::PermissionDenied,
        ).with_file_path(path.to_string())
    }

    pub fn file_not_found(path: &str) -> Self {
        Self::new(
            "IO_FILE_NOT_FOUND".to_string(),
            format!("File not found: {}", path),
            std::io::ErrorKind::NotFound,
        ).with_file_path(path.to_string())
    }

    pub fn connection_refused(address: &str) -> Self {
        Self::new(
            "IO_CONNECTION_REFUSED".to_string(),
            format!("Connection refused to: {}", address),
            std::io::ErrorKind::ConnectionRefused,
        )
    }
}

impl CursedErrorTrait for IoError {
    fn category(&self) -> ErrorCategory {
        ErrorCategory::Io
    }

    fn severity(&self) -> ErrorSeverity {
        match self.io_kind {
            std::io::ErrorKind::NotFound => ErrorSeverity::Error,
            std::io::ErrorKind::PermissionDenied => ErrorSeverity::Error,
            std::io::ErrorKind::ConnectionRefused => ErrorSeverity::Warning,
            std::io::ErrorKind::ConnectionReset => ErrorSeverity::Warning,
            std::io::ErrorKind::ConnectionAborted => ErrorSeverity::Warning,
            std::io::ErrorKind::NotConnected => ErrorSeverity::Error,
            std::io::ErrorKind::TimedOut => ErrorSeverity::Warning,
            std::io::ErrorKind::Interrupted => ErrorSeverity::Info,
            _ => ErrorSeverity::Error,
        }
    }

    fn error_code(&self) -> &str {
        &self.code
    }

    fn message(&self) -> &str {
        &self.message
    }

    fn source_location(&self) -> Option<&SourceLocation> {
        self.source_location.as_ref()
    }

    fn cause(&self) -> Option<&dyn CursedErrorTrait> {
        None
    }

    fn to_cursed_error(&self) -> CursedError {
        CursedError::Io(std::io::Error::new(self.io_kind, self.message.clone()))
    }
    
    fn clone_box(&self) -> Box<dyn CursedErrorTrait> {
        Box::new(self.clone())
    }
}

/// Cryptographic operation errors
#[derive(Debug, Clone)]
pub struct CryptoError {
    pub code: String,
    pub message: String,
    pub source_location: Option<SourceLocation>,
    pub crypto_operation: CryptoOperation,
    pub algorithm: Option<String>,
}

/// Types of cryptographic operations that can fail
#[derive(Debug, Clone, PartialEq)]
pub enum CryptoOperation {
    KeyGeneration,
    Encryption,
    Decryption,
    Signing,
    Verification,
    Hashing,
    RandomGeneration,
    KeyDerivation,
    CertificateValidation,
    Other(String),
}

impl CryptoError {
    pub fn new(code: String, message: String, operation: CryptoOperation) -> Self {
        Self {
            code,
            message,
            source_location: None,
            crypto_operation: operation,
            algorithm: None,
        }
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.source_location = Some(location);
        self
    }

    pub fn with_algorithm(mut self, algorithm: String) -> Self {
        self.algorithm = Some(algorithm);
        self
    }

    pub fn key_generation(message: &str) -> Self {
        Self::new(
            "CRYPTO_KEY_GENERATION".to_string(),
            format!("Key generation failed: {}", message),
            CryptoOperation::KeyGeneration,
        )
    }

    pub fn encryption_failed(message: &str) -> Self {
        Self::new(
            "CRYPTO_ENCRYPTION_FAILED".to_string(),
            format!("Encryption failed: {}", message),
            CryptoOperation::Encryption,
        )
    }

    pub fn decryption_failed(message: &str) -> Self {
        Self::new(
            "CRYPTO_DECRYPTION_FAILED".to_string(),
            format!("Decryption failed: {}", message),
            CryptoOperation::Decryption,
        )
    }

    pub fn invalid_key(message: &str) -> Self {
        Self::new(
            "CRYPTO_INVALID_KEY".to_string(),
            format!("Invalid key: {}", message),
            CryptoOperation::KeyGeneration,
        )
    }

    pub fn hash_error(message: &str) -> Self {
        Self::new(
            "CRYPTO_HASH_ERROR".to_string(),
            format!("Hash computation failed: {}", message),
            CryptoOperation::Hashing,
        )
    }

    pub fn random_error(message: &str) -> Self {
        Self::new(
            "CRYPTO_RANDOM_ERROR".to_string(),
            format!("Random generation failed: {}", message),
            CryptoOperation::RandomGeneration,
        )
    }

    pub fn general(message: &str) -> Self {
        Self::new(
            "CRYPTO_GENERAL".to_string(),
            format!("Cryptographic error: {}", message),
            CryptoOperation::Other("general".to_string()),
        )
    }
}

impl CursedErrorTrait for CryptoError {
    fn category(&self) -> ErrorCategory {
        ErrorCategory::Runtime  // Add Crypto category if needed
    }

    fn severity(&self) -> ErrorSeverity {
        match self.crypto_operation {
            CryptoOperation::KeyGeneration => ErrorSeverity::Error,
            CryptoOperation::Encryption => ErrorSeverity::Error,
            CryptoOperation::Decryption => ErrorSeverity::Error,
            CryptoOperation::Signing => ErrorSeverity::Error,
            CryptoOperation::Verification => ErrorSeverity::Warning,
            CryptoOperation::Hashing => ErrorSeverity::Error,
            CryptoOperation::RandomGeneration => ErrorSeverity::Critical,
            CryptoOperation::KeyDerivation => ErrorSeverity::Error,
            CryptoOperation::CertificateValidation => ErrorSeverity::Warning,
            CryptoOperation::Other(_) => ErrorSeverity::Error,
        }
    }

    fn error_code(&self) -> &str {
        &self.code
    }

    fn message(&self) -> &str {
        &self.message
    }

    fn source_location(&self) -> Option<&SourceLocation> {
        self.source_location.as_ref()
    }

    fn cause(&self) -> Option<&dyn CursedErrorTrait> {
        None
    }

    fn to_cursed_error(&self) -> CursedError {
        CursedError::Runtime(self.message.clone())
    }
    
    fn clone_box(&self) -> Box<dyn CursedErrorTrait> {
        Box::new(self.clone())
    }
}

impl fmt::Display for CryptoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}] {} (operation: {:?})", self.code, self.message, self.crypto_operation)?;
        if let Some(algorithm) = &self.algorithm {
            write!(f, " [algorithm: {}]", algorithm)?;
        }
        Ok(())
    }
}

impl std::error::Error for CryptoError {}

impl fmt::Display for IoError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "I/O Error [{}]: {}", self.code, self.message)?;
        if let Some(path) = &self.file_path {
            write!(f, " (file: {})", path)?;
        }
        if let Some(loc) = &self.source_location {
            write!(f, " at {}", loc)?;
        }
        Ok(())
    }
}

/// Parse specific errors
#[derive(Debug, Clone)]
pub struct ParseError {
    pub code: String,
    pub message: String,
    pub source_location: Option<SourceLocation>,
    pub expected: Option<String>,
    pub found: Option<String>,
    pub suggestion: Option<String>,
}

impl ParseError {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            source_location: None,
            expected: None,
            found: None,
            suggestion: None,
        }
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.source_location = Some(location);
        self
    }

    pub fn with_expected(mut self, expected: String) -> Self {
        self.expected = Some(expected);
        self
    }

    pub fn with_found(mut self, found: String) -> Self {
        self.found = Some(found);
        self
    }

    pub fn with_suggestion(mut self, suggestion: String) -> Self {
        self.suggestion = Some(suggestion);
        self
    }

    pub fn syntax_error(message: &str, line: usize, column: usize) -> Self {
        Self::new(
            "PARSE_SYNTAX_ERROR".to_string(),
            message.to_string(),
        ).with_location(SourceLocation::new(
            std::path::PathBuf::from("<unknown>"),
            line as u32,
            column as u32,
        ))
    }

    pub fn unexpected_token(expected: &str, found: &str, line: usize, column: usize) -> Self {
        Self::new(
            "PARSE_UNEXPECTED_TOKEN".to_string(),
            format!("Unexpected token"),
        )
        .with_location(SourceLocation::new(
            std::path::PathBuf::from("<unknown>"),
            line as u32,
            column as u32,
        ))
        .with_expected(expected.to_string())
        .with_found(found.to_string())
    }

    pub fn unclosed_delimiter(delimiter: &str, line: usize, column: usize) -> Self {
        Self::new(
            "PARSE_UNCLOSED_DELIMITER".to_string(),
            format!("Unclosed delimiter: {}", delimiter),
        )
        .with_location(SourceLocation::new(
            std::path::PathBuf::from("<unknown>"),
            line as u32,
            column as u32,
        ))
        .with_suggestion(format!("Add closing {}", delimiter))
    }
}

impl CursedErrorTrait for ParseError {
    fn category(&self) -> ErrorCategory {
        ErrorCategory::Parse
    }

    fn severity(&self) -> ErrorSeverity {
        ErrorSeverity::Error
    }

    fn error_code(&self) -> &str {
        &self.code
    }

    fn message(&self) -> &str {
        &self.message
    }

    fn source_location(&self) -> Option<&SourceLocation> {
        self.source_location.as_ref()
    }

    fn cause(&self) -> Option<&dyn CursedErrorTrait> {
        None
    }

    fn to_cursed_error(&self) -> CursedError {
        if let Some(loc) = &self.source_location {
            CursedError::parse_error_with_location(
                self.message.clone(),
                loc.line as usize,
                loc.column as usize,
            )
        } else {
            CursedError::Parse(self.message.clone())
        }
    }
    
    fn clone_box(&self) -> Box<dyn CursedErrorTrait> {
        Box::new(self.clone())
    }
}

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Parse Error [{}]: {}", self.code, self.message)?;
        if let Some(loc) = &self.source_location {
            write!(f, " at {}", loc)?;
        }
        if let (Some(expected), Some(found)) = (&self.expected, &self.found) {
            write!(f, " (expected: {}, found: {})", expected, found)?;
        }
        if let Some(suggestion) = &self.suggestion {
            write!(f, " (suggestion: {})", suggestion)?;
        }
        Ok(())
    }
}

/// Runtime specific errors
#[derive(Debug, Clone)]
pub struct RuntimeError {
    pub code: String,
    pub message: String,
    pub source_location: Option<SourceLocation>,
    pub runtime_context: Option<String>,
    pub stack_trace: Option<Vec<String>>,
}

impl RuntimeError {
    pub fn new(code: String, message: String) -> Self {
        Self {
            code,
            message,
            source_location: None,
            runtime_context: None,
            stack_trace: None,
        }
    }

    pub fn with_location(mut self, location: SourceLocation) -> Self {
        self.source_location = Some(location);
        self
    }

    pub fn with_context(mut self, context: String) -> Self {
        self.runtime_context = Some(context);
        self
    }

    pub fn with_stack_trace(mut self, stack_trace: Vec<String>) -> Self {
        self.stack_trace = Some(stack_trace);
        self
    }

    pub fn division_by_zero(line: usize, column: usize) -> Self {
        Self::new(
            "RUNTIME_DIVISION_BY_ZERO".to_string(),
            "Division by zero".to_string(),
        ).with_location(SourceLocation::new(
            std::path::PathBuf::from("<unknown>"),
            line as u32,
            column as u32,
        ))
    }

    pub fn null_pointer_access(line: usize, column: usize) -> Self {
        Self::new(
            "RUNTIME_NULL_POINTER".to_string(),
            "Null pointer access".to_string(),
        ).with_location(SourceLocation::new(
            std::path::PathBuf::from("<unknown>"),
            line as u32,
            column as u32,
        ))
    }

    pub fn index_out_of_bounds(index: usize, length: usize, line: usize, column: usize) -> Self {
        Self::new(
            "RUNTIME_INDEX_OUT_OF_BOUNDS".to_string(),
            format!("Index {} out of bounds for length {}", index, length),
        ).with_location(SourceLocation::new(
            std::path::PathBuf::from("<unknown>"),
            line as u32,
            column as u32,
        ))
    }

    pub fn type_assertion_failed(expected: &str, actual: &str, line: usize, column: usize) -> Self {
        Self::new(
            "RUNTIME_TYPE_ASSERTION_FAILED".to_string(),
            format!("Type assertion failed: expected {}, got {}", expected, actual),
        ).with_location(SourceLocation::new(
            std::path::PathBuf::from("<unknown>"),
            line as u32,
            column as u32,
        ))
    }
}

impl CursedErrorTrait for RuntimeError {
    fn category(&self) -> ErrorCategory {
        ErrorCategory::Runtime
    }

    fn severity(&self) -> ErrorSeverity {
        match self.code.as_str() {
            "RUNTIME_DIVISION_BY_ZERO" => ErrorSeverity::Error,
            "RUNTIME_NULL_POINTER" => ErrorSeverity::Critical,
            "RUNTIME_INDEX_OUT_OF_BOUNDS" => ErrorSeverity::Error,
            "RUNTIME_TYPE_ASSERTION_FAILED" => ErrorSeverity::Error,
            _ => ErrorSeverity::Error,
        }
    }

    fn error_code(&self) -> &str {
        &self.code
    }

    fn message(&self) -> &str {
        &self.message
    }

    fn source_location(&self) -> Option<&SourceLocation> {
        self.source_location.as_ref()
    }

    fn cause(&self) -> Option<&dyn CursedErrorTrait> {
        None
    }

    fn to_cursed_error(&self) -> CursedError {
        CursedError::Runtime(self.message.clone())
    }
    
    fn clone_box(&self) -> Box<dyn CursedErrorTrait> {
        Box::new(self.clone())
    }
}

impl fmt::Display for RuntimeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Runtime Error [{}]: {}", self.code, self.message)?;
        if let Some(loc) = &self.source_location {
            write!(f, " at {}", loc)?;
        }
        if let Some(context) = &self.runtime_context {
            write!(f, " (context: {})", context)?;
        }
        Ok(())
    }
}

/// Error chain for tracking error propagation
#[derive(Debug)]
pub struct ErrorChain {
    pub errors: Vec<Box<dyn CursedErrorTrait>>,
    pub timestamp: SystemTime,
    pub chain_id: String,
}

impl Clone for ErrorChain {
    fn clone(&self) -> Self {
        Self {
            errors: self.errors.iter().map(|e| e.clone_box()).collect(),
            timestamp: self.timestamp,
            chain_id: self.chain_id.clone(),
        }
    }
}

impl ErrorChain {
    pub fn new() -> Self {
        let timestamp = SystemTime::now();
        let chain_id = format!(
            "chain_{}",
            timestamp.duration_since(UNIX_EPOCH)
                .unwrap_or_default()
                .as_nanos()
        );

        Self {
            errors: Vec::new(),
            timestamp,
            chain_id,
        }
    }

    pub fn add_error(&mut self, error: Box<dyn CursedErrorTrait>) {
        self.errors.push(error);
    }

    pub fn root_cause(&self) -> Option<&dyn CursedErrorTrait> {
        self.errors.first().map(|e| e.as_ref())
    }

    pub fn most_recent(&self) -> Option<&dyn CursedErrorTrait> {
        self.errors.last().map(|e| e.as_ref())
    }

    pub fn len(&self) -> usize {
        self.errors.len()
    }

    pub fn is_empty(&self) -> bool {
        self.errors.is_empty()
    }

    pub fn highest_severity(&self) -> ErrorSeverity {
        self.errors
            .iter()
            .map(|e| e.severity())
            .max()
            .unwrap_or(ErrorSeverity::Info)
    }
}

impl fmt::Display for ErrorChain {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Error Chain [{}]:", self.chain_id)?;
        for (i, error) in self.errors.iter().enumerate() {
            writeln!(f, "  {}: {}", i + 1, error)?;
        }
        Ok(())
    }
}

/// Error manager for collecting and organizing errors
#[derive(Debug)]
pub struct ErrorManager {
    pub errors: Arc<Mutex<Vec<ErrorChain>>>,
    pub categories: Arc<Mutex<HashMap<ErrorCategory, usize>>>,
    pub severities: Arc<Mutex<HashMap<ErrorSeverity, usize>>>,
    pub configuration: ErrorManagerConfig,
}

#[derive(Debug, Clone)]
pub struct ErrorManagerConfig {
    pub max_error_chains: usize,
    pub auto_cleanup: bool,
    pub severity_threshold: ErrorSeverity,
    pub enable_monitoring: bool,
    pub enable_colored_output: bool,
}

impl Default for ErrorManagerConfig {
    fn default() -> Self {
        Self {
            max_error_chains: 1000,
            auto_cleanup: true,
            severity_threshold: ErrorSeverity::Warning,
            enable_monitoring: true,
            enable_colored_output: true,
        }
    }
}

impl ErrorManager {
    pub fn new() -> Self {
        Self::with_config(ErrorManagerConfig::default())
    }

    pub fn with_config(config: ErrorManagerConfig) -> Self {
        Self {
            errors: Arc::new(Mutex::new(Vec::new())),
            categories: Arc::new(Mutex::new(HashMap::new())),
            severities: Arc::new(Mutex::new(HashMap::new())),
            configuration: config,
        }
    }

    pub fn add_error(&self, error: Box<dyn CursedErrorTrait>) -> Result<(), Error> {
        let mut errors = self.errors.lock()
            .map_err(|_| CursedError::system_error("Failed to acquire error manager lock"))?;

        // Create new chain or add to existing
        let mut chain = ErrorChain::new();
        chain.add_error(error.clone_box());
        errors.push(chain);

        // Update statistics
        self.update_category_stats(error.category())?;
        self.update_severity_stats(error.severity())?;

        // Auto-cleanup if enabled
        if self.configuration.auto_cleanup && errors.len() > self.configuration.max_error_chains {
            errors.remove(0);
        }

        Ok(())
    }

    pub fn add_error_to_chain(&self, chain_id: &str, error: Box<dyn CursedErrorTrait>) -> Result<(), Error> {
        let mut errors = self.errors.lock()
            .map_err(|_| CursedError::system_error("Failed to acquire error manager lock"))?;

        if let Some(chain) = errors.iter_mut().find(|c| c.chain_id == chain_id) {
            chain.add_error(error.clone_box());
            self.update_category_stats(error.category())?;
            self.update_severity_stats(error.severity())?;
            Ok(())
        } else {
            Err(CursedError::Runtime(format!("Error chain not found: {}", chain_id)))
        }
    }

    pub fn get_errors_by_category(&self, category: ErrorCategory) -> Result<(), Error> {
        let errors = self.errors.lock()
            .map_err(|_| CursedError::system_error("Failed to acquire error manager lock"))?;

        Ok(errors.iter()
            .filter(|chain| {
                chain.errors.iter().any(|e| e.category() == category)
            })
            .cloned()
            .collect())
    }

    pub fn get_errors_by_severity(&self, severity: ErrorSeverity) -> Result<(), Error> {
        let errors = self.errors.lock()
            .map_err(|_| CursedError::system_error("Failed to acquire error manager lock"))?;

        Ok(errors.iter()
            .filter(|chain| chain.highest_severity() >= severity)
            .cloned()
            .collect())
    }

    pub fn get_error_count_by_category(&self, category: ErrorCategory) -> Result<(), Error> {
        let categories = self.categories.lock()
            .map_err(|_| CursedError::system_error("Failed to acquire categories lock"))?;

        Ok(categories.get(&category).copied().unwrap_or(0))
    }

    pub fn get_error_count_by_severity(&self, severity: ErrorSeverity) -> Result<(), Error> {
        let severities = self.severities.lock()
            .map_err(|_| CursedError::system_error("Failed to acquire severities lock"))?;

        Ok(severities.get(&severity).copied().unwrap_or(0))
    }

    pub fn clear_errors(&self) -> Result<(), Error> {
        let mut errors = self.errors.lock()
            .map_err(|_| CursedError::system_error("Failed to acquire error manager lock"))?;
        
        let mut categories = self.categories.lock()
            .map_err(|_| CursedError::system_error("Failed to acquire categories lock"))?;
        
        let mut severities = self.severities.lock()
            .map_err(|_| CursedError::system_error("Failed to acquire severities lock"))?;

        errors.clear();
        categories.clear();
        severities.clear();

        Ok(())
    }

    pub fn get_statistics(&self) -> Result<(), Error> {
        let errors = self.errors.lock()
            .map_err(|_| CursedError::system_error("Failed to acquire error manager lock"))?;
        
        let categories = self.categories.lock()
            .map_err(|_| CursedError::system_error("Failed to acquire categories lock"))?;
        
        let severities = self.severities.lock()
            .map_err(|_| CursedError::system_error("Failed to acquire severities lock"))?;

        Ok(ErrorStatistics {
            total_errors: errors.len(),
            total_chains: errors.len(),
            categories_breakdown: categories.clone(),
            severities_breakdown: severities.clone(),
            highest_severity: errors.iter()
                .map(|chain| chain.highest_severity())
                .max()
                .unwrap_or(ErrorSeverity::Info),
        })
    }

    fn update_category_stats(&self, category: ErrorCategory) -> Result<(), Error> {
        let mut categories = self.categories.lock()
            .map_err(|_| CursedError::system_error("Failed to acquire categories lock"))?;
        
        *categories.entry(category).or_insert(0) += 1;
        Ok(())
    }

    fn update_severity_stats(&self, severity: ErrorSeverity) -> Result<(), Error> {
        let mut severities = self.severities.lock()
            .map_err(|_| CursedError::system_error("Failed to acquire severities lock"))?;
        
        *severities.entry(severity).or_insert(0) += 1;
        Ok(())
    }
}

impl Default for ErrorManager {
    fn default() -> Self {
        Self::new()
    }
}

/// Error statistics for monitoring
#[derive(Debug, Clone)]
pub struct ErrorStatistics {
    pub total_errors: usize,
    pub total_chains: usize,
    pub categories_breakdown: HashMap<ErrorCategory, usize>,
    pub severities_breakdown: HashMap<ErrorSeverity, usize>,
    pub highest_severity: ErrorSeverity,
}

impl fmt::Display for ErrorStatistics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Error Statistics:")?;
        writeln!(f, "  Total Errors: {}", self.total_errors)?;
        writeln!(f, "  Total Chains: {}", self.total_chains)?;
        writeln!(f, "  Highest Severity: {}", self.highest_severity.name())?;
        
        writeln!(f, "  Categories:")?;
        for (category, count) in &self.categories_breakdown {
            writeln!(f, "    {}: {}", category.name(), count)?;
        }
        
        writeln!(f, "  Severities:")?;
        for (severity, count) in &self.severities_breakdown {
            writeln!(f, "    {}: {}", severity.name(), count)?;
        }
        
        Ok(())
    }
}

/// Helper functions for creating common error types
pub mod error_constructors {
    use super::*;

    pub fn io_error(code: &str, message: &str, kind: std::io::ErrorKind) -> Box<dyn CursedErrorTrait> {
        Box::new(IoError::new(code.to_string(), message.to_string(), kind))
    }

    pub fn parse_error(code: &str, message: &str) -> Box<dyn CursedErrorTrait> {
        Box::new(ParseError::new(code.to_string(), message.to_string()))
    }

    pub fn runtime_error(code: &str, message: &str) -> Box<dyn CursedErrorTrait> {
        Box::new(RuntimeError::new(code.to_string(), message.to_string()))
    }

    pub fn syntax_error(message: &str, line: usize, column: usize) -> Box<dyn CursedErrorTrait> {
        Box::new(ParseError::syntax_error(message, line, column))
    }

    pub fn file_not_found(path: &str) -> Box<dyn CursedErrorTrait> {
        Box::new(IoError::file_not_found(path))
    }

    pub fn permission_denied(path: &str) -> Box<dyn CursedErrorTrait> {
        Box::new(IoError::permission_denied(path))
    }

    pub fn division_by_zero(line: usize, column: usize) -> Box<dyn CursedErrorTrait> {
        Box::new(RuntimeError::division_by_zero(line, column))
    }

    pub fn type_assertion_failed(expected: &str, actual: &str, line: usize, column: usize) -> Box<dyn CursedErrorTrait> {
        Box::new(RuntimeError::type_assertion_failed(expected, actual, line, column))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_categories() {
        assert_eq!(ErrorCategory::Parse.name(), "Parse");
        assert_eq!(ErrorCategory::Runtime.name(), "Runtime");
        assert_eq!(ErrorCategory::Io.name(), "I/O");
    }

    #[test]
    fn test_error_severities() {
        assert_eq!(ErrorSeverity::Error.name(), "ERROR");
        assert_eq!(ErrorSeverity::Critical.name(), "CRITICAL");
        assert_eq!(ErrorSeverity::Fatal.name(), "FATAL");
        
        assert!(ErrorSeverity::Critical > ErrorSeverity::Error);
        assert!(ErrorSeverity::Fatal > ErrorSeverity::Critical);
    }

    #[test]
    fn test_io_error() {
        let error = IoError::file_not_found("/path/to/file.txt");
        assert_eq!(error.category(), ErrorCategory::Io);
        assert_eq!(error.severity(), ErrorSeverity::Error);
        assert_eq!(error.error_code(), "IO_FILE_NOT_FOUND");
        assert!(error.is_recoverable());
        assert!(!error.should_panic());
    }

    #[test]
    fn test_parse_error() {
        let error = ParseError::syntax_error("Missing semicolon", 10, 5);
        assert_eq!(error.category(), ErrorCategory::Parse);
        assert_eq!(error.severity(), ErrorSeverity::Error);
        assert_eq!(error.error_code(), "PARSE_SYNTAX_ERROR");
        
        let location = error.source_location().unwrap();
        assert_eq!(location.line, 10);
        assert_eq!(location.column, 5);
    }

    #[test]
    fn test_runtime_error() {
        let error = RuntimeError::division_by_zero(20, 15);
        assert_eq!(error.category(), ErrorCategory::Runtime);
        assert_eq!(error.severity(), ErrorSeverity::Error);
        assert_eq!(error.error_code(), "RUNTIME_DIVISION_BY_ZERO");
    }

    #[test]
    fn test_error_chain() {
        let mut chain = ErrorChain::new();
        assert!(chain.is_empty());
        assert_eq!(chain.len(), 0);

        let error1 = error_constructors::runtime_error("TEST_ERROR_1", "First error");
        let error2 = error_constructors::parse_error("TEST_ERROR_2", "Second error");

        chain.add_error(error1);
        chain.add_error(error2);

        assert!(!chain.is_empty());
        assert_eq!(chain.len(), 2);
        
        let root = chain.root_cause().unwrap();
        assert_eq!(root.error_code(), "TEST_ERROR_1");
        
        let recent = chain.most_recent().unwrap();
        assert_eq!(recent.error_code(), "TEST_ERROR_2");
    }

    #[test]
    fn test_error_manager() {
        let manager = ErrorManager::new();
        
        let error1 = error_constructors::io_error("TEST_IO", "I/O test", std::io::ErrorKind::NotFound);
        let error2 = error_constructors::parse_error("TEST_PARSE", "Parse test");
        
        manager.add_error(error1).unwrap();
        manager.add_error(error2).unwrap();
        
        let io_count = manager.get_error_count_by_category(ErrorCategory::Io).unwrap();
        let parse_count = manager.get_error_count_by_category(ErrorCategory::Parse).unwrap();
        
        assert_eq!(io_count, 1);
        assert_eq!(parse_count, 1);
        
        let stats = manager.get_statistics().unwrap();
        assert_eq!(stats.total_errors, 2);
        assert_eq!(stats.total_chains, 2);
    }

    #[test]
    fn test_error_constructors() {
        let file_error = error_constructors::file_not_found("/missing/file.txt");
        assert_eq!(file_error.category(), ErrorCategory::Io);
        assert_eq!(file_error.error_code(), "IO_FILE_NOT_FOUND");

        let syntax_error = error_constructors::syntax_error("Invalid syntax", 5, 10);
        assert_eq!(syntax_error.category(), ErrorCategory::Parse);
        assert_eq!(syntax_error.error_code(), "PARSE_SYNTAX_ERROR");

        let div_error = error_constructors::division_by_zero(15, 20);
        assert_eq!(div_error.category(), ErrorCategory::Runtime);
        assert_eq!(div_error.error_code(), "RUNTIME_DIVISION_BY_ZERO");
    }

    #[test]
    fn test_error_display() {
        let error = IoError::file_not_found("/test/file.txt");
        let display = format!("{}", error);
        assert!(display.contains("IO_FILE_NOT_FOUND"));
        assert!(display.contains("/test/file.txt"));

        let parse_error = ParseError::unexpected_token("semicolon", "comma", 10, 5);
        let display = format!("{}", parse_error);
        assert!(display.contains("PARSE_UNEXPECTED_TOKEN"));
        assert!(display.contains("expected: semicolon"));
        assert!(display.contains("found: comma"));
    }

    #[test]
    fn test_error_severity_ordering() {
        let severities = vec![
            ErrorSeverity::Info,
            ErrorSeverity::Warning,
            ErrorSeverity::Error,
            ErrorSeverity::Critical,
            ErrorSeverity::Fatal,
        ];

        for i in 0..severities.len() {
            for j in i + 1..severities.len() {
                assert!(severities[j] > severities[i]);
            }
        }
    }

    #[test]
    fn test_error_conversion() {
        let io_error = IoError::file_not_found("/test.txt");
        let cursed_error = io_error.to_cursed_error();
        
        match cursed_error {
            CursedError::Io(ref err) => {
                assert_eq!(err.kind(), std::io::ErrorKind::NotFound);
            }
            _ => panic!("Expected IoError conversion"),
        }
    }
}
