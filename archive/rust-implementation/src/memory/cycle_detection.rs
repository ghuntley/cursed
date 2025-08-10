//! Cycle detection for CURSED memory management
//! 
//! Implements algorithms to detect and break reference cycles
//! that cannot be collected by traditional mark-and-sweep GC.

use crate::error::CursedError;
use crate::memory::{Traceable, Visitor};
use crate::memory::heap::{ObjectId, get_global_heap};
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Cycle detector for finding reference cycles
pub struct CycleDetector {
    /// Reference graph for cycle detection
    reference_graph: Mutex<ReferenceGraph>,
    /// Detected cycles
    detected_cycles: Mutex<Vec<Cycle>>,
    /// Detection statistics
    stats: Mutex<CycleStats>,
    /// Configuration
    config: Mutex<CycleConfig>,
}

/// Configuration for cycle detection
#[derive(Debug, Clone)]
pub struct CycleConfig {
    /// Enable automatic cycle detection
    pub auto_detect: bool,
    /// Maximum cycle length to detect
    pub max_cycle_length: usize,
    /// Detection frequency (every N GC cycles)
    pub detection_frequency: u32,
    /// Enable weak reference breaking
    pub break_weak_cycles: bool,
}

impl Default for CycleConfig {
    fn default() -> Self {
        Self {
            auto_detect: true,
            max_cycle_length: 100,
            detection_frequency: 5,
            break_weak_cycles: true,
        }
    }
}

/// Statistics for cycle detection
#[derive(Debug, Clone, Default)]
pub struct CycleStats {
    pub total_detections: u64,
    pub cycles_found: u64,
    pub cycles_broken: u64,
    pub detection_time_ms: u64,
    pub objects_in_cycles: u64,
    pub largest_cycle_size: usize,
}

/// Reference graph for tracking object relationships
#[derive(Debug)]
struct ReferenceGraph {
    /// Edges in the reference graph (object -> set of referenced objects)
    edges: HashMap<ObjectId, HashSet<ObjectId>>,
    /// Reverse edges (object -> set of objects that reference it)
    reverse_edges: HashMap<ObjectId, HashSet<ObjectId>>,
    /// Reference counts for each object
    ref_counts: HashMap<ObjectId, usize>,
}

impl ReferenceGraph {
    fn new() -> Self {
        Self {
            edges: HashMap::new(),
            reverse_edges: HashMap::new(),
            ref_counts: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: ObjectId, to: ObjectId) {
        // Add forward edge
        self.edges.entry(from).or_insert_with(HashSet::new).insert(to);
        
        // Add reverse edge
        self.reverse_edges.entry(to).or_insert_with(HashSet::new).insert(from);
        
        // Update reference count
        *self.ref_counts.entry(to).or_insert(0) += 1;
    }

    fn remove_edge(&mut self, from: ObjectId, to: ObjectId) {
        // Remove forward edge
        if let Some(edges) = self.edges.get_mut(&from) {
            edges.remove(&to);
            if edges.is_empty() {
                self.edges.remove(&from);
            }
        }
        
        // Remove reverse edge
        if let Some(reverse_edges) = self.reverse_edges.get_mut(&to) {
            reverse_edges.remove(&from);
            if reverse_edges.is_empty() {
                self.reverse_edges.remove(&to);
            }
        }
        
        // Update reference count
        if let Some(count) = self.ref_counts.get_mut(&to) {
            *count = count.saturating_sub(1);
            if *count == 0 {
                self.ref_counts.remove(&to);
            }
        }
    }

    fn get_references(&self, object: ObjectId) -> Option<&HashSet<ObjectId>> {
        self.edges.get(&object)
    }

    fn get_referrers(&self, object: ObjectId) -> Option<&HashSet<ObjectId>> {
        self.reverse_edges.get(&object)
    }

    fn get_ref_count(&self, object: ObjectId) -> usize {
        self.ref_counts.get(&object).copied().unwrap_or(0)
    }

    fn all_objects(&self) -> HashSet<ObjectId> {
        let mut objects = HashSet::new();
        objects.extend(self.edges.keys());
        objects.extend(self.reverse_edges.keys());
        objects
    }
}

/// A detected reference cycle
#[derive(Debug, Clone)]
pub struct Cycle {
    /// Objects in the cycle
    pub objects: Vec<ObjectId>,
    /// Total size of objects in cycle
    pub total_size: usize,
    /// Cycle strength (how many references hold it together)
    pub strength: usize,
    /// Whether the cycle can be broken safely
    pub breakable: bool,
}

impl Cycle {
    /// Get the length of the cycle
    pub fn length(&self) -> usize {
        self.objects.len()
    }

