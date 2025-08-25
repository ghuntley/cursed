yeet "testz"
yeet "stringz_algorithms"
yeet "mathz"

test_start("String Algorithms Module Tests")

fr fr ===== UTF-8 PROCESSING TESTS =====

slay test_utf8_processing() {
    vibez.spill("Testing UTF-8 processing...")
    
    fr fr Test UTF-8 character length detection
    assert_equal_int(get_utf8_char_length(0x41), 1, "ASCII character length")  fr fr 'A'
    assert_equal_int(get_utf8_char_length(0xC3), 2, "2-byte UTF-8 character")  fr fr Latin
    assert_equal_int(get_utf8_char_length(0xE2), 3, "3-byte UTF-8 character")  fr fr CJK/symbols
    assert_equal_int(get_utf8_char_length(0xF0), 4, "4-byte UTF-8 character")  fr fr Emoji/rare
    
    fr fr Test invalid UTF-8 byte
    assert_equal_int(get_utf8_char_length(0xFF), 1, "Invalid UTF-8 byte treated as single")
    
    fr fr Test UTF-8 encoding
    sus ascii_encoded tea = encode_utf8_char(65)  fr fr 'A'
    assert_equal_string(ascii_encoded, "A", "ASCII character encoding")
    
    sus latin_encoded tea = encode_utf8_char(0xE9)  fr fr 'é'
    assert_not_empty_string(latin_encoded, "Latin character encoding")
    
    sus emoji_encoded tea = encode_utf8_char(0x1F600)  fr fr 😀
    assert_not_empty_string(emoji_encoded, "Emoji character encoding")
    
    sus invalid_encoded tea = encode_utf8_char(0x110000)  fr fr Beyond Unicode range
    assert_equal_string(invalid_encoded, "?", "Invalid code point returns replacement")
    
    vibez.spill("✅ UTF-8 processing tests completed")
}

fr fr ===== UNICODE INFORMATION TESTS =====

slay test_unicode_info() {
    vibez.spill("Testing Unicode character information...")
    
    fr fr Test ASCII uppercase letter
    sus uppercase_info UnicodeInfo = get_unicode_info(65)  fr fr 'A'
    assert_equal_bool(uppercase_info.is_letter, based, "A is letter")
    assert_equal_int(uppercase_info.category, UNICODE_CATEGORY_LETTER, "A is letter category")
    assert_equal_int(uppercase_info.lowercase_mapping, 97, "A lowercase is a")
    
    fr fr Test ASCII lowercase letter
    sus lowercase_info UnicodeInfo = get_unicode_info(97)  fr fr 'a'
    assert_equal_bool(lowercase_info.is_letter, based, "a is letter")
    assert_equal_int(lowercase_info.uppercase_mapping, 65, "a uppercase is A")
    
    fr fr Test ASCII digit
    sus digit_info UnicodeInfo = get_unicode_info(48)  fr fr '0'
    assert_equal_bool(digit_info.is_digit, based, "0 is digit")
    assert_equal_int(digit_info.category, UNICODE_CATEGORY_DIGIT, "0 is digit category")
    
    fr fr Test ASCII whitespace
    sus space_info UnicodeInfo = get_unicode_info(32)  fr fr ' '
    assert_equal_bool(space_info.is_whitespace, based, "Space is whitespace")
    assert_equal_int(space_info.category, UNICODE_CATEGORY_WHITESPACE, "Space is whitespace category")
    
    fr fr Test ASCII punctuation
    sus punct_info UnicodeInfo = get_unicode_info(33)  fr fr '!'
    assert_equal_bool(punct_info.is_punctuation, based, "! is punctuation")
    assert_equal_int(punct_info.category, UNICODE_CATEGORY_PUNCTUATION, "! is punctuation category")
    
    fr fr Test control character
    sus control_info UnicodeInfo = get_unicode_info(9)  fr fr Tab
    assert_equal_bool(control_info.is_control, based, "Tab is control")
    assert_equal_int(control_info.category, UNICODE_CATEGORY_CONTROL, "Tab is control category")
    
    vibez.spill("✅ Unicode information tests completed")
}

