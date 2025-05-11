//! Improved Finalization Order for Circular References
//!
//! This module implements an improved approach to object finalization
//! that properly handles circular references. It ensures objects are
//! finalized in the correct order even when there are cycles in the
//! object graph.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use tracing::{debug, error, info, trace, warn, instrument};

use crate::memory::{Gc, Tag, Traceable, Visitor};
use crate::memory::cycle_detector::ReferenceCollector;

/// Represents a node in the finalization dependency graph
#[derive(Debug)]
struct FinalizationNode {
    /// The object ID
    id: usize,
    /// Objects that depend on this object
    dependents: HashSet<usize>,
    /// Objects this object depends on
    dependencies: HashSet<usize>,
    /// Whether this object has been finalized
    finalized: bool,
}

/// Manages the finalization order of objects with circular references
#[derive(Debug, Default)]
pub struct FinalizationOrderManager {
    /// Maps object IDs to their finalization nodes
    nodes: HashMap<usize, FinalizationNode>,
    /// Objects ready for finalization (no remaining dependencies)
    ready_queue: VecDeque<usize>,
}

impl FinalizationOrderManager {
    /// Create a new finalization order manager
    pub fn new() -> Self {
        Self {
            nodes: HashMap::new(),
            ready_queue: VecDeque::new(),
        }
    }
    
    /// Add an object to the finalization dependency graph
    #[instrument(skip(self), fields(obj_id = format!("{:#x}", obj_id)))]
    pub fn add_object(&mut self, obj_id: usize) {
        trace!(?obj_id, "Adding object to finalization graph");
        
        // Skip if the object is already in the graph
        if self.nodes.contains_key(&obj_id) {
            trace!(?obj_id, "Object already in graph");
            return;
        }
        
        // Create a new node for this object
        let node = FinalizationNode {
            id: obj_id,
            dependents: HashSet::new(),
            dependencies: HashSet::new(),
            finalized: false,
        };
        
        self.nodes.insert(obj_id, node);
        trace!(?obj_id, "Added object to graph");
    }
    
    /// Add a dependency relationship between two objects
    #[instrument(skip(self), fields(from = format!("{:#x}", from), to = format!("{:#x}", to)))]
    pub fn add_dependency(&mut self, from: usize, to: usize) {
        // Skip self-dependencies
        if from == to {
            trace!("Skipping self-dependency");
            return;
        }
        
        // Make sure both objects are in the graph
        self.add_object(from);
        self.add_object(to);
        
        // Add the dependency
        if let Some(from_node) = self.nodes.get_mut(&from) {
            let is_new = from_node.dependencies.insert(to);
            if is_new {
                trace!("Added new dependency");
            }
        }
        
        // Add the dependent relationship
        if let Some(to_node) = self.nodes.get_mut(&to) {
            let is_new = to_node.dependents.insert(from);
            if is_new {
                trace!("Added new dependent");
            }
        }
    }
    
    /// Build the finalization graph from a set of objects
    #[instrument(skip(self, objects), fields(count = objects.len()))]
    pub fn build_graph(&mut self, objects: &[usize]) {
        info!(count = objects.len(), "Building finalization graph");
        
        // First, add all objects to the graph
        for &obj_id in objects {
            self.add_object(obj_id);
        }
        
        // Then, collect references for each object
        for &obj_id in objects {
            let references = ReferenceCollector::collect_references(obj_id);
            trace!(obj_id = format!("{:#x}", obj_id), refs_count = references.len(), "Found references");
            
            // Add dependencies for references (only for objects in our set)
            for &ref_id in &references {
                if objects.contains(&ref_id) {
                    // obj_id depends on ref_id
                    self.add_dependency(obj_id, ref_id);
                }
            }
        }
        
        // Initialize the ready queue with objects that have no dependencies
        for &obj_id in objects {
            if let Some(node) = self.nodes.get(&obj_id) {
                if node.dependencies.is_empty() {
                    trace!(obj_id = format!("{:#x}", obj_id), "Object has no dependencies, adding to ready queue");
                    self.ready_queue.push_back(obj_id);
                } else {
                    trace!(obj_id = format!("{:#x}", obj_id), deps = node.dependencies.len(), "Object has dependencies");
                }
            }
        }
        
        info!(ready_count = self.ready_queue.len(), "Ready queue initialized");
    }
    
    /// Get the next object ready for finalization
    pub fn next_ready(&mut self) -> Option<usize> {
        self.ready_queue.pop_front()
    }
    
