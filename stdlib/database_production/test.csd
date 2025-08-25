yeet "testz"
yeet "database_production"

test_start("Database Production Module Tests")

fr fr === Connection Pool Tests ===
test_case("Database Pool Initialization") {
    sus result lit = db_pool_initialize(10)
    assert_eq_bool(result, based)
    
    fr fr Check initial state
    assert_eq_int(db_pool_active_count, 0)
    assert_eq_int(db_connections_in_use, 0)
}

test_case("Connection Pool Acquire") {
    sus connection_string tea = "postgresql://localhost:5432/test"
    sus conn_id normie = db_pool_acquire_connection(connection_string)
    
    assert_greater_than_or_equal(conn_id, 0)
    assert_less_than(conn_id, 10)
    assert_greater_than(db_connections_in_use, 0)
}

test_case("Connection Pool Release") {
    sus connection_string tea = "postgresql://localhost:5432/test"
    sus conn_id normie = db_pool_acquire_connection(connection_string)
    
    sus initial_in_use normie = db_connections_in_use
    sus result lit = db_pool_release_connection(conn_id)
    
    assert_eq_bool(result, based)
    assert_less_than(db_connections_in_use, initial_in_use)
}

test_case("Connection Pool Reuse") {
    sus connection_string tea = "postgresql://localhost:5432/reuse_test"
    
    sus conn1 normie = db_pool_acquire_connection(connection_string)
    db_pool_release_connection(conn1)
    
    sus conn2 normie = db_pool_acquire_connection(connection_string)
    assert_eq_int(conn1, conn2)  fr fr Should reuse same connection
}

fr fr === Database Driver Tests ===
test_case("PostgreSQL Connection") {
    sus conn_string tea = "postgresql://user:pass@localhost:5432/testdb"
    sus conn_id normie = postgres_connect(conn_string)
    
    assert_greater_than_or_equal(conn_id, 0)
    
    sus is_connected lit = postgres_is_connected(conn_id)
    assert_eq_bool(is_connected, based)
    
    postgres_disconnect(conn_id)
}

test_case("MySQL Connection") {
    sus conn_string tea = "mysql://user:pass@localhost:3306/testdb"
    sus conn_id normie = mysql_connect(conn_string)
    
    assert_greater_than_or_equal(conn_id, 0)
    
    sus is_connected lit = mysql_is_connected(conn_id)
    assert_eq_bool(is_connected, based)
    
    mysql_disconnect(conn_id)
}

test_case("SQLite Connection") {
    sus db_file tea = "/tmp/test.db"
    sus conn_id normie = sqlite_connect(db_file)
    
    assert_greater_than_or_equal(conn_id, 0)
    
    sus is_connected lit = sqlite_is_connected(conn_id)
    assert_eq_bool(is_connected, based)
    
    sqlite_disconnect(conn_id)
}

fr fr === Query Execution Tests ===
test_case("Basic Query Execution") {
    sus conn_id normie = sqlite_connect(":memory:")
    
    fr fr Create table
    sus create_sql tea = "CREATE TABLE users (id INTEGER PRIMARY KEY, name TEXT, email TEXT)"
    sus create_result = db_execute_query(conn_id, create_sql)
    assert_not_null(create_result)
    
    fr fr Insert data
    sus insert_sql tea = "INSERT INTO users (name, email) VALUES ('Test User', 'test@example.com')"
    sus insert_result = db_execute_query(conn_id, insert_sql)
    assert_not_null(insert_result)
    
    sqlite_disconnect(conn_id)
}

test_case("Prepared Statement Execution") {
    sus conn_id normie = sqlite_connect(":memory:")
    
    fr fr Create table
    sus create_sql tea = "CREATE TABLE products (id INTEGER PRIMARY KEY, name TEXT, price REAL)"
    db_execute_query(conn_id, create_sql)
    
    fr fr Prepare insert statement
    sus insert_sql tea = "INSERT INTO products (name, price) VALUES (?, ?)"
    sus stmt_id normie = db_prepare_statement(conn_id, insert_sql)
    assert_greater_than_or_equal(stmt_id, 0)
    
    fr fr Bind parameters and execute
    db_bind_parameter(stmt_id, 1, "Test Product")
    db_bind_parameter(stmt_id, 2, 29.99)
    sus result = db_execute_prepared(stmt_id)
    assert_not_null(result)
    
    db_finalize_statement(stmt_id)
    sqlite_disconnect(conn_id)
}

