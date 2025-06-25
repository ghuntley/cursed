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
    }
    
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
    }
    
    /// vibe Get relationships for this entity
    fn relationships() -> Vec<Relationship> {
        Vec::new() // Default: no relationships
    }
    
    /// tea Get entity metadata
    fn metadata() -> EntityMetadata;
    
    /// fr fr Convert to timestamped if supported
    fn as_timestamped(&self) -> Option<&dyn Timestamped> {
        None
    }
    
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
}

/// fr fr Foreign key trait for entities with foreign key relationships
pub trait ForeignKey<T>: Entity {
    /// Get foreign key field name
    fn foreign_key_name() -> &'static str;
    
    /// Get foreign key value
    fn fk(&self) -> Option<T>;
    
    /// Set foreign key value
    fn set_fk(&mut self, value: T);
}

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
}

/// fr fr Entity metadata for introspection and tooling
#[derive(Debug, Clone)]
pub struct EntityMetadata {
    /// Table name
    pub table_name: String,
    /// Primary key field name
    pub primary_key: String,
    /// All field names
    pub fields: Vec<String>,
    /// Relationships defined on this entity
    pub relationships: Vec<String>,
    /// Validation rules
    pub validation_rules: Vec<String>,
    /// Indexes defined
    pub indexes: Vec<IndexDefinition>,
    /// Entity version for schema evolution
    pub version: u32,
}

/// fr fr Column definition for schema generation
#[derive(Debug, Clone, PartialEq)]
pub struct ColumnDefinition {
    /// Column name
    pub name: String,
    /// SQL data type
    pub sql_type: SqlColumnType,
    /// Whether column allows NULL
    pub nullable: bool,
    /// Default value if any
    pub default: Option<String>,
    /// Whether this is a primary key
    pub primary_key: bool,
    /// Whether this is a foreign key
    pub foreign_key: Option<ForeignKeyDefinition>,
    /// Column constraints
    pub constraints: Vec<ColumnConstraint>,
}

/// fr fr SQL column types for database schema
#[derive(Debug, Clone, PartialEq)]
pub enum SqlColumnType {
    Integer,
    BigInteger,
    SmallInteger,
    Decimal { precision: u32, scale: u32 },
    Float,
    Double,
    Boolean,
    Text,
    VarChar { length: u32 },
    Char { length: u32 },
    Binary,
    VarBinary { length: u32 },
    Date,
    DateTime,
    Timestamp,
    Json,
    Uuid,
}

impl SqlColumnType {
    /// highkey Convert to database-specific SQL type string
    pub fn to_sql(&self, dialect: &str) -> String {
        match (self, dialect) {
            (SqlColumnType::Integer, "postgresql") => "INTEGER".to_string(),
            (SqlColumnType::BigInteger, "postgresql") => "BIGINT".to_string(),
            (SqlColumnType::SmallInteger, "postgresql") => "SMALLINT".to_string(),
            (SqlColumnType::Decimal { precision, scale }, "postgresql") => {
                format!("DECIMAL({}, {})", precision, scale)
            }
            (SqlColumnType::Float, "postgresql") => "REAL".to_string(),
            (SqlColumnType::Double, "postgresql") => "DOUBLE PRECISION".to_string(),
            (SqlColumnType::Boolean, "postgresql") => "BOOLEAN".to_string(),
            (SqlColumnType::Text, "postgresql") => "TEXT".to_string(),
            (SqlColumnType::VarChar { length }, "postgresql") => format!("VARCHAR({})", length),
            (SqlColumnType::DateTime, "postgresql") => "TIMESTAMP".to_string(),
            (SqlColumnType::Json, "postgresql") => "JSONB".to_string(),
            (SqlColumnType::Uuid, "postgresql") => "UUID".to_string(),
            
            // SQLite mappings
            (SqlColumnType::Integer, "sqlite") => "INTEGER".to_string(),
            (SqlColumnType::BigInteger, "sqlite") => "INTEGER".to_string(),
            (SqlColumnType::Float | SqlColumnType::Double, "sqlite") => "REAL".to_string(),
            (SqlColumnType::Boolean, "sqlite") => "INTEGER".to_string(),
            (SqlColumnType::Text | SqlColumnType::VarChar { .. }, "sqlite") => "TEXT".to_string(),
            (SqlColumnType::DateTime | SqlColumnType::Timestamp, "sqlite") => "TEXT".to_string(),
            (SqlColumnType::Json, "sqlite") => "TEXT".to_string(),
            
            // Default fallbacks
            _ => format!("{:?}", self).to_uppercase(),
        }
    }
}

