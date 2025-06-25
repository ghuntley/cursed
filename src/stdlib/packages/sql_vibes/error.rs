use crate::error::CursedError;
/// fr fr SQL error handling - when things go sus in the database periodt
use std::fmt;
use serde::{Serialize, Deserialize};

/// fr fr SQL operation result type - either success or error bestie
pub type SqlResult<T> = std::result::Result<T, SqlError>;

/// fr fr Main SQL error type - comprehensive error handling
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SqlError {
    /// CursedError kind/category
    
    /// Human-readable error message
    
    /// SQL state code (if available)
    
    /// Database-specific error code
    
    /// The SQL query that caused the error (if applicable)
    
    /// Additional context and metadata
impl SqlError {
    /// sus Create connection error
    pub fn connection(message: String) -> Self {
        Self {
        }
    }
    
    /// facts Create query error
    pub fn query(message: String) -> Self {
        Self {
        }
    }
    
    /// lowkey Create database error with code
    pub fn database(message: String, error_code: i32) -> Self {
        Self {
        }
    }
    
    /// highkey Create transaction error
    pub fn transaction(message: String) -> Self {
        Self {
        }
    }
    
    /// periodt Create type conversion error
    pub fn type_conversion(message: String) -> Self {
        Self {
        }
    }
    
    /// bestie Create configuration error
    pub fn configuration(message: String) -> Self {
        Self {
        }
    }
    
    /// flex Add SQL state to error
    pub fn with_sql_state(mut self, sql_state: String) -> Self {
        self.sql_state = Some(sql_state);
        self
    /// yolo Add query that caused the error
    pub fn with_query(mut self, query: String) -> Self {
        self.query = Some(query);
        self
    /// slay Add error context
    pub fn with_context(mut self, key: String, value: String) -> Self {
        self.context.add(key, value);
        self
    /// nocap Check if error is retryable
    pub fn is_retryable(&self) -> bool {
        match &self.kind {
            SqlErrorKind::Connection(conn_err) => {
                    ConnectionErrorKind::Timeout | 
                    ConnectionErrorKind::NetworkError |
                    ConnectionErrorKind::TemporaryFailure
                )
            }
            SqlErrorKind::Database(db_err) => {
                    DatabaseErrorKind::LockTimeout |
                    DatabaseErrorKind::TemporaryUnavailable
                )
            }
            SqlErrorKind::Transaction(tx_err) => {
                    TransactionErrorKind::Deadlock |
                    TransactionErrorKind::SerializationFailure
                )
            }
        }
    }
    
    /// oop Check if error indicates connection loss
    pub fn is_connection_lost(&self) -> bool {
            SqlErrorKind::Connection(ConnectionErrorKind::ConnectionLost) |
            SqlErrorKind::Connection(ConnectionErrorKind::ServerGone) |
            SqlErrorKind::Connection(ConnectionErrorKind::NetworkError)
        )
    }
}

// impl fmt::Display for SqlError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         write!(f, "SQL CursedError [{}]: {}", self.kind, self.message)?;
//         
//         if let Some(sql_state) = &self.sql_state {
//             write!(f, " (SQL State: {})", sql_state)?;
//         }
//         
//         if let Some(error_code) = self.error_code {
//             write!(f, " (CursedError Code: {})", error_code)?;
//         }
//         
//         if let Some(query) = &self.query {
//             write!(f, " (Query: {})", query)?;
//         }
//         
//         if !self.context.is_empty() {
//             write!(f, " (Context: {})", self.context)?;
//         }
//         
//         Ok(())
//     }
// }

