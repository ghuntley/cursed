/// Test implementation verification for TestVibes framework
/// 
/// This test verifies that the TestVibes framework is correctly implemented
/// and provides all the functionality specified in the spec.

use cursed::stdlib::test_vibes::*;
use cursed::stdlib::value::Value;
use std::collections::HashMap;

#[test]
fn test_vibe_test_basic_functionality() {
    let test = VibeTest::new("test_basic");
    
    // Test basic methods
    assert_eq!(test.Name(), "test_basic");
    assert!(!test.Failed());
    assert!(!test.Skipped());
    
    // Test logging
    let result = test.Log(&[Value::String("Test message".to_string())]);
    assert!(result.is_ok());
    
    let logs = test.get_logs();
    assert_eq!(logs.len(), 1);
    assert_eq!(logs[0], "Test message");
}

#[test]
fn test_vibe_test_assertions() {
    let test = VibeTest::new("test_assertions");
    
    // Test successful assertions
    let result = Assert(&test, true, "Should pass");
    assert!(result.is_ok());
    
    let result = AssertEqual(&test, &Value::Int(42), &Value::Int(42), "Should be equal");
    assert!(result.is_ok());
    
    let result = AssertNotEqual(&test, &Value::Int(42), &Value::Int(24), "Should not be equal");
    assert!(result.is_ok());
    
    let result = AssertNil(&test, &Value::Nil, "Should be nil");
    assert!(result.is_ok());
    
    let result = AssertNotNil(&test, &Value::String("test".to_string()), "Should not be nil");
    assert!(result.is_ok());
}

#[test]
fn test_vibe_test_string_assertions() {
    let test = VibeTest::new("test_string_assertions");
    
    let result = AssertContainsSubtea(&test, "hello world", "world", "Should contain substring");
    assert!(result.is_ok());
    
    let result = AssertHasPrefix(&test, "hello world", "hello", "Should have prefix");
    assert!(result.is_ok());
    
    let result = AssertHasSuffix(&test, "hello world", "world", "Should have suffix");
    assert!(result.is_ok());
    
    let result = AssertMatchesRegex(&test, "hello123", "*123", "Should match pattern");
    assert!(result.is_ok());
}

#[test]
fn test_vibe_test_numeric_assertions() {
    let test = VibeTest::new("test_numeric_assertions");
    
    let result = AssertGreater(&test, &Value::Int(10), &Value::Int(5), "10 > 5");
    assert!(result.is_ok());
    
    let result = AssertLess(&test, &Value::Int(5), &Value::Int(10), "5 < 10");
    assert!(result.is_ok());
    
    let result = AssertZero(&test, &Value::Int(0), "Should be zero");
    assert!(result.is_ok());
    
    let result = AssertNotZero(&test, &Value::Int(42), "Should not be zero");
    assert!(result.is_ok());
}

#[test]
fn test_vibe_test_collection_assertions() {
    let test = VibeTest::new("test_collection_assertions");
    
    let array = Value::Array(vec![Value::Int(1), Value::Int(2), Value::Int(3)]);
    
    let result = AssertLen(&test, &array, 3, "Array should have length 3");
    assert!(result.is_ok());
    
    let result = AssertNotEmpty(&test, &array, "Array should not be empty");
    assert!(result.is_ok());
    
    let result = AssertContains(&test, &array, &Value::Int(2), "Array should contain 2");
    assert!(result.is_ok());
    
    let result = AssertNotContains(&test, &array, &Value::Int(5), "Array should not contain 5");
    assert!(result.is_ok());
}

#[test]
fn test_vibe_bench_basic_functionality() {
    let mut bench = VibeBench::new("test_bench", 1000);
    
    assert_eq!(bench.Name(), "test_bench");
    assert_eq!(bench.N, 1000);
    assert!(!bench.Failed());
    assert!(!bench.Skipped());
    
    // Test timer functionality
    let result = bench.ResetTimer();
    assert!(result.is_ok());
    
    let result = bench.StartTimer();
    assert!(result.is_ok());
    
    // Simulate some work
    std::thread::sleep(std::time::Duration::from_millis(1));
    
    let result = bench.StopTimer();
    assert!(result.is_ok());
    
    // Test result generation
    let benchmark_result = bench.result();
    assert_eq!(benchmark_result.name, "test_bench");
    assert_eq!(benchmark_result.iterations, 1000);
    assert!(benchmark_result.ns_per_op > 0.0);
}

