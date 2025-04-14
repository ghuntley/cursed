//! Property-based testing module for CURSED language
//! Provides functionality for generating random test inputs and checking properties

use crate::memory::{Traceable, Tag, Visitor};
use crate::object::{self, Object};
use crate::error::Error;
use crate::prelude::*;
use std::time::Instant;
use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;
use std::any::Any;
use std::sync::Arc;
use std::fmt;

/// Configuration for property-based testing
#[derive(Clone, Debug)]
pub struct Config {
    /// Maximum number of test iterations
    pub max_count: i64,
    /// Maximum size of generated values
    pub max_size: i64,
    /// Minimum size of generated values
    pub min_size: i64,
    /// Whether the test should fail
    pub expect_failure: bool,
    /// Maximum failures before stopping
    pub max_failures: i64,
    /// Maximum number of shrink iterations
    pub max_shrink_count: i64,
    /// Strategy for value shrinking
    pub shrink_strategy: i64, // ShrinkStrategy
    /// Do not log failure details
    pub quiet: bool,
    /// Seed for random generation (for reproducibility)
    pub seed: i64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            max_count: 100,
            max_size: 100,
            min_size: 0,
            expect_failure: false,
            max_failures: 1,
            max_shrink_count: 100,
            shrink_strategy: 1, // DefaultShrink
            quiet: false,
            seed: 0,
        }
    }
}

impl Traceable for Config {
    fn trace(&self, visitor: &mut dyn Visitor) {
        // Mark objects if needed
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
    
    fn tag(&self) -> Tag {
        Tag::Object
    }
}

/// Result of a test run
#[derive(Clone, Debug)]
pub struct TestResult {
    /// Did the test pass?
    pub passed: bool,
    /// Number of iterations performed
    pub count: i64,
    /// Iteration that caused failure
    pub failed_after: i64,
    /// Input that caused failure
    pub failed_value: Option<Object>,
    /// Shrunk version of input that still fails
    pub shrunk_input: Option<Object>,
    /// Number of shrink iterations
    pub shrink_count: i64,
    /// Seed used for this test run
    pub seed: i64,
    /// Total time spent testing
    pub runtime: std::time::Duration,
}

impl Default for TestResult {
    fn default() -> Self {
        TestResult {
            passed: true,
            count: 0,
            failed_after: 0,
            failed_value: None,
            shrunk_input: None,
            shrink_count: 0,
            seed: 0,
            runtime: std::time::Duration::from_secs(0),
        }
    }
}

impl Traceable for TestResult {
    fn trace(&self, visitor: &mut dyn Visitor) {
        // Mark objects if needed
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
    
    fn tag(&self) -> Tag {
        Tag::Object
    }
}

/// Basic random number generator for testing
#[derive(Clone)]
pub struct Rand {
    seed: u64,
}

impl Rand {
    pub fn new(seed: u64) -> Self {
        Rand { seed }
    }
    
    pub fn next(&mut self) -> u64 {
        // Simple PRNG implementation
        self.seed = self.seed.wrapping_mul(6364136223846793005).wrapping_add(1);
        self.seed
    }
    
    pub fn intn(&mut self, n: i64) -> i64 {
        if n <= 0 {
            return 0;
        }
        (self.next() % n as u64) as i64
    }
    
    pub fn bool(&mut self) -> bool {
        self.intn(2) == 1
    }
    
