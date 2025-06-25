/// fr fr Query execution and context handling for SQLSlay
/// 
/// This module provides query execution infrastructure with context awareness,
/// parameter binding, and result processing.

use std::collections::HashMap;
use std::time::{Duration, Instant};
use super::{DatabaseError, DatabaseErrorKind, SqlValue, VibeContext};
use crate::error::CursedError;

/// fr fr Context for query execution with metadata
#[derive(Debug, Clone)]
pub struct QueryContext {
    /// fr fr Query timeout
    pub timeout: Option<Duration>,
    /// fr fr Query tags for logging and monitoring
    pub tags: HashMap<String, String>,
    /// fr fr Whether to enable query tracing
    pub trace_enabled: bool,
    /// fr fr Query execution start time
    pub start_time: Option<Instant>,
}

impl Default for QueryContext {
    fn default() -> Self {
        Self {
            timeout: None,
            tags: HashMap::new(),
            trace_enabled: false,
            start_time: None,
        }
    }
}

/// fr fr Result of query execution with metadata
#[derive(Debug, Clone)]
pub struct QueryResult {
    /// fr fr Column names
    pub column_names: Vec<String>,
    /// fr fr Column types
    pub column_types: Vec<String>,
    /// fr fr Result rows
    pub rows: Vec<Vec<SqlValue>>,
    /// fr fr Execution duration
    pub execution_duration: Option<Duration>,
    /// fr fr Query that was executed
    pub query: String,
    /// fr fr Number of rows affected (for non-SELECT queries)
    pub rows_affected: Option<i64>,
    /// fr fr Last insert ID (for INSERT queries)
    pub last_insert_id: Option<i64>,
    /// fr fr Any error that occurred
    pub error: Option<DatabaseError>,
}

impl QueryResult {
    /// slay Create a new successful query result
    pub fn new(
        column_names: Vec<String>,
        column_types: Vec<String>,
        rows: Vec<Vec<SqlValue>>,
        query: String,
    ) -> Self {
        Self {
            column_names,
            column_types,
            rows,
            execution_duration: None,
            query,
            rows_affected: None,
            last_insert_id: None,
            error: None,
        }
    }

    /// slay Create an error result
    pub fn with_error(query: String, error: DatabaseError) -> Self {
        Self {
            column_names: Vec::from([]),
            column_types: Vec::from([]),
            rows: Vec::from([]),
            execution_duration: None,
            query,
            rows_affected: None,
            last_insert_id: None,
            error: Some(error),
        }
    }
}

/// fr fr Query executor trait for different execution strategies
pub trait QueryExecutor: Send + Sync {
    /// slay Execute a query that returns rows
    fn execute_query(
        &self,
        query: &str,
        args: &[SqlValue],
        context: QueryContext,
    ) -> crate::error::Result<()>;

    /// slay Execute a query that doesn't return rows
    fn execute_statement(
        &self,
        query: &str,
        args: &[SqlValue],
        context: QueryContext,
    ) -> crate::error::Result<()>;

    /// slay Prepare a statement for later execution
    fn prepare_statement(&self, query: &str) -> crate::error::Result<()>;
}

/// fr fr Prepared statement interface
pub trait PreparedStatement: Send + Sync {
    /// slay Execute this prepared statement with arguments
    fn execute(&self, args: &[SqlValue], context: QueryContext) -> crate::error::Result<()>;

    /// slay Get the original query string
    fn query(&self) -> &str;

    /// slay Get parameter count
    fn parameter_count(&self) -> usize;

    /// slay Close this prepared statement
    fn close(&self) -> crate::error::Result<()>;
}
