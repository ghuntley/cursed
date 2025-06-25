use crate::error::Error;
/// fr fr SQL error handling - when things go sus in the database periodt
use std::fmt;
use std::error::Error as StdError;
use serde::{Serialize, Deserialize};

/// fr fr SQL operation result type - either success or error bestie
pub type SqlResult<T> = std::result::Result<T, SqlError>;

/// fr fr Main SQL error type - comprehensive error handling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlError {
    /// Error kind/category
    pub kind: SqlErrorKind,
    
    /// Human-readable error message
    pub message: String,
    
    /// SQL state code (if available)
    pub sql_state: Option<String>,
    
    /// Database-specific error code
    pub error_code: Option<i32>,
    
    /// The SQL query that caused the error (if applicable)
    pub query: Option<String>,
    
    /// Additional context and metadata
    pub context: ErrorContext,
}

impl SqlError {
    /// sus Create connection error
    pub fn connection(message: String) -> Self {
        Self {
            kind: SqlErrorKind::Connection(ConnectionErrorKind::General),
            message,
            sql_state: None,
            error_code: None,
            query: None,
            context: ErrorContext::default(),
        }
    }
    
    /// facts Create query error
    pub fn query(message: String) -> Self {
        Self {
            kind: SqlErrorKind::Query(QueryErrorKind::SyntaxError),
            message,
            sql_state: None,
            error_code: None,
            query: None,
            context: ErrorContext::default(),
        }
    }
    
    /// lowkey Create database error with code
    pub fn database(message: String, error_code: i32) -> Self {
        Self {
            kind: SqlErrorKind::Database(DatabaseErrorKind::General),
            message,
            sql_state: None,
            error_code: Some(error_code),
            query: None,
            context: ErrorContext::default(),
        }
    }
    
    /// highkey Create transaction error
    pub fn transaction(message: String) -> Self {
        Self {
            kind: SqlErrorKind::Transaction(TransactionErrorKind::General),
            message,
            sql_state: None,
            error_code: None,
            query: None,
            context: ErrorContext::default(),
        }
    }
    
    /// periodt Create type conversion error
    pub fn type_conversion(message: String) -> Self {
        Self {
            kind: SqlErrorKind::TypeConversion,
            message,
            sql_state: None,
            error_code: None,
            query: None,
            context: ErrorContext::default(),
        }
    }
    
    /// bestie Create configuration error
    pub fn configuration(message: String) -> Self {
        Self {
            kind: SqlErrorKind::Configuration,
            message,
            sql_state: None,
            error_code: None,
            query: None,
            context: ErrorContext::default(),
        }
    }
    
    /// flex Add SQL state to error
    pub fn with_sql_state(mut self, sql_state: String) -> Self {
        self.sql_state = Some(sql_state);
        self
    }
    
    /// yolo Add query that caused the error
    pub fn with_query(mut self, query: String) -> Self {
        self.query = Some(query);
        self
    }
    
    /// slay Add error context
    pub fn with_context(mut self, key: String, value: String) -> Self {
        self.context.add(key, value);
        self
    }
    
    /// nocap Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        match &self.kind {
            SqlErrorKind::Connection(conn_err) => {
                matches!(conn_err, 
                    ConnectionErrorKind::Timeout | 
                    ConnectionErrorKind::NetworkError |
                    ConnectionErrorKind::TemporaryFailure
                )
            }
            SqlErrorKind::Database(db_err) => {
                matches!(db_err, 
                    DatabaseErrorKind::LockTimeout |
                    DatabaseErrorKind::TemporaryUnavailable
                )
            }
            SqlErrorKind::Transaction(tx_err) => {
                matches!(tx_err, 
                    TransactionErrorKind::Deadlock |
                    TransactionErrorKind::SerializationFailure
                )
            }
            _ => false,
        }
    }
    
    /// oop Check if error indicates connection loss
    pub fn is_connection_lost(&self) -> bool {
        matches!(&self.kind, 
            SqlErrorKind::Connection(ConnectionErrorKind::ConnectionLost) |
            SqlErrorKind::Connection(ConnectionErrorKind::ServerGone) |
            SqlErrorKind::Connection(ConnectionErrorKind::NetworkError)
        )
    }
}

