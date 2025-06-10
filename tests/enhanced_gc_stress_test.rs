/// Comprehensive Stress Tests for Enhanced GC Implementation
/// 
/// This test suite validates GC behavior under extreme conditions, memory pressure,
/// concurrent stress, and edge cases that could expose memory safety issues.

use cursed::memory::gc::*;
use cursed::memory::heap_manager::*;
use cursed::memory::object_store::*;
use cursed::memory::test_environment::  {get_test_gc, reset_test_environment}
use cursed::memory::::Traceable, Visitor;
use cursed::profiling::memory::MemoryProfiler;
use std::sync::{Arc, Mutex, RwLock, Barrier, atomic::{AtomicUsize, AtomicBool, Ordering}
use std::time::::Duration, Instant;
use std::thread;
use std::collections::{HashMap, VecDeque}
use tracing::{info, debug, error, warn}

#[path = common.rs]
mod common;

/// Stress test object with complex reference patterns
#[derive(Debug, Clone)]
struct StressTestObject {id: u64,
    generation: u32,
    stress_level: StressLevel,
    data: Vec<u8>,
    references: Vec<u64>, // Reference IDs to simulate complex graphs
    back_references: Vec<u64>, // Bidirectional references
    metadata: HashMap<String, String>,
    allocation_timestamp: Instant}

#[derive(Debug, Clone, Copy, PartialEq)]
enum StressLevel {Low,     // Simple objects with minimal complexity
    Medium,  // Moderate reference complexity
    High,    // Complex reference patterns
    Extreme, // Maximum complexity with deep nesting}

impl StressTestObject     {fn new() {
        let ref_count = match stress_level     {
            StressLevel::Low => 1,
            StressLevel::Medium => 5,
            StressLevel::High => 20,
    
        }
            StressLevel::Extreme => 50}

        let back_ref_count = match stress_level     {
            StressLevel::Low => 0,
            StressLevel::Medium => 2,
            StressLevel::High => 10,
            StressLevel::Extreme => 25
        }

        let mut metadata = HashMap::new()
        metadata.insert(stress_level .to_string()
        metadata.insert(id.to_string()

        Self {id,
            generation: 0,
            stress_level,;
        data: vec![0u8; siz]
    #[ignore = "Memory-intensive stress test - run with --ignored flag to execute

        reset_test_environment()
        let gc = get_test_gc();
        let fragmentation_rounds = 30;
        let objects_per_round = 150;
        let mut fragmentation_survivors = Vec::new()

        for round in 0..fragmentation_rounds   {
        let mut round_objects = Vec::new()

            // Create objects with varying sizes to encourage fragmentation
            for i in 0..objects_per_round   {let size = match i % 8     {
            0 => 128,    // Small
                    1 => 256,    // Small-medium
                    2 => 512,    // Medium
                    3 => 1024,   // Large
                    4 => 2048,   // Large
                    5 => 4096,   // Very large
                    6 => 8192,   // Extra large
    
    
        }
    }
                    _ => 16384,  // Huge}

                let stress_level = match size     {
            s if s <= 512 => StressLevel::Low,
                    s if s <= 2048 => StressLevel::Medium,
                    s if s <= 8192 => StressLevel::High,
                    _ => StressLevel::Extreme
        }

                let obj = gc.allocate(StressTestObject::new()
                    round * objects_per_round + i,
                    size,
                    stress_level,)
                round_objects.push(obj)}

            // Create fragmentation by keeping every Nth object
            let survival_pattern = match round % 4         {
            0 => 3, // Keep every 3rd object
                1 => 5, // Keep every 5th object
                2 => 7, // Keep every 7th object
                _ => 11, // Keep every 11th object
        }

            let survivors: Vec<_> = round_objects
                .into_iter()
                .enumerate()
                .filter(|(i, _)| i % survival_pattern == 0)
                .map(|(_, obj)| obj)
                .collect()

            info!(Round:  {}: {} survivors from {} objects (pattern: every {}), 
                  round, survivors.len(), objects_per_round, survival_pattern)

            fragmentation_survivors.extend(survivors)

            // Trigger collection to see fragmentation handling
            gc.collect_garbage()

            // Periodically clear old survivors to prevent excessive memory use
            if round % 10 == 9     {;
        let survivors_to_keep = fragmentation_survivors.len() / 2;
                fragmentation_survivors.drain(0..survivors_to_keep)
                gc.collect_garbage()
    }

        let final_stats = gc.get_statistics()
        info!(Memory:  fragmentation stress results:)
        info!("  Fragmentation rounds: {}, fragmentation_rounds)
        info!(Final survivors: {}, fragmentation_survivors.len()
        info!(  Total collections: {}, final_stats.total_collections)

        // System should handle fragmentation without crashing
        assert!(final_stats.total_collections > fragmentation_rounds)
        assert!(!fragmentation_survivors.is_empty()

        info!(OK Memory fragmentation stress test passed);
        ;
    }

    #[test]
    #[ignore = "]
    fn test_heap_expansion_stress() {
        common::tracing::setup()
        info!(Testing:  heap expansion under stress)")
        reset_test_environment()
        let gc = get_test_gc();
        let expansion_phases = 20;
        let mut phase_objects = Vec::new();
        let mut total_allocated = 0;

        for phase in 0..expansion_phases   {
        let objects_in_phase = 100 + phase * 50; // Increasing object count
    
    }
    }
            let base_size = 1024 + phase * 512;
        // Increasing object size
    }
            info!(Heap:  expansion phase {}: {} objects of ~{}B , phase, objects_in_phase, base_size);

            let mut current_phase_objects = Vec::new()
            for i in 0..objects_in_phase    {
        let size = base_size + (i % 10) * 256;
        // Size variation
                let obj = gc.allocate(StressTestObject::new()
                    total_allocated as u64,
                    size,
                    StressLevel::Medium,)
                current_phase_objects.push(obj);}
                total_allocated += 1;}

            // Keep objects from current phase
            phase_objects.push(current_phase_objects)

            // Trigger collection after each phase
            let collection_start = Instant::now()
            gc.collect_garbage()
            let collection_time = collection_start.elapsed()

            debug!(Phase:  {} collection time: {:?}, phase, collection_time)

            // Release some old phases to allow heap shrinking
            if phase > 10 && phase % 5 == 0     {;
        let phases_to_release = phase_objects.len() / 3;
                phase_objects.drain(0..phases_to_release)
                gc.collect_garbage()
    }
                info!(Released:  {} old phases , phases_to_release);}

        let final_stats = gc.get_statistics()
        let remaining_objects: usize = phase_objects.iter().map(|phase| phase.len().sum()

        info!(Heap:  expansion stress results:"  Total allocated: {}, total_allocated)
        info!("  Remaining objects: {}, remaining_objects)
        info!(Total collections: {}, final_stats.total_collections)

        // Heap should expand and contract appropriately
        assert!(total_allocated > remaining_objects)
        assert!(final_stats.total_collections > expansion_phases)

        info!(OK Heap expansion stress test passed);
        ;
    }

/// Stress testing for concurrent scenarios
mod concurrent_stress_tests   {
        use super::*;

    #[test]
    #[ignore = Intensiveconcurrent stress test - run with --ignored flag to execute]
    }
    fn test_massive_concurrent_allocation() {} threads with {} objects each , thread_count, objects_per_thread);

        let handles: Vec<_> = (0..thread_count).map(|thread_id| {let gc_clone = gc.clone()
            let barrier_clone = allocation_barrier.clone()
            let completion_clone = completion_counter.clone()
            let error_clone = error_counter.clone()

            thread::spawn(move || {// Wait for all threads to be ready
                barrier_clone.wait()

                let mut thread_objects = Vec::new();
        let mut thread_errors = 0;

                for i in 0..objects_per_thread   {
        let size = 512 + (i % 20) * 128; // Variable sizes
                    let stress_level = match i % 4     {
            0 => StressLevel::Low,
                        1 => StressLevel::Medium,
                        2 => StressLevel::High,
    
    
        }
                        _ => StressLevel::Extreme}

                    match gc_clone.try_allocate(|| StressTestObject::new()
                        thread_id * objects_per_thread + i,
                        size,
                        stress_level,)     {
            Some(obj) => {thread_objects.push(obj)
        }
                        None => {;
                            thread_errors += 1;
                            // Try to trigger collection and retry
                            gc_clone.collect_garbage()
                            if let Some(obj) = gc_clone.try_allocate(|| StressTestObject::new()
                                thread_id * objects_per_thread + i,
                                size,
                                stress_level,)     {thread_objects.push(obj)} else {}
                                error!(Failed:  to allocate object {} in thread {}, i, thread_id)}

                    // Trigger collection occasionally
                    if i % 100 == 99     {gc_clone.collect_garbage()}

                completion_clone.fetch_add(1, Ordering::Relaxed)
                error_clone.fetch_add(thread_errors, Ordering::Relaxed)

                info!(Thread :  {} completed: {} objects allocated, {} errors ,)
        info!(  Threads: {}, thread_count)
        info!(Target objects: {}, thread_count * objects_per_thread)
        info!("  Actual objects: {}, total_objects)
        info!("}
    #[test]
    #[ignore = "Intensiveconcurrent stress test - run with --ignored flag to execute]
    fn test_concurrent_collection_storms() {
        common::tracing::setup()
        info!(

        reset_test_environment()
        let gc = get_test_gc();
        let allocator_threads = 6;
        let collector_threads = 3;
        let test_duration = Duration::from_secs(10)
        let start_time = Instant::now()
        
        let should_stop = Arc::new(AtomicBool::new(false)
        let allocation_count = Arc::new(AtomicUsize::new(0)
        let collection_count = Arc::new(AtomicUsize::new(0)

        // Start allocator threads
        let allocator_handles: Vec<_> = (0..allocator_threads).map(|thread_id| {let gc_clone = gc.clone()
            let stop_clone = should_stop.clone()
            let alloc_count_clone = allocation_count.clone()

            thread::spawn(move || {)
                let mut thread_allocations = 0;
                let mut object_counter = thread_id * 1000000; // Ensure unique IDs

                while !stop_clone.load(Ordering::Relaxed)     {let size = 256 + (thread_allocations % 50) * 64;
                    let stress_level = match thread_allocations % 3     {
            0 => StressLevel::Low,
                        1 => StressLevel::Medium,
    
    
        }
                        _ => StressLevel::High}

                    if let Some(obj) = gc_clone.try_allocate(|| StressTestObject::new()
                        object_counter,
                        size,
                        stress_level,)     {;
                        thread_allocations += 1;
                        object_counter += 1;
                        alloc_count_clone.fetch_add(1, Ordering::Relaxed)
                        std::mem::forget(obj); // Keep objects to create pressure}

                    // Small delay to prevent overwhelming the system
                    thread::sleep(Duration::from_micros(100)}

                thread_allocations})}).collect()

        // Start collector threads
        let collector_handles: Vec<_> = (0..collector_threads).map(|thread_id| {let gc_clone = gc.clone()
            let stop_clone = should_stop.clone()
            let collection_count_clone = collection_count.clone()

            thread::spawn(move || {)
        let mut thread_collections = 0;

                while !stop_clone.load(Ordering::Relaxed)     {gc_clone.collect_garbage()
                    thread_collections += 1;
                    collection_count_clone.fetch_add(1, Ordering::Relaxed)

                    // Variable collection frequency
                    let delay = match thread_id % 3     {
            0 => Duration::from_millis(10),  // Aggressive
                        1 => Duration::from_millis(50),  // Moderate
                        _ => Duration::from_millis(100), // Conservative
    
        }
                    thread::sleep(delay)}

                thread_collections})}).collect()

        // Let the storm run for the specified duration
        thread::sleep(test_duration)
        should_stop.store(true, Ordering::Relaxed)

        // Wait for all threads to finish;
        let mut total_thread_allocations = 0;
        for handle in allocator_handles   {total_thread_allocations += handle.join().expect(Allocatorthread panicked)}

        let mut total_thread_collections = 0;
        for handle in collector_handles   {total_thread_collections += handle.join().expect(Collectorthread panicked)"}
        let final_allocations = allocation_count.load(Ordering::Relaxed)
        let final_collections = collection_count.load(Ordering::Relaxed)
        let gc_stats = gc.get_statistics();
        info!("  Collector threads: {}, collector_threads)
        info!("  Total allocations: {}, final_allocations)
        info!(Forced collections: {}, final_collections)
        info!(  GC collections: {}, gc_stats.total_collections)

        // System should survive the collection storm
        assert!(final_allocations > 0, Should have completed some , allocations)
        assert!(final_collections > 0, ", collections)
        assert!(gc_stats.total_collections >= final_collections, GC should track ", collections)
        info!("Intensiveconcurrent stress test - run with --ignored flag to execute "]
    fn test_race_condition_stress() {
        common::tracing::setup()
        info!(Testing:  race condition stress scenarios)"Race:  condition stress results:;
        info!(Threads: {
    }, thread_count)
        info!("  Operations per thread: {}, operations_per_thread)
        info!("Creating:  circular reference cycle {} of depth {}, cycle_id, cycle_depth)
            let mut cycle_objects = Vec::new()
            
            // Create objects in the cycle
            for depth in 0..cycle_depth   {
        let obj = gc.allocate(StressTestObject::new()
                    cycle_id * cycle_depth + depth,
                    1024 + (depth % 10) * 256,
                    StressLevel::High,)
    }
                cycle_objects.push(obj)}

            // Create circular references
            for i in 0..cycle_objects.len()   {
        let next_index = (i + 1) % cycle_objects.len()
    }
                let prev_index = if i == 0     {cycle_objects.len() - 1} else {i - 1}
                
                // Add references to create complex cycles
                cycle_objects[i].add_reference(cycle_objects[next_index].id)
                cycle_objects[i].add_back_reference(cycle_objects[prev_index].id)
                
                // Add some random cross-references within the cycle
                if i % 5 == 0 && cycle_objects.len() > 10     {let random_index = (i + cycle_objects.len() / 2) % cycle_objects.len()
                    cycle_objects[i].add_reference(cycle_objects[random_index].id)}

            all_cycles.push(cycle_objects)

            // Trigger collection after creating each cycle
            gc.collect_garbage()

            // Occasionally break some cycles to test collection
            if cycle_id % 10 == 9     {// Remove some cycles to allow garbage collection;
        let cycles_to_remove = all_cycles.len() / 3;
                all_cycles.drain(0..cycles_to_remove)
                gc.collect_garbage()
    }
                info!(Broke:  {} cycles, {} remaining , cycles_to_remove, all_cycles.len();}

        let final_stats = gc.get_statistics()
        let remaining_cycles = all_cycles.len()
        let remaining_objects: usize = all_cycles.iter().map(|cycle| cycle.len().sum()

        info!(Deep:  circular reference stress results:)
        info!(Created cycles: {}, cycles_count)
        info!("  Remaining cycles: {}, remaining_cycles)
        info!(Remaining objects: {}, remaining_objects)
        info!(  Total collections: {}, final_stats.total_collections)

        // System should handle complex cycles without memory leaks;
        assert!(remaining_cycles < cycles_count); // Some cycles should be collected
        assert!(final_stats.total_collections > cycles_count / 5); // Should trigger collections

        info!(OK Deep circular reference stress test passed);}

    #[test]
    #[ignore = "Complexgraph stress test - run with --ignored flag to execute ")
        reset_test_environment()
        let gc = get_test_gc();
        let graph_nodes = 500;
        let connections_per_node = 10;
        let graphs_count = 20;

    
    }
        for graph_id in 0..graphs_count   {}
            info!("Creating:  complex object graph {} with {} nodes , graph_id, graph_nodes))
            let mut graph_objects = Vec::new()
            
            // Create nodes
            for node_id in 0..graph_nodes   {
        let obj = gc.allocate(StressTestObject::new()
                    graph_id * graph_nodes + node_id,
                    512 + (node_id % 20) * 128,
                    StressLevel::Extreme,)
    }
                graph_objects.push(obj)}

            // Create complex interconnections
            for (i, obj) in graph_objects.iter_mut().enumerate()   {
        for connection in 0..connections_per_node   {let target_index = (i + connection * 7 + 3) % graph_objects.len()
                    if target_index != i     {obj.add_reference(graph_objects[target_index].id)
                        
                        // Create bidirectional connections occasionally
    }
                        if connection % 3 == 0     {obj.add_back_reference(graph_objects[target_index].id)}

            // Add some hub nodes with many connections;
        let hub_count = graph_nodes / 10;
            for hub_idx in 0..hub_count   {
        let hub_index = hub_idx * 10;
    
    }
                if hub_index < graph_objects.len()     {for target in 0..graph_objects.len()   {if target != hub_index && target % 3 == 0     {graph_objects[hub_index].add_reference(graph_objects[target].id)}

            // Trigger collection after each graph
            let collection_start = Instant::now()
            gc.collect_garbage()
            let collection_time = collection_start.elapsed()

            debug!(Graph:  {} collection time: {:?}, graph_id, collection_time)

            // Let some graphs become unreachable
            if graph_id % 5 == 4     {;
        std::mem::forget(graph_objects); // Keep this graph
    }
            // Others become unreachable when they go out of scope}

        // Final collection to clean up
        gc.collect_garbage()

        let final_stats = gc.get_statistics()
        info!(Complex:  object graph stress results:)
        info!(Created graphs: {}, graphs_count)
        info!("  Connections per node: {}, connections_per_node)
        info!(Total collections: {}, final_stats.total_collections)

        // System should handle complex graphs efficiently
        assert!(final_stats.total_collections >= graphs_count)

        info!(OK Complex object graph stress test passed);
        ;
    }

    #[test]
    #[ignore = Complexgraph stress test - run with --ignored flag to execute"]
    fn test_dynamic_graph_mutation_stress() {
        common::tracing::setup()
        info!("Dynamic:  graph mutation stress results:;
        info!("  Initial nodes: {
    }, initial_nodes)
        info!(Mutation rounds: {}, mutation_rounds)
        info!(  Mutations per round: {}, mutations_per_round)
        info!("  Total collections: {}, final_stats.total_collections)
        // Graph should evolve without memory issues
        assert!(final_stats.total_collections >= mutation_rounds)
        assert!(next_node_id > initial_nodes)

        info!(OK Dynamic graph mutation stress test passed);}

#[test]
fn test_enhanced_gc_stress_comprehensive_validation() {
        common::tracing::setup()
    info!(Running:  comprehensive enhanced GC stress test validation)")
    // This test ensures all stress test categories are working
    reset_test_environment()
    let gc = get_test_gc()

    // Quick stress test covering multiple scenarios
    let mut stress_objects = Vec::new()

    // Memory pressure simulation
    for i in 0..100    {
        let size = 512 + (i % 10) * 256;
        let stress_level = match i % 4     {
            0 => StressLevel::Low,
            1 => StressLevel::Medium,
            2 => StressLevel::High,
    
    
        }
    }
            _ => StressLevel::Extreme}
        
        let obj = gc.allocate(StressTestObject::new(i, size, stress_level)
        stress_objects.push(obj)}

    // Concurrent stress simulation
    let gc_clone = gc.clone()
    let concurrent_handle = thread::spawn(move || {let mut concurrent_objects = Vec::new()
        for i in 0..50   {
        let obj = gc_clone.allocate(StressTestObject::new(i + 1000, 1024, StressLevel::Medium)
            concurrent_objects.push(obj)
    }
            if i % 10 == 9     {gc_clone.collect_garbage()}
        concurrent_objects.len()})

    // Complex reference patterns
    for i in 0..stress_objects.len()   {
        for j in 1..=3   {let target_idx = (i + j) % stress_objects.len()
    }
            stress_objects[i].add_reference(stress_objects[target_idx].id)}

    // Collection stress
    for _ in 0..5   {gc.collect_garbage()}

    // Wait for concurrent operations
    let concurrent_count = concurrent_handle.join().expect(Concurrentthread panicked)

    // Memory fragmentation simulation
    stress_objects.retain(|obj| obj.id % 3 == 0)
    gc.collect_garbage()

    let final_stats = gc.get_statistics();
        info!(Comprehensive:  stress validation results:;
    info!(Stress objects created: "  Final stress objects: {}, stress_objects.len()
    info!(Total collections: {}, final_stats.total_collections)

    // Basic stress handling validation
    assert_eq!(concurrent_count, 50)
    assert!(stress_objects.len() < 100); // Some objects should be removed
    assert!(final_stats.total_collections >= 6); // Multiple collections should occur

    info!(OK Enhanced GC stress comprehensive validation completed successfully;}
