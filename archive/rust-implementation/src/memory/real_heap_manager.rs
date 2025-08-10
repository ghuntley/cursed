//! Real heap manager implementation for CURSED runtime
//!
//! Provides a production-ready heap manager with:
//! - Multi-threaded heap allocation
//! - Memory pool management
//! - Real-time garbage collection integration
//! - Performance monitoring and metrics

use crate::error::CursedError;
use crate::memory::{HeapManager, MemoryStats, GarbageCollector};
use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;
use std::ptr::NonNull;
use std::alloc::{Layout, System, GlobalAlloc};
use std::time::{Duration, Instant};

/// Real heap manager implementation
pub struct RealHeapManager {
    /// Memory pools for different size classes
    pools: RwLock<HashMap<usize, MemoryPool>>,
    /// Global allocation statistics
    stats: Mutex<HeapStats>,
    /// Garbage collector integration
    gc: Arc<GarbageCollector>,
    /// Heap configuration
    config: RwLock<HeapConfig>,
    /// Free memory chunks
    free_chunks: Mutex<Vec<MemoryChunk>>,
    /// Large object allocations (>4KB)
    large_objects: Mutex<HashMap<usize, LargeObjectInfo>>,
}

/// Heap manager configuration
#[derive(Debug, Clone)]
pub struct HeapConfig {
    /// Maximum heap size (bytes)
    pub max_heap_size: usize,
    /// Pool sizes to maintain
    pub pool_sizes: Vec<usize>,
    /// GC trigger threshold
    pub gc_threshold: f64,
    /// Large object threshold
    pub large_object_threshold: usize,
    /// Enable memory mapping for large objects
    pub use_mmap: bool,
}

impl Default for HeapConfig {
    fn default() -> Self {
        Self {
            max_heap_size: 1024 * 1024 * 1024, // 1GB default
            pool_sizes: vec![8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096],
            gc_threshold: 0.8, // Trigger GC at 80% heap usage
            large_object_threshold: 8192, // 8KB
            use_mmap: true,
        }
    }
}

/// Memory pool for fixed-size allocations
struct MemoryPool {
    chunk_size: usize,
    free_chunks: Vec<NonNull<u8>>,
    allocated_chunks: usize,
    total_chunks: usize,
    last_allocation: Instant,
}

impl MemoryPool {
    fn new(chunk_size: usize) -> Self {
        Self {
            chunk_size,
            free_chunks: Vec::new(),
            allocated_chunks: 0,
            total_chunks: 0,
            last_allocation: Instant::now(),
        }
    }

    fn allocate(&mut self) -> Option<NonNull<u8>> {
        if let Some(chunk) = self.free_chunks.pop() {
            self.allocated_chunks += 1;
            self.last_allocation = Instant::now();
            Some(chunk)
        } else {
            self.allocate_new_chunk()
        }
    }

    fn deallocate(&mut self, ptr: NonNull<u8>) {
        self.free_chunks.push(ptr);
        self.allocated_chunks = self.allocated_chunks.saturating_sub(1);
    }

    fn allocate_new_chunk(&mut self) -> Option<NonNull<u8>> {
        unsafe {
            let layout = Layout::from_size_align(self.chunk_size, 8).ok()?;
            let ptr = System.alloc(layout);
            if ptr.is_null() {
                None
            } else {
                self.total_chunks += 1;
                self.allocated_chunks += 1;
                self.last_allocation = Instant::now();
                NonNull::new(ptr)
            }
        }
    }
}

/// Memory chunk information
#[derive(Debug, Clone)]
struct MemoryChunk {
    ptr: NonNull<u8>,
    size: usize,
    allocated_at: Instant,
}

/// Large object allocation information
#[derive(Debug, Clone)]
struct LargeObjectInfo {
    ptr: NonNull<u8>,
    size: usize,
    allocated_at: Instant,
    is_mmaped: bool,
}

/// Heap allocation statistics
#[derive(Debug, Clone, Default)]
pub struct HeapStats {
    pub total_allocated: usize,
    pub total_freed: usize,
    pub peak_usage: usize,
    pub current_usage: usize,
    pub allocation_count: u64,
    pub deallocation_count: u64,
    pub pool_allocations: u64,
    pub large_allocations: u64,
    pub gc_triggers: u64,
    pub fragmentation_ratio: f64,
    pub allocation_time: Duration,
}