test_case("Query Result Processing") {
    sus conn_id normie = sqlite_connect(":memory:")
    
    fr fr Create and populate table
    db_execute_query(conn_id, "CREATE TABLE test (id INTEGER, value TEXT)")
    db_execute_query(conn_id, "INSERT INTO test VALUES (1, 'first'), (2, 'second'), (3, 'third')")
    
    fr fr Select data
    sus select_sql tea = "SELECT * FROM test ORDER BY id"
    sus result = db_execute_query(conn_id, select_sql)
    
    assert_not_null(result)
    sus row_count normie = db_result_row_count(result)
    assert_eq_int(row_count, 3)
    
    sus first_row = db_result_get_row(result, 0)
    assert_not_null(first_row)
    
    db_free_result(result)
    sqlite_disconnect(conn_id)
}

fr fr === Transaction Tests ===
test_case("Transaction Management") {
    sus conn_id normie = sqlite_connect(":memory:")
    
    db_execute_query(conn_id, "CREATE TABLE accounts (id INTEGER PRIMARY KEY, balance REAL)")
    db_execute_query(conn_id, "INSERT INTO accounts (balance) VALUES (100.0), (50.0)")
    
    fr fr Start transaction
    sus begin_result lit = db_begin_transaction(conn_id)
    assert_eq_bool(begin_result, based)
    
    fr fr Update balances
    db_execute_query(conn_id, "UPDATE accounts SET balance = balance - 25 WHERE id = 1")
    db_execute_query(conn_id, "UPDATE accounts SET balance = balance + 25 WHERE id = 2")
    
    fr fr Commit transaction
    sus commit_result lit = db_commit_transaction(conn_id)
    assert_eq_bool(commit_result, based)
    
    fr fr Verify changes
    sus result = db_execute_query(conn_id, "SELECT balance FROM accounts WHERE id = 1")
    sus balance1 = db_result_get_float(result, 0, 0)
    assert_eq_float(balance1, 75.0)
    
    db_free_result(result)
    sqlite_disconnect(conn_id)
}

test_case("Transaction Rollback") {
    sus conn_id normie = sqlite_connect(":memory:")
    
    db_execute_query(conn_id, "CREATE TABLE test_rollback (value INTEGER)")
    db_execute_query(conn_id, "INSERT INTO test_rollback VALUES (42)")
    
    fr fr Start transaction
    db_begin_transaction(conn_id)
    
    fr fr Make changes
    db_execute_query(conn_id, "UPDATE test_rollback SET value = 999")
    
    fr fr Rollback transaction
    sus rollback_result lit = db_rollback_transaction(conn_id)
    assert_eq_bool(rollback_result, based)
    
    fr fr Verify rollback
    sus result = db_execute_query(conn_id, "SELECT value FROM test_rollback")
    sus value normie = db_result_get_int(result, 0, 0)
    assert_eq_int(value, 42)  fr fr Should be original value
    
    db_free_result(result)
    sqlite_disconnect(conn_id)
}

fr fr === ORM Tests ===
test_case("ORM Model Definition") {
    sus user_model = orm_define_model("users", [
        orm_field("id", "INTEGER", "PRIMARY KEY"),
        orm_field("name", "TEXT", "NOT NULL"),
        orm_field("email", "TEXT", "UNIQUE"),
        orm_field("created_at", "TIMESTAMP", "DEFAULT CURRENT_TIMESTAMP")
    ])
    
    assert_not_null(user_model)
    
    sus table_name tea = orm_model_get_table_name(user_model)
    assert_eq_string(table_name, "users")
}

