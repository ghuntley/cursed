// Comprehensive String Module Test
// Tests all 52+ string functions in the enhanced runtime bridge

yeet ("stdlib/testz");

slay main() {
    // Test basic string operations
    test_start("String Length");
    sus text tea = "Hello, 世界!";
    sus length normie = string_len(text);
    assert_eq_int(length, 9); // Unicode character count
    
    test_start("String Empty Check");
    sus empty tea = "";
    sus non_empty tea = "test";
    assert_true(string_is_empty(empty));
    assert_false(string_is_empty(non_empty));
    
    // Test string trimming
    test_start("String Trimming");
    sus whitespace tea = "  hello world  ";
    sus trimmed tea = string_trim(whitespace);
    sus trimmed_start tea = string_trim_start(whitespace);
    sus trimmed_end tea = string_trim_end(whitespace);
    assert_eq_string(trimmed, "hello world");
    assert_eq_string(trimmed_start, "hello world  ");
    assert_eq_string(trimmed_end, "  hello world");
    
    // Test case conversion
    test_start("Case Conversion");
    sus mixed tea = "Hello World";
    sus upper tea = string_to_upper(mixed);
    sus lower tea = string_to_lower(mixed);
    sus capitalized tea = string_capitalize("hello");
    assert_eq_string(upper, "HELLO WORLD");
    assert_eq_string(lower, "hello world");
    assert_eq_string(capitalized, "Hello");
    
    // Test string searching
    test_start("String Contains & Search");
    sus haystack tea = "The quick brown fox jumps";
    assert_true(string_contains(haystack, "quick"));
    assert_false(string_contains(haystack, "lazy"));
    assert_true(string_starts_with(haystack, "The"));
    assert_true(string_ends_with(haystack, "jumps"));
    
    sus index normie = string_index_of(haystack, "quick");
    sus last_index normie = string_last_index_of(haystack, "o");
    assert_eq_int(index, 4);
    assert_eq_int(last_index, 17); // Last 'o' in 'fox'
    
    sus count normie = string_count_occurrences(haystack, "o");
    assert_eq_int(count, 2); // 'o' appears in "brown" and "fox"
    
    // Test string slicing and character access
    test_start("String Slicing");
    sus slice tea = string_slice(haystack, 4, 9);
    sus substring tea = string_substring(haystack, 10, 5);
    sus char_at tea = string_char_at(haystack, 0);
    assert_eq_string(slice, "quick");
    assert_eq_string(substring, "brown");
    assert_eq_string(char_at, "T");
    
    // Test string replacement
    test_start("String Replacement");
    sus original tea = "hello world hello";
    sus replace_first tea = string_replace(original, "hello", "hi");
    sus replace_all tea = string_replace_all(original, "hello", "hi");
    assert_eq_string(replace_first, "hi world hello");
    assert_eq_string(replace_all, "hi world hi");
    
    // Test string repeat and reverse
    test_start("String Manipulation");
    sus repeated tea = string_repeat("ha", 3);
    sus reversed tea = string_reverse("hello");
    assert_eq_string(repeated, "hahaha");
    assert_eq_string(reversed, "olleh");
    
    // Test string validation
    test_start("String Validation");
    assert_true(string_is_numeric("123"));
    assert_true(string_is_numeric("-45.67"));
    assert_false(string_is_numeric("abc"));
    
    assert_true(string_is_alpha("hello"));
    assert_false(string_is_alpha("hello123"));
    
    assert_true(string_is_alphanumeric("hello123"));
    assert_false(string_is_alphanumeric("hello-123"));
    
    assert_true(string_is_whitespace("   \t\n"));
    assert_false(string_is_whitespace("  a  "));
    
    assert_true(string_is_ascii("hello"));
    assert_false(string_is_ascii("hello世界"));
    
    // Test string conversion
    test_start("String Conversion");
    sus int_str tea = string_from_int(42);
    sus float_str tea = string_from_float(3.14);
    sus bool_str tea = string_from_bool(based);
    assert_eq_string(int_str, "42");
    assert_eq_string(bool_str, "based");
    
    sus str_int normie = string_to_int("123");
    sus str_float meal = string_to_float("3.14");
    sus str_bool lit = string_to_bool("based");
    assert_eq_int(str_int, 123);
    assert_true(str_bool);
    
    // Test string escape/unescape
    test_start("String Escaping");
    sus with_newlines tea = "hello\nworld\ttab";
    sus escaped tea = string_escape(with_newlines);
    sus unescaped tea = string_unescape(escaped);
    assert_eq_string(escaped, "hello\\nworld\\ttab");
    assert_eq_string(unescaped, with_newlines);
    
    // Test string splitting
    test_start("String Splitting");
    sus csv tea = "apple,banana,cherry";
    sus parts [tea] = string_split(csv, ",");
    assert_eq_int(parts.length, 3);
    
    sus multiline tea = "line1\nline2\nline3";
    sus lines [tea] = string_split_lines(multiline);
    assert_eq_int(lines.length, 3);
    
    sus spaced tea = "word1  word2\tword3";
    sus words [tea] = string_split_whitespace(spaced);
    assert_eq_int(words.length, 3);
    
    // Test string joining
    test_start("String Joining");
    sus items [tea] = ["a", "b", "c"];
    sus joined tea = string_join(items, ", ");
    assert_eq_string(joined, "a, b, c");
    
    // Test string padding
    test_start("String Padding");
    sus short tea = "hi";
    sus padded_left tea = string_pad_left(short, 5, "*");
    sus padded_right tea = string_pad_right(short, 5, "*");
    sus padded_center tea = string_pad_center(short, 6, "*");
    assert_eq_string(padded_left, "***hi");
    assert_eq_string(padded_right, "hi***");
    assert_eq_string(padded_center, "**hi**");
    
    // Test regular expressions
    test_start("Regular Expressions");
    sus email tea = "test@example.com";
    sus email_pattern tea = "\\w+@\\w+\\.\\w+";
    assert_true(regex_match(email_pattern, email));
    
    sus found tea = regex_find("\\d+", "age: 25 years");
    assert_eq_string(found, "25");
    
    sus phone tea = "Call 123-456-7890 or 987-654-3210";
    sus phone_pattern tea = "\\d{3}-\\d{3}-\\d{4}";
    sus all_phones [tea] = regex_find_all(phone_pattern, phone);
    assert_eq_int(all_phones.length, 2);
    
    sus replaced tea = regex_replace("\\d+", "I have 5 cats", "many");
    assert_eq_string(replaced, "I have many cats");
    
    // Test string hashing and similarity
    test_start("String Utilities");
    sus hash1 normie = string_hash("hello");
    sus hash2 normie = string_hash("hello");
    sus hash3 normie = string_hash("world");
    assert_eq_int(hash1, hash2); // Same strings should have same hash
    assert_ne_int(hash1, hash3); // Different strings should have different hashes
    
    sus similarity meal = string_similarity("kitten", "sitting");
    assert_true(similarity > 0.5); // Should be reasonably similar
    
    sus distance normie = string_levenshtein_distance("kitten", "sitting");
    assert_eq_int(distance, 3); // Known edit distance
    
    // Test string formatting
    test_start("String Formatting");
    sus template tea = "Hello {0}, you have {1} messages";
    sus args [tea] = ["Alice", "5"];
    sus formatted tea = string_format(template, args);
    assert_eq_string(formatted, "Hello Alice, you have 5 messages");
    
    // Test byte conversion
    test_start("Byte Conversion");
    sus original_text tea = "Hello, UTF-8! 🌟";
    sus bytes [byte] = string_to_bytes(original_text);
    sus reconstructed tea = string_from_bytes(bytes);
    assert_eq_string(reconstructed, original_text);
    
    print_test_summary();
}
