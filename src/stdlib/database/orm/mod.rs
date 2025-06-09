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

// Re-export main types for easy access
pub use entity::{
    Entity, EntityManager, EntityState, EntityMetadata,
    PrimaryKey, ForeignKey, Timestamped
};
pub use query_builder::{
    VibeQuery, FluentQueryBuilder, QueryExecutor,
    WhereClause, JoinClause, OrderByClause, GroupByClause
};
pub use migration::{
    Migration, MigrationManager, MigrationStatus, SchemaVersion,
    CreateTable, DropTable, AddColumn, DropColumn, AddIndex
};
pub use relationships::{
    Relationship, RelationshipType, RelationshipManager,
    HasOne, HasMany, BelongsTo, BelongsToMany, LazyLoader, EagerLoader
};
pub use cache::{
    QueryCache, EntityCache, CacheStrategy, CacheStats,
    MemoryCache, RedisCache, CacheInvalidator
};
pub use validation::{
    Validator, ValidationRule, ValidationError, ValidationContext,
    Required, MinLength, MaxLength, EmailFormat, CustomValidator
};
pub use transaction_ops::{
    TransactionalRepository, TransactionScope, UnitOfWork,
    TransactionState, TransactionMetrics
};
pub use schema::{
    SchemaBuilder, TableSchema, ColumnSchema, IndexSchema,
    DatabaseSchema, SchemaComparator, SchemaMigrator
};
pub use mapping::{
    TypeMapper, ColumnMapper, ResultMapper,
    SqlTypeMapping, CustomMapping, MappingRegistry
};

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{instrument, debug, info, warn, error};

use super::{DatabaseError, DatabaseErrorKind, SqlValue, DB};

/// fr fr Main ORM context that coordinates all ORM operations
#[derive(Debug)]
pub struct OrmContext {
    /// Database connection
    pub db: Arc<DB>,
    /// Entity manager for model operations
    pub entity_manager: EntityManager,
    /// Migration manager for schema changes
    pub migration_manager: MigrationManager,
    /// Relationship manager for association handling
    pub relationship_manager: RelationshipManager,
    /// Query cache for performance optimization
    pub query_cache: Arc<Mutex<QueryCache>>,
    /// Type mapper for SQL type conversions
    pub type_mapper: TypeMapper,
    /// Configuration settings
    pub config: OrmConfig,
}

impl OrmContext {
    /// slay Create new ORM context with database connection
    #[instrument(skip(db))]
    pub fn new(db: Arc<DB>, config: OrmConfig) -> Self {
        info!("Creating new ORM context");
        
        let entity_manager = EntityManager::new(db.clone());
        let migration_manager = MigrationManager::new(db.clone());
        let relationship_manager = RelationshipManager::new();
        let query_cache = Arc::new(Mutex::new(QueryCache::new(config.cache_config.clone())));
        let type_mapper = TypeMapper::new();
        
        Self {
            db,
            entity_manager,
            migration_manager,
            relationship_manager,
            query_cache,
            type_mapper,
            config,
        }
    }

    /// facts Get repository for specific entity type
    #[instrument(skip(self))]
    pub fn repository<T: Entity>(&self) -> Repository<T> {
        debug!(entity = T::table_name(), "Creating repository");
        Repository::new(
            self.db.clone(),
            self.query_cache.clone(),
            self.config.clone(),
        )
    }

    /// periodt Execute migrations to update database schema
    #[instrument(skip(self))]
    pub async fn migrate(&self) -> Result<Vec<MigrationStatus>, DatabaseError> {
        info!("Executing database migrations");
        self.migration_manager.migrate().await
    }

    /// bestie Clear all caches
    #[instrument(skip(self))]
    pub fn clear_caches(&self) -> Result<(), DatabaseError> {
        debug!("Clearing all ORM caches");
        if let Ok(mut cache) = self.query_cache.lock() {
            cache.clear();
        }
        self.entity_manager.clear_caches();
        Ok(())
    }

    /// yolo Get ORM statistics and metrics
    #[instrument(skip(self))]
    pub fn stats(&self) -> OrmStats {
        let cache_stats = self.query_cache.lock()
            .map(|cache| cache.stats())
            .unwrap_or_default();
        
        OrmStats {
            cache_stats,
            entity_stats: self.entity_manager.stats(),
            migration_stats: self.migration_manager.stats(),
        }
    }
}

