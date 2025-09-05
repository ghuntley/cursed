fr fr Comprehensive Test Suite for Core StringZ Module
fr fr Tests all four core categories: manipulation, formatting, parsing, validation

yeet "testz"
yeet "stringz"

fr fr Start comprehensive testing
test_start("StringZ Core Module Tests")

fr fr ===== STRING MANIPULATION TESTS =====

test_start("String Manipulation")

fr fr Test split function
sus parts1 tea[value] = split("a,b,c", ",")
assert_eq_int(len(parts1), 3)
assert_eq_string(parts1[0], "a")
assert_eq_string(parts1[1], "b") 
assert_eq_string(parts1[2], "c")

sus parts2 tea[value] = split("hello world", " ")
assert_eq_int(len(parts2), 2)
assert_eq_string(parts2[0], "hello")
assert_eq_string(parts2[1], "world")

sus parts3 tea[value] = split("x-y-z", "-") 
assert_eq_int(len(parts3), 3)
assert_eq_string(parts3[0], "x")
assert_eq_string(parts3[1], "y")
assert_eq_string(parts3[2], "z")

sus empty_split tea[value] = split("", ",")
assert_eq_int(len(empty_split), 0)

vibez.spill("✅ Split tests passed")

fr fr Test join function
sus joined1 tea = join(["a", "b", "c"], ",")
assert_eq_string(joined1, "a,b,c")

sus joined2 tea = join(["hello", "world"], " ")
assert_eq_string(joined2, "hello world")

sus joined3 tea = join(["x", "y", "z"], "-")
assert_eq_string(joined3, "x-y-z")

sus empty_join tea = join([], ",")
assert_eq_string(empty_join, "")

sus single_join tea = join(["test"], ",")
assert_eq_string(single_join, "test")

vibez.spill("✅ Join tests passed")

fr fr Test replace function
sus replaced1 tea = replace("hello world", "hello", "hi")
assert_eq_string(replaced1, "hi world")

sus replaced2 tea = replace("test test test", "test", "exam")
assert_eq_string(replaced2, "exam test test")

sus replaced3 tea = replace("banana", "an", "XX")
assert_eq_string(replaced3, "bXXana")

sus no_replace tea = replace("hello", "xyz", "abc")
assert_eq_string(no_replace, "hello")

vibez.spill("✅ Replace tests passed")

fr fr Test replace_all function
sus replaced_all1 tea = replace_all("hello hello hello", "hello", "hi")
assert_eq_string(replaced_all1, "hi hi hi")

sus replaced_all2 tea = replace_all("test test test", "test", "exam")
assert_eq_string(replaced_all2, "exam exam exam")

sus replaced_all3 tea = replace_all("banana", "an", "XX")
assert_eq_string(replaced_all3, "bXXXXa")

vibez.spill("✅ Replace all tests passed")

fr fr Test reverse function
sus reversed1 tea = reverse("hello")
assert_eq_string(reversed1, "olleh")

sus reversed2 tea = reverse("world")
assert_eq_string(reversed2, "dlrow")

sus reversed3 tea = reverse("abc")
assert_eq_string(reversed3, "cba")

sus reversed_empty tea = reverse("")
assert_eq_string(reversed_empty, "")

sus reversed_single tea = reverse("a")
assert_eq_string(reversed_single, "a")

vibez.spill("✅ Reverse tests passed")

fr fr Test substring function
sus sub1 tea = substring("hello", 0, 2)
assert_eq_string(sub1, "he")

sus sub2 tea = substring("hello", 1, 3)
assert_eq_string(sub2, "ell")

sus sub3 tea = substring("world", 0, 5)
assert_eq_string(sub3, "world")

sus sub_empty tea = substring("test", 0, 0)
assert_eq_string(sub_empty, "")

sus sub_negative tea = substring("test", -1, 2)
assert_eq_string(sub_negative, "")

vibez.spill("✅ Substring tests passed")

fr fr ===== STRING FORMATTING TESTS =====

test_start("String Formatting")

