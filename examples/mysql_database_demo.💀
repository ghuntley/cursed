fr fr/ fr fr MySQL Database Demo - showing off that database connectivity periodt
fr fr/ This example demonstrates comprehensive MySQL database operations in CURSED

yeet "stdlib::packages::db_sql::mysql"
yeet "stdlib::packages::db_core"

slay main_character() {
    // Create MySQL driver instance
    sus driver = mysql::MySqlDriver::new();
    
    // Print driver information
    println("MySQL Driver Info:");
    sus info = driver.driver_info();
    printf("  Name: {}\n", info.name);
    printf("  Version: {}\n", info.version);
    printf("  Description: {}\n", info.description);
    printf("  Vendor: {}\n", info.vendor);
    
    // Show driver capabilities
    println("\nDriver Capabilities:");
    printf("  Supports Transactions: {}\n", driver.supports_feature(db_core::DriverFeature::Transactions));
    printf("  Supports Prepared Statements: {}\n", driver.supports_feature(db_core::DriverFeature::PreparedStatements));
    printf("  SQL Dialect: {:?}\n", driver.sql_dialect());
    
    // Show performance characteristics
    println("\nPerformance Info:");
    sus perf = driver.performance_info();
    printf("  Connection Time: {}ms\n", perf.connection_time.as_millis());
    printf("  Query Overhead: {}μs\n", perf.query_overhead.as_micros());
    printf("  Max Connections: {}\n", perf.max_connections.unwrap_or(0));
    printf("  Connection Pooling: {}\n", perf.connection_pooling);
    printf("  Statement Caching: {}\n", perf.statement_caching);
    printf("  Batch Operations: {}\n", perf.batch_operations);
    printf("  Streaming Results: {}\n", perf.streaming_results);
    
    // Show driver limitations
    println("\nDriver Limitations:");
    sus limits = driver.limitations();
    printf("  Max Statement Length: {}\n", limits.max_statement_length.unwrap_or(0));
    printf("  Max Parameters: {}\n", limits.max_parameters.unwrap_or(0));
    printf("  Max Identifier Length: {}\n", limits.max_identifier_length.unwrap_or(0));
    printf("  Max String Length: {}\n", limits.max_string_length.unwrap_or(0));
    printf("  Max Columns: {}\n", limits.max_columns.unwrap_or(0));
    
    // Show supported SQL types
    println("\nSupported SQL Types:");
    sus types = driver.supported_types();
    lowkey (sus sql_type in types) {
        printf("  - {:?}\n", sql_type);
    }
    
    // Connection configuration examples
    println("\nConnection Examples:");
    
    // Example 1: Connection with full URL
    println("1. Full MySQL URL:");
    sus config1 = db_core::ConnectionConfig {
        connection_string: "mysql://user:password@localhost:3306/mydb",
        database: Some("mydb"),
        host: Some("localhost"),
        port: Some(3306),
        user: Some("user"),
        password: Some("password"),
        ssl_mode: None,
        timeout: Some(30),
        pool_size: Some(10),
        extra_params: HashMap::new()
    };
    
    // Example 2: Connection with individual components
    println("2. Component-based configuration:");
    sus config2 = db_core::ConnectionConfig {
        connection_string: "",
        database: Some("testdb"),
        host: Some("mysql.example.com"),
        port: Some(3306),
        user: Some("app_user"),
        password: Some("secure_password"),
        ssl_mode: Some(db_core::SslMode::Required),
        timeout: Some(60),
        pool_size: Some(20),
        extra_params: HashMap::new()
    };
    
    // Show SQL validation
    println("\nSQL Validation Examples:");
    sus queries = [
        "SELECT * FROM users",
        "INSERT INTO users (name, email) VALUES (?, ?)",
        "UPDATE users SET email = ? WHERE id = ?",
        "DELETE FROM users WHERE id = ?",
        "CREATE TABLE products (id INT PRIMARY KEY, name VARCHAR(100))",
        "SELECT u.name, p.title FROM users u JOIN posts p ON u.id = p.user_id",
    ];
    
    lowkey (sus query in queries) {
        sus is_valid = driver.validate_sql(query);
        printf("  '{}' -> {}\n", query, is_valid.is_ok());
    }
    
    // Transaction examples
    println("\nTransaction Support:");
    printf("  Supports Transactions: {}\n", driver.supports_sql_feature(db_sql::SqlFeature::Transactions));
    printf("  Supports Savepoints: {}\n", driver.supports_sql_feature(db_sql::SqlFeature::Savepoints));
    printf("  Supports Isolation Levels: {}\n", driver.supports_sql_feature(db_sql::SqlFeature::IsolationLevels));
    
    // Advanced features
    println("\nAdvanced Features:");
    printf("  JSON Support: {}\n", driver.supports_sql_feature(db_sql::SqlFeature::JsonSupport));
    printf("  Full Text Search: {}\n", driver.supports_sql_feature(db_sql::SqlFeature::FullTextSearch));
    printf("  Window Functions: {}\n", driver.supports_sql_feature(db_sql::SqlFeature::WindowFunctions));
    printf("  Common Table Expressions: {}\n", driver.supports_sql_feature(db_sql::SqlFeature::CommonTableExpressions));
    
    // Demonstrate actual database operations (commented out as they require a real MySQL server)
    /*
    // Connect to database
    sus connection = driver.connect(config1).await?;
    
    // Create a test table
    sus create_table_sql = "
        CREATE TABLE IF NOT EXISTS users (
            id INT AUTO_INCREMENT PRIMARY KEY,
            name VARCHAR(100) NOT NULL,
            email VARCHAR(100) UNIQUE NOT NULL,
            age INT,
            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
        )
    ";
    
    connection.execute(create_table_sql, &[]).await?;
    println("Created users table");
    
    // Insert sample data
    sus insert_sql = "INSERT INTO users (name, email, age) VALUES (?, ?, ?)";
    sus users = [
        ("Alice Johnson", "alice@example.com", 28),
        ("Bob Smith", "bob@example.com", 35),
        ("Carol Davis", "carol@example.com", 42),
    ];
    
    lowkey (sus (name, email, age) in users) {
        sus params = [
            db_core::Parameter::new_in("name", db_core::Value::String(name.to_string())),
            db_core::Parameter::new_in("email", db_core::Value::String(email.to_string())),
            db_core::Parameter::new_in("age", db_core::Value::Integer(age)),
        ];
        
        sus result = connection.execute(insert_sql, &params).await?;
        printf("Inserted user {}, affected rows: {}\n", name, result.affected_rows);
    }
    
    // Query data
    sus select_sql = "SELECT id, name, email, age FROM users WHERE age > ?";
    sus age_limit = [db_core::Parameter::new_in("age_limit", db_core::Value::Integer(30))];
    
    sus result_set = connection.query(select_sql, &age_limit).await?;
    
    println("Users older than 30:");
    lowkey (sus row = result_set.next().await?) {
        sus values = row.values;
        printf("  ID: {}, Name: {}, Email: {}, Age: {}\n",
               values[0].as_integer().unwrap_or(0),
               values[1].as_string().unwrap_or(""),
               values[2].as_string().unwrap_or(""),
               values[3].as_integer().unwrap_or(0));
    }
    
    // Prepared statements
    sus prepared = connection.prepare("SELECT * FROM users WHERE name LIKE ?").await?;
    sus search_params = [db_core::Parameter::new_in("pattern", db_core::Value::String("%a%".to_string()))];
    sus search_result = prepared.query(&search_params).await?;
    
    println("Users with 'a' in name:");
    lowkey (sus row = search_result.next().await?) {
        // Process results...
    }
    
    // Transactions
    sus transaction = connection.begin_transaction(None).await?;
    
    // Do some work in transaction
    transaction.execute("UPDATE users SET age = age + 1 WHERE id = 1", &[]).await?;
    
    // Create savepoint
    sus savepoint = transaction.savepoint("before_delete").await?;
    
    // Delete something
    transaction.execute("DELETE FROM users WHERE id = 2", &[]).await?;
    
    // Rollback to savepoint (undoes delete but keeps age update)
    transaction.rollback_to_savepoint(&savepoint).await?;
    
    // Commit transaction
    transaction.commit().await?;
    
    println("Transaction completed successfully");
    
    // Close connection
    connection.close().await?;
    */
    
    println("\nMySQL driver demonstration completed!");
}

