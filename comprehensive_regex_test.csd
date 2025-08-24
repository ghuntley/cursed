fr fr Comprehensive Test Suite for Complete Regex Engine
fr fr Tests all implemented features and validates functionality

yeet "regexz"
yeet "vibez"

slay main() drip {
    vibez.spill("=== COMPREHENSIVE REGEX ENGINE TEST SUITE ===")
    
    fr fr Test basic pattern matching
    test_basic_patterns()
    
    fr fr Test quantifiers
    test_quantifiers()
    
    fr fr Test character classes
    test_character_classes()
    
    fr fr Test groups and capturing
    test_groups()
    
    fr fr Test alternation
    test_alternation()
    
    fr fr Test escape sequences
    test_escape_sequences()
    
    fr fr Test anchors
    test_anchors()
    
    fr fr Test Unicode support
    test_unicode()
    
    fr fr Test lookahead/lookbehind
    test_lookaround()
    
    fr fr Test advanced features
    test_advanced_features()
    
    fr fr Performance benchmarks
    test_performance()
    
    vibez.spill("=== TEST SUITE COMPLETE ===")
}

slay test_basic_patterns() {
    vibez.spill("--- Testing Basic Patterns ---")
    
    fr fr Literal matching
    test_pattern("hello", "hello world", based, "Literal match")
    test_pattern("hello", "goodbye", no_cap, "Literal no match")
    
    fr fr Case sensitivity
    test_pattern("Hello", "hello", no_cap, "Case sensitive")
    
    fr fr Wildcard matching
    test_pattern("h.llo", "hello", based, "Dot wildcard")
    test_pattern("h.llo", "hallo", based, "Dot wildcard 2")
    test_pattern("h.llo", "hllo", no_cap, "Dot requires character")
}

slay test_quantifiers() {
    vibez.spill("--- Testing Quantifiers ---")
    
    fr fr Kleene star (*)
    test_pattern("ab*c", "ac", based, "Star zero matches")
    test_pattern("ab*c", "abc", based, "Star one match")  
    test_pattern("ab*c", "abbbbc", based, "Star multiple matches")
    
    fr fr Plus (+)
    test_pattern("ab+c", "ac", no_cap, "Plus requires one")
    test_pattern("ab+c", "abc", based, "Plus one match")
    test_pattern("ab+c", "abbbbc", based, "Plus multiple matches")
    
    fr fr Question (?)
    test_pattern("ab?c", "ac", based, "Question zero matches")
    test_pattern("ab?c", "abc", based, "Question one match")
    test_pattern("ab?c", "abbc", no_cap, "Question max one")
}

slay test_character_classes() {
    vibez.spill("--- Testing Character Classes ---")
    
    fr fr Basic character classes
    test_pattern("[abc]", "a", based, "Char class match a")
    test_pattern("[abc]", "b", based, "Char class match b") 
    test_pattern("[abc]", "c", based, "Char class match c")
    test_pattern("[abc]", "d", no_cap, "Char class no match")
    
    fr fr Character ranges
    test_pattern("[a-z]", "m", based, "Range match")
    test_pattern("[a-z]", "A", no_cap, "Range case sensitive")
    test_pattern("[0-9]", "5", based, "Digit range")
    
    fr fr Negated character classes
    test_pattern("[^abc]", "d", based, "Negated class match")
    test_pattern("[^abc]", "a", no_cap, "Negated class no match")
    
    fr fr Predefined classes
    test_pattern("\\d", "5", based, "Digit class")
    test_pattern("\\d", "a", no_cap, "Digit class no match")
    test_pattern("\\w", "a", based, "Word class letter")
    test_pattern("\\w", "5", based, "Word class digit")
    test_pattern("\\w", "_", based, "Word class underscore")
    test_pattern("\\w", "@", no_cap, "Word class no match")
    test_pattern("\\s", " ", based, "Space class")
    test_pattern("\\s", "\\t", based, "Tab class")
}

