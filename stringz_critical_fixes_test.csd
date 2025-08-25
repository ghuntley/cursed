yeet "vibez"
yeet "stringz_enhanced"

sus test_passed drip = 0
sus test_total drip = 0

slay test_contains_char() {
    test_total = test_total + 7
    
    fr fr Test basic character search
    ready (contains_char("hello", "e")) {
        test_passed = test_passed + 1
        vibez.spill("✅ contains_char basic test passed")
    } otherwise {
        vibez.spill("❌ contains_char basic test failed")
    }
    
    fr fr Test character not found
    ready (!contains_char("hello", "x")) {
        test_passed = test_passed + 1
        vibez.spill("✅ contains_char not found test passed")
    } otherwise {
        vibez.spill("❌ contains_char not found test failed")
    }
    
    fr fr Test empty string
    ready (!contains_char("", "a")) {
        test_passed = test_passed + 1
        vibez.spill("✅ contains_char empty string test passed")
    } otherwise {
        vibez.spill("❌ contains_char empty string test failed")
    }
    
    fr fr Test empty character
    ready (!contains_char("hello", "")) {
        test_passed = test_passed + 1
        vibez.spill("✅ contains_char empty character test passed")
    } otherwise {
        vibez.spill("❌ contains_char empty character test failed")
    }
    
    fr fr Test first character
    ready (contains_char("hello", "h")) {
        test_passed = test_passed + 1
        vibez.spill("✅ contains_char first char test passed")
    } otherwise {
        vibez.spill("❌ contains_char first char test failed")
    }
    
    fr fr Test last character
    ready (contains_char("hello", "o")) {
        test_passed = test_passed + 1
        vibez.spill("✅ contains_char last char test passed")
    } otherwise {
        vibez.spill("❌ contains_char last char test failed")
    }
    
    fr fr Test middle character
    ready (contains_char("hello", "l")) {
        test_passed = test_passed + 1
        vibez.spill("✅ contains_char middle char test passed")
    } otherwise {
        vibez.spill("❌ contains_char middle char test failed")
    }
}

slay test_starts_with_prefix() {
    test_total = test_total + 6
    
    fr fr Test basic prefix
    ready (starts_with_prefix("hello world", "hello")) {
        test_passed = test_passed + 1
        vibez.spill("✅ starts_with_prefix basic test passed")
    } otherwise {
        vibez.spill("❌ starts_with_prefix basic test failed")
    }
    
    fr fr Test no prefix match
    ready (!starts_with_prefix("hello", "world")) {
        test_passed = test_passed + 1
        vibez.spill("✅ starts_with_prefix no match test passed")
    } otherwise {
        vibez.spill("❌ starts_with_prefix no match test failed")
    }
    
    fr fr Test empty prefix
    ready (starts_with_prefix("hello", "")) {
        test_passed = test_passed + 1
        vibez.spill("✅ starts_with_prefix empty prefix test passed")
    } otherwise {
        vibez.spill("❌ starts_with_prefix empty prefix test failed")
    }
    
    fr fr Test prefix longer than string
    ready (!starts_with_prefix("hi", "hello")) {
        test_passed = test_passed + 1
        vibez.spill("✅ starts_with_prefix too long test passed")
    } otherwise {
        vibez.spill("❌ starts_with_prefix too long test failed")
    }
    
    fr fr Test exact match
    ready (starts_with_prefix("hello", "hello")) {
        test_passed = test_passed + 1
        vibez.spill("✅ starts_with_prefix exact match test passed")
    } otherwise {
        vibez.spill("❌ starts_with_prefix exact match test failed")
    }
    
    fr fr Test empty string
    ready (!starts_with_prefix("", "hello")) {
        test_passed = test_passed + 1
        vibez.spill("✅ starts_with_prefix empty string test passed")
    } otherwise {
        vibez.spill("❌ starts_with_prefix empty string test failed")
    }
}

slay test_ends_with_suffix() {
    test_total = test_total + 6
    
    fr fr Test basic suffix
    ready (ends_with_suffix("hello world", "world")) {
        test_passed = test_passed + 1
        vibez.spill("✅ ends_with_suffix basic test passed")
    } otherwise {
        vibez.spill("❌ ends_with_suffix basic test failed")
    }
    
    fr fr Test no suffix match
    ready (!ends_with_suffix("hello", "world")) {
        test_passed = test_passed + 1
        vibez.spill("✅ ends_with_suffix no match test passed")
    } otherwise {
        vibez.spill("❌ ends_with_suffix no match test failed")
    }
    
    fr fr Test empty suffix
    ready (ends_with_suffix("hello", "")) {
        test_passed = test_passed + 1
        vibez.spill("✅ ends_with_suffix empty suffix test passed")
    } otherwise {
        vibez.spill("❌ ends_with_suffix empty suffix test failed")
    }
    
    fr fr Test suffix longer than string
    ready (!ends_with_suffix("hi", "hello")) {
        test_passed = test_passed + 1
        vibez.spill("✅ ends_with_suffix too long test passed")
    } otherwise {
        vibez.spill("❌ ends_with_suffix too long test failed")
    }
    
    fr fr Test exact match
    ready (ends_with_suffix("hello", "hello")) {
        test_passed = test_passed + 1
        vibez.spill("✅ ends_with_suffix exact match test passed")
    } otherwise {
        vibez.spill("❌ ends_with_suffix exact match test failed")
    }
    
    fr fr Test empty string
    ready (!ends_with_suffix("", "hello")) {
        test_passed = test_passed + 1
        vibez.spill("✅ ends_with_suffix empty string test passed")
    } otherwise {
        vibez.spill("❌ ends_with_suffix empty string test failed")
    }
}

