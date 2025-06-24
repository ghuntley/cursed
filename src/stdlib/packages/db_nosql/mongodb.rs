use crate::error::Error;
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
    bson::{doc, Document, Bson},
    options::{
        ClientOptions, CollectionOptions, CreateCollectionOptions, 
        DatabaseOptions, FindOptions, InsertManyOptions, UpdateOptions,
        DeleteOptions, IndexOptions, AggregateOptions, ReadPreference,
        WriteConcern, ReadConcern
    },
    results::{
        InsertOneResult, InsertManyResult, UpdateResult, DeleteResult,
        CreateIndexResult
    },
    Client, Database, Collection, IndexModel, Cursor
};

use futures::stream::TryStreamExt;

use crate::stdlib::value::Value;
use crate::error::CursedError;
use crate::stdlib::packages::ErrorKind;

/// MongoDB-specific error types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MongoDbError {
    /// Connection failed
    ConnectionFailed(String),
    /// Authentication failed
    AuthenticationFailed(String),
    /// Database operation failed
    OperationFailed(String),
    /// Collection not found
    CollectionNotFound(String),
    /// Index operation failed
    IndexOperationFailed(String),
    /// Aggregation pipeline failed
    AggregationFailed(String),
    /// Document serialization failed
    SerializationFailed(String),
    /// Document deserialization failed
    DeserializationFailed(String),
    /// Query parsing failed
    QueryParsingFailed(String),
    /// Timeout occurred
    TimeoutError(String),
    /// Replica set configuration error
    ReplicaSetError(String),
    /// Sharding error
    ShardingError(String),
    /// Transaction failed
    TransactionFailed(String),
    /// Invalid configuration
    InvalidConfiguration(String),
    /// General MongoDB error
    General(String),
}

impl From<MongoDbError> for CursedError {
    fn from(err: MongoDbError) -> Self {
        CursedError::new(
            ErrorKind::DatabaseError,
            format!("MongoDB error: {:?}", err),
            None,
        )
    }
}

impl From<mongodb::error::Error> for MongoDbError {
    fn from(err: mongodb::error::Error) -> Self {
        MongoDbError::General(err.to_string())
    }
}

/// MongoDB connection configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MongoDbConfig {
    /// Connection string (e.g., "mongodb://user:pass@host:port/db")
    pub connection_string: String,
    /// Database name
    pub database_name: String,
    /// Connection timeout in seconds
    pub connect_timeout: Option<u64>,
    /// Server selection timeout in seconds
    pub server_selection_timeout: Option<u64>,
    /// Socket timeout in seconds
    pub socket_timeout: Option<u64>,
    /// Heartbeat frequency in seconds
    pub heartbeat_frequency: Option<u64>,
    /// Maximum pool size
    pub max_pool_size: Option<u32>,
    /// Minimum pool size
    pub min_pool_size: Option<u32>,
    /// Maximum idle time in seconds
    pub max_idle_time: Option<u64>,
    /// Read preference
    pub read_preference: Option<String>,
    /// Write concern
    pub write_concern: Option<WriteConcernConfig>,
    /// Read concern
    pub read_concern: Option<String>,
    /// Enable SSL/TLS
    pub enable_ssl: bool,
    /// SSL certificate path
    pub ssl_cert_path: Option<String>,
    /// SSL key path
    pub ssl_key_path: Option<String>,
    /// SSL CA file path
    pub ssl_ca_file_path: Option<String>,
    /// Authentication mechanism
    pub auth_mechanism: Option<String>,
    /// Application name
    pub app_name: Option<String>,
    /// Replica set name
    pub replica_set: Option<String>,
    /// Enable retryable writes
    pub retry_writes: bool,
    /// Enable retryable reads
    pub retry_reads: bool,
    /// Compressors
    pub compressors: Option<Vec<String>>,
}

/// Write concern configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WriteConcernConfig {
    /// Number of nodes to acknowledge
    pub w: Option<i32>,
    /// Write concern string
    pub w_string: Option<String>,
    /// Journal acknowledgment
    pub journal: Option<bool>,
    /// Timeout in milliseconds
    pub timeout: Option<u64>,
}