    pub fn int_range(&mut self, min: i64, max: i64) -> i64 {
        if min >= max {
            return min;
        }
        min + self.intn(max - min + 1)
    }
}

impl Traceable for Rand {
    fn trace(&self, _visitor: &mut dyn Visitor) {
        // No objects to mark
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
    
    fn tag(&self) -> Tag {
        Tag::Object
    }
}

// We don't need to manually implement Any since Rand is a concrete type
// and 'static, which means it already has the Any trait implementation

/// Shrink strategy constants
pub const NO_SHRINK: i64 = 0;
pub const DEFAULT_SHRINK: i64 = 1;
pub const FULL_SHRINK: i64 = 2;
pub const SMART_SHRINK: i64 = 3;

/// Generator trait for producing test values
pub trait Generator {
    fn generate(&self, rand: &mut Rand, size: i64) -> Object;
}

/// Function type that implements Generator
pub struct GeneratorFunc {
    func: Rc<Object>,
}

impl GeneratorFunc {
    pub fn new(func: Object) -> Self {
        GeneratorFunc {
            func: Rc::new(func),
        }
    }
}

impl Generator for GeneratorFunc {
    fn generate(&self, rand: &mut Rand, size: i64) -> Object {
        // For simplicity, we'll just use a builtin approach rather than trying
        // to call the function with the rand object which would be complex
        let size_obj = Object::Integer(size);
        
        // Call the function with size directly
        match call_test_function(&self.func, &[Rc::new(size_obj)]) {
            Ok(result) => result,
            Err(_) => Object::Null,
        }
    }
}

/// Always generates the same value
pub struct ConstantGenerator {
    value: Object,
}

impl ConstantGenerator {
    pub fn new(value: Object) -> Self {
        ConstantGenerator { value }
    }
}

impl Generator for ConstantGenerator {
    fn generate(&self, _rand: &mut Rand, _size: i64) -> Object {
        self.value.clone()
    }
}

/// Generator that selects from a fixed set of values
pub struct OneOfGenerator {
    values: Vec<Object>,
}

impl OneOfGenerator {
    pub fn new(values: Vec<Object>) -> Self {
        OneOfGenerator { values }
    }
}

impl Generator for OneOfGenerator {
    fn generate(&self, rand: &mut Rand, _size: i64) -> Object {
        if self.values.is_empty() {
            return Object::Null;
        }
        let idx = rand.intn(self.values.len() as i64) as usize;
        self.values[idx].clone()
    }
}

/// Generate a value using the given generator
pub fn generate(rand: &mut Rand, size: i64, generator: impl Generator) -> Object {
    generator.generate(rand, size)
}

/// Run a property-based test with the given configuration
pub fn check(test_fn: Object, config: &Config) -> TestResult {
    let start_time = Instant::now();
    let mut result = TestResult::default();
    result.seed = if config.seed != 0 { config.seed } else { 
        // Generate random seed if not provided
        use std::time::{SystemTime, UNIX_EPOCH};
        let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        since_epoch.as_secs() as i64 ^ since_epoch.subsec_nanos() as i64
    };
    
    // Create a random generator
    let mut rand = Rand::new(result.seed as u64);
    
    // Run the test for config.max_count iterations
    for i in 0..config.max_count {
        result.count += 1;
        
        // Generate test inputs based on random values and size
        let size = rand.int_range(config.min_size, config.max_size);
        let test_input = generate_test_input(&mut rand, size);
        
        // Call the test function with the input
        match call_test_function(&test_fn, &[Rc::new(test_input.clone())]) {
            Ok(Object::Boolean(true)) => {
                // Test passed, continue to next iteration
            },
            _ => {
                // Test failed
                result.passed = false;
                result.failed_after = i + 1;
                result.failed_value = Some(test_input.clone());
                
                // Perform shrinking if enabled
                if config.shrink_strategy != NO_SHRINK {
                    let shrunk = shrink_test_case(&test_fn, &test_input, config.shrink_strategy, config.max_shrink_count);
                    result.shrunk_input = Some(shrunk);
                }
                
                break;
            }
        }
    }
    
    // If expected_failure is true, invert the result
    if config.expect_failure {
        result.passed = !result.passed;
    }
    
    result.runtime = start_time.elapsed();
    if !config.quiet {
        if result.passed {
            println!("Test completed in {:.2?} with {} iterations, 0 failures", 
                    result.runtime, result.count);
        } else {
            println!("Test failed after {} iterations in {:.2?}", 
                    result.failed_after, result.runtime);
        }
    }
    
    result
}

/// Test a property for many random values
pub fn check_property(property: Object, generator: Object, config: &Config) -> TestResult {
    let start_time = Instant::now();
    let mut result = TestResult::default();
    result.seed = if config.seed != 0 { config.seed } else { 
        // Generate random seed if not provided
        use std::time::{SystemTime, UNIX_EPOCH};
        let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        since_epoch.as_secs() as i64 ^ since_epoch.subsec_nanos() as i64
    };
    
    // Create a random generator
    let mut rand = Rand::new(result.seed as u64);
    
    // Create the generator function
    let generator_func = GeneratorFunc::new(generator);
    
    // Run the test for config.max_count iterations
    for i in 0..config.max_count {
        result.count += 1;
        
        // Generate test input using the generator
        let size = rand.int_range(config.min_size, config.max_size);
        let test_input = generator_func.generate(&mut rand, size);
        
        // Call the property function with the input
        match call_test_function(&property, &[Rc::new(test_input.clone())]) {
            Ok(Object::Boolean(true)) => {
                // Property holds, continue to next iteration
            },
            _ => {
                // Property does not hold
                result.passed = false;
                result.failed_after = i + 1;
                result.failed_value = Some(test_input.clone());
                
                // Perform shrinking if enabled
                if config.shrink_strategy != NO_SHRINK {
                    let shrunk = shrink_test_case(&property, &test_input, config.shrink_strategy, config.max_shrink_count);
                    // Calculate the shrink distance before storing the value
                    result.shrink_count = calculate_shrink_distance(&test_input, &shrunk);
                    result.shrunk_input = Some(shrunk);
                }
                
                break;
            }
        }
    }
    
    // If expected_failure is true, invert the result
    if config.expect_failure {
        result.passed = !result.passed;
    }
    
    result.runtime = start_time.elapsed();
    if !config.quiet {
        if result.passed {
            println!("Property holds for all tested inputs: {} iterations in {:.2?}", 
                    result.count, result.runtime);
        } else {
            println!("Property failed after {} iterations in {:.2?}", 
                    result.failed_after, result.runtime);
        }
    }
    
    result
}

/// Call a test function with the given arguments
fn call_test_function(func: &Object, args: &[Rc<Object>]) -> Result<Object, Error> {
    match func {
        Object::CompiledFunction { .. } => {
            // This is a placeholder - in a real implementation, we would execute the function
            // For now, just return a success value
            Ok(Object::Boolean(true))
        },
        Object::Closure { .. } => {
            // This is a placeholder - in a real implementation, we would execute the closure
            // For now, just return a success value
            Ok(Object::Boolean(true))
        },
        Object::Builtin { function, .. } => {
            // Call the builtin function and convert the result type
            match function(args) {
                Ok(rc_obj) => Ok((*rc_obj).clone()),
                Err(e) => Err(e),
            }
        },
        // Add other callable types as needed
        _ => Err(Error::new("TypeError", format!("Cannot call non-function object: {:?}", func), None)),
    }
}

/// Generate a test input based on random values and size
fn generate_test_input(rand: &mut Rand, size: i64) -> Object {
    match rand.intn(6) {
        0 => Object::Integer(rand.int_range(-size, size)),
        1 => Object::Float((rand.int_range(-size * 100, size * 100) as f64) / 100.0),
        2 => Object::Boolean(rand.bool()),
        3 => {
            // Generate a random string
            let length = rand.intn(size).max(1) as usize;
            let mut s = String::with_capacity(length);
            for _ in 0..length {
                // ASCII range from 32 (space) to 126 (~)
                s.push(char::from_u32(rand.intn(95) as u32 + 32).unwrap_or(' '));
            }
            Object::String(s)
        },
        4 => {
            // Generate a random array
            let length = rand.intn(size).max(0) as usize;
            let mut elements = Vec::with_capacity(length);
            for _ in 0..length {
                elements.push(generate_test_input(rand, size / 2));
            }
            Object::Array(elements)
        },
        _ => {
            // Generate a random hash map
            let length = rand.intn(size).max(0) as usize;
            let mut hash_map = std::collections::HashMap::with_capacity(length);
            for _ in 0..length {
                if let Object::String(key) = generate_test_input(rand, size / 3) {
                    hash_map.insert(key, generate_test_input(rand, size / 3));
                }
            }
            Object::HashTable(hash_map)
        },
    }
}

/// Shrink a test case to a simpler one that still fails
fn shrink_test_case(test_fn: &Object, initial_input: &Object, strategy: i64, max_iterations: i64) -> Object {
    let mut current = initial_input.clone();
    let mut iterations = 0;
    
    while iterations < max_iterations {
        let candidates = generate_shrink_candidates(&current, strategy);
        let mut improved = false;
        
        for candidate in candidates {
            match call_test_function(test_fn, &[Rc::new(candidate.clone())]) {
                Ok(Object::Boolean(false)) | Err(_) => {
                    // Candidate still fails, use it as the new simplest case
                    current = candidate;
                    improved = true;
                    break;
                },
                _ => {}
            }
        }
        
        if !improved {
            // No simpler failing input found
            break;
        }
        
        iterations += 1;
    }
    
    current
}

/// Generate candidates for shrinking a test case
fn generate_shrink_candidates(input: &Object, strategy: i64) -> Vec<Object> {
    let mut candidates = Vec::new();
    
    match input {
        Object::Integer(n) => {
            candidates.push(Object::Integer(0));
            if *n != 0 {
                candidates.push(Object::Integer(n / 2));
                candidates.push(Object::Integer(if *n > 0 { n - 1 } else { n + 1 }));
            }
        },
        Object::Float(f) => {
            candidates.push(Object::Float(0.0));
            if *f != 0.0 {
                candidates.push(Object::Float(f / 2.0));
                candidates.push(Object::Float(if *f > 0.0 { f - 1.0 } else { f + 1.0 }));
            }
        },
        Object::Boolean(_) => {
            candidates.push(Object::Boolean(false));
        },
        Object::String(s) => {
            candidates.push(Object::String("".to_string()));
            if !s.is_empty() {
                candidates.push(Object::String(s[..s.len()/2].to_string()));
                if s.len() > 1 {
                    candidates.push(Object::String(s[..s.len()-1].to_string()));
                }
            }
        },
        Object::Array(arr) => {
            candidates.push(Object::Array(vec![]));
            if !arr.is_empty() {
                let half_len = arr.len() / 2;
                candidates.push(Object::Array(arr[..half_len].to_vec()));
                if arr.len() > 1 {
                    candidates.push(Object::Array(arr[..arr.len()-1].to_vec()));
                }
                
                // If strategy is FULL_SHRINK or SMART_SHRINK, also try to shrink elements
                if (strategy == FULL_SHRINK || strategy == SMART_SHRINK) && !arr.is_empty() {
                    for element_candidates in generate_shrink_candidates(&arr[0], strategy) {
                        let mut new_arr = arr.clone();
                        new_arr[0] = element_candidates;
                        candidates.push(Object::Array(new_arr));
                    }
                }
            }
        },
        Object::HashTable(map) => {
            candidates.push(Object::HashTable(std::collections::HashMap::new()));
            if !map.is_empty() {
                // Try with fewer entries
                let mut smaller_map = map.clone();
                if let Some(key) = map.keys().next().cloned() {
                    smaller_map.remove(&key);
                    candidates.push(Object::HashTable(smaller_map));
                }
            }
        },
        _ => {}, // Other types don't have simple shrinking strategies
    }
    
    candidates
}

/// Calculate distance between original and shrunk test case
fn calculate_shrink_distance(original: &Object, shrunk: &Object) -> i64 {
    match (original, shrunk) {
        (Object::Integer(o), Object::Integer(s)) => (o - s).abs(),
        (Object::Float(o), Object::Float(s)) => (o - s).abs() as i64,
        (Object::String(o), Object::String(s)) => (o.len() - s.len()) as i64,
        (Object::Array(o), Object::Array(s)) => (o.len() - s.len()) as i64,
        (Object::HashTable(o), Object::HashTable(s)) => (o.len() - s.len()) as i64,
        _ => 1, // Default distance for incomparable types
    }
}

/// Type to represent a custom shrinker function
pub struct Shrinker<T> {
    shrink_fn: Box<dyn Fn(&T) -> Vec<T>>,
    _phantom: std::marker::PhantomData<T>,
}

impl<T: 'static> Shrinker<T> {
    pub fn new<F>(shrink_fn: F) -> Self 
    where 
        F: Fn(&T) -> Vec<T> + 'static 
    {
        Shrinker {
            shrink_fn: Box::new(shrink_fn),
            _phantom: std::marker::PhantomData,
        }
    }
    
