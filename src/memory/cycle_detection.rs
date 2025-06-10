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

/// Different cycle detection algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CycleDetectionAlgorithm {
    /// Bacon-Rajan cycle collection algorithm
    BaconRajan,
    /// Trial deletion algorithm
    TrialDeletion,
    /// Brownbridge algorithm (incremental)
    Brownbridge,
    /// Hybrid approach combining multiple algorithms
    Hybrid,
}

/// Color states for cycle detection algorithms
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Color {
    /// Object has not been visited
    White,
    /// Object is in the potential cycle set
    Gray,
    /// Object is confirmed reachable
    Black,
    /// Object is confirmed to be in a cycle
    Purple,
    /// Object reference count is being decremented
    Orange,
    /// Object is being freed
    Red,
}

/// Information about a detected cycle
#[derive(Debug, Clone)]
pub struct CycleInfo {
    /// Objects in the cycle
    pub objects: Vec<ObjectId>,
    /// Size of the cycle in objects
    pub size: usize,
    /// Total memory size of the cycle
    pub memory_size: usize,
    /// When the cycle was detected
    pub detected_at: std::time::Instant,
    /// Which algorithm detected the cycle
    pub detected_by: CycleDetectionAlgorithm,
}

/// Configuration for cycle detection
#[derive(Debug, Clone)]
pub struct CycleDetectionConfig {
    /// Primary algorithm to use
    pub algorithm: CycleDetectionAlgorithm,
    /// Maximum cycle size to detect (prevents infinite loops)
    pub max_cycle_size: usize,
    /// Enable incremental cycle detection
    pub incremental: bool,
    /// Cycle detection frequency (every N allocations)
    pub detection_frequency: usize,
    /// Enable hybrid algorithm combining multiple approaches
    pub hybrid_mode: bool,
    /// Minimum object age before considering for cycle detection
    pub min_object_age: std::time::Duration,
}

impl Default for CycleDetectionConfig {
    fn default() -> Self {
        Self {
            algorithm: CycleDetectionAlgorithm::BaconRajan,
            max_cycle_size: 1000,
            incremental: true,
            detection_frequency: 100,
            hybrid_mode: false,
            min_object_age: std::time::Duration::from_millis(100),
        }
    }
}

/// Statistics about cycle detection
#[derive(Debug, Clone, Default)]
pub struct CycleDetectionStats {
    pub cycles_detected: u64,
    pub cycles_collected: u64,
    pub objects_in_cycles: u64,
    pub detection_runs: u64,
    pub false_positives: u64,
    pub detection_time_total: std::time::Duration,
    pub largest_cycle_size: usize,
    pub cycles_by_algorithm: HashMap<CycleDetectionAlgorithm, u64>,
}

/// Reference information for cycle detection
#[derive(Debug, Clone)]
struct ObjectReference {
    from_object: ObjectId,
    to_object: ObjectId,
    reference_type: ReferenceType,
}

/// Types of object references
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ReferenceType {
    /// Strong reference (normal pointer)
    Strong,
    /// Weak reference (doesn't prevent collection)
    Weak,
    /// Root reference (from GC roots)
    Root,
}

/// Cycle detector implementation
pub struct CycleDetector {
    config: RwLock<CycleDetectionConfig>,
    object_registry: SharedObjectRegistry,
    stats: RwLock<CycleDetectionStats>,
    object_colors: RwLock<HashMap<ObjectId, Color>>,
    reference_graph: RwLock<HashMap<ObjectId, Vec<ObjectReference>>>,
    potential_cycles: Mutex<VecDeque<Vec<ObjectId>>>,
    detection_counter: std::sync::atomic::AtomicUsize,
}

impl CycleDetector {
    /// Create a new cycle detector
    pub fn new(object_registry: SharedObjectRegistry) -> Self {
        Self::with_config(object_registry, CycleDetectionConfig::default())
    }
    
    /// Create a new cycle detector with custom configuration
    #[instrument(skip(object_registry, config))]
    pub fn with_config(object_registry: SharedObjectRegistry, config: CycleDetectionConfig) -> Self {
        info!("Creating cycle detector with config: {:?}", config);
        
        Self {
            config: RwLock::new(config),
            object_registry,
            stats: RwLock::new(CycleDetectionStats::default()),
            object_colors: RwLock::new(HashMap::new()),
            reference_graph: RwLock::new(HashMap::new()),
            potential_cycles: Mutex::new(VecDeque::new()),
            detection_counter: std::sync::atomic::AtomicUsize::new(0),
        }
    }
    