impl fmt::Display for SqlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SQL Error [{}]: {}", self.kind, self.message)?;
        
        if let Some(sql_state) = &self.sql_state {
            write!(f, " (SQL State: {})", sql_state)?;
        }
        
        if let Some(error_code) = self.error_code {
            write!(f, " (Error Code: {})", error_code)?;
        }
        
        if let Some(query) = &self.query {
            write!(f, " (Query: {})", query)?;
        }
        
        if !self.context.is_empty() {
            write!(f, " (Context: {})", self.context)?;
        }
        
        Ok(())
    }
}

impl Error for SqlError {}

/// fr fr SQL error categories - different types of database fails
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SqlErrorKind {
    /// Connection-related errors
    Connection(ConnectionErrorKind),
    
    /// Query-related errors
    Query(QueryErrorKind),
    
    /// Database-specific errors
    Database(DatabaseErrorKind),
    
    /// Transaction-related errors
    Transaction(TransactionErrorKind),
    
    /// Type conversion errors
    TypeConversion,
    
    /// Configuration errors
    Configuration,
    
    /// Driver-specific errors
    Driver,
    
    /// Pool-related errors
    Pool,
    
    /// Migration errors
    Migration,
    
    /// Unknown/other errors
    Unknown,
}

impl fmt::Display for SqlErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SqlErrorKind::Connection(kind) => write!(f, "Connection({})", kind),
            SqlErrorKind::Query(kind) => write!(f, "Query({})", kind),
            SqlErrorKind::Database(kind) => write!(f, "Database({})", kind),
            SqlErrorKind::Transaction(kind) => write!(f, "Transaction({})", kind),
            SqlErrorKind::TypeConversion => write!(f, "TypeConversion"),
            SqlErrorKind::Configuration => write!(f, "Configuration"),
            SqlErrorKind::Driver => write!(f, "Driver"),
            SqlErrorKind::Pool => write!(f, "Pool"),
            SqlErrorKind::Migration => write!(f, "Migration"),
            SqlErrorKind::Unknown => write!(f, "Unknown"),
        }
    }
}

/// fr fr Connection error subtypes - when networking goes wrong
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionErrorKind {
    /// General connection error
    General,
    
    /// Connection timeout
    Timeout,
    
    /// Authentication failed
    AuthenticationFailed,
    
    /// Database not found
    DatabaseNotFound,
    
    /// Connection refused by server
    ConnectionRefused,
    
    /// Network error
    NetworkError,
    
    /// Connection lost during operation
    ConnectionLost,
    
    /// Server has gone away
    ServerGone,
    
    /// Too many connections
    TooManyConnections,
    
    /// SSL/TLS error
    SslError,
    
    /// Temporary failure (should retry)
    TemporaryFailure,
}

impl fmt::Display for ConnectionErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionErrorKind::General => write!(f, "General"),
            ConnectionErrorKind::Timeout => write!(f, "Timeout"),
            ConnectionErrorKind::AuthenticationFailed => write!(f, "AuthenticationFailed"),
            ConnectionErrorKind::DatabaseNotFound => write!(f, "DatabaseNotFound"),
            ConnectionErrorKind::ConnectionRefused => write!(f, "ConnectionRefused"),
            ConnectionErrorKind::NetworkError => write!(f, "NetworkError"),
            ConnectionErrorKind::ConnectionLost => write!(f, "ConnectionLost"),
            ConnectionErrorKind::ServerGone => write!(f, "ServerGone"),
            ConnectionErrorKind::TooManyConnections => write!(f, "TooManyConnections"),
            ConnectionErrorKind::SslError => write!(f, "SslError"),
            ConnectionErrorKind::TemporaryFailure => write!(f, "TemporaryFailure"),
        }
    }
}

/// fr fr Query error subtypes - when SQL goes sus
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryErrorKind {
    /// SQL syntax error
    SyntaxError,
    
    /// Table doesn't exist
    TableNotFound,
    
    /// Column doesn't exist
    ColumnNotFound,
    
    /// Constraint violation
    ConstraintViolation,
    
    /// Data type mismatch
    TypeMismatch,
    
    /// Division by zero
    DivisionByZero,
    
    /// Value out of range
    ValueOutOfRange,
    
    /// Prepared statement error
    PreparedStatementError,
    
    /// Parameter binding error
    ParameterError,
    
    /// Query timeout
    QueryTimeout,
    
    /// Query too complex
    QueryTooComplex,
}