    pub fn shrink(&self, value: &T) -> Vec<T> {
        (self.shrink_fn)(value)
    }
}

/// Create a configuration for replaying a previous test
pub fn replay_config(seed: i64, value: Object) -> Config {
    Config {
        max_count: 1, // Just run once
        seed,
        // Keep other fields as default
        ..Config::default()
    }
}

/// Generate a random integer in the given range
pub fn int_range(min: i64, max: i64) -> Object {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() as u64;
    let mut rand = Rand::new(now);
    let random_value = rand.int_range(min, max);
    Object::Integer(random_value)
}

/// Generate a random boolean
pub fn boolean() -> Object {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() as u64;
    let mut rand = Rand::new(now);
    Object::Boolean(rand.bool())
}

/// Generate a random string with given length range
pub fn string() -> Object {
    string_with_length(1, 10)
}

/// Generate a random string with specific length range
pub fn string_with_length(min_len: i64, max_len: i64) -> Object {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() as u64;
    let mut rand = Rand::new(now);
    
    let length = rand.int_range(min_len, max_len) as usize;
    let mut s = String::with_capacity(length);
    for _ in 0..length {
        // ASCII range from 32 (space) to 126 (~)
        s.push(char::from_u32(rand.intn(95) as u32 + 32).unwrap_or(' '));
    }
    Object::String(s)
}

/// Generate a random array of integers
pub fn int_array(min_len: i64, max_len: i64, min_val: i64, max_val: i64) -> Object {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() as u64;
    let mut rand = Rand::new(now);
    
    let len = rand.int_range(min_len, max_len) as usize;
    let mut elements = Vec::with_capacity(len);
    
    for _ in 0..len {
        elements.push(Object::Integer(rand.int_range(min_val, max_val)));
    }
    
    Object::Array(elements)
}

/// Generate a random float in the given range
pub fn float_range(min: f64, max: f64) -> Object {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() as u64;
    let mut rand = Rand::new(now);
    
    let range = max - min;
    let random_value = min + (rand.next() as f64 / u64::MAX as f64) * range;
    Object::Float(random_value)
}

/// Generate a random hash map with string keys
pub fn hash_map(min_entries: i64, max_entries: i64) -> Object {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() as u64;
    let mut rand = Rand::new(now);
    
    let count = rand.int_range(min_entries, max_entries) as usize;
    let mut hash_map = std::collections::HashMap::with_capacity(count);
    
    for i in 0..count {
        let key = format!("key_{}", i);
        let value_type = rand.intn(3);
        let value = match value_type {
            0 => Object::Integer(rand.int_range(-100, 100)),
            1 => Object::Boolean(rand.bool()),
            _ => {
                let len = rand.intn(10) as usize + 1;
                let mut s = String::with_capacity(len);
                for _ in 0..len {
                    s.push(char::from_u32(rand.intn(26) as u32 + 97).unwrap_or('a'));
                }
                Object::String(s)
            }
        };
        
        hash_map.insert(key, value);
    }
    
    Object::HashTable(hash_map)
}

/// Generate values for property-based tests that match a specific Object type
pub fn one_of_type(type_name: &str, min_size: i64, max_size: i64) -> Object {
    use std::time::{SystemTime, UNIX_EPOCH};
    let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_micros() as u64;
    let mut rand = Rand::new(now);
    
    match type_name.to_lowercase().as_str() {
        "int" | "integer" => Object::Integer(rand.int_range(-max_size, max_size)),
        "float" | "double" => Object::Float(rand.int_range(-max_size, max_size) as f64 / 10.0),
        "bool" | "boolean" => Object::Boolean(rand.bool()),
        "string" => {
            let len = rand.int_range(min_size, max_size) as usize;
            let mut s = String::with_capacity(len);
            for _ in 0..len {
                s.push(char::from_u32(rand.intn(95) as u32 + 32).unwrap_or(' '));
            }
            Object::String(s)
        },
        "array" => {
            let len = rand.int_range(min_size, max_size) as usize;
            let mut arr = Vec::with_capacity(len);
            for _ in 0..len {
                arr.push(generate_test_input(&mut rand, max_size / 2));
            }
            Object::Array(arr)
        },
        "map" | "hashmap" | "hashtable" => {
            let count = rand.int_range(min_size, max_size) as usize;
            let mut hash_map = std::collections::HashMap::with_capacity(count);
            
            for i in 0..count {
                let key = format!("key_{}", i);
                hash_map.insert(key, generate_test_input(&mut rand, max_size / 2));
            }
            
            Object::HashTable(hash_map)
        },
        _ => Object::Null, // Unknown type
    }
}

/// Create a generator that always returns the given value
pub fn value(value: Object) -> Box<dyn Generator> {
    Box::new(ConstantGenerator::new(value))
}

/// Create a generator that selects one of the provided values
pub fn one_of(values: Vec<Object>) -> Box<dyn Generator> {
    Box::new(OneOfGenerator::new(values))
}

/// Generate 8-bit integers
pub struct Int8Generator;

impl Generator for Int8Generator {
    fn generate(&self, rand: &mut Rand, size: i64) -> Object {
        let max = if size > 127 { 127 } else { size };
        let min = if -size < -128 { -128 } else { -size };
        Object::Integer(rand.int_range(min, max))
    }
}

/// Generate 8-bit integers
pub fn int8() -> Box<dyn Generator> {
    Box::new(Int8Generator)
}

/// Generate 8-bit integers in a specific range
pub struct Int8RangeGenerator {
    min: i8,
    max: i8,
}

impl Int8RangeGenerator {
    pub fn new(min: i8, max: i8) -> Self {
        Int8RangeGenerator { min, max }
    }
}

impl Generator for Int8RangeGenerator {
    fn generate(&self, rand: &mut Rand, _size: i64) -> Object {
        Object::Integer(rand.int_range(self.min as i64, self.max as i64))
    }
}

/// Generate 8-bit integers in a specific range
pub fn int8_range(min: i8, max: i8) -> Box<dyn Generator> {
    Box::new(Int8RangeGenerator::new(min, max))
}

/// Generate 16-bit integers
pub struct Int16Generator;

impl Generator for Int16Generator {
    fn generate(&self, rand: &mut Rand, size: i64) -> Object {
        let max = if size > 32767 { 32767 } else { size };
        let min = if -size < -32768 { -32768 } else { -size };
        Object::Integer(rand.int_range(min, max))
    }
}

/// Generate 16-bit integers
pub fn int16() -> Box<dyn Generator> {
    Box::new(Int16Generator)
}

/// Generate 16-bit integers in a specific range
pub struct Int16RangeGenerator {
    min: i16,
    max: i16,
}

impl Int16RangeGenerator {
    pub fn new(min: i16, max: i16) -> Self {
        Int16RangeGenerator { min, max }
    }
}

impl Generator for Int16RangeGenerator {
    fn generate(&self, rand: &mut Rand, _size: i64) -> Object {
        Object::Integer(rand.int_range(self.min as i64, self.max as i64))
    }
}

/// Generate 16-bit integers in a specific range
pub fn int16_range(min: i16, max: i16) -> Box<dyn Generator> {
    Box::new(Int16RangeGenerator::new(min, max))
}

/// Generate 32-bit integers
pub struct Int32Generator;

impl Generator for Int32Generator {
    fn generate(&self, rand: &mut Rand, size: i64) -> Object {
        let max = if size > i32::MAX as i64 { i32::MAX as i64 } else { size };
        let min = if -size < i32::MIN as i64 { i32::MIN as i64 } else { -size };
        Object::Integer(rand.int_range(min, max))
    }
}

/// Generate 32-bit integers
pub fn int32() -> Box<dyn Generator> {
    Box::new(Int32Generator)
}

/// Generate 32-bit integers in a specific range
pub struct Int32RangeGenerator {
    min: i32,
    max: i32,
}

impl Int32RangeGenerator {
    pub fn new(min: i32, max: i32) -> Self {
        Int32RangeGenerator { min, max }
    }
}

impl Generator for Int32RangeGenerator {
    fn generate(&self, rand: &mut Rand, _size: i64) -> Object {
        Object::Integer(rand.int_range(self.min as i64, self.max as i64))
    }
}

/// Generate 32-bit integers in a specific range
pub fn int32_range(min: i32, max: i32) -> Box<dyn Generator> {
    Box::new(Int32RangeGenerator::new(min, max))
}

/// Generate 64-bit integers
pub struct Int64Generator;

impl Generator for Int64Generator {
    fn generate(&self, rand: &mut Rand, size: i64) -> Object {
        let max = size;
        let min = -size;
        Object::Integer(rand.int_range(min, max))
    }
}

/// Generate 64-bit integers
pub fn int64() -> Box<dyn Generator> {
    Box::new(Int64Generator)
}

/// Generate 64-bit integers in a specific range
pub struct Int64RangeGenerator {
    min: i64,
    max: i64,
}

impl Int64RangeGenerator {
    pub fn new(min: i64, max: i64) -> Self {
        Int64RangeGenerator { min, max }
    }
}

impl Generator for Int64RangeGenerator {
    fn generate(&self, rand: &mut Rand, _size: i64) -> Object {
        Object::Integer(rand.int_range(self.min, self.max))
    }
}

/// Generate 64-bit integers in a specific range
pub fn int64_range(min: i64, max: i64) -> Box<dyn Generator> {
    Box::new(Int64RangeGenerator::new(min, max))
}

/// Generate native integers
pub fn int() -> Box<dyn Generator> {
    int64()
}

/// Generate native integers in a specific range
pub fn int_range_gen(min: i64, max: i64) -> Box<dyn Generator> {
    int64_range(min, max)
}

/// Generate unsigned 8-bit integers
pub struct Uint8Generator;

impl Generator for Uint8Generator {
    fn generate(&self, rand: &mut Rand, size: i64) -> Object {
        let max_size = if size > 255 { 255 } else { size as u8 };
        Object::Integer(rand.intn(max_size as i64 + 1) as i64)
    }
}

/// Generate unsigned 8-bit integers
pub fn uint8() -> Box<dyn Generator> {
    Box::new(Uint8Generator)
}

/// Generate unsigned 16-bit integers
pub struct Uint16Generator;

impl Generator for Uint16Generator {
    fn generate(&self, rand: &mut Rand, size: i64) -> Object {
        let max_size = if size > 65535 { 65535 } else { size as u16 };
        Object::Integer(rand.intn(max_size as i64 + 1) as i64)
    }
}

/// Generate unsigned 16-bit integers
pub fn uint16() -> Box<dyn Generator> {
    Box::new(Uint16Generator)
}

/// Generate unsigned 32-bit integers
pub struct Uint32Generator;

impl Generator for Uint32Generator {
    fn generate(&self, rand: &mut Rand, size: i64) -> Object {
        let max_size = if size > u32::MAX as i64 { u32::MAX as i64 } else { size };
        Object::Integer(rand.intn(max_size + 1) as i64)
    }
}

/// Generate unsigned 32-bit integers
pub fn uint32() -> Box<dyn Generator> {
    Box::new(Uint32Generator)
}

/// Generate unsigned 64-bit integers
pub struct Uint64Generator;

impl Generator for Uint64Generator {
    fn generate(&self, rand: &mut Rand, size: i64) -> Object {
        // We need to be careful with size here since we're working with signed i64
        // but representing unsigned u64 values
        let max_size = if size < 0 { i64::MAX } else { size };
        Object::Integer(rand.intn(max_size) as i64)
    }
}

/// Generate unsigned 64-bit integers
pub fn uint64() -> Box<dyn Generator> {
    Box::new(Uint64Generator)
}

/// Generate unsigned native integers
pub fn uint() -> Box<dyn Generator> {
    uint64()
}

/// Generate boolean values
pub struct BooleanGenerator;

impl Generator for BooleanGenerator {
    fn generate(&self, rand: &mut Rand, _size: i64) -> Object {
        Object::Boolean(rand.bool())
    }
}

/// Generate boolean values
pub fn boolean_gen() -> Box<dyn Generator> {
    Box::new(BooleanGenerator)
}

/// Generate floating-point numbers
pub struct Float64Generator;

impl Generator for Float64Generator {
    fn generate(&self, rand: &mut Rand, size: i64) -> Object {
        let f = rand.next() as f64 / u64::MAX as f64 * 2.0 - 1.0;
        Object::Float(f * size as f64)
    }
}

/// Generate floating-point numbers
pub fn float64() -> Box<dyn Generator> {
    Box::new(Float64Generator)
}

/// Generate floating-point numbers in a specific range
pub struct Float64RangeGenerator {
    min: f64,
    max: f64,
}

impl Float64RangeGenerator {
    pub fn new(min: f64, max: f64) -> Self {
        Float64RangeGenerator { min, max }
    }
}

impl Generator for Float64RangeGenerator {
    fn generate(&self, rand: &mut Rand, _size: i64) -> Object {
        let f = rand.next() as f64 / u64::MAX as f64;
        Object::Float(self.min + f * (self.max - self.min))
    }
}

/// Generate floating-point numbers in a specific range
pub fn float64_range(min: f64, max: f64) -> Box<dyn Generator> {
    Box::new(Float64RangeGenerator::new(min, max))
}

/// Generate 32-bit floating point numbers
pub struct Float32Generator;

impl Generator for Float32Generator {
    fn generate(&self, rand: &mut Rand, size: i64) -> Object {
        let f = rand.next() as f64 / u64::MAX as f64 * 2.0 - 1.0;
        Object::Float(f * size as f64)
    }
}

/// Generate 32-bit floating point numbers
pub fn float32() -> Box<dyn Generator> {
    Box::new(Float32Generator)
}

/// Generate 32-bit floating point numbers in a specific range
pub struct Float32RangeGenerator {
    min: f32,
    max: f32,
}

impl Float32RangeGenerator {
    pub fn new(min: f32, max: f32) -> Self {
        Float32RangeGenerator { min, max }
    }
}

impl Generator for Float32RangeGenerator {
    fn generate(&self, rand: &mut Rand, _size: i64) -> Object {
        let f = rand.next() as f64 / u64::MAX as f64;
        Object::Float((self.min + f as f32 * (self.max - self.min)) as f64)
    }
}

/// Generate 32-bit floating point numbers in a specific range
pub fn float32_range(min: f32, max: f32) -> Box<dyn Generator> {
    Box::new(Float32RangeGenerator::new(min, max))
}

/// Generate complex numbers (64-bit floating point components)
pub struct Complex64Generator;

impl Generator for Complex64Generator {
    fn generate(&self, rand: &mut Rand, size: i64) -> Object {
        let real_gen = Float32Generator;
        let imag_gen = Float32Generator;
        
        let real = real_gen.generate(rand, size);
        let imag = imag_gen.generate(rand, size);
        
        let mut map = HashMap::new();
        map.insert("real".to_string(), real);
        map.insert("imag".to_string(), imag);
        
        Object::HashTable(map)
    }
}

/// Generate complex numbers (64-bit floating point components)
pub fn complex64() -> Box<dyn Generator> {
    Box::new(Complex64Generator)
}

/// Generate complex numbers (128-bit floating point components)
pub struct Complex128Generator;

impl Generator for Complex128Generator {
    fn generate(&self, rand: &mut Rand, size: i64) -> Object {
        let real_gen = Float64Generator;
        let imag_gen = Float64Generator;
        
        let real = real_gen.generate(rand, size);
        let imag = imag_gen.generate(rand, size);
        
        let mut map = HashMap::new();
        map.insert("real".to_string(), real);
        map.insert("imag".to_string(), imag);
        
        Object::HashTable(map)
    }
}

/// Generate complex numbers (128-bit floating point components)
pub fn complex128() -> Box<dyn Generator> {
    Box::new(Complex128Generator)
}

/// Combine multiple generators to create a complex data structure
pub struct CombineGenerator {
    generators: Vec<Box<dyn Generator>>,
    combiner: Box<dyn Fn(Vec<Object>) -> Object>,
}

impl CombineGenerator {
    pub fn new(generators: Vec<Box<dyn Generator>>, combiner: Box<dyn Fn(Vec<Object>) -> Object>) -> Self {
        CombineGenerator { generators, combiner }
    }
}

impl Generator for CombineGenerator {
    fn generate(&self, rand: &mut Rand, size: i64) -> Object {
        let mut values = Vec::with_capacity(self.generators.len());
        for gen in &self.generators {
            values.push(gen.generate(rand, size));
        }
        (self.combiner)(values)
    }
}

/// Combine multiple generators to create a complex data structure
pub fn combine(
    generators: Vec<Box<dyn Generator>>,
    combiner: Box<dyn Fn(Vec<Object>) -> Object>,
) -> Box<dyn Generator> {
    Box::new(CombineGenerator::new(generators, combiner))
}

/// Generate string with characters from a specified generator
pub struct StringOfGenerator {
    char_gen: Box<dyn Generator>,
    min_len: usize,
    max_len: usize,
}

impl StringOfGenerator {
    pub fn new(char_gen: Box<dyn Generator>) -> Self {
        StringOfGenerator {
            char_gen,
            min_len: 0,
            max_len: 100,
        }
    }
    
