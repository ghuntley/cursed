/// Comprehensive integration tests for the no_cap string conversion module
use cursed::stdlib::no_cap::*;

#[test]
fn test_no_cap_module_initialization() {
    let result = init_no_cap();
    assert!(result.is_ok(), "Module initialization should succeed");

    let stats = get_no_cap_stats();
    assert_eq!(stats.functions_available, 10, "Should have 10 available functions");
    assert!(stats.conversions_supported.len() >= 4, "Should support at least 4 conversion types");
    assert!(stats.slang_terms.len() >= 7, "Should have at least 7 Gen Z slang terms");
}

#[test]
fn test_facts_check_comprehensive_integration() {
    // Test all true values from the specification
    let true_values = vec![
        "1", "t", "T", "based", "TRUE", "True", "facts", "FACTS", "Facts", "no cap", "fr fr"
    ];
    
    for value in &true_values {
        let result = FactsCheck(value.to_string());
        assert!(result.is_ok(), "Should parse '{}' as boolean", value);
        let (parsed, _) = result.unwrap();
        assert_eq!(parsed, true, "Should parse '{}' as true", value);
        
        // Test that YeetBool produces consistent output
        assert_eq!(YeetBool(parsed), "facts", "YeetBool should always return 'facts' for true");
    }
    
    // Test all false values from the specification
    let false_values = vec![
        "0", "f", "F", "false", "FALSE", "False", "cap", "CAP", "idk"
    ];
    
    for value in &false_values {
        let result = FactsCheck(value.to_string());
        assert!(result.is_ok(), "Should parse '{}' as boolean", value);
        let (parsed, _) = result.unwrap();
        assert_eq!(parsed, false, "Should parse '{}' as false", value);
        
        // Test that YeetBool produces consistent output
        assert_eq!(YeetBool(parsed), "cap", "YeetBool should always return 'cap' for false");
    }
    
    // Test invalid values
    let invalid_values = vec!["maybe", "yes", "no", "2", "", "true1", "FALSE ", "   "];
    for value in &invalid_values {
        let result = FactsCheck(value.to_string());
        assert!(result.is_err(), "Should fail to parse '{}' as boolean", value);
        
        // Verify error type is syntax error
        match result.unwrap_err() {
            NoCapError::Syntax(_) => {},
            other => panic!("Expected Syntax error for '{}', got {:?}", value, other),
        }
    }
}

#[test]
fn test_integer_parsing_integration() {
    // Test YoinkInt with various bases and bit sizes
    let test_cases = vec![
        // (input, base, bit_size, expected_result)
        ("123", 10, 32, Ok(123i64)),
        ("-456", 10, 32, Ok(-456i64)),
        ("0", 10, 32, Ok(0i64)),
        ("+789", 10, 32, Ok(789i64)),
        
        // Hex values
        ("0xFF", 0, 32, Ok(255i64)),   // Auto-detect hex
        ("ff", 16, 32, Ok(255i64)),    // Explicit hex
        ("0x10", 0, 32, Ok(16i64)),
        
        // Binary values
        ("0b1010", 0, 32, Ok(10i64)),  // Auto-detect binary
        ("1010", 2, 32, Ok(10i64)),    // Explicit binary
        
        // Octal values
        ("010", 0, 32, Ok(8i64)),      // Auto-detect octal
        ("10", 8, 32, Ok(8i64)),       // Explicit octal
        
        // Different bit sizes - valid ranges
        ("127", 10, 8, Ok(127i64)),    // Max for 8-bit signed
        ("-128", 10, 8, Ok(-128i64)),  // Min for 8-bit signed
        ("32767", 10, 16, Ok(32767i64)), // Max for 16-bit signed
        
        // Error cases
        ("128", 10, 8, Err(())),       // Out of range for 8-bit signed
        ("-129", 10, 8, Err(())),      // Out of range for 8-bit signed
        ("abc", 10, 32, Err(())),      // Invalid number
        ("", 10, 32, Err(())),         // Empty string
        ("123.45", 10, 32, Err(())),   // Floating point
    ];
    
    for (input, base, bit_size, expected) in test_cases {
        let result = YoinkInt(input.to_string(), base, bit_size);
        
        match expected {
            Ok(expected_value) => {
                assert!(result.is_ok(), "Should parse '{}' with base {} and bit_size {}", input, base, bit_size);
                let (value, _) = result.unwrap();
                assert_eq!(value, expected_value, "Parsing '{}' should yield {}", input, expected_value);
                
                // Test roundtrip with YeetInt
                let formatted = YeetInt(value, if base == 0 { 10 } else { base });
                if base == 10 || base == 0 {
                    // For decimal, we can do exact roundtrip
                    let roundtrip = YoinkInt(formatted, 10, bit_size);
                    assert!(roundtrip.is_ok(), "Roundtrip should work for {}", value);
                    let (roundtrip_value, _) = roundtrip.unwrap();
                    assert_eq!(roundtrip_value, value, "Roundtrip mismatch for {}", value);
                }
            }
            Err(_) => {
                assert!(result.is_err(), "Should fail to parse '{}' with base {} and bit_size {}", input, base, bit_size);
            }
        }
    }
}

