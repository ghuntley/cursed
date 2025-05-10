//! LRU Cached Field Accessors Implementation Module for LLVM code generation
//!
//! This module provides enhanced field accessor generation with LRU caching,
//! proper LLVM error handling and propagation using the Result<T, Error> type.

use inkwell::types::{BasicType, BasicTypeEnum};
use inkwell::values::FunctionValue;
use crate::ast::declarations::{FunctionStatement, SquadStatement, CollabStatement, GenericConstraint};
use crate::ast::traits::Node;
use crate::codegen::llvm::LlvmCodeGenerator;
use crate::codegen::llvm::improved_field_accessors::ImprovedFieldAccessors;
use crate::core::type_checker::Type;
use crate::error::Error;
use crate::core::generic_instantiation::GenericInstantiator;
use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex, RwLock};
use tracing::{debug, info, warn, error, span, Level};

/// Statistics for the LRU cache
struct LruCacheStats {
    /// Number of cache hits
    hits: usize,
    /// Number of cache misses
    misses: usize,
    /// Number of entries evicted from the cache
    evictions: usize,
    /// Total number of lookups
    lookups: usize,
}

impl LruCacheStats {
    /// Create new empty stats
    fn new() -> Self {
        Self {
            hits: 0,
            misses: 0,
            evictions: 0,
            lookups: 0,
        }
    }

    /// Record a cache hit
    fn record_hit(&mut self) {
        self.hits += 1;
        self.lookups += 1;
    }

    /// Record a cache miss
    fn record_miss(&mut self) {
        self.misses += 1;
        self.lookups += 1;
    }

    /// Record a cache eviction
    fn record_eviction(&mut self) {
        self.evictions += 1;
    }

    /// Get the cache hit rate
    fn hit_rate(&self) -> f64 {
        if self.lookups == 0 {
            0.0
        } else {
            self.hits as f64 / self.lookups as f64
        }
    }

    /// Get cache statistics as a formatted string
    fn as_string(&self) -> String {
        format!(
            "LRU Cache Stats: hits={}, misses={}, evictions={}, hit_rate={:.2}%",
            self.hits,
            self.misses,
            self.evictions,
            self.hit_rate() * 100.0
        )
    }
}

/// Entry in the LRU cache
struct LruCacheEntry {
    /// Cache key (specialized_name + field_name)
    key: String,
    /// Timestamp when the entry was last accessed
    last_accessed: std::time::Instant,
}

/// LRU Cache for field accessors
pub struct FieldAccessorLruCache {
    /// Map from cache key to boolean indicating accessor existence
    cache: HashMap<String, bool>,
    /// Queue of entries in LRU order
    lru_queue: VecDeque<LruCacheEntry>,
    /// Maximum number of entries in the cache
    capacity: usize,
    /// Cache statistics
    stats: LruCacheStats,
}

impl FieldAccessorLruCache {
    /// Create a new LRU cache with the specified capacity
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: HashMap::with_capacity(capacity),
            lru_queue: VecDeque::with_capacity(capacity),
            capacity,
            stats: LruCacheStats::new(),
        }
    }

    /// Create a new cache with default capacity
    pub fn default() -> Self {
        Self::new(1000)
    }

    /// Create a cache key from struct name and field name
    fn create_key(specialized_name: &str, field_name: &str, accessor_type: &str) -> String {
        format!("{}_{}_{}", specialized_name, accessor_type, field_name)
    }

    /// Check if an accessor exists in the cache
    pub fn accessor_exists(&mut self, specialized_name: &str, field_name: &str, accessor_type: &str) -> bool {
        let key = Self::create_key(specialized_name, field_name, accessor_type);
        
        // Check if key exists in cache
        if let Some(&exists) = self.cache.get(&key) {
            // Update entry's position in the LRU queue
            self.update_lru_position(&key);
            self.stats.record_hit();
            return exists;
        }
        
        // Cache miss
        self.stats.record_miss();
        false
    }

    /// Add an accessor to the cache
    pub fn add_accessor(&mut self, specialized_name: &str, field_name: &str, accessor_type: &str, exists: bool) {
        let key = Self::create_key(specialized_name, field_name, accessor_type);
        
        // Check if we need to evict an entry
        if self.cache.len() >= self.capacity && !self.cache.contains_key(&key) {
            self.evict_lru_entry();
        }
        
        // Add or update entry
        self.cache.insert(key.clone(), exists);
        
        // Add to LRU queue
        self.lru_queue.push_back(LruCacheEntry {
            key,
            last_accessed: std::time::Instant::now(),
        });
    }

    /// Update an entry's position in the LRU queue
    fn update_lru_position(&mut self, key: &str) {
        // Find and remove the entry from current position
        if let Some(pos) = self.lru_queue.iter().position(|e| e.key == key) {
            let mut entry = self.lru_queue.remove(pos).unwrap();
            entry.last_accessed = std::time::Instant::now();
            // Add back to the end (most recently used)
            self.lru_queue.push_back(entry);
        }
    }

    /// Evict the least recently used entry
    fn evict_lru_entry(&mut self) {
        if let Some(entry) = self.lru_queue.pop_front() {
            self.cache.remove(&entry.key);
            self.stats.record_eviction();
            debug!("LRU cache eviction: {}", entry.key);
        }
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> String {
        self.stats.as_string()
    }

    /// Clear the cache
    pub fn clear(&mut self) {
        self.cache.clear();
        self.lru_queue.clear();
        debug!("LRU cache cleared: {}", self.stats.as_string());
    }
}

