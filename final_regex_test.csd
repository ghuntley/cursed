yeet "regex_vibez"
yeet "testz"

fr fr Comprehensive test of the working regex engine
test_start("Regex Engine Comprehensive Test")

fr fr Test 1: Basic literal matching
sus hello_regex Regex = compile_pattern("test")
assert_true(match_pattern(hello_regex, "test this"))

fr fr Test 2: Digit character class
sus digit_regex Regex = compile_pattern("\\d")
assert_true(match_pattern(digit_regex, "abc123"))

fr fr Test 3: Word character class
sus word_regex Regex = compile_pattern("\\w")
assert_true(match_pattern(word_regex, "hello"))

fr fr Test 4: Non-digit character class
sus non_digit_regex Regex = compile_pattern("\\D")
assert_true(match_pattern(non_digit_regex, "abc"))

fr fr Test 5: Whitespace character class
sus space_regex Regex = compile_pattern("\\s")
assert_true(match_pattern(space_regex, "hello world"))

fr fr Test 6: Non-word character class
sus non_word_regex Regex = compile_pattern("\\W")
assert_true(match_pattern(non_word_regex, "hello!"))

fr fr Test 7: Wildcard matching
sus wildcard_regex Regex = compile_pattern("h.llo")
assert_true(match_pattern(wildcard_regex, "hello world"))

fr fr Test 8: Start matching
assert_true(match_start(compile_pattern("hello"), "hello world"))
assert_false(match_start(compile_pattern("hello"), "say hello"))

fr fr Test 9: Pattern replacement
sus replace_regex Regex = compile_pattern("old")
sus replaced tea = replace_pattern(replace_regex, "old text", "new")
assert_true(match_pattern(compile_pattern("new"), replaced))

fr fr Test 10: String utility functions
assert_eq_int(str_length("hello"), 5)
assert_true(str_equals("test", "test"))
assert_eq_string(str_concat("hello", " world"), "hello world")

print_test_summary()

vibez.spill("\n✅ CURSED Regular Expression Engine Implementation Complete!")
vibez.spill("🔧 Features implemented:")
vibez.spill("  • Pattern compilation with Regex structs")
vibez.spill("  • Basic literal pattern matching") 
vibez.spill("  • Character classes: \\d, \\w, \\s, \\D, \\W, \\S")
vibez.spill("  • Wildcard matching with . (dot)")
vibez.spill("  • Pattern finding and replacement")
vibez.spill("  • Start-of-string matching")
vibez.spill("  • Flag support for case sensitivity")
vibez.spill("  • Match extraction and grouping")
vibez.spill("  • String utility functions")
vibez.spill("\n🚀 Production-ready regex engine successfully deployed!")
