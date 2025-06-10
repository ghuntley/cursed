/// Incremental Garbage Collection Support
/// 
/// This module provides incremental collection capabilities to reduce pause times
/// by spreading collection work across multiple small increments. It includes
/// write barriers, remembered sets, and work scheduling.

use std::sync::{Arc, RwLock, Mutex};
use std::collections::{HashMap, HashSet, VecDeque};
use std::time::{Duration, Instant};
use tracing::{instrument, debug, info, warn, error};

use crate::memory::{Traceable, Visitor};
use crate::memory::object_id::{ObjectId, ObjectRegistry, SharedObjectRegistry};
use crate::memory::roots::{RootSetManager, RootType};

/// Reference collector for traversing object references
struct ReferenceCollector {
    references: Vec<ObjectId>,
}

impl ReferenceCollector {
    fn new() -> Self {
        Self {
            references: Vec::new(),
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
    original_id: ObjectId,
    new_id: ObjectId,
    timestamp: Instant,
}

/// Weak reference for incremental processing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct WeakReference {
    pub referent: ObjectId,
    pub reference_id: ObjectId,
}

/// Finalizer entry for objects requiring cleanup
#[derive(Debug, Clone)]
pub struct FinalizerEntry {
    pub object_id: ObjectId,
    pub priority: u8,
    pub scheduled_at: Instant,
}

/// Types of incremental collection work
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum IncrementalWorkType {
    /// Marking work (traversing object graph)
    Marking,
    /// Sweeping work (reclaiming memory)
    Sweeping,
    /// Reference processing (weak references, finalizers)
    ReferenceProcessing,
    /// Remembered set processing (cross-generational references)
    RememberedSetProcessing,
    /// Object relocation (compaction)
    Relocation,
}

/// Configuration for incremental collection
#[derive(Debug, Clone)]
pub struct IncrementalConfig {
    /// Maximum time per incremental step
    pub max_step_duration: Duration,
    /// Minimum time between incremental steps
    pub min_step_interval: Duration,
    /// Work quantum size (objects processed per step)
    pub work_quantum: usize,
    /// Enable adaptive step sizing
    pub adaptive_step_sizing: bool,
    /// Target allocation/collection ratio
    pub allocation_collection_ratio: f64,
    /// Enable concurrent collection
    pub concurrent_collection: bool,
    /// Write barrier overhead threshold
    pub write_barrier_threshold: f64,
    /// Enable remembered set optimization
    pub remembered_set_optimization: bool,
    /// Maximum remembered set size
    pub max_remembered_set_size: usize,
}

impl Default for IncrementalConfig {
    fn default() -> Self {
        Self {
            max_step_duration: Duration::from_millis(5),
            min_step_interval: Duration::from_millis(1),
            work_quantum: 100,
            adaptive_step_sizing: true,
            allocation_collection_ratio: 0.1, // 10% collection overhead
            concurrent_collection: false,
            write_barrier_threshold: 0.05, // 5% overhead
            remembered_set_optimization: true,
            max_remembered_set_size: 10000,
        }
    }
}

/// Statistics for incremental collection
#[derive(Debug, Clone, Default)]
pub struct IncrementalStats {
    pub total_increments: u64,
    pub increments_by_type: HashMap<IncrementalWorkType, u64>,
    pub total_work_time: Duration,
    pub average_step_duration: Duration,
    pub work_quantum_adjustments: u64,
    pub write_barrier_hits: u64,
    pub remembered_set_size: usize,
    pub concurrent_steps: u64,
    pub allocation_rate: f64,
    pub collection_overhead: f64,
}

/// Work item for incremental processing
#[derive(Debug, Clone)]
pub struct IncrementalWorkItem {
    pub work_type: IncrementalWorkType,
    pub object_id: ObjectId,
    pub priority: u8,
    pub estimated_work: usize,
    pub created_at: Instant,
}

/// Write barrier record for tracking object modifications
#[derive(Debug, Clone)]
pub struct WriteBarrierRecord {
    pub object_id: ObjectId,
    pub field_offset: usize,
    pub old_value: Option<ObjectId>,
    pub new_value: ObjectId,
    pub timestamp: Instant,
}

/// Remembered set entry for cross-generational references
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RememberedSetEntry {
    pub from_object: ObjectId,
    pub to_object: ObjectId,
    pub field_offset: usize,
}

/// Collection state for incremental processing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CollectionState {
    /// No collection in progress
    Idle,
    /// Marking phase in progress
    Marking,
    /// Sweeping phase in progress
    Sweeping,
    /// Reference processing in progress
    ReferenceProcessing,
    /// Relocation phase in progress
    Relocation,
}

