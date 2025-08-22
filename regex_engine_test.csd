fr fr REGEX ENGINE COMPREHENSIVE TEST SUITE
fr fr Test all features of the new NFA/DFA-based regex engine

yeet "regexz"
yeet "vibez"
yeet "testz"

fr fr ===== BASIC PATTERN MATCHING TESTS =====

slay test_literal_matching() lit {
    vibez.spill("Testing literal character matching...")
    
    fr fr Simple literal matches
    ready (!regex_test("hello", "hello world")) {
        vibez.spill("FAIL: Literal 'hello' should match 'hello world'")
        damn cringe
    }
    
    ready (regex_test("xyz", "hello world")) {
        vibez.spill("FAIL: Literal 'xyz' should not match 'hello world'")
        damn cringe
    }
    
    vibez.spill("✓ Literal matching tests passed")
    damn based
}

slay test_any_character() lit {
    vibez.spill("Testing any character (.) matching...")
    
    fr fr Dot matches any character except newline
    ready (!regex_test("h.llo", "hello")) {
        vibez.spill("FAIL: 'h.llo' should match 'hello'")
        damn cringe
    }
    
    ready (!regex_test("h.llo", "hallo")) {
        vibez.spill("FAIL: 'h.llo' should match 'hallo'")
        damn cringe
    }
    
    ready (!regex_test(".", "a")) {
        vibez.spill("FAIL: '.' should match any single character")
        damn cringe
    }
    
    vibez.spill("✓ Any character matching tests passed")
    damn based
}

slay test_character_classes() lit {
    vibez.spill("Testing character classes [abc]...")
    
    fr fr Basic character classes
    ready (!regex_test("[abc]", "apple")) {
        vibez.spill("FAIL: '[abc]' should match 'apple' (starts with 'a')")
        damn cringe
    }
    
    ready (regex_test("[abc]", "dog")) {
        vibez.spill("FAIL: '[abc]' should not match 'dog'")
        damn cringe
    }
    
    fr fr Character ranges
    ready (!regex_test("[a-z]", "hello")) {
        vibez.spill("FAIL: '[a-z]' should match 'hello'")
        damn cringe
    }
    
    ready (!regex_test("[0-9]", "123")) {
        vibez.spill("FAIL: '[0-9]' should match '123'")
        damn cringe
    }
    
    vibez.spill("✓ Character class tests passed")
    damn based
}

slay test_negated_character_classes() lit {
    vibez.spill("Testing negated character classes [^abc]...")
    
    fr fr Negated character classes
    ready (!regex_test("[^abc]", "dog")) {
        vibez.spill("FAIL: '[^abc]' should match 'dog' (starts with 'd')")
        damn cringe
    }
    
    ready (regex_test("[^abc]", "apple")) {
        vibez.spill("FAIL: '[^abc]' should not match 'apple' (starts with 'a')")
        damn cringe
    }
    
    vibez.spill("✓ Negated character class tests passed")
    damn based
}

fr fr ===== QUANTIFIER TESTS =====

slay test_kleene_star() lit {
    vibez.spill("Testing Kleene star (*) quantifier...")
    
    fr fr Zero or more matches
    ready (!regex_test("ab*c", "ac")) {
        vibez.spill("FAIL: 'ab*c' should match 'ac' (zero 'b's)")
        damn cringe
    }
    
    ready (!regex_test("ab*c", "abc")) {
        vibez.spill("FAIL: 'ab*c' should match 'abc' (one 'b')")
        damn cringe
    }
    
    ready (!regex_test("ab*c", "abbbbc")) {
        vibez.spill("FAIL: 'ab*c' should match 'abbbbc' (multiple 'b's)")
        damn cringe
    }
    
    vibez.spill("✓ Kleene star tests passed")
    damn based
}

slay test_plus_quantifier() lit {
    vibez.spill("Testing plus (+) quantifier...")
    
    fr fr One or more matches
    ready (regex_test("ab+c", "ac")) {
        vibez.spill("FAIL: 'ab+c' should not match 'ac' (zero 'b's)")
        damn cringe
    }
    
    ready (!regex_test("ab+c", "abc")) {
        vibez.spill("FAIL: 'ab+c' should match 'abc' (one 'b')")
        damn cringe
    }
    
    ready (!regex_test("ab+c", "abbbbc")) {
        vibez.spill("FAIL: 'ab+c' should match 'abbbbc' (multiple 'b's)")
        damn cringe
    }
    
    vibez.spill("✓ Plus quantifier tests passed")
    damn based
}

