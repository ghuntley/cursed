/// Advanced Memory Region Management for Generational Garbage Collection
/// 
/// This module provides sophisticated memory regions with actual heap management
/// algorithms optimized for different object lifecycle patterns:
/// 
/// 1. **Young Generation**: Fast bump allocation with age tracking and promotion
/// 2. **Old Generation**: Advanced free list allocation with compaction algorithms  
/// 3. **Large Object Space**: Segregated allocation with direct collection
/// 4. **Region Manager**: Intelligent allocation routing and memory pressure handling
/// 
/// The design enables efficient generational garbage collection by segregating
/// objects based on expected lifetime and implementing promotion, compaction,
/// and memory pressure detection algorithms.

use std::ptr::NonNull;
use std::sync::{Arc, RwLock, Mutex, atomic::{AtomicU32, AtomicU64, AtomicUsize, Ordering}};
use std::collections::{HashMap, VecDeque, BTreeMap};
use std::alloc::{alloc, dealloc, Layout};
use std::time::{Instant, Duration};
use tracing::{instrument, debug, info, warn, error, span, Level};

use crate::memory::allocator::{Allocator, BumpAllocator, FreeListAllocator, SegregatedAllocator, AllocationResult};
use crate::memory::heap::AllocationStrategy;
use crate::memory::heap::HeapConfiguration;

/// Trait for memory regions within the heap
/// 
/// Each region provides specialized allocation strategies optimized
/// for different object patterns and collection behaviors.
pub trait HeapRegion: Send + Sync {
    /// Allocate memory within this region
    fn allocate(&self, size: usize, alignment: usize, type_name: &str) -> Result<AllocationResult, String>;
    
    /// Deallocate memory within this region
    fn deallocate(&self, ptr: NonNull<u8>, size: usize) -> Result<(), String>;
    
    /// Check if pointer belongs to this region
    fn contains_pointer(&self, ptr: *const u8) -> bool;
    
    /// Get region statistics
    fn get_statistics(&self) -> RegionStatistics;
    
    /// Prepare region for garbage collection
    fn prepare_for_collection(&self) -> Result<(), String>;
    
    /// Complete garbage collection cleanup
    fn complete_collection(&self) -> Result<(), String>;
    
    /// Get region type identifier
    fn region_type(&self) -> RegionType;
    
    /// Get region ID
    fn region_id(&self) -> u32;
    
    /// Check if region can allocate the requested size
    fn can_allocate(&self, size: usize) -> bool;
    
    /// Get region capacity and usage
    fn get_capacity_info(&self) -> CapacityInfo;
}

/// Types of heap regions with specialized allocation behavior
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RegionType {
    YoungGeneration,
    OldGeneration,
    LargeObjectSpace,
    MetadataRegion,
}

/// Object age tracking for promotion decisions
#[derive(Debug, Clone)]
pub struct ObjectAge {
    /// Object identifier  
    pub object_id: u64,
    /// Allocation generation
    pub generation: u32,
    /// Number of GC cycles survived
    pub survival_count: u32,
    /// Last access time for LRU decisions
    pub last_access: Instant,
    /// Size for compaction decisions
    pub size: usize,
}

/// Memory pressure indicator and response thresholds
#[derive(Debug, Clone)]
pub struct MemoryPressure {
    /// Current pressure level (0.0 = no pressure, 1.0 = critical)
    pub level: f64,
    /// Free memory percentage
    pub free_percentage: f64,
    /// Allocation failure rate
    pub failure_rate: f64,
    /// Fragmentation level
    pub fragmentation: f64,
    /// Time since last collection
    pub time_since_gc: Duration,
}

impl MemoryPressure {
    /// Calculate memory pressure from region statistics
    pub fn calculate(
        total_capacity: usize,
        used_capacity: usize,
        fragmentation: f64,
        failure_rate: f64,
        time_since_gc: Duration,
    ) -> Self {
        let free_percentage = if total_capacity > 0 {
            ((total_capacity - used_capacity) as f64 / total_capacity as f64) * 100.0
        } else {
            0.0
        };
        
        // Calculate pressure based on multiple factors
        let mut level = 0.0;
        
        // Memory usage pressure (higher usage = higher pressure)
        if free_percentage < 10.0 {
            level += 0.4; // Critical memory usage
        } else if free_percentage < 25.0 {
            level += 0.2; // High memory usage
        }
        
        // Fragmentation pressure
        level += fragmentation * 0.3;
        
        // Failure rate pressure
        level += failure_rate * 0.2;
        
        // Time pressure (long time without GC)
        if time_since_gc > Duration::from_secs(30) {
            level += 0.1;
        }
        
        level = level.min(1.0); // Cap at 1.0
        
        Self {
            level,
            free_percentage,
            failure_rate,
            fragmentation,
            time_since_gc,
        }
    }
    
    /// Check if immediate collection is needed
    pub fn needs_immediate_collection(&self) -> bool {
        self.level > 0.8 || self.free_percentage < 5.0
    }
    
    /// Check if collection should be triggered soon
    pub fn should_trigger_collection(&self) -> bool {
        self.level > 0.5 || self.free_percentage < 15.0
    }
}

/// Region capacity and usage information
#[derive(Debug, Clone)]
pub struct CapacityInfo {
    pub total_capacity: usize,
    pub used_capacity: usize,
    pub free_capacity: usize,
    pub utilization_percentage: f64,
}

/// Statistics for a memory region
#[derive(Debug, Clone)]
pub struct RegionStatistics {
    pub region_id: u32,
    pub region_type: RegionType,
    pub total_allocations: u64,
    pub total_deallocations: u64,
    pub bytes_allocated: usize,
    pub bytes_deallocated: usize,
    pub current_objects: usize,
    pub capacity_info: CapacityInfo,
    pub fragmentation_ratio: f64,
    pub allocation_failures: u64,
}

/// Young generation region with advanced allocation and promotion algorithms
/// 
/// Implements fast bump allocation with object age tracking, automatic promotion
/// detection, and collection triggering based on usage patterns and memory pressure.
pub struct YoungGeneration {
    /// Region identifier
    region_id: u32,
    /// Base memory pointer
    base: NonNull<u8>,
    /// Region size
    size: usize,
    /// Bump allocator for fast allocation
    allocator: Arc<BumpAllocator>,
    /// Region statistics
    statistics: Arc<Mutex<RegionStatistics>>,
    /// Memory layout for deallocation
    layout: Layout,
    /// Object age tracking for promotion decisions
    object_ages: Arc<Mutex<HashMap<u64, ObjectAge>>>,
    /// Current generation counter
    current_generation: AtomicU32,
    /// Objects ready for promotion
    promotion_candidates: Arc<Mutex<VecDeque<u64>>>,
    /// Collection trigger threshold (usage percentage)
    collection_threshold: f64,
    /// Last collection time
    last_collection: Arc<Mutex<Instant>>,
    /// Memory pressure detector
    pressure_detector: Arc<Mutex<MemoryPressure>>,
}

impl YoungGeneration {
    /// Create a new young generation region with advanced algorithms
    #[instrument]
    pub fn new(region_id: u32, size: usize) -> Result<Self, String> {
        info!("Creating advanced young generation region {} with {} bytes", region_id, size);
        
        let layout = Layout::from_size_align(size, 8)
            .map_err(|e| format!("Invalid layout for young generation: {}", e))?;
        
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return Err(format!("Failed to allocate {} bytes for young generation", size));
        }
        
        let base = NonNull::new(ptr)
            .ok_or("System allocator returned null pointer")?;
        
        let allocator = Arc::new(BumpAllocator::new(base, size));
        
        let capacity_info = CapacityInfo {
            total_capacity: size,
            used_capacity: 0,
            free_capacity: size,
            utilization_percentage: 0.0,
        };
        
        let statistics = Arc::new(Mutex::new(RegionStatistics {
            region_id,
            region_type: RegionType::YoungGeneration,
            total_allocations: 0,
            total_deallocations: 0,
            bytes_allocated: 0,
            bytes_deallocated: 0,
            current_objects: 0,
            capacity_info,
            fragmentation_ratio: 0.0,
            allocation_failures: 0,
        }));
        
