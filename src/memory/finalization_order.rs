//! Finalization order management for garbage collection
//!
//! This module manages the finalization order of objects to ensure proper cleanup,
//! particularly for objects with dependencies on each other.

/// Finalization graph for managing object dependencies
/// This is a simple type alias for testing purposes
pub type FinalizationGraph = HashMap<usize, HashSet<usize>>;

use std::collections::{HashMap, HashSet, VecDeque};

/// Finalize objects in the correct order based on dependencies
pub fn finalize_objects_ordered(addresses: &[usize]) {
    // Get the global object storage
    let storage = crate::memory::global_object_storage();
    if let Ok(mut storage_lock) = storage.write() {
        // Build a map of object dependencies
        let mut dependencies = HashMap::new();
        
        for &addr in addresses {
            if storage_lock.contains(addr) {
                // Get the dependencies for this object
                // In a real implementation we would have direct access
                // For this minimal implementation, assume no dependencies
                dependencies.insert(addr, HashSet::new());
            }
        }
        
        // Calculate the order
        let order = calculate_finalization_order(&dependencies);
        
        // Remove objects in the correct order
        for addr in order {
            if addresses.contains(&addr) {
                storage_lock.remove(addr);
            }
        }
    }
}

/// Calculate the finalization order of objects based on dependencies
/// Returns a list of objects in the order they should be finalized
pub fn calculate_finalization_order(objects: &HashMap<usize, HashSet<usize>>) -> Vec<usize> {
    // Create a graph of dependencies (adjacency list)
    let mut graph: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut in_degree: HashMap<usize, usize> = HashMap::new();
    
    // Initialize the graph and in-degree count
    for (&id, deps) in objects.iter() {
        // Make sure all nodes are in the graph
        graph.entry(id).or_insert_with(Vec::new);
        in_degree.entry(id).or_insert(0);
        
        // Add dependencies
        for &dep in deps.iter() {
            graph.entry(dep).or_insert_with(Vec::new).push(id);
            *in_degree.entry(id).or_insert(0) += 1;
        }
    }
    
    // Topological sort using Kahn's algorithm
    let mut result = Vec::new();
    let mut queue = VecDeque::new();
    
    // Find all nodes with in-degree 0 (no dependencies)
    for (&id, &degree) in in_degree.iter() {
        if degree == 0 {
            queue.push_back(id);
        }
    }
    
    // Process the queue
    while let Some(id) = queue.pop_front() {
        result.push(id);
        
        // Decrease in-degree of adjacent nodes
        if let Some(neighbors) = graph.get(&id) {
            for &neighbor in neighbors.iter() {
                if let Some(degree) = in_degree.get_mut(&neighbor) {
                    *degree -= 1;
                    if *degree == 0 {
                        queue.push_back(neighbor);
                    }
                }
            }
        }
    }
    
    // Check for cycles
    if result.len() != objects.len() {
        // There's a cycle, so we need to handle it
        // For now, include remaining objects in any order
        for (&id, _) in objects.iter() {
            if !result.contains(&id) {
                result.push(id);
            }
        }
    }
    
    // We want objects without dependencies to be finalized first, so reverse the result
    result.reverse();
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_linear_dependencies() {
        // Create a linear dependency chain: 1 -> 2 -> 3
        let mut objects = HashMap::new();
        
        objects.insert(1, HashSet::new());
        
        let mut deps2 = HashSet::new();
        deps2.insert(1);
        objects.insert(2, deps2);
        
        let mut deps3 = HashSet::new();
        deps3.insert(2);
        objects.insert(3, deps3);
        
        let order = calculate_finalization_order(&objects);
        
        // Expected order: 1, 2, 3 (dependencies first)
        assert_eq!(order, vec![1, 2, 3]);
    }
    
    #[test]
    fn test_complex_dependencies() {
        // Create a more complex dependency graph
        let mut objects = HashMap::new();
        
        objects.insert(1, HashSet::new());
        objects.insert(2, HashSet::new());
        
        let mut deps3 = HashSet::new();
        deps3.insert(1);
        deps3.insert(2);
        objects.insert(3, deps3);
        
        let mut deps4 = HashSet::new();
        deps4.insert(2);
        deps4.insert(3);
        objects.insert(4, deps4);
        
        let order = calculate_finalization_order(&objects);
        
        // Check that dependencies come before dependents
        assert!(order.iter().position(|&x| x == 1).unwrap() < order.iter().position(|&x| x == 3).unwrap());
        assert!(order.iter().position(|&x| x == 2).unwrap() < order.iter().position(|&x| x == 3).unwrap());
        assert!(order.iter().position(|&x| x == 2).unwrap() < order.iter().position(|&x| x == 4).unwrap());
        assert!(order.iter().position(|&x| x == 3).unwrap() < order.iter().position(|&x| x == 4).unwrap());
    }
    
    #[test]
    fn test_cyclic_dependencies() {
        // Create a cycle: 1 -> 2 -> 3 -> 1
        let mut objects = HashMap::new();
        
        let mut deps1 = HashSet::new();
        deps1.insert(3);
        objects.insert(1, deps1);
        
        let mut deps2 = HashSet::new();
        deps2.insert(1);
        objects.insert(2, deps2);
        
        let mut deps3 = HashSet::new();
        deps3.insert(2);
        objects.insert(3, deps3);
        
        let order = calculate_finalization_order(&objects);
        
        // All objects should be included
        assert_eq!(order.len(), 3);
        assert!(order.contains(&1));
        assert!(order.contains(&2));
        assert!(order.contains(&3));
    }
}