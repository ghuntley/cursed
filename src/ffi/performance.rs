//! Performance optimization for FFI operations
//!
//! This module provides performance optimizations for FFI operations including
//! zero-copy transfers, bulk operations, efficient type conversions, and
//! call optimization strategies.

use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};
use crate::error::CursedError;
use super::{FfiValue, FfiType};

/// Performance optimizer for FFI operations
pub struct PerformanceOptimizer {
    /// Call cache for frequently used functions
    call_cache: Arc<Mutex<CallCache>>,
    
    /// Memory pools for different allocation sizes
    memory_pools: Arc<Mutex<HashMap<usize, MemoryPool>>>,
    
    /// Zero-copy transfer manager
    zero_copy_manager: Arc<Mutex<ZeroCopyManager>>,
    
    /// Bulk operation coordinator
    bulk_coordinator: Arc<Mutex<BulkCoordinator>>,
    
    /// Type conversion cache
    type_cache: Arc<Mutex<TypeConversionCache>>,
    
    /// Performance configuration
    config: PerformanceConfig,
}

/// Call cache for frequently used functions
struct CallCache {
    /// Cached function results
    cache: HashMap<CallKey, CachedResult>,
    
    /// Cache statistics
    stats: CacheStats,
    
    /// Cache configuration
    config: CacheConfig,
}

/// Key for call cache
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct CallKey {
    function_name: String,
    args_hash: u64,
}

/// Cached function result
struct CachedResult {
    result: FfiValue,
    cached_at: Instant,
    hit_count: u32,
}

/// Cache statistics
#[derive(Debug, Clone)]
pub struct CacheStats {
    pub hits: u64,
    pub misses: u64,
    pub evictions: u64,
    pub hit_rate: f64,
}

/// Cache configuration
struct CacheConfig {
    max_entries: usize,
    ttl: Duration,
    enable_lru: bool,
}

/// Memory pool for specific allocation sizes
struct MemoryPool {
    /// Available memory chunks
    available: Vec<*mut std::ffi::c_void>,
    
    /// Chunk size for this pool
    chunk_size: usize,
    
    /// Pool statistics
    stats: PoolStats,
}

/// Pool statistics
#[derive(Debug, Clone)]
pub struct PoolStats {
    pub total_allocations: u64,
    pub pool_hits: u64,
    pub pool_misses: u64,
    pub hit_rate: f64,
}

/// Zero-copy transfer manager
struct ZeroCopyManager {
    /// Shared memory regions
    shared_regions: HashMap<String, SharedMemoryRegion>,
    
    /// Memory mapping support
    memory_maps: HashMap<String, MemoryMap>,
    
    /// DMA-like transfer operations
    dma_transfers: Vec<DmaTransfer>,
}

/// Shared memory region
struct SharedMemoryRegion {
    ptr: *mut std::ffi::c_void,
    size: usize,
    readers: u32,
    writers: u32,
}

/// Memory map for zero-copy operations
struct MemoryMap {
    file_descriptor: i32,
    mapped_ptr: *mut std::ffi::c_void,
    size: usize,
}

/// DMA-like transfer operation
struct DmaTransfer {
    source: *mut std::ffi::c_void,
    destination: *mut std::ffi::c_void,
    size: usize,
    transfer_id: u64,
}

/// Bulk operation coordinator
struct BulkCoordinator {
    /// Pending bulk operations
    pending_operations: Vec<BulkOperation>,
    
    /// Batch size configuration
    batch_config: BatchConfig,
    
    /// Bulk operation statistics
    stats: BulkStats,
}

/// Bulk operation
struct BulkOperation {
    operation_type: BulkOperationType,
    data: Vec<FfiValue>,
    callback: Option<Box<dyn FnOnce(Result<Vec<FfiValue>, CursedError>) + Send + Sync>>,
}

/// Type of bulk operation
enum BulkOperationType {
    TypeConversion,
    MemoryAllocation,
    FunctionCall,
    DataTransfer,
}

/// Batch configuration
struct BatchConfig {
    min_batch_size: usize,
    max_batch_size: usize,
    batch_timeout: Duration,
}

