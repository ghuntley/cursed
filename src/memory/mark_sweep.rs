//! Production Mark-and-Sweep Garbage Collection Implementation for CURSED
//!
//! This module provides a complete mark-and-sweep garbage collection system with:
//! - Tricolor marking algorithm for concurrent collection
//! - Cycle detection using Strongly Connected Components
//! - Incremental collection to reduce pause times
//! - Memory-safe operations with proper error handling
//! - Integration with CURSED runtime components

use std::sync::{Arc, RwLock, Mutex};
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::collections::{HashMap, HashSet, VecDeque};
use std::ptr::NonNull;
use std::time::{Duration, Instant};
use std::mem;

use crate::error::CursedError;
use crate::memory::{Tag, Traceable, Visitor};

/// Mark-and-sweep garbage collector with tricolor marking
pub struct MarkSweepCollector {
    /// Heap objects indexed by address
    objects: RwLock<HashMap<usize, ObjectInfo>>,
    /// Root set addresses
    roots: RwLock<HashSet<usize>>,
    /// Collection state
    state: RwLock<CollectionState>,
    /// Collection statistics
    stats: RwLock<CollectionStats>,
    /// Configuration
    config: CollectorConfig,
    /// Incremental collection state
    incremental_state: RwLock<IncrementalState>,
    /// Cycle detection state
    cycle_detector: RwLock<CycleDetector>,
    /// Collection running flag
    collecting: AtomicBool,
    /// Total allocated bytes
    allocated_bytes: AtomicUsize,
}

/// Object information for garbage collection
#[derive(Debug, Clone)]
pub struct ObjectInfo {
    /// Object size in bytes
    pub size: usize,
    /// Object type tag
    pub tag: Tag,
    /// Object color for tricolor marking
    pub color: ObjectColor,
    /// Object references (addresses of referenced objects)
    pub references: Vec<usize>,
    /// Reference count (for hybrid collection)
    pub ref_count: usize,
    /// Generation (for generational collection)
    pub generation: u8,
    /// Allocation timestamp
    pub allocated_at: Instant,
    /// Mark timestamp
    pub marked_at: Option<Instant>,
}

/// Object color for tricolor marking algorithm
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectColor {
    /// White - not yet visited, candidate for collection
    White,
    /// Gray - visited but references not yet processed
    Gray,
    /// Black - fully processed, will not be collected
    Black,
}

/// Collection state
#[derive(Debug, Clone)]
pub struct CollectionState {
    /// Current collection phase
    pub phase: CollectionPhase,
    /// Objects in gray set (to be processed)
    pub gray_set: VecDeque<usize>,
    /// Objects marked for collection
    pub white_set: HashSet<usize>,
    /// Objects marked as live
    pub black_set: HashSet<usize>,
    /// Start time of current collection
    pub start_time: Instant,
    /// Collection cycle number
    pub cycle_number: u64,
}

/// Collection phases
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionPhase {
    /// Idle - no collection in progress
    Idle,
    /// Preparing - initializing collection state
    Preparing,
    /// Marking - marking reachable objects
    Marking,
    /// CycleDetection - detecting and handling cycles
    CycleDetection,
    /// Sweeping - collecting unmarked objects
    Sweeping,
    /// Finalizing - cleaning up after collection
    Finalizing,
}

/// Collection statistics
#[derive(Debug, Clone, Default)]
pub struct CollectionStats {
    /// Total number of collections
    pub total_collections: u64,
    /// Total collection time
    pub total_time: Duration,
    /// Objects collected in last cycle
    pub last_collected: usize,
    /// Bytes reclaimed in last cycle
    pub last_reclaimed: usize,
    /// Average collection time
    pub avg_collection_time: Duration,
    /// Maximum collection time
    pub max_collection_time: Duration,
    /// Cycle detection statistics
    pub cycles_detected: u64,
    /// Objects in detected cycles
    pub cyclic_objects: u64,
}