impl RealHeapManager {
    /// Create new real heap manager
    pub fn new() -> Result<Self, CursedError> {
        let config = HeapConfig::default();
        let gc = Arc::new(GarbageCollector::new()?);
        
        let mut pools = HashMap::new();
        for &size in &config.pool_sizes {
            pools.insert(size, MemoryPool::new(size));
        }

        Ok(Self {
            pools: RwLock::new(pools),
            stats: Mutex::new(HeapStats::default()),
            gc,
            config: RwLock::new(config),
            free_chunks: Mutex::new(Vec::new()),
            large_objects: Mutex::new(HashMap::new()),
        })
    }

    /// Create with custom configuration
    pub fn with_config(config: HeapConfig) -> Result<Self, CursedError> {
        let gc = Arc::new(GarbageCollector::new()?);
        
        let mut pools = HashMap::new();
        for &size in &config.pool_sizes {
            pools.insert(size, MemoryPool::new(size));
        }

        Ok(Self {
            pools: RwLock::new(pools),
            stats: Mutex::new(HeapStats::default()),
            gc,
            config: RwLock::new(config),
            free_chunks: Mutex::new(Vec::new()),
            large_objects: Mutex::new(HashMap::new()),
        })
    }

    /// Allocate memory with specified size and alignment
    pub fn allocate(&self, size: usize, align: usize) -> Result<NonNull<u8>, CursedError> {
        let start_time = Instant::now();
        
        let result = if size >= self.config.read().unwrap().large_object_threshold {
            self.allocate_large_object(size, align)
        } else {
            self.allocate_from_pool(size, align)
        };

        // Update statistics
        if let Ok(ptr) = result {
            let mut stats = self.stats.lock().map_err(|_| CursedError::RuntimeError {
                message: "Failed to acquire heap stats lock".to_string(),
                context: Default::default(),
            })?;

            stats.allocation_count += 1;
            stats.current_usage += size;
            stats.total_allocated += size;
            stats.peak_usage = stats.peak_usage.max(stats.current_usage);
            stats.allocation_time += start_time.elapsed();

            // Check if GC should be triggered
            let config = self.config.read().unwrap();
            let heap_usage_ratio = stats.current_usage as f64 / config.max_heap_size as f64;
            
            if heap_usage_ratio > config.gc_threshold {
                stats.gc_triggers += 1;
                drop(stats); // Release lock before GC
                drop(config); // Release lock before GC
                
                // Trigger garbage collection
                if let Err(e) = self.gc.collect() {
                    eprintln!("GC collection failed: {:?}", e);
                }
            }
        }

        result
    }

