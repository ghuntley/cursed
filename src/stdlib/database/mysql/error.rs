/// fr fr MySQL-specific error types and conversions
/// 
/// This module provides comprehensive error handling for MySQL operations,
/// mapping MySQL errors to CURSED database errors with proper context.

use std::fmt;
use crate::stdlib::database::{DatabaseError, DatabaseErrorKind, SqlStateCode};

/// fr fr MySQL-specific error type
#[derive(Debug, Clone)]
pub enum MySqlError {
    /// Connection-related errors
    Connection(String),
    /// Query execution errors
    Query(String, Option<String>), // message, query
    /// Transaction errors
    Transaction(String),
    /// Authentication errors
    Authentication(String),
    /// Constraint violation errors
    ConstraintViolation(String, Option<String>), // message, constraint_name
    /// Type conversion errors
    TypeConversion(String, String), // from_type, to_type
    /// Pool-related errors
    Pool(String),
    /// Configuration errors
    Configuration(String),
    /// Timeout errors
    Timeout(String),
    /// Data integrity errors
    DataIntegrity(String),
    /// Server errors
    Server(u16, String), // error_code, message
    /// Client errors
    Client(String),
    /// Unknown MySQL errors
    Unknown(String),
}

impl fmt::Display for MySqlError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MySqlError::Connection(msg) => write!(f, "MySQL Connection Error: {}", msg),
            MySqlError::Query(msg, query) => {
                if let Some(q) = query {
                    write!(f, "MySQL Query Error: {} (Query: {})", msg, q)
                } else {
                    write!(f, "MySQL Query Error: {}", msg)
                }
            }
            MySqlError::Transaction(msg) => write!(f, "MySQL Transaction Error: {}", msg),
            MySqlError::Authentication(msg) => write!(f, "MySQL Authentication Error: {}", msg),
            MySqlError::ConstraintViolation(msg, constraint) => {
                if let Some(c) = constraint {
                    write!(f, "MySQL Constraint Violation: {} (Constraint: {})", msg, c)
                } else {
                    write!(f, "MySQL Constraint Violation: {}", msg)
                }
            }
            MySqlError::TypeConversion(from, to) => {
                write!(f, "MySQL Type Conversion Error: Cannot convert {} to {}", from, to)
            }
            MySqlError::Pool(msg) => write!(f, "MySQL Pool Error: {}", msg),
            MySqlError::Configuration(msg) => write!(f, "MySQL Configuration Error: {}", msg),
            MySqlError::Timeout(msg) => write!(f, "MySQL Timeout Error: {}", msg),
            MySqlError::DataIntegrity(msg) => write!(f, "MySQL Data Integrity Error: {}", msg),
            MySqlError::Server(code, msg) => write!(f, "MySQL Server Error {}: {}", code, msg),
            MySqlError::Client(msg) => write!(f, "MySQL Client Error: {}", msg),
            MySqlError::Unknown(msg) => write!(f, "MySQL Unknown Error: {}", msg),
        }
    }
}

impl std::error::Error for MySqlError {}

impl MySqlError {
    /// Convert MySQL error to DatabaseError with proper categorization
    pub fn to_database_error(&self) -> DatabaseError {
        let (kind, sql_state) = match self {
            MySqlError::Connection(_) => (DatabaseErrorKind::ConnectionError, Some(SqlStateCode::ConnectionException("001".to_string()))),
            MySqlError::Query(_, _) => (DatabaseErrorKind::QueryError, Some(SqlStateCode::SyntaxErrorOrAccessRuleViolation("000".to_string()))),
            MySqlError::Transaction(_) => (DatabaseErrorKind::TransactionError, Some(SqlStateCode::InvalidTransactionState("000".to_string()))),
            MySqlError::Authentication(_) => (DatabaseErrorKind::AuthenticationError, Some(SqlStateCode::InvalidAuthorizationSpecification("000".to_string()))),
            MySqlError::ConstraintViolation(_, _) => (DatabaseErrorKind::ConstraintViolation, Some(SqlStateCode::IntegrityConstraintViolation("000".to_string()))),
            MySqlError::TypeConversion(_, _) => (DatabaseErrorKind::ConversionError, Some(SqlStateCode::DataException("000".to_string()))),
            MySqlError::Pool(_) => (DatabaseErrorKind::PoolError, None),
            MySqlError::Configuration(_) => (DatabaseErrorKind::ConfigurationError, None),
            MySqlError::Timeout(_) => (DatabaseErrorKind::TimeoutError, None),
            MySqlError::DataIntegrity(_) => (DatabaseErrorKind::DataIntegrityError, Some(SqlStateCode::DataException("001".to_string()))),
            MySqlError::Server(code, _) => {
                // Map common MySQL server error codes
                match code {
                    1045 => (DatabaseErrorKind::AuthenticationError, Some(SqlStateCode::InvalidAuthorizationSpecification("000".to_string()))),
                    1146 => (DatabaseErrorKind::SchemaError, Some(SqlStateCode::InvalidCatalogName("000".to_string()))),
                    1062 => (DatabaseErrorKind::ConstraintViolation, Some(SqlStateCode::IntegrityConstraintViolation("001".to_string()))),
                    1064 => (DatabaseErrorKind::SyntaxError, Some(SqlStateCode::SyntaxErrorOrAccessRuleViolation("001".to_string()))),
                    1205 => (DatabaseErrorKind::TransactionError, Some(SqlStateCode::TransactionRollback("001".to_string()))),
                    _ => (DatabaseErrorKind::SqlError, None),
                }
            }
            MySqlError::Client(_) => (DatabaseErrorKind::DriverError, None),
            MySqlError::Unknown(_) => (DatabaseErrorKind::Unknown, None),
        };

        let mut error = DatabaseError::new(kind, &self.to_string());
        
        if let Some(state) = sql_state {
            error.sql_state = Some(state);
        }

        // Add vendor code for server errors
        if let MySqlError::Server(code, _) = self {
            error.vendor_code = Some(*code as i32);
        }

        error
    }

