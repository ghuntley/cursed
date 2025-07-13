yeet "testz"
yeet "database"
yeet "database/postgres"
yeet "database/mysql"
yeet "database/sqlite"

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

# Test MySQL configuration  
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

# Test SQLite configuration
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
assert_true(stringz.contains(pg_conn, "pg_conn_"))

# Test MySQL connection
sus mysql_conn tea = connect_database(mysql_config)
assert_true(stringz.contains(mysql_conn, "mysql_conn_"))

# Test SQLite connection
sus sqlite_conn tea = connect_sqlite("/tmp/test.db")
assert_true(stringz.contains(sqlite_conn, "sqlite_conn_"))

print_test_summary()

# Test connection pooling
test_start("Connection Pool Management")

sus pool ConnectionPool = create_connection_pool(pg_config)
assert_eq_int(pool.max_connections, 10)
assert_eq_int(pool.current_connections, 0)

sus pool_conn tea = get_connection_from_pool(pool)
assert_true(pool_conn != "")
assert_eq_int(pool.current_connections, 1)

sus return_success lit = return_connection_to_pool(pool, pool_conn)
assert_true(return_success)
assert_eq_int(pool.current_connections, 0)

print_test_summary()

# Test SQL query execution
test_start("SQL Query Execution")

sus test_conn tea = connect_database(pg_config)

# Test SELECT query
sus select_result QueryResult = execute_query(test_conn, "SELECT * FROM users", [])
assert_true(select_result.success)
assert_eq_int(select_result.columns.length, 3)
assert_eq_string(select_result.columns[0], "id")
assert_eq_string(select_result.columns[1], "name")
assert_eq_string(select_result.columns[2], "email")
assert_eq_int(select_result.rows.length, 2)

# Test INSERT query
sus insert_result QueryResult = execute_query(test_conn, "INSERT INTO users (name, email) VALUES (?, ?)", ["John Doe", "john@example.com"])
assert_true(insert_result.success)
assert_eq_int(insert_result.affected_rows, 1)
assert_true(insert_result.last_insert_id != "")

# Test UPDATE query
sus update_result QueryResult = execute_query(test_conn, "UPDATE users SET email = ? WHERE id = ?", ["newemail@example.com", "1"])
assert_true(update_result.success)
assert_eq_int(update_result.affected_rows, 1)

# Test DELETE query
sus delete_result QueryResult = execute_query(test_conn, "DELETE FROM users WHERE id = ?", ["1"])
assert_true(delete_result.success)
assert_eq_int(delete_result.affected_rows, 1)

print_test_summary()

# Test prepared statements
test_start("Prepared Statements")

sus stmt PreparedStatement = prepare_statement(test_conn, "SELECT * FROM users WHERE id = ? AND name = ?")
assert_true(stmt.statement_id != "")
assert_eq_string(stmt.sql_query, "SELECT * FROM users WHERE id = ? AND name = ?")
assert_eq_int(stmt.parameter_count, 2)

sus stmt_result QueryResult = execute_prepared_statement(stmt, ["1", "John Doe"])
assert_true(stmt_result.success)

# Test parameter count mismatch
sus wrong_params_result QueryResult = execute_prepared_statement(stmt, ["1"])
assert_false(wrong_params_result.success)
assert_eq_string(wrong_params_result.error_message, "Parameter count mismatch")

print_test_summary()

# Test transaction management
test_start("Transaction Management")

sus tx Transaction = begin_transaction(test_conn)
assert_true(tx.is_active)
assert_true(tx.transaction_id != "")
assert_eq_string(tx.connection_id, test_conn)
assert_eq_string(tx.isolation_level, "READ_COMMITTED")

# Test commit
sus commit_success lit = commit_transaction(tx)
assert_true(commit_success)
assert_false(tx.is_active)

# Test rollback
sus tx2 Transaction = begin_transaction(test_conn)
assert_true(tx2.is_active)

sus rollback_success lit = rollback_transaction(tx2)
assert_true(rollback_success)
assert_false(tx2.is_active)

print_test_summary()

# Test query builder
test_start("Query Builder")

sus builder QueryBuilder = new_query_builder("users")
assert_eq_string(builder.table_name, "users")
assert_eq_int(builder.select_fields.length, 0)

builder = query_select(builder, ["id", "name", "email"])
assert_eq_int(builder.select_fields.length, 3)

builder = query_where(builder, "age > 18")
builder = query_where(builder, "status = 'active'")
assert_eq_int(builder.where_conditions.length, 2)

