//! Memory optimization for CURSED runtime
//! 
//! Provides advanced memory optimization techniques including:
//! - Memory layout optimization
//! - Cache-aware data structure management
//! - Memory pool allocation
//! - Fragmentation reduction

use crate::error::CursedError;
use crate::memory::{HeapManager, MemoryStats};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use std::time::{Duration, Instant};

/// Memory optimization engine
pub struct MemoryOptimizer {
    /// Optimization configuration
    config: Mutex<OptimizationConfig>,
    /// Memory usage statistics
    stats: Mutex<OptimizationStats>,
    /// Object size tracking for pool allocation
    size_tracker: Mutex<HashMap<usize, ObjectSizeStats>>,
    /// Cache-aware allocations
    cache_allocator: Mutex<CacheAwareAllocator>,
}

/// Memory optimization configuration
#[derive(Debug, Clone)]
pub struct OptimizationConfig {
    /// Enable memory layout optimization
    pub layout_optimization: bool,
    /// Enable cache-aware allocation
    pub cache_aware_allocation: bool,
    /// Enable memory pooling
    pub memory_pooling: bool,
    /// Cache line size for alignment
    pub cache_line_size: usize,
    /// Memory pool sizes to maintain
    pub pool_sizes: Vec<usize>,
    /// Maximum memory fragmentation allowed
    pub max_fragmentation: f64,
}

impl Default for OptimizationConfig {
    fn default() -> Self {
        Self {
            layout_optimization: true,
            cache_aware_allocation: true,
            memory_pooling: true,
            cache_line_size: 64, // Common cache line size
            pool_sizes: vec![8, 16, 32, 64, 128, 256, 512, 1024, 2048, 4096],
            max_fragmentation: 0.15, // 15% max fragmentation
        }
    }
}

/// Memory optimization statistics
#[derive(Debug, Clone, Default)]
pub struct OptimizationStats {
    pub cache_hits: u64,
    pub cache_misses: u64,
    pub pool_allocations: u64,
    pub direct_allocations: u64,
    pub layout_optimizations: u64,
    pub fragmentation_reductions: u64,
    pub bytes_saved: u64,
    pub optimization_time: Duration,
}

/// Object size statistics for pool management
#[derive(Debug, Clone, Default)]
struct ObjectSizeStats {
    allocations: u64,
    deallocations: u64,
    total_size: u64,
    last_access: Instant,
}

/// Cache-aware allocator for optimal memory layout
struct CacheAwareAllocator {
    cache_line_size: usize,
    aligned_allocations: u64,
    total_allocations: u64,
}

impl Default for CacheAwareAllocator {
    fn default() -> Self {
        Self {
            cache_line_size: 64,
            aligned_allocations: 0,
            total_allocations: 0,
        }
    }
}

impl MemoryOptimizer {
    /// Create new memory optimizer
    pub fn new() -> Self {
        Self {
            config: Mutex::new(OptimizationConfig::default()),
            stats: Mutex::new(OptimizationStats::default()),
            size_tracker: Mutex::new(HashMap::new()),
            cache_allocator: Mutex::new(CacheAwareAllocator::default()),
        }
    }

    /// Create with custom configuration
    pub fn with_config(config: OptimizationConfig) -> Self {
        Self {
            config: Mutex::new(config),
            stats: Mutex::new(OptimizationStats::default()),
            size_tracker: Mutex::new(HashMap::new()),
            cache_allocator: Mutex::new(CacheAwareAllocator::default()),
        }
    }

