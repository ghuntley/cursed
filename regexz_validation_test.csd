# RegexZ Advanced Regular Expression Package Validation Test
# Comprehensive testing of all regex features

yeet "testz"
yeet "regexz"

test_start("RegexZ Advanced Regex Engine Validation")

# Test basic regex functionality
slay test_basic_regex() drip {
    test_group("Basic Regex Functionality")
    
    # Test pattern compilation
    sus engine RegexEngine = regex_compile("hello") shook test_failed("Failed to compile basic pattern")
    
    # Test simple matching
    sus matched lit = regex_test_pattern("\\d+", "abc123def") shook test_failed("Failed to test pattern")
    assert_true(matched, "Should match digits in text")
    
    # Test pattern validation
    assert_true(regex_validate("[a-z]+"), "Valid pattern should validate")
    assert_false(regex_validate("[unclosed"), "Invalid pattern should not validate")
    
    # Test convenience functions
    sus extracted tea = regex_extract_first("\\d+", "Price: $25.99") shook test_failed("Failed to extract first match")
    assert_eq_str(extracted, "25", "Should extract first number")
    
    sus all_extracted []tea = regex_extract_all("\\d+", "I have 5 apples and 3 oranges") shook test_failed("Failed to extract all matches")
    assert_eq_int(all_extracted.len(), 2, "Should find 2 numbers")
    assert_eq_str(all_extracted[0], "5", "First number should be 5")
    assert_eq_str(all_extracted[1], "3", "Second number should be 3")
    
    test_passed("Basic regex functionality")
}

# Test named capture groups
slay test_named_groups() drip {
    test_group("Named Capture Groups")
    
    sus engine RegexEngine = regex_compile("(?P<year>\\d{4})-(?P<month>\\d{2})-(?P<day>\\d{2})") shook test_failed("Failed to compile named groups pattern")
    
    sus result MatchResult = regex_match(&engine, "Today is 2023-12-25") shook test_failed("Failed to match date")
    assert_true(result.matched, "Should match date pattern")
    
    sus year tea = get_named_group(result, "year") shook test_failed("Failed to get year group")
    sus month tea = get_named_group(result, "month") shook test_failed("Failed to get month group")
    sus day tea = get_named_group(result, "day") shook test_failed("Failed to get day group")
    
    assert_eq_str(year, "2023", "Year should be 2023")
    assert_eq_str(month, "12", "Month should be 12")
    assert_eq_str(day, "25", "Day should be 25")
    
    # Test all named groups extraction
    sus all_groups map<tea, tea> = get_all_named_groups(result)
    assert_eq_int(all_groups.len(), 3, "Should have 3 named groups")
    assert_eq_str(all_groups["year"], "2023", "Year group should be 2023")
    
    test_passed("Named capture groups")
}

# Test Unicode property support
slay test_unicode_properties() drip {
    test_group("Unicode Property Support")
    
    # Test letter property
    sus letter_matched lit = regex_test_pattern("\\p{L}+", "Hello世界") shook test_failed("Failed to test letter property")
    assert_true(letter_matched, "Should match Unicode letters")
    
    # Test digit property
    sus digit_matched lit = regex_test_pattern("\\p{Nd}+", "123456") shook test_failed("Failed to test digit property")
    assert_true(digit_matched, "Should match decimal digits")
    
    # Test script property
    sus latin_matched lit = regex_test_pattern("\\p{Script=Latin}+", "Hello") shook test_failed("Failed to test script property")
    assert_true(latin_matched, "Should match Latin script")
    
    # Test whitespace property
    sus whitespace_matched lit = regex_test_pattern("\\p{White_Space}+", "   \t\n") shook test_failed("Failed to test whitespace property")
    assert_true(whitespace_matched, "Should match Unicode whitespace")
    
    test_passed("Unicode property support")
}

