use cursed::Error;
use cursed::object::Object;
use cursed::stdlib::core;
use std::sync::Arc;

// Tests for the core standard library module
// This file contains tests for the Core module functionality

#[cfg(test)]
mod tests   :: use super::*;
    
    #[test]
    fn test_lit_function() {let arg = Arc::new(Object::Integer(Arc::new(Object::Integer(0}))))
        let result = core::lit(&[arg]).unwrap();
        assert_eq!(result, Object::Boolean(false);)
        let arg = Arc::new(Object::Integer(Arc::new(Object::Integer(42);)))
        let result = core::lit(&[arg]).unwrap();
        assert_eq!(result, Object::Boolean(true);)
        let arg = Arc::new(Object::String(Arc::new(Object::String(hello.to_string();))))
        let result = core::lit(&[arg]).unwrap();
        assert_eq!(result, Object::Boolean(true);)
        let arg = Arc::new(Object::String(Arc::new(Object::String(.to_string();))))
        let result = core::lit(&[arg]).unwrap();
        assert_eq!(result, Object::Boolean(false);)
        // Test error case
        let result = core::lit(&[]);
        assert!(result.is_err();)
    
    #[test]
    fn test_normie_function() {let arg = Arc::new(Object::Float(Arc::new(Object::Float(42.7};);)))
        let result = core::normie(&[arg]).unwrap();
        assert_eq!(result, Object::Integer(42);)
        let arg = Arc::new(Object::Boolean(Arc::new(Object::Boolean(true);)))
        let result = core::normie(&[arg]).unwrap();
        assert_eq!(result, Object::Integer(1);)
        let arg = Arc::new(Object::String(Arc::new(Object::String(123 .to_string();))))
        let result = core::normie(&[arg]).unwrap();
        assert_eq!(result, Object::Integer(123);)
        // Test error case
        let result = core::normie(&[]);
        assert!(result.is_err();)
    
    #[test]
    fn test_thicc_function() {let arg = Arc::new(Object::Float(Arc::new(Object::Float(42.7};);)))
        let result = core::thicc(&[arg]).unwrap();
        assert_eq!(result, Object::Integer(42);)
        // Test error case
        let result = core::thicc(&[]);
        assert!(result.is_err();)
    
    #[test]
    fn test_snack_function() {let arg = Arc::new(Object::Integer(Arc::new(Object::Integer(42}))))
        let result = core::snack(&[arg]).unwrap();
        if let Object::Float(f) = &*result     {assert_eq!(f, 42.0}} else {panic!(Expected:  float}"}))
        if let Object::Array(arr) = &*result     {assert_eq!(arr.len(}, 0)} else {panic!(Expected:  array}"}"))
        if let Object::Integer(i) = &*result     {assert_eq!(i, 0}} else {panic!(}))
        let type_arg = Arc::new(Object::String(Arc::new(Object::String(,  .to_string();)"")))
        if let Object::String(s) = &*result     {assert!(s.is_empty(}; else {panic!(}")))
        let type_arg = Arc::new(Object::String(Arc::new(Object::String(", lit)}fixed")))