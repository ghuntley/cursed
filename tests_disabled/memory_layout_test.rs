/// Memory layout and alignment tests
/// Tests memory allocation patterns, alignment, fragmentation, and layout optimization

#[path = "common.rs"]
pub mod common;

use cursed::memory::{
    HeapManager, HeapConfig, ObjectRegistry, ObjectStore, RealHeapManager, 
    RealHeapConfig, Allocator, BumpAllocator, FreeListAllocator, SegregatedAllocator,
    AllocationResult, AllocationStrategy, HeapRegion, RegionManager, RegionType,
    ObjectHeader, MetadataManager, MemoryLayout, utils
};
use std::sync::Arc;
use std::ptr::NonNull;
use std::mem::{size_of, align_of};
use tracing::{info, debug, error, warn};

macro_rules! init_tracing {
    () => {
        common::tracing::setup();
    };
}

#[derive(Debug, Clone)]
struct LayoutTestObject {
    pub id: u32,
    pub size: usize,
    pub align: usize,
    pub data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Test basic memory alignment requirements
    #[test]
    fn test_memory_alignment() {
        init_tracing!();
        info!("Testing memory alignment");

        // Test alignment utilities
        assert_eq!(utils::align_of::<u8>(), 1);
        assert_eq!(utils::align_of::<u16>(), 2);
        assert_eq!(utils::align_of::<u32>(), 4);
        assert_eq!(utils::align_of::<u64>(), 8);

        // Test size calculations
        assert_eq!(utils::size_of::<u8>(), 1);
        assert_eq!(utils::size_of::<u32>(), 4);
        assert_eq!(utils::size_of::<u64>(), 8);

        // Test alignment size calculations
        assert_eq!(utils::align_size(5, 8), 8);
        assert_eq!(utils::align_size(8, 8), 8);
        assert_eq!(utils::align_size(9, 8), 16);
        assert_eq!(utils::align_size(15, 16), 16);
        assert_eq!(utils::align_size(17, 16), 32);

        // Test pointer alignment checking
        let aligned_ptr = 0x1000 as *const u64; // 8-byte aligned
        let unaligned_ptr = 0x1001 as *const u64; // Not 8-byte aligned
        
        assert!(utils::is_aligned(aligned_ptr));
        assert!(!utils::is_aligned(unaligned_ptr));

        info!("Memory alignment test passed");
    }

    /// Test heap manager memory layout
    #[test]
    fn test_heap_layout() {
        init_tracing!();
        info!("Testing heap layout");

        let config = HeapConfig {
            initial_size: 1024 * 1024, // 1MB
            max_size: Some(10 * 1024 * 1024), // 10MB
            growth_factor: 2.0,
            shrink_threshold: 0.25,
            enable_compaction: true,
            alignment: 8,
        };

        let registry = Arc::new(ObjectRegistry::new());
        let heap = HeapManager::new(config, registry);

        // Test various allocation sizes and alignments
        let test_cases = vec![
            (8, "u64"),
            (16, "small struct"),
            (64, "medium object"),
            (256, "large object"),
            (1024, "very large object"),
        ];

        let mut allocations = Vec::new();

        for (size, description) in test_cases {
            debug!("Testing allocation: {} bytes ({})", size, description);
            
            let result = heap.allocate::<u8>(size, description);
            assert!(result.is_ok());
            
            let (id, ptr) = result.unwrap();
            assert!(!id.is_null());
            assert!(heap.is_valid_pointer(ptr.as_ptr()));
            
            // Check alignment
            assert!(utils::is_aligned::<u64>(ptr.as_ptr() as *const u64));
            
            allocations.push((id, ptr, size));
        }

        // Verify heap statistics
        let stats = heap.get_stats().unwrap();
        assert_eq!(stats.active_objects, allocations.len());
        assert!(stats.total_used > 0);
        assert!(stats.total_allocated >= stats.total_used);

        // Test fragmentation
        let fragmentation = utils::fragmentation_ratio(
            stats.total_free, 
            stats.largest_free_block
        );
        debug!("Heap fragmentation ratio: {:.2}", fragmentation);
        assert!(fragmentation >= 0.0 && fragmentation <= 1.0);

        info!("Heap layout test passed");
    }

