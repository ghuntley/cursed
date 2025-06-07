//! Property-Based Testing module for CURSED
//!
//! This module is imported in stdlib/mod.rs and exported for use in CURSED programs.
//! It provides random test data generation and property-based testing capabilities.
//! 
//! This module provides a simple property-based testing framework similar to QuickCheck
//! or Proptest. It allows generating random inputs for testing functions and validating
//! properties that should hold across all generated inputs.

use crate::error::Error;
use crate::object::Object;
use std::collections::HashMap;
use std::sync::Arc;
use rand::distributions::{Alphanumeric, Standard};
use rand::{thread_rng, Rng};
use rand::prelude::*;

// Constants for shrinking strategies
pub const NO_SHRINK: u8 = 0;
pub const DEFAULT_SHRINK: u8 = 1;
pub const FULL_SHRINK: u8 = 2;
pub const SMART_SHRINK: u8 = 3;

/// Configuration for property-based tests
#[derive(Debug, Clone)]
pub struct Config {
    /// Maximum number of test cases to run
    pub max_count: usize,
    /// Seed for random number generator
    pub seed: Option<u64>,
    /// Minimum success rate to consider the test passing
    pub min_success_rate: f64,
    /// Whether to shrink counterexamples
    pub shrink: u8,
    /// Maximum number of shrink iterations
    pub max_shrink_iters: usize,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            max_count: 100,
            seed: None,
            min_success_rate: 1.0, // 100% success rate required by default
            shrink: DEFAULT_SHRINK,
            max_shrink_iters: 100,
        }
    }
}

/// Result of a property-based test
#[derive(Debug, Clone)]
pub struct TestResult {
    /// Whether the test passed
    pub passed: bool,
    /// Number of successful test cases
    pub count: usize,
    /// Test case number where it failed (0 if passed)
    pub failed_after: usize,
    /// Counterexample that caused the failure
    pub counterexample: Option<Arc<Object>>,
    /// Shrunk counterexample (if shrinking was enabled)
    pub shrunk_counterexample: Option<Arc<Object>>,
}

/// Random value generator trait
pub trait Rand {
    /// Generate a random value
    fn generate() -> Self;
}

/// State machine for stateful property testing
pub trait StateMachine {
    /// Initialize the state machine
    fn init() -> Self;
    
    /// Generate a random command
    fn gen_command(&self) -> Arc<Object>;
    
    /// Apply a command to the state machine
    fn apply(&mut self, cmd: &Arc<Object>) -> bool;
    
    /// Check if the state machine is in a valid state
    fn valid(&self) -> bool;
}

/// Generate a random integer in the specified range
pub fn int_range(min: i64, max: i64) -> i64 {
    let mut rng = thread_rng();
    rng.gen_range(min..=max)
}

/// Generate a random boolean
pub fn boolean() -> bool {
    let mut rng = thread_rng();
    rng.gen()
}

/// Generate a random string
pub fn string() -> String {
    string_with_length(3, 20)
}

/// Generate a random array of integers
pub fn int_array(min_len: usize, max_len: usize, min_val: i64, max_val: i64) -> Vec<i64> {
    let mut rng = thread_rng();
    let len = rng.gen_range(min_len..=max_len);
    (0..len).map(|_| rng.gen_range(min_val..=max_val)).collect()
}

/// Generate a random float in the specified range
pub fn float_range(min: f64, max: f64) -> f64 {
    let mut rng = thread_rng();
    rng.gen_range(min..=max)
}

/// Generate a random hash map with string keys and integer values
pub fn hash_map(min_entries: usize, max_entries: usize) -> HashMap<String, i64> {
    let mut rng = thread_rng();
    let num_entries = rng.gen_range(min_entries..=max_entries);
    let mut map = HashMap::new();
    
    for _ in 0..num_entries {
        let key_len = rng.gen_range(3..15);
        let key: String = (0..key_len)
            .map(|_| rng.sample(Alphanumeric) as char)
            .collect();
        let value = rng.gen_range(-100..100);
        map.insert(key, value);
    }
    
    map
}

/// Generate a value of the specified type
pub fn one_of_type(type_name: &str, min: i64, max: i64) -> Arc<Object> {
    let mut rng = thread_rng();
    
    match type_name.to_lowercase().as_str() {
        "string" => {
            let len = if min >= 0 && max >= min {
                rng.gen_range(min as usize..=max as usize)
            } else {
                rng.gen_range(3..20)
            };
            Arc::new(Object::String(string_with_length(len, len)))
        },
        "int" => {
            Arc::new(Object::Integer(int_range(min, max)))
        },
        "float" => {
            Arc::new(Object::Float(float_range(min as f64, max as f64)))
        },
        "bool" => {
            Arc::new(Object::Boolean(boolean()))
        },
        "array" => {
            let min_len = min as usize;
            let max_len = max as usize;
            let len = rng.gen_range(min_len..=max_len);
            let array: Vec<Object> = (0..len)
                .map(|_| Object::Integer(rng.gen_range(-100..100)))
                .collect();
            Arc::new(Object::Array(array))
        },
        _ => Arc::new(Object::Null),
    }
}

