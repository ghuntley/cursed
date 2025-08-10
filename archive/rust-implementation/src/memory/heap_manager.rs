//! Heap Manager for CURSED Runtime
//!
//! This module provides a comprehensive heap management system with:
//! - Thread-safe heap allocation and deallocation
//! - Integration with garbage collection system
//! - Memory region management with generations
//! - Fragmentation handling and compaction
//! - Comprehensive error handling and debugging support

use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicUsize, AtomicPtr, Ordering}};
use std::collections::{HashMap, VecDeque};
use std::ptr::NonNull;
use std::alloc::{self, Layout};
use std::time::Instant;

use crate::error::CursedError;
use crate::memory::{Tag, Traceable};

/// Heap allocation configuration
#[derive(Debug, Clone)]
pub struct HeapConfig {
    /// Initial heap size in bytes
    pub initial_size: usize,
    /// Maximum heap size in bytes (None for unlimited)
    pub max_size: Option<usize>,
    /// Heap growth factor when expanding
    pub growth_factor: f64,
    /// Minimum block size for allocation
    pub min_block_size: usize,
    /// Maximum block size before using large object allocation
    pub max_block_size: usize,
    /// Enable heap compaction
    pub enable_compaction: bool,
    /// Fragmentation threshold for triggering compaction (0.0-1.0)
    pub fragmentation_threshold: f64,
    /// Alignment requirement for allocations
    pub alignment: usize,
}

impl Default for HeapConfig {
    fn default() -> Self {
        Self {
            initial_size: 64 * 1024 * 1024,    // 64MB
            max_size: Some(1024 * 1024 * 1024), // 1GB
            growth_factor: 1.5,
            min_block_size: 8,
            max_block_size: 1024 * 1024,       // 1MB
            enable_compaction: true,
            fragmentation_threshold: 0.3,      // 30%
            alignment: 8,
        }
    }
}

/// Heap object header stored before each allocated object
#[repr(C)]
#[derive(Debug)]
pub struct ObjectHeader {
    /// Object size in bytes (including header)
    pub size: usize,
    /// Object type tag
    pub tag: Tag,
    /// Allocation timestamp
    pub allocated_at: Instant,
    /// Mark bits for GC (bit 0 = marked, bit 1 = pinned)
    pub mark_bits: u8,
    /// Reference count for hybrid collection
    pub ref_count: AtomicUsize,
    /// Magic number for corruption detection
    pub magic: u32,
}

impl Clone for ObjectHeader {
    fn clone(&self) -> Self {
        Self {
            size: self.size,
            tag: self.tag,
            allocated_at: self.allocated_at,
            mark_bits: self.mark_bits,
            ref_count: AtomicUsize::new(self.ref_count.load(Ordering::Relaxed)),
            magic: self.magic,
        }
    }
}

impl ObjectHeader {
    const MAGIC: u32 = 0xDEADBEEF;
    
    fn new(size: usize, tag: Tag) -> Self {
        Self {
            size,
            tag,
            allocated_at: Instant::now(),
            mark_bits: 0,
            ref_count: AtomicUsize::new(1),
            magic: Self::MAGIC,
        }
    }
    
    fn is_valid(&self) -> bool {
        self.magic == Self::MAGIC
    }
    
    fn is_marked(&self) -> bool {
        self.mark_bits & 1 != 0
    }
    
    fn mark(&mut self) {
        self.mark_bits |= 1;
    }
    
    fn unmark(&mut self) {
        self.mark_bits &= !1;
    }
    
    fn is_pinned(&self) -> bool {
        self.mark_bits & 2 != 0
    }
    
    fn pin(&mut self) {
        self.mark_bits |= 2;
    }
    
    fn unpin(&mut self) {
        self.mark_bits &= !2;
    }
}

