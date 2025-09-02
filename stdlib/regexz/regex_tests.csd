# Comprehensive Regular Expression Test Suite
# Tests for all advanced regex features

yeet "testz"
yeet "regexz/regex_api"
yeet "regexz/regex_engine"
yeet "regexz/unicode_support"

# Initialize test framework
test_start("Advanced Regular Expression Tests")

# Basic pattern matching tests
slay test_basic_patterns() drip {
    test_group("Basic Pattern Matching")
    
    # Simple literal matching
    sus engine RegexEngine = regex_new("hello") shook test_failed("Failed to compile 'hello'")
    sus result MatchResult = regex_match(&engine, "hello world") shook test_failed("Failed to match 'hello'")
    assert_true(result.matched, "Should match 'hello' in 'hello world'")
    assert_eq_str(result.full_match, "hello", "Should extract 'hello'")
    
    # Character classes
    sus digit_engine RegexEngine = regex_new("\\d+") shook test_failed("Failed to compile '\\d+'")
    sus digit_result MatchResult = regex_match(&digit_engine, "abc123def") shook test_failed("Failed to match digits")
    assert_true(digit_result.matched, "Should match digits")
    assert_eq_str(digit_result.full_match, "123", "Should extract '123'")
    
    # Word boundaries
    sus word_engine RegexEngine = regex_new("\\btest\\b") shook test_failed("Failed to compile word boundary")
    sus word_result MatchResult = regex_match(&word_engine, "testing test tested") shook test_failed("Failed to match word")
    assert_true(word_result.matched, "Should match word 'test'")
    
    # Anchors
    sus anchor_engine RegexEngine = regex_new("^start") shook test_failed("Failed to compile anchor")
    sus anchor_result MatchResult = regex_match(&anchor_engine, "start of line") shook test_failed("Failed to match anchor")
    assert_true(anchor_result.matched, "Should match at start of line")
    
    test_passed("Basic pattern matching")
}

# Quantifier tests
slay test_quantifiers() drip {
    test_group("Quantifier Tests")
    
    # Star quantifier
    sus star_engine RegexEngine = regex_new("a*") shook test_failed("Failed to compile 'a*'")
    sus star_result1 MatchResult = regex_match(&star_engine, "aaab") shook test_failed("Failed to match 'a*'")
    assert_true(star_result1.matched, "Should match zero or more 'a'")
    assert_eq_str(star_result1.full_match, "aaa", "Should match 'aaa'")
    
    sus star_result2 MatchResult = regex_match(&star_engine, "bbb") shook test_failed("Failed to match empty 'a*'")
    assert_true(star_result2.matched, "Should match empty string for 'a*'")
    assert_eq_str(star_result2.full_match, "", "Should match empty string")
    
    # Plus quantifier
    sus plus_engine RegexEngine = regex_new("a+") shook test_failed("Failed to compile 'a+'")
    sus plus_result1 MatchResult = regex_match(&plus_engine, "aaab") shook test_failed("Failed to match 'a+'")
    assert_true(plus_result1.matched, "Should match one or more 'a'")
    assert_eq_str(plus_result1.full_match, "aaa", "Should match 'aaa'")
    
    sus plus_result2 MatchResult = regex_match(&plus_engine, "bbb") shook test_failed("Failed to match 'a+'")
    assert_false(plus_result2.matched, "Should not match empty string for 'a+'")
    
    # Question quantifier
    sus question_engine RegexEngine = regex_new("colou?r") shook test_failed("Failed to compile 'colou?r'")
    sus question_result1 MatchResult = regex_match(&question_engine, "color") shook test_failed("Failed to match 'color'")
    assert_true(question_result1.matched, "Should match 'color'")
    assert_eq_str(question_result1.full_match, "color", "Should match 'color'")
    
    sus question_result2 MatchResult = regex_match(&question_engine, "colour") shook test_failed("Failed to match 'colour'")
    assert_true(question_result2.matched, "Should match 'colour'")
    assert_eq_str(question_result2.full_match, "colour", "Should match 'colour'")
    
    # Range quantifiers
    sus range_engine RegexEngine = regex_new("a{2,4}") shook test_failed("Failed to compile 'a{2,4}'")
    sus range_result1 MatchResult = regex_match(&range_engine, "aa") shook test_failed("Failed to match 'aa'")
    assert_true(range_result1.matched, "Should match 'aa'")
    
    sus range_result2 MatchResult = regex_match(&range_engine, "aaaaa") shook test_failed("Failed to match 'aaaaa'")
    assert_true(range_result2.matched, "Should match first 4 'a's")
    assert_eq_str(range_result2.full_match, "aaaa", "Should match exactly 4 'a's")
    
    # Non-greedy quantifiers
    sus nongreedy_engine RegexEngine = regex_new("a+?") shook test_failed("Failed to compile 'a+?'")
    sus nongreedy_result MatchResult = regex_match(&nongreedy_engine, "aaab") shook test_failed("Failed to match non-greedy")
    assert_true(nongreedy_result.matched, "Should match non-greedy")
    assert_eq_str(nongreedy_result.full_match, "a", "Should match single 'a'")
    
    test_passed("Quantifier tests")
}