#[test]
fn test_unsigned_integer_parsing_integration() {
    let test_cases = vec![
        // (input, base, bit_size, expected_result)
        ("123", 10, 32, Ok(123u64)),
        ("0", 10, 32, Ok(0u64)),
        ("+789", 10, 32, Ok(789u64)),
        
        // Hex values
        ("0xFF", 0, 32, Ok(255u64)),
        ("ff", 16, 32, Ok(255u64)),
        
        // Binary values
        ("0b1010", 0, 32, Ok(10u64)),
        ("1010", 2, 32, Ok(10u64)),
        
        // Range tests
        ("255", 10, 8, Ok(255u64)),    // Max for 8-bit unsigned
        ("65535", 10, 16, Ok(65535u64)), // Max for 16-bit unsigned
        
        // Error cases
        ("-123", 10, 32, Err(())),     // Negative not allowed
        ("256", 10, 8, Err(())),       // Out of range for 8-bit unsigned
        ("abc", 10, 32, Err(())),      // Invalid number
    ];
    
    for (input, base, bit_size, expected) in test_cases {
        let result = YoinkUint(input.to_string(), base, bit_size);
        
        match expected {
            Ok(expected_value) => {
                assert!(result.is_ok(), "Should parse '{}' as unsigned with base {} and bit_size {}", input, base, bit_size);
                let (value, _) = result.unwrap();
                assert_eq!(value, expected_value, "Parsing '{}' should yield {}", input, expected_value);
                
                // Test roundtrip with YeetUint
                let formatted = YeetUint(value, if base == 0 { 10 } else { base });
                if base == 10 || base == 0 {
                    let roundtrip = YoinkUint(formatted, 10, bit_size);
                    assert!(roundtrip.is_ok(), "Roundtrip should work for {}", value);
                    let (roundtrip_value, _) = roundtrip.unwrap();
                    assert_eq!(roundtrip_value, value, "Roundtrip mismatch for {}", value);
                }
            }
            Err(_) => {
                assert!(result.is_err(), "Should fail to parse '{}' as unsigned with base {} and bit_size {}", input, base, bit_size);
            }
        }
    }
}

