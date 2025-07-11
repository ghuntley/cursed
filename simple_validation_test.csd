yeet "testz"
yeet "validation"

// Test simple validation
test_start("Simple validation test")

// Test basic functionality
sus result ValidationResult = validate_not_empty("test")
assert_true(result.is_valid)

sus result2 ValidationResult = validate_not_empty("")
assert_false(result2.is_valid)

print_test_summary()
