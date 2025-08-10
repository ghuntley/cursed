//! PostgreSQL type mapping implementation

use crate::error::CursedError;
use super::connection::PostgresValue;
use std::collections::HashMap;

/// Result type for PostgreSQL type operations
pub type PostgresTypeResult<T> = Result<T, CursedError>;

/// PostgreSQL type mapper
pub struct PostgresTypeMapper {
    type_mappings: HashMap<String, PostgresType>,
}

/// PostgreSQL type information
#[derive(Debug, Clone)]
pub struct PostgresType {
    pub name: String,
    pub oid: u32,
    pub size: i16,
    pub is_array: bool,
    pub element_type: Option<u32>,
}

impl PostgresTypeMapper {
    /// Create a new PostgreSQL type mapper
    pub fn new() -> Self {
        let mut mapper = Self {
            type_mappings: HashMap::new(),
        };
        mapper.initialize_standard_types();
        mapper
    }
    
    /// Initialize standard PostgreSQL types
    fn initialize_standard_types(&mut self) {
        // Standard PostgreSQL types
        self.add_type("bool", 16, 1, false, None);
        self.add_type("int2", 21, 2, false, None);
        self.add_type("int4", 23, 4, false, None);
        self.add_type("int8", 20, 8, false, None);
        self.add_type("float4", 700, 4, false, None);
        self.add_type("float8", 701, 8, false, None);
        self.add_type("text", 25, -1, false, None);
        self.add_type("varchar", 1043, -1, false, None);
        self.add_type("char", 1042, -1, false, None);
        self.add_type("bytea", 17, -1, false, None);
        self.add_type("timestamp", 1114, 8, false, None);
        self.add_type("timestamptz", 1184, 8, false, None);
        self.add_type("date", 1082, 4, false, None);
        self.add_type("time", 1083, 8, false, None);
        self.add_type("timetz", 1266, 12, false, None);
        self.add_type("interval", 1186, 16, false, None);
        self.add_type("numeric", 1700, -1, false, None);
        self.add_type("money", 790, 8, false, None);
        self.add_type("uuid", 2950, 16, false, None);
        self.add_type("json", 114, -1, false, None);
        self.add_type("jsonb", 3802, -1, false, None);
        self.add_type("xml", 142, -1, false, None);
        
        println!("🔄 Initialized PostgreSQL type mapper with {} types", self.type_mappings.len());
    }
    
    /// Add a type to the mapper
    fn add_type(&mut self, name: &str, oid: u32, size: i16, is_array: bool, element_type: Option<u32>) {
        let pg_type = PostgresType {
            name: name.to_string(),
            oid,
            size,
            is_array,
            element_type,
        };
        self.type_mappings.insert(name.to_string(), pg_type);
    }
    
    /// Get type by name
    pub fn get_type(&self, name: &str) -> Option<&PostgresType> {
        self.type_mappings.get(name)
    }
    
    /// Get type by OID
    pub fn get_type_by_oid(&self, oid: u32) -> Option<&PostgresType> {
        self.type_mappings.values().find(|t| t.oid == oid)
    }
    
    /// List all available types
    pub fn list_types(&self) -> Vec<&PostgresType> {
        self.type_mappings.values().collect()
    }
}

impl Default for PostgresTypeMapper {
    fn default() -> Self {
        Self::new()
    }
}

/// Map a PostgreSQL value to a Cursed value
pub fn map_postgres_value(pg_value: &PostgresValue) -> PostgresTypeResult<CursedValue> {
    match pg_value {
        PostgresValue::Null => Ok(CursedValue::Null),
        PostgresValue::Bool(b) => Ok(CursedValue::Bool(*b)),
        PostgresValue::Int32(i) => Ok(CursedValue::Int32(*i)),
        PostgresValue::Int64(i) => Ok(CursedValue::Int64(*i)),
        PostgresValue::Float32(f) => Ok(CursedValue::Float32(*f)),
        PostgresValue::Float64(f) => Ok(CursedValue::Float64(*f)),
        PostgresValue::Text(s) => Ok(CursedValue::String(s.clone())),
        PostgresValue::Bytes(b) => Ok(CursedValue::Bytes(b.clone())),
    }
}

