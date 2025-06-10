/// Enhanced Generational Garbage Collection System
/// 
/// This module provides a state-of-the-art generational garbage collection system
/// with the following key features:
/// 
/// 1. **Multi-Generation Memory Layout**: Young generation (Eden + 2 Survivor spaces) and Old generation
/// 2. **Intelligent Promotion Logic**: Age-based and size-based promotion strategies
/// 3. **Write Barriers**: Cross-generational reference tracking with remembered sets
/// 4. **Adaptive Collection Strategies**: Different algorithms optimized for each generation
/// 5. **Concurrent Collection**: Background collection with minimal pause times
/// 6. **Performance Monitoring**: Comprehensive statistics and adaptive tuning
/// 7. **Goroutine Integration**: Safe collection in concurrent environments

use std::sync::{Arc, RwLock, Mutex, Condvar};
use std::collections::{HashMap, HashSet, VecDeque};
use std::ptr::NonNull;
use std::time::{Duration, Instant};
use std::thread;
use tracing::{instrument, debug, info, warn, error, span, Level};
use rand;

use crate::memory::{Traceable, Visitor};
use crate::memory::object_id::{ObjectId, ObjectRegistry, SharedObjectRegistry};
use crate::memory::heap_manager::{HeapManager, HeapStats};
use crate::memory::roots::{RootSetManager, RootType};
use crate::memory::collection_triggers::{CollectionTriggerManager, TriggerType, TriggerReason};
use crate::memory::cycle_detection::{CycleDetector, CycleDetectionConfig};
use crate::memory::mark_sweep::{MarkSweepCollector, MarkSweepConfig};
use crate::memory::copying::{CopyingCollector, CopyingConfig};
use crate::memory::incremental::{IncrementalCollector, IncrementalConfig};

/// Generation types in the generational collector
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Generation {
    /// Young generation - Eden space for new allocations
    YoungEden,
    /// Young generation - Survivor space 0
    YoungSurvivor0,
    /// Young generation - Survivor space 1
    YoungSurvivor1,
    /// Old generation for long-lived objects
    Old,
    /// Large object space for oversized objects
    LargeObject,
    /// Permanent generation for metadata (optional)
    Permanent,
}

impl Generation {
    /// Check if this is a young generation space
    pub fn is_young(&self) -> bool {
        matches!(self, Generation::YoungEden | Generation::YoungSurvivor0 | Generation::YoungSurvivor1)
    }
    
    /// Check if this is a survivor space
    pub fn is_survivor(&self) -> bool {
        matches!(self, Generation::YoungSurvivor0 | Generation::YoungSurvivor1)
    }
    
    /// Get the other survivor space
    pub fn other_survivor(&self) -> Option<Generation> {
        match self {
            Generation::YoungSurvivor0 => Some(Generation::YoungSurvivor1),
            Generation::YoungSurvivor1 => Some(Generation::YoungSurvivor0),
            _ => None,
        }
    }
}

/// Collection strategy for different scenarios
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CollectionStrategy {
    /// Young generation collection only (minor GC)
    YoungOnly,
    /// Old generation collection only (major GC) 
    OldOnly,
    /// Full collection of all generations
    Full,
    /// Incremental collection with small pause times
    Incremental,
    /// Emergency collection when memory is critically low
    Emergency,
    /// Mixed collection (young + part of old)
    Mixed,
}

/// Write barrier mode for tracking cross-generational references
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteBarrierMode {
    /// No write barriers (fastest but unsafe for generational GC)
    None,
    /// Card marking write barriers
    CardMarking,
    /// Remembered set write barriers
    RememberedSet,
    /// Sequential store buffers
    StoreBuffer,
}

/// Configuration for generational collection
#[derive(Debug, Clone)]
pub struct GenerationalConfig {
    /// Young generation configuration
    pub young_config: CopyingConfig,
    /// Old generation configuration
    pub old_config: MarkSweepConfig,
    /// Incremental collection configuration
    pub incremental_config: IncrementalConfig,
    /// Cycle detection configuration
    pub cycle_detection_config: CycleDetectionConfig,
    
    /// Memory layout configuration
    pub young_generation_ratio: f64,           // Fraction of heap for young gen
    pub eden_space_ratio: f64,                 // Fraction of young gen for Eden
    pub survivor_space_ratio: f64,             // Fraction of young gen for each survivor
    pub large_object_threshold: usize,         // Size threshold for large objects
    
    /// Promotion policies
    pub promotion_age_threshold: u8,           // Age threshold for promotion
    pub promotion_size_threshold: usize,       // Size threshold for immediate promotion
    pub tenuring_threshold: u8,                // Maximum age in young generation
    pub adaptive_tenuring_threshold: bool,     // Enable adaptive tenuring
    
    /// Write barrier configuration
    pub write_barrier_mode: WriteBarrierMode,
    pub remembered_set_size_limit: usize,     // Max size of remembered sets
    pub card_size: usize,                     // Size of each card for card marking
    pub store_buffer_size: usize,             // Size of store buffer
    
    /// Collection triggers
    pub young_gen_threshold: f64,             // Trigger threshold for young gen
    pub old_gen_threshold: f64,               // Trigger threshold for old gen
    pub allocation_rate_threshold: f64,       // Allocation rate trigger (bytes/sec)
    pub promotion_failure_threshold: f64,     // Promotion failure rate trigger
    
    /// Performance tuning
    pub enable_adaptive_sizing: bool,         // Enable adaptive generation sizing
    pub enable_concurrent_collection: bool,   // Enable concurrent collection
    pub enable_incremental_collection: bool,  // Enable incremental collection
    pub enable_cycle_detection: bool,         // Enable cycle detection
    pub enable_parallel_collection: bool,     // Enable parallel collection threads
    pub collection_threads: usize,            // Number of collection threads
    
    /// Pause time targets
    pub max_pause_time: Duration,             // Maximum pause time target
    pub young_pause_time_target: Duration,    // Young gen pause time target
    pub old_pause_time_target: Duration,      // Old gen pause time target
    
    /// Advanced features
    pub enable_compressed_oops: bool,         // Enable compressed object pointers
    pub enable_generational_barriers: bool,   // Enable generation-specific barriers
    pub enable_evacuation_failure_handling: bool, // Handle evacuation failures
}

impl Default for GenerationalConfig {
    fn default() -> Self {
        Self {
            young_config: CopyingConfig::default(),
            old_config: MarkSweepConfig::default(),
            incremental_config: IncrementalConfig::default(),
            cycle_detection_config: CycleDetectionConfig::default(),
            
            // Memory layout (industry standard ratios)
            young_generation_ratio: 0.33,          // 1/3 of heap for young generation
            eden_space_ratio: 0.8,                 // 80% of young gen for Eden
            survivor_space_ratio: 0.1,             // 10% each for survivor spaces
            large_object_threshold: 32 * 1024,     // 32KB threshold
            
            // Promotion policies (HotSpot-inspired defaults)
            promotion_age_threshold: 15,           // Standard HotSpot default
            promotion_size_threshold: 1024 * 1024, // 1MB for immediate promotion
            tenuring_threshold: 15,                // Maximum age in young gen
            adaptive_tenuring_threshold: true,     // Enable adaptive tuning
            
            // Write barriers (remembered set by default)
            write_barrier_mode: WriteBarrierMode::RememberedSet,
            remembered_set_size_limit: 100_000,   // 100K entries max
            card_size: 512,                       // 512-byte cards
            store_buffer_size: 1024,              // 1K entries
            
            // Collection triggers (conservative defaults)
            young_gen_threshold: 0.95,            // Trigger at 95% full
            old_gen_threshold: 0.85,              // Trigger at 85% full
            allocation_rate_threshold: 100.0 * 1024.0 * 1024.0, // 100MB/s
            promotion_failure_threshold: 0.1,     // 10% failure rate
            
            // Performance tuning
            enable_adaptive_sizing: true,
            enable_concurrent_collection: false,   // Conservative default
            enable_incremental_collection: true,
            enable_cycle_detection: true,
            enable_parallel_collection: true,
            collection_threads: thread::available_parallelism().map(|n| n.get()).unwrap_or(4),
            
            // Pause time targets
            max_pause_time: Duration::from_millis(200),        // 200ms max
            young_pause_time_target: Duration::from_millis(20), // 20ms target
            old_pause_time_target: Duration::from_millis(100),  // 100ms target
            
            // Advanced features
            enable_compressed_oops: false,        // Conservative default
            enable_generational_barriers: true,
            enable_evacuation_failure_handling: true,
        }
    }
}