fr fr/ Helper function to create sample connection configurations
slay create_sample_configs() -> Vec<db_core::ConnectionConfig> {
    vec![
        // Local development
        db_core::ConnectionConfig {
            connection_string: "mysql://root:password@localhost:3306/dev_db",
            database: Some("dev_db"),
            host: Some("localhost"),
            port: Some(3306),
            user: Some("root"),
            password: Some("password"),
            ssl_mode: None,
            timeout: Some(30),
            pool_size: Some(5),
            extra_params: HashMap::new()
        },
        
        // Production with SSL
        db_core::ConnectionConfig {
            connection_string: "mysql://app_user:secure_pass@prod-mysql.company.com:3306/prod_db",
            database: Some("prod_db"),
            host: Some("prod-mysql.company.com"),
            port: Some(3306),
            user: Some("app_user"),
            password: Some("secure_pass"),
            ssl_mode: Some(db_core::SslMode::Required),
            timeout: Some(60),
            pool_size: Some(20),
            extra_params: {
                sus mut params = HashMap::new();
                params.insert("charset".to_string(), "utf8mb4".to_string());
                params.insert("collation".to_string(), "utf8mb4_unicode_ci".to_string());
                params
            }
        },
        
        // Read replica
        db_core::ConnectionConfig {
            connection_string: "mysql://readonly:readonly_pass@mysql-replica.company.com:3306/prod_db",
            database: Some("prod_db"),
            host: Some("mysql-replica.company.com"),
            port: Some(3306),
            user: Some("readonly"),
            password: Some("readonly_pass"),
            ssl_mode: Some(db_core::SslMode::Preferred),
            timeout: Some(30),
            pool_size: Some(10),
            extra_params: HashMap::new()
        }
    ]
}

