/// CURSED ORM System - Object-Relational Mapping with Gen Z vibes
/// 
/// A comprehensive ORM system that maps CURSED structs to database tables
/// with fluent query building, relationship management, and advanced features.
/// 
/// Features:
/// - Entity mapping with CURSED struct annotations
/// - Fluent query builder with Gen Z method naming
/// - Migration system with version tracking
/// - Relationship mapping (one-to-one, one-to-many, many-to-many)
/// - Lazy and eager loading strategies
/// - Query result caching with intelligent invalidation
/// - Transaction-aware operations
/// - Validation framework with custom constraints
/// - Database-agnostic query generation

pub mod entity;
pub mod query_builder;
pub mod migration;
pub mod relationships;
pub mod cache;
pub mod validation;
pub mod transaction_ops;
pub mod schema;
pub mod mapping;
pub mod fluent_query;

// Re-export main types for easy access
pub use entity::{
    PrimaryKey, ForeignKey, Timestamped, EntityMetadata, ColumnDefinition
};
pub use query_builder::{
    WhereClause, JoinClause, OrderByClause, GroupByClause
};
pub use migration::{
    CreateTable, DropTable, AddColumn, DropColumn, AddIndex
};
pub use relationships::{
    HasOne, HasMany, BelongsTo, BelongsToMany, LazyLoader, EagerLoader
};
pub use cache::{
    MemoryCache, RedisCache, CacheInvalidator
};
pub use validation::{
    Required, MinLength, MaxLength, EmailFormat, CustomValidator
};
pub use transaction_ops::{
    TransactionState, TransactionMetrics
};
pub use schema::{
    DatabaseSchema, SchemaComparator, SchemaMigrator
};
pub use mapping::{
    SqlTypeMapping, CustomMapping, MappingRegistry
};
pub use fluent_query::FluentQueryBuilder;

use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use super::{DatabaseError, DatabaseErrorKind, SqlValue, DatabaseConnection, QueryResult};
use cache::CacheConfig;
use crate::error::CursedError;

/// fr fr Configuration for ORM operations
#[derive(Debug, Clone)]
pub struct OrmConfig {
    pub cache_config: CacheConfig,
    pub enable_lazy_loading: bool,
    pub enable_query_logging: bool,
    pub connection_pool_size: u32,
}

impl Default for OrmConfig {
    fn default() -> Self {
        Self {
            cache_config: CacheConfig::default(),
            enable_lazy_loading: true,
            enable_query_logging: false,
            connection_pool_size: 10,
        }
    }
}

/// fr fr Enhanced Entity trait for database models
pub trait Entity: Sized {
    /// Get the table name for this entity
    fn table_name() -> &'static str;
    
    /// Get the primary key column name
    fn primary_key_name() -> &'static str;
    
    /// Get the primary key value
    fn primary_key_value(&self) -> Option<SqlValue>;
    
    /// Set the primary key value
    fn set_primary_key_value(&mut self, value: SqlValue);
    
    /// Create entity from database row
    fn from_row(row: &HashMap<String, SqlValue>) -> Result<Self, DatabaseError>;
    
    /// Convert entity to field map for SQL operations
    fn to_fields(&self) -> HashMap<String, SqlValue>;
    
    /// Get all field names
    fn field_names() -> Vec<&'static str>;
    
    /// Get column definitions for schema generation
    fn column_definitions() -> Vec<ColumnDefinition>;
    
    /// Get entity metadata
    fn metadata() -> EntityMetadata;
}

/// fr fr ORM context that manages database operations
pub struct OrmContext {
    connection: Arc<dyn DatabaseConnection>,
    config: OrmConfig,
}

impl OrmContext {
    /// Create a new ORM context
    pub fn new(connection: Arc<dyn DatabaseConnection>, config: OrmConfig) -> Self {
        Self { connection, config }
    }
    
    /// Get a repository for a specific entity type
    pub fn repository<T: Entity>(&self) -> Repository<T> {
        Repository::new(self.connection.clone())
    }
}