/// Thread-safe LRU cache for field accessors
pub struct ThreadSafeFieldAccessorLruCache {
    /// The internal LRU cache protected by a mutex
    cache: Arc<Mutex<FieldAccessorLruCache>>,
}

impl ThreadSafeFieldAccessorLruCache {
    /// Create a new thread-safe LRU cache
    pub fn new(capacity: usize) -> Self {
        Self {
            cache: Arc::new(Mutex::new(FieldAccessorLruCache::new(capacity))),
        }
    }

    /// Create a cache with default capacity
    pub fn default() -> Self {
        Self::new(1000)
    }

    /// Check if an accessor exists in the cache
    pub fn accessor_exists(&self, specialized_name: &str, field_name: &str, accessor_type: &str) -> bool {
        if let Ok(mut cache) = self.cache.lock() {
            cache.accessor_exists(specialized_name, field_name, accessor_type)
        } else {
            // If lock fails, assume accessor doesn't exist
            warn!("Failed to acquire lock for LRU cache check");
            false
        }
    }

    /// Add an accessor to the cache
    pub fn add_accessor(&self, specialized_name: &str, field_name: &str, accessor_type: &str, exists: bool) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.add_accessor(specialized_name, field_name, accessor_type, exists);
        } else {
            warn!("Failed to acquire lock for LRU cache update");
        }
    }

    /// Get cache statistics
    pub fn get_stats(&self) -> String {
        if let Ok(cache) = self.cache.lock() {
            cache.get_stats()
        } else {
            "LRU Cache Stats: Unable to acquire lock".to_string()
        }
    }

    /// Clear the cache
    pub fn clear(&self) {
        if let Ok(mut cache) = self.cache.lock() {
            cache.clear();
        } else {
            warn!("Failed to acquire lock for LRU cache clear");
        }
    }
}

/// Trait for field accessor generation with LRU caching
pub trait LruCachedFieldAccessors<'ctx> {
    /// Generate field accessor methods for a specialized struct with LRU caching
    fn generate_lru_cached_field_accessors(
        &mut self,
        generic_struct: &SquadStatement,
        specialized_name: &str,
        type_args: &[Type],
    ) -> Result<(), Error>;

    /// Check if a field accessor already exists using the LRU cache
    fn field_accessor_exists_with_lru_cache(
        &self,
        specialized_name: &str,
        field_name: &str,
        accessor_type: &str,
    ) -> bool;
}

