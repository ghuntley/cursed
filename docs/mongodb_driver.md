# MongoDB Driver for CURSED Programming Language

## Overview

The CURSED MongoDB driver provides comprehensive NoSQL database functionality with full async support, connection pooling, and seamless integration with CURSED's type system. This enterprise-grade driver supports all major MongoDB operations and follows modern database interaction patterns.

## Features

### Core Functionality
- **Complete CRUD Operations**: Create, read, update, and delete documents
- **Advanced Querying**: Complex query building with filtering, projection, and sorting
- **Aggregation Pipelines**: Full support for MongoDB aggregation framework
- **Index Management**: Create, drop, and list indexes with various options
- **Collection Management**: Create, drop, and manage collections
- **Database Operations**: Multi-database support with database-level operations

### Connection Management
- **Connection Pooling**: Configurable connection pools with min/max settings
- **Replica Set Support**: Full replica set configuration and automatic failover
- **SSL/TLS Security**: Complete SSL/TLS support with certificate configuration
- **Authentication**: Multiple authentication mechanisms
- **Retry Logic**: Configurable retry for reads and writes
- **Timeout Management**: Comprehensive timeout configuration

### CURSED Integration
- **Value Type Conversion**: Seamless conversion between CURSED Values and BSON
- **Error System Integration**: MongoDB errors integrate with CURSED error handling
- **Async/Await Support**: Full tokio runtime integration
- **Type Safety**: Compile-time type safety with runtime validation

## Quick Start

### Basic Connection

```cursed
import "stdlib::packages::db_nosql";

// Configure and connect
facts config = MongoDbConfig {
    connection_string: "mongodb://localhost:27017",
    database_name: "my_app",
    max_pool_size: Some(20),
    retry_writes: true,
};

facts connection = MongoDbConnection::new(config).await?;
facts db = connection.default_database();
facts collection = db.collection("users");
```

### CRUD Operations

```cursed
// Create a document
facts user = Value::Object({
    sus mut obj = HashMap::new();
    obj.insert("name", Value::String("Alice"));
    obj.insert("email", Value::String("alice@example.com"));
    obj.insert("age", Value::Int(28));
    obj
});

// Insert document
facts insert_id = collection.insert_one(&user).await?;

// Find documents
facts query = MongoDbQueryBuilder::new()
    .filter("age", &Value::Int(28))?
    .sort("name", 1)
    .limit(10);

facts users = collection.find(query).await?;

// Update document
facts update_filter = MongoDbQueryBuilder::new()
    .filter("email", &Value::String("alice@example.com"))?;

facts update_data = Value::Object({
    sus mut obj = HashMap::new();
    obj.insert("age", Value::Int(29));
    obj
});

facts modified = collection.update_one(update_filter, &update_data).await?;

// Delete document
facts delete_filter = MongoDbQueryBuilder::new()
    .filter("email", &Value::String("alice@example.com"))?;

facts deleted = collection.delete_one(delete_filter).await?;
```

## Configuration

### Connection Configuration

```cursed
facts config = MongoDbConfig {
    // Basic connection
    connection_string: "mongodb://user:pass@host:port/db",
    database_name: "my_database",
    
    // Connection pooling
    max_pool_size: Some(25),
    min_pool_size: Some(5),
    max_idle_time: Some(600),
    
    // Timeouts
    connect_timeout: Some(10),
    server_selection_timeout: Some(30),
    socket_timeout: Some(30),
    
    // Read/Write preferences
    read_preference: Some("primaryPreferred"),
    write_concern: Some(WriteConcernConfig {
        w: Some(1),
        journal: Some(true),
        timeout: Some(5000),
    }),
    
    // SSL/TLS
    enable_ssl: true,
    ssl_cert_path: Some("/path/to/cert.pem"),
    ssl_key_path: Some("/path/to/key.pem"),
    
    // Replica set
    replica_set: Some("rs0"),
    
    // Retry options
    retry_writes: true,
    retry_reads: true,
    
    // Application identification
    app_name: Some("my-cursed-app"),
};
```

### Write Concern Configuration

```cursed
facts write_concern = WriteConcernConfig {
    w: Some(2),                    // Acknowledge from 2 nodes
    journal: Some(true),           // Wait for journal sync
    timeout: Some(10000),          // 10 second timeout
};

// Or with string acknowledgment
facts majority_write = WriteConcernConfig {
    w_string: Some("majority"),    // Wait for majority
    journal: Some(true),
    timeout: Some(5000),
};
```

## Advanced Querying