    /// Test different allocator strategies
    #[test]
    fn test_allocator_strategies() {
        init_tracing!();
        info!("Testing allocator strategies");

        // Test bump allocator
        let bump_config = 4096; // 4KB bump region
        let mut bump_allocator = BumpAllocator::new(bump_config);
        
        // Allocate sequential blocks
        for i in 0..10 {
            let size = 64 + i * 32; // Varying sizes
            let result = bump_allocator.allocate(size, 8);
            assert!(result.is_ok());
            
            let ptr = result.unwrap().ptr;
            assert!(utils::is_aligned::<u64>(ptr.as_ptr() as *const u64));
        }

        let bump_stats = bump_allocator.get_statistics();
        assert!(bump_stats.allocations_count > 0);
        assert!(bump_stats.bytes_allocated > 0);
        debug!("Bump allocator stats: {:?}", bump_stats);

        // Test free list allocator
        let freelist_config = 8192; // 8KB free list region
        let mut freelist_allocator = FreeListAllocator::new(freelist_config);

        // Allocate and deallocate to create free list
        let mut ptrs = Vec::new();
        for i in 0..20 {
            let size = 128;
            let result = freelist_allocator.allocate(size, 8);
            assert!(result.is_ok());
            ptrs.push(result.unwrap().ptr);
        }

        // Deallocate every other allocation to fragment memory
        for i in (0..ptrs.len()).step_by(2) {
            freelist_allocator.deallocate(ptrs[i], 128);
        }

        // Allocate again - should reuse freed blocks
        for _i in 0..5 {
            let result = freelist_allocator.allocate(128, 8);
            assert!(result.is_ok());
        }

        let freelist_stats = freelist_allocator.get_statistics();
        assert!(freelist_stats.deallocations_count > 0);
        debug!("Free list allocator stats: {:?}", freelist_stats);

        // Test segregated allocator
        let size_classes = vec![32, 64, 128, 256, 512, 1024];
        let mut segregated_allocator = SegregatedAllocator::new(size_classes);

        // Test allocations in different size classes
        for &size_class in &[32, 64, 128, 256, 512, 1024] {
            let result = segregated_allocator.allocate(size_class, 8);
            assert!(result.is_ok());
            
            let allocation = result.unwrap();
            assert!(allocation.actual_size >= size_class);
            assert!(utils::is_aligned::<u64>(allocation.ptr.as_ptr() as *const u64));
        }

        let segregated_stats = segregated_allocator.get_statistics();
        assert!(segregated_stats.size_classes_used > 0);
        debug!("Segregated allocator stats: {:?}", segregated_stats);

        info!("Allocator strategies test passed");
    }

    /// Test heap regions and organization
    #[test]
    fn test_heap_regions() {
        init_tracing!();
        info!("Testing heap regions");

        let young_region = HeapRegion::new(RegionType::YoungGeneration, 512 * 1024); // 512KB
        let old_region = HeapRegion::new(RegionType::OldGeneration, 2 * 1024 * 1024); // 2MB
        let large_region = HeapRegion::new(RegionType::LargeObjectSpace, 1024 * 1024); // 1MB

        // Test region properties
        assert_eq!(young_region.region_type(), RegionType::YoungGeneration);
        assert_eq!(young_region.capacity(), 512 * 1024);
        assert_eq!(young_region.used(), 0);
        assert!(young_region.available() > 0);

        assert_eq!(old_region.region_type(), RegionType::OldGeneration);
        assert_eq!(old_region.capacity(), 2 * 1024 * 1024);

        assert_eq!(large_region.region_type(), RegionType::LargeObjectSpace);
        assert_eq!(large_region.capacity(), 1024 * 1024);

        // Test region manager
        let mut region_manager = RegionManager::new();
        region_manager.add_region(young_region);
        region_manager.add_region(old_region);
        region_manager.add_region(large_region);

        let total_capacity = region_manager.total_capacity();
        let total_used = region_manager.total_used();
        let region_count = region_manager.region_count();

        assert_eq!(region_count, 3);
        assert_eq!(total_capacity, 512 * 1024 + 2 * 1024 * 1024 + 1024 * 1024);
        assert_eq!(total_used, 0);

        // Test allocation strategies by region type
        let young_strategy = region_manager.get_allocation_strategy(RegionType::YoungGeneration);
        let old_strategy = region_manager.get_allocation_strategy(RegionType::OldGeneration);
        let large_strategy = region_manager.get_allocation_strategy(RegionType::LargeObjectSpace);

        assert_eq!(young_strategy, AllocationStrategy::BumpPointer);
        assert_eq!(old_strategy, AllocationStrategy::FreeList);
        assert_eq!(large_strategy, AllocationStrategy::BestFit);

        let stats = region_manager.get_statistics();
        assert_eq!(stats.total_regions, 3);
        assert!(stats.fragmentation_ratio >= 0.0);

        info!("Heap regions test passed");
    }

