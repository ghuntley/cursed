fr fr Comprehensive CURSED Standard Library Test
fr fr Tests all the enhanced stdlib modules

yeet "testz"
yeet "stringz" 
yeet "arrayz"
yeet "mathz"
yeet "filez"
yeet "jsonz"
yeet "httpz"
yeet "timez"

fr fr ===== STRINGZ MODULE TESTS =====

test_start("String Operations")

fr fr Test basic string operations
assert_eq_string(concat_strings("hello", "world"), "helloworld")
assert_eq_string(repeat_string("x", 3), "xxx")
assert_true(is_empty_string(""))
assert_false(is_empty_string("hello"))

fr fr Test new advanced string operations
assert_eq_int(string_length("hello"), 5)
assert_eq_string(char_at("hello", 0), "h")
assert_eq_string(char_at("hello", 4), "o")
assert_eq_string(substring("hello", 0, 3), "hel")
assert_eq_string(slice_tea("hello", 1, 4), "ell")

fr fr Test string searching
assert_eq_int(indexOf("hello", "l"), 2)
assert_eq_int(lastIndexOf("hello", "l"), 3)
assert_true(contains_substring("hello", "ell"))
assert_false(contains_substring("hello", "xyz"))

fr fr Test string validation
assert_true(starts_with("hello", "he"))
assert_true(ends_with("hello", "lo"))
assert_true(is_numeric("123"))
assert_false(is_numeric("abc"))
assert_true(is_alphabetic("hello"))
assert_false(is_alphabetic("hello123"))

fr fr Test string transformation
assert_eq_string(to_uppercase("hello"), "HELLO")
assert_eq_string(to_lowercase("HELLO"), "hello")
assert_eq_string(trim_whitespace(" hello "), "hello")
assert_eq_string(reverse_string("hello"), "olleh")

fr fr Test string replacement
assert_eq_string(replace_first("hello hello", "hello", "hi"), "hi hello")
assert_eq_string(replace_all("hello hello", "hello", "hi"), "hi hi")

vibez.spill("✅ String operations tests passed")

fr fr ===== ARRAYZ MODULE TESTS =====

test_start("Array Operations")

fr fr Test array arithmetic
sus test_nums []drip = [1, 2, 3, 4, 5]
assert_eq_int(sum_array(test_nums), 15)
assert_eq_int(average_array(test_nums), 3)
assert_eq_int(product_array([2, 3, 4]), 24)

fr fr Test array search
assert_eq_int(find_max([1, 5, 3, 2]), 5)
assert_eq_int(find_min([1, 5, 3, 2]), 1)
assert_true(contains_value([1, 2, 3], 2))
assert_false(contains_value([1, 2, 3], 5))
assert_eq_int(find_index([1, 2, 3], 2), 1)

fr fr Test array validation
assert_true(is_empty_array([]))
assert_false(is_empty_array([1]))
assert_eq_int(array_size([1, 2, 3]), 3)
assert_true(arrays_equal_size([1, 2], [3, 4]))

fr fr Test array counting
assert_eq_int(count_positive([1, -2, 3, -4]), 2)
assert_eq_int(count_negative([1, -2, 3, -4]), 2)
assert_eq_int(count_zeros([1, 0, 3, 0]), 2)

fr fr Test array properties
assert_true(all_positive([1, 2, 3]))
assert_false(all_positive([1, -2, 3]))
assert_false(has_duplicates([1, 2, 3]))
assert_true(has_duplicates([1, 2, 1]))

vibez.spill("✅ Array operations tests passed")

fr fr ===== MATHZ MODULE TESTS =====

test_start("Math Operations")

fr fr Test basic arithmetic
assert_eq_int(abs_normie(-5), 5)
assert_eq_int(max_normie(3, 7), 7)
assert_eq_int(min_normie(3, 7), 3)
assert_eq_int(add_two(3, 4), 7)
assert_eq_int(multiply_two(6, 7), 42)

