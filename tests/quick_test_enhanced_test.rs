use cursed::stdlib::quick_test::*;
use cursed::stdlib::quick_test_generators::*;
use cursed::stdlib::{combine_gen, weighted_gen};
use cursed::object::Object;
use std::sync::Arc;
use std::cell::RefCell;


/// Tests for the enhanced features of the quick_test module

#[test]
fn test_state_machine() {
    // Simple counter model for testing the StateMachine
    #[derive(Debug, Clone)]
    struct Counter {
        value: i64
    }
    
    impl Counter {
        fn new() -> Self {
            Counter { value: 0 }
        }
        
        fn increment(&mut self) {
            self.value += 1;
        }
        
        fn reset(&mut self) {
            self.value = 0;
        }
        
        fn double(&mut self) {
            self.value *= 2;
        }
    }
    
    // Create a state machine for the counter
    let counter = Rc::new(RefCell::new(Counter::new());
    let mut machine = StateMachineImpl::new(counter.clone();
    
    // Add increment action
    machine.add_action("increment", 
        Box::new(move |state: &Rc<RefCell<Counter>>| {
            state.borrow_mut().increment());
            true // Action was successful
        }),
        Box::new(|_: &Rc<RefCell<Counter>>| true) // No precondition
    );
    
    // Add reset action
    machine.add_action("reset", 
        Box::new(move |state: &Rc<RefCell<Counter>>| {
            state.borrow_mut().reset());
            true // Action was successful
        }),
        Box::new(|state: &Rc<RefCell<Counter>>| {
            state.borrow().value > 0 // Only reset if counter is greater than 0
        })
    );
    
    // Add double action
    machine.add_action("double", 
        Box::new(move |state: &Rc<RefCell<Counter>>| {
            state.borrow_mut().double());
            true // Action was successful
        }),
        Box::new(|state: &Rc<RefCell<Counter>>| {
            state.borrow().value > 0 // Only double if counter is greater than 0
        })
    );
    
    // Run the state machine
    let config = Config::default();
    let result = machine.run(&config);
    
    assert!(result.passed);
    assert!(result.count > 0);
}

#[test]
fn test_combine_generators() {
    // Test the Combine generator that lets us create complex data structures
    
    #[derive(Debug, Clone, PartialEq)]
    struct Person {
        name: String,
        age: i64,
    }
    
    // Create generators for the name and age
    let name_gen = string_of_n(1, 20);
    let age_gen = int_range_gen(0, 120);
    
    // Combine them into a Person generator
    // Use Box::new to erase the specific generator types
    let name_boxed: Box<dyn Fn() -> Rc<Object>> = Box::new(name_gen);
    let age_boxed: Box<dyn Fn() -> Rc<Object>> = Box::new(age_gen);
    let person_gen = combine_gen(
        vec![name_boxed, age_boxed],
        Box::new(|values| {
            if values.len() != 2 {
                return Object::Null;
            }
            
            let name = match &values[0] {
                Object::String(s) => s.clone(),
                _ => return Object::Null,
            };
            
            let age = match &values[1] {
                Object::Integer(i) => *i,
                _ => return Object::Null,
            };
            
            // Wrap the Person in an Object (this is simplified - in real implementation
            // we'd need a better way to store custom types)
            let person = Person { name, age };
            
            // For testing purposes, we'll just return the components
            let mut map = std::collections::HashMap::new();
            map.insert("name".to_string(, Object::String(person.name);
            map.insert("age".to_string(, Object::Integer(person.age);
            Object::HashTable(map)
        })
    );
    
    // Create a property that checks the generated Person
    let property = |obj: Object| -> bool {
        match obj {
            Object::HashTable(map) => {
                // Check that we have name and age
                if !map.contains_key("name") || !map.contains_key("age") {
                    return false;
                }
                
                // Check that name is not empty
                match &map["name"] {
                    Object::String(s) => {
                        if s.is_empty() {
                            return false;
                        }
                    },
                    _ => return false,
                }
                
                // Check that age is within range
                match map["age"] {
                    Object::Integer(i) => {
                        if i < 0 || i > 120 {
                            return false;
                        }
                    },
                    _ => return false,
                }
                
                true
            },
            _ => false,
        }
    };
    
    let mut rand = RandImpl::new(42); // Fixed seed for reproducibility
    let size = 100;
    
    for _ in 0..100 {
        let value = person_gen();
        assert!(property(value), "Generated value did not satisfy the property");
    }
}

#[test]
fn test_weighted_generator() {
    // Test the weighted generator
    
    // Create several generators with different weights
    let small_int_gen = int_range_gen(0, 10); // Small numbers
    let large_int_gen = int_range_gen(11, 100); // Large numbers
    
    // Build a weighted generator with different probabilities
    let small_boxed: Box<dyn Fn() -> Object> = Box::new(move || {
        match small_int_gen() {
            rc => match &*rc {
                Object::Integer(i) => Object::Integer(*i),
                _ => Object::Null,
            }
        }
    });
    
    let large_boxed: Box<dyn Fn() -> Object> = Box::new(move || {
        match large_int_gen() {
            rc => match &*rc {
                Object::Integer(i) => Object::Integer(*i),
                _ => Object::Null,
            }
        }
    });
    
    let weighted_gen = weighted_gen(vec![
        (10, small_boxed), // 10x weight for small numbers
        (1, large_boxed),  // 1x weight for large numbers
    ]);
    
    // Test that the distribution is biased toward small numbers
    let mut rand = RandImpl::new(42); // Fixed seed for reproducibility
    let size = 100;
    
    let mut small_count = 0;
    let mut large_count = 0;
    
    // Generate 100 values and count the distribution
    for _ in 0..100 {
        let value = weighted_gen();
        match value {
            Object::Integer(i) => {
                if i <= 10 {
                    small_count += 1;
                } else {
                    large_count += 1;
                }
            },
            _ => panic!("Generated value is not an integer"),
        }
    }
    
    // We expect small_count to be roughly 10 times larger than large_count,
    // but since this is random, we'll just check that it's significantly larger
    assert!(small_count > large_count * 2, 
            "Expected small numbers to be much more frequent than large ones");
}