# Capture group tests
slay test_capture_groups() drip {
    test_group("Capture Group Tests")
    
    # Basic capture groups
    sus group_engine RegexEngine = regex_new("(\\d+)-(\\d+)-(\\d+)") shook test_failed("Failed to compile date pattern")
    sus group_result MatchResult = regex_match(&group_engine, "2023-12-25") shook test_failed("Failed to match date")
    assert_true(group_result.matched, "Should match date pattern")
    assert_eq_str(group_result.full_match, "2023-12-25", "Should match full date")
    assert_eq_int(group_result.groups.len(), 3, "Should have 3 capture groups")
    assert_eq_str(group_result.groups[0].value, "2023", "Should capture year")
    assert_eq_str(group_result.groups[1].value, "12", "Should capture month")
    assert_eq_str(group_result.groups[2].value, "25", "Should capture day")
    
    # Named capture groups
    sus named_engine RegexEngine = regex_new("(?P<year>\\d+)-(?P<month>\\d+)-(?P<day>\\d+)") shook test_failed("Failed to compile named groups")
    sus named_result MatchResult = regex_match(&named_engine, "2023-12-25") shook test_failed("Failed to match named groups")
    assert_true(named_result.matched, "Should match named groups")
    
    sus year_value tea = get_named_group(named_result, "year") shook test_failed("Failed to get year group")
    sus month_value tea = get_named_group(named_result, "month") shook test_failed("Failed to get month group")
    sus day_value tea = get_named_group(named_result, "day") shook test_failed("Failed to get day group")
    
    assert_eq_str(year_value, "2023", "Should extract year")
    assert_eq_str(month_value, "12", "Should extract month")
    assert_eq_str(day_value, "25", "Should extract day")
    
    # Non-capturing groups
    sus noncap_engine RegexEngine = regex_new("(?:foo|bar)(\\d+)") shook test_failed("Failed to compile non-capturing")
    sus noncap_result MatchResult = regex_match(&noncap_engine, "foo123") shook test_failed("Failed to match non-capturing")
    assert_true(noncap_result.matched, "Should match non-capturing group")
    assert_eq_int(noncap_result.groups.len(), 1, "Should have only 1 capture group")
    assert_eq_str(noncap_result.groups[0].value, "123", "Should capture only the number")
    
    # Nested capture groups
    sus nested_engine RegexEngine = regex_new("((a+)(b+))") shook test_failed("Failed to compile nested groups")
    sus nested_result MatchResult = regex_match(&nested_engine, "aaabbb") shook test_failed("Failed to match nested groups")
    assert_true(nested_result.matched, "Should match nested groups")
    assert_eq_int(nested_result.groups.len(), 3, "Should have 3 capture groups")
    assert_eq_str(nested_result.groups[0].value, "aaabbb", "Should capture outer group")
    assert_eq_str(nested_result.groups[1].value, "aaa", "Should capture first inner group")
    assert_eq_str(nested_result.groups[2].value, "bbb", "Should capture second inner group")
    
    test_passed("Capture group tests")
}