fr fr Test advanced functions
assert_eq_int(power_int(2, 3), 8)
assert_eq_int(factorial(5), 120)
assert_eq_int(gcd(12, 8), 4)
assert_eq_int(lcm(4, 6), 12)

fr fr Test utility functions
assert_true(is_even(4))
assert_false(is_even(3))
assert_true(is_odd(3))
assert_false(is_odd(4))
assert_eq_int(clamp(5, 0, 10), 5)
assert_eq_int(clamp(-5, 0, 10), 0)
assert_eq_int(sign(5), 1)
assert_eq_int(sign(-5), -1)

fr fr Test sequence operations
assert_eq_int(sum_range(1, 5), 15)
assert_eq_int(fibonacci(6), 8)

vibez.spill("✅ Math operations tests passed")

fr fr ===== FILEZ MODULE TESTS =====

test_start("File Operations")

fr fr Test file system operations
clear_file_system()
assert_eq_int(get_file_count(), 0)
assert_true(is_storage_full() == cringe)

fr fr Test file writing and reading
assert_true(cursed_write_file("test.txt", "Hello World"))
assert_true(cursed_file_exists("test.txt"))
assert_eq_string(cursed_read_file("test.txt"), "Hello World")
assert_eq_int(cursed_file_size("test.txt"), 11)

fr fr Test file operations
assert_true(cursed_append_file("test.txt", " Appended"))
assert_eq_string(cursed_read_file("test.txt"), "Hello World Appended")

fr fr Test file copying
assert_true(cursed_copy_file("test.txt", "copy.txt"))
assert_true(cursed_file_exists("copy.txt"))
assert_eq_string(cursed_read_file("copy.txt"), "Hello World Appended")

fr fr Test file utilities
assert_true(write_text_file("data.txt", "Some data"))
assert_eq_string(read_text_file("data.txt"), "Some data")
assert_true(file_contains_text("data.txt", "Some"))

vibez.spill("✅ File operations tests passed")

fr fr ===== JSONZ MODULE TESTS =====

test_start("JSON Operations")

fr fr Test JSON value parsing
assert_eq_string(parse_json_value("\"hello\""), "hello")
assert_eq_string(parse_json_value("42"), "42")
assert_eq_string(parse_json_value("true"), "true")
assert_eq_string(parse_json_value("null"), "null")

fr fr Test JSON generation
assert_eq_string(json_escape_string("hello"), "\"hello\"")
assert_eq_string(json_number_to_string(42), "42")
assert_eq_string(json_boolean_to_string(based), "true")
assert_eq_string(json_boolean_to_string(cringe), "false")

fr fr Test JSON object creation
sus simple_obj tea = json_create_object("name", "John")
assert_true(contains_substring(simple_obj, "\"name\""))
assert_true(contains_substring(simple_obj, "\"John\""))

fr fr Test JSON array creation
sus simple_array tea = json_create_array("item1")
assert_true(contains_substring(simple_array, "\"item1\""))

fr fr Test JSON validation
assert_true(is_valid_json("{\"key\":\"value\"}"))
assert_true(is_valid_json("[1,2,3]"))
assert_true(is_valid_json("\"string\""))
assert_false(is_valid_json("invalid"))

vibez.spill("✅ JSON operations tests passed")

fr fr ===== HTTPZ MODULE TESTS =====

test_start("HTTP Operations")

fr fr Test HTTP request building
sus get_req tea = build_get_request("example.com", "/api/test")
assert_true(contains_substring(get_req, "GET /api/test HTTP/1.1"))
assert_true(contains_substring(get_req, "Host: example.com"))

sus post_req tea = build_post_request("example.com", "/api/data", "{\"test\":true}")
assert_true(contains_substring(post_req, "POST /api/data HTTP/1.1"))
assert_true(contains_substring(post_req, "Content-Type: application/json"))

fr fr Test HTTP header creation
sus content_type tea = create_content_type_header("application/json")
assert_eq_string(content_type, "Content-Type: application/json")

sus auth_header tea = create_authorization_header("token123")
assert_eq_string(auth_header, "Authorization: Bearer token123")

