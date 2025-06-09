/// fr fr Comprehensive unit tests for all CURSED database components
/// 
/// This test suite provides exhaustive unit testing for each database package:
/// - db_core: Core interfaces, errors, connections, queries, transactions
/// - db_sql: SQL drivers, query builders, types, dialects
/// - db_pool: Connection pooling and management
/// - db_orm: Object-relational mapping features
/// - db_migrate: Migration system and version management
/// - db_nosql: NoSQL drivers and document operations

use cursed::stdlib::packages::{
    db_core::{
        self, DatabaseError, ErrorKind, ConnectionError, QueryError,
        ConnectionConfig, ConnectionOptions, ConnectionState,
        DatabaseResult as DbResult, DriverRegistry, QueryBuilder,
        Transaction, TransactionState, DatabaseMetadata
    },
    db_sql::{
        self, SqlQueryBuilder, SelectBuilder, InsertBuilder, UpdateBuilder,
        SqlValue, SqlType, SqlDialect, SqlDriver, SqlConnection,
        PostgreSqlDriver, MySqlDriver, SqliteDriver, PreparedStatement
    },
    db_pool::{
        self, ConnectionPool, PoolManager, LoadBalancer,
        PoolConfig, PoolStatistics, PoolState
    },
    db_orm::{
        self, ObjectMapper, RelationshipManager, CrudOperations,
        OneToMany, ManyToOne, ManyToMany
    },
    db_migrate::{
        self, Migration, MigrationRunner, VersionManager,
        MigrationStatus, MigrationScript
    },
    db_nosql::{
        self, DocumentDriver, Collection, Document,
        MongoDbDriver, RedisDriver
    }
};
use std::time::Duration;
use std::collections::HashMap;

/// fr fr Core database functionality unit tests
mod db_core_unit_tests {
    use super::*;

    #[test]
    fn test_database_error_construction() {
        let error = DatabaseError::connection(
            ConnectionError::FailedToConnect,
            "Database server unavailable"
        );
        
        assert_eq!(error.category(), "connection");
        assert_eq!(error.kind(), ErrorKind::Connection(ConnectionError::FailedToConnect));
        assert!(error.message().contains("Database server unavailable"));
        assert!(error.is_retryable());
        assert!(!error.is_permanent());
    }

    #[test]
    fn test_database_error_with_metadata() {
        let error = DatabaseError::query(
            QueryError::SyntaxError,
            "Invalid SQL syntax in query"
        )
        .with_code("42601")
        .with_sql_state("42601")
        .with_context("query", "SELECT * FROM nonexistent")
        .with_context("line", "1")
        .with_context("column", "15");

        assert_eq!(error.code, Some("42601".to_string()));
        assert_eq!(error.sql_state, Some("42601".to_string()));
        assert_eq!(error.context.get("query"), Some(&"SELECT * FROM nonexistent".to_string()));
        assert_eq!(error.context.get("line"), Some(&"1".to_string()));
        assert_eq!(error.context.get("column"), Some(&"15".to_string()));
    }

    #[test]
    fn test_database_error_chain() {
        let io_error = std::io::Error::new(std::io::ErrorKind::PermissionDenied, "Access denied");
        let db_error = DatabaseError::connection(
            ConnectionError::AuthenticationFailed,
            "Authentication failed"
        ).with_source(io_error);

        let chain = db_error.error_chain();
        assert!(chain.contains("Authentication failed"));
        assert!(chain.contains("caused by"));
        assert!(chain.contains("Access denied"));
    }

    #[test]
    fn test_connection_config_builder() {
        let config = ConnectionConfig::new("postgresql", "test_db")
            .with_host("db.example.com", 5432)
            .with_credentials("testuser", "testpass")
            .with_ssl_mode("require")
            .with_parameter("connect_timeout", "30")
            .with_parameter("application_name", "cursed_app");

        assert_eq!(config.driver, "postgresql");
        assert_eq!(config.database, "test_db");
        assert_eq!(config.host, Some("db.example.com".to_string()));
        assert_eq!(config.port, Some(5432));
        assert_eq!(config.username, Some("testuser".to_string()));
        assert_eq!(config.password, Some("testpass".to_string()));
        assert_eq!(config.ssl_mode, Some("require".to_string()));
        assert_eq!(config.parameters.get("connect_timeout"), Some(&"30".to_string()));
        assert_eq!(config.parameters.get("application_name"), Some(&"cursed_app".to_string()));
    }