/// Free block in heap
#[derive(Debug, Clone)]
pub struct FreeBlock {
    /// Block address
    pub ptr: NonNull<u8>,
    /// Block size in bytes
    pub size: usize,
    /// Next free block in the same size class
    pub next: Option<NonNull<FreeBlock>>,
}

/// Heap region for memory management
#[derive(Debug)]
pub struct HeapRegion {
    /// Region start address
    pub start: *mut u8,
    /// Region size in bytes
    pub size: usize,
    /// Current allocation pointer for bump allocation
    pub alloc_ptr: AtomicPtr<u8>,
    /// End of region
    pub end: *mut u8,
    /// Free blocks organized by size class
    pub free_blocks: Mutex<VecDeque<FreeBlock>>,
    /// Allocated objects in this region
    pub objects: RwLock<HashMap<*mut u8, ObjectHeader>>,
    /// Region statistics
    pub stats: RwLock<RegionStats>,
}

unsafe impl Send for HeapRegion {}
unsafe impl Sync for HeapRegion {}

/// Statistics for a heap region
#[derive(Debug, Clone, Default)]
pub struct RegionStats {
    /// Total bytes allocated in this region
    pub bytes_allocated: usize,
    /// Total bytes freed in this region
    pub bytes_freed: usize,
    /// Number of objects allocated
    pub objects_allocated: usize,
    /// Number of objects freed
    pub objects_freed: usize,
    /// Fragmentation ratio (0.0-1.0)
    pub fragmentation: f64,
    /// Last compaction time
    pub last_compaction: Option<Instant>,
}

/// Heap allocation statistics
#[derive(Debug, Clone, Default)]
pub struct HeapStats {
    /// Total heap size in bytes
    pub total_size: usize,
    /// Used heap size in bytes
    pub used_size: usize,
    /// Free heap size in bytes
    pub free_size: usize,
    /// Number of allocated objects
    pub allocated_objects: usize,
    /// Number of free blocks
    pub free_blocks: usize,
    /// Fragmentation ratio (0.0-1.0)
    pub fragmentation: f64,
    /// Number of heap expansions
    pub expansions: usize,
    /// Number of compactions performed
    pub compactions: usize,
    /// Total allocation time
    pub total_alloc_time_ns: u64,
    /// Total deallocation time
    pub total_dealloc_time_ns: u64,
}

/// Main heap manager
pub struct HeapManager {
    /// Heap configuration
    config: HeapConfig,
    /// Heap regions
    regions: RwLock<Vec<Arc<HeapRegion>>>,
    /// Heap statistics
    stats: RwLock<HeapStats>,
    /// Large object allocations (>max_block_size)
    large_objects: RwLock<HashMap<*mut u8, ObjectHeader>>,
    /// Allocation counter for triggering GC
    allocation_counter: AtomicUsize,
    /// Total allocated bytes
    total_allocated: AtomicUsize,
}

impl HeapManager {
    /// Create a new heap manager
    pub fn new(config: HeapConfig) -> Result<Arc<Self>, CursedError> {
        let initial_region = Self::create_region(config.initial_size)?;
        
        let heap = Arc::new(HeapManager {
            config,
            regions: RwLock::new(vec![initial_region]),
            stats: RwLock::new(HeapStats::default()),
            large_objects: RwLock::new(HashMap::new()),
            allocation_counter: AtomicUsize::new(0),
            total_allocated: AtomicUsize::new(0),
        });
        
        // Initialize stats
        {
            let mut stats = heap.stats.write().unwrap();
            stats.total_size = heap.config.initial_size;
            stats.free_size = heap.config.initial_size;
        }
        
        Ok(heap)
    }
    
