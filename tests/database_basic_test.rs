/// fr fr Basic database functionality tests for SQLSlay
/// 
/// These tests validate the fundamental database operations including
/// connection management, query execution, and error handling.
/// 
/// Why these tests are critical for database reliability:
/// - Database operations involve external state that must be validated
/// - Connection pooling requires proper resource management testing
/// - Error scenarios must be handled gracefully to prevent data corruption
/// - Type conversion between CURSED and SQL types needs validation
/// - Transaction isolation and consistency must be verified

#[cfg(test)]
mod tests {
    use cursed::stdlib::database::{
        DB, SqlValue, DatabaseError, DatabaseConfig,
        driver::{register_driver, MockDriver}
    };

    /// fr fr Test basic database connection opening
    #[test]
    fn test_database_connection_opening() {
        // Register mock driver for testing
        let mock_driver = MockDriver::new("test_mock".to_string());
        register_driver("test_mock".to_string(), Box::new(mock_driver)).unwrap();

        // Test database opening
        let result = DB::open("test_mock".to_string(), "test://localhost/test".to_string());
        assert!(result.is_ok(), "Database connection should open successfully");

        let db = result.unwrap();
        assert_eq!(db.driver_name, "test_mock");
        assert_eq!(db.data_source_name, "test://localhost/test");
    }

    /// fr fr Test SQL value types and conversions
    #[test]
    fn test_sql_value_types() {
        // Test various SQL value types
        let null_val = SqlValue::Null;
        let bool_val = SqlValue::Boolean(true);
        let int_val = SqlValue::Integer(42);
        let float_val = SqlValue::Float(3.14);
        let string_val = SqlValue::String("test".to_string());
        let bytes_val = SqlValue::Bytes(vec![1, 2, 3, 4]);

        // Test display formatting
        assert_eq!(format!("{}", null_val), "NULL");
        assert_eq!(format!("{}", bool_val), "true");
        assert_eq!(format!("{}", int_val), "42");
        assert_eq!(format!("{}", float_val), "3.14");
        assert_eq!(format!("{}", string_val), "'test'");
        assert_eq!(format!("{}", bytes_val), "BLOB(4 bytes)");
    }

    /// fr fr Test database error creation and categorization
    #[test]
    fn test_database_error_handling() {
        use cursed::stdlib::database::{DatabaseErrorKind, error::DatabaseError};

        // Test error creation
        let conn_error = DatabaseError::connection_error("Connection failed");
        assert_eq!(conn_error.kind, DatabaseErrorKind::ConnectionError);
        assert_eq!(conn_error.message, "Connection failed");

        let query_error = DatabaseError::query_error("Invalid SQL syntax");
        assert_eq!(query_error.kind, DatabaseErrorKind::QueryError);

        // Test error categorization
        assert!(conn_error.is_retryable());
        assert!(!query_error.is_retryable());

        // Test severity levels
        use cursed::stdlib::database::error::ErrorSeverity;
        assert_eq!(conn_error.severity(), ErrorSeverity::Warning);
        assert_eq!(query_error.severity(), ErrorSeverity::Error);
    }

    /// fr fr Test driver registration and retrieval
    #[test]
    fn test_driver_registry() {
        use cursed::stdlib::database::driver::{get_driver, has_driver, list_drivers};

        // Register a new mock driver for this test
        let mock_driver = MockDriver::new("test_registry".to_string());
        let register_result = register_driver("test_registry".to_string(), Box::new(mock_driver));
        
        // It's okay if the driver is already registered (from other tests)
        if register_result.is_err() {
            // Driver might already be registered, which is fine
        }

        // Test driver retrieval
        let driver_result = get_driver("test_registry");
        assert!(driver_result.is_ok(), "Registered driver should be retrievable: {:?}", driver_result.err());

        // Test driver listing
        let drivers = list_drivers().unwrap();
        assert!(drivers.contains(&"test_registry".to_string()), "Driver list should contain test_registry: {:?}", drivers);
    }

    /// fr fr Test database configuration
    #[test]
    fn test_database_configuration() {
        let config = DatabaseConfig::default();
        
        // Verify default configuration values
        assert_eq!(config.max_open_connections, 100);
        assert_eq!(config.max_idle_connections, 10);
        assert_eq!(config.connection_max_lifetime_seconds, 3600);
        assert_eq!(config.query_timeout_seconds, 300);
        assert!(config.enable_pool_monitoring);
        assert_eq!(config.max_retry_attempts, 3);
    }

    /// fr fr Test SQL isolation levels
    #[test]
    fn test_isolation_levels() {
        use cursed::stdlib::database::SqlIsolationLevel;

        let levels = vec![
            SqlIsolationLevel::LevelDefault,
            SqlIsolationLevel::LevelReadCommitted,
            SqlIsolationLevel::LevelSerializable,
        ];

        for level in levels {
            let level_str = format!("{}", level);
            assert!(!level_str.is_empty(), "Isolation level should have string representation");
        }
    }