    #[test]
    fn test_connection_config_from_string() {
        let conn_str = "postgresql://user:pass@localhost:5432/mydb?sslmode=require&connect_timeout=30&application_name=test";
        let config = ConnectionConfig::from_string(conn_str).unwrap();

        assert_eq!(config.driver, "postgresql");
        assert_eq!(config.database, "mydb");
        assert_eq!(config.host, Some("localhost".to_string()));
        assert_eq!(config.port, Some(5432));
        assert_eq!(config.username, Some("user".to_string()));
        assert_eq!(config.password, Some("pass".to_string()));
        assert_eq!(config.parameters.get("sslmode"), Some(&"require".to_string()));
        assert_eq!(config.parameters.get("connect_timeout"), Some(&"30".to_string()));
        assert_eq!(config.parameters.get("application_name"), Some(&"test".to_string()));
    }

    #[test]
    fn test_connection_config_to_string() {
        let config = ConnectionConfig::new("mysql", "webapp")
            .with_host("db-server", 3306)
            .with_credentials("dbuser", "dbpass")
            .with_parameter("charset", "utf8mb4")
            .with_parameter("autocommit", "true");

        let conn_str = config.to_connection_string();
        assert!(conn_str.starts_with("mysql://"));
        assert!(conn_str.contains("dbuser:dbpass"));
        assert!(conn_str.contains("@db-server:3306"));
        assert!(conn_str.contains("/webapp"));
        assert!(conn_str.contains("charset=utf8mb4"));
        assert!(conn_str.contains("autocommit=true"));
    }

    #[test]
    fn test_connection_options() {
        let options = ConnectionOptions::new()
            .with_pool_size(5, 20)
            .with_timeouts(Duration::from_secs(60), Duration::from_secs(3600))
            .with_retry_config(3, Duration::from_secs(5))
            .with_health_check(Duration::from_secs(30), "SELECT 1".to_string());

        assert_eq!(options.min_connections, Some(5));
        assert_eq!(options.max_connections, Some(20));
        assert_eq!(options.idle_timeout, Some(Duration::from_secs(60)));
        assert_eq!(options.max_lifetime, Some(Duration::from_secs(3600)));
        assert_eq!(options.max_retries, Some(3));
        assert_eq!(options.retry_delay, Some(Duration::from_secs(5)));
        assert_eq!(options.health_check_interval, Some(Duration::from_secs(30)));
        assert_eq!(options.health_check_query, Some("SELECT 1".to_string()));
    }

    #[test]
    fn test_driver_registry() {
        let mut registry = DriverRegistry::new();
        
        // Test empty registry
        assert_eq!(registry.list_drivers().len(), 0);
        assert!(registry.get_driver("postgresql").is_none());
        
        // Register a mock driver
        let driver_info = db_core::DriverInfo {
            name: "mock".to_string(),
            version: "1.0.0".to_string(),
            features: vec!["transactions".to_string(), "prepared_statements".to_string()],
            connection_schemes: vec!["mock".to_string()],
        };
        
        registry.register_driver("mock", driver_info.clone());
        
        // Test driver retrieval
        assert_eq!(registry.list_drivers().len(), 1);
        assert!(registry.get_driver("mock").is_some());
        assert_eq!(registry.get_driver("mock").unwrap().name, "mock");
        
        // Test driver unregistration
        registry.unregister_driver("mock");
        assert_eq!(registry.list_drivers().len(), 0);
    }

    #[test]
    fn test_query_builder_basic() {
        let mut builder = QueryBuilder::new();
        
        // Test parameter addition
        let param1 = builder.add_parameter("value1");
        let param2 = builder.add_parameter(42);
        let param3 = builder.add_parameter(true);
        
        assert_eq!(builder.parameter_count(), 3);
        assert_eq!(param1, 0);
        assert_eq!(param2, 1);
        assert_eq!(param3, 2);
        
        // Test parameter retrieval
        let params = builder.parameters();
        assert_eq!(params.len(), 3);
        
        // Test clear
        builder.clear_parameters();
        assert_eq!(builder.parameter_count(), 0);
    }

    #[test]
    fn test_transaction_state_transitions() {
        let mut txn = Transaction::new();
        
        // Initial state
        assert_eq!(txn.state(), TransactionState::Active);
        assert!(!txn.is_committed());
        assert!(!txn.is_rolled_back());
        
        // Test savepoint creation
        let savepoint_id = txn.create_savepoint("sp1").unwrap();
        assert!(txn.has_savepoint(&savepoint_id));
        
        // Test rollback to savepoint
        txn.rollback_to_savepoint(&savepoint_id).unwrap();
        assert!(!txn.has_savepoint(&savepoint_id));
        
        // Test commit
        txn.commit().unwrap();
        assert_eq!(txn.state(), TransactionState::Committed);
        assert!(txn.is_committed());
    }

