/// Type mapping system for CURSED ORM
/// 
/// Provides type conversion between CURSED types and SQL types,
/// custom mapping support, and result mapping for query results.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{instrument, debug, info, warn, error};

use super::super::{DatabaseError, DatabaseErrorKind, SqlValue};
use super::entity::{Entity, SqlColumnType};

/// fr fr Type mapper for converting between CURSED and SQL types
#[derive(Debug)]
pub struct TypeMapper {
    /// Built-in type mappings
    builtin_mappings: HashMap<String, SqlTypeMapping>,
    /// Custom type mappings
    custom_mappings: Arc<Mutex<HashMap<String, Box<dyn CustomMapping>>>>,
    /// Mapping registry
    registry: MappingRegistry,
}

impl TypeMapper {
    /// slay Create new type mapper
    #[instrument]
    pub fn new() -> Self {
        info!("Creating new type mapper");
        
        let mut mapper = Self {
            builtin_mappings: HashMap::new(),
            custom_mappings: Arc::new(Mutex::new(HashMap::new())),
            registry: MappingRegistry::new(),
        };
        
        mapper.register_builtin_mappings();
        mapper
    }

    /// facts Register built-in type mappings
    #[instrument(skip(self))]
    fn register_builtin_mappings(&mut self) {
        debug!("Registering built-in type mappings");
        
        // Integer types
        self.builtin_mappings.insert(
            "i32".to_string(),
            SqlTypeMapping::Simple {
                sql_type: SqlColumnType::Integer,
                nullable: false,
            },
        );
        
        self.builtin_mappings.insert(
            "i64".to_string(),
            SqlTypeMapping::Simple {
                sql_type: SqlColumnType::BigInteger,
                nullable: false,
            },
        );
        
        self.builtin_mappings.insert(
            "Option<i32>".to_string(),
            SqlTypeMapping::Simple {
                sql_type: SqlColumnType::Integer,
                nullable: true,
            },
        );
        
        self.builtin_mappings.insert(
            "Option<i64>".to_string(),
            SqlTypeMapping::Simple {
                sql_type: SqlColumnType::BigInteger,
                nullable: true,
            },
        );
        
        // Float types
        self.builtin_mappings.insert(
            "f32".to_string(),
            SqlTypeMapping::Simple {
                sql_type: SqlColumnType::Float,
                nullable: false,
            },
        );
        
        self.builtin_mappings.insert(
            "f64".to_string(),
            SqlTypeMapping::Simple {
                sql_type: SqlColumnType::Double,
                nullable: false,
            },
        );
        
        // String types
        self.builtin_mappings.insert(
            "String".to_string(),
            SqlTypeMapping::Simple {
                sql_type: SqlColumnType::Text,
                nullable: false,
            },
        );
        
        self.builtin_mappings.insert(
            "Option<String>".to_string(),
            SqlTypeMapping::Simple {
                sql_type: SqlColumnType::Text,
                nullable: true,
            },
        );
        
        // Boolean type
        self.builtin_mappings.insert(
            "bool".to_string(),
            SqlTypeMapping::Simple {
                sql_type: SqlColumnType::Boolean,
                nullable: false,
            },
        );
        
        // DateTime types
        self.builtin_mappings.insert(
            "SystemTime".to_string(),
            SqlTypeMapping::Simple {
                sql_type: SqlColumnType::Timestamp,
                nullable: false,
            },
        );
        
        // Byte arrays
        self.builtin_mappings.insert(
            "Vec<u8>".to_string(),
            SqlTypeMapping::Simple {
                sql_type: SqlColumnType::Binary,
                nullable: false,
            },
        );
        
        // JSON types
        self.builtin_mappings.insert(
            "serde_json::Value".to_string(),
            SqlTypeMapping::Simple {
                sql_type: SqlColumnType::Json,
                nullable: true,
            },
        );
        
        debug!(count = self.builtin_mappings.len(), "Built-in type mappings registered");
    }

