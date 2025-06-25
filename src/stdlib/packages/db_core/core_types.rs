/// fr fr Core database types - the foundation of all database operations
///
/// This module provides the fundamental types that all database packages use.
/// These are the basic building blocks periodt!

use std::collections::HashMap;
use std::fmt;
use std::sync::Arc;
use serde::{Serialize, Deserialize};

/// fr fr Database value type - represents any value stored in database
#[derive(Debug, Clone, PartialEq)]
pub enum DatabaseValue {
    Null,
    Bool(bool),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    UInt8(u8),
    UInt16(u16),
    UInt32(u32),
    UInt64(u64),
    Float32(f32),
    Float64(f64),
    String(String),
    Bytes(Vec<u8>),
    Date(chrono::NaiveDate),
    Time(chrono::NaiveTime),
    DateTime(chrono::NaiveDateTime),
    Timestamp(chrono::DateTime<chrono::Utc>),
    Json(serde_json::Value),
    Uuid(uuid::Uuid),
    Array(Vec<DatabaseValue>),
    Map(HashMap<String, DatabaseValue>),
}

impl fmt::Display for DatabaseValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DatabaseValue::Null => write!(f, "NULL"),
            DatabaseValue::Bool(b) => write!(f, "{}", b),
            DatabaseValue::Int8(i) => write!(f, "{}", i),
            DatabaseValue::Int16(i) => write!(f, "{}", i),
            DatabaseValue::Int32(i) => write!(f, "{}", i),
            DatabaseValue::Int64(i) => write!(f, "{}", i),
            DatabaseValue::UInt8(i) => write!(f, "{}", i),
            DatabaseValue::UInt16(i) => write!(f, "{}", i),
            DatabaseValue::UInt32(i) => write!(f, "{}", i),
            DatabaseValue::UInt64(i) => write!(f, "{}", i),
            DatabaseValue::Float32(f_val) => write!(f, "{}", f_val),
            DatabaseValue::Float64(f_val) => write!(f, "{}", f_val),
            DatabaseValue::String(s) => write!(f, "'{}'", s),
            DatabaseValue::Bytes(b) => write!(f, "BYTES[{}]", b.len()),
            DatabaseValue::Date(d) => write!(f, "{}", d),
            DatabaseValue::Time(t) => write!(f, "{}", t),
            DatabaseValue::DateTime(dt) => write!(f, "{}", dt),
            DatabaseValue::Timestamp(ts) => write!(f, "{}", ts),
            DatabaseValue::Json(j) => write!(f, "{}", j),
            DatabaseValue::Uuid(u) => write!(f, "{}", u),
            DatabaseValue::Array(arr) => write!(f, "[{}]", arr.len()),
            DatabaseValue::Map(m) => write!(f, "{{{}}}", m.len()),
        }
    }
}

/// fr fr Column metadata
#[derive(Debug, Clone, PartialEq)]
pub struct ColumnMetadata {
    pub name: String,
    pub column_type: ColumnType,
    pub is_nullable: bool,
    pub is_primary_key: bool,
    pub is_unique: bool,
    pub is_auto_increment: bool,
    pub default_value: Option<DatabaseValue>,
    pub max_length: Option<usize>,
    pub precision: Option<u32>,
    pub scale: Option<u32>,
    pub table_name: Option<String>,
    pub schema_name: Option<String>,
}

/// fr fr Column type enumeration
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ColumnType {
    // Numeric types
    TinyInt,
    SmallInt,
    Int,
    BigInt,
    Float,
    Double,
    Decimal { precision: u32, scale: u32 },
    
    // String types
    Char { length: u32 },
    VarChar { max_length: Option<u32> },
    Text,
    
    // Binary types
    Binary { length: u32 },
    VarBinary { max_length: Option<u32> },
    Blob,
    
    // Date/Time types
    Date,
    Time,
    DateTime,
    Timestamp,
    
    // Boolean type
    Boolean,
    
    // JSON type
    Json,
    
    // UUID type
    Uuid,
    
    // Array type
    Array(Box<ColumnType>),
    
    // Custom/Unknown type
    Custom(String),
}

