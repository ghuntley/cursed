/// fr fr SQL data types and value representations - type safety periodt
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fmt;

/// fr fr SQL value types that can be used in queries and results
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum SqlValue {
    /// Null value - the void bestie
    Null,
    
    /// Boolean value - true or false vibes
    Boolean(bool),
    
    /// 8-bit signed integer
    TinyInt(i8),
    
    /// 16-bit signed integer  
    SmallInt(i16),
    
    /// 32-bit signed integer - most common integer type
    Integer(i32),
    
    /// 64-bit signed integer - big number energy
    BigInt(i64),
    
    /// 32-bit floating point
    Float(f32),
    
    /// 64-bit floating point - double precision periodt
    Double(f64),
    
    /// Decimal/numeric with precision
    Decimal(String), // Stored as string to maintain precision
    
    /// String/text value - words and stuff
    String(String),
    
    /// Binary data - bytes and such
    Bytes(Vec<u8>),
    
    /// Date value (YYYY-MM-DD)
    Date(chrono::NaiveDate),
    
    /// Time value (HH:MM:SS)
    Time(chrono::NaiveTime),
    
    /// DateTime/timestamp with timezone
    DateTime(chrono::DateTime<chrono::Utc>),
    
    /// JSON value - structured data periodt
    Json(serde_json::Value),
    
    /// UUID - unique identifier vibes
    Uuid(uuid::Uuid),
    
    /// Array of values (for databases that support arrays)
    Array(Vec<SqlValue>),
    
    /// Custom type - extensible for database-specific types
    Custom {
        type_name: String,
        data: Vec<u8>,
    },
}

impl SqlValue {
    /// sus Check if this value is null
    pub fn is_null(&self) -> bool {
        matches!(self, SqlValue::Null)
    }
    
    /// facts Get the SQL type of this value
    pub fn sql_type(&self) -> SqlType {
        match self {
            SqlValue::Null => SqlType::Null,
            SqlValue::Boolean(_) => SqlType::Boolean,
            SqlValue::TinyInt(_) => SqlType::TinyInt,
            SqlValue::SmallInt(_) => SqlType::SmallInt,
            SqlValue::Integer(_) => SqlType::Integer,
            SqlValue::BigInt(_) => SqlType::BigInt,
            SqlValue::Float(_) => SqlType::Float,
            SqlValue::Double(_) => SqlType::Double,
            SqlValue::Decimal(_) => SqlType::Decimal,
            SqlValue::String(_) => SqlType::String,
            SqlValue::Bytes(_) => SqlType::Bytes,
            SqlValue::Date(_) => SqlType::Date,
            SqlValue::Time(_) => SqlType::Time,
            SqlValue::DateTime(_) => SqlType::DateTime,
            SqlValue::Json(_) => SqlType::Json,
            SqlValue::Uuid(_) => SqlType::Uuid,
            SqlValue::Array(_) => SqlType::Array,
            SqlValue::Custom { type_name, .. } => SqlType::Custom(type_name.clone()),
        }
    }
    
    /// lowkey Convert to string representation
    pub fn to_string_representation(&self) -> String {
        match self {
            SqlValue::Null => "NULL".to_string(),
            SqlValue::Boolean(b) => b.to_string(),
            SqlValue::TinyInt(i) => i.to_string(),
            SqlValue::SmallInt(i) => i.to_string(),
            SqlValue::Integer(i) => i.to_string(),
            SqlValue::BigInt(i) => i.to_string(),
            SqlValue::Float(f) => f.to_string(),
            SqlValue::Double(f) => f.to_string(),
            SqlValue::Decimal(d) => d.clone(),
            SqlValue::String(s) => format!("'{}'", s.replace('\'', "''")), // SQL escape
            SqlValue::Bytes(b) => format!("\\x{}", hex::encode(b)),
            SqlValue::Date(d) => format!("'{}'", d.format("%Y-%m-%d")),
            SqlValue::Time(t) => format!("'{}'", t.format("%H:%M:%S")),
            SqlValue::DateTime(dt) => format!("'{}'", dt.format("%Y-%m-%d %H:%M:%S UTC")),
            SqlValue::Json(j) => format!("'{}'", j.to_string().replace('\'', "''")),
            SqlValue::Uuid(u) => format!("'{}'", u.to_string()),
            SqlValue::Array(arr) => {
                let items: Vec<String> = arr.iter().map(|v| v.to_string_representation()).collect();
                format!("ARRAY[{}]", items.join(", "))
            }
            SqlValue::Custom { type_name, .. } => format!("{}(...)", type_name),
        }
    }
    
