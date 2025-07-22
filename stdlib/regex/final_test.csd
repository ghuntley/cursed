fr fr Final working regex test

yeet "testz"

fr fr Test basic pattern matching functions
test_start("Regex Basic Functions")

fr fr Test exact string matching
assert_true(match_pattern("hello", "hello"))
assert_false(match_pattern("hello", "world"))
assert_true(match_pattern("", ""))

fr fr Test wildcard matching
assert_true(match_wildcard("hello", "*"))
assert_true(match_wildcard("hello", "h?llo"))

fr fr Test pattern replacement
sus result tea = replace_pattern("hello world", "hello", "hi")
assert_eq_string(result, "hi world")

fr fr Test regex compilation
sus engine RegexEngine = regex_compile_pcre("test", 0)
assert_eq_string(engine.pattern, "test")
assert_false(engine.unicode_enabled)

fr fr Test with Unicode flag
sus unicode_engine RegexEngine = regex_compile_pcre("test", 32)
assert_true(unicode_engine.unicode_enabled)

fr fr Test split functionality
sus parts [tea] = split_by_pattern("a,b,c", ",")
assert_eq_int(len(parts), 3)

fr fr Test Unicode functions
assert_true(is_unicode_letter(65)) fr fr 'A'
assert_true(is_unicode_number(48)) fr fr '0'
assert_true(is_unicode_separator(32)) fr fr Space

fr fr Test pattern validation
assert_true(regex_validate_input("hello", 100))
assert_false(regex_validate_input("hello", 3))

fr fr Test catastrophic backtracking detection
assert_true(has_catastrophic_backtracking("(.*)*"))
assert_false(has_catastrophic_backtracking("hello"))

fr fr Test helper functions
assert_eq_int(count_occurrences("hello", "l"), 2)
assert_eq_string(int_to_string(42), "42")

print_test_summary()
