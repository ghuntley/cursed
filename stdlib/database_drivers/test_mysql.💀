yeet "testz"
yeet "database_drivers"
yeet "mysql"

fr fr MySQL Database Driver Tests
fr fr Comprehensive test suite for MySQL driver functionality

test_start("MySQL driver comprehensive tests")

fr fr Test 1: Configuration creation
test_start("MySQL configuration creation")
config := create_mysql_config()
assert_eq_string(config.host, "localhost")
assert_eq_int(config.port, 3306)
assert_eq_string(config.database, "mysql")
assert_eq_string(config.username, "root")
assert_eq_string(config.ssl_mode, "PREFERRED")
assert_eq_string(config.charset, "utf8mb4")
assert_eq_string(config.collation, "utf8mb4_unicode_ci")
assert_eq_int(config.connect_timeout, 30)
assert_eq_int(config.max_connections, 100)
assert_true(config.auto_reconnect)
assert_false(config.compress)
vibez.spill("✅ MySQL configuration created successfully")

fr fr Test 2: Connection creation
test_start("MySQL connection creation")
connection := create_mysql_connection(config)
assert_eq_string(connection.config.host, "localhost")
assert_eq_int(connection.config.port, 3306)
assert_eq_string(connection.config.database, "mysql")
assert_false(connection.is_connected)
assert_eq_string(connection.server_version, "")
assert_eq_int(connection.protocol_version, 10)
assert_eq_int(connection.thread_id, 0)
assert_eq_string(connection.charset, "utf8mb4")
assert_eq_int(connection.query_count, 0)
assert_true(connection.autocommit)
vibez.spill("✅ MySQL connection created successfully")

fr fr Test 3: Database connection
test_start("MySQL database connection")
connect_result := connect_mysql(&connection)
assert_true(connect_result)
assert_true(connection.is_connected)
assert_eq_string(connection.server_version, "8.0.35-MySQL")
assert_eq_int(connection.protocol_version, 10)
assert_eq_int(connection.thread_id, 123456)
assert_eq_int(connection.server_capabilities, 0xFFFFF7FF)
assert_eq_int(connection.server_status, 0x0002)
vibez.spill("✅ MySQL database connection established")

fr fr Test 4: Query execution - SELECT
test_start("MySQL SELECT query execution")
select_result := execute_mysql_query(&connection, "SELECT * FROM users")
assert_true(select_result.success)
assert_eq_int(select_result.rows_affected, 0)
assert_eq_int(len(select_result.columns), 4)
assert_eq_string(select_result.columns[0], "id")
assert_eq_string(select_result.columns[1], "name")
assert_eq_string(select_result.columns[2], "email")
assert_eq_string(select_result.columns[3], "created_at")
assert_eq_int(len(select_result.rows), 3)
assert_eq_string(select_result.rows[0][0], "1")
assert_eq_string(select_result.rows[0][1], "John Doe")
assert_eq_string(select_result.rows[1][1], "Jane Smith")
assert_eq_string(select_result.rows[2][1], "Bob Johnson")
assert_eq_int(select_result.error_code, 0)
assert_eq_string(select_result.error_message, "")
assert_eq_string(select_result.server_info, "3 rows in set")
vibez.spill("✅ MySQL SELECT query executed successfully")

fr fr Test 5: Query execution - INSERT
test_start("MySQL INSERT query execution")
insert_result := execute_mysql_query(&connection, "INSERT INTO users (name, email) VALUES ('Test User', 'test@example.com')")
assert_true(insert_result.success)
assert_eq_int(insert_result.rows_affected, 1)
assert_eq_int(len(insert_result.columns), 0)
assert_eq_int(insert_result.error_code, 0)
assert_eq_string(insert_result.error_message, "")
assert_eq_int(insert_result.execution_time, 20)
assert_eq_int(insert_result.insert_id, 1)
assert_eq_string(insert_result.server_info, "1 row affected")
vibez.spill("✅ MySQL INSERT query executed successfully")