### Query Builder

```cursed
// Complex query with multiple conditions
facts query = MongoDbQueryBuilder::new()
    .filter("department", &Value::String("Engineering"))?
    .filter("active", &Value::Bool(true))?
    .filter("age", &Value::Object({
        sus mut range = HashMap::new();
        range.insert("$gte", Value::Int(25));
        range.insert("$lte", Value::Int(65));
        range
    }))?
    .project(&["name", "email", "department"])
    .sort("name", 1)
    .limit(50)
    .skip(10);

facts results = collection.find(query).await?;
```

### Text Search

```cursed
// Create text index
collection.create_index(
    doc! { "title": "text", "content": "text" },
    Some(IndexOptions::builder().name("text_search_idx").build())
).await?;

// Search with text query
facts text_query = MongoDbQueryBuilder::new()
    .filter("$text", &Value::Object({
        sus mut search = HashMap::new();
        search.insert("$search", Value::String("mongodb database"));
        search
    }))?;

facts search_results = collection.find(text_query).await?;
```

## Aggregation Pipelines

### Basic Aggregation

```cursed
facts pipeline = AggregationPipelineBuilder::new()
    .match_stage(doc! { "status": "active" })
    .group_stage(doc! {
        "_id": "$department",
        "count": { "$sum": 1 },
        "avg_salary": { "$avg": "$salary" }
    })
    .sort_stage(doc! { "avg_salary": -1 })
    .limit_stage(5)
    .build();

facts aggregation_results = collection.aggregate(pipeline).await?;
```

### Advanced Aggregation with Lookups

```cursed
facts advanced_pipeline = AggregationPipelineBuilder::new()
    .match_stage(doc! { "active": true })
    .lookup_stage("departments", "dept_id", "_id", "department_info")
    .unwind_stage("$department_info")
    .project_stage(doc! {
        "employee_name": "$name",
        "department_name": "$department_info.name",
        "budget": "$department_info.budget"
    })
    .group_stage(doc! {
        "_id": "$department_name",
        "total_employees": { "$sum": 1 },
        "total_budget": { "$first": "$budget" }
    })
    .custom_stage(doc! {
        "$addFields": {
            "budget_per_employee": {
                "$divide": ["$total_budget", "$total_employees"]
            }
        }
    })
    .sort_stage(doc! { "budget_per_employee": -1 })
    .build();

facts results = collection.aggregate(advanced_pipeline).await?;
```

## Index Management

### Creating Indexes

```cursed
// Single field index
facts email_index = collection.create_index(
    doc! { "email": 1 },
    Some(IndexOptions::builder()
        .unique(true)
        .name("email_unique_idx")
        .build())
).await?;

// Compound index
facts compound_index = collection.create_index(
    doc! { "department": 1, "salary": -1 },
    Some(IndexOptions::builder()
        .name("dept_salary_idx")
        .build())
).await?;

// Text index
facts text_index = collection.create_index(
    doc! { "title": "text", "content": "text" },
    Some(IndexOptions::builder()
        .name("fulltext_search")
        .build())
).await?;

// Geospatial index
facts geo_index = collection.create_index(
    doc! { "location": "2dsphere" },
    Some(IndexOptions::builder()
        .name("location_idx")
        .build())
).await?;
```

### Managing Indexes

```cursed
// List all indexes
facts indexes = collection.list_indexes().await?;
for index in indexes {
    println(&format!("Index: {:?}", index))?;
}

// Drop specific index
collection.drop_index("email_unique_idx").await?;
```

## Collection Management

### Creating Collections

```cursed
// Regular collection
db.create_collection("products", None).await?;

// Capped collection
db.create_collection("logs", Some(
    CreateCollectionOptions::builder()
        .capped(true)
        .size(1024 * 1024 * 100)  // 100MB
        .max(10000)               // Max 10k documents
        .build()
)).await?;

// Collection with validation
db.create_collection("validated_users", Some(
    CreateCollectionOptions::builder()
        .validator(doc! {
            "$jsonSchema": {
                "bsonType": "object",
                "required": ["name", "email"],
                "properties": {
                    "name": { "bsonType": "string" },
                    "email": { "bsonType": "string" }
                }
            }
        })
        .validation_level("strict")
        .validation_action("error")
        .build()
)).await?;
```

### Collection Operations

```cursed
// List collections
facts collections = db.list_collections().await?;
for collection_name in collections {
    println(&format!("Collection: {}", collection_name))?;
}

// Drop collection
db.drop_collection("old_data").await?;

// Get collection statistics
facts stats = db.run_command(doc! {
    "collStats": "users",
    "scale": 1024  // KB
}).await?;
```