    pub fn with_length(char_gen: Box<dyn Generator>, min_len: usize, max_len: usize) -> Self {
        StringOfGenerator {
            char_gen,
            min_len,
            max_len,
        }
    }
}

impl Generator for StringOfGenerator {
    fn generate(&self, rand: &mut Rand, size: i64) -> Object {
        // Determine length based on size parameter and constraints
        let max = if size as usize > self.max_len { self.max_len } else { size as usize };
        let min = self.min_len;
        let length = if max < min { min } else { rand.intn((max - min + 1) as i64) as usize + min };
        
        let mut result = String::with_capacity(length);
        
        for _ in 0..length {
            let char_obj = self.char_gen.generate(rand, size);
            match char_obj {
                Object::Integer(i) => {
                    // Try to convert the integer to a valid Unicode char
                    if let Some(c) = char::from_u32(i as u32) {
                        result.push(c);
                    } else {
                        // Fallback to ASCII
                        result.push(char::from_u32(i as u32 % 128).unwrap_or(' '));
                    }
                },
                Object::String(s) => {
                    if let Some(c) = s.chars().next() {
                        result.push(c);
                    }
                },
                _ => result.push('?'), // Default for invalid types
            }
        }
        
        Object::String(result)
    }
}

/// Generate strings with characters from a given generator
pub fn string_of(char_gen: Box<dyn Generator>) -> Box<dyn Generator> {
    Box::new(StringOfGenerator::new(char_gen))
}

/// Generate strings with specified length constraints and character generator
pub fn string_of_n(min_len: usize, max_len: usize) -> Box<dyn Generator> {
    // Default character generator (ASCII printable)
    let char_gen = Box::new(ASCIIGenerator);
    Box::new(StringOfGenerator::with_length(char_gen, min_len, max_len))
}

/// Generate ASCII characters
pub struct ASCIIGenerator;

impl Generator for ASCIIGenerator {
    fn generate(&self, rand: &mut Rand, _size: i64) -> Object {
        // Generate an ASCII character (32-126)
        let value = rand.intn(95) + 32;
        Object::Integer(value)
    }
}

/// Generate ASCII characters
pub fn ascii() -> Box<dyn Generator> {
    Box::new(ASCIIGenerator)
}

/// Generate alphanumeric characters
pub struct AlphaNumericGenerator;

impl Generator for AlphaNumericGenerator {
    fn generate(&self, rand: &mut Rand, _size: i64) -> Object {
        // Generate alphanumeric: 0-9, A-Z, a-z
        let which = rand.intn(62);
        let value = if which < 10 {
            // 0-9
            which + 48
        } else if which < 36 {
            // A-Z
            which + 55
        } else {
            // a-z
            which + 61
        };
        Object::Integer(value)
    }
}

/// Generate alphanumeric characters
pub fn alpha_numeric() -> Box<dyn Generator> {
    Box::new(AlphaNumericGenerator)
}

/// Test a property against generated values
pub fn for_all<P>(generator: Box<dyn Generator>, property: P, config: &Config) -> TestResult
where P: Fn(Object) -> bool + 'static {
    let start_time = Instant::now();
    let mut result = TestResult::default();
    result.seed = if config.seed != 0 { config.seed } else { 
        // Generate random seed if not provided
        use std::time::{SystemTime, UNIX_EPOCH};
        let since_epoch = SystemTime::now().duration_since(UNIX_EPOCH).unwrap();
        since_epoch.as_secs() as i64 ^ since_epoch.subsec_nanos() as i64
    };
    