# Lookahead and lookbehind tests
slay test_lookaround() drip {
    test_group("Lookahead and Lookbehind Tests")
    
    # Positive lookahead
    sus pos_ahead_engine RegexEngine = regex_new("\\d+(?=px)") shook test_failed("Failed to compile positive lookahead")
    sus pos_ahead_result MatchResult = regex_match(&pos_ahead_engine, "100px") shook test_failed("Failed to match positive lookahead")
    assert_true(pos_ahead_result.matched, "Should match with positive lookahead")
    assert_eq_str(pos_ahead_result.full_match, "100", "Should match only the digits")
    
    sus pos_ahead_fail MatchResult = regex_match(&pos_ahead_engine, "100pt") shook test_failed("Failed to test positive lookahead failure")
    assert_false(pos_ahead_fail.matched, "Should not match without 'px'")
    
    # Negative lookahead
    sus neg_ahead_engine RegexEngine = regex_new("\\d+(?!px)") shook test_failed("Failed to compile negative lookahead")
    sus neg_ahead_result MatchResult = regex_match(&neg_ahead_engine, "100pt") shook test_failed("Failed to match negative lookahead")
    assert_true(neg_ahead_result.matched, "Should match with negative lookahead")
    assert_eq_str(neg_ahead_result.full_match, "100", "Should match the digits")
    
    sus neg_ahead_fail MatchResult = regex_match(&neg_ahead_engine, "100px") shook test_failed("Failed to test negative lookahead failure")
    assert_false(neg_ahead_fail.matched, "Should not match when followed by 'px'")
    
    # Positive lookbehind
    sus pos_behind_engine RegexEngine = regex_new("(?<=\\$)\\d+") shook test_failed("Failed to compile positive lookbehind")
    sus pos_behind_result MatchResult = regex_match(&pos_behind_engine, "$100") shook test_failed("Failed to match positive lookbehind")
    assert_true(pos_behind_result.matched, "Should match with positive lookbehind")
    assert_eq_str(pos_behind_result.full_match, "100", "Should match only the digits")
    
    sus pos_behind_fail MatchResult = regex_match(&pos_behind_engine, "€100") shook test_failed("Failed to test positive lookbehind failure")
    assert_false(pos_behind_fail.matched, "Should not match without '$' prefix")
    
    # Negative lookbehind
    sus neg_behind_engine RegexEngine = regex_new("(?<!\\$)\\d+") shook test_failed("Failed to compile negative lookbehind")
    sus neg_behind_result MatchResult = regex_match(&neg_behind_engine, "€100") shook test_failed("Failed to match negative lookbehind")
    assert_true(neg_behind_result.matched, "Should match with negative lookbehind")
    assert_eq_str(neg_behind_result.full_match, "100", "Should match the digits")
    
    sus neg_behind_fail MatchResult = regex_match(&neg_behind_engine, "$100") shook test_failed("Failed to test negative lookbehind failure")
    assert_false(neg_behind_fail.matched, "Should not match when preceded by '$'")
    
    # Complex lookaround combinations
    sus complex_engine RegexEngine = regex_new("(?<=\\w)\\d+(?=\\w)") shook test_failed("Failed to compile complex lookaround")
    sus complex_result MatchResult = regex_match(&complex_engine, "abc123def") shook test_failed("Failed to match complex lookaround")
    assert_true(complex_result.matched, "Should match with complex lookaround")
    assert_eq_str(complex_result.full_match, "123", "Should match digits between word chars")
    
    test_passed("Lookaround tests")
}