    /// Create a connection error
    pub fn connection_error(msg: &str) -> Self {
        MySqlError::Connection(msg.to_string())
    }

    /// Create a query error
    pub fn query_error(msg: &str, query: Option<&str>) -> Self {
        MySqlError::Query(msg.to_string(), query.map(|s| s.to_string()))
    }

    /// Create a transaction error
    pub fn transaction_error(msg: &str) -> Self {
        MySqlError::Transaction(msg.to_string())
    }

    /// Create an authentication error
    pub fn authentication_error(msg: &str) -> Self {
        MySqlError::Authentication(msg.to_string())
    }

    /// Create a constraint violation error
    pub fn constraint_violation(msg: &str, constraint: Option<&str>) -> Self {
        MySqlError::ConstraintViolation(msg.to_string(), constraint.map(|s| s.to_string()))
    }

    /// Create a type conversion error
    pub fn type_conversion_error(from: &str, to: &str) -> Self {
        MySqlError::TypeConversion(from.to_string(), to.to_string())
    }

    /// Create a pool error
    pub fn pool_error(msg: &str) -> Self {
        MySqlError::Pool(msg.to_string())
    }

    /// Create a configuration error
    pub fn configuration_error(msg: &str) -> Self {
        MySqlError::Configuration(msg.to_string())
    }

    /// Create a timeout error
    pub fn timeout_error(msg: &str) -> Self {
        MySqlError::Timeout(msg.to_string())
    }

    /// Create a server error
    pub fn server_error(code: u16, msg: &str) -> Self {
        MySqlError::Server(code, msg.to_string())
    }

    /// Create a client error
    pub fn client_error(msg: &str) -> Self {
        MySqlError::Client(msg.to_string())
    }
    
    /// Helper methods for production driver compatibility
    pub fn connection(msg: String) -> Self {
        MySqlError::Connection(msg)
    }
    
    pub fn query(msg: String) -> Self {
        MySqlError::Query(msg, None)
    }
    
    pub fn transaction(msg: String) -> Self {
        MySqlError::Transaction(msg)
    }
    
    pub fn type_conversion(msg: String) -> Self {
        MySqlError::TypeConversion("unknown".to_string(), msg)
    }
    
    pub fn configuration(msg: String) -> Self {
        MySqlError::Configuration(msg)
    }
    
    pub fn internal(msg: String) -> Self {
        MySqlError::Unknown(msg)
    }
    
    pub fn validation(msg: String) -> Self {
        MySqlError::Client(format!("Validation Error: {}", msg))
    }
}

/// Result type for MySQL operations
pub type MySqlResult<(), Error>;

/// Convert mysql crate errors to MySqlError
impl From<mysql::Error> for MySqlError {
    fn from(err: mysql::Error) -> Self {
        match err {
            mysql::Error::Io(io_err) => {
                MySqlError::Connection(format!("IO Error: {}", io_err))
            }
            mysql::Error::Driver(driver_err) => {
                MySqlError::Client(format!("Driver Error: {:?}", driver_err))
            }
            mysql::Error::Server(server_err) => {
                MySqlError::Server(
                    server_err.code,
                    format!("Server Error: {} (State: {})", server_err.message, server_err.state)
                )
            }
            mysql::Error::Url(url_err) => {
                MySqlError::Configuration(format!("URL Error: {}", url_err))
            }
            mysql::Error::FromValue { value, err } => {
                MySqlError::TypeConversion(
                    format!("{:?}", value),
                    format!("target type: {}", err)
                )
            }
            mysql::Error::FromRow { row } => {
                MySqlError::TypeConversion(
                    "row".to_string(),
                    format!("target struct: {:?}", row)
                )
            }
            mysql::Error::General(other) => {
                MySqlError::Unknown(format!("Other Error: {}", other))
            }
            _ => MySqlError::Unknown(format!("Unknown MySQL Error: {:?}", err)),
        }
    }
}

/// Convert MySqlError to DatabaseError
impl From<MySqlError> for DatabaseError {
    fn from(err: MySqlError) -> Self {
        err.to_database_error()
    }
}

/// Helper macro for creating MySQL errors with location
#[macro_export]
macro_rules! mysql_error {
    ($error_type:ident, $msg:expr) => {
        MySqlError::$error_type($msg.to_string())
    };
    ($error_type:ident, $msg:expr, $($arg:tt)*) => {
        MySqlError::$error_type(format!($msg, $($arg)*))
    };
}

/// Helper macro for returning MySQL errors
#[macro_export]
macro_rules! mysql_bail {
    ($error_type:ident, $msg:expr) => {
        return Err(mysql_error!($error_type, $msg))
    };
    ($error_type:ident, $msg:expr, $($arg:tt)*) => {
        return Err(mysql_error!($error_type, $msg, $($arg)*))
    };
}
