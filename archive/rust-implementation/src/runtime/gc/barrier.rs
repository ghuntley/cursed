//! Generational garbage collection write barriers for CURSED runtime
//!
//! This module implements write barriers that track cross-generational references
//! and trigger minor collections appropriately while maintaining memory safety.
//! Optimized for young object allocation patterns.

use crate::error::CursedError;
use crate::memory::heap::{ObjectId, get_global_heap};
use crate::memory::generational::{get_global_generational_gc, GenerationType};
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex, RwLock, atomic::{AtomicUsize, AtomicBool, Ordering}};
use std::sync::atomic::AtomicPtr;
use std::time::{Duration, Instant};

/// Write barrier implementation for generational garbage collection
pub struct GenerationalWriteBarrier {
    /// Remembered set tracking old->young references
    remembered_set: Arc<RwLock<HashSet<CrossGenerationalRef>>>,
    /// Card table for efficient write barrier checks
    card_table: Arc<RwLock<CardTable>>,
    /// Statistics and monitoring
    stats: Arc<Mutex<WriteBarrierStats>>,
    /// Configuration
    config: Arc<RwLock<WriteBarrierConfig>>,
    /// Collection trigger state
    collection_state: Arc<CollectionState>,
}

/// Cross-generational reference tracking
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CrossGenerationalRef {
    /// Old generation object ID
    old_object: ObjectId,
    /// Young generation object ID
    young_object: ObjectId,
    /// Field offset in old object
    field_offset: usize,
    /// Reference timestamp for aging
    timestamp: u64,
}

/// Card table for efficient write barrier implementation
#[derive(Debug)]
pub struct CardTable {
    /// Card bits (one bit per card)
    cards: Vec<AtomicBool>,
    /// Card size in bytes (typically 512 bytes)
    card_size: usize,
    /// Base address of managed heap
    heap_base: usize,
    /// Size of managed heap
    heap_size: usize,
}

/// Write barrier configuration
#[derive(Debug, Clone)]
pub struct WriteBarrierConfig {
    /// Card size for card table (default 512 bytes)
    pub card_size: usize,
    /// Minor collection trigger threshold
    pub minor_collection_threshold: usize,
    /// Maximum remembered set size before forced collection
    pub max_remembered_set_size: usize,
    /// Enable card table optimization
    pub enable_card_table: bool,
    /// Enable generational collection
    pub enable_generational: bool,
}

impl Default for WriteBarrierConfig {
    fn default() -> Self {
        Self {
            card_size: 512,
            minor_collection_threshold: 1024 * 1024, // 1MB
            max_remembered_set_size: 10000,
            enable_card_table: true,
            enable_generational: true,
        }
    }
}

/// Write barrier statistics
#[derive(Debug, Clone, Default)]
pub struct WriteBarrierStats {
    /// Total write barrier invocations
    pub total_barriers: u64,
    /// Cross-generational writes detected
    pub cross_gen_writes: u64,
    /// Card table updates
    pub card_updates: u64,
    /// Minor collections triggered
    pub minor_collections_triggered: u64,
    /// Average barrier overhead in nanoseconds
    pub avg_barrier_overhead_ns: u64,
    /// Remembered set size
    pub remembered_set_size: usize,
}

/// Collection state for triggering minor collections
#[derive(Debug)]
pub struct CollectionState {
    /// Young generation size
    young_gen_size: AtomicUsize,
    /// Number of cross-generational references
    cross_gen_refs: AtomicUsize,
    /// Flag indicating collection in progress
    collection_in_progress: AtomicBool,
    /// Last collection timestamp
    last_collection: Mutex<Instant>,
}

impl CardTable {
    /// Create new card table for given heap range
    pub fn new(heap_base: usize, heap_size: usize, card_size: usize) -> Self {
        let num_cards = (heap_size + card_size - 1) / card_size;
        let cards = (0..num_cards).map(|_| AtomicBool::new(false)).collect();
        
        Self {
            cards,
            card_size,
            heap_base,
            heap_size,
        }
    }
    
    /// Get card index for given address
    fn get_card_index(&self, address: usize) -> Option<usize> {
        if address < self.heap_base || address >= self.heap_base + self.heap_size {
            return None;
        }
        Some((address - self.heap_base) / self.card_size)
    }
    
