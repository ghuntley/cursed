yeet "testz"
yeet "validation"

// Test email validation
test_start("Email validation tests")

// Valid emails
sus valid_email_result ValidationResult = validate_email("test@example.com")
assert_true(valid_email_result.is_valid)

sus valid_email_result2 ValidationResult = validate_email("user.name@domain.co.uk")
assert_true(valid_email_result2.is_valid)

sus valid_email_result3 ValidationResult = validate_email("simple@test.org")
assert_true(valid_email_result3.is_valid)

// Invalid emails
sus invalid_email_result ValidationResult = validate_email("")
assert_false(invalid_email_result.is_valid)

sus invalid_email_result2 ValidationResult = validate_email("no-at-symbol")
assert_false(invalid_email_result2.is_valid)

sus invalid_email_result3 ValidationResult = validate_email("@no-local-part.com")
assert_false(invalid_email_result3.is_valid)

sus invalid_email_result4 ValidationResult = validate_email("no-domain@")
assert_false(invalid_email_result4.is_valid)

sus invalid_email_result5 ValidationResult = validate_email("multiple@@symbols.com")
assert_false(invalid_email_result5.is_valid)

sus invalid_email_result6 ValidationResult = validate_email("no-dot@nodomain")
assert_false(invalid_email_result6.is_valid)

sus invalid_email_result7 ValidationResult = validate_email("consecutive..dots@domain.com")
assert_false(invalid_email_result7.is_valid)

sus invalid_email_result8 ValidationResult = validate_email("starts.with.dot@.domain.com")
assert_false(invalid_email_result8.is_valid)

sus invalid_email_result9 ValidationResult = validate_email("ends.with.dot@domain.com.")
assert_false(invalid_email_result9.is_valid)

// Test phone number validation
test_start("Phone number validation tests")

// Valid phone numbers
sus valid_phone_result ValidationResult = validate_phone_number("1234567890")
assert_true(valid_phone_result.is_valid)

sus valid_phone_result2 ValidationResult = validate_phone_number("+1 (555) 123-4567")
assert_true(valid_phone_result2.is_valid)

sus valid_phone_result3 ValidationResult = validate_phone_number("+44 20 7123 4567")
assert_true(valid_phone_result3.is_valid)

sus valid_phone_result4 ValidationResult = validate_phone_number("555-123-4567")
assert_true(valid_phone_result4.is_valid)

sus valid_phone_result5 ValidationResult = validate_phone_number("(555) 123 4567")
assert_true(valid_phone_result5.is_valid)

// Invalid phone numbers
sus invalid_phone_result ValidationResult = validate_phone_number("")
assert_false(invalid_phone_result.is_valid)

sus invalid_phone_result2 ValidationResult = validate_phone_number("123")
assert_false(invalid_phone_result2.is_valid)

sus invalid_phone_result3 ValidationResult = validate_phone_number("123456789012345678901234567890")
assert_false(invalid_phone_result3.is_valid)

sus invalid_phone_result4 ValidationResult = validate_phone_number("abc-def-ghij")
assert_false(invalid_phone_result4.is_valid)

sus invalid_phone_result5 ValidationResult = validate_phone_number("+0123456789")
assert_false(invalid_phone_result5.is_valid)

// Test URL validation
test_start("URL validation tests")

// Valid URLs
sus valid_url_result ValidationResult = validate_url("https://www.example.com")
assert_true(valid_url_result.is_valid)

sus valid_url_result2 ValidationResult = validate_url("http://test.org/path")
assert_true(valid_url_result2.is_valid)

sus valid_url_result3 ValidationResult = validate_url("https://subdomain.domain.com/path/to/resource")
assert_true(valid_url_result3.is_valid)

sus valid_url_result4 ValidationResult = validate_url("ftp://files.example.com")
assert_true(valid_url_result4.is_valid)

sus valid_url_result5 ValidationResult = validate_url("https://example.com:8080/api")
assert_true(valid_url_result5.is_valid)

sus valid_url_result6 ValidationResult = validate_url("file:///path/to/file")
assert_true(valid_url_result6.is_valid)

sus valid_url_result7 ValidationResult = validate_url("file://localhost/path")
assert_true(valid_url_result7.is_valid)

// Invalid URLs
sus invalid_url_result ValidationResult = validate_url("")
assert_false(invalid_url_result.is_valid)

sus invalid_url_result2 ValidationResult = validate_url("not-a-url")
assert_false(invalid_url_result2.is_valid)

sus invalid_url_result3 ValidationResult = validate_url("http://")
assert_false(invalid_url_result3.is_valid)

sus invalid_url_result4 ValidationResult = validate_url("https://")
assert_false(invalid_url_result4.is_valid)

sus invalid_url_result5 ValidationResult = validate_url("http://.example.com")
assert_false(invalid_url_result5.is_valid)

