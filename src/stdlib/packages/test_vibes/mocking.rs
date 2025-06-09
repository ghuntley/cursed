/// fr fr Mocking framework for the TestVibes testing system
use crate::stdlib::packages::test_vibes::core::VibeTest;
use std::collections::HashMap;
use std::any::Any;
use std::sync::{Arc, Mutex};

/// fr fr Mock object for testing
#[derive(Debug)]
pub struct MockVibe {
    name: String,
    expectations: Arc<Mutex<HashMap<String, Vec<Expectation>>>>,
    stubs: Arc<Mutex<HashMap<String, Stub>>>,
    call_counts: Arc<Mutex<HashMap<String, usize>>>,
}

impl MockVibe {
    /// fr fr Create a new mock object
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            expectations: Arc::new(Mutex::new(HashMap::new())),
            stubs: Arc::new(Mutex::new(HashMap::new())),
            call_counts: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    /// fr fr Set up an expectation for a method call
    pub fn expect(&self, method_name: &str) -> Expectation {
        let mut expectations = self.expectations.lock().unwrap();
        let expectation = Expectation::new(method_name);
        
        expectations
            .entry(method_name.to_string())
            .or_insert_with(Vec::new)
            .push(expectation.clone());
        
        expectation
    }

    /// fr fr Set up a stub for a method call
    pub fn stub(&self, method_name: &str, return_values: Vec<String>) -> Stub {
        let mut stubs = self.stubs.lock().unwrap();
        let stub = Stub::new(method_name, return_values);
        stubs.insert(method_name.to_string(), stub.clone());
        stub
    }

    /// fr fr Record a method call
    pub fn record_call(&self, method_name: &str, args: Vec<String>) -> Option<String> {
        // Increment call count
        {
            let mut counts = self.call_counts.lock().unwrap();
            *counts.entry(method_name.to_string()).or_insert(0) += 1;
        }

        // Check expectations
        {
            let mut expectations = self.expectations.lock().unwrap();
            if let Some(method_expectations) = expectations.get_mut(method_name) {
                for expectation in method_expectations.iter_mut() {
                    if expectation.matches(&args) {
                        expectation.record_call();
                        if let Some(return_value) = expectation.get_return_value(&args) {
                            return Some(return_value);
                        }
                    }
                }
            }
        }

        // Check stubs
        {
            let stubs = self.stubs.lock().unwrap();
            if let Some(stub) = stubs.get(method_name) {
                return stub.get_return_value();
            }
        }

        None
    }

    /// fr fr Verify all expectations were met
    pub fn verify(&self, t: &mut VibeTest) {
        let expectations = self.expectations.lock().unwrap();
        let call_counts = self.call_counts.lock().unwrap();

        for (method_name, method_expectations) in expectations.iter() {
            let actual_calls = call_counts.get(method_name).unwrap_or(&0);

            for expectation in method_expectations {
                if !expectation.is_satisfied(*actual_calls) {
                    t.fail_vibe(&format!(
                        "Mock '{}' expectation failed for method '{}': expected {} calls, got {}",
                        self.name, method_name, expectation.get_expected_calls(), actual_calls
                    ));
                }
            }
        }
    }

    /// fr fr Get the name of this mock
    pub fn name(&self) -> &str {
        &self.name
    }

    /// fr fr Get call count for a method
    pub fn call_count(&self, method_name: &str) -> usize {
        let counts = self.call_counts.lock().unwrap();
        *counts.get(method_name).unwrap_or(&0)
    }

    /// fr fr Reset all call counts and expectations
    pub fn reset(&self) {
        self.expectations.lock().unwrap().clear();
        self.stubs.lock().unwrap().clear();
        self.call_counts.lock().unwrap().clear();
    }
}

impl Clone for MockVibe {
    fn clone(&self) -> Self {
        Self {
            name: self.name.clone(),
            expectations: Arc::clone(&self.expectations),
            stubs: Arc::clone(&self.stubs),
            call_counts: Arc::clone(&self.call_counts),
        }
    }
}

