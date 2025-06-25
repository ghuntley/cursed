/// Enhanced Incremental Garbage Collection with Comprehensive Mutator Coordination
/// 
/// This module provides advanced incremental collection capabilities designed to minimize
/// pause times through sophisticated work scheduling, time-bounded collection phases,
/// and seamless integration with goroutine-aware collection:
///
/// 1. **Time-Bounded Collection**: Configurable pause time targets with adaptive work quantum
/// 2. **Mutator Coordination**: Write barriers, remembered sets, and safe point coordination  
/// 3. **Resumable Collection State**: Full collection state persistence and resumption
/// 4. **Work Scheduling**: Prioritized work queues with incremental mark and sweep phases
/// 5. **Goroutine Integration**: Cooperative collection with concurrent goroutine support
/// 6. **Progress Tracking**: Comprehensive monitoring and adaptive performance tuning

use std::sync::{Arc, RwLock, Mutex, Condvar};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{Duration, Instant};
use std::thread;
use tracing::{instrument, debug, info, warn, error, span, Level};

use crate::memory::{Traceable, Visitor};
use crate::memory::object_id::{ObjectId, ObjectRegistry, SharedObjectRegistry};
use crate::memory::roots::{RootSetManager, RootType};

/// Reference collector for traversing object references
struct ReferenceCollector {
impl ReferenceCollector {
    fn new() -> Self {
        Self {
        }
    }
    
