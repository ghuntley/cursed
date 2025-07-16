yeet "testz"
yeet "database_drivers"
yeet "postgresql"
yeet "mysql"
yeet "sqlite"

# Complete Database Driver Tests
# Tests the registry and all individual database drivers

test_start("Complete Database Driver Tests - Registry + PostgreSQL + MySQL + SQLite")

# Initialize database driver registry
registry := create_driver_registry()

# Test 1: Initialize default drivers
test_start("Default drivers initialization")
success := init_default_drivers(&registry)
assert_true(success)
assert_eq_int(driver_count(&registry), 5)
vibez.spill("✅ Default drivers initialized successfully")

# Test 2: List registered drivers
test_start("List registered drivers")
drivers := list_drivers(&registry)
assert_eq_int(len(drivers), 5)
vibez.spill("✅ Drivers listed successfully")

# Test 3: Registry connection management
test_start("Registry connection management")
connection := create_connection(&registry, "postgresql")
assert_eq_string(connection.driver_name, "postgresql")
assert_true(connection.is_open)
assert_eq_int(connection.connection_id, 1)
vibez.spill("✅ Registry connection management working")

# Test 4: PostgreSQL Driver Tests
test_start("PostgreSQL driver comprehensive tests")
vibez.spill("🐘 Testing PostgreSQL driver...")

# PostgreSQL Configuration and Connection
pg_config := create_postgresql_config()
assert_eq_string(pg_config.host, "localhost")
assert_eq_int(pg_config.port, 5432)
assert_eq_string(pg_config.database, "postgres")
assert_eq_string(pg_config.username, "postgres")
assert_eq_string(pg_config.ssl_mode, "prefer")

pg_conn := create_postgresql_connection(pg_config)
assert_false(pg_conn.is_connected)
assert_eq_string(pg_conn.server_version, "")
assert_eq_string(pg_conn.client_encoding, "UTF8")
assert_eq_string(pg_conn.time_zone, "UTC")
assert_eq_string(pg_conn.transaction_status, "idle")

pg_connect_result := connect_postgresql(&pg_conn)
assert_true(pg_connect_result)
assert_true(pg_conn.is_connected)
assert_eq_string(pg_conn.server_version, "PostgreSQL 14.10")
assert_eq_int(pg_conn.process_id, 12345)
assert_eq_int(pg_conn.secret_key, 67890)

# PostgreSQL Query Execution
pg_select_result := execute_postgresql_query(&pg_conn, "SELECT * FROM users")
assert_true(pg_select_result.success)
assert_eq_int(len(pg_select_result.columns), 4)
assert_eq_string(pg_select_result.columns[0], "id")
assert_eq_string(pg_select_result.columns[1], "name")
assert_eq_string(pg_select_result.columns[2], "email")
assert_eq_string(pg_select_result.columns[3], "created_at")
assert_eq_int(len(pg_select_result.rows), 2)
assert_eq_string(pg_select_result.rows[0][0], "1")
assert_eq_string(pg_select_result.rows[0][1], "John Doe")

pg_insert_result := execute_postgresql_query(&pg_conn, "INSERT INTO users (name, email) VALUES ('Test User', 'test@example.com')")
assert_true(pg_insert_result.success)
assert_eq_int(pg_insert_result.rows_affected, 1)

# PostgreSQL Prepared Statements
pg_stmt := prepare_postgresql_statement(&pg_conn, "SELECT * FROM users WHERE id = $1 AND name = $2")
assert_true(pg_stmt.is_prepared)
assert_eq_int(pg_stmt.connection_id, pg_conn.connection_id)
assert_eq_string(pg_stmt.query, "SELECT * FROM users WHERE id = $1 AND name = $2")
assert_eq_int(pg_stmt.parameter_count, 2)

bind_result1 := bind_parameter(&pg_stmt, 0, "1")
bind_result2 := bind_parameter(&pg_stmt, 1, "John Doe")
assert_true(bind_result1)
assert_true(bind_result2)

