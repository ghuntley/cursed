yeet "testz"
yeet "string"

fr fr ========================================
fr fr CURSED String Library Test Suite
fr fr ========================================

slay test_string_length() {
    testz.test_start("String Length Functions")
    
    fr fr Test string length
    testz.assert_eq_int(string_len("hello"), 5)
    testz.assert_eq_int(string_len(""), 0)
    testz.assert_eq_int(string_len("CURSED"), 6)
    
    fr fr Test empty string check
    testz.assert_true(string_is_empty(""))
    testz.assert_false(string_is_empty("hello"))
}

slay test_string_case_conversion() {
    testz.test_start("String Case Conversion")
    
    fr fr Test uppercase conversion
    testz.assert_eq_string(string_to_upper("hello"), "HELLO")
    testz.assert_eq_string(string_to_upper("CURSED"), "CURSED")
    testz.assert_eq_string(string_to_upper("MiXeD"), "MIXED")
    
    fr fr Test lowercase conversion
    testz.assert_eq_string(string_to_lower("HELLO"), "hello")
    testz.assert_eq_string(string_to_lower("cursed"), "cursed")
    testz.assert_eq_string(string_to_lower("MiXeD"), "mixed")
    
    fr fr Test capitalize
    testz.assert_eq_string(string_capitalize("hello"), "Hello")
    testz.assert_eq_string(string_capitalize("HELLO"), "Hello")
    testz.assert_eq_string(string_capitalize(""), "")
}

slay test_string_trimming() {
    testz.test_start("String Trimming Functions")
    
    fr fr Test trim all whitespace
    testz.assert_eq_string(string_trim("  hello  "), "hello")
    testz.assert_eq_string(string_trim("\t\ntest\r\n"), "test")
    testz.assert_eq_string(string_trim("no-spaces"), "no-spaces")
    
    fr fr Test trim start
    testz.assert_eq_string(string_trim_start("  hello  "), "hello  ")
    testz.assert_eq_string(string_trim_start("\t\ntest"), "test")
    
    fr fr Test trim end
    testz.assert_eq_string(string_trim_end("  hello  "), "  hello")
    testz.assert_eq_string(string_trim_end("test\r\n"), "test")
}

slay test_string_search() {
    testz.test_start("String Search Functions")
    
    fr fr Test contains
    testz.assert_true(string_contains("hello world", "world"))
    testz.assert_true(string_contains("hello world", "hello"))
    testz.assert_false(string_contains("hello world", "xyz"))
    
    fr fr Test starts with
    testz.assert_true(string_starts_with("hello world", "hello"))
    testz.assert_true(string_starts_with("hello world", ""))
    testz.assert_false(string_starts_with("hello world", "world"))
    
    fr fr Test ends with
    testz.assert_true(string_ends_with("hello world", "world"))
    testz.assert_true(string_ends_with("hello world", ""))
    testz.assert_false(string_ends_with("hello world", "hello"))
}

slay test_string_indexing() {
    testz.test_start("String Indexing Functions")
    
    fr fr Test index of
    testz.assert_eq_int(string_index_of("hello world", "world"), 6)
    testz.assert_eq_int(string_index_of("hello world", "hello"), 0)
    testz.assert_eq_int(string_index_of("hello world", "xyz"), -1)
    
    fr fr Test last index of
    testz.assert_eq_int(string_last_index_of("hello hello", "hello"), 6)
    testz.assert_eq_int(string_last_index_of("hello hello", "xyz"), -1)
    
    fr fr Test count occurrences
    testz.assert_eq_int(string_count_occurrences("hello hello hello", "hello"), 3)
    testz.assert_eq_int(string_count_occurrences("hello world", "l"), 3)
    testz.assert_eq_int(string_count_occurrences("hello world", "xyz"), 0)
}

slay test_string_slicing() {
    testz.test_start("String Slicing Functions")
    
    fr fr Test slice
    testz.assert_eq_string(string_slice("hello world", 0, 5), "hello")
    testz.assert_eq_string(string_slice("hello world", 6, 11), "world")
    testz.assert_eq_string(string_slice("hello world", 2, 8), "llo wo")
    
    fr fr Test substring
    testz.assert_eq_string(string_substring("hello world", 0, 5), "hello")
    testz.assert_eq_string(string_substring("hello world", 6, 5), "world")
    testz.assert_eq_string(string_substring("hello world", 2, 6), "llo wo")
    
    fr fr Test char at
    testz.assert_eq_string(string_char_at("hello", 0), "h")
    testz.assert_eq_string(string_char_at("hello", 4), "o")
    testz.assert_eq_string(string_char_at("hello", 2), "l")
}

