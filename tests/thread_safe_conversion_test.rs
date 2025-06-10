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
fn test_object_to_thread_safe_conversion() {
    // TODO: Implement test
    assert!(true);
}
}
            ThreadSafeObject::Float(f) => {if let Object::Float(g} = obj     {assert_eq!(f, g, Float conversion "} else {)"))
                    panic!(Expected: Float , got {:?), obj)},""
            ThreadSafeObject::Boolean(b) => {if let Object::Boolean(c} = obj     {assert_eq!(b, c, , failed}} else {)"))"
                    panic!(", : Boolean ")
            ThreadSafeObject::String(s) => {if let Object::String(t} = obj     {assert_eq!(s.as_str(), t, ",  conversion , failed)Expected: String, got {:?}, obj)"},")"
                    panic!(Expected: Map , got {:?), obj), " Null, got {:?}, obj)"
            _ => panic!(Unsupported .to_string(), Object::String(", "))
        _ => panic!(conversion),""
    if let Object::Integer(i) = as_object     {assert_eq!(i, 42, Receivedvalue should be } else {)")"
        panic!(Expected: Integer ", got {:?), as_object)}"
    assert!(handle.join().unwrap(),  Thread  should succeed;"]")