# Unicode property tests
slay test_unicode_properties() drip {
    test_group("Unicode Property Tests")
    
    # Initialize Unicode support
    init_unicode_regexz()
    
    # General category tests
    sus letter_engine RegexEngine = regex_new("\\p{L}+") shook test_failed("Failed to compile letter property")
    sus letter_result MatchResult = regex_match(&letter_engine, "Hello世界") shook test_failed("Failed to match letters")
    assert_true(letter_result.matched, "Should match Unicode letters")
    
    # Script property tests
    sus latin_engine RegexEngine = regex_new("\\p{Script=Latin}+") shook test_failed("Failed to compile Latin script")
    sus latin_result MatchResult = regex_match(&latin_engine, "Hello") shook test_failed("Failed to match Latin script")
    assert_true(latin_result.matched, "Should match Latin script")
    
    # Block property tests
    sus basic_latin_engine RegexEngine = regex_new("\\p{Block=Basic_Latin}+") shook test_failed("Failed to compile Basic Latin block")
    sus basic_latin_result MatchResult = regex_match(&basic_latin_engine, "Hello") shook test_failed("Failed to match Basic Latin block")
    assert_true(basic_latin_result.matched, "Should match Basic Latin block")
    
    # Derived property tests
    sus whitespace_engine RegexEngine = regex_new("\\p{White_Space}+") shook test_failed("Failed to compile whitespace property")
    sus whitespace_result MatchResult = regex_match(&whitespace_engine, "   \t\n") shook test_failed("Failed to match whitespace")
    assert_true(whitespace_result.matched, "Should match Unicode whitespace")
    
    sus digit_engine RegexEngine = regex_new("\\p{Nd}+") shook test_failed("Failed to compile digit property")
    sus digit_result MatchResult = regex_match(&digit_engine, "1234567890") shook test_failed("Failed to match decimal digits")
    assert_true(digit_result.matched, "Should match decimal number category")
    
    # Negated properties
    sus non_letter_engine RegexEngine = regex_new("\\P{L}+") shook test_failed("Failed to compile negated letter property")
    sus non_letter_result MatchResult = regex_match(&non_letter_engine, "123!@#") shook test_failed("Failed to match non-letters")
    assert_true(non_letter_result.matched, "Should match non-letter characters")
    
    test_passed("Unicode property tests")
}

# Character class tests
slay test_character_classes() drip {
    test_group("Character Class Tests")
    
    # Basic character classes
    sus vowel_engine RegexEngine = regex_new("[aeiou]+") shook test_failed("Failed to compile vowel class")
    sus vowel_result MatchResult = regex_match(&vowel_engine, "beautiful") shook test_failed("Failed to match vowels")
    assert_true(vowel_result.matched, "Should match vowels")
    assert_eq_str(vowel_result.full_match, "eau", "Should match 'eau'")
    
    # Negated character classes
    sus consonant_engine RegexEngine = regex_new("[^aeiou]+") shook test_failed("Failed to compile consonant class")
    sus consonant_result MatchResult = regex_match(&consonant_engine, "beautiful") shook test_failed("Failed to match consonants")
    assert_true(consonant_result.matched, "Should match consonants")
    assert_eq_str(consonant_result.full_match, "b", "Should match first consonant")
    
    # Range character classes
    sus range_engine RegexEngine = regex_new("[a-z]+") shook test_failed("Failed to compile range class")
    sus range_result MatchResult = regex_match(&range_engine, "Hello123") shook test_failed("Failed to match range")
    assert_true(range_result.matched, "Should match lowercase letters")
    assert_eq_str(range_result.full_match, "ello", "Should match 'ello'")
    
    # Multiple ranges
    sus multi_range_engine RegexEngine = regex_new("[a-zA-Z0-9]+") shook test_failed("Failed to compile multi-range class")
    sus multi_range_result MatchResult = regex_match(&multi_range_engine, "Hello123World") shook test_failed("Failed to match multi-range")
    assert_true(multi_range_result.matched, "Should match alphanumeric")
    assert_eq_str(multi_range_result.full_match, "Hello123World", "Should match full string")
    
    # Predefined classes
    sus word_class_engine RegexEngine = regex_new("\\w+") shook test_failed("Failed to compile word class")
    sus word_class_result MatchResult = regex_match(&word_class_engine, "hello_world123") shook test_failed("Failed to match word class")
    assert_true(word_class_result.matched, "Should match word characters")
    assert_eq_str(word_class_result.full_match, "hello_world123", "Should match full word")
    
    sus digit_class_engine RegexEngine = regex_new("\\d+") shook test_failed("Failed to compile digit class")
    sus digit_class_result MatchResult = regex_match(&digit_class_engine, "abc123def") shook test_failed("Failed to match digit class")
    assert_true(digit_class_result.matched, "Should match digits")
    assert_eq_str(digit_class_result.full_match, "123", "Should match digits")
    
    sus space_class_engine RegexEngine = regex_new("\\s+") shook test_failed("Failed to compile space class")
    sus space_class_result MatchResult = regex_match(&space_class_engine, "hello   world") shook test_failed("Failed to match space class")
    assert_true(space_class_result.matched, "Should match whitespace")
    assert_eq_str(space_class_result.full_match, "   ", "Should match spaces")
    
    test_passed("Character class tests")
}