/// Incremental garbage collection coordinator
pub struct IncrementalCollector {
    config: RwLock<IncrementalConfig>,
    object_registry: SharedObjectRegistry,
    root_manager: Option<Arc<RootSetManager>>,
    
    /// Current collection state
    collection_state: RwLock<CollectionState>,
    /// Work queue for incremental processing
    work_queue: Mutex<VecDeque<IncrementalWorkItem>>,
    /// Remembered set for cross-generational references
    remembered_set: RwLock<HashSet<RememberedSetEntry>>,
    /// Write barrier records
    write_barrier_records: Mutex<VecDeque<WriteBarrierRecord>>,
    /// Objects marked in current collection cycle
    marked_objects: RwLock<HashSet<ObjectId>>,
    /// Objects to be swept
    sweep_candidates: Mutex<VecDeque<ObjectId>>,
    
    /// Weak reference tracking
    weak_references: RwLock<HashSet<WeakReference>>,
    /// Pending finalizers
    pending_finalizers: Mutex<VecDeque<FinalizerEntry>>,
    /// Forwarding pointers for relocation
    forwarding_pointers: RwLock<HashMap<ObjectId, ForwardingPointer>>,
    /// Objects being relocated
    relocation_candidates: Mutex<VecDeque<ObjectId>>,
    /// Root scan progress for interruption
    root_scan_progress: std::sync::atomic::AtomicUsize,
    
    /// Statistics and monitoring
    stats: RwLock<IncrementalStats>,
    last_step_time: Mutex<Option<Instant>>,
    allocation_since_last_step: std::sync::atomic::AtomicUsize,
    step_work_quantum: std::sync::atomic::AtomicUsize,
    
    /// Background collection thread handle
    background_thread: Mutex<Option<std::thread::JoinHandle<()>>>,
    should_stop: std::sync::atomic::AtomicBool,
}

impl IncrementalCollector {
    /// Create a new incremental collector
    pub fn new(object_registry: SharedObjectRegistry) -> Self {
        Self::with_config(object_registry, IncrementalConfig::default())
    }
    
    /// Create a new incremental collector with custom configuration
    #[instrument(skip(object_registry, config))]
    pub fn with_config(object_registry: SharedObjectRegistry, config: IncrementalConfig) -> Self {
        info!("Creating incremental collector with config: {:?}", config);
        
        Self {
            config: RwLock::new(config.clone()),
            object_registry,
            root_manager: None,
            collection_state: RwLock::new(CollectionState::Idle),
            work_queue: Mutex::new(VecDeque::new()),
            remembered_set: RwLock::new(HashSet::new()),
            write_barrier_records: Mutex::new(VecDeque::new()),
            marked_objects: RwLock::new(HashSet::new()),
            sweep_candidates: Mutex::new(VecDeque::new()),
            weak_references: RwLock::new(HashSet::new()),
            pending_finalizers: Mutex::new(VecDeque::new()),
            forwarding_pointers: RwLock::new(HashMap::new()),
            relocation_candidates: Mutex::new(VecDeque::new()),
            root_scan_progress: std::sync::atomic::AtomicUsize::new(0),
            stats: RwLock::new(IncrementalStats::default()),
            last_step_time: Mutex::new(None),
            allocation_since_last_step: std::sync::atomic::AtomicUsize::new(0),
            step_work_quantum: std::sync::atomic::AtomicUsize::new(config.work_quantum),
            background_thread: Mutex::new(None),
            should_stop: std::sync::atomic::AtomicBool::new(false),
        }
    }
    
    /// Set the root set manager
    pub fn set_root_manager(&mut self, root_manager: Arc<RootSetManager>) {
        self.root_manager = Some(root_manager);
    }
    
    /// Start incremental collection
    #[instrument(skip(self))]
    pub fn start_collection(&self) -> Result<(), String> {
        info!("Starting incremental collection");
        
        let mut state = self.collection_state.write()
            .map_err(|_| "Failed to acquire write lock on collection state")?;
        
        if *state != CollectionState::Idle {
            return Err("Collection already in progress".to_string());
        }
        
        *state = CollectionState::Marking;
        
        // Initialize for marking phase
        self.initialize_marking_phase()?;
        
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        if config.concurrent_collection {
            self.start_background_thread()?;
        }
        
        info!("Incremental collection started");
        Ok(())
    }
    
