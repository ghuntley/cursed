// CURSED Regex Module Tests
// Comprehensive test suite for regex pattern matching

yeet "testz"
yeet "regex"

slay test_all_regex() {
    test_start("Regex Module Tests")
    
    test_basic_pattern_matching()
    test_wildcard_matching()
    test_find_matches()
    test_replace_patterns()
    test_split_by_pattern()
    test_character_classes()
    test_validation_functions()
    test_match_positions()
    test_match_counting()
    test_complex_patterns()
    
    print_test_summary()
}

slay test_basic_pattern_matching() {
    test_start("Basic Pattern Matching")
    
    // Exact matches
    assert_true(regex.match_pattern("hello", "hello"))
    assert_false(regex.match_pattern("hello", "world"))
    assert_false(regex.match_pattern("", "hello"))
    assert_true(regex.match_pattern("", ""))
    
    // Case sensitivity
    assert_false(regex.match_pattern("Hello", "hello"))
    assert_true(regex.match_pattern("Test", "Test"))
    
    // Partial matches (should fail for exact match)
    assert_false(regex.match_pattern("hello world", "hello"))
    assert_false(regex.match_pattern("hello", "hello world"))
}

slay test_wildcard_matching() {
    test_start("Wildcard Pattern Matching")
    
    // Asterisk wildcard
    assert_true(regex.match_wildcard("hello", "h*"))
    assert_true(regex.match_wildcard("hello", "*lo"))
    assert_true(regex.match_wildcard("hello", "h*o"))
    assert_true(regex.match_wildcard("hello", "*"))
    
    // Question mark wildcard
    assert_true(regex.match_wildcard("hello", "h?llo"))
    assert_true(regex.match_wildcard("hello", "????o"))
    assert_false(regex.match_wildcard("hello", "h?lo"))
    
    // Combined wildcards
    assert_true(regex.match_wildcard("hello", "h*l?o"))
    assert_true(regex.match_wildcard("test123", "test*"))
    assert_true(regex.match_wildcard("abc", "?*"))
}

slay test_find_matches() {
    test_start("Find Matches")
    
    // Single matches
    sus matches [tea] = regex.find_matches("hello world", "hello")
    assert_eq_int(len(matches), 1)
    assert_eq_string(matches[0], "hello")
    
    // Multiple matches
    sus multi [tea] = regex.find_matches("test test test", "test")
    assert_eq_int(len(multi), 3)
    
    // No matches
    sus none [tea] = regex.find_matches("hello", "world")
    assert_eq_int(len(none), 0)
    
    // Empty pattern
    sus empty [tea] = regex.find_matches("hello", "")
    assert_eq_int(len(empty), 0)
}

slay test_replace_patterns() {
    test_start("Replace Patterns")
    
    // Single replacement
    sus result1 tea = regex.replace_pattern("hello world", "hello", "hi")
    assert_eq_string(result1, "hi world")
    
    // No replacement
    sus result2 tea = regex.replace_pattern("hello world", "xyz", "abc")
    assert_eq_string(result2, "hello world")
    
    // Multiple replacements
    sus result3 tea = regex.replace_all_patterns("test test test", "test", "exam")
    assert_eq_string(result3, "exam exam exam")
    
    // Empty replacement
    sus result4 tea = regex.replace_pattern("hello world", "hello ", "")
    assert_eq_string(result4, "world")
}

slay test_split_by_pattern() {
    test_start("Split By Pattern")
    
    // Split by space
    sus parts1 [tea] = regex.split_by_pattern("hello world test", " ")
    assert_eq_int(len(parts1), 3)
    assert_eq_string(parts1[0], "hello")
    assert_eq_string(parts1[1], "world")
    assert_eq_string(parts1[2], "test")
    
    // Split by comma
    sus parts2 [tea] = regex.split_by_pattern("a,b,c", ",")
    assert_eq_int(len(parts2), 3)
    assert_eq_string(parts2[0], "a")
    assert_eq_string(parts2[1], "b")
    assert_eq_string(parts2[2], "c")
    
    // No split (pattern not found)
    sus parts3 [tea] = regex.split_by_pattern("hello", ",")
    assert_eq_int(len(parts3), 1)
    assert_eq_string(parts3[0], "hello")
}

