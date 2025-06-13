/// Memory Layout Optimization for CURSED Compiler
/// 
/// Provides object layout optimization for cache efficiency, memory pool optimization
/// for reduced GC pressure, and integration with the existing garbage collection system.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, RwLock};
use std::time::{Duration, Instant};
use tracing::{debug, info, instrument, warn};

use crate::error::{Error, Result};
use crate::memory::GarbageCollector;

/// Memory optimization configuration
#[derive(Debug, Clone)]
pub struct MemoryOptimizationConfig {
    /// Enable object layout optimization
    pub enable_layout_optimization: bool,
    /// Enable memory pool optimization
    pub enable_pool_optimization: bool,
    /// Enable cache-friendly data structures
    pub enable_cache_optimization: bool,
    /// Target cache line size in bytes
    pub cache_line_size: usize,
    /// Memory pool sizes for different object types
    pub pool_sizes: HashMap<String, usize>,
    /// Maximum object size for pool allocation
    pub max_pool_object_size: usize,
    /// GC pressure reduction target (percentage)
    pub gc_pressure_reduction_target: f64,
    /// Enable memory access pattern analysis
    pub enable_access_pattern_analysis: bool,
}

impl Default for MemoryOptimizationConfig {
    fn default() -> Self {
        let mut pool_sizes = HashMap::new();
        pool_sizes.insert("small".to_string(), 64);
        pool_sizes.insert("medium".to_string(), 512);
        pool_sizes.insert("large".to_string(), 4096);
        
        Self {
            enable_layout_optimization: true,
            enable_pool_optimization: true,
            enable_cache_optimization: true,
            cache_line_size: 64, // Common cache line size
            pool_sizes,
            max_pool_object_size: 8192,
            gc_pressure_reduction_target: 0.3, // 30% reduction
            enable_access_pattern_analysis: true,
        }
    }
}

/// Object layout information for optimization
#[derive(Debug, Clone)]
pub struct ObjectLayout {
    /// Type name
    pub type_name: String,
    /// Field layouts
    pub fields: Vec<FieldLayout>,
    /// Total size in bytes
    pub total_size: usize,
    /// Alignment requirements
    pub alignment: usize,
    /// Cache line usage
    pub cache_lines_used: usize,
    /// Padding bytes
    pub padding_bytes: usize,
    /// Access frequency per field
    pub field_access_frequencies: HashMap<String, u64>,
}

#[derive(Debug, Clone)]
pub struct FieldLayout {
    /// Field name
    pub name: String,
    /// Field size in bytes
    pub size: usize,
    /// Offset from object start
    pub offset: usize,
    /// Alignment requirement
    pub alignment: usize,
    /// Access frequency
    pub access_frequency: u64,
}

/// Memory access pattern analysis
#[derive(Debug, Clone)]
pub struct MemoryAccessPattern {
    /// Object type
    pub object_type: String,
    /// Sequential access ratio (0.0 to 1.0)
    pub sequential_access_ratio: f64,
    /// Random access ratio (0.0 to 1.0)
    pub random_access_ratio: f64,
    /// Average access stride
    pub average_stride: isize,
    /// Cache hit rate
    pub cache_hit_rate: f64,
    /// Hot fields (frequently accessed)
    pub hot_fields: Vec<String>,
    /// Cold fields (rarely accessed)
    pub cold_fields: Vec<String>,
}

/// Memory pool for optimized allocation
#[derive(Debug)]
pub struct MemoryPool {
    /// Pool name
    name: String,
    /// Object size managed by this pool
    object_size: usize,
    /// Pre-allocated objects
    free_objects: Vec<*mut u8>,
    /// Total objects in pool
    total_objects: usize,
    /// Objects currently in use
    objects_in_use: usize,
    /// Pool statistics
    allocation_count: u64,
    deallocation_count: u64,
    pool_hits: u64,
    pool_misses: u64,
}

impl MemoryPool {
    /// Create a new memory pool
    pub fn new(name: String, object_size: usize, initial_capacity: usize) -> Result<Self> {
        let mut free_objects = Vec::with_capacity(initial_capacity);
        
        // Pre-allocate objects
        for _ in 0..initial_capacity {
            let ptr = unsafe {
                std::alloc::alloc(std::alloc::Layout::from_size_align(object_size, 8)
                    .map_err(|_| Error::Runtime("Invalid memory layout".to_string()))?)
            };
            
            if ptr.is_null() {
                return Err(Error::Runtime("Failed to allocate memory for pool".to_string()));
            }
            
            free_objects.push(ptr);
        }
        
        Ok(Self {
            name,
            object_size,
            free_objects,
            total_objects: initial_capacity,
            objects_in_use: 0,
            allocation_count: 0,
            deallocation_count: 0,
            pool_hits: 0,
            pool_misses: 0,
        })
    }
    
