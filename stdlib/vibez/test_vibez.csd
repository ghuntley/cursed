yeet "testz"
yeet "vibez"

fr fr Core I/O Operations Test Suite

test_start("Basic Output Functions")

fr fr Test basic spill function
vibez.spill("Testing basic spill")
assert_true(based)

fr fr Test spillf with string formatting
vibez.spillf("Hello %s", "World")
assert_true(based)

fr fr Test spillstr function
sus formatted tea = vibez.spillstr("Value: %d", "42")
assert_eq_string(formatted, "Value: 42")

fr fr Test spill_values function
vibez.spill_values("Testing", "multiple", "values")
assert_true(based)

fr fr Test enhanced formatting
sus result1 tea = vibez.format_string_enhanced("Hello %s", "World")
assert_eq_string(result1, "Hello World")

sus result2 tea = vibez.format_string_enhanced("User: %s, ID: %d", "Alice", "123")
assert_eq_string(result2, "User: Alice, ID: 123")

fr fr Test number formatting
sus number_str tea = vibez.format_number_enhanced("42")
assert_eq_string(number_str, "42")

fr fr Test boolean formatting
sus bool_str tea = vibez.format_bool(based)
assert_eq_string(bool_str, "true")

fr fr Test error and warning messages
vibez.spill_error("Test error message")
vibez.spill_warning("Test warning message")
vibez.spill_debug("Test debug message")
assert_true(based)

fr fr Test colored output
vibez.spill_colored("Red text", "red")
vibez.spill_colored("Green text", "green")
vibez.spill_colored("Blue text", "blue")
assert_true(based)

print_test_summary()