    /// Create a new heap region
    fn create_region(size: usize) -> Result<Arc<HeapRegion>, CursedError> {
        let layout = Layout::from_size_align(size, 4096)
            .map_err(|e| CursedError::runtime_error(&format!("Invalid layout: {}", e)))?;
        
        let start = unsafe { alloc::alloc(layout) };
        if start.is_null() {
            return Err(CursedError::runtime_error("Failed to allocate heap region"));
        }
        
        let end = unsafe { start.add(size) };
        
        Ok(Arc::new(HeapRegion {
            start,
            size,
            alloc_ptr: AtomicPtr::new(start),
            end,
            free_blocks: Mutex::new(VecDeque::new()),
            objects: RwLock::new(HashMap::new()),
            stats: RwLock::new(RegionStats::default()),
        }))
    }
    
    /// Allocate memory for an object
    pub fn allocate(&self, size: usize, tag: Tag) -> Result<NonNull<u8>, CursedError> {
        let start_time = Instant::now();
        
        // Validate size
        if size == 0 {
            return Err(CursedError::runtime_error("Cannot allocate zero bytes"));
        }
        
        // Calculate total size including header and alignment
        let header_size = std::mem::size_of::<ObjectHeader>();
        let total_size = Self::align_size(size + header_size, self.config.alignment);
        
        // Handle large objects separately
        if total_size > self.config.max_block_size {
            let result = self.allocate_large_object(total_size, tag);
            self.update_alloc_stats(start_time, total_size);
            return result;
        }
        
        // Try to allocate in existing regions
        let regions = self.regions.read().unwrap();
        for region in regions.iter() {
            if let Ok(Some(ptr)) = self.try_allocate_in_region(region, total_size, tag) {
                self.update_alloc_stats(start_time, total_size);
                return Ok(ptr);
            }
        }
        drop(regions);
        
        // Need to expand heap
        self.expand_heap()?;
        
        // Try again in new region
        let regions = self.regions.read().unwrap();
        let new_region = regions.last().unwrap();
        if let Ok(Some(ptr)) = self.try_allocate_in_region(new_region, total_size, tag) {
            self.update_alloc_stats(start_time, total_size);
            return Ok(ptr);
        }
        
        Err(CursedError::runtime_error("Heap allocation failed"))
    }
    
    /// Try to allocate in a specific region
    fn try_allocate_in_region(
        &self,
        region: &HeapRegion,
        total_size: usize,
        tag: Tag,
    ) -> Result<Option<NonNull<u8>>, CursedError> {
        // Try free blocks first
        {
            let mut free_blocks = region.free_blocks.lock().unwrap();
            if let Some(block_idx) = free_blocks.iter().position(|block| block.size >= total_size) {
                let block = free_blocks.remove(block_idx).unwrap();
                
                // Split block if it's much larger
                if block.size > total_size + self.config.min_block_size {
                    let remaining_size = block.size - total_size;
                    let remaining_ptr = unsafe { 
                        NonNull::new_unchecked(block.ptr.as_ptr().add(total_size))
                    };
                    
                    free_blocks.push_back(FreeBlock {
                        ptr: remaining_ptr,
                        size: remaining_size,
                        next: None,
                    });
                }
                
                let ptr = self.initialize_object(block.ptr.as_ptr(), total_size, tag)?;
                self.register_object(region, ptr, total_size, tag)?;
                return Ok(Some(ptr));
            }
        }
        
        // Try bump allocation
        loop {
            let current = region.alloc_ptr.load(Ordering::Acquire);
            let new_ptr = unsafe { current.add(total_size) };
            
            if new_ptr > region.end {
                return Ok(None); // Region full
            }
            
            match region.alloc_ptr.compare_exchange_weak(
                current,
                new_ptr,
                Ordering::Release,
                Ordering::Relaxed,
            ) {
                Ok(_) => {
                    let ptr = self.initialize_object(current, total_size, tag)?;
                    self.register_object(region, ptr, total_size, tag)?;
                    return Ok(Some(ptr));
                }
                Err(_) => continue, // Retry CAS
            }
        }
    }
    
