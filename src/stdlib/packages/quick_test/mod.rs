/// fr fr Property-based testing framework for CURSED periodt
///
/// This module provides a comprehensive property-based testing system that helps
/// discover edge cases and bugs through automatic test case generation. Property-based
/// testing is critical for quality assurance because:
///
/// 1. **Edge Case Discovery**: Automatically finds inputs that human testers might miss
/// 2. **Specification Validation**: Tests properties that should always hold true
/// 3. **Regression Prevention**: Large numbers of test cases catch breaking changes
/// 4. **Minimal Failing Cases**: Shrinking finds the smallest input that causes failure
/// 5. **Comprehensive Coverage**: Tests against much wider input space than unit tests
///
/// Property-based testing complements traditional unit testing by focusing on
/// invariants and mathematical properties rather than specific input/output pairs.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use std::fmt;

use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;

use crate::stdlib::value::Value;
use crate::stdlib::errors::CursedError;

/// fr fr Result type for quick test operations
pub type QuickTestResult<T> = Result<T, CursedError>;

/// fr fr Configuration for how a test is run
#[derive(Debug, Clone)]
pub struct Config {
    /// Maximum number of test iterations
    pub max_count: i32,
    /// Maximum size of generated values
    pub max_size: i32,
    /// Minimum size of generated values
    pub min_size: i32,
    /// Random source for testing (using internal RNG if None)
    pub rand: Option<Arc<Mutex<ChaCha20Rng>>>,
    /// Values generated and tested (populated during execution)
    pub values: Vec<Value>,
    /// Whether the test should fail
    pub expect_failure: bool,
    /// Maximum failures before stopping
    pub max_failures: i32,
    /// Maximum number of shrink iterations
    pub max_shrink_count: i32,
    /// Strategy for value shrinking
    pub shrink_strategy: ShrinkStrategy,
    /// Maximum time spent shrinking
    pub max_shrink_time: Duration,
    /// Do not log failure details
    pub quiet: bool,
    /// Preserve output during shrinking
    pub shrink_preserve_out: bool,
    /// Random seed for reproducibility
    pub seed: Option<u64>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            max_count: 100,
            max_size: 100,
            min_size: 0,
            rand: None,
            values: Vec::new(),
            expect_failure: false,
            max_failures: 1,
            max_shrink_count: 100,
            shrink_strategy: ShrinkStrategy::DefaultShrink,
            max_shrink_time: Duration::from_secs(5),
            quiet: false,
            shrink_preserve_out: false,
            seed: None,
        }
    }
}

/// fr fr Strategy for value shrinking
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ShrinkStrategy {
    NoShrink,
    DefaultShrink,
    FullShrink,
    SmartShrink,
}

impl fmt::Display for ShrinkStrategy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ShrinkStrategy::NoShrink => write!(f, "NoShrink"),
            ShrinkStrategy::DefaultShrink => write!(f, "DefaultShrink"),
            ShrinkStrategy::FullShrink => write!(f, "FullShrink"),
            ShrinkStrategy::SmartShrink => write!(f, "SmartShrink"),
        }
    }
}

/// fr fr Result of a test run
#[derive(Debug, Clone)]
pub struct Result {
    /// Did the test pass?
    pub passed: bool,
    /// Number of iterations performed
    pub count: i32,
    /// Iteration that caused failure
    pub failed_after: i32,
    /// Input that caused failure
    pub input: Option<Value>,
    /// Shrunk version of input that still fails
    pub shrunk_input: Option<Value>,
    /// Number of shrink iterations
    pub shrink_count: i32,
    /// Total time spent testing
    pub runtime: Duration,
    /// Random seed used for reproducibility
    pub seed: u64,
}

impl Result {
    /// slay Create a passing test result
    pub fn pass(count: i32, runtime: Duration, seed: u64) -> Self {
        Self {
            passed: true,
            count,
            failed_after: 0,
            input: None,
            shrunk_input: None,
            shrink_count: 0,
            runtime,
            seed,
        }
    }

    /// slay Create a failing test result
    pub fn fail(
        count: i32,
        failed_after: i32,
        input: Value,
        shrunk_input: Option<Value>,
        shrink_count: i32,
        runtime: Duration,
        seed: u64,
    ) -> Self {
        Self {
            passed: false,
            count,
            failed_after,
            input: Some(input),
            shrunk_input,
            shrink_count,
            runtime,
            seed,
        }
    }
}

