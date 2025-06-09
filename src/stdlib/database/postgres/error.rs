/// PostgreSQL-specific error handling for CURSED database operations
/// 
/// This module provides comprehensive error handling for PostgreSQL operations
/// including native PostgreSQL error codes, detailed error context, and
/// integration with the CURSED error system.

use std::fmt;
use super::super::{DatabaseError, DatabaseErrorKind};
use super::ffi::ErrorDetails;

/// fr fr PostgreSQL-specific error type with detailed context
#[derive(Debug, Clone)]
pub struct PostgreSQLError {
    /// fr fr Kind of database error
    pub kind: DatabaseErrorKind,
    /// fr fr Primary error message
    pub message: String,
    /// fr fr PostgreSQL SQLSTATE code
    pub sqlstate: Option<String>,
    /// fr fr Error severity level
    pub severity: Option<String>,
    /// fr fr Detailed error message
    pub detail: Option<String>,
    /// fr fr Hint for resolving the error
    pub hint: Option<String>,
    /// fr fr Position in query where error occurred
    pub position: Option<String>,
    /// fr fr Error context information
    pub context: Option<String>,
    /// fr fr Schema name related to error
    pub schema_name: Option<String>,
    /// fr fr Table name related to error
    pub table_name: Option<String>,
    /// fr fr Column name related to error
    pub column_name: Option<String>,
    /// fr fr Constraint name related to error
    pub constraint_name: Option<String>,
}

impl PostgreSQLError {
    /// slay Create a new PostgreSQL error
    pub fn new(kind: DatabaseErrorKind, message: String) -> Self {
        Self {
            kind,
            message,
            sqlstate: None,
            severity: None,
            detail: None,
            hint: None,
            position: None,
            context: None,
            schema_name: None,
            table_name: None,
            column_name: None,
            constraint_name: None,
        }
    }
    
    /// slay Create error from PostgreSQL error details
    pub fn from_pg_error(details: ErrorDetails) -> Self {
        let message = details.message.clone().unwrap_or_else(|| "Unknown PostgreSQL error".to_string());
        let kind = Self::determine_error_kind(&details.sqlstate);
        
        Self {
            kind,
            message,
            sqlstate: details.sqlstate,
            severity: details.severity,
            detail: details.detail,
            hint: details.hint,
            position: details.position,
            context: details.context,
            schema_name: details.schema_name,
            table_name: details.table_name,
            column_name: details.column_name,
            constraint_name: details.constraint_name,
        }
    }
    
    /// slay Determine error kind from SQLSTATE code
    fn determine_error_kind(sqlstate: &Option<String>) -> DatabaseErrorKind {
        match sqlstate.as_deref() {
            Some("08000") | Some("08003") | Some("08006") | Some("08001") | Some("08004") => {
                DatabaseErrorKind::ConnectionError
            }
            Some("42000") | Some("42601") | Some("42611") | Some("42P01") | Some("42703") => {
                DatabaseErrorKind::SyntaxError
            }
            Some("23000") | Some("23001") | Some("23502") | Some("23503") | Some("23505") | Some("23514") => {
                DatabaseErrorKind::ConstraintViolation
            }
            Some("25000") | Some("25001") | Some("25002") | Some("25008") => {
                DatabaseErrorKind::TransactionError
            }
            Some("28000") | Some("28P01") => {
                DatabaseErrorKind::AuthenticationError
            }
            Some("53000") | Some("53100") | Some("53200") | Some("53300") => {
                DatabaseErrorKind::ResourceError
            }
            Some("XX000") | Some("XX001") | Some("XX002") => {
                DatabaseErrorKind::InternalError
            }
            _ => DatabaseErrorKind::QueryError,
        }
    }
    
