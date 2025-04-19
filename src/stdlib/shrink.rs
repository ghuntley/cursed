//! Shrinking strategies for finding minimal failing test cases

use crate::object::Object;
use crate::error::Error;
use std::rc::Rc;
use std::time::Instant;
use std::collections::HashSet;

#[derive(Debug, Clone)]
/// Constants for shrinking strategies
pub const NO_SHRINK: i64 = 0;
pub const DEFAULT_SHRINK: i64 = 1;
pub const FULL_SHRINK: i64 = 2;
pub const SMART_SHRINK: i64 = 3;

/// Configuration for property-based testing
#[derive(Debug, Clone)]
pub struct Config {
    pub trials: i64,
    pub max_shrink_count: i64,
    pub max_shrink_time: i64,  // in nanoseconds
    pub shrink_strategy: i64,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            trials: 100,
            max_shrink_count: 100,
            max_shrink_time: 1_000_000_000, // 1 second
            shrink_strategy: DEFAULT_SHRINK,
        }
    }
}

/// Shrinker for reducing test cases to minimal failing examples
pub struct Shrinker {
    shrink_fn: Box<dyn Fn(&Object) -> Vec<Object>>,
}

impl Shrinker {
    pub fn new<F>(shrink_fn: F) -> Self
    where
        F: Fn(&Object) -> Vec<Object> + 'static,
    {
        Shrinker {
            shrink_fn: Box::new(shrink_fn),
        }
    }
    
    /// Shrink a failing test case to find a simpler one that still fails
    pub fn shrink<F>(&self, test_fn: F, input: Rc<Object>, config: &Config) -> Option<Rc<Object>>
    where
        F: Fn(&Object) -> bool,
    {
        let start_time = Instant::now();
        let mut best_value = input.clone();
        let mut shrink_count = 0;
        let mut tried = HashSet::new(); // Track already tried values to avoid cycles
        
        // Add the original value to the tried set
        tried.insert(format!("{:?}", best_value));
        
        // Continue shrinking until we can't find a simpler failing case
        let mut found_better = true;
        while found_better && shrink_count < config.max_shrink_count {
            found_better = false;
            let candidates = (self.shrink_fn)(&best_value);
            
            for candidate in candidates {
                // Skip candidates we've already tried
                let candidate_str = format!("{:?}", candidate);
                if tried.contains(&candidate_str) {
                    continue;
                }
                tried.insert(candidate_str);
                
                let candidate_rc = Rc::new(candidate);
                
                // Check if the candidate still fails the test
                if !test_fn(&candidate_rc) {
                    best_value = candidate_rc;
                    found_better = true;
                    shrink_count += 1;
                    break;
                }
                
                // Check if we've hit the time limit for shrinking
                if config.max_shrink_time > 0 && 
                   start_time.elapsed().as_nanos() as i64 > config.max_shrink_time {
                    return Some(best_value);
                }
                
                // Check if we've hit the shrink iteration limit
                if shrink_count >= config.max_shrink_count {
                    return Some(best_value);
                }
            }
        }
        
        if shrink_count > 0 {
            Some(best_value)
        } else {
            None // No shrinking possible
        }
    }
}