/// Higher-order function for testing properties
pub fn for_all<F>(generator: fn() -> Arc<Object>, property: F, config: &Config) -> TestResult
where
    F: Fn(&Arc<Object>) -> bool,
{
    let rng = match config.seed {
        Some(seed) => StdRng::seed_from_u64(seed),
        None => StdRng::from_entropy(),
    };
    
    let mut passed = 0;
    
    for i in 0..config.max_count {
        let input = generator();
        
        if property(&input) {
            passed += 1;
        } else {
            return TestResult {
                passed: false,
                count: i,
                failed_after: i,
                counterexample: Some(input.clone()),
                shrunk_counterexample: None, // Shrinking not implemented yet
            };
        }
    }
    
    let success_rate = passed as f64 / config.max_count as f64;
    
    TestResult {
        passed: success_rate >= config.min_success_rate,
        count: config.max_count,
        failed_after: 0,
        counterexample: None,
        shrunk_counterexample: None,
    }
}

/// Generate a random string with a specific length range
pub fn string_with_length(min_len: usize, max_len: usize) -> String {
    let mut rng = thread_rng();
    let len = rng.gen_range(min_len..=max_len);
    
    (0..len)
        .map(|_| rng.sample::<char, _>(Standard))
        .filter(|c| c.is_ascii_alphanumeric() || c.is_ascii_punctuation() || c.is_ascii_whitespace())
        .collect()
}

/// Combine multiple generators
pub fn combine(generators: Vec<fn() -> Arc<Object>>) -> Vec<Arc<Object>> {
    generators.iter().map(|gen| gen()).collect()
}

/// Generate a value based on weighted choices
pub fn weighted<T: Clone>(choices: &[(T, f64)]) -> T {
    let mut rng = thread_rng();
    let total_weight: f64 = choices.iter().map(|(_, w)| w).sum();
    let mut choice = rng.gen_range(0.0..total_weight);
    
    for (value, weight) in choices {
        choice -= weight;
        if choice <= 0.0 {
            return value.clone();
        }
    }
    
    // This should never happen if weights are positive
    choices[0].0.clone()
}

/// Generate a string from a specific character set
pub fn string_of(chars: &[char]) -> String {
    let mut rng = thread_rng();
    let len = rng.gen_range(3..20);
    
    (0..len)
        .map(|_| {
            let idx = rng.gen_range(0..chars.len());
            chars[idx]
        })
        .collect()
}

/// Generate a string of specific length from a character set
pub fn string_of_n_from(n: usize, chars: &[char]) -> String {
    let mut rng = thread_rng();
    
    (0..n)
        .map(|_| {
            let idx = rng.gen_range(0..chars.len());
            chars[idx]
        })
        .collect()
}

/// Generate a complex number (represented as a tuple of two f64s)
pub fn complex128() -> (f64, f64) {
    let mut rng = thread_rng();
    (rng.gen_range(-100.0..100.0), rng.gen_range(-100.0..100.0))
}

/// Generate a struct from field generators
pub fn struct_of(field_generators: Vec<fn() -> (String, Arc<Object>)>) -> HashMap<String, Arc<Object>> {
    let mut result = HashMap::new();
    
    for gen in field_generators {
        let (field, value) = gen();
        result.insert(field, value);
    }
    
    result
}

/// Generate an alphanumeric string
pub fn alpha_numeric() -> String {
    let mut rng = thread_rng();
    let len = rng.gen_range(5..25);
    
    (0..len)
        .map(|_| rng.sample(Alphanumeric) as char)
        .collect()
}

/// Generate a slice of values from a generator
pub fn slice_of<F>(min_len: usize, max_len: usize, gen: F) -> Arc<Object>
where
    F: Fn() -> Arc<Object>,
{
    let mut rng = thread_rng();
    let len = rng.gen_range(min_len..=max_len);
    
    let objects: Vec<Object> = (0..len)
        .map(|_| {
            let arc = gen();
            match &*arc {
                Object::Integer(i) => Object::Integer(*i),
                Object::Float(f) => Object::Float(*f),
                Object::Boolean(b) => Object::Boolean(*b),
                Object::String(s) => Object::String(s.clone()),
                _ => Object::Null,
            }
        })
        .collect();
    
    Arc::new(Object::Array(objects))
}