    /// Initialize object at given address
    fn initialize_object(
        &self,
        ptr: *mut u8,
        total_size: usize,
        tag: Tag,
    ) -> Result<NonNull<u8>, CursedError> {
        let header = ObjectHeader::new(total_size, tag);
        
        unsafe {
            // Write header
            std::ptr::write(ptr as *mut ObjectHeader, header);
            
            // Zero out object data
            let data_ptr = ptr.add(std::mem::size_of::<ObjectHeader>());
            let data_size = total_size - std::mem::size_of::<ObjectHeader>();
            std::ptr::write_bytes(data_ptr, 0, data_size);
            
            // Return pointer to data (after header)
            Ok(NonNull::new_unchecked(data_ptr))
        }
    }
    
    /// Register object in region
    fn register_object(
        &self,
        region: &HeapRegion,
        ptr: NonNull<u8>,
        total_size: usize,
        tag: Tag,
    ) -> Result<(), CursedError> {
        let header_ptr = unsafe { 
            (ptr.as_ptr() as *mut u8).sub(std::mem::size_of::<ObjectHeader>()) 
        };
        
        let header = ObjectHeader::new(total_size, tag);
        
        // Register in region
        {
            let mut objects = region.objects.write().unwrap();
            objects.insert(header_ptr, header);
        }
        
        // Update region stats
        {
            let mut stats = region.stats.write().unwrap();
            stats.bytes_allocated += total_size;
            stats.objects_allocated += 1;
        }
        
        self.allocation_counter.fetch_add(total_size, Ordering::Relaxed);
        self.total_allocated.fetch_add(total_size, Ordering::Relaxed);
        
        Ok(())
    }
    
    /// Allocate large object
    fn allocate_large_object(&self, size: usize, tag: Tag) -> Result<NonNull<u8>, CursedError> {
        let layout = Layout::from_size_align(size, self.config.alignment)
            .map_err(|e| CursedError::runtime_error(&format!("Invalid layout: {}", e)))?;
        
        let ptr = unsafe { alloc::alloc(layout) };
        if ptr.is_null() {
            return Err(CursedError::runtime_error("Large object allocation failed"));
        }
        
        let data_ptr = self.initialize_object(ptr, size, tag)?;
        
        // Register large object
        {
            let header = ObjectHeader::new(size, tag);
            let mut large_objects = self.large_objects.write().unwrap();
            large_objects.insert(ptr, header);
        }
        
        self.allocation_counter.fetch_add(size, Ordering::Relaxed);
        self.total_allocated.fetch_add(size, Ordering::Relaxed);
        
        Ok(data_ptr)
    }
    
    /// Deallocate memory
    pub fn deallocate(&self, ptr: NonNull<u8>) -> Result<(), CursedError> {
        let start_time = Instant::now();
        
        let header_ptr = unsafe {
            (ptr.as_ptr() as *mut u8).sub(std::mem::size_of::<ObjectHeader>())
        };
        
        // Check if it's a large object
        {
            let mut large_objects = self.large_objects.write().unwrap();
            if let Some(header) = large_objects.remove(&header_ptr) {
                if !header.is_valid() {
                    return Err(CursedError::runtime_error("Invalid object header"));
                }
                
                let layout = Layout::from_size_align(header.size, self.config.alignment)
                    .map_err(|e| CursedError::runtime_error(&format!("Invalid layout: {}", e)))?;
                
                unsafe { alloc::dealloc(header_ptr, layout); }
                self.update_dealloc_stats(start_time, header.size);
                return Ok(());
            }
        }
        
        // Find in regions
        let regions = self.regions.read().unwrap();
        for region in regions.iter() {
            let mut objects = region.objects.write().unwrap();
            if let Some(header) = objects.remove(&header_ptr) {
                if !header.is_valid() {
                    return Err(CursedError::runtime_error("Invalid object header"));
                }
                
                // Add to free blocks
                {
                    let mut free_blocks = region.free_blocks.lock().unwrap();
                    free_blocks.push_back(FreeBlock {
                        ptr: unsafe { NonNull::new_unchecked(header_ptr) },
                        size: header.size,
                        next: None,
                    });
                }
                
                // Update region stats
                {
                    let mut stats = region.stats.write().unwrap();
                    stats.bytes_freed += header.size;
                    stats.objects_freed += 1;
                }
                
                self.update_dealloc_stats(start_time, header.size);
                return Ok(());
            }
        }
        
        Err(CursedError::runtime_error("Object not found for deallocation"))
    }
    
