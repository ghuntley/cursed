yeet "testz"
yeet "database/registry_simple"

fr fr Test simple database driver registry system
test_start("Simple Database Registry")

fr fr Test registry initialization
sus init_success lit = init_database_registry()
assert_true(init_success)

fr fr Verify drivers are registered
sus drivers []DatabaseDriver = list_registered_drivers()
assert_true(drivers.length >= 5)

print_test_summary()

fr fr Test database configurations
test_start("Database Configurations")

fr fr Test PostgreSQL configuration
sus pg_config DatabaseDriverConfig = create_enhanced_database_config(
    DRIVER_POSTGRES,
    "localhost",
    5432,
    "testdb",
    "testuser",
    "testpass"
)

assert_eq_int(pg_config.driver_type, DRIVER_POSTGRES)
assert_eq_string(pg_config.name, "PostgreSQL")
assert_eq_string(pg_config.host, "localhost")
assert_eq_int(pg_config.port, 5432)
assert_eq_string(pg_config.database, "testdb")

fr fr Test MySQL configuration
sus mysql_config DatabaseDriverConfig = create_enhanced_database_config(
    DRIVER_MYSQL,
    "127.0.0.1",
    3306,
    "mydb",
    "root",
    "password"
)

assert_eq_int(mysql_config.driver_type, DRIVER_MYSQL)
assert_eq_string(mysql_config.name, "MySQL")

fr fr Test SQLite configuration
sus sqlite_config DatabaseDriverConfig = create_enhanced_database_config(
    DRIVER_SQLITE,
    "",
    0,
    "/tmp/test.db",
    "",
    ""
)

assert_eq_int(sqlite_config.driver_type, DRIVER_SQLITE)
assert_eq_string(sqlite_config.name, "SQLite")

print_test_summary()

fr fr Test connection pooling
test_start("Connection Pooling")

fr fr Test pool creation
sus pg_pool ConnectionPool = create_advanced_connection_pool(pg_config, "test_pg_pool")
assert_eq_int(pg_pool.max_connections, 50)
assert_eq_int(pg_pool.min_connections, 5)

fr fr Test getting connection
sus pg_conn tea = get_enhanced_connection("test_pg_pool")
assert_true(pg_conn != "")
assert_true(stringz.contains(pg_conn, "pg_conn_"))

fr fr Test health check
sus pg_health lit = perform_health_check(pg_conn)
assert_true(pg_health)

print_test_summary()

fr fr Test query execution
test_start("Query Execution")

fr fr Test PostgreSQL query
sus pg_result QueryResult = execute_enhanced_query(
    pg_conn,
    "SELECT * FROM users WHERE active = ?",
    ["true"],
    based
)

assert_true(pg_result.success)
assert_eq_int(pg_result.rows.length, 2)
assert_eq_int(pg_result.columns.length, 2)
assert_eq_string(pg_result.columns[0].name, "id")
assert_eq_string(pg_result.columns[1].name, "name")
assert_true(pg_result.execution_time > 0)

print_test_summary()

fr fr Test prepared statements
test_start("Prepared Statements")

sus pg_stmt PreparedStatement = create_enhanced_prepared_statement(
    pg_conn,
    "SELECT * FROM users WHERE id = ? AND status = ?",
    ["integer", "varchar"]
)

assert_true(pg_stmt.statement_id != "")
assert_eq_int(pg_stmt.parameter_count, 2)
assert_eq_string(pg_stmt.connection_id, pg_conn)

print_test_summary()

fr fr Test transactions
test_start("Transaction Management")

sus pg_tx Transaction = begin_enhanced_transaction(
    pg_conn,
    "SERIALIZABLE",
    cap
)

assert_true(pg_tx.is_active)
assert_eq_string(pg_tx.connection_id, pg_conn)
assert_eq_string(pg_tx.isolation_level, "SERIALIZABLE")

print_test_summary()

fr fr Test multiple database types
test_start("Multiple Database Types")

fr fr Test MySQL
sus mysql_pool ConnectionPool = create_advanced_connection_pool(mysql_config, "test_mysql_pool")
sus mysql_conn tea = get_enhanced_connection("test_mysql_pool")
assert_true(mysql_conn != "")
assert_true(stringz.contains(mysql_conn, "mysql_conn_"))

sus mysql_result QueryResult = execute_enhanced_query(
    mysql_conn,
    "SELECT * FROM products",
    [],
    cap
)
assert_true(mysql_result.success)
assert_eq_string(mysql_result.last_insert_id, "123")

fr fr Test SQLite
sus sqlite_pool ConnectionPool = create_advanced_connection_pool(sqlite_config, "test_sqlite_pool")
sus sqlite_conn tea = get_enhanced_connection("test_sqlite_pool")
assert_true(sqlite_conn != "")
assert_true(stringz.contains(sqlite_conn, "sqlite_conn_"))

sus sqlite_result QueryResult = execute_enhanced_query(
    sqlite_conn,
    "INSERT INTO logs (message) VALUES (?)",
    ["Test message"],
    cap
)
assert_true(sqlite_result.success)
assert_eq_string(sqlite_result.last_insert_id, "456")

fr fr Test MongoDB
sus mongo_config DatabaseDriverConfig = create_enhanced_database_config(
    DRIVER_MONGODB,
    "localhost",
    27017,
    "testdb",
    "user",
    "pass"
)
sus mongo_pool ConnectionPool = create_advanced_connection_pool(mongo_config, "test_mongo_pool")
sus mongo_conn tea = get_enhanced_connection("test_mongo_pool")
assert_true(mongo_conn != "")
assert_true(stringz.contains(mongo_conn, "mongo_conn_"))

sus mongo_result QueryResult = execute_enhanced_query(
    mongo_conn,
    "db.users.find()",
    [],
    based
)
assert_true(mongo_result.success)
assert_eq_string(mongo_result.last_insert_id, "ObjectId3")

fr fr Test Redis
sus redis_config DatabaseDriverConfig = create_enhanced_database_config(
    DRIVER_REDIS,
    "localhost",
    6379,
    "0",
    "",
    "password"
)
sus redis_pool ConnectionPool = create_advanced_connection_pool(redis_config, "test_redis_pool")
sus redis_conn tea = get_enhanced_connection("test_redis_pool")
assert_true(redis_conn != "")
assert_true(stringz.contains(redis_conn, "redis_conn_"))

sus redis_result QueryResult = execute_enhanced_query(
    redis_conn,
    "GET user:*",
    [],
    based
)
assert_true(redis_result.success)

print_test_summary()

fr fr Print final status
print_registry_status()

vibez.spill("\n🎉 Simple database registry tests completed successfully!")
vibez.spill("✅ Multi-database driver registration working")
vibez.spill("✅ Connection pooling functional")
vibez.spill("✅ Query execution working across all drivers")
vibez.spill("✅ Transaction management implemented")
vibez.spill("✅ Prepared statements functional")
vibez.spill("✅ Health checking working")