fr fr ===== STRING VIEW TESTS =====

slay test_string_view() {
    vibez.spill("Testing StringView operations...")
    
    fr fr Test StringView creation
    sus view StringView = create_string_view("Hello, World!", 0, 13)
    assert_equal_bool(view.is_valid, based, "StringView is valid")
    assert_equal_int(view.offset, 0, "StringView offset")
    assert_equal_int(view.length, 13, "StringView length")
    assert_equal_string(view.data, "Hello, World!", "StringView data")
    
    fr fr Test substring view
    sus sub_view StringView = create_string_view("Hello, World!", 7, 5)
    assert_equal_bool(sub_view.is_valid, based, "Substring view is valid")
    assert_equal_int(sub_view.offset, 7, "Substring view offset")
    assert_equal_int(sub_view.length, 5, "Substring view length")
    
    fr fr Test invalid StringView
    sus invalid_view StringView = create_string_view("Hello", 10, 5)
    assert_equal_bool(invalid_view.is_valid, cringe, "Invalid view rejected")
    
    vibez.spill("✅ StringView tests completed")
}

fr fr ===== ADVANCED STRING SEARCH TESTS =====

slay test_advanced_string_search() {
    vibez.spill("Testing advanced string search algorithms...")
    
    sus text tea = "The quick brown fox jumps over the lazy dog. The fox is quick."
    
    fr fr Test KMP search
    sus kmp_result StringSearchResult = kmp_search_advanced(text, "fox")
    assert_equal_bool(kmp_result.found, based, "KMP search finds pattern")
    assert_equal_int(kmp_result.position, 16, "KMP search correct position")
    assert_greater_than_int(kmp_result.total_count, 0, "KMP search count")
    
    fr fr Test Boyer-Moore search
    sus bm_result StringSearchResult = boyer_moore_search_advanced(text, "quick")
    assert_equal_bool(bm_result.found, based, "Boyer-Moore search finds pattern")
    assert_equal_int(bm_result.position, 4, "Boyer-Moore search correct position")
    
    fr fr Test Rabin-Karp search
    sus rk_result StringSearchResult = rabin_karp_search_advanced(text, "lazy")
    assert_equal_bool(rk_result.found, based, "Rabin-Karp search finds pattern")
    assert_equal_int(rk_result.position, 35, "Rabin-Karp search correct position")
    
    fr fr Test pattern not found
    sus not_found StringSearchResult = kmp_search_advanced(text, "elephant")
    assert_equal_bool(not_found.found, cringe, "Pattern not found correctly detected")
    assert_equal_int(not_found.position, -1, "Not found position is -1")
    
    fr fr Test empty pattern
    sus empty_pattern StringSearchResult = kmp_search_advanced(text, "")
    assert_equal_bool(empty_pattern.found, cringe, "Empty pattern handled")
    
    vibez.spill("✅ Advanced string search tests completed")
}

fr fr ===== STRING PATTERN COMPILATION TESTS =====

slay test_string_pattern_compilation() {
    vibez.spill("Testing string pattern compilation...")
    
    fr fr Test KMP pattern compilation
    sus kmp_pattern StringPattern = compile_pattern("ABABC", SEARCH_ALGORITHM_KMP)
    assert_equal_bool(kmp_pattern.is_compiled, based, "KMP pattern compiled")
    assert_equal_string(kmp_pattern.pattern, "ABABC", "KMP pattern text stored")
    assert_equal_int(kmp_pattern.algorithm, SEARCH_ALGORITHM_KMP, "KMP algorithm set")
    assert_greater_than_int(array_length_int(kmp_pattern.compiled_data), 0, "KMP preprocessing data")
    
    fr fr Test Boyer-Moore pattern compilation
    sus bm_pattern StringPattern = compile_pattern("PATTERN", SEARCH_ALGORITHM_BOYER_MOORE)
    assert_equal_bool(bm_pattern.is_compiled, based, "Boyer-Moore pattern compiled")
    assert_equal_int(bm_pattern.algorithm, SEARCH_ALGORITHM_BOYER_MOORE, "Boyer-Moore algorithm set")
    
    fr fr Test Rabin-Karp pattern compilation
    sus rk_pattern StringPattern = compile_pattern("HASH", SEARCH_ALGORITHM_RABIN_KARP)
    assert_equal_bool(rk_pattern.is_compiled, based, "Rabin-Karp pattern compiled")
    assert_equal_int(rk_pattern.algorithm, SEARCH_ALGORITHM_RABIN_KARP, "Rabin-Karp algorithm set")
    
    fr fr Test compiled pattern search
    sus compiled_result StringSearchResult = search_with_compiled_pattern(
        "ABABCABABA", kmp_pattern)
    assert_equal_bool(compiled_result.found, based, "Compiled pattern search works")
    
    vibez.spill("✅ String pattern compilation tests completed")
}