#[test]
fn test_float_parsing_integration() {
    // Test regular float values
    let regular_cases = vec![
        ("123.45", 64, 123.45),
        ("-67.89", 64, -67.89),
        ("0.0", 64, 0.0),
        ("1.23e2", 64, 123.0),
        ("1.23E-2", 64, 0.0123),
        (".5", 64, 0.5),
        ("5.", 64, 5.0),
        ("123", 64, 123.0),  // Integers should parse as floats
    ];
    
    for (input, bit_size, expected) in regular_cases {
        let result = YoinkFloat(input.to_string(), bit_size);
        assert!(result.is_ok(), "Should parse '{}' as float", input);
        let (value, _) = result.unwrap();
        assert!((value - expected).abs() < 1e-10, "Expected {} but got {} for input '{}'", expected, value, input);
    }
    
    // Test special values including Gen Z slang
    let special_cases = vec![
        // (input, is_nan, is_infinite, is_positive)
        ("NaN", true, false, false),
        ("nan", true, false, false),
        ("sus", true, false, false),        // Gen Z slang for NaN
        
        ("inf", false, true, true),
        ("Inf", false, true, true),
        ("+inf", false, true, true),
        ("infinity", false, true, true),
        ("bussin", false, true, true),      // Gen Z slang for +inf
        
        ("-inf", false, true, false),
        ("-infinity", false, true, false),
        ("busted", false, true, false),     // Gen Z slang for -inf
    ];
    
    for (input, should_be_nan, should_be_infinite, should_be_positive) in special_cases {
        let result = YoinkFloat(input.to_string(), 64);
        assert!(result.is_ok(), "Should parse special float '{}'", input);
        let (value, _) = result.unwrap();
        
        if should_be_nan {
            assert!(value.is_nan(), "Expected NaN for '{}'", input);
            assert_eq!(SussyFloat(value), "sus", "SussyFloat should return 'sus' for NaN");
        } else if should_be_infinite {
            assert!(value.is_infinite(), "Expected infinite for '{}'", input);
            if should_be_positive {
                assert!(value.is_sign_positive(), "Expected positive infinity for '{}'", input);
                assert_eq!(SussyFloat(value), "bussin", "SussyFloat should return 'bussin' for +inf");
            } else {
                assert!(value.is_sign_negative(), "Expected negative infinity for '{}'", input);
                assert_eq!(SussyFloat(value), "busted", "SussyFloat should return 'busted' for -inf");
            }
        }
    }
    
    // Test 32-bit vs 64-bit precision
    let high_precision = "1.23456789012345678901234567890";
    let result32 = YoinkFloat(high_precision.to_string(), 32);
    let result64 = YoinkFloat(high_precision.to_string(), 64);
    
    assert!(result32.is_ok() && result64.is_ok(), "Should parse high precision float in both formats");
    let (val32, _) = result32.unwrap();
    let (val64, _) = result64.unwrap();
    
    // 32-bit should have less precision
    assert_ne!(val32, val64, "32-bit and 64-bit should have different precision");
}

#[test]
fn test_formatting_functions_integration() {
    // Test YeetInt with various bases
    let int_cases = vec![
        (0, 10, "0"),
        (123, 10, "123"),
        (-456, 10, "-456"),
        (255, 16, "ff"),
        (-255, 16, "-ff"),
        (10, 2, "1010"),
        (-10, 2, "-1010"),
        (35, 36, "z"),
        (8, 8, "10"),
    ];
    
    for (value, base, expected) in int_cases {
        let result = YeetInt(value, base);
        assert_eq!(result, expected, "YeetInt({}, {}) should return '{}'", value, base, expected);
    }
    
    // Test YeetUint
    let uint_cases = vec![
        (0, 10, "0"),
        (123, 10, "123"),
        (255, 16, "ff"),
        (10, 2, "1010"),
    ];
    
    for (value, base, expected) in uint_cases {
        let result = YeetUint(value, base);
        assert_eq!(result, expected, "YeetUint({}, {}) should return '{}'", value, base, expected);
    }
    
    // Test YeetFloat with different formats
    let float_cases = vec![
        (123.45, b'f', 2, 64, "123.45"),
        (123.456, b'f', 2, 64, "123.46"), // Should round
        (1234.5, b'e', 2, 64, "1.23e"),   // Should start with this
        (0.0, b'f', 2, 64, "0.00"),
    ];
    
    for (value, fmt, prec, bit_size, expected_start) in float_cases {
        let result = YeetFloat(value, fmt, prec, bit_size);
        if expected_start.ends_with('e') {
            assert!(result.to_lowercase().starts_with(expected_start), 
                   "YeetFloat({}, {}, {}, {}) should start with '{}', got '{}'", 
                   value, fmt as char, prec, bit_size, expected_start, result);
        } else {
            assert_eq!(result, expected_start, 
                      "YeetFloat({}, {}, {}, {}) should return '{}', got '{}'", 
                      value, fmt as char, prec, bit_size, expected_start, result);
        }
    }
}

#[test]
fn test_convenience_functions_integration() {
    // Test Atoi/Itoa roundtrip for various values
    let test_values = vec![
        0, 1, -1, 42, -42, 123456, -123456, 
        i32::MAX, i32::MIN, 1337, -2024
    ];
    
    for original in test_values {
        // Forward conversion
        let string_form = Itoa(original);
        assert!(!string_form.is_empty(), "Itoa should not return empty string for {}", original);
        
        // Backward conversion
        let result = Atoi(string_form.clone());
        assert!(result.is_ok(), "Atoi should parse string '{}' generated by Itoa({})", string_form, original);
        
        let (parsed, _) = result.unwrap();
        assert_eq!(parsed, original, "Roundtrip failed: {} -> '{}' -> {}", original, string_form, parsed);
    }
    
    // Test Atoi error cases
    let error_cases = vec!["", "abc", "123.45", "999999999999999999999", " ", "12 34"];
    for case in error_cases {
        let result = Atoi(case.to_string());
        assert!(result.is_err(), "Atoi should fail for invalid input '{}'", case);
    }
}

