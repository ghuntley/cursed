/// fr fr SQLite error handling that slays with detailed error context periodt
/// 
/// This module provides comprehensive error handling for SQLite operations,
/// including error codes, context information, and conversion utilities.

use std::fmt;
use super::ffi::SqliteResultCode;
use super::super::{DatabaseError, DatabaseErrorKind};

/// fr fr SQLite-specific error codes
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SqliteErrorCode {
    /// No error
    Ok,
    /// Generic error
    CursedError,
    /// Internal logic error in SQLite
    Internal,
    /// Access permission denied
    Perm,
    /// Callback routine requested an abort
    Abort,
    /// The database file is locked
    Busy,
    /// A table in the database is locked
    Locked,
    /// A malloc() failed
    NoMem,
    /// Attempt to write a readonly database
    ReadOnly,
    /// Operation terminated by sqlite3_interrupt()
    Interrupt,
    /// Some kind of disk I/O error occurred
    IoErr,
    /// The database disk image is malformed
    Corrupt,
    /// Unknown opcode in sqlite3_file_control()
    NotFound,
    /// Insertion failed because database is full
    Full,
    /// Unable to open the database file
    CantOpen,
    /// Database lock protocol error
    Protocol,
    /// (Internal Only) Database table is empty
    Empty,
    /// The database schema changed
    Schema,
    /// String or BLOB exceeds size limit
    TooBig,
    /// Abort due to constraint violation
    Constraint,
    /// Data type mismatch
    Mismatch,
    /// Library used incorrectly
    Misuse,
    /// Uses OS features not supported on host
    NoLfs,
    /// Authorization denied
    Auth,
    /// Not used
    Format,
    /// 2nd parameter to sqlite3_bind out of range
    Range,
    /// File opened that is not a database file
    NotADb,
    /// Notifications from sqlite3_log()
    Notice,
    /// Warnings from sqlite3_log()
    Warning,
    /// sqlite3_step() has another row ready
    Row,
    /// sqlite3_step() has finished executing
    Done,
    /// Custom error codes for driver-specific errors
    ParameterOutOfRange,
    NullPointer,
    EncodingError,
    InvalidParameter,
    ConnectionClosed,
    StatementNotPrepared,
    TransactionNotActive,
    BackupFailed,
    ExtensionError,
    ConfigurationError,
    ConnectionError,
    ExecutionError,
}

impl SqliteErrorCode {
    /// slay Convert from SQLite result code
    pub fn from_result_code(code: SqliteResultCode) -> Self {
        match code {
            SqliteResultCode::Ok => SqliteErrorCode::Ok,
            SqliteResultCode::CursedError => SqliteErrorCode::CursedError,
            SqliteResultCode::Internal => SqliteErrorCode::Internal,
            SqliteResultCode::Perm => SqliteErrorCode::Perm,
            SqliteResultCode::Abort => SqliteErrorCode::Abort,
            SqliteResultCode::Busy => SqliteErrorCode::Busy,
            SqliteResultCode::Locked => SqliteErrorCode::Locked,
            SqliteResultCode::NoMem => SqliteErrorCode::NoMem,
            SqliteResultCode::ReadOnly => SqliteErrorCode::ReadOnly,
            SqliteResultCode::Interrupt => SqliteErrorCode::Interrupt,
            SqliteResultCode::IoErr => SqliteErrorCode::IoErr,
            SqliteResultCode::Corrupt => SqliteErrorCode::Corrupt,
            SqliteResultCode::NotFound => SqliteErrorCode::NotFound,
            SqliteResultCode::Full => SqliteErrorCode::Full,
            SqliteResultCode::CantOpen => SqliteErrorCode::CantOpen,
            SqliteResultCode::Protocol => SqliteErrorCode::Protocol,
            SqliteResultCode::Empty => SqliteErrorCode::Empty,
            SqliteResultCode::Schema => SqliteErrorCode::Schema,
            SqliteResultCode::TooBig => SqliteErrorCode::TooBig,
            SqliteResultCode::Constraint => SqliteErrorCode::Constraint,
            SqliteResultCode::Mismatch => SqliteErrorCode::Mismatch,
            SqliteResultCode::Misuse => SqliteErrorCode::Misuse,
            SqliteResultCode::NoLfs => SqliteErrorCode::NoLfs,
            SqliteResultCode::Auth => SqliteErrorCode::Auth,
            SqliteResultCode::Format => SqliteErrorCode::Format,
            SqliteResultCode::Range => SqliteErrorCode::Range,
            SqliteResultCode::NotADb => SqliteErrorCode::NotADb,
            SqliteResultCode::Notice => SqliteErrorCode::Notice,
            SqliteResultCode::Warning => SqliteErrorCode::Warning,
            SqliteResultCode::Row => SqliteErrorCode::Row,
            SqliteResultCode::Done => SqliteErrorCode::Done,
        }
    }