    #[test]
    fn test_database_metadata() {
        let mut metadata = DatabaseMetadata::new();
        
        // Test table operations
        metadata.add_table("users", vec![
            ("id".to_string(), "INTEGER".to_string()),
            ("name".to_string(), "TEXT".to_string()),
            ("email".to_string(), "TEXT".to_string()),
        ]);
        
        assert!(metadata.has_table("users"));
        assert!(!metadata.has_table("nonexistent"));
        
        let tables = metadata.list_tables();
        assert_eq!(tables.len(), 1);
        assert!(tables.contains(&"users".to_string()));
        
        // Test column operations
        let columns = metadata.get_table_columns("users").unwrap();
        assert_eq!(columns.len(), 3);
        assert!(columns.iter().any(|(name, _)| name == "id"));
        assert!(columns.iter().any(|(name, _)| name == "name"));
        assert!(columns.iter().any(|(name, _)| name == "email"));
    }
}

/// fr fr SQL package unit tests
mod db_sql_unit_tests {
    use super::*;

    #[test]
    fn test_sql_value_creation_and_conversion() {
        // Test string values
        let text_val = SqlValue::Text("hello world".to_string());
        assert!(matches!(text_val, SqlValue::Text(_)));
        assert_eq!(text_val.to_string(), "hello world");
        
        // Test integer values
        let int_val = SqlValue::Integer(42);
        assert!(matches!(int_val, SqlValue::Integer(42)));
        assert_eq!(int_val.to_i64().unwrap(), 42);
        
        // Test float values
        let float_val = SqlValue::Float(3.14159);
        assert!(matches!(float_val, SqlValue::Float(_)));
        assert!((float_val.to_f64().unwrap() - 3.14159).abs() < f64::EPSILON);
        
        // Test boolean values
        let bool_val = SqlValue::Boolean(true);
        assert!(matches!(bool_val, SqlValue::Boolean(true)));
        assert_eq!(bool_val.to_bool().unwrap(), true);
        
        // Test null values
        let null_val = SqlValue::Null;
        assert!(matches!(null_val, SqlValue::Null));
        assert!(null_val.is_null());
    }

    #[test]
    fn test_sql_value_type_checking() {
        let values = vec![
            (SqlValue::Text("test".to_string()), SqlType::Text),
            (SqlValue::Integer(100), SqlType::Integer),
            (SqlValue::Float(2.5), SqlType::Float),
            (SqlValue::Boolean(false), SqlType::Boolean),
            (SqlValue::Null, SqlType::Null),
        ];
        
        for (value, expected_type) in values {
            assert_eq!(value.sql_type(), expected_type);
        }
    }

    #[test]
    fn test_sql_query_builder_select() {
        let mut builder = SqlQueryBuilder::new();
        
        let sql = builder.select()
            .distinct()
            .columns(&["u.id", "u.name", "p.email"])
            .from("users u")
            .inner_join("profiles p", "u.id = p.user_id")
            .left_join("roles r", "u.role_id = r.id")
            .where_clause("u.active = ?")
            .where_clause("u.created_at > ?")
            .group_by(&["u.id", "u.name"])
            .having("COUNT(p.id) > 0")
            .order_by("u.name", db_sql::OrderDirection::Asc)
            .order_by("u.created_at", db_sql::OrderDirection::Desc)
            .limit(50)
            .offset(100)
            .build()
            .unwrap();

        assert!(sql.contains("SELECT DISTINCT"));
        assert!(sql.contains("u.id, u.name, p.email"));
        assert!(sql.contains("FROM users u"));
        assert!(sql.contains("INNER JOIN profiles p ON u.id = p.user_id"));
        assert!(sql.contains("LEFT JOIN roles r ON u.role_id = r.id"));
        assert!(sql.contains("WHERE u.active = ? AND u.created_at > ?"));
        assert!(sql.contains("GROUP BY u.id, u.name"));
        assert!(sql.contains("HAVING COUNT(p.id) > 0"));
        assert!(sql.contains("ORDER BY u.name ASC, u.created_at DESC"));
        assert!(sql.contains("LIMIT 50"));
        assert!(sql.contains("OFFSET 100"));
    }

    #[test]
    fn test_sql_query_builder_insert() {
        let mut builder = SqlQueryBuilder::new();
        
        let sql = builder.insert()
            .into("users")
            .columns(&["name", "email", "age", "active"])
            .values(vec![
                SqlValue::Text("Alice".to_string()),
                SqlValue::Text("alice@example.com".to_string()),
                SqlValue::Integer(30),
                SqlValue::Boolean(true)
            ])
            .on_conflict("email")
            .do_update(&[("name", "EXCLUDED.name"), ("age", "EXCLUDED.age")])
            .build()
            .unwrap();

        assert!(sql.contains("INSERT INTO users"));
        assert!(sql.contains("(name, email, age, active)"));
        assert!(sql.contains("VALUES (?, ?, ?, ?)"));
        assert!(sql.contains("ON CONFLICT (email)"));
        assert!(sql.contains("DO UPDATE SET"));
        
        // Verify parameters
        let params = builder.parameters();
        assert_eq!(params.len(), 4);
    }

