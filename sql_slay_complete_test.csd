// SQL Slay Complete Test - Self-contained module and tests
fr fr ========================================
fr fr SQL Slay Database Operations Module
fr fr ========================================

// Database Connection Structure
sus db_connection_type tea = "sqlite"
sus db_connection_host tea = "localhost"
sus db_connection_port normie = 5432
sus db_connection_name tea = "database"
sus db_connection_user tea = "user"
sus db_connection_password tea = "password"
sus db_connection_active lit = cap

// Connection Management Functions
slay db_connect(host tea, port normie, dbname tea, user tea, password tea) lit {
    db_connection_host = host
    db_connection_port = port
    db_connection_name = dbname
    db_connection_user = user
    db_connection_password = password
    db_connection_active = based
    damn based
}

slay db_disconnect() lit {
    db_connection_active = cap
    db_connection_host = ""
    db_connection_port = 0
    db_connection_name = ""
    db_connection_user = ""
    db_connection_password = ""
    damn based
}

slay db_is_connected() lit {
    damn db_connection_active
}

slay db_get_connection_info() tea {
    damn db_connection_host + ":" + db_connection_port + "/" + db_connection_name
}

// SQL Query Builder Functions
slay sql_select(table tea, columns tea, where_clause tea) tea {
    sus query tea = "SELECT " + columns + " FROM " + table
    lowkey where_clause != "" {
        query = query + " WHERE " + where_clause
    }
    damn query
}

slay sql_insert(table tea, columns tea, values tea) tea {
    sus query tea = "INSERT INTO " + table + " (" + columns + ") VALUES (" + values + ")"
    damn query
}

slay sql_update(table tea, set_clause tea, where_clause tea) tea {
    sus query tea = "UPDATE " + table + " SET " + set_clause
    lowkey where_clause != "" {
        query = query + " WHERE " + where_clause
    }
    damn query
}

slay sql_delete(table tea, where_clause tea) tea {
    sus query tea = "DELETE FROM " + table
    lowkey where_clause != "" {
        query = query + " WHERE " + where_clause
    }
    damn query
}

// SQL Execution Functions
slay sql_execute(query tea) lit {
    lowkey !db_connection_active {
        damn cap
    }
    vibez.spill("Executing SQL: " + query)
    damn based
}

slay sql_execute_insert(query tea) normie {
    lowkey !db_connection_active {
        damn 0
    }
    vibez.spill("Executing INSERT: " + query)
    damn 1
}

fr fr ========================================
fr fr Test Framework
fr fr ========================================

sus test_count normie = 0
sus test_passed normie = 0
sus test_failed normie = 0

slay test_start(name tea) {
    test_count = test_count + 1
    vibez.spill("Running test: " + name)
}

slay test_pass(message tea) {
    test_passed = test_passed + 1
    vibez.spill("  ✓ PASS: " + message)
}

slay test_fail(message tea) {
    test_failed = test_failed + 1
    vibez.spill("  ✗ FAIL: " + message)
}

slay assert_eq_string(actual tea, expected tea) {
    lowkey actual == expected {
        test_pass("assert_eq_string: '" + actual + "' == '" + expected + "'")
    } highkey {
        test_fail("assert_eq_string failed: got '" + actual + "', expected '" + expected + "'")
    }
}

slay assert_true(condition lit) {
    lowkey condition {
        test_pass("assert_true: condition is true")
    } highkey {
        test_fail("assert_true: condition is false")
    }
}

slay assert_false(condition lit) {
    lowkey !condition {
        test_pass("assert_false: condition is false")
    } highkey {
        test_fail("assert_false: condition is true")
    }
}

slay assert_eq_int(actual normie, expected normie) {
    lowkey actual == expected {
        test_pass("assert_eq_int: " + actual + " == " + expected)
    } highkey {
        test_fail("assert_eq_int failed: got " + actual + ", expected " + expected)
    }
}

slay print_test_summary() {
    vibez.spill("=== TEST SUMMARY ===")
    vibez.spill("Total tests: " + test_count)
    vibez.spill("Passed: " + test_passed)
    vibez.spill("Failed: " + test_failed)
    
    lowkey test_failed == 0 {
        vibez.spill("🎉 ALL TESTS PASSED! 🎉")
    } highkey {
        vibez.spill("❌ SOME TESTS FAILED!")
    }
}

fr fr ========================================
fr fr Test Functions
fr fr ========================================

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
    
    sus insert_rows normie = sql_execute_insert("INSERT INTO users VALUES (3, 'Bob')")
    assert_eq_int(insert_rows, 1)
    
    db_disconnect()
}

slay run_sql_tests() {
    vibez.spill("🗄️  Running SQL Slay Tests")
    vibez.spill("==========================")
    
    test_db_connection()
    test_sql_builders()
    test_sql_execution()
    
    print_test_summary()
}

fr fr Run all tests
run_sql_tests()