    /// Test object metadata and headers
    #[test]
    fn test_object_metadata() {
        init_tracing!();
        info!("Testing object metadata");

        let mut metadata_manager = MetadataManager::new();

        // Test object header creation
        let header1 = ObjectHeader::new(1001, 256, 8, "test_object");
        assert_eq!(header1.object_id(), 1001);
        assert_eq!(header1.size(), 256);
        assert_eq!(header1.alignment(), 8);
        assert_eq!(header1.type_name(), "test_object");
        assert!(header1.is_allocated());
        assert!(!header1.is_marked());

        // Test header state changes
        let mut header2 = ObjectHeader::new(1002, 512, 16, "large_object");
        header2.mark();
        assert!(header2.is_marked());

        header2.unmark();
        assert!(!header2.is_marked());

        header2.set_forwarding_address(0x2000);
        assert_eq!(header2.forwarding_address(), Some(0x2000));

        // Test metadata management
        metadata_manager.register_object(header1);
        metadata_manager.register_object(header2);

        let metadata1 = metadata_manager.get_metadata(1001);
        assert!(metadata1.is_some());
        assert_eq!(metadata1.unwrap().object_id(), 1001);

        let metadata2 = metadata_manager.get_metadata(1002);
        assert!(metadata2.is_some());
        assert_eq!(metadata2.unwrap().size(), 512);

        // Test metadata statistics
        let metadata_stats = metadata_manager.get_statistics();
        assert_eq!(metadata_stats.objects_tracked, 2);
        assert!(metadata_stats.metadata_overhead > 0);

        // Test memory layout calculations
        let layout = MemoryLayout::new(1024, 8);
        assert_eq!(layout.size(), 1024);
        assert_eq!(layout.alignment(), 8);
        assert_eq!(layout.aligned_size(), utils::align_size(1024, 8));

        let layout_with_header = layout.with_header();
        assert!(layout_with_header.size() > layout.size());
        assert_eq!(layout_with_header.alignment(), layout.alignment());

        info!("Object metadata test passed");
    }

    /// Test memory fragmentation patterns
    #[test]
    fn test_fragmentation_patterns() {
        init_tracing!();
        info!("Testing fragmentation patterns");

        let config = HeapConfig {
            initial_size: 64 * 1024, // 64KB
            max_size: Some(256 * 1024), // 256KB
            growth_factor: 1.5,
            shrink_threshold: 0.3,
            enable_compaction: false, // Disable to observe fragmentation
            alignment: 8,
        };

        let registry = Arc::new(ObjectRegistry::new());
        let heap = HeapManager::new(config, registry);

        // Phase 1: Allocate many small objects
        let mut small_allocations = Vec::new();
        for i in 0..100 {
            let result = heap.allocate::<u8>(64, &format!("small_{}", i));
            assert!(result.is_ok());
            small_allocations.push(result.unwrap());
        }

        let stats_after_small = heap.get_stats().unwrap();
        debug!("After small allocations: used={}, free={}", 
               stats_after_small.total_used, stats_after_small.total_free);

        // Phase 2: Deallocate every other small object to create holes
        for i in (0..small_allocations.len()).step_by(2) {
            let (id, _) = small_allocations[i];
            heap.deallocate(id).unwrap();
        }

        let stats_after_dealloc = heap.get_stats().unwrap();
        let fragmentation_1 = utils::fragmentation_ratio(
            stats_after_dealloc.total_free,
            stats_after_dealloc.largest_free_block
        );
        debug!("After deallocations: fragmentation={:.2}", fragmentation_1);

        // Phase 3: Try to allocate larger objects (should show fragmentation effects)
        let mut large_allocations = Vec::new();
        for i in 0..20 {
            let result = heap.allocate::<u8>(256, &format!("large_{}", i));
            if result.is_ok() {
                large_allocations.push(result.unwrap());
            } else {
                debug!("Large allocation {} failed due to fragmentation", i);
            }
        }

        let final_stats = heap.get_stats().unwrap();
        let final_fragmentation = utils::fragmentation_ratio(
            final_stats.total_free,
            final_stats.largest_free_block
        );
        debug!("Final fragmentation: {:.2}", final_fragmentation);

        // Should show increased fragmentation after deallocations
        assert!(fragmentation_1 > 0.1); // Some fragmentation should occur
        assert!(final_stats.active_objects > 0);

        info!("Fragmentation patterns test passed");
    }