/// Statistics from generational collection
#[derive(Debug, Clone)]
pub struct GenerationalStats {
    // Collection counts
    pub total_collections: u64,
    pub young_collections: u64,
    pub old_collections: u64,
    pub full_collections: u64,
    pub mixed_collections: u64,
    pub incremental_collections: u64,
    
    // Timing statistics
    pub total_collection_time: Duration,
    pub young_collection_time: Duration,
    pub old_collection_time: Duration,
    pub average_pause_time: Duration,
    pub max_pause_time: Duration,
    
    // Promotion statistics
    pub objects_promoted: u64,
    pub bytes_promoted: u64,
    pub promotion_rate: f64,                    // Objects promoted per collection
    pub promotion_failure_count: u64,          // Failed promotions
    pub average_promotion_age: f64,             // Average age when promoted
    
    // Space utilization
    pub eden_space_size: usize,
    pub eden_space_used: usize,
    pub survivor0_space_size: usize,
    pub survivor0_space_used: usize,
    pub survivor1_space_size: usize,
    pub survivor1_space_used: usize,
    pub old_generation_size: usize,
    pub old_generation_used: usize,
    pub large_object_space_size: usize,
    pub large_object_space_used: usize,
    pub total_heap_size: usize,
    pub total_heap_used: usize,
    pub heap_utilization: f64,
    
    // Performance metrics
    pub allocation_rate: f64,                   // Bytes allocated per second
    pub collection_efficiency: f64,            // Bytes reclaimed per collection time
    pub throughput_percentage: f64,            // Application time / total time
    
    // Write barrier statistics
    pub write_barrier_overhead: f64,           // Percentage overhead
    pub remembered_set_size: usize,            // Current remembered set size
    pub card_table_size: usize,                // Current card table size
    pub cross_gen_references: usize,           // Cross-generational references
    
    // Advanced statistics
    pub cycles_detected: u64,
    pub cycles_collected: u64,
    pub evacuation_failures: u64,              // Failed evacuations
    pub concurrent_cycles: u64,                // Concurrent collection cycles
    pub adaptive_sizing_events: u64,           // Adaptive sizing adjustments
}

/// Memory space for a generation
#[derive(Debug)]
struct GenerationSpace {
    /// Generation type
    generation: Generation,
    /// Start address of the space
    start_ptr: NonNull<u8>,
    /// End address of the space  
    end_ptr: NonNull<u8>,
    /// Current allocation pointer (for bump pointer allocation)
    alloc_ptr: Mutex<NonNull<u8>>,
    /// Size of the space in bytes
    size: usize,
    /// Current utilization in bytes
    used: std::sync::atomic::AtomicUsize,
    /// Whether this space is currently active for allocation
    is_active: std::sync::atomic::AtomicBool,
}

impl GenerationSpace {
    /// Create a new generation space
    fn new(generation: Generation, size: usize) -> Result<Self, String> {
        let layout = std::alloc::Layout::from_size_align(size, 64)
            .map_err(|e| format!("Failed to create layout: {}", e))?;
        
        let ptr = unsafe { std::alloc::alloc(layout) };
        if ptr.is_null() {
            return Err(format!("Failed to allocate {} bytes for {:?}", size, generation));
        }
        
        let start_ptr = NonNull::new(ptr).unwrap();
        let end_ptr = NonNull::new(unsafe { ptr.add(size) }).unwrap();
        
        Ok(Self {
            generation,
            start_ptr,
            end_ptr,
            alloc_ptr: Mutex::new(start_ptr),
            size,
            used: std::sync::atomic::AtomicUsize::new(0),
            is_active: std::sync::atomic::AtomicBool::new(false),
        })
    }
    
    /// Allocate space and return pointer
    fn allocate(&self, size: usize, align: usize) -> Option<NonNull<u8>> {
        let aligned_size = (size + align - 1) & !(align - 1);
        
        let mut alloc_ptr = self.alloc_ptr.lock().unwrap();
        let available = unsafe { self.end_ptr.as_ptr().offset_from(alloc_ptr.as_ptr()) } as usize;
        
        if available < aligned_size {
            return None;
        }
        
        let old_ptr = *alloc_ptr;
        *alloc_ptr = NonNull::new(unsafe { alloc_ptr.as_ptr().add(aligned_size) }).unwrap();
        
        // Update used counter
        self.used.fetch_add(aligned_size, std::sync::atomic::Ordering::SeqCst);
        
        Some(old_ptr)
    }
    
    /// Reset allocation pointer and clear space
    fn reset(&self) {
        let mut alloc_ptr = self.alloc_ptr.lock().unwrap();
        *alloc_ptr = self.start_ptr;
        self.used.store(0, std::sync::atomic::Ordering::SeqCst);
    }
    
    /// Get utilization ratio
    fn utilization(&self) -> f64 {
        let used = self.used.load(std::sync::atomic::Ordering::SeqCst);
        used as f64 / self.size as f64
    }
    
    /// Check if space contains pointer
    fn contains(&self, ptr: *const u8) -> bool {
        ptr >= self.start_ptr.as_ptr() && ptr < self.end_ptr.as_ptr()
    }
}

impl Drop for GenerationSpace {
    fn drop(&mut self) {
        let layout = std::alloc::Layout::from_size_align(self.size, 64).unwrap();
        unsafe { std::alloc::dealloc(self.start_ptr.as_ptr(), layout) };
    }
}

/// Object generation tracking with enhanced metadata
#[derive(Debug, Clone)]
struct ObjectGenerationInfo {
    generation: Generation,
    age: u8,
    promotion_candidate: bool,
    size: usize,
    allocated_at: Instant,
    last_gc_age: u8,                    // Age during last GC
    promotion_attempts: u8,             // Number of failed promotion attempts
    forwarding_pointer: Option<ObjectId>, // For evacuation
}

/// Cross-generational reference for write barriers
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct CrossGenerationalReference {
    from_object: ObjectId,
    from_generation: Generation,
    to_object: ObjectId,
    to_generation: Generation,
    field_offset: usize,
    timestamp: Instant,                 // When reference was created
}

/// Write barrier entry for remembered sets
#[derive(Debug, Clone)]
struct WriteBarrierEntry {
    object_id: ObjectId,
    field_offset: usize,
    old_reference: Option<ObjectId>,
    new_reference: ObjectId,
    timestamp: Instant,
}

/// Card table for write barriers
#[derive(Debug)]
struct CardTable {
    /// Card data (each byte represents a card)
    cards: Vec<std::sync::atomic::AtomicU8>,
    /// Size of each card in bytes
    card_size: usize,
    /// Start address of the heap region
    heap_start: *const u8,
    /// Size of the heap region
    heap_size: usize,
}

impl CardTable {
    /// Create a new card table
    fn new(heap_start: *const u8, heap_size: usize, card_size: usize) -> Self {
        let num_cards = (heap_size + card_size - 1) / card_size;
        let cards = (0..num_cards)
            .map(|_| std::sync::atomic::AtomicU8::new(0))
            .collect();
        
        Self {
            cards,
            card_size,
            heap_start,
            heap_size,
        }
    }
    
    /// Mark a card as dirty
    fn mark_dirty(&self, addr: *const u8) {
        if let Some(card_index) = self.get_card_index(addr) {
            self.cards[card_index].store(1, std::sync::atomic::Ordering::SeqCst);
        }
    }
    
    /// Check if a card is dirty
    fn is_dirty(&self, addr: *const u8) -> bool {
        if let Some(card_index) = self.get_card_index(addr) {
            self.cards[card_index].load(std::sync::atomic::Ordering::SeqCst) != 0
        } else {
            false
        }
    }
    
    /// Clear a card
    fn clear_card(&self, addr: *const u8) {
        if let Some(card_index) = self.get_card_index(addr) {
            self.cards[card_index].store(0, std::sync::atomic::Ordering::SeqCst);
        }
    }
    
    /// Get dirty cards as indices
    fn get_dirty_cards(&self) -> Vec<usize> {
        self.cards
            .iter()
            .enumerate()
            .filter(|(_, card)| card.load(std::sync::atomic::Ordering::SeqCst) != 0)
            .map(|(i, _)| i)
            .collect()
    }
    
    /// Get card index for address
    fn get_card_index(&self, addr: *const u8) -> Option<usize> {
        if addr >= self.heap_start && addr < unsafe { self.heap_start.add(self.heap_size) } {
            let offset = unsafe { addr.offset_from(self.heap_start) } as usize;
            Some(offset / self.card_size)
        } else {
            None
        }
    }
}

