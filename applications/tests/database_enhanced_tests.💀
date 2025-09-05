fr fr DATABASE_ENHANCED TESTS - Comprehensive test suite
fr fr Tests for all database functionality including CRUD operations, transactions, migrations

yeet "database_enhanced"
yeet "json"
yeet "stringz"
yeet "timez"
yeet "fs"
yeet "vibez"

fr fr ===== TEST CONFIGURATION =====

sus test_db_url tea = "file://./test_database"
sus test_results []tea = []
sus tests_passed drip = 0
sus tests_failed drip = 0

fr fr ===== TEST UTILITIES =====

slay assert_true(condition lit, test_name tea) {
    ready (condition) {
        test_results[test_results.length] = "✅ PASS: " + test_name
        tests_passed = tests_passed + 1
        vibez.spill("✅ PASS: " + test_name)
    } otherwise {
        test_results[test_results.length] = "❌ FAIL: " + test_name
        tests_failed = tests_failed + 1
        vibez.spill("❌ FAIL: " + test_name)
    }
}

slay assert_equal(expected tea, actual tea, test_name tea) {
    ready (expected == actual) {
        test_results[test_results.length] = "✅ PASS: " + test_name
        tests_passed = tests_passed + 1
        vibez.spill("✅ PASS: " + test_name)
    } otherwise {
        test_results[test_results.length] = "❌ FAIL: " + test_name + " - Expected: " + expected + ", Got: " + actual
        tests_failed = tests_failed + 1
        vibez.spill("❌ FAIL: " + test_name + " - Expected: " + expected + ", Got: " + actual)
    }
}

slay assert_not_empty(value tea, test_name tea) {
    ready (value != "" && value != "0") {
        test_results[test_results.length] = "✅ PASS: " + test_name
        tests_passed = tests_passed + 1
        vibez.spill("✅ PASS: " + test_name)
    } otherwise {
        test_results[test_results.length] = "❌ FAIL: " + test_name + " - Value is empty"
        tests_failed = tests_failed + 1
        vibez.spill("❌ FAIL: " + test_name + " - Value is empty")
    }
}

slay cleanup_test_database() {
    ready (fs.directory_exists("./test_database")) {
        fr fr Remove test database files
        vibez.spill("Cleaning up test database...")
    }
}

fr fr ===== CONNECTION TESTS =====

slay test_database_connections() {
    vibez.spill("Running database connection tests...")
    
    fr fr Test SQLite connection
    sus sqlite_conn database_enhanced.DatabaseConnection = database_enhanced.create_connection("sqlite://./test.db")
    assert_true(sqlite_conn.is_connected, "SQLite connection establishment")
    assert_equal("sqlite", sqlite_conn.database_type, "SQLite connection type")
    database_enhanced.close_connection(sqlite_conn)
    
    fr fr Test file database connection
    sus file_conn database_enhanced.DatabaseConnection = database_enhanced.create_connection("file://./test_file_db")
    assert_true(file_conn.is_connected, "File database connection establishment")
    assert_equal("file", file_conn.database_type, "File database connection type")
    database_enhanced.close_connection(file_conn)
    
    fr fr Test memory database connection
    sus memory_conn database_enhanced.DatabaseConnection = database_enhanced.create_connection("memory://test")
    assert_true(memory_conn.is_connected, "Memory database connection establishment")
    assert_equal("memory", memory_conn.database_type, "Memory database connection type")
    database_enhanced.close_connection(memory_conn)
    
    fr fr Test PostgreSQL connection (simulated)
    sus pg_conn database_enhanced.DatabaseConnection = database_enhanced.create_connection("postgres://user:pass@localhost:5432/testdb")
    assert_true(pg_conn.is_connected, "PostgreSQL connection establishment")
    assert_equal("postgresql", pg_conn.database_type, "PostgreSQL connection type")
    database_enhanced.close_connection(pg_conn)
}

fr fr ===== TABLE CREATION TESTS =====