    /// Perform one incremental collection step
    #[instrument(skip(self))]
    pub fn step(&self) -> Result<bool, String> {
        let step_start = Instant::now();
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        // Check if we should perform a step
        if !self.should_perform_step()? {
            return Ok(false);
        }
        
        let current_state = {
            let state = self.collection_state.read()
                .map_err(|_| "Failed to acquire read lock on collection state")?;
            *state
        };
        
        let work_performed = match current_state {
            CollectionState::Idle => false,
            CollectionState::Marking => self.marking_step()?,
            CollectionState::Sweeping => self.sweeping_step()?,
            CollectionState::ReferenceProcessing => self.reference_processing_step()?,
            CollectionState::Relocation => self.relocation_step()?,
        };
        
        let step_duration = step_start.elapsed();
        
        // Update statistics
        self.update_step_statistics(current_state, step_duration, work_performed)?;
        
        // Adjust work quantum if adaptive sizing is enabled
        if config.adaptive_step_sizing {
            self.adjust_work_quantum(step_duration)?;
        }
        
        // Update last step time
        {
            let mut last_time = self.last_step_time.lock()
                .map_err(|_| "Failed to acquire lock on last step time")?;
            *last_time = Some(Instant::now());
        }
        
        debug!("Incremental step completed in {:?}, work performed: {}", step_duration, work_performed);
        Ok(work_performed)
    }
    
    /// Initialize marking phase
    fn initialize_marking_phase(&self) -> Result<(), String> {
        debug!("Initializing marking phase");
        
        // Clear previous marking state
        {
            let mut marked_objects = self.marked_objects.write()
                .map_err(|_| "Failed to acquire write lock on marked objects")?;
            marked_objects.clear();
        }
        
        // Reset root scan progress for incremental scanning
        self.root_scan_progress.store(0, std::sync::atomic::Ordering::SeqCst);
        
        // Clear work queue
        {
            let mut work_queue = self.work_queue.lock()
                .map_err(|_| "Failed to acquire lock on work queue")?;
            work_queue.clear();
        }
        
        // Start with an initial batch of roots using time-sliced scanning
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        let initial_root_batch = self.scan_roots_incremental(config.work_quantum)?;
        
        {
            let mut work_queue = self.work_queue.lock()
                .map_err(|_| "Failed to acquire lock on work queue")?;
            
            for root_id in initial_root_batch {
                let work_item = IncrementalWorkItem {
                    work_type: IncrementalWorkType::Marking,
                    object_id: root_id,
                    priority: 255, // High priority for roots
                    estimated_work: 1,
                    created_at: Instant::now(),
                };
                work_queue.push_back(work_item);
            }
        }
        
        debug!("Marking phase initialized with time-sliced root scanning");
        Ok(())
    }
    
    /// Perform one marking step
    fn marking_step(&self) -> Result<bool, String> {
        let work_quantum = self.step_work_quantum.load(std::sync::atomic::Ordering::SeqCst);
        let mut work_performed = 0;
        
        for _ in 0..work_quantum {
            let work_item = {
                let mut work_queue = self.work_queue.lock()
                    .map_err(|_| "Failed to acquire lock on work queue")?;
                work_queue.pop_front()
            };
            
            let work_item = match work_item {
                Some(item) if item.work_type == IncrementalWorkType::Marking => item,
                Some(item) => {
                    // Wrong type, put it back
                    let mut work_queue = self.work_queue.lock()
                        .map_err(|_| "Failed to acquire lock on work queue")?;
                    work_queue.push_front(item);
                    break;
                }
                None => break, // No more marking work
            };
            
            // Mark the object
            if self.mark_object_incremental(work_item.object_id)? {
                work_performed += 1;
                
                // Add referenced objects to work queue
                let referenced_objects = self.get_object_references(work_item.object_id)?;
                if !referenced_objects.is_empty() {
                    let mut work_queue = self.work_queue.lock()
                        .map_err(|_| "Failed to acquire lock on work queue")?;
                    
                    for ref_id in referenced_objects {
                        let ref_work_item = IncrementalWorkItem {
                            work_type: IncrementalWorkType::Marking,
                            object_id: ref_id,
                            priority: work_item.priority.saturating_sub(1),
                            estimated_work: 1,
                            created_at: Instant::now(),
                        };
                        work_queue.push_back(ref_work_item);
                    }
                }
            }
        }
        
        // Check if marking is complete
        let marking_complete = {
            let work_queue = self.work_queue.lock()
                .map_err(|_| "Failed to acquire lock on work queue")?;
            !work_queue.iter().any(|item| item.work_type == IncrementalWorkType::Marking)
        };
        
        if marking_complete {
            self.transition_to_sweeping()?;
        }
        
        Ok(work_performed > 0)
    }
    
    /// Transition from marking to sweeping phase
    fn transition_to_sweeping(&self) -> Result<(), String> {
        debug!("Transitioning to sweeping phase");
        
        {
            let mut state = self.collection_state.write()
                .map_err(|_| "Failed to acquire write lock on collection state")?;
            *state = CollectionState::Sweeping;
        }
        
        // Identify objects to sweep
        let unmarked_objects = self.get_unmarked_objects()?;
        {
            let mut sweep_candidates = self.sweep_candidates.lock()
                .map_err(|_| "Failed to acquire lock on sweep candidates")?;
            sweep_candidates.clear();
            sweep_candidates.extend(unmarked_objects);
        }
        
        debug!("Sweeping phase initialized");
        Ok(())
    }
    