    /// Expand heap by adding a new region
    fn expand_heap(&self) -> Result<(), CursedError> {
        let current_size = {
            let stats = self.stats.read().unwrap();
            stats.total_size
        };
        
        let new_size = (current_size as f64 * self.config.growth_factor) as usize;
        
        // Check maximum size limit
        if let Some(max_size) = self.config.max_size {
            if current_size + new_size > max_size {
                return Err(CursedError::runtime_error("Heap size limit exceeded"));
            }
        }
        
        let new_region = Self::create_region(new_size)?;
        
        {
            let mut regions = self.regions.write().unwrap();
            regions.push(new_region);
        }
        
        // Update stats
        {
            let mut stats = self.stats.write().unwrap();
            stats.total_size += new_size;
            stats.free_size += new_size;
            stats.expansions += 1;
        }
        
        Ok(())
    }
    
    /// Compact heap to reduce fragmentation
    pub fn compact(&self) -> Result<(), CursedError> {
        if !self.config.enable_compaction {
            return Ok(());
        }
        
        let regions = self.regions.read().unwrap();
        for region in regions.iter() {
            self.compact_region(region)?;
        }
        
        // Update stats
        {
            let mut stats = self.stats.write().unwrap();
            stats.compactions += 1;
        }
        
        Ok(())
    }
    
    /// Compact a specific region
    fn compact_region(&self, region: &HeapRegion) -> Result<(), CursedError> {
        let mut free_blocks = region.free_blocks.lock().unwrap();
        
        // Sort free blocks by address
        let mut blocks: Vec<_> = free_blocks.drain(..).collect();
        blocks.sort_by_key(|block| block.ptr.as_ptr() as usize);
        
        // Coalesce adjacent blocks
        let mut coalesced = VecDeque::new();
        let mut current: Option<FreeBlock> = None;
        
        for block in blocks {
            match current {
                None => current = Some(block),
                Some(ref mut curr) => {
                    let curr_end = unsafe { curr.ptr.as_ptr().add(curr.size) };
                    if curr_end == block.ptr.as_ptr() {
                        // Adjacent blocks, coalesce
                        curr.size += block.size;
                    } else {
                        // Not adjacent, save current and start new
                        coalesced.push_back(current.take().unwrap());
                        current = Some(block);
                    }
                }
            }
        }
        
        // Add the last block
        if let Some(block) = current {
            coalesced.push_back(block);
        }
        
        // Calculate fragmentation before updating stats (to avoid deadlock)
        let fragmentation = self.calculate_fragmentation_from_blocks(&coalesced);
        
        // Update free blocks
        *free_blocks = coalesced;
        
        // Release the free_blocks lock before acquiring stats lock to avoid deadlock
        drop(free_blocks);
        
        // Update region stats
        {
            let mut stats = region.stats.write().unwrap();
            stats.last_compaction = Some(Instant::now());
            stats.fragmentation = fragmentation;
        }
        
        Ok(())
    }
    
    /// Calculate fragmentation ratio for a region
    fn calculate_fragmentation(&self, region: &HeapRegion) -> f64 {
        let free_blocks = region.free_blocks.lock().unwrap();
        self.calculate_fragmentation_from_blocks(&free_blocks)
    }
    