pg_exec_result := execute_prepared_statement(&pg_stmt)
assert_true(pg_exec_result.success)
assert_eq_int(pg_exec_result.rows_affected, 1)

# PostgreSQL Transactions
pg_tx := begin_postgresql_transaction(&pg_conn, "READ COMMITTED")
assert_true(pg_tx.is_active)
assert_eq_int(pg_tx.connection_id, pg_conn.connection_id)
assert_eq_string(pg_tx.isolation_level, "READ COMMITTED")
assert_eq_string(pg_conn.transaction_status, "active")

commit_postgresql_transaction(&pg_conn, &pg_tx)
assert_false(pg_tx.is_active)
assert_eq_string(pg_conn.transaction_status, "idle")

# PostgreSQL Savepoints
pg_tx2 := begin_postgresql_transaction(&pg_conn, "SERIALIZABLE")
savepoint_result := create_savepoint(&pg_tx2, "sp1")
assert_true(savepoint_result)
assert_eq_int(len(pg_tx2.savepoints), 1)
assert_eq_string(pg_tx2.savepoints[0], "sp1")

rollback_savepoint_result := rollback_to_savepoint(&pg_tx2, "sp1")
assert_true(rollback_savepoint_result)
commit_postgresql_transaction(&pg_conn, &pg_tx2)

# PostgreSQL Connection Pool
pg_pool := create_postgresql_pool(pg_config, 10)
assert_eq_int(pg_pool.max_connections, 10)
assert_eq_int(pg_pool.current_connections, 0)

pg_pool_conn := get_pool_connection(&pg_pool)
assert_true(pg_pool_conn.is_connected)
assert_eq_int(pg_pool.current_connections, 1)

return_pool_connection(&pg_pool, pg_pool_conn.connection_id)
assert_eq_int(len(pg_pool.available_connections), 1)

# PostgreSQL Health Check
health_result := health_check_postgresql(&pg_conn)
assert_true(health_result)

disconnect_postgresql(&pg_conn)
assert_false(pg_conn.is_connected)

vibez.spill("✅ PostgreSQL driver tests completed successfully!")

# Test 5: MySQL Driver Tests
test_start("MySQL driver comprehensive tests")
vibez.spill("🐬 Testing MySQL driver...")

# MySQL Configuration and Connection
mysql_config := create_mysql_config()
assert_eq_string(mysql_config.host, "localhost")
assert_eq_int(mysql_config.port, 3306)
assert_eq_string(mysql_config.database, "mysql")
assert_eq_string(mysql_config.username, "root")
assert_eq_string(mysql_config.ssl_mode, "PREFERRED")
assert_eq_string(mysql_config.charset, "utf8mb4")

mysql_conn := create_mysql_connection(mysql_config)
assert_false(mysql_conn.is_connected)
assert_eq_string(mysql_conn.server_version, "")
assert_eq_int(mysql_conn.protocol_version, 10)
assert_eq_int(mysql_conn.thread_id, 0)
assert_eq_string(mysql_conn.charset, "utf8mb4")
assert_true(mysql_conn.autocommit)

mysql_connect_result := connect_mysql(&mysql_conn)
assert_true(mysql_connect_result)
assert_true(mysql_conn.is_connected)
assert_eq_string(mysql_conn.server_version, "8.0.35-MySQL")
assert_eq_int(mysql_conn.protocol_version, 10)
assert_eq_int(mysql_conn.thread_id, 123456)

# MySQL Query Execution
mysql_select_result := execute_mysql_query(&mysql_conn, "SELECT * FROM users")
assert_true(mysql_select_result.success)
assert_eq_int(len(mysql_select_result.columns), 4)
assert_eq_string(mysql_select_result.columns[0], "id")
assert_eq_string(mysql_select_result.columns[1], "name")
assert_eq_string(mysql_select_result.columns[2], "email")
assert_eq_string(mysql_select_result.columns[3], "created_at")
assert_eq_int(len(mysql_select_result.rows), 3)
assert_eq_string(mysql_select_result.rows[0][1], "John Doe")
assert_eq_string(mysql_select_result.rows[1][1], "Jane Smith")
assert_eq_string(mysql_select_result.rows[2][1], "Bob Johnson")

