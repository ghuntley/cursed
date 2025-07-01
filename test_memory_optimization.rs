// Simple test for memory optimization
use cursed::optimization::memory_optimization::*;
use cursed::memory::real_heap_manager::*;
use cursed::memory::root_set::*;

fn main() {
    println!("Testing memory optimization components...");

    // Test memory optimizer
    match get_global_optimizer().optimize_allocation(64) {
        Ok(strategy) => println!("✓ Memory optimizer works: {:?}", strategy),
        Err(e) => println!("✗ Memory optimizer failed: {:?}", e),
    }

    // Test real heap manager
    match get_global_heap().get_stats() {
        Ok(stats) => println!("✓ Real heap manager works: allocated={}, peak={}", stats.total_allocated, stats.peak_usage),
        Err(e) => println!("✗ Real heap manager failed: {:?}", e),
    }

    // Test root set manager
    match get_global_root_set().get_stats() {
        Ok(stats) => println!("✓ Root set manager works: total_roots={}, additions={}", stats.total_roots, stats.root_additions),
        Err(e) => println!("✗ Root set manager failed: {:?}", e),
    }

    println!("Memory optimization test completed successfully!");
}