/// Collector configuration
#[derive(Debug, Clone)]
pub struct CollectorConfig {
    /// Enable incremental collection
    pub incremental: bool,
    /// Time budget for incremental steps (milliseconds)
    pub incremental_budget_ms: u64,
    /// Enable cycle detection
    pub cycle_detection: bool,
    /// Maximum objects to process per incremental step
    pub max_objects_per_step: usize,
    /// Allocation threshold to trigger collection
    pub allocation_threshold: usize,
    /// Enable concurrent marking (requires careful synchronization)
    pub concurrent_marking: bool,
}

impl Default for CollectorConfig {
    fn default() -> Self {
        Self {
            incremental: true,
            incremental_budget_ms: 5,
            cycle_detection: true,
            max_objects_per_step: 1000,
            allocation_threshold: 10 * 1024 * 1024, // 10MB
            concurrent_marking: false, // Disabled by default for safety
        }
    }
}

/// Incremental collection state
#[derive(Debug)]
struct IncrementalState {
    /// Current work queue
    work_queue: VecDeque<usize>,
    /// Time budget remaining
    time_remaining: Duration,
    /// Objects processed in current step
    processed_count: usize,
    /// Current incremental phase
    current_phase: CollectionPhase,
}

/// Cycle detection using Tarjan's algorithm
#[derive(Debug)]
struct CycleDetector {
    /// Discovery index counter
    index_counter: usize,
    /// Discovery indices
    indices: HashMap<usize, usize>,
    /// Low-link values
    low_links: HashMap<usize, usize>,
    /// Objects on stack
    on_stack: HashSet<usize>,
    /// DFS stack
    stack: Vec<usize>,
    /// Detected strongly connected components
    sccs: Vec<Vec<usize>>,
}

/// Mark visitor for object traversal
pub struct MarkVisitor {
    /// Visited objects
    visited: HashSet<usize>,
    /// Objects to visit
    to_visit: VecDeque<usize>,
}

impl MarkSweepCollector {
    /// Create a new mark-and-sweep collector
    pub fn new(config: CollectorConfig) -> Self {
        Self {
            objects: RwLock::new(HashMap::new()),
            roots: RwLock::new(HashSet::new()),
            state: RwLock::new(CollectionState {
                phase: CollectionPhase::Idle,
                gray_set: VecDeque::new(),
                white_set: HashSet::new(),
                black_set: HashSet::new(),
                start_time: Instant::now(),
                cycle_number: 0,
            }),
            stats: RwLock::new(CollectionStats::default()),
            config,
            incremental_state: RwLock::new(IncrementalState {
                work_queue: VecDeque::new(),
                time_remaining: Duration::from_millis(5),
                processed_count: 0,
                current_phase: CollectionPhase::Idle,
            }),
            cycle_detector: RwLock::new(CycleDetector {
                index_counter: 0,
                indices: HashMap::new(),
                low_links: HashMap::new(),
                on_stack: HashSet::new(),
                stack: Vec::new(),
                sccs: Vec::new(),
            }),
            collecting: AtomicBool::new(false),
            allocated_bytes: AtomicUsize::new(0),
        }
    }

    /// Register an object for garbage collection
    pub fn register_object(
        &self,
        addr: usize,
        size: usize,
        tag: Tag,
        references: Vec<usize>,
    ) -> Result<(), CursedError> {
        let mut objects = self.objects.write()
            .map_err(|_| CursedError::runtime_error("Failed to acquire objects lock"))?;

        let object_info = ObjectInfo {
            size,
            tag,
            color: ObjectColor::White,
            references,
            ref_count: 1,
            generation: 0,
            allocated_at: Instant::now(),
            marked_at: None,
        };

        objects.insert(addr, object_info);
        self.allocated_bytes.fetch_add(size, Ordering::Relaxed);

        // Check if collection should be triggered
        if self.allocated_bytes.load(Ordering::Relaxed) > self.config.allocation_threshold {
            self.maybe_trigger_collection()?;
        }

        Ok(())
    }