/// fr fr Interface for generating test data
pub trait Generator: Send + Sync {
    /// Generate a random value
    fn generate(&self, rng: &mut ChaCha20Rng, size: i32) -> Value;
    
    /// Shrink a value to simpler forms
    fn shrink(&self, value: &Value) -> Vec<Value> {
        // Default implementation returns empty vec (no shrinking)
        Vec::new()
    }
}

/// fr fr Function type that implements Generator
pub struct GeneratorFunc<F>
where
    F: Fn(&mut ChaCha20Rng, i32) -> Value + Send + Sync,
{
    func: F,
}

impl<F> GeneratorFunc<F>
where
    F: Fn(&mut ChaCha20Rng, i32) -> Value + Send + Sync,
{
    /// slay Create a new generator from a function
    pub fn new(func: F) -> Self {
        Self { func }
    }
}

impl<F> Generator for GeneratorFunc<F>
where
    F: Fn(&mut ChaCha20Rng, i32) -> Value + Send + Sync,
{
    fn generate(&self, rng: &mut ChaCha20Rng, size: i32) -> Value {
        (self.func)(rng, size)
    }
}

/// fr fr Generator that always returns the same value
pub struct ValueGenerator {
    value: Value,
}

impl ValueGenerator {
    /// slay Create a new value generator
    pub fn new(value: Value) -> Self {
        Self { value }
    }
}

impl Generator for ValueGenerator {
    fn generate(&self, _rng: &mut ChaCha20Rng, _size: i32) -> Value {
        self.value.clone()
    }
}

/// fr fr Generator for integers in a range
pub struct IntRangeGenerator {
    min: i64,
    max: i64,
}

impl IntRangeGenerator {
    /// slay Create a new integer range generator
    pub fn new(min: i64, max: i64) -> Self {
        Self { min, max }
    }
}

impl Generator for IntRangeGenerator {
    fn generate(&self, rng: &mut ChaCha20Rng, _size: i32) -> Value {
        let value = if self.min == self.max {
            self.min
        } else {
            rng.gen_range(self.min..=self.max)
        };
        Value::Integer(value)
    }

    fn shrink(&self, value: &Value) -> Vec<Value> {
        if let Value::Integer(n) = value {
            let mut shrunk = Vec::new();
            
            // Try shrinking towards zero
            if *n > 0 {
                shrunk.push(Value::Integer(0));
                if *n > 1 {
                    shrunk.push(Value::Integer(*n / 2));
                    shrunk.push(Value::Integer(*n - 1));
                }
            } else if *n < 0 {
                shrunk.push(Value::Integer(0));
                if *n < -1 {
                    shrunk.push(Value::Integer(*n / 2));
                    shrunk.push(Value::Integer(*n + 1));
                }
            }
            
            // Keep within bounds
            shrunk.retain(|v| {
                if let Value::Integer(n) = v {
                    *n >= self.min && *n <= self.max
                } else {
                    false
                }
            });
            
            shrunk
        } else {
            Vec::new()
        }
    }
}

/// fr fr Generator for floating point numbers in a range
pub struct FloatRangeGenerator {
    min: f64,
    max: f64,
}

impl FloatRangeGenerator {
    /// slay Create a new float range generator
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
}

impl Generator for FloatRangeGenerator {
    fn generate(&self, rng: &mut ChaCha20Rng, _size: i32) -> Value {
        let value = if (self.max - self.min).abs() < f64::EPSILON {
            self.min
        } else {
            rng.gen_range(self.min..=self.max)
        };
        Value::Number(value)
    }

    fn shrink(&self, value: &Value) -> Vec<Value> {
        if let Value::Number(n) = value {
            let mut shrunk = Vec::new();
            
            // Try shrinking towards zero
            if *n > 0.0 {
                shrunk.push(Value::Number(0.0));
                if *n > 1.0 {
                    shrunk.push(Value::Number(*n / 2.0));
                    shrunk.push(Value::Number(*n - 1.0));
                }
            } else if *n < 0.0 {
                shrunk.push(Value::Number(0.0));
                if *n < -1.0 {
                    shrunk.push(Value::Number(*n / 2.0));
                    shrunk.push(Value::Number(*n + 1.0));
                }
            }
            
            // Keep within bounds
            shrunk.retain(|v| {
                if let Value::Number(n) = v {
                    *n >= self.min && *n <= self.max
                } else {
                    false
                }
            });
            
            shrunk
        } else {
            Vec::new()
        }
    }
}