## Transactions

### Basic Transactions

```cursed
// Start transaction
facts transaction = MongoDbTransaction::start(&connection).await?;

// Perform operations within transaction
facts accounts = db.collection("accounts");

// Transfer money between accounts
facts from_filter = MongoDbQueryBuilder::new()
    .filter("account_id", &Value::String("acc_001"))?;

facts to_filter = MongoDbQueryBuilder::new()
    .filter("account_id", &Value::String("acc_002"))?;

// Deduct from source account
accounts.update_one(from_filter, &Value::Object({
    sus mut update = HashMap::new();
    update.insert("$inc", Value::Object({
        sus mut inc = HashMap::new();
        inc.insert("balance", Value::Float(-100.0));
        inc
    }));
    update
})).await?;

// Add to destination account
accounts.update_one(to_filter, &Value::Object({
    sus mut update = HashMap::new();
    update.insert("$inc", Value::Object({
        sus mut inc = HashMap::new();
        inc.insert("balance", Value::Float(100.0));
        inc
    }));
    update
})).await?;

// Commit transaction
transaction.commit().await?;
```

### Transaction Error Handling

```cursed
facts transaction = MongoDbTransaction::start(&connection).await?;

match perform_transaction_operations(&accounts).await {
    Ok(_) => {
        transaction.commit().await?;
        println("Transaction committed successfully")?;
    }
    Err(e) => {
        transaction.abort().await?;
        println(&format!("Transaction aborted due to error: {:?}", e))?;
    }
}
```

## Connection Management

### Multiple Connections

```cursed
facts driver = MongoDbDriver::new();

// Add multiple named connections
driver.add_connection("primary", primary_config).await?;
driver.add_connection("analytics", analytics_config).await?;
driver.add_connection("cache", cache_config).await?;

// Use specific connections
facts primary_conn = driver.get_connection("primary").await?;
facts analytics_conn = driver.get_connection("analytics").await?;

// List all connections
facts connection_names = driver.list_connections().await;
```

### Connection Health Monitoring

```cursed
// Test connection health
match connection.ping().await {
    Ok(_) => println("Connection is healthy")?,
    Err(e) => println(&format!("Connection issue: {:?}", e))?,
}

// Get connection configuration
facts config = connection.config();
println(&format!("Connected to: {}", config.connection_string))?;
```

## Error Handling

### MongoDB Error Types

```cursed
match collection.insert_one(&document).await {
    Ok(insert_id) => println(&format!("Inserted: {}", insert_id))?,
    Err(MongoDbError::ConnectionFailed(msg)) => {
        println(&format!("Connection failed: {}", msg))?;
    }
    Err(MongoDbError::AuthenticationFailed(msg)) => {
        println(&format!("Auth failed: {}", msg))?;
    }
    Err(MongoDbError::OperationFailed(msg)) => {
        println(&format!("Operation failed: {}", msg))?;
    }
    Err(MongoDbError::TimeoutError(msg)) => {
        println(&format!("Timeout: {}", msg))?;
    }
    Err(e) => {
        println(&format!("Other error: {:?}", e))?;
    }
}
```

### Error Recovery

```cursed
facts max_retries = 3;
facts retry_count = 0;

while retry_count < max_retries {
    match collection.insert_one(&document).await {
        Ok(result) => {
            println("Insert successful")?;
            break;
        }
        Err(MongoDbError::TimeoutError(_)) if retry_count < max_retries - 1 => {
            retry_count += 1;
            println(&format!("Retry {} due to timeout", retry_count))?;
            tokio::time::sleep(Duration::from_millis(1000 * retry_count)).await;
        }
        Err(e) => {
            println(&format!("Insert failed after {} retries: {:?}", retry_count, e))?;
            break;
        }
    }
}
```

## Performance Optimization

### Connection Pooling

```cursed
facts optimized_config = MongoDbConfig {
    // Optimize pool size for your workload
    max_pool_size: Some(50),      // High concurrency
    min_pool_size: Some(10),      // Keep connections warm
    max_idle_time: Some(300),     // 5 minute idle timeout
    
    // Optimize timeouts
    connect_timeout: Some(5),     // Fast connection establishment
    server_selection_timeout: Some(10),  // Quick server selection
    socket_timeout: Some(60),     // Longer socket timeout for large ops
    
    ..MongoDbConfig::default()
};
```

### Efficient Queries

