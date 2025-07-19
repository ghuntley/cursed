/// Heap Allocation Optimizer
/// 
/// Provides advanced heap allocation strategies and optimizations
/// for production workloads with minimal fragmentation and high performance.

use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::collections::{HashMap, BTreeMap, VecDeque};
use std::time::{Duration, Instant};
use std::ptr::NonNull;
use std::alloc::{Layout, alloc, dealloc};

use crate::runtime::gc::{GarbageCollector, HeapObject, ObjectMetadata, HeapRegion};
use crate::runtime::memory::{MemoryManager, ObjectHandle};
use crate::runtime::memory_profiler::MemoryProfiler;
use crate::error::CursedError;
use crate::memory::Tag;

/// Heap allocation strategy
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum AllocationStrategy {
    /// First-fit allocation
    FirstFit,
    /// Best-fit allocation
    BestFit,
    /// Worst-fit allocation
    WorstFit,
    /// Next-fit allocation
    NextFit,
    /// Buddy allocation
    Buddy,
    /// Slab allocation
    Slab,
    /// Thread-local allocation
    ThreadLocal,
    /// Size-class allocation
    SizeClass,
}

/// Heap optimizer configuration
#[derive(Debug, Clone)]
pub struct HeapOptimizerConfig {
    /// Primary allocation strategy
    pub allocation_strategy: AllocationStrategy,
    /// Enable thread-local allocation buffers
    pub thread_local_buffers: bool,
    /// Thread-local buffer size
    pub tlab_size: usize,
    /// Enable size-class allocation
    pub size_class_allocation: bool,
    /// Large object threshold
    pub large_object_threshold: usize,
    /// Enable prefaulting
    pub prefault_memory: bool,
    /// Alignment requirement
    pub alignment: usize,
    /// Enable memory pool reuse
    pub pool_reuse: bool,
    /// Pool sizes for different object sizes
    pub pool_sizes: Vec<usize>,
    /// Enable allocation fast path
    pub fast_path: bool,
    /// Enable allocation statistics
    pub track_statistics: bool,
}

impl Default for HeapOptimizerConfig {
    fn default() -> Self {
        Self {
            allocation_strategy: AllocationStrategy::SizeClass,
            thread_local_buffers: true,
            tlab_size: 1024 * 1024, // 1MB TLAB
            size_class_allocation: true,
            large_object_threshold: 256 * 1024, // 256KB
            prefault_memory: true,
            alignment: 8, // 8-byte alignment
            pool_reuse: true,
            pool_sizes: vec![32, 64, 128, 256, 512, 1024, 2048, 4096, 8192],
            fast_path: true,
            track_statistics: true,
        }
    }
}

/// Size class for optimized allocation
#[derive(Debug, Clone)]
pub struct SizeClass {
    /// Size class index
    pub index: usize,
    /// Object size for this class
    pub size: usize,
    /// Number of objects per chunk
    pub objects_per_chunk: usize,
    /// Free list for this size class
    pub free_list: VecDeque<NonNull<u8>>,
    /// Allocated chunks
    pub chunks: Vec<NonNull<u8>>,
    /// Statistics
    pub allocated_objects: usize,
    pub freed_objects: usize,
    pub total_chunks: usize,
}

/// Thread-local allocation buffer
#[derive(Debug)]
pub struct ThreadLocalBuffer {
    /// Buffer start address
    pub start: NonNull<u8>,
    /// Buffer end address
    pub end: NonNull<u8>,
    /// Current allocation pointer
    pub current: NonNull<u8>,
    /// Buffer size
    pub size: usize,
    /// Remaining space
    pub remaining: usize,
    /// Buffer generation
    pub generation: usize,
}