builder = query_order_by(builder, "name ASC")
builder = query_limit(builder, 10)
builder = query_offset(builder, 20)

sus built_query tea = build_select_query(builder)
assert_true(stringz.contains(built_query, "SELECT id, name, email"))
assert_true(stringz.contains(built_query, "FROM users"))
assert_true(stringz.contains(built_query, "WHERE age > 18 AND status = 'active'"))
assert_true(stringz.contains(built_query, "ORDER BY name ASC"))
assert_true(stringz.contains(built_query, "LIMIT 10"))
assert_true(stringz.contains(built_query, "OFFSET 20"))

print_test_summary()

# Test migration system
test_start("Migration System")

sus migration Migration = create_migration(
    "001",
    "Create users table",
    "CREATE TABLE users (id SERIAL PRIMARY KEY, name VARCHAR(255), email VARCHAR(255))",
    "DROP TABLE users"
)

assert_eq_string(migration.version, "001")
assert_eq_string(migration.description, "Create users table")
assert_true(stringz.contains(migration.up_sql, "CREATE TABLE users"))
assert_true(stringz.contains(migration.down_sql, "DROP TABLE users"))

sus apply_success lit = apply_migration(test_conn, migration)
assert_true(apply_success)

sus rollback_migration_success lit = rollback_migration(test_conn, migration)
assert_true(rollback_migration_success)

print_test_summary()

# Test ORM functionality
test_start("ORM Record Management")

sus user_record Record = new_record("users")
assert_eq_string(user_record.table_name, "users")
assert_true(user_record.is_new)
assert_false(user_record.is_dirty)

sus set_success lit = set_field(user_record, "name", "Alice Johnson")
assert_true(set_success)
assert_true(user_record.is_dirty)

sus name_value tea = get_field(user_record, "name")
assert_eq_string(name_value, "Alice Johnson")

set_field(user_record, "email", "alice@example.com")
set_field(user_record, "age", "25")

sus save_success lit = save_record(test_conn, user_record)
assert_true(save_success)
assert_false(user_record.is_new)
assert_false(user_record.is_dirty)

print_test_summary()

# Test PostgreSQL specific features
test_start("PostgreSQL Specific Features")

sus pg_specific_config PostgresConfig = postgres_create_config(
    "localhost",
    5432,
    "testdb",
    "testuser",
    "testpass"
)

assert_eq_string(pg_specific_config.host, "localhost")
assert_eq_int(pg_specific_config.port, 5432)
assert_eq_string(pg_specific_config.sslmode, "prefer")

sus pg_conn_string tea = postgres_connection_string(pg_specific_config)
assert_true(stringz.contains(pg_conn_string, "postgresql://"))
assert_true(stringz.contains(pg_conn_string, "sslmode=prefer"))

# Test PostgreSQL data type formatting
sus pg_text_value tea = postgres_format_value("John's Data", "text")
assert_eq_string(pg_text_value, "'John''s Data'")

sus pg_json_value tea = postgres_format_value('{"key": "value"}', "json")
assert_true(stringz.contains(pg_json_value, "::json"))

# Test PostgreSQL JSON operations
sus json_extract tea = postgres_json_extract("data", "user.name")
assert_eq_string(json_extract, "data->'user.name'")

sus json_extract_text tea = postgres_json_extract_text("data", "user.email")
assert_eq_string(json_extract_text, "data->>'user.email'")

print_test_summary()

# Test MySQL specific features
test_start("MySQL Specific Features")

sus mysql_specific_config MySQLConfig = mysql_create_config(
    "127.0.0.1",
    3306,
    "mydb",
    "root",
    "password"
)

assert_eq_string(mysql_specific_config.host, "127.0.0.1")
assert_eq_int(mysql_specific_config.port, 3306)
assert_eq_string(mysql_specific_config.charset, "utf8mb4")

sus mysql_conn_string tea = mysql_connection_string(mysql_specific_config)
assert_true(stringz.contains(mysql_conn_string, "mysql://"))
assert_true(stringz.contains(mysql_conn_string, "charset=utf8mb4"))

# Test MySQL data type formatting
sus mysql_text_value tea = mysql_format_value("Test's Data", "varchar")
assert_true(stringz.contains(mysql_text_value, "\\'"))

sus mysql_bool_true tea = mysql_format_value("true", "boolean")
assert_eq_string(mysql_bool_true, "1")

sus mysql_bool_false tea = mysql_format_value("false", "boolean")
assert_eq_string(mysql_bool_false, "0")

