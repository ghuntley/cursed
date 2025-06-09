/// PostgreSQL type system and value conversions for CURSED database operations
/// 
/// This module provides comprehensive support for PostgreSQL data types including
/// arrays, JSON/JSONB, custom types, and proper type conversions between
/// PostgreSQL and CURSED types.

use std::collections::HashMap;
use serde_json::Value as JsonValue;
use super::super::SqlValue;
use super::ffi::PostgreSQLOid;
use super::error::PostgreSQLError;

/// fr fr PostgreSQL-specific data types with enhanced support
#[derive(Debug, Clone, PartialEq)]
pub enum PostgreSQLType {
    /// Boolean type
    Boolean,
    /// Small integer (16-bit)
    SmallInt,
    /// Integer (32-bit)
    Integer,
    /// Big integer (64-bit)
    BigInt,
    /// Real/float (32-bit)
    Real,
    /// Double precision (64-bit)
    DoublePrecision,
    /// Numeric/decimal with precision
    Numeric(Option<u16>, Option<u16>),
    /// Variable character with limit
    Varchar(Option<u32>),
    /// Fixed character with length
    Char(u32),
    /// Text (unlimited)
    Text,
    /// Binary data
    Bytea,
    /// Date
    Date,
    /// Time without timezone
    Time,
    /// Time with timezone
    Timetz,
    /// Timestamp without timezone
    Timestamp,
    /// Timestamp with timezone
    Timestamptz,
    /// Interval
    Interval,
    /// UUID
    Uuid,
    /// JSON
    Json,
    /// JSONB (binary JSON)
    Jsonb,
    /// Network address
    Inet,
    /// MAC address
    Macaddr,
    /// Array of any type
    Array(Box<PostgreSQLType>),
    /// Custom user-defined type
    Custom(String),
    /// Composite type (struct-like)
    Composite(String, Vec<(String, PostgreSQLType)>),
    /// Enum type
    Enum(String, Vec<String>),
    /// Range type
    Range(Box<PostgreSQLType>),
    /// Domain type (constrained type)
    Domain(String, Box<PostgreSQLType>),
}

impl PostgreSQLType {
    /// slay Get PostgreSQL type from OID
    pub fn from_oid(oid: u32) -> Self {
        match oid {
            16 => Self::Boolean,
            20 => Self::BigInt,
            21 => Self::SmallInt,
            23 => Self::Integer,
            25 => Self::Text,
            700 => Self::Real,
            701 => Self::DoublePrecision,
            1043 => Self::Varchar(None),
            1082 => Self::Date,
            1083 => Self::Time,
            1114 => Self::Timestamp,
            1184 => Self::Timestamptz,
            1186 => Self::Interval,
            1700 => Self::Numeric(None, None),
            2950 => Self::Uuid,
            114 => Self::Json,
            3802 => Self::Jsonb,
            869 => Self::Inet,
            17 => Self::Bytea,
            _ if oid >= 1000 => {
                // Array types typically add 1000 to the base type
                let base_oid = oid - 1000;
                if base_oid <= 999 {
                    Self::Array(Box::new(Self::from_oid(base_oid)))
                } else {
                    Self::Custom(format!("unknown_oid_{}", oid))
                }
            }
            _ => Self::Custom(format!("unknown_oid_{}", oid)),
        }
    }
    
    /// slay Get OID for this type
    pub fn to_oid(&self) -> Option<u32> {
        match self {
            Self::Boolean => Some(16),
            Self::BigInt => Some(20),
            Self::SmallInt => Some(21),
            Self::Integer => Some(23),
            Self::Text => Some(25),
            Self::Real => Some(700),
            Self::DoublePrecision => Some(701),
            Self::Varchar(_) => Some(1043),
            Self::Date => Some(1082),
            Self::Time => Some(1083),
            Self::Timestamp => Some(1114),
            Self::Timestamptz => Some(1184),
            Self::Interval => Some(1186),
            Self::Numeric(_, _) => Some(1700),
            Self::Uuid => Some(2950),
            Self::Json => Some(114),
            Self::Jsonb => Some(3802),
            Self::Inet => Some(869),
            Self::Bytea => Some(17),
            Self::Array(inner) => inner.to_oid().map(|oid| oid + 1000),
            _ => None,
        }
    }
    
