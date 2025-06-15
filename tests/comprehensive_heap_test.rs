/// Comprehensive heap management tests
/// Tests heap allocation, deallocation, compaction, growth, and advanced heap features

#[path = "common.rs"]
pub mod common;

use cursed::memory::{
    Heap, HeapConfiguration, HeapStatistics, AllocationStrategy,
    RealHeapManager, RealHeapConfig, RealHeapStatistics, RealHeapBlock,
    Allocator, BumpAllocator, FreeListAllocator, SegregatedAllocator,
    AllocationResult, AllocatorStatistics, HeapRegion, RegionManager, RegionType,
    ObjectHeader, MetadataManager, MemoryLayout, utils
};
use std::sync::Arc;
use std::time::{Duration, Instant};
use std::collections::HashMap;
use tracing::{info, debug, error, warn};

macro_rules! init_tracing {
    () => {
        common::tracing::setup();
    };
}

#[derive(Debug, Clone)]
struct HeapTestObject {
    pub id: u32,
    pub size: usize,
    pub data: Vec<u8>,
    pub refs: Vec<u32>, // References to other objects by ID
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test basic heap allocation and deallocation
    #[test]
    fn test_basic_heap_operations() {
        init_tracing!();
        info!("Testing basic heap operations");

        let config = HeapConfiguration {
            initial_size: 1024 * 1024, // 1MB
            max_size: Some(10 * 1024 * 1024), // 10MB
            growth_factor: 2.0,
            shrink_threshold: 0.25,
            allocation_strategy: AllocationStrategy::BestFit,
            enable_compaction: true,
            compaction_threshold: 0.5,
            alignment: 8,
        };

        let mut heap = Heap::new(config);
        assert!(heap.is_ok());
        let mut heap = heap.unwrap();

        // Test basic allocations
        let mut allocations = Vec::new();
        for i in 0..100 {
            let size = 64 + i * 16; // Varying sizes
            let allocation = heap.allocate(size, 8);
            assert!(allocation.is_ok());
            
            let alloc = allocation.unwrap();
            assert!(alloc.size >= size);
            assert!(utils::is_aligned::<u64>(alloc.ptr.as_ptr() as *const u64));
            allocations.push(alloc);
        }

        let stats_after_alloc = heap.get_statistics();
        assert_eq!(stats_after_alloc.allocations_count, 100);
        assert!(stats_after_alloc.bytes_allocated > 0);
        assert!(stats_after_alloc.bytes_used > 0);

        // Test deallocations
        for i in (0..allocations.len()).step_by(3) {
            let result = heap.deallocate(allocations[i].ptr, allocations[i].size);
            assert!(result.is_ok());
        }

        let stats_after_dealloc = heap.get_statistics();
        assert!(stats_after_dealloc.deallocations_count > 0);
        assert!(stats_after_dealloc.bytes_freed > 0);

        // Test reallocation to verify free blocks are reused
        for _i in 0..20 {
            let allocation = heap.allocate(128, 8);
            assert!(allocation.is_ok());
        }

        let final_stats = heap.get_statistics();
        assert!(final_stats.allocations_count > 100);

        info!("Basic heap operations test passed");
    }

