#[cfg(test)]
mod thread_safe_objects_tests {
    use std::thread;
    use std::sync::{Arc, Mutex};
    use cursed::error::Error;
    use cursed::object_thread_safe::{ThreadSafeObject, ThreadSafeCallable, ThreadSafeValue};

    #[test]
    fn test_thread_safe_object_creation() {
        let obj = ThreadSafeObject::new(42); // Create from i64
        
        if let ThreadSafeValue::Integer(val) = obj.get() {
            assert_eq!(val, 42);
        } else {
            panic!("Expected Integer value");
        }
    }

    #[test]
    fn test_thread_safe_object_modification() {
        let obj = ThreadSafeObject::new(42);
        
        // Modify the object
        obj.set(100);
        
        // Verify the change
        if let ThreadSafeValue::Integer(val) = obj.get() {
            assert_eq!(val, 100);
        } else {
            panic!("Expected Integer value");
        }
    }

    #[test]
    fn test_thread_safe_object_sharing() {
        let obj = Arc::new(ThreadSafeObject::new(0));
        let mut handles = vec![];
        
        // Spawn 5 threads that each increment the counter by 1
        for _ in 0..5 {
            let obj_clone = Arc::clone(&obj);
            let handle = thread::spawn(move || {
                // Get current value
                let current = if let ThreadSafeValue::Integer(val) = obj_clone.get() {
                    val
                } else {
                    panic!("Expected Integer value");
                };
                
                // Set new value
                obj_clone.set(current + 1);
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Verify final value is 5
        if let ThreadSafeValue::Integer(val) = obj.get() {
            assert_eq!(val, 5);
        } else {
            panic!("Expected Integer value");
        }
    }

    #[test]
    fn test_thread_safe_callable() {
        // Create a callable that adds two integers
        struct Adder;
        
        impl ThreadSafeCallable for Adder {
            fn call(&self, args: Vec<ThreadSafeValue>) -> Result<ThreadSafeValue, Error> {
                if args.len() != 2 {
                    return Err(Error::from_str("Expected exactly 2 arguments"));
                }
                
                let a = if let ThreadSafeValue::Integer(val) = &args[0] {
                    *val
                } else {
                    return Err(Error::from_str("First argument must be an integer"));
                };
                
                let b = if let ThreadSafeValue::Integer(val) = &args[1] {
                    *val
                } else {
                    return Err(Error::from_str("Second argument must be an integer"));
                };
                
                Ok(ThreadSafeValue::Integer(a + b))
            }
        }
        
        // Create a thread-safe callable
        let adder = Arc::new(Adder);
        
        // Call it from multiple threads
        let mut handles = vec![];
        let results = Arc::new(Mutex::new(Vec::new()));
        
        for i in 0..5 {
            let adder_clone = Arc::clone(&adder);
            let results_clone = Arc::clone(&results);
            
            let handle = thread::spawn(move || {
                let args = vec![
                    ThreadSafeValue::Integer(i), 
                    ThreadSafeValue::Integer(10)
                ];
                let result = adder_clone.call(args).unwrap();
                
                // Store the result
                let mut results = results_clone.lock().unwrap();
                results.push(result);
            });
            
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Check results (should be 10, 11, 12, 13, 14)
        let results_guard = results.lock().unwrap();
        let collected_results: Vec<i64> = results_guard
            .iter()
            .map(|val| if let ThreadSafeValue::Integer(v) = val { *v } else { panic!("Expected Integer") })
            .collect();
        
        // Sort because threads might complete in any order
        let mut expected = vec![10, 11, 12, 13, 14];
        let mut actual = collected_results.clone();
        expected.sort();
        actual.sort();
        
        assert_eq!(actual, expected);
    }
    
    #[test]
    fn test_threadsafe_string_handling() {
        let obj = ThreadSafeObject::new("Hello, world!".to_string());
        
        if let ThreadSafeValue::String(val) = obj.get() {
            assert_eq!(val, "Hello, world!");
        } else {
            panic!("Expected String value");
        }
    }
    
    #[test]
    fn test_threadsafe_array() {
        let arr = vec![
            ThreadSafeValue::Integer(1),
            ThreadSafeValue::Integer(2),
            ThreadSafeValue::Integer(3)
        ];
        
        let obj = ThreadSafeObject::new(ThreadSafeValue::Array(arr));
        
        if let ThreadSafeValue::Array(values) = obj.get() {
            assert_eq!(values.len(), 3);
            if let ThreadSafeValue::Integer(val) = &values[0] {
                assert_eq!(*val, 1);
            } else {
                panic!("Expected Integer at index 0");
            }
        } else {
            panic!("Expected Array value");
        }
    }
}