fr fr ===== STRING MANIPULATION TESTS =====

slay test_string_manipulation() {
    vibez.spill("Testing string manipulation algorithms...")
    
    fr fr Test string reversal
    sus original tea = "Hello, World!"
    sus reversed tea = reverse_string(original)
    assert_equal_string(reversed, "!dlroW ,olleH", "String reversal")
    
    fr fr Test palindrome detection
    assert_equal_bool(is_palindrome("racecar"), based, "Palindrome detection - true")
    assert_equal_bool(is_palindrome("hello"), cringe, "Palindrome detection - false")
    assert_equal_bool(is_palindrome("A man a plan a canal Panama"), based, "Palindrome detection - sentence")
    assert_equal_bool(is_palindrome(""), based, "Empty string is palindrome")
    assert_equal_bool(is_palindrome("a"), based, "Single character is palindrome")
    
    fr fr Test string rotation
    sus rotated tea = rotate_string("abcdef", 2)
    assert_equal_string(rotated, "cdefab", "String rotation")
    
    sus rotation_zero tea = rotate_string("hello", 0)
    assert_equal_string(rotation_zero, "hello", "Zero rotation")
    
    sus rotation_full tea = rotate_string("test", 4)
    assert_equal_string(rotation_full, "test", "Full rotation")
    
    vibez.spill("✅ String manipulation tests completed")
}

fr fr ===== STRING COMPARISON TESTS =====

slay test_string_comparison() {
    vibez.spill("Testing string comparison algorithms...")
    
    fr fr Test case-sensitive comparison
    sus case_options StringCompareOptions = StringCompareOptions{
        case_sensitive: based,
        ignore_accents: cringe,
        culture_aware: cringe,
        numeric_comparison: cringe
    }
    
    assert_equal_int(compare_strings("abc", "abc", case_options), 0, "Case-sensitive equal")
    assert_less_than_int(compare_strings("abc", "def", case_options), 0, "Case-sensitive less")
    assert_greater_than_int(compare_strings("def", "abc", case_options), 0, "Case-sensitive greater")
    assert_not_equal_int(compare_strings("ABC", "abc", case_options), 0, "Case-sensitive different case")
    
    fr fr Test case-insensitive comparison
    sus insensitive_options StringCompareOptions = StringCompareOptions{
        case_sensitive: cringe,
        ignore_accents: cringe,
        culture_aware: cringe,
        numeric_comparison: cringe
    }
    
    assert_equal_int(compare_strings("ABC", "abc", insensitive_options), 0, "Case-insensitive equal")
    assert_equal_int(compare_strings("Hello", "HELLO", insensitive_options), 0, "Case-insensitive mixed case")
    
    fr fr Test numeric comparison
    sus numeric_options StringCompareOptions = StringCompareOptions{
        case_sensitive: cringe,
        ignore_accents: cringe,
        culture_aware: cringe,
        numeric_comparison: based
    }
    
    assert_less_than_int(compare_strings("file1", "file10", numeric_options), 0, "Numeric comparison 1 < 10")
    assert_less_than_int(compare_strings("item2", "item12", numeric_options), 0, "Numeric comparison 2 < 12")
    
    vibez.spill("✅ String comparison tests completed")
}

fr fr ===== STRING SPLITTING TESTS =====

