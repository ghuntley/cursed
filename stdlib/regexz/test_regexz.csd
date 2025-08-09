yeet "regexz"
yeet "testz"

fr fr ===== REGEXZ MODULE TESTS =====

test_start("Basic Pattern Matching")

fr fr Test simple patterns
assert_true(regex_match("hello", "hello"))
assert_true(regex_match("hello world", "hello"))
assert_false(regex_match("hello", "world"))

fr fr Test alpha patterns
assert_true(regex_match("HelloWorld", "^[a-zA-Z]+$"))
assert_false(regex_match("Hello123", "^[a-zA-Z]+$"))

fr fr Test numeric patterns
assert_true(regex_match("12345", "^[0-9]+$"))
assert_false(regex_match("123a45", "^[0-9]+$"))

test_start("Email Validation")

assert_true(regex_match("user@example.com", "^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\\.[a-zA-Z]{2,}$"))
assert_true(is_email_format("test@domain.org"))
assert_false(is_email_format("invalid.email"))
assert_false(is_email_format("@domain.com"))
assert_false(is_email_format("user@"))

test_start("URL Validation")

assert_true(regex_match("http://example.com", "^https?://.*"))
assert_true(regex_match("https://secure.site.com", "^https?://.*"))
assert_false(regex_match("ftp://files.com", "^https?://.*"))

test_start("Phone Number Validation")

assert_true(regex_match("123-456-7890", "^[0-9]{3}-[0-9]{3}-[0-9]{4}$"))
assert_true(is_phone_format("555-123-4567"))
assert_false(is_phone_format("123-45-6789"))
assert_false(is_phone_format("abc-def-ghij"))

test_start("Date Format Validation")

assert_true(regex_match("2024-08-10", "^[0-9]{4}-[0-9]{2}-[0-9]{2}$"))
assert_true(is_date_format("2023-12-25"))
assert_false(is_date_format("24-08-10"))
assert_false(is_date_format("2024/08/10"))

test_start("String Replacement")

assert_eq_string(regex_replace("hello world", "world", "universe"), "hello universe")
assert_eq_string(regex_replace("test", "missing", "replacement"), "test")

sus replaced_all tea = regex_replace_all("foo bar foo", "foo", "baz")
assert_eq_string(replaced_all, "baz bar baz")

test_start("Pattern Finding")

assert_eq_int(regex_find("hello world", "world"), 6)
assert_eq_int(regex_find("test", "missing"), -1)
assert_eq_int(regex_find("anything", ".*"), 0)

test_start("Text Extraction")

sus emails []tea = regex_extract_emails("Contact us at info@company.com or support@help.org")
assert_eq_int(len(emails), 2)

sus urls []tea = regex_extract_urls("Visit http://example.com or https://secure.site.com")
assert_eq_int(len(urls), 2)

sus numbers []tea = regex_extract_numbers("The year 2024 has 365 days")
assert_eq_int(len(numbers), 2)

sus words []tea = regex_extract_words("Hello 123 World!")
assert_eq_int(len(words), 2)

test_start("Character Type Detection")

assert_true(is_alpha_only("HelloWorld"))
assert_false(is_alpha_only("Hello123"))

assert_true(is_numeric_only("12345"))
assert_false(is_numeric_only("123a45"))

assert_true(is_alphanumeric_only("Hello123"))
assert_false(is_alphanumeric_only("Hello-123"))

test_start("IP Address Validation")

assert_true(validate_ip_address("192.168.1.1"))
assert_true(validate_ip_address("255.255.255.255"))
assert_false(validate_ip_address("256.1.1.1"))
assert_false(validate_ip_address("192.168.1"))

test_start("MAC Address Validation")

assert_true(validate_mac_address("AA:BB:CC:DD:EE:FF"))
assert_true(validate_mac_address("00:11:22:33:44:55"))
assert_false(validate_mac_address("AA-BB-CC-DD-EE-FF"))
assert_false(validate_mac_address("AA:BB:CC:DD:EE"))

test_start("Credit Card Validation")

assert_true(validate_credit_card("1234567890123456"))
assert_true(validate_credit_card("1234 5678 9012 3456"))
assert_true(validate_credit_card("1234-5678-9012-3456"))
assert_false(validate_credit_card("123456789012345a"))

test_start("Pattern Counting")

assert_eq_int(regex_count_matches("hello hello hello", "hello"), 3)
assert_eq_int(regex_count_matches("abcabc", "abc"), 2)
assert_eq_int(regex_count_matches("test", "missing"), 0)

test_start("Text Splitting")

sus parts []tea = regex_split("a,b,c", ",")
assert_eq_int(len(parts), 3)

sus words_split []tea = regex_split("one two three", " ")
assert_eq_int(len(words_split), 3)

test_start("Regex Escaping")

sus escaped tea = regex_escape("hello.world*")
assert_true(contains_substring(escaped, "\\."))
assert_true(contains_substring(escaped, "\\*"))

print_test_summary()
