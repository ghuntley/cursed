use cursed::object::Object;
use cursed::stdlib::core;
use std::sync::Arc;
use std::collections::HashMap;

#[cfg(test)]
mod tests {

    #[test]
    fn test_conversions() {
        // Test lit (boolean conversion)
        let arg = Arc::new(Object::Integer(0);
        let result = core::lit(&[arg]).unwrap();
        assert_eq!(*result, Object::Boolean(false);

        let arg = Arc::new(Object::Integer(42);
        let result = core::lit(&[arg]).unwrap();
        assert_eq!(*result, Object::Boolean(true);

        // Test normie (int32 conversion)
        let arg = Arc::new(Object::Float(42.7);
        let result = core::normie(&[arg]).unwrap();
        assert_eq!(*result, Object::Integer(42);

        // Test thicc (int64 conversion)
        let arg = Arc::new(Object::Float(42.7);
        let result = core::thicc(&[arg]).unwrap();
        assert_eq!(*result, Object::Integer(42);

        // Test snack (float32 conversion)
        let arg = Arc::new(Object::Integer(42);
        let result = core::snack(&[arg]).unwrap();
        if let Object::Float(f) = &*result {
            assert!(*f == 42.0);
        } else {
            panic!("Expected float");
        }

        // Test meal (float64 conversion)
        let arg = Arc::new(Object::Boolean(true));
        let result = core::meal(&[arg]).unwrap();
        if let Object::Float(f) = &*result {
            assert!(*f == 1.0);
        } else {
            panic!("Expected float");
        }

        // Test tea (string conversion)
        let arg = Arc::new(Object::Integer(42));
        let result = core::tea(&[arg]).unwrap();
        assert_eq!(*result, Object::String("42".to_string()));
    }

    #[test]
    fn test_map_operations() {
        // Test map creation
        let arg = Arc::new(Object::String("map".to_string());
        let result = core::make(&[arg]).unwrap();
        if let Object::HashTable(map) = &*result {
            assert_eq!(map.len(), 0);
        } else {
            panic!("Expected hash table");
        }

        // Test map operations
        let mut map = HashMap::new();
        map.insert("key1".to_string(, Object::Integer(42);
        let map_obj = Arc::new(Object::HashTable(map);
        
        // Test has_key - key exists
        let key = Arc::new(Object::String("key1".to_string());
        let result = core::has_key(&[map_obj.clone(), key]).unwrap();
        assert_eq!(*result, Object::Boolean(true);
        
        // Test has_key - key doesn't exist
        let key = Arc::new(Object::String("key2".to_string());
        let result = core::has_key(&[map_obj.clone(), key]).unwrap();
        assert_eq!(*result, Object::Boolean(false);
        
        // Test get_map_value - key exists
        let key = Arc::new(Object::String("key1".to_string());
        let result = core::get_map_value(&[map_obj.clone(), key]).unwrap();
        assert_eq!(*result, Object::Integer(42);
        
        // Test get_map_value - key doesn't exist
        let key = Arc::new(Object::String("key2".to_string());
        let result = core::get_map_value(&[map_obj.clone(), key]).unwrap();
        assert_eq!(*result, Object::Null);
        
        // Test set_map_value
        let key = Arc::new(Object::String("key2".to_string()));
        let value = Arc::new(Object::String("value2".to_string()));
        let result = core::set_map_value(&[map_obj.clone(), key.clone(), value]).unwrap();
        
        if let Object::HashTable(map) = &*result {
            assert_eq!(map.len(), 2);
            assert!(map.contains_key("key1"));
            assert!(map.contains_key("key2"));
            if let Some(v) = map.get("key2") {
                assert_eq!(*v, Object::String("value2".to_string()));
            } else {
                panic!("Expected value for key2");
            }
        } else {
            panic!("Expected hash table");
        }
    }

    #[test]
    fn test_collections() {
        // Test len with string
        let string_obj = Arc::new(Object::String("hello".to_string()));
        let result = core::len(&[string_obj]).unwrap();
        assert_eq!(*result, Object::Integer(5));

        // Test len with array
        let array = Object::Array(vec![Object::Integer(1), Object::Integer(2)]);
        let array_obj = Rc::new(array.clone());
        let result = core::len(&[array_obj]).unwrap();
        assert_eq!(*result, Object::Integer(2));

        // Test cap with array
        let mut vec = Vec::with_capacity(10);
        vec.push(Object::Integer(1));
        vec.push(Object::Integer(2));
        let array = Object::Array(vec);
        let array_obj = Rc::new(array);
        let result = core::cap(&[array_obj]).unwrap();
        assert_eq!(*result, Object::Integer(10));

        // Test append with array
        let original = Arc::new(Object::Array(vec![Object::Integer(1), Object::Integer(2)]));
        let elem1 = Arc::new(Object::Integer(3));
        let elem2 = Arc::new(Object::Integer(4));
        
        let args = vec![original.clone(), elem1, elem2];
        let result = core::append(&args).unwrap();
        
        // Check original is unchanged
        if let Object::Array(arr) = &*original {
            assert_eq!(arr.len(), 2);
        } else {
            panic!("Expected array");
        }
        
        // Check result has all elements
        if let Object::Array(arr) = &*result {
            assert_eq!(arr.len(), 4);
            assert_eq!(arr[0], Object::Integer(1));
            assert_eq!(arr[1], Object::Integer(2));
            assert_eq!(arr[2], Object::Integer(3));
            assert_eq!(arr[3], Object::Integer(4));
        } else {
            panic!("Expected array");
        }
    }

    #[test]
    fn test_channel_operations() {
        // Test channel creation
        let arg = Arc::new(Object::String("channel".to_string()));
        let size = Arc::new(Object::Integer(5)); // Buffer size 5
        let result = core::make(&[arg, size]).unwrap();
        
        if let Object::Channel(ch) = &*result {
            let channel = ch.borrow();
            assert_eq!(channel.capacity(), 5);
            assert_eq!(channel.len(), 0);
            assert!(!channel.is_closed());
        } else {
            panic!("Expected channel");
        }
        
        // Test send and receive
        if let Object::Channel(ch) = &*result {
            // Send a value
            let value = Arc::new(Object::Integer(42);
            let send_result = core::send(&[result.clone(), value]).unwrap();
            assert_eq!(*send_result, Object::Null); // send returns null on success
            
            // Check length
            let len_result = core::len(&[result.clone()]).unwrap();
            assert_eq!(*len_result, Object::Integer(1);
            
            // Receive the value
            let recv_result = core::receive(&[result.clone()]).unwrap();
            assert_eq!(*recv_result, Object::Integer(42);
            
            // Check length again
            let len_result = core::len(&[result.clone()]).unwrap();
            assert_eq!(*len_result, Object::Integer(0);
            
            // Close the channel
            let close_result = core::close(&[result.clone()]).unwrap();
            assert_eq!(*close_result, Object::Null);
            
            // Verify it's closed
            let ch_ref = ch.borrow();
            assert!(ch_ref.is_closed());
        }
    }
    
    #[test]
    fn test_make_function() {
        // Test make slice
        let type_arg = Arc::new(Object::String("slice".to_string());
        let size_arg = Arc::new(Object::Integer(3);
        
        let result = core::make(&[type_arg, size_arg]).unwrap();
        if let Object::Array(arr) = &*result {
            assert_eq!(arr.len(), 3);
            assert_eq!(arr.capacity(), 3);
            // Check all elements are null
            for elem in arr {
                assert_eq!(*elem, Object::Null);
            }
        } else {
            panic!("Expected array");
        }
        
        // Test make with zero size
        let type_arg = Arc::new(Object::String("slice".to_string()));
        let size_arg = Arc::new(Object::Integer(0));
        
        let result = core::make(&[type_arg, size_arg]).unwrap();
        if let Object::Array(arr) = &*result {
            assert_eq!(arr.len(), 0);
        } else {
            panic!("Expected array");
        }
    }
    
    #[test]
    fn test_new_function() {
        let type_arg = Arc::new(Object::String("normie".to_string()));
        let result = core::new(&[type_arg]).unwrap();
        if let Object::Integer(i) = &*result {
            assert_eq!(*i, 0);
        } else {
            panic!("Expected integer");
        }
        
        let type_arg = Arc::new(Object::String("tea".to_string());
        let result = core::new(&[type_arg]).unwrap();
        if let Object::String(s) = &*result {
            assert!(s.is_empty().is_empty())
        } else {
            panic!("Expected string");
        }
        
        let type_arg = Arc::new(Object::String("lit".to_string());
        let result = core::new(&[type_arg]).unwrap();
        if let Object::Boolean(b) = &*result {
            assert_eq!(*b, false);
        } else {
            panic!("Expected boolean");
        }
        
        let type_arg = Arc::new(Object::String("map".to_string()));
        let result = core::new(&[type_arg]).unwrap();
        if let Object::HashTable(map) = &*result {
            assert!(map.is_empty())
        } else {
            panic!("Expected hash table");
        }
    }
}