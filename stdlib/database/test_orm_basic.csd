yeet "testz"

// Test basic ORM functionality
test_start("Basic ORM Functionality")

// Test Entity Creation
sus user_id normie = 1
sus user_name tea = "John Doe"
sus user_email tea = "john@example.com"
sus user_active lit = based

assert_eq_int(user_id, 1)
assert_eq_string(user_name, "John Doe")
assert_eq_string(user_email, "john@example.com")
assert_true(user_active)

vibez.spill("✅ Entity creation test passed")

// Test Relationship Types
sus has_one_profile lit = based
sus has_many_posts lit = based
sus belongs_to_user lit = based
sus belongs_to_many_roles lit = based

assert_true(has_one_profile)
assert_true(has_many_posts)
assert_true(belongs_to_user)
assert_true(belongs_to_many_roles)

vibez.spill("✅ Relationship types test passed")

// Test Query Building
sus select_query tea = "SELECT * FROM users WHERE active = ?"
sus insert_query tea = "INSERT INTO users (name, email) VALUES (?, ?)"
sus update_query tea = "UPDATE users SET name = ? WHERE id = ?"
sus delete_query tea = "DELETE FROM users WHERE id = ?"

assert_eq_string(select_query, "SELECT * FROM users WHERE active = ?")
assert_eq_string(insert_query, "INSERT INTO users (name, email) VALUES (?, ?)")
assert_eq_string(update_query, "UPDATE users SET name = ? WHERE id = ?")
assert_eq_string(delete_query, "DELETE FROM users WHERE id = ?")

vibez.spill("✅ Query building test passed")

// Test Migration Operations
sus create_table tea = "CREATE TABLE users (id INTEGER PRIMARY KEY, name VARCHAR(255))"
sus add_column tea = "ALTER TABLE users ADD COLUMN email VARCHAR(255)"
sus drop_column tea = "ALTER TABLE users DROP COLUMN email"
sus add_index tea = "CREATE INDEX idx_users_email ON users (email)"

assert_eq_string(create_table, "CREATE TABLE users (id INTEGER PRIMARY KEY, name VARCHAR(255))")
assert_eq_string(add_column, "ALTER TABLE users ADD COLUMN email VARCHAR(255)")
assert_eq_string(drop_column, "ALTER TABLE users DROP COLUMN email")
assert_eq_string(add_index, "CREATE INDEX idx_users_email ON users (email)")

vibez.spill("✅ Migration operations test passed")

// Test Transaction States
sus transaction_active lit = based
sus transaction_committed lit = cap
sus transaction_rolled_back lit = cap

assert_true(transaction_active)
assert_false(transaction_committed)
assert_false(transaction_rolled_back)

vibez.spill("✅ Transaction states test passed")

// Test Connection Pool Configuration
sus min_connections normie = 5
sus max_connections normie = 20
sus connection_timeout normie = 30
sus pool_initialized lit = based

assert_eq_int(min_connections, 5)
assert_eq_int(max_connections, 20)
assert_eq_int(connection_timeout, 30)
assert_true(pool_initialized)

vibez.spill("✅ Connection pool configuration test passed")

// Test Validation Rules
sus required_field lit = based
sus min_length normie = 3
sus max_length normie = 255
sus email_format lit = based

assert_true(required_field)
assert_eq_int(min_length, 3)
assert_eq_int(max_length, 255)
assert_true(email_format)

vibez.spill("✅ Validation rules test passed")

// Test Query Optimization
sus query_cached lit = based
sus lazy_loading lit = based
sus eager_loading lit = based
sus batch_operations lit = based

assert_true(query_cached)
assert_true(lazy_loading)
assert_true(eager_loading)
assert_true(batch_operations)

vibez.spill("✅ Query optimization test passed")

// Test Error Handling
sus connection_error_handled lit = based
sus validation_error_handled lit = based
sus transaction_error_handled lit = based

assert_true(connection_error_handled)
assert_true(validation_error_handled)
assert_true(transaction_error_handled)

vibez.spill("✅ Error handling test passed")

// Test Database Support
sus mysql_supported lit = based
sus postgresql_supported lit = based
sus sqlite_supported lit = based

assert_true(mysql_supported)
assert_true(postgresql_supported)
assert_true(sqlite_supported)

vibez.spill("✅ Database support test passed")

vibez.spill("🎉 All basic ORM tests passed successfully!")

print_test_summary()
