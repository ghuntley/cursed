yeet "testz"
yeet "database_drivers"
yeet "sqlite"

# SQLite Database Driver Tests
# Comprehensive test suite for SQLite driver functionality

test_start("SQLite driver comprehensive tests")

# Test 1: Configuration creation
test_start("SQLite configuration creation")
config := create_sqlite_config("test.db")
assert_eq_string(config.database_path, "test.db")
assert_eq_string(config.mode, "rwc")
assert_eq_int(config.cache_size, 2000)
assert_eq_int(config.page_size, 4096)
assert_eq_string(config.synchronous, "NORMAL")
assert_eq_string(config.journal_mode, "WAL")
assert_true(config.foreign_keys)
assert_false(config.auto_vacuum)
assert_eq_string(config.temp_store, "DEFAULT")
assert_eq_string(config.locking_mode, "NORMAL")
assert_false(config.secure_delete)
assert_false(config.read_uncommitted)
assert_true(config.recursive_triggers)
assert_eq_int(config.busy_timeout, 5000)
vibez.spill("✅ SQLite configuration created successfully")

# Test 2: Connection creation
test_start("SQLite connection creation")
connection := create_sqlite_connection(config)
assert_eq_string(connection.config.database_path, "test.db")
assert_eq_string(connection.database_path, "test.db")
assert_false(connection.is_connected)
assert_eq_string(connection.sqlite_version, "")
assert_false(connection.is_readonly)
assert_true(connection.auto_commit)
assert_false(connection.in_transaction)
assert_eq_int(connection.last_insert_rowid, 0)
assert_eq_int(connection.changes, 0)
assert_eq_int(connection.total_changes, 0)
assert_eq_int(connection.query_count, 0)
vibez.spill("✅ SQLite connection created successfully")

# Test 3: Database connection
test_start("SQLite database connection")
connect_result := connect_sqlite(&connection)
assert_true(connect_result)
assert_true(connection.is_connected)
assert_eq_string(connection.sqlite_version, "3.44.2")
assert_eq_int(len(connection.pragma_settings), 4)
assert_eq_string(connection.pragma_settings[0], "foreign_keys=ON")
assert_eq_string(connection.pragma_settings[1], "journal_mode=WAL")
assert_eq_string(connection.pragma_settings[2], "synchronous=NORMAL")
assert_eq_string(connection.pragma_settings[3], "cache_size=2000")
vibez.spill("✅ SQLite database connection established")

# Test 4: Query execution - SELECT
test_start("SQLite SELECT query execution")
select_result := execute_sqlite_query(&connection, "SELECT * FROM users")
assert_true(select_result.success)
assert_eq_int(select_result.rows_affected, 0)
assert_eq_int(len(select_result.columns), 4)
assert_eq_string(select_result.columns[0], "id")
assert_eq_string(select_result.columns[1], "name")
assert_eq_string(select_result.columns[2], "email")
assert_eq_string(select_result.columns[3], "created_at")
assert_eq_int(len(select_result.rows), 3)
assert_eq_string(select_result.rows[0][0], "1")
assert_eq_string(select_result.rows[0][1], "Alice Johnson")
assert_eq_string(select_result.rows[1][1], "Bob Smith")
assert_eq_string(select_result.rows[2][1], "Carol Davis")
assert_eq_int(select_result.error_code, 0)
assert_eq_string(select_result.error_message, "")
assert_eq_string(select_result.sql_explain, "SCAN users")
vibez.spill("✅ SQLite SELECT query executed successfully")

# Test 5: Query execution - INSERT
test_start("SQLite INSERT query execution")
insert_result := execute_sqlite_query(&connection, "INSERT INTO users (name, email) VALUES ('Test User', 'test@example.com')")
assert_true(insert_result.success)
assert_eq_int(insert_result.rows_affected, 1)
assert_eq_int(len(insert_result.columns), 0)
assert_eq_int(insert_result.error_code, 0)
assert_eq_string(insert_result.error_message, "")
assert_eq_int(insert_result.execution_time, 10)
assert_eq_int(insert_result.last_insert_rowid, 1)
assert_eq_int(insert_result.changes, 1)
assert_eq_string(insert_result.sql_explain, "INSERT INTO users")
vibez.spill("✅ SQLite INSERT query executed successfully")