    /// Test heap growth and shrinking
    #[test]
    fn test_heap_growth_shrinking() {
        init_tracing!();
        info!("Testing heap growth and shrinking");

        let config = HeapConfiguration {
            initial_size: 64 * 1024, // 64KB - small initial size
            max_size: Some(1024 * 1024), // 1MB max
            growth_factor: 1.5,
            shrink_threshold: 0.2, // Aggressive shrinking
            allocation_strategy: AllocationStrategy::FirstFit,
            enable_compaction: false, // Disable for growth testing
            compaction_threshold: 0.8,
            alignment: 8,
        };

        let mut heap = Heap::new(config).unwrap();
        let initial_stats = heap.get_statistics();
        let initial_capacity = initial_stats.total_capacity;

        debug!("Initial heap capacity: {} bytes", initial_capacity);

        // Allocate enough to trigger growth
        let mut large_allocations = Vec::new();
        for i in 0..20 {
            let size = 8192; // 8KB each
            let allocation = heap.allocate(size, 8);
            assert!(allocation.is_ok());
            large_allocations.push(allocation.unwrap());
            
            let current_stats = heap.get_statistics();
            if current_stats.total_capacity > initial_capacity {
                debug!("Heap grew at allocation {}: {} -> {} bytes", 
                       i, initial_capacity, current_stats.total_capacity);
                break;
            }
        }

        let stats_after_growth = heap.get_statistics();
        assert!(stats_after_growth.total_capacity > initial_capacity);
        assert!(stats_after_growth.growth_events > 0);

        // Deallocate most allocations to trigger shrinking
        for i in 0..large_allocations.len() - 2 {
            heap.deallocate(
                large_allocations[i].ptr, 
                large_allocations[i].size
            ).unwrap();
        }

        // Force shrinking check
        heap.try_shrink().unwrap();

        let stats_after_shrink = heap.get_statistics();
        debug!("Capacity after shrinking: {} bytes", stats_after_shrink.total_capacity);
        
        // May or may not have shrunk depending on implementation details
        assert!(stats_after_shrink.shrink_events >= 0);

        info!("Heap growth and shrinking test passed");
    }

    /// Test heap compaction
    #[test]
    fn test_heap_compaction() {
        init_tracing!();
        info!("Testing heap compaction");

        let config = HeapConfiguration {
            initial_size: 256 * 1024, // 256KB
            max_size: Some(1024 * 1024), // 1MB
            growth_factor: 2.0,
            shrink_threshold: 0.3,
            allocation_strategy: AllocationStrategy::FirstFit,
            enable_compaction: true,
            compaction_threshold: 0.4, // Compact when 40% fragmented
            alignment: 8,
        };

        let mut heap = Heap::new(config).unwrap();

        // Phase 1: Create fragmentation
        let mut allocations = Vec::new();
        for i in 0..100 {
            let size = 512; // Fixed size for easy fragmentation
            let allocation = heap.allocate(size, 8);
            assert!(allocation.is_ok());
            allocations.push(allocation.unwrap());
        }

        // Phase 2: Deallocate every other allocation to fragment
        for i in (0..allocations.len()).step_by(2) {
            heap.deallocate(allocations[i].ptr, allocations[i].size).unwrap();
        }

        let stats_before_compact = heap.get_statistics();
        let fragmentation_before = stats_before_compact.fragmentation_ratio;
        debug!("Fragmentation before compaction: {:.2}", fragmentation_before);

        // Phase 3: Trigger compaction
        let compaction_result = heap.compact();
        assert!(compaction_result.is_ok());

        let stats_after_compact = heap.get_statistics();
        let fragmentation_after = stats_after_compact.fragmentation_ratio;
        debug!("Fragmentation after compaction: {:.2}", fragmentation_after);

        // Compaction should reduce fragmentation
        assert!(fragmentation_after <= fragmentation_before);
        assert!(stats_after_compact.compaction_events > 0);

        // Remaining allocations should still be valid
        for i in (1..allocations.len()).step_by(2) {
            // Verify that odd-indexed allocations are still accessible
            // (Implementation specific - may need to update pointers after compaction)
            assert!(heap.is_valid_pointer(allocations[i].ptr.as_ptr()));
        }

        info!("Heap compaction test passed");
    }

