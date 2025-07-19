# Test basic stdlib functionality with simplified modules
yeet "testz"
yeet "vibez"
yeet "stringz"

# Test basic testz functions
test_start("basic stdlib test")

# Test vibez functions
vibez.spill("Testing vibez.spill")
vibez.spill_error("Testing error message")

# Test stringz functions
sus test_string tea = "hello"
sus str_length normie = stringz.length(test_string)
assert_eq_int(str_length, 5)

sus concat_result tea = stringz.concat("hello", " world")
vibez.spill("Concatenated: " + concat_result)

assert_true(based)
assert_false(cap)

print_test_summary()
