//! Generational garbage collection for CURSED runtime
//! 
//! Implements a generational garbage collector that segregates objects
//! by age, optimizing for the observation that most objects die young.

use crate::error::CursedError;
use crate::memory::{Traceable, Visitor};
use crate::memory::heap::{ObjectId, get_global_heap};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

/// Generational garbage collector
pub struct GenerationalCollector {
    /// Young generation (newly allocated objects)
    young_generation: Mutex<Generation>,
    /// Old generation (long-lived objects)
    old_generation: Mutex<Generation>,
    /// Remembered set (old->young references)
    remembered_set: Mutex<HashSet<ObjectId>>,
    /// Collection statistics
    stats: Mutex<GenerationalStats>,
    /// Configuration
    config: Mutex<GenerationalConfig>,
}

/// Configuration for generational collection
#[derive(Debug, Clone)]
pub struct GenerationalConfig {
    /// Threshold for promoting objects to old generation
    pub promotion_threshold: u32,
    /// Young generation size limit
    pub young_gen_size_limit: usize,
    /// Old generation collection frequency
    pub old_gen_collection_ratio: u32,
}

impl Default for GenerationalConfig {
    fn default() -> Self {
        Self {
            promotion_threshold: 2,
            young_gen_size_limit: 16 * 1024 * 1024, // 16MB
            old_gen_collection_ratio: 10, // Collect old gen every 10 young collections
        }
    }
}

/// Statistics for generational collection
#[derive(Debug, Clone, Default)]
pub struct GenerationalStats {
    pub young_collections: u64,
    pub old_collections: u64,
    pub young_collection_time_ms: u64,
    pub old_collection_time_ms: u64,
    pub promotions: u64,
    pub young_gen_size: usize,
    pub old_gen_size: usize,
}

/// A generation in the generational collector
#[derive(Debug)]
struct Generation {
    /// Objects in this generation
    objects: HashMap<ObjectId, GenerationObject>,
    /// Total size of this generation
    total_size: usize,
}

/// Object in a generation
#[derive(Debug)]
struct GenerationObject {
    /// Object ID
    id: ObjectId,
    /// Survival count (how many collections it survived)
    survival_count: u32,
    /// Object size
    size: usize,
    /// Marked during current collection
    marked: bool,
}

impl Generation {
    fn new() -> Self {
        Self {
            objects: HashMap::new(),
            total_size: 0,
        }
    }

    fn add_object(&mut self, id: ObjectId, size: usize) {
        let obj = GenerationObject {
            id,
            survival_count: 0,
            size,
            marked: false,
        };
        self.objects.insert(id, obj);
        self.total_size += size;
    }

    fn remove_object(&mut self, id: ObjectId) -> Option<GenerationObject> {
        if let Some(obj) = self.objects.remove(&id) {
            self.total_size -= obj.size;
            Some(obj)
        } else {
            None
        }
    }

    fn mark_object(&mut self, id: ObjectId) -> bool {
        if let Some(obj) = self.objects.get_mut(&id) {
            if !obj.marked {
                obj.marked = true;
                return true;
            }
        }
        false
    }

    fn unmark_all(&mut self) {
        for obj in self.objects.values_mut() {
            obj.marked = false;
        }
    }

    fn sweep_unmarked(&mut self) -> Vec<ObjectId> {
        let mut removed = Vec::new();
        self.objects.retain(|&id, obj| {
            if obj.marked {
                true
            } else {
                self.total_size -= obj.size;
                removed.push(id);
                false
            }
        });
        removed
    }

    fn get_survivors(&mut self, threshold: u32) -> Vec<ObjectId> {
        let mut survivors = Vec::new();
        for obj in self.objects.values_mut() {
            if obj.marked {
                obj.survival_count += 1;
                if obj.survival_count >= threshold {
                    survivors.push(obj.id);
                }
            }
        }
        survivors
    }
}

impl GenerationalCollector {
    /// Create a new generational collector
    pub fn new() -> Self {
        Self {
            young_generation: Mutex::new(Generation::new()),
            old_generation: Mutex::new(Generation::new()),
            remembered_set: Mutex::new(HashSet::new()),
            stats: Mutex::new(GenerationalStats::default()),
            config: Mutex::new(GenerationalConfig::default()),
        }
    }

    /// Configure the collector
    pub fn configure(&self, config: GenerationalConfig) {
        let mut current_config = self.config.lock().unwrap();
        *current_config = config;
    }

    /// Allocate object in young generation
    pub fn allocate_young(&self, id: ObjectId, size: usize) -> Result<(), CursedError> {
        let mut young_gen = self.young_generation.lock().unwrap();
        young_gen.add_object(id, size);
        
        let mut stats = self.stats.lock().unwrap();
        stats.young_gen_size += size;
        
        Ok(())
    }

    /// Check if young generation collection is needed
    pub fn should_collect_young(&self) -> bool {
        let config = self.config.lock().unwrap();
        let young_gen = self.young_generation.lock().unwrap();
        
        young_gen.total_size >= config.young_gen_size_limit
    }