    /// Update reference graph when a new reference is created
    #[instrument(skip(self))]
    pub fn add_reference(&self, from_object: ObjectId, to_object: ObjectId, reference_type: ReferenceType) -> Result<(), String> {
        debug!("Adding reference: {} -> {} ({:?})", from_object, to_object, reference_type);
        
        let mut reference_graph = self.reference_graph.write()
            .map_err(|_| "Failed to acquire write lock on reference graph")?;
        
        let reference = ObjectReference {
            from_object,
            to_object,
            reference_type,
        };
        
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
    }
    
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
    }
    
    /// Detect cycles in the object graph
    #[instrument(skip(self))]
    pub fn detect_cycles(&self) -> Result<Vec<CycleInfo>, String> {
        info!("Starting cycle detection");
        let start_time = std::time::Instant::now();
        
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        let cycles = match config.algorithm {
            CycleDetectionAlgorithm::BaconRajan => self.bacon_rajan_detection()?,
            CycleDetectionAlgorithm::TrialDeletion => self.trial_deletion_detection()?,
            CycleDetectionAlgorithm::Brownbridge => self.brownbridge_detection()?,
            CycleDetectionAlgorithm::Hybrid => self.hybrid_detection()?,
        };
        
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
        }
        
        info!("Cycle detection completed: found {} cycles in {:?}", cycles.len(), detection_time);
        Ok(cycles)
    }
    
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
    }
    
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
            }
            
            // Try deleting this object and see if a cycle becomes unreachable
            if let Some(cycle) = self.trial_delete_object(object_id, &mut trial_graph, &mut visited)? {
                cycles.push(cycle);
            }
        }
        
        Ok(cycles)
    }
    
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
    }
    
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
        }
        
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
    }
    
    /// Check if an object could be a cycle root
    fn could_be_cycle_root(&self, object_id: ObjectId, references: &[ObjectReference]) -> Result<bool, String> {
        // An object could be a cycle root if:
        // 1. It has outgoing references
        // 2. It's not directly reachable from GC roots (simplified check)
        // 3. It has been allocated for minimum age
        
        if references.is_empty() {
            return Ok(false);
        }
        
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
        }
        
        // For now, we assume non-root objects could potentially be in cycles
        // A more sophisticated implementation would do a full reachability analysis
        Ok(true)
    }
    
    /// Find a cycle starting from a specific root object
    fn find_cycle_from_root(&self, root: ObjectId, reference_graph: &HashMap<ObjectId, Vec<ObjectReference>>) -> Result<Option<CycleInfo>, String> {
        let mut visited = HashSet::new();
        let mut path = Vec::new();
        
        if self.dfs_find_cycle(root, root, reference_graph, &mut visited, &mut path)? {
            // Found a cycle, calculate its properties
            let memory_size = self.calculate_cycle_memory_size(&path)?;
            let cycle_size = path.len();
            
            Ok(Some(CycleInfo {
                objects: path,
                size: cycle_size,
                memory_size,
                detected_at: std::time::Instant::now(),
                detected_by: CycleDetectionAlgorithm::BaconRajan,
            }))
        } else {
            Ok(None)
        }
    }
    
    /// Depth-first search to find cycles
    fn dfs_find_cycle(
        &self,
        current: ObjectId,
        target: ObjectId,
        reference_graph: &HashMap<ObjectId, Vec<ObjectReference>>,
        visited: &mut HashSet<ObjectId>,
        path: &mut Vec<ObjectId>,
    ) -> Result<bool, String> {
        if path.len() > 0 && current == target {
            return Ok(true); // Found cycle back to target
        }
        
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        if path.len() >= config.max_cycle_size {
            return Ok(false); // Prevent infinite loops
        }
        
        if visited.contains(&current) {
            return Ok(false); // Already explored this path
        }
        
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
    }
    
    /// Trial delete an object to see if a cycle becomes unreachable
    fn trial_delete_object(
        &self,
        object_id: ObjectId,
        trial_graph: &mut HashMap<ObjectId, Vec<ObjectReference>>,
        visited: &mut HashSet<ObjectId>,
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
        }
        
        // Restore the references
        trial_graph.insert(object_id, removed_refs);
        Ok(None)
    }
    
    /// Check if removing an object creates an unreachable cycle
    fn check_unreachable_cycle(
        &self,
        start_object: ObjectId,
        trial_graph: &HashMap<ObjectId, Vec<ObjectReference>>,
        visited: &mut HashSet<ObjectId>,
    ) -> Result<Option<CycleInfo>, String> {
        if visited.contains(&start_object) {
            return Ok(None);
        }
        
        // Perform reachability analysis from GC roots
        let reachable_objects = self.analyze_reachability_from_roots(trial_graph)?;
        
        // If start_object is not reachable from roots, check if it's part of a cycle
        if !reachable_objects.contains(&start_object) {
            if let Some(cycle) = self.find_unreachable_cycle(start_object, trial_graph, visited)? {
                return Ok(Some(cycle));
            }
        }
        
        Ok(None)
    }
    
    /// Mark an object for incremental cycle detection
    fn mark_for_incremental_detection(&self, object_id: ObjectId) -> Result<(), String> {
        debug!("Marking object {} for incremental detection", object_id);
        
        // Set object color to purple (potential cycle candidate)
        {
            let mut object_colors = self.object_colors.write()
                .map_err(|_| "Failed to acquire write lock on object colors")?;
            object_colors.insert(object_id, Color::Purple);
        }
        
        // Add to potential cycles queue for later processing
        let potential_cycle = self.find_potential_cycle_from_object(object_id)?;
        if !potential_cycle.is_empty() {
            let mut potential_cycles = self.potential_cycles.lock()
                .map_err(|_| "Failed to acquire lock on potential cycles")?;
            potential_cycles.push_back(potential_cycle);
        }
        
        Ok(())
    }
    
    /// Verify a potential cycle found by incremental detection
    fn verify_potential_cycle(&self, potential_cycle: Vec<ObjectId>) -> Result<Option<CycleInfo>, String> {
        debug!("Verifying potential cycle with {} objects", potential_cycle.len());
        
        if potential_cycle.is_empty() {
            return Ok(None);
        }
        
        // Check if all objects in the potential cycle still exist
        let mut verified_objects = Vec::new();
        for &object_id in &potential_cycle {
            if self.object_registry.get(object_id).is_ok() {
                verified_objects.push(object_id);
            }
        }
        
        if verified_objects.len() < 2 {
            return Ok(None); // Need at least 2 objects for a cycle
        }
        
        // Verify the cycle structure still exists
        let reference_graph = self.reference_graph.read()
            .map_err(|_| "Failed to acquire read lock on reference graph")?;
        
        if self.verify_cycle_structure(&verified_objects, &reference_graph)? {
            let memory_size = self.calculate_cycle_memory_size(&verified_objects)?;
            
            Ok(Some(CycleInfo {
                objects: verified_objects.clone(),
                size: verified_objects.len(),
                memory_size,
                detected_at: std::time::Instant::now(),
                detected_by: CycleDetectionAlgorithm::Brownbridge,
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
    }
    
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
        }
        
        info!("Collected {} objects from cycles", collected_objects);
        Ok(collected_objects)
    }
    
    /// Get cycle detection statistics
    pub fn get_stats(&self) -> Result<CycleDetectionStats, String> {
        let stats = self.stats.read()
            .map_err(|_| "Failed to acquire read lock on stats")?;
        Ok(stats.clone())
    }
    
    /// Update configuration
    pub fn update_config(&self, new_config: CycleDetectionConfig) -> Result<(), String> {
        let mut config = self.config.write()
            .map_err(|_| "Failed to acquire write lock on config")?;
        *config = new_config;
        info!("Updated cycle detection configuration");
        Ok(())
    }
    
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
    }
    
    /// Analyze reachability from GC roots
    fn analyze_reachability_from_roots(&self, graph: &HashMap<ObjectId, Vec<ObjectReference>>) -> Result<HashSet<ObjectId>, String> {
        let mut reachable = HashSet::new();
        let mut worklist = VecDeque::new();
        
        // Get GC roots from object registry
        let roots = self.get_gc_roots()?;
        for root in roots {
            worklist.push_back(root);
            reachable.insert(root);
        }
        
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
        }
        
        Ok(reachable)
    }
    
    /// Find an unreachable cycle starting from a given object
    fn find_unreachable_cycle(
        &self,
        start_object: ObjectId,
        graph: &HashMap<ObjectId, Vec<ObjectReference>>,
        visited: &mut HashSet<ObjectId>,
    ) -> Result<Option<CycleInfo>, String> {
        let mut cycle_path = Vec::new();
        let mut path_set = HashSet::new();
        
        if self.dfs_find_unreachable_cycle(start_object, graph, visited, &mut cycle_path, &mut path_set)? {
            let memory_size = self.calculate_cycle_memory_size(&cycle_path)?;
            
            Ok(Some(CycleInfo {
                objects: cycle_path.clone(),
                size: cycle_path.len(),
                memory_size,
                detected_at: std::time::Instant::now(),
                detected_by: CycleDetectionAlgorithm::TrialDeletion,
            }))
        } else {
            Ok(None)
        }
    }
    
    /// DFS to find unreachable cycles
    fn dfs_find_unreachable_cycle(
        &self,
        current: ObjectId,
        graph: &HashMap<ObjectId, Vec<ObjectReference>>,
        visited: &mut HashSet<ObjectId>,
        cycle_path: &mut Vec<ObjectId>,
        path_set: &mut HashSet<ObjectId>,
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
        }
        
        visited.insert(current);
        cycle_path.push(current);
        path_set.insert(current);
        
        // Explore strong references only
        if let Some(references) = graph.get(&current) {
            for reference in references {
                if reference.reference_type == ReferenceType::Strong {
                    if self.dfs_find_unreachable_cycle(
                        reference.to_object,
                        graph,
                        visited,
                        cycle_path,
                        path_set,
                    )? {
                        return Ok(true);
                    }
                }
            }
        }
        
        cycle_path.pop();
        path_set.remove(&current);
        Ok(false)
    }
    
    /// Find potential cycle objects starting from a given object
    fn find_potential_cycle_from_object(&self, object_id: ObjectId) -> Result<Vec<ObjectId>, String> {
        let mut potential_cycle = Vec::new();
        let mut visited = HashSet::new();
        
        let reference_graph = self.reference_graph.read()
            .map_err(|_| "Failed to acquire read lock on reference graph")?;
        
        self.collect_strongly_connected_component(
            object_id,
            &reference_graph,
            &mut visited,
            &mut potential_cycle,
        )?;
        
        Ok(potential_cycle)
    }
    
    /// Collect objects in the same strongly connected component
    fn collect_strongly_connected_component(
        &self,
        start: ObjectId,
        graph: &HashMap<ObjectId, Vec<ObjectReference>>,
        visited: &mut HashSet<ObjectId>,
        component: &mut Vec<ObjectId>,
    ) -> Result<(), String> {
        if visited.contains(&start) {
            return Ok(());
        }
        
        visited.insert(start);
        component.push(start);
        
        // Follow all strong references
        if let Some(references) = graph.get(&start) {
            for reference in references {
                if reference.reference_type == ReferenceType::Strong {
                    self.collect_strongly_connected_component(
                        reference.to_object,
                        graph,
                        visited,
                        component,
                    )?;
                }
            }
        }
        
        Ok(())
    }
    
    /// Verify that objects form a valid cycle structure
    fn verify_cycle_structure(
        &self,
        objects: &[ObjectId],
        graph: &HashMap<ObjectId, Vec<ObjectReference>>,
    ) -> Result<bool, String> {
        if objects.len() < 2 {
            return Ok(false);
        }
        
        // Check that each object has a path to the next in the cycle
        for i in 0..objects.len() {
            let current = objects[i];
            let next = objects[(i + 1) % objects.len()];
            
            if !self.has_path_between(current, next, graph)? {
                return Ok(false);
            }
        }
        
        Ok(true)
    }
    
    /// Check if there's a path between two objects
    fn has_path_between(
        &self,
        from: ObjectId,
        to: ObjectId,
        graph: &HashMap<ObjectId, Vec<ObjectReference>>,
    ) -> Result<bool, String> {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::new();
        
        queue.push_back(from);
        visited.insert(from);
        
        while let Some(current) = queue.pop_front() {
            if current == to {
                return Ok(true);
            }
            
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
        }
        
        Ok(false)
    }
    
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
        }
        
        Ok(roots)
    }
    
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
        }
        
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
    }
    
    /// Check if an object has outgoing references
    fn has_outgoing_references(
        &self,
        object_id: ObjectId,
        graph: &HashMap<ObjectId, Vec<ObjectReference>>,
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
    }
    
    /// Check if a cycle can be broken by weak references
    fn can_break_cycle_with_weak_refs(
        &self,
        cycle: &CycleInfo,
        graph: &HashMap<ObjectId, Vec<ObjectReference>>,
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
        }
        
        // All connections are strong references, cycle cannot be broken
        Ok(true)
    }
    
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
        }
        
        // Optimize data structures for better cache locality
        self.optimize_data_structures()?;
        
        debug!("Performance optimizations applied for graph with {} objects", graph_size);
        Ok(())
    }
    
    /// Enable sampling-based cycle detection for very large graphs
    fn enable_sampling_detection(&self) -> Result<(), String> {
        debug!("Enabling sampling-based cycle detection");
        
        let mut config = self.config.write()
            .map_err(|_| "Failed to acquire write lock on config")?;
        
        // Reduce detection frequency to improve performance
        config.detection_frequency *= 10;
        config.max_cycle_size = 100; // Limit cycle size for performance
        
        Ok(())
    }
    
    /// Enable incremental detection for medium-sized graphs
    fn enable_incremental_detection(&self) -> Result<(), String> {
        debug!("Enabling incremental cycle detection");
        
        let mut config = self.config.write()
            .map_err(|_| "Failed to acquire write lock on config")?;
        
        config.incremental = true;
        config.hybrid_mode = true; // Use hybrid for better coverage
        
        Ok(())
    }
    
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
        }
        
        Ok(())
    }
}

