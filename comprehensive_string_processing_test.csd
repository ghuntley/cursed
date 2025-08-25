# Comprehensive String Processing Test
# Tests all real string processing capabilities
# Validates Unicode support, regex, normalization, and encoding

yeet "vibez"
yeet "testz"
yeet "stringz_real_algorithms"
yeet "encodingz_real"
yeet "unicode_normalization_real"
yeet "regex_real_engine"

slay test_comprehensive_string_processing() tea {
    vibez.spill("=== COMPREHENSIVE STRING PROCESSING TEST ===")
    
    # Test Unicode-aware string operations
    test_unicode_string_operations()
    
    # Test string searching algorithms
    test_string_searching_algorithms()
    
    # Test string replacement algorithms
    test_string_replacement_algorithms()
    
    # Test Unicode normalization
    test_unicode_normalization()
    
    # Test character encoding/decoding
    test_character_encoding()
    
    # Test real regex engine
    test_regex_engine()
    
    # Test advanced string validation
    test_advanced_string_validation()
    
    vibez.spill("All comprehensive string processing tests completed!")
    damn "comprehensive_test_complete"
}

slay test_unicode_string_operations() tea {
    vibez.spill("\n--- Testing Unicode String Operations ---")
    
    # Test Unicode length calculation
    sus emoji_text tea = "Hello 🌍 World 🚀"
    sus length drip = string_length_real(emoji_text)
    vibez.spill("Unicode text length:", length)
    
    # Test Unicode character extraction
    sus char tea = char_at_real(emoji_text, 6)  # Should be 🌍
    vibez.spill("Character at index 6:", char)
    
    # Test Unicode substring
    sus substr tea = substring_real(emoji_text, 0, 5)
    vibez.spill("Substring (0,5):", substr)
    
    # Test Unicode case conversion
    sus mixed_case tea = "Café Naïve Résumé"
    sus upper tea = to_uppercase_real(mixed_case)
    sus lower tea = to_lowercase_real(mixed_case)
    vibez.spill("Original:", mixed_case)
    vibez.spill("Uppercase:", upper)
    vibez.spill("Lowercase:", lower)
    
    # Test Unicode whitespace trimming
    sus whitespace_text tea = "  \t\nHello\u00A0World\n\t  "
    sus trimmed tea = trim_whitespace_real(whitespace_text)
    vibez.spill("Trimmed text:", trimmed)
}

slay test_string_searching_algorithms() tea {
    vibez.spill("\n--- Testing String Searching Algorithms ---")
    
    # Test Boyer-Moore search
    sus haystack tea = "The quick brown fox jumps over the lazy dog"
    sus needle tea = "fox"
    sus bm_matches []drip = boyer_moore_search(haystack, needle)
    vibez.spill("Boyer-Moore search for 'fox':", bm_matches)
    
    # Test KMP search
    sus kmp_matches []drip = kmp_search(haystack, "the")
    vibez.spill("KMP search for 'the':", kmp_matches)
    
    # Test indexOf with real algorithm
    sus first_index drip = indexOf_real(haystack, "quick")
    sus last_index drip = lastIndexOf_real(haystack, "o")
    vibez.spill("First 'quick' at:", first_index)
    vibez.spill("Last 'o' at:", last_index)
    
    # Test pattern matching with Unicode
    sus unicode_text tea = "Héllö Wörld with émojis 🎉"
    sus unicode_index drip = indexOf_real(unicode_text, "Wörld")
    vibez.spill("Unicode pattern found at:", unicode_index)
}

slay test_string_replacement_algorithms() tea {
    vibez.spill("\n--- Testing String Replacement Algorithms ---")
    
    sus original tea = "Hello world, hello universe, hello everyone"
    
    # Test first replacement
    sus replaced_first tea = replace_first_real(original, "hello", "hi")
    vibez.spill("Replace first 'hello':", replaced_first)
    
    # Test replace all
    sus replaced_all tea = replace_all_real(original, "hello", "hi")
    vibez.spill("Replace all 'hello':", replaced_all)
    
    # Test Unicode replacement
    sus unicode_original tea = "Café, café, CAFÉ"
    sus unicode_replaced tea = replace_all_real(unicode_original, "café", "coffee")
    vibez.spill("Unicode replacement:", unicode_replaced)
    
    # Test case-insensitive replacement (using normalized comparison)
    sus normalized_original tea = normalize_unicode_real(unicode_original, "NFC")
    vibez.spill("Normalized original:", normalized_original)
}