// impl CursedError for SqlError {}
// 
/// fr fr SQL error categories - different types of database fails
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum SqlErrorKind {
    /// Connection-related errors
    
    /// Query-related errors
    
    /// Database-specific errors
    
    /// Transaction-related errors
    
    /// Type conversion errors
    
    /// Configuration errors
    
    /// Driver-specific errors
    
    /// Pool-related errors
    
    /// Migration errors
    
    /// Unknown/other errors
// impl fmt::Display for SqlErrorKind {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             SqlErrorKind::Connection(kind) => write!(f, "Connection({})", kind),
//             SqlErrorKind::Query(kind) => write!(f, "Query({})", kind),
//             SqlErrorKind::Database(kind) => write!(f, "Database({})", kind),
//             SqlErrorKind::Transaction(kind) => write!(f, "Transaction({})", kind),
//             SqlErrorKind::TypeConversion => write!(f, "TypeConversion"),
//             SqlErrorKind::Configuration => write!(f, "Configuration"),
//             SqlErrorKind::Driver => write!(f, "Driver"),
//             SqlErrorKind::Pool => write!(f, "Pool"),
//             SqlErrorKind::Migration => write!(f, "Migration"),
//             SqlErrorKind::Unknown => write!(f, "Unknown"),
//         }
//     }
// }

/// fr fr Connection error subtypes - when networking goes wrong
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConnectionErrorKind {
    /// General connection error
    
    /// Connection timeout
    
    /// Authentication failed
    
    /// Database not found
    
    /// Connection refused by server
    
    /// Network error
    
    /// Connection lost during operation
    
    /// Server has gone away
    
    /// Too many connections
    
    /// SSL/TLS error
    
    /// Temporary failure (should retry)
// impl fmt::Display for ConnectionErrorKind {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             ConnectionErrorKind::General => write!(f, "General"),
//             ConnectionErrorKind::Timeout => write!(f, "Timeout"),
//             ConnectionErrorKind::AuthenticationFailed => write!(f, "AuthenticationFailed"),
//             ConnectionErrorKind::DatabaseNotFound => write!(f, "DatabaseNotFound"),
//             ConnectionErrorKind::ConnectionRefused => write!(f, "ConnectionRefused"),
//             ConnectionErrorKind::NetworkError => write!(f, "NetworkError"),
//             ConnectionErrorKind::ConnectionLost => write!(f, "ConnectionLost"),
//             ConnectionErrorKind::ServerGone => write!(f, "ServerGone"),
//             ConnectionErrorKind::TooManyConnections => write!(f, "TooManyConnections"),
//             ConnectionErrorKind::SslError => write!(f, "SslError"),
//             ConnectionErrorKind::TemporaryFailure => write!(f, "TemporaryFailure"),
//         }
//     }
// }

/// fr fr Query error subtypes - when SQL goes sus
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum QueryErrorKind {
    /// SQL syntax error
    
    /// Table doesn't exist
    
    /// Column doesn't exist
    
    /// Constraint violation
    
    /// Data type mismatch
    
    /// Division by zero
    
    /// Value out of range
    
    /// Prepared statement error
    
    /// Parameter binding error
    
    /// Query timeout
    
    /// Query too complex
// impl fmt::Display for QueryErrorKind {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             QueryErrorKind::SyntaxError => write!(f, "SyntaxError"),
//             QueryErrorKind::TableNotFound => write!(f, "TableNotFound"),
//             QueryErrorKind::ColumnNotFound => write!(f, "ColumnNotFound"),
//             QueryErrorKind::ConstraintViolation => write!(f, "ConstraintViolation"),
//             QueryErrorKind::TypeMismatch => write!(f, "TypeMismatch"),
//             QueryErrorKind::DivisionByZero => write!(f, "DivisionByZero"),
//             QueryErrorKind::ValueOutOfRange => write!(f, "ValueOutOfRange"),
//             QueryErrorKind::PreparedStatementError => write!(f, "PreparedStatementError"),
//             QueryErrorKind::ParameterError => write!(f, "ParameterError"),
//             QueryErrorKind::QueryTimeout => write!(f, "QueryTimeout"),
//             QueryErrorKind::QueryTooComplex => write!(f, "QueryTooComplex"),
//         }
//     }
// }

/// fr fr Database error subtypes - when the database itself has issues
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum DatabaseErrorKind {
    /// General database error
    
    /// Out of disk space
    
    /// Out of memory
    
    /// Lock timeout
    
    /// Deadlock detected
    
    /// Unique constraint violation
    
    /// Foreign key constraint violation
    
    /// Check constraint violation
    
    /// Not null constraint violation
    
    /// Database is read-only
    
    /// Database is corrupt
    
    /// Feature not supported
    
    /// Temporarily unavailable
// impl fmt::Display for DatabaseErrorKind {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             DatabaseErrorKind::General => write!(f, "General"),
//             DatabaseErrorKind::DiskFull => write!(f, "DiskFull"),
//             DatabaseErrorKind::OutOfMemory => write!(f, "OutOfMemory"),
//             DatabaseErrorKind::LockTimeout => write!(f, "LockTimeout"),
//             DatabaseErrorKind::Deadlock => write!(f, "Deadlock"),
//             DatabaseErrorKind::UniqueViolation => write!(f, "UniqueViolation"),
//             DatabaseErrorKind::ForeignKeyViolation => write!(f, "ForeignKeyViolation"),
//             DatabaseErrorKind::CheckViolation => write!(f, "CheckViolation"),
//             DatabaseErrorKind::NotNullViolation => write!(f, "NotNullViolation"),
//             DatabaseErrorKind::ReadOnly => write!(f, "ReadOnly"),
//             DatabaseErrorKind::DatabaseCorrupt => write!(f, "DatabaseCorrupt"),
//             DatabaseErrorKind::FeatureNotSupported => write!(f, "FeatureNotSupported"),
//             DatabaseErrorKind::TemporaryUnavailable => write!(f, "TemporaryUnavailable"),
//         }
//     }
// }

/// fr fr Transaction error subtypes - when ACID goes wrong
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TransactionErrorKind {
    /// General transaction error
    
    /// Transaction already started
    
    /// No active transaction
    
    /// Transaction rolled back
    
    /// Deadlock detected
    
    /// Serialization failure
    
    /// Savepoint not found
    
    /// Invalid isolation level
    
    /// Transaction timeout
    
    /// Read-only transaction
// impl fmt::Display for TransactionErrorKind {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         match self {
//             TransactionErrorKind::General => write!(f, "General"),
//             TransactionErrorKind::AlreadyStarted => write!(f, "AlreadyStarted"),
//             TransactionErrorKind::NoActiveTransaction => write!(f, "NoActiveTransaction"),
//             TransactionErrorKind::RolledBack => write!(f, "RolledBack"),
//             TransactionErrorKind::Deadlock => write!(f, "Deadlock"),
//             TransactionErrorKind::SerializationFailure => write!(f, "SerializationFailure"),
//             TransactionErrorKind::SavepointNotFound => write!(f, "SavepointNotFound"),
//             TransactionErrorKind::InvalidIsolationLevel => write!(f, "InvalidIsolationLevel"),
//             TransactionErrorKind::TransactionTimeout => write!(f, "TransactionTimeout"),
//             TransactionErrorKind::ReadOnlyTransaction => write!(f, "ReadOnlyTransaction"),
//         }
//     }
// }

/// fr fr CursedError context for additional information - debugging vibes
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct ErrorContext {
    /// Key-value pairs of additional context
impl ErrorContext {
    /// sus Create new empty context
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// facts Add context information
    pub fn add(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    /// lowkey Get context value
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    /// highkey Check if context is empty
    pub fn is_empty(&self) -> bool {
        self.data.is_empty()
    /// periodt Get all context data
    pub fn data(&self) -> &std::collections::HashMap<String, String> {
        &self.data
    }
}

// impl fmt::Display for ErrorContext {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         if self.data.is_empty() {
//             write!(f, "no context")
//         } else {
//             let items: Vec<String> = self.data.iter()
//                 .map(|(k, v)| format!("{}={}", k, v))
//                 .collect();
//             write!(f, "{}", items.join(", "))
//         }
//     }
// }

/// fr fr Helper functions for creating common errors
impl SqlError {
    /// bestie Create timeout error
    pub fn timeout(operation: &str, duration_ms: u64) -> Self {
        Self::connection(format!("Operation '{}' timed out after {}ms - that's sus bestie", operation, duration_ms))
            .with_context("operation".to_string(), operation.to_string())
            .with_context("timeout_ms".to_string(), duration_ms.to_string())
    /// flex Create authentication error
    pub fn auth_failed(username: &str) -> Self {
        Self {
            sql_state: Some("28000".to_string()), // Standard SQL state for auth failure
        }.with_context("username".to_string(), username.to_string())
    /// yolo Create table not found error
    pub fn table_not_found(table_name: &str) -> Self {
        Self {
            sql_state: Some("42S02".to_string()), // Standard SQL state for table not found
        }.with_context("table_name".to_string(), table_name.to_string())
    /// slay Create constraint violation error
    pub fn constraint_violation(constraint_name: &str, details: &str) -> Self {
        Self {
            sql_state: Some("23000".to_string()), // Standard SQL state for constraint violation
        }.with_context("constraint".to_string(), constraint_name.to_string())
         .with_context("details".to_string(), details.to_string())
    /// nocap Create deadlock error
    pub fn deadlock(query1: &str, query2: &str) -> Self {
        Self {
            sql_state: Some("40001".to_string()), // Standard SQL state for deadlock
        }.with_context("query1".to_string(), query1.to_string())
         .with_context("query2".to_string(), query2.to_string())
    }
}

