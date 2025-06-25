/// Cycle Detection for Garbage Collection
/// 
/// This module implements sophisticated algorithms for detecting and collecting
/// circular references in the object graph. It includes both incremental and
/// batch cycle detection strategies.

use std::sync::{Arc, RwLock, Mutex};
use std::collections::{HashMap, HashSet, VecDeque};
use tracing::{instrument, debug, info, warn};

use crate::memory::{Traceable, Visitor};
use crate::memory::object_id::{ObjectId, ObjectRegistry, SharedObjectRegistry};
use crate::error::CursedError;

/// Different cycle detection algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CycleDetectionAlgorithm {
    /// Bacon-Rajan cycle collection algorithm
    /// Trial deletion algorithm
    /// Brownbridge algorithm (incremental)
    /// Hybrid approach combining multiple algorithms
/// Color states for cycle detection algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    /// Object has not been visited
    /// Object is in the potential cycle set
    /// Object is confirmed reachable
    /// Object is confirmed to be in a cycle
    /// Object reference count is being decremented
    /// Object is being freed
/// Information about a detected cycle
#[derive(Debug, Clone)]
pub struct CycleInfo {
    /// Objects in the cycle
    /// Size of the cycle in objects
    /// Total memory size of the cycle
    /// When the cycle was detected
    /// Which algorithm detected the cycle
/// Configuration for cycle detection
#[derive(Debug, Clone)]
pub struct CycleDetectionConfig {
    /// Primary algorithm to use
    /// Maximum cycle size to detect (prevents infinite loops)
    /// Enable incremental cycle detection
    /// Cycle detection frequency (every N allocations)
    /// Enable hybrid algorithm combining multiple approaches
    /// Minimum object age before considering for cycle detection
impl Default for CycleDetectionConfig {
    fn default() -> Self {
        Self {
        }
    }
/// Statistics about cycle detection
#[derive(Debug, Clone, Default)]
pub struct CycleDetectionStats {
/// Reference information for cycle detection
#[derive(Debug, Clone)]
struct ObjectReference {
/// Types of object references
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReferenceType {
    /// Strong reference (normal pointer)
    /// Weak reference (doesn't prevent collection)
    /// Root reference (from GC roots)
/// Cycle detector implementation
pub struct CycleDetector {
impl CycleDetector {
    /// Create a new cycle detector
    pub fn new(object_registry: SharedObjectRegistry) -> Self {
        Self::with_config(object_registry, CycleDetectionConfig::default())
    /// Create a new cycle detector with custom configuration
    #[instrument(skip(object_registry, config))]
    pub fn with_config(object_registry: SharedObjectRegistry, config: CycleDetectionConfig) -> Self {
        info!("Creating cycle detector with config: {:?}", config);
        
        Self {
        }
    }
    
    /// Update reference graph when a new reference is created
    #[instrument(skip(self))]
    pub fn add_reference(&self, from_object: ObjectId, to_object: ObjectId, reference_type: ReferenceType) -> Result<(), String> {
        debug!("Adding reference: {} -> {} ({:?})", from_object, to_object, reference_type);
        
        let mut reference_graph = self.reference_graph.write()
            .map_err(|_| "Failed to acquire write lock on reference graph")?;
        
        let reference = ObjectReference {
        
        reference_graph.entry(from_object).or_insert_with(Vec::new).push(reference);
        
        // Check if we should trigger incremental cycle detection
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        if config.incremental {
            let count = self.detection_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            if count % config.detection_frequency == 0 {
                // Schedule incremental detection
                self.mark_for_incremental_detection(from_object)?;
            }
        }
        
        Ok(())
    /// Remove a reference from the graph
    #[instrument(skip(self))]
    pub fn remove_reference(&self, from_object: ObjectId, to_object: ObjectId) -> Result<(), String> {
        debug!("Removing reference: {} -> {}", from_object, to_object);
        
        let mut reference_graph = self.reference_graph.write()
            .map_err(|_| "Failed to acquire write lock on reference graph")?;
        
        if let Some(references) = reference_graph.get_mut(&from_object) {
            references.retain(|ref_info| ref_info.to_object != to_object);
            if references.is_empty() {
                reference_graph.remove(&from_object);
            }
        }
        
        Ok(())
    /// Detect cycles in the object graph
    #[instrument(skip(self))]
    pub fn detect_cycles(&self) -> Result<Vec<CycleInfo>, String> {
        info!("Starting cycle detection");
        let start_time = std::time::Instant::now();
        
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        let cycles = match config.algorithm {
        
        let detection_time = start_time.elapsed();
        
        // Update statistics
        {
            let mut stats = self.stats.write()
                .map_err(|_| "Failed to acquire write lock on stats")?;
            
            stats.detection_runs += 1;
            stats.cycles_detected += cycles.len() as u64;
            stats.detection_time_total += detection_time;
            *stats.cycles_by_algorithm.entry(config.algorithm).or_insert(0) += cycles.len() as u64;
            
            for cycle in &cycles {
                stats.objects_in_cycles += cycle.size as u64;
                if cycle.size > stats.largest_cycle_size {
                    stats.largest_cycle_size = cycle.size;
                }
            }
        info!("Cycle detection completed: found {} cycles in {:?}", cycles.len(), detection_time);
        Ok(cycles)
    /// Bacon-Rajan cycle collection algorithm
    fn bacon_rajan_detection(&self) -> Result<Vec<CycleInfo>, String> {
        debug!("Running Bacon-Rajan cycle detection");
        
        let mut cycles = Vec::new();
        let reference_graph = self.reference_graph.read()
            .map_err(|_| "Failed to acquire read lock on reference graph")?;
        
        // Phase 1: Mark potential cycle roots
        let mut potential_roots = HashSet::new();
        for (&object_id, references) in reference_graph.iter() {
            if self.could_be_cycle_root(object_id, references)? {
                potential_roots.insert(object_id);
            }
        }
        
        // Phase 2: For each potential root, try to find cycles
        for &root in &potential_roots {
            if let Some(cycle) = self.find_cycle_from_root(root, &reference_graph)? {
                cycles.push(cycle);
            }
        }
        
        Ok(cycles)
    /// Trial deletion cycle detection algorithm
    fn trial_deletion_detection(&self) -> Result<Vec<CycleInfo>, String> {
        debug!("Running trial deletion cycle detection");
        
        let mut cycles = Vec::new();
        let reference_graph = self.reference_graph.read()
            .map_err(|_| "Failed to acquire read lock on reference graph")?;
        
        // Create a copy of the reference graph for trial deletion
        let mut trial_graph = reference_graph.clone();
        let mut visited = HashSet::new();
        
        for &object_id in reference_graph.keys() {
            if visited.contains(&object_id) {
                continue;
            // Try deleting this object and see if a cycle becomes unreachable
            if let Some(cycle) = self.trial_delete_object(object_id, &mut trial_graph, &mut visited)? {
                cycles.push(cycle);
            }
        }
        
        Ok(cycles)
    /// Brownbridge incremental cycle detection
    fn brownbridge_detection(&self) -> Result<Vec<CycleInfo>, String> {
        debug!("Running Brownbridge incremental cycle detection");
        
        // Process queued potential cycles from incremental detection
        let mut cycles = Vec::new();
        let mut potential_cycles = self.potential_cycles.lock()
            .map_err(|_| "Failed to acquire lock on potential cycles")?;
        
        while let Some(potential_cycle) = potential_cycles.pop_front() {
            if let Some(cycle) = self.verify_potential_cycle(potential_cycle)? {
                cycles.push(cycle);
            }
        }
        
        Ok(cycles)
    /// Hybrid detection using multiple algorithms
    fn hybrid_detection(&self) -> Result<Vec<CycleInfo>, String> {
        debug!("Running hybrid cycle detection");
        
        let mut all_cycles = Vec::new();
        let mut found_cycle_objects = HashSet::new();
        
        // Run Bacon-Rajan first (good for finding large cycles)
        let bacon_cycles = self.bacon_rajan_detection()?;
        for cycle in bacon_cycles {
            for &obj in &cycle.objects {
                found_cycle_objects.insert(obj);
            }
            all_cycles.push(cycle);
        // Run incremental detection for recently changed objects
        let brownbridge_cycles = self.brownbridge_detection()?;
        for cycle in brownbridge_cycles {
            // Only add if we haven't already found this cycle
            if !cycle.objects.iter().any(|obj| found_cycle_objects.contains(obj)) {
                for &obj in &cycle.objects {
                    found_cycle_objects.insert(obj);
                }
                all_cycles.push(cycle);
            }
        }
        
        Ok(all_cycles)
    /// Check if an object could be a cycle root
    fn could_be_cycle_root(&self, object_id: ObjectId, references: &[ObjectReference]) -> Result<bool, String> {
        // An object could be a cycle root if:
        // 1. It has outgoing references
        // 2. It's not directly reachable from GC roots (simplified check)
        // 3. It has been allocated for minimum age
        
        if references.is_empty() {
            return Ok(false);
        // Check object age
        if let Ok(Some(metadata)) = self.object_registry.get(object_id) {
            let config = self.config.read()
                .map_err(|_| "Failed to acquire read lock on config")?;
            
            if metadata.created_at().elapsed() < config.min_object_age {
                return Ok(false);
            }
        }
        
        // Check if object is directly reachable from roots
        let root_objects = self.object_registry.get_root_objects()?;
        
        // If it's a root object itself, it's reachable
        if root_objects.contains(&object_id) {
            return Ok(false); // Root objects can't be in unreferenced cycles
        // For now, we assume non-root objects could potentially be in cycles
        // A more sophisticated implementation would do a full reachability analysis
        Ok(true)
    /// Find a cycle starting from a specific root object
    fn find_cycle_from_root(&self, root: ObjectId, reference_graph: &HashMap<ObjectId, Vec<ObjectReference>>) -> Result<Option<CycleInfo>, String> {
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        
        if self.dfs_find_cycle(root, root, reference_graph, &mut visited, &mut path)? {
            // Found a cycle, calculate its properties
            let memory_size = self.calculate_cycle_memory_size(&path)?;
            let cycle_size = path.len();
            
            Ok(Some(CycleInfo {
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Depth-first search to find cycles
    fn dfs_find_cycle(
    ) -> Result<bool, String> {
        if path.len() > 0 && current == target {
            return Ok(true); // Found cycle back to target
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        if path.len() >= config.max_cycle_size {
            return Ok(false); // Prevent infinite loops
        if visited.contains(&current) {
            return Ok(false); // Already explored this path
        visited.insert(current);
        path.push(current);
        
        // Explore outgoing references
        if let Some(references) = reference_graph.get(&current) {
            for reference in references {
                if reference.reference_type == ReferenceType::Strong {
                    if self.dfs_find_cycle(reference.to_object, target, reference_graph, visited, path)? {
                        return Ok(true);
                    }
                }
            }
        }
        
        path.pop();
        Ok(false)
    /// Trial delete an object to see if a cycle becomes unreachable
    fn trial_delete_object(
    ) -> Result<Option<CycleInfo>, String> {
        // Remove all references from this object
        let removed_refs = trial_graph.remove(&object_id).unwrap_or_default();
        
        // Check if any of the referenced objects are now unreachable
        for reference in &removed_refs {
            if reference.reference_type == ReferenceType::Strong {
                if let Some(cycle) = self.check_unreachable_cycle(reference.to_object, trial_graph, visited)? {
                    // Restore the references before returning
                    trial_graph.insert(object_id, removed_refs);
                    return Ok(Some(cycle));
                }
            }
        // Restore the references
        trial_graph.insert(object_id, removed_refs);
        Ok(None)
    /// Check if removing an object creates an unreachable cycle
    fn check_unreachable_cycle(
    ) -> Result<Option<CycleInfo>, String> {
        if visited.contains(&start_object) {
            return Ok(None);
        // Perform reachability analysis from GC roots
        let reachable_objects = self.analyze_reachability_from_roots(trial_graph)?;
        
        // If start_object is not reachable from roots, check if it's part of a cycle
        if !reachable_objects.contains(&start_object) {
            if let Some(cycle) = self.find_unreachable_cycle(start_object, trial_graph, visited)? {
                return Ok(Some(cycle));
            }
        }
        
        Ok(None)
    /// Mark an object for incremental cycle detection
    fn mark_for_incremental_detection(&self, object_id: ObjectId) -> Result<(), String> {
        debug!("Marking object {} for incremental detection", object_id);
        
        // Set object color to purple (potential cycle candidate)
        {
            let mut object_colors = self.object_colors.write()
                .map_err(|_| "Failed to acquire write lock on object colors")?;
            object_colors.insert(object_id, Color::Purple);
        // Add to potential cycles queue for later processing
        let potential_cycle = self.find_potential_cycle_from_object(object_id)?;
        if !potential_cycle.is_empty() {
            let mut potential_cycles = self.potential_cycles.lock()
                .map_err(|_| "Failed to acquire lock on potential cycles")?;
            potential_cycles.push_back(potential_cycle);
        Ok(())
    /// Verify a potential cycle found by incremental detection
    fn verify_potential_cycle(&self, potential_cycle: Vec<ObjectId>) -> Result<Option<CycleInfo>, String> {
        debug!("Verifying potential cycle with {} objects", potential_cycle.len());
        
        if potential_cycle.is_empty() {
            return Ok(None);
        // Check if all objects in the potential cycle still exist
        let mut verified_objects = Vec::new();
        for &object_id in &potential_cycle {
            if self.object_registry.get(object_id).is_ok() {
                verified_objects.push(object_id);
            }
        }
        
        if verified_objects.len() < 2 {
            return Ok(None); // Need at least 2 objects for a cycle
        // Verify the cycle structure still exists
        let reference_graph = self.reference_graph.read()
            .map_err(|_| "Failed to acquire read lock on reference graph")?;
        
        if self.verify_cycle_structure(&verified_objects, &reference_graph)? {
            let memory_size = self.calculate_cycle_memory_size(&verified_objects)?;
            
            Ok(Some(CycleInfo {
            }))
        } else {
            // Update statistics for false positive
            {
                let mut stats = self.stats.write()
                    .map_err(|_| "Failed to acquire write lock on stats")?;
                stats.false_positives += 1;
            }
            Ok(None)
        }
    }
    
    /// Calculate the total memory size of objects in a cycle
    fn calculate_cycle_memory_size(&self, cycle_objects: &[ObjectId]) -> Result<usize, String> {
        let mut total_size = 0;
        
        for &object_id in cycle_objects {
            if let Ok(Some(metadata)) = self.object_registry.get(object_id) {
                total_size += metadata.size();
            }
        }
        
        Ok(total_size)
    /// Collect objects in detected cycles
    #[instrument(skip(self, cycles))]
    pub fn collect_cycles(&self, cycles: &[CycleInfo]) -> Result<usize, String> {
        info!("Collecting {} detected cycles", cycles.len());
        
        let mut collected_objects = 0;
        
        for cycle in cycles {
            debug!("Collecting cycle with {} objects", cycle.size);
            
            // Actually collect the objects in the cycle
            for &object_id in &cycle.objects {
                match self.object_registry.unregister(object_id) {
                    Ok(Some(metadata)) => {
                        debug!("Collected cyclic object {} (size: {} bytes)", object_id, metadata.size());
                        collected_objects += 1;
                        
                        // Update statistics
                        {
                            let mut stats = self.stats.write()
                                .map_err(|_| "Failed to acquire write lock on cycle detection stats")?;
                            stats.cycles_collected += 1;
                            stats.objects_in_cycles += 1;
                        }
                    }
                    Ok(None) => {
                        debug!("Object {} was already collected", object_id);
                    }
                    Err(e) => {
                        warn!("Failed to collect cyclic object {}: {}", object_id, e);
                        // Continue with other objects in the cycle
                    }
                }
            }
        }
        
        // Update statistics
        {
            let mut stats = self.stats.write()
                .map_err(|_| "Failed to acquire write lock on stats")?;
            stats.cycles_collected += cycles.len() as u64;
        info!("Collected {} objects from cycles", collected_objects);
        Ok(collected_objects)
    /// Get cycle detection statistics
    pub fn get_stats(&self) -> Result<CycleDetectionStats, String> {
        let stats = self.stats.read()
            .map_err(|_| "Failed to acquire read lock on stats")?;
        Ok(stats.clone())
    /// Update configuration
    pub fn update_config(&self, new_config: CycleDetectionConfig) -> Result<(), String> {
        let mut config = self.config.write()
            .map_err(|_| "Failed to acquire write lock on config")?;
        *config = new_config;
        info!("Updated cycle detection configuration");
        Ok(())
    /// Clear the reference graph (for testing or reset)
    pub fn clear_reference_graph(&self) -> Result<(), String> {
        let mut reference_graph = self.reference_graph.write()
            .map_err(|_| "Failed to acquire write lock on reference graph")?;
        reference_graph.clear();
        
        let mut object_colors = self.object_colors.write()
            .map_err(|_| "Failed to acquire write lock on object colors")?;
        object_colors.clear();
        
        info!("Cleared cycle detection reference graph");
        Ok(())
    /// Analyze reachability from GC roots
    fn analyze_reachability_from_roots(&self, graph: &HashMap<ObjectId, Vec<ObjectReference>>) -> Result<HashSet<ObjectId>, String> {
        let mut reachable = HashSet::new();
        let mut worklist = VecDeque::new();
        
        // Get GC roots from object registry
        let roots = self.get_gc_roots()?;
        for root in roots {
            worklist.push_back(root);
            reachable.insert(root);
        // Breadth-first traversal from roots
        while let Some(current) = worklist.pop_front() {
            if let Some(references) = graph.get(&current) {
                for reference in references {
                    // Only follow strong references for reachability
                    if reference.reference_type == ReferenceType::Strong || 
                       reference.reference_type == ReferenceType::Root {
                        if !reachable.contains(&reference.to_object) {
                            reachable.insert(reference.to_object);
                            worklist.push_back(reference.to_object);
                        }
                    }
                }
            }
        Ok(reachable)
    /// Find an unreachable cycle starting from a given object
    fn find_unreachable_cycle(
    ) -> Result<Option<CycleInfo>, String> {
        let mut cycle_path = Vec::new();
        let mut path_set = HashSet::new();
        
        if self.dfs_find_unreachable_cycle(start_object, graph, visited, &mut cycle_path, &mut path_set)? {
            let memory_size = self.calculate_cycle_memory_size(&cycle_path)?;
            
            Ok(Some(CycleInfo {
            }))
        } else {
            Ok(None)
        }
    }
    
    /// DFS to find unreachable cycles
    fn dfs_find_unreachable_cycle(
    ) -> Result<bool, String> {
        if path_set.contains(&current) {
            // Found a cycle - extract the cycle portion
            if let Some(cycle_start) = cycle_path.iter().position(|&obj| obj == current) {
                cycle_path.drain(0..cycle_start);
                return Ok(true);
            }
        }
        
        if visited.contains(&current) {
            return Ok(false);
        visited.insert(current);
        cycle_path.push(current);
        path_set.insert(current);
        
        // Explore strong references only
        if let Some(references) = graph.get(&current) {
            for reference in references {
                if reference.reference_type == ReferenceType::Strong {
                    if self.dfs_find_unreachable_cycle(
                    )? {
                        return Ok(true);
                    }
                }
            }
        }
        
        cycle_path.pop();
        path_set.remove(&current);
        Ok(false)
    /// Find potential cycle objects starting from a given object
    fn find_potential_cycle_from_object(&self, object_id: ObjectId) -> Result<Vec<ObjectId>, String> {
        let mut potential_cycle = Vec::new();
        let mut visited = HashSet::new();
        
        let reference_graph = self.reference_graph.read()
            .map_err(|_| "Failed to acquire read lock on reference graph")?;
        
        self.collect_strongly_connected_component(
        )?;
        
        Ok(potential_cycle)
    /// Collect objects in the same strongly connected component
    fn collect_strongly_connected_component(
    ) -> Result<(), String> {
        if visited.contains(&start) {
            return Ok(());
        visited.insert(start);
        component.push(start);
        
        // Follow all strong references
        if let Some(references) = graph.get(&start) {
            for reference in references {
                if reference.reference_type == ReferenceType::Strong {
                    self.collect_strongly_connected_component(
                    )?;
                }
            }
        Ok(())
    /// Verify that objects form a valid cycle structure
    fn verify_cycle_structure(
    ) -> Result<bool, String> {
        if objects.len() < 2 {
            return Ok(false);
        // Check that each object has a path to the next in the cycle
        for i in 0..objects.len() {
            let current = objects[i];
            let next = objects[(i + 1) % objects.len()];
            
            if !self.has_path_between(current, next, graph)? {
                return Ok(false);
            }
        }
        
        Ok(true)
    /// Check if there's a path between two objects
    fn has_path_between(
    ) -> Result<bool, String> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        queue.push_back(from);
        visited.insert(from);
        
        while let Some(current) = queue.pop_front() {
            if current == to {
                return Ok(true);
            if let Some(references) = graph.get(&current) {
                for reference in references {
                    if reference.reference_type == ReferenceType::Strong {
                        if !visited.contains(&reference.to_object) {
                            visited.insert(reference.to_object);
                            queue.push_back(reference.to_object);
                        }
                    }
                }
            }
        Ok(false)
    /// Get GC roots for reachability analysis
    fn get_gc_roots(&self) -> Result<Vec<ObjectId>, String> {
        // In a real implementation, this would get actual GC roots
        // For now, return objects with root references
        let mut roots = Vec::new();
        let reference_graph = self.reference_graph.read()
            .map_err(|_| "Failed to acquire read lock on reference graph")?;
        
        for (_, references) in reference_graph.iter() {
            for reference in references {
                if reference.reference_type == ReferenceType::Root {
                    roots.push(reference.to_object);
                }
            }
        Ok(roots)
    /// Optimize reference counting for cycle detection
    pub fn optimize_reference_counting(&self) -> Result<(), String> {
        info!("Optimizing reference counting for cycle detection");
        
        let reference_graph = self.reference_graph.read()
            .map_err(|_| "Failed to acquire read lock on reference graph")?;
        
        // Build reference count map
        let mut ref_counts = HashMap::new();
        for (from_object, references) in reference_graph.iter() {
            for reference in references {
                if reference.reference_type == ReferenceType::Strong {
                    *ref_counts.entry(reference.to_object).or_insert(0) += 1;
                }
            }
            ref_counts.entry(*from_object).or_insert(0);
        // Mark objects with suspicious reference counts
        let mut object_colors = self.object_colors.write()
            .map_err(|_| "Failed to acquire write lock on object colors")?;
        
        for (&object_id, &count) in ref_counts.iter() {
            if count == 0 {
                // Object with no incoming references should be white (collectible)
                object_colors.insert(object_id, Color::White);
            } else if self.has_outgoing_references(object_id, &reference_graph) {
                // Object with both incoming and outgoing references is suspicious
                object_colors.insert(object_id, Color::Purple);
            }
        }
        
        debug!("Reference counting optimization completed for {} objects", ref_counts.len());
        Ok(())
    /// Check if an object has outgoing references
    fn has_outgoing_references(
    ) -> bool {
        if let Some(references) = graph.get(&object_id) {
            references.iter().any(|r| r.reference_type == ReferenceType::Strong)
        } else {
            false
        }
    }
    
    /// Handle weak references in cycle detection
    pub fn handle_weak_references(&self, cycles: &mut Vec<CycleInfo>) -> Result<(), String> {
        debug!("Handling weak references in {} cycles", cycles.len());
        
        let reference_graph = self.reference_graph.read()
            .map_err(|_| "Failed to acquire read lock on reference graph")?;
        
        cycles.retain(|cycle| {
            // Check if cycle can be broken by weak references
            self.can_break_cycle_with_weak_refs(cycle, &reference_graph)
                .unwrap_or(true) // Keep cycle if we can't determine
        });
        
        debug!("After weak reference handling: {} cycles remain", cycles.len());
        Ok(())
    /// Check if a cycle can be broken by weak references
    fn can_break_cycle_with_weak_refs(
    ) -> Result<bool, String> {
        // A cycle can be broken if removing all weak references breaks the cycle
        for i in 0..cycle.objects.len() {
            let current = cycle.objects[i];
            let next = cycle.objects[(i + 1) % cycle.objects.len()];
            
            // Check if the connection between current and next is only through weak refs
            if let Some(references) = graph.get(&current) {
                let has_strong_ref_to_next = references.iter().any(|r| {
                    r.to_object == next && r.reference_type == ReferenceType::Strong
                });
                
                if !has_strong_ref_to_next {
                    // This cycle can be broken by weak references
                    return Ok(false);
                }
            }
        // All connections are strong references, cycle cannot be broken
        Ok(true)
    /// Performance optimization for large object graphs
    pub fn optimize_for_large_graphs(&self) -> Result<(), String> {
        info!("Applying performance optimizations for large object graphs");
        
        let reference_graph = self.reference_graph.read()
            .map_err(|_| "Failed to acquire read lock on reference graph")?;
        
        let graph_size = reference_graph.len();
        
        // Apply optimizations based on graph size
        if graph_size > 10000 {
            // For very large graphs, use sampling-based detection
            self.enable_sampling_detection()?;
        } else if graph_size > 1000 {
            // For medium graphs, use incremental detection
            self.enable_incremental_detection()?;
        // Optimize data structures for better cache locality
        self.optimize_data_structures()?;
        
        debug!("Performance optimizations applied for graph with {} objects", graph_size);
        Ok(())
    /// Enable sampling-based cycle detection for very large graphs
    fn enable_sampling_detection(&self) -> Result<(), String> {
        debug!("Enabling sampling-based cycle detection");
        
        let mut config = self.config.write()
            .map_err(|_| "Failed to acquire write lock on config")?;
        
        // Reduce detection frequency to improve performance
        config.detection_frequency *= 10;
        config.max_cycle_size = 100; // Limit cycle size for performance
        
        Ok(())
    /// Enable incremental detection for medium-sized graphs
    fn enable_incremental_detection(&self) -> Result<(), String> {
        debug!("Enabling incremental cycle detection");
        
        let mut config = self.config.write()
            .map_err(|_| "Failed to acquire write lock on config")?;
        
        config.incremental = true;
        config.hybrid_mode = true; // Use hybrid for better coverage
        
        Ok(())
    /// Optimize data structures for better performance
    fn optimize_data_structures(&self) -> Result<(), String> {
        debug!("Optimizing data structures for performance");
        
        // This could involve restructuring internal data for better cache locality
        // For now, we'll just clear old data that's no longer needed
        let mut object_colors = self.object_colors.write()
            .map_err(|_| "Failed to acquire write lock on object colors")?;
        
        // Remove color information for objects that no longer exist
        let mut to_remove = Vec::new();
        for &object_id in object_colors.keys() {
            if self.object_registry.get(object_id).is_err() {
                to_remove.push(object_id);
            }
        }
        
        for object_id in to_remove {
            object_colors.remove(&object_id);
        Ok(())
    }
}

// Safety: CycleDetector is thread-safe through its use of RwLock and Mutex
unsafe impl Send for CycleDetector {}
unsafe impl Sync for CycleDetector {}

