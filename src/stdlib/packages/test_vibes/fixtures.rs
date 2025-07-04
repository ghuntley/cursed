//! Testing functionality for fixtures

use crate::error::CursedError;
use std::collections::HashMap;
use std::any::Any;
use crate::stdlib::packages::CryptoError;

/// Result type for test operations
pub type TestResult<T> = Result<T, CursedError>;

/// Test fixture data container
#[derive(Debug)]
pub struct FixtureVibe {
    pub name: String,
    pub data: HashMap<String, Box<dyn Any + Send + Sync>>,
    pub setup: Option<fn() -> TestResult<()>>,
    pub teardown: Option<fn() -> TestResult<()>>,
}

impl FixtureVibe {
    pub fn new(name: String) -> Self {
        Self {
            name,
            data: HashMap::new(),
            setup: None,
            teardown: None,
        }
    }

    pub fn with_setup(mut self, setup_fn: fn() -> TestResult<()>) -> Self {
        self.setup = Some(setup_fn);
        self
    }

    pub fn with_teardown(mut self, teardown_fn: fn() -> TestResult<()>) -> Self {
        self.teardown = Some(teardown_fn);
        self
    }

    pub fn add_data<T: Any + Send + Sync>(mut self, key: &str, value: T) -> Self {
        self.data.insert(key.to_string(), Box::new(value));
        self
    }

    pub fn get_data<T: Any + Clone>(&self, key: &str) -> Option<T> {
        self.data.get(key)
            .and_then(|boxed| boxed.downcast_ref::<T>())
            .cloned()
    }

    pub fn setup(&self) -> TestResult<()> {
        if let Some(setup_fn) = self.setup {
            setup_fn()
        } else {
            Ok(())
        }
    }

    pub fn teardown(&self) -> TestResult<()> {
        if let Some(teardown_fn) = self.teardown {
            teardown_fn()
        } else {
            Ok(())
        }
    }
}

/// Individual test case within a test suite
#[derive(Debug, Clone)]
pub struct TestCase {
    pub name: String,
    pub description: Option<String>,
    pub test_fn: Option<fn(&FixtureVibe) -> TestResult<()>>,
    pub expected_result: Option<bool>,
    pub tags: Vec<String>,
}

impl TestCase {
    pub fn new(name: String) -> Self {
        Self {
            name,
            description: None,
            test_fn: None,
            expected_result: None,
            tags: Vec::new(),
        }
    }

    pub fn description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    pub fn with_test_fn(mut self, test_fn: fn(&FixtureVibe) -> TestResult<()>) -> Self {
        self.test_fn = Some(test_fn);
        self
    }

    pub fn expected_result(mut self, expected: bool) -> Self {
        self.expected_result = Some(expected);
        self
    }

    pub fn tag(mut self, tag: &str) -> Self {
        self.tags.push(tag.to_string());
        self
    }

    pub fn run(&self, fixture: &FixtureVibe) -> TestResult<()> {
        println!("🧪 Running test case: {}", self.name);
        
        if let Some(test_fn) = self.test_fn {
            let result = test_fn(fixture);
            
            match (&result, self.expected_result) {
                (Ok(()), Some(false)) => {
                    println!("❌ Test case expected to fail but passed: {}", self.name);
                    Err(CursedError::runtime_error("Test case expected to fail but passed"))
                }
                (Err(_), Some(true)) => {
                    println!("❌ Test case expected to pass but failed: {}", self.name);
                    result
                }
                (Ok(()), _) => {
                    println!("✅ Test case passed: {}", self.name);
                    Ok(())
                }
                (Err(_), Some(false)) => {
                    println!("✅ Test case failed as expected: {}", self.name);
                    Ok(())
                }
                (Err(e), _) => {
                    println!("❌ Test case failed: {}: {}", self.name, e);
                    result
                }
            }
        } else {
            println!("⚠️ Test case has no test function: {}", self.name);
            Ok(())
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
pub fn init_fixtures() -> TestResult<()> {
    let handler = TestHandler::new();
    handler.assert_eq(2 + 2, 4)?;
    println!("🧪 Test processing (fixtures) initialized");
    Ok(())
}

/// Test functionality
pub fn test_fixtures() -> TestResult<()> {
    let handler = TestHandler::new();
    handler.run_test("basic_test", || {
        handler.assert_true(true)?;
        handler.assert_false(false)?;
        Ok(())
    })?;
    Ok(())
}

/// Database fixture for testing database operations
#[derive(Debug, Default)]
pub struct DatabaseFixture {
    pub connection_string: String,
    pub test_data: Vec<String>,
}

impl DatabaseFixture {
    /// Create a new database fixture
    pub fn new() -> Self {
        Self {
            connection_string: "sqlite::memory:".to_string(),
            test_data: vec!["test_user".to_string(), "test_data".to_string()],
        }
    }
    
    /// Setup test database
    pub fn setup(&mut self) -> TestResult<()> {
        println!("Setting up test database...");
        Ok(())
    }
    
    /// Teardown test database
    pub fn teardown(&mut self) -> TestResult<()> {
        println!("Tearing down test database...");
        Ok(())
    }
}