mysql_insert_result := execute_mysql_query(&mysql_conn, "INSERT INTO users (name, email) VALUES ('Test User', 'test@example.com')")
assert_true(mysql_insert_result.success)
assert_eq_int(mysql_insert_result.rows_affected, 1)
assert_eq_int(mysql_insert_result.insert_id, 1)

# MySQL Prepared Statements
mysql_stmt := prepare_mysql_statement(&mysql_conn, "SELECT * FROM users WHERE id = ? AND name = ? AND created_at > ?")
assert_true(mysql_stmt.is_prepared)
assert_eq_int(mysql_stmt.connection_id, mysql_conn.connection_id)
assert_eq_string(mysql_stmt.query, "SELECT * FROM users WHERE id = ? AND name = ? AND created_at > ?")
assert_eq_int(mysql_stmt.parameter_count, 3)

bind_mysql_result1 := bind_mysql_parameter(&mysql_stmt, 0, "1")
bind_mysql_result2 := bind_mysql_parameter(&mysql_stmt, 1, "John Doe")
bind_mysql_result3 := bind_mysql_parameter(&mysql_stmt, 2, "2025-01-01 00:00:00")
assert_true(bind_mysql_result1)
assert_true(bind_mysql_result2)
assert_true(bind_mysql_result3)

mysql_exec_result := execute_mysql_prepared_statement(&mysql_stmt)
assert_true(mysql_exec_result.success)
assert_eq_int(mysql_exec_result.rows_affected, 1)

# MySQL Transactions
mysql_tx := begin_mysql_transaction(&mysql_conn, "READ COMMITTED")
assert_true(mysql_tx.is_active)
assert_eq_int(mysql_tx.connection_id, mysql_conn.connection_id)
assert_eq_string(mysql_tx.isolation_level, "READ COMMITTED")
assert_false(mysql_conn.autocommit)

commit_mysql_transaction(&mysql_conn, &mysql_tx)
assert_false(mysql_tx.is_active)
assert_true(mysql_conn.autocommit)

# MySQL Autocommit Management
autocommit_disable_result := set_mysql_autocommit(&mysql_conn, cap)
assert_true(autocommit_disable_result)
assert_false(mysql_conn.autocommit)

autocommit_enable_result := set_mysql_autocommit(&mysql_conn, based)
assert_true(autocommit_enable_result)
assert_true(mysql_conn.autocommit)

# MySQL Connection Pool
mysql_pool := create_mysql_pool(mysql_config, 15)
assert_eq_int(mysql_pool.max_connections, 15)
assert_eq_int(mysql_pool.current_connections, 0)

mysql_pool_conn := get_mysql_pool_connection(&mysql_pool)
assert_true(mysql_pool_conn.is_connected)
assert_eq_int(mysql_pool.current_connections, 1)

return_mysql_pool_connection(&mysql_pool, mysql_pool_conn.connection_id)
assert_eq_int(len(mysql_pool.available_connections), 1)

# MySQL Health Check
mysql_health_result := health_check_mysql(&mysql_conn)
assert_true(mysql_health_result)

# MySQL Processlist
processlist_result := show_mysql_processlist(&mysql_conn)
assert_true(processlist_result.success)
assert_eq_int(len(processlist_result.columns), 8)
assert_eq_string(processlist_result.columns[0], "Id")
assert_eq_string(processlist_result.columns[1], "User")
assert_eq_int(len(processlist_result.rows), 3)

disconnect_mysql(&mysql_conn)
assert_false(mysql_conn.is_connected)

