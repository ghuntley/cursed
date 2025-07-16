yeet "testz"
yeet "database_drivers"
yeet "postgresql"

# PostgreSQL Database Driver Tests
# Comprehensive test suite for PostgreSQL driver functionality

test_start("PostgreSQL driver comprehensive tests")

# Test 1: Configuration creation
test_start("PostgreSQL configuration creation")
config := create_postgresql_config()
assert_eq_string(config.host, "localhost")
assert_eq_int(config.port, 5432)
assert_eq_string(config.database, "postgres")
assert_eq_string(config.username, "postgres")
assert_eq_string(config.ssl_mode, "prefer")
assert_eq_int(config.connect_timeout, 30)
assert_eq_int(config.max_connections, 100)
vibez.spill("✅ PostgreSQL configuration created successfully")

# Test 2: Connection creation
test_start("PostgreSQL connection creation")
connection := create_postgresql_connection(config)
assert_eq_string(connection.config.host, "localhost")
assert_eq_int(connection.config.port, 5432)
assert_eq_string(connection.config.database, "postgres")
assert_false(connection.is_connected)
assert_eq_string(connection.server_version, "")
assert_eq_string(connection.client_encoding, "UTF8")
assert_eq_string(connection.time_zone, "UTC")
assert_eq_string(connection.transaction_status, "idle")
assert_eq_int(connection.query_count, 0)
vibez.spill("✅ PostgreSQL connection created successfully")

# Test 3: Database connection
test_start("PostgreSQL database connection")
connect_result := connect_postgresql(&connection)
assert_true(connect_result)
assert_true(connection.is_connected)
assert_eq_string(connection.server_version, "PostgreSQL 14.10")
assert_eq_int(connection.process_id, 12345)
assert_eq_int(connection.secret_key, 67890)
assert_eq_string(connection.transaction_status, "idle")
vibez.spill("✅ PostgreSQL database connection established")

# Test 4: Query execution - SELECT
test_start("PostgreSQL SELECT query execution")
select_result := execute_postgresql_query(&connection, "SELECT * FROM users")
assert_true(select_result.success)
assert_eq_int(select_result.rows_affected, 0)
assert_eq_int(len(select_result.columns), 4)
assert_eq_string(select_result.columns[0], "id")
assert_eq_string(select_result.columns[1], "name")
assert_eq_string(select_result.columns[2], "email")
assert_eq_string(select_result.columns[3], "created_at")
assert_eq_int(len(select_result.rows), 2)
assert_eq_string(select_result.rows[0][0], "1")
assert_eq_string(select_result.rows[0][1], "John Doe")
assert_eq_string(select_result.error_code, "")
assert_eq_string(select_result.error_message, "")
vibez.spill("✅ PostgreSQL SELECT query executed successfully")

# Test 5: Query execution - INSERT
test_start("PostgreSQL INSERT query execution")
insert_result := execute_postgresql_query(&connection, "INSERT INTO users (name, email) VALUES ('Test User', 'test@example.com')")
assert_true(insert_result.success)
assert_eq_int(insert_result.rows_affected, 1)
assert_eq_int(len(insert_result.columns), 0)
assert_eq_string(insert_result.error_code, "")
assert_eq_string(insert_result.error_message, "")
assert_eq_int(insert_result.execution_time, 25)
vibez.spill("✅ PostgreSQL INSERT query executed successfully")

# Test 6: Query execution - UPDATE
test_start("PostgreSQL UPDATE query execution")
update_result := execute_postgresql_query(&connection, "UPDATE users SET name = 'Updated User' WHERE id = 1")
assert_true(update_result.success)
assert_eq_int(update_result.rows_affected, 1)
assert_eq_string(update_result.error_code, "")
assert_eq_string(update_result.error_message, "")
assert_eq_int(update_result.execution_time, 30)
vibez.spill("✅ PostgreSQL UPDATE query executed successfully")

# Test 7: Query execution - DELETE
test_start("PostgreSQL DELETE query execution")
delete_result := execute_postgresql_query(&connection, "DELETE FROM users WHERE id = 1")
assert_true(delete_result.success)
assert_eq_int(delete_result.rows_affected, 1)
assert_eq_string(delete_result.error_code, "")
assert_eq_string(delete_result.error_message, "")
assert_eq_int(delete_result.execution_time, 35)
vibez.spill("✅ PostgreSQL DELETE query executed successfully")

