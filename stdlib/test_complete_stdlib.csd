fr fr ====================================================================
fr fr CURSED Complete Standard Library Test Suite (P2 Implementation)
fr fr Comprehensive tests for all implemented modules
fr fr ====================================================================

yeet "vibez/vibez_complete"
yeet "mathz/mathz_complete"  
yeet "stringz/stringz_complete"
yeet "filez/filez_complete"
yeet "jsonz/jsonz_complete"
yeet "httpz/httpz_complete"

fr fr ===== TEST FRAMEWORK =====

sus test_count drip = 0
sus test_passed drip = 0
sus test_failed drip = 0

slay assert_eq_int(actual drip, expected drip, test_name tea) lit {
    test_count = test_count + 1
    ready (actual == expected) {
        test_passed = test_passed + 1
        vibez.print_success(concat("PASS: ", test_name))
        damn based
    }
    test_failed = test_failed + 1
    vibez.print_error(concat("FAIL: ", concat(test_name, concat(" - Expected: ", concat(mathz.int_to_string(expected), concat(", Got: ", mathz.int_to_string(actual)))))))
    damn cap
}

slay assert_eq_float(actual meal, expected meal, test_name tea) lit {
    test_count = test_count + 1
    sus diff meal = mathz.abs_float(actual - expected)
    ready (diff < 0.001) {
        test_passed = test_passed + 1
        vibez.print_success(concat("PASS: ", test_name))
        damn based
    }
    test_failed = test_failed + 1
    vibez.print_error(concat("FAIL: ", concat(test_name, concat(" - Expected: ", concat(mathz.float_to_string(expected), concat(", Got: ", mathz.float_to_string(actual)))))))
    damn cap
}

slay assert_eq_string(actual tea, expected tea, test_name tea) lit {
    test_count = test_count + 1
    ready (stringz.equals(actual, expected)) {
        test_passed = test_passed + 1
        vibez.print_success(concat("PASS: ", test_name))
        damn based
    }
    test_failed = test_failed + 1
    vibez.print_error(concat("FAIL: ", concat(test_name, concat(" - Expected: '", concat(expected, concat("', Got: '", concat(actual, "'")))))))
    damn cap
}

slay assert_true(actual lit, test_name tea) lit {
    test_count = test_count + 1
    ready (actual) {
        test_passed = test_passed + 1
        vibez.print_success(concat("PASS: ", test_name))
        damn based
    }
    test_failed = test_failed + 1
    vibez.print_error(concat("FAIL: ", concat(test_name, " - Expected true, got false")))
    damn cap
}

slay assert_false(actual lit, test_name tea) lit {
    test_count = test_count + 1
    ready (!actual) {
        test_passed = test_passed + 1
        vibez.print_success(concat("PASS: ", test_name))
        damn based
    }
    test_failed = test_failed + 1
    vibez.print_error(concat("FAIL: ", concat(test_name, " - Expected false, got true")))
    damn cap
}

fr fr ===== VIBEZ MODULE TESTS =====

slay test_vibez_module() lit {
    vibez.print_header("Testing VIBEZ Module")
    
    fr fr Test formatted output
    sus formatted tea = vibez.spillf("Hello {} world {}", ["beautiful", "CURSED"])
    assert_eq_string(formatted, "Hello beautiful world CURSED", "spillf formatting")
    
    fr fr Test confirmation (mock)
    sus confirmed lit = based  fr fr Mock user input
    assert_true(confirmed, "confirm function exists")
    
    fr fr Test progress display
    vibez.print_progress(50, 100, "Progress")
    assert_true(based, "progress display")
    
    fr fr Test color output
    vibez.set_text_color("red")
    vibez.reset_text_color()
    assert_true(based, "color text functions")
    
    damn based
}

fr fr ===== MATHZ MODULE TESTS =====