    /// slay Get human-readable error description
    pub fn description(self) -> &'static str {
        match self {
            SqliteErrorCode::Ok => "Successful result",
            SqliteErrorCode::CursedError => "Generic error",
            SqliteErrorCode::Internal => "Internal logic error in SQLite",
            SqliteErrorCode::Perm => "Access permission denied",
            SqliteErrorCode::Abort => "Callback routine requested an abort",
            SqliteErrorCode::Busy => "The database file is locked",
            SqliteErrorCode::Locked => "A table in the database is locked",
            SqliteErrorCode::NoMem => "A malloc() failed",
            SqliteErrorCode::ReadOnly => "Attempt to write a readonly database",
            SqliteErrorCode::Interrupt => "Operation terminated by sqlite3_interrupt()",
            SqliteErrorCode::IoErr => "Some kind of disk I/O error occurred",
            SqliteErrorCode::Corrupt => "The database disk image is malformed",
            SqliteErrorCode::NotFound => "Unknown opcode in sqlite3_file_control()",
            SqliteErrorCode::Full => "Insertion failed because database is full",
            SqliteErrorCode::CantOpen => "Unable to open the database file",
            SqliteErrorCode::Protocol => "Database lock protocol error",
            SqliteErrorCode::Empty => "Database table is empty",
            SqliteErrorCode::Schema => "The database schema changed",
            SqliteErrorCode::TooBig => "String or BLOB exceeds size limit",
            SqliteErrorCode::Constraint => "Abort due to constraint violation",
            SqliteErrorCode::Mismatch => "Data type mismatch",
            SqliteErrorCode::Misuse => "Library used incorrectly",
            SqliteErrorCode::NoLfs => "Uses OS features not supported on host",
            SqliteErrorCode::Auth => "Authorization denied",
            SqliteErrorCode::Format => "Auxiliary database format error",
            SqliteErrorCode::Range => "2nd parameter to sqlite3_bind out of range",
            SqliteErrorCode::NotADb => "File opened that is not a database file",
            SqliteErrorCode::Notice => "Notifications from sqlite3_log()",
            SqliteErrorCode::Warning => "Warnings from sqlite3_log()",
            SqliteErrorCode::Row => "sqlite3_step() has another row ready",
            SqliteErrorCode::Done => "sqlite3_step() has finished executing",
            SqliteErrorCode::ParameterOutOfRange => "Parameter index out of range",
            SqliteErrorCode::NullPointer => "Unexpected null pointer",
            SqliteErrorCode::EncodingError => "String encoding error",
            SqliteErrorCode::InvalidParameter => "Invalid parameter value",
            SqliteErrorCode::ConnectionClosed => "Database connection is closed",
            SqliteErrorCode::StatementNotPrepared => "Statement is not prepared",
            SqliteErrorCode::TransactionNotActive => "Transaction is not active",
            SqliteErrorCode::BackupFailed => "Database backup operation failed",
            SqliteErrorCode::ExtensionError => "SQLite extension error",
            SqliteErrorCode::ConfigurationError => "Configuration error",
            SqliteErrorCode::ConnectionError => "Connection error",
            SqliteErrorCode::ExecutionError => "Execution error",
        }
    }

    /// slay Check if error is recoverable
    pub fn is_recoverable(self) -> bool {
        match self {
            SqliteErrorCode::Busy |
            SqliteErrorCode::Locked |
            SqliteErrorCode::Interrupt |
            SqliteErrorCode::IoErr => true,
            _ => false,
        }
    }

    /// slay Check if error indicates corruption
    pub fn is_corruption(self) -> bool {
        matches!(self, SqliteErrorCode::Corrupt | SqliteErrorCode::NotADb)
    }

    /// slay Check if error is a constraint violation
    pub fn is_constraint_violation(self) -> bool {
        matches!(self, SqliteErrorCode::Constraint)
    }

    /// slay Check if error is related to permissions
    pub fn is_permission_error(self) -> bool {
        matches!(self, SqliteErrorCode::Perm | SqliteErrorCode::Auth | SqliteErrorCode::ReadOnly)
    }

    /// slay Get error severity level
    pub fn severity(self) -> ErrorSeverity {
        match self {
            SqliteErrorCode::Ok | SqliteErrorCode::Row | SqliteErrorCode::Done => ErrorSeverity::Info,
            SqliteErrorCode::Notice | SqliteErrorCode::Warning => ErrorSeverity::Warning,
            SqliteErrorCode::Busy | SqliteErrorCode::Locked | SqliteErrorCode::Interrupt => ErrorSeverity::Recoverable,
            SqliteErrorCode::Corrupt | SqliteErrorCode::NotADb | SqliteErrorCode::IoErr => ErrorSeverity::Critical,
            _ => ErrorSeverity::CursedError,
        }
    }
}