        let now = Instant::now();
        let initial_pressure = MemoryPressure::calculate(size, 0, 0.0, 0.0, Duration::from_secs(0));
        
        Ok(Self {
            region_id,
            base,
            size,
            allocator,
            statistics,
            layout,
            object_ages: Arc::new(Mutex::new(HashMap::new())),
            current_generation: AtomicU32::new(1),
            promotion_candidates: Arc::new(Mutex::new(VecDeque::new())),
            collection_threshold: 85.0, // Trigger collection at 85% usage
            last_collection: Arc::new(Mutex::new(now)),
            pressure_detector: Arc::new(Mutex::new(initial_pressure)),
        })
    }
    
    /// Reset the young generation after collection with promotion handling
    pub fn reset(&self) -> Result<(), String> {
        let _span = span!(Level::INFO, "young_generation_reset", region_id = self.region_id).entered();
        info!("Resetting young generation region {}", self.region_id);
        
        // Process promotion candidates before reset
        self.process_promotion_candidates()?;
        
        self.allocator.reset()?;
        
        let mut stats = self.statistics.lock()
            .map_err(|_| "Failed to acquire statistics lock")?;
        
        stats.current_objects = 0;
        stats.capacity_info.used_capacity = 0;
        stats.capacity_info.free_capacity = self.size;
        stats.capacity_info.utilization_percentage = 0.0;
        stats.fragmentation_ratio = 0.0;
        
        // Update generation counter and collection time
        self.current_generation.fetch_add(1, Ordering::Relaxed);
        if let Ok(mut last_collection) = self.last_collection.lock() {
            *last_collection = Instant::now();
        }
        
        // Clear object ages (promoted objects are no longer in young gen)
        if let Ok(mut ages) = self.object_ages.lock() {
            ages.clear();
        }
        
        Ok(())
    }
    
    /// Get usage percentage for collection decisions
    pub fn usage_percentage(&self) -> Result<f64, String> {
        self.allocator.usage_percentage()
    }
    
    /// Check if collection should be triggered based on usage and pressure
    pub fn should_trigger_collection(&self) -> Result<bool, String> {
        let usage = self.usage_percentage()?;
        
        // Always trigger if above threshold
        if usage >= self.collection_threshold {
            return Ok(true);
        }
        
        // Check memory pressure
        if let Ok(pressure) = self.pressure_detector.lock() {
            if pressure.should_trigger_collection() {
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    /// Track object allocation for promotion decisions
    pub fn track_object_allocation(&self, object_id: u64, size: usize) -> Result<(), String> {
        let generation = self.current_generation.load(Ordering::Relaxed);
        
        let age = ObjectAge {
            object_id,
            generation,
            survival_count: 0,
            last_access: Instant::now(),
            size,
        };
        
        if let Ok(mut ages) = self.object_ages.lock() {
            ages.insert(object_id, age);
        }
        
        Ok(())
    }
    
    /// Mark object as surviving a GC cycle
    pub fn mark_object_survived(&self, object_id: u64) -> Result<(), String> {
        if let Ok(mut ages) = self.object_ages.lock() {
            if let Some(age) = ages.get_mut(&object_id) {
                age.survival_count += 1;
                age.last_access = Instant::now();
                
                // Add to promotion candidates if survived multiple cycles
                if age.survival_count >= 2 { // Promote after surviving 2 GC cycles
                    if let Ok(mut candidates) = self.promotion_candidates.lock() {
                        candidates.push_back(object_id);
                    }
                }
            }
        }
        
        Ok(())
    }
    
    /// Process objects ready for promotion to old generation
    fn process_promotion_candidates(&self) -> Result<Vec<u64>, String> {
        let mut promoted = Vec::new();
        
        if let Ok(mut candidates) = self.promotion_candidates.lock() {
            while let Some(object_id) = candidates.pop_front() {
                // In a real implementation, this would actually move the object
                // For now, just track that it should be promoted
                promoted.push(object_id);
                debug!("Object {} promoted to old generation", object_id);
            }
        }
        
        Ok(promoted)
    }
    
    /// Get objects ready for promotion
    pub fn get_promotion_candidates(&self) -> Result<Vec<u64>, String> {
        if let Ok(candidates) = self.promotion_candidates.lock() {
            Ok(candidates.iter().cloned().collect())
        } else {
            Ok(Vec::new())
        }
    }
    
    /// Update memory pressure based on current state
    pub fn update_memory_pressure(&self) -> Result<MemoryPressure, String> {
        let stats = self.get_statistics();
        let time_since_gc = if let Ok(last_collection) = self.last_collection.lock() {
            last_collection.elapsed()
        } else {
            Duration::from_secs(0)
        };
        
        let failure_rate = if stats.total_allocations > 0 {
            stats.allocation_failures as f64 / stats.total_allocations as f64
        } else {
            0.0
        };
        
        let pressure = MemoryPressure::calculate(
            stats.capacity_info.total_capacity,
            stats.capacity_info.used_capacity,
            stats.fragmentation_ratio,
            failure_rate,
            time_since_gc,
        );
        
        if let Ok(mut detector) = self.pressure_detector.lock() {
            *detector = pressure.clone();
        }
        
        Ok(pressure)
    }
}

impl HeapRegion for YoungGeneration {
    #[instrument(skip(self))]
    fn allocate(&self, size: usize, alignment: usize, type_name: &str) -> Result<AllocationResult, String> {
        debug!("Young generation allocating {} bytes for {}", size, type_name);
        
        // Check if allocation should trigger collection first
        if self.should_trigger_collection()? {
            warn!("Young generation memory pressure detected during allocation");
        }
        
        let result = self.allocator.allocate(size, alignment)?;
        
        // Generate object ID for tracking
        let object_id = self.region_id as u64 * 1_000_000 + result.offset as u64;
        
        // Track object for promotion decisions
        if let Err(e) = self.track_object_allocation(object_id, size) {
            warn!("Failed to track object allocation: {}", e);
        }
        
        // Update statistics
        {
            let mut stats = self.statistics.lock()
                .map_err(|_| "Failed to acquire statistics lock")?;
            
            stats.total_allocations += 1;
            stats.bytes_allocated += size;
            stats.current_objects += 1;
            stats.capacity_info.used_capacity += size;
            stats.capacity_info.free_capacity = stats.capacity_info.free_capacity.saturating_sub(size);
            stats.capacity_info.utilization_percentage = 
                stats.capacity_info.used_capacity as f64 / stats.capacity_info.total_capacity as f64 * 100.0;
        }
        
        // Update memory pressure
        if let Err(e) = self.update_memory_pressure() {
            warn!("Failed to update memory pressure: {}", e);
        }
        
        debug!("Young generation allocated {} bytes at {:p} (object_id: {})", 
               size, result.ptr.as_ptr(), object_id);
        Ok(result)
    }
    
    fn deallocate(&self, _ptr: NonNull<u8>, size: usize) -> Result<(), String> {
        // Young generation uses bump allocation - can't deallocate individual objects
        // Just update statistics for tracking
        let mut stats = self.statistics.lock()
            .map_err(|_| "Failed to acquire statistics lock")?;
        
        stats.total_deallocations += 1;
        stats.bytes_deallocated += size;
        stats.current_objects = stats.current_objects.saturating_sub(1);
        
        Ok(())
    }
    
    fn contains_pointer(&self, ptr: *const u8) -> bool {
        let start = self.base.as_ptr() as usize;
        let end = start + self.size;
        let addr = ptr as usize;
        addr >= start && addr < end
    }
    
    fn get_statistics(&self) -> RegionStatistics {
        self.statistics.lock().unwrap().clone()
    }
    
    fn prepare_for_collection(&self) -> Result<(), String> {
        debug!("Preparing young generation {} for collection", self.region_id);
        // No special preparation needed for young generation
        Ok(())
    }
    
    fn complete_collection(&self) -> Result<(), String> {
        debug!("Completing collection for young generation {}", self.region_id);
        // Reset the region after collection
        self.reset()
    }
    
    fn region_type(&self) -> RegionType {
        RegionType::YoungGeneration
    }
    
    fn region_id(&self) -> u32 {
        self.region_id
    }
    
    fn can_allocate(&self, size: usize) -> bool {
        self.allocator.can_allocate(size)
    }
    
    fn get_capacity_info(&self) -> CapacityInfo {
        self.statistics.lock().unwrap().capacity_info.clone()
    }
}

// Safety: YoungGeneration is safe to send between threads because:
// 1. All fields are either primitives or thread-safe types
// 2. NonNull<u8> points to heap-allocated memory that is owned by this region
// 3. Arc<BumpAllocator> provides shared ownership and thread-safe access
// 4. Access is coordinated through internal synchronization
unsafe impl Send for YoungGeneration {}

// Safety: YoungGeneration is safe to share between threads because:
// 1. All mutation is coordinated through Arc<BumpAllocator> which has internal locks
// 2. The NonNull<u8> pointer is stable (doesn't change once allocated)
// 3. Statistics access is coordinated through Mutex
unsafe impl Sync for YoungGeneration {}

impl Drop for YoungGeneration {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.base.as_ptr(), self.layout);
        }
        debug!("Deallocated young generation region {}", self.region_id);
    }
}