vibez.spill("✅ MySQL driver tests completed successfully!")

# Test 6: SQLite Driver Tests
test_start("SQLite driver comprehensive tests")
vibez.spill("🗄️  Testing SQLite driver...")

# SQLite Configuration and Connection
sqlite_config := create_sqlite_config("test.db")
assert_eq_string(sqlite_config.database_path, "test.db")
assert_eq_string(sqlite_config.mode, "rwc")
assert_eq_int(sqlite_config.cache_size, 2000)
assert_eq_int(sqlite_config.page_size, 4096)
assert_eq_string(sqlite_config.synchronous, "NORMAL")
assert_eq_string(sqlite_config.journal_mode, "WAL")
assert_true(sqlite_config.foreign_keys)

sqlite_conn := create_sqlite_connection(sqlite_config)
assert_false(sqlite_conn.is_connected)
assert_eq_string(sqlite_conn.database_path, "test.db")
assert_eq_string(sqlite_conn.sqlite_version, "")
assert_false(sqlite_conn.is_readonly)
assert_true(sqlite_conn.auto_commit)
assert_false(sqlite_conn.in_transaction)

sqlite_connect_result := connect_sqlite(&sqlite_conn)
assert_true(sqlite_connect_result)
assert_true(sqlite_conn.is_connected)
assert_eq_string(sqlite_conn.sqlite_version, "3.44.2")
assert_eq_int(len(sqlite_conn.pragma_settings), 4)

# SQLite Query Execution
sqlite_select_result := execute_sqlite_query(&sqlite_conn, "SELECT * FROM users")
assert_true(sqlite_select_result.success)
assert_eq_int(len(sqlite_select_result.columns), 4)
assert_eq_string(sqlite_select_result.columns[0], "id")
assert_eq_string(sqlite_select_result.columns[1], "name")
assert_eq_string(sqlite_select_result.columns[2], "email")
assert_eq_string(sqlite_select_result.columns[3], "created_at")
assert_eq_int(len(sqlite_select_result.rows), 3)
assert_eq_string(sqlite_select_result.rows[0][1], "Alice Johnson")
assert_eq_string(sqlite_select_result.rows[1][1], "Bob Smith")
assert_eq_string(sqlite_select_result.rows[2][1], "Carol Davis")

sqlite_insert_result := execute_sqlite_query(&sqlite_conn, "INSERT INTO users (name, email) VALUES ('Test User', 'test@example.com')")
assert_true(sqlite_insert_result.success)
assert_eq_int(sqlite_insert_result.rows_affected, 1)
assert_eq_int(sqlite_insert_result.last_insert_rowid, 1)

sqlite_create_result := execute_sqlite_query(&sqlite_conn, "CREATE TABLE test_table (id INTEGER PRIMARY KEY, name TEXT)")
assert_true(sqlite_create_result.success)
assert_eq_int(sqlite_create_result.rows_affected, 0)

sqlite_pragma_result := execute_sqlite_query(&sqlite_conn, "PRAGMA journal_mode")
assert_true(sqlite_pragma_result.success)
assert_eq_int(len(sqlite_pragma_result.columns), 1)
assert_eq_string(sqlite_pragma_result.columns[0], "pragma_value")
assert_eq_string(sqlite_pragma_result.rows[0][0], "WAL")

# SQLite Prepared Statements
sqlite_stmt := prepare_sqlite_statement(&sqlite_conn, "SELECT * FROM users WHERE id = ? AND name = :name")
assert_true(sqlite_stmt.is_prepared)
assert_eq_int(sqlite_stmt.connection_id, sqlite_conn.connection_id)
assert_eq_string(sqlite_stmt.query, "SELECT * FROM users WHERE id = ? AND name = :name")
assert_eq_int(sqlite_stmt.parameter_count, 2)
assert_true(sqlite_stmt.is_readonly)