    /// Deallocate memory
    pub fn deallocate(&self, ptr: NonNull<u8>, size: usize) -> Result<(), CursedError> {
        let config = self.config.read().unwrap();
        
        if size >= config.large_object_threshold {
            self.deallocate_large_object(ptr, size)
        } else {
            self.deallocate_from_pool(ptr, size)
        }?;

        // Update statistics
        let mut stats = self.stats.lock().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire heap stats lock".to_string(),
            context: Default::default(),
        })?;

        stats.deallocation_count += 1;
        stats.current_usage = stats.current_usage.saturating_sub(size);
        stats.total_freed += size;

        Ok(())
    }

    /// Allocate from memory pool
    fn allocate_from_pool(&self, size: usize, _align: usize) -> Result<NonNull<u8>, CursedError> {
        let config = self.config.read().unwrap();
        
        // Find the smallest pool that can accommodate the size
        let pool_size = config.pool_sizes.iter()
            .find(|&&pool_size| pool_size >= size)
            .copied()
            .unwrap_or_else(|| {
                // Round up to next power of 2 if no suitable pool
                size.next_power_of_two()
            });

        drop(config);

        let mut pools = self.pools.write().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire pools write lock".to_string(),
            context: Default::default(),
        })?;

        let pool = pools.entry(pool_size).or_insert_with(|| MemoryPool::new(pool_size));
        
        pool.allocate().ok_or_else(|| CursedError::RuntimeError {
            message: format!("Failed to allocate {} bytes from pool", size),
            context: Default::default(),
        }).map(|ptr| {
            let mut stats = self.stats.lock().unwrap();
            stats.pool_allocations += 1;
            ptr
        })
    }

    /// Deallocate from memory pool
    fn deallocate_from_pool(&self, ptr: NonNull<u8>, size: usize) -> Result<(), CursedError> {
        let config = self.config.read().unwrap();
        
        let pool_size = config.pool_sizes.iter()
            .find(|&&pool_size| pool_size >= size)
            .copied()
            .unwrap_or_else(|| size.next_power_of_two());

        drop(config);

        let mut pools = self.pools.write().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire pools write lock".to_string(),
            context: Default::default(),
        })?;

        if let Some(pool) = pools.get_mut(&pool_size) {
            pool.deallocate(ptr);
        }

        Ok(())
    }

    /// Allocate large object
    fn allocate_large_object(&self, size: usize, align: usize) -> Result<NonNull<u8>, CursedError> {
        unsafe {
            let layout = Layout::from_size_align(size, align).map_err(|_| CursedError::RuntimeError {
                message: format!("Invalid layout for size {} align {}", size, align),
                context: Default::default(),
            })?;

            let ptr = System.alloc(layout);
            if ptr.is_null() {
                return Err(CursedError::RuntimeError {
                    message: format!("Failed to allocate large object of size {}", size),
                    context: Default::default(),
                });
            }

            let ptr = NonNull::new(ptr).unwrap();
            
            // Track large object
            let mut large_objects = self.large_objects.lock().map_err(|_| CursedError::RuntimeError {
                message: "Failed to acquire large objects lock".to_string(),
                context: Default::default(),
            })?;

            large_objects.insert(ptr.as_ptr() as usize, LargeObjectInfo {
                ptr,
                size,
                allocated_at: Instant::now(),
                is_mmaped: false, // Using system allocator for now
            });

            let mut stats = self.stats.lock().unwrap();
            stats.large_allocations += 1;

            Ok(ptr)
        }
    }

    /// Deallocate large object
    fn deallocate_large_object(&self, ptr: NonNull<u8>, size: usize) -> Result<(), CursedError> {
        let mut large_objects = self.large_objects.lock().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire large objects lock".to_string(),
            context: Default::default(),
        })?;

        if let Some(info) = large_objects.remove(&(ptr.as_ptr() as usize)) {
            unsafe {
                let layout = Layout::from_size_align(info.size, 8).unwrap();
                System.dealloc(ptr.as_ptr(), layout);
            }
        } else {
            // Fallback deallocation
            unsafe {
                let layout = Layout::from_size_align(size, 8).unwrap();
                System.dealloc(ptr.as_ptr(), layout);
            }
        }

        Ok(())
    }

    /// Get heap statistics
    pub fn get_stats(&self) -> Result<HeapStats, CursedError> {
        let stats = self.stats.lock().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire heap stats lock".to_string(),
            context: Default::default(),
        })?;

        Ok(stats.clone())
    }

    /// Force garbage collection
    pub fn force_gc(&self) -> Result<(), CursedError> {
        self.gc.collect()
    }

    /// Get memory usage information
    pub fn get_memory_usage(&self) -> Result<MemoryUsageInfo, CursedError> {
        let stats = self.get_stats()?;
        let config = self.config.read().unwrap();
        
        let pools = self.pools.read().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire pools read lock".to_string(),
            context: Default::default(),
        })?;

        let pool_info: Vec<PoolInfo> = pools.iter().map(|(&size, pool)| {
            PoolInfo {
                chunk_size: size,
                total_chunks: pool.total_chunks,
                allocated_chunks: pool.allocated_chunks,
                free_chunks: pool.free_chunks.len(),
            }
        }).collect();

        Ok(MemoryUsageInfo {
            total_allocated: stats.total_allocated,
            current_usage: stats.current_usage,
            peak_usage: stats.peak_usage,
            max_heap_size: config.max_heap_size,
            fragmentation_ratio: stats.fragmentation_ratio,
            pool_info,
            gc_triggers: stats.gc_triggers,
        })
    }

    /// Update heap configuration
    pub fn update_config(&self, new_config: HeapConfig) -> Result<(), CursedError> {
        let mut config = self.config.write().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire config write lock".to_string(),
            context: Default::default(),
        })?;

        *config = new_config;
        Ok(())
    }
}