slay test_table_creation() {
    vibez.spill("Running table creation tests...")
    
    sus conn database_enhanced.DatabaseConnection = database_enhanced.create_connection(test_db_url)
    
    fr fr Test basic table creation
    sus basic_schema tea = json.object_to_string({
        "id": "INTEGER PRIMARY KEY AUTOINCREMENT",
        "name": "TEXT NOT NULL",
        "email": "TEXT UNIQUE"
    })
    
    sus created lit = database_enhanced.create_table(conn, "test_users", basic_schema)
    assert_true(created, "Basic table creation")
    
    fr fr Test table with various data types
    sus complex_schema tea = json.object_to_string({
        "id": "INTEGER PRIMARY KEY AUTOINCREMENT",
        "title": "TEXT NOT NULL",
        "description": "TEXT",
        "price": "DECIMAL(10,2)",
        "is_active": "BOOLEAN DEFAULT 1",
        "created_at": "TEXT NOT NULL",
        "updated_at": "TEXT"
    })
    
    sus complex_created lit = database_enhanced.create_table(conn, "test_products", complex_schema)
    assert_true(complex_created, "Complex table creation")
    
    fr fr Test table creation with invalid schema
    sus invalid_created lit = database_enhanced.create_table(conn, "test_invalid", "{invalid json}")
    assert_true(!invalid_created, "Invalid schema rejection")
    
    database_enhanced.close_connection(conn)
}

fr fr ===== CRUD OPERATION TESTS =====

slay test_crud_operations() {
    vibez.spill("Running CRUD operation tests...")
    
    sus conn database_enhanced.DatabaseConnection = database_enhanced.create_connection(test_db_url)
    
    fr fr Setup test table
    sus schema tea = json.object_to_string({
        "id": "INTEGER PRIMARY KEY AUTOINCREMENT",
        "name": "TEXT NOT NULL",
        "email": "TEXT UNIQUE",
        "age": "INTEGER",
        "created_at": "TEXT NOT NULL"
    })
    
    database_enhanced.create_table(conn, "test_people", schema)
    
    fr fr Test INSERT operations
    sus person_data tea = json.object_to_string({
        "name": "John Doe",
        "email": "john@example.com",
        "age": "30",
        "created_at": timez.format_iso8601(timez.now_millis())
    })
    
    sus insert_result lit = database_enhanced.insert_record(conn, "test_people", person_data)
    assert_true(insert_result, "Single record insert")
    
    fr fr Test batch insert
    sus batch_records []tea = [
        json.object_to_string({
            "name": "Jane Smith",
            "email": "jane@example.com", 
            "age": "25",
            "created_at": timez.format_iso8601(timez.now_millis())
        }),
        json.object_to_string({
            "name": "Bob Johnson",
            "email": "bob@example.com",
            "age": "35",
            "created_at": timez.format_iso8601(timez.now_millis())
        })
    ]
    
    sus batch_result lit = database_enhanced.batch_insert(conn, "test_people", batch_records)
    assert_true(batch_result, "Batch record insert")
    
    fr fr Test SELECT operations
    sus all_records []tea = database_enhanced.find_records(conn, "test_people", "{}")
    assert_true(all_records.length >= 3, "Find all records")
    
    fr fr Test SELECT with conditions
    sus filtered_records []tea = database_enhanced.find_records(conn, "test_people", 
        json.object_to_string({"name": "John Doe"}))
    assert_true(filtered_records.length >= 1, "Find records with conditions")
    
    fr fr Test UPDATE operations
    sus update_data tea = json.object_to_string({
        "age": "31",
        "email": "john.updated@example.com"
    })
    
    sus update_result lit = database_enhanced.update_record(conn, "test_people", 1, update_data)
    assert_true(update_result, "Record update")
    
    fr fr Test DELETE operations  
    sus delete_result lit = database_enhanced.delete_record(conn, "test_people", 1)
    assert_true(delete_result, "Record deletion")
    
    fr fr Verify deletion
    sus remaining_records []tea = database_enhanced.find_records(conn, "test_people", "{}")
    assert_true(remaining_records.length >= 2, "Records remain after deletion")
    
    database_enhanced.close_connection(conn)
}

fr fr ===== TRANSACTION TESTS =====