/// Heap allocation statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct HeapStats {
    /// Total allocations
    pub total_allocations: u64,
    /// Total deallocations
    pub total_deallocations: u64,
    /// Bytes allocated
    pub bytes_allocated: u64,
    /// Bytes deallocated
    pub bytes_deallocated: u64,
    /// Current heap usage
    pub current_heap_usage: usize,
    /// Peak heap usage
    pub peak_heap_usage: usize,
    /// Allocation rate (bytes/second)
    pub allocation_rate: f64,
    /// Deallocation rate (bytes/second)
    pub deallocation_rate: f64,
    /// Fragmentation percentage
    pub fragmentation: f64,
    /// Average allocation size
    pub avg_allocation_size: usize,
    /// Large object allocations
    pub large_object_allocations: u64,
    /// Thread-local buffer utilization
    pub tlab_utilization: f64,
    /// Size class statistics
    pub size_class_stats: HashMap<usize, SizeClassStats>,
}

/// Statistics for a size class
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SizeClassStats {
    /// Total allocations
    pub allocations: u64,
    /// Total deallocations
    pub deallocations: u64,
    /// Current utilization
    pub utilization: f64,
    /// Waste percentage
    pub waste: f64,
}

/// Heap optimizer
pub struct HeapOptimizer {
    /// Configuration
    config: HeapOptimizerConfig,
    /// Size classes
    size_classes: RwLock<Vec<SizeClass>>,
    /// Thread-local buffers (stored as addresses for thread safety)
    thread_local_buffers: RwLock<HashMap<std::thread::ThreadId, ThreadLocalBuffer>>,
    /// Large object allocations
    large_objects: RwLock<HashMap<usize, LargeObjectInfo>>,
    /// Allocation statistics
    stats: RwLock<HeapStats>,
    /// Memory pools
    memory_pools: RwLock<HashMap<usize, MemoryPool>>,
    /// Garbage collector reference
    gc_ref: Option<Arc<GarbageCollector>>,
    /// Memory manager reference
    memory_manager: Option<Arc<MemoryManager>>,
    /// Profiler reference
    profiler: Option<Arc<MemoryProfiler>>,
    /// Running flag
    running: AtomicBool,
    /// Next allocation pointer (for NextFit)
    next_fit_ptr: AtomicUsize,
}

/// Large object information
#[derive(Debug, Clone)]
pub struct LargeObjectInfo {
    /// Object address
    pub address: usize,
    /// Object size
    pub size: usize,
    /// Allocation timestamp
    pub allocated_at: Instant,
    /// Object tag
    pub tag: Tag,
}

/// Memory pool for reusing allocations
#[derive(Debug)]
pub struct MemoryPool {
    /// Pool object size
    pub object_size: usize,
    /// Available objects
    pub available: VecDeque<NonNull<u8>>,
    /// Total pool size
    pub total_size: usize,
    /// Used objects
    pub used_objects: usize,
    /// Pool chunks
    pub chunks: Vec<NonNull<u8>>,
}

impl HeapOptimizer {
    /// Create new heap optimizer
    pub fn new(config: HeapOptimizerConfig) -> Result<Self, CursedError> {
        let mut size_classes = Vec::new();
        
        // Initialize size classes
        if config.size_class_allocation {
            for (index, &size) in config.pool_sizes.iter().enumerate() {
                let objects_per_chunk = std::cmp::max(1, 4096 / size);
                size_classes.push(SizeClass {
                    index,
                    size,
                    objects_per_chunk,
                    free_list: VecDeque::new(),
                    chunks: Vec::new(),
                    allocated_objects: 0,
                    freed_objects: 0,
                    total_chunks: 0,
                });
            }
        }
        
        let stats = HeapStats {
            total_allocations: 0,
            total_deallocations: 0,
            bytes_allocated: 0,
            bytes_deallocated: 0,
            current_heap_usage: 0,
            peak_heap_usage: 0,
            allocation_rate: 0.0,
            deallocation_rate: 0.0,
            fragmentation: 0.0,
            avg_allocation_size: 0,
            large_object_allocations: 0,
            tlab_utilization: 0.0,
            size_class_stats: HashMap::new(),
        };
        
        Ok(Self {
            config,
            size_classes: RwLock::new(size_classes),
            thread_local_buffers: RwLock::new(HashMap::new()),
            large_objects: RwLock::new(HashMap::new()),
            stats: RwLock::new(stats),
            memory_pools: RwLock::new(HashMap::new()),
            gc_ref: None,
            memory_manager: None,
            profiler: None,
            running: AtomicBool::new(false),
            next_fit_ptr: AtomicUsize::new(0),
        })
    }