#[test]
fn test_mock_vibe_functionality() {
    let mock = MockVibe::new("TestService");
    
    assert_eq!(mock.Name, "TestService");
    
    // Test stub creation
    let stub = mock.Stub("get_data", vec![Value::String("test_data".to_string())]);
    assert_eq!(stub.call_count(), 0);
    
    // Test method call
    let result = mock.call_method("get_data", &[]);
    assert!(result.is_ok());
    
    let return_values = result.unwrap();
    assert_eq!(return_values.len(), 1);
    assert_eq!(return_values[0], Value::String("test_data".to_string()));
    
    // Verify call count
    assert_eq!(stub.call_count(), 1);
}

#[test]
fn test_mock_vibe_expectations() {
    let test = VibeTest::new("test_mock_expectations");
    let mock = MockVibe::new("TestService");
    
    // Create expectation
    let expectation = mock.Expect("process_data")
        .WithArgs(vec![Value::Int(123)])
        .Return(vec![Value::String("processed".to_string())])
        .Times(1);
    
    mock.add_expectation(expectation);
    
    // Call the method
    let result = mock.call_method("process_data", &[Value::Int(123)]);
    assert!(result.is_ok());
    
    let return_values = result.unwrap();
    assert_eq!(return_values[0], Value::String("processed".to_string()));
    
    // Verify expectations
    let result = mock.Verify(&test);
    assert!(result.is_ok());
}

#[test]
fn test_test_case_functionality() {
    let test = VibeTest::new("test_table_driven");
    
    let test_case = TestCase::new(
        "addition_test",
        Value::Array(vec![Value::Int(2), Value::Int(3)]),
        Value::Int(5),
        |_t, input, expected| {
            if let (Value::Array(operands), Value::Int(expected_result)) = (input, expected) {
                if operands.len() == 2 {
                    if let (Value::Int(a), Value::Int(b)) = (&operands[0], &operands[1]) {
                        let result = a + b;
                        if result == *expected_result {
                            Ok(())
                        } else {
                            Err(assertion_failed(&format!(
                                "Expected {}, got {}", expected_result, result
                            )).into())
                        }
                    } else {
                        Err(assertion_failed("Expected integer operands").into())
                    }
                } else {
                    Err(assertion_failed("Expected 2 operands").into())
                }
            } else {
                Err(assertion_failed("Invalid input/expected types").into())
            }
        }
    );
    
    let result = test_case.run(&test);
    assert!(result.is_ok());
}

#[test]
fn test_run_test_cases() {
    let test = VibeTest::new("test_run_cases");
    
    let test_cases = vec![
        TestCase::new(
            "case1",
            Value::String("hello".to_string()),
            Value::String("HELLO".to_string()),
            |_t, input, expected| {
                if let (Value::String(input_str), Value::String(expected_str)) = (input, expected) {
                    let result = input_str.to_uppercase();
                    if result == *expected_str {
                        Ok(())
                    } else {
                        Err(assertion_failed(&format!(
                            "Expected '{}', got '{}'", expected_str, result
                        )).into())
                    }
                } else {
                    Err(assertion_failed("Invalid types").into())
                }
            }
        ),
        TestCase::new(
            "case2",
            Value::String("WORLD".to_string()),
            Value::String("world".to_string()),
            |_t, input, expected| {
                if let (Value::String(input_str), Value::String(expected_str)) = (input, expected) {
                    let result = input_str.to_lowercase();
                    if result == *expected_str {
                        Ok(())
                    } else {
                        Err(assertion_failed(&format!(
                            "Expected '{}', got '{}'", expected_str, result
                        )).into())
                    }
                } else {
                    Err(assertion_failed("Invalid types").into())
                }
            }
        ),
    ];
    
    let result = RunTestCases(&test, &test_cases);
    assert!(result.is_ok());
}

#[test]
fn test_fixture_functionality() {
    let test = VibeTest::new("test_fixture");
    
    let fixture = NewFixtureVibe(
        |_t: &VibeTest| -> TestVibesResult<Value> {
            Ok(Value::String("fixture_data".to_string()))
        },
        |_t: &VibeTest, _fixture: &Value| -> TestVibesResult<()> {
            Ok(())
        }
    );
    
    let result = fixture.Run(&test, |_t: &VibeTest, data: &Value| -> TestVibesResult<()> {
        match data {
            Value::String(s) => {
                assert_eq!(s, "fixture_data");
                Ok(())
            }
            _ => panic!("Expected string value"),
        }
    });
    
    assert!(result.is_ok());
}