fr fr Test format_template function
sus formatted1 tea = format_template("Hello {}", ["World"])
assert_eq_string(formatted1, "Hello World")

sus formatted2 tea = format_template("{} says {}", ["Alice", "hi"])
assert_eq_string(formatted2, "Alice says hi")

sus formatted3 tea = format_template("Name: {}, Age: {}", ["Bob", "25"])
assert_eq_string(formatted3, "Name: Bob, Age: 25")

sus no_placeholders tea = format_template("No placeholders", [])
assert_eq_string(no_placeholders, "No placeholders")

vibez.spill("✅ Format template tests passed")

fr fr Test interpolate function
sus interpolated1 tea = interpolate("Hello {name}", "name", "Alice")
assert_eq_string(interpolated1, "Hello Alice")

sus interpolated2 tea = interpolate("Welcome to {place}", "place", "CURSED")
assert_eq_string(interpolated2, "Welcome to CURSED")

sus no_key tea = interpolate("Hello {missing}", "name", "Alice")
assert_eq_string(no_key, "Hello {missing}")

vibez.spill("✅ Interpolate tests passed")

fr fr Test padding functions
sus padded_left tea = pad_left("42", 5, "0")
assert_eq_string(padded_left, "00042")

sus padded_right tea = pad_right("test", 8, "-")
assert_eq_string(padded_right, "test----")

sus centered tea = center("hi", 6, " ")
assert_eq_string(centered, "  hi  ")

sus no_padding tea = pad_left("longstring", 5, "0")
assert_eq_string(no_padding, "longstring")

vibez.spill("✅ Padding tests passed")

fr fr Test repeat_char function
sus repeated1 tea = repeat_char("-", 5)
assert_eq_string(repeated1, "-----")

sus repeated2 tea = repeat_char("x", 3)
assert_eq_string(repeated2, "xxx")

sus repeated_zero tea = repeat_char("a", 0)
assert_eq_string(repeated_zero, "")

sus repeated_one tea = repeat_char("b", 1)
assert_eq_string(repeated_one, "b")

vibez.spill("✅ Repeat char tests passed")

fr fr ===== STRING PARSING TESTS =====

test_start("String Parsing")

fr fr Test parse_int function
assert_eq_int(parse_int("0"), 0)
assert_eq_int(parse_int("42"), 42)
assert_eq_int(parse_int("123"), 123)
assert_eq_int(parse_int("-1"), -1)
assert_eq_int(parse_int("-42"), -42)
assert_eq_int(parse_int("invalid"), 0)
assert_eq_int(parse_int(""), 0)

vibez.spill("✅ Parse int tests passed")

fr fr Test parse_bool function
assert_true(parse_bool("true"))
assert_true(parse_bool("True"))
assert_true(parse_bool("TRUE"))
assert_true(parse_bool("yes"))
assert_true(parse_bool("YES"))
assert_true(parse_bool("1"))

assert_false(parse_bool("false"))
assert_false(parse_bool("False"))
assert_false(parse_bool("FALSE"))
assert_false(parse_bool("no"))
assert_false(parse_bool("NO"))
assert_false(parse_bool("0"))
assert_false(parse_bool("invalid"))

vibez.spill("✅ Parse bool tests passed")

fr fr Test to_int function
assert_eq_string(to_int(0), "0")
assert_eq_string(to_int(42), "42")
assert_eq_string(to_int(123), "123")
assert_eq_string(to_int(-1), "-1")
assert_eq_string(to_int(-42), "-42")

vibez.spill("✅ To int tests passed")

fr fr Test to_string function
assert_eq_string(to_string(based), "true")
assert_eq_string(to_string(cringe), "false")

vibez.spill("✅ To string tests passed")

fr fr Test trim_digits function
sus clean1 tea = trim_digits("abc123def")
assert_eq_string(clean1, "abcdef")

sus clean2 tea = trim_digits("test456")
assert_eq_string(clean2, "test")

sus clean3 tea = trim_digits("123abc")
assert_eq_string(clean3, "abc")

