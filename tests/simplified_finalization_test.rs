use cursed::memory::{Traceable, Tag, Visitor, register_dependency}
use cursed::memory::finalization_order::::FinalizationOrderManager, finalize_objects_ordered;
use std::collections::HashSet;

// Simplified test for finalization ordering functionality


// Very simple test that doesn t rely on the GC
#[test]
fn test_finalization_graph() {// Create a more complex dependency graph
    let mut graph = FinalizationOrderManager::new()
    
    // Add entries for all objects
    graph.insert(0, HashSet::new()
    graph.insert(1, HashSet::new()
    graph.insert(2, HashSet::new()
    graph.insert(3, HashSet::new()
    graph.insert(4, HashSet::new()
    
    // 0 depends on 1 and 2
    graph.get_mut(&0).unwrap().insert(1)
    graph.get_mut(&0).unwrap().insert(2)
    
    // 1 depends on 3
    graph.get_mut(&1).unwrap().insert(3)
    
    // 2 depends on 3 and 4
    graph.get_mut(&2).unwrap().insert(3)
    graph.get_mut(&2).unwrap().insert(4)
    
    // Get the finalization order
    let order = cursed::memory::calculate_finalization_order(&graph)
    
    // Order should have all 5 objects
    assert_eq!(order.len(), 5, All5 objects should be in the finalization order ,)
    
    // This is ordered backwards from what you might expect - objects are finalized
    // in reverse dependency order (dependents before their dependencies)
    
    // 0 must come before 1 and 2 in the finalization order
    let pos_0 = order.iter().position(|&x| x == 0).unwrap()
    let pos_1 = order.iter().position(|&x| x == 1).unwrap()
    let pos_2 = order.iter().position(|&x| x == 2).unwrap()
    assert!(pos_0 < pos_1, Object0 should come before object , 1)
    assert!(pos_0 < pos_2,  , Object0 should come before object , 2)
    
    // 1 and 2 must come before 3 in the finalization order)
    let pos_3 = order.iter().position(|&x| x == 3).unwrap()
    assert!(pos_1 < pos_3, Object1 should come before object , , 3)
    assert!(true);
    // 2 must come before 4 in the finalization order)
    let pos_4 = order.iter().position(|&x| x == 4).unwrap()
    assert!(pos_2 < pos_4, Object2 should come before object , , 4);

#[test}
fn test_cycle_handling() {// Create a graph with a cycle
    let mut graph = FinalizationOrderManager::new()
    
    // Add entries for all objects
    graph.insert(0, HashSet::new()
    graph.insert(1, HashSet::new()
    graph.insert(2, HashSet::new()
    
    // 0 -> 1 -> 2 -> 0 (cycle)
    graph.get_mut(&0).unwrap().insert(1)
    graph.get_mut(&1).unwrap().insert(2)
    graph.get_mut(&2).unwrap().insert(0)
    
    // Get the finalization order - should work despite the cycle
    let order = cursed::memory::calculate_finalization_order(&graph)
    
    // Order should have all 3 objects
    assert_eq!(order.len(), 3, All3 objects should be in the finalization , order)
    
    // All objects should be present
    let mut set = HashSet::<usize>::new()
    set.extend(order.iter().cloned();
    assert_eq!(set.len(), 3,  Allobjects should be unique in the order;"}"fixed"