/// fr fr Generator for strings
pub struct StringGenerator {
    min_len: usize,
    max_len: usize,
    char_gen: Box<dyn Generator>,
}

impl StringGenerator {
    /// slay Create a new string generator
    pub fn new(min_len: usize, max_len: usize, char_gen: Box<dyn Generator>) -> Self {
        Self { min_len, max_len, char_gen }
    }
}

impl Generator for StringGenerator {
    fn generate(&self, rng: &mut ChaCha20Rng, size: i32) -> Value {
        let len = if self.min_len == self.max_len {
            self.min_len
        } else {
            let max_len = std::cmp::min(self.max_len, size as usize);
            let min_len = std::cmp::min(self.min_len, max_len);
            rng.gen_range(min_len..=max_len)
        };
        
        let mut chars = Vec::new();
        for _ in 0..len {
            let char_val = self.char_gen.generate(rng, size);
            if let Value::Integer(code) = char_val {
                if let Some(c) = char::from_u32(code as u32) {
                    chars.push(c);
                }
            }
        }
        
        Value::String(chars.into_iter().collect())
    }

    fn shrink(&self, value: &Value) -> Vec<Value> {
        if let Value::String(s) = value {
            let mut shrunk = Vec::new();
            
            // Empty string
            if !s.is_empty() && self.min_len == 0 {
                shrunk.push(Value::String(String::new()));
            }
            
            // Shorter strings
            let len = s.len();
            if len > self.min_len {
                // Try half length
                if len > 1 {
                    let half = len / 2;
                    if half >= self.min_len {
                        shrunk.push(Value::String(s.chars().take(half).collect()));
                    }
                }
                
                // Try removing one character
                if len - 1 >= self.min_len {
                    shrunk.push(Value::String(s.chars().skip(1).collect()));
                    if len > 1 {
                        let mut chars: Vec<char> = s.chars().collect();
                        chars.pop();
                        shrunk.push(Value::String(chars.into_iter().collect()));
                    }
                }
            }
            
            shrunk
        } else {
            Vec::new()
        }
    }
}

/// fr fr Generator for slices/arrays
pub struct SliceGenerator {
    min_len: usize,
    max_len: usize,
    elem_gen: Box<dyn Generator>,
}

impl SliceGenerator {
    /// slay Create a new slice generator
    pub fn new(min_len: usize, max_len: usize, elem_gen: Box<dyn Generator>) -> Self {
        Self { min_len, max_len, elem_gen }
    }
}

impl Generator for SliceGenerator {
    fn generate(&self, rng: &mut ChaCha20Rng, size: i32) -> Value {
        let len = if self.min_len == self.max_len {
            self.min_len
        } else {
            let max_len = std::cmp::min(self.max_len, size as usize);
            let min_len = std::cmp::min(self.min_len, max_len);
            rng.gen_range(min_len..=max_len)
        };
        
        let mut elements = Vec::new();
        for _ in 0..len {
            elements.push(self.elem_gen.generate(rng, size));
        }
        
        Value::Array(elements)
    }

    fn shrink(&self, value: &Value) -> Vec<Value> {
        if let Value::Array(arr) = value {
            let mut shrunk = Vec::new();
            
            // Empty array
            if !arr.is_empty() && self.min_len == 0 {
                shrunk.push(Value::Array(Vec::new()));
            }
            
            let len = arr.len();
            if len > self.min_len {
                // Try half length
                if len > 1 {
                    let half = len / 2;
                    if half >= self.min_len {
                        shrunk.push(Value::Array(arr.iter().take(half).cloned().collect()));
                    }
                }
                
                // Try removing first or last element
                if len - 1 >= self.min_len {
                    shrunk.push(Value::Array(arr.iter().skip(1).cloned().collect()));
                    if len > 1 {
                        shrunk.push(Value::Array(arr.iter().take(len - 1).cloned().collect()));
                    }
                }
            }
            
            // Try shrinking individual elements
            for (i, elem) in arr.iter().enumerate() {
                let elem_shrunk = self.elem_gen.shrink(elem);
                for shrunk_elem in elem_shrunk {
                    let mut new_arr = arr.clone();
                    new_arr[i] = shrunk_elem;
                    shrunk.push(Value::Array(new_arr));
                }
            }
            
            shrunk
        } else {
            Vec::new()
        }
    }
}