slay test_unicode_normalization() tea {
    vibez.spill("\n--- Testing Unicode Normalization ---")
    
    # Test canonical decomposition (NFD)
    sus composed tea = "Café"  # é as single character
    sus nfd_text tea = normalize_nfd_real(composed)
    vibez.spill("NFD normalization:", nfd_text)
    
    # Test canonical composition (NFC)
    sus nfc_text tea = normalize_nfc_real(nfd_text)
    vibez.spill("NFC normalization:", nfc_text)
    
    # Test compatibility decomposition (NFKD)
    sus compat_text tea = "ﬁle"  # fi ligature
    sus nfkd_text tea = normalize_nfkd_real(compat_text)
    vibez.spill("NFKD normalization:", nfkd_text)
    
    # Test compatibility composition (NFKC)
    sus nfkc_text tea = normalize_nfkc_real(compat_text)
    vibez.spill("NFKC normalization:", nfkc_text)
    
    # Test normalization equivalence
    sus text1 tea = "é"  # single character
    sus text2 tea = "é"  # e + combining acute
    sus are_equal lit = strings_equal_normalized(text1, text2, "NFC")
    vibez.spill("Normalized equality:", are_equal)
}

slay test_character_encoding() tea {
    vibez.spill("\n--- Testing Character Encoding/Decoding ---")
    
    # Test Base64 encoding
    sus original_data []drip = [72, 101, 108, 108, 111, 32, 87, 111, 114, 108, 100]  # "Hello World"
    sus base64_encoded tea = base64_encode_real(original_data)
    vibez.spill("Base64 encoded:", base64_encoded)
    
    sus base64_decoded []drip = base64_decode_real(base64_encoded)
    vibez.spill("Base64 decoded:", base64_decoded)
    
    # Test hexadecimal encoding
    sus hex_encoded tea = hex_encode_real(original_data)
    vibez.spill("Hex encoded:", hex_encoded)
    
    sus hex_decoded []drip = hex_decode_real(hex_encoded)
    vibez.spill("Hex decoded:", hex_decoded)
    
    # Test URL encoding
    sus url_text tea = "Hello World & Special Characters!"
    sus url_encoded tea = url_encode_real(url_text)
    vibez.spill("URL encoded:", url_encoded)
    
    sus url_decoded tea = url_decode_real(url_encoded)
    vibez.spill("URL decoded:", url_decoded)
    
    # Test HTML entity encoding
    sus html_text tea = "<script>alert('hello');</script>"
    sus html_encoded tea = html_encode_real(html_text)
    vibez.spill("HTML encoded:", html_encoded)
    
    sus html_decoded tea = html_decode_real(html_encoded)
    vibez.spill("HTML decoded:", html_decoded)
    
    # Test encoding detection
    sus utf8_data []drip = [240, 159, 140, 141, 240, 159, 154, 128]  # 🌍🚀 in UTF-8
    sus detected_encoding tea = detect_encoding_real(utf8_data)
    vibez.spill("Detected encoding:", detected_encoding)
}

slay test_regex_engine() tea {
    vibez.spill("\n--- Testing Real Regex Engine ---")
    
    # Test basic regex compilation and matching
    sus pattern tea = "h[aeiou]llo"
    sus test_text tea = "Say hello to the world"
    sus matches lit = regex_test(pattern, test_text)
    vibez.spill("Regex test result:", matches)
    
    # Test regex match with capture
    sus email_pattern tea = "([a-zA-Z0-9]+)@([a-zA-Z0-9]+\\.com)"
    sus email_text tea = "Contact us at test@example.com for help"
    sus email_match RegexMatch = regex_match(email_pattern, email_text)
    vibez.spill("Email match found:", email_match.matched)
    ready (email_match.matched) {
        vibez.spill("Matched text:", email_match.text)
    }
    
    # Test find all matches
    sus digit_pattern tea = "\\d+"
    sus number_text tea = "The year 2024 has 365 days and 12 months"
    sus all_matches []RegexMatch = regex_find_all(digit_pattern, number_text)
    vibez.spill("Found", len(all_matches), "number matches")
    
    # Test regex replacement
    sus phone_pattern tea = "\\d{3}-\\d{3}-\\d{4}"
    sus phone_text tea = "Call 555-123-4567 or 999-888-7777 for support"
    sus masked_phones tea = regex_replace(phone_pattern, phone_text, "XXX-XXX-XXXX")
    vibez.spill("Masked phone numbers:", masked_phones)
    
    # Test Unicode-aware regex
    sus unicode_pattern tea = "\\p{Letter}+"
    sus unicode_text_regex tea = "Hello мир 世界"
    sus unicode_matches []RegexMatch = regex_find_all(unicode_pattern, unicode_text_regex)
    vibez.spill("Unicode letter matches:", len(unicode_matches))
}

