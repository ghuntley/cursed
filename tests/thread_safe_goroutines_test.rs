use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

#[test]
fn test_thread_safe_goroutines() {
    use cursed::object_thread_safe::{ThreadSafeObject, ThreadSafeChannel};
    use cursed::core::thread_safe_goroutine::{launch_thread_safe_goroutine, thread_safe_sleep};
    use cursed::error::Error;
    
    // Create a shared counter that goroutines will increment
    let counter = Arc::new(Mutex::new(0));
    let counter_for_closure = Arc::clone(&counter);
    
    // Create a function object that increments the counter
    let increment_fn = ThreadSafeObject::Builtin {
        name: "increment".to_string(),
        function: Arc::new(move |_args: Vec<ThreadSafeObject>| -> Result<ThreadSafeObject, Error> {
            // Get a lock on the counter and increment it
            let mut counter_guard = counter_for_closure.lock().unwrap();
            *counter_guard += 1;
            Ok(ThreadSafeObject::Null)
        }),
    };
    
    // Launch multiple goroutines
    let num_goroutines = 10;
    for _ in 0..num_goroutines {
        launch_thread_safe_goroutine(&increment_fn, vec![]).unwrap();
    }
    
    // Sleep to allow goroutines to complete
    thread::sleep(Duration::from_millis(100));
    
    // Check that the counter was incremented by all goroutines
    let final_count = *counter.lock().unwrap();
    
    // The count should be equal to the number of goroutines
    assert_eq!(final_count, num_goroutines);
}