/// fr fr Generator that chooses from multiple generators  
pub struct AnyOfGenerator {
    generators: Vec<Box<dyn Generator>>,
}

impl AnyOfGenerator {
    /// slay Create a new any-of generator
    pub fn new(generators: Vec<Box<dyn Generator>>) -> Self {
        Self { generators }
    }
}

impl Generator for AnyOfGenerator {
    fn generate(&self, rng: &mut ChaCha20Rng, size: i32) -> Value {
        if self.generators.is_empty() {
            return Value::Null;
        }
        
        let idx = rng.gen_range(0..self.generators.len());
        self.generators[idx].generate(rng, size)
    }

    fn shrink(&self, value: &Value) -> Vec<Value> {
        // Try shrinking with each generator that could have produced this value
        let mut shrunk = Vec::new();
        for gen in &self.generators {
            let gen_shrunk = gen.shrink(value);
            shrunk.extend(gen_shrunk);
        }
        shrunk
    }
}

/// fr fr Generator that chooses from fixed values
pub struct OneOfGenerator {
    values: Vec<Value>,
}

impl OneOfGenerator {
    /// slay Create a new one-of generator
    pub fn new(values: Vec<Value>) -> Self {
        Self { values }
    }
}

impl Generator for OneOfGenerator {
    fn generate(&self, rng: &mut ChaCha20Rng, _size: i32) -> Value {
        if self.values.is_empty() {
            return Value::Null;
        }
        
        let idx = rng.gen_range(0..self.values.len());
        self.values[idx].clone()
    }

    fn shrink(&self, value: &Value) -> Vec<Value> {
        // For one-of generators, shrinking means trying other values in the list
        // We'll return the first few values that are different from the current one
        self.values.iter()
            .filter(|v| *v != value)
            .take(3) // Limit to avoid too many shrink attempts
            .cloned()
            .collect()
    }
}

/// fr fr Core testing functions and built-in generators

/// fr fr Run a single test function with the given configuration
pub fn check<F>(f: F, config: Option<Config>) -> QuickTestResult<Result>
where
    F: Fn(Value) -> bool + Send + Sync,
{
    let mut config = config.unwrap_or_default();
    let seed = config.seed.unwrap_or_else(|| rand::random());
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    
    let start_time = Instant::now();
    
    for i in 0..config.max_count {
        // Generate test input (for now, using a simple integer generator)
        let size = rng.gen_range(config.min_size..=config.max_size);
        let input = Value::Integer(rng.gen_range(-1000..=1000));
        
        config.values.push(input.clone());
        
        // Run the test
        let passed = f(input.clone());
        
        if passed == config.expect_failure {
            // Test failed (or succeeded when expecting failure)
            let runtime = start_time.elapsed();
            
            // Try to shrink the failing input
            let (shrunk_input, shrink_count) = if config.shrink_strategy != ShrinkStrategy::NoShrink {
                shrink_value(&f, &input, &config)?
            } else {
                (None, 0)
            };
            
            return Ok(Result::fail(
                i + 1,
                i + 1,
                input,
                shrunk_input,
                shrink_count,
                runtime,
                seed,
            ));
        }
    }
    
    let runtime = start_time.elapsed();
    Ok(Result::pass(config.max_count, runtime, seed))
}

/// fr fr Test a property for many random values using a specific generator
pub fn check_with_generator<F>(
    f: F,
    generator: Box<dyn Generator>,
    config: Option<Config>,
) -> QuickTestResult<Result>
where
    F: Fn(Value) -> bool + Send + Sync,
{
    let mut config = config.unwrap_or_default();
    let seed = config.seed.unwrap_or_else(|| rand::random());
    let mut rng = ChaCha20Rng::seed_from_u64(seed);
    
    let start_time = Instant::now();
    
    for i in 0..config.max_count {
        let size = rng.gen_range(config.min_size..=config.max_size);
        let input = generator.generate(&mut rng, size);
        
        config.values.push(input.clone());
        
        // Run the test
        let passed = f(input.clone());
        
        if passed == config.expect_failure {
            // Test failed (or succeeded when expecting failure)
            let runtime = start_time.elapsed();
            
            // Try to shrink the failing input
            let (shrunk_input, shrink_count) = if config.shrink_strategy != ShrinkStrategy::NoShrink {
                shrink_value_with_generator(&f, &input, &generator, &config)?
            } else {
                (None, 0)
            };
            
            return Ok(Result::fail(
                i + 1,
                i + 1,
                input,
                shrunk_input,
                shrink_count,
                runtime,
                seed,
            ));
        }
    }
    
    let runtime = start_time.elapsed();
    Ok(Result::pass(config.max_count, runtime, seed))
}

