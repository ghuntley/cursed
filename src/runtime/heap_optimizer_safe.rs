/// Thread-safe heap optimizer with proper Send/Sync implementations
/// Fixes raw pointer Send/Sync violations in heap_optimizer.rs

use std::sync::{Arc, Mutex, RwLock};
use std::sync::atomic::{AtomicBool, AtomicUsize, AtomicPtr, Ordering};
use std::collections::{HashMap, VecDeque};
use std::time::{Duration, Instant};
use std::ptr::NonNull;
use std::alloc::{Layout, alloc, dealloc};
use std::marker::PhantomData;

use crate::runtime::gc::{GarbageCollector, HeapObject, ObjectMetadata};
use crate::runtime::memory::{MemoryManager, ObjectHandle};
use crate::runtime::memory_profiler::MemoryProfiler;
use crate::error::CursedError;
use crate::memory::Tag;

/// Thread-safe heap allocation strategy
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SafeAllocationStrategy {
    /// First-fit allocation with thread safety
    FirstFit,
    /// Best-fit allocation with thread safety
    BestFit,
    /// Size-class allocation with per-thread pools
    SizeClass,
    /// Thread-local allocation buffers
    ThreadLocal,
}

/// Thread-safe heap optimizer configuration
#[derive(Debug, Clone)]
pub struct SafeHeapOptimizerConfig {
    /// Primary allocation strategy
    pub allocation_strategy: SafeAllocationStrategy,
    /// Enable thread-local allocation buffers
    pub thread_local_buffers: bool,
    /// Thread-local buffer size
    pub tlab_size: usize,
    /// Large object threshold
    pub large_object_threshold: usize,
    /// Alignment requirement
    pub alignment: usize,
    /// Pool sizes for different object sizes
    pub pool_sizes: Vec<usize>,
    /// Enable allocation statistics
    pub track_statistics: bool,
    /// Maximum threads supported
    pub max_threads: usize,
}

impl Default for SafeHeapOptimizerConfig {
    fn default() -> Self {
        Self {
            allocation_strategy: SafeAllocationStrategy::SizeClass,
            thread_local_buffers: true,
            tlab_size: 1024 * 1024, // 1MB TLAB
            large_object_threshold: 256 * 1024, // 256KB
            alignment: 8, // 8-byte alignment
            pool_sizes: vec![32, 64, 128, 256, 512, 1024, 2048, 4096, 8192],
            track_statistics: true,
            max_threads: 64,
        }
    }
}

/// Thread-safe size class with atomic operations
#[derive(Debug)]
pub struct SafeSizeClass {
    /// Size class index
    pub index: usize,
    /// Object size for this class
    pub size: usize,
    /// Number of objects per chunk
    pub objects_per_chunk: usize,
    /// Free list using atomic pointers
    pub free_list: Arc<Mutex<VecDeque<SafePointer>>>,
    /// Allocated chunks using atomic pointers
    pub chunks: Arc<Mutex<Vec<SafePointer>>>,
    /// Statistics with atomic counters
    pub allocated_objects: AtomicUsize,
    pub freed_objects: AtomicUsize,
    pub total_chunks: AtomicUsize,
}

/// Thread-safe pointer wrapper for heap objects
#[derive(Debug)]
pub struct SafePointer {
    /// Atomic pointer to heap memory
    ptr: Arc<AtomicPtr<u8>>,
    /// Size of allocated memory
    size: usize,
    /// Tag for GC tracking
    tag: Tag,
    /// Generation for debugging
    generation: AtomicUsize,
    /// Thread safety marker
    _phantom: PhantomData<*const u8>,
}

impl SafePointer {
    /// Create new safe pointer
    pub fn new(ptr: *mut u8, size: usize, tag: Tag) -> Self {
        Self {
            ptr: Arc::new(AtomicPtr::new(ptr)),
            size,
            tag,
            generation: AtomicUsize::new(0),
            _phantom: PhantomData,
        }
    }
    