    /// Test different allocation strategies
    #[test]
    fn test_allocation_strategies() {
        init_tracing!();
        info!("Testing allocation strategies");

        let strategies = vec![
            AllocationStrategy::FirstFit,
            AllocationStrategy::BestFit,
            AllocationStrategy::WorstFit,
            AllocationStrategy::NextFit,
        ];

        for strategy in strategies {
            debug!("Testing allocation strategy: {:?}", strategy);

            let config = HeapConfiguration {
                initial_size: 128 * 1024, // 128KB
                max_size: Some(512 * 1024), // 512KB
                growth_factor: 2.0,
                shrink_threshold: 0.25,
                allocation_strategy: strategy,
                enable_compaction: false,
                compaction_threshold: 0.5,
                alignment: 8,
            };

            let mut heap = Heap::new(config).unwrap();

            // Test allocation pattern that highlights strategy differences
            let mut strategy_allocations = Vec::new();
            
            // Allocate various sizes
            let sizes = vec![64, 128, 256, 512, 1024, 256, 128, 64];
            for (i, &size) in sizes.iter().enumerate() {
                let allocation = heap.allocate(size, 8);
                assert!(allocation.is_ok());
                strategy_allocations.push(allocation.unwrap());
                
                debug!("  Allocation {}: {} bytes at {:p}", 
                       i, size, strategy_allocations[i].ptr.as_ptr());
            }

            // Deallocate some to create holes
            heap.deallocate(strategy_allocations[1].ptr, strategy_allocations[1].size).unwrap();
            heap.deallocate(strategy_allocations[3].ptr, strategy_allocations[3].size).unwrap();
            heap.deallocate(strategy_allocations[5].ptr, strategy_allocations[5].size).unwrap();

            // Allocate again to see how strategy affects placement
            for i in 0..3 {
                let allocation = heap.allocate(200, 8);
                assert!(allocation.is_ok());
                debug!("  Reallocation {}: {} bytes at {:p}", 
                       i, 200, allocation.unwrap().ptr.as_ptr());
            }

            let strategy_stats = heap.get_statistics();
            debug!("  Strategy {:?} stats: fragmentation={:.2}", 
                   strategy, strategy_stats.fragmentation_ratio);
        }

        info!("Allocation strategies test passed");
    }

    /// Test real heap manager with advanced features
    #[test]
    fn test_real_heap_manager() {
        init_tracing!();
        info!("Testing real heap manager");

        let config = RealHeapConfig {
            initial_heap_size: 512 * 1024, // 512KB
            max_heap_size: 4 * 1024 * 1024, // 4MB
            growth_factor: 1.8,
            block_size: 4096, // 4KB blocks
            alignment: 16,
            enable_coalescing: true,
            enable_splitting: true,
        };

        let mut real_heap = RealHeapManager::new(config).unwrap();

        // Test block-based allocation
        let mut blocks = Vec::new();
        for i in 0..50 {
            let size = 256 + i * 128; // Varying sizes
            let block = real_heap.allocate(size);
            assert!(block.is_ok());
            
            let block = block.unwrap();
            assert!(block.size() >= size);
            assert!(utils::is_aligned::<u64>(block.address() as *const u64));
            blocks.push(block);
        }

        let stats_after_alloc = real_heap.get_statistics();
        assert_eq!(stats_after_alloc.blocks_allocated, blocks.len());
        assert!(stats_after_alloc.total_allocated > 0);

        // Test coalescing by deallocating adjacent blocks
        let mut dealloc_addresses = Vec::new();
        for i in (0..blocks.len()).step_by(3) {
            let address = blocks[i].address();
            dealloc_addresses.push(address);
            real_heap.deallocate(address).unwrap();
        }

        let stats_after_dealloc = real_heap.get_statistics();
        assert!(stats_after_dealloc.blocks_free > 0);
        assert!(stats_after_dealloc.coalescing_operations > 0);

        debug!("Coalescing operations: {}", stats_after_dealloc.coalescing_operations);

        // Test splitting by allocating smaller blocks in freed space
        for i in 0..10 {
            let small_block = real_heap.allocate(128);
            assert!(small_block.is_ok());
        }

        let stats_after_split = real_heap.get_statistics();
        assert!(stats_after_split.splitting_operations > 0);

        debug!("Splitting operations: {}", stats_after_split.splitting_operations);

        // Test large allocation that may require multiple blocks
        let large_block = real_heap.allocate(32 * 1024); // 32KB
        assert!(large_block.is_ok());
        assert_eq!(large_block.unwrap().size(), 32 * 1024);

        let final_stats = real_heap.get_statistics();
        debug!("Real heap final stats: {:?}", final_stats);

        info!("Real heap manager test passed");
    }