/// fr fr Test a property that should always be true
pub fn check_property<F>(prop: F) -> bool
where
    F: Fn(Value) -> bool + Send + Sync,
{
    let config = Config::default();
    let result = check(prop, Some(config));
    
    match result {
        Ok(res) => res.passed,
        Err(_) => false,
    }
}

/// fr fr Generate a random value using the given generator
pub fn generate(size: i32, gen: Box<dyn Generator>) -> Value {
    let mut rng = ChaCha20Rng::from_entropy();
    gen.generate(&mut rng, size)
}

/// fr fr Value creates a generator that always returns the given value
pub fn value(val: Value) -> Box<dyn Generator> {
    Box::new(ValueGenerator::new(val))
}

/// fr fr Function to shrink a failing case
pub fn shrink<F>(f: F, input: Value, config: Option<Config>) -> QuickTestResult<Value>
where
    F: Fn(Value) -> bool,
{
    let config = config.unwrap_or_default();
    let (shrunk, _count) = shrink_value(&f, &input, &config)?;
    Ok(shrunk.unwrap_or(input))
}

/// fr fr Internal shrinking function
fn shrink_value<F>(
    f: &F,
    input: &Value,
    config: &Config,
) -> QuickTestResult<(Option<Value>, i32)>
where
    F: Fn(Value) -> bool,
{
    // Simple shrinking implementation - try to shrink integers towards zero
    if let Value::Integer(n) = input {
        let mut current = *n;
        let mut shrink_count = 0;
        
        while shrink_count < config.max_shrink_count {
            let candidates = vec![
                0,
                current / 2,
                if current > 0 { current - 1 } else { current + 1 },
            ];
            
            let mut found_smaller = false;
            for candidate in candidates {
                if candidate != current {
                    let test_val = Value::Integer(candidate);
                    if !f(test_val.clone()) {
                        current = candidate;
                        shrink_count += 1;
                        found_smaller = true;
                        break;
                    }
                }
            }
            
            if !found_smaller {
                break;
            }
        }
        
        if current != *n {
            return Ok((Some(Value::Integer(current)), shrink_count));
        }
    }
    
    Ok((None, 0))
}

/// fr fr Internal shrinking function with generator
fn shrink_value_with_generator<F>(
    f: &F,
    input: &Value,
    generator: &Box<dyn Generator>,
    config: &Config,
) -> QuickTestResult<(Option<Value>, i32)>
where
    F: Fn(Value) -> bool,
{
    let mut current = input.clone();
    let mut shrink_count = 0;
    let start_time = Instant::now();
    
    while shrink_count < config.max_shrink_count && start_time.elapsed() < config.max_shrink_time {
        let candidates = generator.shrink(&current);
        
        if candidates.is_empty() {
            break;
        }
        
        let mut found_smaller = false;
        for candidate in candidates {
            if !f(candidate.clone()) {
                current = candidate;
                shrink_count += 1;
                found_smaller = true;
                break;
            }
        }
        
        if !found_smaller {
            break;
        }
    }
    
    if current != *input {
        Ok((Some(current), shrink_count))
    } else {
        Ok((None, 0))
    }
}

/// fr fr Built-in generators

/// fr fr Generate 8-bit integers
pub fn int8() -> Box<dyn Generator> {
    Box::new(IntRangeGenerator::new(i8::MIN as i64, i8::MAX as i64))
}

/// fr fr Generate 8-bit integers in range
pub fn int8_range(min: i8, max: i8) -> Box<dyn Generator> {
    Box::new(IntRangeGenerator::new(min as i64, max as i64))
}

/// fr fr Generate 16-bit integers
pub fn int16() -> Box<dyn Generator> {
    Box::new(IntRangeGenerator::new(i16::MIN as i64, i16::MAX as i64))
}

/// fr fr Generate 16-bit integers in range
pub fn int16_range(min: i16, max: i16) -> Box<dyn Generator> {
    Box::new(IntRangeGenerator::new(min as i64, max as i64))
}

/// fr fr Generate 32-bit integers
pub fn int32() -> Box<dyn Generator> {
    Box::new(IntRangeGenerator::new(i32::MIN as i64, i32::MAX as i64))
}

