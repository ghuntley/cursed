/// Mocking framework for the TestVibes testing framework
/// 
/// Provides mock objects with expectations and verification capabilities

// use crate::stdlib::value::Value;
use super::{VibeTest, TestVibesResult, expectation_not_met};
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Mock object for testing with expectations
#[derive(Debug, Clone)]
pub struct MockVibe {
    pub Name: String,
    expectations: Arc<Mutex<HashMap<String, Vec<Expectation>>>>,
    stubs: Arc<Mutex<HashMap<String, Stub>>>,
    call_history: Arc<Mutex<Vec<MethodCall>>>,
}

/// Method call record
#[derive(Debug, Clone)]
struct MethodCall {
    method_name: String,
    args: Vec<Value>,
    timestamp: std::time::Instant,
}

/// Method expectation configuration
#[derive(Debug, Clone)]
pub struct Expectation {
    method_name: String,
    expected_args: Option<Vec<Value>>,
    return_values: Vec<Value>,
    return_fn: Option<Arc<dyn Fn(&[Value]) -> Vec<Value> + Send + Sync>>,
    call_count: CallCount,
    actual_calls: Arc<Mutex<usize>>,
}

/// Method stub configuration
#[derive(Debug, Clone)]
pub struct Stub {
    method_name: String,
    return_values: Vec<Value>,
    call_count: Arc<Mutex<usize>>,
}

/// Call count expectation
#[derive(Debug, Clone)]
enum CallCount {
    Exactly(usize),
    AtLeast(usize),
    AtMost(usize),
    Between(usize, usize),
    Any,
}

impl MockVibe {
    /// Create a new mock object
    pub fn new(name: &str) -> Self {
        Self {
            Name: name.to_string(),
            expectations: Arc::new(Mutex::new(HashMap::new())),
            stubs: Arc::new(Mutex::new(HashMap::new())),
            call_history: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Set up an expectation for a method
    pub fn Expect(&self, method_name: &str) -> Expectation {
        Expectation::new(method_name)
    }

    /// Set up a stub for a method
    pub fn Stub(&self, method_name: &str, return_values: Vec<Value>) -> Stub {
        let stub = Stub {
            method_name: method_name.to_string(),
            return_values,
            call_count: Arc::new(Mutex::new(0)),
        };
        
        self.stubs.lock().unwrap().insert(method_name.to_string(), stub.clone());
        stub
    }

    /// Call a method on the mock (used by generated code)
    pub fn call_method(&self, method_name: &str, args: &[Value]) -> TestVibesResult<Vec<Value>> {
        // Record the call
        self.call_history.lock().unwrap().push(MethodCall {
            method_name: method_name.to_string(),
            args: args.to_vec(),
            timestamp: std::time::Instant::now(),
        });

        // Check if there's an expectation for this method
        let expectations = self.expectations.lock().unwrap();
        if let Some(method_expectations) = expectations.get(method_name) {
            for expectation in method_expectations {
                if expectation.matches_call(args) {
                    *expectation.actual_calls.lock().unwrap() += 1;
                    return Ok(expectation.get_return_values(args));
                }
            }
        }
        drop(expectations); // Release lock

        // Check if there's a stub for this method
        let mut stubs = self.stubs.lock().unwrap();
        if let Some(stub) = stubs.get_mut(method_name) {
            *stub.call_count.lock().unwrap() += 1;
            return Ok(stub.return_values.clone());
        }

        // No expectation or stub found
        Err(expectation_not_met(&format!(
            "Unexpected call to {}.{}({:?})",
            self.Name,
            method_name,
            args
        )).into())
    }

    /// Verify all expectations were met
    pub fn Verify(&self, t: &VibeTest) -> TestVibesResult<()> {
        let expectations = self.expectations.lock().unwrap();
        let mut verification_errors = Vec::new();

        for (method_name, method_expectations) in expectations.iter() {
            for expectation in method_expectations {
                if !expectation.is_satisfied() {
                    let actual_calls = *expectation.actual_calls.lock().unwrap();
                    verification_errors.push(format!(
                        "Expectation for {}.{} not met: expected {:?} calls, got {}",
                        self.Name,
                        method_name,
                        expectation.call_count,
                        actual_calls
                    ));
                }
            }
        }

        if !verification_errors.is_empty() {
            let error_message = verification_errors.join("\n");
            return t.Fatal(&[Value::String(error_message)]);
        }

        t.Log(&[Value::String(format!("All expectations verified for {}", self.Name))])?;
        Ok(())
    }

    /// Get call history for debugging
    pub fn get_call_history(&self) -> Vec<(String, Vec<Value>)> {
        self.call_history
            .lock()
            .unwrap()
            .iter()
            .map(|call| (call.method_name.clone(), call.args.clone()))
            .collect()
    }

    /// Reset all expectations and call history
    pub fn reset(&self) {
        self.expectations.lock().unwrap().clear();
        self.stubs.lock().unwrap().clear();
        self.call_history.lock().unwrap().clear();
    }

    /// Add an expectation (internal method)
    pub(crate) fn add_expectation(&self, expectation: Expectation) {
        let method_name = expectation.method_name.clone();
        self.expectations
            .lock()
            .unwrap()
            .entry(method_name)
            .or_insert_with(Vec::new)
            .push(expectation);
    }
}

impl Expectation {
    /// Create a new expectation
    pub fn new(method_name: &str) -> Self {
        Self {
            method_name: method_name.to_string(),
            expected_args: None,
            return_values: vec![Value::Nil],
            return_fn: None,
            call_count: CallCount::Exactly(1),
            actual_calls: Arc::new(Mutex::new(0)),
        }
    }

    /// Set expected arguments
    pub fn WithArgs(mut self, args: Vec<Value>) -> Self {
        self.expected_args = Some(args);
        self
    }

    /// Set return values
    pub fn Return(mut self, values: Vec<Value>) -> Self {
        self.return_values = values;
        self.return_fn = None;
        self
    }

    /// Set return function
    pub fn ReturnFn<F>(mut self, f: F) -> Self
    where
        F: Fn(&[Value]) -> Vec<Value> + Send + Sync + 'static,
    {
        self.return_fn = Some(Arc::new(f));
        self
    }

    /// Set exact call count expectation
    pub fn Times(mut self, n: usize) -> Self {
        self.call_count = CallCount::Exactly(n);
        self
    }

    /// Set minimum call count expectation
    pub fn AtLeast(mut self, n: usize) -> Self {
        self.call_count = CallCount::AtLeast(n);
        self
    }

    /// Set maximum call count expectation
    pub fn AtMost(mut self, n: usize) -> Self {
        self.call_count = CallCount::AtMost(n);
        self
    }

    /// Set call count range expectation
    pub fn Between(mut self, min: usize, max: usize) -> Self {
        self.call_count = CallCount::Between(min, max);
        self
    }

    /// Allow any number of calls
    pub fn AnyTimes(mut self) -> Self {
        self.call_count = CallCount::Any;
        self
    }

    /// Check if this expectation matches the given call
    fn matches_call(&self, args: &[Value]) -> bool {
        if let Some(ref expected_args) = self.expected_args {
            if args.len() != expected_args.len() {
                return false;
            }
            for (actual, expected) in args.iter().zip(expected_args.iter()) {
                if !values_equal(actual, expected) {
                    return false;
                }
            }
        }
        true
    }

    /// Get return values for this call
    fn get_return_values(&self, args: &[Value]) -> Vec<Value> {
        if let Some(ref return_fn) = self.return_fn {
            return_fn(args)
        } else {
            self.return_values.clone()
        }
    }

    /// Check if this expectation is satisfied
    fn is_satisfied(&self) -> bool {
        let actual_calls = *self.actual_calls.lock().unwrap();
        match self.call_count {
            CallCount::Exactly(n) => actual_calls == n,
            CallCount::AtLeast(n) => actual_calls >= n,
            CallCount::AtMost(n) => actual_calls <= n,
            CallCount::Between(min, max) => actual_calls >= min && actual_calls <= max,
            CallCount::Any => true,
        }
    }
}

impl Stub {
    /// Get the number of times this stub was called
    pub fn call_count(&self) -> usize {
        *self.call_count.lock().unwrap()
    }

    /// Reset the call count
    pub fn reset(&self) {
        *self.call_count.lock().unwrap() = 0;
    }
}

// Mock builder for complex setups
pub struct MockBuilder {
    name: String,
    expectations: Vec<Expectation>,
    stubs: HashMap<String, Vec<Value>>,
}

impl MockBuilder {
    /// Create a new mock builder
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            expectations: Vec::new(),
            stubs: HashMap::new(),
        }
    }

