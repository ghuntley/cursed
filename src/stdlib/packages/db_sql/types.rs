/// fr fr SQL data types and values - the foundation of all SQL operations periodt
///
/// This module defines all the SQL data types, values, and conversion logic
/// needed for database operations. Type safety is crucial bestie!

use std::collections::HashMap;
use std::fmt;

/// fr fr SQL value types - all the data types we support
#[derive(Debug, Clone, PartialEq)]
pub enum SqlValue {
    /// NULL value
    Null,
    /// Boolean value (true/false)
    Boolean(bool),
    /// 8-bit integer
    TinyInt(i8),
    /// 16-bit integer
    SmallInt(i16),
    /// 32-bit integer
    Int(i32),
    /// 64-bit integer (main integer type)
    Integer(i64),
    /// Big integer (for very large numbers)
    BigInt(i128),
    /// 32-bit floating point
    Float(f64),
    /// 64-bit floating point
    Double(f64),
    /// Fixed-point decimal
    Decimal(String, u32, u32), // value, precision, scale
    /// Variable-length string
    Text(String),
    /// Fixed-length string
    Char(String, usize), // value, length
    /// Variable-length binary data
    Binary(Vec<u8>),
    /// Fixed-length binary data
    VarBinary(Vec<u8>, usize), // data, length
    /// Date value (YYYY-MM-DD)
    Date(chrono::NaiveDate),
    /// Time value (HH:MM:SS)
    Time(chrono::NaiveTime),
    /// Timestamp value (YYYY-MM-DD HH:MM:SS)
    Timestamp(chrono::NaiveDateTime),
    /// Timestamp with timezone
    TimestampTz(chrono::DateTime<chrono::Utc>),
    /// JSON value
    Json(serde_json::Value),
    /// XML value
    Xml(String),
    /// UUID value
    Uuid(uuid::Uuid),
    /// Array of SQL values
    Array(Vec<SqlValue>),
    /// Custom type value
    Custom(String, Box<SqlValue>), // type_name, value
}

impl Eq for SqlValue {}

impl std::hash::Hash for SqlValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
            SqlValue::Null => 0u8.hash(state),
            SqlValue::Boolean(b) => {
                1u8.hash(state);
                b.hash(state);
            }
            SqlValue::TinyInt(i) => {
                2u8.hash(state);
                i.hash(state);
            }
            SqlValue::SmallInt(i) => {
                3u8.hash(state);
                i.hash(state);
            }
            SqlValue::Int(i) => {
                4u8.hash(state);
                i.hash(state);
            }
            SqlValue::Integer(i) => {
                5u8.hash(state);
                i.hash(state);
            }
            SqlValue::BigInt(i) => {
                6u8.hash(state);
                i.hash(state);
            }
            SqlValue::Float(f) | SqlValue::Double(f) => {
                // Handle NaN and infinity cases for f64
                if f.is_nan() {
                    7u8.hash(state);
                    "NaN".hash(state);
                } else if f.is_infinite() {
                    8u8.hash(state);
                    f.is_sign_positive().hash(state);
                    "Infinity".hash(state);
                } else {
                    9u8.hash(state);
                    f.to_bits().hash(state);
                }
            }
            SqlValue::Decimal(s, p, scale) => {
                10u8.hash(state);
                s.hash(state);
                p.hash(state);
                scale.hash(state);
            }
            SqlValue::Text(s) => {
                11u8.hash(state);
                s.hash(state);
            }
            SqlValue::Char(s, len) => {
                12u8.hash(state);
                s.hash(state);
                len.hash(state);
            }
            SqlValue::Binary(b) => {
                13u8.hash(state);
                b.hash(state);
            }
            SqlValue::VarBinary(b, len) => {
                14u8.hash(state);
                b.hash(state);
                len.hash(state);
            }
            SqlValue::Date(d) => {
                15u8.hash(state);
                d.hash(state);
            }
            SqlValue::Timestamp(ts) => {
                16u8.hash(state);
                ts.hash(state);
            }
            SqlValue::TimestampTz(ts) => {
                17u8.hash(state);
                ts.hash(state);
            }
            SqlValue::Time(t) => {
                18u8.hash(state);
                t.hash(state);
            }
            SqlValue::Json(j) => {
                19u8.hash(state);
                // Hash the JSON as string representation
                j.to_string().hash(state);
            }
            SqlValue::Xml(s) => {
                20u8.hash(state);
                s.hash(state);
            }
            SqlValue::Uuid(u) => {
                21u8.hash(state);
                u.hash(state);
            }
            SqlValue::Array(arr) => {
                22u8.hash(state);
                arr.hash(state);
            }
            SqlValue::Custom(name, val) => {
                23u8.hash(state);
                name.hash(state);
                val.hash(state);
            }
        }
    }
}