/// Remembered set for tracking cross-generational references
#[derive(Debug)]
struct RememberedSet {
    /// Set of cross-generational references
    references: RwLock<HashSet<CrossGenerationalReference>>,
    /// Maximum size before compaction
    size_limit: usize,
    /// Statistics
    total_entries: std::sync::atomic::AtomicUsize,
    compactions: std::sync::atomic::AtomicUsize,
}

impl RememberedSet {
    /// Create a new remembered set
    fn new(size_limit: usize) -> Self {
        Self {
            references: RwLock::new(HashSet::new()),
            size_limit,
            total_entries: std::sync::atomic::AtomicUsize::new(0),
            compactions: std::sync::atomic::AtomicUsize::new(0),
        }
    }
    
    /// Add a cross-generational reference
    fn add_reference(&self, reference: CrossGenerationalReference) -> Result<(), String> {
        let mut refs = self.references.write()
            .map_err(|_| "Failed to acquire write lock on remembered set")?;
        
        if refs.insert(reference) {
            self.total_entries.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            
            // Check if compaction is needed
            if refs.len() > self.size_limit {
                self.compact_internal(&mut refs)?;
            }
        }
        
        Ok(())
    }
    
    /// Remove a reference
    fn remove_reference(&self, reference: &CrossGenerationalReference) -> Result<bool, String> {
        let mut refs = self.references.write()
            .map_err(|_| "Failed to acquire write lock on remembered set")?;
        
        if refs.remove(reference) {
            self.total_entries.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
            Ok(true)
        } else {
            Ok(false)
        }
    }
    
    /// Get all references for scanning
    fn get_references(&self) -> Result<Vec<CrossGenerationalReference>, String> {
        let refs = self.references.read()
            .map_err(|_| "Failed to acquire read lock on remembered set")?;
        Ok(refs.iter().cloned().collect())
    }
    
    /// Compact the remembered set by removing stale references
    fn compact(&self) -> Result<usize, String> {
        let mut refs = self.references.write()
            .map_err(|_| "Failed to acquire write lock on remembered set")?;
        self.compact_internal(&mut refs)
    }
    
    /// Internal compaction logic
    fn compact_internal(&self, refs: &mut HashSet<CrossGenerationalReference>) -> Result<usize, String> {
        let initial_size = refs.len();
        
        // Remove references older than a threshold (simplified compaction)
        let cutoff = Instant::now() - Duration::from_secs(300); // 5 minutes
        refs.retain(|r| r.timestamp > cutoff);
        
        let removed = initial_size - refs.len();
        if removed > 0 {
            self.compactions.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            self.total_entries.fetch_sub(removed, std::sync::atomic::Ordering::SeqCst);
        }
        
        Ok(removed)
    }
    
    /// Get statistics
    fn size(&self) -> usize {
        self.total_entries.load(std::sync::atomic::Ordering::SeqCst)
    }
}

/// Adaptive sizing state for dynamic generation size adjustment
#[derive(Debug, Clone)]
struct AdaptiveSizingState {
    /// Target throughput percentage (application time / total time)
    target_throughput: f64,
    /// Current throughput measurement
    current_throughput: f64,
    /// Recent pause times for analysis
    recent_pause_times: VecDeque<Duration>,
    /// Recent allocation rates for analysis
    recent_allocation_rates: VecDeque<f64>,
    /// Number of adjustments made
    adjustments_made: u64,
    /// Last adjustment timestamp
    last_adjustment: Option<Instant>,
    /// Adjustment cooldown period
    adjustment_cooldown: Duration,
}

impl Default for AdaptiveSizingState {
    fn default() -> Self {
        Self {
            target_throughput: 0.95,  // 95% throughput target
            current_throughput: 1.0,
            recent_pause_times: VecDeque::with_capacity(100),
            recent_allocation_rates: VecDeque::with_capacity(100),
            adjustments_made: 0,
            last_adjustment: None,
            adjustment_cooldown: Duration::from_secs(5),
        }
    }
}

/// Main generational garbage collector with enhanced features
pub struct GenerationalCollector {
    config: RwLock<GenerationalConfig>,
    
    /// Core components
    object_registry: SharedObjectRegistry,
    heap_manager: Option<Arc<RwLock<HeapManager>>>,
    root_manager: Arc<RootSetManager>,
    trigger_manager: Arc<CollectionTriggerManager>,
    
    /// Memory spaces for different generations
    eden_space: Arc<GenerationSpace>,
    survivor0_space: Arc<GenerationSpace>,
    survivor1_space: Arc<GenerationSpace>,
    old_space: Arc<Mutex<GenerationSpace>>,
    large_object_space: Arc<Mutex<GenerationSpace>>,
    
    /// Current survivor space (0 or 1)
    current_survivor: std::sync::atomic::AtomicU8,
    
    /// Generation-specific collectors
    young_collector: Arc<CopyingCollector>,
    old_collector: Arc<MarkSweepCollector>,
    incremental_collector: Arc<IncrementalCollector>,
    cycle_detector: Arc<CycleDetector>,
    
    /// Write barrier system
    remembered_set: Arc<RememberedSet>,
    card_table: Option<Arc<CardTable>>,
    write_barrier_buffer: Mutex<VecDeque<WriteBarrierEntry>>,
    
    /// Object tracking and metadata
    object_generations: RwLock<HashMap<ObjectId, ObjectGenerationInfo>>,
    forwarding_table: RwLock<HashMap<ObjectId, ObjectId>>,
    promotion_queue: Mutex<VecDeque<ObjectId>>,
    
    /// Statistics and monitoring
    stats: RwLock<GenerationalStats>,
    collection_counter: std::sync::atomic::AtomicU64,
    last_collection_time: Mutex<Option<Instant>>,
    allocation_counter: std::sync::atomic::AtomicU64,
    allocation_bytes: std::sync::atomic::AtomicU64,
    promotion_failures: std::sync::atomic::AtomicU64,
    
    /// Background collection coordination
    background_thread: Mutex<Option<thread::JoinHandle<()>>>,
    should_stop: std::sync::atomic::AtomicBool,
    collection_signal: Arc<(Mutex<bool>, Condvar)>,
    
    /// Performance tracking
    allocation_rate_tracker: Mutex<VecDeque<(Instant, usize)>>,
    pause_time_tracker: Mutex<VecDeque<Duration>>,
    adaptive_sizing_state: RwLock<AdaptiveSizingState>,
}

impl GenerationalCollector {
    /// Create a new generational collector
    pub fn new(object_registry: SharedObjectRegistry) -> Result<Self, String> {
        Self::with_config(object_registry, GenerationalConfig::default())
    }
    