impl fmt::Display for QueryErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            QueryErrorKind::SyntaxError => write!(f, "SyntaxError"),
            QueryErrorKind::TableNotFound => write!(f, "TableNotFound"),
            QueryErrorKind::ColumnNotFound => write!(f, "ColumnNotFound"),
            QueryErrorKind::ConstraintViolation => write!(f, "ConstraintViolation"),
            QueryErrorKind::TypeMismatch => write!(f, "TypeMismatch"),
            QueryErrorKind::DivisionByZero => write!(f, "DivisionByZero"),
            QueryErrorKind::ValueOutOfRange => write!(f, "ValueOutOfRange"),
            QueryErrorKind::PreparedStatementError => write!(f, "PreparedStatementError"),
            QueryErrorKind::ParameterError => write!(f, "ParameterError"),
            QueryErrorKind::QueryTimeout => write!(f, "QueryTimeout"),
            QueryErrorKind::QueryTooComplex => write!(f, "QueryTooComplex"),
        }
    }
}

/// fr fr Database error subtypes - when the database itself has issues
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DatabaseErrorKind {
    /// General database error
    General,
    
    /// Out of disk space
    DiskFull,
    
    /// Out of memory
    OutOfMemory,
    
    /// Lock timeout
    LockTimeout,
    
    /// Deadlock detected
    Deadlock,
    
    /// Unique constraint violation
    UniqueViolation,
    
    /// Foreign key constraint violation
    ForeignKeyViolation,
    
    /// Check constraint violation
    CheckViolation,
    
    /// Not null constraint violation
    NotNullViolation,
    
    /// Database is read-only
    ReadOnly,
    
    /// Database is corrupt
    DatabaseCorrupt,
    
    /// Feature not supported
    FeatureNotSupported,
    
    /// Temporarily unavailable
    TemporaryUnavailable,
}

impl fmt::Display for DatabaseErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseErrorKind::General => write!(f, "General"),
            DatabaseErrorKind::DiskFull => write!(f, "DiskFull"),
            DatabaseErrorKind::OutOfMemory => write!(f, "OutOfMemory"),
            DatabaseErrorKind::LockTimeout => write!(f, "LockTimeout"),
            DatabaseErrorKind::Deadlock => write!(f, "Deadlock"),
            DatabaseErrorKind::UniqueViolation => write!(f, "UniqueViolation"),
            DatabaseErrorKind::ForeignKeyViolation => write!(f, "ForeignKeyViolation"),
            DatabaseErrorKind::CheckViolation => write!(f, "CheckViolation"),
            DatabaseErrorKind::NotNullViolation => write!(f, "NotNullViolation"),
            DatabaseErrorKind::ReadOnly => write!(f, "ReadOnly"),
            DatabaseErrorKind::DatabaseCorrupt => write!(f, "DatabaseCorrupt"),
            DatabaseErrorKind::FeatureNotSupported => write!(f, "FeatureNotSupported"),
            DatabaseErrorKind::TemporaryUnavailable => write!(f, "TemporaryUnavailable"),
        }
    }
}

/// fr fr Transaction error subtypes - when ACID goes wrong
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionErrorKind {
    /// General transaction error
    General,
    
    /// Transaction already started
    AlreadyStarted,
    
    /// No active transaction
    NoActiveTransaction,
    
    /// Transaction rolled back
    RolledBack,
    
    /// Deadlock detected
    Deadlock,
    
    /// Serialization failure
    SerializationFailure,
    
    /// Savepoint not found
    SavepointNotFound,
    
    /// Invalid isolation level
    InvalidIsolationLevel,
    
    /// Transaction timeout
    TransactionTimeout,
    
    /// Read-only transaction
    ReadOnlyTransaction,
}

