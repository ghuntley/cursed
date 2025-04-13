//! Finalization ordering for dependencies between objects
//!
//! This module enhances the GC with support for finalization ordering,
//! ensuring that objects are finalized in the correct order based on their
//! dependencies, preventing invalid memory access during finalization.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, RwLock, Mutex};
use std::time::Duration;

use crate::memory::object_storage::global_object_storage;

/// A dependency graph for object finalization
#[derive(Debug, Default)]
pub struct FinalizationGraph {
    /// Maps objects to objects they depend on
    dependencies: HashMap<usize, Vec<usize>>,
    /// Maps objects to objects that depend on them
    dependents: HashMap<usize, Vec<usize>>,
}

impl FinalizationGraph {
    /// Create a new empty finalization graph
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            dependents: HashMap::new(),
        }
    }
    
    /// Add a dependency - obj_a depends on obj_b
    pub fn add_dependency(&mut self, obj_a: usize, obj_b: usize) {
        // obj_a depends on obj_b, so obj_b must be finalized after obj_a
        self.dependencies.entry(obj_a).or_insert_with(Vec::new).push(obj_b);
        self.dependents.entry(obj_b).or_insert_with(Vec::new).push(obj_a);
    }
    
    /// Remove an object from the graph
    pub fn remove_object(&mut self, obj: usize) {
        // Remove from dependencies
        if let Some(deps) = self.dependencies.remove(&obj) {
            // Remove this object from the dependents lists of its dependencies
            for dep in deps {
                if let Some(deps_list) = self.dependents.get_mut(&dep) {
                    deps_list.retain(|&x| x != obj);
                }
            }
        }
        
        // Remove from dependents
        if let Some(deps) = self.dependents.remove(&obj) {
            // Remove this object from the dependencies lists of its dependents
            for dep in deps {
                if let Some(deps_list) = self.dependencies.get_mut(&dep) {
                    deps_list.retain(|&x| x != obj);
                }
            }
        }
    }
    
    /// Get objects that have no dependencies (can be finalized first)
    pub fn get_roots(&self) -> Vec<usize> {
        let mut roots = Vec::new();
        
        for &obj in self.dependencies.keys() {
            // If object has no dependencies or only dependencies on itself
            if !self.dependents.contains_key(&obj) || 
               (self.dependents[&obj].len() == 1 && self.dependents[&obj][0] == obj) {
                roots.push(obj);
            }
        }
        
        // Also include objects that are only dependents
        for &obj in self.dependents.keys() {
            if !self.dependencies.contains_key(&obj) {
                roots.push(obj);
            }
        }
        
        roots
    }
    
    /// Get a topological ordering for finalization
    /// Returns objects in the order they should be finalized
    pub fn finalization_order(&self) -> Vec<usize> {
        let mut result = Vec::new();
        let mut visited = HashSet::new();
        let mut temp_visited = HashSet::new();
        
        // Helper function for depth-first search
        fn visit(
            obj: usize,
            graph: &FinalizationGraph,
            visited: &mut HashSet<usize>,
            temp_visited: &mut HashSet<usize>,
            result: &mut Vec<usize>
        ) -> bool {
            // If we've already fully processed this node
            if visited.contains(&obj) {
                return true;
            }
            
            // If we're currently visiting this node (cycle detected)
            if temp_visited.contains(&obj) {
                // Handle cycle: just continue, in a real implementation 
                // we might want better cycle handling
                return true;
            }
            
            // Mark as being visited
            temp_visited.insert(obj);
            
            // Visit all dependencies first
            if let Some(deps) = graph.dependencies.get(&obj) {
                for &dep in deps {
                    if !visit(dep, graph, visited, temp_visited, result) {
                        return false; // Propagate error
                    }
                }
            }
            
            // Mark as visited and add to result
            temp_visited.remove(&obj);
            visited.insert(obj);
            result.push(obj);
            
            true
        }
        
        // Start with objects that have no dependencies
        let roots = self.get_roots();
        for root in roots {
            if !visit(root, self, &mut visited, &mut temp_visited, &mut result) {
                // If there was a problem, return empty list
                return Vec::new();
            }
        }
        
        // Add any remaining objects (in case of disconnected graph components)
        let mut all_objects = HashSet::new();
        for &obj in self.dependencies.keys() {
            all_objects.insert(obj);
        }
        for &obj in self.dependents.keys() {
            all_objects.insert(obj);
        }
        
        for obj in all_objects {
            if !visited.contains(&obj) {
                if !visit(obj, self, &mut visited, &mut temp_visited, &mut result) {
                    // If there was a problem, return what we have so far
                    return result;
                }
            }
        }
        
        // Reverse the result because we want to finalize in reverse topological order
        // (dependents before dependencies)
        result.reverse();
        result
    }
}

/// Get the global finalization graph
pub fn finalization_graph() -> &'static RwLock<FinalizationGraph> {
    static GRAPH: once_cell::sync::Lazy<RwLock<FinalizationGraph>> = 
        once_cell::sync::Lazy::new(|| RwLock::new(FinalizationGraph::new()));
    &GRAPH
}

/// Register a finalization dependency - obj_a depends on obj_b
pub fn register_dependency(obj_a: usize, obj_b: usize) {
    if let Ok(mut graph) = finalization_graph().write() {
        graph.add_dependency(obj_a, obj_b);
    }
}

/// Finalize a set of objects in the proper order
pub fn finalize_objects_ordered(objects: &[usize]) -> Vec<usize> {
    let mut finalized = Vec::new();
    
    // Get the finalization order from the graph
    let order = {
        if let Ok(graph) = finalization_graph().read() {
            // Filter to only include objects we're finalizing
            let mut ordered = graph.finalization_order();
            ordered.retain(|obj| objects.contains(obj));
            ordered
        } else {
            // If we can't get the lock, just use the original order
            objects.to_vec()
        }
    };
    
    // Now finalize in the determined order
    let storage = global_object_storage();
    for &obj in &order {
        if storage.remove_and_finalize(obj) {
            finalized.push(obj);
        }
    }
    
    // Clean up the graph
    if let Ok(mut graph) = finalization_graph().write() {
        for &obj in &finalized {
            graph.remove_object(obj);
        }
    }
    
    finalized
}