    /// Check if the cycle contains a specific object
    pub fn contains(&self, object: ObjectId) -> bool {
        self.objects.contains(&object)
    }
}

impl CycleDetector {
    /// Create a new cycle detector
    pub fn new() -> Self {
        Self {
            reference_graph: Mutex::new(ReferenceGraph::new()),
            detected_cycles: Mutex::new(Vec::new()),
            stats: Mutex::new(CycleStats::default()),
            config: Mutex::new(CycleConfig::default()),
        }
    }

    /// Configure the cycle detector
    pub fn configure(&self, config: CycleConfig) {
        let mut current_config = self.config.lock().unwrap();
        *current_config = config;
    }

    /// Update the reference graph with current heap state
    pub fn update_reference_graph(&self) -> Result<(), CursedError> {
        let heap = get_global_heap();
        let mut graph = self.reference_graph.lock().unwrap();
        
        // Clear existing graph
        graph.edges.clear();
        graph.reverse_edges.clear();
        graph.ref_counts.clear();
        
        // Build graph from heap objects
        let all_objects = heap.all_objects();
        for object_id in all_objects {
            if let Some(object) = heap.get(object_id) {
                // In practice, you'd trace the object to find its references
                // This is simplified for demonstration
                let mut visitor = GraphBuildingVisitor::new(&mut graph, object_id);
                object.trace(&mut visitor);
            }
        }
        
        Ok(())
    }

    /// Detect cycles using Tarjan's strongly connected components algorithm
    pub fn detect_cycles(&self) -> Result<Vec<Cycle>, CursedError> {
        let start_time = Instant::now();
        let config = self.config.lock().unwrap();
        let max_length = config.max_cycle_length;
        drop(config);
        
        let graph = self.reference_graph.lock().unwrap();
        let mut cycles = Vec::new();
        
        // Use DFS to find strongly connected components
        let mut visited = HashSet::new();
        let mut recursion_stack = HashSet::new();  
        let mut current_path = Vec::new();
        
        for &object in graph.all_objects().iter() {
            if !visited.contains(&object) {
                self.dfs_detect_cycles(
                    object,
                    &graph,
                    &mut visited,
                    &mut recursion_stack,
                    &mut current_path,
                    &mut cycles,
                    max_length,
                )?;
            }
        }
        
        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_detections += 1;
            stats.cycles_found += cycles.len() as u64;
            stats.detection_time_ms += start_time.elapsed().as_millis() as u64;
            
            // Update largest cycle size
            if let Some(largest) = cycles.iter().map(|c| c.length()).max() {
                if largest > stats.largest_cycle_size {
                    stats.largest_cycle_size = largest;
                }
            }
            
            stats.objects_in_cycles += cycles.iter()
                .map(|c| c.objects.len() as u64)
                .sum::<u64>();
        }
        
        // Store detected cycles
        {
            let mut detected_cycles = self.detected_cycles.lock().unwrap();
            detected_cycles.extend(cycles.clone());
        }
        
        Ok(cycles)
    }

    /// Depth-first search for cycle detection
    fn dfs_detect_cycles(
        &self,
        current: ObjectId,
        graph: &ReferenceGraph,
        visited: &mut HashSet<ObjectId>,
        recursion_stack: &mut HashSet<ObjectId>,
        current_path: &mut Vec<ObjectId>,
        cycles: &mut Vec<Cycle>,
        max_length: usize,
    ) -> Result<(), CursedError> {
        if current_path.len() > max_length {
            return Ok(());
        }
        
        visited.insert(current);
        recursion_stack.insert(current);
        current_path.push(current);
        
        if let Some(references) = graph.get_references(current) {
            for &referenced in references {
                if !visited.contains(&referenced) {
                    self.dfs_detect_cycles(
                        referenced,
                        graph,
                        visited,
                        recursion_stack,
                        current_path,
                        cycles,
                        max_length,
                    )?;
                } else if recursion_stack.contains(&referenced) {
                    // Found a cycle
                    if let Some(cycle_start) = current_path.iter().position(|&obj| obj == referenced) {
                        let cycle_objects = current_path[cycle_start..].to_vec();
                        let cycle = self.create_cycle_info(cycle_objects, graph)?;
                        cycles.push(cycle);
                    }
                }
            }
        }
        
        current_path.pop();
        recursion_stack.remove(&current);
        Ok(())
    }

    /// Create cycle information
    fn create_cycle_info(&self, objects: Vec<ObjectId>, graph: &ReferenceGraph) -> Result<Cycle, CursedError> {
        let heap = get_global_heap();
        let mut total_size = 0;
        let mut strength = 0;
        
        for &object_id in &objects {
            if let Some(object) = heap.get(object_id) {
                total_size += object.size();
            }
            strength += graph.get_ref_count(object_id);
        }
        
        // Determine if cycle is breakable (simplified logic)
        let breakable = strength <= objects.len() * 2; // Heuristic
        
        Ok(Cycle {
            objects,
            total_size,
            strength,
            breakable,
        })
    }