# Test 8: Prepared statement creation
test_start("PostgreSQL prepared statement creation")
stmt := prepare_postgresql_statement(&connection, "SELECT * FROM users WHERE id = $1 AND name = $2")
assert_true(stmt.is_prepared)
assert_eq_int(stmt.connection_id, connection.connection_id)
assert_eq_string(stmt.query, "SELECT * FROM users WHERE id = $1 AND name = $2")
assert_eq_int(stmt.parameter_count, 2)
assert_eq_int(len(stmt.parameter_types), 2)
assert_eq_string(stmt.parameter_types[0], "text")
assert_eq_string(stmt.parameter_types[1], "integer")
assert_eq_int(len(stmt.bound_parameters), 2)
vibez.spill("✅ PostgreSQL prepared statement created successfully")

# Test 9: Parameter binding
test_start("PostgreSQL parameter binding")
bind_result1 := bind_parameter(&stmt, 0, "1")
bind_result2 := bind_parameter(&stmt, 1, "John Doe")
assert_true(bind_result1)
assert_true(bind_result2)
assert_eq_string(stmt.bound_parameters[0], "1")
assert_eq_string(stmt.bound_parameters[1], "John Doe")
vibez.spill("✅ PostgreSQL parameters bound successfully")

# Test 10: Prepared statement execution
test_start("PostgreSQL prepared statement execution")
exec_result := execute_prepared_statement(&stmt)
assert_true(exec_result.success)
assert_eq_int(exec_result.rows_affected, 1)
assert_eq_int(len(exec_result.rows), 1)
assert_eq_string(exec_result.rows[0][0], "1")
assert_eq_string(exec_result.rows[0][1], "Test User")
assert_eq_string(exec_result.error_code, "")
assert_eq_string(exec_result.error_message, "")
vibez.spill("✅ PostgreSQL prepared statement executed successfully")

# Test 11: Transaction management
test_start("PostgreSQL transaction management")
tx := begin_postgresql_transaction(&connection, "READ COMMITTED")
assert_true(tx.is_active)
assert_eq_int(tx.connection_id, connection.connection_id)
assert_eq_string(tx.isolation_level, "READ COMMITTED")
assert_eq_int(tx.operations_count, 0)
assert_eq_string(connection.transaction_status, "active")
vibez.spill("✅ PostgreSQL transaction started successfully")

# Test 12: Transaction commit
test_start("PostgreSQL transaction commit")
commit_result := commit_postgresql_transaction(&connection, &tx)
assert_true(commit_result)
assert_false(tx.is_active)
assert_eq_string(connection.transaction_status, "idle")
vibez.spill("✅ PostgreSQL transaction committed successfully")

# Test 13: Transaction rollback
test_start("PostgreSQL transaction rollback")
tx2 := begin_postgresql_transaction(&connection, "SERIALIZABLE")
assert_true(tx2.is_active)
rollback_result := rollback_postgresql_transaction(&connection, &tx2)
assert_true(rollback_result)
assert_false(tx2.is_active)
assert_eq_string(connection.transaction_status, "idle")
vibez.spill("✅ PostgreSQL transaction rolled back successfully")

# Test 14: Savepoint management
test_start("PostgreSQL savepoint management")
tx3 := begin_postgresql_transaction(&connection, "READ COMMITTED")
assert_true(tx3.is_active)
savepoint_result := create_savepoint(&tx3, "sp1")
assert_true(savepoint_result)
assert_eq_int(len(tx3.savepoints), 1)
assert_eq_string(tx3.savepoints[0], "sp1")
rollback_savepoint_result := rollback_to_savepoint(&tx3, "sp1")
assert_true(rollback_savepoint_result)
commit_postgresql_transaction(&connection, &tx3)
vibez.spill("✅ PostgreSQL savepoint management successful")

# Test 15: Connection pool creation
test_start("PostgreSQL connection pool creation")
pool := create_postgresql_pool(config, 10)
assert_eq_int(pool.max_connections, 10)
assert_eq_int(pool.current_connections, 0)
assert_eq_int(len(pool.connections), 0)
assert_eq_int(len(pool.available_connections), 0)
vibez.spill("✅ PostgreSQL connection pool created successfully")

