/// fr fr Database result handling - managing query results and metadata periodt
///
/// This module provides result set management, row iteration, column metadata,
/// and execution statistics for database operations. Results need love too bestie!

use crate::stdlib::packages::db_core::{
    DatabaseError, ErrorKind, QueryError
};
use crate::error::Error;
use crate::stdlib::packages::db_core::error::{DatabaseResult as DbResult};

use std::collections::HashMap;
use std::time::Duration;

/// fr fr Main database query result type for query operations
#[derive(Debug, Clone)]
pub struct DatabaseQueryResult {
    /// Result data
    pub data: ResultData,
    /// Result metadata
    pub metadata: ResultMetadata,
    /// Execution statistics
    pub stats: QueryStats,
}

/// fr fr Result data enumeration
#[derive(Debug, Clone)]
pub enum ResultData {
    /// Row-based result set (for SELECT queries)
    Rows(Vec<Row>),
    /// Execution result (for INSERT/UPDATE/DELETE)
    Execute(ExecuteResult),
    /// Empty result
    Empty,
}

/// fr fr Row representation
#[derive(Debug, Clone)]
pub struct Row {
    /// Column values
    pub values: Vec<ColumnValue>,
    /// Row metadata
    pub metadata: RowMetadata,
}

/// fr fr Column value with type information
#[derive(Debug, Clone)]
pub struct ColumnValue {
    /// Column data
    pub data: Option<Vec<u8>>,
    /// Data type
    pub column_type: ColumnType,
    /// Whether value is null
    pub is_null: bool,
}

/// fr fr Column type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ColumnType {
    /// NULL type
    Null,
    /// Boolean
    Boolean,
    /// 8-bit integer
    TinyInt,
    /// 16-bit integer
    SmallInt,
    /// 32-bit integer
    Integer,
    /// 64-bit integer
    BigInt,
    /// 32-bit floating point
    Float,
    /// 64-bit floating point
    Double,
    /// Fixed-precision decimal
    Decimal(u32, u32), // precision, scale
    /// Variable-length string
    VarChar(usize), // max_length
    /// Fixed-length string
    Char(usize), // length
    /// Text (unlimited string)
    Text,
    /// Binary data
    Binary(usize), // length
    /// Variable binary data
    VarBinary(usize), // max_length
    /// Large binary object
    Blob,
    /// Large character object
    Clob,
    /// Date
    Date,
    /// Time
    Time,
    /// Timestamp
    Timestamp,
    /// Timestamp with timezone
    TimestampTz,
    /// JSON data
    Json,
    /// XML data
    Xml,
    /// UUID
    Uuid,
    /// Array type
    Array(Box<ColumnType>),
    /// Custom type
    Custom(String),
}

/// fr fr Column metadata
#[derive(Debug, Clone)]
pub struct Column {
    /// Column name
    pub name: String,
    /// Column type
    pub column_type: ColumnType,
    /// Column ordinal position (0-based)
    pub ordinal: usize,
    /// Whether column allows NULL values
    pub nullable: bool,
    /// Table name (if available)
    pub table_name: Option<String>,
    /// Schema name (if available)
    pub schema_name: Option<String>,
    /// Column precision (for numeric types)
    pub precision: Option<u32>,
    /// Column scale (for decimal types)
    pub scale: Option<u32>,
    /// Maximum length (for string/binary types)
    pub max_length: Option<usize>,
    /// Whether column is auto-increment
    pub auto_increment: bool,
    /// Whether column is part of primary key
    pub is_primary_key: bool,
    /// Whether column has unique constraint
    pub is_unique: bool,
    /// Column default value (if any)
    pub default_value: Option<String>,
    /// Column comment/description
    pub comment: Option<String>,
}

/// fr fr Row metadata
#[derive(Debug, Clone)]
pub struct RowMetadata {
    /// Row number (1-based)
    pub row_number: usize,
    /// Whether row was inserted/updated/deleted
    pub is_modified: bool,
    /// Row version/timestamp (for optimistic locking)
    pub version: Option<String>,
}

/// fr fr Result set metadata
#[derive(Debug, Clone)]
pub struct ResultMetadata {
    /// Column definitions
    pub columns: Vec<Column>,
    /// Total row count (if known)
    pub total_rows: Option<usize>,
    /// Whether there are more rows available
    pub has_more_rows: bool,
    /// Result set name/identifier
    pub name: Option<String>,
    /// Schema name
    pub schema_name: Option<String>,
    /// Table name (for single-table results)
    pub table_name: Option<String>,
    /// Whether result set is updatable
    pub is_updatable: bool,
    /// Result set type
    pub result_type: ResultType,
}