/// fr fr SQL data types for schema definition
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SqlType {
    /// NULL type
    Null,
    /// Boolean type
    Boolean,
    /// 8-bit integer
    TinyInt,
    /// 16-bit integer
    SmallInt,
    /// 32-bit integer
    Int,
    /// 64-bit integer
    BigInt,
    /// Main integer type (maps to database-specific integer)
    Integer,
    /// 32-bit floating point
    Real,
    /// 64-bit floating point
    Float,
    /// 64-bit floating point (alias)
    Double,
    /// Fixed-point decimal
    Decimal(u32, u32), // precision, scale
    /// Fixed-length character string
    Char(usize), // length
    /// Variable-length character string
    VarChar(usize), // max_length
    /// Text (unlimited length string)
    Text,
    /// Fixed-length binary data
    Binary(usize), // length
    /// Variable-length binary data
    VarBinary(usize), // max_length
    /// Binary large object
    Blob,
    /// Character large object
    Clob,
    /// Date
    Date,
    /// Time
    Time,
    /// Timestamp
    Timestamp,
    /// Timestamp with timezone
    TimestampTz,
    /// Interval
    Interval,
    /// JSON data type
    Json,
    /// JSON binary data type
    JsonB,
    /// XML data type
    Xml,
    /// UUID data type
    Uuid,
    /// Array type
    Array(Box<SqlType>), // element_type
    /// Enum type
    Enum(Vec<String>), // values
    /// Custom type
    Custom(String), // type_name
}

/// fr fr Type aliases for convenience
pub type SqlDateTime = chrono::DateTime<chrono::Utc>;
pub type SqlDecimal = String; // Decimal stored as string with precision
pub type SqlArray = Vec<SqlValue>;
pub type SqlJson = serde_json::Value;

/// fr fr SQL parameter for prepared statements
#[derive(Debug, Clone)]
pub struct SqlParameter {
    /// Parameter name (for named parameters)
    pub name: Option<String>,
    /// Parameter value
    pub value: SqlValue,
    /// Parameter type (optional, for type checking)
    pub sql_type: Option<SqlType>,
    /// Parameter direction (IN, OUT, INOUT)
    pub direction: ParameterDirection,
}

/// fr fr Parameter direction for stored procedures
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterDirection {
    /// Input parameter
    In,
    /// Output parameter
    Out,
    /// Input/Output parameter
    InOut,
}

/// fr fr SQL row representation
#[derive(Debug, Clone)]
pub struct SqlRow {
    /// Column values
    pub values: Vec<SqlValue>,
    /// Column metadata
    pub columns: Vec<SqlColumn>,
}

/// fr fr SQL column metadata
#[derive(Debug, Clone)]
pub struct SqlColumn {
    /// Column name
    pub name: String,
    /// Column type
    pub sql_type: SqlType,
    /// Whether column allows NULL
    pub nullable: bool,
    /// Column ordinal position
    pub ordinal: usize,
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
    /// Default value (if any)
    pub default_value: Option<SqlValue>,
}

/// fr fr SQL NULL representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqlNull;

/// fr fr SQL DateTime helpers
pub mod datetime {
    use super::*;
    use chrono::{NaiveDate, NaiveTime, NaiveDateTime, DateTime, Utc};
    use crate::stdlib::packages::db_core::error::DatabaseResult;