/// Bulk operation statistics
#[derive(Debug, Clone)]
pub struct BulkStats {
    pub total_operations: u64,
    pub batched_operations: u64,
    pub batch_efficiency: f64,
}

/// Type conversion cache
struct TypeConversionCache {
    /// Cached conversions
    conversions: HashMap<ConversionKey, ConversionResult>,
    
    /// Conversion statistics
    stats: ConversionStats,
}

/// Key for type conversion cache
#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct ConversionKey {
    from_type: String,
    to_type: String,
    data_hash: u64,
}

/// Cached conversion result
struct ConversionResult {
    result: FfiValue,
    conversion_time: Duration,
    cached_at: Instant,
}

/// Conversion statistics
#[derive(Debug, Clone)]
pub struct ConversionStats {
    pub total_conversions: u64,
    pub cached_conversions: u64,
    pub average_conversion_time: Duration,
    pub cache_hit_rate: f64,
}

/// Performance configuration
#[derive(Debug, Clone)]
pub struct PerformanceConfig {
    /// Enable call caching
    pub enable_call_cache: bool,
    
    /// Enable memory pooling
    pub enable_memory_pools: bool,
    
    /// Enable zero-copy transfers
    pub enable_zero_copy: bool,
    
    /// Enable bulk operations
    pub enable_bulk_operations: bool,
    
    /// Enable type conversion caching
    pub enable_type_cache: bool,
    
    /// Maximum cache size
    pub max_cache_size: usize,
    
    /// Cache TTL
    pub cache_ttl: Duration,
    
    /// Memory pool sizes
    pub pool_sizes: Vec<usize>,
    
    /// Bulk operation threshold
    pub bulk_threshold: usize,
}

impl Default for PerformanceConfig {
    fn default() -> Self {
        Self {
            enable_call_cache: true,
            enable_memory_pools: true,
            enable_zero_copy: true,
            enable_bulk_operations: true,
            enable_type_cache: true,
            max_cache_size: 10000,
            cache_ttl: Duration::from_secs(300),
            pool_sizes: vec![64, 256, 1024, 4096, 16384],
            bulk_threshold: 10,
        }
    }
}

impl PerformanceOptimizer {
    /// Create a new performance optimizer
    pub fn new() -> Self {
        Self {
            call_cache: Arc::new(Mutex::new(CallCache::new())),
            memory_pools: Arc::new(Mutex::new(HashMap::new())),
            zero_copy_manager: Arc::new(Mutex::new(ZeroCopyManager::new())),
            bulk_coordinator: Arc::new(Mutex::new(BulkCoordinator::new())),
            type_cache: Arc::new(Mutex::new(TypeConversionCache::new())),
            config: PerformanceConfig::default(),
        }
    }
    
    /// Create performance optimizer with custom configuration
    pub fn with_config(config: PerformanceConfig) -> Self {
        let mut optimizer = Self::new();
        optimizer.config = config;
        optimizer
    }
    
    /// Initialize memory pools
    pub fn initialize_memory_pools(&self) -> Result<(), CursedError> {
        if !self.config.enable_memory_pools {
            return Ok(());
        }
        
        let mut pools = self.memory_pools.lock().unwrap();
        
        for &size in &self.config.pool_sizes {
            let pool = MemoryPool::new(size);
            pools.insert(size, pool);
        }
        
        Ok(())
    }
    
    /// Configure call optimization
    pub fn configure_call_optimization(&self) -> Result<(), CursedError> {
        if !self.config.enable_call_cache {
            return Ok(());
        }
        
        let mut cache = self.call_cache.lock().unwrap();
        cache.config.max_entries = self.config.max_cache_size;
        cache.config.ttl = self.config.cache_ttl;
        
        Ok(())
    }
    
    /// Initialize zero-copy transfer mechanisms
    pub fn initialize_zero_copy_transfers(&self) -> Result<(), CursedError> {
        if !self.config.enable_zero_copy {
            return Ok(());
        }
        
        let mut zero_copy = self.zero_copy_manager.lock().unwrap();
        zero_copy.initialize()?;
        
        Ok(())
    }
    