    /// slay Create connection error
    pub fn connection_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::ConnectionError, message.to_string())
    }
    
    /// slay Create query error
    pub fn query_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::QueryError, message.to_string())
    }
    
    /// slay Create transaction error
    pub fn transaction_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::TransactionError, message.to_string())
    }
    
    /// slay Create constraint violation error
    pub fn constraint_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::ConstraintViolation, message.to_string())
    }
    
    /// slay Create syntax error
    pub fn syntax_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::SyntaxError, message.to_string())
    }
    
    /// slay Create authentication error
    pub fn auth_error(message: &str) -> Self {
        Self::new(DatabaseErrorKind::AuthenticationError, message.to_string())
    }
    
    /// slay Check if error is recoverable
    pub fn is_recoverable(&self) -> bool {
        match self.kind {
            DatabaseErrorKind::ConnectionError => true,
            DatabaseErrorKind::TransactionError => true,
            DatabaseErrorKind::ResourceError => true,
            DatabaseErrorKind::TimeoutError => true,
            _ => false,
        }
    }
    
    /// slay Check if error indicates connection loss
    pub fn is_connection_lost(&self) -> bool {
        match self.sqlstate.as_deref() {
            Some("08000") | Some("08003") | Some("08006") | Some("08001") => true,
            _ => false,
        }
    }
    
    /// slay Check if error is a constraint violation
    pub fn is_constraint_violation(&self) -> bool {
        matches!(self.kind, DatabaseErrorKind::ConstraintViolation)
    }
    
    /// slay Check if error is a unique violation
    pub fn is_unique_violation(&self) -> bool {
        self.sqlstate.as_deref() == Some("23505")
    }
    
    /// slay Check if error is a foreign key violation
    pub fn is_foreign_key_violation(&self) -> bool {
        self.sqlstate.as_deref() == Some("23503")
    }
    
    /// slay Check if error is a not null violation
    pub fn is_not_null_violation(&self) -> bool {
        self.sqlstate.as_deref() == Some("23502")
    }
    
    /// slay Get formatted error message with all details
    pub fn detailed_message(&self) -> String {
        let mut msg = format!("PostgreSQL Error: {}", self.message);
        
        if let Some(ref sqlstate) = self.sqlstate {
            msg.push_str(&format!(" (SQLSTATE: {})", sqlstate));
        }
        
        if let Some(ref severity) = self.severity {
            msg.push_str(&format!(" [{}]", severity));
        }
        
        if let Some(ref detail) = self.detail {
            msg.push_str(&format!("\nDetail: {}", detail));
        }
        
        if let Some(ref hint) = self.hint {
            msg.push_str(&format!("\nHint: {}", hint));
        }
        
        if let Some(ref position) = self.position {
            msg.push_str(&format!("\nPosition: {}", position));
        }
        
        if let Some(ref context) = self.context {
            msg.push_str(&format!("\nContext: {}", context));
        }
        
        if let Some(ref constraint) = self.constraint_name {
            msg.push_str(&format!("\nConstraint: {}", constraint));
        }
        
        if let Some(ref table) = self.table_name {
            if let Some(ref column) = self.column_name {
                msg.push_str(&format!("\nLocation: {}.{}", table, column));
            } else {
                msg.push_str(&format!("\nTable: {}", table));
            }
        }
        
        msg
    }
}

impl fmt::Display for PostgreSQLError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.detailed_message())
    }
}

impl std::error::Error for PostgreSQLError {}

impl From<PostgreSQLError> for DatabaseError {
    fn from(pg_error: PostgreSQLError) -> Self {
        DatabaseError::new(pg_error.kind, &pg_error.detailed_message())
    }
}

impl From<String> for PostgreSQLError {
    fn from(message: String) -> Self {
        Self::new(DatabaseErrorKind::QueryError, message)
    }
}

impl From<&str> for PostgreSQLError {
    fn from(message: &str) -> Self {
        Self::new(DatabaseErrorKind::QueryError, message.to_string())
    }
}