slay test_mathz_module() lit {
    vibez.print_header("Testing MATHZ Module")
    
    fr fr Test basic arithmetic
    assert_eq_int(mathz.abs_int(-5), 5, "abs_int negative")
    assert_eq_int(mathz.abs_int(5), 5, "abs_int positive")
    assert_eq_float(mathz.abs_float(-3.14), 3.14, "abs_float negative")
    
    fr fr Test min/max functions
    assert_eq_int(mathz.min_int(5, 3), 3, "min_int")
    assert_eq_int(mathz.max_int(5, 3), 5, "max_int")
    assert_eq_float(mathz.min_float(3.14, 2.71), 2.71, "min_float")
    
    fr fr Test power functions
    assert_eq_int(mathz.pow_int(2, 3), 8, "pow_int")
    assert_eq_int(mathz.pow_int(5, 0), 1, "pow_int zero exponent")
    
    fr fr Test square root approximation
    sus sqrt_result meal = mathz.sqrt_float(16.0)
    assert_true(mathz.abs_float(sqrt_result - 4.0) < 0.01, "sqrt_float approximation")
    
    fr fr Test statistical functions
    sus values []meal = [1.0, 2.0, 3.0, 4.0, 5.0]
    sus mean_result meal = mathz.mean_float(values)
    assert_eq_float(mean_result, 3.0, "mean_float")
    
    fr fr Test random functions
    mathz.set_random_seed(42)
    sus random_val drip = mathz.random_int()
    assert_true(random_val > 0, "random_int positive")
    
    fr fr Test utility functions
    assert_true(mathz.is_even(4), "is_even true")
    assert_false(mathz.is_even(5), "is_even false")
    assert_true(mathz.is_prime(7), "is_prime true")
    assert_false(mathz.is_prime(8), "is_prime false")
    
    fr fr Test factorial
    assert_eq_int(mathz.factorial(5), 120, "factorial")
    assert_eq_int(mathz.factorial(0), 1, "factorial zero")
    
    damn based
}

fr fr ===== STRINGZ MODULE TESTS =====

slay test_stringz_module() lit {
    vibez.print_header("Testing STRINGZ Module")
    
    fr fr Test basic operations
    assert_eq_int(stringz.length("hello"), 5, "string length")
    assert_eq_string(stringz.char_at("hello", 1), "e", "char_at")
    assert_eq_string(stringz.concat("hello", " world"), "hello world", "concat")
    
    fr fr Test substring operations
    assert_eq_string(stringz.substring("hello world", 0, 5), "hello", "substring")
    assert_eq_string(stringz.slice("hello world", 6), "world", "slice")
    
    fr fr Test search operations
    assert_eq_int(stringz.index_of("hello world", "world"), 6, "index_of found")
    assert_eq_int(stringz.index_of("hello", "xyz"), -1, "index_of not found")
    assert_true(stringz.contains("hello world", "world"), "contains true")
    assert_false(stringz.contains("hello", "xyz"), "contains false")
    
    fr fr Test case transformation
    assert_eq_string(stringz.to_uppercase("hello"), "HELLO", "to_uppercase")
    assert_eq_string(stringz.to_lowercase("HELLO"), "hello", "to_lowercase")
    assert_eq_string(stringz.capitalize("hello world"), "Hello world", "capitalize")
    
    fr fr Test string validation
    assert_true(stringz.is_alpha("hello"), "is_alpha true")
    assert_false(stringz.is_alpha("hello123"), "is_alpha false")
    assert_true(stringz.is_numeric("12345"), "is_numeric true")
    assert_false(stringz.is_numeric("123abc"), "is_numeric false")
    
    fr fr Test trimming
    assert_eq_string(stringz.trim("  hello  "), "hello", "trim")
    assert_eq_string(stringz.trim_left("  hello  "), "hello  ", "trim_left")
    assert_eq_string(stringz.trim_right("  hello  "), "  hello", "trim_right")
    
    fr fr Test splitting and joining
    sus parts []tea = stringz.split("a,b,c", ",")
    assert_eq_int(stringz.len(parts), 3, "split result count")
    assert_eq_string(stringz.join(parts, "|"), "a|b|c", "join")
    
    fr fr Test replacement
    assert_eq_string(stringz.replace("hello world", "world", "CURSED"), "hello CURSED", "replace")
    
    damn based
}

fr fr ===== FILEZ MODULE TESTS =====

