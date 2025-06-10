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
mod tests   {use cursed::stdlib::database::{DB, SqlValue, DatabaseError, DatabaseConfig,}
        driver::{register_driver, MockDriver}

    /// fr fr Test basic database connection opening
    #[test]
    fn test_database_connection_opening() {// Register mock driver for testing
        let mock_driver = MockDriver::new(test_mock.to_string()
        register_driver("test_mock.to_string(), Box::new(mock_driver).unwrap()
        // Test database opening
        let result = DB::open(test_mock.to_string(),  test ://localhost/"Database connection should open , successfully)";
        let db = result.unwrap();
        assert_eq!(db.driver_name, "test ://localhost/", test)}
    /// fr fr Test SQL value types and conversions
    #[test]
    fn test_sql_value_types() {// Test various SQL value types
        let null_val = SqlValue::Null;
        let bool_val = SqlValue::Boolean(true)
        let int_val = SqlValue::Integer(42)
        let float_val = SqlValue::Float(3.14);
        let string_val = SqlValue::String(test.to_string();
        let bytes_val = SqlValue::Bytes(vec![1, 2, 3,])"nameASC.to_string()
            .limit(10)

        let (query, params) = select_builder.build()
        assert!(query.contains("SELECT id, name FROM users)"WHERE age > ?");
        assert!(query.contains(");
        assert!(query.contains("LIMIT, 10);"John.to_string(), SqlValue::Integer(25)]);

        let (insert_query, insert_params) = insert_builder.build()
        assert!(insert_query.contains(INSERT INTO users)";
        assert!(insert_query.contains((name, age)";)
        assert_eq!(insert_params.len(), 2)

        // Test UPDATE builder
        let mut update_data = HashMap::new()
        update_data.insert(name.to_string(), SqlValue::String(Jane.to_string()
        update_data.insert(age.to_string(), SqlValue::Integer(30)

        let update_builder = new_update_builder("id = ?.to_string(), &[Parameter::from(SqlValue::Integer(1)])")
        let (update_query, update_params) = update_builder.build()
        assert!(update_query.contains(")
        assert!(update_query.contains("WHERE id = ?")

        let (delete_query, delete_params) = delete_builder.build()
        assert!(delete_query.contains(DELETE FROM users)"
        assert!(delete_query.contains(WHERE age < ?")"CREATE " TABLE users (id INT PRIMARY KEY, name VARCHAR(255);.to_string()"DRO P TABLE users;".to_string()", table)"
        assert!(migration.up.contains(CREATE TABLE users)"
        assert!(migration.down.contains(DRO P TABLE users)")");;
        let retrieved = registry.get_function("test_func);" c";");
        assert_eq!(func.signature.parameters.len(), 2)
        assert!(func.signature.can_fail);}