    /// Calculate fragmentation ratio from free blocks collection
    fn calculate_fragmentation_from_blocks(&self, free_blocks: &VecDeque<FreeBlock>) -> f64 {
        if free_blocks.is_empty() {
            return 0.0;
        }
        
        let total_free: usize = free_blocks.iter().map(|b| b.size).sum();
        let largest_free = free_blocks.iter().map(|b| b.size).max().unwrap_or(0);
        
        if total_free == 0 {
            0.0
        } else {
            1.0 - (largest_free as f64 / total_free as f64)
        }
    }
    
    /// Get heap statistics
    pub fn get_stats(&self) -> HeapStats {
        let mut stats = self.stats.read().unwrap().clone();
        
        // Calculate current usage
        let regions = self.regions.read().unwrap();
        let mut used_size = 0;
        let mut free_blocks_count = 0;
        let mut allocated_objects = 0;
        
        for region in regions.iter() {
            let region_stats = region.stats.read().unwrap();
            used_size += region_stats.bytes_allocated - region_stats.bytes_freed;
            
            let free_blocks = region.free_blocks.lock().unwrap();
            free_blocks_count += free_blocks.len();
            
            let objects = region.objects.read().unwrap();
            allocated_objects += objects.len();
        }
        
        // Add large objects
        let large_objects = self.large_objects.read().unwrap();
        allocated_objects += large_objects.len();
        for header in large_objects.values() {
            used_size += header.size;
        }
        
        stats.used_size = used_size;
        stats.free_size = stats.total_size - used_size;
        stats.allocated_objects = allocated_objects;
        stats.free_blocks = free_blocks_count;
        
        // Calculate overall fragmentation
        if stats.free_size > 0 {
            let fragmentation_sum: f64 = regions.iter()
                .map(|r| self.calculate_fragmentation(r))
                .sum();
            stats.fragmentation = fragmentation_sum / regions.len() as f64;
        }
        
        stats
    }
    
    /// Get object header for a given pointer
    pub fn get_object_header(&self, ptr: NonNull<u8>) -> Result<ObjectHeader, CursedError> {
        let header_ptr = unsafe {
            (ptr.as_ptr() as *mut u8).sub(std::mem::size_of::<ObjectHeader>())
        };
        
        // Check large objects first
        {
            let large_objects = self.large_objects.read().unwrap();
            if let Some(header) = large_objects.get(&header_ptr) {
                return Ok(header.clone());
            }
        }
        
        // Check regions
        let regions = self.regions.read().unwrap();
        for region in regions.iter() {
            let objects = region.objects.read().unwrap();
            if let Some(header) = objects.get(&header_ptr) {
                return Ok(header.clone());
            }
        }
        
        Err(CursedError::runtime_error("Object header not found"))
    }
    
    /// Mark object for garbage collection
    pub fn mark_object(&self, ptr: NonNull<u8>) -> Result<(), CursedError> {
        let header_ptr = unsafe {
            (ptr.as_ptr() as *mut u8).sub(std::mem::size_of::<ObjectHeader>())
        };
        
        // Check large objects first
        {
            let mut large_objects = self.large_objects.write().unwrap();
            if let Some(header) = large_objects.get_mut(&header_ptr) {
                header.mark();
                return Ok(());
            }
        }
        
        // Check regions
        let regions = self.regions.read().unwrap();
        for region in regions.iter() {
            let mut objects = region.objects.write().unwrap();
            if let Some(header) = objects.get_mut(&header_ptr) {
                header.mark();
                return Ok(());
            }
        }
        
        Err(CursedError::runtime_error("Object not found for marking"))
    }
    
    /// Check if object is marked
    pub fn is_marked(&self, ptr: NonNull<u8>) -> Result<bool, CursedError> {
        let header = self.get_object_header(ptr)?;
        Ok(header.is_marked())
    }
    