    /// Mark an object as finalized and update the dependency graph
    #[instrument(skip(self), fields(obj_id = format!("{:#x}", obj_id)))]
    pub fn mark_finalized(&mut self, obj_id: usize) {
        trace!("Marking object as finalized");
        
        // Mark the object as finalized
        if let Some(node) = self.nodes.get_mut(&obj_id) {
            node.finalized = true;
            
            // Get the dependents to update
            let dependents = node.dependents.clone();
            trace!(dependents_count = dependents.len(), "Found dependents to update");
            
            // Update each dependent
            for &dep_id in &dependents {
                self.update_dependent(dep_id, obj_id);
            }
        }
    }
    
    /// Update a dependent after its dependency has been finalized
    #[instrument(skip(self), fields(dep_id = format!("{:#x}", dep_id), dependency_id = format!("{:#x}", dependency_id)))]
    fn update_dependent(&mut self, dep_id: usize, dependency_id: usize) {
        trace!("Updating dependent");
        
        if let Some(node) = self.nodes.get_mut(&dep_id) {
            // Remove the finalized dependency
            node.dependencies.remove(&dependency_id);
            
            // If no more dependencies, add to ready queue
            if node.dependencies.is_empty() && !node.finalized {
                trace!("Dependent has no more dependencies, adding to ready queue");
                self.ready_queue.push_back(dep_id);
            } else {
                trace!(deps_remaining = node.dependencies.len(), "Dependent still has dependencies");
            }
        }
    }
    
    /// Handle circular references by breaking cycles
    #[instrument(skip(self))]
    pub fn handle_circular_references(&mut self) {
        // If the ready queue is empty but we still have unfinalized objects,
        // we have circular references
        if self.ready_queue.is_empty() {
            let unfinalized = self.nodes.iter()
                .filter(|(_, node)| !node.finalized)
                .map(|(id, _)| *id)
                .collect::<Vec<_>>();
                
            if !unfinalized.is_empty() {
                warn!(count = unfinalized.len(), "Detected circular references");
                
                // Break cycles by adding the first unfinalized object to ready queue
                if let Some(&first) = unfinalized.first() {
                    warn!(obj_id = format!("{:#x}", first), "Breaking cycle by adding to ready queue");
                    self.ready_queue.push_back(first);
                }
            }
        }
    }
    
    /// Finalize all objects in the correct order
    #[instrument(skip(self, finalizer), fields(objects_count = self.nodes.len()))]
    pub fn finalize_all<F>(&mut self, mut finalizer: F) 
    where
        F: FnMut(usize)
    {
        info!(objects_count = self.nodes.len(), "Finalizing all objects");
        
        // Process objects until we're done
        let mut processed_count = 0;
        
        while let Some(obj_id) = self.next_ready() {
            // Finalize the object
            trace!(obj_id = format!("{:#x}", obj_id), "Finalizing object");
            finalizer(obj_id);
            processed_count += 1;
            
            // Mark it as finalized in our graph
            self.mark_finalized(obj_id);
            
            // If the ready queue is empty, handle circular references
            if self.ready_queue.is_empty() {
                self.handle_circular_references();
            }
        }
        
        info!(processed_count, "Finalization complete");
    }
}

/// Finalize a set of objects in the correct order, handling circular references
#[instrument(skip(objects), fields(count = objects.len()))]
pub fn finalize_objects_in_order(objects: &[usize]) {
    info!(count = objects.len(), "Finalizing objects in order");
    
    // Create a finalization manager
    let mut manager = FinalizationOrderManager::new();
    
    // Build the dependency graph
    manager.build_graph(objects);
    
    // Finalize objects in the correct order
    manager.finalize_all(|obj_id| {
        // Get the object from global storage and finalize it
        let storage = crate::memory::object_storage::global_object_storage();
        if let Ok(storage_lock) = storage.read() {
            if let Some(obj_box) = storage_lock.get_dyn_traceable(obj_id) {
                trace!(obj_id = format!("{:#x}", obj_id), "Finalizing object");
                
                // Call finalize on the object
                unsafe {
                    let obj = &mut *(obj_box.as_ptr() as *mut dyn Traceable);
                    obj.finalize();
                }
            }
        }
    });
}

/// Global function to finalize objects in order
pub fn finalize_objects_ordered(objects: &[usize]) {
    finalize_objects_in_order(objects);
}