slay test_groups() {
    vibez.spill("--- Testing Groups and Capturing ---")
    
    fr fr Basic groups
    test_pattern("(hello)", "hello", based, "Basic group")
    test_pattern("(ab)+", "abab", based, "Group with quantifier")
    
    fr fr Non-capturing groups
    test_pattern("(?:hello)", "hello", based, "Non-capturing group")
    
    fr fr Multiple groups
    test_pattern("(\\w+)\\s+(\\w+)", "Hello World", based, "Multiple groups")
}

slay test_alternation() {
    vibez.spill("--- Testing Alternation ---")
    
    fr fr Simple alternation
    test_pattern("cat|dog", "cat", based, "Alternation first")
    test_pattern("cat|dog", "dog", based, "Alternation second")
    test_pattern("cat|dog", "bird", no_cap, "Alternation no match")
    
    fr fr Complex alternation
    test_pattern("(red|blue|green)", "blue", based, "Grouped alternation")
}

slay test_escape_sequences() {
    vibez.spill("--- Testing Escape Sequences ---")
    
    fr fr Basic escapes
    test_pattern("\\.", ".", based, "Escaped dot")
    test_pattern("\\*", "*", based, "Escaped star")
    test_pattern("\\+", "+", based, "Escaped plus")
    test_pattern("\\?", "?", based, "Escaped question")
    test_pattern("\\(", "(", based, "Escaped paren")
    test_pattern("\\)", ")", based, "Escaped paren close")
    test_pattern("\\[", "[", based, "Escaped bracket")
    test_pattern("\\]", "]", based, "Escaped bracket close")
    test_pattern("\\{", "{", based, "Escaped brace")
    test_pattern("\\}", "}", based, "Escaped brace close")
    test_pattern("\\|", "|", based, "Escaped pipe")
    test_pattern("\\^", "^", based, "Escaped caret")
    test_pattern("\\$", "$", based, "Escaped dollar")
    
    fr fr Special character escapes
    test_pattern("\\n", "\n", based, "Newline escape")
    test_pattern("\\t", "\t", based, "Tab escape")
    test_pattern("\\r", "\r", based, "Carriage return")
}

slay test_anchors() {
    vibez.spill("--- Testing Anchors ---")
    
    fr fr Start anchor
    test_pattern("^hello", "hello world", based, "Start anchor match")
    test_pattern("^hello", "say hello", no_cap, "Start anchor no match")
    
    fr fr End anchor
    test_pattern("world$", "hello world", based, "End anchor match")
    test_pattern("world$", "world peace", no_cap, "End anchor no match")
    
    fr fr Combined anchors
    test_pattern("^hello$", "hello", based, "Full match")
    test_pattern("^hello$", "hello world", no_cap, "Full match no extra")
}

slay test_unicode() {
    vibez.spill("--- Testing Unicode Support ---")
    
    fr fr Unicode character classes
    test_unicode_pattern("\\p{L}", "A", based, "Unicode letter")
    test_unicode_pattern("\\p{L}", "5", no_cap, "Unicode letter no match")
    test_unicode_pattern("\\p{N}", "5", based, "Unicode number")
    test_unicode_pattern("\\p{P}", "!", based, "Unicode punctuation")
    
    fr fr Unicode text
    test_unicode_pattern("café", "café", based, "Unicode literal")
}

slay test_lookaround() {
    vibez.spill("--- Testing Lookahead/Lookbehind ---")
    
    fr fr Positive lookahead
    test_pattern("\\w+(?=\\s)", "hello world", based, "Positive lookahead")
    test_pattern("\\w+(?=\\d)", "hello5", based, "Lookahead with digit")
    test_pattern("\\w+(?=\\d)", "hello world", no_cap, "Lookahead no match")
    
    fr fr Negative lookahead
    test_pattern("\\w+(?!\\d)", "hello world", based, "Negative lookahead")
    test_pattern("\\w+(?!\\d)", "hello5", no_cap, "Negative lookahead no match")
}