    /// slay Get SQL type name
    pub fn sql_name(&self) -> String {
        match self {
            Self::Boolean => "BOOLEAN".to_string(),
            Self::SmallInt => "SMALLINT".to_string(),
            Self::Integer => "INTEGER".to_string(),
            Self::BigInt => "BIGINT".to_string(),
            Self::Real => "REAL".to_string(),
            Self::DoublePrecision => "DOUBLE PRECISION".to_string(),
            Self::Numeric(Some(p), Some(s)) => format!("NUMERIC({}, {})", p, s),
            Self::Numeric(Some(p), None) => format!("NUMERIC({})", p),
            Self::Numeric(None, None) => "NUMERIC".to_string(),
            Self::Varchar(Some(len)) => format!("VARCHAR({})", len),
            Self::Varchar(None) => "VARCHAR".to_string(),
            Self::Char(len) => format!("CHAR({})", len),
            Self::Text => "TEXT".to_string(),
            Self::Bytea => "BYTEA".to_string(),
            Self::Date => "DATE".to_string(),
            Self::Time => "TIME".to_string(),
            Self::Timetz => "TIME WITH TIME ZONE".to_string(),
            Self::Timestamp => "TIMESTAMP".to_string(),
            Self::Timestamptz => "TIMESTAMP WITH TIME ZONE".to_string(),
            Self::Interval => "INTERVAL".to_string(),
            Self::Uuid => "UUID".to_string(),
            Self::Json => "JSON".to_string(),
            Self::Jsonb => "JSONB".to_string(),
            Self::Inet => "INET".to_string(),
            Self::Array(inner) => format!("{}[]", inner.sql_name()),
            Self::Custom(name) => name.clone(),
            Self::Composite(name, _) => name.clone(),
            Self::Enum(name, _) => name.clone(),
            Self::Range(inner) => format!("{}RANGE", inner.sql_name()),
            Self::Domain(name, _) => name.clone(),
            _ => "UNKNOWN".to_string(),
        }
    }
    
    /// slay Check if type is an array
    pub fn is_array(&self) -> bool {
        matches!(self, Self::Array(_))
    }
    
    /// slay Check if type is numeric
    pub fn is_numeric(&self) -> bool {
        matches!(self, Self::SmallInt | Self::Integer | Self::BigInt | Self::Real | Self::DoublePrecision | Self::Numeric(_, _))
    }
    
    /// slay Check if type supports JSON operations
    pub fn is_json(&self) -> bool {
        matches!(self, Self::Json | Self::Jsonb)
    }
}

/// fr fr PostgreSQL value wrapper with type information
#[derive(Debug, Clone, PartialEq)]
pub struct PostgreSQLValue {
    /// fr fr The actual value
    pub value: SqlValue,
    /// fr fr PostgreSQL type information
    pub pg_type: PostgreSQLType,
}

impl PostgreSQLValue {
    /// slay Create a new PostgreSQL value
    pub fn new(value: SqlValue, pg_type: PostgreSQLType) -> Self {
        Self { value, pg_type }
    }
    
