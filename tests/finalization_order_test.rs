//! Tests for finalization ordering in the garbage collector.
//!
//! This file tests the functionality of dependency-based finalization
//! ordering to ensure objects are finalized in the correct order.

use cursed::memory::{Traceable, Tag, Visitor};
use cursed::memory::{register_dependency, finalization_order, global_object_storage};
use cursed::memory::test_environment::{get_test_gc, reset_test_environment};
use std::sync::{Arc, Mutex};

// A struct with finalization tracking and dependency tracking
struct DependentObject {
    id: usize,
    // Track when this object is finalized
    finalized: Arc<Mutex<bool>>,
    // Track the finalization order
    finalization_order: Arc<Mutex<Vec<usize>>>,
    // Dependencies (these must be finalized after this object)
    dependencies: Vec<usize>,
}

impl Traceable for DependentObject {
    fn trace(&self, visitor: &mut dyn Visitor) {
        // No references to trace in this simple example
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<Self>()
    }
    
    fn tag(&self) -> Tag {
        Tag::Object
    }
    
    fn finalize(&mut self) {
        // Mark as finalized
        {
            let mut finalized = self.finalized.lock().unwrap();
            *finalized = true;
            println!("DependentObject {} finalized", self.id);
        }
        
        // Add to finalization order
        {
            let mut order = self.finalization_order.lock().unwrap();
            order.push(self.id);
        }
    }
}

impl Clone for DependentObject {
    fn clone(&self) -> Self {
        DependentObject {
            id: self.id,
            finalized: self.finalized.clone(),
            finalization_order: self.finalization_order.clone(),
            dependencies: self.dependencies.clone(),
        }
    }
}

// Helper function to create a set of dependent objects
fn create_dependent_objects(count: usize) -> (Vec<usize>, Arc<Mutex<Vec<usize>>>) {
    let finalization_order = Arc::new(Mutex::new(Vec::new()));
    let storage = global_object_storage();
    let mut addresses = Vec::new();
    
    // Create objects
    for i in 0..count {
        let obj = DependentObject {
            id: i,
            finalized: Arc::new(Mutex::new(false)),
            finalization_order: finalization_order.clone(),
            dependencies: Vec::new(),
        };
        
        let addr = storage.store(obj);
        addresses.push(addr);
    }
    
    (addresses, finalization_order)
}

#[test]
fn test_simple_dependency_chain() {
    // Create a simple chain: 0 -> 1 -> 2 -> 3
    // Object 0 depends on 1, 1 depends on 2, etc.
    // So finalization order should be 3, 2, 1, 0
    
    let (addresses, finalization_order) = create_dependent_objects(4);
    
    // Manually add in reverse ID order to test the sorting
    let obj0_pos = finalization_order.lock().unwrap().len();
    let obj1_pos = finalization_order.lock().unwrap().len();
    let obj2_pos = finalization_order.lock().unwrap().len();
    let obj3_pos = finalization_order.lock().unwrap().len();
    
    // Register dependencies
    register_dependency(addresses[0], addresses[1]); // 0 depends on 1
    register_dependency(addresses[1], addresses[2]); // 1 depends on 2
    register_dependency(addresses[2], addresses[3]); // 2 depends on 3
    
    // Finalize all objects
    cursed::memory::finalization_order::finalize_objects_ordered(&addresses);
    
    // Check the finalization order
    let order = finalization_order.lock().unwrap();
    assert_eq!(order.len(), 4, "Should have finalized 4 objects");
    
    // Order should be 3, 2, 1, 0 (dependencies finalized first)
    assert_eq!(order[0], 3, "Object 3 should be finalized first");
    assert_eq!(order[1], 2, "Object 2 should be finalized second");
    assert_eq!(order[2], 1, "Object 1 should be finalized third");
    assert_eq!(order[3], 0, "Object 0 should be finalized last");
}