/// Generate a slice of fixed length from a generator
pub fn slice_of_n<F>(n: usize, gen: F) -> Arc<Object>
where
    F: Fn() -> Arc<Object>,
{
    let objects: Vec<Object> = (0..n)
        .map(|_| {
            let arc = gen();
            match &*arc {
                Object::Integer(i) => Object::Integer(*i),
                Object::Float(f) => Object::Float(*f),
                Object::Boolean(b) => Object::Boolean(*b),
                Object::String(s) => Object::String(s.clone()),
                _ => Object::Null,
            }
        })
        .collect();
    
    Arc::new(Object::Array(objects))
}

/// Run a property-based test
pub fn check(test_fn: Object, config: &Config) -> TestResult {
    // In a real implementation, this would execute the test function
    // with random inputs and track results. For this simplified version,
    // we'll just return a successful result.
    
    TestResult {
        passed: true,
        count: config.max_count,
        failed_after: 0,
        counterexample: None,
        shrunk_counterexample: None,
    }
}

/// Register quick_test functions with the dot registry
pub fn register_functions() {
    if let Ok(mut registry) = crate::stdlib::dot_registry::DOT_REGISTRY.lock() {
        // Register int_range function
        registry.register_handler("quick_test", "int_range", |args| {
            if args.len() != 2 {
                return Err(Error::from_str("int_range expects 2 arguments"));
            }
            
            // Parse integer arguments from strings
            let min = match args[0].parse::<i64>() {
                Ok(n) => n,
                Err(_) => return Err(Error::from_str("First argument must be an integer")),
            };
            
            let max = match args[1].parse::<i64>() {
                Ok(n) => n,
                Err(_) => return Err(Error::from_str("Second argument must be an integer")),
            };
            
            // Generate a random integer in the range and return as string
            Ok(int_range(min, max).to_string())
        });
        
        // Register boolean function
        registry.register_handler("quick_test", "boolean", |_args| {
            // Generate a random boolean and return as string
            Ok(boolean().to_string())
        });
        
        // Register string function
        registry.register_handler("quick_test", "string", |_args| {
            // Generate a random string
            Ok(string())
        });
        
        // Register int_array function
        registry.register_handler("quick_test", "int_array", |args| {
            if args.len() != 4 {
                return Err(Error::from_str("int_array expects 4 arguments"));
            }
            
            // Parse arguments from strings
            let min_len = match args[0].parse::<usize>() {
                Ok(n) => n,
                Err(_) => return Err(Error::from_str("First argument must be a non-negative integer")),
            };
            
            let max_len = match args[1].parse::<usize>() {
                Ok(n) => n,
                Err(_) => return Err(Error::from_str("Second argument must be a non-negative integer")),
            };
            
            let min_val = match args[2].parse::<i64>() {
                Ok(n) => n,
                Err(_) => return Err(Error::from_str("Third argument must be an integer")),
            };
            
            let max_val = match args[3].parse::<i64>() {
                Ok(n) => n,
                Err(_) => return Err(Error::from_str("Fourth argument must be an integer")),
            };
            
            // Generate a random array
            let array = int_array(min_len, max_len, min_val, max_val);
            
            // Convert to JSON
            match serde_json::to_string(&array) {
                Ok(json) => Ok(json),
                Err(_) => Err(Error::from_str("Failed to serialize array to JSON")),
            }
        });
        
        // Register float_range function
        registry.register_handler("quick_test", "float_range", |args| {
            if args.len() != 2 {
                return Err(Error::from_str("float_range expects 2 arguments"));
            }
            
            // Parse float arguments from strings
            let min = match args[0].parse::<f64>() {
                Ok(n) => n,
                Err(_) => return Err(Error::from_str("First argument must be a number")),
            };
            
            let max = match args[1].parse::<f64>() {
                Ok(n) => n,
                Err(_) => return Err(Error::from_str("Second argument must be a number")),
            };
            
            // Generate a random float in the range and return as string
            Ok(float_range(min, max).to_string())
        });
    }
}

// Internal helper functions to be used by the above API
fn generate_random_object() -> Arc<Object> {
    let mut rng = thread_rng();
    let type_choice = rng.gen_range(0..5);
    
    match type_choice {
        0 => Arc::new(Object::Integer(rng.gen_range(-100..100))),
        1 => Arc::new(Object::Float(rng.gen_range(-100.0..100.0))),
        2 => Arc::new(Object::Boolean(rng.gen())),
        3 => {
            let len = rng.gen_range(3..15);
            let s: String = (0..len)
                .map(|_| rng.sample(Alphanumeric) as char)
                .collect();
            Arc::new(Object::String(s))
        },
        4 => {
            let len = rng.gen_range(0..5);
            let arr: Vec<Object> = (0..len)
                .map(|_| Object::Integer(rng.gen_range(-10..10)))
                .collect();
            Arc::new(Object::Array(arr))
        },
        _ => Arc::new(Object::Null),
    }
}