    /// slay Convert from raw PostgreSQL bytes
    pub fn from_pg_bytes(bytes: &[u8], pg_type: PostgreSQLType) -> Result<Self, PostgreSQLError> {
        let value = match pg_type {
            PostgreSQLType::Boolean => {
                if bytes.len() != 1 {
                    return Err(PostgreSQLError::query_error("Invalid boolean data"));
                }
                SqlValue::Boolean(bytes[0] != 0)
            }
            PostgreSQLType::SmallInt => {
                if bytes.len() != 2 {
                    return Err(PostgreSQLError::query_error("Invalid smallint data"));
                }
                let val = i16::from_be_bytes([bytes[0], bytes[1]]);
                SqlValue::Integer(val as i64)
            }
            PostgreSQLType::Integer => {
                if bytes.len() != 4 {
                    return Err(PostgreSQLError::query_error("Invalid integer data"));
                }
                let val = i32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                SqlValue::Integer(val as i64)
            }
            PostgreSQLType::BigInt => {
                if bytes.len() != 8 {
                    return Err(PostgreSQLError::query_error("Invalid bigint data"));
                }
                let val = i64::from_be_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3],
                    bytes[4], bytes[5], bytes[6], bytes[7]
                ]);
                SqlValue::Integer(val)
            }
            PostgreSQLType::Real => {
                if bytes.len() != 4 {
                    return Err(PostgreSQLError::query_error("Invalid real data"));
                }
                let val = f32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
                SqlValue::Float(val as f64)
            }
            PostgreSQLType::DoublePrecision => {
                if bytes.len() != 8 {
                    return Err(PostgreSQLError::query_error("Invalid double precision data"));
                }
                let val = f64::from_be_bytes([
                    bytes[0], bytes[1], bytes[2], bytes[3],
                    bytes[4], bytes[5], bytes[6], bytes[7]
                ]);
                SqlValue::Float(val)
            }
            PostgreSQLType::Text | PostgreSQLType::Varchar(_) | PostgreSQLType::Char(_) => {
                let text = String::from_utf8(bytes.to_vec())
                    .map_err(|_| PostgreSQLError::query_error("Invalid UTF-8 text data"))?;
                SqlValue::String(text)
            }
            PostgreSQLType::Bytea => {
                SqlValue::Bytes(bytes.to_vec())
            }
            PostgreSQLType::Json | PostgreSQLType::Jsonb => {
                let text = String::from_utf8(bytes.to_vec())
                    .map_err(|_| PostgreSQLError::query_error("Invalid UTF-8 JSON data"))?;
                let json: JsonValue = serde_json::from_str(&text)
                    .map_err(|_| PostgreSQLError::query_error("Invalid JSON data"))?;
                SqlValue::Json(json)
            }
            PostgreSQLType::Array(ref inner) => {
                // Parse PostgreSQL array format
                Self::parse_array(bytes, inner)?
            }
            _ => {
                // For other types, store as bytes and let application handle
                SqlValue::Bytes(bytes.to_vec())
            }
        };
        
        Ok(Self::new(value, pg_type))
    }
    
    /// slay Parse PostgreSQL array format
    fn parse_array(bytes: &[u8], inner_type: &PostgreSQLType) -> Result<SqlValue, PostgreSQLError> {
        // This is a simplified array parser - PostgreSQL array format is complex
        // In a real implementation, you'd need to handle the full binary array format
        let text = String::from_utf8(bytes.to_vec())
            .map_err(|_| PostgreSQLError::query_error("Invalid UTF-8 array data"))?;
        
        // Simple text array parsing (PostgreSQL text format like {1,2,3})
        if text.starts_with('{') && text.ends_with('}') {
            let inner_text = &text[1..text.len()-1];
            let elements: Vec<&str> = inner_text.split(',').collect();
            
            let mut array_values = Vec::new();
            for element in elements {
                let trimmed = element.trim();
                if trimmed == "NULL" {
                    array_values.push(SqlValue::Null);
                } else {
                    // Convert based on inner type
                    match inner_type {
                        PostgreSQLType::Integer => {
                            let val: i64 = trimmed.parse()
                                .map_err(|_| PostgreSQLError::query_error("Invalid integer in array"))?;
                            array_values.push(SqlValue::Integer(val));
                        }
                        PostgreSQLType::Text => {
                            // Remove quotes if present
                            let text_val = if trimmed.starts_with('"') && trimmed.ends_with('"') {
                                &trimmed[1..trimmed.len()-1]
                            } else {
                                trimmed
                            };
                            array_values.push(SqlValue::String(text_val.to_string()));
                        }
                        _ => {
                            array_values.push(SqlValue::String(trimmed.to_string()));
                        }
                    }
                }
            }
            
            // Store as JSON array for now
            let json_array = JsonValue::Array(
                array_values.into_iter().map(|v| match v {
                    SqlValue::Integer(i) => JsonValue::Number(serde_json::Number::from(i)),
                    SqlValue::String(s) => JsonValue::String(s),
                    SqlValue::Boolean(b) => JsonValue::Bool(b),
                    SqlValue::Float(f) => JsonValue::Number(serde_json::Number::from_f64(f).unwrap_or_else(|| serde_json::Number::from(0))),
                    SqlValue::Null => JsonValue::Null,
                    _ => JsonValue::String(format!("{}", v)),
                }).collect()
            );
            
            Ok(SqlValue::Json(json_array))
        } else {
            Err(PostgreSQLError::query_error("Invalid array format"))
        }
    }
    
    /// slay Convert to PostgreSQL parameter format
    pub fn to_pg_param(&self) -> Result<Vec<u8>, PostgreSQLError> {
        match &self.value {
            SqlValue::Null => Ok(Vec::from([])),
            SqlValue::Boolean(b) => Ok(Vec::from([if *b { 1 } else { 0 }])),
            SqlValue::Integer(i) => {
                match self.pg_type {
                    PostgreSQLType::SmallInt => Ok((*i as i16).to_be_bytes().to_vec()),
                    PostgreSQLType::Integer => Ok((*i as i32).to_be_bytes().to_vec()),
                    PostgreSQLType::BigInt => Ok(i.to_be_bytes().to_vec()),
                    _ => Ok(i.to_string().into_bytes()),
                }
            }
            SqlValue::Float(f) => {
                match self.pg_type {
                    PostgreSQLType::Real => Ok((*f as f32).to_be_bytes().to_vec()),
                    PostgreSQLType::DoublePrecision => Ok(f.to_be_bytes().to_vec()),
                    _ => Ok(f.to_string().into_bytes()),
                }
            }
            SqlValue::String(s) => Ok(s.as_bytes().to_vec()),
            SqlValue::Bytes(b) => Ok(b.clone()),
            SqlValue::Json(j) => Ok(j.to_string().into_bytes()),
            SqlValue::Timestamp(t) => {
                // Convert timestamp to PostgreSQL format
                let duration = t.duration_since(std::time::UNIX_EPOCH)
                    .map_err(|_| PostgreSQLError::query_error("Invalid timestamp"))?;
                Ok(duration.as_secs().to_string().into_bytes())
            }
        }
    }
    
    /// slay Check if value is null
    pub fn is_null(&self) -> bool {
        matches!(self.value, SqlValue::Null)
    }
}