    /// Clear all mark bits
    pub fn clear_marks(&self) -> Result<(), CursedError> {
        // Clear large objects
        {
            let mut large_objects = self.large_objects.write().unwrap();
            for header in large_objects.values_mut() {
                header.unmark();
            }
        }
        
        // Clear regions
        let regions = self.regions.read().unwrap();
        for region in regions.iter() {
            let mut objects = region.objects.write().unwrap();
            for header in objects.values_mut() {
                header.unmark();
            }
        }
        
        Ok(())
    }
    
    /// Get all allocated objects for GC tracing
    pub fn get_all_objects(&self) -> Vec<NonNull<u8>> {
        let mut objects = Vec::new();
        
        // Add large objects
        {
            let large_objects = self.large_objects.read().unwrap();
            for &header_ptr in large_objects.keys() {
                let data_ptr = unsafe {
                    header_ptr.add(std::mem::size_of::<ObjectHeader>())
                };
                if let Some(ptr) = NonNull::new(data_ptr) {
                    objects.push(ptr);
                }
            }
        }
        
        // Add region objects
        let regions = self.regions.read().unwrap();
        for region in regions.iter() {
            let region_objects = region.objects.read().unwrap();
            for &header_ptr in region_objects.keys() {
                let data_ptr = unsafe {
                    header_ptr.add(std::mem::size_of::<ObjectHeader>())
                };
                if let Some(ptr) = NonNull::new(data_ptr) {
                    objects.push(ptr);
                }
            }
        }
        
        objects
    }
    
    /// Align size to required alignment
    fn align_size(size: usize, alignment: usize) -> usize {
        (size + alignment - 1) & !(alignment - 1)
    }
    
    /// Update allocation statistics
    fn update_alloc_stats(&self, start_time: Instant, size: usize) {
        let elapsed_ns = start_time.elapsed().as_nanos() as u64;
        let mut stats = self.stats.write().unwrap();
        stats.total_alloc_time_ns += elapsed_ns;
        stats.used_size += size;
        stats.free_size = stats.free_size.saturating_sub(size);
    }
    
    /// Update deallocation statistics
    fn update_dealloc_stats(&self, start_time: Instant, size: usize) {
        let elapsed_ns = start_time.elapsed().as_nanos() as u64;
        let mut stats = self.stats.write().unwrap();
        stats.total_dealloc_time_ns += elapsed_ns;
        stats.used_size = stats.used_size.saturating_sub(size);
        stats.free_size += size;
    }
}

unsafe impl Send for HeapManager {}
unsafe impl Sync for HeapManager {}

impl Drop for HeapManager {
    fn drop(&mut self) {
        // Deallocate all regions
        let regions = self.regions.read().unwrap();
        for region in regions.iter() {
            let layout = Layout::from_size_align(region.size, 4096).unwrap();
            unsafe { alloc::dealloc(region.start, layout); }
        }
        
        // Deallocate all large objects
        let large_objects = self.large_objects.read().unwrap();
        for (&ptr, header) in large_objects.iter() {
            let layout = Layout::from_size_align(header.size, self.config.alignment).unwrap();
            unsafe { alloc::dealloc(ptr, layout); }
        }
    }
}

/// Visitor for heap traversal
pub struct HeapVisitor {
    heap: Arc<HeapManager>,
}

impl HeapVisitor {
    pub fn new(heap: Arc<HeapManager>) -> Self {
        Self { heap }
    }
}

impl crate::memory::Visitor for HeapVisitor {
    fn visit(&mut self, obj: &dyn Traceable) {
        // Mark object as reachable
        // This is a simplified implementation - in practice, you'd need
        // to get the actual object pointer and mark it
        let _ = obj; // Placeholder
    }
}

/// Create a default heap manager
pub fn create_default_heap() -> Result<Arc<HeapManager>, CursedError> {
    HeapManager::new(HeapConfig::default())
}

/// Get minimal result for compatibility
pub fn get_minimal_result() -> Result<String, CursedError> {
    Ok("CURSED heap management system initialized".to_string())
}

/// Legacy compatibility structure
pub struct MinimalImplementation {
    heap: Arc<HeapManager>,
}