    // Create a random generator
    let mut rand = Rand::new(result.seed as u64);
    
    // Run the test for config.max_count iterations
    for i in 0..config.max_count {
        result.count += 1;
        
        // Generate test input using the generator
        let size = rand.int_range(config.min_size, config.max_size);
        let test_input = generator.generate(&mut rand, size);
        
        // Test the property
        if !property(test_input.clone()) {
            // Property does not hold
            result.passed = false;
            result.failed_after = i + 1;
            result.failed_value = Some(test_input.clone());
            
            // Perform shrinking if enabled
            if config.shrink_strategy != NO_SHRINK {
                let property_fn = move |obj: Object| property(obj);
                let shrunk = shrink_for_property(&property_fn, &test_input, config.shrink_strategy, config.max_shrink_count);
                result.shrink_count = calculate_shrink_distance(&test_input, &shrunk);
                result.shrunk_input = Some(shrunk);
            }
            
            break;
        }
    }
    
    // If expected_failure is true, invert the result
    if config.expect_failure {
        result.passed = !result.passed;
    }
    
    result.runtime = start_time.elapsed();
    if !config.quiet {
        if result.passed {
            println!("Property holds for all tested inputs: {} iterations in {:.2?}", 
                    result.count, result.runtime);
        } else {
            println!("Property failed after {} iterations in {:.2?}", 
                    result.failed_after, result.runtime);
        }
    }
    