impl Default for MongoDbConfig {
    fn default() -> Self {
        Self {
            connection_string: "mongodb://localhost:27017".to_string(),
            database_name: "cursed_db".to_string(),
            connect_timeout: Some(10),
            server_selection_timeout: Some(30),
            socket_timeout: Some(30),
            heartbeat_frequency: Some(10),
            max_pool_size: Some(10),
            min_pool_size: Some(1),
            max_idle_time: Some(600),
            read_preference: Some("primary".to_string()),
            write_concern: Some(WriteConcernConfig {
                w: Some(1),
                w_string: None,
                journal: Some(true),
                timeout: Some(5000),
            }),
            read_concern: Some("local".to_string()),
            enable_ssl: false,
            ssl_cert_path: None,
            ssl_key_path: None,
            ssl_ca_file_path: None,
            auth_mechanism: None,
            app_name: Some("cursed-mongodb-driver".to_string()),
            replica_set: None,
            retry_writes: true,
            retry_reads: true,
            compressors: None,
        }
    }
}

/// Query builder for MongoDB operations
#[derive(Debug, Clone)]
pub struct MongoDbQueryBuilder {
    filter: Document,
    projection: Option<Document>,
    sort: Option<Document>,
    limit: Option<i64>,
    skip: Option<u64>,
}

impl MongoDbQueryBuilder {
    /// Create a new query builder
    pub fn new() -> Self {
        Self {
            filter: Document::new(),
            projection: None,
            sort: None,
            limit: None,
            skip: None,
        }
    }

    /// Add a filter condition
    pub fn filter(mut self, key: &str, value: &Value) -> Result<(), Error> {
        let bson_value = value_to_bson(value)?;
        self.filter.insert(key, bson_value);
        Ok(self)
    }

    /// Add a projection
    pub fn project(mut self, fields: &[&str]) -> Self {
        let mut projection = Document::new();
        for field in fields {
            projection.insert(*field, 1);
        }
        self.projection = Some(projection);
        self
    }

    /// Add sorting
    pub fn sort(mut self, key: &str, direction: i32) -> Self {
        let mut sort = self.sort.unwrap_or_else(Document::new);
        sort.insert(key, direction);
        self.sort = Some(sort);
        self
    }

    /// Set limit
    pub fn limit(mut self, limit: i64) -> Self {
        self.limit = Some(limit);
        self
    }

    /// Set skip
    pub fn skip(mut self, skip: u64) -> Self {
        self.skip = Some(skip);
        self
    }

    /// Build find options
    pub fn build_find_options(self) -> FindOptions {
        FindOptions::builder()
            .projection(self.projection)
            .sort(self.sort)
            .limit(self.limit)
            .skip(self.skip)
            .build()
    }

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
    collection: Collection<Document>,
    name: String,
}

impl MongoDbCollection {
    /// Create a new collection wrapper
    pub fn new(collection: Collection<Document>, name: String) -> Self {
        Self { collection, name }
    }

    /// Get collection name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Find documents
    pub async fn find(&self, query: MongoDbQueryBuilder) -> Result<(), Error> {
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
        }