fr fr Test 6: Query execution - UPDATE
test_start("MySQL UPDATE query execution")
update_result := execute_mysql_query(&connection, "UPDATE users SET name = 'Updated User' WHERE id = 1")
assert_true(update_result.success)
assert_eq_int(update_result.rows_affected, 2)
assert_eq_int(update_result.error_code, 0)
assert_eq_string(update_result.error_message, "")
assert_eq_int(update_result.execution_time, 25)
assert_eq_string(update_result.server_info, "2 rows affected")
vibez.spill("✅ MySQL UPDATE query executed successfully")

fr fr Test 7: Query execution - DELETE
test_start("MySQL DELETE query execution")
delete_result := execute_mysql_query(&connection, "DELETE FROM users WHERE id = 1")
assert_true(delete_result.success)
assert_eq_int(delete_result.rows_affected, 1)
assert_eq_int(delete_result.error_code, 0)
assert_eq_string(delete_result.error_message, "")
assert_eq_int(delete_result.execution_time, 30)
assert_eq_string(delete_result.server_info, "1 row affected")
vibez.spill("✅ MySQL DELETE query executed successfully")

fr fr Test 8: Prepared statement creation
test_start("MySQL prepared statement creation")
stmt := prepare_mysql_statement(&connection, "SELECT * FROM users WHERE id = ? AND name = ? AND created_at > ?")
assert_true(stmt.is_prepared)
assert_eq_int(stmt.connection_id, connection.connection_id)
assert_eq_string(stmt.query, "SELECT * FROM users WHERE id = ? AND name = ? AND created_at > ?")
assert_eq_int(stmt.parameter_count, 3)
assert_eq_int(len(stmt.parameter_types), 3)
assert_eq_string(stmt.parameter_types[0], "varchar")
assert_eq_string(stmt.parameter_types[1], "int")
assert_eq_string(stmt.parameter_types[2], "datetime")
assert_eq_int(len(stmt.bound_parameters), 3)
vibez.spill("✅ MySQL prepared statement created successfully")

fr fr Test 9: Parameter binding
test_start("MySQL parameter binding")
bind_result1 := bind_mysql_parameter(&stmt, 0, "1")
bind_result2 := bind_mysql_parameter(&stmt, 1, "John Doe")
bind_result3 := bind_mysql_parameter(&stmt, 2, "2025-01-01 00:00:00")
assert_true(bind_result1)
assert_true(bind_result2)
assert_true(bind_result3)
assert_eq_string(stmt.bound_parameters[0], "1")
assert_eq_string(stmt.bound_parameters[1], "John Doe")
assert_eq_string(stmt.bound_parameters[2], "2025-01-01 00:00:00")
vibez.spill("✅ MySQL parameters bound successfully")

fr fr Test 10: Prepared statement execution
test_start("MySQL prepared statement execution")
exec_result := execute_mysql_prepared_statement(&stmt)
assert_true(exec_result.success)
assert_eq_int(exec_result.rows_affected, 1)
assert_eq_int(len(exec_result.rows), 1)
assert_eq_string(exec_result.rows[0][0], "1")
assert_eq_string(exec_result.rows[0][1], "Test User")
assert_eq_int(exec_result.error_code, 0)
assert_eq_string(exec_result.error_message, "")
assert_eq_int(exec_result.insert_id, 1)
assert_eq_string(exec_result.server_info, "1 row in set")
vibez.spill("✅ MySQL prepared statement executed successfully")

fr fr Test 11: Transaction management
test_start("MySQL transaction management")
tx := begin_mysql_transaction(&connection, "READ COMMITTED")
assert_true(tx.is_active)
assert_eq_int(tx.connection_id, connection.connection_id)
assert_eq_string(tx.isolation_level, "READ COMMITTED")
assert_eq_int(tx.operations_count, 0)
assert_false(connection.autocommit)
assert_true(tx.autocommit_disabled)
vibez.spill("✅ MySQL transaction started successfully")

fr fr Test 12: Transaction commit
test_start("MySQL transaction commit")
commit_result := commit_mysql_transaction(&connection, &tx)
assert_true(commit_result)
assert_false(tx.is_active)
assert_true(connection.autocommit)
vibez.spill("✅ MySQL transaction committed successfully")