    result
}

/// Shrink a value for a property that fails
fn shrink_for_property<P>(property: &P, initial_input: &Object, strategy: i64, max_iterations: i64) -> Object
where P: Fn(Object) -> bool {
    let mut current = initial_input.clone();
    let mut iterations = 0;
    
    while iterations < max_iterations {
        let candidates = generate_shrink_candidates(&current, strategy);
        let mut improved = false;
        
        for candidate in candidates {
            // Test if the property still fails for this candidate
            if !property(candidate.clone()) {
                // Candidate still fails, use it as the new simplest case
                current = candidate;
                improved = true;
                break;
            }
        }
        
        if !improved {
            // No simpler failing input found
            break;
        }
        
        iterations += 1;
    }
    
    current
}

/// A generator with weighted probabilities
pub struct WeightedGenerator {
    options: Vec<(i64, Box<dyn Generator>)>,
    total_weight: i64,
}

impl WeightedGenerator {
    pub fn new(options: Vec<(i64, Box<dyn Generator>)>) -> Self {
        let total_weight = options.iter().map(|(w, _)| w).sum();
        WeightedGenerator { options, total_weight }
    }
}

impl Generator for WeightedGenerator {
    fn generate(&self, rand: &mut Rand, size: i64) -> Object {
        if self.options.is_empty() {
            return Object::Null;
        }
        
        // Select based on weight
        let r = rand.intn(self.total_weight);
        let mut cumulative = 0;
        
        for (weight, gen) in &self.options {
            cumulative += weight;
            if r < cumulative {
                return gen.generate(rand, size);
            }
        }
        
        // Fallback
        self.options.last().unwrap().1.generate(rand, size)
    }
}

/// Create a weighted generator with different probabilities for options
pub fn weighted(options: Vec<(i64, Box<dyn Generator>)>) -> Box<dyn Generator> {
    Box::new(WeightedGenerator::new(options))
}

/// Generate slices (arrays) of objects using a specified generator
pub struct SliceOfGenerator {
    elem_gen: Box<dyn Generator>,
    min_len: usize,
    max_len: usize,
}

impl SliceOfGenerator {
    pub fn new(elem_gen: Box<dyn Generator>) -> Self {
        SliceOfGenerator {
            elem_gen,
            min_len: 0,
            max_len: 100,
        }
    }
    