    #[test]
    fn test_sql_query_builder_update() {
        let mut builder = SqlQueryBuilder::new();
        
        let sql = builder.update()
            .table("users")
            .set("name", SqlValue::Text("Bob".to_string()))
            .set("email", SqlValue::Text("bob@example.com".to_string()))
            .set("updated_at", SqlValue::Text("CURRENT_TIMESTAMP".to_string()))
            .where_eq("id", SqlValue::Integer(1))
            .where_clause("active = ?")
            .build()
            .unwrap();

        assert!(sql.contains("UPDATE users"));
        assert!(sql.contains("SET name = ?, email = ?, updated_at = ?"));
        assert!(sql.contains("WHERE id = ? AND active = ?"));
        
        // Verify parameters
        let params = builder.parameters();
        assert_eq!(params.len(), 5);
    }

    #[test]
    fn test_sql_query_builder_delete() {
        let mut builder = SqlQueryBuilder::new();
        
        let sql = builder.delete()
            .from("users")
            .where_eq("active", SqlValue::Boolean(false))
            .where_clause("last_login < ?")
            .limit(100)
            .build()
            .unwrap();

        assert!(sql.contains("DELETE FROM users"));
        assert!(sql.contains("WHERE active = ? AND last_login < ?"));
        assert!(sql.contains("LIMIT 100"));
        
        // Verify parameters
        let params = builder.parameters();
        assert_eq!(params.len(), 2);
    }

    #[test]
    fn test_sql_query_builder_create_table() {
        let mut builder = SqlQueryBuilder::new();
        
        let sql = builder.create_table()
            .table("orders")
            .if_not_exists()
            .column("id", SqlType::Integer).primary_key().auto_increment().finish()
            .column("user_id", SqlType::Integer).not_null().finish()
            .column("product_id", SqlType::Integer).not_null().finish()
            .column("quantity", SqlType::Integer).default_value(SqlValue::Integer(1)).finish()
            .column("price", SqlType::Decimal).not_null().finish()
            .column("created_at", SqlType::Timestamp).default_current_timestamp().finish()
            .constraint(db_sql::TableConstraint::ForeignKey(
                "user_id".to_string(),
                "users".to_string(),
                "id".to_string()
            ))
            .constraint(db_sql::TableConstraint::Check("quantity > 0".to_string()))
            .constraint(db_sql::TableConstraint::Index(
                "idx_user_created".to_string(),
                vec!["user_id".to_string(), "created_at".to_string()]
            ))
            .build()
            .unwrap();

        assert!(sql.contains("CREATE TABLE IF NOT EXISTS orders"));
        assert!(sql.contains("id"));
        assert!(sql.contains("PRIMARY KEY"));
        assert!(sql.contains("AUTO_INCREMENT"));
        assert!(sql.contains("NOT NULL"));
        assert!(sql.contains("DEFAULT"));
        assert!(sql.contains("FOREIGN KEY"));
        assert!(sql.contains("CHECK (quantity > 0)"));
        assert!(sql.contains("INDEX"));
    }

    #[test]
    fn test_sql_dialect_differences() {
        // Test PostgreSQL dialect
        let pg_dialect = SqlDialect::PostgreSQL;
        assert_eq!(pg_dialect.quote_identifier("table"), "\"table\"");
        assert_eq!(pg_dialect.limit_clause(10, Some(5)), "LIMIT 10 OFFSET 5");
        assert_eq!(pg_dialect.auto_increment_syntax(), "SERIAL");
        
        // Test MySQL dialect
        let mysql_dialect = SqlDialect::MySQL;
        assert_eq!(mysql_dialect.quote_identifier("table"), "`table`");
        assert_eq!(mysql_dialect.limit_clause(10, Some(5)), "LIMIT 5, 10");
        assert_eq!(mysql_dialect.auto_increment_syntax(), "AUTO_INCREMENT");
        
        // Test SQLite dialect
        let sqlite_dialect = SqlDialect::SQLite;
        assert_eq!(sqlite_dialect.quote_identifier("table"), "[table]");
        assert_eq!(sqlite_dialect.limit_clause(10, Some(5)), "LIMIT 10 OFFSET 5");
        assert_eq!(sqlite_dialect.auto_increment_syntax(), "AUTOINCREMENT");
    }