/// fr fr Foreign key definition for relationships
#[derive(Debug, Clone, PartialEq)]
pub struct ForeignKeyDefinition {
    /// Referenced table name
    pub referenced_table: String,
    /// Referenced column name
    pub referenced_column: String,
    /// ON DELETE action
    pub on_delete: ForeignKeyAction,
    /// ON UPDATE action
    pub on_update: ForeignKeyAction,
}

/// fr fr Foreign key actions
#[derive(Debug, Clone, PartialEq)]
pub enum ForeignKeyAction {
    Cascade,
    SetNull,
    Restrict,
    NoAction,
    SetDefault,
}

impl std::fmt::Display for ForeignKeyAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ForeignKeyAction::Cascade => write!(f, "CASCADE"),
            ForeignKeyAction::SetNull => write!(f, "SET NULL"),
            ForeignKeyAction::Restrict => write!(f, "RESTRICT"),
            ForeignKeyAction::NoAction => write!(f, "NO ACTION"),
            ForeignKeyAction::SetDefault => write!(f, "SET DEFAULT"),
        }
    }
}

/// fr fr Column constraints
#[derive(Debug, Clone, PartialEq)]
pub enum ColumnConstraint {
    NotNull,
    Unique,
    Check(String),
    Default(String),
}

/// fr fr Index definition for performance optimization
#[derive(Debug, Clone, PartialEq)]
pub struct IndexDefinition {
    /// Index name
    pub name: String,
    /// Columns included in index
    pub columns: Vec<String>,
    /// Whether index is unique
    pub unique: bool,
    /// Index type (B-tree, Hash, etc.)
    pub index_type: IndexType,
    /// Partial index condition
    pub condition: Option<String>,
}

/// fr fr Index types
#[derive(Debug, Clone, PartialEq)]
pub enum IndexType {
    BTree,
    Hash,
    Gin,
    Gist,
    Partial,
    Composite,
}

/// fr fr Entity state for tracking changes
#[derive(Debug, Clone, PartialEq)]
pub enum EntityState {
    /// New entity not yet persisted
    New,
    /// Entity loaded from database
    Clean,
    /// Entity has been modified
    Dirty,
    /// Entity marked for deletion
    Deleted,
}

/// fr fr Entity manager for coordinating entity operations
#[derive(Debug)]
pub struct EntityManager {
    /// Database connection
    db: Arc<DB>,
    /// Entity registry
    registry: Arc<Mutex<HashMap<String, Box<dyn EntityInfo>>>>,
    /// Entity state tracking
    state_tracker: Arc<Mutex<HashMap<String, EntityState>>>,
    /// Cache for entity metadata
    metadata_cache: Arc<Mutex<HashMap<String, EntityMetadata>>>,
}

impl EntityManager {
    /// slay Create new entity manager
    #[instrument(skip(db))]
    pub fn new(db: Arc<DB>) -> Self {
        info!("Creating new entity manager");
        Self {
            db,
            registry: Arc::new(Mutex::new(HashMap::new())),
            state_tracker: Arc::new(Mutex::new(HashMap::new())),
            metadata_cache: Arc::new(Mutex::new(HashMap::new())),
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
        }
        
        if let Ok(mut cache) = self.metadata_cache.lock() {
            cache.insert(T::table_name().to_string(), metadata);
        }
        
        debug!(entity = T::table_name(), "Entity type registered successfully");
        Ok(())
    }

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
        }
        
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
            total_queries: registered_entities as u64,
            cache_hits: tracked_entities as u64,
            cache_misses: 0,
        }
    }
}

/// fr fr Entity info trait for runtime introspection
trait EntityInfo: Debug + Send + Sync {
    fn table_name(&self) -> &str;
    fn metadata(&self) -> &EntityMetadata;
    fn validate_row(&self, row: &HashMap<String, SqlValue>) -> crate::error::Result<()>;
}

/// fr fr Concrete implementation of entity info
#[derive(Debug)]
struct ConcreteEntityInfo<T: Entity> {
    metadata: EntityMetadata,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Entity> ConcreteEntityInfo<T> {
    fn new(metadata: EntityMetadata) -> Self {
        Self {
            metadata,
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Entity> EntityInfo for ConcreteEntityInfo<T> {
    fn table_name(&self) -> &str {
        &self.metadata.table_name
    }
    
    fn metadata(&self) -> &EntityMetadata {
        &self.metadata
    }
    
    fn validate_row(&self, row: &HashMap<String, SqlValue>) -> crate::error::Result<()> {
        // Validate that row can be converted to entity
        T::from_row(row)?;
        Ok(())
    }
}