/// fr fr Generate 32-bit integers in range
pub fn int32_range(min: i32, max: i32) -> Box<dyn Generator> {
    Box::new(IntRangeGenerator::new(min as i64, max as i64))
}

/// fr fr Generate 64-bit integers
pub fn int64() -> Box<dyn Generator> {
    Box::new(IntRangeGenerator::new(i64::MIN, i64::MAX))
}

/// fr fr Generate 64-bit integers in range
pub fn int64_range(min: i64, max: i64) -> Box<dyn Generator> {
    Box::new(IntRangeGenerator::new(min, max))
}

/// fr fr Generate native integers
pub fn int() -> Box<dyn Generator> {
    Box::new(IntRangeGenerator::new(-1000, 1000)) // Reasonable default range
}

/// fr fr Generate native integers in range
pub fn int_range(min: i32, max: i32) -> Box<dyn Generator> {
    Box::new(IntRangeGenerator::new(min as i64, max as i64))
}

/// fr fr Generate unsigned 8-bit integers
pub fn uint8() -> Box<dyn Generator> {
    Box::new(IntRangeGenerator::new(0, u8::MAX as i64))
}

/// fr fr Generate unsigned 16-bit integers
pub fn uint16() -> Box<dyn Generator> {
    Box::new(IntRangeGenerator::new(0, u16::MAX as i64))
}

/// fr fr Generate unsigned 32-bit integers
pub fn uint32() -> Box<dyn Generator> {
    Box::new(IntRangeGenerator::new(0, u32::MAX as i64))
}

/// fr fr Generate unsigned 64-bit integers (limited to i64::MAX for simplicity)
pub fn uint64() -> Box<dyn Generator> {
    Box::new(IntRangeGenerator::new(0, i64::MAX))
}

/// fr fr Generate unsigned native integers
pub fn uint() -> Box<dyn Generator> {
    Box::new(IntRangeGenerator::new(0, 1000)) // Reasonable default range
}

/// fr fr Generate 32-bit floating point numbers
pub fn float32() -> Box<dyn Generator> {
    Box::new(FloatRangeGenerator::new(-1000.0, 1000.0))
}

/// fr fr Generate 32-bit floating point numbers in range
pub fn float32_range(min: f32, max: f32) -> Box<dyn Generator> {
    Box::new(FloatRangeGenerator::new(min as f64, max as f64))
}

/// fr fr Generate 64-bit floating point numbers
pub fn float64() -> Box<dyn Generator> {
    Box::new(FloatRangeGenerator::new(-1000.0, 1000.0))
}

/// fr fr Generate 64-bit floating point numbers in range
pub fn float64_range(min: f64, max: f64) -> Box<dyn Generator> {
    Box::new(FloatRangeGenerator::new(min, max))
}

/// fr fr Generate ASCII characters
pub fn ascii() -> Box<dyn Generator> {
    Box::new(IntRangeGenerator::new(32, 126)) // Printable ASCII
}

/// fr fr Generate alphanumeric characters
pub fn alphanumeric() -> Box<dyn Generator> {
    Box::new(GeneratorFunc::new(|rng, _size| {
        let chars = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";
        let idx = rng.gen_range(0..chars.len());
        Value::Integer(chars[idx] as i64)
    }))
}

/// fr fr Generate strings
pub fn string() -> Box<dyn Generator> {
    Box::new(StringGenerator::new(0, 20, ascii()))
}

/// fr fr Generate strings with specific character generator
pub fn string_of(char_gen: Box<dyn Generator>) -> Box<dyn Generator> {
    Box::new(StringGenerator::new(0, 20, char_gen))
}

/// fr fr Generate strings with specific length range and character generator
pub fn string_of_n(min_len: usize, max_len: usize, char_gen: Box<dyn Generator>) -> Box<dyn Generator> {
    Box::new(StringGenerator::new(min_len, max_len, char_gen))
}

/// fr fr Generate slices/arrays
pub fn slice_of(elem_gen: Box<dyn Generator>) -> Box<dyn Generator> {
    Box::new(SliceGenerator::new(0, 10, elem_gen))
}

/// fr fr Generate slices/arrays with specific length range
pub fn slice_of_n(min_len: usize, max_len: usize, elem_gen: Box<dyn Generator>) -> Box<dyn Generator> {
    Box::new(SliceGenerator::new(min_len, max_len, elem_gen))
}