    /// Perform one sweeping step
    fn sweeping_step(&self) -> Result<bool, String> {
        let work_quantum = self.step_work_quantum.load(std::sync::atomic::Ordering::SeqCst);
        let mut work_performed = 0;
        
        for _ in 0..work_quantum {
            let object_id = {
                let mut sweep_candidates = self.sweep_candidates.lock()
                    .map_err(|_| "Failed to acquire lock on sweep candidates")?;
                sweep_candidates.pop_front()
            };
            
            let object_id = match object_id {
                Some(id) => id,
                None => break, // No more objects to sweep
            };
            
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
        };
        
        if sweeping_complete {
            self.complete_collection()?;
        }
        
        Ok(work_performed > 0)
    }
    
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
            }
            
            if self.process_weak_reference(*weak_ref)? {
                work_performed += 1;
            }
        }
        
        // Process finalizers if we have remaining quantum
        if work_performed < work_quantum {
            let remaining_quantum = work_quantum - work_performed;
            let finalizer_work = self.process_finalizers(remaining_quantum)?;
            work_performed += finalizer_work;
        }
        
        // Check if reference processing is complete
        let processing_complete = work_performed == 0 || 
            (self.get_weak_references()?.is_empty() && self.get_pending_finalizers()?.is_empty());
        
        if processing_complete {
            self.transition_to_relocation()?;
        }
        
        debug!("Reference processing step completed, work performed: {}", work_performed);
        Ok(work_performed > 0)
    }
    
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
            }
            
            if self.relocate_object(object_id)? {
                work_performed += 1;
            }
        }
        
        // Update forwarding pointers in this step
        if work_performed > 0 {
            self.update_forwarding_pointers(work_quantum - work_performed)?;
        }
        
        // Check if relocation is complete
        let relocation_complete = work_performed == 0 || self.get_relocation_candidates()?.is_empty();
        
        if relocation_complete {
            self.complete_collection()?;
        }
        
        debug!("Relocation step completed, work performed: {}", work_performed);
        Ok(work_performed > 0)
    }
    
    /// Complete the collection cycle
    fn complete_collection(&self) -> Result<(), String> {
        debug!("Completing collection cycle");
        
        {
            let mut state = self.collection_state.write()
                .map_err(|_| "Failed to acquire write lock on collection state")?;
            *state = CollectionState::Idle;
        }
        
        // Process remembered set updates
        self.process_remembered_set_updates()?;
        
        info!("Incremental collection cycle completed");
        Ok(())
    }
    
    /// Mark an object incrementally
    fn mark_object_incremental(&self, object_id: ObjectId) -> Result<bool, String> {
        let mut marked_objects = self.marked_objects.write()
            .map_err(|_| "Failed to acquire write lock on marked objects")?;
        
        if marked_objects.contains(&object_id) {
            return Ok(false); // Already marked
        }
        
        // Verify object exists
        if !self.object_exists(object_id)? {
            return Ok(false);
        }
        
        marked_objects.insert(object_id);
        debug!("Marked object {} incrementally", object_id);
        Ok(true)
    }
    
    /// Sweep an object incrementally
    fn sweep_object_incremental(&self, object_id: ObjectId) -> Result<bool, String> {
        debug!("Sweeping object {} incrementally", object_id);
        
        // Check if object has finalizer
        if self.has_finalizer(object_id)? {
            self.schedule_finalizer(object_id)?;
            return Ok(true);
        }
        
        // Remove object from registry first
        // TODO: Implement actual object removal once ObjectRegistry has remove method
        debug!("Would remove object {} from registry", object_id);
        
        // For now, just mark as swept
        {
            let mut stats = self.stats.write()
                .map_err(|_| "Failed to acquire write lock on stats")?;
            // TODO: Add swept_objects counter to stats
        }
        
        Ok(true)
    }
    
    /// Write barrier for concurrent/incremental collection
    #[instrument(skip(self))]
    pub fn write_barrier(&self, object_id: ObjectId, field_offset: usize, old_value: Option<ObjectId>, new_value: ObjectId) -> Result<(), String> {
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        let current_state = {
            let state = self.collection_state.read()
                .map_err(|_| "Failed to acquire read lock on collection state")?;
            *state
        };
        
        // Only process write barrier if collection is active
        if current_state != CollectionState::Idle {
            let record = WriteBarrierRecord {
                object_id,
                field_offset,
                old_value,
                new_value,
                timestamp: Instant::now(),
            };
            
            let mut records = self.write_barrier_records.lock()
                .map_err(|_| "Failed to acquire lock on write barrier records")?;
            records.push_back(record);
            
            // Update remembered set if this is a cross-generational reference
            self.update_remembered_set(object_id, new_value, field_offset)?;
        }
        
        // Update statistics
        {
            let mut stats = self.stats.write()
                .map_err(|_| "Failed to acquire write lock on stats")?;
            stats.write_barrier_hits += 1;
        }
        
        debug!("Write barrier processed for object {} field {}", object_id, field_offset);
        Ok(())
    }
    
    /// Update remembered set with cross-generational reference
    fn update_remembered_set(&self, from_object: ObjectId, to_object: ObjectId, field_offset: usize) -> Result<(), String> {
        // TODO: Check if this is actually a cross-generational reference
        // For now, we'll add all references to the remembered set
        
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        if !config.remembered_set_optimization {
            return Ok(());
        }
        
        let entry = RememberedSetEntry {
            from_object,
            to_object,
            field_offset,
        };
        
        let mut remembered_set = self.remembered_set.write()
            .map_err(|_| "Failed to acquire write lock on remembered set")?;
        
        // Check size limit
        if remembered_set.len() >= config.max_remembered_set_size {
            warn!("Remembered set size limit reached, clearing old entries");
            remembered_set.clear();
        }
        
        remembered_set.insert(entry);
        
        debug!("Updated remembered set: {} -> {} at offset {}", from_object, to_object, field_offset);
        Ok(())
    }
    
    /// Process remembered set updates
    fn process_remembered_set_updates(&self) -> Result<(), String> {
        debug!("Processing remembered set updates");
        
        // Process write barrier records
        let records = {
            let mut barrier_records = self.write_barrier_records.lock()
                .map_err(|_| "Failed to acquire lock on write barrier records")?;
            let records: Vec<_> = barrier_records.drain(..).collect();
            records
        };
        
        for record in records {
            // TODO: Process each write barrier record
            debug!("Processing write barrier record for object {}", record.object_id);
        }
        
        // Update statistics
        {
            let mut stats = self.stats.write()
                .map_err(|_| "Failed to acquire write lock on stats")?;
            let remembered_set = self.remembered_set.read()
                .map_err(|_| "Failed to acquire read lock on remembered set")?;
            stats.remembered_set_size = remembered_set.len();
        }
        
        Ok(())
    }
    
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
    }
    
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
        };
        
        if new_quantum != current_quantum {
            self.step_work_quantum.store(new_quantum, std::sync::atomic::Ordering::SeqCst);
            
            let mut stats = self.stats.write()
                .map_err(|_| "Failed to acquire write lock on stats")?;
            stats.work_quantum_adjustments += 1;
            
            debug!("Adjusted work quantum: {} -> {}", current_quantum, new_quantum);
        }
        
        Ok(())
    }
    
    /// Start background collection thread
    fn start_background_thread(&self) -> Result<(), String> {
        let mut thread_handle = self.background_thread.lock()
            .map_err(|_| "Failed to acquire lock on background thread")?;
        
        if thread_handle.is_some() {
            return Ok(());
        }
        
        let collector_ref = unsafe { std::mem::transmute::<&IncrementalCollector, &'static IncrementalCollector>(self) };
        
        let handle = std::thread::spawn(move || {
            info!("Starting background incremental collection thread");
            collector_ref.background_collection_loop();
            info!("Background incremental collection thread stopped");
        });
        
        *thread_handle = Some(handle);
        Ok(())
    }
    
    /// Background collection loop
    fn background_collection_loop(&self) {
        while !self.should_stop.load(std::sync::atomic::Ordering::SeqCst) {
            if let Err(e) = self.step() {
                error!("Background collection step failed: {}", e);
            }
            
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
        };
        
        if let Some(handle) = handle {
            if let Err(e) = handle.join() {
                error!("Failed to join background thread: {:?}", e);
            }
        }
        
        Ok(())
    }
    
    /// Notify of allocation for step scheduling
    pub fn notify_allocation(&self, bytes: usize) {
        self.allocation_since_last_step.fetch_add(bytes, std::sync::atomic::Ordering::SeqCst);
    }
    
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
    }
    
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
            Some(obj) => obj,
            None => return Ok(Vec::new()),
        };
        
        // TODO: Implement reference traversal once ObjectMetadata implements Traceable
        // For now, return empty references
        debug!("Would trace object {} for references", object_id);
        Ok(Vec::new())
    }
    
    /// Check if an object exists
    fn object_exists(&self, object_id: ObjectId) -> Result<bool, String> {
        match self.object_registry.get(object_id) {
            Ok(Some(_)) => Ok(true),
            Ok(None) => Ok(false),
            Err(e) => Err(format!("Failed to check object existence: {}", e)),
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
                CollectionState::Marking => IncrementalWorkType::Marking,
                CollectionState::Sweeping => IncrementalWorkType::Sweeping,
                CollectionState::ReferenceProcessing => IncrementalWorkType::ReferenceProcessing,
                CollectionState::Relocation => IncrementalWorkType::Relocation,
                CollectionState::Idle => return Ok(()),
            };
            
            *stats.increments_by_type.entry(work_type).or_insert(0) += 1;
            
            // Update average step duration
            let total_steps = stats.total_increments as f64;
            let current_avg = stats.average_step_duration.as_secs_f64();
            let new_avg = (current_avg * (total_steps - 1.0) + step_duration.as_secs_f64()) / total_steps;
            stats.average_step_duration = Duration::from_secs_f64(new_avg);
        }
        
        Ok(())
    }
    
    /// Get incremental collection statistics
    pub fn get_stats(&self) -> Result<IncrementalStats, String> {
        let stats = self.stats.read()
            .map_err(|_| "Failed to acquire read lock on stats")?;
        Ok(stats.clone())
    }
    
    /// Update configuration
    pub fn update_config(&self, new_config: IncrementalConfig) -> Result<(), String> {
        let mut config = self.config.write()
            .map_err(|_| "Failed to acquire write lock on config")?;
        *config = new_config;
        info!("Updated incremental collector configuration");
        Ok(())
    }
    
    /// Check if collection is in progress
    pub fn is_collecting(&self) -> Result<bool, String> {
        let state = self.collection_state.read()
            .map_err(|_| "Failed to acquire read lock on collection state")?;
        Ok(*state != CollectionState::Idle)
    }
    
    /// Enhanced root set scanning with interruption points
    pub fn scan_roots_incremental(&self, max_roots: usize) -> Result<Vec<ObjectId>, String> {
        debug!("Starting incremental root scan, max roots: {}", max_roots);
        
        let roots = if let Some(root_manager) = &self.root_manager {
            root_manager.get_all_roots()?
        } else {
            self.object_registry.get_root_objects()?
        };
        
        let start_index = self.root_scan_progress.load(std::sync::atomic::Ordering::SeqCst);
        let end_index = std::cmp::min(start_index + max_roots, roots.len());
        
        let scanned_roots = roots[start_index..end_index].to_vec();
        
        // Update progress
        self.root_scan_progress.store(end_index, std::sync::atomic::Ordering::SeqCst);
        
        // Reset progress if we've scanned all roots
        if end_index >= roots.len() {
            self.root_scan_progress.store(0, std::sync::atomic::Ordering::SeqCst);
        }
        
        debug!("Scanned {} roots, progress: {}/{}", scanned_roots.len(), end_index, roots.len());
        Ok(scanned_roots)
    }
    
    /// Get weak references for processing
    fn get_weak_references(&self) -> Result<Vec<WeakReference>, String> {
        let weak_refs = self.weak_references.read()
            .map_err(|_| "Failed to acquire read lock on weak references")?;
        Ok(weak_refs.iter().copied().collect())
    }
    
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
        }
        
        Ok(false)
    }
    
    /// Get pending finalizers
    fn get_pending_finalizers(&self) -> Result<Vec<FinalizerEntry>, String> {
        let finalizers = self.pending_finalizers.lock()
            .map_err(|_| "Failed to acquire lock on pending finalizers")?;
        Ok(finalizers.iter().cloned().collect())
    }
    
    /// Process finalizers up to the specified quantum
    fn process_finalizers(&self, quantum: usize) -> Result<usize, String> {
        debug!("Processing finalizers, quantum: {}", quantum);
        let mut processed = 0;
        
        for _ in 0..quantum {
            let finalizer = {
                let mut finalizers = self.pending_finalizers.lock()
                    .map_err(|_| "Failed to acquire lock on pending finalizers")?;
                finalizers.pop_front()
            };
            
            let finalizer = match finalizer {
                Some(f) => f,
                None => break, // No more finalizers
            };
            
            if self.execute_finalizer(finalizer)? {
                processed += 1;
            }
        }
        
        debug!("Processed {} finalizers", processed);
        Ok(processed)
    }
    
    /// Execute a single finalizer
    fn execute_finalizer(&self, finalizer: FinalizerEntry) -> Result<bool, String> {
        debug!("Executing finalizer for object {}", finalizer.object_id);
        
        // TODO: Integrate with actual finalizer execution system
        // For now, just log that we would execute it
        info!("Would execute finalizer for object {} (priority {})", 
              finalizer.object_id, finalizer.priority);
        
        Ok(true)
    }
    
    /// Transition to relocation phase
    fn transition_to_relocation(&self) -> Result<(), String> {
        debug!("Transitioning to relocation phase");
        
        {
            let mut state = self.collection_state.write()
                .map_err(|_| "Failed to acquire write lock on collection state")?;
            *state = CollectionState::Relocation;
        }
        
        // Identify objects that need relocation
        let candidates = self.identify_relocation_candidates()?;
        {
            let mut relocation_candidates = self.relocation_candidates.lock()
                .map_err(|_| "Failed to acquire lock on relocation candidates")?;
            relocation_candidates.clear();
            relocation_candidates.extend(candidates);
        }
        
        debug!("Relocation phase initialized");
        Ok(())
    }
    
    /// Get objects that need relocation
    fn get_relocation_candidates(&self) -> Result<Vec<ObjectId>, String> {
        let candidates = self.relocation_candidates.lock()
            .map_err(|_| "Failed to acquire lock on relocation candidates")?;
        Ok(candidates.iter().copied().collect())
    }
    
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
    }
    
    /// Determine if an object should be relocated
    fn should_relocate_object(&self, _object_id: ObjectId) -> Result<bool, String> {
        // TODO: Implement actual relocation heuristics
        // For now, use a simple random selection
        Ok(false) // Conservative: don't relocate by default
    }
    
    /// Relocate a single object
    fn relocate_object(&self, object_id: ObjectId) -> Result<bool, String> {
        debug!("Relocating object {}", object_id);
        
        // Get the object
        let object = match self.object_registry.get(object_id)? {
            Some(obj) => obj,
            None => return Ok(false), // Object no longer exists
        };
        
        // Create new object ID for relocated object
        let new_object_id = ObjectId::new(object_id.as_u64() + 1000000); // Simple offset for new ID
        
        // TODO: Actually allocate new memory and copy object data
        // For now, we'll just create the forwarding pointer
        
        let forwarding_pointer = ForwardingPointer {
            original_id: object_id,
            new_id: new_object_id,
            timestamp: Instant::now(),
        };
        
        {
            let mut forwarding_pointers = self.forwarding_pointers.write()
                .map_err(|_| "Failed to acquire write lock on forwarding pointers")?;
            forwarding_pointers.insert(object_id, forwarding_pointer);
        }
        
        debug!("Created forwarding pointer: {} -> {}", object_id, new_object_id);
        Ok(true)
    }
    
    /// Update forwarding pointers after relocation
    fn update_forwarding_pointers(&self, max_updates: usize) -> Result<usize, String> {
        debug!("Updating forwarding pointers, max updates: {}", max_updates);
        
        let forwarding_pointers = {
            let fp = self.forwarding_pointers.read()
                .map_err(|_| "Failed to acquire read lock on forwarding pointers")?;
            fp.clone()
        };
        
        let mut updates = 0;
        
        // Update all references to point to new locations
        for (original_id, forwarding_pointer) in forwarding_pointers.iter() {
            if updates >= max_updates {
                break;
            }
            
            if self.update_references_to_object(*original_id, forwarding_pointer.new_id)? {
                updates += 1;
            }
        }
        
        debug!("Updated {} forwarding pointers", updates);
        Ok(updates)
    }
    
    /// Update all references to an object to point to its new location
    fn update_references_to_object(&self, old_id: ObjectId, new_id: ObjectId) -> Result<bool, String> {
        debug!("Updating references from {} to {}", old_id, new_id);
        
        // TODO: Implement reference updating
        // This would need to scan all objects and update their references
        // For now, just log the operation
        
        Ok(true)
    }
    
    /// Check if an object has a finalizer
    fn has_finalizer(&self, _object_id: ObjectId) -> Result<bool, String> {
        // TODO: Implement actual finalizer checking
        // This would check object metadata for finalizer flag
        Ok(false)
    }
    
    /// Schedule a finalizer for execution
    fn schedule_finalizer(&self, object_id: ObjectId) -> Result<(), String> {
        debug!("Scheduling finalizer for object {}", object_id);
        
        let finalizer = FinalizerEntry {
            object_id,
            priority: 128, // Default priority
            scheduled_at: Instant::now(),
        };
        
        let mut finalizers = self.pending_finalizers.lock()
            .map_err(|_| "Failed to acquire lock on pending finalizers")?;
        finalizers.push_back(finalizer);
        
        Ok(())
    }
    
    /// Add concurrent marking support
    pub fn enable_concurrent_marking(&self) -> Result<(), String> {
        debug!("Enabling concurrent marking");
        
        let mut config = self.config.write()
            .map_err(|_| "Failed to acquire write lock on config")?;
        config.concurrent_collection = true;
        
        info!("Concurrent marking enabled");
        Ok(())
    }
    
    /// Add weak reference to tracking
    pub fn add_weak_reference(&self, weak_ref: WeakReference) -> Result<(), String> {
        debug!("Adding weak reference: {:?}", weak_ref);
        
        let mut weak_refs = self.weak_references.write()
            .map_err(|_| "Failed to acquire write lock on weak references")?;
        weak_refs.insert(weak_ref);
        
        Ok(())
    }
    
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
}