    /// Mark card as dirty
    pub fn mark_card(&self, address: usize) -> bool {
        if let Some(card_idx) = self.get_card_index(address) {
            if card_idx < self.cards.len() {
                self.cards[card_idx].store(true, Ordering::Relaxed);
                return true;
            }
        }
        false
    }
    
    /// Check if card is dirty
    pub fn is_card_dirty(&self, address: usize) -> bool {
        if let Some(card_idx) = self.get_card_index(address) {
            if card_idx < self.cards.len() {
                return self.cards[card_idx].load(Ordering::Relaxed);
            }
        }
        false
    }
    
    /// Clear all dirty cards
    pub fn clear_dirty_cards(&self) {
        for card in &self.cards {
            card.store(false, Ordering::Relaxed);
        }
    }
    
    /// Get list of dirty card indices
    pub fn get_dirty_cards(&self) -> Vec<usize> {
        let mut dirty_cards = Vec::new();
        for (idx, card) in self.cards.iter().enumerate() {
            if card.load(Ordering::Relaxed) {
                dirty_cards.push(idx);
            }
        }
        dirty_cards
    }
}

impl CollectionState {
    pub fn new() -> Self {
        Self {
            young_gen_size: AtomicUsize::new(0),
            cross_gen_refs: AtomicUsize::new(0),
            collection_in_progress: AtomicBool::new(false),
            last_collection: Mutex::new(Instant::now()),
        }
    }
    
    /// Check if minor collection should be triggered
    pub fn should_trigger_minor_collection(&self, threshold: usize) -> bool {
        let young_size = self.young_gen_size.load(Ordering::Relaxed);
        let cross_refs = self.cross_gen_refs.load(Ordering::Relaxed);
        let in_progress = self.collection_in_progress.load(Ordering::Relaxed);
        
        !in_progress && (young_size >= threshold || cross_refs >= threshold / 10)
    }
    
    /// Mark collection as in progress
    pub fn start_collection(&self) -> bool {
        self.collection_in_progress.compare_exchange(
            false, true, Ordering::AcqRel, Ordering::Relaxed
        ).is_ok()
    }
    
    /// Mark collection as complete
    pub fn finish_collection(&self) {
        self.collection_in_progress.store(false, Ordering::Release);
        *self.last_collection.lock().unwrap() = Instant::now();
    }
}

impl GenerationalWriteBarrier {
    /// Create new write barrier with default configuration
    pub fn new() -> Self {
        let config = WriteBarrierConfig::default();
        let heap_base = 0x1000_0000; // Example heap base
        let heap_size = 256 * 1024 * 1024; // 256MB heap
        
        Self {
            remembered_set: Arc::new(RwLock::new(HashSet::new())),
            card_table: Arc::new(RwLock::new(CardTable::new(heap_base, heap_size, config.card_size))),
            stats: Arc::new(Mutex::new(WriteBarrierStats::default())),
            config: Arc::new(RwLock::new(config)),
            collection_state: Arc::new(CollectionState::new()),
        }
    }
    
    /// Configure write barrier
    pub fn configure(&self, config: WriteBarrierConfig) {
        let mut current_config = self.config.write().unwrap();
        *current_config = config;
    }
    
    /// Core write barrier implementation - called on every pointer write
    /// This is the critical path that must be optimized for performance
    pub fn write_barrier_slow_path(
        &self,
        object_addr: usize,
        field_offset: usize,
        new_value: usize,
        old_value: usize,
    ) -> Result<(), CursedError> {
        let start_time = Instant::now();
        let config = self.config.read().unwrap();
        
        // Fast path: if generational collection is disabled, exit early
        if !config.enable_generational {
            return Ok(());
        }
        
        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.total_barriers += 1;
        }
        
        // Check if this is a cross-generational reference
        if self.is_cross_generational_write(object_addr, new_value)? {
            self.handle_cross_generational_write(object_addr, field_offset, new_value)?;
        }
        
        // Update card table if enabled
        if config.enable_card_table {
            let card_table = self.card_table.read().unwrap();
            card_table.mark_card(object_addr);
            
            let mut stats = self.stats.lock().unwrap();
            stats.card_updates += 1;
        }
        
        // Check if minor collection should be triggered
        if self.should_trigger_minor_collection()? {
            self.trigger_minor_collection()?;
        }
        
