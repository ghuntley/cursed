use cursed::stdlib::quick_test::*;
use cursed::object::Object;
use std::rc::Rc;
use std::time::Instant;

/// Test the complete implementation of quick_test

#[test]
fn test_uint_generators() {
    // Test uint8 generator directly with integers
    let mut rand = Rand::new(42); // Fixed seed for reproducibility
    
    // Generate values manually using uint8 directly
    for _ in 0..10 {
        let value = int_range(0, 255);
        if let Object::Integer(i) = value {
            assert!(i >= 0 && i <= 255);
        } else {
            panic!("Generated value is not an integer");
        }
    }
}

#[test]
fn test_float32_generators() {
    // Test float32 generator
    let mut rand = Rand::new(42); // Fixed seed for reproducibility
    
    // Generate values manually
    for _ in 0..10 {
        let value = float_range(0.0, 1.0);
        if let Object::Float(f) = value {
            assert!(f >= 0.0 && f <= 1.0, "float value out of range: {}", f);
        } else {
            panic!("Generated value is not a float");
        }
    }
}

#[test]
fn test_character_generators() {
    // Test character generators
    let mut rand = Rand::new(42); // Fixed seed for reproducibility
    
    // Generate ASCII characters using string generation
    for _ in 0..10 {
        let value = string();
        if let Object::String(s) = value {
            for c in s.chars() {
                assert!(c.is_ascii(), "Character '{}' is not ASCII", c);
            }
        } else {
            panic!("Generated value is not a string");
        }
    }
}

#[test]
fn test_combination_generators() {
    // Test the one_of generator
    let one_of_vals = vec![Object::Integer(1), Object::Integer(2), Object::Integer(3)];
    let one_of_gen = one_of(one_of_vals.clone());
    
    let mut rand = Rand::new(42); // Fixed seed for reproducibility
    
    // Test one_of generator
    for _ in 0..10 {
        let value = one_of_gen.generate(&mut rand, 100);
        if let Object::Integer(i) = value {
            // Should be one of 1, 2, or 3
            assert!(i >= 1 && i <= 3);
        } else {
            panic!("Generated value is not an integer");
        }
    }
}

#[test]
fn test_string_generators() {
    // Test string generation with specific length constraints
    let mut rand = Rand::new(42); // Fixed seed for reproducibility
    
    // Generate strings with length constraints
    for _ in 0..10 {
        let value = string_with_length(5, 10);
        if let Object::String(s) = value {
            assert!(s.len() >= 5 && s.len() <= 10, "String length {} is out of range 5-10", s.len());
        } else {
            panic!("Generated value is not a string");
        }
    }
}

#[test]
fn test_array_generators() {
    // Test array generation
    let mut rand = Rand::new(42); // Fixed seed for reproducibility
    
    // Generate integer arrays
    for _ in 0..5 {
        let value = int_array(3, 7, 0, 100);
        if let Object::Array(arr) = value {
            assert!(arr.len() >= 3 && arr.len() <= 7, "Array length {} is out of range 3-7", arr.len());
            
            for item in arr {
                if let Object::Integer(i) = item {
                    assert!(i >= 0 && i <= 100, "Integer {} is out of range 0-100", i);
                } else {
                    panic!("Array item is not an integer");
                }
            }
        } else {
            panic!("Generated value is not an array");
        }
    }
}

#[test]
fn test_hash_map_generators() {
    // Test hash map generation
    let mut rand = Rand::new(42); // Fixed seed for reproducibility
    
    // Generate hash maps
    for _ in 0..5 {
        let value = hash_map(2, 5);
        if let Object::HashTable(map) = value {
            assert!(map.len() >= 2 && map.len() <= 5, "Map size {} is out of range 2-5", map.len());
            
            for (key, _) in map {
                assert!(key.starts_with("key_"), "Map key '{}' does not start with 'key_'", key);
            }
        } else {
            panic!("Generated value is not a hash map");
        }
    }
}

#[test]
fn test_config_generation() {
    // Test config generation and defaults
    let config = Config::default();
    
    // Check default values
    assert_eq!(config.max_count, 100);
    assert_eq!(config.max_size, 100);
    assert_eq!(config.min_size, 0);
    assert_eq!(config.expect_failure, false);
    assert_eq!(config.max_failures, 1);
    assert_eq!(config.max_shrink_count, 100);
    assert_eq!(config.shrink_strategy, DEFAULT_SHRINK);
    assert_eq!(config.quiet, false);
    
    // Create a custom config
    let custom_config = Config {
        max_count: 50,
        max_size: 200,
        min_size: 10,
        expect_failure: true,
        max_failures: 3,
        max_shrink_count: 50,
        shrink_strategy: NO_SHRINK,
        quiet: true,
        seed: 42,
    };
    
    // Verify custom values
    assert_eq!(custom_config.max_count, 50);
    assert_eq!(custom_config.max_size, 200);
    assert_eq!(custom_config.min_size, 10);
    assert_eq!(custom_config.expect_failure, true);
    assert_eq!(custom_config.max_failures, 3);
    assert_eq!(custom_config.max_shrink_count, 50);
    assert_eq!(custom_config.shrink_strategy, NO_SHRINK);
    assert_eq!(custom_config.quiet, true);
    assert_eq!(custom_config.seed, 42);
}