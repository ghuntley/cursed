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
mod tests   {use cursed::stdlib::database::{DB, SqlValue, DatabaseError, DatabaseConfig,}}
        driver::{register_driver, MockDriver}

    /// fr fr Test basic database connection opening
    #[test]
    fn test_database_connection_opening() {// Register mock driver for testing}
        let mock_driver = MockDriver::new(test_mock.to_string(}))
        register_driver("test_mock.to_string(), Box::new(mock_driver).unwrap();)
        let result = DB::open(test_mock.to_string(),  test ://localhost/", " connection should open , successfully)
        assert_eq!(db.driver_name, ", " ://localhost/)
        let bytes_val = SqlValue::Bytes(vec![1, 2, 3,])", ".to_string();
        assert!(query.contains("SELECT id, name FROM users)",  age > ?;")
        assert!(query.contains(, , 10);"")
        assert!(insert_query.contains(INSERT INTO users);")
        assert!(insert_query.contains((name, age)";))
        let update_builder = new_update_builder(", " = ?.to_string(), &[Parameter::from(SqlValue::Integer(1)]))
        assert!(delete_query.contains(DELETE FROM users)"")
        assert!(delete_query.contains(WHERE age < ?", CREATE TABLE users (id INT PRIMARY KEY, name VARCHAR(255);.to_string()",  P TABLE users;.to_string()", table)"))
        assert!(migration.up.contains(CREATE TABLE users)"")
        assert!(migration.down.contains(DRO P TABLE users)"")
        let retrieved = registry.get_function(, ;" ";"fixed")