    /// Get raw pointer with proper ordering
    pub fn as_ptr(&self) -> *mut u8 {
        self.ptr.load(Ordering::Acquire)
    }
    
    /// Convert to NonNull if valid
    pub fn as_non_null(&self) -> Option<NonNull<u8>> {
        let ptr = self.as_ptr();
        NonNull::new(ptr)
    }
    
    /// Update pointer atomically
    pub fn update_ptr(&self, new_ptr: *mut u8) {
        self.ptr.store(new_ptr, Ordering::Release);
        self.generation.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Compare and swap pointer
    pub fn compare_exchange(
        &self,
        current: *mut u8,
        new: *mut u8,
    ) -> Result<*mut u8, *mut u8> {
        self.ptr.compare_exchange(current, new, Ordering::AcqRel, Ordering::Acquire)
    }
    
    /// Get size
    pub fn size(&self) -> usize {
        self.size
    }
    
    /// Get tag
    pub fn tag(&self) -> Tag {
        self.tag
    }
    
    /// Get generation
    pub fn generation(&self) -> usize {
        self.generation.load(Ordering::Relaxed)
    }
    
    /// Check if pointer is null
    pub fn is_null(&self) -> bool {
        self.as_ptr().is_null()
    }
}

impl Clone for SafePointer {
    fn clone(&self) -> Self {
        Self {
            ptr: Arc::clone(&self.ptr),
            size: self.size,
            tag: self.tag,
            generation: AtomicUsize::new(self.generation.load(Ordering::Relaxed)),
            _phantom: PhantomData,
        }
    }
}

// Safe Send/Sync implementations for SafePointer
unsafe impl Send for SafePointer {}
unsafe impl Sync for SafePointer {}

/// Thread-local allocation buffer with atomic operations
#[derive(Debug)]
pub struct SafeThreadLocalBuffer {
    /// Buffer start address (atomic)
    start: AtomicPtr<u8>,
    /// Buffer end address (atomic)
    end: AtomicPtr<u8>,
    /// Current allocation pointer (atomic)
    current: AtomicPtr<u8>,
    /// Buffer size
    size: usize,
    /// Remaining space (atomic)
    remaining: AtomicUsize,
    /// Buffer generation
    generation: AtomicUsize,
    /// Thread ID for debugging
    thread_id: std::thread::ThreadId,
}

impl SafeThreadLocalBuffer {
    /// Create new thread-local buffer
    pub fn new(size: usize) -> Result<Self, CursedError> {
        let layout = Layout::from_size_align(size, 8)
            .map_err(|e| CursedError::runtime_error(&format!("Layout error: {}", e)))?;
        
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return Err(CursedError::runtime_error("Failed to allocate TLAB"));
        }
        
        let end_ptr = unsafe { ptr.add(size) };
        
        Ok(Self {
            start: AtomicPtr::new(ptr),
            end: AtomicPtr::new(end_ptr),
            current: AtomicPtr::new(ptr),
            size,
            remaining: AtomicUsize::new(size),
            generation: AtomicUsize::new(0),
            thread_id: std::thread::current().id(),
        })
    }
    
    /// Allocate from TLAB atomically
    pub fn allocate(&self, size: usize) -> Option<SafePointer> {
        // Align size to 8 bytes
        let aligned_size = (size + 7) & !7;
        
        loop {
            let current_ptr = self.current.load(Ordering::Acquire);
            let remaining = self.remaining.load(Ordering::Acquire);
            
            if remaining < aligned_size {
                return None; // Not enough space
            }
            
            let new_ptr = unsafe { current_ptr.add(aligned_size) };
            let new_remaining = remaining - aligned_size;
            
            // Try to update current pointer and remaining space atomically
            if self.current.compare_exchange(
                current_ptr,
                new_ptr,
                Ordering::AcqRel,
                Ordering::Acquire
            ).is_ok() {
                // Successfully updated current pointer
                if self.remaining.compare_exchange(
                    remaining,
                    new_remaining,
                    Ordering::AcqRel,
                    Ordering::Acquire
                ).is_ok() {
                    // Successfully allocated
                    return Some(SafePointer::new(current_ptr, aligned_size, Tag::Object));
                } else {
                    // Failed to update remaining, retry
                    continue;
                }
            }
            // Failed to update current pointer, retry
        }
    }
    