#[test]
fn test_utilities_random_functions() {
    // Test random string generation
    let random_str = RandomString(10);
    assert_eq!(random_str.len(), 10);
    
    // Test random int generation
    let random_int = RandomInt(1, 100);
    assert!(random_int >= 1 && random_int <= 100);
    
    // Test random float generation
    let random_float = RandomFloat(0.0, 1.0);
    assert!(random_float >= 0.0 && random_float < 1.0);
    
    // Test random bytes generation
    let random_bytes = RandomBytes(5);
    assert_eq!(random_bytes.len(), 5);
}

#[test]
fn test_temp_file_creation() {
    let test = VibeTest::new("test_temp_file");
    
    let result = TempFile(&test, "test_pattern");
    assert!(result.is_ok());
    
    let (handle, path) = result.unwrap();
    assert!(std::path::Path::new(&path).exists());
    
    // Verify cleanup on drop
    drop(handle);
}

#[test]
fn test_temp_dir_creation() {
    let test = VibeTest::new("test_temp_dir");
    
    let result = TempDir(&test, "test_dir");
    assert!(result.is_ok());
    
    let dir_path = result.unwrap();
    assert!(std::path::Path::new(&dir_path).is_dir());
}

#[test]
fn test_vibe_testing_manager() {
    let manager = VibeTestingManager::new();
    
    // In a real implementation, we would add tests and run them
    // For now, just verify the manager can be created
    let exit_code = manager.Run();
    assert_eq!(exit_code, 0); // No tests should pass
}

#[test]
fn test_error_types() {
    // Test error creation functions
    let test_error = test_failed("Test failed");
    assert!(matches!(test_error, TestVibesError::TestFailed(_)));
    
    let skip_error = test_skipped("Test skipped");
    assert!(matches!(skip_error, TestVibesError::TestSkipped(_)));
    
    let assertion_error = assertion_failed("Assertion failed");
    assert!(matches!(assertion_error, TestVibesError::AssertionFailed(_)));
    
    let expectation_error = expectation_not_met("Expectation not met");
    assert!(matches!(expectation_error, TestVibesError::ExpectationNotMet(_)));
    
    let timeout_error = timeout_exceeded("Timeout exceeded");
    assert!(matches!(timeout_error, TestVibesError::TimeoutExceeded(_)));
}

#[test]
fn test_type_assertions() {
    let test = VibeTest::new("test_type_assertions");
    
    let result = AssertType(&test, "int", &Value::Int(42), "Should be integer type");
    assert!(result.is_ok());
    
    let result = AssertType(&test, "string", &Value::String("test".to_string()), "Should be string type");
    assert!(result.is_ok());
    
    let result = AssertType(&test, "bool", &Value::Bool(true), "Should be boolean type");
    assert!(result.is_ok());
    
    let result = AssertType(&test, "array", &Value::Array(vec![]), "Should be array type");
    assert!(result.is_ok());
    
    let result = AssertType(&test, "object", &Value::Object(HashMap::new()), "Should be object type");
    assert!(result.is_ok());
}

#[test]
fn test_comprehensive_functionality() {
    // This test verifies that all major components work together
    let test = VibeTest::new("comprehensive_test");
    
    // Create a mock
    let mock = MockVibe::new("ComprehensiveService");
    let stub = mock.Stub("get_config", vec![Value::String("test_config".to_string())]);
    
    // Use the mock
    let result = mock.call_method("get_config", &[]);
    assert!(result.is_ok());
    
    // Verify stub was called
    assert_eq!(stub.call_count(), 1);
    
    // Create a fixture
    let fixture = NewFixtureVibe(
        |_t: &VibeTest| -> TestVibesResult<Value> {
            Ok(Value::Object(HashMap::new()))
        },
        |_t: &VibeTest, _fixture: &Value| -> TestVibesResult<()> {
            Ok(())
        }
    );
    
    // Run test with fixture
    let result = fixture.Run(&test, |t: &VibeTest, _data: &Value| -> TestVibesResult<()> {
        // Run various assertions
        AssertTrue(t, true, "Should be true")?;
        AssertEqual(t, &Value::Int(1), &Value::Int(1), "Should be equal")?;
        AssertNotNil(t, &Value::String("test".to_string()), "Should not be nil")?;
        
        Ok(())
    });
    
    assert!(result.is_ok());
    assert!(!test.Failed());
}