    /// slay Create a SQL Date value
    pub fn sql_date(year: i32, month: u32, day: u32) -> DatabaseResult<SqlValue> {
        let date = NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| crate::stdlib::packages::db_core::DatabaseError::new(
                crate::stdlib::packages::db_core::ErrorKind::DataConversion,
                "Invalid date values"
            ))?;
        Ok(SqlValue::Date(date))
    }

    /// slay Create a SQL Time value
    pub fn sql_time(hour: u32, minute: u32, second: u32) -> DatabaseResult<SqlValue> {
        let time = NaiveTime::from_hms_opt(hour, minute, second)
            .ok_or_else(|| crate::stdlib::packages::db_core::DatabaseError::new(
                crate::stdlib::packages::db_core::ErrorKind::DataConversion,
                "Invalid time values"
            ))?;
        Ok(SqlValue::Time(time))
    }

    /// slay Create a SQL Timestamp value
    pub fn sql_timestamp(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> DatabaseResult<SqlValue> {
        let date = NaiveDate::from_ymd_opt(year, month, day)
            .ok_or_else(|| crate::stdlib::packages::db_core::DatabaseError::new(
                crate::stdlib::packages::db_core::ErrorKind::DataConversion,
                "Invalid date values"
            ))?;
        let time = NaiveTime::from_hms_opt(hour, minute, second)
            .ok_or_else(|| crate::stdlib::packages::db_core::DatabaseError::new(
                crate::stdlib::packages::db_core::ErrorKind::DataConversion,
                "Invalid time values"
            ))?;
        Ok(SqlValue::Timestamp(NaiveDateTime::new(date, time)))
    }

    /// slay Get current timestamp
    pub fn now() -> SqlValue {
        SqlValue::TimestampTz(Utc::now())
    }
}

/// fr fr SQL Decimal helpers
pub mod decimal {
    use super::*;
    use crate::stdlib::packages::db_core::error::DatabaseResult;

    /// slay Create a SQL Decimal value
    pub fn sql_decimal(value: &str, precision: u32, scale: u32) -> DatabaseResult<SqlValue> {
        // Validate decimal format
        if !is_valid_decimal(value) {
            return Err(crate::stdlib::packages::db_core::DatabaseError::new(
                crate::stdlib::packages::db_core::ErrorKind::DataConversion,
                "Invalid decimal format"
            ));
        }
        
        Ok(SqlValue::Decimal(value.to_string(), precision, scale))
    }

    fn is_valid_decimal(value: &str) -> bool {
        value.chars().all(|c| c.is_ascii_digit() || c == '.' || c == '-' || c == '+')
    }
}

/// fr fr SQL Array helpers
pub mod array {
    use super::*;

    /// slay Create a SQL Array value
    pub fn sql_array(values: Vec<SqlValue>) -> SqlValue {
        SqlValue::Array(values)
    }

    /// slay Create a SQL Array type
    pub fn sql_array_type(element_type: SqlType) -> SqlType {
        SqlType::Array(Box::new(element_type))
    }
}

/// fr fr JSON helpers for SQL
pub mod json {
    use super::*;
    use serde_json::Value;
    use crate::stdlib::packages::db_core::error::DatabaseResult;

    /// slay Create a SQL JSON value from string
    pub fn sql_json_from_str(json_str: &str) -> DatabaseResult<SqlValue> {
        let value: Value = serde_json::from_str(json_str)
            .map_err(|e| crate::stdlib::packages::db_core::DatabaseError::new(
                crate::stdlib::packages::db_core::ErrorKind::DataConversion,
                &format!("Invalid JSON: {}", e)
            ))?;
        Ok(SqlValue::Json(value))
    }

    /// slay Create a SQL JSON value from serde_json::Value
    pub fn sql_json(value: Value) -> SqlValue {
        SqlValue::Json(value)
    }
}

impl SqlValue {
    /// slay Check if value is NULL
    pub fn is_null(&self) -> bool {
        matches!(self, SqlValue::Null)
    }

    /// slay Get the SQL type of this value
    pub fn sql_type(&self) -> SqlType {
        match self {
            SqlValue::Null => SqlType::Null,
            SqlValue::Boolean(_) => SqlType::Boolean,
            SqlValue::TinyInt(_) => SqlType::TinyInt,
            SqlValue::SmallInt(_) => SqlType::SmallInt,
            SqlValue::Int(_) => SqlType::Int,
            SqlValue::Integer(_) => SqlType::Integer,
            SqlValue::BigInt(_) => SqlType::BigInt,
            SqlValue::Float(_) => SqlType::Float,
            SqlValue::Double(_) => SqlType::Double,
            SqlValue::Decimal(_, precision, scale) => SqlType::Decimal(*precision, *scale),
            SqlValue::Text(_) => SqlType::Text,
            SqlValue::Char(_, length) => SqlType::Char(*length),
            SqlValue::Binary(_) => SqlType::Blob,
            SqlValue::VarBinary(_, length) => SqlType::VarBinary(*length),
            SqlValue::Date(_) => SqlType::Date,
            SqlValue::Time(_) => SqlType::Time,
            SqlValue::Timestamp(_) => SqlType::Timestamp,
            SqlValue::TimestampTz(_) => SqlType::TimestampTz,
            SqlValue::Json(_) => SqlType::Json,
            SqlValue::Xml(_) => SqlType::Xml,
            SqlValue::Uuid(_) => SqlType::Uuid,
            SqlValue::Array(values) => {
                if let Some(first) = values.first() {
                    SqlType::Array(Box::new(first.sql_type()))
                } else {
                    SqlType::Array(Box::new(SqlType::Null))
                }
            }
            SqlValue::Custom(type_name, _) => SqlType::Custom(type_name.clone()),
        }
    }

