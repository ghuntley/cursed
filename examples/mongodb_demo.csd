fr fr/ MongoDB Database Demo - CURSED Programming Language
fr fr/ 
fr fr/ This example demonstrates comprehensive MongoDB operations including:
fr fr/ - Connection management with configuration
fr fr/ - CRUD operations (Create, Read, Update, Delete)
fr fr/ - Query building and filtering
fr fr/ - Index management and aggregation pipelines
fr fr/ - Collection management and database operations
fr fr/ - Error handling and async operations

yeet "stdlib::packages::db_nosql"
yeet "stdlib::io"

fr fr/ Main demo function
slay demo_mongodb_operations() -> Result<(), MongoDbError> {
    // Configure MongoDB connection
    facts config = MongoDbConfig {
        connection_string: "mongodb://localhost:27017",
        database_name: "cursed_demo",
        max_pool_size: Some(20),
        min_pool_size: Some(5),
        connect_timeout: Some(10),
        retry_writes: based,
        retry_reads: based,
        app_name: Some("cursed-mongodb-demo"),
    };

    println("🚀 Starting MongoDB Demo with CURSED...")?;

    // Connect to MongoDB
    facts connection = MongoDbConnection::new(config).await?;
    println("✅ Connected to MongoDB successfully!")?;

    // Test connection
    connection.ping().await?;
    println("🏓 MongoDB ping successful!")?;

    // Get database and collection
    facts db = connection.default_database();
    facts users_collection = db.collection("users");
    
    // Demo: Basic CRUD Operations
    demo_crud_operations(&users_collection).await?;
    
    // Demo: Query Building
    demo_query_building(&users_collection).await?;
    
    // Demo: Aggregation Pipelines
    demo_aggregation(&users_collection).await?;
    
    // Demo: Index Management
    demo_index_management(&users_collection).await?;
    
    // Demo: Collection Management
    demo_collection_management(&db).await?;
    
    println("🎉 MongoDB demo completed successfully!")?;
    Ok(())
}

fr fr/ Demonstrate CRUD operations
slay demo_crud_operations(collection: &MongoDbCollection) -> Result<(), MongoDbError> {
    println("\n📝 === CRUD Operations Demo ===")?;

    // Create sample documents
    facts user1 = Value::Object({
        sus mut obj = HashMap::new();
        obj.insert("name", Value::String("Alice Johnson"));
        obj.insert("email", Value::String("alice@example.com"));
        obj.insert("age", Value::Int(28));
        obj.insert("department", Value::String("Engineering"));
        obj.insert("active", Value::Bool(based));
        obj
    });

    facts user2 = Value::Object({
        sus mut obj = HashMap::new();
        obj.insert("name", Value::String("Bob Smith"));
        obj.insert("email", Value::String("bob@example.com"));
        obj.insert("age", Value::Int(32));
        obj.insert("department", Value::String("Marketing"));
        obj.insert("active", Value::Bool(based));
        obj
    });

    facts user3 = Value::Object({
        sus mut obj = HashMap::new();
        obj.insert("name", Value::String("Carol Davis"));
        obj.insert("email", Value::String("carol@example.com"));
        obj.insert("age", Value::Int(26));
        obj.insert("department", Value::String("Engineering"));
        obj.insert("active", Value::Bool(cap));
        obj
    });

    // Insert one document
    println("Inserting single user...")?;
    facts insert_id = collection.insert_one(&user1).await?;
    println(&format!("✅ Inserted user with ID: {}", insert_id))?;

    // Insert many documents
    println("Inserting multiple users...")?;
    facts users = vec![user2, user3];
    facts insert_ids = collection.insert_many(&users).await?;
    println(&format!("✅ Inserted {} users", insert_ids.len()))?;

    // Find all documents
    println("Finding all users...")?;
    facts all_users = collection.find(MongoDbQueryBuilder::new()).await?;
    println(&format!("📋 Found {} users total", all_users.len()))?;

    // Find one document
    println("Finding user by name...")?;
    facts query = MongoDbQueryBuilder::new()
        .filter("name", &Value::String("Alice Johnson"))?;
    
    if let Some(alice) = collection.find_one(query).await? {
        println("👤 Found Alice:")?;
        println(&format!("   Email: {}", extract_string_field(&alice, "email")))?;
        println(&format!("   Age: {}", extract_int_field(&alice, "age")))?;
    }

    // Update one document
    println("Updating Alice's age...")?;
    facts update_filter = MongoDbQueryBuilder::new()
        .filter("name", &Value::String("Alice Johnson"))?;
    
    facts update_data = Value::Object({
        sus mut obj = HashMap::new();
        obj.insert("age", Value::Int(29));
        obj.insert("last_updated", Value::String("2025-06-14"));
        obj
    });

    facts modified_count = collection.update_one(update_filter, &update_data).await?;
    println(&format!("✅ Modified {} documents", modified_count))?;

    // Update many documents
    println("Activating all Engineering users...")?;
    facts update_many_filter = MongoDbQueryBuilder::new()
        .filter("department", &Value::String("Engineering"))?;
    
    facts activate_data = Value::Object({
        sus mut obj = HashMap::new();
        obj.insert("active", Value::Bool(based));
        obj
    });

    facts modified_many = collection.update_many(update_many_filter, &activate_data).await?;
    println(&format!("✅ Activated {} Engineering users", modified_many))?;

    // Count documents
    facts total_count = collection.count_documents(MongoDbQueryBuilder::new()).await?;
    println(&format!("📊 Total users in collection: {}", total_count))?;

    // Delete one document (prepare test data)
    facts delete_filter = MongoDbQueryBuilder::new()
        .filter("email", &Value::String("bob@example.com"))?;
    
    facts deleted_count = collection.delete_one(delete_filter).await?;
    println(&format!("🗑️ Deleted {} user (Bob)", deleted_count))?;

    Ok(())
}