```cursed
// Use projections to reduce network transfer
facts efficient_query = MongoDbQueryBuilder::new()
    .filter("active", &Value::Bool(true))?
    .project(&["name", "email"])  // Only fetch needed fields
    .limit(100);                  // Limit result size

// Use indexes for sorting
facts indexed_query = MongoDbQueryBuilder::new()
    .sort("created_at", -1)       // Sort by indexed field
    .limit(20);

// Batch operations
facts documents = vec![doc1, doc2, doc3];
facts insert_ids = collection.insert_many(&documents).await?;
```

### Aggregation Optimization

```cursed
facts optimized_pipeline = AggregationPipelineBuilder::new()
    .match_stage(doc! { "active": true })     // Filter early
    .project_stage(doc! {                     // Project early
        "name": 1,
        "department": 1,
        "salary": 1
    })
    .group_stage(doc! {
        "_id": "$department",
        "avg_salary": { "$avg": "$salary" }
    })
    .sort_stage(doc! { "avg_salary": -1 })
    .limit_stage(10)
    .build();
```

## Best Practices

### Security

1. **Use Authentication**: Always configure proper authentication
2. **Enable SSL/TLS**: Use encrypted connections in production
3. **Validate Input**: Sanitize all user input before queries
4. **Use Least Privilege**: Grant minimal required permissions
5. **Monitor Access**: Log and monitor database access patterns

### Performance

1. **Create Indexes**: Index frequently queried fields
2. **Use Projections**: Only fetch required fields
3. **Batch Operations**: Use insert_many, update_many when possible
4. **Optimize Aggregations**: Filter and project early in pipelines
5. **Connection Pooling**: Configure appropriate pool sizes

### Reliability

1. **Handle Errors**: Implement comprehensive error handling
2. **Use Transactions**: For multi-operation consistency
3. **Retry Logic**: Implement retry for transient failures
4. **Health Checks**: Monitor connection health
5. **Graceful Shutdown**: Properly close connections

### Monitoring

```cursed
// Log operations for monitoring
facts query_start = Instant::now();
facts results = collection.find(query).await?;
facts query_duration = query_start.elapsed();

if query_duration > Duration::from_millis(100) {
    println(&format!("Slow query detected: {:?}", query_duration))?;
}
```

## API Reference

### MongoDbConfig
- `connection_string: String` - MongoDB connection URI
- `database_name: String` - Default database name
- `max_pool_size: Option<u32>` - Maximum connection pool size
- `connect_timeout: Option<u64>` - Connection timeout in seconds
- `retry_writes: bool` - Enable retryable writes

### MongoDbConnection
- `new(config: MongoDbConfig) -> Result<Self, MongoDbError>` - Create connection
- `database(name: Option<&str>) -> MongoDbDatabase` - Get database
- `ping() -> Result<(), MongoDbError>` - Test connection

### MongoDbDatabase
- `collection(name: &str) -> MongoDbCollection` - Get collection
- `create_collection(name: &str, options: Option<CreateCollectionOptions>) -> Result<(), MongoDbError>`
- `list_collections() -> Result<Vec<String>, MongoDbError>`

### MongoDbCollection
- `find(query: MongoDbQueryBuilder) -> Result<Vec<Value>, MongoDbError>`
- `find_one(query: MongoDbQueryBuilder) -> Result<Option<Value>, MongoDbError>`
- `insert_one(document: &Value) -> Result<String, MongoDbError>`
- `insert_many(documents: &[Value]) -> Result<Vec<String>, MongoDbError>`
- `update_one(filter: MongoDbQueryBuilder, update: &Value) -> Result<u64, MongoDbError>`
- `delete_one(filter: MongoDbQueryBuilder) -> Result<u64, MongoDbError>`
- `aggregate(pipeline: Vec<Document>) -> Result<Vec<Value>, MongoDbError>`
- `create_index(keys: Document, options: Option<IndexOptions>) -> Result<String, MongoDbError>`

### MongoDbQueryBuilder
- `new() -> Self` - Create new query builder
- `filter(key: &str, value: &Value) -> Result<Self, MongoDbError>` - Add filter
- `project(fields: &[&str]) -> Self` - Set projection
- `sort(key: &str, direction: i32) -> Self` - Add sorting
- `limit(limit: i64) -> Self` - Set limit
- `skip(skip: u64) -> Self` - Set skip

This comprehensive documentation covers all aspects of the MongoDB driver, from basic usage to advanced optimization techniques, providing CURSED developers with everything needed to build robust database-driven applications.