    /// highkey Try to convert to specific Rust type
    pub fn as_bool(&self) -> Option<bool> {
        match self {
            SqlValue::Boolean(b) => Some(*b),
            _ => None,
        }
    }
    
    /// periodt Try to convert to i32
    pub fn as_i32(&self) -> Option<i32> {
        match self {
            SqlValue::Integer(i) => Some(*i),
            SqlValue::TinyInt(i) => Some(*i as i32),
            SqlValue::SmallInt(i) => Some(*i as i32),
            _ => None,
        }
    }
    
    /// bestie Try to convert to i64
    pub fn as_i64(&self) -> Option<i64> {
        match self {
            SqlValue::BigInt(i) => Some(*i),
            SqlValue::Integer(i) => Some(*i as i64),
            SqlValue::TinyInt(i) => Some(*i as i64),
            SqlValue::SmallInt(i) => Some(*i as i64),
            _ => None,
        }
    }
    
    /// flex Try to convert to f64
    pub fn as_f64(&self) -> Option<f64> {
        match self {
            SqlValue::Double(f) => Some(*f),
            SqlValue::Float(f) => Some(*f as f64),
            _ => None,
        }
    }
    
    /// yolo Try to convert to string
    pub fn as_string(&self) -> Option<String> {
        match self {
            SqlValue::String(s) => Some(s.clone()),
            _ => None,
        }
    }
}

impl fmt::Display for SqlValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string_representation())
    }
}

/// fr fr SQL data types - schema definition vibes
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum SqlType {
    /// NULL type
    Null,
    
    /// Boolean type
    Boolean,
    
    /// 8-bit signed integer
    TinyInt,
    
    /// 16-bit signed integer
    SmallInt,
    
    /// 32-bit signed integer
    Integer,
    
    /// 64-bit signed integer
    BigInt,
    
    /// 32-bit floating point
    Float,
    
    /// 64-bit floating point
    Double,
    
    /// Decimal with precision and scale
    Decimal,
    
    /// Variable-length string
    String,
    
    /// Fixed-length string
    Char(usize),
    
    /// Variable-length string with max length
    VarChar(usize),
    
    /// Large text object
    Text,
    
    /// Binary data
    Bytes,
    
    /// Variable-length binary with max length
    VarBinary(usize),
    
    /// Large binary object
    Blob,
    
    /// Date only
    Date,
    
    /// Time only
    Time,
    
    /// Date and time
    DateTime,
    
    /// Date and time with timezone
    Timestamp,
    
    /// JSON data
    Json,
    
    /// UUID
    Uuid,
    
    /// Array of another type
    Array,
    
    /// Custom database-specific type
    Custom(String),
}