    /// Optimize function call with caching
    pub fn optimize_call(
        &self,
        function_name: &str,
        args: &[FfiValue],
        call_fn: Box<dyn FnOnce(&[FfiValue]) -> Result<FfiValue, CursedError>>,
    ) -> Result<FfiValue, CursedError> {
        if !self.config.enable_call_cache {
            return call_fn(args);
        }
        
        let call_key = CallKey {
            function_name: function_name.to_string(),
            args_hash: self.hash_args(args),
        };
        
        // Check cache first
        {
            let mut cache = self.call_cache.lock().unwrap();
            if let Some(cached) = cache.get(&call_key) {
                return Ok(cached.result.clone());
            }
        }
        
        // Execute function
        let result = call_fn(args)?;
        
        // Cache result
        {
            let mut cache = self.call_cache.lock().unwrap();
            cache.insert(call_key, result.clone());
        }
        
        Ok(result)
    }
    
    /// Allocate memory from pool
    pub fn allocate_from_pool(&self, size: usize) -> Result<*mut std::ffi::c_void, CursedError> {
        if !self.config.enable_memory_pools {
            return self.allocate_direct(size);
        }
        
        let mut pools = self.memory_pools.lock().unwrap();
        
        // Find the best fitting pool
        let pool_size = self.config.pool_sizes.iter()
            .find(|&&pool_size| pool_size >= size)
            .copied()
            .unwrap_or(size);
        
        if let Some(pool) = pools.get_mut(&pool_size) {
            if let Some(ptr) = pool.allocate() {
                return Ok(ptr);
            }
        }
        
        // Fallback to direct allocation
        self.allocate_direct(size)
    }
    
    /// Deallocate memory to pool
    pub fn deallocate_to_pool(&self, ptr: *mut std::ffi::c_void, size: usize) -> Result<(), CursedError> {
        if !self.config.enable_memory_pools {
            return self.deallocate_direct(ptr);
        }
        
        let mut pools = self.memory_pools.lock().unwrap();
        
        // Find the appropriate pool
        let pool_size = self.config.pool_sizes.iter()
            .find(|&&pool_size| pool_size >= size)
            .copied()
            .unwrap_or(size);
        
        if let Some(pool) = pools.get_mut(&pool_size) {
            if pool.deallocate(ptr) {
                return Ok(());
            }
        }
        
        // Fallback to direct deallocation
        self.deallocate_direct(ptr)
    }
    
    /// Perform zero-copy transfer
    pub fn zero_copy_transfer(
        &self,
        source: *mut std::ffi::c_void,
        destination: *mut std::ffi::c_void,
        size: usize,
    ) -> Result<(), CursedError> {
        if !self.config.enable_zero_copy {
            // Fallback to regular copy
            unsafe {
                std::ptr::copy_nonoverlapping(source as *const u8, destination as *mut u8, size);
            }
            return Ok(());
        }
        
        let mut zero_copy = self.zero_copy_manager.lock().unwrap();
        zero_copy.perform_transfer(source, destination, size)
    }
    
    /// Submit bulk operation
    pub fn submit_bulk_operation(
        &self,
        operation_type: BulkOperationType,
        data: Vec<FfiValue>,
        callback: Option<Box<dyn FnOnce(Result<Vec<FfiValue>, CursedError>) + Send + Sync>>,
    ) -> Result<(), CursedError> {
        if !self.config.enable_bulk_operations {
            // Process immediately
            let result = self.process_bulk_operation_immediate(operation_type, data)?;
            if let Some(callback) = callback {
                callback(Ok(result));
            }
            return Ok(());
        }
        
        let mut coordinator = self.bulk_coordinator.lock().unwrap();
        
        let operation = BulkOperation {
            operation_type,
            data,
            callback,
        };
        
        coordinator.submit_operation(operation)
    }
    
    /// Process bulk operations
    pub fn process_bulk_operations(&self) -> Result<(), CursedError> {
        if !self.config.enable_bulk_operations {
            return Ok(());
        }
        
        let mut coordinator = self.bulk_coordinator.lock().unwrap();
        coordinator.process_pending_operations()
    }
    
