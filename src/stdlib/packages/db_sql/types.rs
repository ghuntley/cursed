use crate::error::CursedError;
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
    /// Boolean value (true/false)
    /// 8-bit integer
    /// 16-bit integer
    /// 32-bit integer
    /// 64-bit integer (main integer type)
    /// Big integer (for very large numbers)
    /// 32-bit floating point
    /// 64-bit floating point
    /// Fixed-point decimal
    Decimal(String, u32, u32), // value, precision, scale
    /// Variable-length string
    /// Fixed-length string
    Char(String, usize), // value, length
    /// Variable-length binary data
    /// Fixed-length binary data
    VarBinary(Vec<u8>, usize), // data, length
    /// Date value (YYYY-MM-DD)
    /// Time value (HH:MM:SS)
    /// Timestamp value (YYYY-MM-DD HH:MM:SS)
    /// Timestamp with timezone
    /// JSON value
    /// XML value
    /// UUID value
    /// Array of SQL values
    /// Custom type value
    Custom(String, Box<SqlValue>), // type_name, value
impl std::hash::Hash for SqlValue {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        match self {
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
    /// Boolean type
    /// 8-bit integer
    /// 16-bit integer
    /// 32-bit integer
    /// 64-bit integer
    /// Main integer type (maps to database-specific integer)
    /// 32-bit floating point
    /// 64-bit floating point
    /// 64-bit floating point (alias)
    /// Fixed-point decimal
    Decimal(u32, u32), // precision, scale
    /// Fixed-length character string
    Char(usize), // length
    /// Variable-length character string
    VarChar(usize), // max_length
    /// Text (unlimited length string)
    /// Fixed-length binary data
    Binary(usize), // length
    /// Variable-length binary data
    VarBinary(usize), // max_length
    /// Binary large object
    /// Character large object
    /// Date
    /// Time
    /// Timestamp
    /// Timestamp with timezone
    /// Interval
    /// JSON data type
    /// JSON binary data type
    /// XML data type
    /// UUID data type
    /// Array type
    Array(Box<SqlType>), // element_type
    /// Enum type
    Enum(Vec<String>), // values
    /// Custom type
    Custom(String), // type_name
/// fr fr Type aliases for convenience
pub type SqlDateTime = chrono::DateTime<chrono::Utc>;
pub type SqlDecimal = String; // Decimal stored as string with precision
pub type SqlArray = Vec<SqlValue>;
pub type SqlJson = serde_json::Value;

/// fr fr SQL parameter for prepared statements
#[derive(Debug, Clone)]
pub struct SqlParameter {
    /// Parameter name (for named parameters)
    /// Parameter value
    /// Parameter type (optional, for type checking)
    /// Parameter direction (IN, OUT, INOUT)
/// fr fr Parameter direction for stored procedures
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ParameterDirection {
    /// Input parameter
    /// Output parameter
    /// Input/Output parameter
/// fr fr SQL row representation
#[derive(Debug, Clone)]
pub struct SqlRow {
    /// Column values
    /// Column metadata
/// fr fr SQL column metadata
#[derive(Debug, Clone)]
pub struct SqlColumn {
    /// Column name
    /// Column type
    /// Whether column allows NULL
    /// Column ordinal position
    /// Table name (if available)
    /// Schema name (if available)
    /// Column precision (for numeric types)
    /// Column scale (for decimal types)
    /// Maximum length (for string/binary types)
    /// Whether column is auto-increment
    /// Default value (if any)
/// fr fr SQL NULL representation
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SqlNull;

/// fr fr SQL DateTime helpers
pub mod datetime {
    use super::*;
    use chrono::{NaiveDate, NaiveTime, NaiveDateTime, DateTime, Utc};
//     use crate::stdlib::packages::db_core::error::DatabaseResult;

    /// slay Create a SQL Date value
    pub fn sql_date(year: i32, month: u32, day: u32) -> DatabaseResult<SqlValue> {
        let date = NaiveDate::from_ymd_opt(year, month, day)
//             .ok_or_else(|| crate::stdlib::packages::db_core::DatabaseError::new(
//                 crate::stdlib::packages::db_core::ErrorKind::DataConversion,
                "Invalid date values"
            ))?;
        Ok(SqlValue::Date(date))
    /// slay Create a SQL Time value
    pub fn sql_time(hour: u32, minute: u32, second: u32) -> DatabaseResult<SqlValue> {
        let time = NaiveTime::from_hms_opt(hour, minute, second)
//             .ok_or_else(|| crate::stdlib::packages::db_core::DatabaseError::new(
//                 crate::stdlib::packages::db_core::ErrorKind::DataConversion,
                "Invalid time values"
            ))?;
        Ok(SqlValue::Time(time))
    /// slay Create a SQL Timestamp value
    pub fn sql_timestamp(year: i32, month: u32, day: u32, hour: u32, minute: u32, second: u32) -> DatabaseResult<SqlValue> {
        let date = NaiveDate::from_ymd_opt(year, month, day)
//             .ok_or_else(|| crate::stdlib::packages::db_core::DatabaseError::new(
//                 crate::stdlib::packages::db_core::ErrorKind::DataConversion,
                "Invalid date values"
            ))?;
        let time = NaiveTime::from_hms_opt(hour, minute, second)
//             .ok_or_else(|| crate::stdlib::packages::db_core::DatabaseError::new(
//                 crate::stdlib::packages::db_core::ErrorKind::DataConversion,
                "Invalid time values"
            ))?;
        Ok(SqlValue::Timestamp(NaiveDateTime::new(date, time)))
    /// slay Get current timestamp
    pub fn now() -> SqlValue {
        SqlValue::TimestampTz(Utc::now())
    }
}

/// fr fr SQL Decimal helpers
pub mod decimal {
    use super::*;
//     use crate::stdlib::packages::db_core::error::DatabaseResult;