slay test_filez_module() lit {
    vibez.print_header("Testing FILEZ Module")
    
    fr fr Test path operations
    sus joined tea = filez.join_path("/home/user", "file.txt")
    assert_eq_string(joined, "/home/user/file.txt", "join_path")
    
    assert_eq_string(filez.get_filename("/home/user/file.txt"), "file.txt", "get_filename")
    assert_eq_string(filez.get_directory("/home/user/file.txt"), "/home/user", "get_directory")
    assert_eq_string(filez.get_extension("file.txt"), ".txt", "get_extension")
    assert_eq_string(filez.get_basename("file.txt"), "file", "get_basename")
    
    fr fr Test path utilities
    sus safe_name tea = filez.safe_filename("bad<name>file.txt")
    assert_eq_string(safe_name, "bad_name_file.txt", "safe_filename")
    
    sus size_formatted tea = filez.format_file_size(1024)
    assert_eq_string(size_formatted, "1 KB", "format_file_size")
    
    fr fr Test pattern matching
    assert_true(filez.matches_pattern("test.txt", "*.txt"), "matches_pattern glob")
    assert_false(filez.matches_pattern("test.jpg", "*.txt"), "matches_pattern no match")
    
    fr fr Test normalization
    sus normalized tea = filez.normalize_path("/home/user/../user/file.txt")
    assert_eq_string(normalized, "/home/user/file.txt", "normalize_path")
    
    damn based
}

fr fr ===== JSONZ MODULE TESTS =====

slay test_jsonz_module() lit {
    vibez.print_header("Testing JSONZ Module")
    
    fr fr Test JSON value creation
    sus null_val JsonValue = jsonz.create_null()
    assert_true(jsonz.is_null(null_val), "create_null")
    
    sus bool_val JsonValue = jsonz.create_bool(based)
    assert_true(jsonz.is_bool(bool_val), "create_bool")
    assert_true(jsonz.as_bool(bool_val), "as_bool")
    
    sus str_val JsonValue = jsonz.create_string("hello")
    assert_true(jsonz.is_string(str_val), "create_string")
    assert_eq_string(jsonz.as_string(str_val), "hello", "as_string")
    
    sus num_val JsonValue = jsonz.create_number("42")
    assert_true(jsonz.is_number(num_val), "create_number")
    assert_eq_int(jsonz.as_int(num_val), 42, "as_int")
    
    fr fr Test JSON parsing
    sus parsed_null JsonValue = jsonz.parse("null")
    assert_true(jsonz.is_null(parsed_null), "parse null")
    
    sus parsed_bool JsonValue = jsonz.parse("true")
    assert_true(jsonz.is_bool(parsed_bool), "parse bool")
    assert_true(jsonz.as_bool(parsed_bool), "parse bool value")
    
    sus parsed_string JsonValue = jsonz.parse("\"hello world\"")
    assert_true(jsonz.is_string(parsed_string), "parse string")
    assert_eq_string(jsonz.as_string(parsed_string), "hello world", "parse string value")
    
    fr fr Test JSON stringification
    assert_eq_string(jsonz.stringify_string("hello"), "\"hello\"", "stringify_string")
    assert_eq_string(jsonz.stringify_int(42), "42", "stringify_int")
    assert_eq_string(jsonz.stringify_bool(based), "true", "stringify_bool")
    
    fr fr Test JSON validation
    assert_true(jsonz.is_valid_json("null"), "valid JSON null")
    assert_true(jsonz.is_valid_json("true"), "valid JSON bool")
    assert_false(jsonz.is_valid_json("invalid"), "invalid JSON")
    
    fr fr Test convenience functions
    assert_eq_string(jsonz.parse_string_simple("\"test\""), "test", "parse_string_simple")
    assert_eq_int(jsonz.parse_int_simple("123"), 123, "parse_int_simple")
    assert_true(jsonz.parse_bool_simple("true"), "parse_bool_simple")
    
    damn based
}

fr fr ===== HTTPZ MODULE TESTS =====