# Test MySQL JSON operations
sus mysql_json_extract tea = mysql_json_extract("data", "user.name")
assert_true(stringz.contains(mysql_json_extract, "JSON_EXTRACT"))

print_test_summary()

# Test SQLite specific features
test_start("SQLite Specific Features")

sus sqlite_specific_config SQLiteConfig = sqlite_create_config("/tmp/test.db")
assert_eq_string(sqlite_specific_config.database_path, "/tmp/test.db")
assert_eq_string(sqlite_specific_config.journal_mode, "WAL")
assert_true(sqlite_specific_config.foreign_keys)

sus sqlite_conn_string tea = sqlite_connection_string(sqlite_specific_config)
assert_true(stringz.contains(sqlite_conn_string, "file:/tmp/test.db"))

# Test SQLite pragmas
sus foreign_keys_pragma tea = sqlite_enable_foreign_keys()
assert_eq_string(foreign_keys_pragma, "PRAGMA foreign_keys = ON")

sus journal_pragma tea = sqlite_set_journal_mode("WAL")
assert_eq_string(journal_pragma, "PRAGMA journal_mode = WAL")

# Test SQLite data type formatting
sus sqlite_text_value tea = sqlite_format_value("Test's Data", "text")
assert_eq_string(sqlite_text_value, "'Test''s Data'")

sus sqlite_bool_true tea = sqlite_format_value("true", "boolean")
assert_eq_string(sqlite_bool_true, "1")

# Test SQLite JSON operations
sus sqlite_json_extract tea = sqlite_json_extract("data", "user.name")
assert_true(stringz.contains(sqlite_json_extract, "json_extract"))

# Test SQLite date functions
sus current_time tea = sqlite_current_timestamp()
assert_eq_string(current_time, "datetime('now')")

print_test_summary()

# Test utility functions
test_start("Utility Functions")

sus param_count normie = count_parameters("SELECT * FROM users WHERE id = ? AND name = ? AND email = ?")
assert_eq_int(param_count, 3)

sus zero_params normie = count_parameters("SELECT * FROM users")
assert_eq_int(zero_params, 0)

sus escaped_string tea = escape_string("John's \"quoted\" data")
assert_true(stringz.contains(escaped_string, "''"))

sus close_success lit = close_connection(test_conn)
assert_true(close_success)

print_test_summary()

# Test error handling
test_start("Database Error Handling")

# Test PostgreSQL error parsing
sus pg_duplicate_error tea = postgres_parse_error("duplicate key value violates unique constraint")
assert_eq_string(pg_duplicate_error, "DUPLICATE_KEY_ERROR")

sus pg_fk_error tea = postgres_parse_error("foreign key constraint fails")
assert_eq_string(pg_fk_error, "FOREIGN_KEY_ERROR")

# Test MySQL error code parsing
sus mysql_duplicate_error tea = mysql_parse_error_code(1062)
assert_eq_string(mysql_duplicate_error, "DUPLICATE_ENTRY")

sus mysql_table_error tea = mysql_parse_error_code(1146)
assert_eq_string(mysql_table_error, "TABLE_DOESNT_EXIST")

# Test SQLite error parsing
sus sqlite_unique_error tea = sqlite_parse_error("UNIQUE constraint failed")
assert_eq_string(sqlite_unique_error, "UNIQUE_CONSTRAINT_ERROR")

sus sqlite_locked_error tea = sqlite_parse_error("database is locked")
assert_eq_string(sqlite_locked_error, "DATABASE_LOCKED_ERROR")

print_test_summary()

# Test performance features
test_start("Database Performance Features")

# Test PostgreSQL pool creation
sus pg_pool PostgresPool = postgres_create_pool(pg_specific_config, 5, 20)
assert_eq_int(pg_pool.min_connections, 5)
assert_eq_int(pg_pool.max_connections, 20)

# Test MySQL pool creation
sus mysql_pool MySQLPool = mysql_create_pool(mysql_specific_config, 3, 15)
assert_eq_int(mysql_pool.min_connections, 3)
assert_eq_int(mysql_pool.max_connections, 15)

# Test SQLite pool creation
sus sqlite_pool SQLitePool = sqlite_create_pool(sqlite_specific_config, 10)
assert_eq_int(sqlite_pool.max_connections, 10)

print_test_summary()

vibez.spill("🎉 All database tests completed successfully!")
vibez.spill("Database layer provides comprehensive support for PostgreSQL, MySQL, and SQLite")
vibez.spill("Features include: connection pooling, query builder, ORM, migrations, and transactions")