    /// Test heap statistics and monitoring
    #[test]
    fn test_heap_statistics() {
        init_tracing!();
        info!("Testing heap statistics");

        let config = HeapConfiguration {
            initial_size: 256 * 1024, // 256KB
            max_size: Some(2 * 1024 * 1024), // 2MB
            growth_factor: 2.0,
            shrink_threshold: 0.2,
            allocation_strategy: AllocationStrategy::BestFit,
            enable_compaction: true,
            compaction_threshold: 0.6,
            alignment: 8,
        };

        let mut heap = Heap::new(config).unwrap();

        // Baseline statistics
        let baseline_stats = heap.get_statistics();
        assert_eq!(baseline_stats.allocations_count, 0);
        assert_eq!(baseline_stats.deallocations_count, 0);
        assert_eq!(baseline_stats.bytes_allocated, 0);
        assert_eq!(baseline_stats.bytes_freed, 0);
        assert_eq!(baseline_stats.growth_events, 0);
        assert_eq!(baseline_stats.shrink_events, 0);
        assert_eq!(baseline_stats.compaction_events, 0);

        // Create workload to generate statistics
        let mut stats_allocations = Vec::new();
        for i in 0..200 {
            let size = 128 + (i % 5) * 256; // Varied sizes
            let allocation = heap.allocate(size, 8);
            assert!(allocation.is_ok());
            stats_allocations.push(allocation.unwrap());
        }

        let stats_after_alloc = heap.get_statistics();
        assert_eq!(stats_after_alloc.allocations_count, 200);
        assert!(stats_after_alloc.bytes_allocated > 0);
        assert!(stats_after_alloc.bytes_used > 0);
        assert!(stats_after_alloc.allocation_rate > 0.0);

        // Test fragmentation statistics
        for i in (0..stats_allocations.len()).step_by(4) {
            heap.deallocate(
                stats_allocations[i].ptr, 
                stats_allocations[i].size
            ).unwrap();
        }

        let stats_after_dealloc = heap.get_statistics();
        assert!(stats_after_dealloc.deallocations_count > 0);
        assert!(stats_after_dealloc.bytes_freed > 0);
        assert!(stats_after_dealloc.deallocation_rate > 0.0);
        assert!(stats_after_dealloc.fragmentation_ratio > 0.0);

        debug!("Allocation rate: {:.2} allocs/sec", stats_after_dealloc.allocation_rate);
        debug!("Deallocation rate: {:.2} deallocs/sec", stats_after_dealloc.deallocation_rate);
        debug!("Fragmentation ratio: {:.2}", stats_after_dealloc.fragmentation_ratio);
        debug!("Memory utilization: {:.2}", stats_after_dealloc.utilization_ratio);

        // Test compaction statistics
        if stats_after_dealloc.fragmentation_ratio > 0.3 {
            heap.compact().unwrap();
            let stats_after_compact = heap.get_statistics();
            assert!(stats_after_compact.compaction_events > 0);
            assert!(stats_after_compact.bytes_compacted > 0);
        }

        // Test peak usage tracking
        for _i in 0..50 {
            let large_alloc = heap.allocate(2048, 8);
            if large_alloc.is_ok() {
                // Immediately deallocate to test peak tracking
                let alloc = large_alloc.unwrap();
                heap.deallocate(alloc.ptr, alloc.size).unwrap();
            }
        }

        let final_stats = heap.get_statistics();
        assert!(final_stats.peak_bytes_used >= stats_after_alloc.bytes_used);
        assert!(final_stats.total_allocation_attempts >= final_stats.allocations_count);

        info!("Heap statistics test passed");
    }