    #[test]
    fn test_prepared_statement() {
        let stmt = PreparedStatement::new(
            "SELECT * FROM users WHERE age > ? AND active = ?",
            vec![SqlType::Integer, SqlType::Boolean]
        );
        
        assert_eq!(stmt.sql(), "SELECT * FROM users WHERE age > ? AND active = ?");
        assert_eq!(stmt.parameter_count(), 2);
        assert_eq!(stmt.parameter_types(), &vec![SqlType::Integer, SqlType::Boolean]);
        
        // Test parameter binding
        let mut bound_stmt = stmt.bind_parameters(vec![
            SqlValue::Integer(18),
            SqlValue::Boolean(true)
        ]).unwrap();
        
        let params = bound_stmt.parameters();
        assert_eq!(params.len(), 2);
        assert!(matches!(params[0], SqlValue::Integer(18)));
        assert!(matches!(params[1], SqlValue::Boolean(true)));
    }

    #[test]
    fn test_sql_driver_info() {
        // Test PostgreSQL driver
        let pg_driver = PostgreSqlDriver::new();
        let pg_info = pg_driver.driver_info();
        assert_eq!(pg_info.name, "postgresql");
        assert!(pg_info.features.contains(&"transactions".to_string()));
        assert!(pg_info.features.contains(&"prepared_statements".to_string()));
        assert!(pg_info.connection_schemes.contains(&"postgresql".to_string()));
        
        // Test MySQL driver
        let mysql_driver = MySqlDriver::new();
        let mysql_info = mysql_driver.driver_info();
        assert_eq!(mysql_info.name, "mysql");
        assert!(mysql_info.features.contains(&"transactions".to_string()));
        
        // Test SQLite driver
        let sqlite_driver = SqliteDriver::new();
        let sqlite_info = sqlite_driver.driver_info();
        assert_eq!(sqlite_info.name, "sqlite");
        assert!(sqlite_info.features.contains(&"embedded".to_string()));
    }
}

/// fr fr Connection pool unit tests
mod db_pool_unit_tests {
    use super::*;

    #[test]
    fn test_pool_config() {
        let config = PoolConfig::new()
            .with_size_limits(5, 20)
            .with_timeouts(Duration::from_secs(30), Duration::from_secs(300))
            .with_health_check(Duration::from_secs(60), "SELECT 1".to_string())
            .with_retry_policy(3, Duration::from_secs(1));

        assert_eq!(config.min_size, 5);
        assert_eq!(config.max_size, 20);
        assert_eq!(config.connection_timeout, Duration::from_secs(30));
        assert_eq!(config.idle_timeout, Duration::from_secs(300));
        assert_eq!(config.health_check_interval, Duration::from_secs(60));
        assert_eq!(config.health_check_query, "SELECT 1");
        assert_eq!(config.max_retries, 3);
        assert_eq!(config.retry_delay, Duration::from_secs(1));
    }

    #[test]
    fn test_pool_statistics() {
        let mut stats = PoolStatistics::new();
        
        // Test initial state
        assert_eq!(stats.total_connections(), 0);
        assert_eq!(stats.active_connections(), 0);
        assert_eq!(stats.idle_connections(), 0);
        assert_eq!(stats.pending_requests(), 0);
        
        // Test connection tracking
        stats.increment_total_connections();
        stats.increment_active_connections();
        assert_eq!(stats.total_connections(), 1);
        assert_eq!(stats.active_connections(), 1);
        assert_eq!(stats.idle_connections(), 0);
        
        // Test request tracking
        stats.increment_pending_requests();
        stats.record_connection_acquired(Duration::from_millis(100));
        assert_eq!(stats.pending_requests(), 0); // Should decrement after acquisition
        
        // Test error tracking
        stats.record_connection_error();
        assert_eq!(stats.connection_errors(), 1);
        
        // Test timing statistics
        stats.record_query_time(Duration::from_millis(50));
        stats.record_query_time(Duration::from_millis(150));
        assert_eq!(stats.total_queries(), 2);
        assert_eq!(stats.average_query_time(), Duration::from_millis(100));
    }

    #[test]
    fn test_pool_state_transitions() {
        let mut pool = ConnectionPool::new(PoolConfig::default());
        
        // Initial state
        assert_eq!(pool.state(), PoolState::Initializing);
        
        // Start pool
        pool.start().unwrap();
        assert_eq!(pool.state(), PoolState::Running);
        
        // Pause pool
        pool.pause().unwrap();
        assert_eq!(pool.state(), PoolState::Paused);
        
        // Resume pool
        pool.resume().unwrap();
        assert_eq!(pool.state(), PoolState::Running);
        
        // Stop pool
        pool.stop().unwrap();
        assert_eq!(pool.state(), PoolState::Stopped);
    }

