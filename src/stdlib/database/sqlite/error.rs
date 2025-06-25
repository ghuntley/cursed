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
    /// Generic error
    /// Internal logic error in SQLite
    /// Access permission denied
    /// Callback routine requested an abort
    /// The database file is locked
    /// A table in the database is locked
    /// A malloc() failed
    /// Attempt to write a readonly database
    /// Operation terminated by sqlite3_interrupt()
    /// Some kind of disk I/O error occurred
    /// The database disk image is malformed
    /// Unknown opcode in sqlite3_file_control()
    /// Insertion failed because database is full
    /// Unable to open the database file
    /// Database lock protocol error
    /// (Internal Only) Database table is empty
    /// The database schema changed
    /// String or BLOB exceeds size limit
    /// Abort due to constraint violation
    /// Data type mismatch
    /// Library used incorrectly
    /// Uses OS features not supported on host
    /// Authorization denied
    /// Not used
    /// 2nd parameter to sqlite3_bind out of range
    /// File opened that is not a database file
    /// Notifications from sqlite3_log()
    /// Warnings from sqlite3_log()
    /// sqlite3_step() has another row ready
    /// sqlite3_step() has finished executing
    /// Custom error codes for driver-specific errors
impl SqliteErrorCode {
    /// slay Convert from SQLite result code
    pub fn from_result_code(code: SqliteResultCode) -> Self {
        match code {
        }
    }