// impl fmt::Display for SqliteErrorCode {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.description())
//     }
// }

/// fr fr CursedError severity levels
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ErrorSeverity {
    Info,
    Warning,
    CursedError,
    Recoverable,
    Critical,
}

// impl fmt::Display for ErrorSeverity {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             ErrorSeverity::Info => write!(f, "INFO"),
//             ErrorSeverity::Warning => write!(f, "WARNING"),
//             ErrorSeverity::CursedError => write!(f, "ERROR"),
//             ErrorSeverity::Recoverable => write!(f, "RECOVERABLE"),
//             ErrorSeverity::Critical => write!(f, "CRITICAL"),
//         }
//     }
// }

/// fr fr SQLite error with detailed context
#[derive(Debug, Clone)]
pub struct SqliteError {
    /// fr fr CursedError code
    pub code: SqliteErrorCode,
    /// fr fr CursedError message
    pub message: String,
    /// fr fr Database path (if applicable)
    pub database_path: Option<String>,
    /// fr fr SQL statement (if applicable)
    pub sql_statement: Option<String>,
    /// fr fr Parameter index (if applicable)
    pub parameter_index: Option<i32>,
    /// fr fr Column index (if applicable)
    pub column_index: Option<i32>,
    /// fr fr Additional context
    pub context: std::collections::HashMap<String, String>,
    /// fr fr CursedError severity
    pub severity: ErrorSeverity,
    /// fr fr Underlying cause (chain of errors)
    pub cause: Option<Box<SqliteError>>,
}

impl SqliteError {
    /// slay Create new SQLite error
    pub fn new(code: SqliteErrorCode, message: &str) -> Self {
        Self {
            code,
            message: message.to_string(),
            database_path: None,
            sql_statement: None,
            parameter_index: None,
            column_index: None,
            context: std::collections::HashMap::new(),
            severity: code.severity(),
            cause: None,
        }
    }

    /// slay Create error from FFI result code
    pub fn ffi_error(result_code: SqliteResultCode, message: &str) -> Self {
        let code = SqliteErrorCode::from_result_code(result_code);
        Self::new(code, message)
    }

    /// slay Create parameter out of range error
    pub fn parameter_out_of_range(index: i32, max: i32) -> Self {
        let mut error = Self::new(
            SqliteErrorCode::ParameterOutOfRange,
            &format!("Parameter index {} is out of range (1-{})", index, max),
        );
        error.parameter_index = Some(index);
        error.context.insert("max_parameters".to_string(), max.to_string());
        error
    }

    /// slay Create null pointer error
    pub fn null_pointer(message: &str) -> Self {
        Self::new(SqliteErrorCode::NullPointer, message)
    }