test_case("ORM Create and Save") {
    sus conn_id normie = sqlite_connect(":memory:")
    
    sus user_model = orm_define_model("users", [
        orm_field("id", "INTEGER", "PRIMARY KEY AUTOINCREMENT"),
        orm_field("name", "TEXT", "NOT NULL"),
        orm_field("email", "TEXT", "UNIQUE")
    ])
    
    fr fr Create table
    sus create_result lit = orm_create_table(conn_id, user_model)
    assert_eq_bool(create_result, based)
    
    fr fr Create and save instance
    sus user = orm_create_instance(user_model)
    orm_set_field(user, "name", "John Doe")
    orm_set_field(user, "email", "john@example.com")
    
    sus save_result lit = orm_save(conn_id, user)
    assert_eq_bool(save_result, based)
    
    sqlite_disconnect(conn_id)
}

test_case("ORM Query and Find") {
    sus conn_id normie = sqlite_connect(":memory:")
    
    sus product_model = orm_define_model("products", [
        orm_field("id", "INTEGER", "PRIMARY KEY"),
        orm_field("name", "TEXT", "NOT NULL"),
        orm_field("price", "REAL", "NOT NULL")
    ])
    
    orm_create_table(conn_id, product_model)
    
    fr fr Create test data
    sus product1 = orm_create_instance(product_model)
    orm_set_field(product1, "name", "Widget A")
    orm_set_field(product1, "price", 19.99)
    orm_save(conn_id, product1)
    
    sus product2 = orm_create_instance(product_model)
    orm_set_field(product2, "name", "Widget B")
    orm_set_field(product2, "price", 29.99)
    orm_save(conn_id, product2)
    
    fr fr Query data
    sus all_products = orm_find_all(conn_id, product_model)
    assert_not_null(all_products)
    
    sus count normie = orm_result_count(all_products)
    assert_eq_int(count, 2)
    
    fr fr Find specific product
    sus expensive_products = orm_find_where(conn_id, product_model, "price > 25.0")
    sus expensive_count normie = orm_result_count(expensive_products)
    assert_eq_int(expensive_count, 1)
    
    sqlite_disconnect(conn_id)
}

fr fr === Security Tests ===
test_case("SQL Injection Prevention") {
    sus conn_id normie = sqlite_connect(":memory:")
    
    db_execute_query(conn_id, "CREATE TABLE secure_test (id INTEGER, data TEXT)")
    db_execute_query(conn_id, "INSERT INTO secure_test VALUES (1, 'safe_data')")
    
    fr fr Try SQL injection with prepared statement (should be safe)
    sus malicious_input tea = "'; DROP TABLE secure_test; --"
    sus stmt_id normie = db_prepare_statement(conn_id, "SELECT * FROM secure_test WHERE data = ?")
    db_bind_parameter(stmt_id, 1, malicious_input)
    
    sus result = db_execute_prepared(stmt_id)
    sus row_count normie = db_result_row_count(result)
    assert_eq_int(row_count, 0)  fr fr Should find no rows, but table should still exist
    
    db_free_result(result)
    db_finalize_statement(stmt_id)
    
    fr fr Verify table still exists
    sus table_check = db_execute_query(conn_id, "SELECT COUNT(*) FROM secure_test")
    sus original_count normie = db_result_get_int(table_check, 0, 0)
    assert_eq_int(original_count, 1)  fr fr Original data should be intact
    
    db_free_result(table_check)
    sqlite_disconnect(conn_id)
}

test_case("Connection String Validation") {
    fr fr Test valid connection strings
    sus valid_pg tea = "postgresql://user:pass@localhost:5432/db"
    assert_eq_bool(db_validate_connection_string(valid_pg), based)
    
    sus valid_mysql tea = "mysql://user:pass@localhost:3306/db"
    assert_eq_bool(db_validate_connection_string(valid_mysql), based)
    
    sus valid_sqlite tea = "/path/to/database.db"
    assert_eq_bool(db_validate_connection_string(valid_sqlite), based)
    
    fr fr Test invalid connection strings
    sus invalid1 tea = "invalid://bad"
    assert_eq_bool(db_validate_connection_string(invalid1), cap)
    
    sus empty_string tea = ""
    assert_eq_bool(db_validate_connection_string(empty_string), cap)
}

