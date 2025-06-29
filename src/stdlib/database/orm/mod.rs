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
    PrimaryKey, ForeignKey, Timestamped
// };
pub use query_builder::{
    WhereClause, JoinClause, OrderByClause, GroupByClause
// };
pub use migration::{
    CreateTable, DropTable, AddColumn, DropColumn, AddIndex
// };
pub use relationships::{
    HasOne, HasMany, BelongsTo, BelongsToMany, LazyLoader, EagerLoader
// };
pub use cache::{
    MemoryCache, RedisCache, CacheInvalidator
// };
pub use validation::{
    Required, MinLength, MaxLength, EmailFormat, CustomValidator
// };
pub use transaction_ops::{
    TransactionState, TransactionMetrics
// };
pub use schema::{
    DatabaseSchema, SchemaComparator, SchemaMigrator
// };
pub use mapping::{
    SqlTypeMapping, CustomMapping, MappingRegistry
// };

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{instrument, debug, info, warn, error};

use super::{DatabaseError, DatabaseErrorKind, SqlValue, DB};
use cache::CacheConfig;
use crate::error::CursedError;

/// fr fr Main ORM context that coordinates all ORM operations
#[derive(Debug)]
pub struct OrmContext {
    /// Database connection
    /// Entity manager for model operations
    /// Migration manager for schema changes
    /// Relationship manager for association handling
    /// Query cache for performance optimization
    /// Type mapper for SQL type conversions
    /// Configuration settings
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
        }
    }

    /// facts Get repository for specific entity type
    #[instrument(skip(self))]
    pub fn repository<T: Entity>(&self) -> Repository<T> {
        debug!(entity = T::table_name(), "Creating repository");
        Repository::new(
        )
    /// periodt Execute migrations to update database schema
    #[instrument(skip(self))]
    pub async fn migrate(&self) -> crate::error::Result<()> {
        info!("Executing database migrations");
        self.migration_manager.migrate().await
    /// bestie Clear all caches
    #[instrument(skip(self))]
    pub fn clear_caches(&self) -> crate::error::Result<()> {
        debug!("Clearing all ORM caches");
        if let Ok(mut cache) = self.query_cache.lock() {
            cache.clear();
        }
        self.entity_manager.clear_caches();
        Ok(())
    /// yolo Get ORM statistics and metrics
    #[instrument(skip(self))]
    pub fn stats(&self) -> OrmStats {
        let cache_stats = self.query_cache.lock()
            .map(|cache| cache.stats())
            .unwrap_or_default();
        
        OrmStats {
        }
    }
/// fr fr Repository pattern implementation for entity operations
#[derive(Debug)]
pub struct Repository<T: Entity> {
impl<T: Entity> Repository<T> {
    /// slay Create new repository
    pub fn new(
        config: OrmConfig
    ) -> Self {
        Self {
        }
    }

    /// facts Find entity by primary key with caching
    #[instrument(skip(self))]
    pub async fn find_by_vibe(&self, id: SqlValue) -> crate::error::Result<()> {
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
            .where_clause(&format!("{} = ?", T::primary_key_name()), Vec::from([id.clone()]))
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
    /// sus Find entities matching criteria
    #[instrument(skip(self))]
    pub async fn find_where_its_at(&self, conditions: &[(&str, SqlValue)]) -> crate::error::Result<()> {
        debug!(entity = T::table_name(), conditions = ?conditions, "Finding entities with conditions");
        
        let mut query = self.query();
        for (field, value) in conditions {
            query = query.where_clause(&format!("{} = ?", field), Vec::from([value.clone()]));
        query.execute().await
    /// periodt Save entity (create or update)
    #[instrument(skip(self, entity))]
    pub async fn save_it(&self, entity: &T) -> crate::error::Result<()> {
        info!(entity = T::table_name(), "Saving entity");
        
        // Validate entity
        entity.validate()?;
        
        // Update timestamps if applicable
        let mut entity = entity.clone();
        let needs_created_at = entity.primary_key_value().is_none();
        if let Some(timestamped) = entity.as_timestamped_mut() {
            timestamped.touch_updated_at();
            if needs_created_at {
                timestamped.touch_created_at();
            }
        }
        
        let result = if entity.primary_key_value().is_some() {
            self.update_entity(&entity).await?
        } else {
            self.create_entity(&entity).await?
        
        // Invalidate related caches
        self.invalidate_caches(&result).await?;
        
        Ok(result)
    /// lowkey Delete entity
    #[instrument(skip(self, entity))]
    pub async fn delete_sus(&self, entity: &T) -> crate::error::Result<()> {
        info!(entity = T::table_name(), "Deleting entity");
        
        let pk_value = entity.primary_key_value()
            .ok_or_else(|| DatabaseError::validation_error("Entity must have primary key for deletion"))?;
        
        let sql = format!(
            T::primary_key_name()
        );
        
        debug!(sql = %sql, pk_value = ?pk_value, "Executing DELETE");
        
        // Execute deletion with real database execution
        let result = self.db.exec(sql, Vec::from([pk_value]))?;
        
        let rows_affected = result.rows_affected()?;
        let deleted = rows_affected > 0;
        
        if deleted {
            info!(rows_affected = rows_affected, "Entity deleted successfully");
            // Invalidate caches
            self.invalidate_caches(entity).await?;
        } else {
            warn!("No entity was deleted - entity may not exist");
        Ok(deleted)
    /// highkey Create fluent query builder
    #[instrument(skip(self))]
    pub fn query(&self) -> FluentQueryBuilder<T> {
        debug!(entity = T::table_name(), "Creating query builder");
        FluentQueryBuilder::new(T::table_name(), self.db.clone())
    /// slay Get database connection
    pub fn db(&self) -> &Arc<DB> {
        &self.db
    /// bestie Bulk insert entities with transaction
    #[instrument(skip(self, entities))]
    pub async fn bulk_insert_vibes(&self, entities: &[T]) -> crate::error::Result<()> {
        info!(entity = T::table_name(), count = entities.len(), "Bulk inserting entities");
        
        if entities.is_empty() {
            return Ok(Vec::new());
        // Validate all entities
        for entity in entities {
            entity.validate()?;
        // Use transaction for bulk operation  
        let ctx = super::VibeContext::default();
        let mut tx = self.db.begin_tx(ctx, None)?;
        
        let mut results = Vec::new();
        
        // Get field structure from first entity
        let first_fields = entities[0].to_fields();
        let field_names: Vec<String> = first_fields.keys().cloned().collect();
        
        // Build batch INSERT statement
        let placeholders_per_row: Vec<String> = (1..=field_names.len())
            .map(|i| format!("${}", i))
            .collect();
        
        for (entity_index, entity) in entities.iter().enumerate() {
            let fields = entity.to_fields();
            let field_values: Vec<SqlValue> = field_names.iter()
                .map(|name| fields.get(name).cloned().unwrap_or(SqlValue::Null))
                .collect();
            
            // Build INSERT for this entity
            let sql = format!(
                placeholders_per_row.join(", ")
            );
            
            debug!(
                "Executing batch INSERT"
            );
            
            // Execute insert within transaction
            let result = tx.exec(sql, field_values)?;
            
            // Get the inserted ID if available
            let mut created_entity = entity.clone();
            if let Ok(insert_id) = result.last_insert_id() {
                created_entity.set_primary_key_value(SqlValue::Integer(insert_id));
            results.push(created_entity);
        // Commit transaction
        tx.commit()?;
        
        // Clear relevant caches
        if let Ok(mut cache) = self.query_cache.lock() {
            cache.invalidate_pattern(&format!("{}:*", T::table_name()));
        info!(created = results.len(), "Bulk insert completed");
        Ok(results)
    /// facts Load relationships eagerly
    #[instrument(skip(self, entity))]
    pub async fn with_vibes<R: Entity>(&self, entity: &T, relationship: &str) -> crate::error::Result<()> {
        debug!(
            "Loading relationship eagerly"
        );
        
        let relationships = T::relationships();
        let rel_def = relationships.iter()
            .find(|r| r.name() == relationship)
            .ok_or_else(|| DatabaseError::validation_error(&format!("Relationship '{}' not found", relationship)))?;
        
        match rel_def.relationship_type() {
            RelationshipType::HasMany { foreign_key, local_key: _ } => {
                let pk_value = entity.primary_key_value()
                    .ok_or_else(|| DatabaseError::validation_error("Entity must have primary key for relationship loading"))?;
                
                let query = format!(
                    foreign_key
                );
                
                // Execute query and map results
                let sql = format!("SELECT * FROM {} WHERE {} = $1", T::table_name(), foreign_key);
                let results = self.db.query(sql, vec![pk_value])?;
                
                // Convert results to entities
                let related_entities: Vec<T> = results.into_iter()
                    .filter_map(|row| T::from_row(&row).ok())
                    .collect();
                
                Ok(related_entities)
            }
            _ => {
                warn!("Relationship type not yet implemented for eager loading");
                Ok(Vec::new())
            }
        }
    // Helper methods
    async fn create_entity(&self, entity: &T) -> crate::error::Result<()> {
        debug!(entity = T::table_name(), "Creating new entity");
        
        let fields = entity.to_fields();
        let field_names: Vec<String> = fields.keys().cloned().collect();
        let field_values: Vec<SqlValue> = fields.values().cloned().collect();
        
        // Build INSERT statement
        let placeholders: Vec<String> = (1..=field_names.len())
            .map(|i| format!("${}", i))
            .collect();
        
        let sql = format!(
            placeholders.join(", ")
        );
        
        debug!(sql = %sql, values = ?field_values, "Executing INSERT");
        
        // Execute insert
        let result = self.db.exec(sql, field_values)?;
        
        // Get the inserted ID if available
        let mut created_entity = entity.clone();
        if let Ok(insert_id) = result.last_insert_id() {
            created_entity.set_primary_key_value(SqlValue::Integer(insert_id));
        info!("Entity created successfully");
        Ok(created_entity)
    async fn update_entity(&self, entity: &T) -> crate::error::Result<()> {
        debug!(entity = T::table_name(), "Updating existing entity");
        
        let pk_value = entity.primary_key_value()
            .ok_or_else(|| CursedError::Runtime("Entity must have primary key for update".to_string()))?;
        
        let fields = entity.to_fields();
        let mut field_assignments = Vec::new();
        let mut field_values = Vec::new();
        let mut param_index = 1;
        
        // Build SET clause
        for (field_name, field_value) in fields.iter() {
            if field_name != T::primary_key_name() {  // Skip primary key
                field_assignments.push(format!("{} = ${}", field_name, param_index));
                field_values.push(field_value.clone());
                param_index += 1;
            }
        }
        
        // Add primary key value for WHERE clause
        field_values.push(pk_value);
        
        let sql = format!(
            param_index
        );
        
        debug!(sql = %sql, values = ?field_values, "Executing UPDATE");
        
        // Execute update
        let result = self.db.exec(sql, field_values)?;
        
        if result.rows_affected()? == 0 {
            return Err(DatabaseError::not_found("No rows were updated").into());
        info!("Entity updated successfully");
        Ok(entity.clone())
    async fn invalidate_caches(&self, entity: &T) -> crate::error::Result<()> {
        if let Ok(mut cache) = self.query_cache.lock() {
            let pk_value = entity.primary_key_value();
            if let Some(pk) = pk_value {
                let cache_key = format!("{}:{}", T::table_name(), pk);
                cache.remove(&cache_key);
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
    /// Query timeout settings
    /// Enable query logging
    /// Connection pool settings
    /// Migration settings
impl Default for OrmConfig {
    fn default() -> Self {
        Self {
        }
    }


/// fr fr Pool configuration placeholder
#[derive(Debug, Clone, Default)]
pub struct PoolConfig {
/// fr fr Migration configuration placeholder  
#[derive(Debug, Clone, Default)]
pub struct MigrationConfig {
/// fr fr ORM statistics and metrics
#[derive(Debug, Clone, Default)]
pub struct OrmStats {
/// fr fr Entity operation statistics placeholder
#[derive(Debug, Clone, Default)]
pub struct EntityStats {
/// fr fr Migration operation statistics placeholder
#[derive(Debug, Clone, Default)]
pub struct MigrationStats {
}
