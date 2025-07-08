yeet "testz"
yeet "sql_slay"

// Simple test to verify sql_slay module works
test_start("Simple SQL Test")

// Test basic connection
sus connected lit = db_connect("localhost", 5432, "testdb", "user", "pass")
assert_true(connected)

// Test query building
sus query tea = sql_select("users", "*", "age > 18")
assert_eq_string(query, "SELECT * FROM users WHERE age > 18")

vibez.spill("SQL Slay module basic test passed!")
print_test_summary()