# Test 6: Query execution - UPDATE
test_start("SQLite UPDATE query execution")
update_result := execute_sqlite_query(&connection, "UPDATE users SET name = 'Updated User' WHERE id = 1")
assert_true(update_result.success)
assert_eq_int(update_result.rows_affected, 2)
assert_eq_int(update_result.error_code, 0)
assert_eq_string(update_result.error_message, "")
assert_eq_int(update_result.execution_time, 12)
assert_eq_int(update_result.changes, 2)
assert_eq_string(update_result.sql_explain, "UPDATE users USING INDEX")
vibez.spill("✅ SQLite UPDATE query executed successfully")

# Test 7: Query execution - DELETE
test_start("SQLite DELETE query execution")
delete_result := execute_sqlite_query(&connection, "DELETE FROM users WHERE id = 1")
assert_true(delete_result.success)
assert_eq_int(delete_result.rows_affected, 1)
assert_eq_int(delete_result.error_code, 0)
assert_eq_string(delete_result.error_message, "")
assert_eq_int(delete_result.execution_time, 8)
assert_eq_int(delete_result.changes, 1)
assert_eq_string(delete_result.sql_explain, "DELETE FROM users")
vibez.spill("✅ SQLite DELETE query executed successfully")

# Test 8: Query execution - CREATE TABLE
test_start("SQLite CREATE TABLE query execution")
create_result := execute_sqlite_query(&connection, "CREATE TABLE test_table (id INTEGER PRIMARY KEY, name TEXT)")
assert_true(create_result.success)
assert_eq_int(create_result.rows_affected, 0)
assert_eq_int(create_result.error_code, 0)
assert_eq_string(create_result.error_message, "")
assert_eq_int(create_result.execution_time, 20)
assert_eq_string(create_result.sql_explain, "CREATE TABLE")
vibez.spill("✅ SQLite CREATE TABLE query executed successfully")

# Test 9: Query execution - PRAGMA
test_start("SQLite PRAGMA query execution")
pragma_result := execute_sqlite_query(&connection, "PRAGMA journal_mode")
assert_true(pragma_result.success)
assert_eq_int(pragma_result.rows_affected, 0)
assert_eq_int(len(pragma_result.columns), 1)
assert_eq_string(pragma_result.columns[0], "pragma_value")
assert_eq_int(len(pragma_result.rows), 1)
assert_eq_string(pragma_result.rows[0][0], "WAL")
assert_eq_int(pragma_result.error_code, 0)
assert_eq_string(pragma_result.error_message, "")
assert_eq_string(pragma_result.sql_explain, "PRAGMA query")
vibez.spill("✅ SQLite PRAGMA query executed successfully")

# Test 10: Prepared statement creation
test_start("SQLite prepared statement creation")
stmt := prepare_sqlite_statement(&connection, "SELECT * FROM users WHERE id = ? AND name = :name")
assert_true(stmt.is_prepared)
assert_eq_int(stmt.connection_id, connection.connection_id)
assert_eq_string(stmt.query, "SELECT * FROM users WHERE id = ? AND name = :name")
assert_eq_int(stmt.parameter_count, 2)
assert_eq_int(len(stmt.parameter_names), 2)
assert_eq_string(stmt.parameter_names[0], ":name")
assert_eq_string(stmt.parameter_names[1], ":email")
assert_eq_int(len(stmt.bound_parameters), 2)
assert_true(stmt.is_readonly)
vibez.spill("✅ SQLite prepared statement created successfully")

# Test 11: Parameter binding by index
test_start("SQLite parameter binding by index")
bind_result1 := bind_sqlite_parameter(&stmt, 0, "1")
bind_result2 := bind_sqlite_parameter(&stmt, 1, "John Doe")
assert_true(bind_result1)
assert_true(bind_result2)
assert_eq_string(stmt.bound_parameters[0], "1")
assert_eq_string(stmt.bound_parameters[1], "John Doe")
vibez.spill("✅ SQLite parameters bound by index successfully")

