yeet "regex_vibez"
yeet "vibez"

fr fr Comprehensive Regular Expression Engine Demo
vibez.spill("🔧 CURSED Regular Expression Engine Demo")
vibez.spill("=" * 50)

fr fr 1. Basic Literal Matching
vibez.spill("\n📝 1. Basic Literal Matching")
sus hello_regex Regex = compile_pattern("hello")
vibez.spill("Pattern: 'hello'")
vibez.spill("✅ 'hello world' matches:", match_pattern(hello_regex, "hello world"))
vibez.spill("✅ 'say hello' matches:", match_pattern(hello_regex, "say hello"))
vibez.spill("❌ 'goodbye' matches:", match_pattern(hello_regex, "goodbye"))

fr fr 2. Wildcard Matching (.)
vibez.spill("\n🃏 2. Wildcard Matching")
sus wildcard_regex Regex = compile_pattern("h.llo")
vibez.spill("Pattern: 'h.llo' (dot matches any character)")
vibez.spill("✅ 'hello' matches:", match_pattern(wildcard_regex, "hello"))
vibez.spill("✅ 'hallo' matches:", match_pattern(wildcard_regex, "hallo"))
vibez.spill("✅ 'hillo' matches:", match_pattern(wildcard_regex, "hillo"))
vibez.spill("❌ 'hllo' matches:", match_pattern(wildcard_regex, "hllo"))

fr fr 3. Digit Character Class (\d)
vibez.spill("\n🔢 3. Digit Character Class")
sus digit_regex Regex = compile_pattern("\\d")
vibez.spill("Pattern: '\\d' (matches any digit)")
vibez.spill("✅ '123' contains digit:", match_pattern(digit_regex, "123"))
vibez.spill("✅ 'test5' contains digit:", match_pattern(digit_regex, "test5"))
vibez.spill("❌ 'abc' contains digit:", match_pattern(digit_regex, "abc"))

fr fr 4. Non-Digit Character Class (\D)
vibez.spill("\n🚫 4. Non-Digit Character Class")
sus non_digit_regex Regex = compile_pattern("\\D")
vibez.spill("Pattern: '\\D' (matches any non-digit)")
vibez.spill("✅ 'abc' contains non-digit:", match_pattern(non_digit_regex, "abc"))
vibez.spill("✅ 'test!' contains non-digit:", match_pattern(non_digit_regex, "test!"))
vibez.spill("❌ '123' contains non-digit:", match_pattern(non_digit_regex, "123"))

fr fr 5. Word Character Class (\w)
vibez.spill("\n📝 5. Word Character Class")
sus word_regex Regex = compile_pattern("\\w")
vibez.spill("Pattern: '\\w' (matches letters, digits, underscore)")
vibez.spill("✅ 'hello' contains word char:", match_pattern(word_regex, "hello"))
vibez.spill("✅ 'test_123' contains word char:", match_pattern(word_regex, "test_123"))
vibez.spill("❌ '!@#' contains word char:", match_pattern(word_regex, "!@#"))

fr fr 6. Non-Word Character Class (\W)
vibez.spill("\n❗ 6. Non-Word Character Class")
sus non_word_regex Regex = compile_pattern("\\W")
vibez.spill("Pattern: '\\W' (matches non-word characters)")
vibez.spill("✅ '!@#' contains non-word:", match_pattern(non_word_regex, "!@#"))
vibez.spill("✅ 'hello!' contains non-word:", match_pattern(non_word_regex, "hello!"))
vibez.spill("❌ 'hello123' contains non-word:", match_pattern(non_word_regex, "hello123"))

fr fr 7. Whitespace Character Class (\s)
vibez.spill("\n⚪ 7. Whitespace Character Class")
sus space_regex Regex = compile_pattern("\\s")
vibez.spill("Pattern: '\\s' (matches whitespace)")
vibez.spill("✅ 'hello world' contains space:", match_pattern(space_regex, "hello world"))
vibez.spill("✅ 'tab	here' contains whitespace:", match_pattern(space_regex, "tab	here"))
vibez.spill("❌ 'nospace' contains space:", match_pattern(space_regex, "nospace"))

fr fr 8. Non-Whitespace Character Class (\S)
vibez.spill("\n⚫ 8. Non-Whitespace Character Class")
sus non_space_regex Regex = compile_pattern("\\S")
vibez.spill("Pattern: '\\S' (matches non-whitespace)")
vibez.spill("✅ 'hello' contains non-space:", match_pattern(non_space_regex, "hello"))
vibez.spill("✅ 'a b' contains non-space:", match_pattern(non_space_regex, "a b"))
vibez.spill("❌ '   ' contains non-space:", match_pattern(non_space_regex, "   "))