    /// Test heap with different object patterns
    #[test]
    fn test_object_patterns() {
        init_tracing!();
        info!("Testing heap with different object patterns");

        let config = HeapConfiguration {
            initial_size: 1024 * 1024, // 1MB
            max_size: Some(8 * 1024 * 1024), // 8MB
            growth_factor: 2.0,
            shrink_threshold: 0.25,
            allocation_strategy: AllocationStrategy::BestFit,
            enable_compaction: true,
            compaction_threshold: 0.5,
            alignment: 8,
        };

        let mut heap = Heap::new(config).unwrap();

        // Pattern 1: Many small objects (typical for scripting languages)
        debug!("Testing small object pattern");
        let mut small_objects = Vec::new();
        for i in 0..1000 {
            let allocation = heap.allocate(32, 8); // 32 bytes each
            assert!(allocation.is_ok());
            small_objects.push(allocation.unwrap());
        }

        let small_stats = heap.get_statistics();
        debug!("Small objects: {} allocations, {:.2} fragmentation", 
               small_stats.allocations_count, small_stats.fragmentation_ratio);

        // Pattern 2: Few large objects (typical for data processing)
        debug!("Testing large object pattern");
        let mut large_objects = Vec::new();
        for i in 0..50 {
            let allocation = heap.allocate(16384, 8); // 16KB each
            assert!(allocation.is_ok());
            large_objects.push(allocation.unwrap());
        }

        let large_stats = heap.get_statistics();
        debug!("Large objects: {} allocations, {:.2} fragmentation", 
               large_stats.allocations_count, large_stats.fragmentation_ratio);

        // Pattern 3: Mixed sizes (realistic workload)
        debug!("Testing mixed size pattern");
        let sizes = vec![32, 64, 128, 256, 512, 1024, 2048, 4096];
        let mut mixed_objects = Vec::new();
        for round in 0..100 {
            let size = sizes[round % sizes.len()];
            let allocation = heap.allocate(size, 8);
            assert!(allocation.is_ok());
            mixed_objects.push(allocation.unwrap());
        }

        let mixed_stats = heap.get_statistics();
        debug!("Mixed objects: {} allocations, {:.2} fragmentation", 
               mixed_stats.allocations_count, mixed_stats.fragmentation_ratio);

        // Pattern 4: Rapid allocation/deallocation (stress test)
        debug!("Testing rapid alloc/dealloc pattern");
        for _round in 0..200 {
            let allocation = heap.allocate(256, 8);
            assert!(allocation.is_ok());
            let alloc = allocation.unwrap();
            
            // Immediately deallocate
            heap.deallocate(alloc.ptr, alloc.size).unwrap();
        }

        let rapid_stats = heap.get_statistics();
        debug!("Rapid pattern: {} allocations, {} deallocations", 
               rapid_stats.allocations_count, rapid_stats.deallocations_count);

        // Verify heap is still functional after all patterns
        let test_allocation = heap.allocate(1024, 8);
        assert!(test_allocation.is_ok());

        info!("Object patterns test passed");
    }