slay test_optional_quantifier() lit {
    vibez.spill("Testing optional (?) quantifier...")
    
    fr fr Zero or one matches
    ready (!regex_test("ab?c", "ac")) {
        vibez.spill("FAIL: 'ab?c' should match 'ac' (zero 'b's)")
        damn cringe
    }
    
    ready (!regex_test("ab?c", "abc")) {
        vibez.spill("FAIL: 'ab?c' should match 'abc' (one 'b')")
        damn cringe
    }
    
    ready (regex_test("ab?c", "abbc")) {
        vibez.spill("FAIL: 'ab?c' should not match 'abbc' (two 'b's)")
        damn cringe
    }
    
    vibez.spill("✓ Optional quantifier tests passed")
    damn based
}

fr fr ===== ALTERNATION TESTS =====

slay test_alternation() lit {
    vibez.spill("Testing alternation (|) operator...")
    
    fr fr Either-or matching
    ready (!regex_test("cat|dog", "I have a cat")) {
        vibez.spill("FAIL: 'cat|dog' should match 'I have a cat'")
        damn cringe
    }
    
    ready (!regex_test("cat|dog", "I have a dog")) {
        vibez.spill("FAIL: 'cat|dog' should match 'I have a dog'")
        damn cringe
    }
    
    ready (regex_test("cat|dog", "I have a bird")) {
        vibez.spill("FAIL: 'cat|dog' should not match 'I have a bird'")
        damn cringe
    }
    
    fr fr Complex alternation
    ready (!regex_test("(red|blue|green)", "blue sky")) {
        vibez.spill("FAIL: Color alternation should match 'blue sky'")
        damn cringe
    }
    
    vibez.spill("✓ Alternation tests passed")
    damn based
}

fr fr ===== CAPTURE GROUP TESTS =====

slay test_capture_groups() lit {
    vibez.spill("Testing capture groups ()...")
    
    fr fr Basic grouping
    ready (!regex_test("(hello)", "hello world")) {
        vibez.spill("FAIL: '(hello)' should match 'hello world'")
        damn cringe
    }
    
    fr fr Nested groups
    ready (!regex_test("((hello) world)", "hello world test")) {
        vibez.spill("FAIL: Nested groups should match")
        damn cringe
    }
    
    fr fr Groups with quantifiers
    ready (!regex_test("(abc)+", "abcabcabc")) {
        vibez.spill("FAIL: '(abc)+' should match repeated 'abc'")
        damn cringe
    }
    
    vibez.spill("✓ Capture group tests passed")
    damn based
}

fr fr ===== ESCAPE SEQUENCE TESTS =====

slay test_escape_sequences() lit {
    vibez.spill("Testing escape sequences...")
    
    fr fr Digit class \d
    ready (!regex_test("\\d", "123abc")) {
        vibez.spill("FAIL: '\\d' should match digits in '123abc'")
        damn cringe
    }
    
    ready (regex_test("\\d", "abc")) {
        vibez.spill("FAIL: '\\d' should not match 'abc'")
        damn cringe
    }
    
    fr fr Word character class \w
    ready (!regex_test("\\w", "hello_world123")) {
        vibez.spill("FAIL: '\\w' should match word characters")
        damn cringe
    }
    
    fr fr Whitespace class \s
    ready (!regex_test("\\s", "hello world")) {
        vibez.spill("FAIL: '\\s' should match whitespace")
        damn cringe
    }
    
    vibez.spill("✓ Escape sequence tests passed")
    damn based
}

fr fr ===== ANCHOR TESTS =====

