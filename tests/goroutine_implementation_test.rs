#[cfg(test)]
mod goroutine_implementation_tests {
    use std::sync::{Arc, Mutex};
    use std::thread;
    use std::time::Duration;
    use cursed::core::goroutine;
    use cursed::object::Object;
    use cursed::object_thread_safe::{ThreadSafeObject, ThreadSafeCallable, ThreadSafeValue};
    use cursed::error::Error;
    use cursed::core::thread_safe_goroutine;

    // Helper struct for testing callable objects
    struct TestCallable {
        result: Arc<Mutex<Vec<i64>>>,
        value: i64,
    }

    impl ThreadSafeCallable for TestCallable {
        fn call(&self, _args: Vec<ThreadSafeValue>) -> Result<ThreadSafeValue, Error> {
            // Add value to the shared result vector
            let mut result = self.result.lock().unwrap();
            result.push(self.value);
            
            // Return the value that was added
            Ok(ThreadSafeValue::Integer(self.value))
        }
    }

    #[test]
    fn test_launch_goroutine_fn() {
        // Create a shared vector to store results
        let results = Arc::new(Mutex::new(Vec::new()));
        let results_clone = Arc::clone(&results);
        
        // Launch a goroutine that modifies the results
        let result = goroutine::launch_goroutine_fn(move || {
            // Add a value to the results
            let mut result_guard = results_clone.lock().unwrap();
            result_guard.push(42);
        }).unwrap();
        
        // Verify the result is Null (goroutine launched successfully)
        assert!(matches!(result, Object::Null));
        
        // Sleep to allow goroutine to complete execution
        thread::sleep(Duration::from_millis(100));
        
        // Verify the goroutine executed correctly
        let result_values = results.lock().unwrap();
        assert_eq!(*result_values, vec![42]);
    }

    #[test]
    fn test_simple_goroutine() {
        // Launch a simple goroutine with a function name and argument
        let result = goroutine::launch_simple_goroutine("test_function", 42).unwrap();
        
        // Verify the result is Null (goroutine launched successfully)
        assert!(matches!(result, Object::Null));
        
        // Sleep to allow goroutine to complete execution
        thread::sleep(Duration::from_millis(100));
        
        // With the current implementation we can't verify the function was called
        // This is mainly testing that the API works without errors
    }

    #[test]
    fn test_thread_safe_goroutine() {
        // Create a shared vector to store results
        let results = Arc::new(Mutex::new(Vec::new()));
        let results_clone = Arc::clone(&results);
        
        // Create a callable that adds a value to the results
        let callable = Arc::new(TestCallable {
            result: results_clone,
            value: 42,
        });
        
        // Create thread-safe arguments
        let args = vec![ThreadSafeObject::new(1)];
        
        // Run the goroutine
        thread_safe_goroutine::run_goroutine(callable, args).unwrap();
        
        // Sleep briefly to allow goroutine to complete
        thread::sleep(Duration::from_millis(100));
        
        // Verify the goroutine executed
        let result_values = results.lock().unwrap();
        assert_eq!(*result_values, vec![42]);
    }

    #[test]
    fn test_goroutine_fn() {
        // Create a shared vector to store results
        let results = Arc::new(Mutex::new(Vec::new()));
        let results_clone = Arc::clone(&results);
        
        // Run a function as a goroutine
        thread_safe_goroutine::run_goroutine_fn(move || {
            // Add a value to the results
            let mut result_guard = results_clone.lock().unwrap();
            result_guard.push(42);
        }).unwrap();
        
        // Sleep briefly to allow goroutine to complete
        thread::sleep(Duration::from_millis(100));
        
        // Verify the goroutine executed
        let result_values = results.lock().unwrap();
        assert_eq!(*result_values, vec![42]);
    }

    #[test]
    fn test_multiple_goroutines() {
        // Create a shared vector to store results
        let results = Arc::new(Mutex::new(Vec::new()));
        
        // Launch multiple goroutines that add different values
        let mut handles = vec![];
        
        for i in 0..5 {
            let results_clone = Arc::clone(&results);
            
            // Create a callable that adds a value to the results
            let callable = Arc::new(TestCallable {
                result: results_clone,
                value: i,
            });
            
            // Create thread-safe arguments
            let args = vec![ThreadSafeObject::new(i)];
            
            // Run the goroutine in a separate thread to simulate concurrent execution
            let handle = thread::spawn(move || {
                thread_safe_goroutine::run_goroutine(callable, args).unwrap();
            });
            
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Verify results (order might vary)
        let result_values = results.lock().unwrap();
        assert_eq!(result_values.len(), 5);
        
        // Sorted results should be [0, 1, 2, 3, 4]
        let mut sorted_results = result_values.clone();
        sorted_results.sort();
        assert_eq!(sorted_results, vec![0, 1, 2, 3, 4]);
    }
    
    #[test]
    fn test_wait_all_goroutines() {
        // Create a shared vector to store results
        let results = Arc::new(Mutex::new(Vec::new()));
        
        // Launch 5 goroutines that sleep for different durations
        for i in 0..5 {
            let results_clone = Arc::clone(&results);
            let delay_ms = i * 10; // Make each goroutine sleep a bit longer
            
            thread_safe_goroutine::run_goroutine_fn(move || {
                // Sleep for the specified duration
                thread::sleep(Duration::from_millis(delay_ms));
                
                // Add the value to the results
                let mut result_guard = results_clone.lock().unwrap();
                result_guard.push(i);
            }).unwrap();
        }
        
        // Wait for all goroutines to complete (with a reasonable timeout)
        thread_safe_goroutine::wait_all_goroutines(1000).unwrap();
        
        // Verify all goroutines have completed
        let result_values = results.lock().unwrap();
        assert_eq!(result_values.len(), 5);
        
        // Sorted results should be [0, 1, 2, 3, 4]
        let mut sorted_results = result_values.clone();
        sorted_results.sort();
        assert_eq!(sorted_results, vec![0, 1, 2, 3, 4]);
    }
}