slay test_character_classes() {
    test_start("Character Classes")
    
    // Digits
    assert_true(regex.is_digit("5"))
    assert_true(regex.is_digit("0"))
    assert_true(regex.is_digit("9"))
    assert_false(regex.is_digit("a"))
    assert_false(regex.is_digit("Z"))
    assert_false(regex.is_digit(" "))
    
    // Letters
    assert_true(regex.is_letter("a"))
    assert_true(regex.is_letter("Z"))
    assert_true(regex.is_letter("m"))
    assert_false(regex.is_letter("5"))
    assert_false(regex.is_letter(" "))
    
    // Whitespace
    assert_true(regex.is_whitespace(" "))
    assert_true(regex.is_whitespace("\t"))
    assert_true(regex.is_whitespace("\n"))
    assert_false(regex.is_whitespace("a"))
    assert_false(regex.is_whitespace("5"))
    
    // Alphanumeric
    assert_true(regex.is_alphanumeric("a"))
    assert_true(regex.is_alphanumeric("5"))
    assert_true(regex.is_alphanumeric("Z"))
    assert_false(regex.is_alphanumeric(" "))
    assert_false(regex.is_alphanumeric("@"))
}

slay test_validation_functions() {
    test_start("Validation Functions")
    
    // Email validation
    assert_true(regex.is_valid_email("test@example.com"))
    assert_true(regex.is_valid_email("user.name@domain.co.uk"))
    assert_false(regex.is_valid_email("invalid-email"))
    assert_false(regex.is_valid_email("@example.com"))
    assert_false(regex.is_valid_email("test@"))
    
    // URL validation
    assert_true(regex.is_valid_url("http://example.com"))
    assert_true(regex.is_valid_url("https://www.example.com"))
    assert_false(regex.is_valid_url("ftp://example.com"))
    assert_false(regex.is_valid_url("example.com"))
    assert_false(regex.is_valid_url(""))
    
    // Phone validation
    assert_true(regex.is_valid_phone("123-456-7890"))
    assert_true(regex.is_valid_phone("(123) 456-7890"))
    assert_true(regex.is_valid_phone("+1234567890"))
    assert_false(regex.is_valid_phone("123"))
    assert_false(regex.is_valid_phone("abc-def-ghij"))
    
    // IP address validation
    assert_true(regex.is_valid_ip("192.168.1.1"))
    assert_true(regex.is_valid_ip("255.255.255.255"))
    assert_true(regex.is_valid_ip("0.0.0.0"))
    assert_false(regex.is_valid_ip("256.1.1.1"))
    assert_false(regex.is_valid_ip("192.168.1"))
    assert_false(regex.is_valid_ip("192.168.1.1.1"))
}

slay test_match_positions() {
    test_start("Match Positions")
    
    // Single position
    sus pos1 [normie] = regex.get_match_positions("hello world", "world")
    assert_eq_int(len(pos1), 1)
    assert_eq_int(pos1[0], 6)
    
    // Multiple positions
    sus pos2 [normie] = regex.get_match_positions("test test test", "test")
    assert_eq_int(len(pos2), 3)
    assert_eq_int(pos2[0], 0)
    assert_eq_int(pos2[1], 5)
    assert_eq_int(pos2[2], 10)
    
    // No positions
    sus pos3 [normie] = regex.get_match_positions("hello", "world")
    assert_eq_int(len(pos3), 0)
}

slay test_match_counting() {
    test_start("Match Counting")
    
    // Count matches
    assert_eq_int(regex.count_matches("hello world", "l"), 3)
    assert_eq_int(regex.count_matches("test test test", "test"), 3)
    assert_eq_int(regex.count_matches("hello", "world"), 0)
    assert_eq_int(regex.count_matches("", "test"), 0)
    
    // Contains pattern
    assert_true(regex.contains_pattern("hello world", "world"))
    assert_true(regex.contains_pattern("test", "test"))
    assert_false(regex.contains_pattern("hello", "world"))
    assert_false(regex.contains_pattern("", "test"))
}

slay test_complex_patterns() {
    test_start("Complex Patterns")
    
    // Pattern validation
    assert_true(regex.is_valid_pattern("hello"))
    assert_true(regex.is_valid_pattern("test*"))
    assert_true(regex.is_valid_pattern("a?b"))
    assert_false(regex.is_valid_pattern(""))
    
    // Match results with details
    sus matches [regex.MatchResult] = regex.find_all_matches("hello world", "l")
    assert_eq_int(len(matches), 3)
    
    vibes len(matches) > 0 {
        assert_eq_string(matches[0].text, "l")
        assert_eq_int(matches[0].length, 1)
        assert_true(matches[0].start >= 0)
        assert_true(matches[0].end > matches[0].start)
    }
    
    // Group extraction
    sus groups [tea] = regex.extract_groups("hello", "hello")
    assert_eq_int(len(groups), 1)
    assert_eq_string(groups[0], "hello")
}

// Run all tests
test_all_regex()