slay test_transactions() {
    vibez.spill("Running transaction tests...")
    
    sus conn database_enhanced.DatabaseConnection = database_enhanced.create_connection(test_db_url)
    
    fr fr Setup test table
    sus schema tea = json.object_to_string({
        "id": "INTEGER PRIMARY KEY AUTOINCREMENT",
        "account": "TEXT NOT NULL",
        "balance": "INTEGER NOT NULL"
    })
    
    database_enhanced.create_table(conn, "test_accounts", schema)
    
    fr fr Insert initial data
    sus account1_data tea = json.object_to_string({
        "account": "Account1",
        "balance": "1000"
    })
    
    sus account2_data tea = json.object_to_string({
        "account": "Account2", 
        "balance": "500"
    })
    
    database_enhanced.insert_record(conn, "test_accounts", account1_data)
    database_enhanced.insert_record(conn, "test_accounts", account2_data)
    
    fr fr Test successful transaction
    sus begin_result lit = database_enhanced.begin_transaction(conn)
    assert_true(begin_result, "Transaction begin")
    
    sus transfer_from tea = json.object_to_string({"balance": "900"})
    sus transfer_to tea = json.object_to_string({"balance": "600"})
    
    database_enhanced.update_record(conn, "test_accounts", 1, transfer_from)
    database_enhanced.update_record(conn, "test_accounts", 2, transfer_to)
    
    sus commit_result lit = database_enhanced.commit_transaction(conn)
    assert_true(commit_result, "Transaction commit")
    
    fr fr Test transaction rollback
    database_enhanced.begin_transaction(conn)
    
    sus rollback_update tea = json.object_to_string({"balance": "0"})
    database_enhanced.update_record(conn, "test_accounts", 1, rollback_update)
    
    sus rollback_result lit = database_enhanced.rollback_transaction(conn)
    assert_true(rollback_result, "Transaction rollback")
    
    fr fr Verify rollback worked
    sus accounts_after_rollback []tea = database_enhanced.find_records(conn, "test_accounts", "{}")
    assert_true(accounts_after_rollback.length == 2, "Records preserved after rollback")
    
    database_enhanced.close_connection(conn)
}

fr fr ===== QUERY EXECUTION TESTS =====

slay test_query_execution() {
    vibez.spill("Running query execution tests...")
    
    sus conn database_enhanced.DatabaseConnection = database_enhanced.create_connection(test_db_url)
    
    fr fr Test SELECT query
    sus select_result database_enhanced.QueryResult = database_enhanced.execute_query(conn, 
        "SELECT * FROM test_people LIMIT 5")
    assert_true(select_result.success, "SELECT query execution")
    assert_true(select_result.execution_time_ms >= 0, "SELECT query timing")
    
    fr fr Test CREATE TABLE query
    sus create_result database_enhanced.QueryResult = database_enhanced.execute_query(conn,
        "CREATE TABLE test_query_table (id INTEGER PRIMARY KEY, data TEXT)")
    assert_true(create_result.success, "CREATE TABLE query execution")
    
    fr fr Test INSERT query  
    sus insert_result database_enhanced.QueryResult = database_enhanced.execute_query(conn,
        "INSERT INTO test_query_table (data) VALUES ('test data')")
    assert_true(insert_result.success, "INSERT query execution")
    assert_true(insert_result.rows_affected >= 0, "INSERT rows affected")
    
    fr fr Test UPDATE query
    sus update_result database_enhanced.QueryResult = database_enhanced.execute_query(conn,
        "UPDATE test_query_table SET data = 'updated data' WHERE id = 1")
    assert_true(update_result.success, "UPDATE query execution")
    
    fr fr Test DELETE query
    sus delete_result database_enhanced.QueryResult = database_enhanced.execute_query(conn,
        "DELETE FROM test_query_table WHERE id = 1")
    assert_true(delete_result.success, "DELETE query execution")
    
    fr fr Test invalid query
    sus invalid_result database_enhanced.QueryResult = database_enhanced.execute_query(conn,
        "INVALID SQL SYNTAX HERE")
    assert_true(!invalid_result.success, "Invalid query rejection")
    
    database_enhanced.close_connection(conn)
}

fr fr ===== MIGRATION TESTS =====

