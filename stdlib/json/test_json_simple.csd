yeet "json"

fr fr ==========================================
fr fr Simple JSON Test (No external dependencies)
fr fr ==========================================

sus test_count normie = 0
sus test_passed normie = 0
sus test_failed normie = 0

slay test_start(name tea) {
    test_count = test_count + 1
    vibez.spill("Running test: " + name)
}

slay test_pass(message tea) {
    test_passed = test_passed + 1
    vibez.spill("  ✓ PASS: " + message)
}

slay test_fail(message tea) {
    test_failed = test_failed + 1
    vibez.spill("  ✗ FAIL: " + message)
}

slay assert_eq_string(actual tea, expected tea) {
    bestie actual == expected {
        test_pass("assert_eq_string: " + actual + " == " + expected)
    } else {
        test_fail("assert_eq_string failed: got " + actual + ", expected " + expected)
    }
}

slay assert_true(value lit) {
    bestie value == based {
        test_pass("assert_true: value is based")
    } else {
        test_fail("assert_true failed: got cap, expected based")
    }
}

slay assert_false(value lit) {
    bestie value == cap {
        test_pass("assert_false: value is cap")
    } else {
        test_fail("assert_false failed: got based, expected cap")
    }
}

slay print_test_summary() {
    vibez.spill("")
    vibez.spill("=== TEST SUMMARY ===")
    vibez.spill("Total tests: " + tea(test_count))
    vibez.spill("Passed: " + tea(test_passed))
    vibez.spill("Failed: " + tea(test_failed))
    
    bestie test_failed == 0 {
        vibez.spill("🎉 ALL TESTS PASSED! 🎉")
    } else {
        vibez.spill("❌ Some tests failed")
    }
}

fr fr ==========================================
fr fr JSON Test Functions
fr fr ==========================================

slay test_json_parse_string() {
    test_start("JSON Parse String Values") fr fr Test parsing quoted strings
    sus str_result tea = json.parse_value("\"hello world\"")
    assert_eq_string(str_result, "hello world")
    
    sus empty_result tea = json.parse_value("\"\"")
    assert_eq_string(empty_result, "")
    
    sus special_result tea = json.parse_value("\"test\"")
    assert_eq_string(special_result, "test")
}

slay test_json_parse_numbers() {
    test_start("JSON Parse Number Values") fr fr Test parsing numbers
    sus int_result tea = json.parse_value("42")
    assert_eq_string(int_result, "42")
    
    sus float_result tea = json.parse_value("3.14")
    assert_eq_string(float_result, "3.14")
    
    sus zero_result tea = json.parse_value("0")
    assert_eq_string(zero_result, "0")
}

slay test_json_parse_literals() {
    test_start("JSON Parse Literal Values") fr fr Test parsing booleans and null
    sus true_result tea = json.parse_value("true")
    assert_eq_string(true_result, "true")
    
    sus false_result tea = json.parse_value("false")
    assert_eq_string(false_result, "false")
    
    sus null_result tea = json.parse_value("null")
    assert_eq_string(null_result, "null")
}

slay test_json_validation() {
    test_start("JSON Validation") fr fr Test valid JSON
    assert_true(json.validate("\"hello\""))
    assert_true(json.validate("42"))
    assert_true(json.validate("true"))
    assert_true(json.validate("false"))
    assert_true(json.validate("null")) fr fr Test invalid JSON
    assert_false(json.validate("invalid"))
    assert_false(json.validate(""))
}

slay test_json_stringify() {
    test_start("JSON Stringify") fr fr Test stringifying values
    sus str_json tea = json.stringify("hello")
    assert_eq_string(str_json, "\"hello\"")
    
    sus num_json tea = json.stringify("42")
    assert_eq_string(num_json, "42")
    
    sus bool_json tea = json.stringify("true")
    assert_eq_string(bool_json, "true")
}

slay test_json_utilities() {
    test_start("JSON String Utilities") fr fr Test is_numeric
    assert_true(json.is_numeric("42"))
    assert_true(json.is_numeric("3.14"))
    assert_true(json.is_numeric("-42"))
    assert_false(json.is_numeric("abc"))
    assert_false(json.is_numeric("")) fr fr Test string utilities
    sus trimmed tea = json.string_trim("  hello  ")
    assert_eq_string(trimmed, "hello")
    
    assert_true(json.string_starts_with("hello world", "hello"))
    assert_false(json.string_starts_with("hello world", "world"))
    
    assert_true(json.string_ends_with("hello world", "world"))
    assert_false(json.string_ends_with("hello world", "hello"))
}

slay test_json_round_trip() {
    test_start("JSON Round-Trip Processing") fr fr Test string round-trip
    sus original tea = "hello"
    sus stringified tea = json.stringify(original)
    sus parsed tea = json.parse_value(stringified)
    assert_eq_string(parsed, original) fr fr Test number round-trip
    sus num_original tea = "42"
    sus num_stringified tea = json.stringify(num_original)
    sus num_parsed tea = json.parse_value(num_stringified)
    assert_eq_string(num_parsed, num_original)
}

slay run_all_json_tests() {
    vibez.spill("🔧 Running Simple CURSED JSON Tests")
    vibez.spill("===================================")
    
    test_json_parse_string()
    test_json_parse_numbers()
    test_json_parse_literals()
    test_json_validation()
    test_json_stringify()
    test_json_utilities()
    test_json_round_trip()
    
    print_test_summary()
}

fr fr Auto-run tests
run_all_json_tests()
