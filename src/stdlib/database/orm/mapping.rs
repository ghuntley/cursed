/// Type mapping system for CURSED ORM
/// 
/// Provides type conversion between CURSED types and SQL types,
/// custom mapping support, and result mapping for query results.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{instrument, debug, info, warn, error};

use crate::error::CursedError;
use super::super::{DatabaseError, DatabaseErrorKind, SqlValue};
use super::entity::{Entity, SqlColumnType};

/// fr fr Type mapper for converting between CURSED and SQL types
#[derive(Debug)]
pub struct TypeMapper {
    /// Built-in type mappings
    /// Custom type mappings
    /// Mapping registry
impl TypeMapper {
    /// slay Create new type mapper
    #[instrument]
    pub fn new() -> Self {
        info!("Creating new type mapper");
        
        let mut mapper = Self {
        
        mapper.register_builtin_mappings();
        mapper
    /// facts Register built-in type mappings
    #[instrument(skip(self))]
    fn register_builtin_mappings(&mut self) {
        debug!("Registering built-in type mappings");
        
        // Integer types
        self.builtin_mappings.insert(
            SqlTypeMapping::Simple {
        );
        
        self.builtin_mappings.insert(
            SqlTypeMapping::Simple {
        );
        
        self.builtin_mappings.insert(
            SqlTypeMapping::Simple {
        );
        
        self.builtin_mappings.insert(
            SqlTypeMapping::Simple {
        );
        
        // Float types
        self.builtin_mappings.insert(
            SqlTypeMapping::Simple {
        );
        
        self.builtin_mappings.insert(
            SqlTypeMapping::Simple {
        );
        
        // String types
        self.builtin_mappings.insert(
            SqlTypeMapping::Simple {
        );
        
        self.builtin_mappings.insert(
            SqlTypeMapping::Simple {
        );
        
        // Boolean type
        self.builtin_mappings.insert(
            SqlTypeMapping::Simple {
        );
        
        // DateTime types
        self.builtin_mappings.insert(
            SqlTypeMapping::Simple {
        );
        
        // Byte arrays
        self.builtin_mappings.insert(
            SqlTypeMapping::Simple {
        );
        
        // JSON types
        self.builtin_mappings.insert(
            SqlTypeMapping::Simple {
        );
        
        debug!(count = self.builtin_mappings.len(), "Built-in type mappings registered");
    /// periodt Map CURSED type to SQL type
    #[instrument(skip(self))]
    pub fn map_to_sql(&self, cursed_type: &str) -> crate::error::Result<()> {
        debug!(cursed_type = cursed_type, "Mapping CURSED type to SQL");
        
        // Check built-in mappings first
        if let Some(mapping) = self.builtin_mappings.get(cursed_type) {
            debug!("Found built-in mapping");
            return Ok(mapping.clone());
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
        error!(cursed_type = cursed_type, "No mapping found for CURSED type");
        Err(DatabaseError::validation_error(&format!(
            cursed_type
        )))
    /// bestie Map SQL value to CURSED value
    #[instrument(skip(self))]
    pub fn map_from_sql(&self, sql_value: &SqlValue, target_type: &str) -> crate::error::Result<()> {
        debug!(target_type = target_type, "Mapping SQL value to CURSED type");
        
        match (sql_value, target_type) {
            (SqlValue::Null, type_name) if type_name.starts_with("Option<") => {
                // Return None for Option types
                Ok(Box::new(Option::<()>::None))
            }
            _ => {
                error!(target_type = target_type, sql_value = ?sql_value, "Cannot map SQL value to CURSED type");
                Err(DatabaseError::validation_error(&format!(
                    sql_value, target_type
                )))
            }
        }
    /// yolo Register custom mapping
    #[instrument(skip(self, mapping))]
    pub fn register_custom_mapping(&self, cursed_type: &str, mapping: Box<dyn CustomMapping>) -> crate::error::Result<()> {
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
    /// lit Infer basic type mapping
    fn infer_basic_type(&self, type_name: &str) -> Option<SqlTypeMapping> {
        match type_name {
            "u8" | "u16" | "u32" | "i8" | "i16" | "i32" => Some(SqlTypeMapping::Simple {
            "u64" | "i64" | "usize" | "isize" => Some(SqlTypeMapping::Simple {
            "f32" => Some(SqlTypeMapping::Simple {
            "f64" => Some(SqlTypeMapping::Simple {
            "bool" => Some(SqlTypeMapping::Simple {
            "str" | "&str" => Some(SqlTypeMapping::Simple {
        }
    }
/// fr fr SQL type mapping configuration
#[derive(Debug, Clone)]
pub enum SqlTypeMapping {
    /// Simple direct mapping
    Simple {
    /// Complex mapping with custom converter
    Complex {
        converter: String, // Name of converter function
    /// Serialized mapping (JSON/Binary)
    Serialized {
impl SqlTypeMapping {
    /// Get SQL column type
    pub fn sql_type(&self) -> &SqlColumnType {
        match self {
        }
    }

    /// Check if nullable
    pub fn is_nullable(&self) -> bool {
        match self {
            SqlTypeMapping::Serialized { .. } => true, // Serialized types are typically nullable
        }
    }

    /// Set nullable flag
    pub fn set_nullable(&mut self, nullable: bool) {
        match self {
            SqlTypeMapping::Serialized { .. } => {}, // Cannot change serialized nullability
        }
    }
/// fr fr Serialization format for complex types
#[derive(Debug, Clone, PartialEq)]
pub enum SerializationFormat {
    /// JSON serialization
    /// Binary serialization
    /// MessagePack serialization
    /// Custom serialization
/// fr fr Custom mapping trait
pub trait CustomMapping: Send + Sync + std::fmt::Debug {
    /// Get mapping name
    fn name(&self) -> &str;
    
    /// Convert to SQL type mapping
    fn to_sql_mapping(&self) -> SqlTypeMapping;
    
    /// Convert CURSED value to SQL value
    fn to_sql_value(&self, value: Box<dyn std::any::Any>) -> crate::error::Result<()>;
    
    /// Convert SQL value to CURSED value
    fn from_sql_value(&self, sql_value: &SqlValue) -> crate::error::Result<()>;
/// fr fr Column mapper for result mapping
#[derive(Debug)]
pub struct ColumnMapper {
    /// Type mapper reference
    /// Column mappings
impl ColumnMapper {
    /// slay Create new column mapper
    #[instrument(skip(type_mapper))]
    pub fn new(type_mapper: Arc<TypeMapper>) -> Self {
        debug!("Creating new column mapper");
        Self {
        }
    }

    /// facts Add column mapping
    #[instrument(skip(self))]
    pub fn add_mapping(&mut self, column_name: &str, mapping_info: ColumnMappingInfo) {
        debug!(column = column_name, "Adding column mapping");
        self.column_mappings.insert(column_name.to_string(), mapping_info);
    /// periodt Map database row to entity fields
    #[instrument(skip(self, row))]
    pub fn map_row(&self, row: &HashMap<String, SqlValue>) -> crate::error::Result<()> {
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
    /// Entity field name
    /// Target CURSED type
    /// Custom converter if any
/// fr fr Result mapper for query results
#[derive(Debug)]
pub struct ResultMapper {
    /// Type mapper reference
    /// Column mapper
impl ResultMapper {
    /// slay Create new result mapper
    #[instrument(skip(type_mapper))]
    pub fn new(type_mapper: Arc<TypeMapper>) -> Self {
        debug!("Creating new result mapper");
        
        let column_mapper = ColumnMapper::new(type_mapper.clone());
        
        Self {
        }
    }

    /// facts Map query results to entities
    #[instrument(skip(self, rows))]
    pub fn map_to_entities<T: Entity>(&self, rows: &[HashMap<String, SqlValue>]) -> crate::error::Result<()> {
        debug!(entity = T::table_name(), row_count = rows.len(), "Mapping query results to entities");
        
        let mut entities = Vec::new();
        
        for row in rows {
            let entity = T::from_row(row)?;
            entities.push(entity);
        info!(entity = T::table_name(), mapped_count = entities.len(), "Results mapped to entities");
        Ok(entities)
    /// periodt Map single row to entity
    #[instrument(skip(self, row))]
    pub fn map_to_entity<T: Entity>(&self, row: &HashMap<String, SqlValue>) -> crate::error::Result<()> {
        debug!(entity = T::table_name(), "Mapping row to entity");
        T::from_row(row)
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
    /// Registered custom mappings
    /// Statistics
impl MappingRegistry {
    /// slay Create new mapping registry
    #[instrument]
    pub fn new() -> Self {
        debug!("Creating new mapping registry");
        Self {
        }
    }

    /// facts Register type mapping
    #[instrument(skip(self))]
    pub fn register_type_mapping(&self, type_name: &str, mapping: SqlTypeMapping) -> crate::error::Result<()> {
        debug!(type_name = type_name, "Registering type mapping");
        
        if let Ok(mut mappings) = self.type_mappings.lock() {
            mappings.insert(type_name.to_string(), mapping);
            
            // Update stats
            if let Ok(mut stats) = self.stats.lock() {
                stats.registered_mappings += 1;
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
/// fr fr Mapping statistics
#[derive(Debug, Clone, Default)]
pub struct MappingStats {
    /// Number of registered mappings
    /// Mapping cache hits
    /// Mapping cache misses
    /// Custom mappings registered
impl MappingStats {
    /// Calculate hit ratio
    pub fn hit_ratio(&self) -> f64 {
        if self.mapping_hits + self.mapping_misses == 0 {
            0.0
        } else {
            self.mapping_hits as f64 / (self.mapping_hits + self.mapping_misses) as f64
        }
    }