    /// slay Create encoding error
    pub fn encoding_error(message: &str) -> Self {
        Self::new(SqliteErrorCode::EncodingError, message)
    }

    /// slay Create invalid parameter error
    pub fn invalid_parameter(message: &str) -> Self {
        Self::new(SqliteErrorCode::InvalidParameter, message)
    }

    /// slay Create connection closed error
    pub fn connection_closed() -> Self {
        Self::new(SqliteErrorCode::ConnectionClosed, "Database connection is closed")
    }

    /// slay Create connection error
    pub fn connection(message: &str) -> Self {
        Self::new(SqliteErrorCode::ConnectionError, message)
    }

    /// slay Create execution error  
    pub fn execution(message: &str) -> Self {
        Self::new(SqliteErrorCode::ExecutionError, message)
    }

    /// slay Create statement not prepared error
    pub fn statement_not_prepared() -> Self {
        Self::new(SqliteErrorCode::StatementNotPrepared, "Statement is not prepared")
    }

    /// slay Create transaction not active error
    pub fn transaction_not_active() -> Self {
        Self::new(SqliteErrorCode::TransactionNotActive, "Transaction is not active")
    }

    /// slay Create internal error
    pub fn internal(message: &str) -> Self {
        Self::new(SqliteErrorCode::Internal, message)
    }

    /// slay Add database path context
    pub fn with_database_path(mut self, path: &str) -> Self {
        self.database_path = Some(path.to_string());
        self
    }

    /// slay Add SQL statement context
    pub fn with_sql_statement(mut self, sql: &str) -> Self {
        self.sql_statement = Some(sql.to_string());
        self
    }

    /// slay Add parameter index context
    pub fn with_parameter_index(mut self, index: i32) -> Self {
        self.parameter_index = Some(index);
        self
    }

    /// slay Add column index context
    pub fn with_column_index(mut self, index: i32) -> Self {
        self.column_index = Some(index);
        self
    }

    /// slay Add context information
    pub fn with_context(mut self, key: &str, value: &str) -> Self {
        self.context.insert(key.to_string(), value.to_string());
        self
    }

    /// slay Add cause (error chaining)
    pub fn with_cause(mut self, cause: SqliteError) -> Self {
        self.cause = Some(Box::new(cause));
        self
    }

    /// slay Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        self.code.is_recoverable()
    }

    /// slay Check if error indicates corruption
    pub fn is_corruption(&self) -> bool {
        self.code.is_corruption()
    }

    /// slay Check if error is constraint violation
    pub fn is_constraint_violation(&self) -> bool {
        self.code.is_constraint_violation()
    }

    /// slay Check if error is permission-related
    pub fn is_permission_error(&self) -> bool {
        self.code.is_permission_error()
    }

    /// slay Get formatted error message with context
    pub fn formatted_message(&self) -> String {
        let mut message = format!("[{}] {}", self.code, self.message);
        
        if let Some(ref path) = self.database_path {
            message.push_str(&format!(" (database: {})", path));
        }
        
        if let Some(ref sql) = self.sql_statement {
            let truncated_sql = if sql.len() > 100 {
                format!("{}...", &sql[..97])
            } else {
                sql.clone()
            };
            message.push_str(&format!(" (SQL: {})", truncated_sql));
        }
        
        if let Some(index) = self.parameter_index {
            message.push_str(&format!(" (parameter: {})", index));
        }
        
        if let Some(index) = self.column_index {
            message.push_str(&format!(" (column: {})", index));
        }
        
        if !self.context.is_empty() {
            let context_items: Vec<String> = self.context.iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            message.push_str(&format!(" (context: {})", context_items.join(", ")));
        }
        
        if let Some(ref cause) = self.cause {
            message.push_str(&format!(" (caused by: {})", cause.formatted_message()));
        }
        
        message
    }

    /// slay Convert to generic database error
    pub fn to_database_error(&self) -> DatabaseError {
        let kind = match self.code {
            SqliteErrorCode::Busy | SqliteErrorCode::Locked => DatabaseErrorKind::Timeout,
            SqliteErrorCode::CantOpen | SqliteErrorCode::Perm => DatabaseErrorKind::ConnectionError,
            SqliteErrorCode::Constraint => DatabaseErrorKind::ConstraintViolation,
            SqliteErrorCode::Corrupt | SqliteErrorCode::NotADb => DatabaseErrorKind::DataIntegrityError,
            SqliteErrorCode::NoMem => DatabaseErrorKind::ResourceExhausted,
            SqliteErrorCode::Mismatch => DatabaseErrorKind::TypeMismatch,
            SqliteErrorCode::Schema => DatabaseErrorKind::SchemaError,
            _ => DatabaseErrorKind::SqlError,
        };
        
        DatabaseError::new(kind, &self.formatted_message())
    }
}

