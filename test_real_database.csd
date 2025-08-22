fr fr Test Real Database Implementation
fr fr Comprehensive test suite for real SQLite functionality

yeet "dbz"
yeet "vibez"
yeet "stringz"
yeet "testz"

fr fr ===== TEST DATABASE SETUP =====

slay test_database_setup() lit {
    vibez.spill("Setting up test database...")
    
    fr fr Test database file
    sus test_db_file tea = "test_database.db"
    
    fr fr Clean up any existing test database
    fr fr file_delete(test_db_file)  // Would need real file operations
    
    fr fr Open test database
    sus connection DatabaseConnection = dbz.sqlite_open(test_db_file)
    
    ready (!connection.is_connected) {
        vibez.spill("FAILED: Could not create test database")
        damn cringe
    }
    
    vibez.spill("Test database created successfully")
    damn based
}

fr fr ===== CREATE TABLE TEST =====

slay test_create_table() lit {
    vibez.spill("Testing CREATE TABLE...")
    
    sus connection DatabaseConnection = dbz.sqlite_open("test_database.db")
    
    ready (!connection.is_connected) {
        vibez.spill("FAILED: Cannot connect to database")
        damn cringe
    }
    
    fr fr Create users table
    sus create_sql tea = "CREATE TABLE IF NOT EXISTS users (id INTEGER PRIMARY KEY, name TEXT NOT NULL, email TEXT UNIQUE)"
    sus result QueryResult = dbz.sqlite_query(connection, create_sql)
    
    ready (!result.success) {
        vibez.spill("FAILED: Could not create users table")
        dbz.db_close(connection)
        damn cringe
    }
    
    vibez.spill("SUCCESS: Users table created")
    dbz.db_close(connection)
    damn based
}

fr fr ===== INSERT DATA TEST =====

slay test_insert_data() lit {
    vibez.spill("Testing INSERT operations...")
    
    sus connection DatabaseConnection = dbz.sqlite_open("test_database.db")
    
    ready (!connection.is_connected) {
        vibez.spill("FAILED: Cannot connect to database")
        damn cringe
    }
    
    fr fr Insert test data
    sus insert_sql tea = "INSERT INTO users (name, email) VALUES ('John Doe', 'john@example.com')"
    sus result QueryResult = dbz.sqlite_query(connection, insert_sql)
    
    ready (!result.success) {
        vibez.spill("FAILED: Could not insert data")
        dbz.db_close(connection)
        damn cringe
    }
    
    ready (result.rows_affected != 1) {
        vibez.spill("FAILED: Expected 1 row affected, got " + stringz.number_to_string(result.rows_affected))
        dbz.db_close(connection)
        damn cringe
    }
    
    vibez.spill("SUCCESS: Data inserted, rows affected: " + stringz.number_to_string(result.rows_affected))
    vibez.spill("Last insert ID: " + stringz.number_to_string(result.last_insert_id))
    
    dbz.db_close(connection)
    damn based
}

fr fr ===== SELECT DATA TEST =====

slay test_select_data() lit {
    vibez.spill("Testing SELECT operations...")
    
    sus connection DatabaseConnection = dbz.sqlite_open("test_database.db")
    
    ready (!connection.is_connected) {
        vibez.spill("FAILED: Cannot connect to database")
        damn cringe
    }
    
    fr fr Select all users
    sus select_sql tea = "SELECT id, name, email FROM users"
    sus result QueryResult = dbz.sqlite_query(connection, select_sql)
    
    ready (!result.success) {
        vibez.spill("FAILED: Could not select data")
        dbz.db_close(connection)
        damn cringe
    }
    
    vibez.spill("SUCCESS: Data selected")
    vibez.spill("Columns: " + stringz.join(result.column_names, ", "))
    vibez.spill("Row count: " + stringz.number_to_string(array_length(result.rows)))
    
    fr fr Display results
    sus i drip = 0
    bestie (i < array_length(result.rows)) {
        vibez.spill("Row " + stringz.number_to_string(i + 1) + ": " + result.rows[i])
        i = i + 1
    }
    
    dbz.db_close(connection)
    damn based
}

fr fr ===== UPDATE DATA TEST =====