        Ok(results)
    }

    /// Find one document
    pub async fn find_one(&self, query: MongoDbQueryBuilder) -> Result<(), Error> {
        let filter = query.get_filter().clone();
        let options = query.build_find_options();
        
        let result = self.collection
            .find_one(filter, options)
            .await
            .map_err(MongoDbError::from)?;

        match result {
            Some(doc) => Ok(Some(bson_to_value(&Bson::Document(doc))?)),
            None => Ok(None),
        }
    }

    /// Insert one document
    pub async fn insert_one(&self, document: &Value) -> Result<(), Error> {
        let doc = value_to_document(document)?;
        let result = self.collection
            .insert_one(doc, None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(result.inserted_id.to_string())
    }

    /// Insert many documents
    pub async fn insert_many(&self, documents: &[Value]) -> Result<(), Error> {
        let docs: Result<(), Error> = documents
            .iter()
            .map(value_to_document)
            .collect();
        
        let docs = docs?;
        let result = self.collection
            .insert_many(docs, None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(result.inserted_ids.values().map(|id| id.to_string()).collect())
    }

    /// Update one document
    pub async fn update_one(
        &self,
        filter: MongoDbQueryBuilder,
        update: &Value,
    ) -> Result<(), Error> {
        let filter_doc = filter.get_filter().clone();
        let update_doc = value_to_document(update)?;
        let update_doc = doc! { "$set": update_doc };

        let result = self.collection
            .update_one(filter_doc, update_doc, None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(result.modified_count)
    }

    /// Update many documents
    pub async fn update_many(
        &self,
        filter: MongoDbQueryBuilder,
        update: &Value,
    ) -> Result<(), Error> {
        let filter_doc = filter.get_filter().clone();
        let update_doc = value_to_document(update)?;
        let update_doc = doc! { "$set": update_doc };

        let result = self.collection
            .update_many(filter_doc, update_doc, None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(result.modified_count)
    }

    /// Delete one document
    pub async fn delete_one(&self, filter: MongoDbQueryBuilder) -> Result<(), Error> {
        let filter_doc = filter.get_filter().clone();

        let result = self.collection
            .delete_one(filter_doc, None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(result.deleted_count)
    }

    /// Delete many documents
    pub async fn delete_many(&self, filter: MongoDbQueryBuilder) -> Result<(), Error> {
        let filter_doc = filter.get_filter().clone();

        let result = self.collection
            .delete_many(filter_doc, None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(result.deleted_count)
    }

    /// Count documents
    pub async fn count_documents(&self, filter: MongoDbQueryBuilder) -> Result<(), Error> {
        let filter_doc = filter.get_filter().clone();

        let count = self.collection
            .count_documents(filter_doc, None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(count)
    }

    /// Aggregate pipeline
    pub async fn aggregate(&self, pipeline: Vec<Document>) -> Result<(), Error> {
        let mut cursor = self.collection
            .aggregate(pipeline, None)
            .await
            .map_err(MongoDbError::from)?;

        let mut results = Vec::new();
        while let Some(doc) = cursor.try_next().await.map_err(MongoDbError::from)? {
            let value = bson_to_value(&Bson::Document(doc))?;
            results.push(value);
        }

        Ok(results)
    }

    /// Create index
    pub async fn create_index(&self, keys: Document, options: Option<IndexOptions>) -> Result<(), Error> {
        let index_model = IndexModel::builder()
            .keys(keys)
            .options(options)
            .build();

        let result = self.collection
            .create_index(index_model, None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(result.index_name)
    }

    /// Drop index
    pub async fn drop_index(&self, index_name: &str) -> Result<(), Error> {
        self.collection
            .drop_index(index_name, None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(())
    }

    /// List indexes
    pub async fn list_indexes(&self) -> Result<(), Error> {
        let mut cursor = self.collection
            .list_indexes(None)
            .await
            .map_err(MongoDbError::from)?;

        let mut results = Vec::new();
        while let Some(doc) = cursor.try_next().await.map_err(MongoDbError::from)? {
            let value = bson_to_value(&Bson::Document(doc))?;
            results.push(value);
        }

        Ok(results)
    }
}

/// MongoDB database operations
#[derive(Debug, Clone)]
pub struct MongoDbDatabase {
    database: Database,
    name: String,
}

impl MongoDbDatabase {
    /// Create a new database wrapper
    pub fn new(database: Database, name: String) -> Self {
        Self { database, name }
    }

    /// Get database name
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get collection
    pub fn collection(&self, name: &str) -> MongoDbCollection {
        let collection = self.database.collection(name);
        MongoDbCollection::new(collection, name.to_string())
    }

    /// Create collection
    pub async fn create_collection(&self, name: &str, options: Option<CreateCollectionOptions>) -> Result<(), Error> {
        self.database
            .create_collection(name, options)
            .await
            .map_err(MongoDbError::from)?;

        Ok(())
    }

    /// Drop collection
    pub async fn drop_collection(&self, name: &str) -> Result<(), Error> {
        self.database
            .collection::<Document>(name)
            .drop(None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(())
    }

    /// List collections
    pub async fn list_collections(&self) -> Result<(), Error> {
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
    }

    /// Run command
    pub async fn run_command(&self, command: Document) -> Result<(), Error> {
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
    client: Client,
    config: MongoDbConfig,
}

impl MongoDbConnection {
    /// Create a new connection
    pub async fn new(config: MongoDbConfig) -> Result<(), Error> {
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
    }

    /// Build client options from config
    async fn build_client_options(config: &MongoDbConfig) -> Result<(), Error> {
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
        }

        // Connection pooling
        if let Some(max_size) = config.max_pool_size {
            client_options.max_pool_size = Some(max_size);
        }
        if let Some(min_size) = config.min_pool_size {
            client_options.min_pool_size = Some(min_size);
        }
        if let Some(idle_time) = config.max_idle_time {
            client_options.max_idle_time = Some(Duration::from_secs(idle_time));
        }

        // Read preference
        if let Some(read_pref) = &config.read_preference {
            client_options.read_preference = Some(match read_pref.as_str() {
                "primary" => ReadPreference::Primary,
                "primaryPreferred" => ReadPreference::PrimaryPreferred { options: None },
                "secondary" => ReadPreference::Secondary { options: None },
                "secondaryPreferred" => ReadPreference::SecondaryPreferred { options: None },
                "nearest" => ReadPreference::Nearest { options: None },
                _ => ReadPreference::Primary,
            });
        }

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
        }

        // Read concern
        if let Some(read_concern) = &config.read_concern {
            client_options.read_concern = Some(match read_concern.as_str() {
                "local" => ReadConcern::Local,
                "available" => ReadConcern::Available,
                "majority" => ReadConcern::Majority,
                "linearizable" => ReadConcern::Linearizable,
                "snapshot" => ReadConcern::Snapshot,
                _ => ReadConcern::Local,
            });
        }

        // Application name
        if let Some(app_name) = &config.app_name {
            client_options.app_name = Some(app_name.clone());
        }

        // Replica set
        if let Some(replica_set) = &config.replica_set {
            client_options.replica_set_name = Some(replica_set.clone());
        }

        // Retry options
        client_options.retry_writes = Some(config.retry_writes);
        client_options.retry_reads = Some(config.retry_reads);

        // Compressors
        if let Some(compressors) = &config.compressors {
            client_options.compressors = Some(compressors.iter().filter_map(|c| {
                match c.as_str() {
                    "snappy" => Some(mongodb::options::Compressor::Snappy),
                    "zlib" => Some(mongodb::options::Compressor::Zlib { level: None }),
                    "zstd" => Some(mongodb::options::Compressor::Zstd { level: None }),
                    _ => None,
                }
            }).collect());
        }

        Ok(client_options)
    }

    /// Get database
    pub fn database(&self, name: Option<&str>) -> MongoDbDatabase {
        let db_name = name.unwrap_or(&self.config.database_name);
        let database = self.client.database(db_name);
        MongoDbDatabase::new(database, db_name.to_string())
    }

    /// Get default database
    pub fn default_database(&self) -> MongoDbDatabase {
        self.database(None)
    }

    /// List databases
    pub async fn list_databases(&self) -> Result<(), Error> {
        let databases = self.client
            .list_databases(None, None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(databases.into_iter().map(|spec| spec.name).collect())
    }

    /// Drop database
    pub async fn drop_database(&self, name: &str) -> Result<(), Error> {
        self.client
            .database(name)
            .drop(None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(())
    }

    /// Test connection
    pub async fn ping(&self) -> Result<(), Error> {
        self.client
            .database("admin")
            .run_command(doc! {"ping": 1}, None)
            .await
            .map_err(|e| MongoDbError::ConnectionFailed(e.to_string()))?;

        Ok(())
    }

    /// Get connection configuration
    pub fn config(&self) -> &MongoDbConfig {
        &self.config
    }
}

/// MongoDB driver with connection management
#[derive(Debug)]
pub struct MongoDbDriver {
    connections: Arc<RwLock<HashMap<String, MongoDbConnection>>>,
}

impl MongoDbDriver {
    /// Create a new MongoDB driver
    pub fn new() -> Self {
        Self {
            connections: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Add a connection with a name
    pub async fn add_connection(&self, name: String, config: MongoDbConfig) -> Result<(), Error> {
        let connection = MongoDbConnection::new(config).await?;
        let mut connections = self.connections.write().await;
        connections.insert(name, connection);
        Ok(())
    }

    /// Get a connection by name
    pub async fn get_connection(&self, name: &str) -> Result<(), Error> {
        let connections = self.connections.read().await;
        connections.get(name)
            .cloned()
            .ok_or_else(|| MongoDbError::ConnectionFailed(format!("Connection '{}' not found", name)))
    }

    /// Remove a connection
    pub async fn remove_connection(&self, name: &str) -> Result<(), Error> {
        let mut connections = self.connections.write().await;
        connections.remove(name)
            .ok_or_else(|| MongoDbError::ConnectionFailed(format!("Connection '{}' not found", name)))?;
        Ok(())
    }

    /// List all connections
    pub async fn list_connections(&self) -> Vec<String> {
        let connections = self.connections.read().await;
        connections.keys().cloned().collect()
    }

    /// Create a default connection
    pub async fn connect(config: MongoDbConfig) -> Result<(), Error> {
        MongoDbConnection::new(config).await
    }

    /// Create connection with default configuration
    pub async fn connect_default() -> Result<(), Error> {
        Self::connect(MongoDbConfig::default()).await
    }
}

impl Default for MongoDbDriver {
    fn default() -> Self {
        Self::new()
    }
}

/// Utility functions for BSON/Value conversion
fn value_to_bson(value: &Value) -> Result<(), Error> {
    match value {
        Value::Null => Ok(Bson::Null),
        Value::Bool(b) => Ok(Bson::Boolean(*b)),
        Value::Int(i) => Ok(Bson::Int64(*i)),
        Value::Float(f) => Ok(Bson::Double(*f)),
        Value::String(s) => Ok(Bson::String(s.clone())),
        Value::Array(arr) => {
            let bson_arr: Result<(), Error> = arr.iter()
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
}

fn bson_to_value(bson: &Bson) -> Result<(), Error> {
    match bson {
        Bson::Null => Ok(Value::Null),
        Bson::Boolean(b) => Ok(Value::Bool(*b)),
        Bson::Int32(i) => Ok(Value::Int(*i as i64)),
        Bson::Int64(i) => Ok(Value::Int(*i)),
        Bson::Double(f) => Ok(Value::Float(*f)),
        Bson::String(s) => Ok(Value::String(s.clone())),
        Bson::Array(arr) => {
            let values: Result<(), Error> = arr.iter()
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
        Bson::ObjectId(oid) => Ok(Value::String(oid.to_hex())),
        Bson::DateTime(dt) => Ok(Value::String(dt.to_rfc3339_string())),
        Bson::Binary(bin) => Ok(Value::String(format!("Binary({} bytes)", bin.bytes.len()))),
        Bson::Decimal128(dec) => Ok(Value::String(dec.to_string())),
        _ => Ok(Value::String(format!("Unsupported BSON type: {:?}", bson))),
    }
}

fn value_to_document(value: &Value) -> Result<(), Error> {
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
        )),
    }
}

/// Builder for MongoDB aggregation pipelines
#[derive(Debug, Clone)]
pub struct AggregationPipelineBuilder {
    stages: Vec<Document>,
}

impl AggregationPipelineBuilder {
    /// Create a new pipeline builder
    pub fn new() -> Self {
        Self {
            stages: Vec::new(),
        }
    }

    /// Add a match stage
    pub fn match_stage(mut self, filter: Document) -> Self {
        self.stages.push(doc! { "$match": filter });
        self
    }

    /// Add a group stage
    pub fn group_stage(mut self, group_doc: Document) -> Self {
        self.stages.push(doc! { "$group": group_doc });
        self
    }

    /// Add a sort stage
    pub fn sort_stage(mut self, sort_doc: Document) -> Self {
        self.stages.push(doc! { "$sort": sort_doc });
        self
    }

    /// Add a project stage
    pub fn project_stage(mut self, project_doc: Document) -> Self {
        self.stages.push(doc! { "$project": project_doc });
        self
    }

    /// Add a limit stage
    pub fn limit_stage(mut self, limit: i64) -> Self {
        self.stages.push(doc! { "$limit": limit });
        self
    }

    /// Add a skip stage
    pub fn skip_stage(mut self, skip: i64) -> Self {
        self.stages.push(doc! { "$skip": skip });
        self
    }

    /// Add a lookup stage
    pub fn lookup_stage(
        mut self,
        from: &str,
        local_field: &str,
        foreign_field: &str,
        as_field: &str,
    ) -> Self {
        self.stages.push(doc! {
            "$lookup": {
                "from": from,
                "localField": local_field,
                "foreignField": foreign_field,
                "as": as_field
            }
        });
        self
    }

    /// Add an unwind stage
    pub fn unwind_stage(mut self, path: &str) -> Self {
        self.stages.push(doc! { "$unwind": path });
        self
    }

    /// Add a custom stage
    pub fn custom_stage(mut self, stage: Document) -> Self {
        self.stages.push(stage);
        self
    }

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
    session: mongodb::ClientSession,
}

impl MongoDbTransaction {
    /// Start a new transaction
    pub async fn start(connection: &MongoDbConnection) -> Result<(), Error> {
        let mut session = connection.client.start_session(None)
            .await
            .map_err(MongoDbError::from)?;

        session.start_transaction(None)
            .await
            .map_err(MongoDbError::from)?;

        Ok(Self { session })
    }

    /// Commit the transaction
    pub async fn commit(mut self) -> Result<(), Error> {
        self.session.commit_transaction()
            .await
            .map_err(MongoDbError::from)?;

        Ok(())
    }

    /// Abort the transaction
    pub async fn abort(mut self) -> Result<(), Error> {
        self.session.abort_transaction()
            .await
            .map_err(MongoDbError::from)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::value::Value;
    use std::collections::HashMap;

    #[test]
    fn test_config_default() {
        let config = MongoDbConfig::default();
        assert_eq!(config.connection_string, "mongodb://localhost:27017");
        assert_eq!(config.database_name, "cursed_db");
        assert_eq!(config.max_pool_size, Some(10));
        assert_eq!(config.retry_writes, true);
    }

    #[test]
    fn test_query_builder() {
        let mut builder = MongoDbQueryBuilder::new();
        
        // Test filter
        let name_value = Value::String("test".to_string());
        builder = builder.filter("name", &name_value).unwrap();
        
        // Test projection and sorting
        builder = builder
            .project(&["name", "age"])
            .sort("age", -1)
            .limit(10)
            .skip(5);

        let options = builder.build_find_options();
        assert_eq!(options.limit, Some(10));
        assert_eq!(options.skip, Some(5));
        assert!(options.projection.is_some());
        assert!(options.sort.is_some());
    }

    #[test]
    fn test_value_to_bson_conversion() {
        // Test basic types
        assert!(matches!(value_to_bson(&Value::Null), Ok(Bson::Null)));
        assert!(matches!(value_to_bson(&Value::Bool(true)), Ok(Bson::Boolean(true))));
        assert!(matches!(value_to_bson(&Value::Int(42)), Ok(Bson::Int64(42))));
        assert!(matches!(value_to_bson(&Value::Float(3.14)), Ok(Bson::Double(_))));
        assert!(matches!(value_to_bson(&Value::String("test".to_string())), Ok(Bson::String(_))));

        // Test array
        let arr = Value::Array(vec![Value::Int(1), Value::Int(2)]);
        assert!(matches!(value_to_bson(&arr), Ok(Bson::Array(_))));

        // Test object
        let mut obj = HashMap::new();
        obj.insert("key".to_string(), Value::String("value".to_string()));
        let obj_val = Value::Object(obj);
        assert!(matches!(value_to_bson(&obj_val), Ok(Bson::Document(_))));
    }

    #[test]
    fn test_bson_to_value_conversion() {
        // Test basic types
        assert!(matches!(bson_to_value(&Bson::Null), Ok(Value::Null)));
        assert!(matches!(bson_to_value(&Bson::Boolean(true)), Ok(Value::Bool(true))));
        assert!(matches!(bson_to_value(&Bson::Int64(42)), Ok(Value::Int(42))));
        assert!(matches!(bson_to_value(&Bson::Double(3.14)), Ok(Value::Float(_))));
        assert!(matches!(bson_to_value(&Bson::String("test".to_string())), Ok(Value::String(_))));

        // Test array
        let arr = Bson::Array(vec![Bson::Int64(1), Bson::Int64(2)]);
        assert!(matches!(bson_to_value(&arr), Ok(Value::Array(_))));

        // Test document
        let mut doc = Document::new();
        doc.insert("key", "value");
        let doc_bson = Bson::Document(doc);
        assert!(matches!(bson_to_value(&doc_bson), Ok(Value::Object(_))));
    }

    #[test]
    fn test_aggregation_pipeline_builder() {
        let pipeline = AggregationPipelineBuilder::new()
            .match_stage(doc! { "status": "active" })
            .group_stage(doc! { "_id": "$category", "count": { "$sum": 1 } })
            .sort_stage(doc! { "count": -1 })
            .limit_stage(10)
            .build();

        assert_eq!(pipeline.len(), 4);
        assert!(pipeline[0].contains_key("$match"));
        assert!(pipeline[1].contains_key("$group"));
        assert!(pipeline[2].contains_key("$sort"));
        assert!(pipeline[3].contains_key("$limit"));
    }

    #[test]
    fn test_error_conversions() {
        let mongo_error = MongoDbError::ConnectionFailed("test".to_string());
        let cursed_error: CursedError = mongo_error.into();
        assert!(matches!(cursed_error.kind(), ErrorKind::DatabaseError));
    }

    #[tokio::test]
    async fn test_driver_connection_management() {
        let driver = MongoDbDriver::new();
        let config = MongoDbConfig::default();
        
        // Test adding connection (will fail without actual MongoDB, but tests the interface)
        let result = driver.add_connection("test".to_string(), config).await;
        // We expect this to fail in test environment without MongoDB
        assert!(result.is_err());
        
        // Test connection listing
        let connections = driver.list_connections().await;
        assert!(connections.is_empty());
    }
}