        // Update barrier overhead statistics
        let overhead = start_time.elapsed();
        let mut stats = self.stats.lock().unwrap();
        stats.avg_barrier_overhead_ns = 
            (stats.avg_barrier_overhead_ns + overhead.as_nanos() as u64) / 2;
        
        Ok(())
    }
    
    /// Fast inline write barrier check - optimized for common case
    #[inline(always)]
    pub fn write_barrier_fast_path(
        &self,
        object_addr: usize,
        new_value: usize,
    ) -> bool {
        // Fast generation check - if both objects are in same generation, no barrier needed
        let config = self.config.read().unwrap();
        if !config.enable_generational {
            return false;
        }
        
        // Quick generation check (this would be optimized with actual generation bits)
        let object_gen = self.get_object_generation(object_addr);
        let value_gen = self.get_object_generation(new_value);
        
        // Only need slow path if old->young reference
        object_gen == Generation::Old && value_gen == Generation::Young
    }
    
    /// Handle cross-generational write
    fn handle_cross_generational_write(
        &self,
        object_addr: usize,
        field_offset: usize,
        target_addr: usize,
    ) -> Result<(), CursedError> {
        let old_object = ObjectId(object_addr as u64);
        let young_object = ObjectId(target_addr as u64);
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        
        let cross_ref = CrossGenerationalRef {
            old_object,
            young_object,
            field_offset,
            timestamp,
        };
        
        // Add to remembered set
        {
            let mut remembered_set = self.remembered_set.write().unwrap();
            remembered_set.insert(cross_ref);
        }
        
        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.cross_gen_writes += 1;
            stats.remembered_set_size = self.remembered_set.read().unwrap().len();
        }
        
        // Update collection state
        self.collection_state.cross_gen_refs.fetch_add(1, Ordering::Relaxed);
        
        // Check if remembered set is getting too large
        let config = self.config.read().unwrap();
        if stats.remembered_set_size >= config.max_remembered_set_size {
            drop(config);
            self.trigger_minor_collection()?;
        }
        
        Ok(())
    }
    
    /// Check if write is cross-generational (old->young)
    fn is_cross_generational_write(&self, object_addr: usize, target_addr: usize) -> Result<bool, CursedError> {
        if target_addr == 0 {
            return Ok(false); // Null pointer
        }
        
        let object_gen = self.get_object_generation(object_addr);
        let target_gen = self.get_object_generation(target_addr);
        
        Ok(object_gen == Generation::Old && target_gen == Generation::Young)
    }
    
    /// Get object generation (simplified - would use actual object headers)
    fn get_object_generation(&self, addr: usize) -> Generation {
        // This is a placeholder - in reality would check object header
        // For now, assume addresses < 0x2000_0000 are young generation
        if addr < 0x2000_0000 {
            Generation::Young
        } else {
            Generation::Old
        }
    }
    
    /// Check if minor collection should be triggered
    fn should_trigger_minor_collection(&self) -> Result<bool, CursedError> {
        let config = self.config.read().unwrap();
        Ok(self.collection_state.should_trigger_minor_collection(
            config.minor_collection_threshold
        ))
    }
    
    /// Trigger minor collection
    fn trigger_minor_collection(&self) -> Result<(), CursedError> {
        if !self.collection_state.start_collection() {
            return Ok(()); // Collection already in progress
        }
        
        // Update statistics
        {
            let mut stats = self.stats.lock().unwrap();
            stats.minor_collections_triggered += 1;
        }
        
        // Get generational collector and perform minor collection
        let gc = get_global_generational_gc();
        match gc.collect_young() {
            Ok(result) => {
                // Clear remembered set entries for collected objects
                self.clean_remembered_set_after_collection()?;
                
                // Clear dirty cards
                if self.config.read().unwrap().enable_card_table {
                    let card_table = self.card_table.read().unwrap();
                    card_table.clear_dirty_cards();
                }
                
                // Reset collection state
                self.collection_state.young_gen_size.store(0, Ordering::Relaxed);
                self.collection_state.cross_gen_refs.store(0, Ordering::Relaxed);
                
                println!("Minor GC completed: {}", result);
                Ok(())
            }
            Err(e) => {
                self.collection_state.finish_collection();
                Err(e)
            }
        }?;
        
        self.collection_state.finish_collection();
        Ok(())
    }
    
    /// Clean remembered set after collection
    fn clean_remembered_set_after_collection(&self) -> Result<(), CursedError> {
        let mut remembered_set = self.remembered_set.write().unwrap();
        
        // Remove entries where young object was collected (simplified)
        // In practice, this would check if objects are still alive
        remembered_set.retain(|cross_ref| {
            // Keep references to objects that survived collection
            // This is simplified - would check actual object liveness
            true
        });
        
        // Update statistics
        let mut stats = self.stats.lock().unwrap();
        stats.remembered_set_size = remembered_set.len();
        
        Ok(())
    }
    
    /// Get current statistics
    pub fn stats(&self) -> WriteBarrierStats {
        self.stats.lock().unwrap().clone()
    }
    
    /// Get remembered set size
    pub fn remembered_set_size(&self) -> usize {
        self.remembered_set.read().unwrap().len()
    }
    
    /// Get dirty card count
    pub fn dirty_card_count(&self) -> usize {
        let card_table = self.card_table.read().unwrap();
        card_table.get_dirty_cards().len()
    }
}