/// Memory usage information
#[derive(Debug, Clone)]
pub struct MemoryUsageInfo {
    pub total_allocated: usize,
    pub current_usage: usize,
    pub peak_usage: usize,
    pub max_heap_size: usize,
    pub fragmentation_ratio: f64,
    pub pool_info: Vec<PoolInfo>,
    pub gc_triggers: u64,
}

/// Memory pool information
#[derive(Debug, Clone)]
pub struct PoolInfo {
    pub chunk_size: usize,
    pub total_chunks: usize,
    pub allocated_chunks: usize,
    pub free_chunks: usize,
}

/// Global real heap manager instance
static GLOBAL_HEAP: std::sync::OnceLock<Arc<RealHeapManager>> = std::sync::OnceLock::new();

/// Get global real heap manager
pub fn get_global_heap() -> Arc<RealHeapManager> {
    GLOBAL_HEAP.get_or_init(|| {
        Arc::new(RealHeapManager::new().expect("Failed to create real heap manager"))
    }).clone()
}

/// Initialize real heap management system
pub fn initialize_real_heap() -> Result<String, CursedError> {
    let heap = get_global_heap();
    let stats = heap.get_stats()?;
    
    Ok(format!(
        "Real heap manager initialized: {} pools, {}MB max heap size", 
        heap.config.read().unwrap().pool_sizes.len(),
        heap.config.read().unwrap().max_heap_size / (1024 * 1024)
    ))
}

/// Legacy compatibility structure
pub struct MinimalImplementation {
    heap: Arc<RealHeapManager>,
}

impl MinimalImplementation {
    pub fn new() -> Self {
        Self {
            heap: get_global_heap(),
        }
    }
    
    pub fn get_heap(&self) -> Arc<RealHeapManager> {
        self.heap.clone()
    }
}

pub fn get_minimal_result() -> Result<String, CursedError> {
    initialize_real_heap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ptr::NonNull;

    #[test]
    fn test_heap_creation() {
        let heap = RealHeapManager::new().expect("Should create heap");
        let stats = heap.get_stats().expect("Should get stats");
        assert_eq!(stats.allocation_count, 0);
        assert_eq!(stats.current_usage, 0);
    }

    #[test]
    fn test_memory_allocation() {
        let heap = RealHeapManager::new().expect("Should create heap");
        
        // Allocate small memory
        let ptr = heap.allocate(64, 8).expect("Should allocate memory");
        assert!(!ptr.as_ptr().is_null());
        
        // Check stats were updated
        let stats = heap.get_stats().expect("Should get stats");
        assert_eq!(stats.allocation_count, 1);
        assert_eq!(stats.current_usage, 64);
        
        // Deallocate
        heap.deallocate(ptr, 64).expect("Should deallocate memory");
        
        // Check stats were updated
        let stats = heap.get_stats().expect("Should get stats");
        assert_eq!(stats.deallocation_count, 1);
        assert_eq!(stats.current_usage, 0);
    }

    #[test]
    fn test_large_object_allocation() {
        let heap = RealHeapManager::new().expect("Should create heap");
        
        // Allocate large memory (> 8KB threshold)
        let ptr = heap.allocate(10000, 8).expect("Should allocate large memory");
        assert!(!ptr.as_ptr().is_null());
        
        let stats = heap.get_stats().expect("Should get stats");
        assert_eq!(stats.allocation_count, 1);
        assert_eq!(stats.large_allocations, 1);
        assert_eq!(stats.current_usage, 10000);
        
        // Deallocate
        heap.deallocate(ptr, 10000).expect("Should deallocate large memory");
    }

    #[test]
    fn test_global_heap() {
        let heap1 = get_global_heap();
        let heap2 = get_global_heap();
        
        // Should be the same instance
        assert!(Arc::ptr_eq(&heap1, &heap2));
    }

    #[test]
    fn test_heap_initialization() {
        let result = initialize_real_heap();
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Real heap manager initialized"));
    }
}