/// fr fr Execute result for non-query operations
#[derive(Debug, Clone)]
pub struct ExecuteResult {
    /// Number of rows affected
    pub rows_affected: u64,
    /// Last inserted ID (if applicable)
    pub last_insert_id: Option<u64>,
    /// Generated keys (for batch inserts)
    pub generated_keys: Vec<u64>,
    /// Warnings generated during execution
    pub warnings: Vec<String>,
    /// Execution metadata
    pub metadata: ExecuteMetadata,
}

/// fr fr Execute metadata
#[derive(Debug, Clone)]
pub struct ExecuteMetadata {
    /// Operation type
    pub operation: ExecuteOperation,
    /// Target table/collection
    pub target: Option<String>,
    /// Whether operation was successful
    pub success: bool,
    /// Execution time
    pub execution_time: Option<Duration>,
}

/// fr fr Execute operation types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecuteOperation {
    Insert,
    Update,
    Delete,
    Create,
    Alter,
    Drop,
    Call,
    Batch,
    Unknown,
}

/// fr fr Query execution statistics
#[derive(Debug, Clone)]
pub struct QueryStats {
    /// Query execution time
    pub execution_time: Option<Duration>,
    /// Parse time
    pub parse_time: Option<Duration>,
    /// Plan time
    pub plan_time: Option<Duration>,
    /// Network time
    pub network_time: Option<Duration>,
    /// Rows processed
    pub rows_processed: Option<usize>,
    /// Rows returned
    pub rows_returned: Option<usize>,
    /// Memory used (bytes)
    pub memory_used: Option<usize>,
    /// Disk I/O operations
    pub disk_io_ops: Option<usize>,
    /// Cache hits
    pub cache_hits: Option<usize>,
    /// Cache misses
    pub cache_misses: Option<usize>,
    /// Query plan used
    pub query_plan: Option<String>,
    /// Execution warnings
    pub warnings: Vec<String>,
}

/// fr fr Result set types
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ResultType {
    /// Forward-only result set
    ForwardOnly,
    /// Scrollable result set
    Scrollable,
    /// Updatable result set
    Updatable,
    /// Cached result set
    Cached,
}

/// fr fr Result set implementation
#[derive(Debug)]
pub struct ResultSetImpl {
    /// Rows data
    rows: Vec<Row>,
    /// Current position
    position: usize,
    /// Metadata
    metadata: ResultMetadata,
    /// Whether result set is closed
    closed: bool,
}

impl ResultSetImpl {
    /// slay Create a new result set
    pub fn new(rows: Vec<Row>, metadata: ResultMetadata) -> Self {
        Self {
            rows,
            position: 0,
            metadata,
            closed: false,
        }
    }

    /// slay Get next row
    pub fn next(&mut self) -> DbResult<Option<&Row>> {
        if self.closed {
            return Err(DatabaseError::query(
                QueryError::ResultSetExhausted,
                "Result set is closed"
            ));
        }

        if self.position < self.rows.len() {
            let row = &self.rows[self.position];
            self.position += 1;
            Ok(Some(row))
        } else {
            Ok(None)
        }
    }

    /// slay Get all remaining rows
    pub fn collect_remaining(&mut self) -> DbResult<Vec<&Row>> {
        if self.closed {
            return Err(DatabaseError::query(
                QueryError::ResultSetExhausted,
                "Result set is closed"
            ));
        }

        let remaining: Vec<&Row> = self.rows[self.position..].iter().collect();
        self.position = self.rows.len();
        Ok(remaining)
    }

    /// slay Get row by index
    pub fn get_row(&self, index: usize) -> Option<&Row> {
        if self.closed {
            return None;
        }
        self.rows.get(index)
    }

    /// slay Get total row count
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }

    /// slay Check if there are more rows
    pub fn has_next(&self) -> bool {
        !self.closed && self.position < self.rows.len()
    }

    /// slay Get current position
    pub fn position(&self) -> usize {
        self.position
    }

    /// slay Reset to beginning
    pub fn reset(&mut self) {
        self.position = 0;
    }

    /// slay Close the result set
    pub fn close(&mut self) {
        self.closed = true;
        self.rows.clear();
    }

    /// slay Get metadata
    pub fn metadata(&self) -> &ResultMetadata {
        &self.metadata
    }
}

impl Row {
    /// slay Create a new row
    pub fn new(values: Vec<ColumnValue>) -> Self {
        Self {
            values,
            metadata: RowMetadata {
                row_number: 0,
                is_modified: false,
                version: None,
            },
        }
    }