slay test_update_data() lit {
    vibez.spill("Testing UPDATE operations...")
    
    sus connection DatabaseConnection = dbz.sqlite_open("test_database.db")
    
    ready (!connection.is_connected) {
        vibez.spill("FAILED: Cannot connect to database")
        damn cringe
    }
    
    fr fr Update user data
    sus update_sql tea = "UPDATE users SET name = 'John Smith' WHERE email = 'john@example.com'"
    sus result QueryResult = dbz.sqlite_query(connection, update_sql)
    
    ready (!result.success) {
        vibez.spill("FAILED: Could not update data")
        dbz.db_close(connection)
        damn cringe
    }
    
    vibez.spill("SUCCESS: Data updated, rows affected: " + stringz.number_to_string(result.rows_affected))
    
    dbz.db_close(connection)
    damn based
}

fr fr ===== DELETE DATA TEST =====

slay test_delete_data() lit {
    vibez.spill("Testing DELETE operations...")
    
    sus connection DatabaseConnection = dbz.sqlite_open("test_database.db")
    
    ready (!connection.is_connected) {
        vibez.spill("FAILED: Cannot connect to database")
        damn cringe
    }
    
    fr fr Insert another record first
    sus insert_sql tea = "INSERT INTO users (name, email) VALUES ('Jane Smith', 'jane@example.com')"
    sus insert_result QueryResult = dbz.sqlite_query(connection, insert_sql)
    
    ready (!insert_result.success) {
        vibez.spill("FAILED: Could not insert test data for deletion")
        dbz.db_close(connection)
        damn cringe
    }
    
    fr fr Delete the record
    sus delete_sql tea = "DELETE FROM users WHERE email = 'jane@example.com'"
    sus result QueryResult = dbz.sqlite_query(connection, delete_sql)
    
    ready (!result.success) {
        vibez.spill("FAILED: Could not delete data")
        dbz.db_close(connection)
        damn cringe
    }
    
    vibez.spill("SUCCESS: Data deleted, rows affected: " + stringz.number_to_string(result.rows_affected))
    
    dbz.db_close(connection)
    damn based
}

fr fr ===== TRANSACTION TEST =====

slay test_transactions() lit {
    vibez.spill("Testing transaction support...")
    
    sus connection DatabaseConnection = dbz.sqlite_open("test_database.db")
    
    ready (!connection.is_connected) {
        vibez.spill("FAILED: Cannot connect to database")
        damn cringe
    }
    
    fr fr Begin transaction
    sus begin_success lit = dbz.db_begin_transaction(connection)
    ready (!begin_success) {
        vibez.spill("FAILED: Could not begin transaction")
        dbz.db_close(connection)
        damn cringe
    }
    
    fr fr Insert data in transaction
    sus insert_sql tea = "INSERT INTO users (name, email) VALUES ('Transaction Test', 'trans@example.com')"
    sus result QueryResult = dbz.sqlite_query(connection, insert_sql)
    
    ready (!result.success) {
        vibez.spill("FAILED: Could not insert in transaction")
        dbz.db_rollback_transaction(connection)
        dbz.db_close(connection)
        damn cringe
    }
    
    fr fr Rollback transaction
    sus rollback_success lit = dbz.db_rollback_transaction(connection)
    ready (!rollback_success) {
        vibez.spill("FAILED: Could not rollback transaction")
        dbz.db_close(connection)
        damn cringe
    }
    
    vibez.spill("SUCCESS: Transaction rollback completed")
    
    dbz.db_close(connection)
    damn based
}

fr fr ===== PREPARED STATEMENTS TEST =====

slay test_prepared_statements() lit {
    vibez.spill("Testing prepared statements...")
    
    sus connection DatabaseConnection = dbz.sqlite_open("test_database.db")
    
    ready (!connection.is_connected) {
        vibez.spill("FAILED: Cannot connect to database")
        damn cringe
    }
    
    fr fr Prepare statement
    sus prepared_sql tea = "INSERT INTO users (name, email) VALUES (?, ?)"
    sus statement PreparedStatement = dbz.db_prepare_statement(connection, prepared_sql)
    
    ready (!statement.is_prepared) {
        vibez.spill("FAILED: Could not prepare statement")
        dbz.db_close(connection)
        damn cringe
    }
    
    fr fr Execute prepared statement
    sus parameters []tea = ["Prepared User", "prepared@example.com"]
    sus result QueryResult = dbz.db_execute_prepared(connection, statement, parameters)
    
    ready (!result.success) {
        vibez.spill("FAILED: Could not execute prepared statement")
        dbz.db_close(connection)
        damn cringe
    }
    
    vibez.spill("SUCCESS: Prepared statement executed, rows affected: " + stringz.number_to_string(result.rows_affected))
    
    dbz.db_close(connection)
    damn based
}