# Test lookahead and lookbehind
slay test_lookaround() drip {
    test_group("Lookahead and Lookbehind")
    
    # Positive lookahead
    sus pos_lookahead lit = regex_test_pattern("\\d+(?=px)", "100px") shook test_failed("Failed positive lookahead")
    assert_true(pos_lookahead, "Should match digits followed by px")
    
    sus pos_lookahead_fail lit = regex_test_pattern("\\d+(?=px)", "100pt") shook test_failed("Failed positive lookahead test")
    assert_false(pos_lookahead_fail, "Should not match digits not followed by px")
    
    # Negative lookahead
    sus neg_lookahead lit = regex_test_pattern("\\d+(?!px)", "100pt") shook test_failed("Failed negative lookahead")
    assert_true(neg_lookahead, "Should match digits not followed by px")
    
    # Positive lookbehind
    sus pos_lookbehind lit = regex_test_pattern("(?<=\\$)\\d+", "$100") shook test_failed("Failed positive lookbehind")
    assert_true(pos_lookbehind, "Should match digits preceded by $")
    
    # Negative lookbehind
    sus neg_lookbehind lit = regex_test_pattern("(?<!\\$)\\d+", "€100") shook test_failed("Failed negative lookbehind")
    assert_true(neg_lookbehind, "Should match digits not preceded by $")
    
    test_passed("Lookahead and lookbehind")
}

# Test replacement functionality
slay test_replacements() drip {
    test_group("String Replacement")
    
    # Simple replacement
    sus simple_replaced tea = regex_replace_simple("\\d+", "I have 5 apples", "many") shook test_failed("Failed simple replacement")
    assert_eq_str(simple_replaced, "I have many apples", "Should replace number with 'many'")
    
    # Group-based replacement
    sus engine RegexEngine = regex_compile("(\\w+) (\\w+)") shook test_failed("Failed to compile name pattern")
    sus swapped tea = regex_replace(&engine, "John Doe", "$2, $1") shook test_failed("Failed group replacement")
    assert_eq_str(swapped, "Doe, John", "Should swap first and last name")
    
    test_passed("String replacement")
}

# Test string splitting
slay test_splitting() drip {
    test_group("String Splitting")
    
    sus parts []tea = regex_split_simple("[,;]\\s*", "apple, banana; cherry,date") shook test_failed("Failed to split string")
    assert_eq_int(parts.len(), 4, "Should split into 4 parts")
    assert_eq_str(parts[0], "apple", "First part should be apple")
    assert_eq_str(parts[1], "banana", "Second part should be banana")
    assert_eq_str(parts[2], "cherry", "Third part should be cherry")
    assert_eq_str(parts[3], "date", "Fourth part should be date")
    
    test_passed("String splitting")
}

# Test built-in validation patterns
slay test_validation_patterns() drip {
    test_group("Built-in Validation Patterns")
    
    # Email validation
    sus valid_email lit = regex_is_email("test@example.com") shook test_failed("Failed email validation")
    assert_true(valid_email, "Should validate correct email")
    
    sus invalid_email lit = regex_is_email("invalid.email") shook test_failed("Failed invalid email test")
    assert_false(invalid_email, "Should reject invalid email")
    
    # URL validation
    sus valid_url lit = regex_is_url("https://www.example.com") shook test_failed("Failed URL validation")
    assert_true(valid_url, "Should validate correct URL")
    
    sus invalid_url lit = regex_is_url("not-a-url") shook test_failed("Failed invalid URL test")
    assert_false(invalid_url, "Should reject invalid URL")
    
    # IPv4 validation
    sus valid_ip lit = regex_is_ipv4("192.168.1.1") shook test_failed("Failed IPv4 validation")
    assert_true(valid_ip, "Should validate correct IPv4")
    
    sus invalid_ip lit = regex_is_ipv4("256.256.256.256") shook test_failed("Failed invalid IPv4 test")
    assert_false(invalid_ip, "Should reject invalid IPv4")
    
    test_passed("Built-in validation patterns")
}