    /// slay Create row with metadata
    pub fn with_metadata(values: Vec<ColumnValue>, metadata: RowMetadata) -> Self {
        Self { values, metadata }
    }

    /// slay Get value by column index
    pub fn get(&self, index: usize) -> Option<&ColumnValue> {
        self.values.get(index)
    }

    /// slay Get value by column name
    pub fn get_by_name(&self, name: &str, columns: &[Column]) -> Option<&ColumnValue> {
        columns.iter()
            .position(|col| col.name == name)
            .and_then(|index| self.values.get(index))
    }

    /// slay Get column count
    pub fn column_count(&self) -> usize {
        self.values.len()
    }

    /// slay Convert to hash map for easier access
    pub fn to_map(&self, columns: &[Column]) -> HashMap<String, &ColumnValue> {
        let mut map = HashMap::new();
        for (i, column) in columns.iter().enumerate() {
            if let Some(value) = self.values.get(i) {
                map.insert(column.name.clone(), value);
            }
        }
        map
    }

    /// slay Check if row is empty
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl ColumnValue {
    /// slay Create a NULL value
    pub fn null(column_type: ColumnType) -> Self {
        Self {
            data: None,
            column_type,
            is_null: true,
        }
    }

    /// slay Create a value with data
    pub fn with_data(data: Vec<u8>, column_type: ColumnType) -> Self {
        Self {
            data: Some(data),
            column_type,
            is_null: false,
        }
    }

    /// slay Get data as bytes
    pub fn as_bytes(&self) -> Option<&[u8]> {
        if self.is_null {
            None
        } else {
            self.data.as_deref()
        }
    }

    /// slay Get data as string (UTF-8)
    pub fn as_string(&self) -> DbResult<Option<String>> {
        if self.is_null {
            return Ok(None);
        }

        if let Some(data) = &self.data {
            let string = String::from_utf8(data.clone())
                .map_err(|e| DatabaseError::new(
                    ErrorKind::DataConversion,
                    &format!("Invalid UTF-8 data: {}", e)
                ))?;
            Ok(Some(string))
        } else {
            Ok(None)
        }
    }

    /// slay Get data as integer
    pub fn as_i64(&self) -> DbResult<Option<i64>> {
        if self.is_null {
            return Ok(None);
        }

        match &self.column_type {
            ColumnType::TinyInt | ColumnType::SmallInt | 
            ColumnType::Integer | ColumnType::BigInt => {
                if let Some(data) = &self.data {
                    if data.len() >= 8 {
                        let bytes: [u8; 8] = data[0..8].try_into()
                            .map_err(|_| DatabaseError::new(
                                ErrorKind::DataConversion,
                                "Invalid integer data"
                            ))?;
                        Ok(Some(i64::from_le_bytes(bytes)))
                    } else {
                        Err(DatabaseError::new(
                            ErrorKind::DataConversion,
                            "Insufficient data for integer conversion"
                        ))
                    }
                } else {
                    Ok(None)
                }
            }
            _ => {
                // Try to parse as string
                if let Some(string) = self.as_string()? {
                    string.parse::<i64>()
                        .map(Some)
                        .map_err(|e| DatabaseError::new(
                            ErrorKind::DataConversion,
                            &format!("Cannot parse integer: {}", e)
                        ))
                } else {
                    Ok(None)
                }
            }
        }
    }

    /// slay Get data as float
    pub fn as_f64(&self) -> DbResult<Option<f64>> {
        if self.is_null {
            return Ok(None);
        }

        match &self.column_type {
            ColumnType::Float | ColumnType::Double => {
                if let Some(data) = &self.data {
                    if data.len() >= 8 {
                        let bytes: [u8; 8] = data[0..8].try_into()
                            .map_err(|_| DatabaseError::new(
                                ErrorKind::DataConversion,
                                "Invalid float data"
                            ))?;
                        Ok(Some(f64::from_le_bytes(bytes)))
                    } else {
                        Err(DatabaseError::new(
                            ErrorKind::DataConversion,
                            "Insufficient data for float conversion"
                        ))
                    }
                } else {
                    Ok(None)
                }
            }
            _ => {
                // Try to parse as string
                if let Some(string) = self.as_string()? {
                    string.parse::<f64>()
                        .map(Some)
                        .map_err(|e| DatabaseError::new(
                            ErrorKind::DataConversion,
                            &format!("Cannot parse float: {}", e)
                        ))
                } else {
                    Ok(None)
                }
            }
        }
    }

    /// slay Get data as boolean
    pub fn as_bool(&self) -> DbResult<Option<bool>> {
        if self.is_null {
            return Ok(None);
        }

        match &self.column_type {
            ColumnType::Boolean => {
                if let Some(data) = &self.data {
                    if !data.is_empty() {
                        Ok(Some(data[0] != 0))
                    } else {
                        Ok(Some(false))
                    }
                } else {
                    Ok(None)
                }
            }
            _ => {
                // Try to parse as string
                if let Some(string) = self.as_string()? {
                    let lower = string.to_lowercase();
                    match lower.as_str() {
                        "true" | "t" | "yes" | "y" | "1" => Ok(Some(true)),
                        "false" | "f" | "no" | "n" | "0" => Ok(Some(false)),
                        _ => Err(DatabaseError::new(
                            ErrorKind::DataConversion,
                            &format!("Cannot parse boolean: {}", string)
                        ))
                    }
                } else {
                    Ok(None)
                }
            }
        }
    }
}

impl Column {
    /// slay Create a new column
    pub fn new(name: &str, column_type: ColumnType, ordinal: usize) -> Self {
        Self {
            name: name.to_string(),
            column_type,
            ordinal,
            nullable: true,
            table_name: None,
            schema_name: None,
            precision: None,
            scale: None,
            max_length: None,
            auto_increment: false,
            is_primary_key: false,
            is_unique: false,
            default_value: None,
            comment: None,
        }
    }

