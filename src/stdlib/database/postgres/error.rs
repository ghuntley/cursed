//! PostgreSQL error handling implementation

use crate::error::CursedError;
use std::fmt;

/// PostgreSQL error types
#[derive(Debug, Clone)]
pub struct PostgresError {
    pub kind: PostgresErrorKind,
    pub message: String,
    pub code: Option<String>,
    pub detail: Option<String>,
    pub hint: Option<String>,
    pub position: Option<u32>,
}

/// PostgreSQL error kinds
#[derive(Debug, Clone)]
pub enum PostgresErrorKind {
    Connection,
    Authentication,
    Protocol,
    Sql,
    Transaction,
    Constraint,
    DataType,
    Timeout,
    Pool,
    Internal,
    Unknown,
}

impl PostgresError {
    /// Create a new PostgreSQL error
    pub fn new(kind: PostgresErrorKind, message: String) -> Self {
        Self {
            kind,
            message,
            code: None,
            detail: None,
            hint: None,
            position: None,
        }
    }
    
    /// Create a connection error
    pub fn connection(message: &str) -> Self {
        Self::new(PostgresErrorKind::Connection, message.to_string())
    }
    
    /// Create an authentication error
    pub fn authentication(message: &str) -> Self {
        Self::new(PostgresErrorKind::Authentication, message.to_string())
    }
    
    /// Create a protocol error
    pub fn protocol(message: &str) -> Self {
        Self::new(PostgresErrorKind::Protocol, message.to_string())
    }
    
    /// Create a SQL error
    pub fn sql(message: &str) -> Self {
        Self::new(PostgresErrorKind::Sql, message.to_string())
    }
    
    /// Create a transaction error
    pub fn transaction(message: &str) -> Self {
        Self::new(PostgresErrorKind::Transaction, message.to_string())
    }
    
    /// Create a constraint error
    pub fn constraint(message: &str) -> Self {
        Self::new(PostgresErrorKind::Constraint, message.to_string())
    }
    
    /// Create a data type error
    pub fn data_type(message: &str) -> Self {
        Self::new(PostgresErrorKind::DataType, message.to_string())
    }
    
    /// Create a timeout error
    pub fn timeout(message: &str) -> Self {
        Self::new(PostgresErrorKind::Timeout, message.to_string())
    }
    
    /// Create a pool error
    pub fn pool(message: &str) -> Self {
        Self::new(PostgresErrorKind::Pool, message.to_string())
    }
    
    /// Create an internal error
    pub fn internal(message: &str) -> Self {
        Self::new(PostgresErrorKind::Internal, message.to_string())
    }
    
    /// Set the PostgreSQL error code
    pub fn with_code(mut self, code: String) -> Self {
        self.code = Some(code);
        self
    }
    
    /// Set the error detail
    pub fn with_detail(mut self, detail: String) -> Self {
        self.detail = Some(detail);
        self
    }
    
    /// Set the error hint
    pub fn with_hint(mut self, hint: String) -> Self {
        self.hint = Some(hint);
        self
    }
    
    /// Set the error position
    pub fn with_position(mut self, position: u32) -> Self {
        self.position = Some(position);
        self
    }
    
    /// Get the error kind
    pub fn kind(&self) -> &PostgresErrorKind {
        &self.kind
    }
    
    /// Get the error message
    pub fn message(&self) -> &str {
        &self.message
    }
    
    /// Get the PostgreSQL error code
    pub fn code(&self) -> Option<&str> {
        self.code.as_deref()
    }
    
    /// Get the error detail
    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }
    
    /// Get the error hint
    pub fn hint(&self) -> Option<&str> {
        self.hint.as_deref()
    }
    
    /// Get the error position
    pub fn position(&self) -> Option<u32> {
        self.position
    }
    
    /// Check if this is a connection error
    pub fn is_connection_error(&self) -> bool {
        matches!(self.kind, PostgresErrorKind::Connection)
    }
    
    /// Check if this is an authentication error
    pub fn is_authentication_error(&self) -> bool {
        matches!(self.kind, PostgresErrorKind::Authentication)
    }
    
    /// Check if this is a SQL error
    pub fn is_sql_error(&self) -> bool {
        matches!(self.kind, PostgresErrorKind::Sql)
    }
    
    /// Check if this is a transaction error
    pub fn is_transaction_error(&self) -> bool {
        matches!(self.kind, PostgresErrorKind::Transaction)
    }
    
    /// Check if this is a constraint violation
    pub fn is_constraint_error(&self) -> bool {
        matches!(self.kind, PostgresErrorKind::Constraint)
    }
    
    /// Check if this is a timeout error
    pub fn is_timeout_error(&self) -> bool {
        matches!(self.kind, PostgresErrorKind::Timeout)
    }
    
    /// Convert to CursedError
    pub fn to_cursed_error(&self) -> CursedError {
        CursedError::runtime_error(&self.message)
    }
}

impl fmt::Display for PostgresError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PostgreSQL {:?}: {}", self.kind, self.message)?;
        
        if let Some(code) = &self.code {
            write!(f, " (code: {})", code)?;
        }
        
        if let Some(detail) = &self.detail {
            write!(f, "\nDetail: {}", detail)?;
        }
        
        if let Some(hint) = &self.hint {
            write!(f, "\nHint: {}", hint)?;
        }
        
        if let Some(position) = self.position {
            write!(f, "\nPosition: {}", position)?;
        }
        
        Ok(())
    }
}

