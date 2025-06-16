/// Comprehensive integration tests for the no_cap module
#[cfg(test)]
mod integration_tests {
    use super::super::*;

    #[test]
    fn test_module_initialization() {
        let result = init_no_cap();
        assert!(result.is_ok());

        let stats = get_no_cap_stats();
        assert_eq!(stats.functions_available, 10);
        assert!(stats.conversions_supported.contains(&"bool".to_string()));
        assert!(stats.slang_terms.contains(&"facts".to_string()));
    }

    #[test]
    fn test_facts_check_comprehensive() {
        // Test all valid true values
        let true_values = vec![
            "1", "t", "T", "based", "TRUE", "True", "facts", "FACTS", "Facts", "no cap", "fr fr"
        ];
        
        for value in true_values {
            let result = FactsCheck(value.to_string());
            assert!(result.is_ok(), "Failed to parse '{}' as true", value);
            let (parsed, _) = result.unwrap();
            assert_eq!(parsed, true, "Expected true for '{}'", value);
        }

        // Test all valid false values
        let false_values = vec![
            "0", "f", "F", "false", "FALSE", "False", "cap", "CAP", "idk"
        ];
        
        for value in false_values {
            let result = FactsCheck(value.to_string());
            assert!(result.is_ok(), "Failed to parse '{}' as false", value);
            let (parsed, _) = result.unwrap();
            assert_eq!(parsed, false, "Expected false for '{}'", value);
        }

        // Test invalid values
        let invalid_values = vec!["maybe", "yes", "no", "2", "", "true1", "FALSE "];
        
        for value in invalid_values {
            let result = FactsCheck(value.to_string());
            assert!(result.is_err(), "Should fail for invalid value '{}'", value);
        }
    }

    #[test]
    fn test_yoink_int_comprehensive() {
        // Test basic decimal parsing
        let test_cases = vec![
            ("123", 10, 32, Ok(123i64)),
            ("-456", 10, 32, Ok(-456i64)),
            ("0", 10, 32, Ok(0i64)),
            ("+789", 10, 32, Ok(789i64)),
        ];

        for (input, base, bit_size, expected) in test_cases {
            let result = YoinkInt(input.to_string(), base, bit_size);
            match expected {
                Ok(exp_value) => {
                    assert!(result.is_ok(), "Failed to parse '{}' with base {}", input, base);
                    let (value, _) = result.unwrap();
                    assert_eq!(value, exp_value);
                }
                Err(_) => {
                    assert!(result.is_err(), "Should fail for input '{}'", input);
                }
            }
        }

        // Test different bases with auto-detection (base 0)
        let base_test_cases = vec![
            ("0xFF", 0, 32, 255i64),
            ("0x10", 0, 32, 16i64),
            ("0b1010", 0, 32, 10i64),
            ("0b0001", 0, 32, 1i64),
            ("010", 0, 32, 8i64), // Octal
            ("123", 0, 32, 123i64), // Decimal
        ];

        for (input, base, bit_size, expected) in base_test_cases {
            let result = YoinkInt(input.to_string(), base, bit_size);
            assert!(result.is_ok(), "Failed to parse '{}' with auto-detection", input);
            let (value, _) = result.unwrap();
            assert_eq!(value, expected);
        }

        // Test range checking for different bit sizes
        let range_test_cases = vec![
            ("127", 10, 8, true),    // Valid for 8-bit
            ("128", 10, 8, false),   // Invalid for 8-bit signed
            ("-128", 10, 8, true),   // Valid for 8-bit
            ("-129", 10, 8, false),  // Invalid for 8-bit signed
            ("32767", 10, 16, true), // Valid for 16-bit
            ("32768", 10, 16, false), // Invalid for 16-bit signed
        ];

        for (input, base, bit_size, should_succeed) in range_test_cases {
            let result = YoinkInt(input.to_string(), base, bit_size);
            if should_succeed {
                assert!(result.is_ok(), "Should succeed for '{}' with {}-bit", input, bit_size);
            } else {
                assert!(result.is_err(), "Should fail for '{}' with {}-bit", input, bit_size);
            }
        }
    }

