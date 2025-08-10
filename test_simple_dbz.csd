yeet "dbz"
yeet "testz"

fr fr ===== SIMPLE DATABASE DRIVER TEST =====

test_start("PostgreSQL Connection")

sus pg_conn drip = db_connect_postgres("localhost", 5432, "testdb", "user", "pass")
assert_true(pg_conn > 0)

test_start("MySQL Connection")

sus mysql_conn drip = db_connect_mysql("localhost", 3306, "testdb", "user", "pass")
assert_true(mysql_conn > 0)

test_start("Simple Query Test")

assert_eq_int(db_execute("CREATE TABLE test (id INT)"), 0)

sus results []tea = db_query("SELECT * FROM users")
assert_true(len(results) > 0)

test_start("Disconnection Test")

assert_true(db_disconnect(pg_conn))
assert_true(db_disconnect(mysql_conn))

print_test_summary()

vibez.spill("✅ Enhanced database drivers working!")