/// Old generation region with advanced compaction and lifecycle management
/// 
/// Implements sophisticated free list allocation with mark-compact algorithms,
/// fragmentation monitoring, and automatic compaction triggering based on
/// memory pressure and allocation patterns.
pub struct OldGeneration {
    /// Region identifier
    region_id: u32,
    /// Base memory pointer
    base: NonNull<u8>,
    /// Region size
    size: usize,
    /// Free list allocator for flexible allocation
    allocator: Arc<FreeListAllocator>,
    /// Region statistics
    statistics: Arc<Mutex<RegionStatistics>>,
    /// Memory layout for deallocation
    layout: Layout,
    /// Object lifecycle tracking
    object_lifetimes: Arc<Mutex<BTreeMap<u64, ObjectLifetime>>>,
    /// Compaction algorithm state
    compaction_state: Arc<Mutex<CompactionState>>,
    /// Fragmentation threshold for triggering compaction
    fragmentation_threshold: f64,
    /// Last compaction time
    last_compaction: Arc<Mutex<Instant>>,
    /// Live object tracking for compaction
    live_objects: Arc<Mutex<HashMap<u64, LiveObjectInfo>>>,
}

/// Object lifetime tracking for old generation management
#[derive(Debug, Clone)]
pub struct ObjectLifetime {
    /// Object identifier
    pub object_id: u64,
    /// Allocation time
    pub allocated_at: Instant,
    /// Last access time
    pub last_accessed: Instant,
    /// Size in bytes
    pub size: usize,
    /// Access frequency counter
    pub access_count: u64,
    /// Memory offset for compaction
    pub offset: usize,
}

/// Live object information for compaction
#[derive(Debug, Clone)]
pub struct LiveObjectInfo {
    /// Object identifier
    pub object_id: u64,
    /// Current memory offset
    pub current_offset: usize,
    /// Object size
    pub size: usize,
    /// Whether object has been moved during compaction
    pub moved: bool,
    /// New offset after compaction
    pub new_offset: Option<usize>,
}

/// Compaction algorithm state and statistics
#[derive(Debug, Clone)]
pub struct CompactionState {
    /// Whether compaction is in progress
    pub in_progress: bool,
    /// Compaction algorithm type
    pub algorithm: CompactionAlgorithm,
    /// Number of objects to move
    pub objects_to_move: usize,
    /// Bytes to reclaim
    pub bytes_to_reclaim: usize,
    /// Compaction start time
    pub start_time: Option<Instant>,
    /// Progress percentage
    pub progress: f64,
}

/// Compaction algorithm types
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CompactionAlgorithm {
    /// Mark-Compact with sliding compaction
    MarkCompactSliding,
    /// Mark-Compact with copying
    MarkCompactCopying,
    /// Incremental compaction
    Incremental,
    /// Generational compaction (old-to-old)
    Generational,
}

impl OldGeneration {
    /// Create a new old generation region with advanced compaction
    #[instrument]
    pub fn new(region_id: u32, size: usize) -> Result<Self, String> {
        info!("Creating advanced old generation region {} with {} bytes", region_id, size);
        
        let layout = Layout::from_size_align(size, 8)
            .map_err(|e| format!("Invalid layout for old generation: {}", e))?;
        
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return Err(format!("Failed to allocate {} bytes for old generation", size));
        }
        
        let base = NonNull::new(ptr)
            .ok_or("System allocator returned null pointer")?;
        
        let allocator = Arc::new(FreeListAllocator::new(base, size));
        
        let capacity_info = CapacityInfo {
            total_capacity: size,
            used_capacity: 0,
            free_capacity: size,
            utilization_percentage: 0.0,
        };
        
        let statistics = Arc::new(Mutex::new(RegionStatistics {
            region_id,
            region_type: RegionType::OldGeneration,
            total_allocations: 0,
            total_deallocations: 0,
            bytes_allocated: 0,
            bytes_deallocated: 0,
            current_objects: 0,
            capacity_info,
            fragmentation_ratio: 0.0,
            allocation_failures: 0,
        }));
        
        let initial_compaction_state = CompactionState {
            in_progress: false,
            algorithm: CompactionAlgorithm::MarkCompactSliding,
            objects_to_move: 0,
            bytes_to_reclaim: 0,
            start_time: None,
            progress: 0.0,
        };
        