    /// Collect young generation
    pub fn collect_young(&self) -> Result<GenerationalResult, CursedError> {
        let start_time = Instant::now();
        let config = self.config.lock().unwrap();
        
        // Mark phase: mark reachable objects in young generation
        let mut young_gen = self.young_generation.lock().unwrap();
        let mut old_gen = self.old_generation.lock().unwrap();
        let remembered_set = self.remembered_set.lock().unwrap();
        
        young_gen.unmark_all();
        
        // Mark from remembered set (old->young references)
        for &old_id in remembered_set.iter() {
            // In practice, you'd trace from old objects that reference young objects
            // This is simplified for demonstration
        }
        
        // Identify survivors and candidates for promotion
        let survivors = young_gen.get_survivors(config.promotion_threshold);
        let promoted_objects = survivors.len();
        
        // Promote survivors to old generation
        for survivor_id in survivors {
            if let Some(obj) = young_gen.remove_object(survivor_id) {
                old_gen.add_object(obj.id, obj.size);
            }
        }
        
        // Sweep unmarked objects
        let collected_objects = young_gen.sweep_unmarked();
        
        let collection_time = start_time.elapsed();
        
        // Update statistics
        let mut stats = self.stats.lock().unwrap();
        stats.young_collections += 1;
        stats.young_collection_time_ms += collection_time.as_millis() as u64;
        stats.promotions += promoted_objects as u64;
        stats.young_gen_size = young_gen.total_size;
        stats.old_gen_size = old_gen.total_size;
        
        Ok(GenerationalResult {
            generation: GenerationType::Young,
            collected_objects: collected_objects.len(),
            promoted_objects,
            collection_time,
        })
    }

    /// Collect old generation (full collection)
    pub fn collect_old(&self) -> Result<GenerationalResult, CursedError> {
        let start_time = Instant::now();
        
        // Full collection of both generations
        let mut young_gen = self.young_generation.lock().unwrap();
        let mut old_gen = self.old_generation.lock().unwrap();
        
        young_gen.unmark_all();
        old_gen.unmark_all();
        
        // Mark reachable objects in both generations
        // This would involve tracing from roots through both generations
        
        // Sweep both generations
        let young_collected = young_gen.sweep_unmarked();
        let old_collected = old_gen.sweep_unmarked();
        
        let collection_time = start_time.elapsed();
        
        // Update statistics
        let mut stats = self.stats.lock().unwrap();
        stats.old_collections += 1;
        stats.old_collection_time_ms += collection_time.as_millis() as u64;
        stats.young_gen_size = young_gen.total_size;
        stats.old_gen_size = old_gen.total_size;
        
        Ok(GenerationalResult {
            generation: GenerationType::Old,
            collected_objects: young_collected.len() + old_collected.len(),
            promoted_objects: 0,
            collection_time,
        })
    }

    /// Decide whether to collect young or old generation
    pub fn collect(&self) -> Result<GenerationalResult, CursedError> {
        let stats = self.stats.lock().unwrap();
        let config = self.config.lock().unwrap();
        
        // Check if we should do a full collection
        if stats.young_collections % config.old_gen_collection_ratio as u64 == 0 {
            drop(stats);
            drop(config);
            self.collect_old()
        } else {
            drop(stats);
            drop(config);
            self.collect_young()
        }
    }

    /// Get statistics
    pub fn stats(&self) -> GenerationalStats {
        let stats = self.stats.lock().unwrap();
        stats.clone()
    }

    /// Add reference to remembered set
    pub fn add_remembered_reference(&self, old_id: ObjectId) {
        let mut remembered_set = self.remembered_set.lock().unwrap();
        remembered_set.insert(old_id);
    }

    /// Remove reference from remembered set
    pub fn remove_remembered_reference(&self, old_id: ObjectId) {
        let mut remembered_set = self.remembered_set.lock().unwrap();
        remembered_set.remove(&old_id);
    }
}

impl Default for GenerationalCollector {
    fn default() -> Self {
        Self::new()
    }
}

/// Type of generation being collected
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum GenerationType {
    Young,
    Old,
}

/// Result of generational collection
#[derive(Debug, Clone)]
pub struct GenerationalResult {
    pub generation: GenerationType,
    pub collected_objects: usize,
    pub promoted_objects: usize,
    pub collection_time: Duration,
}

impl std::fmt::Display for GenerationalResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, 
            "Generational GC ({:?}): collected {} objects, promoted {} in {:.2}ms",
            self.generation,
            self.collected_objects,
            self.promoted_objects,
            self.collection_time.as_secs_f64() * 1000.0
        )
    }
}

/// Global generational collector
static GLOBAL_GENERATIONAL_GC: std::sync::LazyLock<Arc<GenerationalCollector>> = 
    std::sync::LazyLock::new(|| Arc::new(GenerationalCollector::new()));

/// Get the global generational collector
pub fn get_global_generational_gc() -> Arc<GenerationalCollector> {
    Arc::clone(&GLOBAL_GENERATIONAL_GC)
}

/// Compatibility exports
pub use GenerationalCollector as MinimalImplementation;

/// Convenience function for compatibility
pub fn get_minimal_result() -> Result<String, CursedError> {
    let gc = get_global_generational_gc();
    let stats = gc.stats();
    Ok(format!("Generational GC ready - {} young, {} old collections", 
               stats.young_collections, stats.old_collections))
}