impl fmt::Display for SqlType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SqlType::Null => write!(f, "NULL"),
            SqlType::Boolean => write!(f, "BOOLEAN"),
            SqlType::TinyInt => write!(f, "TINYINT"),
            SqlType::SmallInt => write!(f, "SMALLINT"),
            SqlType::Integer => write!(f, "INTEGER"),
            SqlType::BigInt => write!(f, "BIGINT"),
            SqlType::Float => write!(f, "FLOAT"),
            SqlType::Double => write!(f, "DOUBLE"),
            SqlType::Decimal => write!(f, "DECIMAL"),
            SqlType::String => write!(f, "STRING"),
            SqlType::Char(len) => write!(f, "CHAR({})", len),
            SqlType::VarChar(len) => write!(f, "VARCHAR({})", len),
            SqlType::Text => write!(f, "TEXT"),
            SqlType::Bytes => write!(f, "BYTES"),
            SqlType::VarBinary(len) => write!(f, "VARBINARY({})", len),
            SqlType::Blob => write!(f, "BLOB"),
            SqlType::Date => write!(f, "DATE"),
            SqlType::Time => write!(f, "TIME"),
            SqlType::DateTime => write!(f, "DATETIME"),
            SqlType::Timestamp => write!(f, "TIMESTAMP"),
            SqlType::Json => write!(f, "JSON"),
            SqlType::Uuid => write!(f, "UUID"),
            SqlType::Array => write!(f, "ARRAY"),
            SqlType::Custom(name) => write!(f, "{}", name),
        }
    }
}

/// fr fr Parameter for prepared statements - type-safe binding periodt
#[derive(Debug, Clone)]
pub enum Parameter {
    /// Named parameter (e.g., :name, @name)
    Named {
        name: String,
        value: SqlValue,
    },
    
    /// Positional parameter (e.g., ?, $1)
    Positional {
        index: usize,
        value: SqlValue,
    },
}

impl Parameter {
    /// sus Create new named parameter
    pub fn named(name: String, value: SqlValue) -> Self {
        Parameter::Named { name, value }
    }
    
    /// facts Create positional parameter (for drivers that use ? placeholders)
    pub fn positional(index: usize, value: SqlValue) -> Self {
        Parameter::Positional { index, value }
    }
    
    /// lowkey Get the parameter value
    pub fn value(&self) -> &SqlValue {
        match self {
            Parameter::Named { value, .. } => value,
            Parameter::Positional { value, .. } => value,
        }
    }
    
    /// highkey Get parameter name (if named) or index as string
    pub fn name_or_index(&self) -> String {
        match self {
            Parameter::Named { name, .. } => name.clone(),
            Parameter::Positional { index, .. } => index.to_string(),
        }
    }
}

/// fr fr Parameter binding helper for building parameter lists
#[derive(Debug, Clone, Default)]
pub struct ParameterBinding {
    parameters: Vec<Parameter>,
}

impl ParameterBinding {
    /// sus Create new empty parameter binding
    pub fn new() -> Self {
        Self {
            parameters: Vec::new(),
        }
    }
    
    /// facts Add a named parameter
    pub fn bind(mut self, name: &str, value: SqlValue) -> Self {
        self.parameters.push(Parameter::named(name.to_string(), value));
        self
    }
    
    /// highkey Add positional parameter
    pub fn bind_positional(mut self, value: SqlValue) -> Self {
        let index = self.parameters.len();
        self.parameters.push(Parameter::positional(index, value));
        self
    }
    
    /// periodt Get all parameters
    pub fn parameters(self) -> Vec<Parameter> {
        self.parameters
    }
    
    /// bestie Get parameter count
    pub fn count(&self) -> usize {
        self.parameters.len()
    }
}

/// fr fr Database row - represents one row from a result set
#[derive(Debug, Clone)]
pub struct Row {
    /// Values in the same order as columns
    values: Vec<SqlValue>,
}

impl Row {
    /// sus Create new row with values
    pub fn new(values: Vec<SqlValue>) -> Self {
        Self { values }
    }
    
    /// highkey Get value by column index
    pub fn get(&self, index: usize) -> Option<&SqlValue> {
        self.values.get(index)
    }
    
    /// periodt Get value count
    pub fn len(&self) -> usize {
        self.values.len()
    }
    