    #[test]
    fn test_yoink_uint_comprehensive() {
        // Test basic unsigned parsing
        let test_cases = vec![
            ("123", 10, 32, Ok(123u64)),
            ("0", 10, 32, Ok(0u64)),
            ("+789", 10, 32, Ok(789u64)),
        ];

        for (input, base, bit_size, expected) in test_cases {
            let result = YoinkUint(input.to_string(), base, bit_size);
            match expected {
                Ok(exp_value) => {
                    assert!(result.is_ok(), "Failed to parse '{}' with base {}", input, base);
                    let (value, _) = result.unwrap();
                    assert_eq!(value, exp_value);
                }
                Err(_) => {
                    assert!(result.is_err(), "Should fail for input '{}'", input);
                }
            }
        }

        // Test that negative values are rejected
        let negative_cases = vec!["-123", "-1", "-0"];
        for input in negative_cases {
            let result = YoinkUint(input.to_string(), 10, 32);
            assert!(result.is_err(), "Should reject negative value '{}'", input);
        }

        // Test range checking
        let range_cases = vec![
            ("255", 10, 8, true),   // Valid for 8-bit unsigned
            ("256", 10, 8, false),  // Invalid for 8-bit unsigned
            ("65535", 10, 16, true), // Valid for 16-bit unsigned
            ("65536", 10, 16, false), // Invalid for 16-bit unsigned
        ];

        for (input, base, bit_size, should_succeed) in range_cases {
            let result = YoinkUint(input.to_string(), base, bit_size);
            if should_succeed {
                assert!(result.is_ok(), "Should succeed for '{}' with {}-bit unsigned", input, bit_size);
            } else {
                assert!(result.is_err(), "Should fail for '{}' with {}-bit unsigned", input, bit_size);
            }
        }
    }

    #[test]
    fn test_yoink_float_comprehensive() {
        // Test basic float parsing
        let basic_cases = vec![
            ("123.45", 64, 123.45),
            ("-67.89", 64, -67.89),
            ("0.0", 64, 0.0),
            ("1.23e2", 64, 123.0),
            ("1.23E-2", 64, 0.0123),
            (".5", 64, 0.5),
            ("5.", 64, 5.0),
        ];

        for (input, bit_size, expected) in basic_cases {
            let result = YoinkFloat(input.to_string(), bit_size);
            assert!(result.is_ok(), "Failed to parse float '{}'", input);
            let (value, _) = result.unwrap();
            assert!((value - expected).abs() < 1e-10, "Expected {} but got {} for input '{}'", expected, value, input);
        }

        // Test special values including Gen Z slang
        let special_cases = vec![
            ("NaN", true, false, false),
            ("nan", true, false, false),
            ("sus", true, false, false),  // Gen Z slang for NaN
            ("inf", false, true, true),
            ("Inf", false, true, true),
            ("+inf", false, true, true),
            ("infinity", false, true, true),
            ("bussin", false, true, true), // Gen Z slang for +inf
            ("-inf", false, true, false),
            ("-infinity", false, true, false),
            ("busted", false, true, false), // Gen Z slang for -inf
        ];

        for (input, should_be_nan, should_be_infinite, should_be_positive) in special_cases {
            let result = YoinkFloat(input.to_string(), 64);
            assert!(result.is_ok(), "Failed to parse special float '{}'", input);
            let (value, _) = result.unwrap();
            
            if should_be_nan {
                assert!(value.is_nan(), "Expected NaN for '{}'", input);
            } else if should_be_infinite {
                assert!(value.is_infinite(), "Expected infinite for '{}'", input);
                if should_be_positive {
                    assert!(value.is_sign_positive(), "Expected positive infinity for '{}'", input);
                } else {
                    assert!(value.is_sign_negative(), "Expected negative infinity for '{}'", input);
                }
            }
        }

        // Test 32-bit vs 64-bit precision
        let precision_test = "1.2345678901234567890";
        let result32 = YoinkFloat(precision_test.to_string(), 32);
        let result64 = YoinkFloat(precision_test.to_string(), 64);
        
        assert!(result32.is_ok());
        assert!(result64.is_ok());
        
        let (val32, _) = result32.unwrap();
        let (val64, _) = result64.unwrap();
        
        // 32-bit should have less precision
        assert_ne!(val32, val64);
    }