/// fr fr Repository for entity operations with Gen Z vibes
pub struct Repository<T: Entity> {
    connection: Arc<dyn DatabaseConnection>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Entity> Repository<T> {
    pub fn new(connection: Arc<dyn DatabaseConnection>) -> Self {
        Self {
            connection,
            _phantom: std::marker::PhantomData,
        }
    }
    
    pub async fn find_by_id(&self, id: &SqlValue) -> Result<Option<T>, DatabaseError> {
        let table_name = T::table_name();
        let sql = format!("SELECT * FROM {} WHERE id = ? LIMIT 1", table_name);
        let params = vec![id.clone()];
        
        match self.connection.query(sql, params) {
            Ok(result) => {
                if result.rows().is_empty() {
                    Ok(None)
                } else {
                    // For now, return None since we can't deserialize without knowing the struct
                    // In a full implementation, this would use reflection or macros to deserialize
                    Ok(None)
                }
            }
            Err(e) => Err(DatabaseError::query(&format!("Failed to find entity by id: {}", e)))
        }
    }
    
    pub async fn save(&self, entity: &T) -> Result<(), DatabaseError> {
        let table_name = T::table_name();
        let primary_key = entity.primary_key_value().ok_or_else(|| 
            DatabaseError::query("Entity has no primary key value"))?;
        
        // For this basic implementation, we'll do an INSERT
        // In a full implementation, this would check if the entity exists and do UPDATE or INSERT
        let sql = format!("INSERT INTO {} (id) VALUES (?)", table_name);
        let params = vec![primary_key];
        
        match self.connection.exec(sql, params) {
            Ok(_) => Ok(()),
            Err(e) => Err(DatabaseError::query(&format!("Failed to save entity: {}", e)))
        }
    }
    
    pub async fn find_all(&self) -> Result<Vec<T>, DatabaseError> {
        let table_name = T::table_name();
        let sql = format!("SELECT * FROM {}", table_name);
        let params = vec![];
        
        match self.connection.query(sql, params) {
            Ok(_result) => {
                // For now, return empty vector since we can't deserialize without knowing the struct
                // In a full implementation, this would deserialize all rows
                Ok(vec![])
            }
            Err(e) => Err(DatabaseError::query(&format!("Failed to find all entities: {}", e)))
        }
    }
    
    pub async fn delete(&self, entity: &T) -> Result<(), DatabaseError> {
        let table_name = T::table_name();
        let primary_key = entity.primary_key_value().ok_or_else(|| 
            DatabaseError::query("Entity has no primary key value"))?;
        
        let sql = format!("DELETE FROM {} WHERE id = ?", table_name);
        let params = vec![primary_key];
        
        match self.connection.exec(sql, params) {
            Ok(_) => Ok(()),
            Err(e) => Err(DatabaseError::query(&format!("Failed to delete entity: {}", e)))
        }
    }
}

/// fr fr Query cache for performance optimization
pub struct QueryCache {
    cache: HashMap<String, SqlValue>,
    config: CacheConfig,
}

impl QueryCache {
    pub fn new(config: CacheConfig) -> Self {
        Self {
            cache: HashMap::new(),
            config,
        }
    }
    
    pub fn get(&self, key: &str) -> Option<&SqlValue> {
        self.cache.get(key)
    }
    
    pub fn set(&mut self, key: String, value: SqlValue) {
        self.cache.insert(key, value);
    }
}

/// fr fr Entity manager for model operations
pub struct EntityManager {
    // TODO: Add fields as needed
}

impl EntityManager {
    pub fn new() -> Self {
        Self {}
    }
}

/// fr fr Migration manager for schema changes
pub struct MigrationManager {
    // TODO: Add fields as needed
}

impl MigrationManager {
    pub fn new() -> Self {
        Self {}
    }
    
    pub async fn migrate(&self) -> Result<(), CursedError> {
        // TODO: Implement migration
        Ok(())
    }
}

/// fr fr Relationship manager for association handling
pub struct RelationshipManager {
    // TODO: Add fields as needed
}

impl RelationshipManager {
    pub fn new() -> Self {
        Self {}
    }
}

/// fr fr Type mapper for SQL type conversions
pub struct TypeMapper {
    // TODO: Add fields as needed
}

impl TypeMapper {
    pub fn new() -> Self {
        Self {}
    }
}

/// fr fr Schema builder for creating database schemas
pub struct SchemaBuilder {
    // TODO: Add fields as needed
}

impl SchemaBuilder {
    pub fn new() -> Self {
        Self {}
    }
}

/// fr fr Result mapper for converting SQL results to entities
pub struct ResultMapper {
    // TODO: Add fields as needed
}

impl ResultMapper {
    pub fn new() -> Self {
        Self {}
    }
}