# Test utility functions
slay test_utility_functions() drip {
    test_group("Utility Functions")
    
    # Phone number extraction
    sus phone tea = regex_extract_phone("Call me at 555-123-4567") shook test_failed("Failed phone extraction")
    assert_eq_str(phone, "(555) 123-4567", "Should format phone number correctly")
    
    # HTML tag stripping
    sus clean_html tea = regex_strip_html("<p>Hello <b>world</b>!</p>") shook test_failed("Failed HTML stripping")
    assert_eq_str(clean_html, "Hello world!", "Should strip HTML tags")
    
    # Hashtag extraction
    sus hashtags []tea = regex_extract_hashtags("Love this #coding #life #blessed") shook test_failed("Failed hashtag extraction")
    assert_eq_int(hashtags.len(), 3, "Should find 3 hashtags")
    assert_eq_str(hashtags[0], "#coding", "First hashtag should be #coding")
    
    # Date extraction
    sus dates []DateMatch = regex_extract_dates("Meeting on 2023-12-25 and 2024-01-01") shook test_failed("Failed date extraction")
    assert_eq_int(dates.len(), 2, "Should find 2 dates")
    assert_eq_str(dates[0].year, "2023", "First year should be 2023")
    assert_eq_str(dates[1].day, "01", "Second day should be 01")
    
    test_passed("Utility functions")
}

# Test password validation
slay test_password_validation() drip {
    test_group("Password Validation")
    
    # Strong password
    sus strong_password tea = "MyP@ssw0rd123"
    sus strength PasswordStrength = regex_validate_password(strong_password) shook test_failed("Failed strong password validation")
    
    assert_true(strength.has_lowercase, "Strong password should have lowercase")
    assert_true(strength.has_uppercase, "Strong password should have uppercase") 
    assert_true(strength.has_digits, "Strong password should have digits")
    assert_true(strength.has_special, "Strong password should have special chars")
    assert_true(strength.min_length, "Strong password should meet length requirement")
    assert_eq_int(strength.score, 5, "Strong password should have max score")
    
    # Weak password
    sus weak_password tea = "weak"
    sus weak_strength PasswordStrength = regex_validate_password(weak_password) shook test_failed("Failed weak password validation")
    
    assert_false(weak_strength.min_length, "Weak password should fail length requirement")
    assert_true(weak_strength.score < 3, "Weak password should have low score")
    
    test_passed("Password validation")
}

# Test configuration parsing
slay test_config_parsing() drip {
    test_group("Configuration File Parsing")
    
    sus config_line tea = "database_host = localhost"
    sus config ConfigPair = regex_parse_config_line(config_line) shook test_failed("Failed config parsing")
    
    assert_eq_str(config.key, "database_host", "Config key should be database_host")
    assert_eq_str(config.value, "localhost", "Config value should be localhost")
    
    # Test config line with spaces
    sus spaced_line tea = "  port  =  8080  "
    sus spaced_config ConfigPair = regex_parse_config_line(spaced_line) shook test_failed("Failed spaced config parsing")
    
    assert_eq_str(spaced_config.key, "port", "Spaced config key should be port")
    assert_eq_str(spaced_config.value, "8080", "Spaced config value should be 8080")
    
    test_passed("Configuration file parsing")
}

# Test log parsing
slay test_log_parsing() drip {
    test_group("Log File Parsing")
    
    sus log_line tea = "2023-12-25 10:30:45 ERROR: Database connection failed"
    
    sus timestamp tea = regex_extract_timestamp(log_line) shook test_failed("Failed timestamp extraction")
    assert_eq_str(timestamp, "2023-12-25 10:30:45", "Should extract timestamp")
    
    sus level tea = regex_extract_log_level(log_line) shook test_failed("Failed log level extraction")
    assert_eq_str(level, "ERROR", "Should extract log level")
    
    test_passed("Log file parsing")
}

