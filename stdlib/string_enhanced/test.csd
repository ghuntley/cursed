yeet "testz"
yeet "string_enhanced"

test_start("string_enhanced Tests")

// Test String Interning
test_case("String Interning - Basic Operations") {
    sus intern StringIntern = StringIntern_new()
    
    sus str1 tea = "hello_world"
    sus str2 tea = "cursed_lang"
    sus str3 tea = "hello_world"  // Duplicate
    
    // Intern strings
    sus (id1, intern1) = StringIntern_intern(intern, str1)
    sus (id2, intern2) = StringIntern_intern(intern1, str2)
    sus (id3, intern3) = StringIntern_intern(intern2, str3)
    
    // First two strings should have different IDs
    assert(id1 != id2)
    
    // Third string should have same ID as first (interned)
    assert_eq_int(id1, id3)
    
    // Retrieve strings by ID
    sus retrieved1 tea = StringIntern_get(intern3, id1)
    sus retrieved2 tea = StringIntern_get(intern3, id2)
    
    assert_eq_string(retrieved1, str1)
    assert_eq_string(retrieved2, str2)
}

// Test Advanced String Operations
test_case("Advanced String Operations") {
    sus text tea = "CURSED programming language"
    
    // Test string splitting
    sus words []tea = string_split_advanced(text, " ")
    assert_eq_int(len(words), 3)
    assert_eq_string(words[0], "CURSED")
    assert_eq_string(words[1], "programming")
    assert_eq_string(words[2], "language")
    
    // Test string joining
    sus rejoined tea = string_join_advanced(words, "_")
    assert_eq_string(rejoined, "CURSED_programming_language")
    
    // Test string trimming
    sus padded tea = "   \t  hello world  \n  "
    sus trimmed tea = string_trim_advanced(padded)
    assert_eq_string(trimmed, "hello world")
    
    // Test string case conversion
    sus lowercase tea = string_to_lower_advanced(text)
    assert_eq_string(lowercase, "cursed programming language")
    
    sus uppercase tea = string_to_upper_advanced(text)
    assert_eq_string(uppercase, "CURSED PROGRAMMING LANGUAGE")
}

// Test String Pattern Matching
test_case("String Pattern Matching") {
    sus text tea = "The CURSED language is awesome and CURSED rocks!"
    sus pattern tea = "CURSED"
    
    // Find all occurrences
    sus matches []normie = string_find_all_advanced(text, pattern)
    assert_eq_int(len(matches), 2)
    assert_eq_int(matches[0], 4)   // First occurrence
    assert_eq_int(matches[1], 35)  // Second occurrence
    
    // Test pattern replacement
    sus replaced tea = string_replace_all_advanced(text, "CURSED", "BLESSED")
    assert_eq_string(replaced, "The BLESSED language is awesome and BLESSED rocks!")
    
    // Test case-insensitive search
    sus case_matches []normie = string_find_all_case_insensitive(text, "cursed")
    assert_eq_int(len(case_matches), 2)
    
    // Test pattern validation
    assert(string_matches_pattern("abc123", "[a-z]+[0-9]+"))
    assert(!string_matches_pattern("123abc", "[a-z]+[0-9]+"))
}

// Test Unicode String Handling
test_case("Unicode String Handling") {
    sus unicode_text tea = "Hello 世界 🌍 CURSED"
    
    // Test Unicode length (character count vs byte count)
    sus char_count normie = string_unicode_length(unicode_text)
    sus byte_count normie = string_byte_length(unicode_text)
    
    assert(char_count < byte_count)  // Unicode chars take more bytes
    assert_eq_int(char_count, 15)   // Should count emojis as single chars
    
    // Test Unicode substring
    sus substring tea = string_unicode_substring(unicode_text, 6, 2)
    assert_eq_string(substring, "世界")
    
    // Test Unicode normalization
    sus normalized tea = string_unicode_normalize(unicode_text)
    assert(string_length(normalized) > 0)
    
    // Test Unicode categories
    assert(string_is_unicode_letter("世"))
    assert(string_is_unicode_digit("９"))  // Full-width 9
    assert(string_is_unicode_symbol("🌍"))
}

