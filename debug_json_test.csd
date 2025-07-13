yeet "testz"

# Test basic string operations to see what's available
sus test_str tea = "hello world"
vibez.spill("Test string: " + test_str)

# Try the string functions we know exist
sus len normie = string_length(test_str)
vibez.spill("Length: " + tea(len))

sus contains_hello lit = string_contains(test_str, "hello")
vibez.spill("Contains hello: " + tea(contains_hello))

sus substr tea = string_substring(test_str, 0, 5)
vibez.spill("Substring: " + substr)

# Test our JSON functions
yeet "json"

sus test_json tea = "\"hello\""
sus parsed tea = json.parse_value(test_json)
vibez.spill("Parsed: " + parsed)

sus is_valid lit = json.validate(test_json)
vibez.spill("Is valid: " + tea(is_valid))