    /// slay Convert to SQL string representation
    pub fn to_sql(&self) -> String {
        match self {
            SqlValue::Null => "NULL".to_string(),
            SqlValue::Boolean(b) => if *b { "TRUE".to_string() } else { "FALSE".to_string() },
            SqlValue::TinyInt(n) => n.to_string(),
            SqlValue::SmallInt(n) => n.to_string(),
            SqlValue::Int(n) => n.to_string(),
            SqlValue::Integer(n) => n.to_string(),
            SqlValue::BigInt(n) => n.to_string(),
            SqlValue::Float(f) => f.to_string(),
            SqlValue::Double(f) => f.to_string(),
            SqlValue::Decimal(s, _, _) => s.clone(),
            SqlValue::Text(s) => format!("'{}'", s.replace('\'', "''")),
            SqlValue::Char(s, _) => format!("'{}'", s.replace('\'', "''")),
            SqlValue::Binary(data) => format!("'\\x{}'", hex::encode(data)),
            SqlValue::VarBinary(data, _) => format!("'\\x{}'", hex::encode(data)),
            SqlValue::Date(d) => format!("'{}'", d.format("%Y-%m-%d")),
            SqlValue::Time(t) => format!("'{}'", t.format("%H:%M:%S")),
            SqlValue::Timestamp(dt) => format!("'{}'", dt.format("%Y-%m-%d %H:%M:%S")),
            SqlValue::TimestampTz(dt) => format!("'{}'", dt.format("%Y-%m-%d %H:%M:%S%z")),
            SqlValue::Json(j) => format!("'{}'", j.to_string().replace('\'', "''")),
            SqlValue::Xml(x) => format!("'{}'", x.replace('\'', "''")),
            SqlValue::Uuid(u) => format!("'{}'", u),
            SqlValue::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| v.to_sql()).collect();
                format!("ARRAY[{}]", elements.join(", "))
            }
            SqlValue::Custom(_, value) => value.to_sql(),
        }
    }

    /// slay Try to convert to Rust boolean
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            SqlValue::Boolean(b) => Some(*b),
            SqlValue::Integer(0) => Some(false),
            SqlValue::Integer(_) => Some(true),
            SqlValue::Text(s) => match s.to_lowercase().as_str() {
                "true" | "t" | "yes" | "y" | "1" => Some(true),
                "false" | "f" | "no" | "n" | "0" => Some(false),
                _ => None,
            },
            _ => None,
        }
    }

    /// slay Try to convert to Rust integer
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            SqlValue::TinyInt(n) => Some(*n as i64),
            SqlValue::SmallInt(n) => Some(*n as i64),
            SqlValue::Int(n) => Some(*n as i64),
            SqlValue::Integer(n) => Some(*n),
            SqlValue::BigInt(n) => (*n).try_into().ok(),
            SqlValue::Float(f) => Some(*f as i64),
            SqlValue::Double(f) => Some(*f as i64),
            SqlValue::Text(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// slay Try to convert to Rust float
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            SqlValue::Float(f) => Some(*f),
            SqlValue::Double(f) => Some(*f),
            SqlValue::TinyInt(n) => Some(*n as f64),
            SqlValue::SmallInt(n) => Some(*n as f64),
            SqlValue::Int(n) => Some(*n as f64),
            SqlValue::Integer(n) => Some(*n as f64),
            SqlValue::Text(s) => s.parse().ok(),
            _ => None,
        }
    }

    /// slay Try to convert to Rust string
    pub fn as_string(&self) -> Option<String> {
        match self {
            SqlValue::Text(s) => Some(s.clone()),
            SqlValue::Char(s, _) => Some(s.clone()),
            SqlValue::Xml(s) => Some(s.clone()),
            _ => Some(self.to_sql()),
        }
    }
}