/// fr fr Array type wrapper for PostgreSQL arrays
#[derive(Debug, Clone, PartialEq)]
pub struct ArrayType {
    /// fr fr Element type
    pub element_type: PostgreSQLType,
    /// fr fr Array dimensions
    pub dimensions: Vec<u32>,
    /// fr fr Array values
    pub values: Vec<PostgreSQLValue>,
}

impl ArrayType {
    /// slay Create a new array type
    pub fn new(element_type: PostgreSQLType) -> Self {
        Self {
            element_type,
            dimensions: Vec::from([]),
            values: Vec::from([]),
        }
    }
    
    /// slay Add value to array
    pub fn push(&mut self, value: PostgreSQLValue) {
        self.values.push(value);
    }
    
    /// slay Get array length
    pub fn len(&self) -> usize {
        self.values.len()
    }
    
    /// slay Check if array is empty
    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }
    
    /// slay Convert to SQL value
    pub fn to_sql_value(&self) -> SqlValue {
        let json_array = JsonValue::Array(
            self.values.iter().map(|v| match &v.value {
                SqlValue::Integer(i) => JsonValue::Number(serde_json::Number::from(*i)),
                SqlValue::String(s) => JsonValue::String(s.clone()),
                SqlValue::Boolean(b) => JsonValue::Bool(*b),
                SqlValue::Float(f) => JsonValue::Number(serde_json::Number::from_f64(*f).unwrap_or_else(|| serde_json::Number::from(0))),
                SqlValue::Null => JsonValue::Null,
                SqlValue::Json(j) => j.clone(),
                _ => JsonValue::String(format!("{}", v.value)),
            }).collect()
        );
        
        SqlValue::Json(json_array)
    }
}

/// fr fr JSON type wrapper for PostgreSQL JSON operations
#[derive(Debug, Clone, PartialEq)]
pub struct JsonType {
    /// fr fr JSON value
    pub value: JsonValue,
    /// fr fr Whether this is JSONB (binary JSON)
    pub is_binary: bool,
}