impl<'ctx> LruCachedFieldAccessors<'ctx> for LlvmCodeGenerator<'ctx> {
    #[tracing::instrument(skip(self, generic_struct), fields(struct_name = %generic_struct.name.value, specialized_name = %specialized_name), level = "debug")]
    fn generate_lru_cached_field_accessors(
        &mut self,
        generic_struct: &SquadStatement,
        specialized_name: &str,
        type_args: &[Type],
    ) -> Result<(), Error> {
        // Create a GenericInstantiator to handle type parameter substitution
        let mut instantiator = GenericInstantiator::new();
        
        // Set up type parameter mappings
        for (i, type_param) in generic_struct.type_parameters.iter().enumerate() {
            instantiator.add_type_param(&type_param.value, type_args[i].clone());
        }
        
        // Get the specialized struct type
        let struct_type = self.context()
            .get_struct_type(specialized_name)
            .ok_or_else(|| Error::codegen(format!("Struct type not found: {}", specialized_name)))?;
        
        // Initialize the LRU cache if not already initialized
        self.ensure_lru_field_accessor_cache();
        
        // Create getter and setter methods for each field
        for (i, field) in generic_struct.fields.iter().enumerate() {
            let field_name = &field.name.value;
            let field_index = i as u32;
            
            // Generate getter and setter names
            let getter_name = format!("{}_get_{}", specialized_name, field_name);
            let setter_name = format!("{}_set_{}", specialized_name, field_name);
            
            // Check if field accessors already exist using the LRU cache
            let getter_exists = self.field_accessor_exists_with_lru_cache(specialized_name, field_name, "get");
            let setter_exists = self.field_accessor_exists_with_lru_cache(specialized_name, field_name, "set");
            
            if getter_exists && setter_exists {
                debug!("Field accessors for {}.{} already exist (from LRU cache), skipping generation", specialized_name, field_name);
                continue;
            }
            
            // First, double-check with the actual module to ensure we don't have cache inconsistencies
            let actual_getter_exists = self.module().get_function(&getter_name).is_some();
            let actual_setter_exists = self.module().get_function(&setter_name).is_some();
            
            // Update the cache with the actual values
            self.update_lru_field_accessor_cache(specialized_name, field_name, "get", actual_getter_exists);
            self.update_lru_field_accessor_cache(specialized_name, field_name, "set", actual_setter_exists);
            
            // If both actually exist, skip generation
            if actual_getter_exists && actual_setter_exists {
                debug!("Field accessors for {}.{} exist after verification, skipping generation", specialized_name, field_name);
                continue;
            }
            
            // Extract the field's type
            let field_type_expr = &field.type_name;
            let generic_field_type = Type::Named(field_type_expr.string());
            let concrete_field_type = instantiator.instantiate_type(&generic_field_type)?;
            
            // Get LLVM types
            let field_llvm_type = self.type_to_llvm_basic(&concrete_field_type)?;
            let struct_ptr_type = struct_type.ptr_type(inkwell::AddressSpace::default());
            
            // Create getter function if it doesn't exist
            if !actual_getter_exists {
                // Create a span with field information for better tracing
                let _span = tracing::info_span!("field_accessor", field_name = %field_name, field_index = field_index, accessor_type = "getter").entered();
                
                let getter_fn_type = field_llvm_type.fn_type(&[struct_ptr_type.into()], false);
                let getter_fn = self.module().add_function(&getter_name, getter_fn_type, None);
                
                // Create getter function body
                let getter_entry = self.context().append_basic_block(getter_fn, "entry");
                self.builder().position_at_end(getter_entry);
                
                // Get function parameter (struct pointer)
                let struct_ptr = getter_fn.get_nth_param(0)
                    .ok_or_else(|| Error::codegen(format!("Failed to get function parameter for {}", getter_name)))?;
                
                // Build GEP instruction to get the field pointer
                let pointer_type = struct_type.ptr_type(inkwell::AddressSpace::default());
                let field_ptr = unsafe {
                    match self.builder()
                        .build_struct_gep(
                            pointer_type, 
                            struct_ptr.into_pointer_value(), 
                            field_index, 
                            &format!("field_ptr_{}", field_name)
                        ) {
                            Ok(ptr) => ptr,
                            Err(e) => return Err(Error::codegen(format!("Failed to build field GEP for field '{}': {}", field_name, e)))
                        }
                };
                
                // Get the correct element type for this field
                let elem_type = struct_type
                    .get_field_type_at_index(field_index)
                    .ok_or_else(|| Error::codegen(format!("Cannot get field type at index {} for field '{}'", field_index, field_name)))?;
    
                // Load the field value
                let field_value = match self.builder()
                    .build_load(
                        elem_type, 
                        field_ptr, 
                        &format!("field_value_{}", field_name)
                    ) {
                        Ok(val) => val,
                        Err(e) => return Err(Error::codegen(format!("Failed to build load for field '{}': {}", field_name, e)))
                    };
                
                // Return the field value
                match self.builder().build_return(Some(&field_value)) {
                    Ok(_) => {},
                    Err(e) => return Err(Error::codegen(format!("Failed to build return for getter '{}': {}", getter_name, e)))
                };
                
                debug!("Generated getter method for field '{}' in struct '{}'", field_name, specialized_name);
                
                // Update the LRU cache
                self.update_lru_field_accessor_cache(specialized_name, field_name, "get", true);
            }
            
            // Create setter function if it doesn't exist
            if !actual_setter_exists {
                // Create a span for setter tracing
                let _span = tracing::info_span!("field_accessor", field_name = %field_name, field_index = field_index, accessor_type = "setter").entered();
                
                let setter_fn_type = self.context()
                    .void_type()
                    .fn_type(&[struct_ptr_type.into(), field_llvm_type.into()], false);
                    
                let setter_fn = self.module().add_function(&setter_name, setter_fn_type, None);
                
                // Create setter function body
                let setter_entry = self.context().append_basic_block(setter_fn, "entry");
                self.builder().position_at_end(setter_entry);
                
                // Get function parameters
                let struct_ptr = setter_fn.get_nth_param(0)
                    .ok_or_else(|| Error::codegen(format!("Failed to get struct pointer parameter for {}", setter_name)))?;
                let value = setter_fn.get_nth_param(1)
                    .ok_or_else(|| Error::codegen(format!("Failed to get value parameter for {}", setter_name)))?;
                
                // Build GEP instruction to get the field pointer
                let pointer_type = struct_type.ptr_type(inkwell::AddressSpace::default());
                let field_ptr = unsafe {
                    match self.builder()
                        .build_struct_gep(
                            pointer_type, 
                            struct_ptr.into_pointer_value(), 
                            field_index, 
                            &format!("field_ptr_{}", field_name)
                        ) {
                            Ok(ptr) => ptr,
                            Err(e) => return Err(Error::codegen(format!("Failed to build field GEP for setter '{}': {}", setter_name, e)))
                        }
                };
                
                // Store the new value
                match self.builder().build_store(field_ptr, value) {
                    Ok(_) => {},
                    Err(e) => return Err(Error::codegen(format!("Failed to build store for field '{}': {}", field_name, e)))
                };
                
                // Return void
                match self.builder().build_return(None) {
                    Ok(_) => {},
                    Err(e) => return Err(Error::codegen(format!("Failed to build return for setter '{}': {}", setter_name, e)))
                };
                
                debug!("Generated setter method for field '{}' in struct '{}'", field_name, specialized_name);
                
                // Update the LRU cache
                self.update_lru_field_accessor_cache(specialized_name, field_name, "set", true);
            }
        }
        
        info!("Successfully generated all field accessors for struct '{}' with LRU caching!", specialized_name);
        
        // Log cache stats periodically
        let log_stats = rand::random::<f32>() < 0.05; // 5% chance to log stats
        if log_stats {
            if let Some(stats) = self.get_lru_field_accessor_cache_stats() {
                info!("Field accessor LRU cache stats: {}", stats);
            }
        }
        
        Ok(())
    }
    
    fn field_accessor_exists_with_lru_cache(
        &self,
        specialized_name: &str,
        field_name: &str,
        accessor_type: &str
    ) -> bool {
        // Try to use LRU cache if available
        if let Some(lru_cache) = &self.lru_field_accessor_cache {
            lru_cache.accessor_exists(specialized_name, field_name, accessor_type)
        } else {
            // Fall back to traditional method
            let accessor_name = format!("{}_{}_{}", specialized_name, accessor_type, field_name);
            self.module().get_function(&accessor_name).is_some()
        }
    }
}

