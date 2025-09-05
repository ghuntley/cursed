yeet "dbz"
yeet "testz"

fr fr ===== DBZ MODULE TESTS =====

test_start("Database Connection - SQLite")

sus sqlite_conn drip = db_connect_sqlite("/tmp/test.db")
assert_true(sqlite_conn > 0)
assert_true(db_is_connected(sqlite_conn))

test_start("Database Connection - PostgreSQL")

sus postgres_conn drip = db_connect_postgres("localhost", 5432, "testdb", "user", "pass")
assert_true(postgres_conn > 0)
assert_true(db_is_connected(postgres_conn))

test_start("Database Connection - MySQL")

sus mysql_conn drip = db_connect_mysql("localhost", 3306, "testdb", "user", "pass")
assert_true(mysql_conn > 0)
assert_true(db_is_connected(mysql_conn))

test_start("SQL Query Execution")

assert_eq_int(db_execute("CREATE TABLE users (id INT, name TEXT, email TEXT)"), 0)
assert_eq_int(db_execute("INSERT INTO users VALUES (1, 'John', 'john@example.com')"), 0)
assert_eq_int(db_execute("UPDATE users SET name='Jane' WHERE id=1"), 0)
assert_eq_int(db_execute("DELETE FROM users WHERE id=1"), 0)

test_start("SELECT Queries")

sus user_results tea[value] = db_query("SELECT * FROM users")
assert_true(len(user_results) > 0)
assert_true(contains_substring(user_results[0], "name"))

sus product_results tea[value] = db_query("SELECT * FROM products")
assert_true(len(product_results) > 0)

test_start("Single Result Queries")

sus first_user tea = db_query_single("SELECT * FROM users")
assert_true(string_length(first_user) > 0)

sus user_count tea = db_query_scalar("SELECT COUNT(*) FROM users")
assert_true(string_length(user_count) > 0)

test_start("Prepared Statements")

sus stmt_id drip = db_prepare("INSERT INTO users (name, email) VALUES (?, ?)")
assert_true(stmt_id > 0)

assert_true(db_bind_param(stmt_id, 1, "Test User"))
assert_true(db_bind_param(stmt_id, 2, "test@example.com"))

assert_eq_int(db_execute_prepared(stmt_id), 0)
assert_true(db_finalize_statement(stmt_id))

test_start("Transaction Management")

assert_eq_int(db_begin_transaction(), 0)
assert_eq_int(db_execute("INSERT INTO users VALUES (99, 'Trans User', 'trans@example.com')"), 0)
assert_eq_int(db_commit_transaction(), 0)

assert_eq_int(db_begin_transaction(), 0)
assert_eq_int(db_execute("INSERT INTO users VALUES (98, 'Rollback User', 'rollback@example.com')"), 0)
assert_eq_int(db_rollback_transaction(), 0)

test_start("Schema Operations")

assert_eq_int(db_create_table("test_table", "id INT PRIMARY KEY, name TEXT"), 0)
assert_true(db_table_exists("test_table"))

sus tables tea[value] = db_list_tables()
assert_true(len(tables) > 0)

sus table_desc tea[value] = db_describe_table("users")
assert_true(len(table_desc) > 0)

assert_eq_int(db_drop_table("test_table"), 0)

test_start("CRUD Operations")

assert_eq_int(db_insert("users", "name, email", "'Bob', 'bob@example.com'"), 0)
assert_eq_int(db_update("users", "name='Robert'", "name='Bob'"), 0)

sus selected_users tea[value] = db_select("users", "*", "name='Robert'")
assert_true(len(selected_users) > 0)

sus user_count_num drip = db_count("users", "")
assert_true(user_count_num >= 0)

assert_eq_int(db_delete("users", "name='Robert'"), 0)

test_start("SQL Injection Protection")

assert_true(validate_sql_injection("normal input"))
assert_false(validate_sql_injection("'; DROP TABLE users; --"))
assert_false(validate_sql_injection("1' OR '1'='1"))

test_start("String Escaping")

sus escaped tea = escape_sql_string("John's Data")
assert_true(contains_substring(escaped, "''"))

sus safe_table tea = sanitize_table_name("table_name123")
assert_eq_string(safe_table, "table_name123")

sus unsafe_table tea = sanitize_table_name("table'; DROP--")
assert_false(contains_substring(unsafe_table, "'"))
assert_false(contains_substring(unsafe_table, ";"))

test_start("Query Builder")

sus columns tea[value] = ["name", "email"]
sus conditions tea[value] = ["id > 0", "name IS NOT NULL"]
sus select_query tea = build_select_query("users", columns, conditions)

assert_true(contains_substring(select_query, "SELECT"))
assert_true(contains_substring(select_query, "name, email"))
assert_true(contains_substring(select_query, "FROM users"))
assert_true(contains_substring(select_query, "WHERE"))

test_start("Case Conversion Utilities")

assert_true(starts_with_upper("select", "SELECT"))
assert_true(starts_with_upper("INSERT", "insert"))
assert_true(contains_substring_upper("Select * FROM users", "FROM"))

sus upper_text tea = to_upper("hello world")
assert_true(contains_substring(upper_text, "HELLO"))
assert_true(contains_substring(upper_text, "WORLD"))

test_start("Character Validation")

assert_true(is_alphanumeric_char("A"))
assert_true(is_alphanumeric_char("5"))
assert_true(is_alphanumeric_char("z"))
assert_false(is_alphanumeric_char("@"))
assert_false(is_alphanumeric_char("'"))

test_start("String to Number Conversion")

assert_eq_int(string_to_number("42"), 42)
assert_eq_int(string_to_number("0"), 0)
assert_eq_int(string_to_number("invalid"), 0)

test_start("Database Disconnection")

assert_true(db_disconnect(mysql_conn))
assert_false(db_is_connected(mysql_conn))

print_test_summary()