impl JsonType {
    /// slay Create a new JSON type
    pub fn new(value: JsonValue, is_binary: bool) -> Self {
        Self { value, is_binary }
    }
    
    /// slay Create JSON type
    pub fn json(value: JsonValue) -> Self {
        Self::new(value, false)
    }
    
    /// slay Create JSONB type
    pub fn jsonb(value: JsonValue) -> Self {
        Self::new(value, true)
    }
    
    /// slay Get JSON path value
    pub fn get_path(&self, path: &[&str]) -> Option<&JsonValue> {
        let mut current = &self.value;
        
        for key in path {
            match current {
                JsonValue::Object(obj) => {
                    current = obj.get(*key)?;
                }
                JsonValue::Array(arr) => {
                    if let Ok(index) = key.parse::<usize>() {
                        current = arr.get(index)?;
                    } else {
                        return None;
                    }
                }
                _ => return None,
            }
        }
        
        Some(current)
    }
    
    /// slay Set JSON path value
    pub fn set_path(&mut self, path: &[&str], new_value: JsonValue) -> Result<(), PostgreSQLError> {
        if path.is_empty() {
            self.value = new_value;
            return Ok(());
        }
        
        let mut current = &mut self.value;
        let last_key = path[path.len() - 1];
        
        for key in &path[..path.len() - 1] {
            match current {
                JsonValue::Object(ref mut obj) => {
                    current = obj.entry(key.to_string()).or_insert(JsonValue::Object(serde_json::Map::new()));
                }
                JsonValue::Array(ref mut arr) => {
                    if let Ok(index) = key.parse::<usize>() {
                        if index >= arr.len() {
                            return Err(PostgreSQLError::query_error("Array index out of bounds"));
                        }
                        current = &mut arr[index];
                    } else {
                        return Err(PostgreSQLError::query_error("Invalid array index"));
                    }
                }
                _ => return Err(PostgreSQLError::query_error("Cannot navigate non-object/array JSON")),
            }
        }
        
        match current {
            JsonValue::Object(ref mut obj) => {
                obj.insert(last_key.to_string(), new_value);
            }
            JsonValue::Array(ref mut arr) => {
                if let Ok(index) = last_key.parse::<usize>() {
                    if index >= arr.len() {
                        return Err(PostgreSQLError::query_error("Array index out of bounds"));
                    }
                    arr[index] = new_value;
                } else {
                    return Err(PostgreSQLError::query_error("Invalid array index"));
                }
            }
            _ => return Err(PostgreSQLError::query_error("Cannot set value on non-object/array JSON")),
        }
        
        Ok(())
    }
    
    /// slay Convert to SQL value
    pub fn to_sql_value(&self) -> SqlValue {
        SqlValue::Json(self.value.clone())
    }
    
    /// slay Get PostgreSQL type
    pub fn pg_type(&self) -> PostgreSQLType {
        if self.is_binary {
            PostgreSQLType::Jsonb
        } else {
            PostgreSQLType::Json
        }
    }
}

/// fr fr Type registry for custom PostgreSQL types
#[derive(Debug, Clone)]
pub struct TypeRegistry {
    /// fr fr Custom types by name
    pub types: HashMap<String, PostgreSQLType>,
    /// fr fr Type aliases
    pub aliases: HashMap<String, String>,
}

impl TypeRegistry {
    /// slay Create a new type registry
    pub fn new() -> Self {
        Self {
            types: HashMap::new(),
            aliases: HashMap::new(),
        }
    }
    
    /// slay Register a custom type
    pub fn register_type(&mut self, name: String, pg_type: PostgreSQLType) {
        self.types.insert(name, pg_type);
    }
    
    /// slay Register a type alias
    pub fn register_alias(&mut self, alias: String, target: String) {
        self.aliases.insert(alias, target);
    }
    
