fr fr Advanced Regex Engine Demonstration
fr fr Showcases complete regex functionality with real-world examples

yeet "regexz"
yeet "vibez"

slay main() drip {
    vibez.spill("=== CURSED ADVANCED REGEX ENGINE DEMO ===")
    vibez.spill("")
    
    fr fr Demonstrate basic functionality
    demo_basic_matching()
    
    fr fr Show advanced quantifiers
    demo_quantifiers()
    
    fr fr Character classes and Unicode
    demo_character_classes()
    
    fr fr Groups and capturing
    demo_groups()
    
    fr fr Practical applications
    demo_real_world_patterns()
    
    fr fr Advanced features
    demo_advanced_features()
    
    vibez.spill("")
    vibez.spill("=== DEMO COMPLETE ===")
}

slay demo_basic_matching() {
    vibez.spill("--- Basic Pattern Matching ---")
    
    fr fr Simple literal matching
    demo_match("hello", "hello world", "Simple literal match")
    demo_match("world", "hello world", "Find substring")
    
    fr fr Case sensitivity
    demo_match("Hello", "hello world", "Case sensitive (should fail)")
    demo_match_with_flags("Hello", "hello world", "i", "Case insensitive")
    
    fr fr Wildcard matching
    demo_match("h.llo", "hello", "Dot matches any character")
    demo_match("c.t", "cat", "Wildcard in word")
    demo_match("c.t", "cut", "Different wildcard match")
    
    vibez.spill("")
}

slay demo_quantifiers() {
    vibez.spill("--- Quantifiers ---")
    
    fr fr Kleene star: zero or more
    demo_match("ab*c", "ac", "Star: zero matches")
    demo_match("ab*c", "abc", "Star: one match")
    demo_match("ab*c", "abbbbc", "Star: multiple matches")
    
    fr fr Plus: one or more
    demo_match("ab+c", "abc", "Plus: one match")
    demo_match("ab+c", "abbbbc", "Plus: multiple matches")
    demo_match("ab+c", "ac", "Plus: requires at least one (fails)")
    
    fr fr Question: zero or one
    demo_match("colou?r", "color", "Optional 'u' - without")
    demo_match("colou?r", "colour", "Optional 'u' - with")
    
    fr fr Practical quantifier usage
    demo_match("\\d+", "abc123def", "Find digits")
    demo_match("\\w+@\\w+\\.\\w+", "user@domain.com", "Simple email pattern")
    
    vibez.spill("")
}

slay demo_character_classes() {
    vibez.spill("--- Character Classes ---")
    
    fr fr Basic character classes
    demo_match("[aeiou]", "hello", "Find vowel")
    demo_match("[0-9]", "abc5def", "Find digit in range")
    demo_match("[a-zA-Z]", "Hello123", "Find letter")
    
    fr fr Negated classes
    demo_match("[^0-9]", "123abc", "Find non-digit")
    demo_match("[^aeiou]", "hello", "Find consonant")
    
    fr fr Predefined classes
    demo_match("\\d+", "Price: $25.50", "Find numbers")
    demo_match("\\w+", "hello-world", "Find word characters")
    demo_match("\\s+", "hello   world", "Find whitespace")
    
    fr fr Unicode character classes (with Unicode flag)
    demo_match_with_flags("\\p{L}+", "café", "u", "Unicode letters")
    demo_match_with_flags("\\p{N}+", "123", "u", "Unicode numbers")
    demo_match_with_flags("\\p{P}", "Hello!", "u", "Unicode punctuation")
    
    vibez.spill("")
}

slay demo_groups() {
    vibez.spill("--- Groups and Capturing ---")
    
    fr fr Basic capturing groups
    demo_extract("(\\w+) (\\w+)", "John Doe", "Extract first and last name")
    demo_extract("(\\d{4})-(\\d{2})-(\\d{2})", "2024-08-24", "Extract date parts")
    
    fr fr Non-capturing groups
    demo_match("(?:Mr|Ms|Dr)\\. \\w+", "Dr. Smith", "Non-capturing group for titles")
    
    fr fr Named capturing groups (if supported)
    demo_extract("(?<year>\\d{4})-(?<month>\\d{2})-(?<day>\\d{2})", 
                 "2024-08-24", "Named groups for date")
    
    fr fr Backreferences
    demo_match("(\\w+) \\1", "hello hello", "Backreference - repeated word")
    demo_match("(\\w+) \\1", "hello world", "Backreference - different words (fails)")
    
    vibez.spill("")
}

slay demo_real_world_patterns() {
    vibez.spill("--- Real-World Pattern Examples ---")
    
    fr fr Email validation
    sus email_pattern tea = "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"
    demo_match(email_pattern, "user@example.com", "Valid email")
    demo_match(email_pattern, "invalid.email", "Invalid email (fails)")
    demo_match(email_pattern, "test.user+tag@domain.co.uk", "Complex valid email")
    
    fr fr URL validation
    sus url_pattern tea = "^https?://[a-zA-Z0-9.-]+(?:\\.[a-zA-Z]{2,})+(?:/.*)?$"
    demo_match(url_pattern, "https://www.example.com", "HTTPS URL")
    demo_match(url_pattern, "http://subdomain.example.org/path", "HTTP with path")
    
    fr fr Phone number extraction
    sus phone_pattern tea = "\\(?([0-9]{3})\\)?[-. ]?([0-9]{3})[-. ]?([0-9]{4})"
    demo_extract(phone_pattern, "Call me at (555) 123-4567", "Extract phone number")
    demo_extract(phone_pattern, "Phone: 555.123.4567", "Different format")
    
    fr fr IPv4 address
    sus ip_pattern tea = "\\b(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\\b"
    demo_match(ip_pattern, "Server IP: 192.168.1.1", "Valid IP address")
    demo_match(ip_pattern, "Invalid: 256.1.1.1", "Invalid IP (fails)")
    
    fr fr HTML tag removal
    sus html_pattern tea = "<[^>]*>"
    demo_replace(html_pattern, "<p>Hello <b>world</b>!</p>", "", "Remove HTML tags")
    
    fr fr Extract hashtags
    sus hashtag_pattern tea = "#[a-zA-Z0-9_]+"
    demo_extract_all(hashtag_pattern, "Love #coding and #regex! #CURSED", "Extract hashtags")
    
    vibez.spill("")
}

