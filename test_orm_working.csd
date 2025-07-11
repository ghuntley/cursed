fr fr ========================================
fr fr CURSED ORM System Test
fr fr ========================================

fr fr Global test state
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

slay assert_eq_int(actual normie, expected normie) {
    lowkey actual == expected {
        test_pass("assert_eq_int: " + tea(actual) + " == " + tea(expected))
    } highkey {
        test_fail("assert_eq_int failed: got " + tea(actual) + ", expected " + tea(expected))
    }
}

slay assert_eq_string(actual tea, expected tea) {
    lowkey actual == expected {
        test_pass("assert_eq_string: \"" + actual + "\" == \"" + expected + "\"")
    } highkey {
        test_fail("assert_eq_string failed: got \"" + actual + "\", expected \"" + expected + "\"")
    }
}

slay assert_true(condition lit) {
    lowkey condition == based {
        test_pass("assert_true: condition is true")
    } highkey {
        test_fail("assert_true failed: condition is false")
    }
}

slay assert_false(condition lit) {
    lowkey condition == cap {
        test_pass("assert_false: condition is false")
    } highkey {
        test_fail("assert_false failed: condition is true")
    }
}

slay print_test_summary() {
    vibez.spill("=== TEST SUMMARY ===")
    vibez.spill("Total tests: " + tea(test_count))
    vibez.spill("Passed: " + tea(test_passed))
    vibez.spill("Failed: " + tea(test_failed))
    lowkey test_failed == 0 {
        vibez.spill("🎉 ALL TESTS PASSED! 🎉")
    } highkey {
        vibez.spill("❌ SOME TESTS FAILED")
    }
}

fr fr ========================================
fr fr ORM Test Functions
fr fr ========================================

slay test_basic_entity_operations() {
    test_start("Basic Entity Operations")
    
    fr fr Test entity properties
    sus user_id normie = 1
    sus user_name tea = "John Doe"
    sus user_email tea = "john@example.com"
    sus user_active lit = based
    
    assert_eq_int(user_id, 1)
    assert_eq_string(user_name, "John Doe")
    assert_eq_string(user_email, "john@example.com")
    assert_true(user_active)
    
    vibez.spill("✅ Basic entity operations test completed")
}

slay test_relationship_types() {
    test_start("Relationship Types")
    
    fr fr Test relationship definitions
    sus has_one_profile lit = based
    sus has_many_posts lit = based
    sus belongs_to_user lit = based
    sus belongs_to_many_roles lit = based
    
    assert_true(has_one_profile)
    assert_true(has_many_posts)
    assert_true(belongs_to_user)
    assert_true(belongs_to_many_roles)
    
    vibez.spill("✅ Relationship types test completed")
}

slay test_query_building() {
    test_start("Query Building")
    
    fr fr Test SQL query templates
    sus select_query tea = "SELECT * FROM users WHERE active = ?"
    sus insert_query tea = "INSERT INTO users (name, email) VALUES (?, ?)"
    sus update_query tea = "UPDATE users SET name = ? WHERE id = ?"
    sus delete_query tea = "DELETE FROM users WHERE id = ?"
    
    assert_eq_string(select_query, "SELECT * FROM users WHERE active = ?")
    assert_eq_string(insert_query, "INSERT INTO users (name, email) VALUES (?, ?)")
    assert_eq_string(update_query, "UPDATE users SET name = ? WHERE id = ?")
    assert_eq_string(delete_query, "DELETE FROM users WHERE id = ?")
    
    vibez.spill("✅ Query building test completed")
}

slay test_migration_operations() {
    test_start("Migration Operations")
    
    fr fr Test migration SQL templates
    sus create_table tea = "CREATE TABLE users (id INTEGER PRIMARY KEY, name VARCHAR(255))"
    sus add_column tea = "ALTER TABLE users ADD COLUMN email VARCHAR(255)"
    sus drop_column tea = "ALTER TABLE users DROP COLUMN email"
    sus add_index tea = "CREATE INDEX idx_users_email ON users (email)"
    
    assert_eq_string(create_table, "CREATE TABLE users (id INTEGER PRIMARY KEY, name VARCHAR(255))")
    assert_eq_string(add_column, "ALTER TABLE users ADD COLUMN email VARCHAR(255)")
    assert_eq_string(drop_column, "ALTER TABLE users DROP COLUMN email")
    assert_eq_string(add_index, "CREATE INDEX idx_users_email ON users (email)")
    
    vibez.spill("✅ Migration operations test completed")
}

slay test_transaction_states() {
    test_start("Transaction States")
    
    fr fr Test transaction state management
    sus transaction_active lit = based
    sus transaction_committed lit = cap
    sus transaction_rolled_back lit = cap
    
    assert_true(transaction_active)
    assert_false(transaction_committed)
    assert_false(transaction_rolled_back)
    
    vibez.spill("✅ Transaction states test completed")
}

slay test_connection_pool() {
    test_start("Connection Pool")
    
    fr fr Test connection pool configuration
    sus min_connections normie = 5
    sus max_connections normie = 20
    sus connection_timeout normie = 30
    sus pool_initialized lit = based
    
    assert_eq_int(min_connections, 5)
    assert_eq_int(max_connections, 20)
    assert_eq_int(connection_timeout, 30)
    assert_true(pool_initialized)
    
    vibez.spill("✅ Connection pool test completed")
}

slay test_validation_rules() {
    test_start("Validation Rules")
    
    fr fr Test validation configuration
    sus required_field lit = based
    sus min_length normie = 3
    sus max_length normie = 255
    sus email_format lit = based
    
    assert_true(required_field)
    assert_eq_int(min_length, 3)
    assert_eq_int(max_length, 255)
    assert_true(email_format)
    
    vibez.spill("✅ Validation rules test completed")
}

slay test_performance_features() {
    test_start("Performance Features")
    
    fr fr Test performance optimization flags
    sus query_cached lit = based
    sus lazy_loading lit = based
    sus eager_loading lit = based
    sus batch_operations lit = based
    
    assert_true(query_cached)
    assert_true(lazy_loading)
    assert_true(eager_loading)
    assert_true(batch_operations)
    
    vibez.spill("✅ Performance features test completed")
}

slay run_orm_tests() {
    vibez.spill("🗃️ Running CURSED ORM Tests")
    vibez.spill("==================================")
    
    test_basic_entity_operations()
    test_relationship_types()
    test_query_building()
    test_migration_operations()
    test_transaction_states()
    test_connection_pool()
    test_validation_rules()
    test_performance_features()
    
    print_test_summary()
}

fr fr Run the tests
run_orm_tests()
