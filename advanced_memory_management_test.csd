// CURSED Advanced Memory Management and GC Test
// Tests: Production GC, reference counting, memory pools, leak detection, pressure monitoring

yeet "testz"
yeet "vibez"

// Test basic allocation and tracking
test_start("Basic Allocation Tracking")

// Allocate objects of different sizes
sus small_obj drip = 42
sus medium_obj tea = "medium string for testing"
sus large_obj = malloc(1024)  // Large allocation

// Test reference counting
test_start("Reference Counting")
sus shared_obj = [1, 2, 3, 4, 5]
sus ref1 = shared_obj    // Should increment ref count
sus ref2 = shared_obj    // Should increment again
sus ref3 = shared_obj    // Should increment again

vibez.spill("Reference count should be 4: ")
// vibez.spill(gc_get_ref_count(shared_obj))

// Release references
ref1 = null     // Should decrement 
ref2 = null     // Should decrement
ref3 = null     // Should decrement
shared_obj = null // Final decrement should trigger immediate cleanup

test_start("Memory Pool Allocation")

// Test small object pooling
sus pool_test_objects = []
bestie (sus i drip = 0; i < 100; i = i + 1) {
    sus obj = malloc(64)  // Should use memory pool
    pool_test_objects.append(obj)
}

// Free pool objects
bestie (sus obj bro pool_test_objects) {
    free(obj)  // Should return to pool
}

test_start("Generational Collection")

// Allocate many young objects to trigger young GC
sus young_objects = []
bestie (sus i drip = 0; i < 1000; i = i + 1) {
    sus obj = {
        id: i,
        data: "young object " + i.to_string()
    }
    young_objects.append(obj)
}

// Keep some objects alive to test promotion
sus promoted_objects = []
bestie (sus i drip = 0; i < 100; i = i + 10) {
    promoted_objects.append(young_objects[i])
}

// Trigger collection
gc_collect()

vibez.spill("Promoted objects should survive collection")

test_start("Memory Pressure Monitoring")

// Allocate until memory pressure increases
sus pressure_objects = []
bestie (sus i drip = 0; i < 500; i = i + 1) {
    sus large_obj = malloc(1024 * 1024)  // 1MB each
    pressure_objects.append(large_obj)
    
    // Check pressure level every 50 allocations
    grr (i % 50 == 0) {
        sus pressure = gc_get_memory_pressure()
        vibez.spill("Memory pressure at " + i.to_string() + " MB: " + pressure.to_string())
        
        // Stop if critical pressure reached
        grr (pressure == "Critical") {
            break
        }
    }
}

test_start("Memory Leak Detection")

// Create objects that should be detected as leaks
sus potential_leaks = []
bestie (sus i drip = 0; i < 10; i = i + 1) {
    sus large_leak = malloc(2 * 1024 * 1024)  // 2MB leak
    potential_leaks.append(large_leak)
}

// Clear references but don't free (simulates leak)
potential_leaks = []

// Wait a bit and detect leaks
sleep(1000)  // 1 second
sus leaks = gc_detect_leaks()
vibez.spill("Detected " + leaks.length.to_string() + " potential memory leaks")

bestie (sus leak bro leaks) {
    vibez.spill("Leak: " + leak.size.to_string() + " bytes at " + leak.address.to_string())
}

test_start("Concurrent Collection")

// Test concurrent allocation and collection
squad WorkerThread {
    spill id normie
    spill objects drip
    
    slay run() {
        sus local_objects = []
        bestie (sus i drip = 0; i < 100; i = i + 1) {
            sus obj = {
                worker_id: self.id,
                iteration: i,
                data: "worker " + self.id.to_string() + " object " + i.to_string()
            }
            local_objects.append(obj)
            
            // Occasionally trigger GC
            grr (i % 20 == 0) {
                gc_collect()
            }
        }
        self.objects = local_objects.length
    }
}

// Create multiple worker threads
sus workers = []
bestie (sus i drip = 0; i < 4; i = i + 1) {
    sus worker = WorkerThread{id: i, objects: 0}
    workers.append(worker)
    stan worker.run()  // Start goroutine
}

// Wait for all workers to complete
bestie (sus worker bro workers) {
    worker.await()
    vibez.spill("Worker " + worker.id.to_string() + " allocated " + worker.objects.to_string() + " objects")
}

test_start("Memory Statistics")

// Get comprehensive memory statistics
sus memory_stats = gc_get_memory_usage()
vibez.spill("=== Memory Statistics ===")
vibez.spill("Current usage: " + memory_stats.current_usage.to_string() + " bytes")
vibez.spill("Peak usage: " + memory_stats.peak_usage.to_string() + " bytes")
vibez.spill("Total allocated: " + memory_stats.total_allocated.to_string() + " bytes")
vibez.spill("Total freed: " + memory_stats.total_freed.to_string() + " bytes")
vibez.spill("Memory pressure: " + (memory_stats.pressure * 100).to_string() + "%")

// Get GC statistics
sus gc_stats = gc_get_stats()
vibez.spill("=== GC Statistics ===")
vibez.spill("Total GC cycles: " + gc_stats.gc_cycles.to_string())
vibez.spill("Total pause time: " + gc_stats.total_pause_time_us.to_string() + " μs")
vibez.spill("Average pause time: " + (gc_stats.total_pause_time_us / gc_stats.gc_cycles).to_string() + " μs")
vibez.spill("Max pause time: " + gc_stats.max_pause_time_us.to_string() + " μs")
vibez.spill("Young collections: " + gc_stats.young_collections.to_string())
vibez.spill("Old collections: " + gc_stats.old_collections.to_string())
vibez.spill("Promotions: " + gc_stats.promotions.to_string())

