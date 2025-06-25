/// fr fr Database result types - handling query results like a boss periodt
///
/// This module defines the result types for database queries and operations.
/// Because getting results right is half the battle bestie!

// use crate::stdlib::packages::db_core::{
    Row, Column, ColumnType, DatabaseValue, DatabaseError
};
// use crate::stdlib::packages::db_core::error::{DatabaseResult as DbResult};
use std::collections::HashMap;

/// fr fr Result of a database query
#[derive(Debug, Clone)]
pub struct DatabaseQueryResult {
    pub rows: Vec<Row>,
    pub metadata: ResultMetadata,
    pub stats: QueryStats,
}

impl DatabaseQueryResult {
    /// slay Create a new query result
    pub fn new(rows: Vec<Row>, metadata: ResultMetadata) -> Self {
        let stats = QueryStats {
            rows_affected: rows.len(),
            execution_time_ms: 0,
            rows_fetched: rows.len(),
            bytes_transferred: 0,
        };

        Self {
            rows,
            metadata,
            stats,
        }
    }

    /// slay Create an empty result
    pub fn empty() -> Self {
        Self {
            rows: Vec::new(),
            metadata: ResultMetadata::empty(),
            stats: QueryStats::default(),
        }
    }

    /// slay Get the number of rows
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// slay Check if result is empty
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }

    /// slay Get a specific row
    pub fn get_row(&self, index: usize) -> Option<&Row> {
        self.rows.get(index)
    }

    /// slay Get all rows
    pub fn get_rows(&self) -> &[Row] {
        &self.rows
    }
}

/// fr fr Result metadata - information about the query result
#[derive(Debug, Clone)]
pub struct ResultMetadata {
    pub columns: Vec<Column>,
    pub table_name: Option<String>,
    pub schema_name: Option<String>,
    pub is_read_only: bool,
    pub has_more_results: bool,
}

impl ResultMetadata {
    /// slay Create new result metadata
    pub fn new(columns: Vec<Column>) -> Self {
        Self {
            columns,
            table_name: None,
            schema_name: None,
            is_read_only: true,
            has_more_results: false,
        }
    }

    /// slay Create empty metadata
    pub fn empty() -> Self {
        Self {
            columns: Vec::new(),
            table_name: None,
            schema_name: None,
            is_read_only: true,
            has_more_results: false,
        }
    }

    /// slay Get column count
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// slay Get column by name
    pub fn get_column(&self, name: &str) -> Option<&Column> {
        self.columns.iter().find(|col| col.name == name)
    }

    /// slay Get column by index
    pub fn get_column_by_index(&self, index: usize) -> Option<&Column> {
        self.columns.get(index)
    }
}

/// fr fr Execute result for non-query operations
#[derive(Debug, Clone)]
pub struct ExecuteResult {
    pub rows_affected: usize,
    pub last_insert_id: Option<DatabaseValue>,
    pub warnings: Vec<String>,
    pub execution_time_ms: u64,
}

impl ExecuteResult {
    /// slay Create a new execute result
    pub fn new(rows_affected: usize) -> Self {
        Self {
            rows_affected,
            last_insert_id: None,
            warnings: Vec::new(),
            execution_time_ms: 0,
        }
    }

    /// slay Create execute result with insert ID
    pub fn with_insert_id(rows_affected: usize, insert_id: DatabaseValue) -> Self {
        Self {
            rows_affected,
            last_insert_id: Some(insert_id),
            warnings: Vec::new(),
            execution_time_ms: 0,
        }
    }

    /// slay Add a warning
    pub fn add_warning(&mut self, warning: String) {
        self.warnings.push(warning);
    }

    /// slay Set execution time
    pub fn set_execution_time(&mut self, time_ms: u64) {
        self.execution_time_ms = time_ms;
    }
}

/// fr fr Query execution statistics
#[derive(Debug, Clone)]
pub struct QueryStats {
    pub rows_affected: usize,
    pub execution_time_ms: u64,
    pub rows_fetched: usize,
    pub bytes_transferred: usize,
}

impl Default for QueryStats {
    fn default() -> Self {
        Self {
            rows_affected: 0,
            execution_time_ms: 0,
            rows_fetched: 0,
            bytes_transferred: 0,
        }
    }
}

/// fr fr Row metadata for additional row information
#[derive(Debug, Clone)]
pub struct RowMetadata {
    pub row_number: usize,
    pub is_deleted: bool,
    pub is_modified: bool,
    pub version: Option<String>,
}

impl RowMetadata {
    /// slay Create new row metadata
    pub fn new(row_number: usize) -> Self {
        Self {
            row_number,
            is_deleted: false,
            is_modified: false,
            version: None,
        }
    }
}

/// fr fr Result type for result sets
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResultType {
    ForwardOnly,
    Scrollable,
}

/// fr fr Result set implementation for streaming results
#[derive(Debug)]
pub struct ResultSetImpl {
    rows: Vec<Row>,
    current_index: usize,
    metadata: ResultMetadata,
}

impl ResultSetImpl {
    /// slay Create a new result set
    pub fn new(rows: Vec<Row>, metadata: ResultMetadata) -> Self {
        Self {
            rows,
            current_index: 0,
            metadata,
        }
    }

    /// slay Create an empty result set
    pub fn empty() -> Self {
        Self {
            rows: Vec::new(),
            current_index: 0,
            metadata: ResultMetadata::empty(),
        }
    }

    /// slay Get next row
    pub fn next(&mut self) -> DbResult<Option<Row>> {
        if self.current_index < self.rows.len() {
            let row = self.rows[self.current_index].clone();
            self.current_index += 1;
            Ok(Some(row))
        } else {
            Ok(None)
        }
    }

    /// slay Collect all remaining rows
    pub fn collect(&mut self) -> DbResult<Vec<Row>> {
        let remaining = self.rows[self.current_index..].to_vec();
        self.current_index = self.rows.len();
        Ok(remaining)
    }

    /// slay Get column metadata
    pub fn columns(&self) -> &[Column] {
        &self.metadata.columns
    }

    /// slay Get result metadata
    pub fn metadata(&self) -> &ResultMetadata {
        &self.metadata
    }

    /// slay Check if there are more rows
    pub fn has_next(&self) -> bool {
        self.current_index < self.rows.len()
    }

    /// slay Get row count
    pub fn row_count(&self) -> Option<usize> {
        Some(self.rows.len())
    }

    /// slay Reset to beginning
    pub fn reset(&mut self) {
        self.current_index = 0;
    }
}