    /// periodt Map CURSED type to SQL type
    #[instrument(skip(self))]
    pub fn map_to_sql(&self, cursed_type: &str) -> Result<SqlTypeMapping, DatabaseError> {
        debug!(cursed_type = cursed_type, "Mapping CURSED type to SQL");
        
        // Check built-in mappings first
        if let Some(mapping) = self.builtin_mappings.get(cursed_type) {
            debug!("Found built-in mapping");
            return Ok(mapping.clone());
        }
        
        // Check custom mappings
        if let Ok(custom_mappings) = self.custom_mappings.lock() {
            if let Some(custom_mapping) = custom_mappings.get(cursed_type) {
                debug!("Found custom mapping");
                return Ok(custom_mapping.to_sql_mapping());
            }
        }
        
        // Try to infer mapping
        if let Some(inferred) = self.infer_mapping(cursed_type) {
            debug!("Inferred mapping");
            return Ok(inferred);
        }
        
        error!(cursed_type = cursed_type, "No mapping found for CURSED type");
        Err(DatabaseError::validation_error(&format!(
            "No mapping found for CURSED type: {}",
            cursed_type
        )))
    }

    /// bestie Map SQL value to CURSED value
    #[instrument(skip(self))]
    pub fn map_from_sql(&self, sql_value: &SqlValue, target_type: &str) -> Result<Box<dyn std::any::Any>, DatabaseError> {
        debug!(target_type = target_type, "Mapping SQL value to CURSED type");
        
        match (sql_value, target_type) {
            (SqlValue::Integer(i), "i32") => Ok(Box::new(*i as i32)),
            (SqlValue::Integer(i), "i64") => Ok(Box::new(*i)),
            (SqlValue::Integer(i), "Option<i32>") => Ok(Box::new(Some(*i as i32))),
            (SqlValue::Integer(i), "Option<i64>") => Ok(Box::new(Some(*i))),
            (SqlValue::Float(f), "f32") => Ok(Box::new(*f as f32)),
            (SqlValue::Float(f), "f64") => Ok(Box::new(*f)),
            (SqlValue::String(s), "String") => Ok(Box::new(s.clone())),
            (SqlValue::String(s), "Option<String>") => Ok(Box::new(Some(s.clone()))),
            (SqlValue::Boolean(b), "bool") => Ok(Box::new(*b)),
            (SqlValue::Timestamp(t), "SystemTime") => Ok(Box::new(*t)),
            (SqlValue::Bytes(b), "Vec<u8>") => Ok(Box::new(b.clone())),
            (SqlValue::Json(j), "serde_json::Value") => Ok(Box::new(j.clone())),
            (SqlValue::Null, type_name) if type_name.starts_with("Option<") => {
                // Return None for Option types
                Ok(Box::new(Option::<()>::None))
            }
            _ => {
                error!(target_type = target_type, sql_value = ?sql_value, "Cannot map SQL value to CURSED type");
                Err(DatabaseError::validation_error(&format!(
                    "Cannot map SQL value {:?} to CURSED type {}",
                    sql_value, target_type
                )))
            }
        }
    }

    /// yolo Register custom mapping
    #[instrument(skip(self, mapping))]
    pub fn register_custom_mapping(&self, cursed_type: &str, mapping: Box<dyn CustomMapping>) -> Result<(), DatabaseError> {
        debug!(cursed_type = cursed_type, "Registering custom mapping");
        
        if let Ok(mut custom_mappings) = self.custom_mappings.lock() {
            custom_mappings.insert(cursed_type.to_string(), mapping);
            debug!("Custom mapping registered successfully");
            Ok(())
        } else {
            Err(DatabaseError::internal_error("Failed to access custom mappings"))
        }
    }

    /// slay Infer mapping from type name
    #[instrument(skip(self))]
    fn infer_mapping(&self, cursed_type: &str) -> Option<SqlTypeMapping> {
        debug!(cursed_type = cursed_type, "Inferring type mapping");
        
        // Handle Option types
        if cursed_type.starts_with("Option<") && cursed_type.ends_with('>') {
            let inner_type = &cursed_type[7..cursed_type.len() - 1];
            if let Some(mut inner_mapping) = self.infer_basic_type(inner_type) {
                inner_mapping.set_nullable(true);
                return Some(inner_mapping);
            }
        }
        
        // Handle basic types
        self.infer_basic_type(cursed_type)
    }