    /// slay Set column as not nullable
    pub fn not_null(mut self) -> Self {
        self.nullable = false;
        self
    }

    /// slay Set table name
    pub fn with_table(mut self, table_name: &str) -> Self {
        self.table_name = Some(table_name.to_string());
        self
    }

    /// slay Set schema name
    pub fn with_schema(mut self, schema_name: &str) -> Self {
        self.schema_name = Some(schema_name.to_string());
        self
    }

    /// slay Set as primary key
    pub fn primary_key(mut self) -> Self {
        self.is_primary_key = true;
        self.nullable = false;
        self
    }

    /// slay Set as unique
    pub fn unique(mut self) -> Self {
        self.is_unique = true;
        self
    }

    /// slay Set as auto-increment
    pub fn auto_increment(mut self) -> Self {
        self.auto_increment = true;
        self
    }

    /// slay Get full column name (schema.table.column)
    pub fn full_name(&self) -> String {
        let mut parts = Vec::new();
        
        if let Some(schema) = &self.schema_name {
            parts.push(schema.clone());
        }
        
        if let Some(table) = &self.table_name {
            parts.push(table.clone());
        }
        
        parts.push(self.name.clone());
        parts.join(".")
    }
}

impl ExecuteResult {
    /// slay Create a new execute result
    pub fn new(operation: ExecuteOperation, rows_affected: u64) -> Self {
        Self {
            rows_affected,
            last_insert_id: None,
            generated_keys: Vec::new(),
            warnings: Vec::new(),
            metadata: ExecuteMetadata {
                operation,
                target: None,
                success: true,
                execution_time: None,
            },
        }
    }

    /// slay Set last insert ID
    pub fn with_last_insert_id(mut self, id: u64) -> Self {
        self.last_insert_id = Some(id);
        self
    }

    /// slay Add generated keys
    pub fn with_generated_keys(mut self, keys: Vec<u64>) -> Self {
        self.generated_keys = keys;
        self
    }

    /// slay Add warning
    pub fn add_warning(&mut self, warning: &str) {
        self.warnings.push(warning.to_string());
    }

    /// slay Check if execution was successful
    pub fn is_success(&self) -> bool {
        self.metadata.success
    }
}

impl QueryStats {
    /// slay Create new query stats
    pub fn new() -> Self {
        Self {
            execution_time: None,
            parse_time: None,
            plan_time: None,
            network_time: None,
            rows_processed: None,
            rows_returned: None,
            memory_used: None,
            disk_io_ops: None,
            cache_hits: None,
            cache_misses: None,
            query_plan: None,
            warnings: Vec::new(),
        }
    }

    /// slay Set execution time
    pub fn with_execution_time(mut self, time: Duration) -> Self {
        self.execution_time = Some(time);
        self
    }

    /// slay Set rows processed
    pub fn with_rows_processed(mut self, rows: usize) -> Self {
        self.rows_processed = Some(rows);
        self
    }

    /// slay Set rows returned
    pub fn with_rows_returned(mut self, rows: usize) -> Self {
        self.rows_returned = Some(rows);
        self
    }

