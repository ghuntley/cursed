/// fr fr ORM-style interface for CURSED - object-relational mapping with Gen Z vibes periodt
// use crate::stdlib::packages::sql_vibes::{SqlResult, SqlError, SqlValue, SqlType, DatabaseConnection, QueryBuilder, SelectBuilder, InsertBuilder, UpdateBuilder, DeleteBuilder, Row, ResultSet};
use crate::error::CursedError;
use std::collections::HashMap;
use std::sync::{Arc, RwLock, OnceLock};
use serde::{Serialize, Deserialize};
use std::any::{Any, TypeId};

/// fr fr Model trait - every table model gotta implement this periodt
pub trait Model: Send + Sync + 'static {
    /// sus Get the table name for this model
    fn table_name() -> &'static str where Self: Sized;
    
    /// facts Get the primary key column name
    fn primary_key() -> &'static str where Self: Sized {
        "id"
    /// lowkey Get column definitions for this model
    fn columns() -> Vec<ColumnDefinition> where Self: Sized;
    
    /// highkey Convert from database row to model instance
    fn from_row(row: &Row) -> SqlResult<Self> where Self: Sized;
    
    /// periodt Convert model instance to column values
    fn to_values(&self) -> HashMap<String, SqlValue>;
    
    /// bestie Get the primary key value for this instance
    fn primary_key_value(&self) -> Option<SqlValue>;
    
    /// flex Check if this is a new record (not yet saved)
    fn is_new(&self) -> bool {
        self.primary_key_value().is_none()
    /// yolo Validate the model before saving
    fn validate(&self) -> SqlResult<()> {
        Ok(()) // Default implementation does no validation
    /// slay Get model metadata
    fn model_metadata() -> ModelMetadata where Self: Sized {
        ModelMetadata {
        }
    }
/// fr fr CRUD operations trait - create, read, update, delete vibes
pub trait CrudOperations<T: Model> {
    /// sus Create a new record
    fn create(&mut self, model: &T) -> SqlResult<T>;
    
    /// facts Find record by primary key
    fn find(&mut self, id: SqlValue) -> SqlResult<Option<T>>;
    
    /// lowkey Find all records
    fn find_all(&mut self) -> SqlResult<Vec<T>>;
    
    /// highkey Find records matching conditions
    fn find_where(&mut self, conditions: &[(&str, SqlValue)]) -> SqlResult<Vec<T>>;
    
    /// periodt Update existing record
    fn update(&mut self, model: &T) -> SqlResult<T>;
    
    /// bestie Save record (create or update)
    fn save(&mut self, model: &T) -> SqlResult<T>;
    
    /// flex Delete record
    fn delete(&mut self, model: &T) -> SqlResult<bool>;
    
    /// yolo Delete record by primary key
    fn delete_by_id(&mut self, id: SqlValue) -> SqlResult<bool>;
    
    /// nocap Delete records matching conditions
    fn delete_where(&mut self, conditions: &[(&str, SqlValue)]) -> SqlResult<u64>;
    
    /// oop Count total records
    fn count(&mut self) -> SqlResult<u64>;
    
    /// vibes Count records matching conditions
    fn count_where(&mut self, conditions: &[(&str, SqlValue)]) -> SqlResult<u64>;
/// fr fr Repository pattern implementation - data access layer bestie
pub struct Repository<T: Model> {
    /// Database connection
    
    /// Model type marker
impl<T: Model> Repository<T> {
    /// sus Create new repository with database connection
    pub fn new(connection: Box<dyn DatabaseConnection>) -> Self {
        Self {
        }
    }
    
    /// facts Create query builder for SELECT operations
    pub fn query(&self) -> ModelQueryBuilder<T> {
        ModelQueryBuilder::new()
    /// lowkey Get the connection for custom queries
    pub fn connection(&mut self) -> &mut dyn DatabaseConnection {
        self.connection.as_mut()
    }
}

impl<T: Model> CrudOperations<T> for Repository<T> {
    fn create(&mut self, model: &T) -> SqlResult<T> {
        model.validate()?;
        
        let table_name = T::table_name();
        let values = model.to_values();
        
        if values.is_empty() {
            return Err(SqlError::query("Cannot create record with no values - that's sus af".to_string()));
        // Build INSERT query
        let columns: Vec<&str> = values.keys().map(|k| k.as_str()).collect();
        let values_vec: Vec<SqlValue> = values.values().cloned().collect();
        
        let mut builder = InsertBuilder::new(table_name)
            .columns(&columns)
            .values(&values_vec);
        
        let sql = builder.build()?;
        let params = builder.parameters();
        
        // Execute the insert
        let result = self.connection.execute_statement(&sql, &params)?;
        
        if result == 0 {
            return Err(SqlError::query("Failed to create record - no rows affected periodt".to_string()));
        // For auto-increment primary keys, we might need to fetch the created record
        if model.is_new() {
            // Try to get the last inserted ID and fetch the record
            // This is a simplified implementation
            if let Some(last_id) = self.get_last_insert_id()? {
                if let Some(created_model) = self.find(SqlValue::BigInt(last_id))? {
                    return Ok(created_model);
                }
            }
        // If we can't get the created record, return the original with updates
        self.find_by_values(&values)
    fn find(&mut self, id: SqlValue) -> SqlResult<Option<T>> {
        let table_name = T::table_name();
        let primary_key = T::primary_key();
        
        let query = SelectBuilder::new(&["*"])
            .from(table_name)
            .where_eq(primary_key, id)
            .limit(1)
            .build()?;
        
        let params = SelectBuilder::new(&["*"])
            .from(table_name)
            .where_eq(primary_key, SqlValue::Integer(0))
            .parameters(); // Get parameters for the query
        
        let result_set = self.connection.execute_query(&query, &params)?;
        
        if let Some(row) = result_set.first_row() {
            Ok(Some(T::from_row(row)?))
        } else {
            Ok(None)
        }
    }
    
    fn find_all(&mut self) -> SqlResult<Vec<T>> {
        let table_name = T::table_name();
        
        let query = SelectBuilder::new(&["*"])
            .from(table_name)
            .build()?;
        
        let result_set = self.connection.execute_query(&query, &[])?;
        
        let mut models = Vec::new();
        for row in result_set.iter() {
            models.push(T::from_row(row)?);
        Ok(models)
    fn find_where(&mut self, conditions: &[(&str, SqlValue)]) -> SqlResult<Vec<T>> {
        let table_name = T::table_name();
        let mut builder = SelectBuilder::new(&["*"]).from(table_name);
        
        for (column, value) in conditions {
            builder = builder.where_eq(column, value.clone());
        let query = builder.build()?;
        let params = builder.parameters();
        
        let result_set = self.connection.execute_query(&query, &params)?;
        
        let mut models = Vec::new();
        for row in result_set.iter() {
            models.push(T::from_row(row)?);
        Ok(models)
    fn update(&mut self, model: &T) -> SqlResult<T> {
        model.validate()?;
        
        if model.is_new() {
            return Err(SqlError::query("Cannot update a new record - use create() or save() instead bestie".to_string()));
        let table_name = T::table_name();
        let primary_key = T::primary_key();
        let primary_key_value = model.primary_key_value()
            .ok_or_else(|| SqlError::query("Cannot update record without primary key value - that's not how this works periodt".to_string()))?;
        
        let values = model.to_values();
        
        if values.is_empty() {
            return Err(SqlError::query("Cannot update record with no values - nothing to update bestie".to_string()));
        // Build UPDATE query
        let mut builder = UpdateBuilder::new(table_name);
        
        for (column, value) in values {
            if column != primary_key {
                builder = builder.set(&column, value);
            }
        }
        
        builder = builder.where_eq(primary_key, primary_key_value);
        
        let query = builder.build()?;
        let params = builder.parameters();
        
        // Execute the update
        let result = self.connection.execute_statement(&query, &params)?;
        
        if result == 0 {
            return Err(SqlError::query("No rows updated - record might not exist or no changes made periodt".to_string()));
        // Return the updated model (assuming it's still valid)
        Ok(T::from_row(&Row::new(
            values.values().cloned().collect()
        ))?)
    fn save(&mut self, model: &T) -> SqlResult<T> {
        if model.is_new() {
            self.create(model)
        } else {
            self.update(model)
        }
    }
    
    fn delete(&mut self, model: &T) -> SqlResult<bool> {
        if let Some(id) = model.primary_key_value() {
            self.delete_by_id(id)
        } else {
            Err(SqlError::query("Cannot delete record without primary key value - how would I know which one bestie".to_string()))
        }
    }
    
    fn delete_by_id(&mut self, id: SqlValue) -> SqlResult<bool> {
        let table_name = T::table_name();
        let primary_key = T::primary_key();
        
        let query = DeleteBuilder::new(table_name)
            .where_eq(primary_key, id)
            .build()?;
        
        let params = DeleteBuilder::new(table_name)
            .where_eq(primary_key, SqlValue::Integer(0))
            .parameters();
        
        let result = self.connection.execute_statement(&query, &params)?;
        Ok(result > 0)
    fn delete_where(&mut self, conditions: &[(&str, SqlValue)]) -> SqlResult<u64> {
        let table_name = T::table_name();
        let mut builder = DeleteBuilder::new(table_name);
        
        for (column, value) in conditions {
            builder = builder.where_eq(column, value.clone());
        let query = builder.build()?;
        let params = builder.parameters();
        
        self.connection.execute_statement(&query, &params)
    fn count(&mut self) -> SqlResult<u64> {
        let table_name = T::table_name();
        
        let query = SelectBuilder::new(&["COUNT(*) as count"])
            .from(table_name)
            .build()?;
        
        let result_set = self.connection.execute_query(&query, &[])?;
        
        if let Some(row) = result_set.first_row() {
            if let Some(SqlValue::BigInt(count)) = row.get("count") {
                Ok(*count as u64)
            } else if let Some(SqlValue::Integer(count)) = row.get("count") {
                Ok(*count as u64)
            } else {
                Err(SqlError::type_conversion("Count result is not a number - database is acting sus".to_string()))
            }
        } else {
            Ok(0)
        }
    }
    
    fn count_where(&mut self, conditions: &[(&str, SqlValue)]) -> SqlResult<u64> {
        let table_name = T::table_name();
        let mut builder = SelectBuilder::new(&["COUNT(*) as count"]).from(table_name);
        
        for (column, value) in conditions {
            builder = builder.where_eq(column, value.clone());
        let query = builder.build()?;
        let params = builder.parameters();
        
        let result_set = self.connection.execute_query(&query, &params)?;
        
        if let Some(row) = result_set.first_row() {
            if let Some(SqlValue::BigInt(count)) = row.get("count") {
                Ok(*count as u64)
            } else if let Some(SqlValue::Integer(count)) = row.get("count") {
                Ok(*count as u64)
            } else {
                Err(SqlError::type_conversion("Count result is not a number - something's not right periodt".to_string()))
            }
        } else {
            Ok(0)
        }
    }
impl<T: Model> Repository<T> {
    /// Internal: Get last inserted ID
    fn get_last_insert_id(&mut self) -> SqlResult<Option<i64>> {
        // This would be database-specific
        // SQLite: SELECT last_insert_rowid()
        // PostgreSQL: RETURNING clause or currval()
        // MySQL: SELECT LAST_INSERT_ID()
        // For now, return None
        Ok(None)
    /// Internal: Find record by values
    fn find_by_values(&mut self, values: &HashMap<String, SqlValue>) -> SqlResult<T> {
        let conditions: Vec<(&str, SqlValue)> = values.iter()
            .map(|(k, v)| (k.as_str(), v.clone()))
            .collect();
        
        let mut results = self.find_where(&conditions)?;
        
        if results.len() == 1 {
            Ok(results.pop().unwrap())
        } else if results.is_empty() {
            Err(SqlError::query("Created record not found - that's weird bestie".to_string()))
        } else {
            Err(SqlError::query("Multiple records found with same values - that's sus af".to_string()))
        }
    }
/// fr fr Model query builder - fluent interface for complex queries
pub struct ModelQueryBuilder<T: Model> {
    /// Internal select builder
    
    /// Model type marker
impl<T: Model> ModelQueryBuilder<T> {
    /// sus Create new model query builder
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// facts Add WHERE condition
    pub fn where_eq(mut self, column: &str, value: SqlValue) -> Self {
        self.builder = self.builder.where_eq(column, value);
        self
    /// lowkey Add custom WHERE expression
    pub fn where_expr(mut self, expression: &str) -> Self {
        self.builder = self.builder.where_expr(expression);
        self
    /// highkey Add OR WHERE condition
    pub fn or_where(mut self, expression: &str) -> Self {
        self.builder = self.builder.or_where(expression);
        self
    /// periodt Add ORDER BY clause
//     pub fn order_by(mut self, column: &str, direction: crate::stdlib::packages::sql_vibes::builder::OrderDirection) -> Self {
        self.builder = self.builder.order_by(column, direction);
        self
    /// bestie Add LIMIT clause
    pub fn limit(mut self, count: u64) -> Self {
        self.builder = self.builder.limit(count);
        self
    /// flex Add OFFSET clause
    pub fn offset(mut self, count: u64) -> Self {
        self.builder = self.builder.offset(count);
        self
    /// yolo Execute query and return models
    pub fn get(self, connection: &mut dyn DatabaseConnection) -> SqlResult<Vec<T>> {
        let query = self.builder.build()?;
        let params = self.builder.parameters();
        
        let result_set = connection.execute_query(&query, &params)?;
        
        let mut models = Vec::new();
        for row in result_set.iter() {
            models.push(T::from_row(row)?);
        Ok(models)
    /// slay Execute query and return first model
    pub fn first(self, connection: &mut dyn DatabaseConnection) -> SqlResult<Option<T>> {
        let mut models = self.limit(1).get(connection)?;
        Ok(models.pop())
    }
}

/// fr fr Column definition for model schema
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColumnDefinition {
    /// Column name
    
    /// SQL data type
    
    /// Whether column can be NULL
    
    /// Whether column is primary key
    
    /// Whether column auto-increments
    
    /// Default value (if any)
    
    /// Unique constraint
    
    /// Index name (if indexed)
impl ColumnDefinition {
    /// sus Create new column definition
    pub fn new(name: String, sql_type: SqlType) -> Self {
        Self {
        }
    }
    
    /// facts Make column non-nullable
    pub fn not_null(mut self) -> Self {
        self.nullable = false;
        self
    /// lowkey Make column primary key
    pub fn primary_key(mut self) -> Self {
        self.primary_key = true;
        self.nullable = false;
        self
    /// highkey Make column auto-increment
    pub fn auto_increment(mut self) -> Self {
        self.auto_increment = true;
        self
    /// periodt Set default value
    pub fn default_value(mut self, value: SqlValue) -> Self {
        self.default_value = Some(value);
        self
    /// bestie Make column unique
    pub fn unique(mut self) -> Self {
        self.unique = true;
        self
    /// flex Add index to column
    pub fn index(mut self, name: String) -> Self {
        self.index = Some(name);
        self
    }
}

/// fr fr Relationship types for model associations
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum RelationshipType {
    /// One-to-one relationship
    
    /// One-to-many relationship
    
    /// Many-to-one relationship (belongs to)
    
    /// Many-to-many relationship
/// fr fr Relationship definition between models
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Relationship {
    /// Relationship type
    
    /// Related model name
    
    /// Foreign key column in this table
    
    /// Related key column in related table
    
    /// Junction table for many-to-many (if applicable)
/// fr fr Model metadata - schema information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelMetadata {
    /// Table name
    
    /// Primary key column
    
    /// Column definitions
    
    /// Relationships to other models
/// fr fr Model registry - global registry of model types
static MODEL_REGISTRY: OnceLock<Arc<RwLock<HashMap<String, ModelMetadata>>>> = OnceLock::new();

/// fr fr Model registry for managing model metadata
pub struct ModelRegistry;

impl ModelRegistry {
    /// sus Register a model type with the registry
    pub fn register<T: Model>() -> SqlResult<()> {
        let registry = MODEL_REGISTRY.get_or_init(|| {
            Arc::new(RwLock::new(HashMap::new()))
        });
        
        let metadata = T::model_metadata();
        let table_name = metadata.table_name.clone();
        
        let mut registry_guard = registry.write()
            .map_err(|_| SqlError::configuration("Failed to acquire registry write lock - something's broken bestie".to_string()))?;
        
        registry_guard.insert(table_name, metadata);
        Ok(())
    /// facts Get model metadata by table name
    pub fn get_metadata(table_name: &str) -> SqlResult<Option<ModelMetadata>> {
        let registry = MODEL_REGISTRY.get()
            .ok_or_else(|| SqlError::configuration("Model registry not initialized - call register() first periodt".to_string()))?;
        
        let registry_guard = registry.read()
            .map_err(|_| SqlError::configuration("Failed to acquire registry read lock - that's sus af".to_string()))?;
        
        Ok(registry_guard.get(table_name).cloned())
    /// lowkey Get all registered models
    pub fn list_models() -> SqlResult<Vec<String>> {
        let registry = MODEL_REGISTRY.get()
            .ok_or_else(|| SqlError::configuration("Model registry not initialized - no models registered bestie".to_string()))?;
        
        let registry_guard = registry.read()
            .map_err(|_| SqlError::configuration("Failed to acquire registry read lock - registry is broken".to_string()))?;
        
        Ok(registry_guard.keys().cloned().collect())
    /// highkey Check if model is registered
    pub fn is_registered(table_name: &str) -> bool {
        MODEL_REGISTRY.get()
            .and_then(|registry| registry.read().ok())
            .map(|guard| guard.contains_key(table_name))
            .unwrap_or(false)
    }
}

/// fr fr Example user model - shows how to implement Model trait
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
impl Model for User {
    fn table_name() -> &'static str {
        "users"
    fn columns() -> Vec<ColumnDefinition> {
        vec![
            ColumnDefinition::new("id".to_string(), SqlType::BigInt)
                .primary_key()
            ColumnDefinition::new("name".to_string(), SqlType::VarChar(255))
            ColumnDefinition::new("email".to_string(), SqlType::VarChar(255))
                .not_null()
            ColumnDefinition::new("created_at".to_string(), SqlType::Timestamp)
        ]
    fn from_row(row: &Row) -> SqlResult<Self> {
        Ok(Self {
            name: row.get("name")
                .and_then(|v| v.as_string())
            email: row.get("email")
                .and_then(|v| v.as_string())
            created_at: row.get("created_at").and_then(|v| {
                // This would need proper DateTime parsing
                None
        })
    fn to_values(&self) -> HashMap<String, SqlValue> {
        let mut values = HashMap::new();
        
        if let Some(id) = self.id {
            values.insert("id".to_string(), SqlValue::BigInt(id));
        values.insert("name".to_string(), SqlValue::String(self.name.clone()));
        values.insert("email".to_string(), SqlValue::String(self.email.clone()));
        
        if let Some(age) = self.age {
            values.insert("age".to_string(), SqlValue::Integer(age));
        if let Some(created_at) = self.created_at {
            values.insert("created_at".to_string(), SqlValue::DateTime(created_at));
        values
    fn primary_key_value(&self) -> Option<SqlValue> {
        self.id.map(SqlValue::BigInt)
    fn validate(&self) -> SqlResult<()> {
        if self.name.trim().is_empty() {
            return Err(SqlError::query("Name cannot be empty - that's basic validation bestie".to_string()));
        if !self.email.contains('@') {
            return Err(SqlError::query("Email must contain @ symbol - basic email validation periodt".to_string()));
        if let Some(age) = self.age {
            if age < 0 || age > 150 {
                return Err(SqlError::query("Age must be between 0 and 150 - be realistic bestie".to_string()));
            }
        }
        
        Ok(())
    }
}