    /// lit Infer basic type mapping
    fn infer_basic_type(&self, type_name: &str) -> Option<SqlTypeMapping> {
        match type_name {
            "u8" | "u16" | "u32" | "i8" | "i16" | "i32" => Some(SqlTypeMapping::Simple {
                sql_type: SqlColumnType::Integer,
                nullable: false,
            }),
            "u64" | "i64" | "usize" | "isize" => Some(SqlTypeMapping::Simple {
                sql_type: SqlColumnType::BigInteger,
                nullable: false,
            }),
            "f32" => Some(SqlTypeMapping::Simple {
                sql_type: SqlColumnType::Float,
                nullable: false,
            }),
            "f64" => Some(SqlTypeMapping::Simple {
                sql_type: SqlColumnType::Double,
                nullable: false,
            }),
            "bool" => Some(SqlTypeMapping::Simple {
                sql_type: SqlColumnType::Boolean,
                nullable: false,
            }),
            "str" | "&str" => Some(SqlTypeMapping::Simple {
                sql_type: SqlColumnType::Text,
                nullable: false,
            }),
            _ => None,
        }
    }
}

/// fr fr SQL type mapping configuration
#[derive(Debug, Clone)]
pub enum SqlTypeMapping {
    /// Simple direct mapping
    Simple {
        sql_type: SqlColumnType,
        nullable: bool,
    },
    /// Complex mapping with custom converter
    Complex {
        sql_type: SqlColumnType,
        nullable: bool,
        converter: String, // Name of converter function
    },
    /// Serialized mapping (JSON/Binary)
    Serialized {
        sql_type: SqlColumnType,
        format: SerializationFormat,
    },
}

impl SqlTypeMapping {
    /// Get SQL column type
    pub fn sql_type(&self) -> &SqlColumnType {
        match self {
            SqlTypeMapping::Simple { sql_type, .. } => sql_type,
            SqlTypeMapping::Complex { sql_type, .. } => sql_type,
            SqlTypeMapping::Serialized { sql_type, .. } => sql_type,
        }
    }

    /// Check if nullable
    pub fn is_nullable(&self) -> bool {
        match self {
            SqlTypeMapping::Simple { nullable, .. } => *nullable,
            SqlTypeMapping::Complex { nullable, .. } => *nullable,
            SqlTypeMapping::Serialized { .. } => true, // Serialized types are typically nullable
        }
    }

    /// Set nullable flag
    pub fn set_nullable(&mut self, nullable: bool) {
        match self {
            SqlTypeMapping::Simple { nullable: ref mut n, .. } => *n = nullable,
            SqlTypeMapping::Complex { nullable: ref mut n, .. } => *n = nullable,
            SqlTypeMapping::Serialized { .. } => {}, // Cannot change serialized nullability
        }
    }
}

/// fr fr Serialization format for complex types
#[derive(Debug, Clone, PartialEq)]
pub enum SerializationFormat {
    /// JSON serialization
    Json,
    /// Binary serialization
    Binary,
    /// MessagePack serialization
    MessagePack,
    /// Custom serialization
    Custom(String),
}

/// fr fr Custom mapping trait
pub trait CustomMapping: Send + Sync {
    /// Get mapping name
    fn name(&self) -> &str;
    
    /// Convert to SQL type mapping
    fn to_sql_mapping(&self) -> SqlTypeMapping;
    
    /// Convert CURSED value to SQL value
    fn to_sql_value(&self, value: Box<dyn std::any::Any>) -> Result<SqlValue, DatabaseError>;
    
    /// Convert SQL value to CURSED value
    fn from_sql_value(&self, sql_value: &SqlValue) -> Result<Box<dyn std::any::Any>, DatabaseError>;
}