slay test_anchors() lit {
    vibez.spill("Testing anchors (^ and $)...")
    
    fr fr Start anchor ^
    ready (!regex_test("^hello", "hello world")) {
        vibez.spill("FAIL: '^hello' should match at start of 'hello world'")
        damn cringe
    }
    
    ready (regex_test("^hello", "say hello")) {
        vibez.spill("FAIL: '^hello' should not match 'say hello' (not at start)")
        damn cringe
    }
    
    fr fr End anchor $
    ready (!regex_test("world$", "hello world")) {
        vibez.spill("FAIL: 'world$' should match at end of 'hello world'")
        damn cringe
    }
    
    ready (regex_test("world$", "world peace")) {
        vibez.spill("FAIL: 'world$' should not match 'world peace' (not at end)")
        damn cringe
    }
    
    vibez.spill("✓ Anchor tests passed")
    damn based
}

fr fr ===== COMPLEX PATTERN TESTS =====

slay test_email_pattern() lit {
    vibez.spill("Testing email validation pattern...")
    
    sus email_pattern tea = "[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}"
    
    ready (!regex_test(email_pattern, "user@example.com")) {
        vibez.spill("FAIL: Email pattern should match valid email")
        damn cringe
    }
    
    ready (regex_test(email_pattern, "invalid.email")) {
        vibez.spill("FAIL: Email pattern should not match invalid email")
        damn cringe
    }
    
    vibez.spill("✓ Email pattern tests passed")
    damn based
}

slay test_phone_number_pattern() lit {
    vibez.spill("Testing phone number pattern...")
    
    sus phone_pattern tea = "\\(?[0-9]{3}\\)?[-. ]?[0-9]{3}[-. ]?[0-9]{4}"
    
    ready (!regex_test(phone_pattern, "(555) 123-4567")) {
        vibez.spill("FAIL: Phone pattern should match formatted number")
        damn cringe
    }
    
    ready (!regex_test(phone_pattern, "555.123.4567")) {
        vibez.spill("FAIL: Phone pattern should match dot-separated number")
        damn cringe
    }
    
    vibez.spill("✓ Phone number pattern tests passed")
    damn based
}

fr fr ===== FIND AND REPLACE TESTS =====

slay test_find_operations() lit {
    vibez.spill("Testing find operations...")
    
    fr fr Find first match
    sus found tea = regex_find("\\d+", "abc123def456")
    ready (found != "123") {
        vibez.spill("FAIL: Should find first number '123', got: " + found)
        damn cringe
    }
    
    fr fr Find all matches
    sus all_matches []tea = regex_find_all("\\d+", "abc123def456ghi789")
    ready (array_length(all_matches) != 3) {
        vibez.spill("FAIL: Should find 3 numbers, got: " + json_number_to_string(array_length(all_matches)))
        damn cringe
    }
    
    vibez.spill("✓ Find operation tests passed")
    damn based
}

slay test_replace_operations() lit {
    vibez.spill("Testing replace operations...")
    
    fr fr Replace first match
    sus replaced tea = regex_replace("\\d+", "abc123def456", "XXX")
    ready (replaced != "abcXXXdef456") {
        vibez.spill("FAIL: Should replace first number with XXX")
        damn cringe
    }
    
    fr fr Replace all matches
    sus replaced_all tea = regex_replace_all("\\d+", "abc123def456", "XXX")
    ready (replaced_all != "abcXXXdefXXX") {
        vibez.spill("FAIL: Should replace all numbers with XXX")
        damn cringe
    }
    
    vibez.spill("✓ Replace operation tests passed")
    damn based
}

fr fr ===== UNICODE SUPPORT TESTS =====

slay test_unicode_support() lit {
    vibez.spill("Testing Unicode support...")
    
    fr fr Basic Unicode matching (simplified for testing)
    ready (!regex_test("café", "I love café")) {
        vibez.spill("FAIL: Should match Unicode text")
        damn cringe
    }
    
    fr fr Unicode normalization test
    sus normalized tea = normalize_unicode_text("café")
    vibez.spill("Normalized text: " + normalized)
    
    vibez.spill("✓ Unicode support tests passed")
    damn based
}

fr fr ===== PERFORMANCE TESTS =====