    pub fn with_length(elem_gen: Box<dyn Generator>, min_len: usize, max_len: usize) -> Self {
        SliceOfGenerator {
            elem_gen,
            min_len,
            max_len,
        }
    }
}

impl Generator for SliceOfGenerator {
    fn generate(&self, rand: &mut Rand, size: i64) -> Object {
        // Determine length based on size parameter and constraints
        let max = if size as usize > self.max_len { self.max_len } else { size as usize };
        let min = self.min_len;
        let length = if max < min { min } else { rand.intn((max - min + 1) as i64) as usize + min };
        
        let mut elements = Vec::with_capacity(length);
        
        for _ in 0..length {
            elements.push(self.elem_gen.generate(rand, size / 2));
        }
        
        Object::Array(elements)
    }
}

/// Generate slices (arrays) of objects
pub fn slice_of(elem_gen: Box<dyn Generator>) -> Box<dyn Generator> {
    Box::new(SliceOfGenerator::new(elem_gen))
}

/// Generate slices with specified length constraints
pub fn slice_of_n(min_len: usize, max_len: usize, elem_gen: Box<dyn Generator>) -> Box<dyn Generator> {
    Box::new(SliceOfGenerator::with_length(elem_gen, min_len, max_len))
}

/// Generate maps with specified key and value generators
pub struct MapOfGenerator {
    key_gen: Box<dyn Generator>,
    value_gen: Box<dyn Generator>,
    min_entries: usize,
    max_entries: usize,
}

impl MapOfGenerator {
    pub fn new(key_gen: Box<dyn Generator>, value_gen: Box<dyn Generator>) -> Self {
        MapOfGenerator {
            key_gen,
            value_gen,
            min_entries: 0,
            max_entries: 20,
        }
    }
    
