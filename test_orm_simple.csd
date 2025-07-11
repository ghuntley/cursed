yeet "testz"

// Simple ORM test
test_start("Simple ORM Test")

// Test basic variables
sus user_id normie = 1
sus user_name tea = "John"
sus user_email tea = "john@test.com"

assert_eq_int(user_id, 1)
assert_eq_string(user_name, "John")
assert_eq_string(user_email, "john@test.com")

// Test boolean values
sus active lit = based
sus archived lit = cap

assert_true(active)
assert_false(archived)

vibez.spill("✅ Basic ORM test completed")

print_test_summary()
