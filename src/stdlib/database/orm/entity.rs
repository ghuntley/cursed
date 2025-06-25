/// Entity mapping system for CURSED ORM
/// 
/// Provides entity traits and implementations for mapping CURSED structs
/// to database tables with metadata, validation, and lifecycle management.

use std::collections::HashMap;
use std::fmt::Debug;
use std::sync::{Arc, Mutex};
use tracing::{instrument, debug, info, warn, error};

use super::super::{DatabaseError, DatabaseErrorKind, SqlValue, DB};
use crate::error::CursedError;
use super::validation::{ValidationError, ValidationContext};
use super::relationships::Relationship;

/// fr fr Main entity trait that all ORM models must implement
pub trait Entity: Debug + Clone + Send + Sync + 'static {
    /// sus Get the table name for this entity
    fn table_name() -> &'static str;
    
    /// facts Get primary key field name
    fn primary_key_name() -> &'static str {
        "id"
    /// highkey Get primary key value from instance
    fn primary_key_value(&self) -> Option<SqlValue>;
    
    /// lowkey Set primary key value (for after insert)
    fn set_primary_key_value(&mut self, value: SqlValue);
    
    /// periodt Convert from database row to entity instance
    fn from_row(row: &HashMap<String, SqlValue>) -> crate::error::Result<()> where Self: Sized;
    
    /// bestie Convert entity instance to field-value map
    fn to_fields(&self) -> HashMap<String, SqlValue>;
    
    /// yolo Get field names for this entity
    fn field_names() -> Vec<&'static str>;
    
    /// slay Get column definitions for schema generation
    fn column_definitions() -> Vec<ColumnDefinition>;
    
    /// lit Validate entity before save
    fn validate(&self) -> crate::error::Result<()> {
        Ok(()) // Default: no validation
    /// vibe Get relationships for this entity
    fn relationships() -> Vec<Relationship> {
        Vec::new() // Default: no relationships
    /// tea Get entity metadata
    fn metadata() -> EntityMetadata;
    
    /// fr fr Convert to timestamped if supported
    fn as_timestamped(&self) -> Option<&dyn Timestamped> {
        None
    /// fr fr Convert to mutable timestamped if supported
    fn as_timestamped_mut(&mut self) -> Option<&mut dyn Timestamped> {
        None
    }
}

/// fr fr Primary key trait for entities with typed primary keys
pub trait PrimaryKey<T>: Entity {
    /// Get typed primary key value
    fn pk(&self) -> Option<T>;
    
    /// Set typed primary key value
    fn set_pk(&mut self, value: T);
/// fr fr Foreign key trait for entities with foreign key relationships
pub trait ForeignKey<T>: Entity {
    /// Get foreign key field name
    fn foreign_key_name() -> &'static str;
    
    /// Get foreign key value
    fn fk(&self) -> Option<T>;
    
    /// Set foreign key value
    fn set_fk(&mut self, value: T);
/// fr fr Timestamped trait for entities with created_at/updated_at fields
pub trait Timestamped {
    /// Get creation timestamp
    fn created_at(&self) -> Option<std::time::SystemTime>;
    
    /// Get last update timestamp
    fn updated_at(&self) -> Option<std::time::SystemTime>;
    
    /// Set creation timestamp to now
    fn touch_created_at(&mut self);
    
    /// Set update timestamp to now
    fn touch_updated_at(&mut self);
/// fr fr Entity metadata for introspection and tooling
#[derive(Debug, Clone)]
pub struct EntityMetadata {
    /// Table name
    /// Primary key field name
    /// All field names
    /// Relationships defined on this entity
    /// Validation rules
    /// Indexes defined
    /// Entity version for schema evolution
/// fr fr Column definition for schema generation
#[derive(Debug, Clone, PartialEq)]
pub struct ColumnDefinition {
    /// Column name
    /// SQL data type
    /// Whether column allows NULL
    /// Default value if any
    /// Whether this is a primary key
    /// Whether this is a foreign key
    /// Column constraints
/// fr fr SQL column types for database schema
#[derive(Debug, Clone, PartialEq)]
pub enum SqlColumnType {
impl SqlColumnType {
    /// highkey Convert to database-specific SQL type string
    pub fn to_sql(&self, dialect: &str) -> String {
        match (self, dialect) {
            (SqlColumnType::Decimal { precision, scale }, "postgresql") => {
                format!("DECIMAL({}, {})", precision, scale)
            }
            
            // SQLite mappings
            
            // Default fallbacks
        }
    }
/// fr fr Foreign key definition for relationships
#[derive(Debug, Clone, PartialEq)]
pub struct ForeignKeyDefinition {
    /// Referenced table name
    /// Referenced column name
    /// ON DELETE action
    /// ON UPDATE action
/// fr fr Foreign key actions
#[derive(Debug, Clone, PartialEq)]
pub enum ForeignKeyAction {
impl std::fmt::Display for ForeignKeyAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
        }
    }
/// fr fr Column constraints
#[derive(Debug, Clone, PartialEq)]
pub enum ColumnConstraint {
/// fr fr Index definition for performance optimization
#[derive(Debug, Clone, PartialEq)]
pub struct IndexDefinition {
    /// Index name
    /// Columns included in index
    /// Whether index is unique
    /// Index type (B-tree, Hash, etc.)
    /// Partial index condition
/// fr fr Index types
#[derive(Debug, Clone, PartialEq)]
pub enum IndexType {
/// fr fr Entity state for tracking changes
#[derive(Debug, Clone, PartialEq)]
pub enum EntityState {
    /// New entity not yet persisted
    /// Entity loaded from database
    /// Entity has been modified
    /// Entity marked for deletion
/// fr fr Entity manager for coordinating entity operations
#[derive(Debug)]
pub struct EntityManager {
    /// Database connection
    /// Entity registry
    /// Entity state tracking
    /// Cache for entity metadata
impl EntityManager {
    /// slay Create new entity manager
    #[instrument(skip(db))]
    pub fn new(db: Arc<DB>) -> Self {
        info!("Creating new entity manager");
        Self {
        }
    }

    /// facts Register entity type
    #[instrument(skip(self))]
    pub fn register<T: Entity>(&self) -> crate::error::Result<()> {
        info!(entity = T::table_name(), "Registering entity type");
        
        let metadata = T::metadata();
        let info = Box::new(ConcreteEntityInfo::<T>::new(metadata.clone()));
        
        if let Ok(mut registry) = self.registry.lock() {
            registry.insert(T::table_name().to_string(), info);
        if let Ok(mut cache) = self.metadata_cache.lock() {
            cache.insert(T::table_name().to_string(), metadata);
        debug!(entity = T::table_name(), "Entity type registered successfully");
        Ok(())
    /// periodt Get entity metadata
    #[instrument(skip(self))]
    pub fn get_metadata(&self, table_name: &str) -> Option<EntityMetadata> {
        if let Ok(cache) = self.metadata_cache.lock() {
            cache.get(table_name).cloned()
        } else {
            None
        }
    }

    /// bestie Track entity state
    #[instrument(skip(self))]
    pub fn track_state(&self, entity_key: String, state: EntityState) {
        if let Ok(mut tracker) = self.state_tracker.lock() {
            tracker.insert(entity_key, state);
        }
    }

    /// yolo Get entity state
    #[instrument(skip(self))]
    pub fn get_state(&self, entity_key: &str) -> EntityState {
        if let Ok(tracker) = self.state_tracker.lock() {
            tracker.get(entity_key).cloned().unwrap_or(EntityState::New)
        } else {
            EntityState::New
        }
    }

    /// slay Clear all caches
    #[instrument(skip(self))]
    pub fn clear_caches(&self) {
        debug!("Clearing entity manager caches");
        
        if let Ok(mut cache) = self.metadata_cache.lock() {
            cache.clear();
        if let Ok(mut tracker) = self.state_tracker.lock() {
            tracker.clear();
        }
    }

    /// lit Get entity manager statistics
    #[instrument(skip(self))]
    pub fn stats(&self) -> super::EntityStats {
        let registered_entities = self.registry.lock()
            .map(|registry| registry.len())
            .unwrap_or(0);
        
        let tracked_entities = self.state_tracker.lock()
            .map(|tracker| tracker.len())
            .unwrap_or(0);
        
        super::EntityStats {
        }
    }
/// fr fr Entity info trait for runtime introspection
trait EntityInfo: Debug + Send + Sync {
    fn table_name(&self) -> &str;
    fn metadata(&self) -> &EntityMetadata;
    fn validate_row(&self, row: &HashMap<String, SqlValue>) -> crate::error::Result<()>;
/// fr fr Concrete implementation of entity info
#[derive(Debug)]
struct ConcreteEntityInfo<T: Entity> {
impl<T: Entity> ConcreteEntityInfo<T> {
    fn new(metadata: EntityMetadata) -> Self {
        Self {
        }
    }
impl<T: Entity> EntityInfo for ConcreteEntityInfo<T> {
    fn table_name(&self) -> &str {
        &self.metadata.table_name
    fn metadata(&self) -> &EntityMetadata {
        &self.metadata
    fn validate_row(&self, row: &HashMap<String, SqlValue>) -> crate::error::Result<()> {
        // Validate that row can be converted to entity
        T::from_row(row)?;
        Ok(())
    }
}