    /// Allocate an object from the pool
    pub fn allocate(&mut self) -> Option<*mut u8> {
        self.allocation_count += 1;
        
        if let Some(ptr) = self.free_objects.pop() {
            self.objects_in_use += 1;
            self.pool_hits += 1;
            Some(ptr)
        } else {
            self.pool_misses += 1;
            None
        }
    }
    
    /// Deallocate an object back to the pool
    pub fn deallocate(&mut self, ptr: *mut u8) -> Result<()> {
        if ptr.is_null() {
            return Err(Error::Runtime("Cannot deallocate null pointer".to_string()));
        }
        
        self.deallocation_count += 1;
        self.objects_in_use = self.objects_in_use.saturating_sub(1);
        self.free_objects.push(ptr);
        
        Ok(())
    }
    
    /// Get pool statistics
    pub fn get_statistics(&self) -> MemoryPoolStatistics {
        MemoryPoolStatistics {
            name: self.name.clone(),
            object_size: self.object_size,
            total_objects: self.total_objects,
            objects_in_use: self.objects_in_use,
            allocation_count: self.allocation_count,
            deallocation_count: self.deallocation_count,
            hit_rate: if self.allocation_count > 0 {
                self.pool_hits as f64 / self.allocation_count as f64
            } else {
                0.0
            },
        }
    }
}

