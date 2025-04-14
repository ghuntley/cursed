use cursed::stdlib::quick_test::*;
use cursed::object::Object;
use std::rc::Rc;
use std::time::Instant;
use rand::rngs::StdRng;
use rand::SeedableRng;

/// Test the complete implementation of quick_test

#[test]
fn test_uint_generators() {
    // Generate values manually using int_range directly
    for _ in 0..10 {
        let value = Object::Integer(int_range(0, 255));
        if let Object::Integer(i) = value {
            assert!(i >= 0 && i <= 255);
        } else {
            panic!("Generated value is not an integer");
        }
    }
}

#[test]
fn test_float32_generators() {
    // Generate values manually
    for _ in 0..10 {
        let value = Object::Float(float_range(0.0, 1.0));
        if let Object::Float(f) = value {
            assert!(f >= 0.0 && f <= 1.0, "float value out of range: {}", f);
        } else {
            panic!("Generated value is not a float");
        }
    }
}

#[test]
fn test_character_generators() {
    // Generate strings
    for _ in 0..10 {
        let value = Object::String(string());
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
fn test_string_generators() {
    // Generate strings with length constraints
    for _ in 0..10 {
        let value = Object::String(string_with_length(5, 10));
        if let Object::String(s) = value {
            assert!(s.len() >= 5 && s.len() <= 10, "String length {} is out of range 5-10", s.len());
        } else {
            panic!("Generated value is not a string");
        }
    }
}

#[test]
fn test_array_generators() {
    // Generate integer arrays
    for _ in 0..5 {
        let arr = int_array(3, 7, 0, 100);
        assert!(arr.len() >= 3 && arr.len() <= 7, "Array length {} is out of range 3-7", arr.len());
        
        for item in arr {
            assert!(item >= 0 && item <= 100, "Integer {} is out of range 0-100", item);
        }
    }
}

#[test]
fn test_hash_map_generators() {
    // Generate hash maps
    for _ in 0..5 {
        let map = hash_map(2, 5);
        assert!(map.len() >= 2 && map.len() <= 5, "Map size {} is out of range 2-5", map.len());
        
        for (key, _) in map {
            assert!(key.len() > 0, "Map key '{}' is empty", key);
        }
    }
}

#[test]
fn test_config_generation() {
    // Test config generation and defaults
    let config = Config::default();
    
    // Check default values
    assert_eq!(config.max_count, 100);
    assert_eq!(config.seed, None);
    assert_eq!(config.min_success_rate, 1.0);
    assert_eq!(config.shrink, DEFAULT_SHRINK);
    assert_eq!(config.max_shrink_iters, 100);
    
    // Create a custom config
    let custom_config = Config {
        max_count: 50,
        seed: Some(42),
        min_success_rate: 0.9,
        shrink: NO_SHRINK,
        max_shrink_iters: 50,
    };
    
    // Verify custom values
    assert_eq!(custom_config.max_count, 50);
    assert_eq!(custom_config.seed, Some(42));
    assert_eq!(custom_config.min_success_rate, 0.9);
    assert_eq!(custom_config.shrink, NO_SHRINK);
    assert_eq!(custom_config.max_shrink_iters, 50);
}