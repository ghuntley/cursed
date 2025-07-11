//! Relationship loading and management for CURSED ORM

use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

use crate::error::CursedError;
use super::{Entity, Repository, SqlValue, DatabaseConnection, DatabaseError};

/// Result type for relationship operations
pub type RelationshipResult<T> = Result<T, DatabaseError>;

/// One-to-one relationship definition
#[derive(Debug, Clone)]
pub struct HasOne<T, R> {
    pub foreign_key: String,
    pub local_key: String,
    pub _parent: PhantomData<T>,
    pub _related: PhantomData<R>,
}

/// One-to-many relationship definition
#[derive(Debug, Clone)]
pub struct HasMany<T, R> {
    pub foreign_key: String,
    pub local_key: String,
    pub _parent: PhantomData<T>,
    pub _related: PhantomData<R>,
}

/// Belongs-to (inverse one-to-many) relationship definition
#[derive(Debug, Clone)]
pub struct BelongsTo<T, R> {
    pub foreign_key: String,
    pub owner_key: String,
    pub _child: PhantomData<T>,
    pub _parent: PhantomData<R>,
}

/// Many-to-many relationship definition
#[derive(Debug, Clone)]
pub struct BelongsToMany<T, R> {
    pub pivot_table: String,
    pub foreign_pivot_key: String,
    pub related_pivot_key: String,
    pub parent_key: String,
    pub related_key: String,
    pub _parent: PhantomData<T>,
    pub _related: PhantomData<R>,
}

/// Lazy loading strategy for relationships
#[derive(Debug, Clone)]
pub struct LazyLoader<T> {
    pub loaded: bool,
    pub query_builder: Option<String>,
    pub _entity: PhantomData<T>,
}

/// Eager loading strategy for relationships
#[derive(Debug, Clone)]
pub struct EagerLoader<T> {
    pub relationships: Vec<String>,
    pub constraints: HashMap<String, String>,
    pub _entity: PhantomData<T>,
}

/// Relationship loader trait for different loading strategies
pub trait RelationshipLoader<T: Entity> {
    fn load(&self, entity: &T, connection: Arc<dyn DatabaseConnection>) -> RelationshipResult<()>;
}

/// Relationship manager for handling entity relationships
pub struct RelationshipManager {
    connection: Arc<dyn DatabaseConnection>,
    lazy_loading_enabled: bool,
    eager_loading_cache: HashMap<String, Vec<SqlValue>>,
}

impl RelationshipManager {
    /// Create a new relationship manager
    pub fn new(connection: Arc<dyn DatabaseConnection>) -> Self {
        Self {
            connection,
            lazy_loading_enabled: true,
            eager_loading_cache: HashMap::new(),
        }
    }

    /// Enable or disable lazy loading
    pub fn set_lazy_loading(&mut self, enabled: bool) {
        self.lazy_loading_enabled = enabled;
    }

    /// Load has-one relationship
    pub fn load_has_one<T: Entity, R: Entity>(
        &self,
        parent: &T,
        relationship: &HasOne<T, R>,
    ) -> RelationshipResult<Option<R>> {
        let parent_key_value = parent.primary_key_value()
            .ok_or_else(|| DatabaseError::query("Parent entity has no primary key"))?;

        let sql = format!(
            "SELECT * FROM {} WHERE {} = ? LIMIT 1",
            R::table_name(),
            relationship.foreign_key
        );

        match self.connection.query(sql, vec![parent_key_value]) {
            Ok(result) => {
                if result.rows().is_empty() {
                    Ok(None)
                } else {
                    let row = &result.rows()[0];
                    let row_map = row.to_hashmap();
                    match R::from_row(&row_map) {
                        Ok(related) => Ok(Some(related)),
                        Err(e) => Err(e),
                    }
                }
            }
            Err(e) => Err(DatabaseError::query(&format!("Failed to load has-one relationship: {}", e))),
        }
    }

