use crate::object::Object;
use crate::memory::{Traceable, Visitor, tagged::Tag};
use std::collections::HashMap;
use std::rc::Rc;

struct CountingVisitor {
    visited_count: usize,
}

impl Visitor for CountingVisitor {
    fn visit(&mut self, _obj: &dyn std::any::Any) {
        self.visited_count += 1;
    }

    fn visit_ptr(&mut self, _ptr: usize, _tag: Tag) {
        self.visited_count += 1;
    }
}

#[derive(Debug, Clone)]
pub struct TestArray {
    elements: Vec<Object>,
}

impl Traceable for TestArray {
    fn trace(&self, visitor: &mut dyn Visitor) {
        for element in &self.elements {
            visitor.visit(element);
        }
    }

    fn size(&self) -> usize {
        std::mem::size_of::<Vec<Object>>() + 
            self.elements.len() * std::mem::size_of::<Object>()
    }
}

// Test function to create a traceable array
pub fn create_test_array() -> TestArray {
    let elements = vec![
        Object::Integer(1),
        Object::Integer(2),
        Object::Integer(3),
    ];
    
    TestArray {
        elements: elements.to_vec(),
    }
}

#[test]
fn test_object_trace() {
    // Create an array object with some elements
    let elements = vec![
        Object::Integer(1),
        Object::Integer(2),
        Object::Integer(3),
    ];
    let array = Object::Array(elements.to_vec());
    
    // Create a visitor to count traced objects
    let mut visitor = CountingVisitor { visited_count: 0 };
    
    // Trace the array
    array.trace(&mut visitor);
    
    // We should have visited 3 objects
    assert_eq!(visitor.visited_count, 3);
    
    // Create a hash table
    let mut map = HashMap::new();
    map.insert("a".to_string(), Object::Integer(1));
    map.insert("b".to_string(), Object::Integer(2));
    
    let hash_table = Object::HashTable(map);
    
    // Reset visitor
    visitor.visited_count = 0;
    
    // Trace the hash table
    hash_table.trace(&mut visitor);
    
    // We should have visited 2 objects
    assert_eq!(visitor.visited_count, 2);
} 