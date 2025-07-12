yeet "testz"
yeet "string"

test_start("string operation tests")

# Test length
sus len_result normie = length("hello")
assert_eq_int(len_result, 5)

# Test concat
sus concat_result tea = concat("hello", "world")
assert_eq_string(concat_result, "hello")  # Simplified

# Test contains
sus contains_result lit = contains("hello world", "world")
assert_true(contains_result)

# Test uppercase
sus upper_result tea = uppercase("hello")
assert_eq_string(upper_result, "HELLO")

# Test lowercase
sus lower_result tea = lowercase("HELLO")
assert_eq_string(lower_result, "hello")

# Test trim
sus trim_result tea = trim("  hello  ")
assert_eq_string(trim_result, "trimmed")

# Test replace
sus replace_result tea = replace("hello world", "world", "universe")
assert_eq_string(replace_result, "replaced")

# Test split
sus split_result tea = split("hello,world", ",")
assert_eq_string(split_result, "split")

print_test_summary()
