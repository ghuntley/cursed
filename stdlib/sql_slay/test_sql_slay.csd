// SQL Slay Module Tests
// Comprehensive test suite for database operations

yeet "testz"
yeet "sql_slay"

// Test Database Connection Management
slay test_db_connection() {
    test_start("Database Connection Tests")
    
    // Test initial connection state
    assert_false(db_is_connected())
    
    // Test connection
    sus connected lit = db_connect("localhost", 5432, "testdb", "testuser", "testpass")
    assert_true(connected)
    assert_true(db_is_connected())
    
    // Test connection info
    sus info tea = db_get_connection_info()
    assert_eq_string(info, "localhost:5432/testdb")
    
    // Test disconnection
    sus disconnected lit = db_disconnect()
    assert_true(disconnected)
    assert_false(db_is_connected())
}

// Test SQL Query Builder Functions
slay test_sql_builders() {
    test_start("SQL Query Builder Tests")
    
    // Test SELECT query building
    sus select_query tea = sql_select("users", "*", "age > 18")
    assert_eq_string(select_query, "SELECT * FROM users WHERE age > 18")
    
    sus select_all tea = sql_select("users", "*", "")
    assert_eq_string(select_all, "SELECT * FROM users")
    
    // Test INSERT query building
    sus insert_query tea = sql_insert("users", "name, age", "'John', 30")
    assert_eq_string(insert_query, "INSERT INTO users (name, age) VALUES ('John', 30)")
    
    // Test UPDATE query building
    sus update_query tea = sql_update("users", "age = 31", "name = 'John'")
    assert_eq_string(update_query, "UPDATE users SET age = 31 WHERE name = 'John'")
    
    sus update_all tea = sql_update("users", "active = 1", "")
    assert_eq_string(update_all, "UPDATE users SET active = 1")
    
    // Test DELETE query building
    sus delete_query tea = sql_delete("users", "age < 18")
    assert_eq_string(delete_query, "DELETE FROM users WHERE age < 18")
    
    sus delete_all tea = sql_delete("users", "")
    assert_eq_string(delete_all, "DELETE FROM users")
}

// Test SQL Execution Functions
slay test_sql_execution() {
    test_start("SQL Execution Tests")
    
    // Test execution without connection
    sus result lit = sql_execute("SELECT * FROM users")
    assert_false(result)
    
    sus rows normie = sql_execute_insert("INSERT INTO users VALUES (1, 'John')")
    assert_eq_int(rows, 0)
    
    // Test execution with connection
    db_connect("localhost", 5432, "testdb", "testuser", "testpass")
    
    sus exec_result lit = sql_execute("SELECT * FROM users")
    assert_true(exec_result)
    
    sus select_result tea = sql_execute_select("SELECT * FROM users")
    assert_eq_string(select_result, "id:1,name:John,age:30|id:2,name:Jane,age:25")
    
    sus insert_rows normie = sql_execute_insert("INSERT INTO users VALUES (3, 'Bob')")
    assert_eq_int(insert_rows, 1)
    
    sus update_rows normie = sql_execute_update("UPDATE users SET age = 31 WHERE name = 'John'")
    assert_eq_int(update_rows, 1)
    
    sus delete_rows normie = sql_execute_delete("DELETE FROM users WHERE age < 18")
    assert_eq_int(delete_rows, 1)
    
    db_disconnect()
}

// Test Transaction Management
slay test_transactions() {
    test_start("Transaction Management Tests")
    
    // Test transaction without connection
    sus begin_result lit = sql_begin_transaction()
    assert_false(begin_result)
    
    // Test transaction with connection
    db_connect("localhost", 5432, "testdb", "testuser", "testpass")
    
    assert_false(sql_in_transaction())
    
    sus begin_ok lit = sql_begin_transaction()
    assert_true(begin_ok)
    assert_true(sql_in_transaction())
    
    sus commit_ok lit = sql_commit()
    assert_true(commit_ok)
    assert_false(sql_in_transaction())
    
    // Test rollback
    sql_begin_transaction()
    assert_true(sql_in_transaction())
    
    sus rollback_ok lit = sql_rollback()
    assert_true(rollback_ok)
    assert_false(sql_in_transaction())
    
    db_disconnect()
}

// Test Table Management
slay test_table_management() {
    test_start("Table Management Tests")
    
    // Test CREATE TABLE
    sus create_query tea = sql_create_table("users", "id INTEGER PRIMARY KEY, name TEXT")
    assert_eq_string(create_query, "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT)")
    
    // Test DROP TABLE
    sus drop_query tea = sql_drop_table("users")
    assert_eq_string(drop_query, "DROP TABLE users")
    
    // Test ALTER TABLE
    sus alter_query tea = sql_alter_table("users", "ADD COLUMN email TEXT")
    assert_eq_string(alter_query, "ALTER TABLE users ADD COLUMN email TEXT")
}