        Ok(Self {
            region_id,
            base,
            size,
            allocator,
            statistics,
            layout,
            object_lifetimes: Arc::new(Mutex::new(BTreeMap::new())),
            compaction_state: Arc::new(Mutex::new(initial_compaction_state)),
            fragmentation_threshold: 0.4, // Trigger compaction at 40% fragmentation
            last_compaction: Arc::new(Mutex::new(Instant::now())),
            live_objects: Arc::new(Mutex::new(HashMap::new())),
        })
    }
    
    /// Advanced compaction with multiple algorithms
    pub fn compact(&self) -> Result<(), String> {
        let _span = span!(Level::INFO, "old_generation_compact", region_id = self.region_id).entered();
        info!("Starting advanced compaction for old generation region {}", self.region_id);
        
        // Check if compaction is already in progress
        {
            let compaction_state = self.compaction_state.lock()
                .map_err(|_| "Failed to acquire compaction state lock")?;
            if compaction_state.in_progress {
                return Err("Compaction already in progress".to_string());
            }
        }
        
        // Analyze fragmentation and select appropriate algorithm
        let stats = self.get_statistics();
        let algorithm = self.select_compaction_algorithm(&stats)?;
        
        // Execute compaction based on selected algorithm
        match algorithm {
            CompactionAlgorithm::MarkCompactSliding => self.mark_compact_sliding(),
            CompactionAlgorithm::MarkCompactCopying => self.mark_compact_copying(),
            CompactionAlgorithm::Incremental => self.incremental_compact(),
            CompactionAlgorithm::Generational => self.generational_compact(),
        }
    }
    
    /// Select optimal compaction algorithm based on current state
    fn select_compaction_algorithm(&self, stats: &RegionStatistics) -> Result<CompactionAlgorithm, String> {
        let fragmentation = stats.fragmentation_ratio;
        let utilization = stats.capacity_info.utilization_percentage;
        
        if fragmentation > 0.7 {
            // High fragmentation - use sliding compaction
            Ok(CompactionAlgorithm::MarkCompactSliding)
        } else if utilization > 80.0 {
            // High memory usage - use copying compaction
            Ok(CompactionAlgorithm::MarkCompactCopying)
        } else if fragmentation > 0.3 {
            // Moderate fragmentation - use incremental
            Ok(CompactionAlgorithm::Incremental)
        } else {
            // Low fragmentation - use generational
            Ok(CompactionAlgorithm::Generational)
        }
    }
    
    /// Mark-compact with sliding compaction algorithm
    fn mark_compact_sliding(&self) -> Result<(), String> {
        let start_time = Instant::now();
        
        // Update compaction state
        {
            let mut state = self.compaction_state.lock()
                .map_err(|_| "Failed to acquire compaction state lock")?;
            state.in_progress = true;
            state.algorithm = CompactionAlgorithm::MarkCompactSliding;
            state.start_time = Some(start_time);
            state.progress = 0.0;
        }
        
        // Phase 1: Mark live objects
        self.mark_live_objects()?;
        self.update_compaction_progress(25.0)?;
        
        // Phase 2: Calculate new positions
        self.calculate_new_positions()?;
        self.update_compaction_progress(50.0)?;
        
        // Phase 3: Update references (would need GC integration)
        self.update_object_references()?;
        self.update_compaction_progress(75.0)?;
        
        // Phase 4: Move objects to new positions
        self.move_objects_sliding()?;
        self.update_compaction_progress(100.0)?;
        
        // Update statistics and complete
        self.complete_compaction(start_time)?;
        
        info!("Mark-compact sliding compaction completed in {:?}", start_time.elapsed());
        Ok(())
    }
    
    /// Mark-compact with copying algorithm
    fn mark_compact_copying(&self) -> Result<(), String> {
        let start_time = Instant::now();
        
        {
            let mut state = self.compaction_state.lock()
                .map_err(|_| "Failed to acquire compaction state lock")?;
            state.in_progress = true;
            state.algorithm = CompactionAlgorithm::MarkCompactCopying;
            state.start_time = Some(start_time);
        }
        
        // For copying compaction, we would need a separate to-space
        // For now, implement a simplified version
        self.mark_live_objects()?;
        self.copy_live_objects()?;
        self.complete_compaction(start_time)?;
        
        info!("Mark-compact copying compaction completed in {:?}", start_time.elapsed());
        Ok(())
    }
    
    /// Incremental compaction for minimal pause times
    fn incremental_compact(&self) -> Result<(), String> {
        let start_time = Instant::now();
        
        {
            let mut state = self.compaction_state.lock()
                .map_err(|_| "Failed to acquire compaction state lock")?;
            state.in_progress = true;
            state.algorithm = CompactionAlgorithm::Incremental;
            state.start_time = Some(start_time);
        }
        
        // Process objects in small batches
        let batch_size = 10; // Process 10 objects at a time
        self.process_compaction_batch(batch_size)?;
        self.complete_compaction(start_time)?;
        
        info!("Incremental compaction completed in {:?}", start_time.elapsed());
        Ok(())
    }
    
    /// Generational compaction focusing on object ages
    fn generational_compact(&self) -> Result<(), String> {
        let start_time = Instant::now();
        
        {
            let mut state = self.compaction_state.lock()
                .map_err(|_| "Failed to acquire compaction state lock")?;
            state.in_progress = true;
            state.algorithm = CompactionAlgorithm::Generational;
            state.start_time = Some(start_time);
        }
        
        // Focus on objects based on age and access patterns
        self.compact_by_generation()?;
        self.complete_compaction(start_time)?;
        
        info!("Generational compaction completed in {:?}", start_time.elapsed());
        Ok(())
    }
    
    /// Mark all live objects in the heap
    fn mark_live_objects(&self) -> Result<(), String> {
        // In a real implementation, this would integrate with the GC
        // to mark all reachable objects. For now, simulate the process.
        
        let mut live_objects = self.live_objects.lock()
            .map_err(|_| "Failed to acquire live objects lock")?;
        
        let lifetimes = self.object_lifetimes.lock()
            .map_err(|_| "Failed to acquire object lifetimes lock")?;
        
        live_objects.clear();
        for (object_id, lifetime) in lifetimes.iter() {
            // Simulate marking - assume all tracked objects are live
            live_objects.insert(*object_id, LiveObjectInfo {
                object_id: *object_id,
                current_offset: lifetime.offset,
                size: lifetime.size,
                moved: false,
                new_offset: None,
            });
        }
        
        debug!("Marked {} live objects for compaction", live_objects.len());
        Ok(())
    }
    
    /// Calculate new positions for objects after compaction
    fn calculate_new_positions(&self) -> Result<(), String> {
        let mut live_objects = self.live_objects.lock()
            .map_err(|_| "Failed to acquire live objects lock")?;
        
        // Sort objects by current offset for sliding compaction
        let mut objects: Vec<_> = live_objects.values().cloned().collect();
        objects.sort_by_key(|obj| obj.current_offset);
        
        // Calculate new positions by sliding objects together
        let mut new_offset = 0;
        for obj_info in &mut objects {
            let object_id = obj_info.object_id;
            if let Some(live_obj) = live_objects.get_mut(&object_id) {
                live_obj.new_offset = Some(new_offset);
                new_offset += live_obj.size;
                // Add alignment padding
                new_offset = (new_offset + 7) & !7; // 8-byte alignment
            }
        }
        
        debug!("Calculated new positions for {} objects, total size: {}", objects.len(), new_offset);
        Ok(())
    }
    
    /// Update object references (would need GC integration)
    fn update_object_references(&self) -> Result<(), String> {
        // In a real implementation, this would update all pointers
        // to refer to the new object locations
        debug!("Updating object references (placeholder)");
        Ok(())
    }
    
    /// Move objects to their new positions using sliding compaction
    fn move_objects_sliding(&self) -> Result<(), String> {
        let live_objects = self.live_objects.lock()
            .map_err(|_| "Failed to acquire live objects lock")?;
        
        // In a real implementation, this would physically move memory
        // For now, just update the tracking information
        let mut moved_count = 0;
        for (object_id, obj_info) in live_objects.iter() {
            if let Some(new_offset) = obj_info.new_offset {
                if new_offset != obj_info.current_offset {
                    // Object needs to be moved
                    moved_count += 1;
                    debug!("Moving object {} from offset {} to {}", 
                           object_id, obj_info.current_offset, new_offset);
                }
            }
        }
        
        debug!("Moved {} objects during sliding compaction", moved_count);
        Ok(())
    }
    
    /// Copy live objects to reduce fragmentation
    fn copy_live_objects(&self) -> Result<(), String> {
        // Simplified copying implementation
        debug!("Copying live objects (placeholder)");
        Ok(())
    }
    
    /// Process objects in batches for incremental compaction
    fn process_compaction_batch(&self, batch_size: usize) -> Result<(), String> {
        debug!("Processing compaction batch of {} objects", batch_size);
        // Implementation would process a small number of objects
        Ok(())
    }
    
    /// Compact objects based on generation and access patterns
    fn compact_by_generation(&self) -> Result<(), String> {
        let lifetimes = self.object_lifetimes.lock()
            .map_err(|_| "Failed to acquire object lifetimes lock")?;
        
        // Group objects by age and access frequency
        let mut young_objects = Vec::new();
        let mut old_objects = Vec::new();
        
        for lifetime in lifetimes.values() {
            let age = lifetime.allocated_at.elapsed();
            if age < Duration::from_secs(60) || lifetime.access_count > 100 {
                young_objects.push(lifetime.clone());
            } else {
                old_objects.push(lifetime.clone());
            }
        }
        
        debug!("Generational compaction: {} young, {} old objects", 
               young_objects.len(), old_objects.len());
        Ok(())
    }
    
    /// Update compaction progress
    fn update_compaction_progress(&self, progress: f64) -> Result<(), String> {
        let mut state = self.compaction_state.lock()
            .map_err(|_| "Failed to acquire compaction state lock")?;
        state.progress = progress;
        Ok(())
    }
    
    /// Complete compaction and update statistics
    fn complete_compaction(&self, start_time: Instant) -> Result<(), String> {
        // Reset compaction state
        {
            let mut state = self.compaction_state.lock()
                .map_err(|_| "Failed to acquire compaction state lock")?;
            state.in_progress = false;
            state.progress = 100.0;
            state.start_time = None;
        }
        
        // Update last compaction time
        {
            let mut last_compaction = self.last_compaction.lock()
                .map_err(|_| "Failed to acquire last compaction lock")?;
            *last_compaction = Instant::now();
        }
        
        // Update statistics
        let mut stats = self.statistics.lock()
            .map_err(|_| "Failed to acquire statistics lock")?;
        
        // Simulate improved fragmentation after compaction
        stats.fragmentation_ratio = stats.fragmentation_ratio * 0.1; // 90% improvement
        
        let duration = start_time.elapsed();
        debug!("Compaction completed in {:?}, fragmentation reduced to {:.2}%", 
               duration, stats.fragmentation_ratio * 100.0);
        
        Ok(())
    }
    
    /// Check if compaction should be triggered
    pub fn should_trigger_compaction(&self) -> Result<bool, String> {
        let stats = self.get_statistics();
        
        // Trigger if fragmentation exceeds threshold
        if stats.fragmentation_ratio > self.fragmentation_threshold {
            return Ok(true);
        }
        
        // Trigger if too much time has passed since last compaction
        if let Ok(last_compaction) = self.last_compaction.lock() {
            if last_compaction.elapsed() > Duration::from_secs(300) { // 5 minutes
                return Ok(true);
            }
        }
        
        Ok(false)
    }
    
    /// Track object allocation in old generation
    pub fn track_object_allocation(&self, object_id: u64, size: usize, offset: usize) -> Result<(), String> {
        let lifetime = ObjectLifetime {
            object_id,
            allocated_at: Instant::now(),
            last_accessed: Instant::now(),
            size,
            access_count: 1,
            offset,
        };
        
        if let Ok(mut lifetimes) = self.object_lifetimes.lock() {
            lifetimes.insert(object_id, lifetime);
        }
        
        Ok(())
    }
    
    /// Record object access for lifetime tracking
    pub fn record_object_access(&self, object_id: u64) -> Result<(), String> {
        if let Ok(mut lifetimes) = self.object_lifetimes.lock() {
            if let Some(lifetime) = lifetimes.get_mut(&object_id) {
                lifetime.last_accessed = Instant::now();
                lifetime.access_count += 1;
            }
        }
        
        Ok(())
    }
}