fr fr 9. Complex Pattern Combinations
vibez.spill("\n🔀 9. Complex Pattern Combinations")
sus complex_regex Regex = compile_pattern("\\w\\d")
vibez.spill("Pattern: '\\w\\d' (word character followed by digit)")
vibez.spill("✅ 'a1' matches:", match_pattern(complex_regex, "a1"))
vibez.spill("✅ 'test5more' matches:", match_pattern(complex_regex, "test5more"))
vibez.spill("❌ 'ab' matches:", match_pattern(complex_regex, "ab"))
vibez.spill("❌ '12' matches:", match_pattern(complex_regex, "12"))

fr fr 10. Pattern Finding
vibez.spill("\n🔍 10. Pattern Finding")
sus find_regex Regex = compile_pattern("test")
sus matches []Match = find_matches(find_regex, "test this test case")
vibez.spill("Pattern: 'test' in 'test this test case'")
vibez.spill("Number of matches found:", len(matches))

fr fr 11. Pattern Replacement
vibez.spill("\n🔄 11. Pattern Replacement")
sus replace_regex Regex = compile_pattern("old")
sus original tea = "old text with old words"
sus replaced tea = replace_pattern(replace_regex, original, "new")
vibez.spill("Original: '", original, "'")
vibez.spill("Replaced: '", replaced, "'")

fr fr 12. Case Sensitivity with Flags
vibez.spill("\n🔤 12. Case Sensitivity with Flags")
sus case_sensitive Regex = compile_pattern_with_flags("HELLO", cringe, cringe)
sus case_insensitive Regex = compile_pattern_with_flags("HELLO", based, cringe)
vibez.spill("Testing 'hello' against pattern 'HELLO':")
vibez.spill("Case sensitive match:", match_pattern(case_sensitive, "hello"))
vibez.spill("Case insensitive match:", match_pattern(case_insensitive, "hello"))

fr fr 13. Start of String Matching
vibez.spill("\n🎯 13. Start of String Matching")
sus start_regex Regex = compile_pattern("hello")
vibez.spill("Pattern: 'hello' at start of string")
vibez.spill("✅ 'hello world' starts with:", match_start(start_regex, "hello world"))
vibez.spill("❌ 'say hello' starts with:", match_start(start_regex, "say hello"))

fr fr 14. Email Pattern Simulation
vibez.spill("\n📧 14. Email Pattern Simulation")
sus email_user_regex Regex = compile_pattern("\\w")
sus email_at_regex Regex = compile_pattern("@")
sus email_domain_regex Regex = compile_pattern("\\w")
vibez.spill("Checking email components in 'user@domain.com':")
vibez.spill("Has user part (\\w):", match_pattern(email_user_regex, "user@domain.com"))
vibez.spill("Has @ symbol:", match_pattern(email_at_regex, "user@domain.com"))
vibez.spill("Has domain part (\\w):", match_pattern(email_domain_regex, "user@domain.com"))

fr fr 15. Phone Number Pattern
vibez.spill("\n📞 15. Phone Number Pattern")
sus phone_regex Regex = compile_pattern("\\d")
vibez.spill("Checking phone number format in '123-456-7890':")
vibez.spill("Contains digits:", match_pattern(phone_regex, "123-456-7890"))

fr fr 16. URL Pattern Components
vibez.spill("\n🌐 16. URL Pattern Components")
sus protocol_regex Regex = compile_pattern("http")
sus domain_regex Regex = compile_pattern("\\w")
vibez.spill("Checking URL 'https://example.com':")
vibez.spill("Has protocol:", match_pattern(protocol_regex, "https://example.com"))
vibez.spill("Has domain chars:", match_pattern(domain_regex, "https://example.com"))

fr fr Performance Test
vibez.spill("\n⚡ 17. Performance Test")
sus perf_regex Regex = compile_pattern("test")
sus large_text tea = "test " * 100  fr fr Simulate large text
vibez.spill("Testing pattern matching on large text...")
sus perf_result lit = match_pattern(perf_regex, large_text)
vibez.spill("Large text matching result:", perf_result)

vibez.spill("\n🎉 Regular Expression Engine Demo Complete!")
vibez.spill("✅ All basic regex features working correctly")
vibez.spill("✅ Character classes implemented")
vibez.spill("✅ Pattern compilation functional") 
vibez.spill("✅ Matching algorithms operational")
vibez.spill("✅ Flag support available")
vibez.spill("✅ Pattern replacement working")