slay test_migrations() {
    vibez.spill("Running migration tests...")
    
    sus conn database_enhanced.DatabaseConnection = database_enhanced.create_connection(test_db_url)
    
    fr fr Create migrations table
    sus migrations_created lit = database_enhanced.create_migration_table(conn)
    assert_true(migrations_created, "Migrations table creation")
    
    fr fr Test migration creation and application
    sus test_migration database_enhanced.Migration = database_enhanced.Migration{
        version: "001",
        name: "create_test_migration_table",
        up_sql: "CREATE TABLE test_migration_result (id INTEGER PRIMARY KEY, name TEXT NOT NULL)",
        down_sql: "DROP TABLE test_migration_result",
        applied_at: ""
    }
    
    sus migration_applied lit = database_enhanced.apply_migration(conn, test_migration)
    assert_true(migration_applied, "Migration application")
    
    fr fr Test that migration was recorded
    sus migration_records []tea = database_enhanced.find_records(conn, "migrations", "{}")
    assert_true(migration_records.length >= 1, "Migration recorded in database")
    
    fr fr Test table was actually created by migration
    sus table_test_result database_enhanced.QueryResult = database_enhanced.execute_query(conn,
        "INSERT INTO test_migration_result (name) VALUES ('test')")
    assert_true(table_test_result.success, "Migration created functional table")
    
    database_enhanced.close_connection(conn)
}

fr fr ===== PERFORMANCE TESTS =====

slay test_performance() {
    vibez.spill("Running performance tests...")
    
    sus conn database_enhanced.DatabaseConnection = database_enhanced.create_connection(test_db_url)
    
    fr fr Setup performance test table
    sus perf_schema tea = json.object_to_string({
        "id": "INTEGER PRIMARY KEY AUTOINCREMENT",
        "data": "TEXT NOT NULL",
        "number": "INTEGER",
        "timestamp": "TEXT NOT NULL"
    })
    
    database_enhanced.create_table(conn, "perf_test", perf_schema)
    
    fr fr Test bulk insert performance
    sus start_time drip = timez.now_millis()
    sus insert_count drip = 100
    
    sus i drip = 0
    bestie (i < insert_count) {
        sus perf_data tea = json.object_to_string({
            "data": "Performance test data " + stringz.from_int(i),
            "number": stringz.from_int(i * 10),
            "timestamp": timez.format_iso8601(timez.now_millis())
        })
        
        database_enhanced.insert_record(conn, "perf_test", perf_data)
        i = i + 1
    }
    
    sus end_time drip = timez.now_millis()
    sus duration drip = end_time - start_time
    
    assert_true(duration > 0, "Performance test completed")
    vibez.spill("Bulk insert performance: " + stringz.from_int(insert_count) + " records in " + 
               stringz.from_int(duration) + "ms")
    
    fr fr Test query performance
    sus query_start drip = timez.now_millis()
    sus all_perf_records []tea = database_enhanced.find_records(conn, "perf_test", "{}")
    sus query_end drip = timez.now_millis()
    sus query_duration drip = query_end - query_start
    
    assert_true(all_perf_records.length >= insert_count, "Performance test data retrieved")
    vibez.spill("Query performance: " + stringz.from_int(all_perf_records.length) + " records in " + 
               stringz.from_int(query_duration) + "ms")
    
    database_enhanced.close_connection(conn)
}

fr fr ===== ERROR HANDLING TESTS =====

slay test_error_handling() {
    vibez.spill("Running error handling tests...")
    
    fr fr Test connection to invalid database
    sus invalid_conn database_enhanced.DatabaseConnection = database_enhanced.create_connection("invalid://bad/url")
    assert_true(!invalid_conn.is_connected, "Invalid connection handling")
    
    fr fr Test operations on closed connection
    sus closed_conn database_enhanced.DatabaseConnection = database_enhanced.create_connection(test_db_url)
    database_enhanced.close_connection(closed_conn)
    
    sus closed_insert lit = database_enhanced.insert_record(closed_conn, "test_table", "{}")
    assert_true(!closed_insert, "Operations on closed connection rejected")
    
    fr fr Test invalid JSON data
    sus valid_conn database_enhanced.DatabaseConnection = database_enhanced.create_connection(test_db_url)
    sus invalid_json_insert lit = database_enhanced.insert_record(valid_conn, "test_table", "invalid json")
    assert_true(!invalid_json_insert, "Invalid JSON data rejected")
    
    fr fr Test operations on non-existent table
    sus nonexistent_records []tea = database_enhanced.find_records(valid_conn, "nonexistent_table", "{}")
    assert_true(nonexistent_records.length == 0, "Non-existent table handling")
    
    database_enhanced.close_connection(valid_conn)
}