#[test]
fn test_gen_z_slang_consistency() {
    // Test boolean Gen Z slang roundtrip
    let slang_bool_cases = vec![
        ("facts", true),
        ("cap", false),
        ("based", true),
        ("idk", false),
        ("no cap", true),
        ("fr fr", true),
    ];
    
    for (slang, expected_bool) in slang_bool_cases {
        let result = FactsCheck(slang.to_string());
        assert!(result.is_ok(), "Should parse Gen Z slang '{}'", slang);
        let (parsed, _) = result.unwrap();
        assert_eq!(parsed, expected_bool, "Gen Z slang '{}' should parse to {}", slang, expected_bool);
    }
    
    // Test float Gen Z slang
    let slang_float_cases = vec![
        ("sus", true, false, false),      // NaN
        ("bussin", false, true, true),    // +Inf
        ("busted", false, true, false),   // -Inf
    ];
    
    for (slang, should_be_nan, should_be_inf, should_be_pos) in slang_float_cases {
        let result = YoinkFloat(slang.to_string(), 64);
        assert!(result.is_ok(), "Should parse Gen Z float slang '{}'", slang);
        let (value, _) = result.unwrap();
        
        if should_be_nan {
            assert!(value.is_nan(), "Gen Z slang '{}' should be NaN", slang);
            assert_eq!(SussyFloat(value), "sus", "SussyFloat should return 'sus' for NaN from '{}'", slang);
        } else if should_be_inf {
            assert!(value.is_infinite(), "Gen Z slang '{}' should be infinite", slang);
            if should_be_pos {
                assert!(value.is_sign_positive(), "Gen Z slang '{}' should be positive", slang);
                assert_eq!(SussyFloat(value), "bussin", "SussyFloat should return 'bussin' for +Inf from '{}'", slang);
            } else {
                assert!(value.is_sign_negative(), "Gen Z slang '{}' should be negative", slang);
                assert_eq!(SussyFloat(value), "busted", "SussyFloat should return 'busted' for -Inf from '{}'", slang);
            }
        }
    }
}

#[test]
fn test_utility_functions_integration() {
    // Test validation functions
    assert!(IsValidInt(&"123".to_string()), "Should validate '123' as int");
    assert!(IsValidInt(&"-456".to_string()), "Should validate '-456' as int");
    assert!(!IsValidInt(&"123.45".to_string()), "Should not validate '123.45' as int");
    assert!(!IsValidInt(&"abc".to_string()), "Should not validate 'abc' as int");
    
    assert!(IsValidFloat(&"123.45".to_string()), "Should validate '123.45' as float");
    assert!(IsValidFloat(&"123".to_string()), "Should validate '123' as float");
    assert!(!IsValidFloat(&"abc".to_string()), "Should not validate 'abc' as float");
    
    assert!(IsValidBool(&"facts".to_string()), "Should validate 'facts' as bool");
    assert!(IsValidBool(&"cap".to_string()), "Should validate 'cap' as bool");
    assert!(!IsValidBool(&"maybe".to_string()), "Should not validate 'maybe' as bool");
    
    // Test base conversion
    let conversion_cases = vec![
        ("255", 10, 16, "ff"),
        ("ff", 16, 10, "255"),
        ("1010", 2, 10, "10"),
        ("10", 10, 2, "1010"),
    ];
    
    for (input, from_base, to_base, expected) in conversion_cases {
        let result = ConvertBase(input.to_string(), from_base, to_base);
        assert!(result.is_ok(), "Base conversion should work for '{}' from base {} to base {}", input, from_base, to_base);
        assert_eq!(result.unwrap(), expected, "Converting '{}' from base {} to base {} should yield '{}'", input, from_base, to_base, expected);
    }
    
    // Test number formatting with separators
    let format_cases = vec![
        (1234567, ',', "1,234,567"),
        (-1234567, ',', "-1,234,567"),
        (123, ',', "123"),
        (0, ',', "0"),
    ];
    
    for (number, separator, expected) in format_cases {
        let result = FormatWithSeparators(number, separator);
        assert_eq!(result, expected, "Formatting {} with separator '{}' should yield '{}'", number, separator, expected);
        
        // Test parsing back
        let parse_result = ParseWithSeparators(result.clone(), separator);
        assert!(parse_result.is_ok(), "Should parse formatted number '{}' back", result);
        let (parsed, _) = parse_result.unwrap();
        assert_eq!(parsed, number, "Parsing '{}' should yield original number {}", result, number);
    }
    
    // Test number type detection
    let type_cases = vec![
        ("123", NumberType::UnsignedInteger),
        ("-123", NumberType::Integer),
        ("123.45", NumberType::Float),
        ("facts", NumberType::Boolean),
        ("abc", NumberType::NotANumber),
    ];
    
    for (input, expected_type) in type_cases {
        let detected_type = GetNumberType(&input.to_string());
        assert_eq!(detected_type, expected_type, "Type detection for '{}' should yield {:?}", input, expected_type);
    }
}