/// fr fr Column mapper for result mapping
#[derive(Debug)]
pub struct ColumnMapper {
    /// Type mapper reference
    type_mapper: Arc<TypeMapper>,
    /// Column mappings
    column_mappings: HashMap<String, ColumnMappingInfo>,
}

impl ColumnMapper {
    /// slay Create new column mapper
    #[instrument(skip(type_mapper))]
    pub fn new(type_mapper: Arc<TypeMapper>) -> Self {
        debug!("Creating new column mapper");
        Self {
            type_mapper,
            column_mappings: HashMap::new(),
        }
    }

    /// facts Add column mapping
    #[instrument(skip(self))]
    pub fn add_mapping(&mut self, column_name: &str, mapping_info: ColumnMappingInfo) {
        debug!(column = column_name, "Adding column mapping");
        self.column_mappings.insert(column_name.to_string(), mapping_info);
    }

    /// periodt Map database row to entity fields
    #[instrument(skip(self, row))]
    pub fn map_row(&self, row: &HashMap<String, SqlValue>) -> Result<HashMap<String, Box<dyn std::any::Any>>, DatabaseError> {
        debug!("Mapping database row to entity fields");
        
        let mut mapped_fields = HashMap::new();
        
        for (column_name, sql_value) in row {
            if let Some(mapping_info) = self.column_mappings.get(column_name) {
                let mapped_value = self.type_mapper.map_from_sql(sql_value, &mapping_info.target_type)?;
                mapped_fields.insert(mapping_info.field_name.clone(), mapped_value);
            } else {
                // Try to infer mapping
                warn!(column = column_name, "No explicit mapping found, attempting inference");
                // For now, just store as SqlValue
                mapped_fields.insert(column_name.clone(), Box::new(sql_value.clone()));
            }
        }
        
        debug!(mapped_fields = mapped_fields.len(), "Row mapped successfully");
        Ok(mapped_fields)
    }
}

/// fr fr Column mapping information
#[derive(Debug, Clone)]
pub struct ColumnMappingInfo {
    /// Database column name
    pub column_name: String,
    /// Entity field name
    pub field_name: String,
    /// Target CURSED type
    pub target_type: String,
    /// Custom converter if any
    pub converter: Option<String>,
}

/// fr fr Result mapper for query results
#[derive(Debug)]
pub struct ResultMapper {
    /// Type mapper reference
    type_mapper: Arc<TypeMapper>,
    /// Column mapper
    column_mapper: ColumnMapper,
}

impl ResultMapper {
    /// slay Create new result mapper
    #[instrument(skip(type_mapper))]
    pub fn new(type_mapper: Arc<TypeMapper>) -> Self {
        debug!("Creating new result mapper");
        
        let column_mapper = ColumnMapper::new(type_mapper.clone());
        
        Self {
            type_mapper,
            column_mapper,
        }
    }

    /// facts Map query results to entities
    #[instrument(skip(self, rows))]
    pub fn map_to_entities<T: Entity>(&self, rows: &[HashMap<String, SqlValue>]) -> Result<Vec<T>, DatabaseError> {
        debug!(entity = T::table_name(), row_count = rows.len(), "Mapping query results to entities");
        
        let mut entities = Vec::new();
        
        for row in rows {
            let entity = T::from_row(row)?;
            entities.push(entity);
        }
        
        info!(entity = T::table_name(), mapped_count = entities.len(), "Results mapped to entities");
        Ok(entities)
    }

    /// periodt Map single row to entity
    #[instrument(skip(self, row))]
    pub fn map_to_entity<T: Entity>(&self, row: &HashMap<String, SqlValue>) -> Result<T, DatabaseError> {
        debug!(entity = T::table_name(), "Mapping row to entity");
        T::from_row(row)
    }

    /// bestie Map entities to database rows
    #[instrument(skip(self, entities))]
    pub fn map_from_entities<T: Entity>(&self, entities: &[T]) -> Vec<HashMap<String, SqlValue>> {
        debug!(entity = T::table_name(), entity_count = entities.len(), "Mapping entities to database rows");
        
        entities.iter()
            .map(|entity| entity.to_fields())
            .collect()
    }
}

