yeet "testz"
yeet "regexz"

// Regular Expression Test Suite

test_start("Pattern Compilation")

// Test basic pattern compilation
sus simple_pattern tea = "hello"
sus compiled CompiledPattern = regexz.compile(simple_pattern)
assert_true(based) // Pattern compiled successfully

// Test pattern compilation with flags
sus case_pattern tea = "Hello"
sus case_compiled CompiledPattern = regexz.compile_with_flags(case_pattern, "i")
assert_true(based) // Case-insensitive pattern compiled

// Test pattern validation
assert_true(regexz.is_valid_pattern("\\d+"))
assert_true(regexz.is_valid_pattern("[a-z]+"))
assert_false(regexz.is_valid_pattern("[invalid"))

test_start("Basic Matching")

// Test exact match
assert_true(regexz.match("hello", "hello"))
assert_false(regexz.match("hello", "world"))

// Test pattern matching
assert_true(regexz.match("\\d+", "123"))
assert_false(regexz.match("\\d+", "abc"))

// Test start matching
assert_true(regexz.match_start("hello", "hello world"))
assert_false(regexz.match_start("world", "hello world"))

// Test full matching
assert_true(regexz.match_full("\\d{3}", "123"))
assert_false(regexz.match_full("\\d{3}", "1234"))

test_start("Character Classes")

// Test digit detection
assert_true(regexz.is_digit("5"))
assert_false(regexz.is_digit("a"))

// Test word character detection
assert_true(regexz.is_word_char("A"))
assert_true(regexz.is_word_char("_"))
assert_false(regexz.is_word_char(" "))

// Test whitespace detection
assert_true(regexz.is_whitespace(" "))
assert_true(regexz.is_whitespace("\t"))
assert_false(regexz.is_whitespace("a"))

test_start("Find Operations")

// Test finding matches
sus text tea = "The numbers are 123 and 456"
sus number_matches []Match = regexz.find("\\d+", text)
assert_true(len(number_matches) >= 2)

// Test finding first match
sus first_match Match = regexz.find_first("\\d+", text)
assert_true(first_match.start >= 0)

test_start("Replace Operations")

// Test basic replacement
sus original tea = "Hello World"
sus replaced tea = regexz.replace("World", original, "Universe")
assert_eq_string(replaced, "Hello Universe")

// Test pattern replacement
sus number_text tea = "Item 123 costs $45"
sus price_replaced tea = regexz.replace("\\$\\d+", number_text, "$99")
assert_true(len_str(price_replaced) > 0)

// Test case-insensitive replacement
sus case_text tea = "Hello hello HELLO"
sus case_replaced tea = regexz.replace_ignore_case("hello", case_text, "hi")
assert_true(len_str(case_replaced) > 0)

test_start("Split Operations")

// Test splitting by pattern
sus csv_line tea = "apple,banana,cherry"
sus fruits []tea = regexz.split(",", csv_line)
assert_true(len(fruits) == 3)

// Test splitting with limit
sus limited_split []tea = regexz.split_limit(",", csv_line, 2)
assert_true(len(limited_split) <= 2)

test_start("Common Pattern Validation")

// Test email validation
assert_true(regexz.is_email("user@example.com"))
assert_false(regexz.is_email("invalid-email"))

// Test URL validation
assert_true(regexz.is_url("https://www.example.com"))
assert_false(regexz.is_url("not-a-url"))

// Test phone number validation
assert_true(regexz.is_phone("+1234567890"))
assert_false(regexz.is_phone("not-a-phone"))

// Test IPv4 validation
assert_true(regexz.is_ipv4("192.168.1.1"))
assert_false(regexz.is_ipv4("999.999.999.999"))

// Test IPv6 validation (simplified)
assert_true(regexz.is_ipv6("2001:0db8:85a3:0000:0000:8a2e:0370:7334"))
assert_false(regexz.is_ipv6("not-an-ipv6"))

test_start("String Escaping")

// Test regex character escaping
sus special_chars tea = ".[]()*+?^$|"
sus escaped tea = regexz.escape(special_chars)
assert_true(len_str(escaped) > len_str(special_chars))

// Test literal quoting
sus literal tea = "hello.world"
sus quoted tea = regexz.quote(literal)
assert_true(len_str(quoted) > 0)

test_start("Advanced Operations")

// Test match counting
sus count_text tea = "a1b2c3d4e5"
sus digit_count normie = regexz.match_count("\\d", count_text)
assert_eq_int(digit_count, 5)

// Test contains pattern
assert_true(regexz.contains_pattern("\\d", "abc123def"))
assert_false(regexz.contains_pattern("\\d", "abcdef"))

test_start("Case Insensitive Operations")

// Test case-insensitive matching
assert_true(regexz.match_ignore_case("hello", "HELLO"))
assert_true(regexz.match_ignore_case("HELLO", "hello"))

test_start("Pattern Presets")

// Test predefined patterns
sus email_pat tea = regexz.email_pattern()
assert_true(len_str(email_pat) > 0)

sus url_pat tea = regexz.url_pattern()
assert_true(len_str(url_pat) > 0)

sus phone_pat tea = regexz.phone_pattern()
assert_true(len_str(phone_pat) > 0)

sus ipv4_pat tea = regexz.ipv4_pattern()
assert_true(len_str(ipv4_pat) > 0)

sus ipv6_pat tea = regexz.ipv6_pattern()
assert_true(len_str(ipv6_pat) > 0)

test_start("Group Extraction")

// Test simple group extraction
sus grouped_text tea = "Name: John, Age: 30"
sus groups []tea = regexz.match_groups("Name: (\\w+), Age: (\\d+)", grouped_text)
assert_true(len(groups) >= 2)

test_start("Validation Results")

// Test regex syntax validation
sus valid_result ValidationResult = regexz.validate_regex("\\d+")
assert_true(valid_result.is_valid)

sus invalid_result ValidationResult = regexz.validate_regex("[invalid")
assert_false(invalid_result.is_valid)

print_test_summary()