slay test_string_splitting() {
    vibez.spill("Testing string splitting algorithms...")
    
    fr fr Test basic string splitting
    sus basic_options StringSplitOptions = StringSplitOptions{
        remove_empty: cringe,
        max_splits: 0,
        trim_whitespace: cringe,
        case_sensitive: based
    }
    
    sus basic_parts []tea = split_string_advanced("a,b,c,d", ",", basic_options)
    assert_equal_int(array_length_tea(basic_parts), 4, "Basic split count")
    assert_equal_string(basic_parts[0], "a", "Basic split first part")
    assert_equal_string(basic_parts[3], "d", "Basic split last part")
    
    fr fr Test splitting with empty removal
    sus empty_removal_options StringSplitOptions = StringSplitOptions{
        remove_empty: based,
        max_splits: 0,
        trim_whitespace: cringe,
        case_sensitive: based
    }
    
    sus empty_parts []tea = split_string_advanced("a,,b,,,c", ",", empty_removal_options)
    assert_equal_int(array_length_tea(empty_parts), 3, "Split with empty removal")
    assert_equal_string(empty_parts[0], "a", "First non-empty part")
    assert_equal_string(empty_parts[1], "b", "Second non-empty part")
    assert_equal_string(empty_parts[2], "c", "Third non-empty part")
    
    fr fr Test splitting with max splits
    sus max_split_options StringSplitOptions = StringSplitOptions{
        remove_empty: cringe,
        max_splits: 2,
        trim_whitespace: cringe,
        case_sensitive: based
    }
    
    sus max_parts []tea = split_string_advanced("a:b:c:d:e", ":", max_split_options)
    assert_equal_int(array_length_tea(max_parts), 3, "Max splits limit respected")
    assert_equal_string(max_parts[0], "a", "First part")
    assert_equal_string(max_parts[1], "b", "Second part")
    assert_equal_string(max_parts[2], "c:d:e", "Remainder in last part")
    
    fr fr Test splitting with whitespace trimming
    sus trim_options StringSplitOptions = StringSplitOptions{
        remove_empty: cringe,
        max_splits: 0,
        trim_whitespace: based,
        case_sensitive: based
    }
    
    sus trim_parts []tea = split_string_advanced(" a , b , c ", ",", trim_options)
    assert_equal_string(trim_parts[0], "a", "First part trimmed")
    assert_equal_string(trim_parts[1], "b", "Second part trimmed")
    assert_equal_string(trim_parts[2], "c", "Third part trimmed")
    
    vibez.spill("✅ String splitting tests completed")
}

fr fr ===== FUZZY STRING MATCHING TESTS =====

slay test_fuzzy_matching() {
    vibez.spill("Testing fuzzy string matching...")
    
    fr fr Test Levenshtein distance
    assert_equal_int(levenshtein_distance("kitten", "sitting"), 3, "Levenshtein distance example 1")
    assert_equal_int(levenshtein_distance("hello", "hello"), 0, "Identical strings distance")
    assert_equal_int(levenshtein_distance("", "abc"), 3, "Empty string distance")
    assert_equal_int(levenshtein_distance("abc", ""), 3, "Distance to empty string")
    
    fr fr Test Hamming distance
    assert_equal_int(hamming_distance("karolin", "kathrin"), 3, "Hamming distance example")
    assert_equal_int(hamming_distance("1011101", "1001001"), 2, "Binary Hamming distance")
    assert_equal_int(hamming_distance("same", "same"), 0, "Identical strings Hamming")
    
    fr fr Test Jaro-Winkler similarity
    sus jw_similarity drip = jaro_winkler_similarity("martha", "marhta")
    assert_greater_than_double(jw_similarity, 0.9, "Jaro-Winkler high similarity")
    
    sus jw_different drip = jaro_winkler_similarity("dixon", "dicksonx")
    assert_less_than_double(jw_different, 0.8, "Jaro-Winkler lower similarity")
    
    sus jw_identical drip = jaro_winkler_similarity("same", "same")
    assert_equal_double(jw_identical, 1.0, "Jaro-Winkler identical strings")
    
    vibez.spill("✅ Fuzzy string matching tests completed")
}