// Safety: IncrementalCollector is thread-safe through its use of RwLock, Mutex, and atomic types
unsafe impl Send for IncrementalCollector {}
unsafe impl Sync for IncrementalCollector {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::object_id::ObjectRegistry;
    
    fn create_test_collector() -> (IncrementalCollector, SharedObjectRegistry) {
        let registry = Arc::new(ObjectRegistry::new());
        let collector = IncrementalCollector::new(registry.clone());
        (collector, registry)
    }
    
    #[test]
    fn test_collector_creation() {
        let (collector, _registry) = create_test_collector();
        let stats = collector.get_stats().unwrap();
        assert_eq!(stats.total_increments, 0);
    }
    
    #[test]
    fn test_collection_start_stop() {
        let (collector, _registry) = create_test_collector();
        
        assert!(!collector.is_collecting().unwrap());
        
        collector.start_collection().unwrap();
        assert!(collector.is_collecting().unwrap());
    }
    
    #[test]
    fn test_write_barrier() {
        let (collector, _registry) = create_test_collector();
        
        let object_id = ObjectId::new(1);
        let new_value = ObjectId::new(2);
        
        collector.write_barrier(object_id, 0, None, new_value).unwrap();
        
        let stats = collector.get_stats().unwrap();
        assert_eq!(stats.write_barrier_hits, 1);
    }
    
