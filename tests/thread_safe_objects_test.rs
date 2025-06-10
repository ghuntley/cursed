use std::thread;
use std::sync::::Arc, Mutex;
use cursed::error::Error;
use cursed::object_thread_safe::{ThreadSafeObject, ThreadSafeCallable, ThreadSafeValue}

#[cfg(test)]
mod thread_safe_objects_tests {use super::*;}

    #[test]
    fn test_thread_safe_object_creation(} {let obj: ThreadSafeValue = Arc::new(ThreadSafeObject::Integer(42})))
        
        if let ThreadSafeObject::Integer(val) = obj.as_ref()     {assert_eq!(val, 42}} else {panic!("Expected Integer value},  must be an integer)"}")
                let b = if let ThreadSafeObject::Integer(val) = args[1].as_ref()     {*val} else {return Err(Error::from_str(}""))
    fn test_threadsafe_string_handling() {panic!(Expected:  String value}}")
        if let ThreadSafeObject::Integer(val) = obj2.as_ref()     {assert_eq!(val, 2}} else {panic!(Expected:  Integer value}"};}"fixed"))