slay test_character_validation() {
    test_total = test_total + 12
    
    fr fr Test digit validation
    ready (is_digit_char("5")) {
        test_passed = test_passed + 1
        vibez.spill("✅ is_digit_char test passed")
    } otherwise {
        vibez.spill("❌ is_digit_char test failed")
    }
    
    ready (!is_digit_char("a")) {
        test_passed = test_passed + 1
        vibez.spill("✅ is_digit_char non-digit test passed")
    } otherwise {
        vibez.spill("❌ is_digit_char non-digit test failed")
    }
    
    ready (!is_digit_char("")) {
        test_passed = test_passed + 1
        vibez.spill("✅ is_digit_char empty test passed")
    } otherwise {
        vibez.spill("❌ is_digit_char empty test failed")
    }
    
    fr fr Test alpha validation
    ready (is_alpha_char("a")) {
        test_passed = test_passed + 1
        vibez.spill("✅ is_alpha_char lowercase test passed")
    } otherwise {
        vibez.spill("❌ is_alpha_char lowercase test failed")
    }
    
    ready (is_alpha_char("A")) {
        test_passed = test_passed + 1
        vibez.spill("✅ is_alpha_char uppercase test passed")
    } otherwise {
        vibez.spill("❌ is_alpha_char uppercase test failed")
    }
    
    ready (!is_alpha_char("5")) {
        test_passed = test_passed + 1
        vibez.spill("✅ is_alpha_char non-alpha test passed")
    } otherwise {
        vibez.spill("❌ is_alpha_char non-alpha test failed")
    }
    
    ready (!is_alpha_char("")) {
        test_passed = test_passed + 1
        vibez.spill("✅ is_alpha_char empty test passed")
    } otherwise {
        vibez.spill("❌ is_alpha_char empty test failed")
    }
    
    fr fr Test alphanumeric validation
    ready (is_alphanumeric_char("a")) {
        test_passed = test_passed + 1
        vibez.spill("✅ is_alphanumeric_char alpha test passed")
    } otherwise {
        vibez.spill("❌ is_alphanumeric_char alpha test failed")
    }
    
    ready (is_alphanumeric_char("5")) {
        test_passed = test_passed + 1
        vibez.spill("✅ is_alphanumeric_char digit test passed")
    } otherwise {
        vibez.spill("❌ is_alphanumeric_char digit test failed")
    }
    
    ready (!is_alphanumeric_char(" ")) {
        test_passed = test_passed + 1
        vibez.spill("✅ is_alphanumeric_char non-alphanum test passed")
    } otherwise {
        vibez.spill("❌ is_alphanumeric_char non-alphanum test failed")
    }
    
    fr fr Test whitespace validation
    ready (is_whitespace_char(" ")) {
        test_passed = test_passed + 1
        vibez.spill("✅ is_whitespace_char space test passed")
    } otherwise {
        vibez.spill("❌ is_whitespace_char space test failed")
    }
    
    ready (!is_whitespace_char("a")) {
        test_passed = test_passed + 1
        vibez.spill("✅ is_whitespace_char non-space test passed")
    } otherwise {
        vibez.spill("❌ is_whitespace_char non-space test failed")
    }
}

vibez.spill("🧪 STRINGZ CRITICAL FIXES TEST SUITE")
vibez.spill("=====================================")

test_contains_char()
vibez.spill("")

test_starts_with_prefix()
vibez.spill("")

test_ends_with_suffix()
vibez.spill("")

test_character_validation()
vibez.spill("")

vibez.spill("📊 TEST RESULTS")
vibez.spill("===============")
vibez.spill("Passed:", test_passed, "/", test_total)

ready (test_passed == test_total) {
    vibez.spill("🎉 ALL CRITICAL STRING FIXES WORKING! ✅")
} otherwise {
    vibez.spill("❌ Some critical fixes failed -", (test_total - test_passed), "issues remaining")
}