impl std::error::Error for PostgresError {}

impl From<PostgresError> for CursedError {
    fn from(err: PostgresError) -> Self {
        err.to_cursed_error()
    }
}

impl fmt::Display for PostgresErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PostgresErrorKind::Connection => write!(f, "Connection"),
            PostgresErrorKind::Authentication => write!(f, "Authentication"),
            PostgresErrorKind::Protocol => write!(f, "Protocol"),
            PostgresErrorKind::Sql => write!(f, "SQL"),
            PostgresErrorKind::Transaction => write!(f, "Transaction"),
            PostgresErrorKind::Constraint => write!(f, "Constraint"),
            PostgresErrorKind::DataType => write!(f, "DataType"),
            PostgresErrorKind::Timeout => write!(f, "Timeout"),
            PostgresErrorKind::Pool => write!(f, "Pool"),
            PostgresErrorKind::Internal => write!(f, "Internal"),
            PostgresErrorKind::Unknown => write!(f, "Unknown"),
        }
    }
}

/// PostgreSQL error codes mapping
pub struct PostgresErrorCode;

impl PostgresErrorCode {
    // Connection errors
    pub const CONNECTION_EXCEPTION: &'static str = "08000";
    pub const CONNECTION_DOES_NOT_EXIST: &'static str = "08003";
    pub const CONNECTION_FAILURE: &'static str = "08006";
    pub const SQLCLIENT_UNABLE_TO_ESTABLISH_SQLCONNECTION: &'static str = "08001";
    pub const SQLSERVER_REJECTED_ESTABLISHMENT_OF_SQLCONNECTION: &'static str = "08004";
    
    // Authentication errors
    pub const INVALID_AUTHORIZATION_SPECIFICATION: &'static str = "28000";
    pub const INVALID_PASSWORD: &'static str = "28P01";
    
    // SQL errors
    pub const SYNTAX_ERROR: &'static str = "42601";
    pub const UNDEFINED_COLUMN: &'static str = "42703";
    pub const UNDEFINED_TABLE: &'static str = "42P01";
    pub const UNDEFINED_FUNCTION: &'static str = "42883";
    pub const DUPLICATE_COLUMN: &'static str = "42701";
    pub const DUPLICATE_TABLE: &'static str = "42P07";
    
    // Transaction errors
    pub const TRANSACTION_ABORTED: &'static str = "25P02";
    pub const IN_FAILED_SQL_TRANSACTION: &'static str = "25P02";
    pub const INVALID_TRANSACTION_STATE: &'static str = "25000";
    
    // Constraint violations
    pub const INTEGRITY_CONSTRAINT_VIOLATION: &'static str = "23000";
    pub const NOT_NULL_VIOLATION: &'static str = "23502";
    pub const FOREIGN_KEY_VIOLATION: &'static str = "23503";
    pub const UNIQUE_VIOLATION: &'static str = "23505";
    pub const CHECK_VIOLATION: &'static str = "23514";
    
    /// Get error kind from PostgreSQL error code
    pub fn kind_from_code(code: &str) -> PostgresErrorKind {
        match &code[0..2] {
            "08" => PostgresErrorKind::Connection,
            "28" => PostgresErrorKind::Authentication,
            "42" => PostgresErrorKind::Sql,
            "25" => PostgresErrorKind::Transaction,
            "23" => PostgresErrorKind::Constraint,
            _ => PostgresErrorKind::Unknown,
        }
    }
    
    /// Check if error code indicates a connection error
    pub fn is_connection_error(code: &str) -> bool {
        code.starts_with("08")
    }
    
    /// Check if error code indicates an authentication error
    pub fn is_authentication_error(code: &str) -> bool {
        code.starts_with("28")
    }
    
    /// Check if error code indicates a SQL error
    pub fn is_sql_error(code: &str) -> bool {
        code.starts_with("42")
    }
    
    /// Check if error code indicates a transaction error
    pub fn is_transaction_error(code: &str) -> bool {
        code.starts_with("25")
    }
    
    /// Check if error code indicates a constraint violation
    pub fn is_constraint_error(code: &str) -> bool {
        code.starts_with("23")
    }
}

/// Error builder for PostgreSQL errors
pub struct PostgresErrorBuilder {
    error: PostgresError,
}

impl PostgresErrorBuilder {
    /// Create a new error builder
    pub fn new(kind: PostgresErrorKind, message: &str) -> Self {
        Self {
            error: PostgresError::new(kind, message.to_string()),
        }
    }
    
    /// Set the error code
    pub fn code(mut self, code: &str) -> Self {
        self.error.code = Some(code.to_string());
        self
    }
    
    /// Set the error detail
    pub fn detail(mut self, detail: &str) -> Self {
        self.error.detail = Some(detail.to_string());
        self
    }
    
    /// Set the error hint
    pub fn hint(mut self, hint: &str) -> Self {
        self.error.hint = Some(hint.to_string());
        self
    }
    
    /// Set the error position
    pub fn position(mut self, position: u32) -> Self {
        self.error.position = Some(position);
        self
    }
    
    /// Build the error
    pub fn build(self) -> PostgresError {
        self.error
    }
}