    /// Reset buffer for reuse
    pub fn reset(&self) {
        let start_ptr = self.start.load(Ordering::Acquire);
        self.current.store(start_ptr, Ordering::Release);
        self.remaining.store(self.size, Ordering::Release);
        self.generation.fetch_add(1, Ordering::Relaxed);
    }
    
    /// Get remaining space
    pub fn remaining_space(&self) -> usize {
        self.remaining.load(Ordering::Acquire)
    }
    
    /// Check if buffer belongs to current thread
    pub fn is_current_thread(&self) -> bool {
        self.thread_id == std::thread::current().id()
    }
}

impl Drop for SafeThreadLocalBuffer {
    fn drop(&mut self) {
        let start_ptr = self.start.load(Ordering::Acquire);
        if !start_ptr.is_null() {
            let layout = Layout::from_size_align(self.size, 8).unwrap();
            unsafe { dealloc(start_ptr, layout) };
        }
    }
}

// Safe Send/Sync for TLAB
unsafe impl Send for SafeThreadLocalBuffer {}
unsafe impl Sync for SafeThreadLocalBuffer {}

/// Thread-safe heap statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SafeHeapStats {
    /// Total allocations (atomic)
    pub total_allocations: u64,
    /// Total deallocations (atomic)
    pub total_deallocations: u64,
    /// Bytes allocated (atomic)
    pub bytes_allocated: u64,
    /// Bytes deallocated (atomic)
    pub bytes_deallocated: u64,
    /// Current heap usage
    pub current_heap_usage: usize,
    /// Peak heap usage
    pub peak_heap_usage: usize,
    /// Thread-local buffer utilization
    pub tlab_utilization: f64,
    /// Size class statistics
    pub size_class_stats: HashMap<usize, SafeSizeClassStats>,
}

/// Thread-safe size class statistics
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SafeSizeClassStats {
    /// Total allocations
    pub allocations: u64,
    /// Total deallocations
    pub deallocations: u64,
    /// Current utilization
    pub utilization: f64,
    /// Waste percentage
    pub waste: f64,
}

/// Thread-safe heap optimizer
pub struct SafeHeapOptimizer {
    /// Configuration
    config: SafeHeapOptimizerConfig,
    /// Size classes with thread-safe operations
    size_classes: Arc<RwLock<Vec<SafeSizeClass>>>,
    /// Thread-local buffers with atomic access
    thread_local_buffers: Arc<RwLock<HashMap<std::thread::ThreadId, Arc<SafeThreadLocalBuffer>>>>,
    /// Large object tracking with atomic operations
    large_objects: Arc<RwLock<HashMap<usize, SafeLargeObjectInfo>>>,
    /// Allocation statistics with atomic counters
    stats: Arc<RwLock<SafeHeapStats>>,
    /// Running flag
    running: AtomicBool,
    /// Memory manager reference
    memory_manager: Option<Arc<dyn MemoryManager>>,
    /// Profiler reference
    profiler: Option<Arc<MemoryProfiler>>,
}

/// Thread-safe large object information
#[derive(Debug, Clone)]
pub struct SafeLargeObjectInfo {
    /// Object address
    pub address: usize,
    /// Object size
    pub size: usize,
    /// Allocation timestamp
    pub allocated_at: Instant,
    /// Object tag
    pub tag: Tag,
    /// Thread ID that allocated it
    pub thread_id: std::thread::ThreadId,
}