    /// Test real heap manager implementation
    #[test]
    fn test_real_heap_manager() {
        init_tracing!();
        info!("Testing real heap manager");

        let config = RealHeapConfig {
            initial_heap_size: 1024 * 1024, // 1MB
            max_heap_size: 10 * 1024 * 1024, // 10MB
            growth_factor: 2.0,
            block_size: 4096, // 4KB blocks
            alignment: 16,
            enable_coalescing: true,
            enable_splitting: true,
        };

        let real_heap = RealHeapManager::new(config);
        assert!(real_heap.is_ok());
        let mut heap = real_heap.unwrap();

        // Test basic allocation
        let alloc1 = heap.allocate(256);
        assert!(alloc1.is_ok());
        let block1 = alloc1.unwrap();
        assert_eq!(block1.size(), 256);
        assert!(utils::is_aligned::<u64>(block1.address() as *const u64));

        // Test multiple allocations
        let mut allocations = Vec::new();
        for i in 0..50 {
            let size = 128 + i * 64;
            let alloc = heap.allocate(size);
            assert!(alloc.is_ok());
            allocations.push(alloc.unwrap());
        }

        let stats_after_alloc = heap.get_statistics();
        assert!(stats_after_alloc.total_allocated > 0);
        assert!(stats_after_alloc.blocks_allocated > 0);
        debug!("Real heap stats after allocation: {:?}", stats_after_alloc);

        // Test deallocation and coalescing
        for i in (0..allocations.len()).step_by(3) {
            heap.deallocate(allocations[i].address()).unwrap();
        }

        let stats_after_dealloc = heap.get_statistics();
        assert!(stats_after_dealloc.blocks_free > 0);
        assert!(stats_after_dealloc.coalescing_operations > 0);
        debug!("Real heap stats after deallocation: {:?}", stats_after_dealloc);

        // Test large allocation
        let large_alloc = heap.allocate(64 * 1024); // 64KB
        assert!(large_alloc.is_ok());
        let large_block = large_alloc.unwrap();
        assert_eq!(large_block.size(), 64 * 1024);

        let final_stats = heap.get_statistics();
        debug!("Final real heap stats: {:?}", final_stats);

        info!("Real heap manager test passed");
    }