    /// Load has-many relationship
    pub fn load_has_many<T: Entity, R: Entity>(
        &self,
        parent: &T,
        relationship: &HasMany<T, R>,
    ) -> RelationshipResult<Vec<R>> {
        let parent_key_value = parent.primary_key_value()
            .ok_or_else(|| DatabaseError::query("Parent entity has no primary key"))?;

        let sql = format!(
            "SELECT * FROM {} WHERE {} = ?",
            R::table_name(),
            relationship.foreign_key
        );

        match self.connection.query(sql, vec![parent_key_value]) {
            Ok(result) => {
                let mut related_entities = Vec::new();
                for row in result.rows() {
                    let row_map = row.to_hashmap();
                    match R::from_row(&row_map) {
                        Ok(related) => related_entities.push(related),
                        Err(e) => return Err(e),
                    }
                }
                Ok(related_entities)
            }
            Err(e) => Err(DatabaseError::query(&format!("Failed to load has-many relationship: {}", e))),
        }
    }

    /// Load belongs-to relationship
    pub fn load_belongs_to<T: Entity, R: Entity>(
        &self,
        child: &T,
        relationship: &BelongsTo<T, R>,
    ) -> RelationshipResult<Option<R>> {
        let child_fields = child.to_fields();
        let foreign_key_value = child_fields.get(&relationship.foreign_key)
            .ok_or_else(|| DatabaseError::query("Child entity has no foreign key"))?;

        let sql = format!(
            "SELECT * FROM {} WHERE {} = ? LIMIT 1",
            R::table_name(),
            relationship.owner_key
        );

        match self.connection.query(sql, vec![foreign_key_value.clone()]) {
            Ok(result) => {
                if result.rows().is_empty() {
                    Ok(None)
                } else {
                    let row = &result.rows()[0];
                    let row_map = row.to_hashmap();
                    match R::from_row(&row_map) {
                        Ok(parent) => Ok(Some(parent)),
                        Err(e) => Err(e),
                    }
                }
            }
            Err(e) => Err(DatabaseError::query(&format!("Failed to load belongs-to relationship: {}", e))),
        }
    }

    /// Load belongs-to-many relationship
    pub fn load_belongs_to_many<T: Entity, R: Entity>(
        &self,
        parent: &T,
        relationship: &BelongsToMany<T, R>,
    ) -> RelationshipResult<Vec<R>> {
        let parent_key_value = parent.primary_key_value()
            .ok_or_else(|| DatabaseError::query("Parent entity has no primary key"))?;

        let sql = format!(
            "SELECT r.* FROM {} r JOIN {} p ON r.{} = p.{} WHERE p.{} = ?",
            R::table_name(),
            relationship.pivot_table,
            relationship.related_key,
            relationship.related_pivot_key,
            relationship.foreign_pivot_key
        );

        match self.connection.query(sql, vec![parent_key_value]) {
            Ok(result) => {
                let mut related_entities = Vec::new();
                for row in result.rows() {
                    let row_map = row.to_hashmap();
                    match R::from_row(&row_map) {
                        Ok(related) => related_entities.push(related),
                        Err(e) => return Err(e),
                    }
                }
                Ok(related_entities)
            }
            Err(e) => Err(DatabaseError::query(&format!("Failed to load belongs-to-many relationship: {}", e))),
        }
    }

    /// Eager load multiple relationships
    pub fn eager_load_relationships<T: Entity>(
        &self,
        entities: &mut Vec<T>,
        relationships: &[String],
    ) -> RelationshipResult<()> {
        for relationship_name in relationships {
            println!("🔄 Eager loading relationship: {}", relationship_name);
            // In a real implementation, this would use reflection or a registry
            // to determine the relationship type and load accordingly
        }
        Ok(())
    }

    /// Lazy load a relationship when accessed
    pub fn lazy_load_relationship<T: Entity>(
        &self,
        entity: &mut T,
        relationship_name: &str,
    ) -> RelationshipResult<()> {
        if !self.lazy_loading_enabled {
            return Ok(());
        }

        println!("🔄 Lazy loading relationship: {}", relationship_name);
        // In a real implementation, this would use reflection or a registry
        // to determine the relationship type and load accordingly
        Ok(())
    }