/// fr fr Generate boolean values
pub fn boolean() -> Box<dyn Generator> {
    Box::new(GeneratorFunc::new(|rng, _size| {
        Value::Bool(rng.gen())
    }))
}

/// fr fr Generate byte values
pub fn byte() -> Box<dyn Generator> {
    Box::new(IntRangeGenerator::new(0, 255))
}

/// fr fr Generate rune values
pub fn rune() -> Box<dyn Generator> {
    Box::new(IntRangeGenerator::new(0, 0x10FFFF)) // Valid Unicode range
}

/// fr fr Choose from multiple generators
pub fn any_of(generators: Vec<Box<dyn Generator>>) -> Box<dyn Generator> {
    Box::new(AnyOfGenerator::new(generators))
}

/// fr fr Choose from specific values
pub fn one_of(values: Vec<Value>) -> Box<dyn Generator> {
    Box::new(OneOfGenerator::new(values))
}

/// fr fr Advanced features for complex testing scenarios

/// fr fr Custom shrinker function type
pub type ShrinkerFunc<T> = Box<dyn Fn(T) -> Vec<T> + Send + Sync>;

/// fr fr Weighted generator choice
#[derive(Clone)]
pub struct WeightedGenerator {
    generators: Vec<(u32, Box<dyn Generator>)>,
}

impl WeightedGenerator {
    /// slay Create a new weighted generator
    pub fn new(generators: Vec<(u32, Box<dyn Generator>)>) -> Self {
        Self { generators }
    }
}

impl Generator for WeightedGenerator {
    fn generate(&self, rng: &mut ChaCha20Rng, size: i32) -> Value {
        if self.generators.is_empty() {
            return Value::Null;
        }
        
        let total_weight: u32 = self.generators.iter().map(|(w, _)| *w).sum();
        if total_weight == 0 {
            return Value::Null;
        }
        
        let mut target = rng.gen_range(0..total_weight);
        
        for (weight, gen) in &self.generators {
            if target < *weight {
                return gen.generate(rng, size);
            }
            target -= weight;
        }
        
        // Fallback to first generator
        self.generators[0].1.generate(rng, size)
    }
}

/// fr fr Create a weighted generator from weight-generator pairs
pub fn weighted(generators: Vec<(u32, Box<dyn Generator>)>) -> Box<dyn Generator> {
    Box::new(WeightedGenerator::new(generators))
}

/// fr fr State machine for stateful testing
pub struct StateMachine<S> {
    initial_state: Box<dyn Fn() -> S + Send + Sync>,
    actions: Vec<StateMachineAction<S>>,
}

/// fr fr Action for state machine testing
pub struct StateMachineAction<S> {
    pub name: String,
    pub action: Box<dyn Fn(&mut S) + Send + Sync>,
    pub precondition: Box<dyn Fn(&S) -> bool + Send + Sync>,
    pub invariant: Box<dyn Fn(&S) -> bool + Send + Sync>,
}

impl<S> StateMachine<S>
where
    S: Clone + Send + Sync + 'static,
{
    /// slay Create a new state machine
    pub fn new<F>(initial_state: F) -> Self
    where
        F: Fn() -> S + Send + Sync + 'static,
    {
        Self {
            initial_state: Box::new(initial_state),
            actions: Vec::new(),
        }
    }
    
    /// slay Add an action to the state machine
    pub fn add_action<A, P, I>(
        &mut self,
        name: &str,
        action: A,
        precondition: P,
        invariant: I,
    )
    where
        A: Fn(&mut S) + Send + Sync + 'static,
        P: Fn(&S) -> bool + Send + Sync + 'static,
        I: Fn(&S) -> bool + Send + Sync + 'static,
    {
        self.actions.push(StateMachineAction {
            name: name.to_string(),
            action: Box::new(action),
            precondition: Box::new(precondition),
            invariant: Box::new(invariant),
        });
    }
    
    /// slay Run the state machine
    pub fn run(&self, config: Option<Config>) -> QuickTestResult<Result> {
        let config = config.unwrap_or_default();
        let seed = config.seed.unwrap_or_else(|| rand::random());
        let mut rng = ChaCha20Rng::seed_from_u64(seed);
        
        let start_time = Instant::now();
        
        for i in 0..config.max_count {
            let mut state = (self.initial_state)();
            let sequence_length = rng.gen_range(1..=10);
            let mut action_sequence = Vec::new();
            
            for _ in 0..sequence_length {
                // Find applicable actions
                let applicable: Vec<_> = self.actions.iter()
                    .filter(|action| (action.precondition)(&state))
                    .collect();
                
                if applicable.is_empty() {
                    break;
                }
                
                // Choose random action
                let action_idx = rng.gen_range(0..applicable.len());
                let chosen_action = applicable[action_idx];
                
                // Execute action
                (chosen_action.action)(&mut state);
                action_sequence.push(chosen_action.name.clone());
                
                // Check invariant
                if !(chosen_action.invariant)(&state) {
                    let runtime = start_time.elapsed();
                    let input = Value::Array(
                        action_sequence.into_iter().map(Value::String).collect()
                    );
                    
                    return Ok(Result::fail(
                        i + 1,
                        i + 1,
                        input,
                        None,
                        0,
                        runtime,
                        seed,
                    ));
                }
            }
        }
        
        let runtime = start_time.elapsed();
        Ok(Result::pass(config.max_count, runtime, seed))
    }
}