# Alternation tests
slay test_alternation() drip {
    test_group("Alternation Tests")
    
    # Basic alternation
    sus alt_engine RegexEngine = regex_new("cat|dog|bird") shook test_failed("Failed to compile alternation")
    sus alt_result1 MatchResult = regex_match(&alt_engine, "I have a cat") shook test_failed("Failed to match 'cat'")
    assert_true(alt_result1.matched, "Should match 'cat'")
    assert_eq_str(alt_result1.full_match, "cat", "Should match 'cat'")
    
    sus alt_result2 MatchResult = regex_match(&alt_engine, "I have a dog") shook test_failed("Failed to match 'dog'")
    assert_true(alt_result2.matched, "Should match 'dog'")
    assert_eq_str(alt_result2.full_match, "dog", "Should match 'dog'")
    
    sus alt_result3 MatchResult = regex_match(&alt_engine, "I have a bird") shook test_failed("Failed to match 'bird'")
    assert_true(alt_result3.matched, "Should match 'bird'")
    assert_eq_str(alt_result3.full_match, "bird", "Should match 'bird'")
    
    sus alt_fail MatchResult = regex_match(&alt_engine, "I have a fish") shook test_failed("Failed to test alternation failure")
    assert_false(alt_fail.matched, "Should not match 'fish'")
    
    # Grouped alternation
    sus group_alt_engine RegexEngine = regex_new("(cat|dog) food") shook test_failed("Failed to compile grouped alternation")
    sus group_alt_result1 MatchResult = regex_match(&group_alt_engine, "cat food is expensive") shook test_failed("Failed to match 'cat food'")
    assert_true(group_alt_result1.matched, "Should match 'cat food'")
    assert_eq_str(group_alt_result1.full_match, "cat food", "Should match 'cat food'")
    assert_eq_str(group_alt_result1.groups[0].value, "cat", "Should capture 'cat'")
    
    sus group_alt_result2 MatchResult = regex_match(&group_alt_engine, "dog food is cheap") shook test_failed("Failed to match 'dog food'")
    assert_true(group_alt_result2.matched, "Should match 'dog food'")
    assert_eq_str(group_alt_result2.full_match, "dog food", "Should match 'dog food'")
    assert_eq_str(group_alt_result2.groups[0].value, "dog", "Should capture 'dog'")
    
    # Complex alternation with quantifiers
    sus complex_alt_engine RegexEngine = regex_new("(red|green|blue)\\s+(car|truck|bike)s?") shook test_failed("Failed to compile complex alternation")
    sus complex_alt_result MatchResult = regex_match(&complex_alt_engine, "red cars are fast") shook test_failed("Failed to match complex alternation")
    assert_true(complex_alt_result.matched, "Should match complex alternation")
    assert_eq_str(complex_alt_result.full_match, "red cars", "Should match 'red cars'")
    assert_eq_str(complex_alt_result.groups[0].value, "red", "Should capture color")
    assert_eq_str(complex_alt_result.groups[1].value, "car", "Should capture vehicle")
    
    test_passed("Alternation tests")
}

# Replacement function tests
slay test_replacements() drip {
    test_group("Replacement Function Tests")
    
    # Basic replacement
    sus basic_engine RegexEngine = regex_new("\\d+") shook test_failed("Failed to compile digit pattern")
    sus replaced tea = regex_replace(&basic_engine, "I have 5 apples and 3 oranges", "many") shook test_failed("Failed to replace")
    assert_eq_str(replaced, "I have many apples and many oranges", "Should replace all numbers")
    
    # Group replacement
    sus group_replace_engine RegexEngine = regex_new("(\\w+)\\s+(\\w+)") shook test_failed("Failed to compile word pattern")
    sus group_replaced tea = regex_replace(&group_replace_engine, "John Doe", "$2, $1") shook test_failed("Failed to replace with groups")
    assert_eq_str(group_replaced, "Doe, John", "Should swap names")
    
    # Named group replacement
    sus named_replace_engine RegexEngine = regex_new("(?P<first>\\w+)\\s+(?P<last>\\w+)") shook test_failed("Failed to compile named pattern")
    sus named_replaced tea = regex_replace(&named_replace_engine, "John Doe", "${last}, ${first}") shook test_failed("Failed to replace with named groups")
    assert_eq_str(named_replaced, "Doe, John", "Should swap names using named groups")
    
    # Function replacement
    sus func_replace_engine RegexEngine = regex_new("\\d+") shook test_failed("Failed to compile number pattern")
    sus func_replaced tea = regex_replace_func(&func_replace_engine, "I have 5 apples", slay(match MatchResult) tea {
        sus num drip = match.full_match.to_int()
        damn (num * 2).to_string()
    }) shook test_failed("Failed to replace with function")
    assert_eq_str(func_replaced, "I have 10 apples", "Should double the number")
    
    test_passed("Replacement tests")
}