sus invalid_url_result6 ValidationResult = validate_url("http://example.com.")
assert_false(invalid_url_result6.is_valid)

sus invalid_url_result7 ValidationResult = validate_url("http://-example.com")
assert_false(invalid_url_result7.is_valid)

sus invalid_url_result8 ValidationResult = validate_url("http://example.com-")
assert_false(invalid_url_result8.is_valid)

// Test credit card validation
test_start("Credit card validation tests")

// Valid credit cards (using test numbers)
sus valid_card_result ValidationResult = validate_credit_card("4111111111111111")  // Visa test number
assert_true(valid_card_result.is_valid)

sus valid_card_result2 ValidationResult = validate_credit_card("5555555555554444")  // Mastercard test number
assert_true(valid_card_result2.is_valid)

sus valid_card_result3 ValidationResult = validate_credit_card("378282246310005")  // American Express test number
assert_true(valid_card_result3.is_valid)

sus valid_card_result4 ValidationResult = validate_credit_card("6011111111111117")  // Discover test number
assert_true(valid_card_result4.is_valid)

sus valid_card_result5 ValidationResult = validate_credit_card("4111-1111-1111-1111")  // With hyphens
assert_true(valid_card_result5.is_valid)

sus valid_card_result6 ValidationResult = validate_credit_card("4111 1111 1111 1111")  // With spaces
assert_true(valid_card_result6.is_valid)

// Invalid credit cards
sus invalid_card_result ValidationResult = validate_credit_card("")
assert_false(invalid_card_result.is_valid)

sus invalid_card_result2 ValidationResult = validate_credit_card("123")
assert_false(invalid_card_result2.is_valid)

sus invalid_card_result3 ValidationResult = validate_credit_card("12345678901234567890123456789012345678901234567890")
assert_false(invalid_card_result3.is_valid)

sus invalid_card_result4 ValidationResult = validate_credit_card("4111111111111112")  // Invalid Luhn checksum
assert_false(invalid_card_result4.is_valid)

sus invalid_card_result5 ValidationResult = validate_credit_card("abcd-efgh-ijkl-mnop")
assert_false(invalid_card_result5.is_valid)

sus invalid_card_result6 ValidationResult = validate_credit_card("4111@1111#1111$1111")
assert_false(invalid_card_result6.is_valid)

// Test date format validation
test_start("Date format validation tests")

// Valid dates - ISO format
sus valid_date_result ValidationResult = validate_date_format("2023-12-31", "YYYY-MM-DD")
assert_true(valid_date_result.is_valid)

sus valid_date_result2 ValidationResult = validate_date_format("2023-01-01", "YYYY-MM-DD")
assert_true(valid_date_result2.is_valid)

sus valid_date_result3 ValidationResult = validate_date_format("2023-06-15", "YYYY-MM-DD")
assert_true(valid_date_result3.is_valid)

// Valid dates - US format
sus valid_date_result4 ValidationResult = validate_date_format("12/31/2023", "MM/DD/YYYY")
assert_true(valid_date_result4.is_valid)

sus valid_date_result5 ValidationResult = validate_date_format("01/01/2023", "MM/DD/YYYY")
assert_true(valid_date_result5.is_valid)

sus valid_date_result6 ValidationResult = validate_date_format("06/15/2023", "MM/DD/YYYY")
assert_true(valid_date_result6.is_valid)

// Valid dates - European format
sus valid_date_result7 ValidationResult = validate_date_format("31/12/2023", "DD/MM/YYYY")
assert_true(valid_date_result7.is_valid)

sus valid_date_result8 ValidationResult = validate_date_format("01/01/2023", "DD/MM/YYYY")
assert_true(valid_date_result8.is_valid)

sus valid_date_result9 ValidationResult = validate_date_format("15/06/2023", "DD/MM/YYYY")
assert_true(valid_date_result9.is_valid)

// Invalid dates
sus invalid_date_result ValidationResult = validate_date_format("", "YYYY-MM-DD")
assert_false(invalid_date_result.is_valid)

sus invalid_date_result2 ValidationResult = validate_date_format("2023-12-31", "")
assert_false(invalid_date_result2.is_valid)

sus invalid_date_result3 ValidationResult = validate_date_format("2023/12/31", "YYYY-MM-DD")
assert_false(invalid_date_result3.is_valid)

sus invalid_date_result4 ValidationResult = validate_date_format("2023-13-01", "YYYY-MM-DD")
assert_false(invalid_date_result4.is_valid)

sus invalid_date_result5 ValidationResult = validate_date_format("2023-12-32", "YYYY-MM-DD")
assert_false(invalid_date_result5.is_valid)

sus invalid_date_result6 ValidationResult = validate_date_format("2023-00-01", "YYYY-MM-DD")
assert_false(invalid_date_result6.is_valid)