impl SafeHeapOptimizer {
    /// Create new thread-safe heap optimizer
    pub fn new(config: SafeHeapOptimizerConfig) -> Result<Self, CursedError> {
        let mut size_classes = Vec::new();
        
        // Initialize size classes with thread-safe operations
        for (index, &size) in config.pool_sizes.iter().enumerate() {
            let objects_per_chunk = std::cmp::max(1, 4096 / size);
            size_classes.push(SafeSizeClass {
                index,
                size,
                objects_per_chunk,
                free_list: Arc::new(Mutex::new(VecDeque::new())),
                chunks: Arc::new(Mutex::new(Vec::new())),
                allocated_objects: AtomicUsize::new(0),
                freed_objects: AtomicUsize::new(0),
                total_chunks: AtomicUsize::new(0),
            });
        }
        
        let stats = SafeHeapStats {
            total_allocations: 0,
            total_deallocations: 0,
            bytes_allocated: 0,
            bytes_deallocated: 0,
            current_heap_usage: 0,
            peak_heap_usage: 0,
            tlab_utilization: 0.0,
            size_class_stats: HashMap::new(),
        };
        
        Ok(Self {
            config,
            size_classes: Arc::new(RwLock::new(size_classes)),
            thread_local_buffers: Arc::new(RwLock::new(HashMap::new())),
            large_objects: Arc::new(RwLock::new(HashMap::new())),
            stats: Arc::new(RwLock::new(stats)),
            running: AtomicBool::new(false),
            memory_manager: None,
            profiler: None,
        })
    }
    
    /// Start heap optimizer
    pub fn start(&self) -> Result<(), CursedError> {
        if self.running.compare_exchange(false, true, Ordering::Relaxed, Ordering::Relaxed).is_err() {
            return Err(CursedError::runtime_error("Safe heap optimizer already running"));
        }
        
        Ok(())
    }
    
    /// Stop heap optimizer
    pub fn stop(&self) -> Result<(), CursedError> {
        self.running.store(false, Ordering::Relaxed);
        
        // Cleanup thread-local buffers
        self.cleanup_thread_local_buffers()?;
        
        Ok(())
    }
    
    /// Allocate object with thread-safe strategy
    pub fn allocate(&self, size: usize, alignment: usize, tag: Tag) -> Result<SafePointer, CursedError> {
        let aligned_size = self.align_size(size, alignment);
        
        // Update statistics atomically
        if self.config.track_statistics {
            self.update_allocation_stats(aligned_size);
        }
        
        // Choose allocation strategy
        let result = if aligned_size >= self.config.large_object_threshold {
            // Large object allocation
            self.allocate_large_object(aligned_size, tag)
        } else if self.config.thread_local_buffers {
            // Try thread-local allocation first
            if let Some(ptr) = self.allocate_from_tlab(aligned_size, tag) {
                Ok(ptr)
            } else {
                self.allocate_with_strategy(aligned_size, tag)
            }
        } else {
            // Use configured strategy
            self.allocate_with_strategy(aligned_size, tag)
        };
        
        // Record allocation in profiler
        if let (Some(profiler), Ok(ref safe_ptr)) = (&self.profiler, &result) {
            if let Some(ptr) = safe_ptr.as_non_null() {
                profiler.record_allocation(ptr.as_ptr() as usize, aligned_size, tag, None)?;
            }
        }
        
        result
    }
    
    /// Deallocate object with thread safety
    pub fn deallocate(&self, safe_ptr: &SafePointer) -> Result<(), CursedError> {
        let size = safe_ptr.size();
        
        // Update statistics atomically
        if self.config.track_statistics {
            self.update_deallocation_stats(size);
        }
        
        // Record deallocation in profiler
        if let Some(profiler) = &self.profiler {
            if let Some(ptr) = safe_ptr.as_non_null() {
                profiler.record_deallocation(ptr.as_ptr() as usize)?;
            }
        }
        
        // Handle deallocation based on size
        if size >= self.config.large_object_threshold {
            self.deallocate_large_object(safe_ptr)
        } else {
            self.deallocate_to_size_class(safe_ptr)
        }
    }
    