/// fr fr Method expectation configuration
#[derive(Debug, Clone)]
pub struct Expectation {
    method_name: String,
    expected_args: Option<Vec<String>>, // Simplified - would be Any in real implementation
    return_values: Vec<String>, // Simplified to avoid complex trait objects
    return_fn: Option<String>, // Simplified to string description
    call_count: Arc<Mutex<usize>>,
    min_calls: Option<usize>,
    max_calls: Option<usize>,
    exact_calls: Option<usize>,
}

impl Expectation {
    /// fr fr Create a new expectation
    pub fn new(method_name: &str) -> Self {
        Self {
            method_name: method_name.to_string(),
            expected_args: None,
            return_values: Vec::new(),
            return_fn: None,
            call_count: Arc::new(Mutex::new(0)),
            min_calls: None,
            max_calls: None,
            exact_calls: Some(1), // Default expectation is exactly one call
        }
    }

    /// fr fr Set expected arguments (simplified version)
    pub fn with_args(mut self, args: Vec<&str>) -> Self {
        self.expected_args = Some(args.iter().map(|s| s.to_string()).collect());
        self
    }

    /// fr fr Set return values (simplified)
    pub fn returns(mut self, values: Vec<String>) -> Self {
        self.return_values = values;
        self
    }

    /// fr fr Set return function (simplified)
    pub fn return_fn(mut self, description: &str) -> Self {
        self.return_fn = Some(description.to_string());
        self
    }

    /// fr fr Set exact number of expected calls
    pub fn times(mut self, n: usize) -> Self {
        self.exact_calls = Some(n);
        self.min_calls = None;
        self.max_calls = None;
        self
    }

    /// fr fr Set minimum number of expected calls
    pub fn at_least(mut self, n: usize) -> Self {
        self.min_calls = Some(n);
        self.exact_calls = None;
        self
    }

    /// fr fr Set maximum number of expected calls
    pub fn at_most(mut self, n: usize) -> Self {
        self.max_calls = Some(n);
        self.exact_calls = None;
        self
    }

    /// fr fr Check if arguments match expectation
    pub fn matches(&self, _args: &[String]) -> bool {
        // Simplified matching - in real implementation would compare actual values
        true
    }

    /// fr fr Record a call to this expectation
    pub fn record_call(&self) {
        let mut count = self.call_count.lock().unwrap();
        *count += 1;
    }

    /// fr fr Get return value for this call (simplified)
    pub fn get_return_value(&self, _args: &[String]) -> Option<String> {
        if self.return_fn.is_some() {
            Some("mocked_return_value".to_string())
        } else if !self.return_values.is_empty() {
            Some(self.return_values[0].clone())
        } else {
            None
        }
    }

    /// fr fr Check if expectation is satisfied
    pub fn is_satisfied(&self, actual_calls: usize) -> bool {
        if let Some(exact) = self.exact_calls {
            actual_calls == exact
        } else {
            let min_ok = self.min_calls.map_or(true, |min| actual_calls >= min);
            let max_ok = self.max_calls.map_or(true, |max| actual_calls <= max);
            min_ok && max_ok
        }
    }

    /// fr fr Get expected number of calls (for error messages)
    pub fn get_expected_calls(&self) -> String {
        if let Some(exact) = self.exact_calls {
            exact.to_string()
        } else {
            let min = self.min_calls.map_or("0".to_string(), |n| n.to_string());
            let max = self.max_calls.map_or("∞".to_string(), |n| n.to_string());
            format!("{}-{}", min, max)
        }
    }
}

/// fr fr Method stub for returning values
#[derive(Debug, Clone)]
pub struct Stub {
    method_name: String,
    return_values: Vec<String>, // Simplified
    call_index: Arc<Mutex<usize>>,
}