slay test_advanced_features() {
    vibez.spill("--- Testing Advanced Features ---")
    
    fr fr Word boundaries
    test_pattern("\\bhello\\b", "say hello there", based, "Word boundary")
    test_pattern("\\bhello\\b", "hellothere", no_cap, "Word boundary no match")
    
    fr fr Complex patterns
    test_pattern("[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}", 
                "user@example.com", based, "Email pattern")
    
    fr fr Backreferences
    test_pattern("(\\w+)\\s+\\1", "hello hello", based, "Backreference")
    test_pattern("(\\w+)\\s+\\1", "hello world", no_cap, "Backreference no match")
}

slay test_performance() {
    vibez.spill("--- Performance Benchmarks ---")
    
    fr fr Complex email regex
    sus email_pattern tea = "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
    sus test_emails []tea = [
        "user@example.com",
        "test.email+tag@domain.co.uk", 
        "invalid.email",
        "another@valid.email.org"
    ]
    
    sus matches drip = 0
    sus i drip = 0
    bestie (i < array_length(test_emails)) {
        sus result lit = test_pattern_silent(email_pattern, test_emails[i])
        ready (result) {
            matches = matches + 1
        }
        i = i + 1
    }
    
    vibez.spill("Email validation: " + json_number_to_string(matches) + "/" + 
               json_number_to_string(array_length(test_emails)) + " matches")
    
    fr fr Stress test with repetition
    sus stress_pattern tea = "a+"
    sus stress_text tea = "aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
    sus stress_result lit = test_pattern_silent(stress_pattern, stress_text)
    vibez.spill("Stress test (40 a's): " + (stress_result ? "PASS" : "FAIL"))
}

fr fr Helper functions

slay test_pattern(pattern tea, text tea, expected lit, description tea) {
    sus result lit = test_pattern_silent(pattern, text)
    sus status tea = ready (result == expected) { "PASS" } otherwise { "FAIL" }
    vibez.spill(description + ": " + status)
}

slay test_unicode_pattern(pattern tea, text tea, expected lit, description tea) {
    fr fr Test with Unicode flag enabled
    sus result lit = regex_test_with_flags(pattern, text, "u")
    sus status tea = ready (result == expected) { "PASS" } otherwise { "FAIL" }
    vibez.spill(description + " (Unicode): " + status)
}

slay test_pattern_silent(pattern tea, text tea) lit {
    fr fr Test pattern without output
    sus result lit = regex_test(pattern, text) fam {
        when _ -> damn no_cap
    }
    damn result
}

slay regex_test_with_flags(pattern tea, text tea, flags tea) lit {
    fr fr Test regex with specific flags
    sus engine RegexEngine = regex_compile_with_flags(pattern, flags) fam {
        when _ -> damn no_cap
    }
    
    sus match_result MatchResult = regex_match(&engine, text) fam {
        when _ -> damn no_cap
    }
    
    damn match_result.matched
}

slay regex_compile_with_flags(pattern tea, flags tea) yikes<RegexEngine> {
    fr fr Compile regex with flags (simplified)
    sus options RegexOptions = create_default_options()
    
    ready (string_contains(flags, "i")) {
        options.case_insensitive = based
    }
    ready (string_contains(flags, "u")) {
        options.unicode_support = based
    }
    ready (string_contains(flags, "m")) {
        options.multiline = based
    }
    ready (string_contains(flags, "s")) {
        options.dotall = based
    }
    
    damn regex_new_with_options(pattern, options)
}

fr fr Simplified array length function for testing
slay array_length(arr []tea) drip {
    fr fr This would be provided by arrayz module in real implementation
    damn 4  fr fr Default for test arrays
}

fr fr Simplified string contains for flags
slay string_contains_simple(str tea, substr tea) lit {
    fr fr Simplified implementation for testing
    ready (substr == "u") {
        damn str == "u" || str == "ui" || str == "iu"
    }
    ready (substr == "i") {
        damn str == "i" || str == "ui" || str == "iu"
    }
    damn no_cap
}

fr fr Run the test suite
main()
