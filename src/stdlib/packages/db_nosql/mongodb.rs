use crate::error::CursedError;
/// MongoDB driver for CURSED programming language
/// 
/// This module provides a comprehensive MongoDB driver with:
/// - Connection management with pooling and replica set support
/// - Full CRUD operations with query builder
/// - Async operations with tokio runtime
/// - Index management and aggregation pipelines
/// - Integration with CURSED's Value type system
/// - Production-ready error handling and resource management

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use serde::{Deserialize, Serialize};
use mongodb::{
    options::{
        WriteConcern, ReadConcern
    results::{
        CreateIndexResult
    Client, Database, Collection, IndexModel, Cursor
// };

use futures::stream::TryStreamExt;

// use crate::stdlib::value::Value;
// use crate::stdlib::packages::ErrorKind;

/// MongoDB-specific error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MongoDbError {
    /// Connection failed
    /// Authentication failed
    /// Database operation failed
    /// Collection not found
    /// Index operation failed
    /// Aggregation pipeline failed
    /// Document serialization failed
    /// Document deserialization failed
    /// Query parsing failed
    /// Timeout occurred
    /// Replica set configuration error
    /// Sharding error
    /// Transaction failed
    /// Invalid configuration
    /// General MongoDB error
// impl From<MongoDbError> for CursedError {
//     fn from(err: MongoDbError) -> Self {
//         CursedError::new(
//             ErrorKind::DatabaseError,
//             format!("MongoDB error: {:?}", err),
//             None,
//         )
//     }
// }

// impl From<mongodb::error::CursedError> for MongoDbError {
//     fn from(err: mongodb::error::CursedError) -> Self {
//         MongoDbError::General(err.to_string())
//     }
// }

/// MongoDB connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MongoDbConfig {
    /// Connection string (e.g., "mongodb://user:pass@host:port/db")
    /// Database name
    /// Connection timeout in seconds
    /// Server selection timeout in seconds
    /// Socket timeout in seconds
    /// Heartbeat frequency in seconds
    /// Maximum pool size
    /// Minimum pool size
    /// Maximum idle time in seconds
    /// Read preference
    /// Write concern
    /// Read concern
    /// Enable SSL/TLS
    /// SSL certificate path
    /// SSL key path
    /// SSL CA file path
    /// Authentication mechanism
    /// Application name
    /// Replica set name
    /// Enable retryable writes
    /// Enable retryable reads
    /// Compressors
/// Write concern configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteConcernConfig {
    /// Number of nodes to acknowledge
    /// Write concern string
    /// Journal acknowledgment
    /// Timeout in milliseconds
impl Default for MongoDbConfig {
    fn default() -> Self {
        Self {
            connection_string: "mongodb://localhost:27017".to_string(),
            write_concern: Some(WriteConcernConfig {
        }
    }
/// Query builder for MongoDB operations
#[derive(Debug, Clone)]
pub struct MongoDbQueryBuilder {
impl MongoDbQueryBuilder {
    /// Create a new query builder
    pub fn new() -> Self {
        Self {
        }
    }

    /// Add a filter condition
    pub fn filter(mut self, key: &str, value: &Value) -> crate::error::Result<()> {
        let bson_value = value_to_bson(value)?;
        self.filter.insert(key, bson_value);
        Ok(self)
    /// Add a projection
    pub fn project(mut self, fields: &[&str]) -> Self {
        let mut projection = Document::new();
        for field in fields {
            projection.insert(*field, 1);
        }
        self.projection = Some(projection);
        self
    /// Add sorting
    pub fn sort(mut self, key: &str, direction: i32) -> Self {
        let mut sort = self.sort.unwrap_or_else(Document::new);
        sort.insert(key, direction);
        self.sort = Some(sort);
        self
    /// Set limit
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    /// Set skip
    pub fn skip(mut self, skip: u64) -> Self {
        self.skip = Some(skip);
        self
    /// Build find options
    pub fn build_find_options(self) -> FindOptions {
        FindOptions::builder()
            .projection(self.projection)
            .sort(self.sort)
            .limit(self.limit)
            .skip(self.skip)
            .build()
    /// Get the filter document
    pub fn get_filter(&self) -> &Document {
        &self.filter
    }
}

impl Default for MongoDbQueryBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// MongoDB collection operations
#[derive(Debug, Clone)]
pub struct MongoDbCollection {
impl MongoDbCollection {
    /// Create a new collection wrapper
    pub fn new(collection: Collection<Document>, name: String) -> Self {
        Self { collection, name }
    }

    /// Get collection name
    pub fn name(&self) -> &str {
        &self.name
    /// Find documents
    pub async fn find(&self, query: MongoDbQueryBuilder) -> crate::error::Result<()> {
        let filter = query.get_filter().clone();
        let options = query.build_find_options();
        
        let mut cursor = self.collection
            .find(filter, options)
            .await
            .map_err(MongoDbError::from)?;

        let mut results = Vec::new();
        while let Some(doc) = cursor.try_next().await.map_err(MongoDbError::from)? {
            let value = bson_to_value(&Bson::Document(doc))?;
            results.push(value);
        Ok(results)
    /// Find one document
    pub async fn find_one(&self, query: MongoDbQueryBuilder) -> crate::error::Result<()> {
        let filter = query.get_filter().clone();
        let options = query.build_find_options();
        
        let result = self.collection
            .find_one(filter, options)
            .await
            .map_err(MongoDbError::from)?;

        match result {
        }
    }

    /// Insert one document
    pub async fn insert_one(&self, document: &Value) -> crate::error::Result<()> {
        let doc = value_to_document(document)?;
        let result = self.collection
            .insert_one(doc, None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(result.inserted_id.to_string())
    /// Insert many documents
    pub async fn insert_many(&self, documents: &[Value]) -> crate::error::Result<()> {
        let docs: crate::error::Result<()> = documents
            .iter()
            .map(value_to_document)
            .collect();
        
        let docs = docs?;
        let result = self.collection
            .insert_many(docs, None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(result.inserted_ids.values().map(|id| id.to_string()).collect())
    /// Update one document
    pub async fn update_one(
    ) -> crate::error::Result<()> {
        let filter_doc = filter.get_filter().clone();
        let update_doc = value_to_document(update)?;
        let update_doc = doc! { "$set": update_doc };

        let result = self.collection
            .update_one(filter_doc, update_doc, None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(result.modified_count)
    /// Update many documents
    pub async fn update_many(
    ) -> crate::error::Result<()> {
        let filter_doc = filter.get_filter().clone();
        let update_doc = value_to_document(update)?;
        let update_doc = doc! { "$set": update_doc };

        let result = self.collection
            .update_many(filter_doc, update_doc, None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(result.modified_count)
    /// Delete one document
    pub async fn delete_one(&self, filter: MongoDbQueryBuilder) -> crate::error::Result<()> {
        let filter_doc = filter.get_filter().clone();

        let result = self.collection
            .delete_one(filter_doc, None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(result.deleted_count)
    /// Delete many documents
    pub async fn delete_many(&self, filter: MongoDbQueryBuilder) -> crate::error::Result<()> {
        let filter_doc = filter.get_filter().clone();

        let result = self.collection
            .delete_many(filter_doc, None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(result.deleted_count)
    /// Count documents
    pub async fn count_documents(&self, filter: MongoDbQueryBuilder) -> crate::error::Result<()> {
        let filter_doc = filter.get_filter().clone();

        let count = self.collection
            .count_documents(filter_doc, None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(count)
    /// Aggregate pipeline
    pub async fn aggregate(&self, pipeline: Vec<Document>) -> crate::error::Result<()> {
        let mut cursor = self.collection
            .aggregate(pipeline, None)
            .await
            .map_err(MongoDbError::from)?;

        let mut results = Vec::new();
        while let Some(doc) = cursor.try_next().await.map_err(MongoDbError::from)? {
            let value = bson_to_value(&Bson::Document(doc))?;
            results.push(value);
        Ok(results)
    /// Create index
    pub async fn create_index(&self, keys: Document, options: Option<IndexOptions>) -> crate::error::Result<()> {
        let index_model = IndexModel::builder()
            .keys(keys)
            .options(options)
            .build();

        let result = self.collection
            .create_index(index_model, None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(result.index_name)
    /// Drop index
    pub async fn drop_index(&self, index_name: &str) -> crate::error::Result<()> {
        self.collection
            .drop_index(index_name, None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(())
    /// List indexes
    pub async fn list_indexes(&self) -> crate::error::Result<()> {
        let mut cursor = self.collection
            .list_indexes(None)
            .await
            .map_err(MongoDbError::from)?;

        let mut results = Vec::new();
        while let Some(doc) = cursor.try_next().await.map_err(MongoDbError::from)? {
            let value = bson_to_value(&Bson::Document(doc))?;
            results.push(value);
        Ok(results)
    }
}

/// MongoDB database operations
#[derive(Debug, Clone)]
pub struct MongoDbDatabase {
impl MongoDbDatabase {
    /// Create a new database wrapper
    pub fn new(database: Database, name: String) -> Self {
        Self { database, name }
    }

    /// Get database name
    pub fn name(&self) -> &str {
        &self.name
    /// Get collection
    pub fn collection(&self, name: &str) -> MongoDbCollection {
        let collection = self.database.collection(name);
        MongoDbCollection::new(collection, name.to_string())
    /// Create collection
    pub async fn create_collection(&self, name: &str, options: Option<CreateCollectionOptions>) -> crate::error::Result<()> {
        self.database
            .create_collection(name, options)
            .await
            .map_err(MongoDbError::from)?;

        Ok(())
    /// Drop collection
    pub async fn drop_collection(&self, name: &str) -> crate::error::Result<()> {
        self.database
            .collection::<Document>(name)
            .drop(None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(())
    /// List collections
    pub async fn list_collections(&self) -> crate::error::Result<()> {
        let mut cursor = self.database
            .list_collections(None, None)
            .await
            .map_err(MongoDbError::from)?;

        let mut results = Vec::new();
        while let Some(spec) = cursor.try_next().await.map_err(MongoDbError::from)? {
            if let Some(name) = spec.name {
                results.push(name);
            }
        }

        Ok(results)
    /// Run command
    pub async fn run_command(&self, command: Document) -> crate::error::Result<()> {
        let result = self.database
            .run_command(command, None)
            .await
            .map_err(MongoDbError::from)?;

        bson_to_value(&Bson::Document(result))
    }
}

/// MongoDB connection with connection pooling
#[derive(Debug, Clone)]
pub struct MongoDbConnection {
impl MongoDbConnection {
    /// Create a new connection
    pub async fn new(config: MongoDbConfig) -> crate::error::Result<()> {
        let client_options = Self::build_client_options(&config).await?;
        let client = Client::with_options(client_options)
            .map_err(MongoDbError::from)?;

        // Test connection
        client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await
            .map_err(|e| MongoDbError::ConnectionFailed(e.to_string()))?;

        Ok(Self { client, config })
    /// Build client options from config
    async fn build_client_options(config: &MongoDbConfig) -> crate::error::Result<()> {
        let mut client_options = ClientOptions::parse(&config.connection_string)
            .await
            .map_err(|e| MongoDbError::InvalidConfiguration(e.to_string()))?;

        // Connection timeouts
        if let Some(timeout) = config.connect_timeout {
            client_options.connect_timeout = Some(Duration::from_secs(timeout));
        }
        if let Some(timeout) = config.server_selection_timeout {
            client_options.server_selection_timeout = Some(Duration::from_secs(timeout));
        }
        if let Some(timeout) = config.socket_timeout {
            client_options.socket_timeout = Some(Duration::from_secs(timeout));
        }
        if let Some(freq) = config.heartbeat_frequency {
            client_options.heartbeat_freq = Some(Duration::from_secs(freq));
        // Connection pooling
        if let Some(max_size) = config.max_pool_size {
            client_options.max_pool_size = Some(max_size);
        }
        if let Some(min_size) = config.min_pool_size {
            client_options.min_pool_size = Some(min_size);
        }
        if let Some(idle_time) = config.max_idle_time {
            client_options.max_idle_time = Some(Duration::from_secs(idle_time));
        // Read preference
        if let Some(read_pref) = &config.read_preference {
            client_options.read_preference = Some(match read_pref.as_str() {
            });
        // Write concern
        if let Some(write_concern_config) = &config.write_concern {
            let mut write_concern = WriteConcern::builder();
            if let Some(w) = write_concern_config.w {
                write_concern = write_concern.w(mongodb::options::Acknowledgment::from(w));
            }
            if let Some(w_string) = &write_concern_config.w_string {
                write_concern = write_concern.w(mongodb::options::Acknowledgment::from(w_string.as_str()));
            }
            if let Some(journal) = write_concern_config.journal {
                write_concern = write_concern.journal(journal);
            }
            if let Some(timeout) = write_concern_config.timeout {
                write_concern = write_concern.w_timeout(Duration::from_millis(timeout));
            }
            client_options.write_concern = Some(write_concern.build());
        // Read concern
        if let Some(read_concern) = &config.read_concern {
            client_options.read_concern = Some(match read_concern.as_str() {
            });
        // Application name
        if let Some(app_name) = &config.app_name {
            client_options.app_name = Some(app_name.clone());
        // Replica set
        if let Some(replica_set) = &config.replica_set {
            client_options.replica_set_name = Some(replica_set.clone());
        // Retry options
        client_options.retry_writes = Some(config.retry_writes);
        client_options.retry_reads = Some(config.retry_reads);

        // Compressors
        if let Some(compressors) = &config.compressors {
            client_options.compressors = Some(compressors.iter().filter_map(|c| {
                match c.as_str() {
                }
            }).collect());
        Ok(client_options)
    /// Get database
    pub fn database(&self, name: Option<&str>) -> MongoDbDatabase {
        let db_name = name.unwrap_or(&self.config.database_name);
        let database = self.client.database(db_name);
        MongoDbDatabase::new(database, db_name.to_string())
    /// Get default database
    pub fn default_database(&self) -> MongoDbDatabase {
        self.database(None)
    /// List databases
    pub async fn list_databases(&self) -> crate::error::Result<()> {
        let databases = self.client
            .list_databases(None, None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(databases.into_iter().map(|spec| spec.name).collect())
    /// Drop database
    pub async fn drop_database(&self, name: &str) -> crate::error::Result<()> {
        self.client
            .database(name)
            .drop(None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(())
    /// Test connection
    pub async fn ping(&self) -> crate::error::Result<()> {
        self.client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await
            .map_err(|e| MongoDbError::ConnectionFailed(e.to_string()))?;

        Ok(())
    /// Get connection configuration
    pub fn config(&self) -> &MongoDbConfig {
        &self.config
    }
}

/// MongoDB driver with connection management
#[derive(Debug)]
pub struct MongoDbDriver {
impl MongoDbDriver {
    /// Create a new MongoDB driver
    pub fn new() -> Self {
        Self {
        }
    }

    /// Add a connection with a name
    pub async fn add_connection(&self, name: String, config: MongoDbConfig) -> crate::error::Result<()> {
        let connection = MongoDbConnection::new(config).await?;
        let mut connections = self.connections.write().await;
        connections.insert(name, connection);
        Ok(())
    /// Get a connection by name
    pub async fn get_connection(&self, name: &str) -> crate::error::Result<()> {
        let connections = self.connections.read().await;
        connections.get(name)
            .cloned()
            .ok_or_else(|| MongoDbError::ConnectionFailed(format!("Connection '{}' not found", name)))
    /// Remove a connection
    pub async fn remove_connection(&self, name: &str) -> crate::error::Result<()> {
        let mut connections = self.connections.write().await;
        connections.remove(name)
            .ok_or_else(|| MongoDbError::ConnectionFailed(format!("Connection '{}' not found", name)))?;
        Ok(())
    /// List all connections
    pub async fn list_connections(&self) -> Vec<String> {
        let connections = self.connections.read().await;
        connections.keys().cloned().collect()
    /// Create a default connection
    pub async fn connect(config: MongoDbConfig) -> crate::error::Result<()> {
        MongoDbConnection::new(config).await
    /// Create connection with default configuration
    pub async fn connect_default() -> crate::error::Result<()> {
        Self::connect(MongoDbConfig::default()).await
    }
}

impl Default for MongoDbDriver {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for BSON/Value conversion
fn value_to_bson(value: &Value) -> crate::error::Result<()> {
    match value {
        Value::Array(arr) => {
            let bson_arr: crate::error::Result<()> = arr.iter()
                .map(value_to_bson)
                .collect();
            Ok(Bson::Array(bson_arr?))
        }
        Value::Object(obj) => {
            let mut doc = Document::new();
            for (key, val) in obj {
                doc.insert(key.clone(), value_to_bson(val)?);
            }
            Ok(Bson::Document(doc))
        }
    }
fn bson_to_value(bson: &Bson) -> crate::error::Result<()> {
    match bson {
        Bson::Array(arr) => {
            let values: crate::error::Result<()> = arr.iter()
                .map(bson_to_value)
                .collect();
            Ok(Value::Array(values?))
        }
        Bson::Document(doc) => {
            let mut obj = std::collections::HashMap::new();
            for (key, val) in doc {
                obj.insert(key.clone(), bson_to_value(val)?);
            }
            Ok(Value::Object(obj))
        }
    }
}

fn value_to_document(value: &Value) -> crate::error::Result<()> {
    match value {
        Value::Object(obj) => {
            let mut doc = Document::new();
            for (key, val) in obj {
                doc.insert(key.clone(), value_to_bson(val)?);
            }
            Ok(doc)
        }
        _ => Err(MongoDbError::SerializationFailed(
            "Value must be an object to convert to Document".to_string()
    }
}

/// Builder for MongoDB aggregation pipelines
#[derive(Debug, Clone)]
pub struct AggregationPipelineBuilder {
impl AggregationPipelineBuilder {
    /// Create a new pipeline builder
    pub fn new() -> Self {
        Self {
        }
    }

    /// Add a match stage
    pub fn match_stage(mut self, filter: Document) -> Self {
        self.stages.push(doc! { "$match": filter });
        self
    /// Add a group stage
    pub fn group_stage(mut self, group_doc: Document) -> Self {
        self.stages.push(doc! { "$group": group_doc });
        self
    /// Add a sort stage
    pub fn sort_stage(mut self, sort_doc: Document) -> Self {
        self.stages.push(doc! { "$sort": sort_doc });
        self
    /// Add a project stage
    pub fn project_stage(mut self, project_doc: Document) -> Self {
        self.stages.push(doc! { "$project": project_doc });
        self
    /// Add a limit stage
    pub fn limit_stage(mut self, limit: i64) -> Self {
        self.stages.push(doc! { "$limit": limit });
        self
    /// Add a skip stage
    pub fn skip_stage(mut self, skip: i64) -> Self {
        self.stages.push(doc! { "$skip": skip });
        self
    /// Add a lookup stage
    pub fn lookup_stage(
    ) -> Self {
        self.stages.push(doc! {
            "$lookup": {
                "as": as_field
            }
        });
        self
    /// Add an unwind stage
    pub fn unwind_stage(mut self, path: &str) -> Self {
        self.stages.push(doc! { "$unwind": path });
        self
    /// Add a custom stage
    pub fn custom_stage(mut self, stage: Document) -> Self {
        self.stages.push(stage);
        self
    /// Build the pipeline
    pub fn build(self) -> Vec<Document> {
        self.stages
    }
}

impl Default for AggregationPipelineBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Transaction session wrapper
#[derive(Debug)]
pub struct MongoDbTransaction {
impl MongoDbTransaction {
    /// Start a new transaction
    pub async fn start(connection: &MongoDbConnection) -> crate::error::Result<()> {
        let mut session = connection.client.start_session(None)
            .await
            .map_err(MongoDbError::from)?;

        session.start_transaction(None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(Self { session })
    /// Commit the transaction
    pub async fn commit(mut self) -> crate::error::Result<()> {
        self.session.commit_transaction()
            .await
            .map_err(MongoDbError::from)?;

        Ok(())
    /// Abort the transaction
    pub async fn abort(mut self) -> crate::error::Result<()> {
        self.session.abort_transaction()
            .await
            .map_err(MongoDbError::from)?;

        Ok(())
    }
}