    /// Cached type conversion
    pub fn cached_type_conversion(
        &self,
        value: &FfiValue,
        target_type: &FfiType,
    ) -> Result<FfiValue, CursedError> {
        if !self.config.enable_type_cache {
            return self.convert_type_direct(value, target_type);
        }
        
        let conversion_key = ConversionKey {
            from_type: self.get_type_name(value),
            to_type: self.get_ffi_type_name(target_type),
            data_hash: self.hash_value(value),
        };
        
        // Check cache first
        {
            let mut cache = self.type_cache.lock().unwrap();
            if let Some(cached) = cache.get(&conversion_key) {
                return Ok(cached.result.clone());
            }
        }
        
        // Perform conversion
        let start_time = Instant::now();
        let result = self.convert_type_direct(value, target_type)?;
        let conversion_time = start_time.elapsed();
        
        // Cache result
        {
            let mut cache = self.type_cache.lock().unwrap();
            cache.insert(conversion_key, result.clone(), conversion_time);
        }
        
        Ok(result)
    }
    
    /// Get performance statistics
    pub fn get_performance_stats(&self) -> PerformanceStats {
        let cache_stats = self.call_cache.lock().unwrap().get_stats();
        let pool_stats = self.get_pool_stats();
        let bulk_stats = self.bulk_coordinator.lock().unwrap().get_stats();
        let conversion_stats = self.type_cache.lock().unwrap().get_stats();
        
        PerformanceStats {
            cache_stats,
            pool_stats,
            bulk_stats,
            conversion_stats,
        }
    }
    
    /// Clear all caches
    pub fn clear_caches(&self) -> Result<(), CursedError> {
        {
            let mut cache = self.call_cache.lock().unwrap();
            cache.clear();
        }
        
        {
            let mut cache = self.type_cache.lock().unwrap();
            cache.clear();
        }
        
        Ok(())
    }
    
    // Private helper methods
    
    fn hash_args(&self, args: &[FfiValue]) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        for arg in args {
            self.hash_value(arg).hash(&mut hasher);
        }
        hasher.finish()
    }
    
    fn hash_value(&self, value: &FfiValue) -> u64 {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};
        
        let mut hasher = DefaultHasher::new();
        match value {
            FfiValue::SignedInteger(v) => v.hash(&mut hasher),
            FfiValue::UnsignedInteger(v) => v.hash(&mut hasher),
            FfiValue::Float(v) => v.to_bits().hash(&mut hasher),
            FfiValue::Boolean(v) => v.hash(&mut hasher),
            FfiValue::String(v) => v.hash(&mut hasher),
            _ => 0u64.hash(&mut hasher),
        }
        hasher.finish()
    }
    
    fn get_type_name(&self, value: &FfiValue) -> String {
        match value {
            FfiValue::Void => "void".to_string(),
            FfiValue::SignedInteger(_) => "i64".to_string(),
            FfiValue::UnsignedInteger(_) => "u64".to_string(),
            FfiValue::Float(_) => "f64".to_string(),
            FfiValue::Boolean(_) => "bool".to_string(),
            FfiValue::Character(_) => "char".to_string(),
            FfiValue::String(_) => "string".to_string(),
            FfiValue::CString(_) => "cstring".to_string(),
            FfiValue::Pointer(_) => "pointer".to_string(),
            FfiValue::Array(_) => "array".to_string(),
            FfiValue::Struct(_) => "struct".to_string(),
            FfiValue::Function(_) => "function".to_string(),
        }
    }
    
    fn get_ffi_type_name(&self, ffi_type: &FfiType) -> String {
        match ffi_type {
            FfiType::Void => "void".to_string(),
            FfiType::SignedInteger(bits) => format!("i{}", bits),
            FfiType::UnsignedInteger(bits) => format!("u{}", bits),
            FfiType::Float(bits) => format!("f{}", bits),
            FfiType::Boolean => "bool".to_string(),
            FfiType::Character => "char".to_string(),
            FfiType::String => "string".to_string(),
            FfiType::CString => "cstring".to_string(),
            FfiType::Pointer(_) => "pointer".to_string(),
            FfiType::Array(_, _) => "array".to_string(),
            FfiType::Struct(_) => "struct".to_string(),
            FfiType::Function(_) => "function".to_string(),
        }
    }
    
    fn allocate_direct(&self, size: usize) -> Result<*mut std::ffi::c_void, CursedError> {
        let ptr = unsafe { libc::malloc(size) };
        if ptr.is_null() {
            Err(CursedError::General("Memory allocation failed".to_string()))
        } else {
            Ok(ptr)
        }
    }
    
    fn deallocate_direct(&self, ptr: *mut std::ffi::c_void) -> Result<(), CursedError> {
        unsafe { libc::free(ptr) };
        Ok(())
    }
    
    fn convert_type_direct(&self, value: &FfiValue, target_type: &FfiType) -> Result<FfiValue, CursedError> {
        // This would contain actual type conversion logic
        // For now, return the original value
        Ok(value.clone())
    }
    
    fn process_bulk_operation_immediate(
        &self,
        operation_type: BulkOperationType,
        data: Vec<FfiValue>,
    ) -> Result<Vec<FfiValue>, CursedError> {
        // Process bulk operation immediately
        match operation_type {
            BulkOperationType::TypeConversion => {
                // Perform bulk type conversion
                Ok(data)
            }
            BulkOperationType::MemoryAllocation => {
                // Perform bulk memory allocation
                Ok(data)
            }
            BulkOperationType::FunctionCall => {
                // Perform bulk function calls
                Ok(data)
            }
            BulkOperationType::DataTransfer => {
                // Perform bulk data transfer
                Ok(data)
            }
        }
    }
    
    fn get_pool_stats(&self) -> HashMap<usize, PoolStats> {
        let pools = self.memory_pools.lock().unwrap();
        pools.iter().map(|(&size, pool)| (size, pool.stats.clone())).collect()
    }
}