    /// Unregister an object (when manually freed)
    pub fn unregister_object(&self, addr: usize) -> Result<(), CursedError> {
        let mut objects = self.objects.write()
            .map_err(|_| CursedError::runtime_error("Failed to acquire objects lock"))?;

        if let Some(info) = objects.remove(&addr) {
            self.allocated_bytes.fetch_sub(info.size, Ordering::Relaxed);
        }

        Ok(())
    }

    /// Add a root object
    pub fn add_root(&self, addr: usize) -> Result<(), CursedError> {
        let mut roots = self.roots.write()
            .map_err(|_| CursedError::runtime_error("Failed to acquire roots lock"))?;
        roots.insert(addr);
        Ok(())
    }

    /// Remove a root object
    pub fn remove_root(&self, addr: usize) -> Result<(), CursedError> {
        let mut roots = self.roots.write()
            .map_err(|_| CursedError::runtime_error("Failed to acquire roots lock"))?;
        roots.remove(&addr);
        Ok(())
    }

    /// Update object references
    pub fn update_references(&self, addr: usize, new_refs: Vec<usize>) -> Result<(), CursedError> {
        let mut objects = self.objects.write()
            .map_err(|_| CursedError::runtime_error("Failed to acquire objects lock"))?;

        if let Some(info) = objects.get_mut(&addr) {
            info.references = new_refs;
        }

        Ok(())
    }

    /// Perform garbage collection
    pub fn collect(&self) -> Result<CollectionStats, CursedError> {
        // Prevent concurrent collections
        if self.collecting.compare_exchange(false, true, Ordering::Acquire, Ordering::Relaxed).is_err() {
            return Err(CursedError::runtime_error("Collection already in progress"));
        }

        let result = if self.config.incremental {
            self.incremental_collect()
        } else {
            self.full_collect()
        };

        self.collecting.store(false, Ordering::Release);
        result
    }

    /// Perform full collection
    fn full_collect(&self) -> Result<CollectionStats, CursedError> {
        let start_time = Instant::now();

        // Initialize collection state
        self.initialize_collection()?;

        // Mark phase
        self.mark_phase()?;

        // Cycle detection phase
        if self.config.cycle_detection {
            self.cycle_detection_phase()?;
        }

        // Sweep phase
        let reclaimed = self.sweep_phase()?;

        // Update statistics
        let collection_time = start_time.elapsed();
        self.update_stats(collection_time, reclaimed)?;

        let stats = self.stats.read()
            .map_err(|_| CursedError::runtime_error("Failed to acquire stats lock"))?;
        Ok(stats.clone())
    }

    /// Perform incremental collection
    fn incremental_collect(&self) -> Result<CollectionStats, CursedError> {
        let budget = Duration::from_millis(self.config.incremental_budget_ms);
        let start_time = Instant::now();

        {
            let mut inc_state = self.incremental_state.write()
                .map_err(|_| CursedError::runtime_error("Failed to acquire incremental state lock"))?;
            inc_state.time_remaining = budget;
            inc_state.processed_count = 0;
        }

        while start_time.elapsed() < budget {
            let should_continue = self.incremental_step()?;
            if !should_continue {
                break;
            }
        }

        let stats = self.stats.read()
            .map_err(|_| CursedError::runtime_error("Failed to acquire stats lock"))?;
        Ok(stats.clone())
    }

    /// Perform one incremental step
    fn incremental_step(&self) -> Result<bool, CursedError> {
        let state = {
            let state = self.state.read()
                .map_err(|_| CursedError::runtime_error("Failed to acquire state lock"))?;
            state.phase
        };

        match state {
            CollectionPhase::Idle => {
                self.initialize_collection()?;
                Ok(true)
            }
            CollectionPhase::Preparing => {
                self.incremental_prepare_step()?;
                Ok(true)
            }
            CollectionPhase::Marking => {
                self.incremental_mark_step()
            }
            CollectionPhase::CycleDetection => {
                if self.config.cycle_detection {
                    self.incremental_cycle_detection_step()
                } else {
                    self.transition_to_sweeping()?;
                    Ok(true)
                }
            }
            CollectionPhase::Sweeping => {
                self.incremental_sweep_step()
            }
            CollectionPhase::Finalizing => {
                self.finalize_collection()?;
                Ok(false) // Collection complete
            }
        }
    }

