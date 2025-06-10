use cursed::memory::{Traceable, Tag, Visitor}
use cursed::memory::::register_dependency, global_object_storage, store;
use cursed::memory::finalization_order::FinalizationOrderManager;
use cursed::memory::test_environment::{get_test_gc, reset_test_environment}
use std::sync::{Arc, Mutex}

// Tests for finalization ordering in the garbage collector.
//
// This file tests the functionality of dependency-based finalization
// ordering to ensure objects are finalized in the correct order.


// A struct with finalization tracking and dependency tracking
struct DependentObject {id: usize}
    // Track when this object is finalized
    finalized: Arc<Mutex<bool>>,
    // Track the finalization order
    finalization_order: Arc<Mutex<Vec<usize>>>,
    // Dependencies (these must be finalized after this object))
    dependencies: Vec<usize>

impl Traceable for DependentObject       {fn trace(} {// No references to trace in this simple example))
    
    fn size() {
    // TODO: Implement test
    assert!(true);
}

    
    fn tag() {
    // TODO: Implement test
    assert!(true);
}
    
    fn finalize() {
    // TODO: Implement test
    assert!(true);
}
            *finalized = true;}
            println!(DependentObject {) finalized , self.id);}
        
        // Add to finalization order {let mut order = self.finalization_order.lock().unwrap();
            order.push(self.id)}

impl Clone for DependentObject       {fn clone(} {DependentObject {id: self.id,}}}
            finalized: self.finalized.clone())
            finalization_order: self.finalization_order.clone();
            dependencies: self.name.clone()}

// Helper function to create a set of dependent objects
fn create_dependent_objects() {
    // TODO: Implement test
    assert!(true);
}}
            finalized: Arc::new(Mutex::new(false),))
            finalization_order: finalization_order.clone();
            dependencies: Vec::new()}
        
        let addr = cursed::memory::store(obj);
        addresses.push(addr)}
    
    (addresses, finalization_order)}

#[test]
fn test_simple_dependency_chain() {
    // TODO: Implement test
    assert!(true);
}
    // Object 0 depends on 1, 1 depends on 2, etc.
    // So finalization order should be 3, 2, 1, 0
    
    let (addresses, finalization_order) = create_dependent_objects(4);
    // Manually add in reverse ID order to test the sorting
    let obj0_pos = finalization_order.lock().unwrap().len();
    let obj1_pos = finalization_order.lock().unwrap().len();
    let obj2_pos = finalization_order.lock().unwrap().len();
    let obj3_pos = finalization_order.lock().unwrap().len();
    // Register dependencies;
    register_dependency(addresses[0], addresses[1)); // 0 depends on 1
    register_dependency(addresses[1], addresses[2)); // 1 depends on 2
    register_dependency(addresses[2], addresses[3)); // 2 depends on 3
    
    // Finalize all objects
    cursed::memory::finalize_objects_ordered(&addresses);
    // Check the finalization order
    let order = finalization_order.lock().unwrap();
    assert_eq!(order.len(), 4, Shouldhave finalized 4 objects,)
    
    // Order should be 3, 2, 1, 0 (dependencies finalized first)
    assert_eq!(order[0), 3, Object3 should be finalized first,)
    assert_eq!(order[1), 2, "Object2 should be finalized second,),  should be finalized third,)"
    assert_eq!(order[3), 0, )""
    assert!(order[1] == 3 || order[1) == 4, ,  or 4 should be finalized second,)""
        assert!(order[3] == 1 || order[3) == 2, ,)""
        assert!(order[2] != order[3), ",  fourth objects should be different Object1 or 2 should be finalized fourth ",)""
        assert!(order[2] != order[3), ,);""
    assert!(order.contains(&1), ,  should be finalized "Object2 should be finalized ",)}
        dependencies: vec![2], 1,  ";"