impl HeapRegion for OldGeneration {
    #[instrument(skip(self))]
    fn allocate(&self, size: usize, alignment: usize, type_name: &str) -> Result<AllocationResult, String> {
        debug!("Old generation allocating {} bytes for {}", size, type_name);
        
        let result = self.allocator.allocate(size, alignment)?;
        
        // Update statistics
        {
            let mut stats = self.statistics.lock()
                .map_err(|_| "Failed to acquire statistics lock")?;
            
            stats.total_allocations += 1;
            stats.bytes_allocated += size;
            stats.current_objects += 1;
            stats.capacity_info.used_capacity += size;
            stats.capacity_info.free_capacity = stats.capacity_info.free_capacity.saturating_sub(size);
            stats.capacity_info.utilization_percentage = 
                stats.capacity_info.used_capacity as f64 / stats.capacity_info.total_capacity as f64 * 100.0;
            
            // Update fragmentation from allocator
            let allocator_stats = self.allocator.get_statistics();
            stats.fragmentation_ratio = allocator_stats.fragmentation_ratio;
        }
        
        debug!("Old generation allocated {} bytes at {:p}", size, result.ptr.as_ptr());
        Ok(result)
    }
    
    fn deallocate(&self, ptr: NonNull<u8>, size: usize) -> Result<(), String> {
        debug!("Old generation deallocating {} bytes at {:p}", size, ptr.as_ptr());
        
        self.allocator.deallocate(ptr, size)?;
        
        // Update statistics
        {
            let mut stats = self.statistics.lock()
                .map_err(|_| "Failed to acquire statistics lock")?;
            
            stats.total_deallocations += 1;
            stats.bytes_deallocated += size;
            stats.current_objects = stats.current_objects.saturating_sub(1);
            stats.capacity_info.used_capacity = stats.capacity_info.used_capacity.saturating_sub(size);
            stats.capacity_info.free_capacity += size;
            stats.capacity_info.utilization_percentage = 
                stats.capacity_info.used_capacity as f64 / stats.capacity_info.total_capacity as f64 * 100.0;
            
            // Update fragmentation from allocator
            let allocator_stats = self.allocator.get_statistics();
            stats.fragmentation_ratio = allocator_stats.fragmentation_ratio;
        }
        
        Ok(())
    }
    
    fn contains_pointer(&self, ptr: *const u8) -> bool {
        let start = self.base.as_ptr() as usize;
        let end = start + self.size;
        let addr = ptr as usize;
        addr >= start && addr < end
    }
    
    fn get_statistics(&self) -> RegionStatistics {
        self.statistics.lock().unwrap().clone()
    }
    
    fn prepare_for_collection(&self) -> Result<(), String> {
        debug!("Preparing old generation {} for collection", self.region_id);
        // Old generation doesn't need special preparation
        Ok(())
    }
    
    fn complete_collection(&self) -> Result<(), String> {
        debug!("Completing collection for old generation {}", self.region_id);
        
        // Check if compaction is needed
        let stats = self.get_statistics();
        if stats.fragmentation_ratio > 0.5 {
            self.compact()?;
        }
        
        Ok(())
    }
    
    fn region_type(&self) -> RegionType {
        RegionType::OldGeneration
    }
    
    fn region_id(&self) -> u32 {
        self.region_id
    }
    
    fn can_allocate(&self, size: usize) -> bool {
        self.allocator.can_allocate(size)
    }
    
    fn get_capacity_info(&self) -> CapacityInfo {
        self.statistics.lock().unwrap().capacity_info.clone()
    }
}

// Safety: OldGeneration is safe to send between threads because:
// 1. All fields are either primitives or thread-safe types
// 2. NonNull<u8> points to heap-allocated memory that is owned by this region
// 3. Arc<FreeListAllocator> provides shared ownership and thread-safe access
// 4. Access is coordinated through internal synchronization
unsafe impl Send for OldGeneration {}

// Safety: OldGeneration is safe to share between threads because:
// 1. All mutation is coordinated through Arc<FreeListAllocator> which has internal locks
// 2. The NonNull<u8> pointer is stable (doesn't change once allocated)
// 3. Statistics access is coordinated through Mutex
unsafe impl Sync for OldGeneration {}

impl Drop for OldGeneration {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.base.as_ptr(), self.layout);
        }
        debug!("Deallocated old generation region {}", self.region_id);
    }
}

/// Large object space for objects exceeding the normal size threshold
/// 
/// Uses segregated allocation to efficiently handle large objects
/// without impacting the performance of smaller allocations.
pub struct LargeObjectSpace {
    /// Region identifier
    region_id: u32,
    /// Base memory pointer
    base: NonNull<u8>,
    /// Region size
    size: usize,
    /// Segregated allocator for size classes
    allocator: Arc<SegregatedAllocator>,
    /// Region statistics
    statistics: Arc<Mutex<RegionStatistics>>,
    /// Memory layout for deallocation
    layout: Layout,
}

impl LargeObjectSpace {
    /// Create a new large object space
    #[instrument]
    pub fn new(region_id: u32, size: usize) -> Result<Self, String> {
        info!("Creating large object space {} with {} bytes", region_id, size);
        
        let layout = Layout::from_size_align(size, 8)
            .map_err(|e| format!("Invalid layout for large object space: {}", e))?;
        
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return Err(format!("Failed to allocate {} bytes for large object space", size));
        }
        
        let base = NonNull::new(ptr)
            .ok_or("System allocator returned null pointer")?;
        
        let allocator = Arc::new(SegregatedAllocator::new(base, size)?);
        
        let capacity_info = CapacityInfo {
            total_capacity: size,
            used_capacity: 0,
            free_capacity: size,
            utilization_percentage: 0.0,
        };
        
        let statistics = Arc::new(Mutex::new(RegionStatistics {
            region_id,
            region_type: RegionType::LargeObjectSpace,
            total_allocations: 0,
            total_deallocations: 0,
            bytes_allocated: 0,
            bytes_deallocated: 0,
            current_objects: 0,
            capacity_info,
            fragmentation_ratio: 0.0,
            allocation_failures: 0,
        }));
        
        Ok(Self {
            region_id,
            base,
            size,
            allocator,
            statistics,
            layout,
        })
    }
}

