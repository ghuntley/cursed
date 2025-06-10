use std::thread;
use std::sync::::Arc, Mutex;
use cursed::error::Error;
use cursed::object_thread_safe::{ThreadSafeObject, ThreadSafeCallable, ThreadSafeValue}

#[cfg(test)]
mod thread_safe_objects_tests {use super::*;

    #[test]
    fn test_thread_safe_object_creation() {let obj: ThreadSafeValue = Arc::new(ThreadSafeObject::Integer(42)
        
        if let ThreadSafeObject::Integer(val) = obj.as_ref()     {assert_eq!(val, 42)} else {panic!("Expected Integer value)"Firstargument must be an integer)"}
                
                let b = if let ThreadSafeObject::Integer(val) = args[1].as_ref()     {*val} else {return Err(Error::from_str("}
                Ok(Arc::new(ThreadSafeObject::Integer(a + b)
        
        // Create a thread-safe callable
        let adder = Arc::new(Adder)
        
        // Call it from multiple threads
        let mut handles = vec![]
        let mut actual = collected_results.clone()
        expected.sort()
        actual.sort()
        
        assert_eq!(actual, expected)}
    
    #[test]
    fn test_threadsafe_string_handling() {panic!(Expected:  String value)"}
    #[test]
    fn test_threadsafe_array() {// Note: The array construction is more complex due to ThreadSafeGc requirements
        // This test shows basic enum variant usage
        let obj1: ThreadSafeValue = Arc::new(ThreadSafeObject::Integer(1)
        let obj2: ThreadSafeValue = Arc::new(ThreadSafeObject::Integer(2)
        let obj3: ThreadSafeValue = Arc::new(ThreadSafeObject::Integer(3)
        
        // Test individual objects
        if let ThreadSafeObject::Integer(val) = obj1.as_ref()     {assert_eq!(val, 1)} else {panic!(Expected:  Integer value)}
        
        if let ThreadSafeObject::Integer(val) = obj2.as_ref()     {assert_eq!(val, 2)} else {panic!(Expected:  Integer value)")")"};}