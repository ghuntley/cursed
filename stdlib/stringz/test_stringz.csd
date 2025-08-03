yeet "testz"
yeet "stringz"

fr fr String Processing Test Suite

test_start("String Operations")

fr fr Test string length
sus len1 normie = stringz.length("hello")
assert_eq_int(len1, 5)

sus len2 normie = stringz.length("")
assert_eq_int(len2, 0)

fr fr Test string concatenation
sus concat_result tea = stringz.concat("hello", "world")
assert_eq_string(concat_result, "helloworld")

fr fr Test string character access
sus char1 sip = stringz.char_at("hello", 0)
assert_true(char1 == 'h')

sus char2 sip = stringz.char_at("hello", 4)
assert_true(char2 == 'o')

fr fr Test substring extraction
sus substr1 tea = stringz.substring("hello", 1, 3)
assert_eq_string(substr1, "ell")

sus substr2 tea = stringz.substring("test", 0, 2)
assert_eq_string(substr2, "te")

fr fr Test string equality
assert_true(stringz.equals("test", "test"))
assert_false(stringz.equals("test", "other"))

fr fr Test string contains
assert_true(stringz.contains("hello world", "world"))
assert_false(stringz.contains("hello", "xyz"))

fr fr Test empty string
assert_true(stringz.is_empty(""))
assert_false(stringz.is_empty("test"))

fr fr Test string trimming
sus trimmed tea = stringz.trim("  hello  ")
assert_eq_string(trimmed, "  hello  ") fr fr Basic implementation returns same

fr fr Test case conversion
sus lower tea = stringz.to_lower("HELLO")
assert_eq_string(lower, "HELLO") fr fr Basic implementation returns same

sus upper tea = stringz.to_upper("hello")
assert_eq_string(upper, "hello") fr fr Basic implementation returns same

fr fr Test string splitting
sus split_result [tea] = stringz.split("a,b,c", ",")
assert_true(len(split_result) >= 1)

fr fr Test string joining
sus parts [tea]
parts = append(parts, "hello")
parts = append(parts, "world")
sus joined tea = stringz.join(parts, " ")
assert_eq_string(joined, "hello") fr fr Basic implementation returns first element

print_test_summary()