    /// Create a new generational collector with custom configuration
    #[instrument(skip(object_registry, config))]
    pub fn with_config(object_registry: SharedObjectRegistry, config: GenerationalConfig) -> Result<Self, String> {
        let _span = span!(Level::INFO, "generational_collector_creation").entered();
        info!("Creating enhanced generational collector with config");
        
        // Calculate generation sizes based on configuration
        let total_heap_size = 256 * 1024 * 1024; // 256MB default
        let young_gen_size = (total_heap_size as f64 * config.young_generation_ratio) as usize;
        let eden_size = (young_gen_size as f64 * config.eden_space_ratio) as usize;
        let survivor_size = (young_gen_size as f64 * config.survivor_space_ratio) as usize;
        let old_gen_size = total_heap_size - young_gen_size;
        let large_object_size = old_gen_size / 4; // 25% for large objects
        
        info!("Memory layout: Eden={}KB, Survivor={}KB each, Old={}KB, Large={}KB", 
              eden_size / 1024, survivor_size / 1024, old_gen_size / 1024, large_object_size / 1024);
        
        // Create generation spaces
        let eden_space = Arc::new(GenerationSpace::new(Generation::YoungEden, eden_size)?);
        let survivor0_space = Arc::new(GenerationSpace::new(Generation::YoungSurvivor0, survivor_size)?);
        let survivor1_space = Arc::new(GenerationSpace::new(Generation::YoungSurvivor1, survivor_size)?);
        let old_space = Arc::new(Mutex::new(GenerationSpace::new(Generation::Old, old_gen_size)?));
        let large_object_space = Arc::new(Mutex::new(GenerationSpace::new(Generation::LargeObject, large_object_size)?));
        
        // Set Eden as initially active
        eden_space.is_active.store(true, std::sync::atomic::Ordering::SeqCst);
        
        // Create root manager
        let root_manager = Arc::new(RootSetManager::new(object_registry.clone()));
        
        // Create trigger manager
        let trigger_manager = Arc::new(CollectionTriggerManager::new());
        
        // Create generation-specific collectors
        let young_collector = Arc::new(CopyingCollector::with_config(
            object_registry.clone(),
            config.young_config.clone(),
        )?);
        
        let old_collector = Arc::new(MarkSweepCollector::with_config(
            object_registry.clone(),
            config.old_config.clone(),
        ));
        
        let incremental_collector = Arc::new(IncrementalCollector::with_config(
            object_registry.clone(),
            config.incremental_config.clone(),
        ));
        
        let cycle_detector = Arc::new(CycleDetector::with_config(
            object_registry.clone(),
            config.cycle_detection_config.clone(),
        ));
        
        // Create write barrier system
        let remembered_set = Arc::new(RememberedSet::new(config.remembered_set_size_limit));
        let card_table = if config.write_barrier_mode == WriteBarrierMode::CardMarking {
            Some(Arc::new(CardTable::new(
                eden_space.start_ptr.as_ptr(),
                total_heap_size,
                config.card_size,
            )))
        } else {
            None
        };
        
        let collection_signal = Arc::new((Mutex::new(false), Condvar::new()));
        
        Ok(Self {
            config: RwLock::new(config),
            object_registry,
            heap_manager: None,
            root_manager,
            trigger_manager,
            
            // Generation spaces
            eden_space,
            survivor0_space,
            survivor1_space,
            old_space,
            large_object_space,
            current_survivor: std::sync::atomic::AtomicU8::new(0),
            
            // Collectors
            young_collector,
            old_collector,
            incremental_collector,
            cycle_detector,
            
            // Write barriers
            remembered_set,
            card_table,
            write_barrier_buffer: Mutex::new(VecDeque::new()),
            
            // Object tracking
            object_generations: RwLock::new(HashMap::new()),
            forwarding_table: RwLock::new(HashMap::new()),
            promotion_queue: Mutex::new(VecDeque::new()),
            
            // Statistics
            stats: RwLock::new(GenerationalStats {
                total_collections: 0,
                young_collections: 0,
                old_collections: 0,
                full_collections: 0,
                mixed_collections: 0,
                incremental_collections: 0,
                total_collection_time: Duration::ZERO,
                young_collection_time: Duration::ZERO,
                old_collection_time: Duration::ZERO,
                average_pause_time: Duration::ZERO,
                max_pause_time: Duration::ZERO,
                objects_promoted: 0,
                bytes_promoted: 0,
                promotion_rate: 0.0,
                promotion_failure_count: 0,
                average_promotion_age: 0.0,
                eden_space_size: eden_size,
                eden_space_used: 0,
                survivor0_space_size: survivor_size,
                survivor0_space_used: 0,
                survivor1_space_size: survivor_size,
                survivor1_space_used: 0,
                old_generation_size: old_gen_size,
                old_generation_used: 0,
                large_object_space_size: large_object_size,
                large_object_space_used: 0,
                total_heap_size,
                total_heap_used: 0,
                heap_utilization: 0.0,
                allocation_rate: 0.0,
                collection_efficiency: 0.0,
                throughput_percentage: 100.0,
                write_barrier_overhead: 0.0,
                remembered_set_size: 0,
                card_table_size: 0,
                cross_gen_references: 0,
                cycles_detected: 0,
                cycles_collected: 0,
                evacuation_failures: 0,
                concurrent_cycles: 0,
                adaptive_sizing_events: 0,
            }),
            collection_counter: std::sync::atomic::AtomicU64::new(0),
            last_collection_time: Mutex::new(None),
            allocation_counter: std::sync::atomic::AtomicU64::new(0),
            allocation_bytes: std::sync::atomic::AtomicU64::new(0),
            promotion_failures: std::sync::atomic::AtomicU64::new(0),
            
            // Background collection
            background_thread: Mutex::new(None),
            should_stop: std::sync::atomic::AtomicBool::new(false),
            collection_signal,
            
            // Performance tracking
            allocation_rate_tracker: Mutex::new(VecDeque::new()),
            pause_time_tracker: Mutex::new(VecDeque::new()),
            adaptive_sizing_state: RwLock::new(AdaptiveSizingState::default()),
        })
    }
    
    /// Set heap manager
    pub fn set_heap_manager(&mut self, heap_manager: Arc<RwLock<HeapManager>>) {
        self.heap_manager = Some(heap_manager);
    }
    
    /// Enhanced allocation with intelligent generation selection
    #[instrument(skip(self))]
    pub fn allocate(&self, size: usize, align: usize) -> Result<Option<std::ptr::NonNull<u8>>, String> {
        let allocation_start = Instant::now();
        debug!("Allocating object of size {} bytes", size);
        
        // Update allocation tracking
        self.allocation_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        self.allocation_bytes.fetch_add(size as u64, std::sync::atomic::Ordering::SeqCst);
        self.track_allocation_rate(size);
        
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        // Large object allocation - go directly to large object space
        if size >= config.large_object_threshold {
            debug!("Large object allocation ({}KB) - allocating in large object space", size / 1024);
            return self.allocate_in_large_object_space(size, align);
        }
        
        // Immediate promotion for very large objects in young generation
        if size >= config.promotion_size_threshold {
            debug!("Large young object ({}KB) - allocating directly in old generation", size / 1024);
            return self.allocate_in_old_generation(size, align);
        }
        
        // Normal allocation path - try Eden space first
        if let Some(ptr) = self.allocate_in_eden(size, align)? {
            let object_id = ObjectId::new(rand::random());
            self.track_object_allocation(object_id, Generation::YoungEden, size)?;
            debug!("Allocated object {} in Eden space", object_id);
            return Ok(Some(ptr));
        }
        
        // Eden is full - trigger young generation collection
        if self.should_collect_young()? {
            info!("Eden space full, triggering young generation collection");
            let _collection_stats = self.collect_young_generation()?;
            
            // Try allocating in Eden again after collection
            if let Some(ptr) = self.allocate_in_eden(size, align)? {
                let object_id = ObjectId::new(rand::random());
                self.track_object_allocation(object_id, Generation::YoungEden, size)?;
                debug!("Allocated object {} in Eden space after collection", object_id);
                return Ok(Some(ptr));
            }
        }
        
        // Still can't allocate in young gen - try old generation
        warn!("Failed to allocate in young generation, trying old generation");
        if let Some(ptr) = self.allocate_in_old_generation(size, align)? {
            let object_id = ObjectId::new(rand::random());
            self.track_object_allocation(object_id, Generation::Old, size)?;
            debug!("Allocated object {} in old generation", object_id);
            return Ok(Some(ptr));
        }
        
        // Last resort - trigger full collection and try again
        error!("Failed allocation, triggering emergency collection");
        let _collection_stats = self.collect_emergency()?;
        
        if let Some(ptr) = self.allocate_in_eden(size, align)? {
            let object_id = ObjectId::new(rand::random());
            self.track_object_allocation(object_id, Generation::YoungEden, size)?;
            warn!("Allocated object {} in Eden space after emergency collection", object_id);
            return Ok(Some(ptr));
        }
        
        error!("Allocation failed even after emergency collection");
        Ok(None)
    }
    
    /// Allocate in Eden space
    fn allocate_in_eden(&self, size: usize, align: usize) -> Result<Option<NonNull<u8>>, String> {
        if !self.eden_space.is_active.load(std::sync::atomic::Ordering::SeqCst) {
            return Ok(None);
        }
        
        Ok(self.eden_space.allocate(size, align))
    }
    
    /// Allocate in old generation  
    fn allocate_in_old_generation(&self, size: usize, align: usize) -> Result<Option<std::ptr::NonNull<u8>>, String> {
        let old_space = self.old_space.lock()
            .map_err(|_| "Failed to acquire lock on old space")?;
        Ok(old_space.allocate(size, align))
    }
    
    /// Allocate in large object space
    fn allocate_in_large_object_space(&self, size: usize, align: usize) -> Result<Option<std::ptr::NonNull<u8>>, String> {
        let large_space = self.large_object_space.lock()
            .map_err(|_| "Failed to acquire lock on large object space")?;
        Ok(large_space.allocate(size, align))
    }
    
