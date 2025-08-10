//! Testing functionality for table_driven

use crate::error::CursedError;
use super::core::VibeTest;

/// Result type for test operations
pub type TestResult<T> = Result<T, CursedError>;

/// A test case for table-driven tests
#[derive(Debug, Clone)]
pub struct TestCase<T> {
    pub name: String,
    pub input: T,
    pub expected: T,
}

impl<T> TestCase<T> {
    /// Create a new test case
    pub fn new(name: String, input: T, expected: T) -> Self {
        Self {
            name,
            input,
            expected,
        }
    }
}

/// Run test cases
pub fn run_test_cases<T, F>(test: &mut VibeTest, cases: Vec<TestCase<T>>, test_fn: F) -> TestResult<()>
where
    T: Clone + PartialEq + std::fmt::Debug,
    F: Fn(&T) -> T,
{
    for case in cases {
        let result = test_fn(&case.input);
        if result != case.expected {
            let error_msg = format!(
                "Test case '{}' failed: expected {:?}, got {:?}",
                case.name, case.expected, result
            );
            test.fail(&error_msg);
            return Err(CursedError::runtime_error(&error_msg));
        }
    }
    test.pass();
    Ok(())
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
pub fn init_table_driven() -> TestResult<()> {
    let handler = TestHandler::new();
    handler.assert_eq(2 + 2, 4)?;
    println!("🧪 Test processing (table_driven) initialized");
    Ok(())
}

/// Test functionality
pub fn test_table_driven() -> TestResult<()> {
    let handler = TestHandler::new();
    handler.run_test("basic_test", || {
        handler.assert_true(true)?;
        handler.assert_false(false)?;
        Ok(())
    })?;
    Ok(())
}
