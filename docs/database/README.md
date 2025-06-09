# CURSED Database Connectivity - Complete Guide

> fr fr comprehensive database package documentation - your guide to database mastery periodt 🗄️

## Table of Contents

1. [Quick Start](#quick-start)
2. [Core Concepts](#core-concepts)
3. [Database Drivers](#database-drivers)
4. [Connection Management](#connection-management)
5. [Query Building](#query-building)
6. [Transaction Management](#transaction-management)
7. [Connection Pooling](#connection-pooling)
8. [ORM Features](#orm-features)
9. [Migrations](#migrations)
10. [Performance & Optimization](#performance--optimization)
11. [Error Handling](#error-handling)
12. [Testing](#testing)
13. [Best Practices](#best-practices)
14. [Troubleshooting](#troubleshooting)

## Quick Start

### Installation

Add the database packages to your `CursedPackage.toml`:

```toml
[dependencies]
db_core = { version = "1.0.0", features = ["async"] }
db_sql = { version = "1.0.0", features = ["sqlite", "postgres", "mysql"] }
db_pool = { version = "1.0.0" }
db_migrate = { version = "1.0.0" }
```

### Basic Usage

```cursed
// Import the database packages
use db_core::*;
use db_sql::*;

sus main() {
    // Connect to database
    let connection = sql_connect("sqlite", ":memory:")?;
    
    // Create a table
    connection.execute("CREATE TABLE users (
        id INTEGER PRIMARY KEY,
        name TEXT NOT NULL,
        email TEXT UNIQUE
    )", [])?;
    
    // Insert data
    connection.execute("INSERT INTO users (name, email) VALUES (?, ?)", [
        "Alice Johnson",
        "alice@example.com"
    ])?;
    
    // Query data
    let results = connection.query("SELECT * FROM users", [])?;
    periodt user in results.rows() {
        let name: tea = user.get("name")?;
        let email: tea = user.get("email")?;
        println!("{} - {}", name, email);
    }
    
    connection.close()?;
}
```

## Core Concepts

### Architecture Overview

The CURSED database system is built with a modular architecture:

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Application   │    │   ORM Layer     │    │   Migration     │
│     Code        │    │   (db_orm)      │    │   (db_migrate)  │
└─────────────────┘    └─────────────────┘    └─────────────────┘
         │                       │                       │
         └───────────────────────┼───────────────────────┘
                                 │
         ┌─────────────────────────────────────────────────┐
         │            Core Database Layer                  │
         │              (db_core)                          │
         └─────────────────────────────────────────────────┘
                                 │
         ┌─────────────────────────────────────────────────┐
         │           SQL Database Layer                    │
         │              (db_sql)                           │
         └─────────────────────────────────────────────────┘
                                 │
         ┌─────────────────────────────────────────────────┐
         │         Connection Pool Manager                 │
         │              (db_pool)                          │
         └─────────────────────────────────────────────────┘
                                 │
    ┌─────────────┐    ┌─────────────┐    ┌─────────────┐
    │   SQLite    │    │ PostgreSQL  │    │   MySQL     │
    │   Driver    │    │   Driver    │    │   Driver    │
    └─────────────┘    └─────────────┘    └─────────────┘
```

### Key Components

- **db_core**: Core interfaces and error handling
- **db_sql**: SQL query building and execution
- **db_pool**: Connection pooling and management
- **db_orm**: Object-relational mapping utilities
- **db_migrate**: Database schema migration system

## Database Drivers

### SQLite

Perfect for development and embedded applications:

```cursed
// In-memory database (fast, temporary)
let connection = sql_connect("sqlite", ":memory:")?;

// File-based database (persistent)
let connection = sql_connect("sqlite", "app_data.db")?;

// With custom configuration
let config = ConnectionConfig::new("sqlite", "app.db")
    .with_parameter("journal_mode", "WAL")
    .with_parameter("cache_size", "10000")
    .with_parameter("synchronous", "NORMAL");

let connection = sql_connect_with_config(config)?;
```

### PostgreSQL

Enterprise-grade relational database:

```cursed
// Basic connection
let connection = sql_connect("postgresql", "postgresql://user:pass@localhost/mydb")?;

// With full configuration
let config = ConnectionConfig::new("postgresql", "myapp")
    .with_host("db.example.com", 5432)
    .with_credentials("app_user", "secure_password")
    .with_ssl_mode("require")
    .with_parameter("application_name", "cursed_app")
    .with_parameter("connect_timeout", "30");

let connection = sql_connect_with_config(config)?;
```

### MySQL

Popular web application database:

```cursed
// MySQL connection
let connection = sql_connect("mysql", "mysql://user:pass@localhost/webapp")?;

// With configuration
let config = ConnectionConfig::new("mysql", "webapp")
    .with_host("mysql.example.com", 3306)
    .with_credentials("web_user", "password")
    .with_parameter("charset", "utf8mb4")
    .with_parameter("autocommit", "true");

let connection = sql_connect_with_config(config)?;
```

## Connection Management

### Basic Connection Lifecycle

```cursed
sus database_lifecycle_example() {
    // 1. Establish connection
    let mut connection = sql_connect("sqlite", ":memory:")?;
    
    // 2. Check connection status
    lowkey connection.is_connected() {
        println!("✅ Connected to {}", connection.driver_name());
    }
    
    // 3. Use connection for operations
    connection.execute("CREATE TABLE test (id INTEGER, name TEXT)", [])?;
    
    // 4. Gracefully close connection
    connection.close()?;
    
    // 5. Verify closure
    assert!(!connection.is_connected());
}
```

### Connection Configuration

```cursed
sus advanced_connection_config() {
    let config = ConnectionConfig::new("postgresql", "myapp")
        .with_host("localhost", 5432)
        .with_credentials("user", "password")
        .with_ssl_mode("prefer")
        .with_timeouts(
            Duration::from_secs(30),    // Connection timeout
            Duration::from_secs(3600)   // Query timeout
        )
        .with_retry_policy(3, Duration::from_secs(1))
        .with_parameter("search_path", "myapp,public");
    
    let connection = sql_connect_with_config(config)?;
    
    // Connection is ready with all settings applied
}
```

### Connection String Parsing

```cursed
sus connection_string_examples() {
    // PostgreSQL with all options
    let pg_config = ConnectionConfig::from_string(
        "postgresql://user:pass@db.example.com:5432/mydb?sslmode=require&connect_timeout=30"
    )?;
    
    // MySQL with charset and timezone
    let mysql_config = ConnectionConfig::from_string(
        "mysql://user:pass@localhost:3306/webapp?charset=utf8mb4&timezone=UTC"
    )?;
    
    // SQLite with journal mode
    let sqlite_config = ConnectionConfig::from_string(
        "sqlite:///path/to/database.db?journal_mode=WAL&cache_size=10000"
    )?;
}
```

## Query Building

### Raw SQL Queries

```cursed
sus raw_sql_examples() {
    let connection = sql_connect("sqlite", ":memory:")?;
    
    // Simple query
    let results = connection.query("SELECT * FROM users WHERE age > ?", [25])?;
    
    // Query with multiple parameters
    let results = connection.query(
        "SELECT * FROM users WHERE age BETWEEN ? AND ? AND active = ?",
        [18, 65, true]
    )?;
    
    // Complex query
    let results = connection.query("
        SELECT u.name, COUNT(p.id) as post_count
        FROM users u
        LEFT JOIN posts p ON u.id = p.user_id
        WHERE u.created_at > ?
        GROUP BY u.id, u.name
        HAVING COUNT(p.id) > ?
        ORDER BY post_count DESC
        LIMIT ?
    ", ["2024-01-01", 5, 10])?;
}
```

### Query Builder

The query builder provides a fluent, type-safe way to construct SQL:

```cursed
sus query_builder_examples() {
    let mut builder = SqlQueryBuilder::new();
    
    // SELECT query
    let sql = builder.select()
        .distinct()
        .columns(&["id", "name", "email", "created_at"])
        .from("users")
        .inner_join("profiles", "users.id = profiles.user_id")
        .where_eq("active", true)
        .where_clause("age >= ?")
        .where_in("role", ["admin", "moderator"])
        .group_by(&["id", "name"])
        .having("COUNT(*) > 1")
        .order_by("name", OrderDirection::Asc)
        .order_by("created_at", OrderDirection::Desc)
        .limit(50)
        .offset(100)
        .build()?;
    
    builder.add_parameter(SqlValue::Integer(18)); // For age >= ?
    
    let results = connection.query(&sql, builder.parameters().clone())?;
    
    // INSERT query
    builder.clear();
    let insert_sql = builder.insert()
        .into("users")
        .columns(&["name", "email", "age"])
        .values(vec![
            SqlValue::Text("John Doe".to_string()),
            SqlValue::Text("john@example.com".to_string()),
            SqlValue::Integer(30)
        ])
        .on_conflict("email")
        .do_update(&[("name", "EXCLUDED.name")])
        .build()?;
    
    connection.execute(&insert_sql, builder.parameters().clone())?;
    
    // UPDATE query
    builder.clear();
    let update_sql = builder.update()
        .table("users")
        .set("last_login", SqlValue::Text("NOW()".to_string()))
        .set("login_count", SqlValue::Text("login_count + 1".to_string()))
        .where_eq("id", SqlValue::Integer(123))
        .build()?;
    
    connection.execute(&update_sql, builder.parameters().clone())?;
    
    // DELETE query
    builder.clear();
    let delete_sql = builder.delete()
        .from("users")
        .where_clause("last_login < ?")
        .where_eq("active", false)
        .limit(100)
        .build()?;
    
    builder.add_parameter(SqlValue::Text("2023-01-01".to_string()));
    connection.execute(&delete_sql, builder.parameters().clone())?;
}
```

### DDL Operations

```cursed
sus ddl_operations() {
    let mut builder = SqlQueryBuilder::new();
    
    // CREATE TABLE
    let create_sql = builder.create_table()
        .table("orders")
        .if_not_exists()
        .column("id", SqlType::Integer).primary_key().auto_increment().finish()
        .column("user_id", SqlType::Integer).not_null().finish()
        .column("product_id", SqlType::Integer).not_null().finish()
        .column("quantity", SqlType::Integer).default_value(SqlValue::Integer(1)).finish()
        .column("price", SqlType::Decimal).not_null().finish()
        .column("status", SqlType::Text).default_value(SqlValue::Text("pending".to_string())).finish()
        .column("created_at", SqlType::Timestamp).default_current_timestamp().finish()
        .constraint(TableConstraint::ForeignKey(
            "user_id".to_string(),
            "users".to_string(),
            "id".to_string()
        ))
        .constraint(TableConstraint::Check("quantity > 0".to_string()))
        .constraint(TableConstraint::Index(
            "idx_orders_user_created".to_string(),
            vec!["user_id".to_string(), "created_at".to_string()]
        ))
        .build()?;
    
    connection.execute(&create_sql, [])?;
    
    // ALTER TABLE
    builder.clear();
    let alter_sql = builder.alter_table()
        .table("orders")
        .add_column("notes", SqlType::Text)
        .add_column("discount", SqlType::Float)
        .add_constraint(TableConstraint::Check("discount >= 0 AND discount <= 1".to_string()))
        .build()?;
    
    connection.execute(&alter_sql, [])?;
    
    // CREATE INDEX
    builder.clear();
    let index_sql = builder.create_index()
        .index("idx_orders_status_created")
        .table("orders")
        .columns(&["status", "created_at"])
        .unique()
        .build()?;
    
    connection.execute(&index_sql, [])?;
}
```

## Transaction Management

### Basic Transactions

```cursed
sus basic_transaction_example() {
    let mut connection = sql_connect("sqlite", ":memory:")?;
    
    // Setup tables
    connection.execute("CREATE TABLE accounts (
        id INTEGER PRIMARY KEY,
        name TEXT,
        balance DECIMAL(10,2)
    )", [])?;
    
    connection.execute("INSERT INTO accounts (name, balance) VALUES (?, ?)", ["Alice", 1000.00])?;
    connection.execute("INSERT INTO accounts (name, balance) VALUES (?, ?)", ["Bob", 500.00])?;
    
    // Begin transaction
    let mut transaction = connection.begin_transaction()?;
    
    bestie {
        // Transfer $200 from Alice to Bob
        let amount = 200.00;
        
        // Debit Alice's account
        let debit_result = transaction.execute(
            "UPDATE accounts SET balance = balance - ? WHERE name = ?",
            [amount, "Alice"]
        )?;
        
        lowkey debit_result.rows_affected() != 1 {
            yolo DatabaseError::new("Failed to debit Alice's account");
        }
        
        // Credit Bob's account
        let credit_result = transaction.execute(
            "UPDATE accounts SET balance = balance + ? WHERE name = ?",
            [amount, "Bob"]
        )?;
        
        lowkey credit_result.rows_affected() != 1 {
            yolo DatabaseError::new("Failed to credit Bob's account");
        }
        
        // Commit the transaction
        transaction.commit()?;
        println!("✅ Transfer completed successfully");
        
    } flex error {
        // Rollback on any error
        transaction.rollback()?;
        println!("❌ Transfer failed: {}", error);
    }
}
```

### Advanced Transactions with Savepoints

```cursed
sus advanced_transaction_example() {
    let mut connection = sql_connect("postgresql", "postgresql://user:pass@localhost/mydb")?;
    
    let mut transaction = connection.begin_transaction()?;
    
    bestie {
        // Initial operations
        transaction.execute("UPDATE settings SET last_update = NOW()", [])?;
        
        // Create savepoint before risky operation
        let savepoint1 = transaction.savepoint("before_user_update")?;
        
        // Risky operation that might fail
        bestie {
            transaction.execute("UPDATE users SET status = 'migrated' WHERE old_system = true", [])?;
            println!("✅ User migration completed");
        } flex migration_error {
            println!("⚠️ User migration failed, rolling back to savepoint");
            transaction.rollback_to_savepoint(&savepoint1)?;
        }
        
        // Another savepoint for the next operation
        let savepoint2 = transaction.savepoint("before_cleanup")?;
        
        // Cleanup operation
        bestie {
            transaction.execute("DELETE FROM temp_data WHERE created_at < NOW() - INTERVAL '1 day'", [])?;
            println!("✅ Cleanup completed");
        } flex cleanup_error {
            println!("⚠️ Cleanup failed, rolling back to savepoint");
            transaction.rollback_to_savepoint(&savepoint2)?;
        }
        
        // Final validation
        let result = transaction.query("SELECT COUNT(*) as count FROM users WHERE status = 'invalid'", [])?;
        let invalid_count: normie = result.rows()[0].get("count")?;
        
        lowkey invalid_count > 0 {
            yolo DatabaseError::new(format!("Found {} invalid users", invalid_count));
        }
        
        // Commit everything
        transaction.commit()?;
        println!("✅ Complex transaction completed successfully");
        
    } flex error {
        transaction.rollback()?;
        println!("❌ Transaction failed: {}", error);
    }
}
```

### Transaction Isolation Levels

```cursed
sus isolation_level_example() {
    // Read Uncommitted (lowest isolation)
    let mut txn = connection.begin_transaction_with_isolation(
        IsolationLevel::ReadUncommitted
    )?;
    
    // Read Committed (default for most databases)
    let mut txn = connection.begin_transaction_with_isolation(
        IsolationLevel::ReadCommitted
    )?;
    
    // Repeatable Read (prevents non-repeatable reads)
    let mut txn = connection.begin_transaction_with_isolation(
        IsolationLevel::RepeatableRead
    )?;
    
    // Serializable (highest isolation)
    let mut txn = connection.begin_transaction_with_isolation(
        IsolationLevel::Serializable
    )?;
    
    // Read-only transaction
    let mut txn = connection.begin_read_only_transaction()?;
}
```

## Connection Pooling

### Basic Pool Setup

```cursed
sus basic_pool_example() {
    // Create pool configuration
    let pool_config = PoolConfig::new()
        .with_name("main_pool")
        .with_size_limits(5, 20)  // min: 5, max: 20 connections
        .with_timeouts(
            Duration::from_secs(30),   // Connection acquisition timeout
            Duration::from_secs(600)   // Idle connection timeout
        )
        .with_connection_config(
            ConnectionConfig::new("postgresql", "myapp")
                .with_host("db.example.com", 5432)
                .with_credentials("app_user", "password")
        );
    
    // Create and start the pool
    let mut pool = ConnectionPool::new(pool_config);
    pool.start().await?;
    
    // Use the pool
    let connection = pool.acquire().await?;
    
    // Perform database operations
    let results = connection.query("SELECT * FROM users LIMIT 10", [])?;
    
    // Return connection to pool
    pool.release(connection).await?;
    
    // Stop the pool when done
    pool.stop().await?;
}
```

### Pool Manager for Multiple Pools

```cursed
sus pool_manager_example() {
    let mut manager = PoolManager::new();
    
    // Create read-only pool
    let read_config = PoolConfig::new()
        .with_name("read_pool")
        .with_size_limits(8, 15)
        .with_connection_config(
            ConnectionConfig::new("postgresql", "myapp")
                .with_host("read-replica.example.com", 5432)
                .with_credentials("read_user", "readonly_pass")
        );
    
    // Create write pool
    let write_config = PoolConfig::new()
        .with_name("write_pool")
        .with_size_limits(3, 8)
        .with_connection_config(
            ConnectionConfig::new("postgresql", "myapp")
                .with_host("primary.example.com", 5432)
                .with_credentials("write_user", "write_pass")
        );
    
    // Create analytics pool
    let analytics_config = PoolConfig::new()
        .with_name("analytics_pool")
        .with_size_limits(2, 5)
        .with_connection_config(
            ConnectionConfig::new("postgresql", "analytics")
                .with_host("analytics.example.com", 5432)
                .with_credentials("analytics_user", "analytics_pass")
        );
    
    // Add pools to manager
    manager.create_pool("read_operations", read_config).await?;
    manager.create_pool("write_operations", write_config).await?;
    manager.create_pool("analytics", analytics_config).await?;
    
    // Start all pools
    manager.start_all_pools().await?;
    
    // Use different pools for different operations
    
    // Read operations
    let read_conn = manager.acquire_from_pool("read_operations").await?;
    let users = read_conn.query("SELECT * FROM users WHERE active = true", [])?;
    manager.release_to_pool("read_operations", read_conn).await?;
    
    // Write operations
    let write_conn = manager.acquire_from_pool("write_operations").await?;
    write_conn.execute("INSERT INTO audit_log (action, timestamp) VALUES (?, NOW())", ["user_login"])?;
    manager.release_to_pool("write_operations", write_conn).await?;
    
    // Analytics queries
    let analytics_conn = manager.acquire_from_pool("analytics").await?;
    let stats = analytics_conn.query("SELECT DATE(created_at) as date, COUNT(*) as count FROM users GROUP BY DATE(created_at)", [])?;
    manager.release_to_pool("analytics", analytics_conn).await?;
    
    // Shutdown all pools
    manager.shutdown_all().await?;
}
```

### Pool Health Monitoring

```cursed
sus pool_monitoring_example() {
    let pool_config = PoolConfig::new()
        .with_name("monitored_pool")
        .with_size_limits(5, 15)
        .with_health_check(
            Duration::from_secs(30),     // Check every 30 seconds
            "SELECT 1 as health_check".to_string()  // Health check query
        )
        .with_connection_config(ConnectionConfig::new("postgresql", "myapp"));
    
    let mut pool = ConnectionPool::new(pool_config);
    pool.start().await?;
    
    // Monitor pool statistics
    let stats = pool.statistics();
    println!("📊 Pool Statistics:");
    println!("  Total connections: {}", stats.total_connections());
    println!("  Active connections: {}", stats.active_connections());
    println!("  Idle connections: {}", stats.idle_connections());
    println!("  Pending requests: {}", stats.pending_requests());
    println!("  Acquisitions: {}", stats.acquisitions());
    println!("  Average acquisition time: {:?}", stats.average_acquisition_time());
    println!("  Connection errors: {}", stats.connection_errors());
    
    // Health check statistics
    let health_stats = pool.health_statistics();
    println!("🏥 Health Statistics:");
    println!("  Healthy connections: {}", health_stats.healthy_connections);
    println!("  Unhealthy connections: {}", health_stats.unhealthy_connections);
    println!("  Last health check: {:?}", health_stats.last_health_check);
    
    // Force a health check
    pool.force_health_check().await?;
    
    pool.stop().await?;
}
```

## ORM Features

### Object Mapping

```cursed
// Define your data structures
struct User {
    id: Option<normie>,
    username: tea,
    email: tea,
    age: normie,
    active: lit,
    created_at: Option<tea>,
}

struct Post {
    id: Option<normie>,
    user_id: normie,
    title: tea,
    content: tea,
    published: lit,
    view_count: normie,
}

sus orm_mapping_example() {
    let mut mapper = ObjectMapper::new();
    
    // Map User struct to users table
    mapper.map_struct("User", "users")
        .field("id", "id", SqlType::Integer)
        .field("username", "username", SqlType::Text)
        .field("email", "email", SqlType::Text)
        .field("age", "age", SqlType::Integer)
        .field("active", "active", SqlType::Boolean)
        .field("created_at", "created_at", SqlType::Timestamp)
        .primary_key("id")
        .finish();
    
    // Map Post struct to posts table
    mapper.map_struct("Post", "posts")
        .field("id", "id", SqlType::Integer)
        .field("user_id", "user_id", SqlType::Integer)
        .field("title", "title", SqlType::Text)
        .field("content", "content", SqlType::Text)
        .field("published", "published", SqlType::Boolean)
        .field("view_count", "view_count", SqlType::Integer)
        .primary_key("id")
        .foreign_key("user_id", "users", "id")
        .finish();
    
    // Use mapper for CRUD operations
    let mut crud = CrudOperations::new();
    
    // CREATE
    let create_user_sql = crud.generate_create_sql("User", &[
        ("username", SqlValue::Text("alice".to_string())),
        ("email", SqlValue::Text("alice@example.com".to_string())),
        ("age", SqlValue::Integer(28)),
        ("active", SqlValue::Boolean(true)),
    ])?;
    
    // READ
    let read_users_sql = crud.generate_read_sql("User", Some("active = true"))?;
    
    // UPDATE
    let update_user_sql = crud.generate_update_sql("User", &[
        ("age", SqlValue::Integer(29)),
    ], "id = 1")?;
    
    // DELETE
    let delete_user_sql = crud.generate_delete_sql("User", "active = false")?;
}
```

### Relationships

```cursed
sus orm_relationships_example() {
    let mut rel_manager = RelationshipManager::new();
    
    // One-to-Many: User has many Posts
    rel_manager.add_one_to_many(
        "User",     // Parent entity
        "posts",    // Relationship name
        "Post",     // Child entity
        "user_id",  // Foreign key in child
        "id"        // Primary key in parent
    );
    
    // Many-to-One: Post belongs to User
    rel_manager.add_many_to_one(
        "Post",     // Child entity
        "user",     // Relationship name
        "User",     // Parent entity
        "user_id",  // Foreign key in child
        "id"        // Primary key in parent
    );
    
    // Many-to-Many: User has many Roles
    rel_manager.add_many_to_many(
        "User",        // First entity
        "roles",       // Relationship name
        "Role",        // Second entity
        "user_roles",  // Junction table
        "user_id",     // First foreign key
        "role_id"      // Second foreign key
    );
    
    // Generate relationship queries
    let user_posts_sql = rel_manager.generate_relationship_query(
        "User", "posts", "user_id = 1"
    )?;
    
    let post_user_sql = rel_manager.generate_relationship_query(
        "Post", "user", "post_id = 1"
    )?;
    
    let user_roles_sql = rel_manager.generate_many_to_many_query(
        "User", "roles", "user_id = 1"
    )?;
}
```

## Migrations

### Creating Migrations

```cursed
sus migration_example() {
    // Create migration manager
    let mut runner = MigrationRunner::new();
    
    // Migration 1: Create users table
    let migration_001 = Migration::new("001", "create_users_table", 1)
        .with_up_script("
            CREATE TABLE users (
                id SERIAL PRIMARY KEY,
                username VARCHAR(255) UNIQUE NOT NULL,
                email VARCHAR(255) UNIQUE NOT NULL,
                password_hash VARCHAR(255) NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
            
            CREATE INDEX idx_users_username ON users(username);
            CREATE INDEX idx_users_email ON users(email);
        ")
        .with_down_script("
            DROP INDEX IF EXISTS idx_users_email;
            DROP INDEX IF EXISTS idx_users_username;
            DROP TABLE IF EXISTS users;
        ");
    
    // Migration 2: Create posts table
    let migration_002 = Migration::new("002", "create_posts_table", 2)
        .with_up_script("
            CREATE TABLE posts (
                id SERIAL PRIMARY KEY,
                user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                title VARCHAR(500) NOT NULL,
                content TEXT,
                published BOOLEAN DEFAULT false,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
            
            CREATE INDEX idx_posts_user_id ON posts(user_id);
            CREATE INDEX idx_posts_published ON posts(published);
            CREATE INDEX idx_posts_created_at ON posts(created_at);
        ")
        .with_down_script("
            DROP INDEX IF EXISTS idx_posts_created_at;
            DROP INDEX IF EXISTS idx_posts_published;
            DROP INDEX IF EXISTS idx_posts_user_id;
            DROP TABLE IF EXISTS posts;
        ")
        .with_dependency("001");  // Depends on users table
    
    // Migration 3: Add user profiles
    let migration_003 = Migration::new("003", "add_user_profiles", 3)
        .with_up_script("
            ALTER TABLE users ADD COLUMN first_name VARCHAR(255);
            ALTER TABLE users ADD COLUMN last_name VARCHAR(255);
            ALTER TABLE users ADD COLUMN bio TEXT;
            ALTER TABLE users ADD COLUMN avatar_url VARCHAR(500);
        ")
        .with_down_script("
            ALTER TABLE users DROP COLUMN avatar_url;
            ALTER TABLE users DROP COLUMN bio;
            ALTER TABLE users DROP COLUMN last_name;
            ALTER TABLE users DROP COLUMN first_name;
        ")
        .with_dependency("001");
    
    // Add migrations to runner
    runner.add_migration(migration_001);
    runner.add_migration(migration_002);
    runner.add_migration(migration_003);
    
    // Run migrations
    runner.run_migrations(&mut connection).await?;
    
    // Check migration status
    let status = runner.get_migration_status("001");
    println!("Migration 001 status: {:?}", status);
    
    // Rollback to specific version
    runner.rollback_to_version(&mut connection, 2).await?;
    
    // Get pending migrations
    let pending = runner.get_pending_migrations();
    println!("Pending migrations: {}", pending.len());
}
```

### Complex Migration with Data

```cursed
sus data_migration_example() {
    let data_migration = Migration::new("004", "migrate_user_data", 4)
        .with_up_script("
            -- Add new columns
            ALTER TABLE users ADD COLUMN full_name VARCHAR(500);
            ALTER TABLE users ADD COLUMN status VARCHAR(50) DEFAULT 'active';
            
            -- Migrate existing data
            UPDATE users 
            SET full_name = COALESCE(first_name || ' ' || last_name, username),
                status = CASE 
                    WHEN created_at < '2023-01-01' THEN 'legacy'
                    ELSE 'active'
                END;
            
            -- Add constraints after data migration
            ALTER TABLE users ALTER COLUMN full_name SET NOT NULL;
            ALTER TABLE users ADD CONSTRAINT check_status 
                CHECK (status IN ('active', 'inactive', 'legacy', 'suspended'));
            
            -- Create index
            CREATE INDEX idx_users_status ON users(status);
        ")
        .with_down_script("
            -- Remove constraints and indexes
            DROP INDEX IF EXISTS idx_users_status;
            ALTER TABLE users DROP CONSTRAINT IF EXISTS check_status;
            
            -- Remove columns
            ALTER TABLE users DROP COLUMN status;
            ALTER TABLE users DROP COLUMN full_name;
        ")
        .with_dependency("003");
    
    runner.add_migration(data_migration);
    runner.run_migrations(&mut connection).await?;
}
```

### Migration Scripts with Transactions

```cursed
sus transactional_migration_example() {
    let complex_migration = Migration::new("005", "complex_schema_change", 5)
        .with_up_script_transactional(true)  // Wrap in transaction
        .with_up_script("
            -- This entire migration runs in a single transaction
            -- If any part fails, everything rolls back
            
            -- Create temporary table
            CREATE TABLE users_temp (
                id SERIAL PRIMARY KEY,
                username VARCHAR(255) UNIQUE NOT NULL,
                email VARCHAR(255) UNIQUE NOT NULL,
                full_name VARCHAR(500) NOT NULL,
                status VARCHAR(50) DEFAULT 'active',
                metadata JSONB,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
            
            -- Copy data with transformations
            INSERT INTO users_temp (id, username, email, full_name, status, created_at)
            SELECT 
                id,
                username,
                email,
                COALESCE(first_name || ' ' || last_name, username) as full_name,
                COALESCE(status, 'active') as status,
                created_at
            FROM users;
            
            -- Drop old table and rename
            DROP TABLE users CASCADE;
            ALTER TABLE users_temp RENAME TO users;
            
            -- Recreate indexes
            CREATE UNIQUE INDEX idx_users_username ON users(username);
            CREATE UNIQUE INDEX idx_users_email ON users(email);
            CREATE INDEX idx_users_status ON users(status);
            
            -- Recreate foreign key constraints
            -- (these would need to be added based on dependent tables)
        ")
        .with_down_script("
            -- Rollback script to restore original schema
            CREATE TABLE users_old (
                id SERIAL PRIMARY KEY,
                username VARCHAR(255) UNIQUE NOT NULL,
                email VARCHAR(255) UNIQUE NOT NULL,
                first_name VARCHAR(255),
                last_name VARCHAR(255),
                bio TEXT,
                avatar_url VARCHAR(500),
                status VARCHAR(50) DEFAULT 'active',
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            );
            
            INSERT INTO users_old (id, username, email, created_at)
            SELECT id, username, email, created_at FROM users;
            
            DROP TABLE users CASCADE;
            ALTER TABLE users_old RENAME TO users;
        ");
    
    runner.add_migration(complex_migration);
    runner.run_migrations(&mut connection).await?;
}
```

## Performance & Optimization

### Query Performance

```cursed
sus query_optimization_examples() {
    // Use indexes effectively
    connection.execute("CREATE INDEX idx_users_email ON users(email)", [])?;
    connection.execute("CREATE INDEX idx_posts_user_created ON posts(user_id, created_at)", [])?;
    
    // Efficient pagination
    let page_size = 20;
    let offset = (page_number - 1) * page_size;
    
    let results = connection.query("
        SELECT id, title, created_at 
        FROM posts 
        WHERE published = true 
        ORDER BY created_at DESC 
        LIMIT ? OFFSET ?
    ", [page_size, offset])?;
    
    // Use prepared statements for repeated queries
    let stmt = connection.prepare("
        SELECT * FROM users 
        WHERE age BETWEEN ? AND ? 
        AND status = ?
    ").await?;
    
    // Execute multiple times with different parameters
    periodt age_range in [(18, 25), (26, 35), (36, 50)] {
        let results = stmt.execute(vec![
            SqlValue::Integer(age_range.0),
            SqlValue::Integer(age_range.1),
            SqlValue::Text("active".to_string())
        ]).await?;
        
        println!("Age group {}-{}: {} users", age_range.0, age_range.1, results.row_count());
    }
    
    // Batch operations with transactions
    let mut txn = connection.begin_transaction().await?;
    
    periodt i in 0..1000 {
        txn.execute("INSERT INTO batch_data (value) VALUES (?)", [format!("value_{}", i)])?;
    }
    
    txn.commit().await?;
    
    // Use aggregation efficiently
    let stats = connection.query("
        SELECT 
            status,
            COUNT(*) as user_count,
            AVG(age) as avg_age,
            MIN(created_at) as first_user,
            MAX(created_at) as latest_user
        FROM users 
        GROUP BY status
        ORDER BY user_count DESC
    ", [])?;
}
```

### Connection Pool Optimization

```cursed
sus pool_optimization_examples() {
    // Optimize pool size based on workload
    let pool_config = PoolConfig::new()
        .with_name("optimized_pool")
        .with_size_limits(
            // Minimum connections: keep some warm connections
            std::cmp::max(2, cpu_cores / 2),
            // Maximum connections: avoid overwhelming database
            std::cmp::min(50, cpu_cores * 4)
        )
        .with_timeouts(
            Duration::from_secs(5),    // Quick timeout for fast failure
            Duration::from_secs(300)   // Reasonable idle timeout
        )
        .with_health_check(
            Duration::from_secs(60),   // Check every minute
            "SELECT 1".to_string()
        )
        .with_retry_policy(3, Duration::from_millis(500));
    
    // Monitor and adjust based on metrics
    let stats = pool.statistics();
    
    lowkey stats.average_wait_time() > Duration::from_millis(100) {
        println!("⚠️ Pool may need more connections (avg wait: {:?})", stats.average_wait_time());
    }
    
    lowkey stats.idle_connections() > (stats.total_connections() / 2) {
        println!("ℹ️ Pool may have too many idle connections");
    }
    
    lowkey stats.acquisition_timeouts() > 0 {
        println!("🚨 Pool is experiencing timeouts: {}", stats.acquisition_timeouts());
    }
}
```

### Monitoring and Profiling

```cursed
sus monitoring_examples() {
    // Query timing
    let start_time = Instant::now();
    let results = connection.query("SLOW QUERY HERE", [])?;
    let query_time = start_time.elapsed();
    
    lowkey query_time > Duration::from_millis(100) {
        println!("⚠️ Slow query detected: {:?}", query_time);
    }
    
    // Connection pool metrics
    let pool_stats = pool.statistics();
    
    // Log metrics periodically
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(60));
        
        loop {
            interval.tick().await;
            
            let stats = pool.statistics();
            println!("📊 Pool Metrics: {} total, {} active, {} idle, {:.2}ms avg wait",
                stats.total_connections(),
                stats.active_connections(),
                stats.idle_connections(),
                stats.average_wait_time().as_millis()
            );
        }
    });
    
    // Query analysis
    let explain_result = connection.query("EXPLAIN ANALYZE SELECT * FROM users WHERE email = ?", [
        "user@example.com"
    ])?;
    
    periodt row in explain_result.rows() {
        println!("Query plan: {}", row.get_string("QUERY PLAN")?);
    }
}
```

## Error Handling

### Comprehensive Error Handling

```cursed
sus error_handling_examples() {
    bestie {
        let connection = sql_connect("postgresql", "invalid://connection/string")?;
        connection.query("SELECT * FROM nonexistent_table", [])?;
        
    } flex error {
        match error.kind() {
            ErrorKind::Connection(conn_err) => {
                match conn_err {
                    ConnectionError::FailedToConnect => {
                        println!("❌ Could not connect to database");
                        // Retry with exponential backoff
                        retry_connection().await?;
                    }
                    ConnectionError::AuthenticationFailed => {
                        println!("🔐 Authentication failed - check credentials");
                        // Don't retry, fix credentials
                    }
                    ConnectionError::Timeout => {
                        println!("⏰ Connection timed out");
                        // Retry with longer timeout
                    }
                    _ => println!("🔗 Other connection error: {}", error)
                }
            }
            
            ErrorKind::Query(query_err) => {
                match query_err {
                    QueryError::SyntaxError => {
                        println!("📝 SQL syntax error: {}", error.message());
                        // Log query for debugging
                    }
                    QueryError::TableNotFound => {
                        println!("🏷️ Table not found - check schema");
                        // Maybe run migrations
                    }
                    QueryError::ConstraintViolation => {
                        println!("⚖️ Constraint violation: {}", error.message());
                        // Handle duplicate keys, etc.
                    }
                    _ => println!("🔍 Other query error: {}", error)
                }
            }
            
            ErrorKind::Transaction(txn_err) => {
                match txn_err {
                    TransactionError::RollbackFailed => {
                        println!("🔄 Rollback failed - connection may be corrupted");
                        // Close and recreate connection
                    }
                    TransactionError::DeadlockDetected => {
                        println!("🔒 Deadlock detected - retrying transaction");
                        // Retry with random delay
                    }
                    _ => println!("📋 Other transaction error: {}", error)
                }
            }
            
            _ => println!("❓ Unknown database error: {}", error)
        }
        
        // Log error with context
        log::error!("Database operation failed: {} (Code: {:?}, Context: {:?})", 
            error.message(), 
            error.code(), 
            error.context()
        );
    }
}

async sus retry_connection() -> Result<DatabaseConnection, DatabaseError> {
    let max_retries = 5;
    let mut retry_count = 0;
    let mut delay = Duration::from_millis(100);
    
    loop {
        bestie {
            return sql_connect("postgresql", "postgresql://user:pass@localhost/db").await;
        } flex error {
            retry_count += 1;
            
            lowkey retry_count >= max_retries {
                return Err(error);
            }
            
            println!("🔄 Retry {} of {} in {:?}", retry_count, max_retries, delay);
            tokio::time::sleep(delay).await;
            
            // Exponential backoff
            delay *= 2;
            lowkey delay > Duration::from_secs(30) {
                delay = Duration::from_secs(30);
            }
        }
    }
}
```

### Pool Error Handling

```cursed
sus pool_error_handling() {
    bestie {
        let connection = pool.acquire().await?;
        
        bestie {
            let results = connection.query("SELECT * FROM users", [])?;
            // Process results
            
        } flex query_error {
            // Handle query error but connection might still be good
            println!("Query failed: {}", query_error);
            
            // Test if connection is still valid
            lowkey connection.query("SELECT 1", []).await.is_err() {
                // Connection is bad, don't return it to pool
                drop(connection);
            } highkey {
                // Connection is good, return to pool
                pool.release(connection).await?;
            }
        }
        
    } flex pool_error {
        match pool_error {
            PoolError::AcquisitionTimeout => {
                println!("⏰ Pool exhausted - all connections in use");
                // Maybe increase pool size or add more servers
            }
            PoolError::AllConnectionsFailed => {
                println!("💥 All connections failed - database may be down");
                // Circuit breaker pattern, stop trying for a while
            }
            PoolError::PoolShutdown => {
                println!("🛑 Pool is shutting down");
                // Graceful shutdown in progress
            }
            _ => println!("🏊‍♂️ Other pool error: {}", pool_error)
        }
    }
}
```

## Testing

### Unit Testing with Mocks

```cursed
#[cfg(test)]
mod tests {
    use super::*;
    use cursed::stdlib::packages::db_test::*;

    #[test]
    sus test_user_service() {
        // Create mock connection
        let mock = MockConnection::new();
        
        // Set up expected query and result
        mock.set_query_result(
            "SELECT * FROM users WHERE id = ?",
            Ok(QueryResult::from_rows(vec![
                mock_row![
                    "id" => 1,
                    "name" => "Alice",
                    "email" => "alice@example.com"
                ]
            ]))
        );
        
        // Test the service
        let user_service = UserService::new(Box::new(mock));
        let user = user_service.get_user(1)?;
        
        assert_eq!(user.name, "Alice");
        assert_eq!(user.email, "alice@example.com");
        
        // Verify mock was called correctly
        assert!(mock.was_query_executed("SELECT * FROM users WHERE id = ?"));
    }

    #[tokio::test]
    async sus test_user_registration() {
        let fixture = DatabaseFixture::new(
            Box::new(SqliteDriver::new().connect(":memory:").await?)
        ).await?;
        
        let user_service = UserService::new(fixture.connection());
        
        // Test user registration
        let new_user = CreateUserRequest {
            username: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password: "secure_password".to_string(),
        };
        
        let user_id = user_service.register_user(new_user).await?;
        assert!(user_id > 0);
        
        // Verify user was created
        let created_user = user_service.get_user(user_id).await?;
        assert_eq!(created_user.username, "testuser");
        assert_eq!(created_user.email, "test@example.com");
    }
}
```

### Integration Testing

```cursed
#[cfg(test)]
mod integration_tests {
    use super::*;
    
    #[tokio::test]
    async sus test_full_user_workflow() {
        // Use test database
        let config = TestConfig::new()
            .with_postgres_url("postgresql://test:test@localhost:5433/cursed_test");
        
        let runner = DatabaseTestRunner::new().with_config(config);
        
        runner.run_against_all_dbs("user_workflow", |mut connection| async move {
            // Setup schema
            connection.execute("CREATE TABLE users (
                id SERIAL PRIMARY KEY,
                username VARCHAR(255) UNIQUE NOT NULL,
                email VARCHAR(255) UNIQUE NOT NULL,
                created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
            )", [])?;
            
            // Test workflow
            let user_service = UserService::new(Box::new(connection));
            
            // 1. Register user
            let user_id = user_service.register_user(CreateUserRequest {
                username: "integration_test".to_string(),
                email: "test@integration.com".to_string(),
                password: "test_password".to_string(),
            }).await?;
            
            // 2. Get user
            let user = user_service.get_user(user_id).await?;
            assert_eq!(user.username, "integration_test");
            
            // 3. Update user
            user_service.update_user(user_id, UpdateUserRequest {
                email: Some("updated@integration.com".to_string()),
                ..Default::default()
            }).await?;
            
            // 4. Verify update
            let updated_user = user_service.get_user(user_id).await?;
            assert_eq!(updated_user.email, "updated@integration.com");
            
            // 5. Delete user
            user_service.delete_user(user_id).await?;
            
            // 6. Verify deletion
            assert!(user_service.get_user(user_id).await.is_err());
            
            Ok(())
        }).await;
    }
}
```

### Performance Testing

```cursed
#[cfg(test)]
mod performance_tests {
    use super::*;
    
    #[tokio::test]
    async sus benchmark_query_performance() {
        let connection = sql_connect("sqlite", ":memory:").await?;
        
        // Setup test data
        setup_large_dataset(&mut connection).await?;
        
        // Benchmark different query types
        let mut comparison = BenchmarkComparison::new();
        
        // Simple query benchmark
        let mut simple_benchmark = DatabaseBenchmark::new("Simple SELECT".to_string(), 100);
        simple_benchmark.run(|| {
            Box::pin(async {
                connection.query("SELECT COUNT(*) FROM test_data", []).await
            })
        }).await;
        comparison.add_benchmark(simple_benchmark);
        
        // Complex query benchmark
        let mut complex_benchmark = DatabaseBenchmark::new("Complex JOIN".to_string(), 50);
        complex_benchmark.run(|| {
            Box::pin(async {
                connection.query("
                    SELECT u.name, COUNT(p.id) as post_count
                    FROM users u
                    LEFT JOIN posts p ON u.id = p.user_id
                    GROUP BY u.id, u.name
                    ORDER BY post_count DESC
                    LIMIT 10
                ", []).await
            })
        }).await;
        comparison.add_benchmark(complex_benchmark);
        
        // Print comparison
        comparison.print_comparison();
        
        // Assert performance requirements
        assert!(simple_benchmark.average_duration() < Duration::from_millis(10));
        assert!(complex_benchmark.average_duration() < Duration::from_millis(100));
    }
}
```

## Best Practices

### Connection Management

```cursed
// ✅ Good: Use connection pooling for web applications
sus good_connection_management() {
    // Create pool once at application startup
    let pool = create_application_pool().await?;
    
    // Use pool for each request
    let connection = pool.acquire().await?;
    let result = connection.query("SELECT * FROM users", [])?;
    pool.release(connection).await?;
    
    // Pool automatically manages connection lifecycle
}

// ❌ Bad: Creating new connections for each request
sus bad_connection_management() {
    // Don't do this - creates connection overhead
    let connection = sql_connect("postgresql", CONNECTION_STRING)?;
    let result = connection.query("SELECT * FROM users", [])?;
    connection.close()?;
}
```

### Query Building

```cursed
// ✅ Good: Use parameterized queries
sus good_query_building() {
    let user_id = get_user_id_from_request();
    let results = connection.query(
        "SELECT * FROM users WHERE id = ? AND active = ?",
        [user_id, true]
    )?;
}

// ❌ Bad: String interpolation (SQL injection risk)
sus bad_query_building() {
    let user_id = get_user_id_from_request();
    let sql = format!("SELECT * FROM users WHERE id = {}", user_id);
    let results = connection.query(&sql, [])?;  // DANGEROUS!
}

// ✅ Good: Use query builder for complex queries
sus good_complex_query() {
    let mut builder = SqlQueryBuilder::new();
    let sql = builder.select()
        .columns(&["u.name", "p.title", "p.created_at"])
        .from("users u")
        .inner_join("posts p", "u.id = p.user_id")
        .where_eq("u.active", true)
        .where_clause("p.published_at > ?")
        .order_by("p.created_at", OrderDirection::Desc)
        .limit(20)
        .build()?;
    
    builder.add_parameter(SqlValue::Text("2024-01-01".to_string()));
    let results = connection.query(&sql, builder.parameters().clone())?;
}
```

### Transaction Management

```cursed
// ✅ Good: Proper transaction handling
sus good_transaction_handling() {
    let mut txn = connection.begin_transaction()?;
    
    bestie {
        // All related operations in single transaction
        let user_id = create_user(&mut txn, user_data)?;
        create_user_profile(&mut txn, user_id, profile_data)?;
        send_welcome_email(&mut txn, user_id)?;
        
        txn.commit()?;
        
    } flex error {
        txn.rollback()?;
        return Err(error);
    }
}

// ❌ Bad: Operations without proper transaction scope
sus bad_transaction_handling() {
    // These could partially succeed, leaving inconsistent state
    let user_id = create_user(&mut connection, user_data)?;
    create_user_profile(&mut connection, user_id, profile_data)?;  // Might fail
    send_welcome_email(&mut connection, user_id)?;  // Might fail
}
```

### Error Handling

```cursed
// ✅ Good: Comprehensive error handling
sus good_error_handling() {
    bestie {
        let results = connection.query("SELECT * FROM users", [])?;
        process_results(results)?;
        
    } flex error {
        // Log error with context
        log::error!("Query failed: {} (Context: {:?})", error, error.context());
        
        // Handle different error types appropriately
        match error.kind() {
            ErrorKind::Connection(_) => {
                // Connection issue - maybe retry
                return retry_with_backoff(|| query_users()).await;
            }
            ErrorKind::Query(QueryError::TableNotFound) => {
                // Schema issue - maybe run migrations
                return run_migrations_and_retry().await;
            }
            _ => return Err(error)
        }
    }
}

// ❌ Bad: Ignoring errors or generic handling
sus bad_error_handling() {
    // Don't ignore errors
    let _ = connection.query("SELECT * FROM users", []);
    
    // Don't use generic error handling
    bestie {
        connection.query("SELECT * FROM users", [])?;
    } flex _ {
        println!("Something went wrong");  // Not helpful!
    }
}
```

### Resource Management

```cursed
// ✅ Good: Proper resource cleanup
sus good_resource_management() {
    let pool = ConnectionPool::new(config);
    pool.start().await?;
    
    // Use RAII or explicit cleanup
    defer! {
        pool.stop().await.unwrap();
    }
    
    // Or use a pool manager that handles cleanup
    let manager = PoolManager::new();
    // manager automatically cleans up when dropped
}

// ✅ Good: Connection lifecycle management
sus good_connection_lifecycle() {
    let connection = pool.acquire().await?;
    
    // Ensure connection is always returned to pool
    let result = bestie {
        connection.query("SELECT * FROM users", [])
    } flex error {
        // Return connection even on error
        pool.release(connection).await?;
        return Err(error);
    };
    
    pool.release(connection).await?;
    result
}
```

## Troubleshooting

### Common Issues

#### Connection Problems

```cursed
// Issue: Connection refused
// Solution: Check database server status and network connectivity
sus debug_connection_refused() {
    bestie {
        let connection = sql_connect("postgresql", "postgresql://user:pass@localhost:5432/db")?;
    } flex error {
        lowkey error.message().contains("connection refused") {
            println!("🔍 Troubleshooting steps:");
            println!("1. Check if PostgreSQL is running: sudo systemctl status postgresql");
            println!("2. Check if port 5432 is open: netstat -tulpn | grep 5432");
            println!("3. Check firewall settings");
            println!("4. Verify host and port in connection string");
        }
    }
}

// Issue: Authentication failed
// Solution: Verify credentials and permissions
sus debug_authentication_failed() {
    bestie {
        let connection = sql_connect("postgresql", "postgresql://user:wrongpass@localhost/db")?;
    } flex error {
        lowkey error.message().contains("authentication failed") {
            println!("🔍 Troubleshooting steps:");
            println!("1. Verify username and password");
            println!("2. Check pg_hba.conf for authentication method");
            println!("3. Ensure user has permission to connect to database");
            println!("4. Check if user account is locked or expired");
        }
    }
}

// Issue: Too many connections
// Solution: Use connection pooling or increase database limits
sus debug_too_many_connections() {
    bestie {
        // Try to create many connections
        periodt i in 0..200 {
            let _conn = sql_connect("postgresql", CONNECTION_STRING)?;
        }
    } flex error {
        lowkey error.message().contains("too many connections") {
            println!("🔍 Solutions:");
            println!("1. Use connection pooling to limit concurrent connections");
            println!("2. Increase max_connections in postgresql.conf");
            println!("3. Close idle connections more aggressively");
            println!("4. Use connection multiplexing tools like PgBouncer");
        }
    }
}
```

#### Query Problems

```cursed
// Issue: Slow queries
sus debug_slow_queries() {
    let start_time = Instant::now();
    let results = connection.query("SELECT * FROM large_table WHERE unindexed_column = ?", ["value"])?;
    let query_time = start_time.elapsed();
    
    lowkey query_time > Duration::from_millis(1000) {
        println!("🐌 Slow query detected: {:?}", query_time);
        println!("🔍 Optimization steps:");
        println!("1. Add index on frequently queried columns");
        println!("2. Use EXPLAIN ANALYZE to understand query plan");
        println!("3. Consider query rewriting or pagination");
        println!("4. Check for table locks or blocking queries");
        
        // Show query plan
        let explain = connection.query("EXPLAIN ANALYZE SELECT * FROM large_table WHERE unindexed_column = ?", ["value"])?;
        periodt row in explain.rows() {
            println!("Plan: {}", row.get_string("QUERY PLAN")?);
        }
    }
}

// Issue: Memory issues with large result sets
sus debug_memory_issues() {
    bestie {
        let results = connection.query("SELECT * FROM huge_table", [])?;
        
        lowkey results.row_count() > 10000 {
            println!("⚠️ Large result set: {} rows", results.row_count());
            println!("🔍 Memory optimization:");
            println!("1. Use LIMIT and OFFSET for pagination");
            println!("2. Process results in streaming fashion");
            println!("3. Use cursors for very large datasets");
            println!("4. Consider aggregation instead of fetching all rows");
        }
        
    } flex error {
        lowkey error.message().contains("out of memory") {
            println!("💾 Memory exhausted!");
            println!("🔍 Solutions:");
            println!("1. Reduce result set size with WHERE clauses");
            println!("2. Use pagination with LIMIT/OFFSET");
            println!("3. Increase available memory");
            println!("4. Use streaming query processing");
        }
    }
}
```

#### Pool Problems

```cursed
// Issue: Pool exhaustion
sus debug_pool_exhaustion() {
    let stats = pool.statistics();
    
    lowkey stats.acquisition_timeouts() > 0 {
        println!("🏊‍♂️ Pool exhaustion detected!");
        println!("📊 Current stats:");
        println!("  Total connections: {}", stats.total_connections());
        println!("  Active connections: {}", stats.active_connections());
        println!("  Idle connections: {}", stats.idle_connections());
        println!("  Pending requests: {}", stats.pending_requests());
        println!("  Acquisition timeouts: {}", stats.acquisition_timeouts());
        
        println!("🔍 Solutions:");
        println!("1. Increase pool size");
        println!("2. Reduce connection hold time");
        println!("3. Check for connection leaks");
        println!("4. Add more database servers");
    }
}

// Issue: Connection leaks
sus debug_connection_leaks() {
    let initial_stats = pool.statistics();
    
    // Simulate application work
    periodt i in 0..10 {
        let connection = pool.acquire().await?;
        // Simulate work but forget to release connection
        lowkey i % 3 == 0 {
            // Intentionally leak some connections for testing
            std::mem::forget(connection);
        } highkey {
            pool.release(connection).await?;
        }
    }
    
    let final_stats = pool.statistics();
    
    lowkey final_stats.idle_connections() < initial_stats.idle_connections() {
        println!("🕳️ Possible connection leak detected!");
        println!("🔍 Debugging steps:");
        println!("1. Ensure all acquired connections are released");
        println!("2. Use try-finally or defer patterns");
        println!("3. Monitor pool statistics in production");
        println!("4. Set connection max lifetime to force cleanup");
    }
}
```

### Performance Debugging

```cursed
sus performance_debugging() {
    // Enable query logging
    connection.execute("SET log_statement = 'all'", [])?;
    connection.execute("SET log_min_duration_statement = 100", [])?;  // Log queries > 100ms
    
    // Monitor active queries
    let active_queries = connection.query("
        SELECT 
            pid,
            now() - pg_stat_activity.query_start AS duration,
            query 
        FROM pg_stat_activity 
        WHERE (now() - pg_stat_activity.query_start) > interval '5 minutes'
    ", [])?;
    
    lowkey active_queries.row_count() > 0 {
        println!("🐌 Long-running queries detected:");
        periodt row in active_queries.rows() {
            let pid: normie = row.get("pid")?;
            let duration: tea = row.get("duration")?;
            let query: tea = row.get("query")?;
            println!("  PID {}: {} - {}", pid, duration, query);
        }
    }
    
    // Check for locks
    let locks = connection.query("
        SELECT 
            blocked_locks.pid AS blocked_pid,
            blocked_activity.usename AS blocked_user,
            blocking_locks.pid AS blocking_pid,
            blocking_activity.usename AS blocking_user,
            blocked_activity.query AS blocked_statement,
            blocking_activity.query AS current_statement_in_blocking_process
        FROM pg_catalog.pg_locks blocked_locks
        JOIN pg_catalog.pg_stat_activity blocked_activity ON blocked_activity.pid = blocked_locks.pid
        JOIN pg_catalog.pg_locks blocking_locks ON blocking_locks.locktype = blocked_locks.locktype
        JOIN pg_catalog.pg_stat_activity blocking_activity ON blocking_activity.pid = blocking_locks.pid
        WHERE NOT blocked_locks.granted AND blocking_locks.granted
    ", [])?;
    
    lowkey locks.row_count() > 0 {
        println!("🔒 Lock contention detected:");
        periodt row in locks.rows() {
            println!("  Blocked PID: {} by PID: {}", 
                row.get_i64("blocked_pid")?, 
                row.get_i64("blocking_pid")?);
        }
    }
}
```

---

## Conclusion

This comprehensive guide covers all aspects of the CURSED database connectivity packages. For more specific examples and advanced usage patterns, check out the individual package documentation and example programs in the `examples/database/` directory.

Remember to always:
- Use connection pooling for production applications
- Handle errors appropriately for your use case
- Monitor performance and optimize queries
- Test thoroughly with realistic data volumes
- Keep your database schema and migrations under version control

Happy coding with CURSED databases! 🗄️✨