impl SqlType {
    /// slay Get SQL string representation
    pub fn to_sql(&self) -> String {
        match self {
            SqlType::Null => "NULL".to_string(),
            SqlType::Boolean => "BOOLEAN".to_string(),
            SqlType::TinyInt => "TINYINT".to_string(),
            SqlType::SmallInt => "SMALLINT".to_string(),
            SqlType::Int => "INT".to_string(),
            SqlType::BigInt => "BIGINT".to_string(),
            SqlType::Integer => "INTEGER".to_string(),
            SqlType::Real => "REAL".to_string(),
            SqlType::Float => "FLOAT".to_string(),
            SqlType::Double => "DOUBLE".to_string(),
            SqlType::Decimal(precision, scale) => format!("DECIMAL({}, {})", precision, scale),
            SqlType::Char(length) => format!("CHAR({})", length),
            SqlType::VarChar(length) => format!("VARCHAR({})", length),
            SqlType::Text => "TEXT".to_string(),
            SqlType::Binary(length) => format!("BINARY({})", length),
            SqlType::VarBinary(length) => format!("VARBINARY({})", length),
            SqlType::Blob => "BLOB".to_string(),
            SqlType::Clob => "CLOB".to_string(),
            SqlType::Date => "DATE".to_string(),
            SqlType::Time => "TIME".to_string(),
            SqlType::Timestamp => "TIMESTAMP".to_string(),
            SqlType::TimestampTz => "TIMESTAMPTZ".to_string(),
            SqlType::Interval => "INTERVAL".to_string(),
            SqlType::Json => "JSON".to_string(),
            SqlType::JsonB => "JSONB".to_string(),
            SqlType::Xml => "XML".to_string(),
            SqlType::Uuid => "UUID".to_string(),
            SqlType::Array(element_type) => format!("{}[]", element_type.to_sql()),
            SqlType::Enum(values) => format!("ENUM({})", values.iter().map(|v| format!("'{}'", v)).collect::<Vec<_>>().join(", ")),
            SqlType::Custom(name) => name.clone(),
        }
    }

    /// slay Check if type is numeric
    pub fn is_numeric(&self) -> bool {
        matches!(self, 
            SqlType::TinyInt | SqlType::SmallInt | SqlType::Int | 
            SqlType::BigInt | SqlType::Integer | SqlType::Real | 
            SqlType::Float | SqlType::Double | SqlType::Decimal(_, _)
        )
    }

    /// slay Check if type is string/text
    pub fn is_text(&self) -> bool {
        matches!(self, 
            SqlType::Char(_) | SqlType::VarChar(_) | SqlType::Text | SqlType::Clob
        )
    }

    /// slay Check if type is binary
    pub fn is_binary(&self) -> bool {
        matches!(self, 
            SqlType::Binary(_) | SqlType::VarBinary(_) | SqlType::Blob
        )
    }

    /// slay Check if type is date/time
    pub fn is_temporal(&self) -> bool {
        matches!(self, 
            SqlType::Date | SqlType::Time | SqlType::Timestamp | 
            SqlType::TimestampTz | SqlType::Interval
        )
    }
}

impl SqlParameter {
    /// slay Create a new input parameter
    pub fn input(value: SqlValue) -> Self {
        Self {
            name: None,
            value,
            sql_type: None,
            direction: ParameterDirection::In,
        }
    }

    /// slay Create a new named input parameter
    pub fn named_input(name: &str, value: SqlValue) -> Self {
        Self {
            name: Some(name.to_string()),
            value,
            sql_type: None,
            direction: ParameterDirection::In,
        }
    }

    /// slay Create a new output parameter
    pub fn output(sql_type: SqlType) -> Self {
        Self {
            name: None,
            value: SqlValue::Null,
            sql_type: Some(sql_type),
            direction: ParameterDirection::Out,
        }
    }

    /// slay Create a new named output parameter
    pub fn named_output(name: &str, sql_type: SqlType) -> Self {
        Self {
            name: Some(name.to_string()),
            value: SqlValue::Null,
            sql_type: Some(sql_type),
            direction: ParameterDirection::Out,
        }
    }
}