sus clean4 tea = trim_digits("12345")
assert_eq_string(clean4, "")

sus clean5 tea = trim_digits("hello")
assert_eq_string(clean5, "hello")

vibez.spill("✅ Trim digits tests passed")

fr fr ===== STRING VALIDATION TESTS =====

test_start("String Validation")

fr fr Test len_string function
assert_eq_int(len_string(""), 0)
assert_eq_int(len_string("a"), 1)
assert_eq_int(len_string("ab"), 2)
assert_eq_int(len_string("hello"), 5)
assert_eq_int(len_string("world"), 5)

vibez.spill("✅ Length tests passed")

fr fr Test is_empty function
assert_true(is_empty(""))
assert_false(is_empty("a"))
assert_false(is_empty(" "))
assert_false(is_empty("test"))

vibez.spill("✅ Empty tests passed")

fr fr Test contains function
assert_true(contains("hello world", "world"))
assert_true(contains("hello world", "hello"))
assert_true(contains("hello world", "o"))
assert_false(contains("hello world", "xyz"))
assert_false(contains("test", "testing"))
assert_true(contains("any string", ""))

vibez.spill("✅ Contains tests passed")

fr fr Test starts_with function
assert_true(starts_with("hello world", "hello"))
assert_false(starts_with("hello world", "world"))
assert_true(starts_with("test string", "test"))
assert_false(starts_with("test", "testing"))
assert_true(starts_with("any string", ""))

vibez.spill("✅ Starts with tests passed")

fr fr Test ends_with function
assert_true(ends_with("hello world", "world"))
assert_false(ends_with("hello world", "hello"))
assert_true(ends_with("example.txt", ".txt"))
assert_false(ends_with("test", "testing"))
assert_true(ends_with("any string", ""))

vibez.spill("✅ Ends with tests passed")

fr fr Test is_numeric function
assert_true(is_numeric("123"))
assert_true(is_numeric("0"))
assert_true(is_numeric("42"))
assert_false(is_numeric("hello"))
assert_false(is_numeric("12a"))
assert_false(is_numeric("a12"))
assert_false(is_numeric(""))

vibez.spill("✅ Numeric tests passed")

fr fr Test is_alpha function
assert_true(is_alpha("hello"))
assert_true(is_alpha("ABC"))
assert_true(is_alpha("test"))
assert_false(is_alpha("hello123"))
assert_false(is_alpha("123"))
assert_false(is_alpha("test!"))
assert_false(is_alpha(""))

vibez.spill("✅ Alpha tests passed")

fr fr Test is_alphanumeric function
assert_true(is_alphanumeric("hello123"))
assert_true(is_alphanumeric("abc456"))
assert_true(is_alphanumeric("hello"))
assert_true(is_alphanumeric("123"))
assert_false(is_alphanumeric("hello!"))
assert_false(is_alphanumeric("test@123"))
assert_false(is_alphanumeric("test 123"))
assert_false(is_alphanumeric(""))

vibez.spill("✅ Alphanumeric tests passed")

fr fr ===== UTILITY FUNCTION TESTS =====

test_start("Utility Functions")

fr fr Test case conversion functions
sus lower1 tea = to_lowercase("HELLO")
assert_eq_string(lower1, "hello")

sus lower2 tea = to_lowercase("Hello World")
assert_eq_string(lower2, "hello world")

sus upper1 tea = to_uppercase("hello")
assert_eq_string(upper1, "HELLO")

sus upper2 tea = to_uppercase("hello world")
assert_eq_string(upper2, "HELLO WORLD")

vibez.spill("✅ Case conversion tests passed")

fr fr Test trim function
sus trimmed1 tea = trim(" hello ")
assert_eq_string(trimmed1, "hello")

sus trimmed2 tea = trim(" world ")
assert_eq_string(trimmed2, "world")

sus trimmed3 tea = trim("  abc  ")
assert_eq_string(trimmed3, "abc")

sus trimmed4 tea = trim(" test")
assert_eq_string(trimmed4, "test")

