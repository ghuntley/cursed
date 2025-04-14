//! Finalization ordering for proper dependency handling

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};

/// A graph for tracking finalization dependencies
pub struct FinalizationGraph {
    /// Map of object IDs to their dependencies
    dependencies: HashMap<usize, HashSet<usize>>,
    /// Map of object IDs to objects that depend on them
    dependents: HashMap<usize, HashSet<usize>>,
}

impl FinalizationGraph {
    /// Create a new empty finalization graph
    pub fn new() -> Self {
        Self {
            dependencies: HashMap::new(),
            dependents: HashMap::new(),
        }
    }
    
    /// Add a dependency relationship (dependent depends on dependency)
    pub fn add_dependency(&mut self, dependent: usize, dependency: usize) {
        // Add to dependencies map
        self.dependencies.entry(dependent).or_insert_with(HashSet::new).insert(dependency);
        
        // Add to dependents map
        self.dependents.entry(dependency).or_insert_with(HashSet::new).insert(dependent);
    }
    
    /// Remove an object from the graph
    pub fn remove_object(&mut self, id: usize) {
        // Remove from dependencies
        if let Some(deps) = self.dependencies.remove(&id) {
            // Update dependents maps
            for dep in deps {
                if let Some(deps) = self.dependents.get_mut(&dep) {
                    deps.remove(&id);
                }
            }
        }
        
        // Remove from dependents
        if let Some(deps) = self.dependents.remove(&id) {
            // Update dependencies maps
            for dep in deps {
                if let Some(deps) = self.dependencies.get_mut(&dep) {
                    deps.remove(&id);
                }
            }
        }
    }
    
    /// Get the finalization order for objects in the graph
    pub fn finalization_order(&self) -> Vec<usize> {
        let mut order = Vec::new();
        
        // Set of objects that have been visited
        let mut visited = HashSet::new();
        
        // Get all objects in the graph
        let mut all_objects = HashSet::new();
        for &id in self.dependencies.keys() {
            all_objects.insert(id);
        }
        for &id in self.dependents.keys() {
            all_objects.insert(id);
        }
        
        // Visit objects in dependency order
        for &id in &all_objects {
            self.visit(id, &mut visited, &mut order);
        }
        
        order
    }
    
    // DFS visit helper for topological sort
    fn visit(&self, id: usize, visited: &mut HashSet<usize>, order: &mut Vec<usize>) {
        // Skip if already visited
        if visited.contains(&id) {
            return;
        }
        
        // Mark as visited
        visited.insert(id);
        
        // Visit dependencies first
        if let Some(deps) = self.dependencies.get(&id) {
            for &dep in deps {
                self.visit(dep, visited, order);
            }
        }
        
        // Add to order
        order.push(id);
    }
}

/// Global finalization graph
lazy_static::lazy_static! {
    static ref FINALIZATION_GRAPH: Arc<Mutex<FinalizationGraph>> = 
        Arc::new(Mutex::new(FinalizationGraph::new()));
}

/// Register a dependency between two objects
pub fn register_dependency(dependent: usize, dependency: usize) {
    let mut graph = FINALIZATION_GRAPH.lock().unwrap();
    graph.add_dependency(dependent, dependency);
}

/// Finalize objects in the correct order
pub fn finalize_objects_ordered(objects: &[usize]) {
    let graph = FINALIZATION_GRAPH.lock().unwrap();
    
    // Get the finalization order for these objects
    let mut objects_set: HashSet<usize> = HashSet::new();
    objects_set.extend(objects);
    
    let mut order = Vec::new();
    
    // Set of objects that have been visited
    let mut visited = HashSet::new();
    
    // Visit objects in dependency order
    for &id in objects {
        graph.visit(id, &mut visited, &mut order);
    }
    
    // Now call finalize on each object in the order
    for id in order {
        // Here we would call finalize on the object
        // global_object_storage().get_and_finalize(id);
    }
}