    /// slay Get human-readable error description
    pub fn description(self) -> &'static str {
        match self {
            SqliteErrorCode::IoErr => "Some kind of disk I/O error occurred",
        }
    }

    /// slay Check if error is recoverable
    pub fn is_recoverable(self) -> bool {
        match self {
            SqliteErrorCode::Busy |
            SqliteErrorCode::Locked |
            SqliteErrorCode::Interrupt |
        }
    }

    /// slay Check if error indicates corruption
    pub fn is_corruption(self) -> bool {
        matches!(self, SqliteErrorCode::Corrupt | SqliteErrorCode::NotADb)
    /// slay Check if error is a constraint violation
    pub fn is_constraint_violation(self) -> bool {
        matches!(self, SqliteErrorCode::Constraint)
    /// slay Check if error is related to permissions
    pub fn is_permission_error(self) -> bool {
        matches!(self, SqliteErrorCode::Perm | SqliteErrorCode::Auth | SqliteErrorCode::ReadOnly)
    /// slay Get error severity level
    pub fn severity(self) -> ErrorSeverity {
        match self {
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
    /// fr fr CursedError message
    /// fr fr Database path (if applicable)
    /// fr fr SQL statement (if applicable)
    /// fr fr Parameter index (if applicable)
    /// fr fr Column index (if applicable)
    /// fr fr Additional context
    /// fr fr CursedError severity
    /// fr fr Underlying cause (chain of errors)
impl SqliteError {
    /// slay Create new SQLite error
    pub fn new(code: SqliteErrorCode, message: &str) -> Self {
        Self {
        }
    }

    /// slay Create error from FFI result code
    pub fn ffi_error(result_code: SqliteResultCode, message: &str) -> Self {
        let code = SqliteErrorCode::from_result_code(result_code);
        Self::new(code, message)
    /// slay Create parameter out of range error
    pub fn parameter_out_of_range(index: i32, max: i32) -> Self {
        let mut error = Self::new(
        );
        error.parameter_index = Some(index);
        error.context.insert("max_parameters".to_string(), max.to_string());
        error
    /// slay Create null pointer error
    pub fn null_pointer(message: &str) -> Self {
        Self::new(SqliteErrorCode::NullPointer, message)
    /// slay Create encoding error
    pub fn encoding_error(message: &str) -> Self {
        Self::new(SqliteErrorCode::EncodingError, message)
    /// slay Create invalid parameter error
    pub fn invalid_parameter(message: &str) -> Self {
        Self::new(SqliteErrorCode::InvalidParameter, message)
    /// slay Create connection closed error
    pub fn connection_closed() -> Self {
        Self::new(SqliteErrorCode::ConnectionClosed, "Database connection is closed")
    /// slay Create connection error
    pub fn connection(message: &str) -> Self {
        Self::new(SqliteErrorCode::ConnectionError, message)
    /// slay Create execution error  
    pub fn execution(message: &str) -> Self {
        Self::new(SqliteErrorCode::ExecutionError, message)
    /// slay Create statement not prepared error
    pub fn statement_not_prepared() -> Self {
        Self::new(SqliteErrorCode::StatementNotPrepared, "Statement is not prepared")
    /// slay Create transaction not active error
    pub fn transaction_not_active() -> Self {
        Self::new(SqliteErrorCode::TransactionNotActive, "Transaction is not active")
    /// slay Create internal error
    pub fn internal(message: &str) -> Self {
        Self::new(SqliteErrorCode::Internal, message)
    /// slay Add database path context
    pub fn with_database_path(mut self, path: &str) -> Self {
        self.database_path = Some(path.to_string());
        self
    /// slay Add SQL statement context
    pub fn with_sql_statement(mut self, sql: &str) -> Self {
        self.sql_statement = Some(sql.to_string());
        self
    /// slay Add parameter index context
    pub fn with_parameter_index(mut self, index: i32) -> Self {
        self.parameter_index = Some(index);
        self
    /// slay Add column index context
    pub fn with_column_index(mut self, index: i32) -> Self {
        self.column_index = Some(index);
        self
    /// slay Add context information
    pub fn with_context(mut self, key: &str, value: &str) -> Self {
        self.context.insert(key.to_string(), value.to_string());
        self
    /// slay Add cause (error chaining)
    pub fn with_cause(mut self, cause: SqliteError) -> Self {
        self.cause = Some(Box::new(cause));
        self
    /// slay Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        self.code.is_recoverable()
    /// slay Check if error indicates corruption
    pub fn is_corruption(&self) -> bool {
        self.code.is_corruption()
    /// slay Check if error is constraint violation
    pub fn is_constraint_violation(&self) -> bool {
        self.code.is_constraint_violation()
    /// slay Check if error is permission-related
    pub fn is_permission_error(&self) -> bool {
        self.code.is_permission_error()
    /// slay Get formatted error message with context
    pub fn formatted_message(&self) -> String {
        let mut message = format!("[{}] {}", self.code, self.message);
        
        if let Some(ref path) = self.database_path {
            message.push_str(&format!(" (database: {})", path));
        if let Some(ref sql) = self.sql_statement {
            let truncated_sql = if sql.len() > 100 {
                format!("{}...", &sql[..97])
            } else {
                sql.clone()
            message.push_str(&format!(" (SQL: {})", truncated_sql));
        if let Some(index) = self.parameter_index {
            message.push_str(&format!(" (parameter: {})", index));
        if let Some(index) = self.column_index {
            message.push_str(&format!(" (column: {})", index));
        if !self.context.is_empty() {
            let context_items: Vec<String> = self.context.iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            message.push_str(&format!(" (context: {})", context_items.join(", ")));
        if let Some(ref cause) = self.cause {
            message.push_str(&format!(" (caused by: {})", cause.formatted_message()));
        message
    /// slay Convert to generic database error
    pub fn to_database_error(&self) -> DatabaseError {
        let kind = match self.code {
        
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
impl SqliteErrorBuilder {
    /// slay Create new error builder
    pub fn new(code: SqliteErrorCode, message: &str) -> Self {
        Self {
        }
    }

    /// slay Add database path
    pub fn database_path(mut self, path: &str) -> Self {
        self.error = self.error.with_database_path(path);
        self
    /// slay Add SQL statement
    pub fn sql_statement(mut self, sql: &str) -> Self {
        self.error = self.error.with_sql_statement(sql);
        self
    /// slay Add parameter index
    pub fn parameter_index(mut self, index: i32) -> Self {
        self.error = self.error.with_parameter_index(index);
        self
    /// slay Add column index
    pub fn column_index(mut self, index: i32) -> Self {
        self.error = self.error.with_column_index(index);
        self
    /// slay Add context
    pub fn context(mut self, key: &str, value: &str) -> Self {
        self.error = self.error.with_context(key, value);
        self
    /// slay Add cause
    pub fn cause(mut self, cause: SqliteError) -> Self {
        self.error = self.error.with_cause(cause);
        self
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
    /// slay Database file is corrupted error
    pub fn database_corrupted(path: &str) -> Self {
        SqliteErrorBuilder::new(SqliteErrorCode::Corrupt, "Database file is corrupted")
            .database_path(path)
            .context("operation", "open_database")
            .build()
    /// slay SQL syntax error
    pub fn syntax_error(sql: &str, message: &str) -> Self {
        SqliteErrorBuilder::new(SqliteErrorCode::CursedError, message)
            .sql_statement(sql)
            .context("operation", "prepare_statement")
            .build()
    /// slay Parameter binding error
    pub fn binding_error(index: i32, message: &str) -> Self {
        SqliteErrorBuilder::new(SqliteErrorCode::Range, message)
            .parameter_index(index)
            .context("operation", "bind_parameter")
            .build()
    /// slay Column access error
    pub fn column_error(index: i32, message: &str) -> Self {
        SqliteErrorBuilder::new(SqliteErrorCode::Range, message)
            .column_index(index)
            .context("operation", "access_column")
            .build()
    }
}