/// fr fr PostgreSQL error codes for specific error conditions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PostgreSQLErrorCode {
    // Connection errors
    ConnectionException,
    ConnectionDoesNotExist,
    ConnectionFailure,
    SqlClientUnableToEstablishConnection,
    SqlServerRejectedEstablishmentOfConnection,
    
    // Syntax errors
    SyntaxError,
    SyntaxErrorOrAccessRuleViolation,
    InsufficientPrivilege,
    CannotCoerce,
    GroupingError,
    WindowingError,
    InvalidRecursion,
    InvalidForeignKey,
    InvalidName,
    NameTooLong,
    ReservedName,
    DatatypeMismatch,
    IndeterminateDatatype,
    CollationMismatch,
    IndeterminateCollation,
    WrongObjectType,
    UndefinedColumn,
    UndefinedFunction,
    UndefinedTable,
    UndefinedParameter,
    UndefinedObject,
    DuplicateColumn,
    DuplicateCursor,
    DuplicateDatabase,
    DuplicateFunction,
    DuplicatePreparedStatement,
    DuplicateSchema,
    DuplicateTable,
    DuplicateAlias,
    DuplicateObject,
    AmbiguousColumn,
    AmbiguousFunction,
    AmbiguousParameter,
    AmbiguousAlias,
    InvalidColumnReference,
    InvalidColumnDefinition,
    InvalidCursorDefinition,
    InvalidDatabaseDefinition,
    InvalidFunctionDefinition,
    InvalidPreparedStatementDefinition,
    InvalidSchemaDefinition,
    InvalidTableDefinition,
    InvalidObjectDefinition,
    
    // Constraint violations
    IntegrityConstraintViolation,
    RestrictViolation,
    NotNullViolation,
    ForeignKeyViolation,
    UniqueViolation,
    CheckViolation,
    ExclusionViolation,
    
    // Transaction errors
    InvalidTransactionState,
    ActiveSqlTransaction,
    BranchTransactionAlreadyActive,
    HeldCursorRequiresSameIsolationLevel,
    InappropriateAccessModeForBranchTransaction,
    InappropriateIsolationLevelForBranchTransaction,
    NoActiveSqlTransactionForBranchTransaction,
    ReadOnlySqlTransaction,
    SchemaAndDataStatementMixingNotSupported,
    NoActiveSqlTransaction,
    InFailedSqlTransaction,
    IdleInTransactionSessionTimeout,
    
    // Authentication errors
    InvalidAuthorizationSpecification,
    InvalidPassword,
    
    // Resource errors
    InsufficientResources,
    DiskFull,
    OutOfMemory,
    TooManyConnections,
    ConfigurationLimitExceeded,
    
    // Internal errors
    InternalError,
    DataCorrupted,
    IndexCorrupted,
    
    // Unknown error
    Unknown,
}