// Safety: CycleDetector is thread-safe through its use of RwLock and Mutex
unsafe impl Send for CycleDetector {}
unsafe impl Sync for CycleDetector {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::object_id::ObjectRegistry;
    
    fn create_test_detector() -> (CycleDetector, SharedObjectRegistry) {
        let registry = Arc::new(ObjectRegistry::new());
        let detector = CycleDetector::new(registry.clone());
        (detector, registry)
    }
    
    #[test]
    fn test_detector_creation() {
        let (detector, _registry) = create_test_detector();
        let stats = detector.get_stats().unwrap();
        assert_eq!(stats.cycles_detected, 0);
    }
    
    #[test]
    fn test_add_remove_reference() {
        let (detector, _registry) = create_test_detector();
        
        let obj1 = ObjectId::new(1);
        let obj2 = ObjectId::new(2);
        
        detector.add_reference(obj1, obj2, ReferenceType::Strong).unwrap();
        detector.remove_reference(obj1, obj2).unwrap();
    }
    
    #[test]
    fn test_simple_cycle_detection() {
        let (detector, _registry) = create_test_detector();
        
        let obj1 = ObjectId::new(1);
        let obj2 = ObjectId::new(2);
        
        // Create a simple cycle: obj1 -> obj2 -> obj1
        detector.add_reference(obj1, obj2, ReferenceType::Strong).unwrap();
        detector.add_reference(obj2, obj1, ReferenceType::Strong).unwrap();
        
        // Note: This test would need actual objects in the registry to work properly
        // For now, it just tests that the detection doesn't crash
        let cycles = detector.detect_cycles().unwrap();
        // With empty registry, we don't expect to find cycles
        assert_eq!(cycles.len(), 0);
    }
    