    #[test]
    fn test_formatting_functions_comprehensive() {
        // Test YeetBool
        assert_eq!(YeetBool(true), "facts");
        assert_eq!(YeetBool(false), "cap");

        // Test YeetInt with different bases
        let int_test_cases = vec![
            (255, 10, "255"),
            (255, 16, "ff"),
            (10, 2, "1010"),
            (-255, 10, "-255"),
            (-255, 16, "-ff"),
            (0, 10, "0"),
            (35, 36, "z"),
        ];

        for (value, base, expected) in int_test_cases {
            let result = YeetInt(value, base);
            assert_eq!(result, expected, "YeetInt({}, {}) should return '{}'", value, base, expected);
        }

        // Test YeetUint
        let uint_test_cases = vec![
            (255, 10, "255"),
            (255, 16, "ff"),
            (10, 2, "1010"),
            (0, 10, "0"),
        ];

        for (value, base, expected) in uint_test_cases {
            let result = YeetUint(value, base);
            assert_eq!(result, expected, "YeetUint({}, {}) should return '{}'", value, base, expected);
        }

        // Test YeetFloat
        let float_test_cases = vec![
            (123.45, b'f', 2, 64, "123.45"),
            (123.456, b'f', 2, 64, "123.46"), // Rounding
            (0.0, b'f', 2, 64, "0.00"),
        ];

        for (value, fmt, prec, bit_size, expected) in float_test_cases {
            let result = YeetFloat(value, fmt, prec, bit_size);
            assert_eq!(result, expected, "YeetFloat({}, {}, {}, {}) should return '{}'", value, fmt as char, prec, bit_size, expected);
        }

        // Test SussyFloat with special values
        assert_eq!(SussyFloat(f64::NAN), "sus");
        assert_eq!(SussyFloat(f64::INFINITY), "bussin");
        assert_eq!(SussyFloat(f64::NEG_INFINITY), "busted");
        assert_eq!(SussyFloat(123.45), "123.45");
    }

    #[test]
    fn test_convenience_functions() {
        // Test Atoi and Itoa roundtrip
        let test_values = vec![0, 123, -456, 2147483647, -2147483648];
        
        for original in test_values {
            let string_form = Itoa(original);
            let result = Atoi(string_form);
            assert!(result.is_ok(), "Roundtrip failed for {}", original);
            let (parsed, _) = result.unwrap();
            assert_eq!(parsed, original, "Roundtrip mismatch for {}", original);
        }

        // Test Atoi error cases
        let error_cases = vec!["", "abc", "123.45", "99999999999999999999"];
        for case in error_cases {
            let result = Atoi(case.to_string());
            assert!(result.is_err(), "Should fail for '{}'", case);
        }
    }

    #[test]
    fn test_error_handling() {
        // Test syntax errors
        let result = FactsCheck("invalid".to_string());
        assert!(result.is_err());
        match result.unwrap_err() {
            NoCapError::Syntax(_) => {},
            _ => panic!("Expected syntax error"),
        }

        // Test range errors
        let result = YoinkInt("99999999999999999999".to_string(), 10, 8);
        assert!(result.is_err());

        // Test invalid input errors
        let result = YoinkFloat("".to_string(), 64);
        assert!(result.is_err());
    }

