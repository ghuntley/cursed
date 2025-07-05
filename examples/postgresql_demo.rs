/// PostgreSQL Driver Demo - Comprehensive Database Operations Example
/// 
/// This example demonstrates the full capabilities of the CURSED PostgreSQL driver:
/// - Connection establishment and configuration
/// - Basic CRUD operations (Create, Read, Update, Delete)
/// - Advanced query operations with parameters
/// - Transaction management with different isolation levels
/// - Prepared statements for performance and security
/// - Batch operations for efficiency
/// - Error handling and recovery patterns
/// - Connection pooling for scalability
/// 
/// Database operations are critical for modern applications because:
/// - Data persistence ensures information survives application restarts
/// - ACID transactions maintain data consistency and integrity
/// - Concurrent access requires proper isolation and locking
/// - Performance optimization affects user experience directly
/// - Security measures protect sensitive information from breaches
/// - Proper error handling prevents data corruption and system failures

use cursed::stdlib::packages::db_sql::postgresql::PostgreSqlDriver;
use cursed::stdlib::packages::db_core::{ConnectionConfig, DatabaseDriver};
use cursed::stdlib::packages::db_sql::{SqlDriver, SqlValue};
use cursed::stdlib::packages::db_sql::drivers::{SqlTransactionIsolation, SqlBatch};
use std::error::Error;
use tracing::{info, error, debug, instrument};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Initialize tracing for observability
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    info!("🚀 Starting PostgreSQL Driver Demo");

    // Example 1: Basic Driver Setup and Configuration
    demonstrate_driver_setup().await?;

    // Example 2: Connection Management
    demonstrate_connection_management().await?;

    // Example 3: Basic Query Operations
    demonstrate_basic_queries().await?;

    // Example 4: Prepared Statements
    demonstrate_prepared_statements().await?;

    // Example 5: Transaction Management
    demonstrate_transactions().await?;

    // Example 6: Batch Operations
    demonstrate_batch_operations().await?;

    // Example 7: Advanced Features
    demonstrate_advanced_features().await?;

    // Example 8: Error Handling Patterns
    demonstrate_error_handling().await?;

    info!("✅ PostgreSQL Driver Demo completed successfully!");
    Ok(())
}

#[instrument]
async fn demonstrate_driver_setup() -> Result<(), Box<dyn Error>> {
    info!("📋 Demonstrating Driver Setup and Configuration");

    // Create PostgreSQL driver instance
    let driver = PostgreSqlDriver::new();
    
    // Display driver information
    let info = driver.driver_info();
    info!("Driver: {} v{}", info.name(), info.version());
    info!("Description: {}", info.description());
    info!("Author: {}", info.author());

    // Check feature support
    info!("🔧 Supported Features:");
    let features = [
        ("Transactions", cursed::stdlib::packages::db_core::DriverFeature::Transactions),
        ("Prepared Statements", cursed::stdlib::packages::db_core::DriverFeature::PreparedStatements),
        ("Connection Pooling", cursed::stdlib::packages::db_core::DriverFeature::ConnectionPooling),
        ("Async Operations", cursed::stdlib::packages::db_core::DriverFeature::AsyncOperations),
        ("Streaming", cursed::stdlib::packages::db_core::DriverFeature::Streaming),
        ("Batching", cursed::stdlib::packages::db_core::DriverFeature::Batching),
        ("Encryption", cursed::stdlib::packages::db_core::DriverFeature::Encryption),
    ];

    for (name, feature) in features {
        let supported = driver.supports_feature(feature);
        info!("  {} {}", if supported { "✅" } else { "❌" }, name);
    }

    // Display supported SQL types
    let types = driver.supported_types();
    info!("📊 Supported SQL Types ({} total):", types.len());
    for sql_type in types.iter().take(10) { // Show first 10
        debug!("  - {:?}", sql_type);
    }

    // Show configuration options
    let config_options = driver.configuration_options();
    info!("⚙️  Configuration Options ({} available):", config_options.len());
    for option in &config_options {
        info!("  - {}: {} (default: {:?})", 
              option.name, 
              option.description,
              option.default_value);
    }

    // Display performance characteristics
    let perf_info = driver.performance_info();
    info!("🏃 Performance Info:");
    info!("  - Connection time: {:?}", perf_info.connection_time);
    info!("  - Query overhead: {:?}", perf_info.query_overhead);
    info!("  - Max connections: {:?}", perf_info.max_connections);
    info!("  - Connection pooling: {}", perf_info.connection_pooling);
    info!("  - Statement caching: {}", perf_info.statement_caching);

    // Display limitations
    let limitations = driver.limitations();
    info!("📏 Driver Limitations:");
    info!("  - Max statement length: {:?} bytes", limitations.max_statement_length);
    info!("  - Max parameters: {:?}", limitations.max_parameters);
    info!("  - Max identifier length: {:?} chars", limitations.max_identifier_length);
    info!("  - Max columns: {:?}", limitations.max_columns);

    Ok(())
}