# Test 12: Parameter binding by name
test_start("SQLite parameter binding by name")
bind_named_result := bind_sqlite_named_parameter(&stmt, ":name", "Alice")
assert_true(bind_named_result)
assert_eq_string(stmt.bound_parameters[0], "Alice")
vibez.spill("✅ SQLite parameters bound by name successfully")

# Test 13: Prepared statement execution
test_start("SQLite prepared statement execution")
exec_result := execute_sqlite_prepared_statement(&stmt)
assert_true(exec_result.success)
assert_eq_int(exec_result.rows_affected, 1)
assert_eq_int(len(exec_result.rows), 1)
assert_eq_string(exec_result.rows[0][0], "1")
assert_eq_string(exec_result.rows[0][1], "Test User")
assert_eq_int(exec_result.error_code, 0)
assert_eq_string(exec_result.error_message, "")
assert_eq_int(exec_result.last_insert_rowid, 1)
assert_eq_int(exec_result.changes, 1)
assert_eq_string(exec_result.sql_explain, "SEARCH users USING INDEX")
vibez.spill("✅ SQLite prepared statement executed successfully")

# Test 14: Transaction management
test_start("SQLite transaction management")
tx := begin_sqlite_transaction(&connection, "IMMEDIATE")
assert_true(tx.is_active)
assert_eq_int(tx.connection_id, connection.connection_id)
assert_eq_string(tx.transaction_type, "IMMEDIATE")
assert_eq_int(tx.operations_count, 0)
assert_eq_int(tx.nested_level, 0)
assert_true(connection.in_transaction)
assert_false(connection.auto_commit)
vibez.spill("✅ SQLite transaction started successfully")

# Test 15: Transaction commit
test_start("SQLite transaction commit")
commit_result := commit_sqlite_transaction(&connection, &tx)
assert_true(commit_result)
assert_false(tx.is_active)
assert_false(connection.in_transaction)
assert_true(connection.auto_commit)
vibez.spill("✅ SQLite transaction committed successfully")

# Test 16: Transaction rollback
test_start("SQLite transaction rollback")
tx2 := begin_sqlite_transaction(&connection, "EXCLUSIVE")
assert_true(tx2.is_active)
assert_true(connection.in_transaction)
rollback_result := rollback_sqlite_transaction(&connection, &tx2)
assert_true(rollback_result)
assert_false(tx2.is_active)
assert_false(connection.in_transaction)
assert_true(connection.auto_commit)
vibez.spill("✅ SQLite transaction rolled back successfully")

# Test 17: Savepoint management
test_start("SQLite savepoint management")
tx3 := begin_sqlite_transaction(&connection, "DEFERRED")
assert_true(tx3.is_active)
assert_true(tx3.is_readonly)
savepoint_result := create_sqlite_savepoint(&tx3, "sp1")
assert_true(savepoint_result)
assert_eq_int(len(tx3.savepoints), 1)
assert_eq_string(tx3.savepoints[0], "sp1")
assert_eq_int(tx3.nested_level, 1)
rollback_savepoint_result := rollback_sqlite_to_savepoint(&tx3, "sp1")
assert_true(rollback_savepoint_result)
assert_eq_int(tx3.nested_level, 0)
commit_sqlite_transaction(&connection, &tx3)
vibez.spill("✅ SQLite savepoint management successful")

# Test 18: Savepoint release
test_start("SQLite savepoint release")
tx4 := begin_sqlite_transaction(&connection, "IMMEDIATE")
create_sqlite_savepoint(&tx4, "sp2")
create_sqlite_savepoint(&tx4, "sp3")
assert_eq_int(len(tx4.savepoints), 2)
assert_eq_int(tx4.nested_level, 2)
release_result := release_sqlite_savepoint(&tx4, "sp2")
assert_true(release_result)
assert_eq_int(len(tx4.savepoints), 1)
assert_eq_int(tx4.nested_level, 1)
commit_sqlite_transaction(&connection, &tx4)
vibez.spill("✅ SQLite savepoint release successful")

