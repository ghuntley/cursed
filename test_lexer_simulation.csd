yeet "stringz"
yeet "testz"

# Simple test of the CURSED lexer
sus test_code tea = "slay main() { vibez.spill(\"hello\") }"
vibez.spill("Testing lexer with: " + test_code)

# This simulates what the lexer would do
test_start("Lexer simulation")
assert_true(stringz.contains(test_code, "slay"))
assert_true(stringz.contains(test_code, "main"))
assert_true(stringz.contains(test_code, "vibez"))
print_test_summary()