#[instrument]
async fn demonstrate_connection_management() -> Result<(), Box<dyn Error>> {
    info!("🔗 Demonstrating Connection Management");

    let driver = PostgreSqlDriver::new();

    // Example connection configurations
    let configs = vec![
        ("Development", ConnectionConfig {
            host: Some("localhost".to_string()),
            port: Some(5432),
            database: Some("cursed_dev".to_string()),
            username: Some("cursed_user".to_string()),
            password: Some("dev_password".to_string()),
            max_connections: Some(5),
            connection_timeout: Some(std::time::Duration::from_secs(30)),
            idle_timeout: Some(std::time::Duration::from_secs(300)),
            ..Default::default()
        }),
        ("Production", ConnectionConfig {
            host: Some("prod-db.example.com".to_string()),
            port: Some(5432),
            database: Some("cursed_prod".to_string()),
            username: Some("cursed_app".to_string()),
            password: Some("secure_production_password".to_string()),
            max_connections: Some(50),
            connection_timeout: Some(std::time::Duration::from_secs(10)),
            idle_timeout: Some(std::time::Duration::from_secs(600)),
            ssl_mode: Some("require".to_string()),
            ..Default::default()
        }),
    ];

    for (env_name, config) in configs {
        info!("🌍 {} Environment Configuration:", env_name);
        info!("  - Host: {}", config.host.as_ref().unwrap_or(&"localhost".to_string()));
        info!("  - Port: {}", config.port.unwrap_or(5432));
        info!("  - Database: {}", config.database.as_ref().unwrap_or(&"unknown".to_string()));
        info!("  - Max Connections: {:?}", config.max_connections);
        info!("  - Connection Timeout: {:?}", config.connection_timeout);
        
        // Validate connection string (without actually connecting)
        let connection_string = format!("postgresql://{}:***@{}:{}/{}",
                                      config.username.as_ref().unwrap_or(&"user".to_string()),
                                      config.host.as_ref().unwrap_or(&"localhost".to_string()),
                                      config.port.unwrap_or(5432),
                                      config.database.as_ref().unwrap_or(&"db".to_string()));
        
        match driver.validate_connection_string(&connection_string) {
            Ok(()) => info!("  ✅ Connection string format valid"),
            Err(e) => info!("  ❌ Connection string validation failed: {}", e),
        }
    }

    // Demonstrate connection pooling setup
    info!("🏊 Connection Pooling Benefits:");
    info!("  - Reduces connection overhead by reusing connections");
    info!("  - Limits concurrent connections to prevent resource exhaustion");
    info!("  - Automatically handles connection lifecycle management");
    info!("  - Provides connection health monitoring and recovery");

    Ok(())
}