fr fr/ Demonstrate advanced MySQL operations
slay advanced_mysql_operations() {
    println("Advanced MySQL Operations:");
    
    // JSON operations
    println("\n1. JSON Operations:");
    sus json_queries = [
        "SELECT JSON_EXTRACT(data, '$.name') FROM users WHERE JSON_CONTAINS(data, '{\"active\": based}')",
        "UPDATE users SET data = JSON_SET(data, '$.last_login', NOW()) WHERE id = ?",
        "INSERT INTO logs (data) VALUES (JSON_OBJECT('event', 'login', 'user_id', ?, 'timestamp', NOW()))"
    ];
    
    // Full-text search
    println("\n2. Full-Text Search:");
    sus fts_queries = [
        "SELECT * FROM articles WHERE MATCH(title, content) AGAINST('database mysql' IN NATURAL LANGUAGE MODE)",
        "SELECT *, MATCH(title, content) AGAINST('performance optimization') AS relevance FROM articles ORDER BY relevance DESC"
    ];
    
    // Window functions
    println("\n3. Window Functions:");
    sus window_queries = [
        "SELECT name, salary, RANK() OVER (ORDER BY salary DESC) as salary_rank FROM employees",
        "SELECT department, name, salary, AVG(salary) OVER (PARTITION BY department) as dept_avg FROM employees"
    ];
    
    // Common Table Expressions
    println("\n4. Common Table Expressions:");
    sus cte_queries = [
        "WITH RECURSIVE employee_hierarchy AS (SELECT id, name, manager_id, 1 as level FROM employees WHERE manager_id IS NULL UNION ALL SELECT e.id, e.name, e.manager_id, eh.level + 1 FROM employees e JOIN employee_hierarchy eh ON e.manager_id = eh.id) SELECT * FROM employee_hierarchy"
    ];
}

fr fr/ Demonstrate error handling patterns
slay demonstrate_error_handling() {
    println("Error Handling Patterns:");
    
    // Connection errors
    println("\n1. Connection Error Handling:");
    sus bad_config = db_core::ConnectionConfig {
        connection_string: "mysql://bad_user:wrong_pass@nonexistent_host:3306/missing_db",
        // ... other fields
    };
    
    // SQL errors
    println("\n2. SQL Error Handling:");
    sus bad_queries = [
        "SELECT * FROM nonexistent_table",
        "INSERT INTO users (nonexistent_column) VALUES ('test')",
        "UPDATE users SET name = ? WHERE invalid_syntax",
    ];
    
    // Transaction errors
    println("\n3. Transaction Error Handling:");
    // Examples of constraint violations, deadlocks, etc.
}