sus invalid_date_result7 ValidationResult = validate_date_format("2023-12-00", "YYYY-MM-DD")
assert_false(invalid_date_result7.is_valid)

sus invalid_date_result8 ValidationResult = validate_date_format("23-12-31", "YYYY-MM-DD")
assert_false(invalid_date_result8.is_valid)

sus invalid_date_result9 ValidationResult = validate_date_format("2023-1-1", "YYYY-MM-DD")
assert_false(invalid_date_result9.is_valid)

sus invalid_date_result10 ValidationResult = validate_date_format("13/31/2023", "MM/DD/YYYY")
assert_false(invalid_date_result10.is_valid)

sus invalid_date_result11 ValidationResult = validate_date_format("12/32/2023", "MM/DD/YYYY")
assert_false(invalid_date_result11.is_valid)

sus invalid_date_result12 ValidationResult = validate_date_format("32/12/2023", "DD/MM/YYYY")
assert_false(invalid_date_result12.is_valid)

sus invalid_date_result13 ValidationResult = validate_date_format("31/13/2023", "DD/MM/YYYY")
assert_false(invalid_date_result13.is_valid)

sus invalid_date_result14 ValidationResult = validate_date_format("2023-12-31", "INVALID-FORMAT")
assert_false(invalid_date_result14.is_valid)

// Test basic string validation
test_start("Basic string validation tests")

// Length validation
sus length_result ValidationResult = validate_length("test", 4)
assert_true(length_result.is_valid)

sus length_result2 ValidationResult = validate_length("test", 5)
assert_false(length_result2.is_valid)

sus length_result3 ValidationResult = validate_length("", 0)
assert_true(length_result3.is_valid)

// Min length validation
sus min_length_result ValidationResult = validate_min_length("test", 3)
assert_true(min_length_result.is_valid)

sus min_length_result2 ValidationResult = validate_min_length("test", 5)
assert_false(min_length_result2.is_valid)

// Max length validation
sus max_length_result ValidationResult = validate_max_length("test", 5)
assert_true(max_length_result.is_valid)

sus max_length_result2 ValidationResult = validate_max_length("test", 3)
assert_false(max_length_result2.is_valid)

// Not empty validation
sus not_empty_result ValidationResult = validate_not_empty("test")
assert_true(not_empty_result.is_valid)

sus not_empty_result2 ValidationResult = validate_not_empty("")
assert_false(not_empty_result2.is_valid)

// Test numeric validation
test_start("Numeric validation tests")

// Positive validation
sus positive_result ValidationResult = validate_positive(5)
assert_true(positive_result.is_valid)

sus positive_result2 ValidationResult = validate_positive(0)
assert_false(positive_result2.is_valid)

sus positive_result3 ValidationResult = validate_positive(-5)
assert_false(positive_result3.is_valid)

// Negative validation
sus negative_result ValidationResult = validate_negative(-5)
assert_true(negative_result.is_valid)

sus negative_result2 ValidationResult = validate_negative(0)
assert_false(negative_result2.is_valid)

sus negative_result3 ValidationResult = validate_negative(5)
assert_false(negative_result3.is_valid)

// Range validation
sus range_result ValidationResult = validate_range(5, 1, 10)
assert_true(range_result.is_valid)

sus range_result2 ValidationResult = validate_range(0, 1, 10)
assert_false(range_result2.is_valid)

sus range_result3 ValidationResult = validate_range(15, 1, 10)
assert_false(range_result3.is_valid)

// Test boolean validation
test_start("Boolean validation tests")

// Is true validation
sus is_true_result ValidationResult = validate_is_true(based)
assert_true(is_true_result.is_valid)

sus is_true_result2 ValidationResult = validate_is_true(cap)
assert_false(is_true_result2.is_valid)

// Is false validation
sus is_false_result ValidationResult = validate_is_false(cap)
assert_true(is_false_result.is_valid)

sus is_false_result2 ValidationResult = validate_is_false(based)
assert_false(is_false_result2.is_valid)

// Test array validation
test_start("Array validation tests")

// Array not empty
sus empty_array []tea = []
sus non_empty_array []tea = ["test"]

sus array_not_empty_result ValidationResult = validate_array_not_empty(non_empty_array)
assert_true(array_not_empty_result.is_valid)

sus array_not_empty_result2 ValidationResult = validate_array_not_empty(empty_array)
assert_false(array_not_empty_result2.is_valid)

// Array length
sus array_length_result ValidationResult = validate_array_length(non_empty_array, 1)
assert_true(array_length_result.is_valid)

sus array_length_result2 ValidationResult = validate_array_length(non_empty_array, 2)
assert_false(array_length_result2.is_valid)

print_test_summary()
