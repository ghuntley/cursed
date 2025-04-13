//! Shrinking strategies for finding minimal failing test cases

use crate::object::ObjectRef;
use crate::prelude::*;
use crate::stdlib::quick_test::{Config, Rand};
use std::time::Instant;

/// Shrinker for reducing test cases to minimal failing examples
pub struct Shrinker {
    /// Function that returns simpler versions of the input value
    shrink_fn: Box<dyn Fn(ObjectRef) -> Vec<ObjectRef>>,
}

impl Shrinker {
    pub fn new<F>(shrink_fn: F) -> Self
    where
        F: Fn(ObjectRef) -> Vec<ObjectRef> + 'static,
    {
        Shrinker {
            shrink_fn: Box::new(shrink_fn),
        }
    }
    
    /// Shrink a failing test case to find a simpler one that still fails
    pub fn shrink(&self, test_fn: &dyn Fn(ObjectRef) -> bool, input: ObjectRef, config: &Config) -> Option<ObjectRef> {
        let start_time = Instant::now();
        let mut best_value = input.clone();
        let mut shrink_count = 0;
        
        // Continue shrinking until we can't find a simpler failing case
        let mut found_better = true;
        while found_better && shrink_count < config.max_shrink_count {
            found_better = false;
            let candidates = (self.shrink_fn)(best_value.clone());
            
            for candidate in candidates {
                // Check if the candidate still fails the test
                if !test_fn(candidate.clone()) {
                    best_value = candidate;
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
        match value.get_type() {
            // For integers, try smaller absolute values
            crate::object::ObjectType::Int => {
                let n = value.as_int().unwrap();
                if n != 0 {
                    candidates.push(ObjectRef::new_int(0));
                }
                
                if n > 0 {
                    candidates.push(ObjectRef::new_int(n / 2));
                    candidates.push(ObjectRef::new_int(n - 1));
                } else if n < 0 {
                    candidates.push(ObjectRef::new_int(n / 2));
                    candidates.push(ObjectRef::new_int(n + 1));
                }
            },
            
            // For strings, try removing characters
            crate::object::ObjectType::String => {
                let s = value.as_string().unwrap();
                if !s.is_empty() {
                    // Try empty string
                    candidates.push(ObjectRef::new_string("".to_string()));
                    
                    // Try removing half the string
                    if s.len() > 1 {
                        let half_len = s.len() / 2;
                        candidates.push(ObjectRef::new_string(s[..half_len].to_string()));
                        candidates.push(ObjectRef::new_string(s[half_len..].to_string()));
                    }
                    
                    // Try removing one character from the beginning/end
                    if s.len() > 1 {
                        candidates.push(ObjectRef::new_string(s[1..].to_string()));
                        candidates.push(ObjectRef::new_string(s[..s.len()-1].to_string()));
                    }
                }
            },
            
            // For arrays, try removing elements
            crate::object::ObjectType::Array => {
                let arr = value.as_array().unwrap();
                if !arr.is_empty() {
                    // Try empty array
                    candidates.push(ObjectRef::new_array(Vec::new()));
                    
                    // Try removing half the elements
                    if arr.len() > 1 {
                        let half_len = arr.len() / 2;
                        candidates.push(ObjectRef::new_array(arr[..half_len].to_vec()));
                        candidates.push(ObjectRef::new_array(arr[half_len..].to_vec()));
                    }
                    
                    // Try removing one element from the beginning/end
                    if arr.len() > 1 {
                        candidates.push(ObjectRef::new_array(arr[1..].to_vec()));
                        candidates.push(ObjectRef::new_array(arr[..arr.len()-1].to_vec()));
                    }
                }
            },
            
            // For other types, we currently don't have shrinking strategies
            _ => {},
        }
        
        candidates
    })
}

/// Shrink a failing test case
pub fn shrink(test_fn: &dyn Fn(ObjectRef) -> bool, input: ObjectRef, config: &Config) -> Option<ObjectRef> {
    match config.shrink_strategy {
        crate::stdlib::quick_test::NO_SHRINK => None,
        crate::stdlib::quick_test::DEFAULT_SHRINK => default_shrinker().shrink(test_fn, input, config),
        crate::stdlib::quick_test::FULL_SHRINK => {
            // For FULL_SHRINK, we use default_shrinker but with more iterations
            let mut full_config = config.clone();
            full_config.max_shrink_count *= 10;
            default_shrinker().shrink(test_fn, input, &full_config)
        },
        crate::stdlib::quick_test::SMART_SHRINK => {
            // TODO: Implement smarter shrinking strategy
            default_shrinker().shrink(test_fn, input, config)
        },
        _ => None,
    }
}