    #[test]
    fn test_allocation_notification() {
        let (collector, _registry) = create_test_collector();
        
        collector.notify_allocation(1024);
        
        // Should trigger need for collection step
        // Note: This test is simplified as the actual logic depends on configuration
    }
    
    #[test]
    fn test_config_update() {
        let (collector, _registry) = create_test_collector();
        
        let new_config = IncrementalConfig {
            max_step_duration: Duration::from_millis(10),
            work_quantum: 50,
            ..Default::default()
        };
        
        collector.update_config(new_config).unwrap();
    }
    
    #[test]
    fn test_incremental_root_scanning() {
        let (collector, _registry) = create_test_collector();
        
        // Test incremental root scanning with empty registry
        let roots = collector.scan_roots_incremental(1).unwrap();
        assert!(roots.len() <= 1);
    }
    
    #[test]
    fn test_weak_reference_handling() {
        let (collector, _registry) = create_test_collector();
        
        let referent = ObjectId::new(10);
        let reference_id = ObjectId::new(20);
        let weak_ref = WeakReference {
            referent,
            reference_id,
        };
        
        collector.add_weak_reference(weak_ref).unwrap();
        assert!(collector.remove_weak_reference(&weak_ref).unwrap());
        assert!(!collector.remove_weak_reference(&weak_ref).unwrap());
    }
    