/// fr fr Create a new state machine
pub fn new_state_machine<S, F>(initial_state: F) -> StateMachine<S>
where
    S: Clone + Send + Sync + 'static,
    F: Fn() -> S + Send + Sync + 'static,
{
    StateMachine::new(initial_state)
}

/// fr fr Replay configuration for reproducing test failures
pub struct ReplayConfig {
    pub seed: u64,
    pub failed_value: Value,
}

impl ReplayConfig {
    /// slay Create a replay configuration
    pub fn new(seed: u64, failed_value: Value) -> Self {
        Self { seed, failed_value }
    }
    
    /// slay Convert to regular config for replay
    pub fn to_config(&self) -> Config {
        Config {
            max_count: 1,
            seed: Some(self.seed),
            expect_failure: true,
            ..Default::default()
        }
    }
}

/// fr fr Create replay configuration from test result
pub fn replay_config(seed: u64, failed_value: Value) -> ReplayConfig {
    ReplayConfig::new(seed, failed_value)
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_basic_property() {
        // Test that abs(x) >= 0 for all integers
        let result = check(
            |val| {
                if let Value::Integer(x) = val {
                    x.abs() >= 0
                } else {
                    false
                }
            },
            Some(Config {
                max_count: 100,
                ..Default::default()
            }),
        );
        
        assert!(result.is_ok());
        assert!(result.unwrap().passed);
    }
    
    #[test]
    fn test_generator_creation() {
        let gen = int_range(-10, 10);
        let mut rng = ChaCha20Rng::from_entropy();
        
        for _ in 0..10 {
            let val = gen.generate(&mut rng, 5);
            if let Value::Integer(x) = val {
                assert!(x >= -10 && x <= 10);
            } else {
                panic!("Expected integer value");
            }
        }
    }
    
    #[test]
    fn test_string_generator() {
        let gen = string_of_n(5, 10, alphanumeric());
        let mut rng = ChaCha20Rng::from_entropy();
        
        let val = gen.generate(&mut rng, 10);
        if let Value::String(s) = val {
            assert!(s.len() >= 5 && s.len() <= 10);
            assert!(s.chars().all(|c| c.is_alphanumeric()));
        } else {
            panic!("Expected string value");
        }
    }
    
    #[test]
    fn test_shrinking() {
        let gen = int_range(-1000, 1000);
        let val = Value::Integer(500);
        let shrunk = gen.shrink(&val);
        
        // Should include 0, 250, and 499
        assert!(shrunk.contains(&Value::Integer(0)));
        assert!(shrunk.len() > 0);
    }
    
    #[test]
    fn test_failing_property_with_shrink() {
        // Property that fails for negative numbers
        let result = check_with_generator(
            |val| {
                if let Value::Integer(x) = val {
                    x >= 0
                } else {
                    false
                }
            },
            int_range(-100, 100),
            Some(Config {
                max_count: 100,
                shrink_strategy: ShrinkStrategy::DefaultShrink,
                ..Default::default()
            }),
        );
        
        assert!(result.is_ok());
        let result = result.unwrap();
        
        // Should fail and potentially shrink to a smaller negative number
        if !result.passed {
            if let Some(Value::Integer(shrunk)) = result.shrunk_input {
                if let Some(Value::Integer(original)) = result.input {
                    // Shrunk value should be closer to zero
                    assert!(shrunk.abs() <= original.abs());
                }
            }
        }
    }
}