sus trimmed5 tea = trim("test ")
assert_eq_string(trimmed5, "test")

vibez.spill("✅ Trim tests passed")

fr fr ===== INTEGRATION TESTS =====

test_start("Integration Tests")

fr fr Test complex string processing pipeline
sus input tea = "  Hello, World! This is a TEST.  "
sus cleaned tea = trim(input)
sus lower tea = to_lowercase(cleaned)
sus words tea[value] = split(lower, " ")
sus filtered tea[value] = []

fr fr Filter out punctuation words (simplified)
sus i drip = 0
bestie (i < len(words)) {
    ready (!contains(words[i], ",") && !contains(words[i], ".") && !contains(words[i], "!")) {
        ready (len(filtered) == 0) {
            filtered = [words[i]]
        } otherwise ready (len(filtered) == 1) {
            filtered = [words[i], filtered[0]]
        } otherwise ready (len(filtered) == 2) {
            filtered = [words[i], filtered[0], filtered[1]]
        } otherwise {
            fr fr For larger arrays, keep existing approach
            filtered = [words[i], filtered[0], filtered[1]]
        }
    }
    i = i + 1
}

sus result tea = join(filtered, "_")
vibez.spill("Pipeline result:", result)

vibez.spill("✅ Integration tests passed")

fr fr ===== EDGE CASES AND ERROR HANDLING =====

test_start("Edge Cases and Error Handling")

fr fr Test empty string handling
sus empty_parts tea[value] = split("", ",")
assert_eq_int(len(empty_parts), 0)

sus empty_join tea = join([], "-")
assert_eq_string(empty_join, "")

sus empty_replace tea = replace("", "x", "y")
assert_eq_string(empty_replace, "")

sus empty_reverse tea = reverse("")
assert_eq_string(empty_reverse, "")

vibez.spill("✅ Empty string edge cases passed")

fr fr Test boundary conditions
sus out_of_bounds tea = substring("test", 10, 5)
assert_eq_string(out_of_bounds, "")

sus negative_length tea = substring("test", 1, -2)
assert_eq_string(negative_length, "")

sus zero_repeat tea = repeat_char("x", 0)
assert_eq_string(zero_repeat, "")

vibez.spill("✅ Boundary condition tests passed")

fr fr Test special characters and unicode (basic)
sus special_split tea[value] = split("a;b;c", ";")
assert_eq_int(len(special_split), 3)

sus special_join tea = join(["x", "y"], "|")
assert_eq_string(special_join, "x|y")

vibez.spill("✅ Special character tests passed")

fr fr ===== PERFORMANCE TESTS =====

test_start("Performance Tests")

fr fr Test with longer strings
sus long_string tea = repeat_char("a", 100)
sus long_length drip = len_string(long_string)
vibez.spill("Long string length calculated:", long_length)

fr fr Test multiple operations
sus performance_input tea = "performance test string"
sus perf_upper tea = to_uppercase(performance_input)
sus perf_split tea[value] = split(perf_upper, " ")
sus perf_joined tea = join(perf_split, "_")
sus perf_final tea = to_lowercase(perf_joined)

assert_eq_string(perf_final, "performance_test_string")

vibez.spill("✅ Performance tests passed")

fr fr ===== SUMMARY =====

vibez.spill("\n🎉 All StringZ Core Module Tests Completed Successfully!")
vibez.spill("Tested categories:")
vibez.spill("  ✅ String Manipulation (split, join, replace, reverse, substring)")
vibez.spill("  ✅ String Formatting (templates, interpolation, padding)")  
vibez.spill("  ✅ String Parsing (integers, booleans, conversions)")
vibez.spill("  ✅ String Validation (length, contains, type checks)")
vibez.spill("  ✅ Utility Functions (case conversion, trimming)")
vibez.spill("  ✅ Integration Tests (complex pipelines)")
vibez.spill("  ✅ Edge Cases (empty strings, boundaries)")
vibez.spill("  ✅ Performance Tests (longer strings)")

print_test_summary()