impl SqlRow {
    /// slay Create a new row
    pub fn new(values: Vec<SqlValue>, columns: Vec<SqlColumn>) -> Self {
        Self { values, columns }
    }

    /// slay Get value by column index
    pub fn get(&self, index: usize) -> Option<&SqlValue> {
        self.values.get(index)
    }

    /// slay Get value by column name
    pub fn get_by_name(&self, name: &str) -> Option<&SqlValue> {
        self.columns.iter()
            .position(|col| col.name == name)
            .and_then(|index| self.values.get(index))
    }

    /// slay Get column count
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }

    /// slay Convert to HashMap for easier access
    pub fn to_map(&self) -> HashMap<String, SqlValue> {
        let mut map = HashMap::new();
        for (i, column) in self.columns.iter().enumerate() {
            if let Some(value) = self.values.get(i) {
                map.insert(column.name.clone(), value.clone());
            }
        }
        map
    }
}

impl SqlColumn {
    /// slay Create a new column
    pub fn new(name: &str, sql_type: SqlType, nullable: bool, ordinal: usize) -> Self {
        Self {
            name: name.to_string(),
            sql_type,
            nullable,
            ordinal,
            table_name: None,
            schema_name: None,
            precision: None,
            scale: None,
            max_length: None,
            auto_increment: false,
            default_value: None,
        }
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

    /// slay Set precision and scale
    pub fn with_precision_scale(mut self, precision: u32, scale: u32) -> Self {
        self.precision = Some(precision);
        self.scale = Some(scale);
        self
    }

    /// slay Set maximum length
    pub fn with_max_length(mut self, max_length: usize) -> Self {
        self.max_length = Some(max_length);
        self
    }
}

/// fr fr Type conversion traits for easier integration
impl From<bool> for SqlValue {
    fn from(value: bool) -> Self {
        SqlValue::Boolean(value)
    }
}

impl From<i8> for SqlValue {
    fn from(value: i8) -> Self {
        SqlValue::TinyInt(value)
    }
}

impl From<i16> for SqlValue {
    fn from(value: i16) -> Self {
        SqlValue::SmallInt(value)
    }
}

impl From<i32> for SqlValue {
    fn from(value: i32) -> Self {
        SqlValue::Int(value)
    }
}

impl From<i64> for SqlValue {
    fn from(value: i64) -> Self {
        SqlValue::Integer(value)
    }
}

impl From<f32> for SqlValue {
    fn from(value: f32) -> Self {
        SqlValue::Float(value as f64)
    }
}

impl From<f64> for SqlValue {
    fn from(value: f64) -> Self {
        SqlValue::Double(value)
    }
}

impl From<String> for SqlValue {
    fn from(value: String) -> Self {
        SqlValue::Text(value)
    }
}

impl From<&str> for SqlValue {
    fn from(value: &str) -> Self {
        SqlValue::Text(value.to_string())
    }
}

impl From<Vec<u8>> for SqlValue {
    fn from(value: Vec<u8>) -> Self {
        SqlValue::Binary(value)
    }
}

impl From<uuid::Uuid> for SqlValue {
    fn from(value: uuid::Uuid) -> Self {
        SqlValue::Uuid(value)
    }
}

impl From<serde_json::Value> for SqlValue {
    fn from(value: serde_json::Value) -> Self {
        SqlValue::Json(value)
    }
}

/// fr fr Conversion to db_core Parameter type
impl From<SqlValue> for crate::stdlib::packages::db_core::Parameter {
    fn from(sql_value: SqlValue) -> Self {
        // Convert SqlValue to input parameter with SQL string representation
        crate::stdlib::packages::db_core::Parameter::input(&sql_value.to_sql())
    }
}

impl fmt::Display for SqlValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_sql())
    }
}

impl fmt::Display for SqlType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_sql())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_value_creation() {
        let text_val = SqlValue::Text("hello".to_string());
        let int_val = SqlValue::Integer(42);
        let bool_val = SqlValue::Boolean(true);
        let null_val = SqlValue::Null;