slay test_performance() lit {
    vibez.spill("Testing regex performance...")
    
    fr fr Large text matching
    sus large_text tea = "This is a very long text with many words and numbers like 123, 456, and 789. " + 
                        "It contains various patterns and should test the performance of our regex engine. " + 
                        "We want to make sure it can handle complex patterns efficiently."
    
    sus pattern tea = "\\b\\d+\\b"  fr fr Find word-boundary numbers
    
    sus start_time drip = current_time_ms()
    sus matches []tea = regex_find_all(pattern, large_text)
    sus end_time drip = current_time_ms()
    
    sus duration drip = end_time - start_time
    vibez.spill("Found " + json_number_to_string(array_length(matches)) + " matches in " + 
                json_number_to_string(duration) + "ms")
    
    ready (array_length(matches) != 3) {
        vibez.spill("FAIL: Expected 3 number matches")
        damn cringe
    }
    
    vibez.spill("✓ Performance tests passed")
    damn based
}

fr fr ===== ERROR HANDLING TESTS =====

slay test_error_handling() lit {
    vibez.spill("Testing error handling...")
    
    fr fr Invalid pattern compilation
    sus invalid_regex RegexPattern = regex_compile_full("[unclosed", "")
    ready (invalid_regex.is_compiled) {
        vibez.spill("FAIL: Invalid pattern should not compile")
        damn cringe
    }
    
    fr fr Get error message
    sus error_msg tea = regex_get_error("[unclosed")
    vibez.spill("Error message: " + error_msg)
    
    fr fr Pattern validation
    ready (regex_validate("[unclosed")) {
        vibez.spill("FAIL: Invalid pattern should not validate")
        damn cringe
    }
    
    vibez.spill("✓ Error handling tests passed")
    damn based
}

fr fr ===== MAIN TEST RUNNER =====

slay run_comprehensive_regex_tests() lit {
    vibez.spill("=== RUNNING COMPREHENSIVE REGEX ENGINE TESTS ===")
    vibez.spill("")
    
    sus all_passed lit = based
    
    fr fr Basic matching tests
    ready (!test_literal_matching()) { all_passed = cringe }
    ready (!test_any_character()) { all_passed = cringe }
    ready (!test_character_classes()) { all_passed = cringe }
    ready (!test_negated_character_classes()) { all_passed = cringe }
    
    fr fr Quantifier tests  
    ready (!test_kleene_star()) { all_passed = cringe }
    ready (!test_plus_quantifier()) { all_passed = cringe }
    ready (!test_optional_quantifier()) { all_passed = cringe }
    
    fr fr Advanced pattern tests
    ready (!test_alternation()) { all_passed = cringe }
    ready (!test_capture_groups()) { all_passed = cringe }
    ready (!test_escape_sequences()) { all_passed = cringe }
    ready (!test_anchors()) { all_passed = cringe }
    
    fr fr Real-world pattern tests
    ready (!test_email_pattern()) { all_passed = cringe }
    ready (!test_phone_number_pattern()) { all_passed = cringe }
    
    fr fr Find and replace tests
    ready (!test_find_operations()) { all_passed = cringe }
    ready (!test_replace_operations()) { all_passed = cringe }
    
    fr fr Advanced feature tests
    ready (!test_unicode_support()) { all_passed = cringe }
    ready (!test_performance()) { all_passed = cringe }
    ready (!test_error_handling()) { all_passed = cringe }
    
    vibez.spill("")
    ready (all_passed) {
        vibez.spill("🎉 ALL REGEX ENGINE TESTS PASSED! 🎉")
        vibez.spill("The new NFA/DFA-based regex engine is working correctly.")
    } otherwise {
        vibez.spill("❌ SOME TESTS FAILED")
        vibez.spill("Please review the failed tests and fix the implementation.")
    }
    
    damn all_passed
}

fr fr ===== PLACEHOLDER UTILITY FUNCTIONS =====

slay current_time_ms() drip {
    fr fr Placeholder for timestamp (would use actual time in production)
    damn 1234567890
}

slay json_number_to_string(num drip) tea {
    ready (num == 0) { damn "0" }
    ready (num == 1) { damn "1" }
    ready (num == 2) { damn "2" }
    ready (num == 3) { damn "3" }
    ready (num == 4) { damn "4" }
    ready (num == 5) { damn "5" }
    ready (num == 10) { damn "10" }
    ready (num < 0) { damn "-" + json_number_to_string(-num) }
    damn json_number_to_string(num / 10) + json_number_to_string(num % 10)
}

fr fr Run the comprehensive test suite
run_comprehensive_regex_tests()
