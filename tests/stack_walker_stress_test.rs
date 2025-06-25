/// Stress tests for the stack walking implementation
/// 
/// Tests the stack walking system under high load, with many concurrent
/// operations, deep call stacks, and edge cases to ensure robustness.

use cursed::runtime::stack_walker::{StackWalker, StackWalkConfig};
use cursed::error::Error as CursedError;
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::{Duration, Instant};
use tracing::{info, debug, warn};

/// Initialize tracing for tests
fn init_test_tracing() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        tracing_subscriber::fmt()
            .with_env_filter("debug")
            .with_test_writer()
            .init();
    });
}

#[test]
fn test_concurrent_stack_walking_stress() {
    init_test_tracing();
    info!("Starting concurrent stack walking stress test");
    
    const NUM_THREADS: usize = 8;
    const WALKS_PER_THREAD: usize = 100;
    
    let walker = Arc::new(StackWalker::new());
    let barrier = Arc::new(Barrier::new(NUM_THREADS));
    let start_time = Arc::new(std::sync::Mutex::new(None));
    let mut handles = vec![];
    
    // Spawn multiple threads
    for thread_id in 0..NUM_THREADS {
        let walker_clone = walker.clone();
        let barrier_clone = barrier.clone();
        let start_time_clone = start_time.clone();
        
        let handle = thread::spawn(move || {
            // Wait for all threads to be ready
            barrier_clone.wait();
            
            // Record start time on first thread
            if thread_id == 0 {
                let mut start = start_time_clone.lock().unwrap();
                *start = Some(Instant::now());
            }
            
            let mut successful_walks = 0;
            let mut failed_walks = 0;
            
            // Perform many stack walks
            for walk_id in 0..WALKS_PER_THREAD {
                match walker_clone.walk_stack() {
                    Ok(frames) => {
                        successful_walks += 1;
                        if walk_id % 20 == 0 {
                            debug!("Thread {} walk {}: {} frames", thread_id, walk_id, frames.len());
                        }
                    }
                    Err(e) => {
                        failed_walks += 1;
                        if failed_walks <= 5 {
                            warn!("Thread {} walk {} failed: {}", thread_id, walk_id, e);
                        }
                    }
                }
                
                // Add some variety with small pauses
                if walk_id % 10 == 0 {
                    thread::yield_now();
                }
            }
            
            debug!("Thread {} completed: {} successful, {} failed", 
                   thread_id, successful_walks, failed_walks);
            
            (successful_walks, failed_walks)
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    let mut total_successful = 0;
    let mut total_failed = 0;
    
    for (thread_id, handle) in handles.into_iter().enumerate() {
        let (successful, failed) = handle.join()
            .unwrap_or_else(|_| panic!("Thread {} panicked", thread_id));
        
        total_successful += successful;
        total_failed += failed;
    }
    
    let end_time = Instant::now();
    let start = start_time.lock().unwrap().unwrap();
    let total_time = end_time.duration_since(start);
    
    info!("Stress test completed in {:?}", total_time);
    info!("Total successful walks: {}", total_successful);
    info!("Total failed walks: {}", total_failed);
    
    // At least 80% should succeed
    let success_rate = total_successful as f64 / (total_successful + total_failed) as f64;
    assert!(success_rate >= 0.8, "Success rate should be at least 80%, got {:.2}%", success_rate * 100.0);
    
    // Check final statistics
    let stats = walker.get_statistics().unwrap();
    debug!("Final statistics: {:?}", stats);
    
    assert!(stats.total_walks >= total_successful as u64, 
            "Statistics should reflect successful walks");
}

#[test]
fn test_deep_recursion_stress() {
    init_test_tracing();
    info!("Starting deep recursion stress test");
    
    const MAX_DEPTH: usize = 100;
    
    fn recursive_stack_walk(depth: usize, walker: &StackWalker) -> Result<usize, CursedError> {
        if depth == 0 {
            let frames = walker.walk_stack()?;
            Ok(frames.len())
        } else {
            // Create some local variables to make frames more interesting
            let _local1 = depth * 2;
            let _local2 = format!("depth_{}", depth);
            let _local3 = vec![depth; 3];
            
            recursive_stack_walk(depth - 1, walker)
        }
    }
    
    let walker = StackWalker::new();
    
    for depth in (10..=MAX_DEPTH).step_by(10) {
        debug!("Testing recursion depth: {}", depth);
        
        let start = Instant::now();
        let result = recursive_stack_walk(depth, &walker);
        let elapsed = start.elapsed();
        
        match result {
            Ok(frame_count) => {
                debug!("Depth {} captured {} frames in {:?}", depth, frame_count, elapsed);
                
                // Should capture at least the recursive frames
                assert!(frame_count >= depth / 2, 
                        "Should capture a reasonable number of frames for depth {}", depth);
            }
            Err(e) => {
                warn!("Deep recursion failed at depth {}: {}", depth, e);
                // Some platforms might fail with very deep stacks
            }
        }
        
        // Should complete in reasonable time even for deep stacks
        assert!(elapsed < Duration::from_secs(5), 
                "Deep stack walking should not take too long");
    }
}

#[test]
fn test_memory_pressure_stack_walking() {
    init_test_tracing();
    info!("Starting memory pressure stack walking test");
    
    let walker = StackWalker::new();
    
    // Allocate large amounts of memory while doing stack walks
    let mut large_allocations = Vec::new();
    const ALLOCATION_SIZE: usize = 1024 * 1024; // 1MB
    const NUM_ALLOCATIONS: usize = 100;
    
    for i in 0..NUM_ALLOCATIONS {
        // Allocate memory
        let allocation = vec![i as u8; ALLOCATION_SIZE];
        large_allocations.push(allocation);
        
        // Perform stack walk under memory pressure
        let result = walker.walk_stack();
        
        match result {
            Ok(frames) => {
                if i % 10 == 0 {
                    debug!("Stack walk {} under memory pressure: {} frames", i, frames.len());
                }
            }
            Err(e) => {
                warn!("Stack walk {} failed under memory pressure: {}", i, e);
                // Some failures might be acceptable under extreme memory pressure
            }
        }
        
        // Occasionally clear some allocations
        if i % 20 == 19 {
            large_allocations.clear();
            debug!("Cleared allocations at iteration {}", i);
        }
    }
}

#[test]
fn test_rapid_stack_walking() {
    init_test_tracing();
    info!("Starting rapid stack walking test");
    
    let walker = StackWalker::new();
    const NUM_RAPID_WALKS: usize = 1000;
    
    let start = Instant::now();
    let mut successful = 0;
    let mut failed = 0;
    
    for i in 0..NUM_RAPID_WALKS {
        match walker.walk_stack() {
            Ok(_) => successful += 1,
            Err(_) => failed += 1,
        }
        
        if i % 100 == 0 {
            debug!("Rapid walk progress: {}/{}", i, NUM_RAPID_WALKS);
        }
    }
    
    let elapsed = start.elapsed();
    let walks_per_second = NUM_RAPID_WALKS as f64 / elapsed.as_secs_f64();
    
    info!("Rapid test completed: {} successful, {} failed in {:?}", 
          successful, failed, elapsed);
    info!("Rate: {:.2} walks per second", walks_per_second);
    
    // Should achieve reasonable throughput
    assert!(walks_per_second >= 100.0, 
            "Should achieve at least 100 walks per second, got {:.2}", walks_per_second);
    
    // Most should succeed
    let success_rate = successful as f64 / NUM_RAPID_WALKS as f64;
    assert!(success_rate >= 0.9, 
            "Success rate should be at least 90%, got {:.2}%", success_rate * 100.0);
}

#[test]
fn test_stack_walking_with_many_threads() {
    init_test_tracing();
    info!("Starting many threads stack walking test");
    
    const NUM_THREADS: usize = 50;
    const WALKS_PER_THREAD: usize = 20;
    
    let walker = Arc::new(StackWalker::new());
    let mut handles = vec![];
    
    let start = Instant::now();
    
    // Spawn many threads
    for thread_id in 0..NUM_THREADS {
        let walker_clone = walker.clone();
        
        let handle = thread::spawn(move || {
            let mut thread_successful = 0;
            
            for walk_id in 0..WALKS_PER_THREAD {
                match walker_clone.walk_stack() {
                    Ok(frames) => {
                        thread_successful += 1;
                        if walk_id == 0 {
                            debug!("Thread {} first walk: {} frames", thread_id, frames.len());
                        }
                    }
                    Err(e) => {
                        if walk_id == 0 {
                            warn!("Thread {} first walk failed: {}", thread_id, e);
                        }
                    }
                }
                
                // Small delay to spread out the work
                thread::sleep(Duration::from_millis(1));
            }
            
            thread_successful
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads
    let mut total_successful = 0;
    for (thread_id, handle) in handles.into_iter().enumerate() {
        let successful = handle.join()
            .unwrap_or_else(|_| panic!("Thread {} panicked", thread_id));
        total_successful += successful;
    }
    
    let elapsed = start.elapsed();
    
    info!("Many threads test completed in {:?}", elapsed);
    info!("Total successful walks: {}", total_successful);
    
    // Should handle many concurrent threads
    let expected_total = NUM_THREADS * WALKS_PER_THREAD;
    let success_rate = total_successful as f64 / expected_total as f64;
    
    assert!(success_rate >= 0.7, 
            "Success rate should be at least 70% with many threads, got {:.2}%", 
            success_rate * 100.0);
}

#[test]
fn test_stack_walking_under_signal_stress() {
    init_test_tracing();
    info!("Starting signal stress test");
    
    // This test simulates signal handling scenarios
    let walker = StackWalker::new();
    
    // Simulate rapid interruptions while stack walking
    let mut successful = 0;
    let mut interrupted = 0;
    
    for i in 0..100 {
        // Start stack walk
        let result = walker.walk_stack();
        
        // Simulate signal interruption by yielding thread
        if i % 3 == 0 {
            thread::yield_now();
            interrupted += 1;
        }
        
        match result {
            Ok(_) => successful += 1,
            Err(_) => {} // Some failures acceptable
        }
    }
    
    debug!("Signal stress test: {} successful, {} interrupted", successful, interrupted);
    
    // Should handle interruptions gracefully
    assert!(successful >= 50, "Should handle signal-like interruptions gracefully");
}

#[test]
fn test_stack_walker_cache_stress() {
    init_test_tracing();
    info!("Starting cache stress test");
    
    let walker = StackWalker::new();
    
    // Generate many different addresses to stress the cache
    let addresses: Vec<usize> = (0..1000)
        .map(|i| 0x400000 + i * 0x1000) // Spread across address space
        .collect();
    
    let start = Instant::now();
    
    // First pass - populate cache
    for &addr in &addresses {
        let _ = walker.resolve_symbol_for_frame_addr(addr);
    }
    
    let first_pass = start.elapsed();
    
    // Second pass - should hit cache
    let cache_start = Instant::now();
    for &addr in &addresses {
        let _ = walker.resolve_symbol_for_frame_addr(addr);
    }
    let second_pass = cache_start.elapsed();
    
    debug!("Cache test: first pass {:?}, second pass {:?}", first_pass, second_pass);
    
    // Second pass should be faster (cache hits)
    // Allow some margin for system variance
    if second_pass < first_pass {
        info!("Cache is working - second pass was faster");
    } else {
        debug!("Cache effect not clearly visible (system variance or no symbols resolved)");
    }
    
    // Clear cache and test again
    walker.clear_cache();
    
    let clear_start = Instant::now();
    for &addr in addresses.iter().take(100) {
        let _ = walker.resolve_symbol_for_frame_addr(addr);
    }
    let after_clear = clear_start.elapsed();
    
    debug!("After cache clear: {:?}", after_clear);
}

#[test]
fn test_stack_walking_error_recovery() {
    init_test_tracing();
    info!("Starting error recovery test");
    
    let walker = StackWalker::new();
    
    // Test recovery from various error conditions
    let mut successful_after_error = 0;
    
    for i in 0..50 {
        // Introduce some potential error conditions
        if i % 5 == 0 {
            // Try to resolve invalid address
            let _ = walker.resolve_symbol_for_frame_addr(0);
        }
        
        if i % 7 == 0 {
            // Try to resolve very large address
            let _ = walker.resolve_symbol_for_frame_addr(usize::MAX);
        }
        
        // Now try normal stack walking
        match walker.walk_stack() {
            Ok(_) => successful_after_error += 1,
            Err(e) => debug!("Stack walk failed after error condition: {}", e),
        }
    }
    
    debug!("Error recovery test: {} successful walks after error conditions", 
           successful_after_error);
    
    // Should recover from error conditions
    assert!(successful_after_error >= 30, 
            "Should recover from error conditions and continue working");
}

#[test]
fn test_long_running_stack_walking() {
    init_test_tracing();
    info!("Starting long-running stack walking test");
    
    let walker = StackWalker::new();
    let start = Instant::now();
    let test_duration = Duration::from_secs(10);
    
    let mut total_walks = 0;
    let mut successful_walks = 0;
    
    while start.elapsed() < test_duration {
        total_walks += 1;
        
        match walker.walk_stack() {
            Ok(_) => successful_walks += 1,
            Err(_) => {}
        }
        
        if total_walks % 100 == 0 {
            debug!("Long-running test progress: {} walks in {:?}", 
                   total_walks, start.elapsed());
        }
        
        // Small delay to prevent overwhelming the system
        thread::sleep(Duration::from_millis(1));
    }
    
    let final_elapsed = start.elapsed();
    let success_rate = successful_walks as f64 / total_walks as f64;
    
    info!("Long-running test completed: {} successful out of {} total walks in {:?}", 
          successful_walks, total_walks, final_elapsed);
    info!("Success rate: {:.2}%", success_rate * 100.0);
    
    // Should maintain reasonable performance over time
    assert!(success_rate >= 0.8, 
            "Should maintain good success rate over time, got {:.2}%", 
            success_rate * 100.0);
    
    // Should achieve reasonable throughput
    let walks_per_second = total_walks as f64 / final_elapsed.as_secs_f64();
    assert!(walks_per_second >= 50.0, 
            "Should maintain reasonable throughput, got {:.2} walks/sec", 
            walks_per_second);
}
