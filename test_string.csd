yeet "testz"
yeet "string"

fr fr ========================================
fr fr CURSED String Library Test Suite
fr fr ========================================

slay test_string_length() {
    test_start("String Length Functions")
    
    fr fr Test string length
    assert_eq_int(string_len("hello"), 5)
    assert_eq_int(string_len(""), 0)
    assert_eq_int(string_len("CURSED"), 6)
    
    fr fr Test empty string check
    assert_true(string_is_empty(""))
    assert_false(string_is_empty("hello"))
}

slay test_string_case_conversion() {
    test_start("String Case Conversion")
    
    fr fr Test uppercase conversion
    assert_eq_string(string_to_upper("hello"), "HELLO")
    assert_eq_string(string_to_upper("CURSED"), "CURSED")
    assert_eq_string(string_to_upper("MiXeD"), "MIXED")
    
    fr fr Test lowercase conversion
    assert_eq_string(string_to_lower("HELLO"), "hello")
    assert_eq_string(string_to_lower("cursed"), "cursed")
    assert_eq_string(string_to_lower("MiXeD"), "mixed")
    
    fr fr Test capitalize
    assert_eq_string(string_capitalize("hello"), "Hello")
    assert_eq_string(string_capitalize("HELLO"), "Hello")
    assert_eq_string(string_capitalize(""), "")
}

slay test_string_trimming() {
    test_start("String Trimming Functions")
    
    fr fr Test trim all whitespace
    assert_eq_string(string_trim("  hello  "), "hello")
    assert_eq_string(string_trim("\t\ntest\r\n"), "test")
    assert_eq_string(string_trim("no-spaces"), "no-spaces")
    
    fr fr Test trim start
    assert_eq_string(string_trim_start("  hello  "), "hello  ")
    assert_eq_string(string_trim_start("\t\ntest"), "test")
    
    fr fr Test trim end
    assert_eq_string(string_trim_end("  hello  "), "  hello")
    assert_eq_string(string_trim_end("test\r\n"), "test")
}

slay test_string_search() {
    test_start("String Search Functions")
    
    fr fr Test contains
    assert_true(string_contains("hello world", "world"))
    assert_true(string_contains("hello world", "hello"))
    assert_false(string_contains("hello world", "xyz"))
    
    fr fr Test starts with
    assert_true(string_starts_with("hello world", "hello"))
    assert_true(string_starts_with("hello world", ""))
    assert_false(string_starts_with("hello world", "world"))
    
    fr fr Test ends with
    assert_true(string_ends_with("hello world", "world"))
    assert_true(string_ends_with("hello world", ""))
    assert_false(string_ends_with("hello world", "hello"))
}

slay test_string_indexing() {
    test_start("String Indexing Functions")
    
    fr fr Test index of
    assert_eq_int(string_index_of("hello world", "world"), 6)
    assert_eq_int(string_index_of("hello world", "hello"), 0)
    assert_eq_int(string_index_of("hello world", "xyz"), -1)
    
    fr fr Test last index of
    assert_eq_int(string_last_index_of("hello hello", "hello"), 6)
    assert_eq_int(string_last_index_of("hello hello", "xyz"), -1)
    
    fr fr Test count occurrences
    assert_eq_int(string_count_occurrences("hello hello hello", "hello"), 3)
    assert_eq_int(string_count_occurrences("hello world", "l"), 3)
    assert_eq_int(string_count_occurrences("hello world", "xyz"), 0)
}

slay test_string_slicing() {
    test_start("String Slicing Functions")
    
    fr fr Test slice
    assert_eq_string(string_slice("hello world", 0, 5), "hello")
    assert_eq_string(string_slice("hello world", 6, 11), "world")
    assert_eq_string(string_slice("hello world", 2, 8), "llo wo")
    
    fr fr Test substring
    assert_eq_string(string_substring("hello world", 0, 5), "hello")
    assert_eq_string(string_substring("hello world", 6, 5), "world")
    assert_eq_string(string_substring("hello world", 2, 6), "llo wo")
    
    fr fr Test char at
    assert_eq_string(string_char_at("hello", 0), "h")
    assert_eq_string(string_char_at("hello", 4), "o")
    assert_eq_string(string_char_at("hello", 2), "l")
}

