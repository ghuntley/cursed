yeet "testz"
yeet "database"

# Database Module Comprehensive Test Suite
# Testing all database connectivity and ORM functionality

test_start("Database Connection Tests")

# Test database connection
assert_true(database_connect("localhost:5432/testdb", 2))
assert_false(database_connect("", 1))
assert_false(database_connect("localhost", 0))
assert_false(database_connect("localhost", 5))

# Test query execution
assert_eq_int(database_execute(1, "SELECT * FROM users"), 1)
assert_eq_int(database_execute(-1, "SELECT * FROM users"), -1)
assert_eq_int(database_execute(1, ""), -1)

# Test transaction management
assert_true(database_begin_transaction(1))
assert_true(database_commit_transaction(1))
assert_true(database_rollback_transaction(1))
assert_false(database_begin_transaction(-1))
assert_false(database_commit_transaction(-1))
assert_false(database_rollback_transaction(-1))

print_test_summary()

test_start("ORM Functionality Tests")

# Test table creation
assert_true(orm_create_table("users", "id INT PRIMARY KEY, name VARCHAR(255)"))
assert_false(orm_create_table("", "id INT"))
assert_false(orm_create_table("users", ""))

# Test record insertion
assert_eq_int(orm_insert_record("users", "{\"name\": \"John\"}"), 1)
assert_eq_int(orm_insert_record("", "{\"name\": \"John\"}"), -1)
assert_eq_int(orm_insert_record("users", ""), -1)

# Test record selection
assert_eq_string(orm_select_records("users", "name='John'"), "{\"records\": []}")
assert_eq_string(orm_select_records("", "name='John'"), "")

# Test record update
assert_true(orm_update_record("users", 1, "{\"name\": \"Jane\"}"))
assert_false(orm_update_record("", 1, "{\"name\": \"Jane\"}"))
assert_false(orm_update_record("users", -1, "{\"name\": \"Jane\"}"))
assert_false(orm_update_record("users", 1, ""))

# Test record deletion
assert_true(orm_delete_record("users", 1))
assert_false(orm_delete_record("", 1))
assert_false(orm_delete_record("users", -1))

print_test_summary()

test_start("Database Schema Management Tests")

# Test schema creation
assert_true(database_create_schema("test_schema"))
assert_false(database_create_schema(""))

# Test schema dropping
assert_true(database_drop_schema("test_schema"))
assert_false(database_drop_schema(""))

# Test migration management
assert_true(database_run_migration("001_create_users.sql"))
assert_false(database_run_migration(""))
assert_true(database_rollback_migration(1))
assert_false(database_rollback_migration(-1))

print_test_summary()

test_start("Connection Pooling Tests")

# Test connection pool creation
assert_eq_int(database_create_pool("localhost:5432/testdb", 10), 1)
assert_eq_int(database_create_pool("", 10), -1)
assert_eq_int(database_create_pool("localhost", 0), -1)

# Test connection pool operations
assert_eq_int(database_get_connection_from_pool(1), 1)
assert_eq_int(database_get_connection_from_pool(-1), -1)
assert_true(database_return_connection_to_pool(1, 1))
assert_false(database_return_connection_to_pool(-1, 1))
assert_false(database_return_connection_to_pool(1, -1))

print_test_summary()

test_start("Database Utilities Tests")

# Test string escaping
assert_eq_string(database_escape_string("test'string"), "test'string")
assert_eq_string(database_escape_string(""), "")

# Test connection validation
assert_true(database_validate_connection(1))
assert_false(database_validate_connection(-1))

# Test database metadata
assert_eq_int(database_get_last_insert_id(1), 1)
assert_eq_int(database_get_last_insert_id(-1), -1)
assert_eq_int(database_get_affected_rows(1), 1)
assert_eq_int(database_get_affected_rows(-1), -1)

print_test_summary()

test_start("Database Backup and Restore Tests")

# Test backup operations
assert_true(database_backup(1, "backup.sql"))
assert_false(database_backup(-1, "backup.sql"))
assert_false(database_backup(1, ""))

# Test restore operations
assert_true(database_restore(1, "backup.sql"))
assert_false(database_restore(-1, "backup.sql"))
assert_false(database_restore(1, ""))

print_test_summary()

test_start("Database Connection Types Tests")

# Test connection type constants
assert_eq_int(ConnectionType_MySQL, 1)
assert_eq_int(ConnectionType_PostgreSQL, 2)
assert_eq_int(ConnectionType_SQLite, 3)
assert_eq_int(ConnectionType_MongoDB, 4)

# Test query result constants
assert_eq_int(QueryResult_Success, 1)
assert_eq_int(QueryResult_Error, 2)
assert_eq_int(QueryResult_NotFound, 3)

print_test_summary()