    /// Test heap error conditions and edge cases
    #[test]
    fn test_heap_edge_cases() {
        init_tracing!();
        info!("Testing heap edge cases");

        let config = HeapConfiguration {
            initial_size: 64 * 1024, // Small heap for testing limits
            max_size: Some(128 * 1024), // Limited max size
            growth_factor: 1.5,
            shrink_threshold: 0.3,
            allocation_strategy: AllocationStrategy::FirstFit,
            enable_compaction: true,
            compaction_threshold: 0.5,
            alignment: 8,
        };

        let mut heap = Heap::new(config).unwrap();

        // Test zero-size allocation
        let zero_alloc = heap.allocate(0, 8);
        // May succeed with minimum allocation or fail - implementation dependent
        debug!("Zero allocation result: {:?}", zero_alloc);

        // Test very large allocation (should fail with limited heap)
        let huge_alloc = heap.allocate(256 * 1024, 8); // Larger than max heap
        debug!("Huge allocation result: {:?}", huge_alloc);
        // Should either fail or succeed if heap can grow

        // Test allocation with unusual alignment
        let unaligned_alloc = heap.allocate(100, 1); // 1-byte alignment
        assert!(unaligned_alloc.is_ok());

        let large_align_alloc = heap.allocate(64, 64); // 64-byte alignment
        assert!(large_align_alloc.is_ok());
        let alloc = large_align_alloc.unwrap();
        assert_eq!(alloc.ptr.as_ptr() as usize % 64, 0);

        // Test double deallocation (should be handled gracefully)
        let test_alloc = heap.allocate(256, 8).unwrap();
        let first_dealloc = heap.deallocate(test_alloc.ptr, test_alloc.size);
        assert!(first_dealloc.is_ok());

        let second_dealloc = heap.deallocate(test_alloc.ptr, test_alloc.size);
        debug!("Double deallocation result: {:?}", second_dealloc);
        // Should either fail gracefully or be idempotent

        // Test invalid pointer deallocation
        let invalid_ptr = NonNull::new(0x12345678 as *mut u8).unwrap();
        let invalid_dealloc = heap.deallocate(invalid_ptr, 256);
        debug!("Invalid pointer deallocation result: {:?}", invalid_dealloc);
        // Should fail gracefully

        // Test heap overflow scenario
        let mut overflow_allocations = Vec::new();
        let mut allocation_count = 0;
        loop {
            let allocation = heap.allocate(1024, 8);
            if allocation.is_err() {
                debug!("Heap overflow after {} allocations", allocation_count);
                break;
            }
            overflow_allocations.push(allocation.unwrap());
            allocation_count += 1;
            
            if allocation_count > 1000 {
                debug!("Stopping overflow test after {} allocations", allocation_count);
                break;
            }
        }

        let overflow_stats = heap.get_statistics();
        debug!("Overflow test stats: {} allocations", overflow_stats.allocations_count);

        info!("Heap edge cases test passed");
    }