    #[test]
    fn test_edge_cases() {
        // Test empty strings
        assert!(FactsCheck("".to_string()).is_err());
        assert!(YoinkInt("".to_string(), 10, 32).is_err());
        assert!(YoinkFloat("".to_string(), 64).is_err());
        assert!(Atoi("".to_string()).is_err());

        // Test whitespace handling
        assert!(FactsCheck("  facts  ".to_string()).is_ok());
        assert!(YoinkInt("  123  ".to_string(), 10, 32).is_ok());
        assert!(YoinkFloat("  123.45  ".to_string(), 64).is_ok());

        // Test zero values
        let (zero_int, _) = YoinkInt("0".to_string(), 10, 32).unwrap();
        assert_eq!(zero_int, 0);
        
        let (zero_float, _) = YoinkFloat("0.0".to_string(), 64).unwrap();
        assert_eq!(zero_float, 0.0);

        // Test plus signs
        let (pos_int, _) = YoinkInt("+123".to_string(), 10, 32).unwrap();
        assert_eq!(pos_int, 123);
        
        let (pos_float, _) = YoinkFloat("+123.45".to_string(), 64).unwrap();
        assert!((pos_float - 123.45).abs() < f64::EPSILON);
    }

    #[test]
    fn test_gen_z_slang_consistency() {
        // Test boolean Gen Z slang
        let (facts_val, _) = FactsCheck("facts".to_string()).unwrap();
        assert_eq!(facts_val, true);
        assert_eq!(YeetBool(facts_val), "facts");

        let (cap_val, _) = FactsCheck("cap".to_string()).unwrap();
        assert_eq!(cap_val, false);
        assert_eq!(YeetBool(cap_val), "cap");

        // Test float Gen Z slang
        let (sus_val, _) = YoinkFloat("sus".to_string(), 64).unwrap();
        assert!(sus_val.is_nan());
        assert_eq!(SussyFloat(sus_val), "sus");

        let (bussin_val, _) = YoinkFloat("bussin".to_string(), 64).unwrap();
        assert!(bussin_val.is_infinite() && bussin_val.is_sign_positive());
        assert_eq!(SussyFloat(bussin_val), "bussin");

        let (busted_val, _) = YoinkFloat("busted".to_string(), 64).unwrap();
        assert!(busted_val.is_infinite() && busted_val.is_sign_negative());
        assert_eq!(SussyFloat(busted_val), "busted");
    }

    #[test]
    fn test_performance_edge_cases() {
        // Test very large numbers
        let large_int = i64::MAX.to_string();
        let result = YoinkInt(large_int, 10, 64);
        assert!(result.is_ok());

        let small_int = i64::MIN.to_string();
        let result = YoinkInt(small_int, 10, 64);
        assert!(result.is_ok());

        // Test very small floats
        let result = YoinkFloat("1e-100".to_string(), 64);
        assert!(result.is_ok());

        // Test very large floats
        let result = YoinkFloat("1e100".to_string(), 64);
        assert!(result.is_ok());
    }
}

#[cfg(test)]
mod benchmark_tests {
    use super::super::*;
    use std::time::Instant;

    #[test]
    fn benchmark_basic_operations() {
        let iterations = 10000;
        
        // Benchmark FactsCheck
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = FactsCheck("facts".to_string());
        }
        let facts_check_time = start.elapsed();
        println!("FactsCheck {} iterations: {:?}", iterations, facts_check_time);

        // Benchmark YoinkInt
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = YoinkInt("12345".to_string(), 10, 32);
        }
        let yoink_int_time = start.elapsed();
        println!("YoinkInt {} iterations: {:?}", iterations, yoink_int_time);

        // Benchmark YoinkFloat
        let start = Instant::now();
        for _ in 0..iterations {
            let _ = YoinkFloat("123.45".to_string(), 64);
        }
        let yoink_float_time = start.elapsed();
        println!("YoinkFloat {} iterations: {:?}", iterations, yoink_float_time);

        // Benchmark Atoi/Itoa roundtrip
        let start = Instant::now();
        for i in 0..iterations {
            let s = Itoa(i as i32);
            let _ = Atoi(s);
        }
        let roundtrip_time = start.elapsed();
        println!("Atoi/Itoa roundtrip {} iterations: {:?}", iterations, roundtrip_time);
    }
}
