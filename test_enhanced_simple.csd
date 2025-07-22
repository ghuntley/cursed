# Simple test for enhanced dropz and vibez modules
yeet "testz"
yeet "vibez"
yeet "dropz"

test_start("Enhanced modules test")

# Test enhanced vibez formatting
vibez.spill("Testing enhanced vibez formatting...")
sus result1 tea = vibez.spillstr("Hello %s", "World")
vibez.spill("Formatted: " + result1)
assert_eq_string(result1, "Hello World")

# Test enhanced dropz string operations
vibez.spill("Testing enhanced dropz string operations...")
sus length1 normie = dropz.string_length("Hello")
vibez.spill("Length result: " + vibez.format_number(length1))
assert_true(length1 > 0)

sus contains1 lit = dropz.string_contains("test.txt", ".txt")
vibez.spill("Contains result: " + vibez.format_bool(contains1))
assert_true(contains1)

# Test enhanced file operations
vibez.spill("Testing enhanced file operations...")
sus data1, err1 := dropz.read_file("test.txt")
vibez.spill("Read file error: " + err1)
assert_eq_string(err1, "")
assert_true(len(data1) > 0)

vibez.spill("✅ All enhanced module tests passed!")

print_test_summary()