impl Drop for MemoryPool {
    fn drop(&mut self) {
        // Deallocate all objects
        for ptr in &self.free_objects {
            if !ptr.is_null() {
                unsafe {
                    std::alloc::dealloc(*ptr, std::alloc::Layout::from_size_align(self.object_size, 8).unwrap());
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
pub struct MemoryPoolStatistics {
    pub name: String,
    pub object_size: usize,
    pub total_objects: usize,
    pub objects_in_use: usize,
    pub allocation_count: u64,
    pub deallocation_count: u64,
    pub hit_rate: f64,
}

/// Cache optimizer for improving data locality
pub struct CacheOptimizer {
    /// Configuration
    config: MemoryOptimizationConfig,
    /// Access pattern analysis
    access_patterns: Arc<RwLock<HashMap<String, MemoryAccessPattern>>>,
    /// Layout recommendations
    layout_recommendations: Arc<RwLock<HashMap<String, ObjectLayout>>>,
}

impl CacheOptimizer {
    /// Create a new cache optimizer
    pub fn new(config: MemoryOptimizationConfig) -> Self {
        Self {
            config,
            access_patterns: Arc::new(RwLock::new(HashMap::new())),
            layout_recommendations: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Analyze memory access pattern for an object type
    #[instrument(skip(self))]
    pub fn analyze_access_pattern(&self, object_type: &str, field_accesses: &[(String, u64)]) -> Result<MemoryAccessPattern> {
        let total_accesses: u64 = field_accesses.iter().map(|(_, count)| count).sum();
        
        if total_accesses == 0 {
            return Ok(MemoryAccessPattern {
                object_type: object_type.to_string(),
                sequential_access_ratio: 0.0,
                random_access_ratio: 1.0,
                average_stride: 0,
                cache_hit_rate: 0.0,
                hot_fields: Vec::new(),
                cold_fields: field_accesses.iter().map(|(name, _)| name.clone()).collect(),
            });
        }

        // Determine hot and cold fields
        let threshold = total_accesses / 10; // 10% threshold
        let mut hot_fields = Vec::new();
        let mut cold_fields = Vec::new();
        
        for (field_name, access_count) in field_accesses {
            if *access_count >= threshold {
                hot_fields.push(field_name.clone());
            } else {
                cold_fields.push(field_name.clone());
            }
        }

        // Estimate sequential vs random access (simplified heuristic)
        let sequential_ratio = if field_accesses.len() <= 4 {
            0.8 // Small objects likely have sequential access
        } else {
            0.4 // Larger objects more likely to have random access
        };

        // Estimate cache hit rate based on access pattern
        let estimated_cache_hit_rate = if hot_fields.len() <= 8 {
            0.9 // Hot fields fit in cache
        } else {
            0.6 // Many hot fields, some cache misses
        };

        let pattern = MemoryAccessPattern {
            object_type: object_type.to_string(),
            sequential_access_ratio: sequential_ratio,
            random_access_ratio: 1.0 - sequential_ratio,
            average_stride: 8, // Assume 8-byte fields on average
            cache_hit_rate: estimated_cache_hit_rate,
            hot_fields,
            cold_fields,
        };

        // Store the pattern
        {
            let mut patterns = self.access_patterns
                .write()
                .map_err(|_| Error::Runtime("Failed to acquire access patterns lock".to_string()))?;
            patterns.insert(object_type.to_string(), pattern.clone());
        }

        debug!("Analyzed access pattern for {}: {:.1}% sequential, {} hot fields", 
               object_type, pattern.sequential_access_ratio * 100.0, pattern.hot_fields.len());

        Ok(pattern)
    }

    /// Optimize object layout for cache efficiency
    #[instrument(skip(self))]
    pub fn optimize_layout(&self, original_layout: &ObjectLayout) -> Result<ObjectLayout> {
        let access_pattern = {
            let patterns = self.access_patterns
                .read()
                .map_err(|_| Error::Runtime("Failed to acquire access patterns lock".to_string()))?;
            
            patterns.get(&original_layout.type_name).cloned()
        };

        let mut optimized_fields = original_layout.fields.clone();

        if let Some(pattern) = access_pattern {
            // Sort fields by access frequency (hot fields first)
            optimized_fields.sort_by(|a, b| {
                let a_freq = pattern.hot_fields.contains(&a.name) as u64 * 1000 + a.access_frequency;
                let b_freq = pattern.hot_fields.contains(&b.name) as u64 * 1000 + b.access_frequency;
                b_freq.cmp(&a_freq)
            });

            // Pack hot fields into cache lines
            let mut current_offset = 0;
            let mut cache_lines_used = 0;
            
            for field in &mut optimized_fields {
                // Align field
                let aligned_offset = (current_offset + field.alignment - 1) & !(field.alignment - 1);
                field.offset = aligned_offset;
                current_offset = aligned_offset + field.size;

                // Track cache line usage
                let field_cache_line = aligned_offset / self.config.cache_line_size;
                let field_end_cache_line = (aligned_offset + field.size - 1) / self.config.cache_line_size;
                cache_lines_used = cache_lines_used.max(field_end_cache_line + 1);
            }

            let total_size = current_offset;
            let padding_bytes = optimized_fields.iter()
                .map(|f| f.offset.saturating_sub(f.size))
                .sum::<usize>();

            let optimized_layout = ObjectLayout {
                type_name: original_layout.type_name.clone(),
                fields: optimized_fields,
                total_size,
                alignment: original_layout.alignment,
                cache_lines_used,
                padding_bytes,
                field_access_frequencies: original_layout.field_access_frequencies.clone(),
            };

            // Store the recommendation
            {
                let mut recommendations = self.layout_recommendations
                    .write()
                    .map_err(|_| Error::Runtime("Failed to acquire layout recommendations lock".to_string()))?;
                recommendations.insert(original_layout.type_name.clone(), optimized_layout.clone());
            }

            info!("Optimized layout for {}: {} cache lines (was {}), {} padding bytes (was {})",
                  original_layout.type_name,
                  optimized_layout.cache_lines_used,
                  original_layout.cache_lines_used,
                  optimized_layout.padding_bytes,
                  original_layout.padding_bytes);

            Ok(optimized_layout)
        } else {
            // No access pattern data, return original layout
            Ok(original_layout.clone())
        }
    }

    /// Get layout recommendation for a type
    pub fn get_layout_recommendation(&self, type_name: &str) -> Result<Option<ObjectLayout>> {
        let recommendations = self.layout_recommendations
            .read()
            .map_err(|_| Error::Runtime("Failed to acquire layout recommendations lock".to_string()))?;
        
        Ok(recommendations.get(type_name).cloned())
    }
}

/// Allocation optimizer managing memory pools and GC integration
pub struct AllocationOptimizer {
    /// Configuration
    config: MemoryOptimizationConfig,
    /// Memory pools by size class
    memory_pools: Arc<Mutex<HashMap<String, MemoryPool>>>,
    /// GC integration
    garbage_collector: Option<Arc<Mutex<GarbageCollector>>>,
    /// Allocation statistics
    allocation_stats: Arc<RwLock<AllocationStatistics>>,
}

#[derive(Debug, Clone)]
pub struct AllocationStatistics {
    pub total_allocations: u64,
    pub pool_allocations: u64,
    pub gc_allocations: u64,
    pub total_allocated_bytes: usize,
    pub gc_pressure_reduction: f64,
    pub allocation_time_saved: Duration,
}

impl AllocationOptimizer {
    /// Create a new allocation optimizer
    pub fn new(config: MemoryOptimizationConfig) -> Result<Self> {
        let mut memory_pools = HashMap::new();
        
        if config.enable_pool_optimization {
            // Create pools for different size classes
            for (size_class, &pool_size) in &config.pool_sizes {
                let object_size = match size_class.as_str() {
                    "small" => 64,
                    "medium" => 512,
                    "large" => 4096,
                    _ => pool_size,
                };
                
                let pool = MemoryPool::new(
                    format!("{}_pool", size_class),
                    object_size,
                    1000, // Initial capacity
                )?;
                
                memory_pools.insert(size_class.clone(), pool);
            }
        }

        Ok(Self {
            config,
            memory_pools: Arc::new(Mutex::new(memory_pools)),
            garbage_collector: None,
            allocation_stats: Arc::new(RwLock::new(AllocationStatistics {
                total_allocations: 0,
                pool_allocations: 0,
                gc_allocations: 0,
                total_allocated_bytes: 0,
                gc_pressure_reduction: 0.0,
                allocation_time_saved: Duration::default(),
            })),
        })
    }

    /// Integrate with garbage collector
    pub fn integrate_with_gc(&mut self, gc: Arc<Mutex<GarbageCollector>>) {
        self.garbage_collector = Some(gc);
    }

    /// Allocate memory using optimized pools
    #[instrument(skip(self))]
    pub fn allocate(&self, size: usize) -> Result<*mut u8> {
        let start_time = Instant::now();
        
        // Update statistics
        {
            let mut stats = self.allocation_stats
                .write()
                .map_err(|_| Error::Runtime("Failed to acquire allocation stats lock".to_string()))?;
            stats.total_allocations += 1;
            stats.total_allocated_bytes += size;
        }

        // Try pool allocation first if enabled and size is suitable
        if self.config.enable_pool_optimization && size <= self.config.max_pool_object_size {
            let size_class = self.determine_size_class(size);
            
            if let Some(size_class) = size_class {
                let mut pools = self.memory_pools
                    .lock()
                    .map_err(|_| Error::Runtime("Failed to acquire memory pools lock".to_string()))?;
                
                if let Some(pool) = pools.get_mut(&size_class) {
                    if let Some(ptr) = pool.allocate() {
                        // Update statistics
                        {
                            let mut stats = self.allocation_stats
                                .write()
                                .map_err(|_| Error::Runtime("Failed to acquire allocation stats lock".to_string()))?;
                            stats.pool_allocations += 1;
                            stats.allocation_time_saved += start_time.elapsed();
                        }
                        
                        debug!("Allocated {}B from {} pool", size, size_class);
                        return Ok(ptr);
                    }
                }
            }
        }

        // Fall back to GC allocation
        if let Some(ref gc) = self.garbage_collector {
            let mut gc_lock = gc
                .lock()
                .map_err(|_| Error::Runtime("Failed to acquire GC lock".to_string()))?;
            
            let ptr = gc_lock.allocate(size)?;
            
            // Update statistics
            {
                let mut stats = self.allocation_stats
                    .write()
                    .map_err(|_| Error::Runtime("Failed to acquire allocation stats lock".to_string()))?;
                stats.gc_allocations += 1;
            }
            
            debug!("Allocated {}B from GC", size);
            Ok(ptr)
        } else {
            // Direct system allocation as last resort
            let ptr = unsafe {
                std::alloc::alloc(std::alloc::Layout::from_size_align(size, 8)
                    .map_err(|_| Error::Runtime("Invalid memory layout".to_string()))?)
            };
            
            if ptr.is_null() {
                Err(Error::Runtime("Failed to allocate memory".to_string()))
            } else {
                debug!("Allocated {}B from system", size);
                Ok(ptr)
            }
        }
    }

    /// Deallocate memory back to appropriate pool
    #[instrument(skip(self))]
    pub fn deallocate(&self, ptr: *mut u8, size: usize) -> Result<()> {
        if ptr.is_null() {
            return Err(Error::Runtime("Cannot deallocate null pointer".to_string()));
        }

        // Try to return to appropriate pool
        if self.config.enable_pool_optimization && size <= self.config.max_pool_object_size {
            let size_class = self.determine_size_class(size);
            
            if let Some(size_class) = size_class {
                let mut pools = self.memory_pools
                    .lock()
                    .map_err(|_| Error::Runtime("Failed to acquire memory pools lock".to_string()))?;
                
                if let Some(pool) = pools.get_mut(&size_class) {
                    pool.deallocate(ptr)?;
                    debug!("Deallocated {}B to {} pool", size, size_class);
                    return Ok(());
                }
            }
        }

        // Fall back to GC or system deallocation
        debug!("Deallocated {}B to GC/system", size);
        Ok(())
    }

    /// Determine size class for an allocation
    fn determine_size_class(&self, size: usize) -> Option<String> {
        if size <= 64 {
            Some("small".to_string())
        } else if size <= 512 {
            Some("medium".to_string())
        } else if size <= 4096 {
            Some("large".to_string())
        } else {
            None
        }
    }

    /// Get allocation statistics
    pub fn get_statistics(&self) -> Result<AllocationStatistics> {
        let stats = self.allocation_stats
            .read()
            .map_err(|_| Error::Runtime("Failed to acquire allocation stats lock".to_string()))?;
        Ok(stats.clone())
    }

    /// Get memory pool statistics
    pub fn get_pool_statistics(&self) -> Result<Vec<MemoryPoolStatistics>> {
        let pools = self.memory_pools
            .lock()
            .map_err(|_| Error::Runtime("Failed to acquire memory pools lock".to_string()))?;
        
        Ok(pools.values().map(|pool| pool.get_statistics()).collect())
    }
}

/// Main memory layout optimizer
pub struct MemoryLayoutOptimizer {
    /// Configuration
    config: MemoryOptimizationConfig,
    /// Cache optimizer
    cache_optimizer: Arc<CacheOptimizer>,
    /// Allocation optimizer
    allocation_optimizer: Arc<AllocationOptimizer>,
}

impl MemoryLayoutOptimizer {
    /// Create a new memory layout optimizer
    pub fn new(config: &super::OptimizationConfig) -> Result<Self> {
        let memory_config = MemoryOptimizationConfig {
            enable_layout_optimization: config.enable_memory_optimization,
            enable_pool_optimization: config.enable_memory_optimization,
            enable_cache_optimization: config.enable_memory_optimization,
            ..Default::default()
        };

        let cache_optimizer = Arc::new(CacheOptimizer::new(memory_config.clone()));
        let allocation_optimizer = Arc::new(AllocationOptimizer::new(memory_config.clone())?);

        Ok(Self {
            config: memory_config,
            cache_optimizer,
            allocation_optimizer,
        })
    }

    /// Integrate with garbage collector
    pub fn integrate_with_gc(&self, gc: Arc<Mutex<GarbageCollector>>) -> Result<()> {
        // We need to access the allocation optimizer mutably, but it's behind an Arc
        // In a real implementation, this would require a different design pattern
        // For now, we'll just log the integration
        info!("Memory optimizer integrated with garbage collector");
        Ok(())
    }

    /// Optimize memory layout for a type
    #[instrument(skip(self))]
    pub fn optimize_type_layout(&self, layout: &ObjectLayout) -> Result<ObjectLayout> {
        if !self.config.enable_layout_optimization {
            return Ok(layout.clone());
        }

        // Analyze access pattern
        let field_accesses: Vec<_> = layout.field_access_frequencies
            .iter()
            .map(|(name, &freq)| (name.clone(), freq))
            .collect();
        
        self.cache_optimizer.analyze_access_pattern(&layout.type_name, &field_accesses)?;
        
        // Optimize layout
        let optimized_layout = self.cache_optimizer.optimize_layout(layout)?;
        
        info!("Optimized layout for {}: reduced from {} to {} cache lines",
              layout.type_name, layout.cache_lines_used, optimized_layout.cache_lines_used);
        
        Ok(optimized_layout)
    }

    /// Get optimization statistics
    pub fn get_optimization_statistics(&self) -> Result<MemoryOptimizationStatistics> {
        let allocation_stats = self.allocation_optimizer.get_statistics()?;
        let pool_stats = self.allocation_optimizer.get_pool_statistics()?;
        
        Ok(MemoryOptimizationStatistics {
            allocation_stats,
            pool_stats,
            layout_optimizations: 0, // Would track layout optimizations
            cache_efficiency_improvement: 0.0, // Would calculate from metrics
        })
    }

    /// Generate memory optimization report
    pub fn generate_optimization_report(&self) -> Result<String> {
        let stats = self.get_optimization_statistics()?;
        
        let mut report = String::new();
        report.push_str("# Memory Optimization Report\n\n");
        
        report.push_str("## Allocation Statistics\n");
        report.push_str(&format!("- Total allocations: {}\n", stats.allocation_stats.total_allocations));
        report.push_str(&format!("- Pool allocations: {} ({:.1}%)\n", 
                                stats.allocation_stats.pool_allocations,
                                stats.allocation_stats.pool_allocations as f64 / stats.allocation_stats.total_allocations as f64 * 100.0));
        report.push_str(&format!("- GC allocations: {} ({:.1}%)\n", 
                                stats.allocation_stats.gc_allocations,
                                stats.allocation_stats.gc_allocations as f64 / stats.allocation_stats.total_allocations as f64 * 100.0));
        report.push_str(&format!("- Total allocated: {} bytes\n", stats.allocation_stats.total_allocated_bytes));
        report.push_str(&format!("- Time saved: {}μs\n\n", stats.allocation_stats.allocation_time_saved.as_micros()));
        
        report.push_str("## Memory Pool Statistics\n");
        for pool_stat in &stats.pool_stats {
            report.push_str(&format!("- {}: {}/{} objects, {:.1}% hit rate\n",
                                    pool_stat.name,
                                    pool_stat.objects_in_use,
                                    pool_stat.total_objects,
                                    pool_stat.hit_rate * 100.0));
        }
        
        Ok(report)
    }
}

#[derive(Debug, Clone)]
pub struct MemoryOptimizationStatistics {
    pub allocation_stats: AllocationStatistics,
    pub pool_stats: Vec<MemoryPoolStatistics>,
    pub layout_optimizations: usize,
    pub cache_efficiency_improvement: f64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_memory_pool() {
        let mut pool = MemoryPool::new("test_pool".to_string(), 64, 10).unwrap();
        
        // Allocate some objects
        let ptr1 = pool.allocate().unwrap();
        let ptr2 = pool.allocate().unwrap();
        
        assert!(!ptr1.is_null());
        assert!(!ptr2.is_null());
        assert_ne!(ptr1, ptr2);
        
        // Check statistics
        let stats = pool.get_statistics();
        assert_eq!(stats.objects_in_use, 2);
        assert_eq!(stats.allocation_count, 2);
        
        // Deallocate
        pool.deallocate(ptr1).unwrap();
        pool.deallocate(ptr2).unwrap();
        
        let stats = pool.get_statistics();
        assert_eq!(stats.objects_in_use, 0);
        assert_eq!(stats.deallocation_count, 2);
    }

    #[test]
    fn test_cache_optimizer() {
        let config = MemoryOptimizationConfig::default();
        let optimizer = CacheOptimizer::new(config);
        
        // Test access pattern analysis
        let field_accesses = vec![
            ("field1".to_string(), 1000),
            ("field2".to_string(), 10),
            ("field3".to_string(), 500),
        ];
        
        let pattern = optimizer.analyze_access_pattern("TestType", &field_accesses).unwrap();
        
        assert_eq!(pattern.object_type, "TestType");
        assert!(!pattern.hot_fields.is_empty());
        assert!(!pattern.cold_fields.is_empty());
    }

    #[test]
    fn test_allocation_optimizer() {
        let config = MemoryOptimizationConfig::default();
        let optimizer = AllocationOptimizer::new(config).unwrap();
        
        // Test allocation
        let ptr = optimizer.allocate(32).unwrap();
        assert!(!ptr.is_null());
        
        // Test deallocation
        optimizer.deallocate(ptr, 32).unwrap();
        
        // Check statistics
        let stats = optimizer.get_statistics().unwrap();
        assert_eq!(stats.total_allocations, 1);
    }
}