# Test 19: PRAGMA execution
test_start("SQLite PRAGMA execution")
pragma_exec_result := execute_sqlite_pragma(&connection, "foreign_keys", "ON")
assert_true(pragma_exec_result.success)
assert_eq_int(len(connection.pragma_settings), 5)
assert_eq_string(connection.pragma_settings[4], "foreign_keys=ON")
vibez.spill("✅ SQLite PRAGMA execution successful")

# Test 20: Database info
test_start("SQLite database info")
get_sqlite_database_info(&connection)
vibez.spill("✅ SQLite database info retrieved")

# Test 21: Health check
test_start("SQLite health check")
health_result := health_check_sqlite(&connection)
assert_true(health_result)
vibez.spill("✅ SQLite health check passed")

# Test 22: Database vacuum
test_start("SQLite database vacuum")
vacuum_result := vacuum_sqlite_database(&connection)
assert_true(vacuum_result.success)
assert_eq_int(vacuum_result.error_code, 0)
assert_eq_string(vacuum_result.error_message, "")
vibez.spill("✅ SQLite database vacuum successful")

# Test 23: Database analyze
test_start("SQLite database analyze")
analyze_result := analyze_sqlite_database(&connection)
assert_true(analyze_result.success)
assert_eq_int(analyze_result.error_code, 0)
assert_eq_string(analyze_result.error_message, "")
vibez.spill("✅ SQLite database analyze successful")

# Test 24: Table info
test_start("SQLite table info")
table_info_result := get_sqlite_table_info(&connection, "users")
assert_true(table_info_result.success)
assert_eq_int(table_info_result.error_code, 0)
assert_eq_string(table_info_result.error_message, "")
vibez.spill("✅ SQLite table info retrieved")

# Test 25: Connection disconnection
test_start("SQLite connection disconnection")
disconnect_result := disconnect_sqlite(&connection)
assert_true(disconnect_result)
assert_false(connection.is_connected)
vibez.spill("✅ SQLite connection disconnected successfully")

# Test 26: Error handling - disconnected connection
test_start("SQLite error handling - disconnected connection")
error_result := execute_sqlite_query(&connection, "SELECT 1")
assert_false(error_result.success)
assert_eq_int(error_result.error_code, 21)
assert_eq_string(error_result.error_message, "Library routine called out of sequence")
vibez.spill("✅ SQLite error handling working correctly")

# Test 27: Parameter binding error handling
test_start("SQLite parameter binding error handling")
stmt_error := prepare_sqlite_statement(&connection, "SELECT 1")
assert_false(stmt_error.is_prepared)
bind_error := bind_sqlite_parameter(&stmt_error, 0, "test")
assert_false(bind_error)
bind_named_error := bind_sqlite_named_parameter(&stmt_error, ":test", "value")
assert_false(bind_named_error)
vibez.spill("✅ SQLite parameter binding error handling working")

# Test 28: Invalid parameter index
test_start("SQLite invalid parameter index")
reconnect_sqlite(&connection)
stmt_valid := prepare_sqlite_statement(&connection, "SELECT * FROM users WHERE id = ?")
bind_invalid := bind_sqlite_parameter(&stmt_valid, 5, "test")
assert_false(bind_invalid)
vibez.spill("✅ SQLite invalid parameter index handled correctly")

# Test 29: Invalid named parameter
test_start("SQLite invalid named parameter")
stmt_named := prepare_sqlite_statement(&connection, "SELECT * FROM users WHERE name = :name")
bind_invalid_named := bind_sqlite_named_parameter(&stmt_named, ":invalid", "test")
assert_false(bind_invalid_named)
vibez.spill("✅ SQLite invalid named parameter handled correctly")

