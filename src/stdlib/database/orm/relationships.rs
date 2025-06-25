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
    pub name: String,
    /// Type of relationship
    pub relationship_type: RelationshipType,
    /// Source entity
    pub source_entity: String,
    /// Target entity
    pub target_entity: String,
    /// Loading strategy
    pub loading_strategy: LoadingStrategy,
}

impl Relationship {
    /// slay Create new relationship
    pub fn new(
        name: String,
        relationship_type: RelationshipType,
        source_entity: String,
        target_entity: String,
    ) -> Self {
        Self {
            name,
            relationship_type,
            source_entity,
            target_entity,
            loading_strategy: LoadingStrategy::Lazy,
        }
    }

    /// facts Get relationship name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// periodt Get relationship type
    pub fn relationship_type(&self) -> &RelationshipType {
        &self.relationship_type
    }

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
        foreign_key: String,
        local_key: String,
    },
    /// One-to-many relationship
    HasMany {
        foreign_key: String,
        local_key: String,
    },
    /// Many-to-one relationship (inverse of HasMany)
    BelongsTo {
        foreign_key: String,
        owner_key: String,
    },
    /// Many-to-many relationship with pivot table
    BelongsToMany {
        pivot_table: String,
        foreign_pivot_key: String,
        related_pivot_key: String,
        local_key: String,
        related_key: String,
    },
    /// Polymorphic relationship
    MorphTo {
        foreign_key: String,
        type_key: String,
    },
    /// Inverse polymorphic relationship
    MorphMany {
        foreign_key: String,
        type_key: String,
        type_value: String,
    },
}

/// fr fr Loading strategies for relationships
#[derive(Debug, Clone, PartialEq)]
pub enum LoadingStrategy {
    /// Load when explicitly requested
    Lazy,
    /// Load automatically with parent entity
    Eager,
    /// Load via separate query to avoid N+1
    EagerBatch,
}

/// fr fr Relationship manager for handling associations
#[derive(Debug)]
pub struct RelationshipManager {
    /// Registered relationships
    relationships: Arc<Mutex<HashMap<String, Vec<Relationship>>>>,
    /// Lazy loader for on-demand loading
    lazy_loader: Arc<LazyLoader>,
    /// Eager loader for batch loading
    eager_loader: Arc<EagerLoader>,
}