impl fmt::Display for TransactionErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TransactionErrorKind::General => write!(f, "General"),
            TransactionErrorKind::AlreadyStarted => write!(f, "AlreadyStarted"),
            TransactionErrorKind::NoActiveTransaction => write!(f, "NoActiveTransaction"),
            TransactionErrorKind::RolledBack => write!(f, "RolledBack"),
            TransactionErrorKind::Deadlock => write!(f, "Deadlock"),
            TransactionErrorKind::SerializationFailure => write!(f, "SerializationFailure"),
            TransactionErrorKind::SavepointNotFound => write!(f, "SavepointNotFound"),
            TransactionErrorKind::InvalidIsolationLevel => write!(f, "InvalidIsolationLevel"),
            TransactionErrorKind::TransactionTimeout => write!(f, "TransactionTimeout"),
            TransactionErrorKind::ReadOnlyTransaction => write!(f, "ReadOnlyTransaction"),
        }
    }
}

/// fr fr Error context for additional information - debugging vibes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ErrorContext {
    /// Key-value pairs of additional context
    pub data: std::collections::HashMap<String, String>,
}

impl ErrorContext {
    /// sus Create new empty context
    pub fn new() -> Self {
        Self {
            data: std::collections::HashMap::new(),
        }
    }
    
    /// facts Add context information
    pub fn add(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }
    
    /// lowkey Get context value
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
    
    /// highkey Check if context is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    }
    
    /// periodt Get all context data
    pub fn data(&self) -> &std::collections::HashMap<String, String> {
        &self.data
    }
}

impl fmt::Display for ErrorContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.data.is_empty() {
            write!(f, "no context")
        } else {
            let items: Vec<String> = self.data.iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect();
            write!(f, "{}", items.join(", "))
        }
    }
}

/// fr fr Helper functions for creating common errors
impl SqlError {
    /// bestie Create timeout error
    pub fn timeout(operation: &str, duration_ms: u64) -> Self {
        Self::connection(format!("Operation '{}' timed out after {}ms - that's sus bestie", operation, duration_ms))
            .with_context("operation".to_string(), operation.to_string())
            .with_context("timeout_ms".to_string(), duration_ms.to_string())
    }
    
    /// flex Create authentication error
    pub fn auth_failed(username: &str) -> Self {
        Self {
            kind: SqlErrorKind::Connection(ConnectionErrorKind::AuthenticationFailed),
            message: format!("Authentication failed for user '{}' - check your credentials bestie", username),
            sql_state: Some("28000".to_string()), // Standard SQL state for auth failure
            error_code: None,
            query: None,
            context: ErrorContext::new(),
        }.with_context("username".to_string(), username.to_string())
    }
    
    /// yolo Create table not found error
    pub fn table_not_found(table_name: &str) -> Self {
        Self {
            kind: SqlErrorKind::Query(QueryErrorKind::TableNotFound),
            message: format!("Table '{}' does not exist - double check that name periodt", table_name),
            sql_state: Some("42S02".to_string()), // Standard SQL state for table not found
            error_code: None,
            query: None,
            context: ErrorContext::new(),
        }.with_context("table_name".to_string(), table_name.to_string())
    }
    
    /// slay Create constraint violation error
    pub fn constraint_violation(constraint_name: &str, details: &str) -> Self {
        Self {
            kind: SqlErrorKind::Database(DatabaseErrorKind::UniqueViolation),
            message: format!("Constraint '{}' violated: {} - data integrity is important bestie", constraint_name, details),
            sql_state: Some("23000".to_string()), // Standard SQL state for constraint violation
            error_code: None,
            query: None,
            context: ErrorContext::new(),
        }.with_context("constraint".to_string(), constraint_name.to_string())
         .with_context("details".to_string(), details.to_string())
    }
    
