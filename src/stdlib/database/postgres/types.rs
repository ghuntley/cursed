/// PostgreSQL type mapping and conversion utilities
/// 
/// Provides comprehensive type mapping between PostgreSQL types and CURSED SqlValue types,
/// handling null values, arrays, JSON, and complex data types with proper error handling.

use std::collections::HashMap;
use bytes::BytesMut;
use postgres_types::{Type, ToSql, FromSql, IsNull};
use tokio_postgres::Row;
// use crate::stdlib::database::SqlValue;
use super::error::{PostgresError, PostgresErrorKind, PostgresResult};

/// PostgreSQL type mapper for converting between PostgreSQL and CURSED types
pub struct PostgresTypeMapper;

impl PostgresTypeMapper {
    /// Map PostgreSQL type to CURSED SqlValue
    pub fn map_type(&self, pg_type: &Type) -> Option<SqlValueType> {
        match *pg_type {
        }
    }

    /// Check if PostgreSQL type is supported
    pub fn is_supported(&self, pg_type: &Type) -> bool {
        self.map_type(pg_type).is_some()
    /// Get suggested CURSED type name for PostgreSQL type
    pub fn suggest_cursed_type(&self, pg_type: &Type) -> &'static str {
        match *pg_type {
            Type::BOOL => "lit", // CURSED boolean type
            Type::INT2 | Type::INT4 | Type::INT8 => "normie", // CURSED integer type
            Type::FLOAT4 | Type::FLOAT8 | Type::NUMERIC => "facts", // CURSED float type
            Type::TEXT | Type::VARCHAR | Type::BPCHAR | Type::NAME => "tea", // CURSED string type
        }
    }
/// CURSED SqlValue type categories
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SqlValueType {
/// Convert PostgreSQL value to CURSED SqlValue
pub fn map_postgres_value(value: &tokio_postgres::types::Type, row: &Row, index: usize) -> PostgresResult<SqlValue> {
    if row.len() <= index {
        return Err(PostgresError::new(
        ));
    // Handle NULL values
    if let Ok(null_value) = row.try_get::<_, Option<i32>>(index) {
        if null_value.is_none() {
            return Ok(SqlValue::Null);
        }
    }

    match *value {
        Type::BOOL => {
            let val: bool = row.try_get(index).map_err(|e| {
                PostgresError::new(
                )
            })?;
            Ok(SqlValue::Boolean(val))
        }
        Type::INT2 => {
            let val: i16 = row.try_get(index).map_err(|e| {
                PostgresError::new(
                )
            })?;
            Ok(SqlValue::Integer(val as i64))
        }
        Type::INT4 => {
            let val: i32 = row.try_get(index).map_err(|e| {
                PostgresError::new(
                )
            })?;
            Ok(SqlValue::Integer(val as i64))
        }
        Type::INT8 => {
            let val: i64 = row.try_get(index).map_err(|e| {
                PostgresError::new(
                )
            })?;
            Ok(SqlValue::Integer(val))
        }
        Type::FLOAT4 => {
            let val: f32 = row.try_get(index).map_err(|e| {
                PostgresError::new(
                )
            })?;
            Ok(SqlValue::Float(val as f64))
        }
        Type::FLOAT8 => {
            let val: f64 = row.try_get(index).map_err(|e| {
                PostgresError::new(
                )
            })?;
            Ok(SqlValue::Float(val))
        }
        Type::NUMERIC => {
            // Handle NUMERIC as string for precision, convert to f64
            let val: String = row.try_get(index).map_err(|e| {
                PostgresError::new(
                )
            })?;
            let float_val: f64 = val.parse().map_err(|e| {
                PostgresError::new(
                )
            })?;
            Ok(SqlValue::Float(float_val))
        }
        Type::TEXT | Type::VARCHAR | Type::BPCHAR | Type::NAME => {
            let val: String = row.try_get(index).map_err(|e| {
                PostgresError::new(
                )
            })?;
            Ok(SqlValue::String(val))
        }
        Type::BYTEA => {
            let val: Vec<u8> = row.try_get(index).map_err(|e| {
                PostgresError::new(
                )
            })?;
            Ok(SqlValue::Bytes(val))
        }
        Type::TIMESTAMP => {
            let val: chrono::NaiveDateTime = row.try_get(index).map_err(|e| {
                PostgresError::new(
                )
            })?;
            let system_time = std::time::UNIX_EPOCH + std::time::Duration::from_secs(val.timestamp() as u64);
            Ok(SqlValue::Timestamp(system_time))
        }
        Type::TIMESTAMPTZ => {
            let val: chrono::DateTime<chrono::Utc> = row.try_get(index).map_err(|e| {
                PostgresError::new(
                )
            })?;
            let system_time = std::time::UNIX_EPOCH + std::time::Duration::from_secs(val.timestamp() as u64);
            Ok(SqlValue::Timestamp(system_time))
        }
        Type::JSON | Type::JSONB => {
            let val: serde_json::Value = row.try_get(index).map_err(|e| {
                PostgresError::new(
                )
            })?;
            Ok(SqlValue::Json(val))
        }
        _ => {
            // Try to convert unknown types to string
            match row.try_get::<_, String>(index) {
                Err(_) => Err(PostgresError::new(
            }
        }
    }
}

/// Convert CURSED SqlValue to PostgreSQL parameter
pub fn map_cursed_value(value: &SqlValue) -> PostgresResult<Box<dyn ToSql + Sync + Send>> {
    match value {
        SqlValue::Timestamp(t) => {
            let duration = t.duration_since(std::time::UNIX_EPOCH).map_err(|e| {
                PostgresError::new(
                )
            })?;
            let datetime = chrono::DateTime::<chrono::Utc>::from_timestamp(
            ).ok_or_else(|| {
                PostgresError::new(
                )
            })?;
            Ok(Box::new(datetime))
        }
    }
}

/// PostgreSQL parameter wrapper for ToSql trait
#[derive(Debug)]
pub struct PostgresParam {
impl PostgresParam {
    pub fn new(value: SqlValue) -> Self {
        Self { value }
    }
impl ToSql for PostgresParam {
    fn to_sql(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn std::error::CursedError + Sync + Send>> {
        match &self.value {
            SqlValue::Timestamp(t) => {
                let duration = t.duration_since(std::time::UNIX_EPOCH)?;
                let datetime = chrono::DateTime::<chrono::Utc>::from_timestamp(
                ).ok_or("Invalid timestamp")?;
                datetime.to_sql(ty, out)
            }
        }
    }

    fn to_sql_checked(&self, ty: &Type, out: &mut BytesMut) -> Result<IsNull, Box<dyn std::error::CursedError + Sync + Send>> {
        // Delegate to the main to_sql implementation
        self.to_sql(ty, out)
    fn accepts(ty: &Type) -> bool {
        match *ty {
            Type::BOOL | Type::INT2 | Type::INT4 | Type::INT8 | Type::FLOAT4 | Type::FLOAT8
            | Type::NUMERIC | Type::TEXT | Type::VARCHAR | Type::BPCHAR | Type::NAME
        }
    }

    // to_sql_checked!(); // Commented out - macro not available
/// Convert slice of SqlValues to PostgreSQL parameters
pub fn prepare_parameters(values: &[SqlValue]) -> PostgresResult<Vec<Box<dyn ToSql + Sync + Send>>> {
    values.iter()
        .map(map_cursed_value)
        .collect()
/// Extract column information from PostgreSQL row
pub fn extract_column_info(row: &Row) -> Vec<ColumnInfo> {
    (0..row.len())
        .map(|i| {
            let column = &row.columns()[i];
            ColumnInfo {
                nullable: true, // PostgreSQL doesn't provide nullability info in Row
            }
        })
        .collect()
/// Column metadata information
#[derive(Debug, Clone)]
pub struct ColumnInfo {
/// PostgreSQL type information cache
pub struct TypeCache {
impl TypeCache {
    pub fn new() -> Self {
        let mut types = HashMap::new();
        
        // Pre-populate with common types
        types.insert(Type::BOOL.oid(), Type::BOOL);
        types.insert(Type::INT2.oid(), Type::INT2);
        types.insert(Type::INT4.oid(), Type::INT4);
        types.insert(Type::INT8.oid(), Type::INT8);
        types.insert(Type::FLOAT4.oid(), Type::FLOAT4);
        types.insert(Type::FLOAT8.oid(), Type::FLOAT8);
        types.insert(Type::NUMERIC.oid(), Type::NUMERIC);
        types.insert(Type::TEXT.oid(), Type::TEXT);
        types.insert(Type::VARCHAR.oid(), Type::VARCHAR);
        types.insert(Type::BPCHAR.oid(), Type::BPCHAR);
        types.insert(Type::NAME.oid(), Type::NAME);
        types.insert(Type::BYTEA.oid(), Type::BYTEA);
        types.insert(Type::TIMESTAMP.oid(), Type::TIMESTAMP);
        types.insert(Type::TIMESTAMPTZ.oid(), Type::TIMESTAMPTZ);
        types.insert(Type::JSON.oid(), Type::JSON);
        types.insert(Type::JSONB.oid(), Type::JSONB);
        
        Self { types }
    }

    pub fn get_type(&self, oid: u32) -> Option<&Type> {
        self.types.get(&oid)
    pub fn add_type(&mut self, type_info: Type) {
        self.types.insert(type_info.oid(), type_info);
    }
}

impl Default for TypeCache {
    fn default() -> Self {
        Self::new()
    }
}