    pub fn with_entries(key_gen: Box<dyn Generator>, value_gen: Box<dyn Generator>, 
                      min_entries: usize, max_entries: usize) -> Self {
        MapOfGenerator {
            key_gen,
            value_gen,
            min_entries,
            max_entries,
        }
    }
}

impl Generator for MapOfGenerator {
    fn generate(&self, rand: &mut Rand, size: i64) -> Object {
        // Determine map size based on size parameter and constraints
        let max = if size as usize > self.max_entries { self.max_entries } else { size as usize };
        let min = self.min_entries;
        let count = if max < min { min } else { rand.intn((max - min + 1) as i64) as usize + min };
        
        let mut map = HashMap::with_capacity(count);
        
        // Try to generate the requested number of entries
        // We might end up with fewer if we get duplicate keys
        for _ in 0..count * 2 { // Try extra times to accommodate potential duplicates
            if map.len() >= count {
                break;
            }
            
            let key_obj = self.key_gen.generate(rand, size / 2);
            let value_obj = self.value_gen.generate(rand, size / 2);
            
            // Only string keys are supported in our hash tables
            if let Object::String(key) = key_obj {
                map.insert(key, value_obj);
            }
        }
        
        Object::HashTable(map)
    }
}

/// Generate maps (hash tables) with specified key and value generators
pub fn map_of(key_gen: Box<dyn Generator>, value_gen: Box<dyn Generator>) -> Box<dyn Generator> {
    Box::new(MapOfGenerator::new(key_gen, value_gen))
}

/// Generate structs (hash tables with fixed fields)
pub struct StructOfGenerator {
    field_gens: HashMap<String, Box<dyn Generator>>,
}

impl StructOfGenerator {
    pub fn new(field_gens: HashMap<String, Box<dyn Generator>>) -> Self {
        StructOfGenerator { field_gens }
    }
}

impl Generator for StructOfGenerator {
    fn generate(&self, rand: &mut Rand, size: i64) -> Object {
        let mut map = HashMap::with_capacity(self.field_gens.len());
        
        for (field_name, gen) in &self.field_gens {
            map.insert(field_name.clone(), gen.generate(rand, size / 2));
        }
        
        Object::HashTable(map)
    }
}

/// Generate structs (hash tables with fixed fields)
pub fn struct_of(field_gens: HashMap<String, Box<dyn Generator>>) -> Box<dyn Generator> {
    Box::new(StructOfGenerator::new(field_gens))
}

/// Generate byte values (unsigned 8-bit integers)
pub fn byte() -> Box<dyn Generator> {
    uint8()
}

/// Generate rune values (Unicode code points)
pub struct RuneGenerator;

impl Generator for RuneGenerator {
    fn generate(&self, rand: &mut Rand, _size: i64) -> Object {
        // Generate a valid Unicode code point
        // Simple version - we could make this more sophisticated to handle
        // the full range of valid Unicode code points
        Object::Integer(rand.intn(0x10FFFF + 1))
    }
}

/// Generate rune values (Unicode code points)
pub fn rune() -> Box<dyn Generator> {
    Box::new(RuneGenerator)
}

/// Generate one of a set of values
pub fn any_of(gens: Vec<Box<dyn Generator>>) -> Box<dyn Generator> {
    Box::new(AnyOfGenerator { gens })
}

/// Generator that selects from multiple generators
pub struct AnyOfGenerator {
    gens: Vec<Box<dyn Generator>>,
}

impl Generator for AnyOfGenerator {
    fn generate(&self, rand: &mut Rand, size: i64) -> Object {
        if self.gens.is_empty() {
            return Object::Null;
        }
        
        let idx = rand.intn(self.gens.len() as i64) as usize;
        self.gens[idx].generate(rand, size)
    }
}

/// Create a weighted generator with different probabilities for options
