/// fr fr Test fixtures and table-driven tests for the TestVibes framework
// use crate::stdlib::packages::test_vibes::core::VibeTest;
use std::any::Any;

/// fr fr Test fixture for setup and teardown
pub struct FixtureVibe<T> {
    setup_fn: Box<dyn Fn(&mut VibeTest) -> T>,
    teardown_fn: Box<dyn Fn(&mut VibeTest, T)>,
}

impl<T> FixtureVibe<T> {
    /// fr fr Create a new fixture with setup and teardown functions
    pub fn new<S, TD>(setup: S, teardown: TD) -> Self
    where
        S: Fn(&mut VibeTest) -> T + 'static,
        TD: Fn(&mut VibeTest, T) + 'static,
    {
        Self {
            setup_fn: Box::new(setup),
            teardown_fn: Box::new(teardown),
        }
    }

    /// fr fr Run a test with the fixture
    pub fn run<F>(&self, t: &mut VibeTest, test_fn: F)
    where
        F: FnOnce(&mut VibeTest, T),
    {
        // Setup phase
        let fixture = (self.setup_fn)(t);
        
        // If setup failed, don't run the test
        if t.failed() {
            t.error(&["Setup failed, skipping test"]);
            return;
        }

        // Run the test with the fixture
        test_fn(t, fixture);

        // Teardown phase - always run even if test failed
        let fixture_for_teardown = (self.setup_fn)(t); // Re-setup for teardown
        (self.teardown_fn)(t, fixture_for_teardown);
    }

    /// fr fr Run multiple tests with the same fixture
    pub fn run_tests<F>(&self, test_name: &str, tests: Vec<(&str, F)>)
    where
        F: Fn(&mut VibeTest, &T) + Clone,
    {
        for (name, test_fn) in tests {
            let mut test = VibeTest::new(format!("{}::{}", test_name, name));
            
            // Setup
            let fixture = (self.setup_fn)(&mut test);
            
            if !test.failed() {
                // Run test
                test_fn(&mut test, &fixture);
            }
            
            // Teardown
            let fixture_for_teardown = (self.setup_fn)(&mut test);
            (self.teardown_fn)(&mut test, fixture_for_teardown);
            
            // Print result
            let result = test.get_result();
            println!("{}: {}", name, if result.passed { "PASS" } else { "FAIL" });
        }
    }
}

/// fr fr Generic fixture for Any type (type-erased)
pub struct GenericFixture {
    setup_fn: Box<dyn Fn(&mut VibeTest) -> Box<dyn Any>>,
    teardown_fn: Box<dyn Fn(&mut VibeTest, Box<dyn Any>)>,
}

impl GenericFixture {
    /// fr fr Create a new generic fixture
    pub fn new<T, S, TD>(setup: S, teardown: TD) -> Self
    where
        T: Any + 'static,
        S: Fn(&mut VibeTest) -> T + 'static,
        TD: Fn(&mut VibeTest, T) + 'static,
    {
        Self {
            setup_fn: Box::new(move |t| Box::new(setup(t))),
            teardown_fn: Box::new(move |t, fixture| {
                if let Ok(concrete_fixture) = fixture.downcast::<T>() {
                    teardown(t, *concrete_fixture);
                }
            }),
        }
    }

    /// fr fr Run a test with the generic fixture
    pub fn run<T, F>(&self, t: &mut VibeTest, test_fn: F)
    where
        T: Any + 'static,
        F: FnOnce(&mut VibeTest, &T),
    {
        // Setup
        let fixture = (self.setup_fn)(t);
        
        if t.failed() {
            t.error(&["Setup failed, skipping test"]);
            return;
        }

        // Downcast and run test
        if let Some(concrete_fixture) = fixture.downcast_ref::<T>() {
            test_fn(t, concrete_fixture);
        } else {
            t.error(&["Failed to downcast fixture to expected type"]);
        }

        // Teardown
        (self.teardown_fn)(t, fixture);
    }
}

/// fr fr Test case for table-driven tests
pub struct TestCase<I, E> {
    pub name: String,
    pub input: I,
    pub expected: E,
    pub setup_fn: Option<Box<dyn Fn(&mut VibeTest)>>,
    pub test_fn: Box<dyn Fn(&mut VibeTest, I, E)>,
}

impl<I, E> TestCase<I, E>
where
    I: Clone,
    E: Clone,
{
    /// fr fr Create a new test case
    pub fn new<F>(name: &str, input: I, expected: E, test_fn: F) -> Self
    where
        F: Fn(&mut VibeTest, I, E) + 'static,
    {
        Self {
            name: name.to_string(),
            input,
            expected,
            setup_fn: None,
            test_fn: Box::new(test_fn),
        }
    }

    /// fr fr Create a test case with setup function
    pub fn with_setup<F, S>(name: &str, input: I, expected: E, setup: S, test_fn: F) -> Self
    where
        F: Fn(&mut VibeTest, I, E) + 'static,
        S: Fn(&mut VibeTest) + 'static,
    {
        Self {
            name: name.to_string(),
            input,
            expected,
            setup_fn: Some(Box::new(setup)),
            test_fn: Box::new(test_fn),
        }
    }

    /// fr fr Run this individual test case
    pub fn run(&self, t: &mut VibeTest) {
        // Run setup if provided
        if let Some(ref setup) = self.setup_fn {
            setup(t);
            
            if t.failed() {
                t.error(&["Setup failed for test case:", &self.name]);
                return;
            }
        }

        // Run the test
        (self.test_fn)(t, self.input.clone(), self.expected.clone());
    }
}