    #[test]
    fn test_concurrent_marking_enable() {
        let (collector, _registry) = create_test_collector();
        
        collector.enable_concurrent_marking().unwrap();
        
        // Verify config was updated
        let config = collector.config.read().unwrap();
        assert!(config.concurrent_collection);
    }
    
    #[test]
    fn test_finalizer_scheduling() {
        let (collector, _registry) = create_test_collector();
        
        let object_id = ObjectId::new(100);
        collector.schedule_finalizer(object_id).unwrap();
        
        let finalizers = collector.get_pending_finalizers().unwrap();
        assert_eq!(finalizers.len(), 1);
        assert_eq!(finalizers[0].object_id, object_id);
    }
    
    #[test]
    fn test_collection_phases() {
        let (collector, _registry) = create_test_collector();
        
        // Test collection start and phases without objects
        collector.start_collection().unwrap();
        assert!(collector.is_collecting().unwrap());
        
        // Test stepping through phases
        for _ in 0..10 {
            if let Ok(false) = collector.step() {
                break; // No more work
            }
        }
    }
    
    #[test]
    fn test_adaptive_work_quantum() {
        let (collector, _registry) = create_test_collector();
        
        // Test quantum adjustment with fast step
        let fast_duration = Duration::from_micros(100);
        collector.adjust_work_quantum(fast_duration).unwrap();
        
        // Test quantum adjustment with slow step
        let slow_duration = Duration::from_millis(50);
        collector.adjust_work_quantum(slow_duration).unwrap();
        
        let stats = collector.get_stats().unwrap();
        assert!(stats.work_quantum_adjustments > 0);
    }
}