    fn into_references(self) -> Vec<ObjectId> {
        self.references
    }
}

impl Visitor for ReferenceCollector {
    fn visit(&mut self, obj: &dyn Traceable) {
        // For now, we'll need to extract ObjectId from the object
        // This is a placeholder implementation
        // TODO: Enhance Traceable trait to provide object IDs
    }
}

/// Forwarding pointer for object relocation
#[derive(Debug, Clone)]
struct ForwardingPointer {
/// Weak reference for incremental processing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WeakReference {
/// Finalizer entry for objects requiring cleanup
#[derive(Debug, Clone)]
pub struct FinalizerEntry {
/// Types of incremental collection work
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IncrementalWorkType {
    /// Marking work (traversing object graph)
    /// Sweeping work (reclaiming memory)
    /// Reference processing (weak references, finalizers)
    /// Remembered set processing (cross-generational references)
    /// Object relocation (compaction)
/// Configuration for incremental collection
#[derive(Debug, Clone)]
pub struct IncrementalConfig {
    /// Maximum time per incremental step
    /// Minimum time between incremental steps
    /// Work quantum size (objects processed per step)
    /// Enable adaptive step sizing
    /// Target allocation/collection ratio
    /// Enable concurrent collection
    /// Write barrier overhead threshold
    /// Enable remembered set optimization
    /// Maximum remembered set size
impl Default for IncrementalConfig {
    fn default() -> Self {
        Self {
            allocation_collection_ratio: 0.1, // 10% collection overhead
            write_barrier_threshold: 0.05, // 5% overhead
        }
    }
/// Statistics for incremental collection
#[derive(Debug, Clone)]
pub struct IncrementalStats {
impl Default for IncrementalStats {
    fn default() -> Self {
        Self {
        }
    }
/// Work item for incremental processing
#[derive(Debug, Clone)]
pub struct IncrementalWorkItem {
/// Write barrier record for tracking object modifications
#[derive(Debug, Clone)]
pub struct WriteBarrierRecord {
/// Remembered set entry for cross-generational references
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RememberedSetEntry {
/// Enhanced collection state for comprehensive incremental processing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CollectionState {
    /// No collection in progress
    /// Initial collection preparation
    /// Root scanning phase in progress
    /// Marking phase in progress  
    /// Concurrent marking (with mutator running)
    /// Final marking phase (stop-the-world)
    /// Sweeping phase in progress
    /// Reference processing in progress
    /// Finalization processing
    /// Relocation phase in progress
    /// Collection completion and cleanup
/// Mutator coordination state for safe point management
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MutatorState {
    /// Mutator running normally
    /// Mutator requested to reach safe point
    /// Mutator at safe point, GC can proceed
    /// Mutator suspended for GC
/// Progress tracking for collection phases
#[derive(Debug, Clone, Default)]
pub struct CollectionProgress {
    /// Current phase
    /// Phase start time
    /// Total work units in current phase
    /// Completed work units in current phase
    /// Estimated time to completion
    /// Work rate (units per second)
    /// Phase progress percentage
impl CollectionProgress {
    /// Update progress with new work completion
    pub fn update_progress(&mut self, completed_units: usize) {
        self.completed_work_units = completed_units;
        
        if self.total_work_units > 0 {
            self.progress_percentage = (self.completed_work_units as f64 / self.total_work_units as f64) * 100.0;
        // Update work rate and estimated completion
        if let Some(start_time) = self.phase_start_time {
            let elapsed = start_time.elapsed();
            if elapsed.as_secs_f64() > 0.0 {
                self.work_rate = self.completed_work_units as f64 / elapsed.as_secs_f64();
                
                if self.work_rate > 0.0 {
                    let remaining_work = self.total_work_units.saturating_sub(self.completed_work_units);
                    let estimated_seconds = remaining_work as f64 / self.work_rate;
                    self.estimated_completion = Some(Duration::from_secs_f64(estimated_seconds));
                }
            }
        }
    }
/// Collection resumption state for persistent collection across interruptions
#[derive(Debug, Clone)]
pub struct CollectionCheckpoint {
    /// Collection phase when checkpointed
    /// Checkpoint timestamp
    /// Serialized work queue state
    /// Marked objects at checkpoint
    /// Sweep candidates at checkpoint
    /// Root scan progress
    /// Collection statistics at checkpoint
    /// Phase-specific data
impl CollectionCheckpoint {
    /// Create a new checkpoint
    pub fn new(phase: CollectionState) -> Self {
        Self {
        }
    }
    
    /// Check if checkpoint is still valid (not too old)
    pub fn is_valid(&self, max_age: Duration) -> bool {
        self.timestamp.elapsed() <= max_age
    }
}

/// Enhanced incremental garbage collection coordinator with comprehensive mutator coordination
pub struct IncrementalCollector {
    
    /// Enhanced collection state and coordination
    
    /// Work scheduling and coordination
    /// Time budget for current collection step
    current_time_budget: std::sync::atomic::AtomicU64, // nanoseconds
    /// Pause time target for adaptive scheduling
    pause_time_target: std::sync::atomic::AtomicU64, // nanoseconds
    
    /// Mutator coordination and safe points
    /// Pending safe point requests by thread ID
    /// Threads currently at safe points
    
    /// Memory barriers and write tracking
    
    /// Collection data structures
    
    /// Reference management
    
    /// Progress tracking and scheduling
    
    /// Performance monitoring and adaptation
    
    /// Background collection coordination
    
    /// Goroutine integration
    goroutine_safe_points: RwLock<HashMap<u64, Instant>>, // goroutine_id -> safe_point_time
/// Safe point coordinator for managing mutator pauses
#[derive(Debug)]
struct SafePointCoordinator {
    /// Target threads that need to reach safe points
    /// Timeout for safe point coordination
    /// Last coordination attempt
impl SafePointCoordinator {
    fn new() -> Self {
        Self {
        }
    }
    
    /// Request safe point from specific threads
    fn request_safe_points(&mut self, thread_ids: Vec<thread::ThreadId>) {
        self.target_threads.extend(thread_ids);
        self.last_coordination = Some(Instant::now());
    /// Check if coordination has timed out
    fn has_timed_out(&self) -> bool {
        if let Some(last) = self.last_coordination {
            last.elapsed() > self.coordination_timeout
        } else {
            false
        }
    }
/// Adaptive scheduler for work quantum and timing optimization
#[derive(Debug)]
struct AdaptiveScheduler {
    /// Recent step durations for analysis
    /// Target step duration
    /// Current work quantum
    /// Quantum adjustment history
    /// Performance trend analysis
impl AdaptiveScheduler {
    fn new(target_duration: Duration, initial_quantum: usize) -> Self {
        Self {
        }
    }
    
    /// Adjust work quantum based on recent performance
    fn adjust_quantum(&mut self, last_duration: Duration) -> usize {
        self.recent_step_durations.push_back(last_duration);
        if self.recent_step_durations.len() > 20 {
            self.recent_step_durations.pop_front();
        if self.recent_step_durations.len() < 3 {
            return self.current_quantum;
        let avg_duration = self.recent_step_durations.iter().sum::<Duration>() / self.recent_step_durations.len() as u32;
        
        let adjustment_factor = if avg_duration > self.target_step_duration {
            // Too slow, reduce quantum
            0.9
        } else if avg_duration < self.target_step_duration / 2 {
            // Too fast, increase quantum
            1.1
        } else {
            // Just right
            1.0
        
        let new_quantum = ((self.current_quantum as f64) * adjustment_factor) as usize;
        let new_quantum = new_quantum.max(1).min(10000); // Reasonable bounds
        
        if new_quantum != self.current_quantum {
            self.quantum_adjustments += 1;
            self.current_quantum = new_quantum;
        self.current_quantum
    }
}

/// Goroutine GC coordinator for integration with concurrent execution
#[derive(Debug)]
pub struct GoroutineGcCoordinator {
    /// Active goroutine IDs
    /// Goroutine local roots
    /// Goroutine stack scan progress
    /// Coordination state
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GoroutineState {
impl GoroutineGcCoordinator {
    pub fn new() -> Self {
        Self {
        }
    }
    
    /// Register a new goroutine
    pub fn register_goroutine(&self, goroutine_id: u64) -> Result<(), String> {
        let mut active = self.active_goroutines.write()
            .map_err(|_| "Failed to acquire write lock on active goroutines")?;
        active.insert(goroutine_id);
        
        let mut state = self.coordination_state.write()
            .map_err(|_| "Failed to acquire write lock on coordination state")?;
        state.insert(goroutine_id, GoroutineState::Running);
        
        debug!("Registered goroutine {} for GC coordination", goroutine_id);
        Ok(())
    /// Unregister a goroutine
    pub fn unregister_goroutine(&self, goroutine_id: u64) -> Result<(), String> {
        let mut active = self.active_goroutines.write()
            .map_err(|_| "Failed to acquire write lock on active goroutines")?;
        active.remove(&goroutine_id);
        
        let mut state = self.coordination_state.write()
            .map_err(|_| "Failed to acquire write lock on coordination state")?;
        state.remove(&goroutine_id);
        
        let mut roots = self.goroutine_roots.write()
            .map_err(|_| "Failed to acquire write lock on goroutine roots")?;
        roots.remove(&goroutine_id);
        
        debug!("Unregistered goroutine {} from GC coordination", goroutine_id);
        Ok(())
    /// Request safe points from all active goroutines
    pub fn request_global_safe_point(&self) -> Result<(), String> {
        let active = self.active_goroutines.read()
            .map_err(|_| "Failed to acquire read lock on active goroutines")?;
        
        let mut state = self.coordination_state.write()
            .map_err(|_| "Failed to acquire write lock on coordination state")?;
        
        for &goroutine_id in active.iter() {
            state.insert(goroutine_id, GoroutineState::SafePointRequested);
        debug!("Requested safe points from {} goroutines", active.len());
        Ok(())
    }
}

impl IncrementalCollector {
    /// Create a new incremental collector
    pub fn new(object_registry: SharedObjectRegistry) -> Self {
        Self::with_config(object_registry, IncrementalConfig::default())
    /// Create a new incremental collector with custom configuration
    #[instrument(skip(object_registry, config))]
    pub fn with_config(object_registry: SharedObjectRegistry, config: IncrementalConfig) -> Self {
        info!("Creating enhanced incremental collector with config: {:?}", config);
        
        Self {
            
            // Enhanced collection state and coordination
            
            // Work scheduling and coordination
            
            // Mutator coordination and safe points
            
            // Memory barriers and write tracking
            
            // Collection data structures
            
            // Reference management
            
            // Progress tracking and scheduling
            
            // Performance monitoring and adaptation
            
            // Background collection coordination
            
            // Goroutine integration
        }
    }
    
    /// Set the root set manager
    pub fn set_root_manager(&mut self, root_manager: Arc<RootSetManager>) {
        self.root_manager = Some(root_manager);
    /// Start incremental collection
    #[instrument(skip(self))]
    pub fn start_collection(&self) -> Result<(), String> {
        info!("Starting incremental collection");
        
        let mut state = self.collection_state.write()
            .map_err(|_| "Failed to acquire write lock on collection state")?;
        
        if *state != CollectionState::Idle {
            return Err("Collection already in progress".to_string());
        *state = CollectionState::Marking;
        
        // Initialize for marking phase
        self.initialize_marking_phase()?;
        
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        if config.concurrent_collection {
            self.start_background_thread()?;
        info!("Incremental collection started");
        Ok(())
    /// Perform one time-bounded incremental collection step with comprehensive coordination
    #[instrument(skip(self))]
    pub fn step(&self) -> Result<bool, String> {
        let _span = span!(Level::DEBUG, "incremental_step").entered();
        let step_start = Instant::now();
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        // Check if we should perform a step
        if !self.should_perform_step()? {
            return Ok(false);
        // Get time budget for this step
        let time_budget = Duration::from_nanos(
            self.current_time_budget.load(std::sync::atomic::Ordering::SeqCst)
        );
        
        let current_state = {
            let state = self.collection_state.read()
                .map_err(|_| "Failed to acquire read lock on collection state")?;
            *state
        
        debug!("Starting incremental step in phase {:?} with time budget {:?}", current_state, time_budget);
        
        // Update progress tracking
        self.update_phase_progress(current_state)?;
        
        // Perform time-bounded work based on current state
        let work_performed = match current_state {
        
        let step_duration = step_start.elapsed();
        
        // Update adaptive scheduler
        if config.adaptive_step_sizing {
            let new_quantum = {
                let mut scheduler = self.adaptive_scheduler.lock()
                    .map_err(|_| "Failed to acquire lock on adaptive scheduler")?;
                scheduler.adjust_quantum(step_duration)
            self.step_work_quantum.store(new_quantum, std::sync::atomic::Ordering::SeqCst);
        // Update statistics and timings
        self.update_step_statistics(current_state, step_duration, work_performed)?;
        self.record_phase_timing(current_state, step_duration)?;
        
        // Update progress tracking
        {
            let mut progress = self.collection_progress.write()
                .map_err(|_| "Failed to acquire write lock on collection progress")?;
            
            // Increment completed work units based on work performed
            if work_performed {
                progress.completed_work_units += 1;
                let completed = progress.completed_work_units;
                progress.update_progress(completed);
            }
        }
        
        // Create checkpoint if this was a significant step
        if work_performed && step_duration > config.max_step_duration / 2 {
            self.create_collection_checkpoint()?;
        // Update last step time
        {
            let mut last_time = self.last_step_time.lock()
                .map_err(|_| "Failed to acquire lock on last step time")?;
            *last_time = Some(Instant::now());
               step_duration, work_performed, current_state);
        Ok(work_performed)
    /// Perform time-bounded step with explicit budget enforcement
    #[instrument(skip(self))]
    pub fn step_with_budget(&self, time_budget: Duration) -> Result<bool, String> {
        let _span = span!(Level::DEBUG, "step_with_budget", ?time_budget).entered();
        
        // Store time budget for this step
        self.current_time_budget.store(time_budget.as_nanos() as u64, std::sync::atomic::Ordering::SeqCst);
        
        // Delegate to regular step method
        self.step()
    /// Initialize marking phase
    fn initialize_marking_phase(&self) -> Result<(), String> {
        debug!("Initializing marking phase");
        
        // Clear previous marking state
        {
            let mut marked_objects = self.marked_objects.write()
                .map_err(|_| "Failed to acquire write lock on marked objects")?;
            marked_objects.clear();
        // Reset root scan progress for incremental scanning
        self.root_scan_progress.store(0, std::sync::atomic::Ordering::SeqCst);
        
        // Clear work queue
        {
            let mut work_queue = self.work_queue.lock()
                .map_err(|_| "Failed to acquire lock on work queue")?;
            work_queue.clear();
        // Start with an initial batch of roots using time-sliced scanning
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        let initial_root_batch = self.scan_roots_incremental(config.work_quantum)?;
        
        {
            let mut work_queue = self.work_queue.lock()
                .map_err(|_| "Failed to acquire lock on work queue")?;
            
            for root_id in initial_root_batch {
                let work_item = IncrementalWorkItem {
                    priority: 255, // High priority for roots
                work_queue.push_back(work_item);
            }
        }
        
        debug!("Marking phase initialized with time-sliced root scanning");
        Ok(())
    /// Preparation step for collection initialization
    fn preparing_step(&self, time_budget: Duration) -> Result<bool, String> {
        let _span = span!(Level::DEBUG, "preparing_step", ?time_budget).entered();
        let step_start = Instant::now();
        
        debug!("Starting collection preparation phase");
        
        // Initialize collection progress tracking
        {
            let mut progress = self.collection_progress.write()
                .map_err(|_| "Failed to acquire write lock on collection progress")?;
            progress.current_phase = Some(CollectionState::Preparing);
            progress.phase_start_time = Some(step_start);
            progress.total_work_units = 100; // Estimated preparation work
            progress.completed_work_units = 0;
        // Clear previous collection state
        {
            let mut marked_objects = self.marked_objects.write()
                .map_err(|_| "Failed to acquire write lock on marked objects")?;
            marked_objects.clear();
        {
            let mut gray_objects = self.gray_objects.write()
                .map_err(|_| "Failed to acquire write lock on gray objects")?;
            gray_objects.clear();
        // Initialize work queues
        {
            let mut work_queue = self.work_queue.lock()
                .map_err(|_| "Failed to acquire lock on work queue")?;
            work_queue.clear();
        {
            let mut priority_queue = self.priority_work_queue.lock()
                .map_err(|_| "Failed to acquire lock on priority work queue")?;
            priority_queue.clear();
        // Reset progress counters
        self.root_scan_progress.store(0, std::sync::atomic::Ordering::SeqCst);
        self.marking_progress.store(0, std::sync::atomic::Ordering::SeqCst);
        self.sweeping_progress.store(0, std::sync::atomic::Ordering::SeqCst);
        
        // Transition to root scanning
        self.transition_to_phase(CollectionState::RootScanning)?;
        
        debug!("Preparation phase completed in {:?}", step_start.elapsed());
        Ok(true)
    /// Root scanning step with time budget
    fn root_scanning_step(&self, time_budget: Duration) -> Result<bool, String> {
        let _span = span!(Level::DEBUG, "root_scanning_step", ?time_budget).entered();
        let step_start = Instant::now();
        
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        // Calculate how many roots we can scan within time budget
        let mut roots_processed = 0;
        let max_roots_per_iteration = config.work_quantum / 4; // Conservative estimate
        
        while step_start.elapsed() < time_budget {
            let root_batch = self.scan_roots_incremental(max_roots_per_iteration)?;
            if root_batch.is_empty() {
                // All roots scanned, transition to marking
                self.transition_to_phase(CollectionState::Marking)?;
                break;
            // Add roots to marking work queue
            {
                let mut work_queue = self.work_queue.lock()
                    .map_err(|_| "Failed to acquire lock on work queue")?;
                
                for root_id in root_batch {
                    let work_item = IncrementalWorkItem {
                        priority: 255, // Highest priority for roots
                    work_queue.push_back(work_item);
                    roots_processed += 1;
                }
            }
            
            // Check time budget
            if step_start.elapsed() > time_budget {
                break;
            }
        }
        
        debug!("Root scanning step processed {} roots in {:?}", roots_processed, step_start.elapsed());
        Ok(roots_processed > 0)
    /// Time-bounded marking step with budget enforcement
    fn marking_step_time_bounded(&self, time_budget: Duration) -> Result<bool, String> {
        let _span = span!(Level::DEBUG, "marking_step_time_bounded", ?time_budget).entered();
        let step_start = Instant::now();
        let mut work_performed = 0;
        
        // Process marking work while within time budget
        while step_start.elapsed() < time_budget {
            let work_item = {
                let mut work_queue = self.work_queue.lock()
                    .map_err(|_| "Failed to acquire lock on work queue")?;
                work_queue.pop_front()
            
            let work_item = match work_item {
                Some(item) => {
                    // Wrong type, put it back and break
                    let mut work_queue = self.work_queue.lock()
                        .map_err(|_| "Failed to acquire lock on work queue")?;
                    work_queue.push_front(item);
                    break;
                }
                None => break, // No more marking work
            
            // Mark the object and process references
            if self.mark_object_incremental(work_item.object_id)? {
                work_performed += 1;
                self.marking_progress.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                
                // Add referenced objects to work queue
                let referenced_objects = self.get_object_references(work_item.object_id)?;
                if !referenced_objects.is_empty() {
                    let mut work_queue = self.work_queue.lock()
                        .map_err(|_| "Failed to acquire lock on work queue")?;
                    
                    for ref_id in referenced_objects {
                        let ref_work_item = IncrementalWorkItem {
                        work_queue.push_back(ref_work_item);
                    }
                }
            // Check time budget every few iterations
            if work_performed % 10 == 0 && step_start.elapsed() > time_budget {
                break;
            }
        }
        
        // Check if marking is complete
        let marking_complete = {
            let work_queue = self.work_queue.lock()
                .map_err(|_| "Failed to acquire lock on work queue")?;
            !work_queue.iter().any(|item| item.work_type == IncrementalWorkType::Marking)
        
        if marking_complete {
            self.transition_to_phase(CollectionState::Sweeping)?;
        debug!("Time-bounded marking step completed {} objects in {:?}", work_performed, step_start.elapsed());
        Ok(work_performed > 0)
    /// Concurrent marking step (marks while mutator runs)
    fn concurrent_marking_step(&self, time_budget: Duration) -> Result<bool, String> {
        let _span = span!(Level::DEBUG, "concurrent_marking_step", ?time_budget).entered();
        let step_start = Instant::now();
        
        // Process write barrier records first
        let barrier_work = self.process_write_barrier_records(time_budget / 4)?;
        
        // Then perform regular marking with remaining budget
        let remaining_budget = time_budget.saturating_sub(step_start.elapsed());
        let marking_work = self.marking_step_time_bounded(remaining_budget)?;
        
        debug!("Concurrent marking step: barrier_work={}, marking_work={}", barrier_work, marking_work);
        Ok(barrier_work || marking_work)
    /// Final marking step (stop-the-world)
    fn final_marking_step(&self, time_budget: Duration) -> Result<bool, String> {
        let _span = span!(Level::DEBUG, "final_marking_step", ?time_budget).entered();
        debug!("Performing final marking phase (stop-the-world)");
        
        // Request safe points from all threads
        self.request_global_safe_point()?;
        
        // Wait for safe points with timeout
        let safe_point_achieved = self.wait_for_safe_points(time_budget / 2)?;
        if !safe_point_achieved {
            warn!("Safe point coordination timed out, proceeding with final marking");
        // Process any remaining write barrier records
        let remaining_budget = time_budget / 2;
        let work_performed = self.process_write_barrier_records(remaining_budget)?;
        
        // Transition to sweeping
        self.transition_to_phase(CollectionState::Sweeping)?;
        
        // Release safe points
        self.release_safe_points()?;
        
        debug!("Final marking completed with safe point coordination");
        Ok(work_performed)
    /// Time-bounded sweeping step
    fn sweeping_step_time_bounded(&self, time_budget: Duration) -> Result<bool, String> {
        let _span = span!(Level::DEBUG, "sweeping_step_time_bounded", ?time_budget).entered();
        let step_start = Instant::now();
        let mut work_performed = 0;
        
        // Process sweep candidates within time budget
        while step_start.elapsed() < time_budget {
            let object_id = {
                let mut sweep_candidates = self.sweep_candidates.lock()
                    .map_err(|_| "Failed to acquire lock on sweep candidates")?;
                sweep_candidates.pop_front()
            
            let object_id = match object_id {
                None => {
                    // No more objects to sweep, transition to reference processing
                    self.transition_to_phase(CollectionState::ReferenceProcessing)?;
                    break;
                }
            
            // Sweep the object
            if self.sweep_object_incremental(object_id)? {
                work_performed += 1;
                self.sweeping_progress.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            // Check time budget every few iterations
            if work_performed % 20 == 0 && step_start.elapsed() > time_budget {
                break;
            }
        }
        
        debug!("Time-bounded sweeping step completed {} objects in {:?}", work_performed, step_start.elapsed());
        Ok(work_performed > 0)
    /// Time-bounded reference processing step
    fn reference_processing_step_time_bounded(&self, time_budget: Duration) -> Result<bool, String> {
        let _span = span!(Level::DEBUG, "reference_processing_step_time_bounded", ?time_budget).entered();
        let step_start = Instant::now();
        let mut work_performed = 0;
        
        // Split time budget between weak references and finalizers
        let weak_ref_budget = time_budget / 2;
        let finalizer_budget = time_budget / 2;
        
        // Process weak references
        let weak_refs = self.get_weak_references()?;
        for weak_ref in weak_refs.iter() {
            if step_start.elapsed() > weak_ref_budget {
                break;
            if self.process_weak_reference(*weak_ref)? {
                work_performed += 1;
            }
        }
        
        // Process finalizers with remaining budget
        let remaining_time = time_budget.saturating_sub(step_start.elapsed());
        let finalizer_quantum = ((remaining_time.as_millis() / 10).max(1)) as usize; // Estimate
        let finalizer_work = self.process_finalizers(finalizer_quantum)?;
        work_performed += finalizer_work;
        
        // Check if reference processing is complete
        let processing_complete = work_performed == 0 || 
            (self.get_weak_references()?.is_empty() && self.get_pending_finalizers()?.is_empty());
        
        if processing_complete {
            self.transition_to_phase(CollectionState::Completing)?;
        debug!("Reference processing step completed {} items in {:?}", work_performed, step_start.elapsed());
        Ok(work_performed > 0)
    /// Finalization step for object cleanup
    fn finalization_step(&self, time_budget: Duration) -> Result<bool, String> {
        let _span = span!(Level::DEBUG, "finalization_step", ?time_budget).entered();
        let step_start = Instant::now();
        
        // Process pending finalizers within time budget
        let finalizer_quantum = ((time_budget.as_millis() / 5).max(1)) as usize;
        let work_performed = self.process_finalizers(finalizer_quantum)?;
        
        // If no more finalizers, transition to completion
        if self.get_pending_finalizers()?.is_empty() {
            self.transition_to_phase(CollectionState::Completing)?;
        debug!("Finalization step completed {} finalizers in {:?}", work_performed, step_start.elapsed());
        Ok(work_performed > 0)
    /// Time-bounded relocation step
    fn relocation_step_time_bounded(&self, time_budget: Duration) -> Result<bool, String> {
        let _span = span!(Level::DEBUG, "relocation_step_time_bounded", ?time_budget).entered();
        let step_start = Instant::now();
        let mut work_performed = 0;
        
        // Get objects that need relocation
        let relocation_candidates = self.get_relocation_candidates()?;
        
        for &object_id in relocation_candidates.iter() {
            if step_start.elapsed() > time_budget {
                break;
            if self.relocate_object(object_id)? {
                work_performed += 1;
            }
        }
        
        // Update forwarding pointers with remaining time
        let remaining_time = time_budget.saturating_sub(step_start.elapsed());
        if remaining_time > Duration::from_millis(1) {
            let forwarding_quantum = ((remaining_time.as_millis() / 2).max(1)) as usize;
            self.update_forwarding_pointers(forwarding_quantum)?;
        // Check if relocation is complete
        let relocation_complete = work_performed == 0 || self.get_relocation_candidates()?.is_empty();
        
        if relocation_complete {
            self.transition_to_phase(CollectionState::Completing)?;
        debug!("Relocation step completed {} objects in {:?}", work_performed, step_start.elapsed());
        Ok(work_performed > 0)
    /// Collection completion step
    fn completing_step(&self, time_budget: Duration) -> Result<bool, String> {
        let _span = span!(Level::DEBUG, "completing_step", ?time_budget).entered();
        let step_start = Instant::now();
        
        debug!("Completing collection cycle");
        
        // Process remembered set updates
        self.process_remembered_set_updates()?;
        
        // Clear collection checkpoint
        {
            let mut checkpoint = self.collection_checkpoint.write()
                .map_err(|_| "Failed to acquire write lock on collection checkpoint")?;
            *checkpoint = None;
        // Update final progress
        {
            let mut progress = self.collection_progress.write()
                .map_err(|_| "Failed to acquire write lock on collection progress")?;
            progress.progress_percentage = 100.0;
            progress.estimated_completion = Some(Duration::ZERO);
        // Transition back to idle
        self.transition_to_phase(CollectionState::Idle)?;
        
        info!("Incremental collection cycle completed in {:?}", step_start.elapsed());
        Ok(true)
    /// Generic phase transition with progress tracking
    fn transition_to_phase(&self, new_phase: CollectionState) -> Result<(), String> {
        let _span = span!(Level::DEBUG, "transition_to_phase", ?new_phase).entered();
        debug!("Transitioning to collection phase: {:?}", new_phase);
        
        // Update collection state
        {
            let mut state = self.collection_state.write()
                .map_err(|_| "Failed to acquire write lock on collection state")?;
            *state = new_phase;
        // Update progress tracking
        {
            let mut progress = self.collection_progress.write()
                .map_err(|_| "Failed to acquire write lock on collection progress")?;
            progress.current_phase = Some(new_phase);
            progress.phase_start_time = Some(Instant::now());
            
            // Reset progress for new phase
            match new_phase {
                CollectionState::RootScanning => {
                    let root_count = self.get_root_objects()?.len();
                    progress.total_work_units = root_count;
                    progress.completed_work_units = 0;
                }
                CollectionState::Marking => {
                    // Estimate marking work based on object count
                    let object_count = self.object_registry.object_count().unwrap_or(1000);
                    progress.total_work_units = object_count;
                    progress.completed_work_units = 0;
                    self.total_marking_work.store(object_count, std::sync::atomic::Ordering::SeqCst);
                }
                CollectionState::Sweeping => {
                    // Initialize sweeping phase
                    let unmarked_objects = self.get_unmarked_objects()?;
                    {
                        let mut sweep_candidates = self.sweep_candidates.lock()
                            .map_err(|_| "Failed to acquire lock on sweep candidates")?;
                        sweep_candidates.clear();
                        sweep_candidates.extend(unmarked_objects.iter().cloned());
                    }
                    progress.total_work_units = unmarked_objects.len();
                    progress.completed_work_units = 0;
                    self.total_sweeping_work.store(unmarked_objects.len(), std::sync::atomic::Ordering::SeqCst);
                    debug!("Initialized sweeping phase with {} candidates", unmarked_objects.len());
                }
                CollectionState::ReferenceProcessing => {
                    let weak_refs = self.get_weak_references()?.len();
                    let finalizers = self.get_pending_finalizers()?.len();
                    progress.total_work_units = weak_refs + finalizers;
                    progress.completed_work_units = 0;
                }
                _ => {
                    // Default work estimate for other phases
                    progress.total_work_units = 50;
                    progress.completed_work_units = 0;
                }
            }
        debug!("Phase transition completed: {:?}", new_phase);
        Ok(())
    /// Transition from marking to sweeping phase (legacy method)
    fn transition_to_sweeping(&self) -> Result<(), String> {
        self.transition_to_phase(CollectionState::Sweeping)
    /// Perform one sweeping step
    fn sweeping_step(&self) -> Result<bool, String> {
        let work_quantum = self.step_work_quantum.load(std::sync::atomic::Ordering::SeqCst);
        let mut work_performed = 0;
        
        for _ in 0..work_quantum {
            let object_id = {
                let mut sweep_candidates = self.sweep_candidates.lock()
                    .map_err(|_| "Failed to acquire lock on sweep candidates")?;
                sweep_candidates.pop_front()
            
            let object_id = match object_id {
                None => break, // No more objects to sweep
            
            // Sweep the object
            if self.sweep_object_incremental(object_id)? {
                work_performed += 1;
            }
        }
        
        // Check if sweeping is complete
        let sweeping_complete = {
            let sweep_candidates = self.sweep_candidates.lock()
                .map_err(|_| "Failed to acquire lock on sweep candidates")?;
            sweep_candidates.is_empty()
        
        if sweeping_complete {
            self.complete_collection()?;
        Ok(work_performed > 0)
    /// Perform reference processing step
    fn reference_processing_step(&self) -> Result<bool, String> {
        debug!("Starting reference processing step");
        let work_quantum = self.step_work_quantum.load(std::sync::atomic::Ordering::SeqCst);
        let mut work_performed = 0;
        
        // Process weak references first
        let weak_refs = self.get_weak_references()?;
        for (i, weak_ref) in weak_refs.iter().enumerate() {
            if i >= work_quantum {
                // Time slice exhausted, continue in next step
                break;
            if self.process_weak_reference(*weak_ref)? {
                work_performed += 1;
            }
        }
        
        // Process finalizers if we have remaining quantum
        if work_performed < work_quantum {
            let remaining_quantum = work_quantum - work_performed;
            let finalizer_work = self.process_finalizers(remaining_quantum)?;
            work_performed += finalizer_work;
        // Check if reference processing is complete
        let processing_complete = work_performed == 0 || 
            (self.get_weak_references()?.is_empty() && self.get_pending_finalizers()?.is_empty());
        
        if processing_complete {
            self.transition_to_relocation()?;
        debug!("Reference processing step completed, work performed: {}", work_performed);
        Ok(work_performed > 0)
    /// Perform relocation step
    fn relocation_step(&self) -> Result<bool, String> {
        debug!("Starting relocation step");
        let work_quantum = self.step_work_quantum.load(std::sync::atomic::Ordering::SeqCst);
        let mut work_performed = 0;
        
        // Get objects that need relocation
        let relocation_candidates = self.get_relocation_candidates()?;
        
        for (i, &object_id) in relocation_candidates.iter().enumerate() {
            if i >= work_quantum {
                // Time slice exhausted, continue in next step
                break;
            if self.relocate_object(object_id)? {
                work_performed += 1;
            }
        }
        
        // Update forwarding pointers in this step
        if work_performed > 0 {
            self.update_forwarding_pointers(work_quantum - work_performed)?;
        // Check if relocation is complete
        let relocation_complete = work_performed == 0 || self.get_relocation_candidates()?.is_empty();
        
        if relocation_complete {
            self.complete_collection()?;
        debug!("Relocation step completed, work performed: {}", work_performed);
        Ok(work_performed > 0)
    /// Complete the collection cycle
    fn complete_collection(&self) -> Result<(), String> {
        debug!("Completing collection cycle");
        
        {
            let mut state = self.collection_state.write()
                .map_err(|_| "Failed to acquire write lock on collection state")?;
            *state = CollectionState::Idle;
        // Process remembered set updates
        self.process_remembered_set_updates()?;
        
        info!("Incremental collection cycle completed");
        Ok(())
    /// Mark an object incrementally
    fn mark_object_incremental(&self, object_id: ObjectId) -> Result<bool, String> {
        let mut marked_objects = self.marked_objects.write()
            .map_err(|_| "Failed to acquire write lock on marked objects")?;
        
        if marked_objects.contains(&object_id) {
            return Ok(false); // Already marked
        // Verify object exists
        if !self.object_exists(object_id)? {
            return Ok(false);
        marked_objects.insert(object_id);
        debug!("Marked object {} incrementally", object_id);
        Ok(true)
    /// Sweep an object incrementally
    fn sweep_object_incremental(&self, object_id: ObjectId) -> Result<bool, String> {
        debug!("Sweeping object {} incrementally", object_id);
        
        // Check if object has finalizer
        if self.has_finalizer(object_id)? {
            self.schedule_finalizer(object_id)?;
            return Ok(true);
        // Remove object from registry first
        match self.object_registry.unregister(object_id) {
            Ok(Some(metadata)) => {
                debug!("Removed object {} from registry (size: {} bytes)", object_id, metadata.size());
                
                // Update statistics
                {
                    let mut stats = self.stats.write()
                        .map_err(|_| "Failed to acquire write lock on stats")?;
                    stats.objects_swept += 1;
                    stats.bytes_reclaimed += metadata.size() as u64;
                }
            }
            Ok(None) => {
                debug!("Object {} was already removed from registry", object_id);
            }
            Err(e) => {
                warn!("Failed to remove object {} from registry: {}", object_id, e);
                return Err(format!("Failed to remove object {}: {}", object_id, e));
            }
        }
        
        Ok(true)
    /// Write barrier for concurrent/incremental collection
    #[instrument(skip(self))]
    pub fn write_barrier(&self, object_id: ObjectId, field_offset: usize, old_value: Option<ObjectId>, new_value: ObjectId) -> Result<(), String> {
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        let current_state = {
            let state = self.collection_state.read()
                .map_err(|_| "Failed to acquire read lock on collection state")?;
            *state
        
        // Only process write barrier if collection is active
        if current_state != CollectionState::Idle {
            let record = WriteBarrierRecord {
            
            let mut records = self.write_barrier_records.lock()
                .map_err(|_| "Failed to acquire lock on write barrier records")?;
            records.push_back(record);
            
            // Update remembered set if this is a cross-generational reference
            self.update_remembered_set(object_id, new_value, field_offset)?;
        // Update statistics
        {
            let mut stats = self.stats.write()
                .map_err(|_| "Failed to acquire write lock on stats")?;
            stats.write_barrier_hits += 1;
        debug!("Write barrier processed for object {} field {}", object_id, field_offset);
        Ok(())
    /// Update remembered set with cross-generational reference
    fn update_remembered_set(&self, from_object: ObjectId, to_object: ObjectId, field_offset: usize) -> Result<(), String> {
        // TODO: Check if this is actually a cross-generational reference
        // For now, we'll add all references to the remembered set
        
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        if !config.remembered_set_optimization {
            return Ok(());
        let entry = RememberedSetEntry {
        
        let mut remembered_set = self.remembered_set.write()
            .map_err(|_| "Failed to acquire write lock on remembered set")?;
        
        // Check size limit
        if remembered_set.len() >= config.max_remembered_set_size {
            warn!("Remembered set size limit reached, clearing old entries");
            remembered_set.clear();
        remembered_set.insert(entry);
        
        debug!("Updated remembered set: {} -> {} at offset {}", from_object, to_object, field_offset);
        Ok(())
    /// Process remembered set updates
    fn process_remembered_set_updates(&self) -> Result<(), String> {
        debug!("Processing remembered set updates");
        
        // Process write barrier records
        let records = {
            let mut barrier_records = self.write_barrier_records.lock()
                .map_err(|_| "Failed to acquire lock on write barrier records")?;
            let records: Vec<_> = barrier_records.drain(..).collect();
            records
        
        for record in records {
            // TODO: Process each write barrier record
            debug!("Processing write barrier record for object {}", record.object_id);
        // Update statistics
        {
            let mut stats = self.stats.write()
                .map_err(|_| "Failed to acquire write lock on stats")?;
            let remembered_set = self.remembered_set.read()
                .map_err(|_| "Failed to acquire read lock on remembered set")?;
            stats.remembered_set_size = remembered_set.len();
        Ok(())
    /// Check if we should perform an incremental step
    fn should_perform_step(&self) -> Result<bool, String> {
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        // Check minimum interval
        if let Some(last_time) = *self.last_step_time.lock()
            .map_err(|_| "Failed to acquire lock on last step time")? {
            if Instant::now().duration_since(last_time) < config.min_step_interval {
                return Ok(false);
            }
        }
        
        // Check allocation pressure
        let allocation_since_last = self.allocation_since_last_step.load(std::sync::atomic::Ordering::SeqCst);
        let target_work = (allocation_since_last as f64 * config.allocation_collection_ratio) as usize;
        
        Ok(target_work > 0)
    /// Adjust work quantum based on step performance
    fn adjust_work_quantum(&self, step_duration: Duration) -> Result<(), String> {
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        let current_quantum = self.step_work_quantum.load(std::sync::atomic::Ordering::SeqCst);
        let target_duration = config.max_step_duration;
        
        let new_quantum = if step_duration > target_duration {
            // Step took too long, reduce quantum
            (current_quantum * 90 / 100).max(1) // Reduce by 10%
        } else if step_duration < target_duration / 2 {
            // Step was too fast, increase quantum
            (current_quantum * 110 / 100).min(config.work_quantum * 2) // Increase by 10%
        } else {
            current_quantum // No change
        
        if new_quantum != current_quantum {
            self.step_work_quantum.store(new_quantum, std::sync::atomic::Ordering::SeqCst);
            
            let mut stats = self.stats.write()
                .map_err(|_| "Failed to acquire write lock on stats")?;
            stats.work_quantum_adjustments += 1;
            
            debug!("Adjusted work quantum: {} -> {}", current_quantum, new_quantum);
        Ok(())
    /// Start background collection thread
    fn start_background_thread(&self) -> Result<(), String> {
        let mut thread_handle = self.background_thread.lock()
            .map_err(|_| "Failed to acquire lock on background thread")?;
        
        if thread_handle.is_some() {
            return Ok(());
        let collector_ref = unsafe { std::mem::transmute::<&IncrementalCollector, &'static IncrementalCollector>(self) };
        
        let handle = std::thread::spawn(move || {
            info!("Starting background incremental collection thread");
            collector_ref.background_collection_loop();
            info!("Background incremental collection thread stopped");
        });
        
        *thread_handle = Some(handle);
        Ok(())
    /// Background collection loop
    fn background_collection_loop(&self) {
        while !self.should_stop.load(std::sync::atomic::Ordering::SeqCst) {
            if let Err(e) = self.step() {
                error!("Background collection step failed: {}", e);
            // Sleep for a short time to avoid busy waiting
            std::thread::sleep(Duration::from_millis(1));
        }
    }
    
    /// Stop background collection
    pub fn stop_background_collection(&self) -> Result<(), String> {
        self.should_stop.store(true, std::sync::atomic::Ordering::SeqCst);
        
        let handle = {
            let mut thread_handle = self.background_thread.lock()
                .map_err(|_| "Failed to acquire lock on background thread")?;
            thread_handle.take()
        
        if let Some(handle) = handle {
            if let Err(e) = handle.join() {
                error!("Failed to join background thread: {:?}", e);
            }
        }
        
        Ok(())
    /// Notify of allocation for step scheduling
    pub fn notify_allocation(&self, bytes: usize) {
        self.allocation_since_last_step.fetch_add(bytes, std::sync::atomic::Ordering::SeqCst);
    /// Get unmarked objects that should be swept
    fn get_unmarked_objects(&self) -> Result<Vec<ObjectId>, String> {
        let marked_objects = self.marked_objects.read()
            .map_err(|_| "Failed to acquire read lock on marked objects")?;
        
        let all_objects = self.object_registry.get_all_objects()?;
        
        let unmarked: Vec<ObjectId> = all_objects
            .into_iter()
            .filter(|&object_id| !marked_objects.contains(&object_id))
            .collect();
        
        Ok(unmarked)
    /// Get all root objects
    fn get_root_objects(&self) -> Result<Vec<ObjectId>, String> {
        if let Some(root_manager) = &self.root_manager {
            root_manager.get_all_roots()
        } else {
            self.object_registry.get_root_objects()
        }
    }
    
    /// Get references from an object to other objects
    fn get_object_references(&self, object_id: ObjectId) -> Result<Vec<ObjectId>, String> {
        debug!("Getting references for object {}", object_id);
        
        // Get the object from registry
        let object = match self.object_registry.get(object_id)? {
        
        // TODO: Implement reference traversal once ObjectMetadata implements Traceable
        // For now, return empty references
        debug!("Would trace object {} for references", object_id);
        Ok(Vec::new())
    /// Request safe points from all active threads and goroutines
    fn request_global_safe_point(&self) -> Result<(), String> {
        let _span = span!(Level::DEBUG, "request_global_safe_point").entered();
        debug!("Requesting global safe point");
        
        // Get current thread list (this would need integration with runtime)
        let current_thread = thread::current().id();
        
        // Request safe point from current thread coordination
        {
            let mut pending_requests = self.pending_safe_point_requests.write()
                .map_err(|_| "Failed to acquire write lock on pending safe point requests")?;
            pending_requests.insert(current_thread);
        // Request safe points from goroutines if coordinator is available
        if let Some(coordinator) = self.goroutine_coordinator.lock()
            .map_err(|_| "Failed to acquire lock on goroutine coordinator")?.as_ref() {
            coordinator.request_global_safe_point()?;
        debug!("Global safe point requested");
        Ok(())
    /// Wait for threads to reach safe points with timeout
    fn wait_for_safe_points(&self, timeout: Duration) -> Result<bool, String> {
        let _span = span!(Level::DEBUG, "wait_for_safe_points", ?timeout).entered();
        let start_time = Instant::now();
        
        while start_time.elapsed() < timeout {
            let all_at_safe_point = {
                let pending = self.pending_safe_point_requests.read()
                    .map_err(|_| "Failed to acquire read lock on pending safe point requests")?;
                let at_safe_point = self.threads_at_safe_point.read()
                    .map_err(|_| "Failed to acquire read lock on threads at safe point")?;
                
                // Check if all pending requests have been satisfied
                pending.iter().all(|thread_id| at_safe_point.contains(thread_id))
            
            if all_at_safe_point {
                debug!("All threads reached safe points in {:?}", start_time.elapsed());
                return Ok(true);
            // Wait a bit before checking again
            thread::sleep(Duration::from_millis(1));
        warn!("Safe point coordination timed out after {:?}", timeout);
        Ok(false)
    /// Release all threads from safe points
    fn release_safe_points(&self) -> Result<(), String> {
        let _span = span!(Level::DEBUG, "release_safe_points").entered();
        debug!("Releasing all safe points");
        
        // Clear pending requests and safe point records
        {
            let mut pending = self.pending_safe_point_requests.write()
                .map_err(|_| "Failed to acquire write lock on pending safe point requests")?;
            pending.clear();
        {
            let mut at_safe_point = self.threads_at_safe_point.write()
                .map_err(|_| "Failed to acquire write lock on threads at safe point")?;
            at_safe_point.clear();
        // Notify condition variable to wake up waiting threads
        self.safe_point_condvar.notify_all();
        
        debug!("All safe points released");
        Ok(())
    /// Process write barrier records with time budget
    fn process_write_barrier_records(&self, time_budget: Duration) -> Result<bool, String> {
        let _span = span!(Level::DEBUG, "process_write_barrier_records", ?time_budget).entered();
        let step_start = Instant::now();
        let mut records_processed = 0;
        
        while step_start.elapsed() < time_budget {
            let record = {
                let mut barrier_records = self.write_barrier_records.lock()
                    .map_err(|_| "Failed to acquire lock on write barrier records")?;
                barrier_records.pop_front()
            
            let record = match record {
                None => break, // No more records
            
            // Process the write barrier record
            debug!("Processing write barrier record for object {}", record.object_id);
            
            // Check if this creates a cross-generational reference
            if let Some(new_value) = record.old_value {
                let cross_gen_ref = (record.object_id, new_value);
                let mut cross_refs = self.cross_generational_refs.write()
                    .map_err(|_| "Failed to acquire write lock on cross generational refs")?;
                cross_refs.insert(cross_gen_ref);
            // Update remembered set
            self.update_remembered_set(record.object_id, record.new_value, record.field_offset)?;
            
            records_processed += 1;
            
            // Check time budget every few records
            if records_processed % 50 == 0 && step_start.elapsed() > time_budget {
                break;
            }
        }
        
        debug!("Processed {} write barrier records in {:?}", records_processed, step_start.elapsed());
        Ok(records_processed > 0)
    /// Update phase progress tracking
    fn update_phase_progress(&self, current_phase: CollectionState) -> Result<(), String> {
        let mut progress = self.collection_progress.write()
            .map_err(|_| "Failed to acquire write lock on collection progress")?;
        
        // Update completed work units based on atomic counters
        match current_phase {
            CollectionState::Marking => {
                let completed = self.marking_progress.load(std::sync::atomic::Ordering::SeqCst);
                progress.completed_work_units = completed;
            }
            CollectionState::Sweeping => {
                let completed = self.sweeping_progress.load(std::sync::atomic::Ordering::SeqCst);
                progress.completed_work_units = completed;
            }
            CollectionState::RootScanning => {
                let completed = self.root_scan_progress.load(std::sync::atomic::Ordering::SeqCst);
                progress.completed_work_units = completed;
            }
            _ => {
                // For other phases, increment completed work units
                progress.completed_work_units += 1;
            }
        }
        
        // Update progress calculations
        let completed = progress.completed_work_units;
        progress.update_progress(completed);
        
        Ok(())
    /// Record timing for a specific collection phase
    fn record_phase_timing(&self, phase: CollectionState, duration: Duration) -> Result<(), String> {
        let mut timings = self.phase_timings.write()
            .map_err(|_| "Failed to acquire write lock on phase timings")?;
        
        let phase_timings = timings.entry(phase).or_insert_with(Vec::new);
        phase_timings.push(duration);
        
        // Keep only recent timings (last 100 measurements)
        if phase_timings.len() > 100 {
            phase_timings.remove(0);
        Ok(())
    /// Create a collection checkpoint for resumable collection
    fn create_collection_checkpoint(&self) -> Result<(), String> {
        let _span = span!(Level::DEBUG, "create_collection_checkpoint").entered();
        
        let current_state = {
            let state = self.collection_state.read()
                .map_err(|_| "Failed to acquire read lock on collection state")?;
            *state
        
        let mut checkpoint = CollectionCheckpoint::new(current_state);
        
        // Capture current collection state
        {
            let work_queue = self.work_queue.lock()
                .map_err(|_| "Failed to acquire lock on work queue")?;
            checkpoint.work_queue_snapshot = work_queue.iter().cloned().collect();
        {
            let marked_objects = self.marked_objects.read()
                .map_err(|_| "Failed to acquire read lock on marked objects")?;
            checkpoint.marked_objects_snapshot = marked_objects.clone();
        {
            let sweep_candidates = self.sweep_candidates.lock()
                .map_err(|_| "Failed to acquire lock on sweep candidates")?;
            checkpoint.sweep_candidates_snapshot = sweep_candidates.iter().cloned().collect();
        checkpoint.root_scan_progress = self.root_scan_progress.load(std::sync::atomic::Ordering::SeqCst);
        
        {
            let stats = self.stats.read()
                .map_err(|_| "Failed to acquire read lock on stats")?;
            checkpoint.stats_snapshot = stats.clone();
        // Store the checkpoint
        {
            let mut stored_checkpoint = self.collection_checkpoint.write()
                .map_err(|_| "Failed to acquire write lock on collection checkpoint")?;
            *stored_checkpoint = Some(checkpoint);
        debug!("Created collection checkpoint for phase {:?}", current_state);
        Ok(())
    /// Resume collection from checkpoint
    pub fn resume_from_checkpoint(&self) -> Result<bool, String> {
        let _span = span!(Level::DEBUG, "resume_from_checkpoint").entered();
        
        let checkpoint = {
            let stored_checkpoint = self.collection_checkpoint.read()
                .map_err(|_| "Failed to acquire read lock on collection checkpoint")?;
            stored_checkpoint.clone()
        
        let checkpoint = match checkpoint {
            None => {
                debug!("No collection checkpoint found");
                return Ok(false);
            }
        
        // Check if checkpoint is still valid (not too old)
        if !checkpoint.is_valid(Duration::from_secs(60)) {
            warn!("Collection checkpoint is too old, discarding");
            let mut stored_checkpoint = self.collection_checkpoint.write()
                .map_err(|_| "Failed to acquire write lock on collection checkpoint")?;
            *stored_checkpoint = None;
            return Ok(false);
        info!("Resuming collection from checkpoint in phase {:?}", checkpoint.phase);
        
        // Restore collection state
        {
            let mut state = self.collection_state.write()
                .map_err(|_| "Failed to acquire write lock on collection state")?;
            *state = checkpoint.phase;
        {
            let mut work_queue = self.work_queue.lock()
                .map_err(|_| "Failed to acquire lock on work queue")?;
            work_queue.clear();
            work_queue.extend(checkpoint.work_queue_snapshot);
        {
            let mut marked_objects = self.marked_objects.write()
                .map_err(|_| "Failed to acquire write lock on marked objects")?;
            *marked_objects = checkpoint.marked_objects_snapshot;
        {
            let mut sweep_candidates = self.sweep_candidates.lock()
                .map_err(|_| "Failed to acquire lock on sweep candidates")?;
            sweep_candidates.clear();
            sweep_candidates.extend(checkpoint.sweep_candidates_snapshot);
        self.root_scan_progress.store(checkpoint.root_scan_progress, std::sync::atomic::Ordering::SeqCst);
        
        debug!("Collection resumed from checkpoint successfully");
        Ok(true)
    /// Set goroutine coordinator for integration
    pub fn set_goroutine_coordinator(&self, coordinator: Arc<GoroutineGcCoordinator>) -> Result<(), String> {
        let mut gc_coordinator = self.goroutine_coordinator.lock()
            .map_err(|_| "Failed to acquire lock on goroutine coordinator")?;
        *gc_coordinator = Some(coordinator);
        debug!("Goroutine coordinator set for incremental collection");
        Ok(())
    /// Get collection progress information
    pub fn get_collection_progress(&self) -> Result<CollectionProgress, String> {
        let progress = self.collection_progress.read()
            .map_err(|_| "Failed to acquire read lock on collection progress")?;
        Ok(progress.clone())
    /// Get current collection phase
    pub fn get_current_phase(&self) -> Result<CollectionState, String> {
        let state = self.collection_state.read()
            .map_err(|_| "Failed to acquire read lock on collection state")?;
        Ok(*state)
    /// Check if an object exists
    fn object_exists(&self, object_id: ObjectId) -> Result<bool, String> {
        match self.object_registry.get(object_id) {
        }
    }
    
    /// Update step statistics
    fn update_step_statistics(&self, work_type: CollectionState, step_duration: Duration, work_performed: bool) -> Result<(), String> {
        let mut stats = self.stats.write()
            .map_err(|_| "Failed to acquire write lock on stats")?;
        
        if work_performed {
            stats.total_increments += 1;
            stats.total_work_time += step_duration;
            
            // Update work type statistics
            let work_type = match work_type {
            
            *stats.increments_by_type.entry(work_type).or_insert(0) += 1;
            
            // Update average step duration
            let total_steps = stats.total_increments as f64;
            let current_avg = stats.average_step_duration.as_secs_f64();
            let new_avg = (current_avg * (total_steps - 1.0) + step_duration.as_secs_f64()) / total_steps;
            stats.average_step_duration = Duration::from_secs_f64(new_avg);
        Ok(())
    /// Get incremental collection statistics
    pub fn get_stats(&self) -> Result<IncrementalStats, String> {
        let stats = self.stats.read()
            .map_err(|_| "Failed to acquire read lock on stats")?;
        Ok(stats.clone())
    /// Update configuration
    pub fn update_config(&self, new_config: IncrementalConfig) -> Result<(), String> {
        let mut config = self.config.write()
            .map_err(|_| "Failed to acquire write lock on config")?;
        *config = new_config;
        info!("Updated incremental collector configuration");
        Ok(())
    /// Check if collection is in progress
    pub fn is_collecting(&self) -> Result<bool, String> {
        let state = self.collection_state.read()
            .map_err(|_| "Failed to acquire read lock on collection state")?;
        Ok(*state != CollectionState::Idle)
    /// Enhanced root set scanning with interruption points
    pub fn scan_roots_incremental(&self, max_roots: usize) -> Result<Vec<ObjectId>, String> {
        debug!("Starting incremental root scan, max roots: {}", max_roots);
        
        let roots = if let Some(root_manager) = &self.root_manager {
            root_manager.get_all_roots()?
        } else {
            self.object_registry.get_root_objects()?
        
        let start_index = self.root_scan_progress.load(std::sync::atomic::Ordering::SeqCst);
        let end_index = std::cmp::min(start_index + max_roots, roots.len());
        
        let scanned_roots = roots[start_index..end_index].to_vec();
        
        // Update progress
        self.root_scan_progress.store(end_index, std::sync::atomic::Ordering::SeqCst);
        
        // Reset progress if we've scanned all roots
        if end_index >= roots.len() {
            self.root_scan_progress.store(0, std::sync::atomic::Ordering::SeqCst);
        debug!("Scanned {} roots, progress: {}/{}", scanned_roots.len(), end_index, roots.len());
        Ok(scanned_roots)
    /// Get weak references for processing
    fn get_weak_references(&self) -> Result<Vec<WeakReference>, String> {
        let weak_refs = self.weak_references.read()
            .map_err(|_| "Failed to acquire read lock on weak references")?;
        Ok(weak_refs.iter().copied().collect())
    /// Process a single weak reference
    fn process_weak_reference(&self, weak_ref: WeakReference) -> Result<bool, String> {
        debug!("Processing weak reference: {:?}", weak_ref);
        
        // Check if referent is still marked
        let marked_objects = self.marked_objects.read()
            .map_err(|_| "Failed to acquire read lock on marked objects")?;
        
        if !marked_objects.contains(&weak_ref.referent) {
            // Referent is being collected, null out the weak reference
            debug!("Nulling weak reference to object {}", weak_ref.referent);
            
            // Remove from weak references set
            let mut weak_refs = self.weak_references.write()
                .map_err(|_| "Failed to acquire write lock on weak references")?;
            weak_refs.remove(&weak_ref);
            
            return Ok(true);
        Ok(false)
    /// Get pending finalizers
    fn get_pending_finalizers(&self) -> Result<Vec<FinalizerEntry>, String> {
        let finalizers = self.pending_finalizers.lock()
            .map_err(|_| "Failed to acquire lock on pending finalizers")?;
        Ok(finalizers.iter().cloned().collect())
    /// Process finalizers up to the specified quantum
    fn process_finalizers(&self, quantum: usize) -> Result<usize, String> {
        debug!("Processing finalizers, quantum: {}", quantum);
        let mut processed = 0;
        
        for _ in 0..quantum {
            let finalizer = {
                let mut finalizers = self.pending_finalizers.lock()
                    .map_err(|_| "Failed to acquire lock on pending finalizers")?;
                finalizers.pop_front()
            
            let finalizer = match finalizer {
                None => break, // No more finalizers
            
            if self.execute_finalizer(finalizer)? {
                processed += 1;
            }
        }
        
        debug!("Processed {} finalizers", processed);
        Ok(processed)
    /// Execute a single finalizer
    fn execute_finalizer(&self, finalizer: FinalizerEntry) -> Result<bool, String> {
        debug!("Executing finalizer for object {}", finalizer.object_id);
        
        // TODO: Integrate with actual finalizer execution system
        // For now, just log that we would execute it
              finalizer.object_id, finalizer.priority);
        
        Ok(true)
    /// Transition to relocation phase
    fn transition_to_relocation(&self) -> Result<(), String> {
        debug!("Transitioning to relocation phase");
        
        {
            let mut state = self.collection_state.write()
                .map_err(|_| "Failed to acquire write lock on collection state")?;
            *state = CollectionState::Relocation;
        // Identify objects that need relocation
        let candidates = self.identify_relocation_candidates()?;
        {
            let mut relocation_candidates = self.relocation_candidates.lock()
                .map_err(|_| "Failed to acquire lock on relocation candidates")?;
            relocation_candidates.clear();
            relocation_candidates.extend(candidates);
        debug!("Relocation phase initialized");
        Ok(())
    /// Get objects that need relocation
    fn get_relocation_candidates(&self) -> Result<Vec<ObjectId>, String> {
        let candidates = self.relocation_candidates.lock()
            .map_err(|_| "Failed to acquire lock on relocation candidates")?;
        Ok(candidates.iter().copied().collect())
    /// Identify objects that should be relocated for compaction
    fn identify_relocation_candidates(&self) -> Result<Vec<ObjectId>, String> {
        debug!("Identifying relocation candidates");
        
        // For now, we'll identify objects based on fragmentation patterns
        // In a real implementation, this would analyze memory layout
        let marked_objects = self.marked_objects.read()
            .map_err(|_| "Failed to acquire read lock on marked objects")?;
        
        let mut candidates = Vec::new();
        
        // Simple heuristic: relocate objects that could benefit from compaction
        for &object_id in marked_objects.iter() {
            if self.should_relocate_object(object_id)? {
                candidates.push(object_id);
            }
        }
        
        debug!("Identified {} relocation candidates", candidates.len());
        Ok(candidates)
    /// Determine if an object should be relocated
    fn should_relocate_object(&self, _object_id: ObjectId) -> Result<bool, String> {
        // TODO: Implement actual relocation heuristics
        // For now, use a simple random selection
        Ok(false) // Conservative: don't relocate by default
    /// Relocate a single object
    fn relocate_object(&self, object_id: ObjectId) -> Result<bool, String> {
        debug!("Relocating object {}", object_id);
        
        // Get the object
        let object = match self.object_registry.get(object_id)? {
            None => return Ok(false), // Object no longer exists
        
        // Create new object ID for relocated object
        let new_object_id = ObjectId::new(object_id.as_u64() + 1000000); // Simple offset for new ID
        
        // TODO: Actually allocate new memory and copy object data
        // For now, we'll just create the forwarding pointer
        
        let forwarding_pointer = ForwardingPointer {
        
        {
            let mut forwarding_pointers = self.forwarding_pointers.write()
                .map_err(|_| "Failed to acquire write lock on forwarding pointers")?;
            forwarding_pointers.insert(object_id, forwarding_pointer);
        debug!("Created forwarding pointer: {} -> {}", object_id, new_object_id);
        Ok(true)
    /// Update forwarding pointers after relocation
    fn update_forwarding_pointers(&self, max_updates: usize) -> Result<usize, String> {
        debug!("Updating forwarding pointers, max updates: {}", max_updates);
        
        let forwarding_pointers = {
            let fp = self.forwarding_pointers.read()
                .map_err(|_| "Failed to acquire read lock on forwarding pointers")?;
            fp.clone()
        
        let mut updates = 0;
        
        // Update all references to point to new locations
        for (original_id, forwarding_pointer) in forwarding_pointers.iter() {
            if updates >= max_updates {
                break;
            if self.update_references_to_object(*original_id, forwarding_pointer.new_id)? {
                updates += 1;
            }
        }
        
        debug!("Updated {} forwarding pointers", updates);
        Ok(updates)
    /// Update all references to an object to point to its new location
    fn update_references_to_object(&self, old_id: ObjectId, new_id: ObjectId) -> Result<bool, String> {
        debug!("Updating references from {} to {}", old_id, new_id);
        
        // TODO: Implement reference updating
        // This would need to scan all objects and update their references
        // For now, just log the operation
        
        Ok(true)
    /// Check if an object has a finalizer
    fn has_finalizer(&self, _object_id: ObjectId) -> Result<bool, String> {
        // TODO: Implement actual finalizer checking
        // This would check object metadata for finalizer flag
        Ok(false)
    /// Schedule a finalizer for execution
    fn schedule_finalizer(&self, object_id: ObjectId) -> Result<(), String> {
        debug!("Scheduling finalizer for object {}", object_id);
        
        let finalizer = FinalizerEntry {
            priority: 128, // Default priority
        
        let mut finalizers = self.pending_finalizers.lock()
            .map_err(|_| "Failed to acquire lock on pending finalizers")?;
        finalizers.push_back(finalizer);
        
        Ok(())
    /// Add concurrent marking support
    pub fn enable_concurrent_marking(&self) -> Result<(), String> {
        debug!("Enabling concurrent marking");
        
        let mut config = self.config.write()
            .map_err(|_| "Failed to acquire write lock on config")?;
        config.concurrent_collection = true;
        
        info!("Concurrent marking enabled");
        Ok(())
    /// Add weak reference to tracking
    pub fn add_weak_reference(&self, weak_ref: WeakReference) -> Result<(), String> {
        debug!("Adding weak reference: {:?}", weak_ref);
        
        let mut weak_refs = self.weak_references.write()
            .map_err(|_| "Failed to acquire write lock on weak references")?;
        weak_refs.insert(weak_ref);
        
        Ok(())
    /// Remove weak reference from tracking
    pub fn remove_weak_reference(&self, weak_ref: &WeakReference) -> Result<bool, String> {
        debug!("Removing weak reference: {:?}", weak_ref);
        
        let mut weak_refs = self.weak_references.write()
            .map_err(|_| "Failed to acquire write lock on weak references")?;
        Ok(weak_refs.remove(weak_ref))
    }
}

impl Drop for IncrementalCollector {
    fn drop(&mut self) {
        if let Err(e) = self.stop_background_collection() {
            error!("Failed to stop background collection during drop: {}", e);
        }
    }
// Safety: IncrementalCollector is thread-safe through its use of RwLock, Mutex, and atomic types
unsafe impl Send for IncrementalCollector {}
unsafe impl Sync for IncrementalCollector {}