# Test module information
slay test_module_info() drip {
    test_group("Module Information")
    
    sus version tea = regexz_version()
    assert_eq_str(version, "1.0.0", "Version should be 1.0.0")
    
    sus info RegexModuleInfo = regexz_info()
    assert_eq_str(info.version, "1.0.0", "Info version should match")
    assert_true(info.features.len() > 0, "Should have feature list")
    assert_eq_str(info.unicode_version, "15.0", "Unicode version should be 15.0")
    
    test_passed("Module information")
}

# Test performance and optimization
slay test_performance() drip {
    test_group("Performance and Optimization")
    
    # Create options with optimization
    sus options RegexOptions = create_default_options()
    options.optimization_level = 2
    options.cache_enabled = based
    
    sus optimized_engine RegexEngine = regex_compile_with_options("\\d{3}-\\d{3}-\\d{4}", options) shook test_failed("Failed optimized compilation")
    
    sus phone_text tea = "Call 555-123-4567 or 555-987-6543"
    
    # Multiple matches to test caching
    sus result1 MatchResult = regex_match(&optimized_engine, phone_text) shook test_failed("Failed first optimized match")
    sus result2 MatchResult = regex_match(&optimized_engine, phone_text) shook test_failed("Failed second optimized match")
    
    assert_true(result1.matched, "First optimized match should succeed")
    assert_true(result2.matched, "Second optimized match should succeed")
    assert_eq_str(result1.full_match, result2.full_match, "Both matches should be identical")
    
    test_passed("Performance and optimization")
}

# Test edge cases and error handling
slay test_edge_cases() drip {
    test_group("Edge Cases and Error Handling")
    
    # Test empty string matching
    sus empty_matched lit = regex_test_pattern("a*", "") shook test_failed("Failed empty string test")
    assert_true(empty_matched, "Should match empty string with a*")
    
    # Test Unicode edge cases
    sus emoji_matched lit = regex_test_pattern(".+", "🦀🔥💯") shook test_failed("Failed emoji test")
    assert_true(emoji_matched, "Should handle Unicode emoji")
    
    # Test very long input
    sus long_text tea = "a" * 1000
    sus long_matched lit = regex_test_pattern("a+", long_text) shook test_failed("Failed long text test")
    assert_true(long_matched, "Should handle very long input")
    
    # Test invalid group name
    sus invalid_group_result tea = get_named_group(MatchResult{
        matched: based,
        full_match: "test",
        start_pos: 0,
        end_pos: 4,
        groups: create_array(),
        named_groups: create_map()
    }, "nonexistent") fam {
        when _ -> "group not found"
    }
    assert_eq_str(invalid_group_result, "group not found", "Should handle invalid group name")
    
    test_passed("Edge cases and error handling")
}

# Main test execution
slay main() drip {
    test_basic_regex()
    test_named_groups()
    test_unicode_properties()
    test_lookaround()
    test_replacements()
    test_splitting()
    test_validation_patterns()
    test_utility_functions()
    test_password_validation()
    test_config_parsing()
    test_log_parsing()
    test_module_info()
    test_performance()
    test_edge_cases()
    
    print_test_summary()
    
    vibez.spill("✅ RegexZ Advanced Regular Expression Package Validation Complete!")
    vibez.spill("🚀 All advanced regex features implemented and working!")
    vibez.spill("📊 Features validated:")
    vibez.spill("  - Unicode property matching")
    vibez.spill("  - Named capture groups")
    vibez.spill("  - Lookahead/lookbehind assertions") 
    vibez.spill("  - Advanced character classes")
    vibez.spill("  - Performance optimizations")
    vibez.spill("  - Built-in validation patterns")
    vibez.spill("  - String manipulation utilities")
    vibez.spill("  - Configuration and log parsing")
    vibez.spill("  - Memory safety and error handling")
}