impl PostgreSQLErrorCode {
    /// slay Get error code from SQLSTATE
    pub fn from_sqlstate(sqlstate: &str) -> Self {
        match sqlstate {
            "08000" => Self::ConnectionException,
            "08003" => Self::ConnectionDoesNotExist,
            "08006" => Self::ConnectionFailure,
            "08001" => Self::SqlClientUnableToEstablishConnection,
            "08004" => Self::SqlServerRejectedEstablishmentOfConnection,
            
            "42000" => Self::SyntaxErrorOrAccessRuleViolation,
            "42601" => Self::SyntaxError,
            "42501" => Self::InsufficientPrivilege,
            "42846" => Self::CannotCoerce,
            "42803" => Self::GroupingError,
            "42P20" => Self::WindowingError,
            "42P19" => Self::InvalidRecursion,
            "42830" => Self::InvalidForeignKey,
            "42602" => Self::InvalidName,
            "42622" => Self::NameTooLong,
            "42939" => Self::ReservedName,
            "42804" => Self::DatatypeMismatch,
            "42P18" => Self::IndeterminateDatatype,
            "42P21" => Self::CollationMismatch,
            "42P22" => Self::IndeterminateCollation,
            "42809" => Self::WrongObjectType,
            "42703" => Self::UndefinedColumn,
            "42883" => Self::UndefinedFunction,
            "42P01" => Self::UndefinedTable,
            "42P02" => Self::UndefinedParameter,
            "42704" => Self::UndefinedObject,
            "42701" => Self::DuplicateColumn,
            "42P03" => Self::DuplicateCursor,
            "42P04" => Self::DuplicateDatabase,
            "42723" => Self::DuplicateFunction,
            "42P05" => Self::DuplicatePreparedStatement,
            "42P06" => Self::DuplicateSchema,
            "42P07" => Self::DuplicateTable,
            "42712" => Self::DuplicateAlias,
            "42710" => Self::DuplicateObject,
            "42702" => Self::AmbiguousColumn,
            "42725" => Self::AmbiguousFunction,
            "42P08" => Self::AmbiguousParameter,
            "42P09" => Self::AmbiguousAlias,
            "42P10" => Self::InvalidColumnReference,
            "42611" => Self::InvalidColumnDefinition,
            "42P11" => Self::InvalidCursorDefinition,
            "42P12" => Self::InvalidDatabaseDefinition,
            "42P13" => Self::InvalidFunctionDefinition,
            "42P14" => Self::InvalidPreparedStatementDefinition,
            "42P15" => Self::InvalidSchemaDefinition,
            "42P16" => Self::InvalidTableDefinition,
            "42P17" => Self::InvalidObjectDefinition,
            
            "23000" => Self::IntegrityConstraintViolation,
            "23001" => Self::RestrictViolation,
            "23502" => Self::NotNullViolation,
            "23503" => Self::ForeignKeyViolation,
            "23505" => Self::UniqueViolation,
            "23514" => Self::CheckViolation,
            "23P01" => Self::ExclusionViolation,
            
            "25000" => Self::InvalidTransactionState,
            "25001" => Self::ActiveSqlTransaction,
            "25002" => Self::BranchTransactionAlreadyActive,
            "25008" => Self::HeldCursorRequiresSameIsolationLevel,
            "25003" => Self::InappropriateAccessModeForBranchTransaction,
            "25004" => Self::InappropriateIsolationLevelForBranchTransaction,
            "25005" => Self::NoActiveSqlTransactionForBranchTransaction,
            "25006" => Self::ReadOnlySqlTransaction,
            "25007" => Self::SchemaAndDataStatementMixingNotSupported,
            "25P01" => Self::NoActiveSqlTransaction,
            "25P02" => Self::InFailedSqlTransaction,
            "25P03" => Self::IdleInTransactionSessionTimeout,
            
            "28000" => Self::InvalidAuthorizationSpecification,
            "28P01" => Self::InvalidPassword,
            
            "53000" => Self::InsufficientResources,
            "53100" => Self::DiskFull,
            "53200" => Self::OutOfMemory,
            "53300" => Self::TooManyConnections,
            "53400" => Self::ConfigurationLimitExceeded,
            
            "XX000" => Self::InternalError,
            "XX001" => Self::DataCorrupted,
            "XX002" => Self::IndexCorrupted,
            
            _ => Self::Unknown,
        }
    }
    