fr fr Test 13: Transaction rollback
test_start("MySQL transaction rollback")
tx2 := begin_mysql_transaction(&connection, "SERIALIZABLE")
assert_true(tx2.is_active)
assert_false(connection.autocommit)
rollback_result := rollback_mysql_transaction(&connection, &tx2)
assert_true(rollback_result)
assert_false(tx2.is_active)
assert_true(connection.autocommit)
vibez.spill("✅ MySQL transaction rolled back successfully")

fr fr Test 14: Autocommit management
test_start("MySQL autocommit management")
autocommit_disable_result := set_mysql_autocommit(&connection, cap)
assert_true(autocommit_disable_result)
assert_false(connection.autocommit)
autocommit_enable_result := set_mysql_autocommit(&connection, based)
assert_true(autocommit_enable_result)
assert_true(connection.autocommit)
vibez.spill("✅ MySQL autocommit management successful")

fr fr Test 15: Connection pool creation
test_start("MySQL connection pool creation")
pool := create_mysql_pool(config, 15)
assert_eq_int(pool.max_connections, 15)
assert_eq_int(pool.current_connections, 0)
assert_eq_int(len(pool.connections), 0)
assert_eq_int(len(pool.available_connections), 0)
assert_eq_int(pool.connection_timeout, 30)
vibez.spill("✅ MySQL connection pool created successfully")

fr fr Test 16: Pool connection management
test_start("MySQL pool connection management")
pool_conn := get_mysql_pool_connection(&pool)
assert_true(pool_conn.is_connected)
assert_eq_int(pool.current_connections, 1)
assert_eq_int(len(pool.connections), 1)
return_result := return_mysql_pool_connection(&pool, pool_conn.connection_id)
assert_true(return_result)
assert_eq_int(len(pool.available_connections), 1)
vibez.spill("✅ MySQL pool connection management successful")

fr fr Test 17: Health check
test_start("MySQL health check")
health_result := health_check_mysql(&connection)
assert_true(health_result)
vibez.spill("✅ MySQL health check passed")

fr fr Test 18: Server information
test_start("MySQL server information")
get_mysql_server_info(&connection)
vibez.spill("✅ MySQL server information retrieved")

fr fr Test 19: Pool statistics
test_start("MySQL pool statistics")
get_mysql_pool_stats(&pool)
vibez.spill("✅ MySQL pool statistics retrieved")

fr fr Test 20: Show processlist
test_start("MySQL show processlist")
processlist_result := show_mysql_processlist(&connection)
assert_true(processlist_result.success)
assert_eq_int(len(processlist_result.columns), 8)
assert_eq_string(processlist_result.columns[0], "Id")
assert_eq_string(processlist_result.columns[1], "User")
assert_eq_string(processlist_result.columns[2], "Host")
assert_eq_int(len(processlist_result.rows), 3)
assert_eq_string(processlist_result.rows[0][1], "root")
assert_eq_string(processlist_result.rows[1][1], "app_user")
assert_eq_string(processlist_result.rows[2][1], "readonly")
assert_eq_string(processlist_result.server_info, "3 rows in set")
vibez.spill("✅ MySQL processlist retrieved successfully")

fr fr Test 21: Connection disconnection
test_start("MySQL connection disconnection")
disconnect_result := disconnect_mysql(&connection)
assert_true(disconnect_result)
assert_false(connection.is_connected)
vibez.spill("✅ MySQL connection disconnected successfully")

fr fr Test 22: Error handling - disconnected connection
test_start("MySQL error handling - disconnected connection")
error_result := execute_mysql_query(&connection, "SELECT 1")
assert_false(error_result.success)
assert_eq_int(error_result.error_code, 2006)
assert_eq_string(error_result.error_message, "MySQL server has gone away")
vibez.spill("✅ MySQL error handling working correctly")

fr fr Test 23: Parameter binding error handling
test_start("MySQL parameter binding error handling")
stmt_error := prepare_mysql_statement(&connection, "SELECT 1")
assert_false(stmt_error.is_prepared)
bind_error := bind_mysql_parameter(&stmt_error, 0, "test")
assert_false(bind_error)
vibez.spill("✅ MySQL parameter binding error handling working")