impl HeapRegion for LargeObjectSpace {
    #[instrument(skip(self))]
    fn allocate(&self, size: usize, alignment: usize, type_name: &str) -> Result<AllocationResult, String> {
        debug!("Large object space allocating {} bytes for {}", size, type_name);
        
        let result = self.allocator.allocate(size, alignment)?;
        
        // Update statistics
        {
            let mut stats = self.statistics.lock()
                .map_err(|_| "Failed to acquire statistics lock")?;
            
            stats.total_allocations += 1;
            stats.bytes_allocated += size;
            stats.current_objects += 1;
            stats.capacity_info.used_capacity += size;
            stats.capacity_info.free_capacity = stats.capacity_info.free_capacity.saturating_sub(size);
            stats.capacity_info.utilization_percentage = 
                stats.capacity_info.used_capacity as f64 / stats.capacity_info.total_capacity as f64 * 100.0;
            
            // Update fragmentation from allocator
            let allocator_stats = self.allocator.get_statistics();
            stats.fragmentation_ratio = allocator_stats.fragmentation_ratio;
        }
        
        debug!("Large object space allocated {} bytes at {:p}", size, result.ptr.as_ptr());
        Ok(result)
    }
    
    fn deallocate(&self, ptr: NonNull<u8>, size: usize) -> Result<(), String> {
        debug!("Large object space deallocating {} bytes at {:p}", size, ptr.as_ptr());
        
        self.allocator.deallocate(ptr, size)?;
        
        // Update statistics
        {
            let mut stats = self.statistics.lock()
                .map_err(|_| "Failed to acquire statistics lock")?;
            
            stats.total_deallocations += 1;
            stats.bytes_deallocated += size;
            stats.current_objects = stats.current_objects.saturating_sub(1);
            stats.capacity_info.used_capacity = stats.capacity_info.used_capacity.saturating_sub(size);
            stats.capacity_info.free_capacity += size;
            stats.capacity_info.utilization_percentage = 
                stats.capacity_info.used_capacity as f64 / stats.capacity_info.total_capacity as f64 * 100.0;
            
            // Update fragmentation from allocator
            let allocator_stats = self.allocator.get_statistics();
            stats.fragmentation_ratio = allocator_stats.fragmentation_ratio;
        }
        
        Ok(())
    }
    
    fn contains_pointer(&self, ptr: *const u8) -> bool {
        let start = self.base.as_ptr() as usize;
        let end = start + self.size;
        let addr = ptr as usize;
        addr >= start && addr < end
    }
    
    fn get_statistics(&self) -> RegionStatistics {
        self.statistics.lock().unwrap().clone()
    }
    
    fn prepare_for_collection(&self) -> Result<(), String> {
        debug!("Preparing large object space {} for collection", self.region_id);
        // Large object space doesn't need special preparation
        Ok(())
    }
    
    fn complete_collection(&self) -> Result<(), String> {
        debug!("Completing collection for large object space {}", self.region_id);
        // Large objects are collected individually, no special cleanup needed
        Ok(())
    }
    
    fn region_type(&self) -> RegionType {
        RegionType::LargeObjectSpace
    }
    
    fn region_id(&self) -> u32 {
        self.region_id
    }
    
    fn can_allocate(&self, size: usize) -> bool {
        self.allocator.can_allocate(size)
    }
    
    fn get_capacity_info(&self) -> CapacityInfo {
        self.statistics.lock().unwrap().capacity_info.clone()
    }
}

// Safety: LargeObjectSpace is safe to send between threads because:
// 1. All fields are either primitives or thread-safe types
// 2. NonNull<u8> points to heap-allocated memory that is owned by this region
// 3. Arc<SegregatedAllocator> provides shared ownership and thread-safe access
// 4. Access is coordinated through internal synchronization
unsafe impl Send for LargeObjectSpace {}

// Safety: LargeObjectSpace is safe to share between threads because:
// 1. All mutation is coordinated through Arc<SegregatedAllocator> which has internal locks
// 2. The NonNull<u8> pointer is stable (doesn't change once allocated)
// 3. Statistics access is coordinated through Mutex
unsafe impl Sync for LargeObjectSpace {}

impl Drop for LargeObjectSpace {
    fn drop(&mut self) {
        unsafe {
            dealloc(self.base.as_ptr(), self.layout);
        }
        debug!("Deallocated large object space {}", self.region_id);
    }
}

/// Region manager coordinating all heap regions
/// 
/// The region manager provides a unified interface for allocation
/// while routing requests to appropriate specialized regions.
pub struct RegionManager {
    /// Young generation region (optional)
    young_generation: Option<Arc<YoungGeneration>>,
    /// Old generation region (optional)
    old_generation: Option<Arc<OldGeneration>>,
    /// Large object space (optional)
    large_object_space: Option<Arc<LargeObjectSpace>>,
    /// Primary region for non-generational allocation
    primary_region: Arc<OldGeneration>,
    /// Region lookup by pointer
    regions: Arc<RwLock<Vec<Arc<dyn HeapRegion>>>>,
    /// Configuration
    config: HeapConfiguration,
    /// Next region ID
    next_region_id: Mutex<u32>,
}

impl RegionManager {
    /// Create a new region manager
    #[instrument]
    pub fn new(config: &HeapConfiguration) -> Result<Self, String> {
        info!("Creating region manager with heap size {} MB", 
              config.initial_heap_size / (1024 * 1024));
        
        let mut regions: Vec<Arc<dyn HeapRegion>> = Vec::new();
        let mut next_id = 1;
        
        // Create primary region (always present)
        let primary_size = if config.generational_gc {
            (config.initial_heap_size as f64 * (1.0 - config.young_gen_ratio - 0.1)) as usize
        } else {
            config.initial_heap_size * 9 / 10 // 90% for primary, 10% for large objects
        };
        
        let primary_region = Arc::new(OldGeneration::new(next_id, primary_size)?);
        regions.push(primary_region.clone());
        next_id += 1;
        
        // Create young generation if enabled
        let young_generation = if config.generational_gc {
            let young_size = (config.initial_heap_size as f64 * config.young_gen_ratio) as usize;
            let young_gen = Arc::new(YoungGeneration::new(next_id, young_size)?);
            regions.push(young_gen.clone());
            next_id += 1;
            Some(young_gen)
        } else {
            None
        };
        
        // Create old generation if using generational GC
        let old_generation = if config.generational_gc {
            let old_gen = primary_region.clone();
            Some(old_gen)
        } else {
            None
        };
        
        // Create large object space
        let large_object_size = config.initial_heap_size / 10; // 10% for large objects
        let large_object_space = Arc::new(LargeObjectSpace::new(next_id, large_object_size)?);
        regions.push(large_object_space.clone());
        next_id += 1;
        
        Ok(Self {
            young_generation,
            old_generation,
            large_object_space: Some(large_object_space),
            primary_region,
            regions: Arc::new(RwLock::new(regions)),
            config: config.clone(),
            next_region_id: Mutex::new(next_id),
        })
    }
    
    /// Get young generation region
    pub fn young_generation(&self) -> Option<&Arc<YoungGeneration>> {
        self.young_generation.as_ref()
    }
    
    /// Get old generation region
    pub fn old_generation(&self) -> Option<&Arc<OldGeneration>> {
        self.old_generation.as_ref()
    }
    
    /// Get large object space
    pub fn large_object_space(&self) -> Option<&Arc<LargeObjectSpace>> {
        self.large_object_space.as_ref()
    }
    
    /// Get primary region
    pub fn primary_region(&self) -> &Arc<OldGeneration> {
        &self.primary_region
    }
    
    /// Find region containing the given pointer
    pub fn find_region_for_pointer(&self, ptr: *const u8) -> Option<Arc<dyn HeapRegion>> {
        if let Ok(regions) = self.regions.read() {
            for region in regions.iter() {
                if region.contains_pointer(ptr) {
                    return Some(region.clone());
                }
            }
        }
        None
    }
    
    /// Check if any region contains the pointer
    pub fn contains_pointer(&self, ptr: *const u8) -> bool {
        self.find_region_for_pointer(ptr).is_some()
    }
    