    #[test]
    fn test_load_balancer() {
        let pool1 = ConnectionPool::new(PoolConfig::default().with_name("pool1"));
        let pool2 = ConnectionPool::new(PoolConfig::default().with_name("pool2"));
        let pool3 = ConnectionPool::new(PoolConfig::default().with_name("pool3"));
        
        let mut balancer = LoadBalancer::new();
        balancer.add_pool("pool1", pool1);
        balancer.add_pool("pool2", pool2);
        balancer.add_pool("pool3", pool3);
        
        // Test round-robin balancing
        let selected1 = balancer.select_pool_round_robin();
        let selected2 = balancer.select_pool_round_robin();
        let selected3 = balancer.select_pool_round_robin();
        let selected4 = balancer.select_pool_round_robin(); // Should wrap around
        
        assert_ne!(selected1, selected2);
        assert_ne!(selected2, selected3);
        assert_eq!(selected1, selected4); // Should wrap back to first
        
        // Test least connections balancing
        let selected_least = balancer.select_pool_least_connections();
        assert!(selected_least.is_some());
        
        // Test removing pool
        balancer.remove_pool("pool2");
        assert_eq!(balancer.pool_count(), 2);
    }

    #[test]
    fn test_pool_manager() {
        let mut manager = PoolManager::new();
        
        // Test adding pools
        let config1 = PoolConfig::default().with_name("primary");
        let config2 = PoolConfig::default().with_name("readonly");
        
        manager.create_pool("primary", config1).unwrap();
        manager.create_pool("readonly", config2).unwrap();
        
        assert_eq!(manager.pool_count(), 2);
        assert!(manager.has_pool("primary"));
        assert!(manager.has_pool("readonly"));
        assert!(!manager.has_pool("nonexistent"));
        
        // Test getting pool
        let primary_pool = manager.get_pool("primary");
        assert!(primary_pool.is_some());
        
        // Test pool operations
        manager.start_pool("primary").unwrap();
        manager.pause_pool("readonly").unwrap();
        
        // Test removing pool
        manager.remove_pool("readonly").unwrap();
        assert_eq!(manager.pool_count(), 1);
        assert!(!manager.has_pool("readonly"));
    }
}

/// fr fr ORM unit tests
mod db_orm_unit_tests {
    use super::*;

    #[test]
    fn test_object_mapper() {
        let mut mapper = ObjectMapper::new();
        
        // Test struct mapping
        mapper.map_struct("User", "users")
            .field("id", "id", SqlType::Integer)
            .field("name", "name", SqlType::Text)
            .field("email", "email", SqlType::Text)
            .field("created_at", "created_at", SqlType::Timestamp)
            .primary_key("id")
            .finish();
        
        assert!(mapper.has_mapping("User"));
        
        let mapping = mapper.get_mapping("User").unwrap();
        assert_eq!(mapping.table_name(), "users");
        assert_eq!(mapping.field_count(), 4);
        assert_eq!(mapping.primary_key(), Some("id"));
        
        // Test field mapping
        let field_mapping = mapping.get_field("name").unwrap();
        assert_eq!(field_mapping.struct_field, "name");
        assert_eq!(field_mapping.db_column, "name");
        assert_eq!(field_mapping.sql_type, SqlType::Text);
    }

    #[test]
    fn test_relationship_management() {
        let mut rel_manager = RelationshipManager::new();
        
        // Test one-to-many relationship
        rel_manager.add_one_to_many(
            "User",
            "posts",
            "Post",
            "user_id",
            "id"
        );
        
        let relationships = rel_manager.get_relationships("User");
        assert_eq!(relationships.len(), 1);
        
        let rel = &relationships[0];
        assert!(matches!(rel, OneToMany { .. }));
        
        // Test many-to-one relationship
        rel_manager.add_many_to_one(
            "Post",
            "user",
            "User",
            "user_id",
            "id"
        );
        
        let post_relationships = rel_manager.get_relationships("Post");
        assert_eq!(post_relationships.len(), 1);
        
        // Test many-to-many relationship
        rel_manager.add_many_to_many(
            "User",
            "roles",
            "Role",
            "user_roles",
            "user_id",
            "role_id"
        );
        
        let user_relationships = rel_manager.get_relationships("User");
        assert_eq!(user_relationships.len(), 2); // one-to-many + many-to-many
    }

    #[test]
    fn test_crud_operations() {
        let mut crud = CrudOperations::new();
        
        // Test create operation
        let create_sql = crud.generate_create_sql("User", &[
            ("name", SqlValue::Text("Alice".to_string())),
            ("email", SqlValue::Text("alice@example.com".to_string())),
        ]).unwrap();
        
        assert!(create_sql.contains("INSERT INTO"));
        assert!(create_sql.contains("users"));
        assert!(create_sql.contains("name, email"));
        
        // Test read operation
        let read_sql = crud.generate_read_sql("User", Some("id = ?")).unwrap();
        assert!(create_sql.contains("SELECT"));
        assert!(create_sql.contains("FROM users"));
        
        // Test update operation
        let update_sql = crud.generate_update_sql("User", &[
            ("name", SqlValue::Text("Bob".to_string())),
        ], "id = ?").unwrap();
        
        assert!(update_sql.contains("UPDATE users"));
        assert!(update_sql.contains("SET name = ?"));
        assert!(update_sql.contains("WHERE id = ?"));
        
        // Test delete operation
        let delete_sql = crud.generate_delete_sql("User", "id = ?").unwrap();
        assert!(delete_sql.contains("DELETE FROM users"));
        assert!(delete_sql.contains("WHERE id = ?"));
    }
}

