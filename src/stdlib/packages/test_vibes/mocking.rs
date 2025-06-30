//! Testing functionality for mocking

use crate::error::CursedError;
use std::collections::HashMap;
use std::any::Any;

/// Result type for test operations
pub type TestResult<T> = Result<T, CursedError>;

/// Mock object for testing interactions
#[derive(Debug)]
pub struct MockVibe {
    pub name: String,
    pub expectations: Vec<Expectation>,
    pub stubs: Vec<Stub>,
    pub call_history: Vec<String>,
    pub strict_mode: bool,
}

impl MockVibe {
    pub fn new(name: String) -> Self {
        Self {
            name,
            expectations: Vec::new(),
            stubs: Vec::new(),
            call_history: Vec::new(),
            strict_mode: false,
        }
    }

    pub fn strict(mut self) -> Self {
        self.strict_mode = true;
        self
    }

    pub fn expect(mut self, expectation: Expectation) -> Self {
        self.expectations.push(expectation);
        self
    }

    pub fn stub(mut self, stub: Stub) -> Self {
        self.stubs.push(stub);
        self
    }

    pub fn was_called(&self, method_name: &str) -> bool {
        self.call_history.iter().any(|call| call == method_name)
    }

    pub fn call_count(&self, method_name: &str) -> usize {
        self.call_history.iter().filter(|call| *call == method_name).count()
    }

    pub fn record_call(&mut self, method_name: &str) {
        self.call_history.push(method_name.to_string());
    }

    pub fn verify(&self) -> TestResult<()> {
        for expectation in &self.expectations {
            if !expectation.is_satisfied(&self.call_history) {
                return Err(CursedError::runtime_error(&format!(
                    "Expectation not met for method: {}", expectation.method_name
                )));
            }
        }

        if self.strict_mode {
            let expected_calls: Vec<_> = self.expectations.iter()
                .map(|e| e.method_name.clone())
                .collect();
            
            for call in &self.call_history {
                if !expected_calls.contains(call) {
                    return Err(CursedError::runtime_error(&format!(
                        "Unexpected call in strict mode: {}", call
                    )));
                }
            }
        }

        Ok(())
    }

    pub fn reset(&mut self) {
        self.call_history.clear();
        self.expectations.clear();
        self.stubs.clear();
    }
}

/// Expectation for mock method calls
#[derive(Debug)]
pub struct Expectation {
    pub method_name: String,
    pub min_calls: usize,
    pub max_calls: Option<usize>,
    pub with_args: Option<Vec<String>>,
    pub return_value: Option<Box<dyn Any + Send + Sync>>,
}

impl Clone for Expectation {
    fn clone(&self) -> Self {
        Self {
            method_name: self.method_name.clone(),
            min_calls: self.min_calls,
            max_calls: self.max_calls,
            with_args: self.with_args.clone(),
            return_value: None, // Can't clone boxed Any trait objects
        }
    }
}

impl Expectation {
    pub fn new(method_name: String) -> Self {
        Self {
            method_name,
            min_calls: 1,
            max_calls: None,
            with_args: None,
            return_value: None,
        }
    }

    pub fn times(mut self, count: usize) -> Self {
        self.min_calls = count;
        self.max_calls = Some(count);
        self
    }

    pub fn at_least(mut self, count: usize) -> Self {
        self.min_calls = count;
        self.max_calls = None;
        self
    }

    pub fn at_most(mut self, count: usize) -> Self {
        self.min_calls = 0;
        self.max_calls = Some(count);
        self
    }

    pub fn with_args(mut self, args: Vec<String>) -> Self {
        self.with_args = Some(args);
        self
    }

    pub fn returns<T: Any + Send + Sync>(mut self, value: T) -> Self {
        self.return_value = Some(Box::new(value));
        self
    }

    pub fn is_satisfied(&self, call_history: &[String]) -> bool {
        let call_count = call_history.iter()
            .filter(|call| *call == &self.method_name)
            .count();

        if call_count < self.min_calls {
            return false;
        }

        if let Some(max) = self.max_calls {
            if call_count > max {
                return false;
            }
        }

        true
    }
}