fr fr ===== CONNECTION POOL TEST =====

slay test_connection_pool() lit {
    vibez.spill("Testing connection pool...")
    
    fr fr Create connection pool
    sus pool ConnectionPool = dbz.create_connection_pool("sqlite", "test_database.db", 3)
    
    fr fr Get connection from pool
    sus connection DatabaseConnection = dbz.pool_get_connection(pool)
    
    ready (!connection.is_connected) {
        vibez.spill("FAILED: Could not get connection from pool")
        damn cringe
    }
    
    fr fr Test query with pooled connection
    sus select_sql tea = "SELECT COUNT(*) as user_count FROM users"
    sus result QueryResult = dbz.sqlite_query(connection, select_sql)
    
    ready (!result.success) {
        vibez.spill("FAILED: Could not query with pooled connection")
        damn cringe
    }
    
    vibez.spill("SUCCESS: Pooled connection query completed")
    
    fr fr Return connection to pool
    sus return_success lit = dbz.pool_return_connection(pool, connection)
    ready (!return_success) {
        vibez.spill("FAILED: Could not return connection to pool")
        damn cringe
    }
    
    vibez.spill("SUCCESS: Connection returned to pool")
    damn based
}

fr fr ===== MAIN TEST RUNNER =====

slay run_database_tests() lit {
    vibez.spill("=== CURSED Database Real Implementation Test Suite ===")
    vibez.spill("")
    
    sus tests_passed drip = 0
    sus tests_total drip = 8
    
    fr fr Run all tests
    ready (test_database_setup()) {
        tests_passed = tests_passed + 1
        vibez.spill("✅ Database setup test passed")
    } otherwise {
        vibez.spill("❌ Database setup test failed")
    }
    
    ready (test_create_table()) {
        tests_passed = tests_passed + 1
        vibez.spill("✅ Create table test passed")
    } otherwise {
        vibez.spill("❌ Create table test failed")
    }
    
    ready (test_insert_data()) {
        tests_passed = tests_passed + 1
        vibez.spill("✅ Insert data test passed")
    } otherwise {
        vibez.spill("❌ Insert data test failed")
    }
    
    ready (test_select_data()) {
        tests_passed = tests_passed + 1
        vibez.spill("✅ Select data test passed")
    } otherwise {
        vibez.spill("❌ Select data test failed")
    }
    
    ready (test_update_data()) {
        tests_passed = tests_passed + 1
        vibez.spill("✅ Update data test passed")
    } otherwise {
        vibez.spill("❌ Update data test failed")
    }
    
    ready (test_delete_data()) {
        tests_passed = tests_passed + 1
        vibez.spill("✅ Delete data test passed")
    } otherwise {
        vibez.spill("❌ Delete data test failed")
    }
    
    ready (test_transactions()) {
        tests_passed = tests_passed + 1
        vibez.spill("✅ Transaction test passed")
    } otherwise {
        vibez.spill("❌ Transaction test failed")
    }
    
    ready (test_prepared_statements()) {
        tests_passed = tests_passed + 1
        vibez.spill("✅ Prepared statements test passed")
    } otherwise {
        vibez.spill("❌ Prepared statements test failed")
    }
    
    fr fr Summary
    vibez.spill("")
    vibez.spill("=== Test Results ===")
    vibez.spill("Tests passed: " + stringz.number_to_string(tests_passed) + "/" + stringz.number_to_string(tests_total))
    
    ready (tests_passed == tests_total) {
        vibez.spill("🎉 ALL TESTS PASSED! Database implementation is working correctly.")
        damn based
    } otherwise {
        vibez.spill("⚠️  Some tests failed. Database implementation needs more work.")
        damn cringe
    }
}

fr fr Execute the test suite
run_database_tests()