slay test_string_splitting() {
    testz.test_start("String Splitting Functions")
    
    fr fr Test split by delimiter
    sus parts [tea] = string_split("a,b,c", ",")
    testz.assert_eq_int(len(parts), 3)
    testz.assert_eq_string(parts[0], "a")
    testz.assert_eq_string(parts[1], "b")
    testz.assert_eq_string(parts[2], "c")
    
    fr fr Test split lines
    sus lines [tea] = string_split_lines("line1\nline2\nline3")
    testz.assert_eq_int(len(lines), 3)
    testz.assert_eq_string(lines[0], "line1")
    testz.assert_eq_string(lines[1], "line2")
    testz.assert_eq_string(lines[2], "line3")
    
    fr fr Test split whitespace
    sus words [tea] = string_split_whitespace("hello   world\t\ntest")
    testz.assert_eq_int(len(words), 3)
    testz.assert_eq_string(words[0], "hello")
    testz.assert_eq_string(words[1], "world")
    testz.assert_eq_string(words[2], "test")
}

slay test_string_replacement() {
    testz.test_start("String Replacement Functions")
    
    fr fr Test replace first occurrence
    testz.assert_eq_string(string_replace("hello hello", "hello", "hi"), "hi hello")
    testz.assert_eq_string(string_replace("hello world", "xyz", "abc"), "hello world")
    
    fr fr Test replace all occurrences
    testz.assert_eq_string(string_replace_all("hello hello", "hello", "hi"), "hi hi")
    testz.assert_eq_string(string_replace_all("hello world", "l", "x"), "hexxo worxd")
    
    fr fr Test repeat
    testz.assert_eq_string(string_repeat("abc", 3), "abcabcabc")
    testz.assert_eq_string(string_repeat("x", 0), "")
    testz.assert_eq_string(string_repeat("test", 1), "test")
}

slay test_string_padding() {
    testz.test_start("String Padding Functions")
    
    fr fr Test pad left
    testz.assert_eq_string(string_pad_left("hello", 10, " "), "     hello")
    testz.assert_eq_string(string_pad_left("hello", 8, "0"), "000hello")
    testz.assert_eq_string(string_pad_left("hello", 5, "x"), "hello")
    
    fr fr Test pad right
    testz.assert_eq_string(string_pad_right("hello", 10, " "), "hello     ")
    testz.assert_eq_string(string_pad_right("hello", 8, "0"), "hello000")
    testz.assert_eq_string(string_pad_right("hello", 5, "x"), "hello")
    
    fr fr Test pad center
    testz.assert_eq_string(string_pad_center("hello", 9, " "), "  hello  ")
    testz.assert_eq_string(string_pad_center("hi", 6, "x"), "xxhixx")
}

slay test_string_validation() {
    testz.test_start("String Validation Functions")
    
    fr fr Test numeric validation
    testz.assert_true(string_is_numeric("123"))
    testz.assert_true(string_is_numeric("123.45"))
    testz.assert_true(string_is_numeric("-123"))
    testz.assert_false(string_is_numeric("abc"))
    testz.assert_false(string_is_numeric("123abc"))
    
    fr fr Test alphabetic validation
    testz.assert_true(string_is_alpha("hello"))
    testz.assert_true(string_is_alpha("HELLO"))
    testz.assert_false(string_is_alpha("hello123"))
    testz.assert_false(string_is_alpha("123"))
    
    fr fr Test alphanumeric validation
    testz.assert_true(string_is_alphanumeric("hello123"))
    testz.assert_true(string_is_alphanumeric("ABC123"))
    testz.assert_false(string_is_alphanumeric("hello!"))
    testz.assert_false(string_is_alphanumeric("123-456"))
    
    fr fr Test whitespace validation
    testz.assert_true(string_is_whitespace("   "))
    testz.assert_true(string_is_whitespace("\t\n\r"))
    testz.assert_false(string_is_whitespace("hello"))
    testz.assert_false(string_is_whitespace("  hello  "))
}