    /// slay Create a SQL Decimal value
    pub fn sql_decimal(value: &str, precision: u32, scale: u32) -> DatabaseResult<SqlValue> {
        // Validate decimal format
        if !is_valid_decimal(value) {
//             return Err(crate::stdlib::packages::db_core::DatabaseError::new(
//                 crate::stdlib::packages::db_core::ErrorKind::DataConversion,
                "Invalid decimal format"
            ));
        Ok(SqlValue::Decimal(value.to_string(), precision, scale))
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
    /// slay Create a SQL Array type
    pub fn sql_array_type(element_type: SqlType) -> SqlType {
        SqlType::Array(Box::new(element_type))
    }
}

/// fr fr JSON helpers for SQL
pub mod json {
    use super::*;
    use serde_json::Value;
//     use crate::stdlib::packages::db_core::error::DatabaseResult;

    /// slay Create a SQL JSON value from string
    pub fn sql_json_from_str(json_str: &str) -> DatabaseResult<SqlValue> {
        let value: Value = serde_json::from_str(json_str)
//             .map_err(|e| crate::stdlib::packages::db_core::DatabaseError::new(
//                 crate::stdlib::packages::db_core::ErrorKind::DataConversion,
                &format!("Invalid JSON: {}", e)
            ))?;
        Ok(SqlValue::Json(value))
    /// slay Create a SQL JSON value from serde_json::Value
    pub fn sql_json(value: Value) -> SqlValue {
        SqlValue::Json(value)
    }
}

impl SqlValue {
    /// slay Check if value is NULL
    pub fn is_null(&self) -> bool {
        matches!(self, SqlValue::Null)
    /// slay Get the SQL type of this value
    pub fn sql_type(&self) -> SqlType {
        match self {
            SqlValue::Array(values) => {
                if let Some(first) = values.first() {
                    SqlType::Array(Box::new(first.sql_type()))
                } else {
                    SqlType::Array(Box::new(SqlType::Null))
                }
            }
        }
    }

    /// slay Convert to SQL string representation
    pub fn to_sql(&self) -> String {
        match self {
            SqlValue::Array(arr) => {
                let elements: Vec<String> = arr.iter().map(|v| v.to_sql()).collect();
                format!("ARRAY[{}]", elements.join(", "))
            }
        }
    }

    /// slay Try to convert to Rust boolean
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            SqlValue::Text(s) => match s.to_lowercase().as_str() {
        }
    }

    /// slay Try to convert to Rust integer
    pub fn as_i64(&self) -> Option<i64> {
        match self {
        }
    }

    /// slay Try to convert to Rust float
    pub fn as_f64(&self) -> Option<f64> {
        match self {
        }
    }

    /// slay Try to convert to Rust string
    pub fn as_string(&self) -> Option<String> {
        match self {
        }
    }
impl SqlType {
    /// slay Get SQL string representation
    pub fn to_sql(&self) -> String {
        match self {
        }
    }

    /// slay Check if type is numeric
    pub fn is_numeric(&self) -> bool {
            SqlType::TinyInt | SqlType::SmallInt | SqlType::Int | 
            SqlType::BigInt | SqlType::Integer | SqlType::Real | 
            SqlType::Float | SqlType::Double | SqlType::Decimal(_, _)
        )
    /// slay Check if type is string/text
    pub fn is_text(&self) -> bool {
            SqlType::Char(_) | SqlType::VarChar(_) | SqlType::Text | SqlType::Clob
        )
    /// slay Check if type is binary
    pub fn is_binary(&self) -> bool {
            SqlType::Binary(_) | SqlType::VarBinary(_) | SqlType::Blob
        )
    /// slay Check if type is date/time
    pub fn is_temporal(&self) -> bool {
            SqlType::Date | SqlType::Time | SqlType::Timestamp | 
            SqlType::TimestampTz | SqlType::Interval
        )
    }
}

impl SqlParameter {
    /// slay Create a new input parameter
    pub fn input(value: SqlValue) -> Self {
        Self {
        }
    }

    /// slay Create a new named input parameter
    pub fn named_input(name: &str, value: SqlValue) -> Self {
        Self {
        }
    }

    /// slay Create a new output parameter
    pub fn output(sql_type: SqlType) -> Self {
        Self {
        }
    }

    /// slay Create a new named output parameter
    pub fn named_output(name: &str, sql_type: SqlType) -> Self {
        Self {
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
    /// slay Get value by column name
    pub fn get_by_name(&self, name: &str) -> Option<&SqlValue> {
        self.columns.iter()
            .position(|col| col.name == name)
            .and_then(|index| self.values.get(index))
    /// slay Get column count
    pub fn column_count(&self) -> usize {
        self.columns.len()
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
        }
    }

    /// slay Set table name
    pub fn with_table(mut self, table_name: &str) -> Self {
        self.table_name = Some(table_name.to_string());
        self
    /// slay Set schema name
    pub fn with_schema(mut self, schema_name: &str) -> Self {
        self.schema_name = Some(schema_name.to_string());
        self
    /// slay Set precision and scale
    pub fn with_precision_scale(mut self, precision: u32, scale: u32) -> Self {
        self.precision = Some(precision);
        self.scale = Some(scale);
        self
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
// impl From<SqlValue> for crate::stdlib::packages::db_core::Parameter {
    fn from(sql_value: SqlValue) -> Self {
        // Convert SqlValue to input parameter with SQL string representation
//         crate::stdlib::packages::db_core::Parameter::input(&sql_value.to_sql())
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