    /// Start heap optimizer
    pub fn start(&self) -> Result<(), CursedError> {
        if self.running.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed).is_err() {
            return Err(CursedError::runtime_error("Heap optimizer already running"));
        }

        // Initialize memory pools
        self.initialize_memory_pools()?;

        // Initialize size classes
        self.initialize_size_classes()?;

        Ok(())
    }

    /// Stop heap optimizer
    pub fn stop(&self) -> Result<(), CursedError> {
        self.running.store(false, Ordering::Relaxed);
        
        // Cleanup memory pools
        self.cleanup_memory_pools()?;
        
        // Cleanup size classes
        self.cleanup_size_classes()?;
        
        Ok(())
    }

    /// Allocate object with optimized strategy
    pub fn allocate(&self, size: usize, alignment: usize, tag: Tag) -> Result<NonNull<u8>, CursedError> {
        let aligned_size = self.align_size(size, alignment);
        
        // Update statistics
        if self.config.track_statistics {
            self.update_allocation_stats(aligned_size);
        }

        // Choose allocation strategy
        let result = if aligned_size >= self.config.large_object_threshold {
            // Large object allocation
            self.allocate_large_object(aligned_size, tag)
        } else if self.config.thread_local_buffers {
            // Try thread-local allocation first
            self.allocate_from_tlab(aligned_size, tag)
                .or_else(|_| self.allocate_with_strategy(aligned_size, tag))
        } else {
            // Use configured strategy
            self.allocate_with_strategy(aligned_size, tag)
        };

        // Record allocation in profiler
        if let (Some(profiler), Ok(ptr)) = (&self.profiler, &result) {
            profiler.record_allocation(ptr.as_ptr() as usize, aligned_size, tag, None)?;
        }

        result
    }

    /// Deallocate object
    pub fn deallocate(&self, ptr: NonNull<u8>, size: usize) -> Result<(), CursedError> {
        // Update statistics
        if self.config.track_statistics {
            self.update_deallocation_stats(size);
        }

        // Record deallocation in profiler
        if let Some(profiler) = &self.profiler {
            profiler.record_deallocation(ptr.as_ptr() as usize)?;
        }

        // Handle deallocation based on size
        if size >= self.config.large_object_threshold {
            self.deallocate_large_object(ptr, size)
        } else if self.config.size_class_allocation {
            self.deallocate_to_size_class(ptr, size)
        } else {
            self.deallocate_direct(ptr, size)
        }
    }

    /// Allocate from thread-local allocation buffer
    fn allocate_from_tlab(&self, size: usize, tag: Tag) -> Result<NonNull<u8>, CursedError> {
        let thread_id = std::thread::current().id();
        let mut tlabs = self.thread_local_buffers.write().unwrap();
        
        // Get or create TLAB for this thread
        let tlab = tlabs.entry(thread_id).or_insert_with(|| {
            self.create_tlab().unwrap_or_else(|_| {
                // Fallback to empty TLAB
                ThreadLocalBuffer {
                    start: NonNull::dangling(),
                    end: NonNull::dangling(),
                    current: NonNull::dangling(),
                    size: 0,
                    remaining: 0,
                    generation: 0,
                }
            })
        });

        // Try to allocate from TLAB
        if tlab.remaining >= size {
            let ptr = tlab.current;
            tlab.current = unsafe { NonNull::new_unchecked(tlab.current.as_ptr().add(size)) };
            tlab.remaining -= size;
            Ok(ptr)
        } else {
            // TLAB is full, need to allocate new one or fallback
            *tlab = self.create_tlab()?;
            if tlab.remaining >= size {
                let ptr = tlab.current;
                tlab.current = unsafe { NonNull::new_unchecked(tlab.current.as_ptr().add(size)) };
                tlab.remaining -= size;
                Ok(ptr)
            } else {
                Err(CursedError::runtime_error("Object too large for TLAB"))
            }
        }
    }

    /// Allocate with configured strategy
    fn allocate_with_strategy(&self, size: usize, tag: Tag) -> Result<NonNull<u8>, CursedError> {
        match self.config.allocation_strategy {
            AllocationStrategy::FirstFit => self.allocate_first_fit(size),
            AllocationStrategy::BestFit => self.allocate_best_fit(size),
            AllocationStrategy::WorstFit => self.allocate_worst_fit(size),
            AllocationStrategy::NextFit => self.allocate_next_fit(size),
            AllocationStrategy::Buddy => self.allocate_buddy(size),
            AllocationStrategy::Slab => self.allocate_slab(size),
            AllocationStrategy::ThreadLocal => self.allocate_from_tlab(size, tag),
            AllocationStrategy::SizeClass => self.allocate_size_class(size),
        }
    }

    /// Allocate using size classes
    fn allocate_size_class(&self, size: usize) -> Result<NonNull<u8>, CursedError> {
        let size_class_index = self.find_size_class(size);
        let mut size_classes = self.size_classes.write().unwrap();
        
        if let Some(size_class) = size_classes.get_mut(size_class_index) {
            // Try to allocate from free list
            if let Some(ptr) = size_class.free_list.pop_front() {
                size_class.allocated_objects += 1;
                return Ok(ptr);
            }
            
            // Need to allocate new chunk
            let chunk_size = size_class.size * size_class.objects_per_chunk;
            let layout = Layout::from_size_align(chunk_size, self.config.alignment)
                .map_err(|e| CursedError::runtime_error(&format!("Layout error: {}", e)))?;
            
            let chunk_ptr = unsafe { alloc(layout) };
            if chunk_ptr.is_null() {
                return Err(CursedError::runtime_error("Failed to allocate chunk"));
            }
            
            let chunk = unsafe { NonNull::new_unchecked(chunk_ptr) };
            size_class.chunks.push(chunk);
            size_class.total_chunks += 1;
            
            // Add objects to free list
            for i in 1..size_class.objects_per_chunk {
                let obj_ptr = unsafe { chunk_ptr.add(i * size_class.size) };
                size_class.free_list.push_back(unsafe { NonNull::new_unchecked(obj_ptr) });
            }
            
            // Return first object
            size_class.allocated_objects += 1;
            Ok(chunk)
        } else {
            Err(CursedError::runtime_error("Size class not found"))
        }
    }

    /// Allocate large object
    fn allocate_large_object(&self, size: usize, tag: Tag) -> Result<NonNull<u8>, CursedError> {
        let layout = Layout::from_size_align(size, self.config.alignment)
            .map_err(|e| CursedError::runtime_error(&format!("Layout error: {}", e)))?;
        
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return Err(CursedError::runtime_error("Failed to allocate large object"));
        }

        let obj_ptr = unsafe { NonNull::new_unchecked(ptr) };
        
        // Record large object
        let mut large_objects = self.large_objects.write().unwrap();
        large_objects.insert(obj_ptr.as_ptr() as usize, LargeObjectInfo {
            address: obj_ptr.as_ptr() as usize,
            size,
            allocated_at: Instant::now(),
            tag,
        });
        
        // Update statistics
        let mut stats = self.stats.write().unwrap();
        stats.large_object_allocations += 1;
        
        Ok(obj_ptr)
    }

    /// Deallocate to size class
    fn deallocate_to_size_class(&self, ptr: NonNull<u8>, size: usize) -> Result<(), CursedError> {
        let size_class_index = self.find_size_class(size);
        let mut size_classes = self.size_classes.write().unwrap();
        
        if let Some(size_class) = size_classes.get_mut(size_class_index) {
            size_class.free_list.push_back(ptr);
            size_class.freed_objects += 1;
        }
        
        Ok(())
    }

    /// Deallocate large object
    fn deallocate_large_object(&self, ptr: NonNull<u8>, size: usize) -> Result<(), CursedError> {
        let mut large_objects = self.large_objects.write().unwrap();
        large_objects.remove(&(ptr.as_ptr() as usize));
        
        let layout = Layout::from_size_align(size, self.config.alignment)
            .map_err(|e| CursedError::runtime_error(&format!("Layout error: {}", e)))?;
        
        unsafe { dealloc(ptr.as_ptr(), layout) };
        
        Ok(())
    }

    /// Deallocate directly
    fn deallocate_direct(&self, ptr: NonNull<u8>, size: usize) -> Result<(), CursedError> {
        let layout = Layout::from_size_align(size, self.config.alignment)
            .map_err(|e| CursedError::runtime_error(&format!("Layout error: {}", e)))?;
        
        unsafe { dealloc(ptr.as_ptr(), layout) };
        
        Ok(())
    }

    /// Create thread-local allocation buffer
    fn create_tlab(&self) -> Result<ThreadLocalBuffer, CursedError> {
        let layout = Layout::from_size_align(self.config.tlab_size, self.config.alignment)
            .map_err(|e| CursedError::runtime_error(&format!("Layout error: {}", e)))?;
        
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return Err(CursedError::runtime_error("Failed to allocate TLAB"));
        }
        
        let start = unsafe { NonNull::new_unchecked(ptr) };
        let end = unsafe { NonNull::new_unchecked(ptr.add(self.config.tlab_size)) };
        
        Ok(ThreadLocalBuffer {
            start,
            end,
            current: start,
            size: self.config.tlab_size,
            remaining: self.config.tlab_size,
            generation: 0,
        })
    }

    /// Find appropriate size class for given size
    fn find_size_class(&self, size: usize) -> usize {
        for (index, &pool_size) in self.config.pool_sizes.iter().enumerate() {
            if size <= pool_size {
                return index;
            }
        }
        self.config.pool_sizes.len() - 1 // Use largest size class
    }

    /// Align size to configured alignment
    fn align_size(&self, size: usize, alignment: usize) -> usize {
        let align = std::cmp::max(self.config.alignment, alignment);
        (size + align - 1) & !(align - 1)
    }

    /// Update allocation statistics
    fn update_allocation_stats(&self, size: usize) {
        let mut stats = self.stats.write().unwrap();
        stats.total_allocations += 1;
        stats.bytes_allocated += size as u64;
        stats.current_heap_usage += size;
        
        if stats.current_heap_usage > stats.peak_heap_usage {
            stats.peak_heap_usage = stats.current_heap_usage;
        }
        
        // Update average allocation size
        stats.avg_allocation_size = (stats.bytes_allocated / stats.total_allocations) as usize;
    }

    /// Update deallocation statistics
    fn update_deallocation_stats(&self, size: usize) {
        let mut stats = self.stats.write().unwrap();
        stats.total_deallocations += 1;
        stats.bytes_deallocated += size as u64;
        stats.current_heap_usage = stats.current_heap_usage.saturating_sub(size);
    }

    /// Initialize memory pools
    fn initialize_memory_pools(&self) -> Result<(), CursedError> {
        if !self.config.pool_reuse {
            return Ok(());
        }

        let mut pools = self.memory_pools.write().unwrap();
        
        for &size in &self.config.pool_sizes {
            let pool = MemoryPool {
                object_size: size,
                available: VecDeque::new(),
                total_size: 0,
                used_objects: 0,
                chunks: Vec::new(),
            };
            pools.insert(size, pool);
        }
        
        Ok(())
    }

    /// Cleanup memory pools
    fn cleanup_memory_pools(&self) -> Result<(), CursedError> {
        let mut pools = self.memory_pools.write().unwrap();
        
        for (_, pool) in pools.iter_mut() {
            // Deallocate all chunks
            for chunk in pool.chunks.drain(..) {
                let layout = Layout::from_size_align(pool.object_size * 100, self.config.alignment)
                    .map_err(|e| CursedError::runtime_error(&format!("Layout error: {}", e)))?;
                unsafe { dealloc(chunk.as_ptr(), layout) };
            }
        }
        
        pools.clear();
        Ok(())
    }

    /// Initialize size classes
    fn initialize_size_classes(&self) -> Result<(), CursedError> {
        // Size classes are already initialized in new()
        Ok(())
    }

    /// Cleanup size classes
    fn cleanup_size_classes(&self) -> Result<(), CursedError> {
        let mut size_classes = self.size_classes.write().unwrap();
        
        for size_class in size_classes.iter_mut() {
            // Deallocate all chunks
            for chunk in size_class.chunks.drain(..) {
                let chunk_size = size_class.size * size_class.objects_per_chunk;
                let layout = Layout::from_size_align(chunk_size, self.config.alignment)
                    .map_err(|e| CursedError::runtime_error(&format!("Layout error: {}", e)))?;
                unsafe { dealloc(chunk.as_ptr(), layout) };
            }
            
            size_class.free_list.clear();
        }
        
        Ok(())
    }

    /// First-fit allocation strategy
    fn allocate_first_fit(&self, size: usize) -> Result<NonNull<u8>, CursedError> {
        // Try to allocate from existing free blocks first
        if let Some(gc) = &self.gc_ref {
            if let Ok(ptr) = gc.try_allocate_first_fit(size) {
                return Ok(ptr);
            }
        }
        
        // Fallback to system allocation
        let layout = Layout::from_size_align(size, self.config.alignment)
            .map_err(|e| CursedError::runtime_error(&format!("Layout error: {}", e)))?;
        
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            // Try GC and retry
            if let Some(gc) = &self.gc_ref {
                gc.collect_garbage()?;
                let ptr = unsafe { alloc(layout) };
                if !ptr.is_null() {
                    return Ok(unsafe { NonNull::new_unchecked(ptr) });
                }
            }
            return Err(CursedError::runtime_error("Failed to allocate - out of memory"));
        }
        
        Ok(unsafe { NonNull::new_unchecked(ptr) })
    }

    /// Best-fit allocation strategy
    fn allocate_best_fit(&self, size: usize) -> Result<NonNull<u8>, CursedError> {
        // Search for the smallest block that fits the request
        if let Some(gc) = &self.gc_ref {
            if let Ok(ptr) = gc.try_allocate_best_fit(size) {
                return Ok(ptr);
            }
        }
        
        // If no suitable free blocks, use memory pools for better fit
        let mut pools = self.memory_pools.write().unwrap();
        
        // Find best fitting pool
        let mut best_pool_size = None;
        let mut best_difference = usize::MAX;
        
        for &pool_size in &self.config.pool_sizes {
            if pool_size >= size {
                let difference = pool_size - size;
                if difference < best_difference {
                    best_difference = difference;
                    best_pool_size = Some(pool_size);
                }
            }
        }
        
        if let Some(pool_size) = best_pool_size {
            if let Some(pool) = pools.get_mut(&pool_size) {
                if let Some(ptr) = pool.available.pop_front() {
                    pool.used_objects += 1;
                    return Ok(ptr);
                }
                
                // Allocate new chunk for pool
                self.allocate_pool_chunk(pool)?;
                if let Some(ptr) = pool.available.pop_front() {
                    pool.used_objects += 1;
                    return Ok(ptr);
                }
            }
        }
        
        // Fallback to first-fit
        self.allocate_first_fit(size)
    }

    /// Worst-fit allocation strategy
    fn allocate_worst_fit(&self, size: usize) -> Result<NonNull<u8>, CursedError> {
        // Search for the largest available block to minimize fragmentation
        if let Some(gc) = &self.gc_ref {
            if let Ok(ptr) = gc.try_allocate_worst_fit(size) {
                return Ok(ptr);
            }
        }
        
        // Use largest available pool
        let pools = self.memory_pools.read().unwrap();
        let mut largest_pool_size = None;
        
        for &pool_size in &self.config.pool_sizes {
            if pool_size >= size {
                largest_pool_size = Some(pool_size);
            }
        }
        
        drop(pools);
        
        if let Some(pool_size) = largest_pool_size {
            return self.allocate_from_pool(pool_size, size);
        }
        
        // Fallback to first-fit
        self.allocate_first_fit(size)
    }

    /// Next-fit allocation strategy
    fn allocate_next_fit(&self, size: usize) -> Result<NonNull<u8>, CursedError> {
        // Start search from last allocation point to reduce fragmentation
        let last_ptr = self.next_fit_ptr.load(Ordering::Relaxed);
        
        if let Some(gc) = &self.gc_ref {
            if let Ok(ptr) = gc.try_allocate_next_fit(size, last_ptr) {
                self.next_fit_ptr.store(ptr.as_ptr() as usize + size, Ordering::Relaxed);
                return Ok(ptr);
            }
        }
        
        // Fallback to first-fit and update next pointer
        let ptr = self.allocate_first_fit(size)?;
        self.next_fit_ptr.store(ptr.as_ptr() as usize + size, Ordering::Relaxed);
        Ok(ptr)
    }

    /// Buddy allocation strategy
    fn allocate_buddy(&self, size: usize) -> Result<NonNull<u8>, CursedError> {
        // Use buddy system for power-of-2 sizes
        let buddy_size = size.next_power_of_two();
        
        if let Some(gc) = &self.gc_ref {
            if let Ok(ptr) = gc.try_allocate_buddy(buddy_size) {
                return Ok(ptr);
            }
        }
        
        // Fallback to size class allocation with power-of-2 rounding
        self.allocate_size_class(buddy_size)
    }

    /// Slab allocation strategy
    fn allocate_slab(&self, size: usize) -> Result<NonNull<u8>, CursedError> {
        // Use dedicated slabs for common object sizes
        let slab_size = self.find_slab_size(size);
        
        // Use memory pools as slab allocators
        self.allocate_from_pool(slab_size, size)
    }

    /// Helper: Find appropriate slab size
    fn find_slab_size(&self, size: usize) -> usize {
        // Common object sizes for slab allocation
        const SLAB_SIZES: &[usize] = &[16, 32, 64, 96, 128, 192, 256, 512, 1024, 2048, 4096];
        
        for &slab_size in SLAB_SIZES {
            if size <= slab_size {
                return slab_size;
            }
        }
        
        // For larger objects, round up to next power of 2
        size.next_power_of_two()
    }

    /// Helper: Allocate from specific memory pool
    fn allocate_from_pool(&self, pool_size: usize, requested_size: usize) -> Result<NonNull<u8>, CursedError> {
        let mut pools = self.memory_pools.write().unwrap();
        
        let pool = pools.entry(pool_size).or_insert_with(|| MemoryPool {
            object_size: pool_size,
            available: VecDeque::new(),
            total_size: 0,
            used_objects: 0,
            chunks: Vec::new(),
        });
        
        // Try to get from available objects
        if let Some(ptr) = pool.available.pop_front() {
            pool.used_objects += 1;
            return Ok(ptr);
        }
        
        // Allocate new chunk
        self.allocate_pool_chunk(pool)?;
        
        if let Some(ptr) = pool.available.pop_front() {
            pool.used_objects += 1;
            Ok(ptr)
        } else {
            Err(CursedError::runtime_error("Failed to allocate from pool"))
        }
    }

    /// Helper: Allocate new chunk for memory pool
    fn allocate_pool_chunk(&self, pool: &mut MemoryPool) -> Result<(), CursedError> {
        const OBJECTS_PER_CHUNK: usize = 64;
        let chunk_size = pool.object_size * OBJECTS_PER_CHUNK;
        
        let layout = Layout::from_size_align(chunk_size, self.config.alignment)
            .map_err(|e| CursedError::runtime_error(&format!("Layout error: {}", e)))?;
        
        let chunk_ptr = unsafe { alloc(layout) };
        if chunk_ptr.is_null() {
            return Err(CursedError::runtime_error("Failed to allocate pool chunk"));
        }
        
        let chunk = unsafe { NonNull::new_unchecked(chunk_ptr) };
        pool.chunks.push(chunk);
        pool.total_size += chunk_size;
        
        // Add objects to available list
        for i in 0..OBJECTS_PER_CHUNK {
            let obj_ptr = unsafe { chunk_ptr.add(i * pool.object_size) };
            pool.available.push_back(unsafe { NonNull::new_unchecked(obj_ptr) });
        }
        
        Ok(())
    }

    /// Get heap statistics
    pub fn get_stats(&self) -> HeapStats {
        self.stats.read().unwrap().clone()
    }

    /// Set garbage collector reference
    pub fn set_gc_ref(&mut self, gc: Arc<GarbageCollector>) {
        self.gc_ref = Some(gc);
    }

    /// Set memory manager reference
    pub fn set_memory_manager(&mut self, memory_manager: Arc<MemoryManager>) {
        self.memory_manager = Some(memory_manager);
    }

    /// Set profiler reference
    pub fn set_profiler(&mut self, profiler: Arc<MemoryProfiler>) {
        self.profiler = Some(profiler);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heap_optimizer_creation() {
        let config = HeapOptimizerConfig::default();
        let optimizer = HeapOptimizer::new(config).unwrap();
        
        let stats = optimizer.get_stats();
        assert_eq!(stats.total_allocations, 0);
        assert_eq!(stats.current_heap_usage, 0);
    }

    #[test]
    fn test_allocation_strategies() {
        let strategies = vec![
            AllocationStrategy::FirstFit,
            AllocationStrategy::BestFit,
            AllocationStrategy::WorstFit,
            AllocationStrategy::NextFit,
            AllocationStrategy::Buddy,
            AllocationStrategy::Slab,
            AllocationStrategy::ThreadLocal,
            AllocationStrategy::SizeClass,
        ];
        
        for strategy in strategies {
            let config = HeapOptimizerConfig {
                allocation_strategy: strategy,
                ..Default::default()
            };
            let optimizer = HeapOptimizer::new(config).unwrap();
            
            // Test allocation
            let result = optimizer.allocate(64, 8, Tag::Object);
            assert!(result.is_ok());
        }
    }

    #[test]
    fn test_size_class_allocation() {
        let config = HeapOptimizerConfig {
            size_class_allocation: true,
            ..Default::default()
        };
        let optimizer = HeapOptimizer::new(config).unwrap();
        
        // Test allocation of different sizes
        let ptr1 = optimizer.allocate(32, 8, Tag::Object).unwrap();
        let ptr2 = optimizer.allocate(64, 8, Tag::Object).unwrap();
        let ptr3 = optimizer.allocate(128, 8, Tag::Object).unwrap();
        
        // Test deallocation
        optimizer.deallocate(ptr1, 32).unwrap();
        optimizer.deallocate(ptr2, 64).unwrap();
        optimizer.deallocate(ptr3, 128).unwrap();
        
        let stats = optimizer.get_stats();
        assert_eq!(stats.total_allocations, 3);
        assert_eq!(stats.total_deallocations, 3);
    }

    #[test]
    fn test_large_object_allocation() {
        let config = HeapOptimizerConfig {
            large_object_threshold: 1024,
            ..Default::default()
        };
        let optimizer = HeapOptimizer::new(config).unwrap();
        
        // Allocate large object
        let ptr = optimizer.allocate(2048, 8, Tag::Array).unwrap();
        
        let stats = optimizer.get_stats();
        assert_eq!(stats.large_object_allocations, 1);
        
        // Deallocate
        optimizer.deallocate(ptr, 2048).unwrap();
    }

    #[test]
    fn test_thread_local_buffers() {
        let config = HeapOptimizerConfig {
            thread_local_buffers: true,
            tlab_size: 4096,
            ..Default::default()
        };
        let optimizer = HeapOptimizer::new(config).unwrap();
        
        // Test TLAB allocation
        let ptr1 = optimizer.allocate(64, 8, Tag::Object).unwrap();
        let ptr2 = optimizer.allocate(128, 8, Tag::Object).unwrap();
        
        let stats = optimizer.get_stats();
        assert_eq!(stats.total_allocations, 2);
        
        // Deallocate
        optimizer.deallocate(ptr1, 64).unwrap();
        optimizer.deallocate(ptr2, 128).unwrap();
    }
}