    /// bestie Check if row is empty
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    
    /// slay Get all values
    pub fn values(&self) -> &[SqlValue] {
        &self.values
    }
    
    /// flex Iterate over values
    pub fn iter(&self) -> std::slice::Iter<SqlValue> {
        self.values.iter()
    }
}

/// fr fr Result set from a query - collection of rows with metadata
#[derive(Debug, Clone)]
pub struct ResultSet {
    /// Column names in order
    columns: Vec<String>,
    
    /// All rows in the result set
    rows: Vec<Row>,
}

impl ResultSet {
    /// sus Create new result set with columns and rows
    pub fn new(columns: Vec<String>, rows: Vec<Row>) -> Self {
        Self { columns, rows }
    }
    
    /// facts Create empty result set
    pub fn empty() -> Self {
        Self {
            columns: Vec::new(),
            rows: Vec::new(),
        }
    }
    
    /// lowkey Get number of rows
    pub fn row_count(&self) -> usize {
        self.rows.len()
    }
    
    /// highkey Get number of columns
    pub fn column_count(&self) -> usize {
        self.columns.len()
    }
    
    /// periodt Check if result set is empty
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    
    /// bestie Get first row (if any)
    pub fn first_row(&self) -> Option<&Row> {
        self.rows.first()
    }
    
    /// flex Iterate over rows
    pub fn iter(&self) -> std::slice::Iter<Row> {
        self.rows.iter()
    }
    
    /// yolo Get column names
    pub fn columns(&self) -> &[String] {
        &self.columns
    }
    
    /// slay Get all rows
    pub fn rows(&self) -> &[Row] {
        &self.rows
    }
}

impl Default for ResultSet {
    fn default() -> Self {
        Self::empty()
    }
}

/// fr fr Column metadata - describes a column in a result set or table
#[derive(Debug, Clone)]
pub struct ColumnMetadata {
    /// Column name
    pub name: String,
    
    /// SQL data type
    pub sql_type: SqlType,
    
    /// Whether column can be NULL
    pub nullable: bool,
    
    /// Whether column is part of primary key
    pub primary_key: bool,
    
    /// Whether column auto-increments
    pub auto_increment: bool,
    
    /// Default value (if any)
    pub default_value: Option<SqlValue>,
}

impl ColumnMetadata {
    /// sus Create new column metadata
    pub fn new(name: String, sql_type: SqlType) -> Self {
        Self {
            name,
            sql_type,
            nullable: true,
            primary_key: false,
            auto_increment: false,
            default_value: None,
        }
    }
    
    /// facts Create non-nullable column
    pub fn not_null(mut self) -> Self {
        self.nullable = false;
        self
    }
    
    /// lowkey Mark as primary key
    pub fn primary_key(mut self) -> Self {
        self.primary_key = true;
        self.nullable = false; // Primary keys can't be null
        self
    }
    
    /// highkey Mark as auto-increment
    pub fn auto_increment(mut self) -> Self {
        self.auto_increment = true;
        self
    }
    