impl fmt::Display for ColumnType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColumnType::TinyInt => write!(f, "TINYINT"),
            ColumnType::SmallInt => write!(f, "SMALLINT"),
            ColumnType::Int => write!(f, "INT"),
            ColumnType::BigInt => write!(f, "BIGINT"),
            ColumnType::Float => write!(f, "FLOAT"),
            ColumnType::Double => write!(f, "DOUBLE"),
            ColumnType::Decimal { precision, scale } => write!(f, "DECIMAL({},{})", precision, scale),
            ColumnType::Char { length } => write!(f, "CHAR({})", length),
            ColumnType::VarChar { max_length } => {
                if let Some(len) = max_length {
                    write!(f, "VARCHAR({})", len)
                } else {
                    write!(f, "VARCHAR")
                }
            },
            ColumnType::Text => write!(f, "TEXT"),
            ColumnType::Binary { length } => write!(f, "BINARY({})", length),
            ColumnType::VarBinary { max_length } => {
                if let Some(len) = max_length {
                    write!(f, "VARBINARY({})", len)
                } else {
                    write!(f, "VARBINARY")
                }
            },
            ColumnType::Blob => write!(f, "BLOB"),
            ColumnType::Date => write!(f, "DATE"),
            ColumnType::Time => write!(f, "TIME"),
            ColumnType::DateTime => write!(f, "DATETIME"),
            ColumnType::Timestamp => write!(f, "TIMESTAMP"),
            ColumnType::Boolean => write!(f, "BOOLEAN"),
            ColumnType::Json => write!(f, "JSON"),
            ColumnType::Uuid => write!(f, "UUID"),
            ColumnType::Array(inner) => write!(f, "ARRAY[{}]", inner),
            ColumnType::Custom(name) => write!(f, "{}", name),
        }
    }
}

/// fr fr Row value container
#[derive(Debug, Clone)]
pub struct RowValue {
    pub columns: Vec<ColumnMetadata>,
    pub values: Vec<DatabaseValue>,
}

impl RowValue {
    /// slay Create a new row
    pub fn new() -> Self {
        Self {
            columns: Vec::new(),
            values: Vec::new(),
        }
    }

    /// slay Add a column with value
    pub fn add_column(&mut self, column: ColumnMetadata, value: DatabaseValue) {
        self.columns.push(column);
        self.values.push(value);
    }

    /// slay Get value by column name
    pub fn get(&self, column_name: &str) -> Option<&DatabaseValue> {
        self.columns
            .iter()
            .position(|col| col.name == column_name)
            .and_then(|idx| self.values.get(idx))
    }

    /// slay Get value by index
    pub fn get_by_index(&self, index: usize) -> Option<&DatabaseValue> {
        self.values.get(index)
    }

    /// slay Get column count
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// slay Check if row is empty
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
}

impl Default for RowValue {
    fn default() -> Self {
        Self::new()
    }
}

/// fr fr Query parameter
#[derive(Debug, Clone)]
pub struct QueryParameter {
    pub name: Option<String>,
    pub value: DatabaseValue,
    pub parameter_type: ColumnType,
}

impl QueryParameter {
    /// slay Create a named parameter
    pub fn named(name: &str, value: DatabaseValue, param_type: ColumnType) -> Self {
        Self {
            name: Some(name.to_string()),
            value,
            parameter_type: param_type,
        }
    }

    /// slay Create a positional parameter
    pub fn positional(value: DatabaseValue, param_type: ColumnType) -> Self {
        Self {
            name: None,
            value,
            parameter_type: param_type,
        }
    }
}

/// fr fr Connection state
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConnectionState {
    Connecting,
    Connected,
    Idle,
    InTransaction,
    Closed,
    Error,
}