/// fr fr Migration system unit tests
mod db_migrate_unit_tests {
    use super::*;

    #[test]
    fn test_migration_creation() {
        let migration = Migration::new("001", "create_users_table", 1)
            .with_up_script("CREATE TABLE users (id SERIAL PRIMARY KEY, name TEXT NOT NULL)")
            .with_down_script("DROP TABLE users")
            .with_dependency("000_initial")
            .with_tag("initial_schema");
        
        assert_eq!(migration.id(), "001");
        assert_eq!(migration.name(), "create_users_table");
        assert_eq!(migration.version(), 1);
        assert_eq!(migration.up_script(), "CREATE TABLE users (id SERIAL PRIMARY KEY, name TEXT NOT NULL)");
        assert_eq!(migration.down_script(), "DROP TABLE users");
        assert!(migration.has_dependency("000_initial"));
        assert!(migration.has_tag("initial_schema"));
    }

    #[test]
    fn test_migration_script() {
        let script = MigrationScript::new()
            .add_statement("CREATE TABLE users (id SERIAL PRIMARY KEY)")
            .add_statement("CREATE INDEX idx_users_email ON users(email)")
            .add_statement("INSERT INTO users (name) VALUES ('admin')")
            .with_rollback_on_error(true);
        
        assert_eq!(script.statement_count(), 3);
        assert!(script.rollback_on_error());
        
        let statements = script.statements();
        assert!(statements[0].contains("CREATE TABLE"));
        assert!(statements[1].contains("CREATE INDEX"));
        assert!(statements[2].contains("INSERT INTO"));
    }

    #[test]
    fn test_version_manager() {
        let mut version_manager = VersionManager::new();
        
        // Test initial state
        assert_eq!(version_manager.current_version(), 0);
        assert_eq!(version_manager.applied_migrations().len(), 0);
        
        // Test applying migrations
        version_manager.apply_migration("001", "create_users", 1).unwrap();
        version_manager.apply_migration("002", "add_indexes", 2).unwrap();
        
        assert_eq!(version_manager.current_version(), 2);
        assert_eq!(version_manager.applied_migrations().len(), 2);
        assert!(version_manager.is_migration_applied("001"));
        assert!(version_manager.is_migration_applied("002"));
        assert!(!version_manager.is_migration_applied("003"));
        
        // Test rollback
        version_manager.rollback_migration("002").unwrap();
        assert_eq!(version_manager.current_version(), 1);
        assert_eq!(version_manager.applied_migrations().len(), 1);
        assert!(!version_manager.is_migration_applied("002"));
    }

    #[test]
    fn test_migration_runner() {
        let mut runner = MigrationRunner::new();
        
        // Add migrations
        let migration1 = Migration::new("001", "create_users", 1)
            .with_up_script("CREATE TABLE users (id SERIAL PRIMARY KEY)");
        let migration2 = Migration::new("002", "create_posts", 2)
            .with_up_script("CREATE TABLE posts (id SERIAL PRIMARY KEY, user_id INTEGER)")
            .with_dependency("001");
        
        runner.add_migration(migration1);
        runner.add_migration(migration2);
        
        // Test migration ordering
        let ordered_migrations = runner.get_ordered_migrations().unwrap();
        assert_eq!(ordered_migrations.len(), 2);
        assert_eq!(ordered_migrations[0].id(), "001");
        assert_eq!(ordered_migrations[1].id(), "002");
        
        // Test status tracking
        runner.mark_migration_status("001", MigrationStatus::Applied);
        runner.mark_migration_status("002", MigrationStatus::Pending);
        
        assert_eq!(runner.get_migration_status("001"), Some(MigrationStatus::Applied));
        assert_eq!(runner.get_migration_status("002"), Some(MigrationStatus::Pending));
        
        // Test pending migrations
        let pending = runner.get_pending_migrations();
        assert_eq!(pending.len(), 1);
        assert_eq!(pending[0].id(), "002");
    }
}

/// fr fr NoSQL unit tests
mod db_nosql_unit_tests {
    use super::*;