/// fr fr Repository pattern implementation for entity operations
#[derive(Debug)]
pub struct Repository<T: Entity> {
    db: Arc<DB>,
    query_cache: Arc<Mutex<QueryCache>>,
    config: OrmConfig,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Entity> Repository<T> {
    /// slay Create new repository
    pub fn new(
        db: Arc<DB>, 
        query_cache: Arc<Mutex<QueryCache>>, 
        config: OrmConfig
    ) -> Self {
        Self {
            db,
            query_cache,
            config,
            _phantom: std::marker::PhantomData,
        }
    }

    /// facts Find entity by primary key with caching
    #[instrument(skip(self))]
    pub async fn find_by_vibe(&self, id: SqlValue) -> Result<Option<T>, DatabaseError> {
        debug!(entity = T::table_name(), id = ?id, "Finding entity by primary key");
        
        // Check cache first
        let cache_key = format!("{}:{}", T::table_name(), id);
        if let Ok(cache) = self.query_cache.lock() {
            if let Some(cached) = cache.get::<T>(&cache_key) {
                debug!("Found entity in cache");
                return Ok(Some(cached));
            }
        }
        
        // Query database
        let query = self.query()
            .where_clause(&format!("{} = ?", T::primary_key_name()), vec![id.clone()])
            .limit(1);
        
        let results = query.execute().await?;
        let entity = results.into_iter().next();
        
        // Cache result
        if let Some(ref entity) = entity {
            if let Ok(mut cache) = self.query_cache.lock() {
                cache.set(cache_key, entity.clone(), self.config.cache_config.default_ttl);
            }
        }
        
        Ok(entity)
    }

    /// sus Find entities matching criteria
    #[instrument(skip(self))]
    pub async fn find_where_its_at(&self, conditions: &[(&str, SqlValue)]) -> Result<Vec<T>, DatabaseError> {
        debug!(entity = T::table_name(), conditions = ?conditions, "Finding entities with conditions");
        
        let mut query = self.query();
        for (field, value) in conditions {
            query = query.where_clause(&format!("{} = ?", field), vec![value.clone()]);
        }
        
        query.execute().await
    }

    /// periodt Save entity (create or update)
    #[instrument(skip(self, entity))]
    pub async fn save_it(&self, entity: &T) -> Result<T, DatabaseError> {
        info!(entity = T::table_name(), "Saving entity");
        
        // Validate entity
        entity.validate()?;
        
        // Update timestamps if applicable
        let mut entity = entity.clone();
        if let Some(timestamped) = entity.as_timestamped_mut() {
            timestamped.touch_updated_at();
            if entity.primary_key_value().is_none() {
                timestamped.touch_created_at();
            }
        }
        
        let result = if entity.primary_key_value().is_some() {
            self.update_entity(&entity).await?
        } else {
            self.create_entity(&entity).await?
        };
        
        // Invalidate related caches
        self.invalidate_caches(&result).await?;
        
        Ok(result)
    }

    /// lowkey Delete entity
    #[instrument(skip(self, entity))]
    pub async fn delete_sus(&self, entity: &T) -> Result<bool, DatabaseError> {
        info!(entity = T::table_name(), "Deleting entity");
        
        let pk_value = entity.primary_key_value()
            .ok_or_else(|| DatabaseError::validation_error("Entity must have primary key for deletion"))?;
        
        let query = format!(
            "DELETE FROM {} WHERE {} = ?",
            T::table_name(),
            T::primary_key_name()
        );
        
        // Execute deletion
        // Note: Actual database execution would happen here
        
        // Invalidate caches
        self.invalidate_caches(entity).await?;
        
        Ok(true)
    }

    /// highkey Create fluent query builder
    #[instrument(skip(self))]
    pub fn query(&self) -> FluentQueryBuilder<T> {
        debug!(entity = T::table_name(), "Creating query builder");
        FluentQueryBuilder::new(T::table_name(), self.db.clone())
    }

    /// bestie Bulk insert entities with transaction
    #[instrument(skip(self, entities))]
    pub async fn bulk_insert_vibes(&self, entities: &[T]) -> Result<Vec<T>, DatabaseError> {
        info!(entity = T::table_name(), count = entities.len(), "Bulk inserting entities");
        
        if entities.is_empty() {
            return Ok(Vec::new());
        }
        
        // Validate all entities
        for entity in entities {
            entity.validate()?;
        }
        
        // Use transaction for bulk operation
        let tx = self.db.begin_transaction().await?;
        
        let mut results = Vec::new();
        for entity in entities {
            let result = self.create_entity(entity).await?;
            results.push(result);
        }
        
        tx.commit().await?;
        
        // Clear relevant caches
        if let Ok(mut cache) = self.query_cache.lock() {
            cache.invalidate_pattern(&format!("{}:*", T::table_name()));
        }
        
        info!(created = results.len(), "Bulk insert completed");
        Ok(results)
    }

    /// facts Load relationships eagerly
    #[instrument(skip(self, entity))]
    pub async fn with_vibes<R: Entity>(&self, entity: &T, relationship: &str) -> Result<Vec<R>, DatabaseError> {
        debug!(
            entity = T::table_name(),
            relationship = relationship,
            "Loading relationship eagerly"
        );
        
        let relationships = T::relationships();
        let rel_def = relationships.iter()
            .find(|r| r.name() == relationship)
            .ok_or_else(|| DatabaseError::validation_error(&format!("Relationship '{}' not found", relationship)))?;
        
        match rel_def.relationship_type() {
            RelationshipType::HasMany { foreign_key } => {
                let pk_value = entity.primary_key_value()
                    .ok_or_else(|| DatabaseError::validation_error("Entity must have primary key for relationship loading"))?;
                
                let query = format!(
                    "SELECT * FROM {} WHERE {} = ?",
                    R::table_name(),
                    foreign_key
                );
                
                // Execute query and map results
                // Note: Actual database execution would happen here
                Ok(Vec::new()) // Placeholder
            }
            _ => {
                warn!("Relationship type not yet implemented for eager loading");
                Ok(Vec::new())
            }
        }
    }

    // Helper methods
    async fn create_entity(&self, entity: &T) -> Result<T, DatabaseError> {
        // Implementation for creating new entity
        Ok(entity.clone()) // Placeholder
    }
    
    async fn update_entity(&self, entity: &T) -> Result<T, DatabaseError> {
        // Implementation for updating existing entity
        Ok(entity.clone()) // Placeholder
    }
    
    async fn invalidate_caches(&self, entity: &T) -> Result<(), DatabaseError> {
        if let Ok(mut cache) = self.query_cache.lock() {
            let pk_value = entity.primary_key_value();
            if let Some(pk) = pk_value {
                let cache_key = format!("{}:{}", T::table_name(), pk);
                cache.remove(&cache_key);
            }
            
            // Invalidate query caches for this table
            cache.invalidate_pattern(&format!("query:{}:*", T::table_name()));
        }
        Ok(())
    }
}

/// fr fr ORM configuration settings
#[derive(Debug, Clone)]
pub struct OrmConfig {
    /// Cache configuration
    pub cache_config: CacheConfig,
    /// Query timeout settings
    pub query_timeout: std::time::Duration,
    /// Enable query logging
    pub enable_query_logging: bool,
    /// Connection pool settings
    pub pool_config: PoolConfig,
    /// Migration settings
    pub migration_config: MigrationConfig,
}

impl Default for OrmConfig {
    fn default() -> Self {
        Self {
            cache_config: CacheConfig::default(),
            query_timeout: std::time::Duration::from_secs(30),
            enable_query_logging: false,
            pool_config: PoolConfig::default(),
            migration_config: MigrationConfig::default(),
        }
    }
}

/// fr fr Cache configuration for ORM operations
#[derive(Debug, Clone)]
pub struct CacheConfig {
    /// Maximum cache size
    pub max_size: usize,
    /// Default time-to-live for cached entries
    pub default_ttl: std::time::Duration,
    /// Enable query result caching
    pub enable_query_cache: bool,
    /// Enable entity caching
    pub enable_entity_cache: bool,
}

impl Default for CacheConfig {
    fn default() -> Self {
        Self {
            max_size: 10000,
            default_ttl: std::time::Duration::from_secs(3600), // 1 hour
            enable_query_cache: true,
            enable_entity_cache: true,
        }
    }
}

/// fr fr Pool configuration placeholder
#[derive(Debug, Clone, Default)]
pub struct PoolConfig {
    pub max_connections: usize,
    pub min_connections: usize,
}

/// fr fr Migration configuration placeholder  
#[derive(Debug, Clone, Default)]
pub struct MigrationConfig {
    pub migrations_dir: String,
    pub auto_migrate: bool,
}

/// fr fr ORM statistics and metrics
#[derive(Debug, Clone, Default)]
pub struct OrmStats {
    pub cache_stats: CacheStats,
    pub entity_stats: EntityStats,
    pub migration_stats: MigrationStats,
}

/// fr fr Entity operation statistics placeholder
#[derive(Debug, Clone, Default)]
pub struct EntityStats {
    pub total_queries: u64,
    pub cache_hits: u64,
    pub cache_misses: u64,
}

/// fr fr Migration operation statistics placeholder
#[derive(Debug, Clone, Default)]
pub struct MigrationStats {
    pub pending_migrations: usize,
    pub applied_migrations: usize,
}