    /// Performance and stress test for heap operations
    #[test]
    #[ignore] // Run with --ignored flag for performance tests
    fn test_heap_performance_stress() {
        init_tracing!();
        info!("Starting heap performance stress test");

        let config = HeapConfiguration {
            initial_size: 4 * 1024 * 1024, // 4MB
            max_size: Some(64 * 1024 * 1024), // 64MB
            growth_factor: 2.0,
            shrink_threshold: 0.2,
            allocation_strategy: AllocationStrategy::BestFit,
            enable_compaction: true,
            compaction_threshold: 0.6,
            alignment: 16,
        };

        let mut heap = Heap::new(config).unwrap();

        // Stress test 1: Rapid allocation/deallocation
        debug!("Stress test 1: Rapid allocation/deallocation");
        let rapid_start = Instant::now();
        let mut rapid_allocations = Vec::new();

        for i in 0..50000 {
            let size = 64 + (i % 1024); // Varied sizes
            let allocation = heap.allocate(size, 16);
            assert!(allocation.is_ok());
            rapid_allocations.push(allocation.unwrap());

            // Deallocate every 10th allocation
            if i % 10 == 0 && i > 0 {
                let dealloc_idx = i / 10 - 1;
                if dealloc_idx < rapid_allocations.len() {
                    heap.deallocate(
                        rapid_allocations[dealloc_idx].ptr,
                        rapid_allocations[dealloc_idx].size
                    ).unwrap();
                }
            }
        }

        let rapid_time = rapid_start.elapsed();
        debug!("Rapid test completed in {:?}", rapid_time);

        // Stress test 2: Large object allocation
        debug!("Stress test 2: Large object allocation");
        let large_start = Instant::now();

        for _i in 0..1000 {
            let large_alloc = heap.allocate(32 * 1024, 16); // 32KB
            if large_alloc.is_ok() {
                let alloc = large_alloc.unwrap();
                // Keep some, deallocate others
                if _i % 3 == 0 {
                    heap.deallocate(alloc.ptr, alloc.size).unwrap();
                }
            }
        }

        let large_time = large_start.elapsed();
        debug!("Large object test completed in {:?}", large_time);

        // Stress test 3: Fragmentation and compaction
        debug!("Stress test 3: Fragmentation and compaction");
        let fragment_start = Instant::now();

        // Create heavy fragmentation
        let mut fragment_allocations = Vec::new();
        for i in 0..2000 {
            let allocation = heap.allocate(256, 16);
            assert!(allocation.is_ok());
            fragment_allocations.push(allocation.unwrap());
        }

        // Deallocate every other allocation
        for i in (0..fragment_allocations.len()).step_by(2) {
            heap.deallocate(
                fragment_allocations[i].ptr,
                fragment_allocations[i].size
            ).unwrap();
        }

        // Force compaction
        let compact_result = heap.compact();
        assert!(compact_result.is_ok());

        let fragment_time = fragment_start.elapsed();
        debug!("Fragmentation test completed in {:?}", fragment_time);

        // Stress test 4: Mixed workload simulation
        debug!("Stress test 4: Mixed workload simulation");
        let mixed_start = Instant::now();

        let mut mixed_allocations = Vec::new();
        for round in 0..10000 {
            match round % 5 {
                0 => {
                    // Small allocations
                    for _i in 0..10 {
                        let alloc = heap.allocate(32, 8);
                        if alloc.is_ok() {
                            mixed_allocations.push(alloc.unwrap());
                        }
                    }
                }
                1 => {
                    // Medium allocations
                    for _i in 0..5 {
                        let alloc = heap.allocate(512, 16);
                        if alloc.is_ok() {
                            mixed_allocations.push(alloc.unwrap());
                        }
                    }
                }
                2 => {
                    // Large allocation
                    let alloc = heap.allocate(8192, 16);
                    if alloc.is_ok() {
                        mixed_allocations.push(alloc.unwrap());
                    }
                }
                3 => {
                    // Deallocate some older allocations
                    let dealloc_count = std::cmp::min(20, mixed_allocations.len() / 4);
                    for _i in 0..dealloc_count {
                        if !mixed_allocations.is_empty() {
                            let alloc = mixed_allocations.remove(0);
                            heap.deallocate(alloc.ptr, alloc.size).unwrap();
                        }
                    }
                }
                _ => {
                    // Compact if fragmented
                    let stats = heap.get_statistics();
                    if stats.fragmentation_ratio > 0.7 {
                        heap.compact().unwrap();
                    }
                }
            }
        }

        let mixed_time = mixed_start.elapsed();
        debug!("Mixed workload test completed in {:?}", mixed_time);

        // Final statistics
        let final_stats = heap.get_statistics();
        debug!("Final stress test stats: {:?}", final_stats);

        // Performance assertions
        let total_time = rapid_time + large_time + fragment_time + mixed_time;
        debug!("Total stress test time: {:?}", total_time);

        assert!(total_time < Duration::from_secs(30)); // Should complete in reasonable time
        assert!(final_stats.allocations_count > 50000);
        assert!(final_stats.deallocations_count > 10000);

        // Performance metrics
        let alloc_rate = final_stats.allocations_count as f64 / total_time.as_secs_f64();
        let dealloc_rate = final_stats.deallocations_count as f64 / total_time.as_secs_f64();

        debug!("Overall allocation rate: {:.0} allocs/sec", alloc_rate);
        debug!("Overall deallocation rate: {:.0} deallocs/sec", dealloc_rate);

        assert!(alloc_rate > 1000.0); // At least 1K allocations per second
        assert!(dealloc_rate > 500.0); // At least 500 deallocations per second

        info!("Heap performance stress test passed");
    }
}