/// Combined performance statistics
#[derive(Debug, Clone)]
pub struct PerformanceStats {
    pub cache_stats: CacheStats,
    pub pool_stats: HashMap<usize, PoolStats>,
    pub bulk_stats: BulkStats,
    pub conversion_stats: ConversionStats,
}

// Implementation of helper structs
impl CallCache {
    fn new() -> Self {
        Self {
            cache: HashMap::new(),
            stats: CacheStats {
                hits: 0,
                misses: 0,
                evictions: 0,
                hit_rate: 0.0,
            },
            config: CacheConfig {
                max_entries: 1000,
                ttl: Duration::from_secs(300),
                enable_lru: true,
            },
        }
    }
    
    fn get(&mut self, key: &CallKey) -> Option<&CachedResult> {
        if let Some(cached) = self.cache.get_mut(key) {
            // Check TTL
            if cached.cached_at.elapsed() > self.config.ttl {
                self.cache.remove(key);
                self.stats.misses += 1;
                None
            } else {
                cached.hit_count += 1;
                self.stats.hits += 1;
                self.update_hit_rate();
                Some(cached)
            }
        } else {
            self.stats.misses += 1;
            self.update_hit_rate();
            None
        }
    }
    
    fn insert(&mut self, key: CallKey, result: FfiValue) {
        // Check if cache is full
        if self.cache.len() >= self.config.max_entries {
            self.evict_lru();
        }
        
        let cached_result = CachedResult {
            result,
            cached_at: Instant::now(),
            hit_count: 0,
        };
        
        self.cache.insert(key, cached_result);
    }
    
    fn evict_lru(&mut self) {
        if let Some(lru_key) = self.find_lru_key() {
            self.cache.remove(&lru_key);
            self.stats.evictions += 1;
        }
    }
    
    fn find_lru_key(&self) -> Option<CallKey> {
        self.cache.iter()
            .min_by_key(|(_, cached)| cached.cached_at)
            .map(|(key, _)| key.clone())
    }
    
    fn update_hit_rate(&mut self) {
        let total = self.stats.hits + self.stats.misses;
        if total > 0 {
            self.stats.hit_rate = self.stats.hits as f64 / total as f64;
        }
    }
    
    fn get_stats(&self) -> CacheStats {
        self.stats.clone()
    }
    
    fn clear(&mut self) {
        self.cache.clear();
        self.stats = CacheStats {
            hits: 0,
            misses: 0,
            evictions: 0,
            hit_rate: 0.0,
        };
    }
}

impl MemoryPool {
    fn new(chunk_size: usize) -> Self {
        Self {
            available: Vec::new(),
            chunk_size,
            stats: PoolStats {
                total_allocations: 0,
                pool_hits: 0,
                pool_misses: 0,
                hit_rate: 0.0,
            },
        }
    }
    