# Split function tests
slay test_splitting() drip {
    test_group("Split Function Tests")
    
    # Basic splitting
    sus split_engine RegexEngine = regex_new(",\\s*") shook test_failed("Failed to compile comma pattern")
    sus parts tea[value] = regex_split(&split_engine, "apple, banana, cherry, date") shook test_failed("Failed to split")
    assert_eq_int(parts.len(), 4, "Should have 4 parts")
    assert_eq_str(parts[0], "apple", "First part should be 'apple'")
    assert_eq_str(parts[1], "banana", "Second part should be 'banana'")
    assert_eq_str(parts[2], "cherry", "Third part should be 'cherry'")
    assert_eq_str(parts[3], "date", "Fourth part should be 'date'")
    
    # Split with complex pattern
    sus complex_split_engine RegexEngine = regex_new("[;,]\\s*") shook test_failed("Failed to compile delimiter pattern")
    sus complex_parts tea[value] = regex_split(&complex_split_engine, "a;b, c,d;  e") shook test_failed("Failed to split complex")
    assert_eq_int(complex_parts.len(), 5, "Should have 5 parts")
    assert_eq_str(complex_parts[0], "a", "Should be 'a'")
    assert_eq_str(complex_parts[1], "b", "Should be 'b'")
    assert_eq_str(complex_parts[2], "c", "Should be 'c'")
    assert_eq_str(complex_parts[3], "d", "Should be 'd'")
    assert_eq_str(complex_parts[4], "e", "Should be 'e'")
    
    # No matches case
    sus no_match_engine RegexEngine = regex_new("xyz") shook test_failed("Failed to compile no match pattern")
    sus no_match_parts tea[value] = regex_split(&no_match_engine, "hello world") shook test_failed("Failed to split no match")
    assert_eq_int(no_match_parts.len(), 1, "Should have 1 part when no matches")
    assert_eq_str(no_match_parts[0], "hello world", "Should return original string")
    
    test_passed("Split tests")
}

# Performance and optimization tests
slay test_performance() drip {
    test_group("Performance and Optimization Tests")
    
    # Cache performance test
    sus cache_engine RegexEngine = regex_new("\\d{3}-\\d{3}-\\d{4}") shook test_failed("Failed to compile phone pattern")
    sus test_text tea = "Call me at 555-123-4567 or 555-987-6543"
    
    # First match (cache miss)
    sus start_time drip = current_time_micros()
    sus result1 MatchResult = regex_match(&cache_engine, test_text) shook test_failed("Failed first match")
    sus first_time drip = current_time_micros() - start_time
    
    # Second match (cache hit)
    start_time = current_time_micros()
    sus result2 MatchResult = regex_match(&cache_engine, test_text) shook test_failed("Failed second match")
    sus second_time drip = current_time_micros() - start_time
    
    assert_true(result1.matched, "First match should succeed")
    assert_true(result2.matched, "Second match should succeed")
    assert_true(second_time < first_time, "Cache should improve performance")
    
    # Pattern compilation optimization
    sus options RegexOptions = create_default_options()
    options.optimization_level = 2
    
    sus optimized_engine RegexEngine = regex_new_with_options("a+b+c+", options) shook test_failed("Failed to compile optimized")
    sus optimized_result MatchResult = regex_match(&optimized_engine, "aaabbbccc") shook test_failed("Failed to match optimized")
    assert_true(optimized_result.matched, "Optimized pattern should work")
    
    test_passed("Performance tests")
}