        assert!(!text_val.is_null());
        assert!(!int_val.is_null());
        assert!(!bool_val.is_null());
        assert!(null_val.is_null());
    }

    #[test]
    fn test_sql_value_type_detection() {
        let text_val = SqlValue::Text("hello".to_string());
        let int_val = SqlValue::Integer(42);
        
        assert_eq!(text_val.sql_type(), SqlType::Text);
        assert_eq!(int_val.sql_type(), SqlType::Integer);
    }

    #[test]
    fn test_sql_value_conversions() {
        let int_val = SqlValue::Integer(42);
        let float_val = SqlValue::Float(3.14);
        let bool_val = SqlValue::Boolean(true);
        let text_val = SqlValue::Text("123".to_string());

        assert_eq!(int_val.as_i64(), Some(42));
        assert_eq!(float_val.as_f64(), Some(3.14));
        assert_eq!(bool_val.as_bool(), Some(true));
        assert_eq!(text_val.as_i64(), Some(123));
    }

    #[test]
    fn test_sql_type_checks() {
        assert!(SqlType::Integer.is_numeric());
        assert!(SqlType::Text.is_text());
        assert!(SqlType::Binary(10).is_binary());
        assert!(SqlType::Timestamp.is_temporal());
        
        assert!(!SqlType::Text.is_numeric());
        assert!(!SqlType::Integer.is_text());
    }

    #[test]
    fn test_sql_parameter_creation() {
        let input_param = SqlParameter::input(SqlValue::Integer(42));
        let named_param = SqlParameter::named_input("id", SqlValue::Integer(1));
        let output_param = SqlParameter::output(SqlType::Text);

        assert_eq!(input_param.direction, ParameterDirection::In);
        assert_eq!(named_param.name, Some("id".to_string()));
        assert_eq!(output_param.direction, ParameterDirection::Out);
    }

    #[test]
    fn test_sql_row_operations() {
        let columns = vec![
            SqlColumn::new("id", SqlType::Integer, false, 0),
            SqlColumn::new("name", SqlType::Text, true, 1),
        ];
        let values = vec![
            SqlValue::Integer(1),
            SqlValue::Text("Alice".to_string()),
        ];
        let row = SqlRow::new(values, columns);

        assert_eq!(row.column_count(), 2);
        assert_eq!(row.get(0), Some(&SqlValue::Integer(1)));
        assert_eq!(row.get_by_name("name"), Some(&SqlValue::Text("Alice".to_string())));

        let map = row.to_map();
        assert_eq!(map.get("id"), Some(&SqlValue::Integer(1)));
        assert_eq!(map.get("name"), Some(&SqlValue::Text("Alice".to_string())));
    }

    #[test]
    fn test_datetime_helpers() {
        let date = datetime::sql_date(2024, 1, 15).unwrap();
        let time = datetime::sql_time(14, 30, 0).unwrap();
        let timestamp = datetime::sql_timestamp(2024, 1, 15, 14, 30, 0).unwrap();

        assert!(matches!(date, SqlValue::Date(_)));
        assert!(matches!(time, SqlValue::Time(_)));
        assert!(matches!(timestamp, SqlValue::Timestamp(_)));
    }

    #[test]
    fn test_json_helpers() {
        let json_val = json::sql_json_from_str(r#"{"name": "Alice", "age": 30}"#).unwrap();
        assert!(matches!(json_val, SqlValue::Json(_)));

        let json_obj = serde_json::json!({"test": true});
        let json_val2 = json::sql_json(json_obj);
        assert!(matches!(json_val2, SqlValue::Json(_)));
    }

    #[test]
    fn test_array_helpers() {
        let arr = array::sql_array(vec![
            SqlValue::Integer(1),
            SqlValue::Integer(2),
            SqlValue::Integer(3),
        ]);
        assert!(matches!(arr, SqlValue::Array(_)));

        let arr_type = array::sql_array_type(SqlType::Integer);
        assert!(matches!(arr_type, SqlType::Array(_)));
    }

    #[test]
    fn test_type_conversions() {
        let bool_val: SqlValue = true.into();
        let int_val: SqlValue = 42i64.into();
        let str_val: SqlValue = "hello".into();
        let string_val: SqlValue = String::from("world").into();

        assert!(matches!(bool_val, SqlValue::Boolean(true)));
        assert!(matches!(int_val, SqlValue::Integer(42)));
        assert!(matches!(str_val, SqlValue::Text(_)));
        assert!(matches!(string_val, SqlValue::Text(_)));
    }
}