    /// Add an expectation
    pub fn expect(mut self, method_name: &str) -> ExpectationBuilder {
        ExpectationBuilder::new(method_name, self)
    }

    /// Add a stub
    pub fn stub(mut self, method_name: &str, return_values: Vec<Value>) -> Self {
        self.stubs.insert(method_name.to_string(), return_values);
        self
    }

    /// Build the mock object
    pub fn build(self) -> MockVibe {
        let mock = MockVibe::new(&self.name);
        
        // Add expectations
        for expectation in self.expectations {
            mock.add_expectation(expectation);
        }
        
        // Add stubs
        for (method_name, return_values) in self.stubs {
            mock.Stub(&method_name, return_values);
        }
        
        mock
    }

    /// Internal method to add expectation
    fn add_expectation(mut self, expectation: Expectation) -> Self {
        self.expectations.push(expectation);
        self
    }
}

/// Builder for creating expectations
pub struct ExpectationBuilder {
    expectation: Expectation,
    mock_builder: MockBuilder,
}

impl ExpectationBuilder {
    fn new(method_name: &str, mock_builder: MockBuilder) -> Self {
        Self {
            expectation: Expectation::new(method_name),
            mock_builder,
        }
    }

    /// Set expected arguments
    pub fn with_args(mut self, args: Vec<Value>) -> Self {
        self.expectation = self.expectation.WithArgs(args);
        self
    }

    /// Set return values
    pub fn returns(mut self, values: Vec<Value>) -> Self {
        self.expectation = self.expectation.Return(values);
        self
    }

    /// Set call count expectation
    pub fn times(mut self, n: usize) -> Self {
        self.expectation = self.expectation.Times(n);
        self
    }

    /// Finish building this expectation and return to mock builder
    pub fn and(self) -> MockBuilder {
        self.mock_builder.add_expectation(self.expectation)
    }

    /// Build the mock with this expectation
    pub fn build(self) -> MockVibe {
        self.mock_builder.add_expectation(self.expectation).build()
    }
}

// Helper functions

/// Check if two values are equal
fn values_equal(a: &Value, b: &Value) -> bool {
    match (a, b) {
        (Value::Nil, Value::Nil) => true,
        (Value::Bool(a), Value::Bool(b)) => a == b,
        (Value::Int(a), Value::Int(b)) => a == b,
        (Value::Float(a), Value::Float(b)) => (a - b).abs() < f64::EPSILON,
        (Value::String(a), Value::String(b)) => a == b,
        (Value::Array(a), Value::Array(b)) => {
            a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| values_equal(x, y))
        }
        (Value::Object(a), Value::Object(b)) => {
            a.len() == b.len() && a.iter().all(|(k, v)| {
                b.get(k).map_or(false, |v2| values_equal(v, v2))
            })
        }
        _ => false,
    }
}