    /// slay Get type by name
    pub fn get_type(&self, name: &str) -> Option<&PostgreSQLType> {
        // Check direct types first
        if let Some(pg_type) = self.types.get(name) {
            return Some(pg_type);
        }
        
        // Check aliases
        if let Some(target) = self.aliases.get(name) {
            return self.types.get(target);
        }
        
        None
    }
    
    /// slay Check if type exists
    pub fn has_type(&self, name: &str) -> bool {
        self.types.contains_key(name) || self.aliases.contains_key(name)
    }
    
    /// slay List all registered types
    pub fn list_types(&self) -> Vec<String> {
        self.types.keys().cloned().collect()
    }
}

impl Default for TypeRegistry {
    fn default() -> Self {
        let mut registry = Self::new();
        
        // Register common aliases
        registry.register_alias("int".to_string(), "integer".to_string());
        registry.register_alias("int4".to_string(), "integer".to_string());
        registry.register_alias("int8".to_string(), "bigint".to_string());
        registry.register_alias("int2".to_string(), "smallint".to_string());
        registry.register_alias("float4".to_string(), "real".to_string());
        registry.register_alias("float8".to_string(), "double precision".to_string());
        registry.register_alias("bool".to_string(), "boolean".to_string());
        
        registry
    }
}

/// fr fr Utility functions for type conversion
pub mod type_utils {
    use super::*;
    
    /// slay Convert SqlValue to PostgreSQLValue with type inference
    pub fn infer_pg_value(value: SqlValue) -> PostgreSQLValue {
        let pg_type = match &value {
            SqlValue::Boolean(_) => PostgreSQLType::Boolean,
            SqlValue::Integer(i) => {
                if *i >= i32::MIN as i64 && *i <= i32::MAX as i64 {
                    PostgreSQLType::Integer
                } else {
                    PostgreSQLType::BigInt
                }
            }
            SqlValue::Float(_) => PostgreSQLType::DoublePrecision,
            SqlValue::String(_) => PostgreSQLType::Text,
            SqlValue::Bytes(_) => PostgreSQLType::Bytea,
            SqlValue::Json(_) => PostgreSQLType::Jsonb,
            SqlValue::Timestamp(_) => PostgreSQLType::Timestamptz,
            SqlValue::Null => PostgreSQLType::Text, // Default for null
        };
        
        PostgreSQLValue::new(value, pg_type)
    }
    
    /// slay Convert PostgreSQLValue to SqlValue
    pub fn to_sql_value(pg_value: PostgreSQLValue) -> SqlValue {
        pg_value.value
    }
    
    /// slay Check if two PostgreSQL types are compatible
    pub fn types_compatible(a: &PostgreSQLType, b: &PostgreSQLType) -> bool {
        match (a, b) {
            // Exact match
            (x, y) if x == y => true,
            
            // Numeric compatibility
            (PostgreSQLType::SmallInt, PostgreSQLType::Integer) |
            (PostgreSQLType::Integer, PostgreSQLType::SmallInt) |
            (PostgreSQLType::SmallInt, PostgreSQLType::BigInt) |
            (PostgreSQLType::BigInt, PostgreSQLType::SmallInt) |
            (PostgreSQLType::Integer, PostgreSQLType::BigInt) |
            (PostgreSQLType::BigInt, PostgreSQLType::Integer) => true,
            
            // Float compatibility
            (PostgreSQLType::Real, PostgreSQLType::DoublePrecision) |
            (PostgreSQLType::DoublePrecision, PostgreSQLType::Real) => true,
            
            // String compatibility
            (PostgreSQLType::Text, PostgreSQLType::Varchar(_)) |
            (PostgreSQLType::Varchar(_), PostgreSQLType::Text) |
            (PostgreSQLType::Char(_), PostgreSQLType::Varchar(_)) |
            (PostgreSQLType::Varchar(_), PostgreSQLType::Char(_)) => true,
            
            // JSON compatibility
            (PostgreSQLType::Json, PostgreSQLType::Jsonb) |
            (PostgreSQLType::Jsonb, PostgreSQLType::Json) => true,
            
            // Array compatibility
            (PostgreSQLType::Array(a_inner), PostgreSQLType::Array(b_inner)) => {
                types_compatible(a_inner, b_inner)
            }
            
            _ => false,
        }
    }
}