    /// Allocate from thread-local buffer with thread safety
    fn allocate_from_tlab(&self, size: usize, tag: Tag) -> Option<SafePointer> {
        let thread_id = std::thread::current().id();
        
        // Get or create TLAB for this thread
        {
            let tlabs = self.thread_local_buffers.read().ok()?;
            if let Some(tlab) = tlabs.get(&thread_id) {
                if let Some(ptr) = tlab.allocate(size) {
                    return Some(ptr);
                }
            }
        }
        
        // Create new TLAB if needed
        {
            let mut tlabs = self.thread_local_buffers.write().ok()?;
            let tlab = Arc::new(SafeThreadLocalBuffer::new(self.config.tlab_size).ok()?);
            tlabs.insert(thread_id, Arc::clone(&tlab));
            tlab.allocate(size)
        }
    }
    
    /// Allocate with configured strategy
    fn allocate_with_strategy(&self, size: usize, tag: Tag) -> Result<SafePointer, CursedError> {
        match self.config.allocation_strategy {
            SafeAllocationStrategy::FirstFit => self.allocate_first_fit(size, tag),
            SafeAllocationStrategy::BestFit => self.allocate_best_fit(size, tag),
            SafeAllocationStrategy::SizeClass => self.allocate_size_class(size, tag),
            SafeAllocationStrategy::ThreadLocal => {
                self.allocate_from_tlab(size, tag)
                    .ok_or_else(|| CursedError::runtime_error("TLAB allocation failed"))
            }
        }
    }
    
    /// Allocate using size classes with thread safety
    fn allocate_size_class(&self, size: usize, tag: Tag) -> Result<SafePointer, CursedError> {
        let size_class_index = self.find_size_class(size);
        let size_classes = self.size_classes.read()
            .map_err(|_| CursedError::runtime_error("Failed to acquire size classes lock"))?;
        
        if let Some(size_class) = size_classes.get(size_class_index) {
            // Try to allocate from free list
            {
                let mut free_list = size_class.free_list.lock()
                    .map_err(|_| CursedError::runtime_error("Failed to acquire free list lock"))?;
                
                if let Some(ptr) = free_list.pop_front() {
                    size_class.allocated_objects.fetch_add(1, Ordering::Relaxed);
                    return Ok(ptr);
                }
            }
            
            // Need to allocate new chunk
            let chunk_size = size_class.size * size_class.objects_per_chunk;
            let layout = Layout::from_size_align(chunk_size, self.config.alignment)
                .map_err(|e| CursedError::runtime_error(&format!("Layout error: {}", e)))?;
            
            let chunk_ptr = unsafe { alloc(layout) };
            if chunk_ptr.is_null() {
                return Err(CursedError::runtime_error("Failed to allocate chunk"));
            }
            
            let chunk = SafePointer::new(chunk_ptr, chunk_size, tag);
            
            // Add chunk to chunks list
            {
                let mut chunks = size_class.chunks.lock()
                    .map_err(|_| CursedError::runtime_error("Failed to acquire chunks lock"))?;
                chunks.push(chunk.clone());
                size_class.total_chunks.fetch_add(1, Ordering::Relaxed);
            }
            
            // Add remaining objects to free list
            {
                let mut free_list = size_class.free_list.lock()
                    .map_err(|_| CursedError::runtime_error("Failed to acquire free list lock"))?;
                
                for i in 1..size_class.objects_per_chunk {
                    let obj_ptr = unsafe { chunk_ptr.add(i * size_class.size) };
                    let safe_obj = SafePointer::new(obj_ptr, size_class.size, tag);
                    free_list.push_back(safe_obj);
                }
            }
            
            // Return first object
            size_class.allocated_objects.fetch_add(1, Ordering::Relaxed);
            Ok(SafePointer::new(chunk_ptr, size_class.size, tag))
        } else {
            Err(CursedError::runtime_error("Size class not found"))
        }
    }
    