# Edge case and error handling tests
slay test_edge_cases() drip {
    test_group("Edge Cases and Error Handling")
    
    # Empty string patterns
    sus empty_result lit = regex_test("", "anything") shook assert_true(nah, "Empty pattern should fail")
    # Should fail to compile empty pattern
    
    # Invalid pattern syntax
    sus invalid_result lit = regex_is_valid("[unclosed")
    assert_false(invalid_result, "Invalid pattern should not validate")
    
    # Very long pattern
    sus long_pattern tea = "a" * 1000
    sus long_engine RegexEngine = regex_new(long_pattern) shook test_failed("Failed to compile long pattern")
    sus long_result MatchResult = regex_match(&long_engine, "a" * 1000) shook test_failed("Failed to match long pattern")
    assert_true(long_result.matched, "Should handle very long patterns")
    
    # Empty string matching
    sus empty_match_engine RegexEngine = regex_new("a*") shook test_failed("Failed to compile empty match pattern")
    sus empty_match_result MatchResult = regex_match(&empty_match_engine, "") shook test_failed("Failed to match empty string")
    assert_true(empty_match_result.matched, "Should match empty string with a*")
    
    # Unicode edge cases
    sus unicode_engine RegexEngine = regex_new(".+") shook test_failed("Failed to compile dot pattern")
    sus unicode_result MatchResult = regex_match(&unicode_engine, "🦀🔥💯") shook test_failed("Failed to match Unicode")
    assert_true(unicode_result.matched, "Should handle Unicode emoji")
    
    # Catastrophic backtracking prevention
    sus backtrack_engine RegexEngine = regex_new("(a+)+b") shook test_failed("Failed to compile backtrack pattern")
    sus backtrack_result MatchResult = regex_match(&backtrack_engine, "a" * 30 + "c") shook test_failed("Failed to handle backtracking")
    assert_false(backtrack_result.matched, "Should prevent catastrophic backtracking")
    
    test_passed("Edge case tests")
}

# Find all matches test
slay test_find_all() drip {
    test_group("Find All Matches Tests")
    
    # Multiple matches
    sus all_engine RegexEngine = regex_new("\\d+") shook test_failed("Failed to compile digit pattern")
    sus all_matches MatchResult[value] = regex_find_all(&all_engine, "I have 5 apples, 3 oranges, and 12 bananas") shook test_failed("Failed to find all")
    
    assert_eq_int(all_matches.len(), 3, "Should find 3 matches")
    assert_eq_str(all_matches[0].full_match, "5", "First match should be '5'")
    assert_eq_str(all_matches[1].full_match, "3", "Second match should be '3'")
    assert_eq_str(all_matches[2].full_match, "12", "Third match should be '12'")
    
    # Overlapping matches test
    sus overlap_engine RegexEngine = regex_new("a.") shook test_failed("Failed to compile overlap pattern")
    sus overlap_matches MatchResult[value] = regex_find_all(&overlap_engine, "abacad") shook test_failed("Failed to find overlapping")
    
    assert_eq_int(overlap_matches.len(), 3, "Should find 3 non-overlapping matches")
    assert_eq_str(overlap_matches[0].full_match, "ab", "First match should be 'ab'")
    assert_eq_str(overlap_matches[1].full_match, "ac", "Second match should be 'ac'")
    assert_eq_str(overlap_matches[2].full_match, "ad", "Third match should be 'ad'")
    
    # No matches case
    sus no_all_engine RegexEngine = regex_new("xyz") shook test_failed("Failed to compile no match pattern")
    sus no_all_matches MatchResult[value] = regex_find_all(&no_all_engine, "hello world") shook test_failed("Failed to find all no matches")
    assert_eq_int(no_all_matches.len(), 0, "Should find no matches")
    
    test_passed("Find all tests")
}

# Main test runner
slay main() drip {
    test_basic_patterns()
    test_quantifiers()
    test_capture_groups()
    test_lookaround()
    test_unicode_properties()
    test_character_classes()
    test_alternation()
    test_replacements()
    test_splitting()
    test_performance()
    test_edge_cases()
    test_find_all()
    
    print_test_summary()
}

# Performance testing utilities
slay current_time_micros() drip {
    # Platform-specific time implementation
    damn 0  # Placeholder
}