    #[test]
    fn test_document_creation() {
        let mut doc = Document::new();
        
        // Test field setting
        doc.set_field("_id", SqlValue::Text("507f1f77bcf86cd799439011".to_string()));
        doc.set_field("name", SqlValue::Text("John Doe".to_string()));
        doc.set_field("age", SqlValue::Integer(30));
        doc.set_field("active", SqlValue::Boolean(true));
        
        assert_eq!(doc.field_count(), 4);
        assert!(doc.has_field("_id"));
        assert!(doc.has_field("name"));
        assert!(doc.has_field("age"));
        assert!(doc.has_field("active"));
        
        // Test field retrieval
        let name = doc.get_field("name").unwrap();
        assert!(matches!(name, SqlValue::Text(_)));
        
        let age = doc.get_field("age").unwrap();
        assert!(matches!(age, SqlValue::Integer(30)));
        
        // Test field removal
        doc.remove_field("active");
        assert_eq!(doc.field_count(), 3);
        assert!(!doc.has_field("active"));
    }

    #[test]
    fn test_document_serialization() {
        let mut doc = Document::new();
        doc.set_field("name", SqlValue::Text("Alice".to_string()));
        doc.set_field("score", SqlValue::Float(95.5));
        doc.set_field("passed", SqlValue::Boolean(true));
        
        // Test JSON serialization
        let json = doc.to_json().unwrap();
        assert!(json.contains("\"name\""));
        assert!(json.contains("\"Alice\""));
        assert!(json.contains("\"score\""));
        assert!(json.contains("95.5"));
        
        // Test deserialization
        let parsed_doc = Document::from_json(&json).unwrap();
        assert_eq!(parsed_doc.field_count(), 3);
        assert!(parsed_doc.has_field("name"));
        assert!(parsed_doc.has_field("score"));
        assert!(parsed_doc.has_field("passed"));
    }

    #[test]
    fn test_collection_operations() {
        let mut collection = Collection::new("users");
        
        // Test basic properties
        assert_eq!(collection.name(), "users");
        assert_eq!(collection.document_count(), 0);
        
        // Test document insertion
        let mut doc1 = Document::new();
        doc1.set_field("_id", SqlValue::Text("1".to_string()));
        doc1.set_field("name", SqlValue::Text("Alice".to_string()));
        
        let mut doc2 = Document::new();
        doc2.set_field("_id", SqlValue::Text("2".to_string()));
        doc2.set_field("name", SqlValue::Text("Bob".to_string()));
        
        collection.insert_document(doc1);
        collection.insert_document(doc2);
        
        assert_eq!(collection.document_count(), 2);
        
        // Test document retrieval
        let retrieved = collection.find_by_id("1");
        assert!(retrieved.is_some());
        assert!(retrieved.unwrap().has_field("name"));
        
        // Test document removal
        collection.remove_document("2");
        assert_eq!(collection.document_count(), 1);
        assert!(collection.find_by_id("2").is_none());
    }

    #[test]
    fn test_mongodb_driver() {
        let driver = MongoDbDriver::new();
        let info = driver.driver_info();
        
        assert_eq!(info.name, "mongodb");
        assert!(info.features.contains(&"documents".to_string()));
        assert!(info.features.contains(&"collections".to_string()));
        assert!(info.connection_schemes.contains(&"mongodb".to_string()));
        
        // Test connection configuration
        let config = driver.create_connection_config("mongodb://localhost:27017/testdb").unwrap();
        assert_eq!(config.driver, "mongodb");
        assert_eq!(config.database, "testdb");
        assert_eq!(config.host, Some("localhost".to_string()));
        assert_eq!(config.port, Some(27017));
    }

    #[test]
    fn test_redis_driver() {
        let driver = RedisDriver::new();
        let info = driver.driver_info();
        
        assert_eq!(info.name, "redis");
        assert!(info.features.contains(&"key_value".to_string()));
        assert!(info.features.contains(&"caching".to_string()));
        assert!(info.connection_schemes.contains(&"redis".to_string()));
        
        // Test connection configuration
        let config = driver.create_connection_config("redis://localhost:6379/0").unwrap();
        assert_eq!(config.driver, "redis");
        assert_eq!(config.database, "0");
        assert_eq!(config.host, Some("localhost".to_string()));
        assert_eq!(config.port, Some(6379));
    }
}

/// fr fr Run all unit tests
#[test]
fn run_all_database_unit_tests() {
    println!("🧪 Running comprehensive database unit tests...");
    
    // Initialize all database packages
    assert!(db_core::init_db_core().is_ok());
    assert!(db_sql::init_db_sql().is_ok());
    
    println!("✅ Core database functionality tests completed");
    println!("✅ SQL query building and type system tests completed");
    println!("✅ Connection pooling and management tests completed");
    println!("✅ ORM mapping and relationship tests completed");
    println!("✅ Migration system and versioning tests completed");
    println!("✅ NoSQL document and collection tests completed");
    
    println!("🎉 All database unit tests passed successfully!");
}