/// Map a Cursed value to a PostgreSQL value
pub fn map_cursed_value(cursed_value: &CursedValue) -> PostgresTypeResult<PostgresValue> {
    match cursed_value {
        CursedValue::Null => Ok(PostgresValue::Null),
        CursedValue::Bool(b) => Ok(PostgresValue::Bool(*b)),
        CursedValue::Int32(i) => Ok(PostgresValue::Int32(*i)),
        CursedValue::Int64(i) => Ok(PostgresValue::Int64(*i)),
        CursedValue::Float32(f) => Ok(PostgresValue::Float32(*f)),
        CursedValue::Float64(f) => Ok(PostgresValue::Float64(*f)),
        CursedValue::String(s) => Ok(PostgresValue::Text(s.clone())),
        CursedValue::Bytes(b) => Ok(PostgresValue::Bytes(b.clone())),
    }
}

/// Cursed value wrapper for database operations
#[derive(Debug, Clone)]
pub enum CursedValue {
    Null,
    Bool(bool),
    Int32(i32),
    Int64(i64),
    Float32(f32),
    Float64(f64),
    String(String),
    Bytes(Vec<u8>),
}

impl CursedValue {
    /// Get the type name of this value
    pub fn type_name(&self) -> &'static str {
        match self {
            CursedValue::Null => "null",
            CursedValue::Bool(_) => "bool",
            CursedValue::Int32(_) => "int32",
            CursedValue::Int64(_) => "int64",
            CursedValue::Float32(_) => "float32",
            CursedValue::Float64(_) => "float64",
            CursedValue::String(_) => "string",
            CursedValue::Bytes(_) => "bytes",
        }
    }
    
    /// Check if this value is null
    pub fn is_null(&self) -> bool {
        matches!(self, CursedValue::Null)
    }
    
    /// Convert to string representation
    pub fn to_string(&self) -> String {
        match self {
            CursedValue::Null => "NULL".to_string(),
            CursedValue::Bool(b) => b.to_string(),
            CursedValue::Int32(i) => i.to_string(),
            CursedValue::Int64(i) => i.to_string(),
            CursedValue::Float32(f) => f.to_string(),
            CursedValue::Float64(f) => f.to_string(),
            CursedValue::String(s) => s.clone(),
            CursedValue::Bytes(b) => format!("\\x{}", hex::encode(b)),
        }
    }
}

/// Helper function to convert hex string to bytes
fn hex_encode(bytes: &[u8]) -> String {
    bytes.iter()
        .map(|b| format!("{:02x}", b))
        .collect::<String>()
}

/// Type conversion utilities
pub struct PostgresTypeConverter;

impl PostgresTypeConverter {
    /// Convert PostgreSQL type name to Cursed type
    pub fn postgres_to_cursed_type(pg_type: &str) -> Option<&'static str> {
        match pg_type {
            "bool" => Some("bool"),
            "int2" | "int4" => Some("int32"),
            "int8" => Some("int64"),
            "float4" => Some("float32"),
            "float8" => Some("float64"),
            "text" | "varchar" | "char" => Some("string"),
            "bytea" => Some("bytes"),
            _ => None,
        }
    }
    
    /// Convert Cursed type to PostgreSQL type name
    pub fn cursed_to_postgres_type(cursed_type: &str) -> Option<&'static str> {
        match cursed_type {
            "bool" => Some("bool"),
            "int32" => Some("int4"),
            "int64" => Some("int8"),
            "float32" => Some("float4"),
            "float64" => Some("float8"),
            "string" => Some("text"),
            "bytes" => Some("bytea"),
            _ => None,
        }
    }
}

/// PostgreSQL array type utilities
pub struct PostgresArrayType {
    element_type: PostgresType,
}

impl PostgresArrayType {
    /// Create a new array type
    pub fn new(element_type: PostgresType) -> Self {
        Self { element_type }
    }
    
    /// Get the element type
    pub fn element_type(&self) -> &PostgresType {
        &self.element_type
    }
    
    /// Check if this is a multi-dimensional array
    pub fn is_multidimensional(&self) -> bool {
        self.element_type.is_array
    }
}

// Mock hex module for compilation
mod hex {
    pub fn encode(bytes: &[u8]) -> String {
        bytes.iter()
            .map(|b| format!("{:02x}", b))
            .collect::<String>()
    }
}