slay test_string_splitting() {
    test_start("String Splitting Functions")
    
    fr fr Test split by delimiter
    sus parts [tea] = string_split("a,b,c", ",")
    assert_eq_int(len(parts), 3)
    assert_eq_string(parts[0], "a")
    assert_eq_string(parts[1], "b")
    assert_eq_string(parts[2], "c")
    
    fr fr Test split lines
    sus lines [tea] = string_split_lines("line1\nline2\nline3")
    assert_eq_int(len(lines), 3)
    assert_eq_string(lines[0], "line1")
    assert_eq_string(lines[1], "line2")
    assert_eq_string(lines[2], "line3")
    
    fr fr Test split whitespace
    sus words [tea] = string_split_whitespace("hello   world\t\ntest")
    assert_eq_int(len(words), 3)
    assert_eq_string(words[0], "hello")
    assert_eq_string(words[1], "world")
    assert_eq_string(words[2], "test")
}

slay test_string_replacement() {
    test_start("String Replacement Functions")
    
    fr fr Test replace first occurrence
    assert_eq_string(string_replace("hello hello", "hello", "hi"), "hi hello")
    assert_eq_string(string_replace("hello world", "xyz", "abc"), "hello world")
    
    fr fr Test replace all occurrences
    assert_eq_string(string_replace_all("hello hello", "hello", "hi"), "hi hi")
    assert_eq_string(string_replace_all("hello world", "l", "x"), "hexxo worxd")
    
    fr fr Test repeat
    assert_eq_string(string_repeat("abc", 3), "abcabcabc")
    assert_eq_string(string_repeat("x", 0), "")
    assert_eq_string(string_repeat("test", 1), "test")
}

slay test_string_padding() {
    test_start("String Padding Functions")
    
    fr fr Test pad left
    assert_eq_string(string_pad_left("hello", 10, " "), "     hello")
    assert_eq_string(string_pad_left("hello", 8, "0"), "000hello")
    assert_eq_string(string_pad_left("hello", 5, "x"), "hello")
    
    fr fr Test pad right
    assert_eq_string(string_pad_right("hello", 10, " "), "hello     ")
    assert_eq_string(string_pad_right("hello", 8, "0"), "hello000")
    assert_eq_string(string_pad_right("hello", 5, "x"), "hello")
    
    fr fr Test pad center
    assert_eq_string(string_pad_center("hello", 9, " "), "  hello  ")
    assert_eq_string(string_pad_center("hi", 6, "x"), "xxhixx")
}

slay test_string_validation() {
    test_start("String Validation Functions")
    
    fr fr Test numeric validation
    assert_true(string_is_numeric("123"))
    assert_true(string_is_numeric("123.45"))
    assert_true(string_is_numeric("-123"))
    assert_false(string_is_numeric("abc"))
    assert_false(string_is_numeric("123abc"))
    
    fr fr Test alphabetic validation
    assert_true(string_is_alpha("hello"))
    assert_true(string_is_alpha("HELLO"))
    assert_false(string_is_alpha("hello123"))
    assert_false(string_is_alpha("123"))
    
    fr fr Test alphanumeric validation
    assert_true(string_is_alphanumeric("hello123"))
    assert_true(string_is_alphanumeric("ABC123"))
    assert_false(string_is_alphanumeric("hello!"))
    assert_false(string_is_alphanumeric("123-456"))
    
    fr fr Test whitespace validation
    assert_true(string_is_whitespace("   "))
    assert_true(string_is_whitespace("\t\n\r"))
    assert_false(string_is_whitespace("hello"))
    assert_false(string_is_whitespace("  hello  "))
}