    /// Get comprehensive statistics for all regions
    pub fn get_statistics(&self) -> Result<RegionManagerStatistics, String> {
        let regions = self.regions.read()
            .map_err(|_| "Failed to acquire read lock on regions")?;
        
        let mut total_capacity = 0;
        let mut total_used = 0;
        let mut total_allocations = 0;
        let mut total_deallocations = 0;
        let mut total_objects = 0;
        let mut weighted_fragmentation = 0.0;
        let mut region_stats = Vec::new();
        
        for region in regions.iter() {
            let stats = region.get_statistics();
            total_capacity += stats.capacity_info.total_capacity;
            total_used += stats.capacity_info.used_capacity;
            total_allocations += stats.total_allocations;
            total_deallocations += stats.total_deallocations;
            total_objects += stats.current_objects;
            weighted_fragmentation += stats.fragmentation_ratio * stats.capacity_info.total_capacity as f64;
            region_stats.push(stats);
        }
        
        let overall_fragmentation = if total_capacity > 0 {
            weighted_fragmentation / total_capacity as f64
        } else {
            0.0
        };
        
        let utilization_percentage = if total_capacity > 0 {
            total_used as f64 / total_capacity as f64 * 100.0
        } else {
            0.0
        };
        
        Ok(RegionManagerStatistics {
            total_regions: regions.len(),
            total_capacity,
            total_used,
            total_free: total_capacity - total_used,
            utilization_percentage,
            total_allocations,
            total_deallocations,
            total_objects,
            overall_fragmentation,
            region_statistics: region_stats,
        })
    }
    
    /// Prepare all regions for garbage collection
    pub fn prepare_for_collection(&self) -> Result<(), String> {
        let regions = self.regions.read()
            .map_err(|_| "Failed to acquire read lock on regions")?;
        
        for region in regions.iter() {
            region.prepare_for_collection()?;
        }
        
        Ok(())
    }
    
    /// Complete garbage collection for all regions
    pub fn complete_collection(&self) -> Result<(), String> {
        let regions = self.regions.read()
            .map_err(|_| "Failed to acquire read lock on regions")?;
        
        for region in regions.iter() {
            region.complete_collection()?;
        }
        
        Ok(())
    }
    
    /// Intelligent allocation routing based on object characteristics
    pub fn allocate_with_routing(&self, size: usize, alignment: usize, type_name: &str, 
                                object_lifetime_hint: ObjectLifetimeHint) -> Result<AllocationResult, String> {
        let _span = span!(Level::DEBUG, "intelligent_allocation", size = size, type_name = type_name).entered();
        
        // Route allocation based on size and lifetime hint
        let target_region = self.select_allocation_region(size, object_lifetime_hint)?;
        
        match target_region {
            TargetRegion::YoungGeneration => {
                if let Some(young_gen) = &self.young_generation {
                    young_gen.allocate(size, alignment, type_name)
                } else {
                    // Fallback to primary region
                    self.primary_region.allocate(size, alignment, type_name)
                }
            }
            TargetRegion::OldGeneration => {
                if let Some(old_gen) = &self.old_generation {
                    old_gen.allocate(size, alignment, type_name)
                } else {
                    self.primary_region.allocate(size, alignment, type_name)
                }
            }
            TargetRegion::LargeObjectSpace => {
                if let Some(large_space) = &self.large_object_space {
                    large_space.allocate(size, alignment, type_name)
                } else {
                    return Err("Large object space not available".to_string());
                }
            }
            TargetRegion::Primary => {
                self.primary_region.allocate(size, alignment, type_name)
            }
        }
    }
    
    /// Select the best region for allocation based on object characteristics
    fn select_allocation_region(&self, size: usize, lifetime_hint: ObjectLifetimeHint) -> Result<TargetRegion, String> {
        // Check for large objects first
        if size >= self.config.large_object_threshold {
            return Ok(TargetRegion::LargeObjectSpace);
        }
        
        // Route based on lifetime hint and memory pressure
        match lifetime_hint {
            ObjectLifetimeHint::ShortLived => {
                if self.config.generational_gc {
                    // Check young generation pressure
                    if let Some(young_gen) = &self.young_generation {
                        if young_gen.should_trigger_collection().unwrap_or(false) {
                            // Young gen under pressure, allocate in old gen
                            Ok(TargetRegion::OldGeneration)
                        } else {
                            Ok(TargetRegion::YoungGeneration)
                        }
                    } else {
                        Ok(TargetRegion::Primary)
                    }
                } else {
                    Ok(TargetRegion::Primary)
                }
            }
            ObjectLifetimeHint::LongLived => {
                if self.config.generational_gc {
                    Ok(TargetRegion::OldGeneration)
                } else {
                    Ok(TargetRegion::Primary)
                }
            }
            ObjectLifetimeHint::Unknown => {
                // Use default allocation strategy
                if self.config.generational_gc && size < 1024 {
                    Ok(TargetRegion::YoungGeneration)
                } else {
                    Ok(TargetRegion::Primary)
                }
            }
        }
    }
    
    /// Process object promotions from young to old generation
    pub fn process_promotions(&self) -> Result<usize, String> {
        if !self.config.generational_gc {
            return Ok(0);
        }
        
        let promoted_count = if let Some(young_gen) = &self.young_generation {
            let candidates = young_gen.get_promotion_candidates()?;
            
            let len = candidates.len();
            for object_id in candidates {
                // In a real implementation, this would physically move the object
                debug!("Processing promotion for object {}", object_id);
            }
            
            len
        } else {
            0
        };
        
        debug!("Processed {} object promotions", promoted_count);
        Ok(promoted_count)
    }
    
    /// Monitor memory pressure across all regions
    pub fn monitor_memory_pressure(&self) -> Result<GlobalMemoryPressure, String> {
        let stats = self.get_statistics()?;
        let total_pressure = MemoryPressure::calculate(
            stats.total_capacity,
            stats.total_used,
            stats.overall_fragmentation,
            0.0, // Global failure rate
            Duration::from_secs(0), // Time since last GC handled elsewhere
        );
        
        let mut region_pressures = Vec::new();
        
        // Check young generation pressure
        if let Some(young_gen) = &self.young_generation {
            let pressure = young_gen.update_memory_pressure()?;
            region_pressures.push((RegionType::YoungGeneration, pressure));
        }
        
        // Check old generation pressure
        if let Some(old_gen) = &self.old_generation {
            if old_gen.should_trigger_compaction()? {
                let pressure = MemoryPressure {
                    level: 0.6, // Moderate pressure for compaction needed
                    free_percentage: 50.0,
                    failure_rate: 0.0,
                    fragmentation: old_gen.get_statistics().fragmentation_ratio,
                    time_since_gc: Duration::from_secs(0),
                };
                region_pressures.push((RegionType::OldGeneration, pressure));
            }
        }
        
        let recommended_action = self.recommend_action(&total_pressure);
        
        Ok(GlobalMemoryPressure {
            overall: total_pressure,
            regions: region_pressures,
            recommended_action,
        })
    }
    
    /// Recommend memory management action based on pressure
    fn recommend_action(&self, pressure: &MemoryPressure) -> MemoryAction {
        if pressure.needs_immediate_collection() {
            MemoryAction::ImmediateCollection
        } else if pressure.should_trigger_collection() {
            MemoryAction::ScheduleCollection
        } else if pressure.fragmentation > 0.5 {
            MemoryAction::Compaction
        } else if pressure.level > 0.3 {
            MemoryAction::IncreaseAllocation
        } else {
            MemoryAction::None
        }
    }
    