// Test String Security Functions
test_case("String Security Functions") {
    sus unsafe_input tea = "<script>alert('xss')</script>"
    sus sql_input tea = "'; DROP TABLE users; --"
    
    // Test HTML escaping
    sus escaped_html tea = string_escape_html(unsafe_input)
    assert(string_contains(escaped_html, "&lt;script&gt;"))
    assert(string_contains(escaped_html, "&lt;/script&gt;"))
    
    // Test SQL escaping
    sus escaped_sql tea = string_escape_sql(sql_input)
    assert(string_contains(escaped_sql, "\\'"))
    assert(!string_contains(escaped_sql, "'; DROP"))
    
    // Test URL encoding
    sus url_input tea = "hello world & special chars!"
    sus url_encoded tea = string_url_encode(url_input)
    assert(string_contains(url_encoded, "%20"))  // space
    assert(string_contains(url_encoded, "%26"))  // ampersand
    
    sus url_decoded tea = string_url_decode(url_encoded)
    assert_eq_string(url_decoded, url_input)
    
    // Test base64 encoding
    sus base64_encoded tea = string_base64_encode("CURSED")
    sus base64_decoded tea = string_base64_decode(base64_encoded)
    assert_eq_string(base64_decoded, "CURSED")
}

// Test String Performance Optimizations
test_case("String Performance Optimizations") {
    sus large_string tea = string_repeat("CURSED ", 10000)
    
    // Test fast string search
    sus start_time drip = get_current_time_ms()
    sus pattern_pos normie = string_fast_find(large_string, "CURSED")
    sus search_time drip = get_current_time_ms() - start_time
    
    assert_eq_int(pattern_pos, 0)
    print_test_status("Fast string search time: " + string_from_int(search_time) + "ms")
    
    // Test string hashing
    start_time = get_current_time_ms()
    sus hash1 drip = string_hash_fast(large_string)
    sus hash_time drip = get_current_time_ms() - start_time
    
    sus hash2 drip = string_hash_fast(large_string)
    assert_eq_int(hash1, hash2)  // Should be deterministic
    print_test_status("String hashing time: " + string_from_int(hash_time) + "ms")
    
    // Test string builder performance
    start_time = get_current_time_ms()
    sus builder StringBuilder = StringBuilder_new()
    bestie (sus i normie = 0; i < 1000; i += 1) {
        builder = StringBuilder_append(builder, "CURSED")
    }
    sus built_string tea = StringBuilder_to_string(builder)
    sus builder_time drip = get_current_time_ms() - start_time
    
    assert_eq_int(string_length(built_string), 6000)  // 1000 * 6
    print_test_status("StringBuilder time: " + string_from_int(builder_time) + "ms")
}

// Test String Formatting
test_case("String Formatting") {
    sus name tea = "CURSED"
    sus version drip = 1.0
    sus count normie = 42
    
    // Test template formatting
    sus template tea = "Language: {0}, Version: {1}, Count: {2}"
    sus formatted tea = string_format(template, [name, string_from_float(version), string_from_int(count)])
    assert_eq_string(formatted, "Language: CURSED, Version: 1.0, Count: 42")
    
    // Test printf-style formatting
    sus printf_result tea = string_printf("Hello %s! You have %d messages.", [name, string_from_int(count)])
    assert_eq_string(printf_result, "Hello CURSED! You have 42 messages.")
    
    // Test padding and alignment
    sus left_padded tea = string_pad_left("test", 10, " ")
    assert_eq_string(left_padded, "      test")
    
    sus right_padded tea = string_pad_right("test", 10, "0")
    assert_eq_string(right_padded, "test000000")
    
    sus centered tea = string_center("test", 10, "-")
    assert_eq_string(centered, "---test---")
}

// Test String Validation
test_case("String Validation") {
    // Test email validation
    assert(string_is_valid_email("test@example.com"))
    assert(string_is_valid_email("user.name+tag@domain.co.uk"))
    assert(!string_is_valid_email("invalid.email"))
    assert(!string_is_valid_email("@domain.com"))
    
    // Test URL validation
    assert(string_is_valid_url("https://example.com"))
    assert(string_is_valid_url("http://sub.domain.org/path?query=value"))
    assert(!string_is_valid_url("not-a-url"))
    assert(!string_is_valid_url("ftp://invalid"))
    
    // Test IP address validation
    assert(string_is_valid_ipv4("192.168.1.1"))
    assert(string_is_valid_ipv4("255.255.255.255"))
    assert(!string_is_valid_ipv4("256.1.1.1"))
    assert(!string_is_valid_ipv4("192.168.1"))
    
    // Test JSON validation
    sus valid_json tea = "{\"name\": \"CURSED\", \"version\": 1.0}"
    sus invalid_json tea = "{name: CURSED, version: 1.0}"
    
    assert(string_is_valid_json(valid_json))
    assert(!string_is_valid_json(invalid_json))
}

print_test_summary()