/// fr fr Mapping registry for managing all mappings
#[derive(Debug)]
pub struct MappingRegistry {
    /// Registered type mappings
    type_mappings: Arc<Mutex<HashMap<String, SqlTypeMapping>>>,
    /// Registered custom mappings
    custom_mappings: Arc<Mutex<HashMap<String, Box<dyn CustomMapping>>>>,
    /// Statistics
    stats: Arc<Mutex<MappingStats>>,
}

impl MappingRegistry {
    /// slay Create new mapping registry
    #[instrument]
    pub fn new() -> Self {
        debug!("Creating new mapping registry");
        Self {
            type_mappings: Arc::new(Mutex::new(HashMap::new())),
            custom_mappings: Arc::new(Mutex::new(HashMap::new())),
            stats: Arc::new(Mutex::new(MappingStats::default())),
        }
    }

    /// facts Register type mapping
    #[instrument(skip(self))]
    pub fn register_type_mapping(&self, type_name: &str, mapping: SqlTypeMapping) -> Result<(), DatabaseError> {
        debug!(type_name = type_name, "Registering type mapping");
        
        if let Ok(mut mappings) = self.type_mappings.lock() {
            mappings.insert(type_name.to_string(), mapping);
            
            // Update stats
            if let Ok(mut stats) = self.stats.lock() {
                stats.registered_mappings += 1;
            }
            
            debug!("Type mapping registered successfully");
            Ok(())
        } else {
            Err(DatabaseError::internal_error("Failed to access type mappings"))
        }
    }

    /// periodt Get type mapping
    #[instrument(skip(self))]
    pub fn get_type_mapping(&self, type_name: &str) -> Option<SqlTypeMapping> {
        if let Ok(mappings) = self.type_mappings.lock() {
            let result = mappings.get(type_name).cloned();
            
            // Update stats
            if let Ok(mut stats) = self.stats.lock() {
                if result.is_some() {
                    stats.mapping_hits += 1;
                } else {
                    stats.mapping_misses += 1;
                }
            }
            
            result
        } else {
            None
        }
    }

    /// bestie Get mapping statistics
    #[instrument(skip(self))]
    pub fn stats(&self) -> MappingStats {
        if let Ok(stats) = self.stats.lock() {
            stats.clone()
        } else {
            MappingStats::default()
        }
    }
}

/// fr fr Mapping statistics
#[derive(Debug, Clone, Default)]
pub struct MappingStats {
    /// Number of registered mappings
    pub registered_mappings: u64,
    /// Mapping cache hits
    pub mapping_hits: u64,
    /// Mapping cache misses
    pub mapping_misses: u64,
    /// Custom mappings registered
    pub custom_mappings: u64,
}