bind_sqlite_result1 := bind_sqlite_parameter(&sqlite_stmt, 0, "1")
bind_sqlite_result2 := bind_sqlite_parameter(&sqlite_stmt, 1, "John Doe")
assert_true(bind_sqlite_result1)
assert_true(bind_sqlite_result2)

bind_named_result := bind_sqlite_named_parameter(&sqlite_stmt, ":name", "Alice")
assert_true(bind_named_result)

sqlite_exec_result := execute_sqlite_prepared_statement(&sqlite_stmt)
assert_true(sqlite_exec_result.success)
assert_eq_int(sqlite_exec_result.rows_affected, 1)

# SQLite Transactions
sqlite_tx := begin_sqlite_transaction(&sqlite_conn, "IMMEDIATE")
assert_true(sqlite_tx.is_active)
assert_eq_int(sqlite_tx.connection_id, sqlite_conn.connection_id)
assert_eq_string(sqlite_tx.transaction_type, "IMMEDIATE")
assert_true(sqlite_conn.in_transaction)
assert_false(sqlite_conn.auto_commit)

commit_sqlite_transaction(&sqlite_conn, &sqlite_tx)
assert_false(sqlite_tx.is_active)
assert_false(sqlite_conn.in_transaction)
assert_true(sqlite_conn.auto_commit)

# SQLite Savepoints
sqlite_tx2 := begin_sqlite_transaction(&sqlite_conn, "DEFERRED")
assert_true(sqlite_tx2.is_active)
assert_true(sqlite_tx2.is_readonly)

sqlite_savepoint_result := create_sqlite_savepoint(&sqlite_tx2, "sp1")
assert_true(sqlite_savepoint_result)
assert_eq_int(len(sqlite_tx2.savepoints), 1)
assert_eq_string(sqlite_tx2.savepoints[0], "sp1")
assert_eq_int(sqlite_tx2.nested_level, 1)

sqlite_rollback_savepoint_result := rollback_sqlite_to_savepoint(&sqlite_tx2, "sp1")
assert_true(sqlite_rollback_savepoint_result)
assert_eq_int(sqlite_tx2.nested_level, 0)

commit_sqlite_transaction(&sqlite_conn, &sqlite_tx2)

# SQLite PRAGMA Operations
pragma_exec_result := execute_sqlite_pragma(&sqlite_conn, "foreign_keys", "ON")
assert_true(pragma_exec_result.success)
assert_eq_int(len(sqlite_conn.pragma_settings), 5)

# SQLite Database Operations
vacuum_result := vacuum_sqlite_database(&sqlite_conn)
assert_true(vacuum_result.success)

analyze_result := analyze_sqlite_database(&sqlite_conn)
assert_true(analyze_result.success)

table_info_result := get_sqlite_table_info(&sqlite_conn, "users")
assert_true(table_info_result.success)

# SQLite Health Check
sqlite_health_result := health_check_sqlite(&sqlite_conn)
assert_true(sqlite_health_result)

disconnect_sqlite(&sqlite_conn)
assert_false(sqlite_conn.is_connected)

vibez.spill("✅ SQLite driver tests completed successfully!")

# Test 7: Cross-driver compatibility
test_start("Cross-driver compatibility tests")
vibez.spill("🔄 Testing cross-driver compatibility...")

# Reconnect all drivers
connect_postgresql(&pg_conn)
connect_mysql(&mysql_conn)
connect_sqlite(&sqlite_conn)

# Test same query across all drivers
query := "SELECT * FROM users WHERE id = 1"

pg_result := execute_postgresql_query(&pg_conn, query)
mysql_result := execute_mysql_query(&mysql_conn, query)
sqlite_result := execute_sqlite_query(&sqlite_conn, query)

assert_true(pg_result.success)
assert_true(mysql_result.success)
assert_true(sqlite_result.success)

# All drivers should return some form of success
assert_eq_string(pg_result.error_code, "")
assert_eq_int(mysql_result.error_code, 0)
assert_eq_int(sqlite_result.error_code, 0)