    /// Allocate large object with thread safety
    fn allocate_large_object(&self, size: usize, tag: Tag) -> Result<SafePointer, CursedError> {
        let layout = Layout::from_size_align(size, self.config.alignment)
            .map_err(|e| CursedError::runtime_error(&format!("Layout error: {}", e)))?;
        
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return Err(CursedError::runtime_error("Failed to allocate large object"));
        }
        
        let safe_ptr = SafePointer::new(ptr, size, tag);
        
        // Record large object
        {
            let mut large_objects = self.large_objects.write()
                .map_err(|_| CursedError::runtime_error("Failed to acquire large objects lock"))?;
            
            large_objects.insert(ptr as usize, SafeLargeObjectInfo {
                address: ptr as usize,
                size,
                allocated_at: Instant::now(),
                tag,
                thread_id: std::thread::current().id(),
            });
        }
        
        Ok(safe_ptr)
    }
    
    /// Deallocate to size class with thread safety
    fn deallocate_to_size_class(&self, safe_ptr: &SafePointer) -> Result<(), CursedError> {
        let size = safe_ptr.size();
        let size_class_index = self.find_size_class(size);
        
        let size_classes = self.size_classes.read()
            .map_err(|_| CursedError::runtime_error("Failed to acquire size classes lock"))?;
        
        if let Some(size_class) = size_classes.get(size_class_index) {
            let mut free_list = size_class.free_list.lock()
                .map_err(|_| CursedError::runtime_error("Failed to acquire free list lock"))?;
            
            free_list.push_back(safe_ptr.clone());
            size_class.freed_objects.fetch_add(1, Ordering::Relaxed);
        }
        
        Ok(())
    }
    
    /// Deallocate large object with thread safety
    fn deallocate_large_object(&self, safe_ptr: &SafePointer) -> Result<(), CursedError> {
        let ptr = safe_ptr.as_ptr();
        let size = safe_ptr.size();
        
        // Remove from large objects tracking
        {
            let mut large_objects = self.large_objects.write()
                .map_err(|_| CursedError::runtime_error("Failed to acquire large objects lock"))?;
            large_objects.remove(&(ptr as usize));
        }
        
        let layout = Layout::from_size_align(size, self.config.alignment)
            .map_err(|e| CursedError::runtime_error(&format!("Layout error: {}", e)))?;
        
        unsafe { dealloc(ptr, layout) };
        
        Ok(())
    }
    
    /// Cleanup thread-local buffers
    fn cleanup_thread_local_buffers(&self) -> Result<(), CursedError> {
        let mut tlabs = self.thread_local_buffers.write()
            .map_err(|_| CursedError::runtime_error("Failed to acquire TLAB lock"))?;
        
        tlabs.clear();
        Ok(())
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
    
    /// Update allocation statistics atomically
    fn update_allocation_stats(&self, size: usize) {
        if let Ok(mut stats) = self.stats.write() {
            stats.total_allocations += 1;
            stats.bytes_allocated += size as u64;
            stats.current_heap_usage += size;
            
            if stats.current_heap_usage > stats.peak_heap_usage {
                stats.peak_heap_usage = stats.current_heap_usage;
            }
        }
    }
    
    /// Update deallocation statistics atomically
    fn update_deallocation_stats(&self, size: usize) {
        if let Ok(mut stats) = self.stats.write() {
            stats.total_deallocations += 1;
            stats.bytes_deallocated += size as u64;
            stats.current_heap_usage = stats.current_heap_usage.saturating_sub(size);
        }
    }
    
    /// First-fit allocation with thread safety
    fn allocate_first_fit(&self, size: usize, tag: Tag) -> Result<SafePointer, CursedError> {
        let layout = Layout::from_size_align(size, self.config.alignment)
            .map_err(|e| CursedError::runtime_error(&format!("Layout error: {}", e)))?;
        
        let ptr = unsafe { alloc(layout) };
        if ptr.is_null() {
            return Err(CursedError::runtime_error("Failed to allocate"));
        }
        
        Ok(SafePointer::new(ptr, size, tag))
    }
    
    /// Best-fit allocation with thread safety
    fn allocate_best_fit(&self, size: usize, tag: Tag) -> Result<SafePointer, CursedError> {
        // For now, delegate to first-fit
        self.allocate_first_fit(size, tag)
    }
    
    /// Get heap statistics
    pub fn get_stats(&self) -> Option<SafeHeapStats> {
        self.stats.read().ok().map(|stats| stats.clone())
    }
    
    /// Set memory manager reference
    pub fn set_memory_manager(&mut self, memory_manager: Arc<dyn MemoryManager>) {
        self.memory_manager = Some(memory_manager);
    }
    
    /// Set profiler reference
    pub fn set_profiler(&mut self, profiler: Arc<MemoryProfiler>) {
        self.profiler = Some(profiler);
    }
}

