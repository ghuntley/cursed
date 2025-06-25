/// Relationship mapping system for CURSED ORM
/// 
/// Provides relationship definitions, lazy/eager loading strategies,
/// and association management with foreign key handling.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{instrument, debug, info, warn, error};

use super::super::{DatabaseError, DatabaseErrorKind, SqlValue, DB};
use super::entity::Entity;
use super::query_builder::FluentQueryBuilder;

/// fr fr Relationship definition between entities
#[derive(Debug, Clone)]
pub struct Relationship {
    /// Relationship name
    /// Type of relationship
    /// Source entity
    /// Target entity
    /// Loading strategy
impl Relationship {
    /// slay Create new relationship
    pub fn new(
    ) -> Self {
        Self {
        }
    }

    /// facts Get relationship name
    pub fn name(&self) -> &str {
        &self.name
    /// periodt Get relationship type
    pub fn relationship_type(&self) -> &RelationshipType {
        &self.relationship_type
    /// bestie Set loading strategy
    pub fn with_loading_strategy(mut self, strategy: LoadingStrategy) -> Self {
        self.loading_strategy = strategy;
        self
    }
}

/// fr fr Types of relationships between entities
#[derive(Debug, Clone)]
pub enum RelationshipType {
    /// One-to-one relationship
    HasOne {
    /// One-to-many relationship
    HasMany {
    /// Many-to-one relationship (inverse of HasMany)
    BelongsTo {
    /// Many-to-many relationship with pivot table
    BelongsToMany {
    /// Polymorphic relationship
    MorphTo {
    /// Inverse polymorphic relationship
    MorphMany {
/// fr fr Loading strategies for relationships
#[derive(Debug, Clone, PartialEq)]
pub enum LoadingStrategy {
    /// Load when explicitly requested
    /// Load automatically with parent entity
    /// Load via separate query to avoid N+1
/// fr fr Relationship manager for handling associations
#[derive(Debug)]
pub struct RelationshipManager {
    /// Registered relationships
    /// Lazy loader for on-demand loading
    /// Eager loader for batch loading
impl RelationshipManager {
    /// slay Create new relationship manager
    #[instrument]
    pub fn new() -> Self {
        info!("Creating new relationship manager");
        Self {
        }
    }

    /// facts Register relationship for entity
    #[instrument(skip(self))]
    pub fn register_relationship(&self, entity_name: &str, relationship: Relationship) -> crate::error::Result<()> {
        debug!(entity = entity_name, relationship = %relationship.name, "Registering relationship");
        
        if let Ok(mut relationships) = self.relationships.lock() {
            let entity_relationships = relationships.entry(entity_name.to_string()).or_insert_with(Vec::new);
            entity_relationships.push(relationship);
            
            debug!("Relationship registered successfully");
            Ok(())
        } else {
            Err(DatabaseError::internal_error("Failed to access relationships registry"))
        }
    }

    /// periodt Get relationships for entity
    #[instrument(skip(self))]
    pub fn get_relationships(&self, entity_name: &str) -> Vec<Relationship> {
        if let Ok(relationships) = self.relationships.lock() {
            relationships.get(entity_name).cloned().unwrap_or_default()
        } else {
            Vec::new()
        }
    }

    /// bestie Load relationship using appropriate strategy
    #[instrument(skip(self, entity, db))]
    pub async fn load_relationship<T: Entity, R: Entity>(
    ) -> crate::error::Result<()> {
        debug!(
            "Loading relationship"
        );

        let relationships = self.get_relationships(T::table_name());
        let relationship = relationships
            .iter()
            .find(|r| r.name == relationship_name)
            .ok_or_else(|| DatabaseError::validation_error(&format!("Relationship '{}' not found", relationship_name)))?;

        match relationship.loading_strategy {
            LoadingStrategy::Lazy => {
                self.lazy_loader.load_relationship(entity, relationship, db).await
            }
            LoadingStrategy::Eager | LoadingStrategy::EagerBatch => {
                self.eager_loader.load_relationship(entity, relationship, db).await
            }
        }
    /// yolo Load multiple relationships for entity
    #[instrument(skip(self, entity, db))]
    pub async fn load_relationships<T: Entity>(
    ) -> crate::error::Result<()> {
        debug!(
            "Loading multiple relationships"
        );

        let mut results = HashMap::new();

        for relationship_name in relationship_names {
            let relationships = self.get_relationships(T::table_name());
            let relationship = relationships
                .iter()
                .find(|r| r.name == *relationship_name)
                .ok_or_else(|| DatabaseError::validation_error(&format!("Relationship '{}' not found", relationship_name)))?;

            // Load as raw rows for now (would need generic handling for different entity types)
            let rows = self.load_relationship_rows(entity, relationship, db.clone()).await?;
            results.insert(relationship_name.to_string(), rows);
        info!(loaded = results.len(), "Multiple relationships loaded");
        Ok(results)
    /// slay Load relationship as raw rows
    async fn load_relationship_rows<T: Entity>(
    ) -> crate::error::Result<()> {
        match &relationship.relationship_type {
            RelationshipType::HasMany { foreign_key, local_key } => {
                let local_value = entity.primary_key_value()
                    .ok_or_else(|| DatabaseError::validation_error("Entity must have primary key for relationship loading"))?;

                // Build query for related records
                let query = format!(
                    foreign_key
                );

                debug!(query = %query, "Executing HasMany relationship query");
                
                // Simulate query execution
                let mut row = HashMap::new();
                row.insert("id".to_string(), SqlValue::Integer(1));
                row.insert(foreign_key.clone(), local_value);
                row.insert("name".to_string(), SqlValue::String("Related Record".to_string()));
                
                Ok(Vec::from([row]))
            }
            RelationshipType::BelongsTo { foreign_key, owner_key } => {
                let foreign_value = entity.to_fields().get(foreign_key).cloned()
                    .ok_or_else(|| DatabaseError::validation_error(&format!("Foreign key '{}' not found in entity", foreign_key)))?;

                let query = format!(
                    owner_key
                );

                debug!(query = %query, "Executing BelongsTo relationship query");
                
                // Simulate query execution
                let mut row = HashMap::new();
                row.insert("id".to_string(), foreign_value);
                row.insert("name".to_string(), SqlValue::String("Parent Record".to_string()));
                
                Ok(Vec::from([row]))
            }
            RelationshipType::BelongsToMany { 
                related_key 
            } => {
                let local_value = entity.primary_key_value()
                    .ok_or_else(|| DatabaseError::validation_error("Entity must have primary key for many-to-many relationship"))?;

                let query = format!(
                    foreign_pivot_key
                );

                debug!(query = %query, "Executing BelongsToMany relationship query");
                
                // Simulate query execution
                let mut row = HashMap::new();
                row.insert("id".to_string(), SqlValue::Integer(1));
                row.insert("name".to_string(), SqlValue::String("Many-to-Many Record".to_string()));
                
                Ok(Vec::from([row]))
            }
            _ => {
                warn!("Relationship type not yet implemented: {:?}", relationship.relationship_type);
                Ok(Vec::new())
            }
        }
    }
}

/// fr fr Lazy loader for on-demand relationship loading
#[derive(Debug)]
pub struct LazyLoader {
    /// Cache for loaded relationships
impl LazyLoader {
    /// slay Create new lazy loader
    pub fn new() -> Self {
        Self {
        }
    }

    /// facts Load relationship lazily
    #[instrument(skip(self, entity, relationship, db))]
    pub async fn load_relationship<T: Entity, R: Entity>(
    ) -> crate::error::Result<()> {
        debug!(
            "Loading relationship lazily"
        );

        // Check cache first
            entity.primary_key_value()
        );

        if let Ok(cache) = self.cache.lock() {
            if let Some(cached_rows) = cache.get(&cache_key) {
                debug!("Found relationship in lazy cache");
                
                let mut results = Vec::new();
                for row in cached_rows {
                    let entity = R::from_row(row)?;
                    results.push(entity);
                }
                return Ok(results);
            }
        }

        // Load from database
        let rows = self.execute_relationship_query(entity, relationship, db).await?;
        
        // Cache the results
        if let Ok(mut cache) = self.cache.lock() {
            cache.insert(cache_key, rows.clone());
        // Convert to entities
        let mut results = Vec::new();
        for row in rows {
            let entity = R::from_row(&row)?;
            results.push(entity);
        debug!(count = results.len(), "Relationship loaded lazily");
        Ok(results)
    /// periodt Execute relationship query
    async fn execute_relationship_query<T: Entity>(
    ) -> crate::error::Result<()> {
        debug!(
            "Executing relationship query"
        );
        
        match &relationship.relationship_type {
            RelationshipType::HasMany { foreign_key, .. } => {
                let local_value = entity.primary_key_value()
                    .ok_or_else(|| DatabaseError::validation_error("Entity must have primary key"))?;

                let sql = format!(
                    foreign_key
                );
                
                debug!(sql = %sql, foreign_key = foreign_key, local_value = ?local_value, "Executing HasMany query");
                
                let rows = db.map_query(sql, Vec::from([local_value]))?;
                Ok(rows)
            }
            RelationshipType::BelongsTo { foreign_key, owner_key } => {
                let foreign_value = entity.to_fields().get(foreign_key).cloned()
                    .ok_or_else(|| DatabaseError::validation_error(&format!("Foreign key '{}' not found in entity", foreign_key)))?;

                let sql = format!(
                    owner_key
                );
                
                debug!(sql = %sql, owner_key = owner_key, foreign_value = ?foreign_value, "Executing BelongsTo query");
                
                let rows = db.map_query(sql, Vec::from([foreign_value]))?;
                Ok(rows)
            }
            _ => {
                warn!("Relationship type not yet implemented for lazy loading: {:?}", relationship.relationship_type);
                Ok(Vec::new())
            }
        }
    /// bestie Clear lazy loading cache
    #[instrument(skip(self))]
    pub fn clear_cache(&self) {
        debug!("Clearing lazy loader cache");
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
        }
    }
/// fr fr Eager loader for batch relationship loading
#[derive(Debug)]
pub struct EagerLoader {
    /// Batch loading configuration
impl EagerLoader {
    /// slay Create new eager loader
    pub fn new() -> Self {
        Self {
        }
    }

    /// facts Load relationship eagerly
    #[instrument(skip(self, entity, relationship, db))]
    pub async fn load_relationship<T: Entity, R: Entity>(
    ) -> crate::error::Result<()> {
        debug!(
            "Loading relationship eagerly"
        );

        // Load relationship data
        let rows = self.batch_load_relationship(entity, relationship, db).await?;
        
        // Convert to entities
        let mut results = Vec::new();
        for row in rows {
            let entity = R::from_row(&row)?;
            results.push(entity);
        debug!(count = results.len(), "Relationship loaded eagerly");
        Ok(results)
    /// periodt Load relationships for multiple entities in batch
    #[instrument(skip(self, entities, relationship, db))]
    pub async fn batch_load_for_entities<T: Entity, R: Entity>(
    ) -> crate::error::Result<()> {
        info!(
            "Batch loading relationship for multiple entities"
        );

        let mut results = HashMap::new();

        // Group entities into batches
        for chunk in entities.chunks(self.batch_size) {
            let primary_keys: Vec<SqlValue> = chunk
                .iter()
                .filter_map(|e| e.primary_key_value())
                .collect();

            if primary_keys.is_empty() {
                continue;
            let batch_results = self.execute_batch_query(relationship, &primary_keys, db.clone()).await?;
            
            // Group results by foreign key
            for row in batch_results {
                let foreign_key_value = match &relationship.relationship_type {
                    RelationshipType::HasMany { foreign_key, .. } => {
                        row.get(foreign_key).cloned()
                    }

                if let Some(fk_value) = foreign_key_value {
                    let entity = R::from_row(&row)?;
                    results.entry(fk_value).or_insert_with(Vec::new).push(entity);
                }
            }
        info!(batches_loaded = results.len(), "Batch loading completed");
        Ok(results)
    /// bestie Execute batch query for relationship
    async fn batch_load_relationship<T: Entity>(
    ) -> crate::error::Result<()> {
        debug!(
            "Batch loading relationship for single entity"
        );
        
        match &relationship.relationship_type {
            RelationshipType::HasMany { foreign_key, .. } => {
                let local_value = entity.primary_key_value()
                    .ok_or_else(|| DatabaseError::validation_error("Entity must have primary key"))?;

                let sql = format!(
                    foreign_key
                );
                
                debug!(sql = %sql, foreign_key = foreign_key, local_value = ?local_value, "Executing eager HasMany query");
                
                let rows = db.map_query(sql, Vec::from([local_value]))?;
                Ok(rows)
            }
            RelationshipType::BelongsTo { foreign_key, owner_key } => {
                let foreign_value = entity.to_fields().get(foreign_key).cloned()
                    .ok_or_else(|| DatabaseError::validation_error(&format!("Foreign key '{}' not found in entity", foreign_key)))?;

                let sql = format!(
                    owner_key
                );
                
                debug!(sql = %sql, owner_key = owner_key, foreign_value = ?foreign_value, "Executing eager BelongsTo query");
                
                let rows = db.map_query(sql, Vec::from([foreign_value]))?;
                Ok(rows)
            }
            _ => {
                warn!("Relationship type not yet implemented for eager loading: {:?}", relationship.relationship_type);
                Ok(Vec::new())
            }
        }
    /// yolo Execute batch query for multiple primary keys
    async fn execute_batch_query(
    ) -> crate::error::Result<()> {
        debug!(
            "Executing batch query"
        );

        if primary_keys.len() == 0 {
            return Ok(Vec::new());
        match &relationship.relationship_type {
            RelationshipType::HasMany { foreign_key, .. } => {
                // Build IN clause for efficient batch loading
                let placeholders: Vec<String> = (1..=primary_keys.len())
                    .map(|i| format!("${}", i))
                    .collect();
                
                let sql = format!(
                    placeholders.join(", ")
                );
                
                debug!(sql = %sql, key_count = primary_keys.len(), "Executing batch HasMany query");
                
                let rows = db.map_query(sql, primary_keys.to_vec())?;
                Ok(rows)
            }
            RelationshipType::BelongsTo { foreign_key, owner_key } => {
                // For BelongsTo, we need to get foreign key values first, then batch load
                // This is more complex and would typically require a different approach
                let placeholders: Vec<String> = (1..=primary_keys.len())
                    .map(|i| format!("${}", i))
                    .collect();
                
                let sql = format!(
                    placeholders.join(", ")
                );
                
                debug!(sql = %sql, key_count = primary_keys.len(), "Executing batch BelongsTo query");
                
                let rows = db.map_query(sql, primary_keys.to_vec())?;
                Ok(rows)
            }
            _ => {
                warn!("Batch query not implemented for relationship type: {:?}", relationship.relationship_type);
                Ok(Vec::new())
            }
        }
    }
}

/// fr fr Relationship builder for fluent relationship definition
pub struct RelationshipBuilder {
impl RelationshipBuilder {
    /// slay Create new relationship builder
    pub fn new(name: &str, source_entity: &str, target_entity: &str) -> Self {
        Self {
        }
    }

    /// facts Build HasOne relationship
    pub fn has_one(self, foreign_key: &str) -> Relationship {
        Relationship::new(
            RelationshipType::HasOne {
        )
    /// periodt Build HasMany relationship
    pub fn has_many(self, foreign_key: &str) -> Relationship {
        Relationship::new(
            RelationshipType::HasMany {
        )
    /// bestie Build BelongsTo relationship
    pub fn belongs_to(self, foreign_key: &str) -> Relationship {
        Relationship::new(
            RelationshipType::BelongsTo {
        )
    /// yolo Build BelongsToMany relationship
    pub fn belongs_to_many(
    ) -> Relationship {
        Relationship::new(
            RelationshipType::BelongsToMany {
        )
    }
}

// Convenience trait implementations
pub trait HasOne<T: Entity>: Entity {
    fn has_one_relationship() -> Relationship;
pub trait HasMany<T: Entity>: Entity {
    fn has_many_relationship() -> Relationship;
pub trait BelongsTo<T: Entity>: Entity {
    fn belongs_to_relationship() -> Relationship;
pub trait BelongsToMany<T: Entity>: Entity {
    fn belongs_to_many_relationship() -> Relationship;