vibez.spill("✅ Cross-driver compatibility tests completed successfully!")

# Test 8: Error handling across drivers
test_start("Error handling across drivers")
vibez.spill("❌ Testing error handling...")

# Disconnect all connections
disconnect_postgresql(&pg_conn)
disconnect_mysql(&mysql_conn)
disconnect_sqlite(&sqlite_conn)

# Test error handling on disconnected connections
pg_error := execute_postgresql_query(&pg_conn, "SELECT 1")
mysql_error := execute_mysql_query(&mysql_conn, "SELECT 1")
sqlite_error := execute_sqlite_query(&sqlite_conn, "SELECT 1")

assert_false(pg_error.success)
assert_false(mysql_error.success)
assert_false(sqlite_error.success)

# Verify error codes are set
assert_eq_string(pg_error.error_code, "08003")
assert_eq_int(mysql_error.error_code, 2006)
assert_eq_int(sqlite_error.error_code, 21)

vibez.spill("✅ Error handling tests completed successfully!")

# Test 9: Performance and load testing
test_start("Performance and load testing")
vibez.spill("⚡ Testing performance...")

# Reconnect for performance testing
connect_postgresql(&pg_conn)
connect_mysql(&mysql_conn)
connect_sqlite(&sqlite_conn)

# Test multiple queries
bestie i := 0; i < 10; i++ {
    pg_perf := execute_postgresql_query(&pg_conn, "SELECT 1")
    mysql_perf := execute_mysql_query(&mysql_conn, "SELECT 1")
    sqlite_perf := execute_sqlite_query(&sqlite_conn, "SELECT 1")
    
    assert_true(pg_perf.success)
    assert_true(mysql_perf.success)
    assert_true(sqlite_perf.success)
}

# Test connection statistics
assert_eq_int(pg_conn.query_count, 11)  # 10 + 1 from earlier test
assert_eq_int(mysql_conn.query_count, 11)
assert_eq_int(sqlite_conn.query_count, 11)

vibez.spill("✅ Performance and load testing completed successfully!")

# Test 10: Advanced features testing
test_start("Advanced features testing")
vibez.spill("🚀 Testing advanced features...")

# Test connection pooling under load
pg_pool_load := create_postgresql_pool(pg_config, 5)
mysql_pool_load := create_mysql_pool(mysql_config, 5)

# Test pool exhaustion
connections := []PostgreSQLConnection{}
bestie i := 0; i < 6; i++ {
    pool_conn := get_pool_connection(&pg_pool_load)
    connections = append(connections, pool_conn)
}

# The 6th connection should fail (pool exhausted)
assert_false(connections[5].is_connected)
assert_eq_string(connections[5].last_error, "Pool exhausted")

# Return connections to pool
bestie i := 0; i < 5; i++ {
    if connections[i].is_connected {
        return_pool_connection(&pg_pool_load, connections[i].connection_id)
    }
}

vibez.spill("✅ Advanced features testing completed successfully!")

# Final cleanup
disconnect_postgresql(&pg_conn)
disconnect_mysql(&mysql_conn)
disconnect_sqlite(&sqlite_conn)

print_test_summary()

vibez.spill("🎉 All Complete Database Driver Tests passed successfully!")
vibez.spill("📊 Registry Management: ✅ Complete")
vibez.spill("🐘 PostgreSQL Driver: ✅ Complete")
vibez.spill("🐬 MySQL Driver: ✅ Complete")
vibez.spill("🗄️  SQLite Driver: ✅ Complete")
vibez.spill("🔄 Cross-driver compatibility: ✅ Complete")
vibez.spill("❌ Error handling: ✅ Complete")
vibez.spill("⚡ Performance testing: ✅ Complete")
vibez.spill("🚀 Advanced features: ✅ Complete")
vibez.spill("🔒 Pure CURSED implementation with production-ready features")