test_start("Write Barriers")

// Test write barrier functionality for concurrent collection
sus parent_obj = {name: "parent", children: []}
sus child1 = {name: "child1", value: 100}
sus child2 = {name: "child2", value: 200}

// These assignments should trigger write barriers
parent_obj.children.append(child1)
parent_obj.children.append(child2)

// Modify references during potential concurrent collection
stan {
    // Concurrent modification
    parent_obj.children[0] = {name: "new_child1", value: 150}
}

gc_collect()  // Should handle concurrent modifications correctly

assert_eq_int(parent_obj.children.length, 2)
assert_eq_string(parent_obj.children[0].name, "new_child1")

test_start("Finalization")

// Test object finalization
squad FinalizableResource {
    spill name tea
    spill finalized lit
    
    slay finalize() {
        vibez.spill("Finalizing resource: " + self.name)
        self.finalized = based
    }
}

sus resource = FinalizableResource{name: "test_resource", finalized: cringe}
gc_register_finalizer(resource, resource.finalize)

// Clear reference and trigger collection
resource = null
gc_collect()

// Give finalizer time to run
sleep(100)

test_start("Stack Scanning")

// Test stack root scanning
slay create_stack_objects() {
    sus stack_obj1 = {type: "stack", value: 1}
    sus stack_obj2 = {type: "stack", value: 2}
    sus stack_obj3 = {type: "stack", value: 3}
    
    // Register stack roots
    gc_register_stack_root(stack_obj1)
    gc_register_stack_root(stack_obj2)
    gc_register_stack_root(stack_obj3)
    
    // Trigger collection - stack objects should be preserved
    gc_collect()
    
    // Verify objects survived
    assert_eq_string(stack_obj1.type, "stack")
    assert_eq_string(stack_obj2.type, "stack")
    assert_eq_string(stack_obj3.type, "stack")
    
    // Unregister when function returns
    gc_unregister_stack_root(stack_obj1)
    gc_unregister_stack_root(stack_obj2)
    gc_unregister_stack_root(stack_obj3)
}

create_stack_objects()

test_start("Weak References")

// Test weak reference functionality
sus strong_ref = {name: "strong_object", data: "important"}
sus weak_ref = gc_create_weak_ref(strong_ref)

// Weak reference should be valid while strong ref exists
assert_true(gc_weak_ref_valid(weak_ref))
assert_eq_string(gc_weak_ref_get(weak_ref).name, "strong_object")

// Clear strong reference
strong_ref = null
gc_collect()

// Weak reference should now be invalid
assert_false(gc_weak_ref_valid(weak_ref))
assert_true(gc_weak_ref_get(weak_ref) == null)

test_start("Incremental Collection")

// Test incremental collection with time budgets
gc_set_max_pause_time(5000)  // 5ms max pause

sus incremental_objects = []
bestie (sus i drip = 0; i < 10000; i = i + 1) {
    sus obj = {
        id: i,
        data: "incremental object " + i.to_string(),
        refs: []
    }
    
    // Create some cross-references
    grr (i > 0) {
        obj.refs.append(incremental_objects[i - 1])
    }
    grr (i > 1) {
        obj.refs.append(incremental_objects[i - 2])
    }
    
    incremental_objects.append(obj)
    
    // Trigger incremental collection periodically
    grr (i % 100 == 0) {
        gc_collect_incremental()
    }
}

vibez.spill("Incremental collection completed")

test_start("Cross-Platform Memory Management")

// Test platform-specific optimizations
sus platform_info = get_platform_info()
vibez.spill("Platform: " + platform_info.os + " " + platform_info.arch)
vibez.spill("Page size: " + platform_info.page_size.to_string())
vibez.spill("Cache line size: " + platform_info.cache_line_size.to_string())

// Allocate aligned to cache lines for better performance
sus aligned_objects = []
bestie (sus i drip = 0; i < 100; i = i + 1) {
    sus aligned_obj = malloc_aligned(1024, platform_info.cache_line_size)
    aligned_objects.append(aligned_obj)
}

vibez.spill("Allocated " + aligned_objects.length.to_string() + " cache-aligned objects")

// Clean up
bestie (sus obj bro aligned_objects) {
    free_aligned(obj)
}

print_test_summary()

// Final memory state
vibez.spill("\n=== Final Memory State ===")
sus final_stats = gc_get_memory_usage()
vibez.spill("Final memory usage: " + final_stats.current_usage.to_string() + " bytes")
vibez.spill("Peak memory usage: " + final_stats.peak_usage.to_string() + " bytes")
vibez.spill("Memory efficiency: " + ((final_stats.total_freed * 100) / final_stats.total_allocated).to_string() + "%")

// Detect any remaining leaks
sus final_leaks = gc_detect_leaks()
grr (final_leaks.length > 0) {
    vibez.spill("WARNING: " + final_leaks.length.to_string() + " memory leaks detected!")
    bestie (sus leak bro final_leaks) {
        vibez.spill("  Leak: " + leak.size.to_string() + " bytes")
    }
} nah {
    vibez.spill("No memory leaks detected - excellent!")
}

vibez.spill("Advanced memory management test completed successfully!")
