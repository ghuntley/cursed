yeet "testz"
yeet "database/mod_simple"

# Test database connection management
test_start("Database Connection Management")

# Test PostgreSQL configuration
sus pg_config DatabaseConfig = create_database_config(
    DB_POSTGRES,
    "localhost",
    5432,
    "testdb",
    "testuser",
    "testpass"
)

assert_eq_int(pg_config.db_type, DB_POSTGRES)
assert_eq_string(pg_config.host, "localhost")
assert_eq_int(pg_config.port, 5432)
assert_eq_string(pg_config.database, "testdb")

print_test_summary()

# Test MySQL configuration  
test_start("MySQL Configuration")

sus mysql_config DatabaseConfig = create_database_config(
    DB_MYSQL,
    "127.0.0.1",
    3306,
    "mydb",
    "root",
    "password"
)

assert_eq_int(mysql_config.db_type, DB_MYSQL)
assert_eq_string(mysql_config.host, "127.0.0.1")
assert_eq_int(mysql_config.port, 3306)

print_test_summary()

# Test SQLite configuration
test_start("SQLite Configuration")

sus sqlite_config DatabaseConfig = create_database_config(
    DB_SQLITE,
    "",
    0,
    "/tmp/test.db",
    "",
    ""
)

assert_eq_int(sqlite_config.db_type, DB_SQLITE)
assert_eq_string(sqlite_config.database, "/tmp/test.db")

print_test_summary()

# Test connection establishment
test_start("Database Connection Establishment")

# Test PostgreSQL connection
sus pg_conn tea = connect_database(pg_config)
assert_eq_string(pg_conn, "pg_conn_12345")

# Test MySQL connection
sus mysql_conn tea = connect_database(mysql_config)
assert_eq_string(mysql_conn, "mysql_conn_67890")

# Test SQLite connection
sus sqlite_conn tea = connect_sqlite("/tmp/test.db")
assert_eq_string(sqlite_conn, "sqlite_conn_54321")

print_test_summary()

# Test query execution
test_start("SQL Query Execution")

sus test_conn tea = connect_database(pg_config)

# Test SELECT query
sus select_result QueryResult = execute_query(test_conn, "SELECT * FROM users")
assert_true(select_result.success)
assert_eq_int(select_result.row_count, 1)

# Test INSERT query
sus insert_result QueryResult = execute_query(test_conn, "INSERT INTO users (name, email) VALUES ('John', 'john@example.com')")
assert_true(insert_result.success)

print_test_summary()

# Test connection cleanup
test_start("Connection Cleanup")

sus close_success lit = close_connection(test_conn)
assert_true(close_success)

print_test_summary()

vibez.spill("🎉 All database tests completed successfully!")
vibez.spill("Database layer provides basic support for PostgreSQL, MySQL, and SQLite")