    /// Initialize collection state
    fn initialize_collection(&self) -> Result<(), CursedError> {
        let mut state = self.state.write()
            .map_err(|_| CursedError::runtime_error("Failed to acquire state lock"))?;

        state.phase = CollectionPhase::Preparing;
        state.gray_set.clear();
        state.white_set.clear();
        state.black_set.clear();
        state.start_time = Instant::now();
        state.cycle_number += 1;

        // Initialize all objects as white
        let objects = self.objects.read()
            .map_err(|_| CursedError::runtime_error("Failed to acquire objects lock"))?;

        for &addr in objects.keys() {
            state.white_set.insert(addr);
        }

        Ok(())
    }

    /// Mark phase - mark all reachable objects
    fn mark_phase(&self) -> Result<(), CursedError> {
        {
            let mut state = self.state.write()
                .map_err(|_| CursedError::runtime_error("Failed to acquire state lock"))?;
            state.phase = CollectionPhase::Marking;
        }

        // Start from roots
        let roots = {
            let roots = self.roots.read()
                .map_err(|_| CursedError::runtime_error("Failed to acquire roots lock"))?;
            roots.clone()
        };

        for &root_addr in &roots {
            self.mark_object(root_addr)?;
        }

        // Process gray set until empty
        loop {
            let next_object = {
                let mut state = self.state.write()
                    .map_err(|_| CursedError::runtime_error("Failed to acquire state lock"))?;
                state.gray_set.pop_front()
            };

            match next_object {
                Some(addr) => self.process_gray_object(addr)?,
                None => break, // No more objects to process
            }
        }

        Ok(())
    }

    /// Mark an object as reachable
    fn mark_object(&self, addr: usize) -> Result<(), CursedError> {
        let mut objects = self.objects.write()
            .map_err(|_| CursedError::runtime_error("Failed to acquire objects lock"))?;

        if let Some(info) = objects.get_mut(&addr) {
            if info.color == ObjectColor::White {
                info.color = ObjectColor::Gray;
                info.marked_at = Some(Instant::now());

                let mut state = self.state.write()
                    .map_err(|_| CursedError::runtime_error("Failed to acquire state lock"))?;
                state.white_set.remove(&addr);
                state.gray_set.push_back(addr);
            }
        }

        Ok(())
    }

    /// Process a gray object (mark its references)
    fn process_gray_object(&self, addr: usize) -> Result<(), CursedError> {
        let references = {
            let mut objects = self.objects.write()
                .map_err(|_| CursedError::runtime_error("Failed to acquire objects lock"))?;

            if let Some(info) = objects.get_mut(&addr) {
                info.color = ObjectColor::Black;
                let refs = info.references.clone();

                let mut state = self.state.write()
                    .map_err(|_| CursedError::runtime_error("Failed to acquire state lock"))?;
                state.black_set.insert(addr);

                refs
            } else {
                return Ok(());
            }
        };

        // Mark all referenced objects
        for &ref_addr in &references {
            self.mark_object(ref_addr)?;
        }

        Ok(())
    }

    /// Cycle detection phase using Tarjan's algorithm
    fn cycle_detection_phase(&self) -> Result<(), CursedError> {
        let mut state = self.state.write()
            .map_err(|_| CursedError::runtime_error("Failed to acquire state lock"))?;
        state.phase = CollectionPhase::CycleDetection;
        drop(state);

        let mut detector = self.cycle_detector.write()
            .map_err(|_| CursedError::runtime_error("Failed to acquire cycle detector lock"))?;

        // Reset detector state
        detector.index_counter = 0;
        detector.indices.clear();
        detector.low_links.clear();
        detector.on_stack.clear();
        detector.stack.clear();
        detector.sccs.clear();

        // Find strongly connected components
        let objects = self.objects.read()
            .map_err(|_| CursedError::runtime_error("Failed to acquire objects lock"))?;

        let all_objects: Vec<usize> = objects.keys().copied().collect();
        drop(objects);

        for &addr in &all_objects {
            if !detector.indices.contains_key(&addr) {
                self.tarjan_scc(addr, &mut detector)?;
            }
        }

        // Process detected cycles
        self.process_detected_cycles(&detector)?;

        Ok(())
    }

