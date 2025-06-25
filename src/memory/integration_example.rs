/// Integration Example for Real Heap Management
/// 
/// This module demonstrates how to integrate the new real heap management
/// system with the existing garbage collection infrastructure.

use std::sync::Arc;
use tracing::{info, debug};

use crate::memory::{
// };
use crate::error::CursedError;
use crate::memory::gc::GcConfig;
use crate::memory::heap_manager::HeapConfig;

/// Example struct that can be stored in the garbage-collected heap
#[derive(Debug, Clone)]
pub struct ExampleObject {
// ExampleObject automatically implements Storable via the blanket impl

impl Traceable for ExampleObject {
    fn trace(&self, visitor: &mut dyn Visitor) {
        // Example objects don't contain GC references in this simple case
        // In a real implementation, you would trace any contained Gc<T> pointers
    }
}

/// Demonstrates real heap allocation and garbage collection
pub fn demonstrate_real_heap_integration() -> Result<(), String> {
    info!("Demonstrating real heap management integration");
    
    // Create enhanced garbage collector with real heap enabled
    let mut enhanced_gc = EnhancedGarbageCollector::with_config(
        true // Enable real heap
    );
    
    info!("Created enhanced GC with real heap management");
    
    // Create some test objects
    let obj1 = ExampleObject {
    
    let obj2 = ExampleObject {
    
    let obj3 = ExampleObject {
    
    // Allocate objects using real heap (falls back to legacy for now)
    info!("Allocating objects...");
    let gc_obj1 = enhanced_gc.allocate_real(obj1)?;
    let gc_obj2 = enhanced_gc.allocate_real(obj2)?;
    let gc_obj3 = enhanced_gc.allocate_real(obj3)?;
    
    debug!("Allocated object 1: id = {}", gc_obj1.id);
    debug!("Allocated object 2: id = {}", gc_obj2.id);
    debug!("Allocated object 3: id = {}", gc_obj3.id);
    
    // Get comprehensive statistics
    let stats_before = enhanced_gc.get_comprehensive_stats_enhanced()?;
          stats_before.total_collections, stats_before.total_objects_collected);
    
    // Check if collection should be triggered
    if let Some(trigger) = enhanced_gc.should_collect_enhanced()? {
        info!("Collection trigger detected: {:?}", trigger);
    } else {
        info!("No collection trigger detected");
    // Trigger enhanced garbage collection
    info!("Triggering enhanced garbage collection...");
    let collection_stats = enhanced_gc.collect_enhanced()?;
    
          collection_stats.bytes_collected);
    
    // Get statistics after collection
    let stats_after = enhanced_gc.get_comprehensive_stats_enhanced()?;
          stats_after.total_collections, stats_after.total_objects_collected);
    
    // Verify objects are still accessible
    debug!("Verifying object access after collection...");
    debug!("Object 1 data: {}", gc_obj1.data);
    debug!("Object 2 data: {}", gc_obj2.data);
    debug!("Object 3 data: {}", gc_obj3.data);
    
    info!("Real heap integration demonstration completed successfully");
    Ok(())
/// Demonstrates standalone real heap manager usage
pub fn demonstrate_standalone_real_heap() -> Result<(), String> {
    info!("Demonstrating standalone real heap manager");
    
    // Create real heap manager directly
    let config = RealHeapConfig {
        initial_block_size: 1024 * 1024, // 1MB
    
    let object_registry = Arc::new(ObjectRegistry::new());
    let heap_manager = RealHeapManager::new(config, object_registry)?;
    
    info!("Created real heap manager");
    
    // Allocate some memory blocks
    let allocations = vec![
    ];
    
    let mut allocated_objects = Vec::new();
    
    for (name, size) in allocations {
        let (object_id, ptr) = heap_manager.allocate(size, 8, &format!("test_{}", name))?;
        info!("Allocated {} bytes for {} (object {})", size, name, object_id);
        allocated_objects.push((object_id, ptr, size));
    // Get heap statistics
    let stats = heap_manager.get_statistics()?;
    info!("Heap stats: {} blocks, {}/{} bytes used, {:.2}% fragmentation",
          stats.overall_fragmentation * 100.0);
    
    // Deallocate some objects
    for (i, (object_id, ptr, size)) in allocated_objects.iter().enumerate() {
        if i % 2 == 0 { // Deallocate every other object
            heap_manager.deallocate(*object_id, *ptr)?;
            info!("Deallocated object {} ({} bytes)", object_id, size);
        }
    }
    
    // Get updated statistics
    let stats_after = heap_manager.get_statistics()?;
    info!("Stats after deallocation: {} blocks, {}/{} bytes used, {:.2}% fragmentation",
          stats_after.overall_fragmentation * 100.0);
    
    // Test memory pressure monitoring
    let pressure = heap_manager.get_memory_pressure();
    info!("Current memory pressure: {:.2}", pressure);
    
    // Test compaction
    if stats_after.overall_fragmentation > 0.3 {
        info!("Triggering compaction due to fragmentation");
        heap_manager.trigger_compaction()?;
        
        let stats_compacted = heap_manager.get_statistics()?;
              stats_compacted.overall_fragmentation * 100.0);
    info!("Standalone real heap demonstration completed successfully");
    Ok(())
/// Performance comparison between real heap and legacy heap
pub fn compare_heap_performance() -> Result<(), String> {
    info!("Comparing heap performance");
    
    use std::time::Instant;
    
    // Test with real heap
    let start_real = Instant::now();
    let enhanced_gc_real = EnhancedGarbageCollector::with_config(
        true // Real heap
    );
    
    let mut real_objects = Vec::new();
    for i in 0..100 {
        let obj = ExampleObject {
        let gc_obj = enhanced_gc_real.allocate_real(obj)?;
        real_objects.push(gc_obj);
    }
    let real_time = start_real.elapsed();
    
    // Test with legacy heap
    let start_legacy = Instant::now();
    let enhanced_gc_legacy = EnhancedGarbageCollector::with_config(
        false // Legacy heap
    );
    
    let mut legacy_objects = Vec::new();
    for i in 0..100 {
        let obj = ExampleObject {
        let gc_obj = enhanced_gc_legacy.allocate_real(obj)?; // Will fall back to legacy
        legacy_objects.push(gc_obj);
    }
    let legacy_time = start_legacy.elapsed();
    
    info!("Performance comparison:");
    info!("  Real heap: {:?} for 100 allocations", real_time);
    info!("  Legacy heap: {:?} for 100 allocations", legacy_time);
    
    let ratio = real_time.as_nanos() as f64 / legacy_time.as_nanos() as f64;
    info!("  Real/Legacy ratio: {:.2}x", ratio);
    
    // Get final statistics
    let real_stats = enhanced_gc_real.get_comprehensive_stats_enhanced()?;
    let legacy_stats = enhanced_gc_legacy.get_comprehensive_stats_enhanced()?;
    
          real_stats.heap_stats.total_capacity, real_stats.heap_stats.used_before);
          legacy_stats.heap_stats.total_capacity, legacy_stats.heap_stats.used_before);
    
    info!("Performance comparison completed");
    Ok(())