fr fr === Performance Tests ===
test_case("Bulk Insert Performance") {
    sus conn_id normie = sqlite_connect(":memory:")
    
    db_execute_query(conn_id, "CREATE TABLE bulk_test (id INTEGER, data TEXT)")
    
    fr fr Prepare statement for bulk insert
    sus stmt_id normie = db_prepare_statement(conn_id, "INSERT INTO bulk_test VALUES (?, ?)")
    
    db_begin_transaction(conn_id)
    
    fr fr Insert 1000 rows
    bestie (sus i normie = 0; i < 1000; i++) {
        db_bind_parameter(stmt_id, 1, i)
        db_bind_parameter(stmt_id, 2, "test_data_" + string(i))
        db_execute_prepared(stmt_id)
    }
    
    db_commit_transaction(conn_id)
    db_finalize_statement(stmt_id)
    
    fr fr Verify all rows inserted
    sus count_result = db_execute_query(conn_id, "SELECT COUNT(*) FROM bulk_test")
    sus total_rows normie = db_result_get_int(count_result, 0, 0)
    assert_eq_int(total_rows, 1000)
    
    db_free_result(count_result)
    sqlite_disconnect(conn_id)
}

test_case("Connection Pool Stress Test") {
    db_pool_initialize(5)  fr fr Small pool for stress testing
    
    sus acquired_connections [10]normie = [0; 10]
    sus successful_acquisitions normie = 0
    
    fr fr Try to acquire more connections than pool size
    bestie (sus i normie = 0; i < 10; i++) {
        sus conn_id normie = db_pool_acquire_connection("test://stress")
        yo conn_id >= 0 {
            acquired_connections[successful_acquisitions] = conn_id
            successful_acquisitions = successful_acquisitions + 1
        }
    }
    
    assert_less_than_or_equal(successful_acquisitions, 5)  fr fr Should not exceed pool size
    
    fr fr Release all acquired connections
    bestie (sus i normie = 0; i < successful_acquisitions; i++) {
        db_pool_release_connection(acquired_connections[i])
    }
    
    assert_eq_int(db_connections_in_use, 0)
}

fr fr === Error Handling Tests ===
test_case("Database Connection Errors") {
    sus invalid_conn tea = "invalid://nonexistent:9999/baddb"
    
    fam {
        sus conn_id normie = postgres_connect(invalid_conn)
        fail("Should have thrown error for invalid connection")
    } shook (err tea) {
        assert_contains(err, "connection")
    }
}

test_case("Invalid Query Handling") {
    sus conn_id normie = sqlite_connect(":memory:")
    
    sus invalid_sql tea = "SELECT * FROM nonexistent_table"
    
    fam {
        sus result = db_execute_query(conn_id, invalid_sql)
        fail("Should have thrown error for invalid query")
    } shook (err tea) {
        assert_contains(err, "table")
    }
    
    sqlite_disconnect(conn_id)
}

test_case("Transaction Error Recovery") {
    sus conn_id normie = sqlite_connect(":memory:")
    
    db_execute_query(conn_id, "CREATE TABLE error_test (id INTEGER PRIMARY KEY)")
    
    db_begin_transaction(conn_id)
    
    fam {
        fr fr This should fail due to constraint violation
        db_execute_query(conn_id, "INSERT INTO error_test VALUES (1)")
        db_execute_query(conn_id, "INSERT INTO error_test VALUES (1)")  fr fr Duplicate primary key
        db_commit_transaction(conn_id)
        fail("Should have thrown error for constraint violation")
    } shook (err tea) {
        sus rollback_result lit = db_rollback_transaction(conn_id)
        assert_eq_bool(rollback_result, based)
    }
    
    sqlite_disconnect(conn_id)
}

print_test_summary()