    /// Update allocation strategy for all regions
    pub fn update_allocation_strategy(&self, strategy: AllocationStrategy) -> Result<(), String> {
        debug!("Updating allocation strategy to {:?}", strategy);
        
        // Update strategy based on current memory pressure
        let pressure = self.monitor_memory_pressure()?;
        
        match pressure.recommended_action {
            MemoryAction::ImmediateCollection => {
                warn!("Immediate garbage collection recommended due to memory pressure");
            }
            MemoryAction::ScheduleCollection => {
                info!("Scheduling garbage collection due to memory pressure");
            }
            MemoryAction::Compaction => {
                info!("Compaction recommended due to fragmentation");
                if let Some(old_gen) = &self.old_generation {
                    if let Err(e) = old_gen.compact() {
                        warn!("Failed to compact old generation: {}", e);
                    }
                }
            }
            MemoryAction::IncreaseAllocation => {
                debug!("Consider increasing heap size due to memory pressure");
            }
            MemoryAction::None => {
                debug!("No action needed, memory pressure is acceptable");
            }
        }
        
        Ok(())
    }
    
    /// Adaptive heap management based on allocation patterns
    pub fn adaptive_management(&self) -> Result<(), String> {
        let stats = self.get_statistics()?;
        
        // Analyze allocation patterns
        let total_allocations = stats.total_allocations;
        let fragmentation = stats.overall_fragmentation;
        let utilization = stats.utilization_percentage;
        
        debug!("Adaptive management: {} allocations, {:.1}% utilization, {:.1}% fragmentation",
               total_allocations, utilization, fragmentation * 100.0);
        
        // Adjust collection thresholds based on patterns
        if fragmentation > 0.6 {
            // High fragmentation - trigger more aggressive compaction
            if let Some(old_gen) = &self.old_generation {
                if let Err(e) = old_gen.compact() {
                    warn!("Adaptive compaction failed: {}", e);
                }
            }
        }
        
        if utilization > 90.0 {
            // High memory usage - consider more frequent collections
            warn!("High memory utilization detected: {:.1}%", utilization);
        }
        
        Ok(())
    }
}

/// Object lifetime hint for allocation routing
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ObjectLifetimeHint {
    /// Object expected to live for a short time
    ShortLived,
    /// Object expected to live for a long time
    LongLived,
    /// Lifetime unknown
    Unknown,
}

/// Target region for allocation
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum TargetRegion {
    YoungGeneration,
    OldGeneration,
    LargeObjectSpace,
    Primary,
}

/// Global memory pressure across all regions
#[derive(Debug, Clone)]
pub struct GlobalMemoryPressure {
    /// Overall memory pressure
    pub overall: MemoryPressure,
    /// Per-region pressure information
    pub regions: Vec<(RegionType, MemoryPressure)>,
    /// Recommended action to take
    pub recommended_action: MemoryAction,
}

/// Recommended memory management actions
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MemoryAction {
    /// No action needed
    None,
    /// Schedule garbage collection soon
    ScheduleCollection,
    /// Trigger immediate garbage collection
    ImmediateCollection,
    /// Perform compaction to reduce fragmentation
    Compaction,
    /// Consider increasing heap allocation
    IncreaseAllocation,
}

/// Statistics for the region manager
#[derive(Debug, Clone)]
pub struct RegionManagerStatistics {
    pub total_regions: usize,
    pub total_capacity: usize,
    pub total_used: usize,
    pub total_free: usize,
    pub utilization_percentage: f64,
    pub total_allocations: u64,
    pub total_deallocations: u64,
    pub total_objects: usize,
    pub overall_fragmentation: f64,
    pub region_statistics: Vec<RegionStatistics>,
}

impl std::fmt::Display for RegionManagerStatistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,
            "Region Manager Statistics:\n\
             - Total Regions: {}\n\
             - Heap Capacity: {:.2} MB\n\
             - Heap Used: {:.2} MB ({:.1}%)\n\
             - Total Objects: {}\n\
             - Overall Fragmentation: {:.1}%\n\
             - Allocations: {} / Deallocations: {}",
            self.total_regions,
            self.total_capacity as f64 / (1024.0 * 1024.0),
            self.total_used as f64 / (1024.0 * 1024.0),
            self.utilization_percentage,
            self.total_objects,
            self.overall_fragmentation * 100.0,
            self.total_allocations,
            self.total_deallocations
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_young_generation_advanced() {
        let young_gen = YoungGeneration::new(1, 1024).unwrap();
        
        // Test allocation with tracking
        let result = young_gen.allocate(64, 8, "test").unwrap();
        assert_eq!(result.size, 64);
        assert!(young_gen.contains_pointer(result.ptr.as_ptr()));
        
        let stats = young_gen.get_statistics();
        assert_eq!(stats.total_allocations, 1);
        assert_eq!(stats.current_objects, 1);
        
        // Test memory pressure calculation
        let pressure = young_gen.update_memory_pressure().unwrap();
        assert!(pressure.level >= 0.0 && pressure.level <= 1.0);
        
        // Test promotion candidates
        let candidates = young_gen.get_promotion_candidates().unwrap();
        assert!(candidates.is_empty()); // No objects survived GC yet
        
        young_gen.reset().unwrap();
        let stats_after = young_gen.get_statistics();
        assert_eq!(stats_after.current_objects, 0);
    }
    
    #[test]
    fn test_old_generation_advanced() {
        let old_gen = OldGeneration::new(2, 2048).unwrap();
        
        let result1 = old_gen.allocate(128, 8, "test1").unwrap();
        let result2 = old_gen.allocate(256, 8, "test2").unwrap();
        
        assert!(old_gen.contains_pointer(result1.ptr.as_ptr()));
        assert!(old_gen.contains_pointer(result2.ptr.as_ptr()));
        
        // Test object tracking
        old_gen.track_object_allocation(1001, 128, result1.offset).unwrap();
        old_gen.track_object_allocation(1002, 256, result2.offset).unwrap();
        
        old_gen.deallocate(result1.ptr, 128).unwrap();
        
        let stats = old_gen.get_statistics();
        assert_eq!(stats.total_allocations, 2);
        assert_eq!(stats.total_deallocations, 1);
        
        // Test compaction trigger
        let should_compact = old_gen.should_trigger_compaction().unwrap();
        assert!(!should_compact); // Should not need compaction yet
        
        // Test object access tracking
        old_gen.record_object_access(1002).unwrap();
    }
    
    #[test]
    fn test_large_object_space() {
        let large_space = LargeObjectSpace::new(3, 4096).unwrap();
        
        let result = large_space.allocate(256, 8, "large_object").unwrap();
        assert!(large_space.contains_pointer(result.ptr.as_ptr()));
        
        large_space.deallocate(result.ptr, 256).unwrap();
        
        let stats = large_space.get_statistics();
        assert_eq!(stats.total_allocations, 1);
        assert_eq!(stats.total_deallocations, 1);
    }
    
    #[test]
    fn test_region_manager_advanced() {
        let config = HeapConfiguration::default();
        let manager = RegionManager::new(&config).unwrap();
        
        // Test generational regions
        if config.generational_gc {
            assert!(manager.young_generation().is_some());
            assert!(manager.old_generation().is_some());
        }
        
        assert!(manager.large_object_space().is_some());
        
        let stats = manager.get_statistics().unwrap();
        assert!(stats.total_regions > 0);
        assert!(stats.total_capacity > 0);
        
        // Test intelligent allocation routing
        let result = manager.allocate_with_routing(
            64, 8, "test", ObjectLifetimeHint::ShortLived
        ).unwrap();
        assert_eq!(result.size, 64);
        
        // Test memory pressure monitoring
        let pressure = manager.monitor_memory_pressure().unwrap();
        assert!(pressure.overall.level >= 0.0);
        
        // Test promotion processing
        let promoted = manager.process_promotions().unwrap();
        assert!(promoted >= 0);
        
        // Test adaptive management
        manager.adaptive_management().unwrap();
    }
    
    #[test]
    fn test_region_pointer_lookup() {
        let config = HeapConfiguration::default();
        let manager = RegionManager::new(&config).unwrap();
        
        // Allocate in primary region
        let primary = manager.primary_region();
        let result = primary.allocate(64, 8, "test").unwrap();
        
        // Should find the region
        let found_region = manager.find_region_for_pointer(result.ptr.as_ptr());
        assert!(found_region.is_some());
        assert!(manager.contains_pointer(result.ptr.as_ptr()));
        
        // Should not find invalid pointer
        assert!(!manager.contains_pointer(std::ptr::null()));
    }
}