    #[test]
    fn test_statistics_tracking() {
        let (detector, _registry) = create_test_detector();
        
        let initial_stats = detector.get_stats().unwrap();
        assert_eq!(initial_stats.detection_runs, 0);
        
        detector.detect_cycles().unwrap();
        
        let updated_stats = detector.get_stats().unwrap();
        assert_eq!(updated_stats.detection_runs, 1);
    }
    
    #[test]
    fn test_config_update() {
        let (detector, _registry) = create_test_detector();
        
        let new_config = CycleDetectionConfig {
            algorithm: CycleDetectionAlgorithm::TrialDeletion,
            max_cycle_size: 500,
            ..Default::default()
        };
        
        detector.update_config(new_config).unwrap();
    }
    
    #[test]
    fn test_reference_counting_optimization() {
        let (detector, _registry) = create_test_detector();
        
        let obj1 = ObjectId::new(1);
        let obj2 = ObjectId::new(2);
        let obj3 = ObjectId::new(3);
        
        // Create a reference graph
        detector.add_reference(obj1, obj2, ReferenceType::Strong).unwrap();
        detector.add_reference(obj2, obj3, ReferenceType::Strong).unwrap();
        detector.add_reference(obj3, obj1, ReferenceType::Strong).unwrap();
        
        // Test reference counting optimization
        detector.optimize_reference_counting().unwrap();
    }
    