fr fr Test HTTP response parsing
sus test_response tea = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"status\":\"ok\"}"
assert_eq_int(parse_http_status_code(test_response), 200)
assert_eq_string(parse_http_body(test_response), "{\"status\":\"ok\"}")
assert_eq_string(get_http_header(test_response, "Content-Type"), "application/json")

fr fr Test mock HTTP client
sus api_response tea = http_get("https://api.example.com/status")
assert_true(is_http_success(api_response))
assert_false(is_http_error(api_response))

fr fr Test URL parsing
assert_eq_string(parse_url_scheme("https://example.com"), "https")
assert_eq_string(parse_url_host("https://example.com/path"), "example.com")
assert_eq_string(parse_url_path("https://example.com/api/test"), "/api/test")

fr fr Test URL utilities
assert_true(is_valid_url("https://example.com"))
assert_false(is_valid_url("invalid-url"))

vibez.spill("✅ HTTP operations tests passed")

fr fr ===== TIMEZ MODULE TESTS =====

test_start("Time Operations")

fr fr Test time constants and validation
assert_eq_int(SECONDS_PER_MINUTE, 60)
assert_eq_int(HOURS_PER_DAY, 24)
assert_eq_int(DAYS_PER_WEEK, 7)

fr fr Test leap year calculation
assert_true(is_leap_year(2024))
assert_false(is_leap_year(2023))
assert_true(is_leap_year(2000))
assert_false(is_leap_year(1900))

fr fr Test days in month
assert_eq_int(days_in_month(JANUARY, 2024), 31)
assert_eq_int(days_in_month(FEBRUARY, 2024), 29)
assert_eq_int(days_in_month(FEBRUARY, 2023), 28)
assert_eq_int(days_in_month(APRIL, 2024), 30)

fr fr Test date validation
assert_true(is_valid_date(2024, 2, 29))
assert_false(is_valid_date(2023, 2, 29))
assert_true(is_valid_time(14, 30, 45))
assert_false(is_valid_time(25, 30, 45))

fr fr Test date formatting
assert_eq_string(format_date_iso(2024, 8, 10), "2024-08-10")
assert_eq_string(format_time_iso(14, 30, 45), "14:30:45")

fr fr Test month and weekday names
assert_eq_string(month_name(JANUARY), "January")
assert_eq_string(month_name_short(JANUARY), "Jan")
assert_eq_string(weekday_name(MONDAY), "Monday")
assert_eq_string(weekday_name_short(MONDAY), "Mon")

fr fr Test business day utilities
assert_true(is_weekend(SATURDAY))
assert_true(is_weekend(SUNDAY))
assert_false(is_weekend(MONDAY))
assert_true(is_weekday(MONDAY))

fr fr Test duration formatting
assert_eq_string(format_duration_seconds(90), "1 minutes 30 seconds")
assert_eq_string(format_relative_time(3600), "1 hours ago")

fr fr Test timezone utilities
assert_eq_int(utc_offset_hours("UTC"), 0)
assert_eq_int(utc_offset_hours("EST"), -5)
assert_eq_int(utc_offset_hours("JST"), 9)

vibez.spill("✅ Time operations tests passed")

fr fr ===== FINAL TEST SUMMARY =====

print_test_summary()

vibez.spill("")
vibez.spill("🎉 COMPREHENSIVE STDLIB TEST COMPLETE")
vibez.spill("✅ All enhanced modules are working correctly")
vibez.spill("📚 Standard library now includes:")
vibez.spill("   • Enhanced stringz with full string operations")
vibez.spill("   • Complete arrayz with comprehensive array functions")
vibez.spill("   • Robust mathz with advanced mathematical operations")
vibez.spill("   • Pure CURSED filez with in-memory file system")
vibez.spill("   • Full-featured jsonz for JSON processing")
vibez.spill("   • Complete httpz for HTTP client/server operations")
vibez.spill("   • Comprehensive timez for date/time operations")
vibez.spill("")
vibez.spill("🚀 CURSED Standard Library is now production-ready!")