# Test 16: Pool connection management
test_start("PostgreSQL pool connection management")
pool_conn := get_pool_connection(&pool)
assert_true(pool_conn.is_connected)
assert_eq_int(pool.current_connections, 1)
assert_eq_int(len(pool.connections), 1)
return_result := return_pool_connection(&pool, pool_conn.connection_id)
assert_true(return_result)
assert_eq_int(len(pool.available_connections), 1)
vibez.spill("✅ PostgreSQL pool connection management successful")

# Test 17: Health check
test_start("PostgreSQL health check")
health_result := health_check_postgresql(&connection)
assert_true(health_result)
vibez.spill("✅ PostgreSQL health check passed")

# Test 18: Server information
test_start("PostgreSQL server information")
get_postgresql_server_info(&connection)
vibez.spill("✅ PostgreSQL server information retrieved")

# Test 19: Pool statistics
test_start("PostgreSQL pool statistics")
get_pool_stats(&pool)
vibez.spill("✅ PostgreSQL pool statistics retrieved")

# Test 20: Connection disconnection
test_start("PostgreSQL connection disconnection")
disconnect_result := disconnect_postgresql(&connection)
assert_true(disconnect_result)
assert_false(connection.is_connected)
vibez.spill("✅ PostgreSQL connection disconnected successfully")

# Test 21: Error handling - disconnected connection
test_start("PostgreSQL error handling - disconnected connection")
error_result := execute_postgresql_query(&connection, "SELECT 1")
assert_false(error_result.success)
assert_eq_string(error_result.error_code, "08003")
assert_eq_string(error_result.error_message, "Connection does not exist")
vibez.spill("✅ PostgreSQL error handling working correctly")

# Test 22: Parameter binding error handling
test_start("PostgreSQL parameter binding error handling")
stmt_error := prepare_postgresql_statement(&connection, "SELECT 1")
assert_false(stmt_error.is_prepared)
bind_error := bind_parameter(&stmt_error, 0, "test")
assert_false(bind_error)
vibez.spill("✅ PostgreSQL parameter binding error handling working")

# Test 23: Invalid parameter index
test_start("PostgreSQL invalid parameter index")
reconnect_postgresql(&connection)
stmt_valid := prepare_postgresql_statement(&connection, "SELECT * FROM users WHERE id = $1")
bind_invalid := bind_parameter(&stmt_valid, 5, "test")
assert_false(bind_invalid)
vibez.spill("✅ PostgreSQL invalid parameter index handled correctly")

# Test 24: Configuration validation
test_start("PostgreSQL configuration validation")
custom_config := PostgreSQLConfig{
    host: "custom.host.com",
    port: 5433,
    database: "custom_db",
    username: "custom_user",
    password: "custom_pass",
    ssl_mode: "require",
    connect_timeout: 60,
    query_timeout: 600,
    max_connections: 200,
    application_name: "custom_app"
}
custom_connection := create_postgresql_connection(custom_config)
assert_eq_string(custom_connection.config.host, "custom.host.com")
assert_eq_int(custom_connection.config.port, 5433)
assert_eq_string(custom_connection.config.database, "custom_db")
assert_eq_string(custom_connection.config.ssl_mode, "require")
vibez.spill("✅ PostgreSQL configuration validation successful")

# Test 25: Multiple connections
test_start("PostgreSQL multiple connections")
config2 := create_postgresql_config()
config2.database = "test_db2"
connection2 := create_postgresql_connection(config2)
connect_result2 := connect_postgresql(&connection2)
assert_true(connect_result2)
assert_true(connection2.is_connected)
assert_eq_string(connection2.config.database, "test_db2")
disconnect_postgresql(&connection2)
vibez.spill("✅ PostgreSQL multiple connections working correctly")

# Helper function for reconnection
slay reconnect_postgresql(connection: *PostgreSQLConnection) {
    if connection.is_connected == cap {
        connect_postgresql(connection)
    }
}

vibez.spill("🎉 All PostgreSQL driver tests completed successfully!")
print_test_summary()