/// Stub for replacing method implementations
#[derive(Debug)]
pub struct Stub {
    pub method_name: String,
    pub implementation: Option<fn() -> TestResult<Box<dyn Any + Send + Sync>>>,
    pub return_value: Option<Box<dyn Any + Send + Sync>>,
    pub should_panic: bool,
}

impl Stub {
    pub fn new(method_name: String) -> Self {
        Self {
            method_name,
            implementation: None,
            return_value: None,
            should_panic: false,
        }
    }

    pub fn with_implementation(mut self, implementation: fn() -> TestResult<Box<dyn Any + Send + Sync>>) -> Self {
        self.implementation = Some(implementation);
        self
    }

    pub fn returns<T: Any + Send + Sync>(mut self, value: T) -> Self {
        self.return_value = Some(Box::new(value));
        self
    }

    pub fn panics(mut self) -> Self {
        self.should_panic = true;
        self
    }

    pub fn call(&self) -> TestResult<Option<Box<dyn Any + Send + Sync>>> {
        if self.should_panic {
            return Err(CursedError::runtime_error("Stubbed method panicked"));
        }

        if let Some(implementation) = self.implementation {
            implementation().map(Some)
        } else if let Some(ref return_value) = self.return_value {
            // Clone the return value (simplified - in practice would need better cloning)
            Ok(self.return_value.as_ref().map(|_| Box::new(()) as Box<dyn Any + Send + Sync>))
        } else {
            Ok(None)
        }
    }
}

/// Test operations handler
pub struct TestHandler {
    verbose: bool,
}

impl TestHandler {
    /// Create a new test handler
    pub fn new() -> Self {
        Self {
            verbose: false,
        }
    }
    
    /// Set verbose mode
    pub fn verbose(mut self, verbose: bool) -> Self {
        self.verbose = verbose;
        self
    }
    
    /// Assert equality
    pub fn assert_eq<T: PartialEq + std::fmt::Debug>(&self, left: T, right: T) -> TestResult<()> {
        if left == right {
            if self.verbose {
                println!("✅ Assertion passed: {:?} == {:?}", left, right);
            }
            Ok(())
        } else {
            Err(CursedError::runtime_error(&format!("Assertion failed: {:?} != {:?}", left, right)))
        }
    }
    
    /// Assert not equality
    pub fn assert_ne<T: PartialEq + std::fmt::Debug>(&self, left: T, right: T) -> TestResult<()> {
        if left != right {
            if self.verbose {
                println!("✅ Assertion passed: {:?} != {:?}", left, right);
            }
            Ok(())
        } else {
            Err(CursedError::runtime_error(&format!("Assertion failed: {:?} == {:?}", left, right)))
        }
    }
    
    /// Assert true
    pub fn assert_true(&self, condition: bool) -> TestResult<()> {
        if condition {
            if self.verbose {
                println!("✅ Assertion passed: condition is true");
            }
            Ok(())
        } else {
            Err(CursedError::runtime_error("Assertion failed: condition is false"))
        }
    }
    
    /// Assert false
    pub fn assert_false(&self, condition: bool) -> TestResult<()> {
        if !condition {
            if self.verbose {
                println!("✅ Assertion passed: condition is false");
            }
            Ok(())
        } else {
            Err(CursedError::runtime_error("Assertion failed: condition is true"))
        }
    }
    
    /// Run a test
    pub fn run_test<F>(&self, name: &str, test_fn: F) -> TestResult<()>
    where
        F: FnOnce() -> TestResult<()>,
    {
        if self.verbose {
            println!("🧪 Running test: {}", name);
        }
        
        match test_fn() {
            Ok(()) => {
                if self.verbose {
                    println!("✅ Test passed: {}", name);
                }
                Ok(())
            }
            Err(e) => {
                println!("❌ Test failed: {}: {}", name, e);
                Err(e)
            }
        }
    }
}

impl Default for TestHandler {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize test processing
pub fn init_mocking() -> TestResult<()> {
    let handler = TestHandler::new();
    handler.assert_eq(2 + 2, 4)?;
    println!("🧪 Test processing (mocking) initialized");
    Ok(())
}

/// Test functionality
pub fn test_mocking() -> TestResult<()> {
    let handler = TestHandler::new();
    handler.run_test("basic_test", || {
        handler.assert_true(true)?;
        handler.assert_false(false)?;
        Ok(())
    })?;
    Ok(())
}