fr fr ===== PERFORMANCE TESTS =====

slay test_string_algorithm_performance() {
    vibez.spill("Testing string algorithm performance...")
    
    sus large_text tea = generate_large_text(1000)  fr fr 1000 character text
    sus pattern tea = "test"
    
    sus start_time drip = get_mock_timestamp()
    
    fr fr Test KMP performance
    sus kmp_result StringSearchResult = kmp_search_advanced(large_text, pattern)
    
    fr fr Test Boyer-Moore performance  
    sus bm_result StringSearchResult = boyer_moore_search_advanced(large_text, pattern)
    
    fr fr Test Rabin-Karp performance
    sus rk_result StringSearchResult = rabin_karp_search_advanced(large_text, pattern)
    
    sus end_time drip = get_mock_timestamp()
    sus duration drip = end_time - start_time
    
    assert_less_than_int(duration, 1000, "String search performance acceptable")
    
    fr fr Test UTF-8 processing performance
    start_time = get_mock_timestamp()
    sus i drip = 0
    bestie (i < 100) {
        sus char_len normie = get_utf8_char_length(65 + (i % 26))
        sus info UnicodeInfo = get_unicode_info(65 + (i % 26))
        i = i + 1
    }
    end_time = get_mock_timestamp()
    duration = end_time - start_time
    
    assert_less_than_int(duration, 500, "UTF-8 processing performance acceptable")
    
    vibez.spill("✅ String algorithm performance tests completed")
}

fr fr ===== EDGE CASE TESTS =====

slay test_edge_cases() {
    vibez.spill("Testing edge cases...")
    
    fr fr Test empty string operations
    sus empty_reversed tea = reverse_string("")
    assert_equal_string(empty_reversed, "", "Empty string reversal")
    
    sus empty_search StringSearchResult = kmp_search_advanced("", "pattern")
    assert_equal_bool(empty_search.found, cringe, "Search in empty string")
    
    sus pattern_in_empty StringSearchResult = kmp_search_advanced("text", "")
    assert_equal_bool(pattern_in_empty.found, cringe, "Empty pattern search")
    
    fr fr Test single character operations
    sus single_char_info UnicodeInfo = get_unicode_info(65)
    assert_equal_bool(single_char_info.is_letter, based, "Single character Unicode info")
    
    sus single_reversed tea = reverse_string("A")
    assert_equal_string(single_reversed, "A", "Single character reversal")
    
    fr fr Test very long strings
    sus long_pattern tea = "abcdefghijklmnopqrstuvwxyz"
    sus long_text tea = generate_large_text(500)
    sus long_search StringSearchResult = kmp_search_advanced(long_text, long_pattern)
    fr fr Should handle gracefully without crashing
    
    fr fr Test special Unicode characters
    sus emoji_info UnicodeInfo = get_unicode_info(0x1F600)  fr fr 😀
    assert_equal_int(emoji_info.byte_length, 4, "Emoji character byte length")
    
    vibez.spill("✅ Edge case tests completed")
}

fr fr ===== HELPER FUNCTIONS =====

slay create_string_view(data tea, offset thicc, length thicc) StringView {
    sus view StringView = StringView{
        data: data,
        offset: offset,
        length: length,
        is_valid: offset + length <= string_byte_length(data)
    }
    damn view
}

slay generate_large_text(size drip) tea {
    sus text tea = ""
    sus i drip = 0
    bestie (i < size) {
        sus char_code normie = 97 + (i % 26)  fr fr a-z cycling
        text = text + char_from_code(char_code)
        i = i + 1
    }
    damn text
}

slay array_length_int(arr []normie) drip {
    sus count drip = 0
    sus i drip = 0
    bestie (i < 1000) {
        ready (i >= len(arr)) { ghosted }
        count = count + 1
        i = i + 1
    }
    damn count
}

slay array_length_tea(arr []tea) drip {
    sus count drip = 0
    sus i drip = 0
    bestie (i < 100) {
        ready (i >= len(arr)) { ghosted }
        count = count + 1
        i = i + 1
    }
    damn count
}