#[test]
fn test_error_handling_integration() {
    // Test that error types are consistent
    let syntax_error = FactsCheck("invalid".to_string()).unwrap_err();
    match syntax_error {
        NoCapError::Syntax(_) => {},
        other => panic!("Expected Syntax error, got {:?}", other),
    }
    
    let range_error = YoinkInt("999999999999999999999".to_string(), 10, 8).unwrap_err();
    match range_error {
        NoCapError::Range(_) | NoCapError::Syntax(_) => {}, // Could be either depending on implementation
        other => panic!("Expected Range or Syntax error, got {:?}", other),
    }
    
    // Test error message format
    let error_msg = syntax_error.to_string();
    assert!(error_msg.contains("sus conversion") || error_msg.contains("invalid"), 
           "Error message should contain expected text, got: {}", error_msg);
}

#[test]
fn test_performance_characteristics() {
    use std::time::Instant;
    
    let iterations = 1000;
    
    // Test parsing performance
    let start = Instant::now();
    for i in 0..iterations {
        let _ = Atoi(i.to_string());
    }
    let atoi_time = start.elapsed();
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = FactsCheck("facts".to_string());
    }
    let facts_check_time = start.elapsed();
    
    let start = Instant::now();
    for _ in 0..iterations {
        let _ = YoinkFloat("123.45".to_string(), 64);
    }
    let float_parse_time = start.elapsed();
    
    // Performance should be reasonable (these are loose bounds)
    assert!(atoi_time.as_millis() < 100, "Atoi performance should be reasonable");
    assert!(facts_check_time.as_millis() < 100, "FactsCheck performance should be reasonable");
    assert!(float_parse_time.as_millis() < 100, "Float parsing performance should be reasonable");
    
    println!("Performance results for {} iterations:", iterations);
    println!("  Atoi: {:?}", atoi_time);
    println!("  FactsCheck: {:?}", facts_check_time);
    println!("  YoinkFloat: {:?}", float_parse_time);
}

#[test]
fn test_edge_cases_and_whitespace() {
    // Test whitespace handling
    let whitespace_cases = vec![
        "  123  ",
        "\t456\t",
        "\n789\n",
        " facts ",
        " 123.45 ",
    ];
    
    for case in whitespace_cases {
        let trimmed = case.trim();
        
        if IsValidInt(&trimmed.to_string()) {
            let result = Atoi(case.to_string());
            assert!(result.is_ok(), "Should handle whitespace in integer '{}'", case);
        }
        
        if IsValidBool(&trimmed.to_string()) {
            let result = FactsCheck(case.to_string());
            assert!(result.is_ok(), "Should handle whitespace in boolean '{}'", case);
        }
        
        if IsValidFloat(&trimmed.to_string()) {
            let result = YoinkFloat(case.to_string(), 64);
            assert!(result.is_ok(), "Should handle whitespace in float '{}'", case);
        }
    }
    
    // Test empty and whitespace-only strings
    let empty_cases = vec!["", "   ", "\t", "\n", " \t\n "];
    for case in empty_cases {
        assert!(Atoi(case.to_string()).is_err(), "Should fail for empty/whitespace string '{}'", case);
        assert!(FactsCheck(case.to_string()).is_err(), "Should fail for empty/whitespace string '{}'", case);
        assert!(YoinkFloat(case.to_string(), 64).is_err(), "Should fail for empty/whitespace string '{}'", case);
    }
}