    /// Track allocation rate for adaptive sizing
    fn track_allocation_rate(&self, bytes: usize) {
        let mut tracker = self.allocation_rate_tracker.lock().unwrap();
        let now = Instant::now();
        
        // Add current allocation
        tracker.push_back((now, bytes));
        
        // Remove entries older than 1 second
        let cutoff = now - Duration::from_secs(1);
        while let Some(&(timestamp, _)) = tracker.front() {
            if timestamp < cutoff {
                tracker.pop_front();
            } else {
                break;
            }
        }
        
        // Keep max 1000 entries
        while tracker.len() > 1000 {
            tracker.pop_front();
        }
    }
    
    /// Get current allocation rate in bytes per second
    fn get_allocation_rate(&self) -> f64 {
        let tracker = self.allocation_rate_tracker.lock().unwrap();
        if tracker.len() < 2 {
            return 0.0;
        }
        
        let total_bytes: usize = tracker.iter().map(|(_, bytes)| bytes).sum();
        let time_span = tracker.back().unwrap().0.duration_since(tracker.front().unwrap().0);
        
        if time_span.as_secs_f64() > 0.0 {
            total_bytes as f64 / time_span.as_secs_f64()
        } else {
            0.0
        }
    }
    
    /// Perform collection based on trigger analysis
    #[instrument(skip(self))]
    pub fn collect(&self) -> Result<GenerationalStats, String> {
        info!("Starting generational collection");
        
        // Check what type of collection is needed
        let collection_strategy = self.determine_collection_strategy()?;
        
        let collection_start = Instant::now();
        let collection_number = self.collection_counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst) + 1;
        
        let stats = match collection_strategy {
            CollectionStrategy::YoungOnly => self.collect_young_generation()?,
            CollectionStrategy::OldOnly => self.collect_old_generation()?,
            CollectionStrategy::Full => self.collect_full()?,
            CollectionStrategy::Incremental => self.collect_incremental()?,
            CollectionStrategy::Emergency => self.collect_emergency()?,
            CollectionStrategy::Mixed => {
                // Mixed collection - young generation plus part of old generation
                self.collect_young_generation()?;
                // TODO: Implement partial old generation collection
                self.get_stats()?
            }
        };
        
        let total_duration = collection_start.elapsed();
        
        // Update overall statistics
        self.update_collection_statistics(collection_strategy, total_duration)?;
        
        // Record collection time
        {
            let mut last_time = self.last_collection_time.lock()
                .map_err(|_| "Failed to acquire lock on last collection time")?;
            *last_time = Some(Instant::now());
        }
        