    /// nocap Create deadlock error
    pub fn deadlock(query1: &str, query2: &str) -> Self {
        Self {
            kind: SqlErrorKind::Transaction(TransactionErrorKind::Deadlock),
            message: "Deadlock detected between transactions - somebody gotta give way periodt".to_string(),
            sql_state: Some("40001".to_string()), // Standard SQL state for deadlock
            error_code: None,
            query: None,
            context: ErrorContext::new(),
        }.with_context("query1".to_string(), query1.to_string())
         .with_context("query2".to_string(), query2.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_error_creation() {
        let error = SqlError::connection("Connection failed".to_string());
        assert!(matches!(error.kind, SqlErrorKind::Connection(_)));
        assert_eq!(error.message, "Connection failed");
        assert!(error.sql_state.is_none());
        assert!(error.error_code.is_none());
    }

    #[test]
    fn test_sql_error_with_details() {
        let error = SqlError::query("Syntax error".to_string())
            .with_sql_state("42000".to_string())
            .with_query("SELECT * FROM invalid_table".to_string())
            .with_context("line".to_string(), "1".to_string());
        
        assert_eq!(error.sql_state, Some("42000".to_string()));
        assert_eq!(error.query, Some("SELECT * FROM invalid_table".to_string()));
        assert_eq!(error.context.get("line"), Some(&"1".to_string()));
    }

    #[test]
    fn test_error_is_retryable() {
        let timeout_error = SqlError {
            kind: SqlErrorKind::Connection(ConnectionErrorKind::Timeout),
            message: "Timeout".to_string(),
            sql_state: None,
            error_code: None,
            query: None,
            context: ErrorContext::default(),
        };
        assert!(timeout_error.is_retryable());
        
        let syntax_error = SqlError::query("Syntax error".to_string());
        assert!(!syntax_error.is_retryable());
    }

    #[test]
    fn test_error_is_connection_lost() {
        let connection_lost_error = SqlError {
            kind: SqlErrorKind::Connection(ConnectionErrorKind::ConnectionLost),
            message: "Connection lost".to_string(),
            sql_state: None,
            error_code: None,
            query: None,
            context: ErrorContext::default(),
        };
        assert!(connection_lost_error.is_connection_lost());
        
        let syntax_error = SqlError::query("Syntax error".to_string());
        assert!(!syntax_error.is_connection_lost());
    }

    #[test]
    fn test_error_context() {
        let mut context = ErrorContext::new();
        assert!(context.is_empty());
        
        context.add("key1".to_string(), "value1".to_string());
        context.add("key2".to_string(), "value2".to_string());
        
        assert!(!context.is_empty());
        assert_eq!(context.get("key1"), Some(&"value1".to_string()));
        assert_eq!(context.get("nonexistent"), None);
        assert_eq!(context.data().len(), 2);
    }

    #[test]
    fn test_helper_error_functions() {
        let timeout_error = SqlError::timeout("query_execution", 5000);
        assert!(matches!(timeout_error.kind, SqlErrorKind::Connection(_)));
        assert!(timeout_error.message.contains("timed out"));
        
        let auth_error = SqlError::auth_failed("john_doe");
        assert!(matches!(auth_error.kind, SqlErrorKind::Connection(ConnectionErrorKind::AuthenticationFailed)));
        assert_eq!(auth_error.sql_state, Some("28000".to_string()));
        
        let table_error = SqlError::table_not_found("users");
        assert!(matches!(table_error.kind, SqlErrorKind::Query(QueryErrorKind::TableNotFound)));
        assert_eq!(table_error.sql_state, Some("42S02".to_string()));
        
        let constraint_error = SqlError::constraint_violation("unique_email", "Duplicate email");
        assert!(matches!(constraint_error.kind, SqlErrorKind::Database(DatabaseErrorKind::UniqueViolation)));
        assert_eq!(constraint_error.sql_state, Some("23000".to_string()));
        
        let deadlock_error = SqlError::deadlock("SELECT * FROM table1", "UPDATE table2");
        assert!(matches!(deadlock_error.kind, SqlErrorKind::Transaction(TransactionErrorKind::Deadlock)));
        assert_eq!(deadlock_error.sql_state, Some("40001".to_string()));
    }

    #[test]
    fn test_error_display() {
        let error = SqlError::connection("Test error".to_string())
            .with_sql_state("28000".to_string())
            .with_context("test".to_string(), "value".to_string());
        
        let display_string = error.to_string();
        assert!(display_string.contains("Test error"));
        assert!(display_string.contains("28000"));
        assert!(display_string.contains("test=value"));
    }

    #[test]
    fn test_error_kind_display() {
        assert_eq!(SqlErrorKind::Connection(ConnectionErrorKind::Timeout).to_string(), "Connection(Timeout)");
        assert_eq!(SqlErrorKind::Query(QueryErrorKind::SyntaxError).to_string(), "Query(SyntaxError)");
        assert_eq!(SqlErrorKind::Database(DatabaseErrorKind::Deadlock).to_string(), "Database(Deadlock)");
        assert_eq!(SqlErrorKind::TypeConversion.to_string(), "TypeConversion");
    }
}
