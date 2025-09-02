yeet "testz"
yeet "regex_vibez"

fr fr Test basic pattern compilation
test_start("Pattern compilation")
sus regex Regex = compile_pattern("hello")
assert_eq_string(regex.pattern, "hello")
print_test_summary()

fr fr Test basic literal matching
test_start("Basic literal matching")
sus hello_regex Regex = compile_pattern("hello")
assert_true(match_pattern(hello_regex, "hello world"))
assert_true(match_pattern(hello_regex, "say hello there"))
assert_false(match_pattern(hello_regex, "goodbye world"))
assert_false(match_pattern(hello_regex, "HELLO"))  fr fr Case sensitive
print_test_summary()

fr fr Test dot wildcard matching
test_start("Dot wildcard matching")
sus wildcard_regex Regex = compile_pattern("h.llo")
assert_true(match_pattern(wildcard_regex, "hello"))
assert_true(match_pattern(wildcard_regex, "hallo"))
assert_true(match_pattern(wildcard_regex, "hillo"))
assert_false(match_pattern(wildcard_regex, "hllo"))   fr fr Missing character
assert_false(match_pattern(wildcard_regex, "heello")) fr fr Extra character
print_test_summary()

fr fr Test digit character class
test_start("Digit character class \\d")
sus digit_regex Regex = compile_pattern("\\d")
assert_true(match_pattern(digit_regex, "123"))
assert_true(match_pattern(digit_regex, "a1b"))
assert_true(match_pattern(digit_regex, "5"))
assert_false(match_pattern(digit_regex, "abc"))
assert_false(match_pattern(digit_regex, ""))
print_test_summary()

fr fr Test non-digit character class
test_start("Non-digit character class \\D")
sus non_digit_regex Regex = compile_pattern("\\D")
assert_true(match_pattern(non_digit_regex, "abc"))
assert_true(match_pattern(non_digit_regex, "a123"))
assert_true(match_pattern(non_digit_regex, "!@#"))
assert_false(match_pattern(non_digit_regex, "123"))
print_test_summary()

fr fr Test word character class
test_start("Word character class \\w")
sus word_regex Regex = compile_pattern("\\w")
assert_true(match_pattern(word_regex, "hello"))
assert_true(match_pattern(word_regex, "test_123"))
assert_true(match_pattern(word_regex, "A"))
assert_true(match_pattern(word_regex, "_var"))
assert_false(match_pattern(word_regex, "!@#"))
print_test_summary()

fr fr Test non-word character class
test_start("Non-word character class \\W")
sus non_word_regex Regex = compile_pattern("\\W")
assert_true(match_pattern(non_word_regex, "!@#"))
assert_true(match_pattern(non_word_regex, "hello!"))
assert_true(match_pattern(non_word_regex, " "))
assert_false(match_pattern(non_word_regex, "hello"))
assert_false(match_pattern(non_word_regex, "123"))
print_test_summary()

fr fr Test whitespace character class
test_start("Whitespace character class \\s")
sus space_regex Regex = compile_pattern("\\s")
assert_true(match_pattern(space_regex, "hello world"))
assert_true(match_pattern(space_regex, "	tab"))  fr fr Tab character
assert_true(match_pattern(space_regex, " "))
assert_false(match_pattern(space_regex, "hello"))
print_test_summary()

fr fr Test non-whitespace character class
test_start("Non-whitespace character class \\S")
sus non_space_regex Regex = compile_pattern("\\S")
assert_true(match_pattern(non_space_regex, "hello"))
assert_true(match_pattern(non_space_regex, "a b"))
assert_false(match_pattern(non_space_regex, "   "))
assert_false(match_pattern(non_space_regex, "	"))
print_test_summary()

fr fr Test pattern finding
test_start("Find pattern matches")
sus find_regex Regex = compile_pattern("test")
sus matches Match[value] = find_matches(find_regex, "test this test case")
assert_true(len(matches) >= 1)  fr fr Should find at least one match
print_test_summary()

fr fr Test pattern replacement
test_start("Pattern replacement")
sus replace_regex Regex = compile_pattern("old")
sus result tea = replace_pattern(replace_regex, "old text", "new")
assert_true(match_pattern(compile_pattern("new"), result))
print_test_summary()

fr fr Test complex patterns
test_start("Complex pattern combinations")
sus complex_regex Regex = compile_pattern("h.ll.")
assert_true(match_pattern(complex_regex, "hello"))
assert_true(match_pattern(complex_regex, "hallo"))
assert_false(match_pattern(complex_regex, "hll"))
print_test_summary()

fr fr Test start matching
test_start("Start of string matching")
sus start_regex Regex = compile_pattern("hello")
assert_true(match_start(start_regex, "hello world"))
assert_false(match_start(start_regex, "say hello"))
print_test_summary()

fr fr Test escaped characters
test_start("Escaped character matching")
sus escaped_regex Regex = compile_pattern("\\.")
assert_true(match_pattern(escaped_regex, "file.txt"))
assert_false(match_pattern(escaped_regex, "filetxt"))
print_test_summary()

fr fr Test multiple character classes in sequence
test_start("Sequential character classes")
sus seq_regex Regex = compile_pattern("\\d\\w")
assert_true(match_pattern(seq_regex, "1a"))
assert_true(match_pattern(seq_regex, "5_"))
assert_false(match_pattern(seq_regex, "ab"))
assert_false(match_pattern(seq_regex, "12"))
print_test_summary()

fr fr Test mixed literal and class patterns
test_start("Mixed literal and character class patterns")
sus mixed_regex Regex = compile_pattern("test\\d")
assert_true(match_pattern(mixed_regex, "test1"))
assert_true(match_pattern(mixed_regex, "test9"))
assert_false(match_pattern(mixed_regex, "test"))
assert_false(match_pattern(mixed_regex, "testa"))
print_test_summary()

fr fr Test case sensitivity with flags
test_start("Case sensitivity with flags")
sus case_regex Regex = compile_pattern_with_flags("HELLO", based, cringe)
assert_true(match_pattern(case_regex, "hello"))  fr fr Should match with case insensitive flag
sus case_sensitive_regex Regex = compile_pattern_with_flags("HELLO", cringe, cringe)
assert_false(match_pattern(case_sensitive_regex, "hello"))  fr fr Should not match case sensitive
print_test_summary()

fr fr Test empty pattern
test_start("Empty pattern handling")
sus empty_regex Regex = compile_pattern("")
assert_true(match_pattern(empty_regex, "anything"))  fr fr Empty pattern should match
print_test_summary()

fr fr Test single character patterns
test_start("Single character patterns")
sus single_regex Regex = compile_pattern("a")
assert_true(match_pattern(single_regex, "a"))
assert_true(match_pattern(single_regex, "cat"))
assert_false(match_pattern(single_regex, "dog"))
print_test_summary()

print_test_summary()