fr fr Test 24: Invalid parameter index
test_start("MySQL invalid parameter index")
reconnect_mysql(&connection)
stmt_valid := prepare_mysql_statement(&connection, "SELECT * FROM users WHERE id = ?")
bind_invalid := bind_mysql_parameter(&stmt_valid, 5, "test")
assert_false(bind_invalid)
vibez.spill("✅ MySQL invalid parameter index handled correctly")

fr fr Test 25: Configuration validation
test_start("MySQL configuration validation")
custom_config := MySQLConfig{
    host: "mysql.example.com",
    port: 3307,
    database: "custom_db",
    username: "custom_user",
    password: "custom_pass",
    ssl_mode: "REQUIRED",
    charset: "utf8",
    collation: "utf8_general_ci",
    connect_timeout: 45,
    read_timeout: 60,
    write_timeout: 60,
    max_connections: 50,
    auto_reconnect: cap,
    compress: based
}
custom_connection := create_mysql_connection(custom_config)
assert_eq_string(custom_connection.config.host, "mysql.example.com")
assert_eq_int(custom_connection.config.port, 3307)
assert_eq_string(custom_connection.config.database, "custom_db")
assert_eq_string(custom_connection.config.ssl_mode, "REQUIRED")
assert_eq_string(custom_connection.config.charset, "utf8")
assert_eq_string(custom_connection.config.collation, "utf8_general_ci")
assert_false(custom_connection.config.auto_reconnect)
assert_true(custom_connection.config.compress)
vibez.spill("✅ MySQL configuration validation successful")

fr fr Test 26: Multiple connections
test_start("MySQL multiple connections")
config2 := create_mysql_config()
config2.database = "test_db2"
config2.port = 3307
connection2 := create_mysql_connection(config2)
connect_result2 := connect_mysql(&connection2)
assert_true(connect_result2)
assert_true(connection2.is_connected)
assert_eq_string(connection2.config.database, "test_db2")
assert_eq_int(connection2.config.port, 3307)
disconnect_mysql(&connection2)
vibez.spill("✅ MySQL multiple connections working correctly")

fr fr Test 27: Connection statistics tracking
test_start("MySQL connection statistics tracking")
reconnect_mysql(&connection)
initial_query_count := connection.query_count
execute_mysql_query(&connection, "SELECT 1")
execute_mysql_query(&connection, "SELECT 2")
assert_eq_int(connection.query_count, initial_query_count + 2)
vibez.spill("✅ MySQL connection statistics tracking working")

fr fr Test 28: Insert ID tracking
test_start("MySQL insert ID tracking")
initial_insert_id := connection.insert_id
insert_result1 := execute_mysql_query(&connection, "INSERT INTO users (name) VALUES ('User1')")
insert_result2 := execute_mysql_query(&connection, "INSERT INTO users (name) VALUES ('User2')")
assert_eq_int(insert_result1.insert_id, initial_insert_id + 1)
assert_eq_int(insert_result2.insert_id, initial_insert_id + 2)
vibez.spill("✅ MySQL insert ID tracking working correctly")

fr fr Test 29: Prepared statement warning count
test_start("MySQL prepared statement warning count")
stmt_warnings := prepare_mysql_statement(&connection, "SELECT * FROM users WHERE id = ?")
bind_mysql_parameter(&stmt_warnings, 0, "1")
exec_warnings := execute_mysql_prepared_statement(&stmt_warnings)
assert_eq_int(exec_warnings.warning_count, 0)
vibez.spill("✅ MySQL prepared statement warning count working")

fr fr Test 30: Pool exhaustion handling
test_start("MySQL pool exhaustion handling")
small_pool := create_mysql_pool(config, 1)
conn1 := get_mysql_pool_connection(&small_pool)
conn2 := get_mysql_pool_connection(&small_pool)
assert_true(conn1.is_connected)
assert_false(conn2.is_connected)
assert_eq_string(conn2.last_error, "Pool exhausted")
vibez.spill("✅ MySQL pool exhaustion handling working")

fr fr Helper function for reconnection
slay reconnect_mysql(connection: *MySQLConnection) {
    if connection.is_connected == cap {
        connect_mysql(connection)
    }
}

vibez.spill("🎉 All MySQL driver tests completed successfully!")
print_test_summary()