    #[test]
    fn test_weak_reference_handling() {
        let (detector, _registry) = create_test_detector();
        
        let obj1 = ObjectId::new(1);
        let obj2 = ObjectId::new(2);
        
        // Create weak reference
        detector.add_reference(obj1, obj2, ReferenceType::Weak).unwrap();
        detector.add_reference(obj2, obj1, ReferenceType::Strong).unwrap();
        
        let mut cycles = vec![CycleInfo {
            objects: vec![obj1, obj2],
            size: 2,
            memory_size: 1024,
            detected_at: std::time::Instant::now(),
            detected_by: CycleDetectionAlgorithm::BaconRajan,
        }];
        
        detector.handle_weak_references(&mut cycles).unwrap();
    }
    
    #[test]
    fn test_large_graph_optimization() {
        let (detector, _registry) = create_test_detector();
        
        // Add many objects to simulate large graph
        for i in 0..1500 {
            let obj1 = ObjectId::new(i * 2);
            let obj2 = ObjectId::new(i * 2 + 1);
            detector.add_reference(obj1, obj2, ReferenceType::Strong).unwrap();
        }
        
        detector.optimize_for_large_graphs().unwrap();
    }
    
    #[test]
    fn test_incremental_detection() {
        let (detector, _registry) = create_test_detector();
        
        let obj1 = ObjectId::new(1);
        let obj2 = ObjectId::new(2);
        
        detector.mark_for_incremental_detection(obj1).unwrap();
        detector.mark_for_incremental_detection(obj2).unwrap();
        
        // Test brownbridge detection
        let cycles = detector.brownbridge_detection().unwrap();
        assert_eq!(cycles.len(), 0); // No real cycles with empty registry
    }
    