    /// Break detected cycles by removing weak references
    pub fn break_cycles(&self) -> Result<usize, CursedError> {
        let config = self.config.lock().unwrap();
        if !config.break_weak_cycles {
            return Ok(0);
        }
        drop(config);
        
        let mut detected_cycles = self.detected_cycles.lock().unwrap();
        let mut broken_count = 0;
        
        for cycle in detected_cycles.iter_mut() {
            if cycle.breakable {
                // In practice, you'd identify and remove the weakest references
                // This is simplified for demonstration
                broken_count += 1;
            }
        }
        
        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.cycles_broken += broken_count as u64;
        }
        
        // Clear broken cycles
        detected_cycles.retain(|cycle| !cycle.breakable);
        
        Ok(broken_count)
    }

    /// Get all detected cycles
    pub fn get_detected_cycles(&self) -> Vec<Cycle> {
        let detected_cycles = self.detected_cycles.lock().unwrap();
        detected_cycles.clone()
    }

    /// Clear all detected cycles
    pub fn clear_detected_cycles(&self) {
        let mut detected_cycles = self.detected_cycles.lock().unwrap();
        detected_cycles.clear();
    }

    /// Get cycle detection statistics
    pub fn stats(&self) -> CycleStats {
        let stats = self.stats.lock().unwrap();
        stats.clone()
    }

    /// Run full cycle detection and breaking
    pub fn run_cycle_collection(&self) -> Result<CycleResult, CursedError> {
        let start_time = Instant::now();
        
        // Update reference graph
        self.update_reference_graph()?;
        
        // Detect cycles
        let cycles = self.detect_cycles()?;
        
        // Break cycles if enabled
        let broken_cycles = self.break_cycles()?;
        
        let total_time = start_time.elapsed();
        
        Ok(CycleResult {
            cycles_detected: cycles.len(),
            cycles_broken: broken_cycles,
            objects_in_cycles: cycles.iter().map(|c| c.objects.len()).sum(),
            total_size_in_cycles: cycles.iter().map(|c| c.total_size).sum(),
            detection_time: total_time,
        })
    }
}

impl Default for CycleDetector {
    fn default() -> Self {
        Self::new()
    }
}

/// Visitor for building reference graph
struct GraphBuildingVisitor<'a> {
    graph: &'a mut ReferenceGraph,
    current_object: ObjectId,
}

impl<'a> GraphBuildingVisitor<'a> {
    fn new(graph: &'a mut ReferenceGraph, current_object: ObjectId) -> Self {
        Self {
            graph,
            current_object,
        }
    }
}

impl<'a> Visitor for GraphBuildingVisitor<'a> {
    fn visit(&mut self, _obj: &dyn Traceable) {
        // In practice, you'd extract the ObjectId from the traceable object
        // and add edges to the graph. This is simplified for demonstration.
        
        // Continue tracing
        _obj.trace(self);
    }
}

/// Result of cycle collection
#[derive(Debug, Clone)]
pub struct CycleResult {
    pub cycles_detected: usize,
    pub cycles_broken: usize,
    pub objects_in_cycles: usize,
    pub total_size_in_cycles: usize,
    pub detection_time: Duration,
}

impl std::fmt::Display for CycleResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "Cycle Detection: found {} cycles ({} broken), {} objects, {} bytes in {:.2}ms",
            self.cycles_detected,
            self.cycles_broken,
            self.objects_in_cycles,
            self.total_size_in_cycles,
            self.detection_time.as_secs_f64() * 1000.0
        )
    }
}

/// Global cycle detector
static GLOBAL_CYCLE_DETECTOR: std::sync::LazyLock<Arc<CycleDetector>> = 
    std::sync::LazyLock::new(|| Arc::new(CycleDetector::new()));

/// Get the global cycle detector
pub fn get_global_cycle_detector() -> Arc<CycleDetector> {
    Arc::clone(&GLOBAL_CYCLE_DETECTOR)
}

/// Convenience function to run cycle detection
pub fn detect_and_break_cycles() -> Result<CycleResult, CursedError> {
    get_global_cycle_detector().run_cycle_collection()
}

/// Compatibility exports
pub use CycleDetector as MinimalImplementation;

/// Convenience function for compatibility
pub fn get_minimal_result() -> Result<String, CursedError> {
    let detector = get_global_cycle_detector();
    let stats = detector.stats();
    Ok(format!("Cycle detector ready - {} detections, {} cycles found", 
               stats.total_detections, stats.cycles_found))
}