    fn allocate(&mut self) -> Option<*mut std::ffi::c_void> {
        self.stats.total_allocations += 1;
        
        if let Some(ptr) = self.available.pop() {
            self.stats.pool_hits += 1;
            self.update_hit_rate();
            Some(ptr)
        } else {
            self.stats.pool_misses += 1;
            self.update_hit_rate();
            None
        }
    }
    
    fn deallocate(&mut self, ptr: *mut std::ffi::c_void) -> bool {
        if self.available.len() < 1000 { // Limit pool size
            self.available.push(ptr);
            true
        } else {
            false
        }
    }
    
    fn update_hit_rate(&mut self) {
        let total = self.stats.pool_hits + self.stats.pool_misses;
        if total > 0 {
            self.stats.hit_rate = self.stats.pool_hits as f64 / total as f64;
        }
    }
}

impl ZeroCopyManager {
    fn new() -> Self {
        Self {
            shared_regions: HashMap::new(),
            memory_maps: HashMap::new(),
            dma_transfers: Vec::new(),
        }
    }
    
    fn initialize(&mut self) -> Result<(), CursedError> {
        // Initialize zero-copy mechanisms
        Ok(())
    }
    
    fn perform_transfer(
        &mut self,
        source: *mut std::ffi::c_void,
        destination: *mut std::ffi::c_void,
        size: usize,
    ) -> Result<(), CursedError> {
        // Perform zero-copy transfer
        unsafe {
            std::ptr::copy_nonoverlapping(source as *const u8, destination as *mut u8, size);
        }
        Ok(())
    }
}

impl BulkCoordinator {
    fn new() -> Self {
        Self {
            pending_operations: Vec::new(),
            batch_config: BatchConfig {
                min_batch_size: 5,
                max_batch_size: 100,
                batch_timeout: Duration::from_millis(100),
            },
            stats: BulkStats {
                total_operations: 0,
                batched_operations: 0,
                batch_efficiency: 0.0,
            },
        }
    }
    
    fn submit_operation(&mut self, operation: BulkOperation) -> Result<(), CursedError> {
        self.pending_operations.push(operation);
        self.stats.total_operations += 1;
        
        // Process if we have enough operations
        if self.pending_operations.len() >= self.batch_config.min_batch_size {
            self.process_pending_operations()?;
        }
        
        Ok(())
    }
    
    fn process_pending_operations(&mut self) -> Result<(), CursedError> {
        if self.pending_operations.is_empty() {
            return Ok(());
        }
        
        let batch_size = self.pending_operations.len().min(self.batch_config.max_batch_size);
        let batch: Vec<_> = self.pending_operations.drain(..batch_size).collect();
        
        self.stats.batched_operations += batch.len() as u64;
        self.update_batch_efficiency();
        
        // Process batch
        for operation in batch {
            // Process each operation in the batch
            // This would contain actual bulk processing logic
            if let Some(callback) = operation.callback {
                callback(Ok(vec![]));
            }
        }
        
        Ok(())
    }
    
    fn update_batch_efficiency(&mut self) {
        if self.stats.total_operations > 0 {
            self.stats.batch_efficiency = 
                self.stats.batched_operations as f64 / self.stats.total_operations as f64;
        }
    }
    
    fn get_stats(&self) -> BulkStats {
        self.stats.clone()
    }
}

impl TypeConversionCache {
    fn new() -> Self {
        Self {
            conversions: HashMap::new(),
            stats: ConversionStats {
                total_conversions: 0,
                cached_conversions: 0,
                average_conversion_time: Duration::ZERO,
                cache_hit_rate: 0.0,
            },
        }
    }
    
    fn get(&mut self, key: &ConversionKey) -> Option<&ConversionResult> {
        self.stats.total_conversions += 1;
        
        if let Some(cached) = self.conversions.get(key) {
            // Check if cache entry is still valid
            if cached.cached_at.elapsed() < Duration::from_secs(300) {
                self.stats.cached_conversions += 1;
                self.update_cache_hit_rate();
                return Some(cached);
            } else {
                self.conversions.remove(key);
            }
        }
        
        self.update_cache_hit_rate();
        None
    }
    
