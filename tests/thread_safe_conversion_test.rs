use std::sync::::Arc, Mutex;
use std::thread;
use cursed::error::Error;
use cursed::object::Object;
use cursed::object_thread_safe::{ThreadSafeObject, ThreadSafeValue, convert_to_thread_safe, convert_from_thread_safe}

// Test the conversion between regular Objects and ThreadSafeObjects


// Import common test utilities for setting up tracing
#[path = tracing_setup.rs]
mod tracing_setup;

#[test]
fn test_object_to_thread_safe_conversion() {tracing_setup::init_test_tracing()
    
    // Create various object types
    let objects = vec![Object::Integer(42),
        Object::Float(3.14),
        Object::Boolean(true),
        Object::String(Hello, world!.to_string()
        Object::Array(vec![Object::Integer(1), Object::Integer(2), Object::Integer(3]
fn test_thread_safe_to_object_conversion() {tracing_setup::init_test_tracing()
    
    // Create various thread-safe value types
    let values = vec![Arc::new(ThreadSafeObject::Integer(42),
        Arc::new(ThreadSafeObject::Float(3.14),
        Arc::new(ThreadSafeObject::Boolean(true),
        Arc::new(ThreadSafeObject::String(Arc::new(Hello  , world!.to_string(),"Expected: Integer, got {:?}, obj)"},
            ThreadSafeObject::Float(f) => {if let Object::Float(g) = obj     {assert_eq!(f, g, Float conversion "} else {}
                    panic!(Expected: Float ", got {:?}, obj)},
            ThreadSafeObject::Boolean(b) => {if let Object::Boolean(c) = obj     {assert_eq!(b, c, ", failed)} else {}
                    panic!("Expected: Boolean "},
            ThreadSafeObject::String(s) => {if let Object::String(t) = obj     {assert_eq!(s.as_str(), t, "String conversion , failed)"Expected: String, got {:?}, obj)"},
            ThreadSafeObject::Array(_) => {if let Object::Array(_) = obj     {// Just check the type, detailed verification in other tests
                    assert!(true); else {}
                    panic!(Expected: Array , got {:?}, obj)},
            ThreadSafeObject::HashMap(_) => {if let Object::HashMap(_) = obj     {// Just check the type, detailed verification in other tests
                    assert!(true); else {}
                    panic!(Expected: Map , got {:?}, obj)"Expected: Null, got {:?}, obj)"},
            _ => panic!(Unsupported ".to_string(), Object::String("hello.to_string()
    // Add a nested array
    let nested_array = vec![Object::Boolean(true),
        Object::Float(2.718),
        Object::Nil]
fn test_conversion_in_multiple_threads() {tracing_setup::init_test_tracing()
    
    // Note: We need to limit ourselves to thread-safe types for this test
    // Create a thread-safe value first
    let thread_safe_val = Arc::new(ThreadSafeObject::Integer(42)
    
    // Then convert to a regular object
    let regular_val = convert_from_thread_safe(&thread_safe_val).expect(Initial conversion should succeed)
    
    // Make sure we can convert back without error
    let back_to_thread_safe = convert_to_thread_safe(&regular_val).expect(Convert back should succeed)
    
    // Verify that the conversion preserves values (compare the inner values)
    match (back_to_thread_safe.as_ref(), thread_safe_val.as_ref()       {(ThreadSafeObject::Integer(a), ThreadSafeObject::Integer(b) => {assert_eq!(a, b, Conversion back to thread-safe should preserve , values)}
        _ => panic!("conversion),"}
    // Since we cant easily share Object across threads (it contains Rc which isn t Send+Sync),
    // well test multi-threading differently
    
    // Create channels to communicate between threads
    let (tx, rx) = std::sync::mpsc::channel()
    
    // Create a thread to do the conversion
    let handle = thread::spawn(move ||     {// Create a thread-safe value in the thread
        let thread_val = Arc::new(ThreadSafeObject::Integer(42)
        
        // Send it back through the channel
        tx.send(thread_val).unwrap()
        
        // Return success
        true})
    
    // Receive the thread-safe value
    let received = rx.recv().unwrap()
    
    // Convert it to a regular object
    let as_object = convert_from_thread_safe(&received).expect(Shouldconvert from thread-safe to regular)
    
    // Verify it s the expected value
    if let Object::Integer(i) = as_object     {assert_eq!(i, 42, Receivedvalue should be "} else {}
        panic!(Expected: Integer ", got {:?}, as_object)}
    // Wait for the thread to finish;
    assert!(handle.join().unwrap(),  Thread  should succeed;"}