#[instrument]
async fn demonstrate_basic_queries() -> Result<(), Box<dyn Error>> {
    info!("📊 Demonstrating Basic Query Operations");

    // Example SQL queries demonstrating different patterns
    let example_queries = vec![
        ("Simple Select", "SELECT id, name, email FROM users WHERE active = true"),
        ("Parameterized Query", "SELECT * FROM products WHERE category = $1 AND price > $2"),
        ("Join Query", "SELECT u.name, p.title FROM users u JOIN posts p ON u.id = p.author_id"),
        ("Aggregate Query", "SELECT category, COUNT(*), AVG(price) FROM products GROUP BY category"),
        ("Window Function", "SELECT name, salary, RANK() OVER (ORDER BY salary DESC) FROM employees"),
        ("Common Table Expression", "WITH recent_orders AS (SELECT * FROM orders WHERE created_at > NOW() - INTERVAL '7 days') SELECT COUNT(*) FROM recent_orders"),
        ("JSON Query", "SELECT data->>'name' as name FROM users WHERE data ? 'preferences'"),
        ("Array Query", "SELECT * FROM posts WHERE tags && ARRAY['postgresql', 'database']"),
        ("Full Text Search", "SELECT * FROM articles WHERE to_tsvector('english', content) @@ plainto_tsquery('postgresql')"),
        ("Upsert (INSERT ON CONFLICT)", "INSERT INTO counters (key, value) VALUES ($1, $2) ON CONFLICT (key) DO UPDATE SET value = counters.value + $2"),
    ];

    info!("🔍 SQL Query Examples ({} patterns):", example_queries.len());
    for (description, sql) in example_queries {
        info!("  📝 {}: {}", description, sql);
        
        // Validate SQL (basic validation)
        let driver = PostgreSqlDriver::new();
        match driver.validate_sql(sql) {
            Ok(()) => debug!("     ✅ SQL syntax appears valid"),
            Err(e) => debug!("     ⚠️  SQL validation warning: {}", e),
        }
    }

    // Example parameter binding
    info!("🔗 Parameter Binding Examples:");
    let parameter_examples = vec![
        ("String Parameter", SqlValue::String("John Doe".to_string())),
        ("Integer Parameter", SqlValue::Integer(42)),
        ("Float Parameter", SqlValue::Float(99.99)),
        ("Boolean Parameter", SqlValue::Boolean(true)),
        ("UUID Parameter", SqlValue::String(uuid::Uuid::new_v4().to_string())),
        ("JSON Parameter", SqlValue::Json(serde_json::json!({"status": "active", "preferences": {"theme": "dark"}}).to_string())),
        ("Null Parameter", SqlValue::Null),
    ];

    for (description, value) in parameter_examples {
        info!("  - {}: {:?}", description, value);
    }

    Ok(())
}

#[instrument]
async fn demonstrate_prepared_statements() -> Result<(), Box<dyn Error>> {
    info!("🎯 Demonstrating Prepared Statements");

    info!("📊 Prepared Statement Benefits:");
    info!("  - Performance: Query parsing and planning done once");
    info!("  - Security: Prevents SQL injection attacks");
    info!("  - Type Safety: Parameter types are validated");
    info!("  - Memory Efficiency: Plans cached for reuse");

    // Example prepared statements
    let prepared_examples = vec![
        ("User Lookup", "SELECT id, name, email, created_at FROM users WHERE email = $1", vec!["email"]),
        ("Product Search", "SELECT * FROM products WHERE name ILIKE $1 AND price BETWEEN $2 AND $3", vec!["search_term", "min_price", "max_price"]),
        ("User Insert", "INSERT INTO users (name, email, password_hash) VALUES ($1, $2, $3) RETURNING id", vec!["name", "email", "password_hash"]),
        ("Order Update", "UPDATE orders SET status = $1, updated_at = NOW() WHERE id = $2", vec!["status", "order_id"]),
        ("Analytics Query", "SELECT DATE(created_at) as date, COUNT(*) as count FROM events WHERE event_type = $1 AND created_at >= $2 GROUP BY DATE(created_at) ORDER BY date", vec!["event_type", "start_date"]),
    ];

    for (name, sql, params) in prepared_examples {
        info!("  📝 {}:", name);
        info!("     SQL: {}", sql);
        info!("     Parameters: [{}]", params.join(", "));
        
        // Count parameters in SQL
        let param_count = sql.matches('$').count();
        info!("     Expected {} parameters", param_count);
    }

    // Demonstrate parameter type mapping
    info!("🔄 Parameter Type Mapping:");
    info!("  - CURSED String → PostgreSQL TEXT/VARCHAR");
    info!("  - CURSED Integer → PostgreSQL INT4/INT8");
    info!("  - CURSED Float → PostgreSQL FLOAT4/FLOAT8");
    info!("  - CURSED Boolean → PostgreSQL BOOL");
    info!("  - CURSED UUID → PostgreSQL UUID");
    info!("  - CURSED JSON → PostgreSQL JSON/JSONB");
    info!("  - CURSED Null → PostgreSQL NULL");

    Ok(())
}