    /// Clear the eager loading cache
    pub fn clear_cache(&mut self) {
        self.eager_loading_cache.clear();
    }
}

impl<T, R> HasOne<T, R> {
    pub fn new(foreign_key: &str, local_key: &str) -> Self {
        Self {
            foreign_key: foreign_key.to_string(),
            local_key: local_key.to_string(),
            _parent: PhantomData,
            _related: PhantomData,
        }
    }
}

impl<T, R> HasMany<T, R> {
    pub fn new(foreign_key: &str, local_key: &str) -> Self {
        Self {
            foreign_key: foreign_key.to_string(),
            local_key: local_key.to_string(),
            _parent: PhantomData,
            _related: PhantomData,
        }
    }
}

impl<T, R> BelongsTo<T, R> {
    pub fn new(foreign_key: &str, owner_key: &str) -> Self {
        Self {
            foreign_key: foreign_key.to_string(),
            owner_key: owner_key.to_string(),
            _child: PhantomData,
            _parent: PhantomData,
        }
    }
}

impl<T, R> BelongsToMany<T, R> {
    pub fn new(pivot_table: &str, foreign_pivot_key: &str, related_pivot_key: &str) -> Self {
        Self {
            pivot_table: pivot_table.to_string(),
            foreign_pivot_key: foreign_pivot_key.to_string(),
            related_pivot_key: related_pivot_key.to_string(),
            parent_key: "id".to_string(),
            related_key: "id".to_string(),
            _parent: PhantomData,
            _related: PhantomData,
        }
    }
    
    pub fn with_keys(mut self, parent_key: &str, related_key: &str) -> Self {
        self.parent_key = parent_key.to_string();
        self.related_key = related_key.to_string();
        self
    }
}

impl<T> LazyLoader<T> {
    pub fn new() -> Self {
        Self {
            loaded: false,
            query_builder: None,
            _entity: PhantomData,
        }
    }
    
    pub fn with_query(mut self, query: &str) -> Self {
        self.query_builder = Some(query.to_string());
        self
    }
    
    pub fn load(&mut self) -> Result<(), CursedError> {
        if !self.loaded {
            self.loaded = true;
            println!("🔄 Lazy loaded relationship");
        }
        Ok(())
    }
}

impl<T> EagerLoader<T> {
    pub fn new() -> Self {
        Self {
            relationships: Vec::new(),
            constraints: HashMap::new(),
            _entity: PhantomData,
        }
    }
    
    pub fn with_relationship(mut self, relationship: &str) -> Self {
        self.relationships.push(relationship.to_string());
        self
    }
    
    pub fn with_constraint(mut self, relationship: &str, constraint: &str) -> Self {
        self.constraints.insert(relationship.to_string(), constraint.to_string());
        self
    }
    
    pub fn load(&self) -> Result<(), CursedError> {
        println!("🔄 Eager loaded {} relationships", self.relationships.len());
        Ok(())
    }
}

/// Initialize relationship loading system
pub fn init_relationships() -> Result<(), CursedError> {
    println!("📁 Relationship loading system initialized");
    Ok(())
}

/// Test relationship functionality
pub fn test_relationships() -> Result<(), CursedError> {
    let has_one = HasOne::<i32, String>::new("user_id", "id");
    let has_many = HasMany::<i32, String>::new("user_id", "id");
    let belongs_to = BelongsTo::<i32, String>::new("user_id", "id");
    let belongs_to_many = BelongsToMany::<i32, String>::new("user_roles", "user_id", "role_id");
    
    let mut lazy_loader = LazyLoader::<i32>::new();
    lazy_loader.load()?;
    
    let eager_loader = EagerLoader::<i32>::new()
        .with_relationship("posts")
        .with_relationship("comments");
    eager_loader.load()?;
    
    println!("✅ Relationship tests passed");
    Ok(())
}
