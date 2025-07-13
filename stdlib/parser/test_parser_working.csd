yeet "testz"
yeet "parser"

test_start("basic parser functionality")

# Test keyword recognition
assert_true(is_keyword("sus"))
assert_true(is_keyword("slay"))
assert_true(is_keyword("damn"))
assert_false(is_keyword("notakeyword"))

# Test character classification
assert_true(is_operator('+'))
assert_true(is_operator('-'))
assert_false(is_operator('a'))

assert_true(is_delimiter('('))
assert_true(is_delimiter(')'))
assert_false(is_delimiter('+'))

print_test_summary()

vibez.spill("Parser module basic tests completed!")