#[instrument]
async fn demonstrate_transactions() -> Result<(), Box<dyn Error>> {
    info!("💳 Demonstrating Transaction Management");

    info!("🛡️  ACID Properties in PostgreSQL:");
    info!("  - Atomicity: All operations succeed or all fail");
    info!("  - Consistency: Database remains in valid state");
    info!("  - Isolation: Concurrent transactions don't interfere");
    info!("  - Durability: Committed changes survive system failures");

    // Isolation levels
    let isolation_levels = vec![
        (SqlTransactionIsolation::ReadUncommitted, "Read Uncommitted", "Fastest, allows dirty reads"),
        (SqlTransactionIsolation::ReadCommitted, "Read Committed", "Default, prevents dirty reads"),
        (SqlTransactionIsolation::RepeatableRead, "Repeatable Read", "Prevents non-repeatable reads"),
        (SqlTransactionIsolation::Serializable, "Serializable", "Strongest, prevents all anomalies"),
    ];

    info!("🔒 Transaction Isolation Levels:");
    for (level, name, description) in isolation_levels {
        info!("  - {:?}: {} - {}", level, name, description);
    }

    // Transaction examples
    let transaction_examples = vec![
        ("Bank Transfer", vec![
            "UPDATE accounts SET balance = balance - $1 WHERE account_id = $2",
            "UPDATE accounts SET balance = balance + $1 WHERE account_id = $3",
            "INSERT INTO transaction_log (from_account, to_account, amount, timestamp) VALUES ($2, $3, $1, NOW())",
        ]),
        ("Order Processing", vec![
            "INSERT INTO orders (customer_id, total_amount) VALUES ($1, $2)",
            "UPDATE inventory SET quantity = quantity - $3 WHERE product_id = $4", 
            "INSERT INTO order_items (order_id, product_id, quantity, price) VALUES (currval('orders_id_seq'), $4, $3, $5)",
        ]),
        ("User Registration", vec![
            "INSERT INTO users (email, password_hash, created_at) VALUES ($1, $2, NOW())",
            "INSERT INTO user_profiles (user_id, first_name, last_name) VALUES (currval('users_id_seq'), $3, $4)",
            "INSERT INTO user_settings (user_id, theme, notifications) VALUES (currval('users_id_seq'), 'default', true)",
        ]),
    ];

    for (scenario, statements) in transaction_examples {
        info!("  💼 {} Transaction:", scenario);
        for (i, stmt) in statements.iter().enumerate() {
            info!("     {}. {}", i + 1, stmt);
        }
        info!("     {} statements must all succeed or all rollback", statements.len());
    }

    info!("🔄 Transaction Lifecycle:");
    info!("  1. BEGIN - Start transaction");
    info!("  2. Execute multiple statements");
    info!("  3. COMMIT - Make changes permanent");
    info!("  4. ROLLBACK - Undo changes (if error occurs)");

    Ok(())
}

#[instrument]
async fn demonstrate_batch_operations() -> Result<(), Box<dyn Error>> {
    info!("📦 Demonstrating Batch Operations");

    info!("⚡ Batch Operation Benefits:");
    info!("  - Performance: Reduced network round-trips");
    info!("  - Atomicity: Multiple operations in single transaction");
    info!("  - Efficiency: Better resource utilization");
    info!("  - Consistency: Coordinated state changes");

    // Example batch operations
    let batch_scenarios = vec![
        ("Bulk Data Import", vec![
            SqlBatch {
                sql: "INSERT INTO products (name, category, price) VALUES ($1, $2, $3)".to_string(),
                parameters: vec![
                    SqlValue::String("Laptop".to_string()),
                    SqlValue::String("Electronics".to_string()),
                    SqlValue::Float(999.99),
                ],
            },
            SqlBatch {
                sql: "INSERT INTO products (name, category, price) VALUES ($1, $2, $3)".to_string(),
                parameters: vec![
                    SqlValue::String("Mouse".to_string()),
                    SqlValue::String("Electronics".to_string()),
                    SqlValue::Float(29.99),
                ],
            },
            SqlBatch {
                sql: "INSERT INTO products (name, category, price) VALUES ($1, $2, $3)".to_string(),
                parameters: vec![
                    SqlValue::String("Keyboard".to_string()),
                    SqlValue::String("Electronics".to_string()),
                    SqlValue::Float(79.99),
                ],
            },
        ]),
        ("Cache Refresh", vec![
            SqlBatch {
                sql: "DELETE FROM cache WHERE expires_at < NOW()".to_string(),
                parameters: vec![],
            },
            SqlBatch {
                sql: "UPDATE cache SET access_count = access_count + 1 WHERE key = $1".to_string(),
                parameters: vec![SqlValue::String("user_preferences".to_string())],
            },
            SqlBatch {
                sql: "INSERT INTO cache (key, value, expires_at) VALUES ($1, $2, $3) ON CONFLICT (key) DO UPDATE SET value = $2, expires_at = $3".to_string(),
                parameters: vec![
                    SqlValue::String("system_config".to_string()),
                    SqlValue::Json(serde_json::json!({"maintenance_mode": false}).to_string()),
                    SqlValue::String("2024-12-31 23:59:59".to_string()),
                ],
            },
        ]),
    ];

    for (scenario, batches) in batch_scenarios {
        info!("  📊 {} Batch ({} operations):", scenario, batches.len());
        for (i, batch) in batches.iter().enumerate() {
            info!("     {}. {} (with {} parameters)", 
                  i + 1, 
                  batch.sql,
                  batch.parameters.len());
        }
    }

    info!("🚀 Batch Processing Strategies:");
    info!("  - Small batches (100-1000 records): Better responsiveness");
    info!("  - Large batches (10k+ records): Better throughput");
    info!("  - Prepared statements: Optimal for repeated operations");
    info!("  - COPY command: Fastest for bulk data loading");

    Ok(())
}