slay test_string_conversion() {
    testz.test_start("String Conversion Functions")
    
    fr fr Test string to integer
    testz.assert_eq_int(string_to_int("123"), 123)
    testz.assert_eq_int(string_to_int("-456"), -456)
    testz.assert_eq_int(string_to_int("0"), 0)
    
    fr fr Test string to float
    testz.assert_eq_string(tea(string_to_float("123.45")), "123.45")
    testz.assert_eq_string(tea(string_to_float("-456.78")), "-456.78")
    testz.assert_eq_string(tea(string_to_float("0.0")), "0.0")
    
    fr fr Test string to boolean
    testz.assert_true(string_to_bool("true"))
    testz.assert_true(string_to_bool("based"))
    testz.assert_false(string_to_bool("false"))
    testz.assert_false(string_to_bool("cap"))
    
    fr fr Test conversions from other types
    testz.assert_eq_string(string_from_int(123), "123")
    testz.assert_eq_string(string_from_int(-456), "-456")
    testz.assert_eq_string(string_from_bool(based), "true")
    testz.assert_eq_string(string_from_bool(cap), "false")
}

slay test_string_utilities() {
    testz.test_start("String Utility Functions")
    
    fr fr Test reverse
    testz.assert_eq_string(string_reverse("hello"), "olleh")
    testz.assert_eq_string(string_reverse("abc"), "cba")
    testz.assert_eq_string(string_reverse(""), "")
    
    fr fr Test join
    sus words [tea] = ["hello", "world", "test"]
    testz.assert_eq_string(string_join(words, " "), "hello world test")
    testz.assert_eq_string(string_join(words, ","), "hello,world,test")
    testz.assert_eq_string(string_join(words, ""), "helloworldtest")
    
    fr fr Test hash
    sus hash1 normie = string_hash("hello")
    sus hash2 normie = string_hash("hello")
    sus hash3 normie = string_hash("world")
    testz.assert_eq_int(hash1, hash2)
    testz.assert_true(hash1 != hash3)
}

slay test_string_distance() {
    testz.test_start("String Distance Functions")
    
    fr fr Test Levenshtein distance
    testz.assert_eq_int(string_levenshtein_distance("hello", "hello"), 0)
    testz.assert_eq_int(string_levenshtein_distance("hello", "hallo"), 1)
    testz.assert_eq_int(string_levenshtein_distance("hello", ""), 5)
    testz.assert_eq_int(string_levenshtein_distance("", "hello"), 5)
    
    fr fr Test string similarity
    testz.assert_eq_string(tea(string_similarity("hello", "hello")), "1.0")
    testz.assert_true(string_similarity("hello", "hallo") > 0.0)
    testz.assert_true(string_similarity("hello", "world") < 1.0)
}

slay test_string_regex() {
    testz.test_start("String Regex Functions")
    
    fr fr Test regex match
    testz.assert_true(regex_match("\\d+", "123"))
    testz.assert_true(regex_match("[a-z]+", "hello"))
    testz.assert_false(regex_match("\\d+", "hello"))
    
    fr fr Test regex find
    testz.assert_eq_string(regex_find("\\d+", "abc123def"), "123")
    testz.assert_eq_string(regex_find("[a-z]+", "123abc456"), "abc")
    
    fr fr Test regex replace
    testz.assert_eq_string(regex_replace("\\d+", "abc123def", "XXX"), "abcXXXdef")
    testz.assert_eq_string(regex_replace("[a-z]+", "123abc456", "YYY"), "123YYY456")
}

slay test_string_edge_cases() {
    testz.test_start("String Edge Cases")
    
    fr fr Test empty string operations
    testz.assert_eq_string(string_trim(""), "")
    testz.assert_eq_string(string_to_upper(""), "")
    testz.assert_eq_string(string_reverse(""), "")
    
    fr fr Test single character operations
    testz.assert_eq_string(string_to_upper("a"), "A")
    testz.assert_eq_string(string_reverse("x"), "x")
    testz.assert_eq_int(string_len("y"), 1)
    
    fr fr Test Unicode handling
    testz.assert_true(string_len("🔥") >= 1)
    testz.assert_true(string_contains("Hello 🌍", "🌍"))
}

slay run_all_string_tests() {
    vibez.spill("📝 Running CURSED String Library Tests")
    vibez.spill("======================================")
    
    test_string_length()
    test_string_case_conversion()
    test_string_trimming()
    test_string_search()
    test_string_indexing()
    test_string_slicing()
    test_string_splitting()
    test_string_replacement()
    test_string_padding()
    test_string_validation()
    test_string_conversion()
    test_string_utilities()
    test_string_distance()
    test_string_regex()
    test_string_edge_cases()
    
    testz.print_test_summary()
    damn testz.run_all_tests()
}

fr fr Auto-run tests when this file is executed
run_all_string_tests()