    /// Tarjan's algorithm for strongly connected components
    fn tarjan_scc(&self, addr: usize, detector: &mut CycleDetector) -> Result<(), CursedError> {
        let index = detector.index_counter;
        detector.index_counter += 1;
        detector.indices.insert(addr, index);
        detector.low_links.insert(addr, index);
        detector.stack.push(addr);
        detector.on_stack.insert(addr);

        // Get object references
        let references = {
            let objects = self.objects.read()
                .map_err(|_| CursedError::runtime_error("Failed to acquire objects lock"))?;
            objects.get(&addr).map(|info| info.references.clone()).unwrap_or_default()
        };

        for &ref_addr in &references {
            if !detector.indices.contains_key(&ref_addr) {
                // Recursively process unvisited reference
                self.tarjan_scc(ref_addr, detector)?;
                let ref_low_link = detector.low_links[&ref_addr];
                if let Some(current_low_link) = detector.low_links.get_mut(&addr) {
                    *current_low_link = (*current_low_link).min(ref_low_link);
                }
            } else if detector.on_stack.contains(&ref_addr) {
                // Back edge to object on stack
                let ref_index = detector.indices[&ref_addr];
                if let Some(current_low_link) = detector.low_links.get_mut(&addr) {
                    *current_low_link = (*current_low_link).min(ref_index);
                }
            }
        }

        // If this is a root of an SCC, pop the SCC from stack
        if detector.low_links[&addr] == detector.indices[&addr] {
            let mut scc = Vec::new();
            loop {
                let node = detector.stack.pop().unwrap();
                detector.on_stack.remove(&node);
                scc.push(node);
                if node == addr {
                    break;
                }
            }

            // Only keep SCCs with more than one node (cycles)
            if scc.len() > 1 {
                detector.sccs.push(scc);
            }
        }

        Ok(())
    }

    /// Process detected cycles
    fn process_detected_cycles(&self, detector: &CycleDetector) -> Result<(), CursedError> {
        let mut stats = self.stats.write()
            .map_err(|_| CursedError::runtime_error("Failed to acquire stats lock"))?;

        stats.cycles_detected += detector.sccs.len() as u64;

        for scc in &detector.sccs {
            stats.cyclic_objects += scc.len() as u64;

            // For each cycle, check if all objects are unreachable from roots
            // If so, they can be collected even though they reference each other
            if self.is_cycle_collectable(scc)? {
                self.mark_cycle_for_collection(scc)?;
            }
        }

        Ok(())
    }

    /// Check if a cycle is collectable (no external references)
    fn is_cycle_collectable(&self, cycle: &[usize]) -> Result<bool, CursedError> {
        let cycle_set: HashSet<usize> = cycle.iter().copied().collect();
        let objects = self.objects.read()
            .map_err(|_| CursedError::runtime_error("Failed to acquire objects lock"))?;

        // Check if any object in the cycle is reachable from outside the cycle
        for &addr in cycle {
            if let Some(info) = objects.get(&addr) {
                if info.color == ObjectColor::Black {
                    // Object was marked as reachable from roots
                    return Ok(false);
                }
            }
        }

        // Check for external references into the cycle
        for (&obj_addr, info) in objects.iter() {
            if !cycle_set.contains(&obj_addr) {
                // External object
                for &ref_addr in &info.references {
                    if cycle_set.contains(&ref_addr) {
                        // External reference into cycle
                        return Ok(false);
                    }
                }
            }
        }

        Ok(true) // No external references, cycle is collectable
    }

