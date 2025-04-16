//! Generator trait for property-based testing
//! 
//! This module defines the Generator trait and related functionality
//! for generating random test values in property-based testing.

use crate::object::Object;
use rand::{Rng, thread_rng};
use std::rc::Rc;

/// Simple random number generator for testing
#[derive(Clone)]
pub struct RandGen {
    seed: u64,
}

impl RandGen {
    pub fn new(seed: u64) -> Self {
        RandGen { seed }
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

/// Generator trait for producing test values
pub trait Generator {
    fn generate(&self, rand: &mut RandGen, size: i64) -> Object;
}

/// Clone a Box<dyn Generator>
pub fn clone_generator<G: Generator + ?Sized>(gen: &Box<G>) -> Box<G> where Box<G>: Clone {
    // This is a simplified implementation just to make the tests compile
    // In real use, this wouldn't work across all generators
    // We'd need to implement proper cloning for each generator type
    gen.clone()
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
    fn generate(&self, _rand: &mut RandGen, _size: i64) -> Object {
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
    fn generate(&self, rand: &mut RandGen, _size: i64) -> Object {
        if self.values.is_empty() {
            return Object::Null;
        }
        let idx = rand.intn(self.values.len() as i64) as usize;
        self.values[idx].clone()
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

/// Function to prepare a generator for use in CURSED
pub fn prepare_generator_object(gen: Box<dyn Generator>) -> Object {
    // This function would convert a Generator into an Object
    // For actual usage, we'd need a way to wrap Generators in Objects
    Object::Null
}

/// Register Generator-related functions
pub fn register_generators() {
    // Register generator functions with the VM
    // This would be implemented to register generators with the VM
}

// Quick test constants
pub const NO_SHRINK: u8 = 0;
pub const DEFAULT_SHRINK: u8 = 1;
pub const FULL_SHRINK: u8 = 2;
pub const SMART_SHRINK: u8 = 3;

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
    pub shrink_strategy: u8,
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
            shrink_strategy: DEFAULT_SHRINK as u8,
            quiet: false,
            seed: 0,
        }
    }
}