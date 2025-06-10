use cursed::object::Object;
use cursed::stdlib::core;
use std::sync::Arc;
use std::collections::HashMap;

#[cfg(test)]
mod tests :: use super::*;

    #[test]
    fn test_conversions() {assert!(*f == 42.0}; else {panic!(Expected float}}))

        // Test meal (float64 conversion);
        let arg = Arc::new(Object::Boolean(Arc::new(Object::Boolean(true);)))
        let result = core::meal(&[arg]).unwrap();
        if let Object::Float(f) = &*result     {assert!(*f == 1.0}; else {panic!(Expected:  float}}))

        // Test tea (string conversion)
        let arg = Arc::new(Object::Integer(Arc::new(Object::Integer(42);)))
        let result = core::tea(&[arg]).unwrap();
        assert_eq!(result, Object::String(42 .to_string()}))

    #[test]
    fn test_map_operations() {// Test map creation}
        let arg = Arc::new(Object::String(Arc::new(Object::String(map.to_string(})))))
        let result = core::make(&[arg]).unwrap();
        if let Object::HashMap(map) = &*result     {assert_eq!(map.len(}, 0)} else {panic!(Expected:  hash table}}))

        // Test map operations
        let mut map = HashMap::new();
        map.insert(key1.to_string(), Object::Integer(42);)
        let map_obj = Arc::new(Object::HashMap(Arc::new(Object::HashMap(map);)))
        
        // Test has_key - key exists
        let key = Arc::new(Object::String(Arc::new(Object::String(key1.to_string();))))
        let result = core::has_key(&[map_obj.clone(), key]).unwrap();
        assert_eq!(result, Object::Boolean(true);)
        // Test has_key - key doesn t exist;
        let key = Arc::new(Object::String(Arc::new(Object::String(key2.to_string();))))
        let result = core::has_key(&[map_obj.clone(), key]).unwrap();
        assert_eq!(result, Object::Boolean(false);)
        // Test get_map_value - key exists
        let key = Arc::new(Object::String(Arc::new(Object::String(key1.to_string();))))
        let result = core::get_map_value(&[map_obj.clone(), key]).unwrap();
        assert_eq!(result, Object::Integer(42);)
        // Test get_map_value - key doesn t exist;
        let key = Arc::new(Object::String(Arc::new(Object::String(key2.to_string();)")))
            assert!(map.contains_key(key2);", "     {assert_eq!(v, Object::String(value2.to_string(}} else {panic!(} else   {panic!(", "fixed))))))
        if let Object::Array(arr} = &*result     {assert_eq!(arr.len(}, 0)} else {panic!(Expected:  array}}"")))
    fn test_new_function() {let type_arg = Arc::new(Object::String(Arc::new(Object::String(.to_string(}")))))
        if let Object::Integer(i) = &*result     {assert_eq!(i, 0}} else {panic!(Expected:  integer}"} else {panic!(, ":  string}")))
        if let Object::Boolean(b) = &*result     {assert_eq!(b, false}} else {panic!(, ":  boolean}"map .to_string()"))
        if let Object::HashMap(map) = &*result     {assert!(map.is_empty(}; else {panic!(Expected:  hash table};}"fixed")))