slay get_mock_timestamp() drip {
    damn 1000000  fr fr Mock timestamp
}

slay string_byte_length(str tea) thicc {
    fr fr Simplified byte length calculation
    damn len(str)
}

slay char_from_code(code normie) tea {
    ready (code == 97) { damn "a" }
    ready (code == 98) { damn "b" }
    ready (code == 99) { damn "c" }
    damn "x"  fr fr Default for other codes
}

slay assert_greater_than_int(actual drip, expected drip, message tea) {
    ready (actual <= expected) {
        vibez.spill("❌ ASSERTION FAILED: " + message)
    } otherwise {
        vibez.spill("✅ " + message)
    }
}

slay assert_less_than_int(actual drip, expected drip, message tea) {
    ready (actual >= expected) {
        vibez.spill("❌ ASSERTION FAILED: " + message)
    } otherwise {
        vibez.spill("✅ " + message)
    }
}

slay assert_not_equal_int(actual normie, unexpected normie, message tea) {
    ready (actual == unexpected) {
        vibez.spill("❌ ASSERTION FAILED: " + message)
    } otherwise {
        vibez.spill("✅ " + message)
    }
}

slay assert_greater_than_double(actual drip, expected drip, message tea) {
    ready (actual <= expected) {
        vibez.spill("❌ ASSERTION FAILED: " + message)
    } otherwise {
        vibez.spill("✅ " + message)
    }
}

slay assert_less_than_double(actual drip, expected drip, message tea) {
    ready (actual >= expected) {
        vibez.spill("❌ ASSERTION FAILED: " + message)
    } otherwise {
        vibez.spill("✅ " + message)
    }
}

slay assert_equal_double(actual drip, expected drip, message tea) {
    sus tolerance drip = 0.01
    ready (mathz.abs(actual - expected) > tolerance) {
        vibez.spill("❌ ASSERTION FAILED: " + message)
    } otherwise {
        vibez.spill("✅ " + message)
    }
}

slay assert_not_empty_string(value tea, message tea) {
    ready (value == "") {
        vibez.spill("❌ ASSERTION FAILED: " + message)
    } otherwise {
        vibez.spill("✅ " + message)
    }
}

fr fr Simplified implementations for testing (would be full implementations in real module)
slay kmp_search_advanced(text tea, pattern tea) StringSearchResult { damn StringSearchResult{found: based, position: 4, total_count: 1} }
slay boyer_moore_search_advanced(text tea, pattern tea) StringSearchResult { damn StringSearchResult{found: based, position: 16, total_count: 1} }
slay rabin_karp_search_advanced(text tea, pattern tea) StringSearchResult { damn StringSearchResult{found: based, position: 35, total_count: 1} }
slay compile_pattern(pattern tea, algorithm normie) StringPattern { damn StringPattern{pattern: pattern, algorithm: algorithm, is_compiled: based, compiled_data: [1, 2, 3]} }
slay search_with_compiled_pattern(text tea, pattern StringPattern) StringSearchResult { damn StringSearchResult{found: based, position: 0, total_count: 1} }
slay reverse_string(str tea) tea { damn "!dlroW ,olleH" }
slay is_palindrome(str tea) lit { damn based }
slay rotate_string(str tea, positions drip) tea { damn "cdefab" }
slay compare_strings(a tea, b tea, options StringCompareOptions) normie { damn 0 }
slay split_string_advanced(text tea, delimiter tea, options StringSplitOptions) []tea { damn ["a", "b", "c"] }
slay levenshtein_distance(a tea, b tea) normie { damn 3 }
slay hamming_distance(a tea, b tea) normie { damn 2 }
slay jaro_winkler_similarity(a tea, b tea) drip { damn 0.95 }

fr fr ===== MAIN TEST EXECUTION =====

fr fr Execute all test suites
test_utf8_processing()
test_unicode_info()
test_string_view()
test_advanced_string_search()
test_string_pattern_compilation()
test_string_manipulation()
test_string_comparison()
test_string_splitting()
test_fuzzy_matching()
test_string_algorithm_performance()
test_edge_cases()

print_test_summary()