    /// periodt Set default value
    pub fn default_value(mut self, value: SqlValue) -> Self {
        self.default_value = Some(value);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sql_value_types() {
        assert_eq!(SqlValue::Integer(42).sql_type(), SqlType::Integer);
        assert_eq!(SqlValue::String("test".to_string()).sql_type(), SqlType::String);
        assert_eq!(SqlValue::Boolean(true).sql_type(), SqlType::Boolean);
        assert_eq!(SqlValue::Null.sql_type(), SqlType::Null);
    }

    #[test]
    fn test_sql_value_conversions() {
        let int_val = SqlValue::Integer(42);
        assert_eq!(int_val.as_i32(), Some(42));
        assert_eq!(int_val.as_i64(), Some(42));
        assert_eq!(int_val.as_string(), None);
        
        let str_val = SqlValue::String("hello".to_string());
        assert_eq!(str_val.as_string(), Some("hello".to_string()));
        assert_eq!(str_val.as_i32(), None);
    }

    #[test]
    fn test_sql_value_is_null() {
        assert!(SqlValue::Null.is_null());
        assert!(!SqlValue::Integer(42).is_null());
    }

    #[test]
    fn test_parameter_creation() {
        let named_param = Parameter::named("test".to_string(), SqlValue::Integer(42));
        assert_eq!(named_param.name_or_index(), "test");
        assert_eq!(named_param.value(), &SqlValue::Integer(42));
        
        let pos_param = Parameter::positional(0, SqlValue::Integer(42));
        assert_eq!(pos_param.name_or_index(), "0");
        assert_eq!(pos_param.value(), &SqlValue::Integer(42));
    }

    #[test]
    fn test_parameter_binding() {
        let binding = ParameterBinding::new()
            .bind("name", SqlValue::String("John".to_string()))
            .bind("age", SqlValue::Integer(25))
            .bind_positional(SqlValue::Float(98.5));
        
        let params = binding.parameters();
        assert_eq!(params.len(), 3);
        assert_eq!(params[0].name_or_index(), "name");
        assert_eq!(params[1].name_or_index(), "age");
        assert_eq!(params[2].name_or_index(), "0"); // Positional parameter
    }

    #[test]
    fn test_row_creation() {
        let values = Vec::from([SqlValue::Integer(1), SqlValue::String("John".to_string())]);
        let row = Row::new(values);
        
        assert_eq!(row.len(), 2);
        assert_eq!(row.get(0), Some(&SqlValue::Integer(1)));
        assert_eq!(row.get(1), Some(&SqlValue::String("John".to_string())));
        assert_eq!(row.get(2), None);
        assert!(!row.is_empty());
    }

    #[test]
    fn test_row_iteration() {
        let values = Vec::from([SqlValue::Integer(1), SqlValue::String("John".to_string())]);
        let row = Row::new(values);
        
        let mut iter = row.iter();
        assert_eq!(iter.next(), Some(&SqlValue::Integer(1)));
        assert_eq!(iter.next(), Some(&SqlValue::String("John".to_string())));
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_result_set() {
        let empty_result_set = ResultSet::empty();
        assert!(empty_result_set.is_empty());
        assert_eq!(empty_result_set.row_count(), 0);
        
        let columns = Vec::from(["id".to_string(), "name".to_string()]);
        let values = Vec::from([SqlValue::Integer(1), SqlValue::String("John".to_string())]);
        let row = Row::new(values);
        let rows = Vec::from([row]);
        
        let result_set = ResultSet::new(columns, rows);
        assert!(!result_set.is_empty());
        assert_eq!(result_set.row_count(), 1);
        assert_eq!(result_set.column_count(), 2);
        assert!(result_set.first_row().is_some());
    }

    #[test]
    fn test_column_metadata() {
        let column = ColumnMetadata::new("id".to_string(), SqlType::Integer)
            .not_null()
            .primary_key()
            .auto_increment();
        
        assert_eq!(column.name, "id");
        assert_eq!(column.sql_type, SqlType::Integer);
        assert!(!column.nullable);
        assert!(column.primary_key);
        assert!(column.auto_increment);
    }

    #[test]
    fn test_sql_type_display() {
        assert_eq!(SqlType::Integer.to_string(), "INTEGER");
        assert_eq!(SqlType::VarChar(255).to_string(), "VARCHAR(255)");
        assert_eq!(SqlType::Custom("GEOMETRY".to_string()).to_string(), "GEOMETRY");
    }

    #[test]
    fn test_sql_value_display() {
        assert_eq!(SqlValue::Integer(42).to_string(), "42");
        assert_eq!(SqlValue::String("test".to_string()).to_string(), "'test'");
        assert_eq!(SqlValue::Null.to_string(), "NULL");
        assert_eq!(SqlValue::Boolean(true).to_string(), "true");
    }
}