/// Default shrinker that tries to reduce various data types
pub fn default_shrinker() -> Shrinker {
    Shrinker::new(|value| {
        let mut candidates = Vec::new();
        
        // Handle different types differently
        match value {
            // For integers, try smaller absolute values
            Object::Integer(n) => {
                if *n != 0 {
                    candidates.push(Object::Integer(0));
                }
                
                if *n > 0 {
                    candidates.push(Object::Integer(n / 2));
                    candidates.push(Object::Integer(n - 1));
                } else if *n < 0 {
                    candidates.push(Object::Integer(n / 2));
                    candidates.push(Object::Integer(n + 1));
                }
            },
            
            // For floating point, try smaller absolute values
            Object::Float(n) => {
                if *n != 0.0 {
                    candidates.push(Object::Float(0.0));
                }
                
                if *n > 0.0 {
                    candidates.push(Object::Float(n / 2.0));
                    candidates.push(Object::Float(n - 1.0));
                } else if *n < 0.0 {
                    candidates.push(Object::Float(n / 2.0));
                    candidates.push(Object::Float(n + 1.0));
                }
            },
            
            // For strings, try removing characters
            Object::String(s) => {
                if !s.is_empty() {
                    // Try empty string
                    candidates.push(Object::String("".to_string()));
                    
                    // Try removing half the string
                    if s.len() > 1 {
                        let half_len = s.len() / 2;
                        candidates.push(Object::String(s[..half_len].to_string()));
                        candidates.push(Object::String(s[half_len..].to_string()));
                    }
                    
                    // Try removing one character from the beginning/end
                    if s.len() > 1 {
                        candidates.push(Object::String(s[1..].to_string()));
                        candidates.push(Object::String(s[..s.len()-1].to_string()));
                    }
                }
            },
            
            // For arrays, try removing elements
            Object::Array(arr) => {
                if !arr.is_empty() {
                    // Try empty array
                    candidates.push(Object::Array(Vec::new()));
                    
                    // Try removing half the elements
                    if arr.len() > 1 {
                        let half_len = arr.len() / 2;
                        let first_half = arr[..half_len].to_vec();
                        let second_half = arr[half_len..].to_vec();
                        candidates.push(Object::Array(first_half));
                        candidates.push(Object::Array(second_half));
                    }
                    
                    // Try removing one element from the beginning/end
                    if arr.len() > 1 {
                        let without_first = arr[1..].to_vec();
                        let without_last = arr[..arr.len()-1].to_vec();
                        candidates.push(Object::Array(without_first));
                        candidates.push(Object::Array(without_last));
                    }
                }
            },
            
            // For hash tables, try removing entries
            Object::HashTable(map) => {
                if !map.is_empty() {
                    // Try empty map
                    candidates.push(Object::HashTable(std::collections::HashMap::new()));
                    
                    // Try removing each key individually
                    for key in map.keys() {
                        let mut new_map = map.clone();
                        new_map.remove(key);
                        candidates.push(Object::HashTable(new_map));
                    }
                }
            },
            
            // For Boolean values, try false (usually simpler)
            Object::Boolean(b) => {
                if *b {
                    candidates.push(Object::Boolean(false));
                }
            },
            
            // For other types, we currently don't have shrinking strategies
            _ => {},
        }
        
        candidates
    })
}

/// Smart shrinker that uses type-specific strategies
pub fn smart_shrinker() -> Shrinker {
    Shrinker::new(|value| {
        // Start with default shrinking strategies
        let mut candidates = default_shrinker().shrink_fn(value);
        
        // Add additional smart shrinking strategies based on type
        match value {
            // For integers, try boundary values
            Object::Integer(n) => {
                // Common boundaries: -1, 0, 1
                candidates.push(Object::Integer(-1));
                candidates.push(Object::Integer(0));
                candidates.push(Object::Integer(1));
                
                // Powers of 2 and their negations
                for i in 0..10 {
                    let pow2 = 1 << i;
                    candidates.push(Object::Integer(pow2));
                    candidates.push(Object::Integer(-pow2));
                }
                
                // Common error offset values: n+1, n-1
                candidates.push(Object::Integer(n + 1));
                candidates.push(Object::Integer(n - 1));
            },
            
            // For strings, try special characters and patterns
            Object::String(s) => {
                // Common special characters
                candidates.push(Object::String(" ".to_string()));
                candidates.push(Object::String("\n".to_string()));
                candidates.push(Object::String("\t".to_string()));
                candidates.push(Object::String("\r".to_string()));
                candidates.push(Object::String("'".to_string()));
                candidates.push(Object::String("\"".to_string()));
                candidates.push(Object::String(",".to_string()));
                candidates.push(Object::String(".".to_string()));
                
                // For larger strings, try removing all but the first and last char
                if s.len() > 2 {
                    let first_char = s.chars().next().unwrap();
                    let last_char = s.chars().last().unwrap();
                    candidates.push(Object::String(format!("{}{}", first_char, last_char))));
                }
                
                // Try removing all whitespace
                let no_whitespace: String = s.chars().filter(|c| !c.is_whitespace()).collect();
                candidates.push(Object::String(no_whitespace));
            },
            
            // More complex strategies for other types...
            _ => {},
        }
        
        candidates
    })
}

/// Shrink a failing test case based on the specified strategy
pub fn shrink<F>(test_fn: F, input: Rc<Object>, config: &Config) -> Option<Rc<Object>>
where
    F: Fn(&Object) -> bool,
{
    match config.shrink_strategy {
        NO_SHRINK => None,
        DEFAULT_SHRINK => default_shrinker().shrink(test_fn, input, config),
        FULL_SHRINK => {
            // For FULL_SHRINK, we use default_shrinker but with more iterations
            let mut full_config = config.clone();
            full_config.max_shrink_count *= 10;
            default_shrinker().shrink(test_fn, input, &full_config)
        },
        SMART_SHRINK => {
            // Use the smart shrinking strategy
            smart_shrinker().shrink(test_fn, input, config)
        },
        _ => None,
    }
}