impl Stub {
    /// fr fr Create a new stub
    pub fn new(method_name: &str, return_values: Vec<String>) -> Self {
        Self {
            method_name: method_name.to_string(),
            return_values,
            call_index: Arc::new(Mutex::new(0)),
        }
    }

    /// fr fr Get return value for the current call
    pub fn get_return_value(&self) -> Option<String> {
        let mut index = self.call_index.lock().unwrap();
        
        if self.return_values.is_empty() {
            return None;
        }

        // Cycle through return values
        let value_index = *index % self.return_values.len();
        *index += 1;

        Some(self.return_values[value_index].clone())
    }
}

/// fr fr Mock builder for complex mock setup
pub struct MockBuilder {
    name: String,
    expectations: Vec<(String, Expectation)>,
    stubs: Vec<(String, Stub)>,
}

impl MockBuilder {
    /// fr fr Create a new mock builder
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            expectations: Vec::new(),
            stubs: Vec::new(),
        }
    }

    /// fr fr Add an expectation to the builder
    pub fn expect(mut self, method_name: &str, expectation: Expectation) -> Self {
        self.expectations.push((method_name.to_string(), expectation));
        self
    }

    /// fr fr Add a stub to the builder
    pub fn stub(mut self, method_name: &str, stub: Stub) -> Self {
        self.stubs.push((method_name.to_string(), stub));
        self
    }

    /// fr fr Build the mock object
    pub fn build(self) -> MockVibe {
        let mock = MockVibe::new(&self.name);

        // Set up expectations
        for (method_name, expectation) in self.expectations {
            let mut expectations = mock.expectations.lock().unwrap();
            expectations
                .entry(method_name)
                .or_insert_with(Vec::new)
                .push(expectation);
        }

        // Set up stubs
        for (method_name, stub) in self.stubs {
            let mut stubs = mock.stubs.lock().unwrap();
            stubs.insert(method_name, stub);
        }

        mock
    }
}

/// fr fr Spy object for recording calls without changing behavior
pub struct SpyVibe {
    name: String,
    call_log: Arc<Mutex<Vec<MethodCall>>>,
}

#[derive(Debug, Clone)]
pub struct MethodCall {
    pub method_name: String,
    pub args: Vec<String>, // Simplified - would be Any in real implementation
    pub timestamp: std::time::Instant,
}

impl SpyVibe {
    /// fr fr Create a new spy object
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            call_log: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// fr fr Record a method call
    pub fn record_call(&self, method_name: &str, args: Vec<&str>) {
        let mut log = self.call_log.lock().unwrap();
        log.push(MethodCall {
            method_name: method_name.to_string(),
            args: args.iter().map(|s| s.to_string()).collect(),
            timestamp: std::time::Instant::now(),
        });
    }

    /// fr fr Get all recorded calls
    pub fn get_calls(&self) -> Vec<MethodCall> {
        let log = self.call_log.lock().unwrap();
        log.clone()
    }

    /// fr fr Get calls for a specific method
    pub fn get_calls_for(&self, method_name: &str) -> Vec<MethodCall> {
        let log = self.call_log.lock().unwrap();
        log.iter()
            .filter(|call| call.method_name == method_name)
            .cloned()
            .collect()
    }

    /// fr fr Verify a method was called
    pub fn verify_called(&self, t: &mut VibeTest, method_name: &str) {
        let calls = self.get_calls_for(method_name);
        if calls.is_empty() {
            t.fail_vibe(&format!("Spy '{}' expected method '{}' to be called, but it wasn't", self.name, method_name));
        }
    }

    /// fr fr Verify a method was called with specific arguments
    pub fn verify_called_with(&self, t: &mut VibeTest, method_name: &str, expected_args: Vec<&str>) {
        let calls = self.get_calls_for(method_name);
        let expected: Vec<String> = expected_args.iter().map(|s| s.to_string()).collect();
        
        let found = calls.iter().any(|call| call.args == expected);
        if !found {
            t.fail_vibe(&format!(
                "Spy '{}' expected method '{}' to be called with args {:?}, but it wasn't",
                self.name, method_name, expected
            ));
        }
    }