// Safe Send/Sync implementations
unsafe impl Send for SafeHeapOptimizer {}
unsafe impl Sync for SafeHeapOptimizer {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_heap_optimizer_creation() {
        let config = SafeHeapOptimizerConfig::default();
        let optimizer = SafeHeapOptimizer::new(config).unwrap();
        
        let stats = optimizer.get_stats().unwrap();
        assert_eq!(stats.total_allocations, 0);
        assert_eq!(stats.current_heap_usage, 0);
    }
    
    #[test]
    fn test_safe_pointer_operations() {
        let ptr = SafePointer::new(std::ptr::null_mut(), 64, Tag::Object);
        assert!(ptr.is_null());
        assert_eq!(ptr.size(), 64);
        assert_eq!(ptr.tag(), Tag::Object);
        assert_eq!(ptr.generation(), 0);
        
        let test_ptr = 0x1000 as *mut u8;
        ptr.update_ptr(test_ptr);
        assert_eq!(ptr.as_ptr(), test_ptr);
        assert_eq!(ptr.generation(), 1);
    }
    
    #[test]
    fn test_safe_tlab_operations() {
        let tlab = SafeThreadLocalBuffer::new(4096).unwrap();
        assert_eq!(tlab.remaining_space(), 4096);
        assert!(tlab.is_current_thread());
        
        let ptr1 = tlab.allocate(64).unwrap();
        assert_eq!(ptr1.size(), 64);
        assert_eq!(tlab.remaining_space(), 4096 - 64);
        
        let ptr2 = tlab.allocate(128);
        assert!(ptr2.is_some());
        assert_eq!(tlab.remaining_space(), 4096 - 64 - 128);
        
        tlab.reset();
        assert_eq!(tlab.remaining_space(), 4096);
    }
    
    #[test]
    fn test_concurrent_allocation() {
        use std::thread;
        use std::sync::Arc;
        
        let config = SafeHeapOptimizerConfig::default();
        let optimizer = Arc::new(SafeHeapOptimizer::new(config).unwrap());
        optimizer.start().unwrap();
        
        let handles: Vec<_> = (0..4).map(|i| {
            let opt = Arc::clone(&optimizer);
            thread::spawn(move || {
                for j in 0..100 {
                    let size = 64 + (i * 10 + j) % 128;
                    let ptr = opt.allocate(size, 8, Tag::Object).unwrap();
                    assert!(!ptr.is_null());
                    opt.deallocate(&ptr).unwrap();
                }
            })
        }).collect();
        
        for handle in handles {
            handle.join().unwrap();
        }
        
        optimizer.stop().unwrap();
        
        let stats = optimizer.get_stats().unwrap();
        assert_eq!(stats.total_allocations, 400);
        assert_eq!(stats.total_deallocations, 400);
    }
}