    /// Test memory layout optimization
    #[test]
    fn test_layout_optimization() {
        init_tracing!();
        info!("Testing memory layout optimization");

        // Test different object size patterns for layout optimization
        let test_patterns = vec![
            ("uniform_small", vec![64; 100]),
            ("uniform_large", vec![1024; 50]),
            ("mixed_sizes", (0..100).map(|i| 64 + i * 16).collect()),
            ("power_of_two", (0..10).map(|i| 1 << (i + 6)).collect()), // 64, 128, 256, etc.
            ("fibonacci", vec![1, 1, 2, 3, 5, 8, 13, 21, 34, 55].iter().map(|&x| x * 64).collect()),
        ];

        for (pattern_name, sizes) in test_patterns {
            debug!("Testing layout pattern: {}", pattern_name);

            let config = HeapConfig {
                initial_size: 2 * 1024 * 1024, // 2MB
                max_size: Some(8 * 1024 * 1024), // 8MB
                growth_factor: 1.5,
                shrink_threshold: 0.2,
                enable_compaction: true,
                alignment: 8,
            };

            let registry = Arc::new(ObjectRegistry::new());
            let heap = HeapManager::new(config, registry);

            let allocation_start = std::time::Instant::now();
            let mut allocations = Vec::new();

            // Allocate according to pattern
            for (i, &size) in sizes.iter().enumerate() {
                let description = format!("{}_{}", pattern_name, i);
                let result = heap.allocate::<u8>(size, &description);
                assert!(result.is_ok());
                allocations.push(result.unwrap());
            }

            let allocation_time = allocation_start.elapsed();
            let stats = heap.get_stats().unwrap();

            debug!("Pattern {}: {} allocations in {:?}", 
                   pattern_name, sizes.len(), allocation_time);
            debug!("  Total used: {} bytes", stats.total_used);
            debug!("  Fragmentation: {:.2}", 
                   utils::fragmentation_ratio(stats.total_free, stats.largest_free_block));

            // Test allocation efficiency (time per allocation)
            let time_per_alloc = allocation_time.as_nanos() / sizes.len() as u128;
            debug!("  Time per allocation: {} ns", time_per_alloc);

            // Should complete allocations reasonably quickly
            assert!(allocation_time.as_millis() < 1000);
            assert_eq!(stats.active_objects, sizes.len());
        }

        info!("Layout optimization test passed");
    }

    /// Performance test for memory operations
    #[test]
    #[ignore] // Run with --ignored flag for performance tests
    fn test_memory_performance() {
        init_tracing!();
        info!("Testing memory performance");

        let config = RealHeapConfig {
            initial_heap_size: 10 * 1024 * 1024, // 10MB
            max_heap_size: 100 * 1024 * 1024, // 100MB
            growth_factor: 2.0,
            block_size: 4096,
            alignment: 16,
            enable_coalescing: true,
            enable_splitting: true,
        };

        let mut heap = RealHeapManager::new(config).unwrap();

        // Performance test: rapid allocations
        let rapid_alloc_start = std::time::Instant::now();
        let mut rapid_allocations = Vec::new();

        for _i in 0..10000 {
            let size = 64 + (_i % 1000); // Varied sizes
            let alloc = heap.allocate(size);
            assert!(alloc.is_ok());
            rapid_allocations.push(alloc.unwrap());
        }

        let rapid_alloc_time = rapid_alloc_start.elapsed();
        debug!("10,000 rapid allocations took: {:?}", rapid_alloc_time);

        // Performance test: rapid deallocations
        let rapid_dealloc_start = std::time::Instant::now();

        for i in (0..rapid_allocations.len()).step_by(2) {
            heap.deallocate(rapid_allocations[i].address()).unwrap();
        }

        let rapid_dealloc_time = rapid_dealloc_start.elapsed();
        debug!("5,000 rapid deallocations took: {:?}", rapid_dealloc_time);

        // Performance test: mixed workload
        let mixed_start = std::time::Instant::now();

        for round in 0..1000 {
            // Allocate
            let size = 128 + round % 512;
            let alloc = heap.allocate(size);
            assert!(alloc.is_ok());
            let new_alloc = alloc.unwrap();

            // Deallocate old allocation occasionally
            if round > 100 && round % 10 == 0 {
                let dealloc_idx = (round - 100) / 10;
                if dealloc_idx < rapid_allocations.len() / 2 {
                    heap.deallocate(rapid_allocations[dealloc_idx].address()).unwrap();
                }
            }

            rapid_allocations.push(new_alloc);
        }

        let mixed_time = mixed_start.elapsed();
        debug!("1,000 mixed operations took: {:?}", mixed_time);

        let final_stats = heap.get_statistics();
        debug!("Performance test final stats: {:?}", final_stats);

        // Performance assertions
        let alloc_rate = 10000.0 / rapid_alloc_time.as_secs_f64();
        let dealloc_rate = 5000.0 / rapid_dealloc_time.as_secs_f64();

        debug!("Allocation rate: {:.0} allocs/sec", alloc_rate);
        debug!("Deallocation rate: {:.0} deallocs/sec", dealloc_rate);

        // Should achieve reasonable performance
        assert!(alloc_rate > 100_000.0); // At least 100K allocations per second
        assert!(dealloc_rate > 100_000.0); // At least 100K deallocations per second

        info!("Memory performance test passed");
    }
}