// Test Utility Functions
slay test_utilities() {
    test_start("Utility Functions Tests")
    
    // Test string escaping
    sus escaped tea = sql_escape_string("test'string")
    assert_eq_string(escaped, "test'string")  // Simple implementation
    
    // Test table name validation
    assert_true(sql_validate_table_name("users"))
    assert_false(sql_validate_table_name(""))
    
    // Test column name validation
    assert_true(sql_validate_column_name("name"))
    assert_false(sql_validate_column_name(""))
    
    // Test result parsing
    sus result_count normie = sql_parse_results("id:1,name:John|id:2,name:Jane")
    assert_eq_int(result_count, 2)
    
    sus empty_count normie = sql_parse_results("")
    assert_eq_int(empty_count, 0)
}

// Test Schema Functions
slay test_schema_functions() {
    test_start("Schema Functions Tests")
    
    // Test get column names
    sus columns tea = sql_get_column_names("users")
    assert_eq_string(columns, "id,name,age,email")
    
    // Test get table schema
    sus schema tea = sql_get_table_schema("users")
    assert_eq_string(schema, "id INTEGER PRIMARY KEY, name TEXT, age INTEGER, email TEXT")
}

// Test Connection Pool Management
slay test_connection_pool() {
    test_start("Connection Pool Tests")
    
    // Test pool initialization
    sus pool_init lit = sql_init_pool(10)
    assert_true(pool_init)
    
    // Test pool status
    sus status normie = sql_get_pool_status()
    assert_eq_int(status, 0)
    
    // Test connection acquisition
    sus acquired lit = sql_pool_acquire()
    assert_true(acquired)
    
    sus new_status normie = sql_get_pool_status()
    assert_eq_int(new_status, 1)
    
    // Test connection release
    sus released lit = sql_pool_release()
    assert_true(released)
    
    sus final_status normie = sql_get_pool_status()
    assert_eq_int(final_status, 0)
}

// Test Complex SQL Operations
slay test_complex_operations() {
    test_start("Complex SQL Operations Tests")
    
    db_connect("localhost", 5432, "testdb", "testuser", "testpass")
    
    // Test transaction with multiple operations
    sql_begin_transaction()
    
    sus insert1 normie = sql_execute_insert("INSERT INTO users VALUES (1, 'Alice')")
    assert_eq_int(insert1, 1)
    
    sus insert2 normie = sql_execute_insert("INSERT INTO users VALUES (2, 'Bob')")
    assert_eq_int(insert2, 1)
    
    sus update_result normie = sql_execute_update("UPDATE users SET age = 25 WHERE name = 'Alice'")
    assert_eq_int(update_result, 1)
    
    sql_commit()
    
    // Test batch operations
    sus results tea = sql_execute_select("SELECT * FROM users WHERE age > 20")
    sus count normie = sql_parse_results(results)
    assert_true(count > 0)
    
    db_disconnect()
}

// Test Error Handling
slay test_error_handling() {
    test_start("Error Handling Tests")
    
    // Test operations without connection
    assert_false(sql_execute("SELECT * FROM users"))
    assert_eq_int(sql_execute_insert("INSERT INTO users VALUES (1, 'test')"), 0)
    assert_eq_int(sql_execute_update("UPDATE users SET name = 'test'"), 0)
    assert_eq_int(sql_execute_delete("DELETE FROM users"), 0)
    
    // Test transaction operations without connection
    assert_false(sql_begin_transaction())
    assert_false(sql_commit())
    assert_false(sql_rollback())
    
    // Test with connection but invalid transaction state
    db_connect("localhost", 5432, "testdb", "testuser", "testpass")
    assert_false(sql_commit())  // No active transaction
    assert_false(sql_rollback())  // No active transaction
    
    db_disconnect()
}

// Test Performance Operations
slay test_performance() {
    test_start("Performance Tests")
    
    db_connect("localhost", 5432, "testdb", "testuser", "testpass")
    
    // Test multiple rapid operations
    sus i normie = 0
    sus success_count normie = 0
    
    bestie i := 0; i < 10; i++ {
        sus query tea = sql_select("users", "*", "id = " + i)
        sus result lit = sql_execute(query)
        bestie result {
            success_count = success_count + 1
        }
    }
    
    assert_eq_int(success_count, 10)
    
    db_disconnect()
}

// Run all tests
test_db_connection()
test_sql_builders()
test_sql_execution()
test_transactions()
test_table_management()
test_utilities()
test_schema_functions()
test_connection_pool()
test_complex_operations()
test_error_handling()
test_performance()

print_test_summary()