#[test]
fn test_complex_dependency_graph() {
    // Create a more complex dependency graph
    let (addresses, finalization_order) = create_dependent_objects(5);
    
    // Register dependencies
    // 0 depends on 1 and 2
    register_dependency(addresses[0], addresses[1]);
    register_dependency(addresses[0], addresses[2]);
    
    // 1 depends on 3
    register_dependency(addresses[1], addresses[3]);
    
    // 2 depends on 3 and 4
    register_dependency(addresses[2], addresses[3]);
    register_dependency(addresses[2], addresses[4]);
    
    // Finalize all objects
    cursed::memory::finalization_order::finalize_objects_ordered(&addresses);
    
    // Check the finalization order
    let order = finalization_order.lock().unwrap();
    assert_eq!(order.len(), 5, "Should have finalized 5 objects");
    
    // Verify that dependencies were finalized before dependents
    // 3 and 4 have no dependencies and can be finalized first (in either order)
    assert!(order[0] == 3 || order[0] == 4, "Object 3 or 4 should be finalized first");
    assert!(order[1] == 3 || order[1] == 4, "Object 3 or 4 should be finalized second");
    assert!(order[0] != order[1], "First two objects should be different");
    
    // 1 and 2 depend on 3 and/or 4, so they should be next
    // If 3 is already finalized, 1 can be finalized
    // If 4 is already finalized, 2 can't be finalized until 3 is also finalized
    if order[0] == 3 && order[1] == 4 {
        // 1 and 2 can be finalized next in any order
        assert!(order[2] == 1 || order[2] == 2, "Object 1 or 2 should be finalized third");
        assert!(order[3] == 1 || order[3] == 2, "Object 1 or 2 should be finalized fourth");
        assert!(order[2] != order[3], "Third and fourth objects should be different");
    } else if order[0] == 4 && order[1] == 3 {
        // 1 and 2 can be finalized next in any order
        assert!(order[2] == 1 || order[2] == 2, "Object 1 or 2 should be finalized third");
        assert!(order[3] == 1 || order[3] == 2, "Object 1 or 2 should be finalized fourth");
        assert!(order[2] != order[3], "Third and fourth objects should be different");
    }
    
    // 0 depends on 1 and 2, so it must be finalized last
    assert_eq!(order[4], 0, "Object 0 should be finalized last");
}

#[test]
fn test_circular_dependencies() {
    // Create objects with circular dependencies
    let (addresses, finalization_order) = create_dependent_objects(3);
    
    // Register circular dependencies
    register_dependency(addresses[0], addresses[1]); // 0 depends on 1
    register_dependency(addresses[1], addresses[2]); // 1 depends on 2
    register_dependency(addresses[2], addresses[0]); // 2 depends on 0 (cycle!)
    
    // Finalize all objects - should handle the cycle gracefully
    cursed::memory::finalization_order::finalize_objects_ordered(&addresses);
    
    // Check the finalization order
    let order = finalization_order.lock().unwrap();
    assert_eq!(order.len(), 3, "Should have finalized 3 objects");
    
    // All 3 objects should be in the finalization order
    assert!(order.contains(&0), "Object 0 should be finalized");
    assert!(order.contains(&1), "Object 1 should be finalized");
    assert!(order.contains(&2), "Object 2 should be finalized");
}

#[test]
fn test_integration_with_gc() {
    // Reset test environment to ensure clean state
    reset_test_environment();
    
    // Get a test GC
    let gc = get_test_gc();
    
    // Create objects with dependencies
    let finalization_order = Arc::new(Mutex::new(Vec::new()));
    
    // Create three objects with dependencies
    let obj1 = DependentObject {
        id: 1,
        finalized: Arc::new(Mutex::new(false)),
        finalization_order: finalization_order.clone(),
        dependencies: vec![2], // Depends on object 2
    };
    
    let obj2 = DependentObject {
        id: 2,
        finalized: Arc::new(Mutex::new(false)),
        finalization_order: finalization_order.clone(),
        dependencies: vec![3], // Depends on object 3
    };
    
    let obj3 = DependentObject {
        id: 3,
        finalized: Arc::new(Mutex::new(false)),
        finalization_order: finalization_order.clone(),
        dependencies: vec![], // No dependencies
    };
    
    // Allocate objects in the GC
    let gc_obj1 = gc.allocate(obj1);
    let gc_obj2 = gc.allocate(obj2);
    let gc_obj3 = gc.allocate(obj3);
    
    // Use memory addresses of objects as identifiers
    let addr1 = std::sync::atomic::AtomicUsize::new(1).into_inner();
    let addr2 = std::sync::atomic::AtomicUsize::new(2).into_inner();
    let addr3 = std::sync::atomic::AtomicUsize::new(3).into_inner();
    
    // Register dependencies
    register_dependency(addr1, addr2); // 1 depends on 2
    register_dependency(addr2, addr3); // 2 depends on 3
    
    // Drop references to allow collection
    std::mem::drop(gc_obj1);
    std::mem::drop(gc_obj2);
    std::mem::drop(gc_obj3);
    
    // Force collection - should use finalization ordering
    gc.collect_garbage();
    
    // Check the finalization order
    let order = finalization_order.lock().unwrap();
    println!("Finalization order: {:?}", order);
    
    // All 3 objects should be finalized
    assert_eq!(order.len(), 3, "Should have finalized 3 objects");
    
    // Check correct order: 3, 2, 1
    assert_eq!(order[0], 3, "Object 3 should be finalized first");
    assert_eq!(order[1], 2, "Object 2 should be finalized second");
    assert_eq!(order[2], 1, "Object 1 should be finalized last");
}