    /// fr fr Test query builders basic functionality
    #[test]
    fn test_query_builders() {
        use cursed::stdlib::database::builder::{
            new_select_builder, new_insert_builder, new_update_builder, new_delete_builder
        };
        use std::collections::HashMap;

        // Test SELECT builder
        let select_builder = new_select_builder("users".to_string())
            .columns(vec!["id".to_string(), "name".to_string()])
            .r#where("age > ?".to_string(), vec![SqlValue::Integer(18)])
            .order_by("name ASC".to_string())
            .limit(10);

        let (query, params) = select_builder.build();
        assert!(query.contains("SELECT id, name FROM users"));
        assert!(query.contains("WHERE age > ?"));
        assert!(query.contains("ORDER BY name ASC"));
        assert!(query.contains("LIMIT 10"));
        assert_eq!(params.len(), 1);

        // Test INSERT builder
        let insert_builder = new_insert_builder("users".to_string())
            .columns(vec!["name".to_string(), "age".to_string()])
            .values(vec![SqlValue::String("John".to_string()), SqlValue::Integer(25)]);

        let (insert_query, insert_params) = insert_builder.build();
        assert!(insert_query.contains("INSERT INTO users"));
        assert!(insert_query.contains("(name, age)"));
        assert_eq!(insert_params.len(), 2);

        // Test UPDATE builder
        let mut update_data = HashMap::new();
        update_data.insert("name".to_string(), SqlValue::String("Jane".to_string()));
        update_data.insert("age".to_string(), SqlValue::Integer(30));

        let update_builder = new_update_builder("users".to_string())
            .set_map(update_data)
            .r#where("id = ?".to_string(), vec![SqlValue::Integer(1)]);

        let (update_query, update_params) = update_builder.build();
        assert!(update_query.contains("UPDATE users SET"));
        assert!(update_query.contains("WHERE id = ?"));
        assert!(update_params.len() >= 3); // 2 SET params + 1 WHERE param

        // Test DELETE builder
        let delete_builder = new_delete_builder("users".to_string())
            .r#where("age < ?".to_string(), vec![SqlValue::Integer(18)]);

        let (delete_query, delete_params) = delete_builder.build();
        assert!(delete_query.contains("DELETE FROM users"));
        assert!(delete_query.contains("WHERE age < ?"));
        assert_eq!(delete_params.len(), 1);
    }

    /// fr fr Test migration system
    #[test]
    fn test_migration_system() {
        use cursed::stdlib::database::{Migration, MigrationStatus};

        // Create a test migration
        let migration = Migration::new(
            1,
            "Create users table".to_string(),
            "CREATE TABLE users (id INT PRIMARY KEY, name VARCHAR(255));".to_string(),
            "DROP TABLE users;".to_string(),
        );

        assert_eq!(migration.version, 1);
        assert_eq!(migration.description, "Create users table");
        assert!(migration.up.contains("CREATE TABLE users"));
        assert!(migration.down.contains("DROP TABLE users"));

        // Test migration status
        assert_eq!(MigrationStatus::Pending, MigrationStatus::Pending);
        assert_ne!(MigrationStatus::Pending, MigrationStatus::Applied);
    }

    /// fr fr Test connection pool configuration
    #[test]
    fn test_connection_pool_config() {
        use cursed::stdlib::database::pool::{PoolConfig, ConnectionPoolBuilder};
        use std::time::Duration;

        let config = PoolConfig::default();
        assert_eq!(config.max_open_connections, 100);
        assert_eq!(config.max_idle_connections, 10);

        // Test pool builder
        let _builder = ConnectionPoolBuilder::new()
            .max_open_connections(50)
            .max_idle_connections(5)
            .connection_max_lifetime(Duration::from_secs(1800))
            .validate_connections(true);

        // Builder should be constructible (we can't access private config field)
        // The fact that this compiles means the builder pattern works correctly
    }

    /// fr fr Test LLVM integration structures
    #[test]
    fn test_llvm_integration_types() {
        use cursed::stdlib::database::llvm_integration::{
            DatabaseFunctionRegistry, DatabaseFunction, FunctionSignature, 
            ParameterType, ReturnType, FunctionImplementation
        };

        let mut registry = DatabaseFunctionRegistry::new();

        let test_function = DatabaseFunction {
            name: "test_func".to_string(),
            signature: FunctionSignature {
                parameters: vec![ParameterType::String, ParameterType::Integer],
                return_type: ReturnType::Boolean,
                can_fail: true,
            },
            implementation: FunctionImplementation::Native("test_impl".to_string()),
        };

        registry.register_function("test_func".to_string(), test_function);
        
        let retrieved = registry.get_function("test_func");
        assert!(retrieved.is_some());
        
        let func = retrieved.unwrap();
        assert_eq!(func.name, "test_func");
        assert_eq!(func.signature.parameters.len(), 2);
        assert!(func.signature.can_fail);
    }
}