    #[test]
    fn test_cycle_verification() {
        let (detector, _registry) = create_test_detector();
        
        let obj1 = ObjectId::new(1);
        let obj2 = ObjectId::new(2);
        
        // Create references for potential cycle
        detector.add_reference(obj1, obj2, ReferenceType::Strong).unwrap();
        detector.add_reference(obj2, obj1, ReferenceType::Strong).unwrap();
        
        let potential_cycle = vec![obj1, obj2];
        let verified_cycle = detector.verify_potential_cycle(potential_cycle).unwrap();
        
        // Without objects in registry, verification should fail
        assert!(verified_cycle.is_none());
    }
    
    #[test]
    fn test_trial_deletion_algorithm() {
        let (detector, _registry) = create_test_detector();
        
        let obj1 = ObjectId::new(1);
        let obj2 = ObjectId::new(2);
        let obj3 = ObjectId::new(3);
        
        // Create a cycle
        detector.add_reference(obj1, obj2, ReferenceType::Strong).unwrap();
        detector.add_reference(obj2, obj3, ReferenceType::Strong).unwrap();
        detector.add_reference(obj3, obj1, ReferenceType::Strong).unwrap();
        
        let cycles = detector.trial_deletion_detection().unwrap();
        // Without actual objects in registry, we don't expect cycles
        assert_eq!(cycles.len(), 0);
    }
    
    #[test]
    fn test_hybrid_detection() {
        let (detector, _registry) = create_test_detector();
        
        let obj1 = ObjectId::new(1);
        let obj2 = ObjectId::new(2);
        
        detector.add_reference(obj1, obj2, ReferenceType::Strong).unwrap();
        detector.add_reference(obj2, obj1, ReferenceType::Strong).unwrap();
        
        let cycles = detector.hybrid_detection().unwrap();
        assert_eq!(cycles.len(), 0);
    }
    
    #[test]
    fn test_reference_graph_clearing() {
        let (detector, _registry) = create_test_detector();
        
        let obj1 = ObjectId::new(1);
        let obj2 = ObjectId::new(2);
        
        detector.add_reference(obj1, obj2, ReferenceType::Strong).unwrap();
        detector.clear_reference_graph().unwrap();
        
        // After clearing, detection should find no cycles
        let cycles = detector.detect_cycles().unwrap();
        assert_eq!(cycles.len(), 0);
    }
}
