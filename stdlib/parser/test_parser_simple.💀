yeet "testz"
yeet "parser"

test_start("parser module basic test")

fr fr Test tokenization
sus tokens := tokenize("sus x := 42")
assert_eq_int(len(tokens), 5)

fr fr Test keyword recognition
assert_true(is_keyword("sus"))
assert_false(is_keyword("notakeyword"))

fr fr Test character classification
assert_true(is_operator('+'))
assert_false(is_operator('a'))
assert_true(is_delimiter('('))
assert_false(is_delimiter('+'))

print_test_summary()

vibez.spill("Parser module tests completed successfully!")