    /// Optimize allocation for given size
    pub fn optimize_allocation(&self, size: usize) -> Result<AllocationStrategy, CursedError> {
        let config = self.config.lock().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire optimization config lock".to_string(),
            context: Default::default(),
        })?;

        let mut stats = self.stats.lock().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire optimization stats lock".to_string(),
            context: Default::default(),
        })?;

        // Check if size fits in memory pools
        if config.memory_pooling {
            for &pool_size in &config.pool_sizes {
                if size <= pool_size {
                    stats.pool_allocations += 1;
                    return Ok(AllocationStrategy::Pool(pool_size));
                }
            }
        }

        // Check for cache-aware allocation
        if config.cache_aware_allocation {
            let aligned_size = self.align_to_cache_line(size, config.cache_line_size);
            stats.direct_allocations += 1;
            return Ok(AllocationStrategy::CacheAligned(aligned_size));
        }

        stats.direct_allocations += 1;
        Ok(AllocationStrategy::Direct(size))
    }

    /// Track object size usage for pool optimization
    pub fn track_allocation(&self, size: usize) -> Result<(), CursedError> {
        let mut tracker = self.size_tracker.lock().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire size tracker lock".to_string(),
            context: Default::default(),
        })?;

        let stats = tracker.entry(size).or_insert_with(|| ObjectSizeStats {
            allocations: 0,
            deallocations: 0,
            total_size: 0,
            last_access: Instant::now(),
        });

        stats.allocations += 1;
        stats.total_size += size as u64;
        stats.last_access = Instant::now();

        Ok(())
    }

    /// Track object deallocation
    pub fn track_deallocation(&self, size: usize) -> Result<(), CursedError> {
        let mut tracker = self.size_tracker.lock().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire size tracker lock".to_string(),
            context: Default::default(),
        })?;

        if let Some(stats) = tracker.get_mut(&size) {
            stats.deallocations += 1;
            stats.last_access = Instant::now();
        }

        Ok(())
    }

    /// Analyze memory fragmentation and suggest optimizations
    pub fn analyze_fragmentation(&self, memory_stats: &MemoryStats) -> Result<FragmentationAnalysis, CursedError> {
        let config = self.config.lock().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire optimization config lock".to_string(),
            context: Default::default(),
        })?;

        let total_allocated = memory_stats.total_allocated;
        let total_free = memory_stats.total_free;
        let largest_free_block = memory_stats.largest_free_block;

        let fragmentation_ratio = if total_free > 0 {
            1.0 - (largest_free_block as f64 / total_free as f64)
        } else {
            0.0
        };

        let needs_optimization = fragmentation_ratio > config.max_fragmentation;

        let mut recommendations = Vec::new();
        
        if needs_optimization {
            recommendations.push("Consider memory compaction".to_string());
            
            if fragmentation_ratio > 0.3 {
                recommendations.push("Enable aggressive memory pooling".to_string());
            }
            
            if memory_stats.allocation_count > 10000 {
                recommendations.push("Increase pool sizes for frequently allocated objects".to_string());
            }
        }

        Ok(FragmentationAnalysis {
            fragmentation_ratio,
            needs_optimization,
            recommendations,
            total_allocated,
            total_free,
            largest_free_block,
        })
    }

    /// Optimize object layout for cache efficiency
    pub fn optimize_object_layout(&self, object_size: usize, field_sizes: &[usize]) -> Result<LayoutOptimization, CursedError> {
        let config = self.config.lock().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire optimization config lock".to_string(),
            context: Default::default(),
        })?;

        if !config.layout_optimization {
            return Ok(LayoutOptimization {
                original_size: object_size,
                optimized_size: object_size,
                field_order: (0..field_sizes.len()).collect(),
                cache_lines_used: (object_size + config.cache_line_size - 1) / config.cache_line_size,
                padding_bytes: 0,
            });
        }

        // Sort fields by size (largest first) for better packing
        let mut field_indices: Vec<usize> = (0..field_sizes.len()).collect();
        field_indices.sort_by(|&a, &b| field_sizes[b].cmp(&field_sizes[a]));

        // Calculate optimized layout
        let mut current_offset = 0;
        let mut total_padding = 0;

        for &field_idx in &field_indices {
            let field_size = field_sizes[field_idx];
            let alignment = field_size.min(8); // Assume max 8-byte alignment
            
            let aligned_offset = (current_offset + alignment - 1) & !(alignment - 1);
            total_padding += aligned_offset - current_offset;
            current_offset = aligned_offset + field_size;
        }

        let optimized_size = current_offset;
        let cache_lines_used = (optimized_size + config.cache_line_size - 1) / config.cache_line_size;

        let mut stats = self.stats.lock().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire optimization stats lock".to_string(),
            context: Default::default(),
        })?;

        stats.layout_optimizations += 1;
        if optimized_size < object_size {
            stats.bytes_saved += (object_size - optimized_size) as u64;
        }

        Ok(LayoutOptimization {
            original_size: object_size,
            optimized_size,
            field_order: field_indices,
            cache_lines_used,
            padding_bytes: total_padding,
        })
    }

    /// Get optimization statistics
    pub fn get_stats(&self) -> Result<OptimizationStats, CursedError> {
        let stats = self.stats.lock().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire optimization stats lock".to_string(),
            context: Default::default(),
        })?;

        Ok(stats.clone())
    }

    /// Update optimization configuration
    pub fn update_config(&self, new_config: OptimizationConfig) -> Result<(), CursedError> {
        let mut config = self.config.lock().map_err(|_| CursedError::RuntimeError {
            message: "Failed to acquire optimization config lock".to_string(),
            context: Default::default(),
        })?;

        *config = new_config;
        Ok(())
    }

    /// Helper function to align size to cache line boundary
    fn align_to_cache_line(&self, size: usize, cache_line_size: usize) -> usize {
        (size + cache_line_size - 1) & !(cache_line_size - 1)
    }
}