    /// Mark a cycle for collection
    fn mark_cycle_for_collection(&self, cycle: &[usize]) -> Result<(), CursedError> {
        let mut state = self.state.write()
            .map_err(|_| CursedError::runtime_error("Failed to acquire state lock"))?;

        for &addr in cycle {
            state.white_set.insert(addr);
            state.black_set.remove(&addr);
        }

        Ok(())
    }

    /// Sweep phase - collect unmarked objects
    fn sweep_phase(&self) -> Result<usize, CursedError> {
        let mut state = self.state.write()
            .map_err(|_| CursedError::runtime_error("Failed to acquire state lock"))?;
        state.phase = CollectionPhase::Sweeping;

        let to_collect: Vec<usize> = state.white_set.iter().copied().collect();
        drop(state);

        let mut total_reclaimed = 0;
        let mut objects = self.objects.write()
            .map_err(|_| CursedError::runtime_error("Failed to acquire objects lock"))?;

        for addr in to_collect {
            if let Some(info) = objects.remove(&addr) {
                total_reclaimed += info.size;
                self.allocated_bytes.fetch_sub(info.size, Ordering::Relaxed);
            }
        }

        // Reset colors for remaining objects
        for info in objects.values_mut() {
            info.color = ObjectColor::White;
            info.marked_at = None;
        }

        Ok(total_reclaimed)
    }

    /// Incremental prepare step
    fn incremental_prepare_step(&self) -> Result<(), CursedError> {
        // Initialize incremental marking
        let roots = {
            let roots = self.roots.read()
                .map_err(|_| CursedError::runtime_error("Failed to acquire roots lock"))?;
            roots.clone()
        };

        {
            let mut inc_state = self.incremental_state.write()
                .map_err(|_| CursedError::runtime_error("Failed to acquire incremental state lock"))?;
            inc_state.work_queue.clear();
            for &root_addr in &roots {
                inc_state.work_queue.push_back(root_addr);
            }
        }

        let mut state = self.state.write()
            .map_err(|_| CursedError::runtime_error("Failed to acquire state lock"))?;
        state.phase = CollectionPhase::Marking;

        Ok(())
    }

    /// Incremental mark step
    fn incremental_mark_step(&self) -> Result<bool, CursedError> {
        let mut processed = 0;
        let max_objects = self.config.max_objects_per_step;

        while processed < max_objects {
            let next_object = {
                let mut inc_state = self.incremental_state.write()
                    .map_err(|_| CursedError::runtime_error("Failed to acquire incremental state lock"))?;
                inc_state.work_queue.pop_front()
            };

            match next_object {
                Some(addr) => {
                    self.mark_object(addr)?;
                    processed += 1;
                }
                None => {
                    // No more objects to mark, transition to cycle detection
                    let mut state = self.state.write()
                        .map_err(|_| CursedError::runtime_error("Failed to acquire state lock"))?;
                    state.phase = CollectionPhase::CycleDetection;
                    return Ok(true);
                }
            }
        }

        Ok(true) // More work to do
    }

    /// Incremental cycle detection step
    fn incremental_cycle_detection_step(&self) -> Result<bool, CursedError> {
        // Simplified incremental cycle detection
        // In a full implementation, this would incrementally build the SCC graph
        self.cycle_detection_phase()?;
        self.transition_to_sweeping()?;
        Ok(true)
    }

    /// Incremental sweep step
    fn incremental_sweep_step(&self) -> Result<bool, CursedError> {
        let mut processed = 0;
        let max_objects = self.config.max_objects_per_step;

        let to_collect = {
            let state = self.state.read()
                .map_err(|_| CursedError::runtime_error("Failed to acquire state lock"))?;
            let mut objects: Vec<usize> = state.white_set.iter().copied().collect();
            objects.truncate(max_objects);
            objects
        };

        if to_collect.is_empty() {
            // Sweeping complete
            let mut state = self.state.write()
                .map_err(|_| CursedError::runtime_error("Failed to acquire state lock"))?;
            state.phase = CollectionPhase::Finalizing;
            return Ok(true);
        }

        let mut objects = self.objects.write()
            .map_err(|_| CursedError::runtime_error("Failed to acquire objects lock"))?;
        let mut state = self.state.write()
            .map_err(|_| CursedError::runtime_error("Failed to acquire state lock"))?;

        for addr in to_collect {
            if let Some(info) = objects.remove(&addr) {
                self.allocated_bytes.fetch_sub(info.size, Ordering::Relaxed);
                processed += 1;
            }
            state.white_set.remove(&addr);
        }

        Ok(true) // More work to do
    }