fr fr ===== SCHEMA MANAGEMENT TESTS =====

slay test_schema_management() {
    vibez.spill("Running schema management tests...")
    
    sus conn database_enhanced.DatabaseConnection = database_enhanced.create_connection(test_db_url)
    
    fr fr Create test table for schema inspection
    sus schema_test_schema tea = json.object_to_string({
        "id": "INTEGER PRIMARY KEY AUTOINCREMENT",
        "title": "TEXT NOT NULL",
        "description": "TEXT",
        "price": "DECIMAL(10,2)",
        "created_at": "TEXT NOT NULL"
    })
    
    database_enhanced.create_table(conn, "schema_test_table", schema_test_schema)
    
    fr fr Test table info retrieval
    sus table_info tea = database_enhanced.get_table_info(conn, "schema_test_table")
    assert_not_empty(table_info, "Table info retrieval")
    
    fr fr Test with non-existent table
    sus missing_table_info tea = database_enhanced.get_table_info(conn, "missing_table")
    assert_equal("", missing_table_info, "Missing table info handling")
    
    database_enhanced.close_connection(conn)
}

fr fr ===== MAIN TEST RUNNER =====

slay run_all_tests() {
    vibez.spill("🚀 Starting Database Enhanced Test Suite")
    vibez.spill("=" + stringz.repeat("=", 50))
    
    fr fr Clean up any previous test data
    cleanup_test_database()
    
    fr fr Run all test suites
    test_database_connections()
    test_table_creation()
    test_crud_operations()
    test_transactions()
    test_query_execution()
    test_migrations()
    test_performance()
    test_error_handling()
    test_schema_management()
    
    fr fr Print final results
    vibez.spill("=" + stringz.repeat("=", 50))
    vibez.spill("🏁 Test Suite Complete")
    vibez.spill("✅ Tests Passed: " + stringz.from_int(tests_passed))
    vibez.spill("❌ Tests Failed: " + stringz.from_int(tests_failed))
    vibez.spill("📊 Total Tests: " + stringz.from_int(tests_passed + tests_failed))
    
    ready (tests_failed > 0) {
        vibez.spill("⚠️  Some tests failed. Review the output above.")
    } otherwise {
        vibez.spill("🎉 All tests passed successfully!")
    }
    
    vibez.spill("=" + stringz.repeat("=", 50))
    
    fr fr Generate test report
    generate_test_report()
}

slay generate_test_report() {
    sus report tea = "# Database Enhanced Test Report\n\n"
    report = report + "Generated: " + timez.format_iso8601(timez.now_millis()) + "\n\n"
    report = report + "## Summary\n\n"
    report = report + "- **Total Tests:** " + stringz.from_int(tests_passed + tests_failed) + "\n"
    report = report + "- **Passed:** " + stringz.from_int(tests_passed) + "\n"
    report = report + "- **Failed:** " + stringz.from_int(tests_failed) + "\n"
    report = report + "- **Success Rate:** " + calculate_success_rate() + "%\n\n"
    report = report + "## Detailed Results\n\n"
    
    sus i drip = 0
    bestie (i < test_results.length) {
        report = report + "- " + test_results[i] + "\n"
        i = i + 1
    }
    
    sus report_file tea = "test_results_database_enhanced.md"
    fs.write_file(report_file, report)
    vibez.spill("📝 Test report written to: " + report_file)
}

slay calculate_success_rate() tea {
    sus total drip = tests_passed + tests_failed
    ready (total == 0) {
        damn "0"
    }
    
    sus rate drip = (tests_passed * 100) / total
    damn stringz.from_int(rate)
}

fr fr Run the test suite
run_all_tests()