    /// slay Get human-readable description
    pub fn description(&self) -> &'static str {
        match self {
            Self::ConnectionException => "Connection exception",
            Self::ConnectionDoesNotExist => "Connection does not exist",
            Self::ConnectionFailure => "Connection failure",
            Self::SqlClientUnableToEstablishConnection => "SQL client unable to establish connection",
            Self::SqlServerRejectedEstablishmentOfConnection => "SQL server rejected establishment of connection",
            
            Self::SyntaxError => "Syntax error",
            Self::SyntaxErrorOrAccessRuleViolation => "Syntax error or access rule violation",
            Self::InsufficientPrivilege => "Insufficient privilege",
            Self::CannotCoerce => "Cannot coerce",
            Self::GroupingError => "Grouping error",
            Self::WindowingError => "Windowing error",
            Self::InvalidRecursion => "Invalid recursion",
            Self::InvalidForeignKey => "Invalid foreign key",
            Self::InvalidName => "Invalid name",
            Self::NameTooLong => "Name too long",
            Self::ReservedName => "Reserved name",
            Self::DatatypeMismatch => "Datatype mismatch",
            Self::IndeterminateDatatype => "Indeterminate datatype",
            Self::CollationMismatch => "Collation mismatch",
            Self::IndeterminateCollation => "Indeterminate collation",
            Self::WrongObjectType => "Wrong object type",
            Self::UndefinedColumn => "Undefined column",
            Self::UndefinedFunction => "Undefined function",
            Self::UndefinedTable => "Undefined table",
            Self::UndefinedParameter => "Undefined parameter",
            Self::UndefinedObject => "Undefined object",
            Self::DuplicateColumn => "Duplicate column",
            Self::DuplicateCursor => "Duplicate cursor",
            Self::DuplicateDatabase => "Duplicate database",
            Self::DuplicateFunction => "Duplicate function",
            Self::DuplicatePreparedStatement => "Duplicate prepared statement",
            Self::DuplicateSchema => "Duplicate schema",
            Self::DuplicateTable => "Duplicate table",
            Self::DuplicateAlias => "Duplicate alias",
            Self::DuplicateObject => "Duplicate object",
            Self::AmbiguousColumn => "Ambiguous column",
            Self::AmbiguousFunction => "Ambiguous function",
            Self::AmbiguousParameter => "Ambiguous parameter",
            Self::AmbiguousAlias => "Ambiguous alias",
            Self::InvalidColumnReference => "Invalid column reference",
            Self::InvalidColumnDefinition => "Invalid column definition",
            Self::InvalidCursorDefinition => "Invalid cursor definition",
            Self::InvalidDatabaseDefinition => "Invalid database definition",
            Self::InvalidFunctionDefinition => "Invalid function definition",
            Self::InvalidPreparedStatementDefinition => "Invalid prepared statement definition",
            Self::InvalidSchemaDefinition => "Invalid schema definition",
            Self::InvalidTableDefinition => "Invalid table definition",
            Self::InvalidObjectDefinition => "Invalid object definition",
            
            Self::IntegrityConstraintViolation => "Integrity constraint violation",
            Self::RestrictViolation => "Restrict violation",
            Self::NotNullViolation => "Not null violation",
            Self::ForeignKeyViolation => "Foreign key violation",
            Self::UniqueViolation => "Unique violation",
            Self::CheckViolation => "Check violation",
            Self::ExclusionViolation => "Exclusion violation",
            
            Self::InvalidTransactionState => "Invalid transaction state",
            Self::ActiveSqlTransaction => "Active SQL transaction",
            Self::BranchTransactionAlreadyActive => "Branch transaction already active",
            Self::HeldCursorRequiresSameIsolationLevel => "Held cursor requires same isolation level",
            Self::InappropriateAccessModeForBranchTransaction => "Inappropriate access mode for branch transaction",
            Self::InappropriateIsolationLevelForBranchTransaction => "Inappropriate isolation level for branch transaction",
            Self::NoActiveSqlTransactionForBranchTransaction => "No active SQL transaction for branch transaction",
            Self::ReadOnlySqlTransaction => "Read only SQL transaction",
            Self::SchemaAndDataStatementMixingNotSupported => "Schema and data statement mixing not supported",
            Self::NoActiveSqlTransaction => "No active SQL transaction",
            Self::InFailedSqlTransaction => "In failed SQL transaction",
            Self::IdleInTransactionSessionTimeout => "Idle in transaction session timeout",
            
            Self::InvalidAuthorizationSpecification => "Invalid authorization specification",
            Self::InvalidPassword => "Invalid password",
            
            Self::InsufficientResources => "Insufficient resources",
            Self::DiskFull => "Disk full",
            Self::OutOfMemory => "Out of memory",
            Self::TooManyConnections => "Too many connections",
            Self::ConfigurationLimitExceeded => "Configuration limit exceeded",
            
            Self::InternalError => "Internal error",
            Self::DataCorrupted => "Data corrupted",
            Self::IndexCorrupted => "Index corrupted",
            
            Self::Unknown => "Unknown error",
        }
    }
}