// Extension methods for LlvmCodeGenerator to manage LRU cache
impl<'ctx> LlvmCodeGenerator<'ctx> {
    /// Ensure LRU field accessor cache is initialized
    pub fn ensure_lru_field_accessor_cache(&mut self) {
        if self.lru_field_accessor_cache.is_none() {
            debug!("Initializing LRU field accessor cache");
            self.lru_field_accessor_cache = Some(ThreadSafeFieldAccessorLruCache::default());
        }
    }
    
    /// Update the LRU field accessor cache
    pub fn update_lru_field_accessor_cache(
        &self,
        specialized_name: &str,
        field_name: &str,
        accessor_type: &str,
        exists: bool
    ) {
        if let Some(lru_cache) = &self.lru_field_accessor_cache {
            lru_cache.add_accessor(specialized_name, field_name, accessor_type, exists);
        }
    }
    
    /// Get LRU field accessor cache statistics
    pub fn get_lru_field_accessor_cache_stats(&self) -> Option<String> {
        self.lru_field_accessor_cache.as_ref().map(|cache| cache.get_stats())
    }
    
    /// Clear the LRU field accessor cache
    pub fn clear_lru_field_accessor_cache(&self) {
        if let Some(lru_cache) = &self.lru_field_accessor_cache {
            lru_cache.clear();
        }
    }
}

// Add lru_field_accessor_cache field to LlvmCodeGenerator
pub trait LruFieldAccessorCacheExtension {
    /// LRU cache for field accessors
    fn lru_field_accessor_cache(&self) -> Option<&ThreadSafeFieldAccessorLruCache>;
    
    /// Set the LRU field accessor cache
    fn set_lru_field_accessor_cache(&mut self, cache: ThreadSafeFieldAccessorLruCache);
}

// Registering function for the module
pub fn register_lru_field_accessors() {
    info!("Registering LRU cached field accessors module");
    // The trait is automatically available to LlvmCodeGenerator
}