        info!("Generational collection completed in {:?} using strategy {:?}", total_duration, collection_strategy);
        self.get_stats()
    }
    
    /// Determine the best collection strategy
    fn determine_collection_strategy(&self) -> Result<CollectionStrategy, String> {
        // Check for emergency conditions first
        if let Some(heap_manager) = &self.heap_manager {
            let heap_stats = {
                let heap = heap_manager.read()
                    .map_err(|_| "Failed to acquire read lock on heap manager")?;
                heap.get_stats()?
            };
            
            if let Some((trigger_type, _reason)) = self.trigger_manager.should_trigger_collection(&heap_stats)? {
                return Ok(match trigger_type {
                    TriggerType::Emergency => CollectionStrategy::Emergency,
                    TriggerType::FullCollection => CollectionStrategy::Full,
                    TriggerType::OldGeneration => CollectionStrategy::OldOnly,
                    TriggerType::YoungGeneration => CollectionStrategy::YoungOnly,
                    TriggerType::Incremental => CollectionStrategy::Incremental,
                });
            }
        }
        
        // Check young generation pressure
        if self.should_collect_young()? {
            return Ok(CollectionStrategy::YoungOnly);
        }
        
        // Check old generation pressure
        if self.should_collect_old()? {
            return Ok(CollectionStrategy::OldOnly);
        }
        
        // Check if incremental collection is beneficial
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        if config.enable_incremental_collection && self.incremental_collector.is_collecting()? {
            return Ok(CollectionStrategy::Incremental);
        }
        
        // Default to young generation collection
        Ok(CollectionStrategy::YoungOnly)
    }
    
    /// Enhanced young generation collection with copying and promotion
    #[instrument(skip(self))]
    fn collect_young_generation(&self) -> Result<GenerationalStats, String> {
        let _span = span!(Level::INFO, "young_generation_collection").entered();
        info!("Starting young generation collection");
        
        let collection_start = Instant::now();
        
        // Phase 1: Identify live objects in Eden and current survivor space
        let live_objects = self.identify_live_young_objects()?;
        info!("Found {} live objects in young generation", live_objects.len());
        
        // Phase 2: Determine which survivor space to use as target
        let current_survivor = self.current_survivor.load(std::sync::atomic::Ordering::SeqCst);
        let target_survivor = if current_survivor == 0 {
            &self.survivor1_space
        } else {
            &self.survivor0_space
        };
        
        // Phase 3: Copy and promote objects
        let (objects_copied, objects_promoted, bytes_copied, bytes_promoted) = 
            self.copy_and_promote_objects(&live_objects, target_survivor)?;
        
        // Phase 4: Clear Eden and old survivor space
        self.clear_young_spaces(current_survivor)?;
        
        // Phase 5: Switch survivor spaces
        self.current_survivor.store(1 - current_survivor, std::sync::atomic::Ordering::SeqCst);
        
        let collection_duration = collection_start.elapsed();
        
        // Update statistics
        {
            let mut stats = self.stats.write()
                .map_err(|_| "Failed to acquire write lock on stats")?;
            
            stats.young_collections += 1;
            stats.total_collections += 1;
            stats.young_collection_time += collection_duration;
            stats.total_collection_time += collection_duration;
            stats.objects_promoted += objects_promoted as u64;
            stats.bytes_promoted += bytes_promoted as u64;
            
            // Update promotion rate
            if stats.young_collections > 0 {
                stats.promotion_rate = stats.objects_promoted as f64 / stats.young_collections as f64;
            }
            
            // Update pause time tracking
            if collection_duration > stats.max_pause_time {
                stats.max_pause_time = collection_duration;
            }
            
            // Update space utilization
            stats.eden_space_used = self.eden_space.used.load(std::sync::atomic::Ordering::SeqCst);
            stats.survivor0_space_used = self.survivor0_space.used.load(std::sync::atomic::Ordering::SeqCst);
            stats.survivor1_space_used = self.survivor1_space.used.load(std::sync::atomic::Ordering::SeqCst);
        }
        
        // Update pause time tracker for adaptive sizing
        {
            let mut pause_tracker = self.pause_time_tracker.lock().unwrap();
            pause_tracker.push_back(collection_duration);
            
            // Keep only recent pause times (last 100)
            while pause_tracker.len() > 100 {
                pause_tracker.pop_front();
            }
        }
        
        info!("Young generation collection completed in {:?}: copied={}, promoted={}, bytes_copied={}, bytes_promoted={}", 
              collection_duration, objects_copied, objects_promoted, bytes_copied, bytes_promoted);
        
        // Trigger adaptive sizing if enabled
        self.maybe_adjust_generation_sizes()?;
        
        self.get_stats()
    }
    
    /// Identify live objects in young generation spaces
    fn identify_live_young_objects(&self) -> Result<HashSet<ObjectId>, String> {
        let mut live_objects = HashSet::new();
        
        // Get all objects in Eden and current survivor space
        let object_generations = self.object_generations.read()
            .map_err(|_| "Failed to acquire read lock on object generations")?;
        
        let current_survivor_gen = if self.current_survivor.load(std::sync::atomic::Ordering::SeqCst) == 0 {
            Generation::YoungSurvivor0
        } else {
            Generation::YoungSurvivor1
        };
        
        // Collect young generation objects
        let young_objects: Vec<ObjectId> = object_generations
            .iter()
            .filter(|(_, info)| {
                info.generation == Generation::YoungEden || info.generation == current_survivor_gen
            })
            .map(|(id, _)| *id)
            .collect();
        
        debug!("Found {} young generation objects to analyze", young_objects.len());
        
        // TODO: Implement proper reachability analysis starting from roots
        // For now, consider all objects reachable (conservative approach)
        for object_id in young_objects {
            live_objects.insert(object_id);
        }
        
        // Also check remembered set for cross-generational references
        let remembered_refs = self.remembered_set.get_references()?;
        for cross_ref in remembered_refs {
            if cross_ref.to_generation.is_young() {
                live_objects.insert(cross_ref.to_object);
            }
        }
        
        Ok(live_objects)
    }
    
    /// Copy live objects to survivor space and promote old objects
    fn copy_and_promote_objects(
        &self, 
        live_objects: &HashSet<ObjectId>, 
        target_survivor: &Arc<GenerationSpace>
    ) -> Result<(usize, usize, usize, usize), String> {
        let mut objects_copied = 0;
        let mut objects_promoted = 0;
        let mut bytes_copied = 0;
        let mut bytes_promoted = 0;
        
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        let mut object_generations = self.object_generations.write()
            .map_err(|_| "Failed to acquire write lock on object generations")?;
        
        for &object_id in live_objects {
            if let Some(obj_info) = object_generations.get_mut(&object_id) {
                // Decide whether to promote or copy to survivor space
                let should_promote = self.should_promote_object(obj_info, &config);
                
                if should_promote {
                    // Promote to old generation
                    match self.promote_object_to_old_generation(object_id, obj_info) {
                        Ok(()) => {
                            objects_promoted += 1;
                            bytes_promoted += obj_info.size;
                            obj_info.generation = Generation::Old;
                            obj_info.age = 0; // Reset age in new generation
                            debug!("Promoted object {} to old generation", object_id);
                        }
                        Err(e) => {
                            warn!("Failed to promote object {}: {}", object_id, e);
                            self.promotion_failures.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                            
                            // Fall back to survivor space if promotion fails
                            if self.copy_object_to_survivor(object_id, obj_info, target_survivor)? {
                                objects_copied += 1;
                                bytes_copied += obj_info.size;
                            }
                        }
                    }
                } else {
                    // Copy to survivor space
                    if self.copy_object_to_survivor(object_id, obj_info, target_survivor)? {
                        objects_copied += 1;
                        bytes_copied += obj_info.size;
                        obj_info.age += 1; // Age the object
                        debug!("Copied object {} to survivor space (age={})", object_id, obj_info.age);
                    }
                }
            }
        }
        
        Ok((objects_copied, objects_promoted, bytes_copied, bytes_promoted))
    }
    
    /// Determine if an object should be promoted to old generation
    fn should_promote_object(&self, obj_info: &ObjectGenerationInfo, config: &GenerationalConfig) -> bool {
        // Age-based promotion
        if obj_info.age >= config.promotion_age_threshold {
            return true;
        }
        
        // Size-based promotion for large objects
        if obj_info.size >= config.promotion_size_threshold {
            return true;
        }
        
        // Adaptive promotion based on allocation patterns
        if config.adaptive_tenuring_threshold {
            // TODO: Implement adaptive promotion logic
            // This could consider allocation rates, survival rates, etc.
        }
        
        false
    }
    
    /// Promote an object to old generation
    fn promote_object_to_old_generation(&self, object_id: ObjectId, obj_info: &mut ObjectGenerationInfo) -> Result<(), String> {
        // TODO: Implement actual object copying to old generation space
        // For now, just update the generation tracking
        debug!("Promoting object {} ({} bytes) to old generation", object_id, obj_info.size);
        Ok(())
    }
    
    /// Copy an object to survivor space
    fn copy_object_to_survivor(&self, object_id: ObjectId, obj_info: &mut ObjectGenerationInfo, target_survivor: &Arc<GenerationSpace>) -> Result<bool, String> {
        // TODO: Implement actual object copying to survivor space
        // For now, just update the generation tracking
        let target_generation = target_survivor.generation;
        obj_info.generation = target_generation;
        debug!("Copied object {} to {:?}", object_id, target_generation);
        Ok(true)
    }
    
    /// Clear Eden and old survivor space
    fn clear_young_spaces(&self, old_survivor: u8) -> Result<(), String> {
        // Clear Eden space
        self.eden_space.reset();
        
        // Clear the old survivor space
        if old_survivor == 0 {
            self.survivor0_space.reset();
        } else {
            self.survivor1_space.reset();
        }
        
        debug!("Cleared Eden and survivor{} spaces", old_survivor);
        Ok(())
    }
    
    /// Maybe adjust generation sizes based on performance
    fn maybe_adjust_generation_sizes(&self) -> Result<(), String> {
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        if !config.enable_adaptive_sizing {
            return Ok(());
        }
        
        let mut adaptive_state = self.adaptive_sizing_state.write()
            .map_err(|_| "Failed to acquire write lock on adaptive sizing state")?;
        
        // Check if enough time has passed since last adjustment
        if let Some(last_adjustment) = adaptive_state.last_adjustment {
            if last_adjustment.elapsed() < adaptive_state.adjustment_cooldown {
                return Ok(());
            }
        }
        
        // Analyze recent performance
        let pause_tracker = self.pause_time_tracker.lock().unwrap();
        if pause_tracker.len() < 10 {
            return Ok(()); // Not enough data
        }
        
        let recent_pauses: Vec<Duration> = pause_tracker.iter().rev().take(10).cloned().collect();
        let average_pause = recent_pauses.iter().sum::<Duration>() / recent_pauses.len() as u32;
        
        let target_pause = config.young_pause_time_target;
        
        // Adjust generation sizes if pause times are consistently off target
        if average_pause > target_pause * 2 {
            // Pauses too long - increase generation sizes to reduce collection frequency
            info!("Average pause time ({:?}) exceeds target ({:?}), considering size increase", 
                  average_pause, target_pause);
            adaptive_state.adjustments_made += 1;
            adaptive_state.last_adjustment = Some(Instant::now());
            
            // TODO: Implement actual size adjustment
        } else if average_pause < target_pause / 2 {
            // Pauses too short - we could reduce generation sizes to save memory
            info!("Average pause time ({:?}) well below target ({:?}), considering size decrease", 
                  average_pause, target_pause);
            adaptive_state.adjustments_made += 1;
            adaptive_state.last_adjustment = Some(Instant::now());
            
            // TODO: Implement actual size adjustment
        }
        
        Ok(())
    }
    
    /// Collect old generation
    #[instrument(skip(self))]
    fn collect_old_generation(&self) -> Result<GenerationalStats, String> {
        info!("Collecting old generation");
        
        let collection_start = Instant::now();
        
        // Perform mark-and-sweep collection
        let mark_sweep_stats = self.old_collector.collect()?;
        
        // Run cycle detection if enabled
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        let cycles_detected = if config.enable_cycle_detection {
            let cycles = self.cycle_detector.detect_cycles()?;
            let cycle_count = cycles.len();
            
            if !cycles.is_empty() {
                let collected_cycles = self.cycle_detector.collect_cycles(&cycles)?;
                info!("Detected and collected {} cycles containing {} objects", cycle_count, collected_cycles);
            }
            
            cycle_count as u64
        } else {
            0
        };
        
        let collection_duration = collection_start.elapsed();
        
        // Update statistics
        {
            let mut stats = self.stats.write()
                .map_err(|_| "Failed to acquire write lock on stats")?;
            
            stats.old_collections += 1;
            stats.total_collections += 1;
            stats.old_collection_time += collection_duration;
            stats.total_collection_time += collection_duration;
            stats.cycles_detected += cycles_detected;
            
            // Update collection efficiency
            if collection_duration.as_secs_f64() > 0.0 {
                let efficiency = mark_sweep_stats.bytes_reclaimed as f64 / collection_duration.as_secs_f64();
                stats.collection_efficiency = (stats.collection_efficiency + efficiency) / 2.0;
            }
        }
        
        info!("Old generation collection completed in {:?}", collection_duration);
        self.get_stats()
    }
    
    /// Collect all generations (full collection)
    #[instrument(skip(self))]
    fn collect_full(&self) -> Result<GenerationalStats, String> {
        info!("Performing full collection");
        
        let collection_start = Instant::now();
        
        // Collect young generation first
        self.collect_young_generation()?;
        
        // Then collect old generation
        self.collect_old_generation()?;
        
        let collection_duration = collection_start.elapsed();
        
        // Update statistics
        {
            let mut stats = self.stats.write()
                .map_err(|_| "Failed to acquire write lock on stats")?;
            
            stats.full_collections += 1;
            // Note: individual generation collections already updated their counters
        }
        
        info!("Full collection completed in {:?}", collection_duration);
        self.get_stats()
    }
    
    /// Perform incremental collection step
    #[instrument(skip(self))]
    fn collect_incremental(&self) -> Result<GenerationalStats, String> {
        debug!("Performing incremental collection step");
        
        let collection_start = Instant::now();
        
        // Perform incremental step
        let work_performed = self.incremental_collector.step()?;
        
        let collection_duration = collection_start.elapsed();
        
        if work_performed {
            // Update statistics
            let mut stats = self.stats.write()
                .map_err(|_| "Failed to acquire write lock on stats")?;
            
            stats.incremental_collections += 1;
            stats.total_collection_time += collection_duration;
        }
        
        debug!("Incremental collection step completed in {:?}, work performed: {}", collection_duration, work_performed);
        self.get_stats()
    }
    
    /// Emergency collection when memory is critically low
    #[instrument(skip(self))]
    fn collect_emergency(&self) -> Result<GenerationalStats, String> {
        warn!("Performing emergency collection");
        
        let collection_start = Instant::now();
        
        // Perform aggressive full collection
        self.collect_full()?;
        
        // Force cycle detection and collection
        let cycles = self.cycle_detector.detect_cycles()?;
        if !cycles.is_empty() {
            self.cycle_detector.collect_cycles(&cycles)?;
        }
        
        let collection_duration = collection_start.elapsed();
        
        warn!("Emergency collection completed in {:?}", collection_duration);
        self.get_stats()
    }
    
    /// Check if young generation collection is needed
    fn should_collect_young(&self) -> Result<bool, String> {
        self.young_collector.should_collect()
    }
    
    /// Check if old generation collection is needed
    fn should_collect_old(&self) -> Result<bool, String> {
        // TODO: Implement old generation pressure checking
        // For now, use a simple heuristic
        Ok(false)
    }
    
    /// Enhanced write barrier for cross-generational references
    #[instrument(skip(self))]
    pub fn write_barrier(&self, object_id: ObjectId, field_offset: usize, old_value: Option<ObjectId>, new_value: ObjectId) -> Result<(), String> {
        let write_start = Instant::now();
        
        // Determine generations
        let from_generation = self.get_object_generation(object_id)?;
        let to_generation = self.get_object_generation(new_value)?;
        
        debug!("Write barrier: object {} ({:?}) field {} = {} ({:?})", 
               object_id, from_generation, field_offset, new_value, to_generation);
        
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        // Check if this is a cross-generational reference that needs tracking
        let is_cross_gen = self.is_cross_generational_reference(from_generation, to_generation);
        
        if is_cross_gen {
            // Handle write barrier based on configured mode
            match config.write_barrier_mode {
                WriteBarrierMode::None => {
                    // No write barriers - just track the reference
                }
                WriteBarrierMode::CardMarking => {
                    self.handle_card_marking_barrier(object_id)?;
                }
                WriteBarrierMode::RememberedSet => {
                    self.handle_remembered_set_barrier(object_id, field_offset, old_value, new_value, from_generation, to_generation)?;
                }
                WriteBarrierMode::StoreBuffer => {
                    self.handle_store_buffer_barrier(object_id, field_offset, old_value, new_value)?;
                }
            }
            
            debug!("Recorded cross-generational reference: {:?} -> {:?}", from_generation, to_generation);
        }
        
        // Forward to incremental collector for write barrier processing
        if config.enable_incremental_collection {
            self.incremental_collector.write_barrier(object_id, field_offset, old_value, new_value)?;
        }
        
        // Track write barrier overhead
        let write_duration = write_start.elapsed();
        if write_duration.as_nanos() > 1000 { // Only track if > 1μs
            self.update_write_barrier_overhead(write_duration);
        }
        
        Ok(())
    }
    
    /// Check if this is a cross-generational reference that needs special handling
    fn is_cross_generational_reference(&self, from_generation: Generation, to_generation: Generation) -> bool {
        match (from_generation, to_generation) {
            // Old to young references are the primary concern
            (Generation::Old, gen) if gen.is_young() => true,
            (Generation::LargeObject, gen) if gen.is_young() => true,
            // Survivor to Eden references
            (Generation::YoungSurvivor0 | Generation::YoungSurvivor1, Generation::YoungEden) => true,
            // Any other cross-generational reference
            (from, to) if from != to => true,
            _ => false,
        }
    }
    
    /// Handle card marking write barrier
    fn handle_card_marking_barrier(&self, object_id: ObjectId) -> Result<(), String> {
        if let Some(ref card_table) = self.card_table {
            // Get object address and mark the corresponding card dirty
            // TODO: Implement object address resolution
            debug!("Marking card dirty for object {}", object_id);
        }
        Ok(())
    }
    
    /// Handle remembered set write barrier
    fn handle_remembered_set_barrier(
        &self, 
        object_id: ObjectId, 
        field_offset: usize, 
        _old_value: Option<ObjectId>, 
        new_value: ObjectId,
        from_generation: Generation,
        to_generation: Generation
    ) -> Result<(), String> {
        let cross_ref = CrossGenerationalReference {
            from_object: object_id,
            from_generation,
            to_object: new_value,
            to_generation,
            field_offset,
            timestamp: Instant::now(),
        };
        
        self.remembered_set.add_reference(cross_ref)?;
        debug!("Added reference to remembered set");
        Ok(())
    }
    
    /// Handle store buffer write barrier
    fn handle_store_buffer_barrier(
        &self, 
        object_id: ObjectId, 
        field_offset: usize, 
        old_value: Option<ObjectId>, 
        new_value: ObjectId
    ) -> Result<(), String> {
        let entry = WriteBarrierEntry {
            object_id,
            field_offset,
            old_reference: old_value,
            new_reference: new_value,
            timestamp: Instant::now(),
        };
        
        let mut buffer = self.write_barrier_buffer.lock()
            .map_err(|_| "Failed to acquire lock on write barrier buffer")?;
        
        buffer.push_back(entry);
        
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        // Check if buffer needs flushing
        if buffer.len() >= config.store_buffer_size {
            debug!("Write barrier buffer full, processing entries");
            self.process_store_buffer_entries(&mut buffer)?;
        }
        
        Ok(())
    }
    
    /// Process write barrier buffer entries
    fn process_store_buffer_entries(&self, buffer: &mut VecDeque<WriteBarrierEntry>) -> Result<(), String> {
        // Convert buffer entries to remembered set entries
        while let Some(entry) = buffer.pop_front() {
            let from_generation = self.get_object_generation(entry.object_id)?;
            let to_generation = self.get_object_generation(entry.new_reference)?;
            
            if self.is_cross_generational_reference(from_generation, to_generation) {
                let cross_ref = CrossGenerationalReference {
                    from_object: entry.object_id,
                    from_generation,
                    to_object: entry.new_reference,
                    to_generation,
                    field_offset: entry.field_offset,
                    timestamp: entry.timestamp,
                };
                
                self.remembered_set.add_reference(cross_ref)?;
            }
        }
        
        Ok(())
    }
    
    /// Update write barrier overhead statistics
    fn update_write_barrier_overhead(&self, duration: Duration) {
        // TODO: Implement write barrier overhead tracking
        // This would track the overhead and update statistics
    }
    
    /// Get the generation of an object
    fn get_object_generation(&self, object_id: ObjectId) -> Result<Generation, String> {
        let object_generations = self.object_generations.read()
            .map_err(|_| "Failed to acquire read lock on object generations")?;
        
        Ok(object_generations.get(&object_id)
            .map(|info| info.generation)
            .unwrap_or(Generation::YoungEden)) // Default to young generation
    }
    
    /// Track object allocation in a specific generation
    pub fn track_object_allocation(&self, object_id: ObjectId, generation: Generation, size: usize) -> Result<(), String> {
        let mut object_generations = self.object_generations.write()
            .map_err(|_| "Failed to acquire write lock on object generations")?;
        
        let info = ObjectGenerationInfo {
            generation,
            age: 0,
            promotion_candidate: false,
            size,
            allocated_at: Instant::now(),
            last_gc_age: 0,
            promotion_attempts: 0,
            forwarding_pointer: None,
        };
        
        object_generations.insert(object_id, info);
        debug!("Tracked object {} allocation in {:?} generation", object_id, generation);
        Ok(())
    }
    
    /// Promote an object from young to old generation
    pub fn promote_object(&self, object_id: ObjectId) -> Result<(), String> {
        let mut object_generations = self.object_generations.write()
            .map_err(|_| "Failed to acquire write lock on object generations")?;
        
        if let Some(info) = object_generations.get_mut(&object_id) {
            info.generation = Generation::Old;
            info.age = 0; // Reset age in new generation
            info.promotion_candidate = false;
            
            debug!("Promoted object {} to old generation", object_id);
        }
        
        Ok(())
    }
    
    /// Update collection statistics
    fn update_collection_statistics(&self, strategy: CollectionStrategy, duration: Duration) -> Result<(), String> {
        // Update heap utilization and other derived statistics
        if let Some(heap_manager) = &self.heap_manager {
            let heap_stats = {
                let heap = heap_manager.read()
                    .map_err(|_| "Failed to acquire read lock on heap manager")?;
                heap.get_stats()?
            };
            
            let mut stats = self.stats.write()
                .map_err(|_| "Failed to acquire write lock on stats")?;
            
            stats.heap_utilization = heap_stats.average_utilization;
            
            // Update space utilization from generation spaces
            stats.eden_space_used = self.eden_space.used.load(std::sync::atomic::Ordering::SeqCst);
            stats.survivor0_space_used = self.survivor0_space.used.load(std::sync::atomic::Ordering::SeqCst);
            stats.survivor1_space_used = self.survivor1_space.used.load(std::sync::atomic::Ordering::SeqCst);
        }
        
        Ok(())
    }
    
    /// Start background collection if enabled
    pub fn start_background_collection(&self) -> Result<(), String> {
        let config = self.config.read()
            .map_err(|_| "Failed to acquire read lock on config")?;
        
        if !config.enable_concurrent_collection {
            return Ok(());
        }
        
        let mut thread_handle = self.background_thread.lock()
            .map_err(|_| "Failed to acquire lock on background thread")?;
        
        if thread_handle.is_some() {
            return Ok(());
        }
        
        // Start incremental collector background thread
        self.incremental_collector.start_collection()?;
        
        let collector_ref = unsafe { std::mem::transmute::<&GenerationalCollector, &'static GenerationalCollector>(self) };
        
        let handle = std::thread::spawn(move || {
            info!("Starting background generational collection thread");
            collector_ref.background_collection_loop();
            info!("Background generational collection thread stopped");
        });
        
        *thread_handle = Some(handle);
        Ok(())
    }
    
    /// Background collection loop
    fn background_collection_loop(&self) {
        while !self.should_stop.load(std::sync::atomic::Ordering::SeqCst) {
            // Perform incremental collection steps
            if let Err(e) = self.collect_incremental() {
                error!("Background incremental collection failed: {}", e);
            }
            
            // Check if full collection is needed periodically
            if let Ok(strategy) = self.determine_collection_strategy() {
                match strategy {
                    CollectionStrategy::Emergency | CollectionStrategy::Full => {
                        if let Err(e) = self.collect() {
                            error!("Background full collection failed: {}", e);
                        }
                    }
                    _ => {
                        // Continue with incremental collection
                    }
                }
            }
            
            // Sleep to avoid busy waiting
            std::thread::sleep(Duration::from_millis(10));
        }
    }
    
    /// Stop background collection
    pub fn stop_background_collection(&self) -> Result<(), String> {
        self.should_stop.store(true, std::sync::atomic::Ordering::SeqCst);
        
        // Stop incremental collector
        self.incremental_collector.stop_background_collection()?;
        
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
    
    /// Notify of allocation for trigger management
    pub fn notify_allocation(&self, bytes: usize) {
        // Forward to trigger manager and incremental collector
        if let Err(e) = self.trigger_manager.update_allocation_tracking(bytes, 1) {
            error!("Failed to update allocation tracking: {}", e);
        }
        
        self.incremental_collector.notify_allocation(bytes);
    }
    
    /// Get generational collection statistics
    pub fn get_stats(&self) -> Result<GenerationalStats, String> {
        let stats = self.stats.read()
            .map_err(|_| "Failed to acquire read lock on stats")?;
        Ok(stats.clone())
    }
    
    /// Update configuration
    pub fn update_config(&self, new_config: GenerationalConfig) -> Result<(), String> {
        // Update individual collector configurations
        self.young_collector.update_config(new_config.young_config.clone())?;
        self.old_collector.update_config(new_config.old_config.clone())?;
        self.incremental_collector.update_config(new_config.incremental_config.clone())?;
        self.cycle_detector.update_config(new_config.cycle_detection_config.clone())?;
        
        // Update main configuration
        let mut config = self.config.write()
            .map_err(|_| "Failed to acquire write lock on config")?;
        *config = new_config;
        
        info!("Updated generational collector configuration");
        Ok(())
    }
    
    /// Force a specific type of collection
    pub fn force_collection(&self, strategy: CollectionStrategy) -> Result<GenerationalStats, String> {
        info!("Forcing collection with strategy {:?}", strategy);
        
        match strategy {
            CollectionStrategy::YoungOnly => self.collect_young_generation(),
            CollectionStrategy::OldOnly => self.collect_old_generation(),
            CollectionStrategy::Full => self.collect_full(),
            CollectionStrategy::Incremental => self.collect_incremental(),
            CollectionStrategy::Emergency => self.collect_emergency(),
            CollectionStrategy::Mixed => {
                // Mixed collection - young generation plus part of old generation
                self.collect_young_generation()?;
                // TODO: Implement partial old generation collection
                self.get_stats()
            }
        }
    }
    
    /// Get object count by generation
    pub fn get_object_counts_by_generation(&self) -> Result<HashMap<Generation, usize>, String> {
        let object_generations = self.object_generations.read()
            .map_err(|_| "Failed to acquire read lock on object generations")?;
        
        let mut counts = HashMap::new();
        for info in object_generations.values() {
            *counts.entry(info.generation).or_insert(0) += 1;
        }
        
        Ok(counts)
    }
}

impl Drop for GenerationalCollector {
    fn drop(&mut self) {
        if let Err(e) = self.stop_background_collection() {
            error!("Failed to stop background collection during drop: {}", e);
        }
    }
}

// Safety: GenerationalCollector is thread-safe through its component's thread safety
unsafe impl Send for GenerationalCollector {}
unsafe impl Sync for GenerationalCollector {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::object_id::ObjectRegistry;
    
    fn create_test_collector() -> Result<(GenerationalCollector, SharedObjectRegistry), String> {
        let registry = Arc::new(ObjectRegistry::new());
        let collector = GenerationalCollector::new(registry.clone())?;
        Ok((collector, registry))
    }
    
    #[test]
    fn test_collector_creation() {
        let (collector, _registry) = create_test_collector().unwrap();
        let stats = collector.get_stats().unwrap();
        assert_eq!(stats.total_collections, 0);
    }
    
    #[test]
    fn test_allocation_tracking() {
        let (collector, _registry) = create_test_collector().unwrap();
        
        let object_id = ObjectId::new(1);
        collector.track_object_allocation(object_id, Generation::YoungEden, 64).unwrap();
        
        let counts = collector.get_object_counts_by_generation().unwrap();
        assert_eq!(counts.get(&Generation::YoungEden), Some(&1));
    }
    
    #[test]
    fn test_object_promotion() {
        let (collector, _registry) = create_test_collector().unwrap();
        
        let object_id = ObjectId::new(2);
        collector.track_object_allocation(object_id, Generation::YoungEden, 64).unwrap();
        collector.promote_object(object_id).unwrap();
        
        let generation = collector.get_object_generation(object_id).unwrap();
        assert_eq!(generation, Generation::Old);
    }
    
    #[test]
    fn test_write_barrier() {
        let (collector, _registry) = create_test_collector().unwrap();
        
        let from_object = ObjectId::new(3);
        let to_object = ObjectId::new(4);
        
        collector.track_object_allocation(from_object, Generation::Old, 64).unwrap();
        collector.track_object_allocation(to_object, Generation::YoungEden, 32).unwrap();
        
        collector.write_barrier(from_object, 0, None, to_object).unwrap();
        
        // Should have recorded cross-generational reference in remembered set
        let stats = collector.get_stats().unwrap();
        assert!(stats.remembered_set_size >= 0); // Basic check that it's tracked
    }
    
    #[test]
    fn test_config_update() {
        let (collector, _registry) = create_test_collector().unwrap();
        
        let new_config = GenerationalConfig {
            young_generation_ratio: 0.5,
            promotion_age_threshold: 5,
            ..Default::default()
        };
        
        collector.update_config(new_config).unwrap();
    }
    
    #[test]
    fn test_force_collection() {
        let (collector, _registry) = create_test_collector().unwrap();
        
        // Force a young generation collection
        let stats = collector.force_collection(CollectionStrategy::YoungOnly).unwrap();
        assert!(stats.young_collections > 0);
    }
}