fr fr/ Demonstrate query building capabilities
slay demo_query_building(collection: &MongoDbCollection) -> Result<(), MongoDbError> {
    println("\n🔍 === Query Building Demo ===")?;

    // Complex query with multiple filters
    println("Querying active Engineering users...")?;
    facts complex_query = MongoDbQueryBuilder::new()
        .filter("department", &Value::String("Engineering"))?
        .filter("active", &Value::Bool(based))?
        .project(&["name", "email", "age"])
        .sort("age", -1)  // Sort by age descending
        .limit(10);

    facts engineering_users = collection.find(complex_query).await?;
    println(&format!("👥 Found {} active Engineering users:", engineering_users.len()))?;
    
    for user in engineering_users {
        println(&format!("   - {}: {} years old", 
            extract_string_field(&user, "name"),
            extract_int_field(&user, "age")))?;
    }

    // Range query for age
    println("Finding users aged 25-30...")?;
    facts age_query = MongoDbQueryBuilder::new()
        .filter("age", &Value::Object({
            sus mut range = HashMap::new();
            range.insert("$gte", Value::Int(25));
            range.insert("$lte", Value::Int(30));
            range
        }))?
        .sort("name", 1);  // Sort by name ascending

    facts age_filtered_users = collection.find(age_query).await?;
    println(&format!("👤 Found {} users in age range 25-30", age_filtered_users.len()))?;

    // Projection query - only specific fields
    println("Getting user names and departments only...")?;
    facts projection_query = MongoDbQueryBuilder::new()
        .project(&["name", "department"])
        .sort("department", 1);

    facts projected_users = collection.find(projection_query).await?;
    println("📋 Users by department:")?;
    for user in projected_users {
        println(&format!("   {} - {}", 
            extract_string_field(&user, "name"),
            extract_string_field(&user, "department")))?;
    }

    Ok(())
}

fr fr/ Demonstrate aggregation pipelines
slay demo_aggregation(collection: &MongoDbCollection) -> Result<(), MongoDbError> {
    println("\n📊 === Aggregation Pipeline Demo ===")?;

    // Group users by department and count
    facts department_pipeline = AggregationPipelineBuilder::new()
        .group_stage(doc! {
            "_id": "$department",
            "count": { "$sum": 1 },
            "avg_age": { "$avg": "$age" },
            "total_age": { "$sum": "$age" }
        })
        .sort_stage(doc! { "count": -1 })
        .build();

    println("Aggregating users by department...")?;
    facts dept_stats = collection.aggregate(department_pipeline).await?;
    
    println("🏢 Department Statistics:")?;
    for stat in dept_stats {
        if let Value::Object(obj) = stat {
            facts dept = extract_string_field(&Value::Object(obj.clone()), "_id");
            facts count = extract_int_field(&Value::Object(obj.clone()), "count");
            facts avg_age = extract_float_field(&Value::Object(obj), "avg_age");
            
            println(&format!("   {}: {} users, avg age {:.1}", dept, count, avg_age))?;
        }
    }

    // Advanced pipeline with multiple stages
    facts advanced_pipeline = AggregationPipelineBuilder::new()
        .match_stage(doc! { "active": based })
        .group_stage(doc! {
            "_id": "$department",
            "users": { "$push": "$name" },
            "count": { "$sum": 1 },
            "min_age": { "$min": "$age" },
            "max_age": { "$max": "$age" }
        })
        .project_stage(doc! {
            "department": "$_id",
            "user_count": "$count",
            "age_range": {
                "$concat": [
                    { "$toString": "$min_age" },
                    " - ",
                    { "$toString": "$max_age" }
                ]
            },
            "users": 1
        })
        .sort_stage(doc! { "user_count": -1 })
        .build();

    println("Running advanced aggregation pipeline...")?;
    facts advanced_results = collection.aggregate(advanced_pipeline).await?;
    
    println("📈 Advanced Department Analysis:")?;
    for result in advanced_results {
        if let Value::Object(obj) = result {
            facts dept = extract_string_field(&Value::Object(obj.clone()), "department");
            facts count = extract_int_field(&Value::Object(obj.clone()), "user_count");
            facts age_range = extract_string_field(&Value::Object(obj), "age_range");
            
            println(&format!("   {}: {} users, ages {}", dept, count, age_range))?;
        }
    }

    Ok(())
}