slay demo_advanced_features() {
    vibez.spill("--- Advanced Features ---")
    
    fr fr Word boundaries
    demo_match("\\bword\\b", "a word here", "Word boundary match")
    demo_match("\\bword\\b", "wordy", "Word boundary (fails in compound)")
    
    fr fr Anchors
    demo_match("^Start", "Start of line", "Start anchor")
    demo_match("^Start", "Not start", "Start anchor (fails)")
    demo_match("end$", "At the end", "End anchor")
    demo_match("end$", "end of middle", "End anchor (fails)")
    
    fr fr Lookahead assertions
    demo_match("\\w+(?=\\s)", "hello world", "Positive lookahead")
    demo_match("\\w+(?!\\d)", "hello123", "Negative lookahead (fails)")
    demo_match("\\w+(?!\\d)", "hello world", "Negative lookahead")
    
    fr fr Complex patterns with multiple features
    sus complex_pattern tea = "^(?=.*[a-z])(?=.*[A-Z])(?=.*\\d)(?=.*[!@#$%^&*]).{8,}$"
    demo_match(complex_pattern, "MyPass123!", "Strong password validation")
    demo_match(complex_pattern, "weakpass", "Weak password (fails)")
    
    fr fr Alternation with groups
    demo_match("(cat|dog|bird) (lover|owner)", "dog owner", "Combined alternation")
    
    vibez.spill("")
}

fr fr Helper functions for demonstrations

slay demo_match(pattern tea, text tea, description tea) {
    vibez.spill("Testing: " + description)
    vibez.spill("Pattern: " + pattern)
    vibez.spill("Text: " + text)
    
    sus result lit = regex_test(pattern, text) fam {
        when error -> {
            vibez.spill("Result: ERROR - " + error)
            damn
        }
    }
    
    vibez.spill("Result: " + (result ? "✓ MATCH" : "✗ NO MATCH"))
    vibez.spill("")
}

slay demo_match_with_flags(pattern tea, text tea, flags tea, description tea) {
    vibez.spill("Testing: " + description + " (flags: " + flags + ")")
    vibez.spill("Pattern: " + pattern)
    vibez.spill("Text: " + text)
    
    fr fr Create options based on flags
    sus options RegexOptions = create_default_options()
    ready (string_contains(flags, "i")) {
        options.case_insensitive = based
    }
    ready (string_contains(flags, "u")) {
        options.unicode_support = based
    }
    
    sus engine RegexEngine = regex_new_with_options(pattern, options) fam {
        when error -> {
            vibez.spill("Result: ERROR - " + error)
            damn
        }
    }
    
    sus match_result MatchResult = regex_match(&engine, text) fam {
        when error -> {
            vibez.spill("Result: ERROR - " + error)
            damn
        }
    }
    
    vibez.spill("Result: " + (match_result.matched ? "✓ MATCH" : "✗ NO MATCH"))
    vibez.spill("")
}

slay demo_extract(pattern tea, text tea, description tea) {
    vibez.spill("Extracting: " + description)
    vibez.spill("Pattern: " + pattern)
    vibez.spill("Text: " + text)
    
    sus engine RegexEngine = regex_new(pattern) fam {
        when error -> {
            vibez.spill("Result: ERROR - " + error)
            damn
        }
    }
    
    sus match_result MatchResult = regex_match(&engine, text) fam {
        when error -> {
            vibez.spill("Result: ERROR - " + error)
            damn
        }
    }
    
    ready (match_result.matched) {
        vibez.spill("✓ Full match: " + match_result.full_match)
        sus i drip = 0
        bestie (i < array_length(match_result.groups)) {
            vibez.spill("  Group " + json_number_to_string(i + 1) + ": " + 
                       match_result.groups[i].value)
            i = i + 1
        }
    } otherwise {
        vibez.spill("✗ NO MATCH")
    }
    vibez.spill("")
}

slay demo_replace(pattern tea, text tea, replacement tea, description tea) {
    vibez.spill("Replacing: " + description)
    vibez.spill("Pattern: " + pattern)
    vibez.spill("Text: " + text)
    vibez.spill("Replacement: " + replacement)
    
    sus result tea = regex_replace_simple(pattern, text, replacement) fam {
        when error -> {
            vibez.spill("Result: ERROR - " + error)
            damn
        }
    }
    
    vibez.spill("Result: " + result)
    vibez.spill("")
}

slay demo_extract_all(pattern tea, text tea, description tea) {
    vibez.spill("Extracting all: " + description)
    vibez.spill("Pattern: " + pattern)
    vibez.spill("Text: " + text)
    
    sus matches []tea = regex_extract_all(pattern, text) fam {
        when error -> {
            vibez.spill("Result: ERROR - " + error)
            damn
        }
    }
    
    vibez.spill("Found " + json_number_to_string(array_length(matches)) + " matches:")
    sus i drip = 0
    bestie (i < array_length(matches)) {
        vibez.spill("  " + json_number_to_string(i + 1) + ": " + matches[i])
        i = i + 1
    }
    vibez.spill("")
}

fr fr Run the demonstration
main()