#[instrument]
async fn demonstrate_advanced_features() -> Result<(), Box<dyn Error>> {
    info!("🚀 Demonstrating Advanced PostgreSQL Features");

    // JSON operations
    info!("📄 JSON/JSONB Operations:");
    let json_examples = vec![
        ("Extract JSON field", "SELECT data->>'name' FROM users WHERE id = $1"),
        ("JSON path query", "SELECT * FROM products WHERE specs @> '{\"color\": \"red\"}'"),
        ("JSON array contains", "SELECT * FROM posts WHERE tags ? 'postgresql'"),
        ("JSON aggregation", "SELECT jsonb_agg(data) FROM events WHERE date = $1"),
        ("Update JSON field", "UPDATE users SET preferences = preferences || '{\"theme\": \"dark\"}' WHERE id = $1"),
    ];

    for (description, sql) in json_examples {
        info!("  - {}: {}", description, sql);
    }

    // Array operations
    info!("📋 Array Operations:");
    let array_examples = vec![
        ("Array contains", "SELECT * FROM posts WHERE tags @> ARRAY['postgresql']"),
        ("Array overlap", "SELECT * FROM posts WHERE tags && ARRAY['database', 'sql']"),
        ("Array length", "SELECT title FROM posts WHERE array_length(tags, 1) > 3"),
        ("Unnest array", "SELECT unnest(tags) as tag FROM posts WHERE id = $1"),
        ("Array aggregation", "SELECT array_agg(DISTINCT category) FROM products"),
    ];

    for (description, sql) in array_examples {
        info!("  - {}: {}", description, sql);
    }

    // Window functions
    info!("🪟 Window Functions:");
    let window_examples = vec![
        ("Row numbering", "SELECT name, ROW_NUMBER() OVER (ORDER BY salary DESC) FROM employees"),
        ("Running total", "SELECT date, amount, SUM(amount) OVER (ORDER BY date) FROM transactions"),
        ("Percentile ranking", "SELECT name, salary, PERCENT_RANK() OVER (ORDER BY salary) FROM employees"),
        ("Lead/Lag", "SELECT date, value, LAG(value) OVER (ORDER BY date) as prev_value FROM metrics"),
        ("Partition ranking", "SELECT dept, name, RANK() OVER (PARTITION BY dept ORDER BY salary DESC) FROM employees"),
    ];

    for (description, sql) in window_examples {
        info!("  - {}: {}", description, sql);
    }

    // Full-text search
    info!("🔍 Full-Text Search:");
    let fts_examples = vec![
        ("Basic search", "SELECT * FROM documents WHERE to_tsvector('english', content) @@ plainto_tsquery('postgresql database')"),
        ("Phrase search", "SELECT * FROM documents WHERE to_tsvector(content) @@ phraseto_tsquery('advanced features')"),
        ("Ranking results", "SELECT *, ts_rank(to_tsvector(content), query) as rank FROM documents, plainto_tsquery('search term') query WHERE to_tsvector(content) @@ query ORDER BY rank DESC"),
        ("Headline generation", "SELECT ts_headline('english', content, plainto_tsquery('postgresql')) FROM documents"),
    ];

    for (description, sql) in fts_examples {
        info!("  - {}: {}", description, sql);
    }

    // Common Table Expressions (CTEs)
    info!("🔗 Common Table Expressions:");
    let cte_examples = vec![
        ("Recursive hierarchy", "WITH RECURSIVE hierarchy AS (SELECT id, name, parent_id FROM categories WHERE parent_id IS NULL UNION ALL SELECT c.id, c.name, c.parent_id FROM categories c JOIN hierarchy h ON c.parent_id = h.id) SELECT * FROM hierarchy"),
        ("Data aggregation", "WITH monthly_sales AS (SELECT DATE_TRUNC('month', date) as month, SUM(amount) as total FROM sales GROUP BY month) SELECT * FROM monthly_sales WHERE total > 10000"),
        ("Window with CTE", "WITH ranked_products AS (SELECT *, ROW_NUMBER() OVER (PARTITION BY category ORDER BY price DESC) as rank FROM products) SELECT * FROM ranked_products WHERE rank <= 3"),
    ];

    for (description, sql) in cte_examples {
        info!("  - {}", description);
        debug!("    SQL: {}", sql);
    }

    Ok(())
}