slay test_string_conversion() {
    test_start("String Conversion Functions")
    
    fr fr Test string to integer
    assert_eq_int(string_to_int("123"), 123)
    assert_eq_int(string_to_int("-456"), -456)
    assert_eq_int(string_to_int("0"), 0)
    
    fr fr Test string to float
    assert_eq_string(tea(string_to_float("123.45")), "123.45")
    assert_eq_string(tea(string_to_float("-456.78")), "-456.78")
    assert_eq_string(tea(string_to_float("0.0")), "0.0")
    
    fr fr Test string to boolean
    assert_true(string_to_bool("true"))
    assert_true(string_to_bool("based"))
    assert_false(string_to_bool("false"))
    assert_false(string_to_bool("cap"))
    
    fr fr Test conversions from other types
    assert_eq_string(string_from_int(123), "123")
    assert_eq_string(string_from_int(-456), "-456")
    assert_eq_string(string_from_bool(based), "true")
    assert_eq_string(string_from_bool(cap), "false")
}

slay test_string_utilities() {
    test_start("String Utility Functions")
    
    fr fr Test reverse
    assert_eq_string(string_reverse("hello"), "olleh")
    assert_eq_string(string_reverse("abc"), "cba")
    assert_eq_string(string_reverse(""), "")
    
    fr fr Test join
    sus words [tea] = ["hello", "world", "test"]
    assert_eq_string(string_join(words, " "), "hello world test")
    assert_eq_string(string_join(words, ","), "hello,world,test")
    assert_eq_string(string_join(words, ""), "helloworldtest")
    
    fr fr Test hash
    sus hash1 normie = string_hash("hello")
    sus hash2 normie = string_hash("hello")
    sus hash3 normie = string_hash("world")
    assert_eq_int(hash1, hash2)
    assert_true(hash1 != hash3)
}

slay test_string_distance() {
    test_start("String Distance Functions")
    
    fr fr Test Levenshtein distance
    assert_eq_int(string_levenshtein_distance("hello", "hello"), 0)
    assert_eq_int(string_levenshtein_distance("hello", "hallo"), 1)
    assert_eq_int(string_levenshtein_distance("hello", ""), 5)
    assert_eq_int(string_levenshtein_distance("", "hello"), 5)
    
    fr fr Test string similarity
    assert_eq_string(tea(string_similarity("hello", "hello")), "1.0")
    assert_true(string_similarity("hello", "hallo") > 0.0)
    assert_true(string_similarity("hello", "world") < 1.0)
}

slay test_string_regex() {
    test_start("String Regex Functions")
    
    fr fr Test regex match
    assert_true(regex_match("\\d+", "123"))
    assert_true(regex_match("[a-z]+", "hello"))
    assert_false(regex_match("\\d+", "hello"))
    
    fr fr Test regex find
    assert_eq_string(regex_find("\\d+", "abc123def"), "123")
    assert_eq_string(regex_find("[a-z]+", "123abc456"), "abc")
    
    fr fr Test regex replace
    assert_eq_string(regex_replace("\\d+", "abc123def", "XXX"), "abcXXXdef")
    assert_eq_string(regex_replace("[a-z]+", "123abc456", "YYY"), "123YYY456")
}

slay test_string_edge_cases() {
    test_start("String Edge Cases")
    
    fr fr Test empty string operations
    assert_eq_string(string_trim(""), "")
    assert_eq_string(string_to_upper(""), "")
    assert_eq_string(string_reverse(""), "")
    
    fr fr Test single character operations
    assert_eq_string(string_to_upper("a"), "A")
    assert_eq_string(string_reverse("x"), "x")
    assert_eq_int(string_len("y"), 1)
    
    fr fr Test Unicode handling
    assert_true(string_len("🔥") >= 1)
    assert_true(string_contains("Hello 🌍", "🌍"))
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
    
    print_test_summary()
    damn run_all_tests()
}

fr fr Auto-run tests when this file is executed
run_all_string_tests()