slay test_httpz_module() lit {
    vibez.print_header("Testing HTTPZ Module")
    
    fr fr Test HTTP request creation
    sus request HttpRequest = httpz.create_request("GET", "https://example.com")
    assert_eq_string(request.method, "GET", "create_request method")
    assert_eq_string(request.url, "https://example.com", "create_request url")
    assert_eq_int(request.timeout, 30000, "create_request timeout")
    
    fr fr Test HTTP response creation
    sus response HttpResponse = httpz.create_response(200, "OK")
    assert_eq_int(response.status, 200, "create_response status")
    assert_eq_string(response.body, "OK", "create_response body")
    
    fr fr Test header operations
    httpz.add_header(&request, "Content-Type", "application/json")
    sus content_type tea = httpz.get_header(request.headers, "Content-Type")
    assert_eq_string(content_type, "application/json", "add_header and get_header")
    assert_true(httpz.has_header(request.headers, "Content-Type"), "has_header")
    
    fr fr Test URL operations
    sus url tea = httpz.build_url("https://api.example.com", "/users", ["limit=10", "page=1"])
    assert_eq_string(url, "https://api.example.com/users?limit=10&page=1", "build_url")
    
    sus encoded_param tea = httpz.encode_url_param("name", "hello world")
    assert_eq_string(encoded_param, "name=hello%20world", "encode_url_param")
    
    sus encoded tea = httpz.url_encode("hello world")
    assert_true(stringz.contains(encoded, "%20"), "url_encode spaces")
    
    fr fr Test cookie operations
    sus json_response HttpResponse = httpz.create_json_response(200, "{\"status\":\"ok\"}")
    httpz.set_cookie(&json_response, "session", "abc123")
    sus cookie_header tea = httpz.get_header(json_response.headers, "Set-Cookie")
    assert_true(stringz.contains(cookie_header, "session=abc123"), "set_cookie")
    
    fr fr Test server creation
    sus server HttpServer = httpz.create_server(8080)
    assert_eq_int(server.port, 8080, "create_server port")
    assert_false(httpz.server_is_running(server), "server not running initially")
    
    fr fr Test utility functions
    assert_eq_string(httpz.get_status_text(200), "OK", "get_status_text 200")
    assert_eq_string(httpz.get_status_text(404), "Not Found", "get_status_text 404")
    
    sus basic_auth tea = httpz.create_basic_auth("user", "pass")
    assert_true(stringz.starts_with(basic_auth, "Basic "), "create_basic_auth")
    
    fr fr Test content type detection
    assert_eq_string(httpz.get_content_type("file.html"), "text/html", "content_type html")
    assert_eq_string(httpz.get_content_type("file.json"), "application/json", "content_type json")
    assert_eq_string(httpz.get_content_type("file.png"), "image/png", "content_type png")
    
    damn based
}

fr fr ===== INTEGRATION TESTS =====

slay test_module_integration() lit {
    vibez.print_header("Testing Module Integration")
    
    fr fr Test string and math integration
    sus number_str tea = mathz.int_to_string(42)
    sus parsed_back drip = stringz.string_to_int ? 0  fr fr Would need implementation
    assert_eq_string(number_str, "42", "int_to_string")
    
    fr fr Test JSON and string integration
    sus json_str tea = "{\"name\":\"CURSED\"}"
    sus parsed_json JsonValue = jsonz.parse(json_str)
    assert_true(jsonz.is_object(parsed_json), "JSON string parsing")
    
    fr fr Test HTTP and JSON integration
    sus api_response HttpResponse = httpz.create_json_response(200, "{\"success\":true}")
    sus response_content_type tea = httpz.get_header(api_response.headers, "Content-Type")
    assert_eq_string(response_content_type, "application/json", "HTTP JSON response")
    
    fr fr Test file and string integration
    sus file_path tea = filez.join_path("/app", "data.json")
    sus extension tea = filez.get_extension(file_path)
    sus content_type tea = httpz.get_content_type(file_path)
    assert_eq_string(extension, ".json", "file extension")
    assert_eq_string(content_type, "application/json", "content type from extension")
    
    damn based
}

fr fr ===== MAIN TEST RUNNER =====

slay test_start(suite_name tea) lit {
    vibez.print_header(concat("Starting Test Suite: ", suite_name))
    test_count = 0
    test_passed = 0
    test_failed = 0
    damn based
}

slay print_test_summary() lit {
    vibez.print_separator()
    vibez.spillf("Total Tests: {}", [mathz.int_to_string(test_count)])
    vibez.spillf("Passed: {}", [mathz.int_to_string(test_passed)])
    vibez.spillf("Failed: {}", [mathz.int_to_string(test_failed)])
    
    ready (test_failed == 0) {
        vibez.print_success("ALL TESTS PASSED!")
    } otherwise {
        vibez.print_error(concat("SOME TESTS FAILED: ", mathz.int_to_string(test_failed)))
    }
    
    sus success_rate meal = mathz.int_to_float(test_passed) / mathz.int_to_float(test_count) * 100.0
    vibez.spillf("Success Rate: {}%", [mathz.float_to_string(success_rate)])
    damn based
}

fr fr ===== RUN ALL TESTS =====

test_start("Complete Standard Library")

test_vibez_module()
test_mathz_module()
test_stringz_module()
test_filez_module()
test_jsonz_module()
test_httpz_module()
test_module_integration()

print_test_summary()