    /// fr fr Clear all recorded calls
    pub fn clear(&self) {
        let mut log = self.call_log.lock().unwrap();
        log.clear();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::stdlib::packages::test_vibes::core::VibeTest;

    #[test]
    fn test_mock_creation() {
        let mock = MockVibe::new("test_mock");
        assert_eq!(mock.name(), "test_mock");
        assert_eq!(mock.call_count("some_method"), 0);
    }

    #[test]
    fn test_mock_expectation() {
        let mock = MockVibe::new("test_mock");
        let expectation = mock.expect("some_method");
        
        // Record a call
        mock.record_call("some_method", vec![]);
        
        let mut test = VibeTest::new("mock_test".to_string());
        mock.verify(&mut test);
        
        // Should pass since we expect exactly one call and made one call
        assert!(!test.failed());
    }

    #[test]
    fn test_mock_verification_failure() {
        let mock = MockVibe::new("test_mock");
        let _expectation = mock.expect("some_method");
        
        // Don't record any calls
        
        let mut test = VibeTest::new("mock_test".to_string());
        mock.verify(&mut test);
        
        // Should fail since we expect one call but made none
        assert!(test.failed());
    }

    #[test]
    fn test_expectation_times() {
        let expectation = Expectation::new("test_method");
        let expectation = expectation.times(3);
        
        assert!(expectation.is_satisfied(3));
        assert!(!expectation.is_satisfied(2));
        assert!(!expectation.is_satisfied(4));
    }

    #[test]
    fn test_expectation_at_least() {
        let expectation = Expectation::new("test_method");
        let expectation = expectation.at_least(2);
        
        assert!(!expectation.is_satisfied(1));
        assert!(expectation.is_satisfied(2));
        assert!(expectation.is_satisfied(5));
    }

    #[test]
    fn test_expectation_at_most() {
        let expectation = Expectation::new("test_method");
        let expectation = expectation.at_most(3);
        
        assert!(expectation.is_satisfied(0));
        assert!(expectation.is_satisfied(2));
        assert!(expectation.is_satisfied(3));
        assert!(!expectation.is_satisfied(4));
    }

    #[test]
    fn test_spy_creation() {
        let spy = SpyVibe::new("test_spy");
        assert_eq!(spy.name, "test_spy");
        assert!(spy.get_calls().is_empty());
    }

    #[test]
    fn test_spy_recording() {
        let spy = SpyVibe::new("test_spy");
        spy.record_call("method1", vec!["arg1", "arg2"]);
        spy.record_call("method2", vec!["arg3"]);
        
        let calls = spy.get_calls();
        assert_eq!(calls.len(), 2);
        assert_eq!(calls[0].method_name, "method1");
        assert_eq!(calls[0].args, vec!["arg1", "arg2"]);
        assert_eq!(calls[1].method_name, "method2");
        assert_eq!(calls[1].args, vec!["arg3"]);
    }

    #[test]
    fn test_spy_verification() {
        let spy = SpyVibe::new("test_spy");
        spy.record_call("test_method", vec!["test_arg"]);
        
        let mut test = VibeTest::new("spy_test".to_string());
        spy.verify_called(&mut test, "test_method");
        
        assert!(!test.failed());
    }

    #[test]
    fn test_spy_verification_failure() {
        let spy = SpyVibe::new("test_spy");
        // Don't record any calls
        
        let mut test = VibeTest::new("spy_test".to_string());
        spy.verify_called(&mut test, "test_method");
        
        assert!(test.failed());
    }

    #[test]
    fn test_mock_builder() {
        let expectation = Expectation::new("test_method").times(2);
        let stub = Stub::new("other_method", vec![]);
        
        let mock = MockBuilder::new("built_mock")
            .expect("test_method", expectation)
            .stub("other_method", stub)
            .build();
        
        assert_eq!(mock.name(), "built_mock");
    }
}