/// Allocation strategy recommendation
#[derive(Debug, Clone)]
pub enum AllocationStrategy {
    /// Use memory pool with specified size
    Pool(usize),
    /// Use cache-aligned allocation with specified size
    CacheAligned(usize),
    /// Use direct allocation with specified size
    Direct(usize),
}

/// Memory fragmentation analysis
#[derive(Debug, Clone)]
pub struct FragmentationAnalysis {
    pub fragmentation_ratio: f64,
    pub needs_optimization: bool,
    pub recommendations: Vec<String>,
    pub total_allocated: usize,
    pub total_free: usize,
    pub largest_free_block: usize,
}

/// Object layout optimization result
#[derive(Debug, Clone)]
pub struct LayoutOptimization {
    pub original_size: usize,
    pub optimized_size: usize,
    pub field_order: Vec<usize>,
    pub cache_lines_used: usize,
    pub padding_bytes: usize,
}

/// Global memory optimizer instance
static GLOBAL_OPTIMIZER: std::sync::OnceLock<Arc<MemoryOptimizer>> = std::sync::OnceLock::new();

/// Get global memory optimizer
pub fn get_global_optimizer() -> Arc<MemoryOptimizer> {
    GLOBAL_OPTIMIZER.get_or_init(|| {
        Arc::new(MemoryOptimizer::new())
    }).clone()
}

/// Initialize memory optimization system
pub fn initialize_memory_optimization() -> Result<String, CursedError> {
    let optimizer = get_global_optimizer();
    
    // Set up default optimization pools
    let config = OptimizationConfig::default();
    optimizer.update_config(config)?;
    
    Ok("Memory optimization system initialized with cache-aware allocation and memory pools".to_string())
}

/// Legacy compatibility structure
pub struct MinimalImplementation {
    optimizer: Arc<MemoryOptimizer>,
}

impl MinimalImplementation {
    pub fn new() -> Self {
        Self {
            optimizer: get_global_optimizer(),
        }
    }
    
    pub fn get_optimizer(&self) -> Arc<MemoryOptimizer> {
        self.optimizer.clone()
    }
}

pub fn get_minimal_result() -> Result<String, CursedError> {
    initialize_memory_optimization()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_optimizer_creation() {
        let optimizer = MemoryOptimizer::new();
        let stats = optimizer.get_stats().expect("Should get stats");
        assert_eq!(stats.cache_hits, 0);
        assert_eq!(stats.pool_allocations, 0);
    }

    #[test]
    fn test_allocation_optimization() {
        let optimizer = MemoryOptimizer::new();
        
        // Test pool allocation for small size
        let strategy = optimizer.optimize_allocation(64).expect("Should optimize allocation");
        match strategy {
            AllocationStrategy::Pool(size) => assert!(size >= 64),
            _ => panic!("Expected pool allocation for small size"),
        }
        
        // Test direct allocation for very large size
        let strategy = optimizer.optimize_allocation(100000).expect("Should optimize allocation");
        match strategy {
            AllocationStrategy::CacheAligned(size) => assert!(size >= 100000),
            AllocationStrategy::Direct(size) => assert_eq!(size, 100000),
            _ => {},
        }
    }

    #[test]
    fn test_allocation_tracking() {
        let optimizer = MemoryOptimizer::new();
        
        // Track some allocations
        optimizer.track_allocation(64).expect("Should track allocation");
        optimizer.track_allocation(128).expect("Should track allocation");
        optimizer.track_deallocation(64).expect("Should track deallocation");
        
        // The internal tracking should work without errors
        let stats = optimizer.get_stats().expect("Should get stats");
        assert_eq!(stats.cache_hits, 0); // No cache operations yet
    }

    #[test]
    fn test_object_layout_optimization() {
        let optimizer = MemoryOptimizer::new();
        let field_sizes = vec![8, 4, 16, 8];
        
        let result = optimizer.optimize_object_layout(40, &field_sizes).expect("Should optimize layout");
        
        // Check that optimization was attempted
        assert!(result.optimized_size > 0);
        assert_eq!(result.field_order.len(), field_sizes.len());
        assert!(result.cache_lines_used > 0);
    }

    #[test]
    fn test_global_optimizer() {
        let optimizer1 = get_global_optimizer();
        let optimizer2 = get_global_optimizer();
        
        // Should be the same instance
        assert!(Arc::ptr_eq(&optimizer1, &optimizer2));
    }

    #[test]
    fn test_initialization() {
        let result = initialize_memory_optimization();
        assert!(result.is_ok());
        assert!(result.unwrap().contains("Memory optimization system initialized"));
    }
}