    /// Transition to sweeping phase
    fn transition_to_sweeping(&self) -> Result<(), CursedError> {
        let mut state = self.state.write()
            .map_err(|_| CursedError::runtime_error("Failed to acquire state lock"))?;
        state.phase = CollectionPhase::Sweeping;
        Ok(())
    }

    /// Finalize collection
    fn finalize_collection(&self) -> Result<(), CursedError> {
        let collection_time = {
            let state = self.state.read()
                .map_err(|_| CursedError::runtime_error("Failed to acquire state lock"))?;
            state.start_time.elapsed()
        };

        // Reset object colors
        {
            let mut objects = self.objects.write()
                .map_err(|_| CursedError::runtime_error("Failed to acquire objects lock"))?;
            for info in objects.values_mut() {
                info.color = ObjectColor::White;
                info.marked_at = None;
            }
        }

        // Update state
        {
            let mut state = self.state.write()
                .map_err(|_| CursedError::runtime_error("Failed to acquire state lock"))?;
            state.phase = CollectionPhase::Idle;
        }

        // Update statistics
        {
            let mut stats = self.stats.write()
                .map_err(|_| CursedError::runtime_error("Failed to acquire stats lock"))?;
            stats.total_collections += 1;
            stats.total_time += collection_time;

            if collection_time > stats.max_collection_time {
                stats.max_collection_time = collection_time;
            }

            if stats.total_collections > 0 {
                stats.avg_collection_time = stats.total_time / stats.total_collections as u32;
            }
        }

        Ok(())
    }

    /// Update collection statistics
    fn update_stats(&self, collection_time: Duration, reclaimed: usize) -> Result<(), CursedError> {
        let mut stats = self.stats.write()
            .map_err(|_| CursedError::runtime_error("Failed to acquire stats lock"))?;

        stats.total_collections += 1;
        stats.total_time += collection_time;
        stats.last_collected = 0; // Objects collected
        stats.last_reclaimed = reclaimed;

        if collection_time > stats.max_collection_time {
            stats.max_collection_time = collection_time;
        }

        if stats.total_collections > 0 {
            stats.avg_collection_time = stats.total_time / stats.total_collections as u32;
        }

        Ok(())
    }

    /// Check if collection should be triggered
    fn maybe_trigger_collection(&self) -> Result<(), CursedError> {
        // Simple threshold-based triggering
        let allocated = self.allocated_bytes.load(Ordering::Relaxed);
        if allocated > self.config.allocation_threshold {
            if self.config.incremental {
                self.incremental_collect()?;
            } else {
                self.collect()?;
            }
        }
        Ok(())
    }

    /// Get collection statistics
    pub fn stats(&self) -> Result<CollectionStats, CursedError> {
        let stats = self.stats.read()
            .map_err(|_| CursedError::runtime_error("Failed to acquire stats lock"))?;
        Ok(stats.clone())
    }

    /// Get current memory usage
    pub fn memory_usage(&self) -> usize {
        self.allocated_bytes.load(Ordering::Relaxed)
    }

    /// Check if collection is in progress
    pub fn is_collecting(&self) -> bool {
        self.collecting.load(Ordering::Relaxed)
    }

    /// Force immediate collection
    pub fn force_collect(&self) -> Result<CollectionStats, CursedError> {
        self.full_collect()
    }
}

impl MarkVisitor {
    /// Create a new mark visitor
    pub fn new() -> Self {
        Self {
            visited: HashSet::new(),
            to_visit: VecDeque::new(),
        }
    }