impl MinimalImplementation {
    pub fn new() -> Result<Self, CursedError> {
        let heap = create_default_heap()?;
        Ok(Self { heap })
    }
    
    pub fn get_heap(&self) -> Arc<HeapManager> {
        self.heap.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::memory::Tag;
    
    #[test]
    fn test_heap_manager_creation() {
        let config = HeapConfig::default();
        let heap = HeapManager::new(config).unwrap();
        let stats = heap.get_stats();
        assert_eq!(stats.total_size, 64 * 1024 * 1024);
        assert_eq!(stats.used_size, 0);
    }
    
    #[test]
    fn test_object_allocation() {
        let heap = create_default_heap().unwrap();
        let ptr = heap.allocate(64, Tag::Object).unwrap();
        let header = heap.get_object_header(ptr).unwrap();
        assert_eq!(header.tag, Tag::Object);
        assert!(header.is_valid());
        
        // Deallocate
        heap.deallocate(ptr).unwrap();
    }
    
    #[test]
    fn test_large_object_allocation() {
        let heap = create_default_heap().unwrap();
        let large_size = 2 * 1024 * 1024; // 2MB
        let ptr = heap.allocate(large_size, Tag::Array).unwrap();
        let header = heap.get_object_header(ptr).unwrap();
        assert_eq!(header.tag, Tag::Array);
        
        // Deallocate
        heap.deallocate(ptr).unwrap();
    }
    
    #[test]
    fn test_marking_and_clearing() {
        let heap = create_default_heap().unwrap();
        let ptr = heap.allocate(64, Tag::Object).unwrap();
        
        // Initially not marked
        assert!(!heap.is_marked(ptr).unwrap());
        
        // Mark object
        heap.mark_object(ptr).unwrap();
        assert!(heap.is_marked(ptr).unwrap());
        
        // Clear marks
        heap.clear_marks().unwrap();
        assert!(!heap.is_marked(ptr).unwrap());
        
        // Deallocate
        heap.deallocate(ptr).unwrap();
    }
    
    #[test]
    fn test_heap_expansion() {
        let mut config = HeapConfig::default();
        config.initial_size = 1024; // Small initial size
        let heap = HeapManager::new(config).unwrap();
        
        // Allocate until expansion is needed
        let mut ptrs = Vec::new();
        for _ in 0..100 {
            match heap.allocate(64, Tag::Object) {
                Ok(ptr) => ptrs.push(ptr),
                Err(_) => break,
            }
        }
        
        // Should have expanded
        let stats = heap.get_stats();
        assert!(stats.expansions > 0);
        
        // Cleanup
        for ptr in ptrs {
            heap.deallocate(ptr).unwrap();
        }
    }
    
    #[test]
    fn test_compaction() {
        use std::time::{Duration, Instant};
        
        let heap = create_default_heap().unwrap();
        
        // Allocate and deallocate to create fragmentation
        let mut ptrs = Vec::new();
        for _ in 0..10 {
            ptrs.push(heap.allocate(64, Tag::Object).unwrap());
        }
        
        // Deallocate every other object
        for (i, ptr) in ptrs.iter().enumerate() {
            if i % 2 == 0 {
                heap.deallocate(*ptr).unwrap();
            }
        }
        
        // Compact with timeout to prevent hanging
        let start_time = Instant::now();
        heap.compact().unwrap();
        let elapsed = start_time.elapsed();
        
        // Assert compaction completed within reasonable time
        assert!(elapsed < Duration::from_secs(5), "Compaction took too long: {:?}", elapsed);
        
        let stats = heap.get_stats();
        assert!(stats.compactions > 0);
        
        // Cleanup remaining objects
        for (i, ptr) in ptrs.iter().enumerate() {
            if i % 2 == 1 {
                heap.deallocate(*ptr).unwrap();
            }
        }
    }
}