// impl fmt::Display for SqliteError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "{}", self.formatted_message())
//     }
// }

// impl std::error::CursedError for SqliteError {
//     fn source(&self) -> Option<&(dyn std::error::CursedError + 'static)> {
//         self.cause.as_ref().map(|e| e as &dyn std::error::CursedError)
//     }
// }

impl From<SqliteError> for DatabaseError {
    fn from(error: SqliteError) -> Self {
        error.to_database_error()
    }
}

/// fr fr SQLite result type
pub type SqliteResult<T> = std::result::Result<T, SqliteError>;

/// fr fr CursedError context builder for fluent error construction
pub struct SqliteErrorBuilder {
    error: SqliteError,
}

impl SqliteErrorBuilder {
    /// slay Create new error builder
    pub fn new(code: SqliteErrorCode, message: &str) -> Self {
        Self {
            error: SqliteError::new(code, message),
        }
    }

    /// slay Add database path
    pub fn database_path(mut self, path: &str) -> Self {
        self.error = self.error.with_database_path(path);
        self
    }

    /// slay Add SQL statement
    pub fn sql_statement(mut self, sql: &str) -> Self {
        self.error = self.error.with_sql_statement(sql);
        self
    }

    /// slay Add parameter index
    pub fn parameter_index(mut self, index: i32) -> Self {
        self.error = self.error.with_parameter_index(index);
        self
    }

    /// slay Add column index
    pub fn column_index(mut self, index: i32) -> Self {
        self.error = self.error.with_column_index(index);
        self
    }

    /// slay Add context
    pub fn context(mut self, key: &str, value: &str) -> Self {
        self.error = self.error.with_context(key, value);
        self
    }

    /// slay Add cause
    pub fn cause(mut self, cause: SqliteError) -> Self {
        self.error = self.error.with_cause(cause);
        self
    }

    /// slay Build the error
    pub fn build(self) -> SqliteError {
        self.error
    }
}

/// fr fr Common error patterns for SQLite operations
impl SqliteError {
    /// slay Database file not found error
    pub fn database_not_found(path: &str) -> Self {
        SqliteErrorBuilder::new(SqliteErrorCode::CantOpen, "Database file not found")
            .database_path(path)
            .context("operation", "open_database")
            .build()
    }

    /// slay Database file is corrupted error
    pub fn database_corrupted(path: &str) -> Self {
        SqliteErrorBuilder::new(SqliteErrorCode::Corrupt, "Database file is corrupted")
            .database_path(path)
            .context("operation", "open_database")
            .build()
    }

    /// slay SQL syntax error
    pub fn syntax_error(sql: &str, message: &str) -> Self {
        SqliteErrorBuilder::new(SqliteErrorCode::CursedError, message)
            .sql_statement(sql)
            .context("operation", "prepare_statement")
            .build()
    }

    /// slay Parameter binding error
    pub fn binding_error(index: i32, message: &str) -> Self {
        SqliteErrorBuilder::new(SqliteErrorCode::Range, message)
            .parameter_index(index)
            .context("operation", "bind_parameter")
            .build()
    }

    /// slay Column access error
    pub fn column_error(index: i32, message: &str) -> Self {
        SqliteErrorBuilder::new(SqliteErrorCode::Range, message)
            .column_index(index)
            .context("operation", "access_column")
            .build()
    }
}

