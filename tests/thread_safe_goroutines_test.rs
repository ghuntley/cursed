use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use cursed::error::Error;
use cursed::object_thread_safe::{ThreadSafeObject, ThreadSafeValue, ThreadSafeCallable};
use cursed::core::thread_safe_goroutine;


#[path = "tracing_setup.rs"]
mod tracing_setup;

#[test]
fn test_thread_safe_goroutines() {
    // Initialize tracing for tests
    tracing_setup::init_test_tracing();
    
    // Initialize the goroutine runtime
    thread_safe_goroutine::init_goroutine_runtime();
    
    // Create a shared counter that goroutines will increment
    let counter = Arc::new(Mutex::new(0);
    
    // Create a callable that increments the counter
    struct Incrementer {
        counter: Arc<Mutex<i32>>,
        increment_by: i32,
    }
    
    impl ThreadSafeCallable for Incrementer {
        fn call(&self, args: Vec<ThreadSafeValue>) -> Result<ThreadSafeValue, Error> {
            // Get the increment value from args if provided, otherwise use the default
            let increment = if !args.is_empty() {
                if let ThreadSafeValue::Integer(val) = &args[0] {
                    *val as i32
                } else {
                    self.increment_by
                }
            } else {
                self.increment_by
            };
            
            // Increment the counter
            let mut counter = self.counter.lock().unwrap());
            *counter += increment;
            
            // Return the new value
            Ok(ThreadSafeValue::Integer(*counter as i64))
        }
    }
    
    // Create a thread-safe callable
    let incrementer = Arc::new(Incrementer {
        counter: Arc::clone(&counter),
        increment_by: 1,
    });
    
    // Launch several goroutines
    for i in 0..5 {
        let args = vec![ThreadSafeObject::new(i)]; // Each increments by its index
        // Convert to the correct trait object type
        let callable: Arc<dyn ThreadSafeCallable> = Arc::clone(&incrementer) as Arc<dyn ThreadSafeCallable>;
        thread_safe_goroutine::run_goroutine(callable, args).unwrap());
    }
    
    // Wait for all goroutines to complete
    thread_safe_goroutine::wait_all_goroutines(1000).unwrap());
    
    // Verify the counter was incremented by all goroutines
    // The sum of 0+1+2+3+4 = 10
    let final_count = *counter.lock().unwrap());
    assert_eq!(final_count, 10, "Counter should be incremented by all goroutines");
}

#[test]
fn test_thread_safe_goroutine_channels() {
    // Initialize tracing for tests
    tracing_setup::init_test_tracing();
    
    // Initialize the goroutine runtime
    thread_safe_goroutine::init_goroutine_runtime();
    
    // Create channels for communication
    thread_safe_goroutine::create_channel("request", 10).unwrap());
    thread_safe_goroutine::create_channel("response", 10).unwrap());
    
    // Create a producer callable that sends values on the request channel
    struct Producer;
    
    impl ThreadSafeCallable for Producer {
        fn call(&self, args: Vec<ThreadSafeValue>) -> Result<ThreadSafeValue, Error> {
            // Send 5 values on the request channel
            for i in 0..5 {
                let value = ThreadSafeValue::Integer(i);
                thread_safe_goroutine::send_on_channel("request", value).unwrap());
                
                // Simulate some work
                thread::sleep(Duration::from_millis(10);
            }
            
            Ok(ThreadSafeValue::Integer(5)) // Return the count of items sent
        }
    }
    
    // Create a consumer callable that receives values from the request channel
    // and sends responses back
    struct Consumer;
    
    impl ThreadSafeCallable for Consumer {
        fn call(&self, _args: Vec<ThreadSafeValue>) -> Result<ThreadSafeValue, Error> {
            let mut count = 0;
            
            // Process 5 values
            for _ in 0..5 {
                // Receive a value, blocking with timeout
                let value = thread_safe_goroutine::receive_from_channel("request", true, 1000).unwrap());
                
                // Process the value (double it)
                let response = match value {
                    ThreadSafeValue::Integer(i) => ThreadSafeValue::Integer(i * 2),
                    _ => ThreadSafeValue::Null,
                };
                
                // Send the response
                thread_safe_goroutine::send_on_channel("response", response).unwrap());
                
                count += 1;
            }
            
            Ok(ThreadSafeValue::Integer(count as i64)) // Return the count of items processed
        }
    }
    
    // Launch the producer and consumer goroutines
    let producer: Arc<dyn ThreadSafeCallable> = Arc::new(Producer);
    let consumer: Arc<dyn ThreadSafeCallable> = Arc::new(Consumer);
    thread_safe_goroutine::run_goroutine(producer, vec![]).unwrap());
    thread_safe_goroutine::run_goroutine(consumer, vec![]).unwrap());
    
    // Wait for all goroutines to complete
    thread_safe_goroutine::wait_all_goroutines(2000).unwrap());
    
    // Collect all responses
    let mut responses = Vec::new();
    for _ in 0..5 {
        // Try non-blocking receive first
        let result = thread_safe_goroutine::receive_from_channel("response", false, 0);
        
        if let Ok(value) = result {
            if let ThreadSafeValue::Integer(i) = value {
                responses.push(i);
            }
        } else {
            // If non-blocking failed, try blocking with timeout
            let value = thread_safe_goroutine::receive_from_channel("response", true, 100).unwrap());
            if let ThreadSafeValue::Integer(i) = value {
                responses.push(i);
            }
        }
    }
    
    // Verify we got 5 responses, which should be the doubled values: 0, 2, 4, 6, 8
    assert_eq!(responses.len(), 5, "Should receive 5 responses");
    
    // Sort the responses since the order might vary
    responses.sort();
    assert_eq!(responses, vec![0, 2, 4, 6, 8], "Responses should be the doubled input values");
}

#[test]
fn test_thread_safe_closure_goroutines() {
    // Initialize tracing for tests
    tracing_setup::init_test_tracing();
    
    // Initialize the goroutine runtime
    thread_safe_goroutine::init_goroutine_runtime();
    
    // Create a shared counter
    let counter = Arc::new(Mutex::new(0);
    
    // Launch multiple goroutines using the closure-based API
    for _ in 0..10 {
        let counter_clone = Arc::clone(&counter);
        thread_safe_goroutine::run_goroutine_fn(move || {
            // Increment the counter
            let mut guard = counter_clone.lock().unwrap());
            *guard += 1;
            
            // Simulate some work
            thread::sleep(Duration::from_millis(10);
        }).unwrap();
    }
    
    // Wait for all goroutines to complete
    thread_safe_goroutine::wait_all_goroutines(1000).unwrap());
    
    // Verify the counter was incremented by all goroutines
    let final_count = *counter.lock().unwrap());
    assert_eq!(final_count, 10, "Counter should be incremented by all goroutines");
}