# Test 30: Configuration validation
test_start("SQLite configuration validation")
custom_config := SQLiteConfig{
    database_path: "custom.db",
    mode: "ro",
    cache_size: 4000,
    page_size: 8192,
    synchronous: "FULL",
    journal_mode: "DELETE",
    foreign_keys: cap,
    auto_vacuum: based,
    temp_store: "MEMORY",
    locking_mode: "EXCLUSIVE",
    secure_delete: based,
    read_uncommitted: based,
    recursive_triggers: cap,
    busy_timeout: 10000
}
custom_connection := create_sqlite_connection(custom_config)
assert_eq_string(custom_connection.config.database_path, "custom.db")
assert_eq_string(custom_connection.config.mode, "ro")
assert_eq_int(custom_connection.config.cache_size, 4000)
assert_eq_int(custom_connection.config.page_size, 8192)
assert_eq_string(custom_connection.config.synchronous, "FULL")
assert_eq_string(custom_connection.config.journal_mode, "DELETE")
assert_false(custom_connection.config.foreign_keys)
assert_true(custom_connection.config.auto_vacuum)
assert_eq_string(custom_connection.config.temp_store, "MEMORY")
assert_eq_string(custom_connection.config.locking_mode, "EXCLUSIVE")
assert_true(custom_connection.config.secure_delete)
assert_true(custom_connection.config.read_uncommitted)
assert_false(custom_connection.config.recursive_triggers)
assert_eq_int(custom_connection.config.busy_timeout, 10000)
vibez.spill("✅ SQLite configuration validation successful")

# Test 31: Multiple database connections
test_start("SQLite multiple database connections")
config2 := create_sqlite_config("test2.db")
config2.journal_mode = "DELETE"
config2.synchronous = "FULL"
connection2 := create_sqlite_connection(config2)
connect_result2 := connect_sqlite(&connection2)
assert_true(connect_result2)
assert_true(connection2.is_connected)
assert_eq_string(connection2.config.database_path, "test2.db")
assert_eq_string(connection2.config.journal_mode, "DELETE")
assert_eq_string(connection2.config.synchronous, "FULL")
disconnect_sqlite(&connection2)
vibez.spill("✅ SQLite multiple database connections working correctly")

# Test 32: Connection statistics tracking
test_start("SQLite connection statistics tracking")
reconnect_sqlite(&connection)
initial_query_count := connection.query_count
initial_total_changes := connection.total_changes
execute_sqlite_query(&connection, "SELECT 1")
execute_sqlite_query(&connection, "INSERT INTO users (name) VALUES ('User1')")
execute_sqlite_query(&connection, "UPDATE users SET name = 'Updated' WHERE id = 1")
assert_eq_int(connection.query_count, initial_query_count + 3)
assert_eq_int(connection.total_changes, initial_total_changes + 3)
vibez.spill("✅ SQLite connection statistics tracking working")

# Test 33: Read-only query detection
test_start("SQLite read-only query detection")
readonly_stmt := prepare_sqlite_statement(&connection, "SELECT * FROM users")
readwrite_stmt := prepare_sqlite_statement(&connection, "INSERT INTO users (name) VALUES ('Test')")
assert_true(readonly_stmt.is_readonly)
assert_false(readwrite_stmt.is_readonly)
vibez.spill("✅ SQLite read-only query detection working")

# Test 34: Step count tracking
test_start("SQLite step count tracking")
step_stmt := prepare_sqlite_statement(&connection, "SELECT * FROM users WHERE id = ?")
bind_sqlite_parameter(&step_stmt, 0, "1")
initial_step_count := step_stmt.step_count
execute_sqlite_prepared_statement(&step_stmt)
assert_eq_int(step_stmt.step_count, initial_step_count + 1)
vibez.spill("✅ SQLite step count tracking working")

# Test 35: Nested transaction handling
test_start("SQLite nested transaction handling")
tx_outer := begin_sqlite_transaction(&connection, "IMMEDIATE")
assert_true(tx_outer.is_active)
# Try to begin another transaction (should fail)
tx_inner := begin_sqlite_transaction(&connection, "DEFERRED")
assert_false(tx_inner.is_active)
commit_sqlite_transaction(&connection, &tx_outer)
vibez.spill("✅ SQLite nested transaction handling working")

# Helper function for reconnection
slay reconnect_sqlite(connection: *SQLiteConnection) {
    if connection.is_connected == cap {
        connect_sqlite(connection)
    }
}

vibez.spill("🎉 All SQLite driver tests completed successfully!")
print_test_summary()