    /// slay Add warning
    pub fn add_warning(&mut self, warning: &str) {
        self.warnings.push(warning.to_string());
    }

    /// slay Get cache hit ratio
    pub fn cache_hit_ratio(&self) -> Option<f64> {
        if let (Some(hits), Some(misses)) = (self.cache_hits, self.cache_misses) {
            let total = hits + misses;
            if total > 0 {
                Some(hits as f64 / total as f64)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl Default for QueryStats {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_value_creation() {
        let null_val = ColumnValue::null(ColumnType::Text);
        assert!(null_val.is_null);
        assert!(null_val.data.is_none());

        let text_data = "hello".as_bytes().to_vec();
        let text_val = ColumnValue::with_data(text_data, ColumnType::Text);
        assert!(!text_val.is_null);
        assert!(text_val.data.is_some());
    }

    #[test]
    fn test_column_value_conversions() {
        let text_data = "42".as_bytes().to_vec();
        let text_val = ColumnValue::with_data(text_data, ColumnType::Text);
        
        let string_result = text_val.as_string().unwrap();
        assert_eq!(string_result, Some("42".to_string()));
        
        let int_result = text_val.as_i64().unwrap();
        assert_eq!(int_result, Some(42));
    }

    #[test]
    fn test_row_operations() {
        let values = vec![
            ColumnValue::with_data("1".as_bytes().to_vec(), ColumnType::Integer),
            ColumnValue::with_data("Alice".as_bytes().to_vec(), ColumnType::Text),
        ];
        let row = Row::new(values);

        assert_eq!(row.column_count(), 2);
        assert!(row.get(0).is_some());
        assert!(row.get(1).is_some());
        assert!(row.get(2).is_none());
    }

    #[test]
    fn test_column_metadata() {
        let column = Column::new("id", ColumnType::Integer, 0)
            .not_null()
            .primary_key()
            .auto_increment()
            .with_table("users")
            .with_schema("public");

        assert!(!column.nullable);
        assert!(column.is_primary_key);
        assert!(column.auto_increment);
        assert_eq!(column.table_name, Some("users".to_string()));
        assert_eq!(column.schema_name, Some("public".to_string()));
        assert_eq!(column.full_name(), "public.users.id");
    }

    #[test]
    fn test_execute_result() {
        let mut result = ExecuteResult::new(ExecuteOperation::Insert, 5)
            .with_last_insert_id(123)
            .with_generated_keys(Vec::from([123, 124, 125, 126, 127]));

        assert_eq!(result.rows_affected, 5);
        assert_eq!(result.last_insert_id, Some(123));
        assert_eq!(result.generated_keys.len(), 5);
        assert!(result.is_success());

        result.add_warning("Data truncated");
        assert_eq!(result.warnings.len(), 1);
    }

    #[test]
    fn test_result_set_iteration() {
        let values1 = vec![
            ColumnValue::with_data("1".as_bytes().to_vec(), ColumnType::Integer),
        ];
        let values2 = vec![
            ColumnValue::with_data("2".as_bytes().to_vec(), ColumnType::Integer),
        ];
        
        let rows = Vec::from([Row::new(values1), Row::new(values2)]);
        let metadata = ResultMetadata {
            columns: Vec::from([Column::new("id", ColumnType::Integer, 0)]),
            total_rows: Some(2),
            has_more_rows: false,
            name: None,
            schema_name: None,
            table_name: None,
            is_updatable: false,
            result_type: ResultType::ForwardOnly,
        };
        
        let mut result_set = ResultSetImpl::new(rows, metadata);
        
        assert_eq!(result_set.row_count(), 2);
        assert!(result_set.has_next());
        
        let row1 = result_set.next().unwrap();
        assert!(row1.is_some());
        
        let row2 = result_set.next().unwrap();
        assert!(row2.is_some());
        
        let row3 = result_set.next().unwrap();
        assert!(row3.is_none());
        assert!(!result_set.has_next());
    }

    #[test]
    fn test_query_stats() {
        let mut stats = QueryStats::new()
            .with_execution_time(Duration::from_millis(100))
            .with_rows_processed(1000)
            .with_rows_returned(50);

        stats.cache_hits = Some(80);
        stats.cache_misses = Some(20);
        stats.add_warning("Index not used");

        assert_eq!(stats.execution_time, Some(Duration::from_millis(100)));
        assert_eq!(stats.rows_processed, Some(1000));
        assert_eq!(stats.rows_returned, Some(50));
        assert_eq!(stats.cache_hit_ratio(), Some(0.8));
        assert_eq!(stats.warnings.len(), 1);
    }
}