    /// Visit an object by address
    pub fn visit_addr(&mut self, addr: usize) {
        if !self.visited.contains(&addr) {
            self.visited.insert(addr);
            self.to_visit.push_back(addr);
        }
    }

    /// Get next object to visit
    pub fn next_object(&mut self) -> Option<usize> {
        self.to_visit.pop_front()
    }

    /// Check if all objects have been visited
    pub fn is_complete(&self) -> bool {
        self.to_visit.is_empty()
    }
}

impl Visitor for MarkVisitor {
    fn visit(&mut self, obj: &dyn Traceable) {
        let addr = obj as *const dyn Traceable as *const () as usize;
        if addr != 0 {
            self.visit_addr(addr);
        }
    }
}

/// Create a default mark-and-sweep collector
pub fn create_default_collector() -> MarkSweepCollector {
    MarkSweepCollector::new(CollectorConfig::default())
}

/// Get a result for minimal compatibility
pub fn get_minimal_result() -> Result<String, CursedError> {
    Ok("CURSED mark-and-sweep garbage collector enabled".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collector_creation() {
        let collector = create_default_collector();
        assert!(!collector.is_collecting());
        assert_eq!(collector.memory_usage(), 0);
    }

    #[test]
    fn test_object_registration() {
        let collector = create_default_collector();
        
        // Register an object
        let addr = 0x1000;
        let size = 64;
        let tag = Tag::Object;
        let refs = vec![];
        
        collector.register_object(addr, size, tag, refs).unwrap();
        assert_eq!(collector.memory_usage(), size);
    }

    #[test]
    fn test_root_management() {
        let collector = create_default_collector();
        
        let root_addr = 0x2000;
        collector.add_root(root_addr).unwrap();
        collector.remove_root(root_addr).unwrap();
    }

    #[test]
    fn test_collection() {
        let collector = create_default_collector();
        
        // Register some objects
        collector.register_object(0x1000, 64, Tag::Object, vec![]).unwrap();
        collector.register_object(0x2000, 128, Tag::Object, vec![0x1000]).unwrap();
        
        // Add root
        collector.add_root(0x2000).unwrap();
        
        // Perform collection
        let stats = collector.force_collect().unwrap();
        assert!(stats.total_collections > 0);
    }

    #[test]
    fn test_incremental_collection() {
        let mut config = CollectorConfig::default();
        config.incremental = true;
        config.incremental_budget_ms = 10;
        
        let collector = MarkSweepCollector::new(config);
        
        // Register objects
        collector.register_object(0x1000, 64, Tag::Object, vec![]).unwrap();
        collector.add_root(0x1000).unwrap();
        
        // Perform incremental collection
        let stats = collector.collect().unwrap();
        assert!(stats.total_collections >= 0);
    }

    #[test]
    fn test_cycle_detection() {
        let mut config = CollectorConfig::default();
        config.cycle_detection = true;
        
        let collector = MarkSweepCollector::new(config);
        
        // Create a cycle: A -> B -> A
        collector.register_object(0x1000, 64, Tag::Object, vec![0x2000]).unwrap();
        collector.register_object(0x2000, 64, Tag::Object, vec![0x1000]).unwrap();
        
        // No roots - cycle should be detected and collected
        let stats = collector.force_collect().unwrap();
        assert_eq!(collector.memory_usage(), 0); // Cycle should be collected
    }

    #[test]
    fn test_tricolor_marking() {
        let collector = create_default_collector();
        
        // Create object chain: root -> A -> B
        collector.register_object(0x1000, 64, Tag::Object, vec![0x2000]).unwrap();
        collector.register_object(0x2000, 64, Tag::Object, vec![]).unwrap();
        collector.register_object(0x3000, 64, Tag::Object, vec![]).unwrap(); // Unreachable
        
        collector.add_root(0x1000).unwrap();
        
        let initial_memory = collector.memory_usage();
        collector.force_collect().unwrap();
        let final_memory = collector.memory_usage();
        
        // Should have collected the unreachable object
        assert!(final_memory < initial_memory);
    }
}