impl fmt::Display for ConnectionState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConnectionState::Connecting => write!(f, "CONNECTING"),
            ConnectionState::Connected => write!(f, "CONNECTED"),
            ConnectionState::Idle => write!(f, "IDLE"),
            ConnectionState::InTransaction => write!(f, "IN_TRANSACTION"),
            ConnectionState::Closed => write!(f, "CLOSED"),
            ConnectionState::Error => write!(f, "ERROR"),
        }
    }
}

/// fr fr Database type conversion helpers
impl DatabaseValue {
    /// slay Convert to boolean
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            DatabaseValue::Bool(b) => Some(*b),
            DatabaseValue::Int32(i) => Some(*i != 0),
            DatabaseValue::Int64(i) => Some(*i != 0),
            DatabaseValue::String(s) => {
                match s.to_lowercase().as_str() {
                    "true" | "t" | "yes" | "y" | "1" => Some(true),
                    "false" | "f" | "no" | "n" | "0" => Some(false),
                    _ => None,
                }
            },
            _ => None,
        }
    }

    /// slay Convert to integer
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            DatabaseValue::Int8(i) => Some(*i as i64),
            DatabaseValue::Int16(i) => Some(*i as i64),
            DatabaseValue::Int32(i) => Some(*i as i64),
            DatabaseValue::Int64(i) => Some(*i),
            DatabaseValue::UInt8(i) => Some(*i as i64),
            DatabaseValue::UInt16(i) => Some(*i as i64),
            DatabaseValue::UInt32(i) => Some(*i as i64),
            DatabaseValue::UInt64(i) => {
                if *i <= i64::MAX as u64 {
                    Some(*i as i64)
                } else {
                    None
                }
            },
            DatabaseValue::Float32(f) => Some(*f as i64),
            DatabaseValue::Float64(f) => Some(*f as i64),
            DatabaseValue::String(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// slay Convert to string
    pub fn as_string(&self) -> Option<String> {
        match self {
            DatabaseValue::String(s) => Some(s.clone()),
            DatabaseValue::Null => Some("NULL".to_string()),
            _ => Some(self.to_string()),
        }
    }

    /// slay Check if value is null
    pub fn is_null(&self) -> bool {
        matches!(self, DatabaseValue::Null)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_database_value_display() {
        assert_eq!(DatabaseValue::Null.to_string(), "NULL");
        assert_eq!(DatabaseValue::Bool(true).to_string(), "true");
        assert_eq!(DatabaseValue::String("test".to_string()).to_string(), "'test'");
    }

    #[test]
    fn test_database_value_conversions() {
        let val = DatabaseValue::Int32(42);
        assert_eq!(val.as_i64(), Some(42));
        assert_eq!(val.as_bool(), Some(true));

        let val = DatabaseValue::String("true".to_string());
        assert_eq!(val.as_bool(), Some(true));
    }

    #[test]
    fn test_row_value() {
        let mut row = RowValue::new();
        let col = ColumnMetadata {
            name: "id".to_string(),
            column_type: ColumnType::Int,
            is_nullable: false,
            is_primary_key: true,
            is_unique: true,
            is_auto_increment: true,
            default_value: None,
            max_length: None,
            precision: None,
            scale: None,
            table_name: Some("users".to_string()),
            schema_name: None,
        };
        
        row.add_column(col, DatabaseValue::Int32(1));
        assert_eq!(row.column_count(), 1);
        assert_eq!(row.get("id"), Some(&DatabaseValue::Int32(1)));
    }

    #[test]
    fn test_column_type_display() {
        assert_eq!(ColumnType::Int.to_string(), "INT");
        assert_eq!(ColumnType::VarChar { max_length: Some(255) }.to_string(), "VARCHAR(255)");
        assert_eq!(ColumnType::Decimal { precision: 10, scale: 2 }.to_string(), "DECIMAL(10,2)");
    }
}