/// fr fr Run multiple test cases
pub fn run_test_cases<I, E>(base_name: &str, test_cases: Vec<TestCase<I, E>>)
where
    I: Clone,
    E: Clone,
{
    for case in test_cases {
        let mut test = VibeTest::new(format!("{}::{}", base_name, case.name));
        case.run(&mut test);
        
        let result = test.get_result();
        let status = if result.passed { "PASS" } else if result.skipped { "SKIP" } else { "FAIL" };
        println!("{} {} ({:.2?})", status, result.name, result.duration);
        
        if result.failed {
            for error in &result.errors {
                println!("    ERROR: {}", error);
            }
        }
    }
}

/// fr fr Parameterized test runner
pub struct ParameterizedTest<P> {
    name: String,
    parameters: Vec<P>,
    test_fn: Box<dyn Fn(&mut VibeTest, P)>,
}

impl<P> ParameterizedTest<P>
where
    P: Clone + std::fmt::Debug,
{
    /// fr fr Create a new parameterized test
    pub fn new<F>(name: &str, parameters: Vec<P>, test_fn: F) -> Self
    where
        F: Fn(&mut VibeTest, P) + 'static,
    {
        Self {
            name: name.to_string(),
            parameters,
            test_fn: Box::new(test_fn),
        }
    }

    /// fr fr Run the parameterized test with all parameters
    pub fn run(&self) {
        for (i, param) in self.parameters.iter().enumerate() {
            let mut test = VibeTest::new(format!("{}[{}]", self.name, i));
            (self.test_fn)(&mut test, param.clone());
            
            let result = test.get_result();
            let status = if result.passed { "PASS" } else if result.skipped { "SKIP" } else { "FAIL" };
            println!("{} {} with param {:?} ({:.2?})", status, result.name, param, result.duration);
            
            if result.failed {
                for error in &result.errors {
                    println!("    ERROR: {}", error);
                }
            }
        }
    }
}

/// fr fr Test suite for organizing related tests
pub struct TestSuite {
    name: String,
    tests: Vec<Box<dyn Fn(&mut VibeTest)>>,
    setup: Option<Box<dyn Fn(&mut VibeTest)>>,
    teardown: Option<Box<dyn Fn(&mut VibeTest)>>,
}

impl TestSuite {
    /// fr fr Create a new test suite
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            tests: Vec::new(),
            setup: None,
            teardown: None,
        }
    }

    /// fr fr Add a test to the suite
    pub fn add_test<F>(&mut self, test_fn: F)
    where
        F: Fn(&mut VibeTest) + 'static,
    {
        self.tests.push(Box::new(test_fn));
    }

    /// fr fr Set suite-level setup function
    pub fn set_setup<F>(&mut self, setup_fn: F)
    where
        F: Fn(&mut VibeTest) + 'static,
    {
        self.setup = Some(Box::new(setup_fn));
    }

    /// fr fr Set suite-level teardown function
    pub fn set_teardown<F>(&mut self, teardown_fn: F)
    where
        F: Fn(&mut VibeTest) + 'static,
    {
        self.teardown = Some(Box::new(teardown_fn));
    }

    /// fr fr Run all tests in the suite
    pub fn run(&self) {
        println!("Running test suite: {}", self.name);
        
        let mut passed = 0;
        let mut failed = 0;
        let mut skipped = 0;

        for (i, test_fn) in self.tests.iter().enumerate() {
            let mut test = VibeTest::new(format!("{}::test_{}", self.name, i));
            
            // Suite setup
            if let Some(ref setup) = self.setup {
                setup(&mut test);
            }
            
            // Run test if setup didn't fail
            if !test.failed() {
                test_fn(&mut test);
            }
            
            // Suite teardown
            if let Some(ref teardown) = self.teardown {
                teardown(&mut test);
            }
            
            // Count results
            let result = test.get_result();
            if result.passed {
                passed += 1;
            } else if result.skipped {
                skipped += 1;
            } else {
                failed += 1;
            }
            
            // Print result
            let status = if result.passed { "PASS" } else if result.skipped { "SKIP" } else { "FAIL" };
            println!("  {} {} ({:.2?})", status, result.name, result.duration);
            
            if result.failed {
                for error in &result.errors {
                    println!("    ERROR: {}", error);
                }
            }
        }
        
        println!("Suite {} completed: {} passed, {} failed, {} skipped", 
                 self.name, passed, failed, skipped);
    }
}

/// fr fr Database fixture example
pub struct DatabaseFixture {
    pub connection_string: String,
    pub connected: bool,
}

impl DatabaseFixture {
    pub fn new(conn_str: &str) -> Self {
        Self {
            connection_string: conn_str.to_string(),
            connected: false,
        }
    }

    pub fn connect(&mut self) -> Result<(), &'static str> {
        // Simulate database connection
        if self.connection_string.is_empty() {
            Err("Invalid connection string")
        } else {
            self.connected = true;
            Ok(())
        }
    }

    pub fn disconnect(&mut self) {
        self.connected = false;
    }

    pub fn execute_query(&self, _query: &str) -> Result<Vec<String>, &'static str> {
        if !self.connected {
            Err("Not connected to database")
        } else {
            Ok(Vec::from(["result1".to_string(), "result2".to_string()]))
        }
    }
}

/// fr fr HTTP server fixture example
pub struct HttpServerFixture {
    pub port: u16,
    pub running: bool,
}

impl HttpServerFixture {
    pub fn new(port: u16) -> Self {
        Self {
            port,
            running: false,
        }
    }

    pub fn start(&mut self) -> Result<(), &'static str> {
        if self.port == 0 {
            Err("Invalid port")
        } else {
            self.running = true;
            Ok(())
        }
    }

    pub fn stop(&mut self) {
        self.running = false;
    }

    pub fn get_url(&self) -> String {
        format!("http://localhost:{}", self.port)
    }
}