impl MappingStats {
    /// Calculate hit ratio
    pub fn hit_ratio(&self) -> f64 {
        if self.mapping_hits + self.mapping_misses == 0 {
            0.0
        } else {
            self.mapping_hits as f64 / (self.mapping_hits + self.mapping_misses) as f64
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_test::traced_test;

    #[traced_test]
    #[test]
    fn test_type_mapper_creation() {
        let mapper = TypeMapper::new();
        
        // Test built-in mappings
        let i32_mapping = mapper.map_to_sql("i32").expect("Should map i32");
        assert!(matches!(i32_mapping, SqlTypeMapping::Simple { sql_type: SqlColumnType::Integer, nullable: false }));
        
        let string_mapping = mapper.map_to_sql("String").expect("Should map String");
        assert!(matches!(string_mapping, SqlTypeMapping::Simple { sql_type: SqlColumnType::Text, nullable: false }));
    }

    #[traced_test]
    #[test]
    fn test_option_type_mapping() {
        let mapper = TypeMapper::new();
        
        let option_i32_mapping = mapper.map_to_sql("Option<i32>").expect("Should map Option<i32>");
        assert!(matches!(option_i32_mapping, SqlTypeMapping::Simple { sql_type: SqlColumnType::Integer, nullable: true }));
        
        let option_string_mapping = mapper.map_to_sql("Option<String>").expect("Should map Option<String>");
        assert!(matches!(option_string_mapping, SqlTypeMapping::Simple { sql_type: SqlColumnType::Text, nullable: true }));
    }

    #[traced_test]
    #[test]
    fn test_sql_value_mapping() {
        let mapper = TypeMapper::new();
        
        let sql_value = SqlValue::Integer(42);
        let mapped = mapper.map_from_sql(&sql_value, "i32").expect("Should map SQL integer to i32");
        let i32_value = mapped.downcast_ref::<i32>().expect("Should be i32");
        assert_eq!(*i32_value, 42);
        
        let sql_string = SqlValue::String("hello".to_string());
        let mapped_string = mapper.map_from_sql(&sql_string, "String").expect("Should map SQL string to String");
        let string_value = mapped_string.downcast_ref::<String>().expect("Should be String");
        assert_eq!(*string_value, "hello");
    }

    #[traced_test]
    #[test]
    fn test_null_value_mapping() {
        let mapper = TypeMapper::new();
        
        let sql_null = SqlValue::Null;
        let mapped = mapper.map_from_sql(&sql_null, "Option<String>").expect("Should map NULL to Option");
        // Note: Actual None checking would require more complex type handling
    }

    #[traced_test]
    #[test]
    fn test_type_inference() {
        let mapper = TypeMapper::new();
        
        // Test inference for types not explicitly registered
        let u32_mapping = mapper.map_to_sql("u32").expect("Should infer u32 mapping");
        assert!(matches!(u32_mapping, SqlTypeMapping::Simple { sql_type: SqlColumnType::Integer, nullable: false }));
        
        let option_f64_mapping = mapper.map_to_sql("Option<f64>").expect("Should infer Option<f64> mapping");
        assert!(matches!(option_f64_mapping, SqlTypeMapping::Simple { sql_type: SqlColumnType::Double, nullable: true }));
    }

    #[traced_test]
    #[test]
    fn test_column_mapper() {
        let type_mapper = Arc::new(TypeMapper::new());
        let mut column_mapper = ColumnMapper::new(type_mapper);
        
        column_mapper.add_mapping("id", ColumnMappingInfo {
            column_name: "id".to_string(),
            field_name: "id".to_string(),
            target_type: "i64".to_string(),
            converter: None,
        });
        
        let mut row = HashMap::new();
        row.insert("id".to_string(), SqlValue::Integer(123));
        
        let mapped = column_mapper.map_row(&row).expect("Should map row");
        assert_eq!(mapped.len(), 1);
    }

    #[traced_test]
    #[test]
    fn test_mapping_registry() {
        let registry = MappingRegistry::new();
        
        let mapping = SqlTypeMapping::Simple {
            sql_type: SqlColumnType::Text,
            nullable: false,
        };
        
        registry.register_type_mapping("CustomType", mapping.clone()).expect("Should register mapping");
        
        let retrieved = registry.get_type_mapping("CustomType").expect("Should retrieve mapping");
        assert!(matches!(retrieved, SqlTypeMapping::Simple { sql_type: SqlColumnType::Text, nullable: false }));
        
        let stats = registry.stats();
        assert_eq!(stats.registered_mappings, 1);
        assert_eq!(stats.mapping_hits, 1);
    }

    #[traced_test]
    #[test]
    fn test_sql_type_mapping_methods() {
        let mut mapping = SqlTypeMapping::Simple {
            sql_type: SqlColumnType::Integer,
            nullable: false,
        };
        
        assert_eq!(*mapping.sql_type(), SqlColumnType::Integer);
        assert!(!mapping.is_nullable());
        
        mapping.set_nullable(true);
        assert!(mapping.is_nullable());
    }

    #[traced_test]
    #[test]
    fn test_mapping_stats() {
        let stats = MappingStats {
            registered_mappings: 10,
            mapping_hits: 8,
            mapping_misses: 2,
            custom_mappings: 3,
        };
        
        assert_eq!(stats.hit_ratio(), 0.8);
    }
}