/// Generation enumeration for compatibility
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Generation {
    Young,
    Old,
}

impl Default for GenerationalWriteBarrier {
    fn default() -> Self {
        Self::new()
    }
}

/// Global write barrier instance
static GLOBAL_WRITE_BARRIER: std::sync::LazyLock<Arc<GenerationalWriteBarrier>> = 
    std::sync::LazyLock::new(|| Arc::new(GenerationalWriteBarrier::new()));

/// Get the global write barrier
pub fn get_global_write_barrier() -> Arc<GenerationalWriteBarrier> {
    Arc::clone(&GLOBAL_WRITE_BARRIER)
}

/// Convenience function for common write barrier call
pub fn write_barrier(object_addr: usize, field_offset: usize, new_value: usize, old_value: usize) -> Result<(), CursedError> {
    let barrier = get_global_write_barrier();
    
    // Fast path check
    if !barrier.write_barrier_fast_path(object_addr, new_value) {
        return Ok(());
    }
    
    // Slow path
    barrier.write_barrier_slow_path(object_addr, field_offset, new_value, old_value)
}

/// High-performance inline write barrier macro for LLVM-generated code
#[macro_export]
macro_rules! inline_write_barrier {
    ($obj_addr:expr, $field_offset:expr, $new_val:expr, $old_val:expr) => {
        {
            // Inline fast path check
            if ($obj_addr >= 0x2000_0000) && ($new_val < 0x2000_0000) && ($new_val != 0) {
                // This is an old->young reference, call slow path
                $crate::runtime::gc::barrier::write_barrier($obj_addr, $field_offset, $new_val, $old_val)?;
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_write_barrier_creation() {
        let barrier = GenerationalWriteBarrier::new();
        assert_eq!(barrier.remembered_set_size(), 0);
    }
    
    #[test]
    fn test_cross_generational_detection() {
        let barrier = GenerationalWriteBarrier::new();
        
        // Old object -> Young object should be detected
        let old_addr = 0x3000_0000;
        let young_addr = 0x1000_0000;
        
        assert!(barrier.is_cross_generational_write(old_addr, young_addr).unwrap());
        
        // Same generation should not be detected
        assert!(!barrier.is_cross_generational_write(young_addr, young_addr).unwrap());
    }
    
    #[test]
    fn test_card_table() {
        let card_table = CardTable::new(0x1000_0000, 1024 * 1024, 512);
        
        let addr = 0x1000_0200;
        assert!(!card_table.is_card_dirty(addr));
        
        card_table.mark_card(addr);
        assert!(card_table.is_card_dirty(addr));
        
        card_table.clear_dirty_cards();
        assert!(!card_table.is_card_dirty(addr));
    }
    
    #[test]
    fn test_remembered_set() {
        let barrier = GenerationalWriteBarrier::new();
        
        // Simulate cross-generational write
        let old_addr = 0x3000_0000;
        let young_addr = 0x1000_0000;
        
        barrier.handle_cross_generational_write(old_addr, 0, young_addr).unwrap();
        assert_eq!(barrier.remembered_set_size(), 1);
        
        let stats = barrier.stats();
        assert_eq!(stats.cross_gen_writes, 1);
    }
}