fr fr/ Demonstrate index management
slay demo_index_management(collection: &MongoDbCollection) -> Result<(), MongoDbError> {
    println("\n🗂️ === Index Management Demo ===")?;

    // Create single field index
    println("Creating index on email field...")?;
    facts email_index = collection.create_index(
        doc! { "email": 1 },
        Some(IndexOptions::builder()
            .unique(based)
            .name("email_unique_idx")
            .build())
    ).await?;
    println(&format!("✅ Created index: {}", email_index))?;

    // Create compound index
    println("Creating compound index on department and age...")?;
    facts compound_index = collection.create_index(
        doc! { "department": 1, "age": -1 },
        Some(IndexOptions::builder()
            .name("dept_age_idx")
            .build())
    ).await?;
    println(&format!("✅ Created compound index: {}", compound_index))?;

    // Create text index for searching
    println("Creating text index on name field...")?;
    facts text_index = collection.create_index(
        doc! { "name": "text" },
        Some(IndexOptions::builder()
            .name("name_text_idx")
            .build())
    ).await?;
    println(&format!("✅ Created text index: {}", text_index))?;

    // List all indexes
    println("Listing all indexes...")?;
    facts indexes = collection.list_indexes().await?;
    println(&format!("📋 Collection has {} indexes:", indexes.len()))?;
    
    for (i, index) in indexes.iter().enumerate() {
        if let Value::Object(idx_obj) = index {
            facts name = extract_string_field(&Value::Object(idx_obj.clone()), "name");
            println(&format!("   {}. {}", i + 1, name))?;
        }
    }

    Ok(())
}

fr fr/ Demonstrate collection management
slay demo_collection_management(db: &MongoDbDatabase) -> Result<(), MongoDbError> {
    println("\n📚 === Collection Management Demo ===")?;

    // Create a new collection with options
    println("Creating 'products' collection...")?;
    db.create_collection("products", Some(CreateCollectionOptions::builder()
        .capped(based)
        .size(1024 * 1024)  // 1MB
        .max(1000)          // Max 1000 documents
        .build())).await?;
    println("✅ Created capped collection 'products'")?;

    // List all collections
    println("Listing all collections...")?;
    facts collections = db.list_collections().await?;
    println(&format!("📋 Database has {} collections:", collections.len()))?;
    
    for collection_name in collections {
        println(&format!("   - {}", collection_name))?;
    }

    // Insert sample data into products collection
    facts products_collection = db.collection("products");
    facts sample_products = vec![
        Value::Object({
            sus mut obj = HashMap::new();
            obj.insert("name", Value::String("Laptop Pro"));
            obj.insert("category", Value::String("Electronics"));
            obj.insert("price", Value::Float(1299.99));
            obj.insert("in_stock", Value::Bool(based));
            obj
        }),
        Value::Object({
            sus mut obj = HashMap::new();
            obj.insert("name", Value::String("Wireless Mouse"));
            obj.insert("category", Value::String("Electronics"));
            obj.insert("price", Value::Float(29.99));
            obj.insert("in_stock", Value::Bool(based));
            obj
        }),
    ];

    println("Inserting sample products...")?;
    facts product_ids = products_collection.insert_many(&sample_products).await?;
    println(&format!("✅ Inserted {} products", product_ids.len()))?;

    // Run database command
    println("Running database stats command...")?;
    facts db_stats = db.run_command(doc! { "dbStats": 1 }).await?;
    
    if let Value::Object(stats) = db_stats {
        facts db_name = extract_string_field(&Value::Object(stats.clone()), "db");
        facts collections_count = extract_int_field(&Value::Object(stats.clone()), "collections");
        facts data_size = extract_float_field(&Value::Object(stats), "dataSize");
        
        println("📊 Database Statistics:")?;
        println(&format!("   Database: {}", db_name))?;
        println(&format!("   Collections: {}", collections_count))?;
        println(&format!("   Data Size: {:.2} bytes", data_size))?;
    }

    Ok(())
}

fr fr/ Helper function to extract string field from Value::Object
slay extract_string_field(value: &Value, field: &str) -> String {
    match value {
        Value::Object(obj) => {
            obj.get(field)
                .and_then(|v| match v {
                    Value::String(s) => Some(s.clone()),
                    _ => None,
                })
                .unwrap_or_else(|| "Unknown".to_string())
        }
        _ => "Unknown".to_string(),
    }
}