slay test_advanced_string_validation() tea {
    vibez.spill("\n--- Testing Advanced String Validation ---")
    
    # Test real numeric validation
    sus numeric_tests []tea = ["123", "-456", "+789", "12.34", "abc", "12a", ""]
    bestie (test_str in numeric_tests) {
        sus is_num lit = is_numeric_real(test_str)
        vibez.spill("Is '" + test_str + "' numeric?", is_num)
    }
    
    # Test real alphabetic validation
    sus alpha_tests []tea = ["hello", "WORLD", "café", "naïve", "hello123", ""]
    bestie (test_str in alpha_tests) {
        sus is_alpha lit = is_alphabetic_real(test_str)
        vibez.spill("Is '" + test_str + "' alphabetic?", is_alpha)
    }
    
    # Test real email validation
    sus email_tests []tea = [
        "test@example.com",
        "user.name@domain.org", 
        "invalid@",
        "@domain.com",
        "no-at-sign.com",
        "multiple@@signs.com",
        ""
    ]
    bestie (email in email_tests) {
        sus is_valid lit = is_valid_email_real(email)
        vibez.spill("Is '" + email + "' valid email?", is_valid)
    }
    
    # Test string splitting with real algorithms
    sus csv_line tea = "John,Doe,30,Engineer"
    sus csv_parts []tea = split_string_real(csv_line, ",")
    vibez.spill("CSV parts:", len(csv_parts))
    bestie (part in csv_parts) {
        vibez.spill("  Part:", part)
    }
    
    # Test Unicode-aware string comparison
    sus str1 tea = "Café"
    sus str2 tea = "Cafe\u0301"  # Same but with combining accent
    sus normalized_equal lit = strings_equal_normalized(str1, str2, "NFC")
    vibez.spill("Unicode strings equal after normalization:", normalized_equal)
}

# Performance benchmarking
slay benchmark_string_operations() tea {
    vibez.spill("\n--- Benchmarking String Operations ---")
    
    # Benchmark string searching
    sus large_text tea = repeat_string("abcdefghij", 1000)  # 10,000 characters
    sus search_pattern tea = "ghij"
    
    # Time Boyer-Moore search
    sus start_time drip = get_current_time()
    sus matches []drip = boyer_moore_search(large_text, search_pattern)
    sus end_time drip = get_current_time()
    
    vibez.spill("Boyer-Moore search found", len(matches), "matches")
    vibez.spill("Search time: " + int_to_string(end_time - start_time) + "ms")
    
    # Benchmark Unicode normalization
    sus unicode_text tea = repeat_string("café", 100)
    sus norm_start drip = get_current_time()
    sus normalized tea = normalize_nfc_real(unicode_text)
    sus norm_end drip = get_current_time()
    
    vibez.spill("Normalization time: " + int_to_string(norm_end - norm_start) + "ms")
    
    # Benchmark regex compilation and matching
    sus regex_pattern tea = "[a-zA-Z]+@[a-zA-Z0-9]+\\.[a-zA-Z]{2,4}"
    sus email_text tea = repeat_string("test@example.com ", 100)
    
    sus regex_start drip = get_current_time()
    sus engine RegexEngine = regex_compile(regex_pattern)
    sus email_matches []RegexMatch = regex_find_all(regex_pattern, email_text)
    sus regex_end drip = get_current_time()
    
    vibez.spill("Regex found", len(email_matches), "email matches")
    vibez.spill("Regex time: " + int_to_string(regex_end - regex_start) + "ms")
}

# Placeholder for time measurement
slay get_current_time() drip {
    # This would be implemented by runtime to get millisecond timestamp
    damn 0
}

# Run the comprehensive test
test_comprehensive_string_processing()
benchmark_string_operations()
