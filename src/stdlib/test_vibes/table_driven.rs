/// Table-driven tests for the TestVibes framework
/// 
/// Provides support for parameterized tests with multiple test cases

// use crate::stdlib::value::Value;
use super::{VibeTest, TestVibesResult};
use std::sync::Arc;

/// A single test case in a table-driven test
#[derive(Clone)]
pub struct TestCase {
impl TestCase {
    /// Create a new test case
    pub fn new<F>(name: &str, input: Value, expected: Value, test_fn: F) -> Self
    where
    {
        Self {
        }
    }

    /// Create a new test case with setup function
    pub fn new_with_setup<S, F>(
    ) -> Self
    where
    {
        Self {
        }
    }

    /// Run this test case
    pub fn run(&self, parent_test: &VibeTest) -> TestVibesResult<()> {
        // Create a sub-test for this case
        let sub_test = VibeTest::new(&format!("{}/{}", parent_test.Name(), self.Name));
        
        // Run setup if provided
        if let Some(ref setup) = self.SetupFn {
            setup(&sub_test)?;
        // Log test case info
        sub_test.Log(&[Value::String(format!(
            value_to_string(&self.Input)
        ))])?;
        
        // Run the actual test function
        let result = (self.TestFn)(&sub_test, &self.Input, &self.Expected);
        
        // Propagate sub-test results to parent
        if sub_test.Failed() {
            parent_test.Fail()?;
            for log in sub_test.get_logs() {
                parent_test.Log(&[Value::String(format!("  {}: {}", self.Name, log))])?;
            }
        } else if sub_test.Skipped() {
            parent_test.Log(&[Value::String(format!("  {}: SKIPPED", self.Name))])?;
        } else {
            parent_test.Log(&[Value::String(format!("  {}: PASSED", self.Name))])?;
        result
    }
}

/// Run a collection of test cases
pub fn RunTestCases(t: &VibeTest, test_cases: &[TestCase]) -> TestVibesResult<()> {
    let mut failed_cases = Vec::new();
    let mut skipped_cases = Vec::new();
    let mut passed_cases = Vec::new();
    
    t.Log(&[Value::String(format!("Running {} test cases", test_cases.len()))])?;
    
    for test_case in test_cases {
        match test_case.run(t) {
            Ok(_) => {
                if t.Skipped() {
                    skipped_cases.push(test_case.Name.clone());
                } else {
                    passed_cases.push(test_case.Name.clone());
                }
            }
            Err(_) => {
                failed_cases.push(test_case.Name.clone());
            }
        }
    // Log summary
    t.Log(&[Value::String(format!(
        skipped_cases.len()
    ))])?;
    
    if !failed_cases.is_empty() {
        t.Log(&[Value::String(format!(
            failed_cases.join(", ")
        ))])?;
        return t.Fatal(&[Value::String("One or more test cases failed".to_string())]);
    Ok(())
/// Builder for creating table-driven test suites
pub struct TableTestBuilder {
impl TableTestBuilder {
    /// Create a new table test builder
    pub fn new(name: &str) -> Self {
        Self {
        }
    }

    /// Add a test case
    pub fn add_case<F>(mut self, name: &str, input: Value, expected: Value, test_fn: F) -> Self
    where
    {
        self.cases.push(TestCase::new(name, input, expected, test_fn));
        self
    /// Add a test case with setup
    pub fn add_case_with_setup<S, F>(
    ) -> Self
    where
    {
        self.cases.push(TestCase::new_with_setup(name, input, expected, setup_fn, test_fn));
        self
    /// Set global setup function
    pub fn with_setup<F>(mut self, setup_fn: F) -> Self
    where
    {
        self.global_setup = Some(Arc::new(setup_fn));
        self
    /// Set global teardown function
    pub fn with_teardown<F>(mut self, teardown_fn: F) -> Self
    where
    {
        self.global_teardown = Some(Arc::new(teardown_fn));
        self
    /// Run all test cases
    pub fn run(self, t: &VibeTest) -> TestVibesResult<()> {
        t.Log(&[Value::String(format!("Running table test: {}", self.name))])?;
        
        // Run global setup
        if let Some(ref setup) = self.global_setup {
            setup(t)?;
        // Run test cases
        let result = RunTestCases(t, &self.cases);
        
        // Run global teardown
        if let Some(ref teardown) = self.global_teardown {
            if let Err(teardown_err) = teardown(t) {
                t.Log(&[Value::String(format!("Teardown failed: {}", teardown_err))])?;
            }
        }
        
        result
    }
}

// Common test case patterns

/// Helper for creating string transformation test cases
pub fn string_transform_cases() -> Vec<TestCase> {
    vec![
        TestCase::new(
            |_t, input, expected| {
                if let (Value::String(input_str), Value::String(expected_str)) = (input, expected) {
                    let result = input_str.to_uppercase();
                    if result == *expected_str {
                        Ok(())
                    } else {
                        Err(super::assertion_failed(&format!(
                            "Expected '{}', got '{}'", expected_str, result
                        )).into())
                    }
                } else {
                    Err(super::assertion_failed("Invalid input/expected types").into())
                }
            }
        TestCase::new(
            |_t, input, expected| {
                if let (Value::String(input_str), Value::String(expected_str)) = (input, expected) {
                    let result = input_str.to_lowercase();
                    if result == *expected_str {
                        Ok(())
                    } else {
                        Err(super::assertion_failed(&format!(
                            "Expected '{}', got '{}'", expected_str, result
                        )).into())
                    }
                } else {
                    Err(super::assertion_failed("Invalid input/expected types").into())
                }
            }
        TestCase::new(
            |_t, input, expected| {
                if let (Value::String(input_str), Value::String(expected_str)) = (input, expected) {
                    let result = input_str.trim();
                    if result == *expected_str {
                        Ok(())
                    } else {
                        Err(super::assertion_failed(&format!(
                            "Expected '{}', got '{}'", expected_str, result
                        )).into())
                    }
                } else {
                    Err(super::assertion_failed("Invalid input/expected types").into())
                }
            }
    ]
/// Helper for creating mathematical operation test cases
pub fn math_operation_cases() -> Vec<TestCase> {
    vec![
        TestCase::new(
            |_t, input, expected| {
                if let (Value::Array(operands), Value::Int(expected_result)) = (input, expected) {
                    if operands.len() == 2 {
                        if let (Value::Int(a), Value::Int(b)) = (&operands[0], &operands[1]) {
                            let result = a + b;
                            if result == *expected_result {
                                Ok(())
                            } else {
                                Err(super::assertion_failed(&format!(
                                    "Expected {}, got {}", expected_result, result
                                )).into())
                            }
                        } else {
                            Err(super::assertion_failed("Expected integer operands").into())
                        }
                    } else {
                        Err(super::assertion_failed("Expected 2 operands").into())
                    }
                } else {
                    Err(super::assertion_failed("Invalid input/expected types").into())
                }
            }
        TestCase::new(
            |_t, input, expected| {
                if let (Value::Array(operands), Value::Int(expected_result)) = (input, expected) {
                    if operands.len() == 2 {
                        if let (Value::Int(a), Value::Int(b)) = (&operands[0], &operands[1]) {
                            let result = a * b;
                            if result == *expected_result {
                                Ok(())
                            } else {
                                Err(super::assertion_failed(&format!(
                                    "Expected {}, got {}", expected_result, result
                                )).into())
                            }
                        } else {
                            Err(super::assertion_failed("Expected integer operands").into())
                        }
                    } else {
                        Err(super::assertion_failed("Expected 2 operands").into())
                    }
                } else {
                    Err(super::assertion_failed("Invalid input/expected types").into())
                }
            }
        TestCase::new(
            |_t, input, expected| {
                if let (Value::Array(operands), Value::Int(expected_result)) = (input, expected) {
                    if operands.len() == 2 {
                        if let (Value::Int(a), Value::Int(b)) = (&operands[0], &operands[1]) {
                            if *b == 0 {
                                return Err(super::assertion_failed("Division by zero").into());
                            }
                            let result = a / b;
                            if result == *expected_result {
                                Ok(())
                            } else {
                                Err(super::assertion_failed(&format!(
                                    "Expected {}, got {}", expected_result, result
                                )).into())
                            }
                        } else {
                            Err(super::assertion_failed("Expected integer operands").into())
                        }
                    } else {
                        Err(super::assertion_failed("Expected 2 operands").into())
                    }
                } else {
                    Err(super::assertion_failed("Invalid input/expected types").into())
                }
            }
    ]
// Helper functions

/// Convert value to string representation
fn value_to_string(value: &Value) -> String {
    match value {
        Value::Array(arr) => {
            let elements: Vec<String> = arr.iter().map(value_to_string).collect();
            format!("[{}]", elements.join(", "))
        }
        Value::Object(obj) => {
            let pairs: Vec<String> = obj.iter()
                .map(|(k, v)| format!("{}: {}", k, value_to_string(v)))
                .collect();
            format!("{{{}}}", pairs.join(", "))
        }
    }