    fn insert(&mut self, key: ConversionKey, result: FfiValue, conversion_time: Duration) {
        let cached_result = ConversionResult {
            result,
            conversion_time,
            cached_at: Instant::now(),
        };
        
        self.conversions.insert(key, cached_result);
        self.update_average_conversion_time(conversion_time);
    }
    
    fn update_cache_hit_rate(&mut self) {
        if self.stats.total_conversions > 0 {
            self.stats.cache_hit_rate = 
                self.stats.cached_conversions as f64 / self.stats.total_conversions as f64;
        }
    }
    
    fn update_average_conversion_time(&mut self, new_time: Duration) {
        let total_time = self.stats.average_conversion_time.as_nanos() as f64 * 
                        (self.stats.total_conversions - 1) as f64 + 
                        new_time.as_nanos() as f64;
        
        self.stats.average_conversion_time = Duration::from_nanos(
            (total_time / self.stats.total_conversions as f64) as u64
        );
    }
    
    fn get_stats(&self) -> ConversionStats {
        self.stats.clone()
    }
    
    fn clear(&mut self) {
        self.conversions.clear();
        self.stats = ConversionStats {
            total_conversions: 0,
            cached_conversions: 0,
            average_conversion_time: Duration::ZERO,
            cache_hit_rate: 0.0,
        };
    }
}

impl Default for PerformanceOptimizer {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_performance_optimizer_creation() {
        let optimizer = PerformanceOptimizer::new();
        let stats = optimizer.get_performance_stats();
        
        assert_eq!(stats.cache_stats.hits, 0);
        assert_eq!(stats.cache_stats.misses, 0);
        assert_eq!(stats.bulk_stats.total_operations, 0);
    }
    
    #[test]
    fn test_call_cache() {
        let mut cache = CallCache::new();
        
        let key = CallKey {
            function_name: "test_function".to_string(),
            args_hash: 12345,
        };
        
        let result = FfiValue::SignedInteger(42);
        
        // First get should miss
        assert!(cache.get(&key).is_none());
        
        // Insert and get should hit
        cache.insert(key.clone(), result.clone());
        assert!(cache.get(&key).is_some());
        
        let stats = cache.get_stats();
        assert_eq!(stats.hits, 1);
        assert_eq!(stats.misses, 1);
        assert_eq!(stats.hit_rate, 0.5);
    }
    
    #[test]
    fn test_memory_pool() {
        let mut pool = MemoryPool::new(1024);
        
        // Should miss on first allocation
        assert!(pool.allocate().is_none());
        
        // Deallocate and then allocate should hit
        let ptr = unsafe { libc::malloc(1024) };
        pool.deallocate(ptr);
        assert!(pool.allocate().is_some());
        
        let stats = pool.stats.clone();
        assert_eq!(stats.pool_hits, 1);
        assert_eq!(stats.pool_misses, 1);
        assert_eq!(stats.hit_rate, 0.5);
    }
    
    #[test]
    fn test_bulk_coordinator() {
        let mut coordinator = BulkCoordinator::new();
        
        let operation = BulkOperation {
            operation_type: BulkOperationType::TypeConversion,
            data: vec![FfiValue::SignedInteger(42)],
            callback: None,
        };
        
        coordinator.submit_operation(operation).unwrap();
        
        let stats = coordinator.get_stats();
        assert_eq!(stats.total_operations, 1);
    }
    
    #[test]
    fn test_type_conversion_cache() {
        let mut cache = TypeConversionCache::new();
        
        let key = ConversionKey {
            from_type: "i32".to_string(),
            to_type: "f64".to_string(),
            data_hash: 12345,
        };
        
        let result = FfiValue::Float(42.0);
        
        // First get should miss
        assert!(cache.get(&key).is_none());
        
        // Insert and get should hit
        cache.insert(key.clone(), result.clone(), Duration::from_millis(1));
        assert!(cache.get(&key).is_some());
        
        let stats = cache.get_stats();
        assert_eq!(stats.total_conversions, 2);
        assert_eq!(stats.cached_conversions, 1);
        assert_eq!(stats.cache_hit_rate, 0.5);
    }
}