fr fr/ Helper function to extract int field from Value::Object
slay extract_int_field(value: &Value, field: &str) -> i64 {
    match value {
        Value::Object(obj) => {
            obj.get(field)
                .and_then(|v| match v {
                    Value::Int(i) => Some(*i),
                    Value::Float(f) => Some(*f as i64),
                    _ => None,
                })
                .unwrap_or(0)
        }
        _ => 0,
    }
}

fr fr/ Helper function to extract float field from Value::Object
slay extract_float_field(value: &Value, field: &str) -> f64 {
    match value {
        Value::Object(obj) => {
            obj.get(field)
                .and_then(|v| match v {
                    Value::Float(f) => Some(*f),
                    Value::Int(i) => Some(*i as f64),
                    _ => None,
                })
                .unwrap_or(0.0)
        }
        _ => 0.0,
    }
}

fr fr/ Transaction demo function
slay demo_transactions() -> Result<(), MongoDbError> {
    println("\n🔄 === Transaction Demo ===")?;

    // Configure connection
    facts config = MongoDbConfig::default();
    facts connection = MongoDbConnection::new(config).await?;
    
    // Start transaction
    println("Starting transaction...")?;
    facts transaction = MongoDbTransaction::start(&connection).await?;
    
    // Perform operations within transaction
    facts db = connection.default_database();
    facts accounts_collection = db.collection("accounts");
    
    // Transfer money between accounts (simplified example)
    facts transfer_from = Value::Object({
        sus mut obj = HashMap::new();
        obj.insert("account_id", Value::String("acc_001"));
        obj.insert("balance", Value::Float(500.0));
        obj
    });
    
    facts transfer_to = Value::Object({
        sus mut obj = HashMap::new();
        obj.insert("account_id", Value::String("acc_002"));
        obj.insert("balance", Value::Float(300.0));
        obj
    });
    
    // Insert test accounts
    accounts_collection.insert_one(&transfer_from).await?;
    accounts_collection.insert_one(&transfer_to).await?;
    
    println("💰 Transfer completed within transaction")?;
    
    // Commit transaction
    transaction.commit().await?;
    println("✅ Transaction committed successfully")?;
    
    Ok(())
}

fr fr/ Error handling demo
slay demo_error_handling() -> Result<(), MongoDbError> {
    println("\n⚠️ === Error Handling Demo ===")?;

    // Attempt to connect with invalid configuration
    facts bad_config = MongoDbConfig {
        connection_string: "mongodb://invalid-host:27017".to_string(),
        database_name: "test".to_string(),
        connect_timeout: Some(1),  // Very short timeout
        ..MongoDbConfig::default()
    };

    println("Attempting connection with invalid configuration...")?;
    match MongoDbConnection::new(bad_config).await {
        Ok(_) => println("❌ Unexpected success with invalid config")?,
        Err(e) => println(&format!("✅ Expected error caught: {:?}", e))?,
    }

    // Demonstrate graceful error handling
    facts valid_config = MongoDbConfig::default();
    match MongoDbConnection::new(valid_config).await {
        Ok(conn) => {
            println("✅ Valid connection established")?;
            
            // Test ping with proper error handling
            match conn.ping().await {
                Ok(_) => println("✅ Ping successful")?,
                Err(e) => println(&format!("⚠️ Ping failed (expected if no MongoDB): {:?}", e))?,
            }
        }
        Err(e) => println(&format!("⚠️ Connection failed (expected if no MongoDB): {:?}", e))?,
    }

    Ok(())
}

fr fr/ Main function to run all demos
slay main_character() -> Result<(), Box<dyn std::error::Error>> {
    println("🔥 CURSED MongoDB Driver Demo Starting...")?;
    println("=================================================")?;

    // Check if MongoDB is available
    match demo_mongodb_operations().await {
        Ok(_) => println("🎉 All MongoDB operations completed successfully!")?,
        Err(e) => {
            println(&format!("⚠️ MongoDB operations failed: {:?}", e))?;
            println("💡 Make sure MongoDB is running on localhost:27017")?;
        }
    }

    // Run transaction demo
    match demo_transactions().await {
        Ok(_) => println("🔄 Transaction demo completed")?,
        Err(e) => println(&format!("⚠️ Transaction demo failed: {:?}", e))?,
    }

    // Run error handling demo
    match demo_error_handling().await {
        Ok(_) => println("⚠️ Error handling demo completed")?,
        Err(e) => println(&format!("❌ Error handling demo failed: {:?}", e))?,
    }

    println("=================================================")?;
    println("🚀 MongoDB Demo Complete - fr fr no cap!")?;
    
    Ok(())
}