#[instrument]
async fn demonstrate_error_handling() -> Result<(), Box<dyn Error>> {
    info!("⚠️  Demonstrating Error Handling Patterns");

    info!("🛡️  Common Database Error Types:");
    let error_scenarios = vec![
        ("Connection Errors", vec![
            "Network timeout during connection",
            "Invalid credentials (authentication failure)",
            "Database server unavailable",
            "SSL/TLS handshake failure",
            "Connection pool exhausted",
        ]),
        ("Query Errors", vec![
            "Syntax errors in SQL statements",
            "Table or column does not exist", 
            "Type mismatch in parameters",
            "Invalid data format",
            "Query timeout exceeded",
        ]),
        ("Constraint Violations", vec![
            "Primary key violation (duplicate ID)",
            "Foreign key constraint violation",
            "NOT NULL constraint violation",
            "CHECK constraint violation",
            "UNIQUE constraint violation",
        ]),
        ("Transaction Errors", vec![
            "Deadlock detection and resolution",
            "Serialization failure in concurrent transactions",
            "Transaction timeout",
            "Savepoint rollback",
            "Read-only transaction violation",
        ]),
        ("Resource Errors", vec![
            "Disk space exhausted",
            "Memory allocation failure", 
            "Too many connections",
            "Lock timeout",
            "Statement complexity limit exceeded",
        ]),
    ];

    for (category, scenarios) in error_scenarios {
        info!("  🔴 {}:", category);
        for scenario in scenarios {
            info!("    - {}", scenario);
        }
    }

    info!("🔧 Error Recovery Strategies:");
    let recovery_strategies = vec![
        ("Retry Logic", "Implement exponential backoff for transient failures"),
        ("Circuit Breaker", "Fail fast when database is consistently unavailable"),
        ("Graceful Degradation", "Serve cached data when database is down"),
        ("Connection Pooling", "Maintain healthy connections and recover from failures"),
        ("Health Checks", "Monitor database connectivity and performance"),
        ("Alerting", "Notify operators of critical database issues"),
        ("Backup Plans", "Have read replicas and backup procedures ready"),
        ("Transaction Isolation", "Choose appropriate isolation levels for use case"),
    ];

    for (strategy, description) in recovery_strategies {
        info!("  ✅ {}: {}", strategy, description);
    }

    info!("📊 Error Monitoring Metrics:");
    let monitoring_metrics = vec![
        "Connection establishment time",
        "Query execution duration",
        "Error rate by error type",
        "Connection pool utilization",
        "Transaction rollback frequency",
        "Deadlock occurrence rate",
        "Resource utilization (CPU, memory, disk)",
        "Replication lag (if using replicas)",
    ];

    for metric in monitoring_metrics {
        info!("  📈 {}", metric);
    }

    Ok(())
}