impl RelationshipManager {
    /// slay Create new relationship manager
    #[instrument]
    pub fn new() -> Self {
        info!("Creating new relationship manager");
        Self {
            relationships: Arc::new(Mutex::new(HashMap::new())),
            lazy_loader: Arc::new(LazyLoader::new()),
            eager_loader: Arc::new(EagerLoader::new()),
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
        &self,
        entity: &T,
        relationship_name: &str,
        db: Arc<DB>,
    ) -> crate::error::Result<()> {
        debug!(
            entity = T::table_name(),
            relationship = relationship_name,
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
    }

    /// yolo Load multiple relationships for entity
    #[instrument(skip(self, entity, db))]
    pub async fn load_relationships<T: Entity>(
        &self,
        entity: &T,
        relationship_names: &[&str],
        db: Arc<DB>,
    ) -> crate::error::Result<()> {
        debug!(
            entity = T::table_name(),
            relationships = ?relationship_names,
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
        }

        info!(loaded = results.len(), "Multiple relationships loaded");
        Ok(results)
    }

    /// slay Load relationship as raw rows
    async fn load_relationship_rows<T: Entity>(
        &self,
        entity: &T,
        relationship: &Relationship,
        db: Arc<DB>,
    ) -> crate::error::Result<()> {
        match &relationship.relationship_type {
            RelationshipType::HasMany { foreign_key, local_key } => {
                let local_value = entity.primary_key_value()
                    .ok_or_else(|| DatabaseError::validation_error("Entity must have primary key for relationship loading"))?;

                // Build query for related records
                let query = format!(
                    "SELECT * FROM {} WHERE {} = ?",
                    relationship.target_entity,
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
                    "SELECT * FROM {} WHERE {} = ?",
                    relationship.target_entity,
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
                pivot_table, 
                foreign_pivot_key, 
                related_pivot_key, 
                local_key, 
                related_key 
            } => {
                let local_value = entity.primary_key_value()
                    .ok_or_else(|| DatabaseError::validation_error("Entity must have primary key for many-to-many relationship"))?;

                let query = format!(
                    "SELECT t.* FROM {} t INNER JOIN {} p ON t.{} = p.{} WHERE p.{} = ?",
                    relationship.target_entity,
                    pivot_table,
                    related_key,
                    related_pivot_key,
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
    cache: Arc<Mutex<HashMap<String, Vec<HashMap<String, SqlValue>>>>>,
}

impl LazyLoader {
    /// slay Create new lazy loader
    pub fn new() -> Self {
        Self {
            cache: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// facts Load relationship lazily
    #[instrument(skip(self, entity, relationship, db))]
    pub async fn load_relationship<T: Entity, R: Entity>(
        &self,
        entity: &T,
        relationship: &Relationship,
        db: Arc<DB>,
    ) -> crate::error::Result<()> {
        debug!(
            entity = T::table_name(),
            relationship = %relationship.name,
            "Loading relationship lazily"
        );

        // Check cache first
        let cache_key = format!("{}:{}:{:?}", 
            T::table_name(), 
            relationship.name, 
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
        }

        // Convert to entities
        let mut results = Vec::new();
        for row in rows {
            let entity = R::from_row(&row)?;
            results.push(entity);
        }

        debug!(count = results.len(), "Relationship loaded lazily");
        Ok(results)
    }

    /// periodt Execute relationship query
    async fn execute_relationship_query<T: Entity>(
        &self,
        entity: &T,
        relationship: &Relationship,
        db: Arc<DB>,
    ) -> crate::error::Result<()> {
        debug!(
            relationship_type = ?relationship.relationship_type,
            "Executing relationship query"
        );
        
        match &relationship.relationship_type {
            RelationshipType::HasMany { foreign_key, .. } => {
                let local_value = entity.primary_key_value()
                    .ok_or_else(|| DatabaseError::validation_error("Entity must have primary key"))?;

                let sql = format!(
                    "SELECT * FROM {} WHERE {} = $1",
                    relationship.target_entity,
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
                    "SELECT * FROM {} WHERE {} = $1",
                    relationship.target_entity,
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
    }

    /// bestie Clear lazy loading cache
    #[instrument(skip(self))]
    pub fn clear_cache(&self) {
        debug!("Clearing lazy loader cache");
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
        }
    }
}

/// fr fr Eager loader for batch relationship loading
#[derive(Debug)]
pub struct EagerLoader {
    /// Batch loading configuration
    batch_size: usize,
}

impl EagerLoader {
    /// slay Create new eager loader
    pub fn new() -> Self {
        Self {
            batch_size: 100,
        }
    }

    /// facts Load relationship eagerly
    #[instrument(skip(self, entity, relationship, db))]
    pub async fn load_relationship<T: Entity, R: Entity>(
        &self,
        entity: &T,
        relationship: &Relationship,
        db: Arc<DB>,
    ) -> crate::error::Result<()> {
        debug!(
            entity = T::table_name(),
            relationship = %relationship.name,
            "Loading relationship eagerly"
        );

        // Load relationship data
        let rows = self.batch_load_relationship(entity, relationship, db).await?;
        
        // Convert to entities
        let mut results = Vec::new();
        for row in rows {
            let entity = R::from_row(&row)?;
            results.push(entity);
        }

        debug!(count = results.len(), "Relationship loaded eagerly");
        Ok(results)
    }

    /// periodt Load relationships for multiple entities in batch
    #[instrument(skip(self, entities, relationship, db))]
    pub async fn batch_load_for_entities<T: Entity, R: Entity>(
        &self,
        entities: &[T],
        relationship: &Relationship,
        db: Arc<DB>,
    ) -> crate::error::Result<()> {
        info!(
            entity_count = entities.len(),
            relationship = %relationship.name,
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
            }

            let batch_results = self.execute_batch_query(relationship, &primary_keys, db.clone()).await?;
            
            // Group results by foreign key
            for row in batch_results {
                let foreign_key_value = match &relationship.relationship_type {
                    RelationshipType::HasMany { foreign_key, .. } => {
                        row.get(foreign_key).cloned()
                    }
                    _ => continue,
                };

                if let Some(fk_value) = foreign_key_value {
                    let entity = R::from_row(&row)?;
                    results.entry(fk_value).or_insert_with(Vec::new).push(entity);
                }
            }
        }

        info!(batches_loaded = results.len(), "Batch loading completed");
        Ok(results)
    }

    /// bestie Execute batch query for relationship
    async fn batch_load_relationship<T: Entity>(
        &self,
        entity: &T,
        relationship: &Relationship,
        db: Arc<DB>,
    ) -> crate::error::Result<()> {
        debug!(
            relationship_type = ?relationship.relationship_type,
            "Batch loading relationship for single entity"
        );
        
        match &relationship.relationship_type {
            RelationshipType::HasMany { foreign_key, .. } => {
                let local_value = entity.primary_key_value()
                    .ok_or_else(|| DatabaseError::validation_error("Entity must have primary key"))?;

                let sql = format!(
                    "SELECT * FROM {} WHERE {} = $1",
                    relationship.target_entity,
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
                    "SELECT * FROM {} WHERE {} = $1",
                    relationship.target_entity,
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
    }

    /// yolo Execute batch query for multiple primary keys
    async fn execute_batch_query(
        &self,
        relationship: &Relationship,
        primary_keys: &[SqlValue],
        db: Arc<DB>,
    ) -> crate::error::Result<()> {
        debug!(
            relationship = %relationship.name,
            key_count = primary_keys.len(),
            "Executing batch query"
        );

        if primary_keys.len() == 0 {
            return Ok(Vec::new());
        }

        match &relationship.relationship_type {
            RelationshipType::HasMany { foreign_key, .. } => {
                // Build IN clause for efficient batch loading
                let placeholders: Vec<String> = (1..=primary_keys.len())
                    .map(|i| format!("${}", i))
                    .collect();
                
                let sql = format!(
                    "SELECT * FROM {} WHERE {} IN ({})",
                    relationship.target_entity,
                    foreign_key,
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
                    "SELECT * FROM {} WHERE {} IN ({})",
                    relationship.target_entity,
                    owner_key,
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
    name: String,
    source_entity: String,
    target_entity: String,
}

impl RelationshipBuilder {
    /// slay Create new relationship builder
    pub fn new(name: &str, source_entity: &str, target_entity: &str) -> Self {
        Self {
            name: name.to_string(),
            source_entity: source_entity.to_string(),
            target_entity: target_entity.to_string(),
        }
    }

    /// facts Build HasOne relationship
    pub fn has_one(self, foreign_key: &str) -> Relationship {
        Relationship::new(
            self.name,
            RelationshipType::HasOne {
                foreign_key: foreign_key.to_string(),
                local_key: "id".to_string(),
            },
            self.source_entity,
            self.target_entity,
        )
    }

    /// periodt Build HasMany relationship
    pub fn has_many(self, foreign_key: &str) -> Relationship {
        Relationship::new(
            self.name,
            RelationshipType::HasMany {
                foreign_key: foreign_key.to_string(),
                local_key: "id".to_string(),
            },
            self.source_entity,
            self.target_entity,
        )
    }

    /// bestie Build BelongsTo relationship
    pub fn belongs_to(self, foreign_key: &str) -> Relationship {
        Relationship::new(
            self.name,
            RelationshipType::BelongsTo {
                foreign_key: foreign_key.to_string(),
                owner_key: "id".to_string(),
            },
            self.source_entity,
            self.target_entity,
        )
    }

    /// yolo Build BelongsToMany relationship
    pub fn belongs_to_many(
        self,
        pivot_table: &str,
        foreign_pivot_key: &str,
        related_pivot_key: &str,
    ) -> Relationship {
        Relationship::new(
            self.name,
            RelationshipType::BelongsToMany {
                pivot_table: pivot_table.to_string(),
                foreign_pivot_key: foreign_pivot_key.to_string(),
                related_pivot_key: related_pivot_key.to_string(),
                local_key: "id".to_string(),
                related_key: "id".to_string(),
            },
            self.source_entity,
            self.target_entity,
        )
    }
}

// Convenience trait implementations
pub trait HasOne<T: Entity>: Entity {
    fn has_one_relationship() -> Relationship;
}

pub trait HasMany<T: Entity>: Entity {
    fn has_many_relationship() -> Relationship;
}

pub trait BelongsTo<T: Entity>: Entity {
    fn belongs_to_relationship() -> Relationship;
}

pub trait BelongsToMany<T: Entity>: Entity {
    fn belongs_to_many_relationship() -> Relationship;
}

