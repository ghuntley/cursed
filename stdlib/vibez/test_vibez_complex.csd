yeet "testz"
yeet "vibez/mod_complex"

test_start("Vibez Complex Module Tests")

// Test basic output functions
assert_true(spill("Hello"))
assert_true(spillln("Hello with newline"))
assert_true(spill_error("Test error"))
assert_true(spill_warning("Test warning"))
assert_true(spill_debug("Test debug"))

// Test formatting functions
sus formatted tea = format_string("Hello %s, number: %d", ["World", "42"])
assert_true(string_length(formatted) > 0)

// Test spillf function
assert_true(spillf("Format test: %s", ["value"]))
assert_true(spillfln("Format test with newline: %s", ["value"]))

// Test color functions
assert_true(set_color("red"))
assert_true(set_color("green"))
assert_true(set_color("reset"))
assert_true(spill_colored("Colored text", "blue"))

// Test utility functions
assert_true(clear_screen())
assert_true(format_bool(based) == "true" || format_bool(based) == "false")

// Test multiple value printing
assert_true(spill_values(["one", "two", "three"]))
assert_true(spill_values_ln(["one", "two", "three"]))
assert_true(spill_sep(",", ["a", "b", "c"]))

// Test helper functions
assert_true(string_length("test") >= 0)
assert_true(string_char_at("test", 0) != "")

print_test_summary()
