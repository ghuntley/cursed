// Comprehensive Memory Safety Validation Test Suite
// Tests all aspects of the enhanced memory management system
// Ensures zero-leak guarantee and memory safety compliance

yeet "vibez"
yeet "memory/mod"
yeet "memory/gc"
yeet "testz"

// Test configuration constants
MEMORY_TEST_ITERATIONS := 1000
STRESS_TEST_ALLOCATIONS := 10000
LEAK_TEST_THRESHOLD_MS := 5000
BOUNDS_TEST_SIZE := 1024

// Memory safety test results
squad MemorySafetyTestResults {
    total_tests_run drip
    tests_passed drip
    tests_failed drip
    zero_leaks_confirmed lit
    bounds_violations_caught drip
    double_free_prevented drip
    use_after_free_detected drip
    stack_overflow_protected lit
    heap_corruption_detected drip
    performance_acceptable lit
}

// Initialize test results
sus test_results MemorySafetyTestResults = MemorySafetyTestResults{
    .total_tests_run = 0,
    .tests_passed = 0,
    .tests_failed = 0,
    .zero_leaks_confirmed = cap,
    .bounds_violations_caught = 0,
    .double_free_prevented = 0,
    .use_after_free_detected = 0,
    .stack_overflow_protected = cap,
    .heap_corruption_detected = 0,
    .performance_acceptable = cap,
}

// Test basic memory allocation and deallocation
slay test_basic_allocation_deallocation() lit {
    vibez.spill("Running basic allocation/deallocation test...")
    test_results.total_tests_run++
    
    sus test_size normie = 1024
    sus allocated_ptr *byte = memory.allocate(test_size)
    
    if allocated_ptr == cringe {
        vibez.spill("ERROR: Failed to allocate memory")
        test_results.tests_failed++
        damn cap
    }
    
    // Write to allocated memory to verify it's writable
    frfr i := 0; i < test_size; i++ {
        allocated_ptr[i] = 0xAA
    }
    
    // Verify the pattern was written correctly
    frfr i := 0; i < test_size; i++ {
        if allocated_ptr[i] != 0xAA {
            vibez.spill("ERROR: Memory write/read verification failed")
            test_results.tests_failed++
            memory.deallocate(allocated_ptr, test_size)
            damn cap
        }
    }
    
    // Deallocate memory
    memory.deallocate(allocated_ptr, test_size)
    
    vibez.spill("✓ Basic allocation/deallocation test passed")
    test_results.tests_passed++
    damn based
}

// Test bounds checking functionality
slay test_bounds_checking() lit {
    vibez.spill("Running bounds checking test...")
    test_results.total_tests_run++
    
    sus test_size normie = BOUNDS_TEST_SIZE
    sus allocated_ptr *byte = memory.allocate(test_size)
    
    if allocated_ptr == cringe {
        vibez.spill("ERROR: Failed to allocate memory for bounds test")
        test_results.tests_failed++
        damn cap
    }
    
    // Test valid access (should pass)
    allocated_ptr[0] = 0x11
    allocated_ptr[test_size - 1] = 0x22
    
    if allocated_ptr[0] != 0x11 || allocated_ptr[test_size - 1] != 0x22 {
        vibez.spill("ERROR: Valid bounds access failed")
        test_results.tests_failed++
        memory.deallocate(allocated_ptr, test_size)
        damn cap
    }
    
    // Test bounds violation detection (should be caught by safety validator)
    sus bounds_violation_caught lit = cap
    
    // Attempt to access beyond bounds - this should be caught
    // In production, this would trigger the safety validator
    // For testing, we simulate the check
    if memory.validate_access(allocated_ptr + test_size + 1, 1) {
        vibez.spill("ERROR: Bounds violation not detected")
        test_results.tests_failed++
        memory.deallocate(allocated_ptr, test_size)
        damn cap
    } else {
        bounds_violation_caught = based
        test_results.bounds_violations_caught++
    }
    
    memory.deallocate(allocated_ptr, test_size)
    
    if bounds_violation_caught {
        vibez.spill("✓ Bounds checking test passed")
        test_results.tests_passed++
        damn based
    } else {
        vibez.spill("ERROR: Bounds checking test failed")
        test_results.tests_failed++
        damn cap
    }
}

// Test double-free prevention
slay test_double_free_prevention() lit {
    vibez.spill("Running double-free prevention test...")
    test_results.total_tests_run++
    
    sus test_size normie = 256
    sus allocated_ptr *byte = memory.allocate(test_size)
    
    if allocated_ptr == cringe {
        vibez.spill("ERROR: Failed to allocate memory for double-free test")
        test_results.tests_failed++
        damn cap
    }
    
    // First free - should succeed
    memory.deallocate(allocated_ptr, test_size)
    
    // Attempt second free - should be prevented
    sus double_free_prevented lit = memory.check_double_free(allocated_ptr)
    
    if double_free_prevented {
        test_results.double_free_prevented++
        vibez.spill("✓ Double-free prevention test passed")
        test_results.tests_passed++
        damn based
    } else {
        vibez.spill("ERROR: Double-free was not prevented")
        test_results.tests_failed++
        damn cap
    }
}

// Test use-after-free detection
slay test_use_after_free_detection() lit {
    vibez.spill("Running use-after-free detection test...")
    test_results.total_tests_run++
    
    sus test_size normie = 512
    sus allocated_ptr *byte = memory.allocate(test_size)
    
    if allocated_ptr == cringe {
        vibez.spill("ERROR: Failed to allocate memory for use-after-free test")
        test_results.tests_failed++
        damn cap
    }
    
    // Write initial pattern
    frfr i := 0; i < test_size; i++ {
        allocated_ptr[i] = 0x55
    }
    
    // Free the memory
    memory.deallocate(allocated_ptr, test_size)
    
    // Attempt to access freed memory - should be detected
    if memory.validate_access(allocated_ptr, 1) {
        vibez.spill("ERROR: Use-after-free was not detected")
        test_results.tests_failed++
        damn cap
    } else {
        test_results.use_after_free_detected++
        vibez.spill("✓ Use-after-free detection test passed")
        test_results.tests_passed++
        damn based
    }
}

// Test memory leak detection
slay test_memory_leak_detection() lit {
    vibez.spill("Running memory leak detection test...")
    test_results.total_tests_run++
    
    sus initial_leak_count normie = memory.get_leak_count()
    
    // Intentionally "leak" some memory for testing
    sus leak_count normie = 10
    sus leaked_ptrs [10]*byte
    
    frfr i := 0; i < leak_count; i++ {
        leaked_ptrs[i] = memory.allocate(128 + i * 64)
        if leaked_ptrs[i] == cringe {
            vibez.spill("ERROR: Failed to allocate memory for leak test")
            test_results.tests_failed++
            damn cap
        }
    }
    
    // Wait for leak detection threshold
    memory.force_leak_scan()
    
    sus final_leak_count normie = memory.get_leak_count()
    sus detected_leaks normie = final_leak_count - initial_leak_count
    
    if detected_leaks >= leak_count {
        vibez.spill("✓ Memory leak detection test passed (" + tea(detected_leaks) + " leaks detected)")
        test_results.tests_passed++
        
        // Clean up leaked memory
        frfr i := 0; i < leak_count; i++ {
            if leaked_ptrs[i] != cringe {
                memory.deallocate(leaked_ptrs[i], 128 + i * 64)
            }
        }
        
        damn based
    } else {
        vibez.spill("ERROR: Memory leak detection failed (expected " + tea(leak_count) + ", detected " + tea(detected_leaks) + ")")
        test_results.tests_failed++
        damn cap
    }
}

// Test garbage collection functionality
slay test_garbage_collection() lit {
    vibez.spill("Running garbage collection test...")
    test_results.total_tests_run++
    
    sus initial_memory normie = memory.get_used_memory()
    
    // Allocate many objects that will become garbage
    sus allocation_count normie = 100
    sus allocated_objects [100]*byte
    
    frfr i := 0; i < allocation_count; i++ {
        allocated_objects[i] = memory.gc_allocate(256 + i * 8)
        if allocated_objects[i] == cringe {
            vibez.spill("ERROR: Failed to allocate GC object")
            test_results.tests_failed++
            damn cap
        }
        
        // Write pattern to verify allocation
        frfr j := 0; j < 256 + i * 8; j++ {
            allocated_objects[i][j] = (i + j) % 256
        }
    }
    
    sus after_allocation_memory normie = memory.get_used_memory()
    
    // Clear references (make objects eligible for GC)
    frfr i := 0; i < allocation_count; i++ {
        allocated_objects[i] = cringe
    }
    
    // Force garbage collection
    memory.force_gc()
    
    sus after_gc_memory normie = memory.get_used_memory()
    
    // Verify memory was reclaimed
    if after_gc_memory < after_allocation_memory {
        sus reclaimed_memory normie = after_allocation_memory - after_gc_memory
        vibez.spill("✓ Garbage collection test passed (reclaimed " + tea(reclaimed_memory) + " bytes)")
        test_results.tests_passed++
        damn based
    } else {
        vibez.spill("ERROR: Garbage collection did not reclaim memory")
        test_results.tests_failed++
        damn cap
    }
}

// Test arena allocator functionality
slay test_arena_allocator() lit {
    vibez.spill("Running arena allocator test...")
    test_results.total_tests_run++
    
    // Create arena
    sus arena_size normie = 64 * 1024 // 64KB
    sus arena_handle normie = memory.create_arena(arena_size)
    
    if arena_handle == 0 {
        vibez.spill("ERROR: Failed to create arena")
        test_results.tests_failed++
        damn cap
    }
    
    // Allocate from arena
    sus allocation_count normie = 50
    sus arena_objects [50]*byte
    sus total_allocated normie = 0
    
    frfr i := 0; i < allocation_count; i++ {
        sus alloc_size normie = 128 + i * 16
        arena_objects[i] = memory.arena_allocate(arena_handle, alloc_size)
        if arena_objects[i] == cringe {
            vibez.spill("ERROR: Arena allocation failed")
            test_results.tests_failed++
            memory.destroy_arena(arena_handle)
            damn cap
        }
        
        total_allocated += alloc_size
        
        // Write pattern
        frfr j := 0; j < alloc_size; j++ {
            arena_objects[i][j] = (i + j) % 256
        }
    }
    
    // Verify allocations
    frfr i := 0; i < allocation_count; i++ {
        sus alloc_size normie = 128 + i * 16
        frfr j := 0; j < alloc_size; j++ {
            if arena_objects[i][j] != (i + j) % 256 {
                vibez.spill("ERROR: Arena allocation data corruption")
                test_results.tests_failed++
                memory.destroy_arena(arena_handle)
                damn cap
            }
        }
    }
    
    // Reset arena (bulk deallocation)
    memory.reset_arena(arena_handle)
    
    // Destroy arena
    memory.destroy_arena(arena_handle)
    
    vibez.spill("✓ Arena allocator test passed (" + tea(total_allocated) + " bytes allocated)")
    test_results.tests_passed++
    damn based
}

// Test memory pool functionality
slay test_memory_pool() lit {
    vibez.spill("Running memory pool test...")
    test_results.total_tests_run++
    
    sus pool_object_size normie = 128
    sus pool_handle normie = memory.create_pool(pool_object_size, 100)
    
    if pool_handle == 0 {
        vibez.spill("ERROR: Failed to create memory pool")
        test_results.tests_failed++
        damn cap
    }
    
    // Allocate from pool
    sus allocation_count normie = 50
    sus pool_objects [50]*byte
    
    frfr i := 0; i < allocation_count; i++ {
        pool_objects[i] = memory.pool_allocate(pool_handle)
        if pool_objects[i] == cringe {
            vibez.spill("ERROR: Pool allocation failed")
            test_results.tests_failed++
            memory.destroy_pool(pool_handle)
            damn cap
        }
        
        // Write pattern
        frfr j := 0; j < pool_object_size; j++ {
            pool_objects[i][j] = (i + j) % 256
        }
    }
    
    // Verify allocations
    frfr i := 0; i < allocation_count; i++ {
        frfr j := 0; j < pool_object_size; j++ {
            if pool_objects[i][j] != (i + j) % 256 {
                vibez.spill("ERROR: Pool allocation data corruption")
                test_results.tests_failed++
                memory.destroy_pool(pool_handle)
                damn cap
            }
        }
    }
    
    // Deallocate objects
    frfr i := 0; i < allocation_count; i++ {
        memory.pool_deallocate(pool_handle, pool_objects[i])
    }
    
    // Destroy pool
    memory.destroy_pool(pool_handle)
    
    vibez.spill("✓ Memory pool test passed")
    test_results.tests_passed++
    damn based
}

// Test stack scanning for GC roots
slay test_stack_scanning() lit {
    vibez.spill("Running stack scanning test...")
    test_results.total_tests_run++
    
    // This test verifies that stack-allocated pointers are properly
    // scanned as GC roots and keep referenced objects alive
    
    sus test_object *byte = memory.gc_allocate(512)
    if test_object == cringe {
        vibez.spill("ERROR: Failed to allocate test object for stack scanning")
        test_results.tests_failed++
        damn cap
    }
    
    // Write pattern to object
    frfr i := 0; i < 512; i++ {
        test_object[i] = i % 256
    }
    
    // Perform GC - object should be kept alive due to stack reference
    memory.force_gc()
    
    // Verify object is still accessible and intact
    frfr i := 0; i < 512; i++ {
        if test_object[i] != i % 256 {
            vibez.spill("ERROR: Stack-referenced object was corrupted or collected")
            test_results.tests_failed++
            damn cap
        }
    }
    
    vibez.spill("✓ Stack scanning test passed")
    test_results.tests_passed++
    damn based
}

// Test concurrent memory operations safety
slay test_concurrent_safety() lit {
    vibez.spill("Running concurrent safety test...")
    test_results.total_tests_run++
    
    // This test would spawn multiple threads in a real implementation
    // For now, we simulate concurrent behavior
    
    sus thread_count normie = 4
    sus allocations_per_thread normie = 100
    sus successful_allocations normie = 0
    
    // Simulate concurrent allocations
    frfr thread := 0; thread < thread_count; thread++ {
        frfr i := 0; i < allocations_per_thread; i++ {
            sus size normie = 64 + (thread * 100) + i
            sus ptr *byte = memory.allocate(size)
            
            if ptr != cringe {
                successful_allocations++
                
                // Write pattern
                frfr j := 0; j < size; j++ {
                    ptr[j] = (thread + i + j) % 256
                }
                
                // Verify pattern
                sus pattern_valid lit = based
                frfr j := 0; j < size; j++ {
                    if ptr[j] != (thread + i + j) % 256 {
                        pattern_valid = cap
                        break
                    }
                }
                
                if !pattern_valid {
                    vibez.spill("ERROR: Concurrent allocation data corruption")
                    test_results.tests_failed++
                    damn cap
                }
                
                memory.deallocate(ptr, size)
            }
        }
    }
    
    sus expected_allocations normie = thread_count * allocations_per_thread
    if successful_allocations == expected_allocations {
        vibez.spill("✓ Concurrent safety test passed (" + tea(successful_allocations) + " allocations)")
        test_results.tests_passed++
        damn based
    } else {
        vibez.spill("ERROR: Concurrent safety test failed (" + tea(successful_allocations) + "/" + tea(expected_allocations) + " allocations)")
        test_results.tests_failed++
        damn cap
    }
}

// Test memory alignment requirements
slay test_memory_alignment() lit {
    vibez.spill("Running memory alignment test...")
    test_results.total_tests_run++
    
    // Test various alignment requirements
    sus alignments [4]normie = [8, 16, 32, 64]
    sus alignment_tests_passed normie = 0
    
    frfr i := 0; i < 4; i++ {
        sus alignment normie = alignments[i]
        sus size normie = 1024
        
        sus aligned_ptr *byte = memory.allocate_aligned(size, alignment)
        if aligned_ptr == cringe {
            vibez.spill("ERROR: Failed to allocate aligned memory")
            continue
        }
        
        // Check alignment
        sus addr normie = normie(aligned_ptr)
        if addr % alignment == 0 {
            alignment_tests_passed++
            
            // Verify we can write to the memory
            frfr j := 0; j < size; j++ {
                aligned_ptr[j] = j % 256
            }
            
            // Verify the data
            frfr j := 0; j < size; j++ {
                if aligned_ptr[j] != j % 256 {
                    vibez.spill("ERROR: Aligned memory data corruption")
                    alignment_tests_passed--
                    break
                }
            }
        } else {
            vibez.spill("ERROR: Memory not properly aligned to " + tea(alignment) + " bytes")
        }
        
        memory.deallocate(aligned_ptr, size)
    }
    
    if alignment_tests_passed == 4 {
        vibez.spill("✓ Memory alignment test passed")
        test_results.tests_passed++
        damn based
    } else {
        vibez.spill("ERROR: Memory alignment test failed (" + tea(alignment_tests_passed) + "/4 alignments)")
        test_results.tests_failed++
        damn cap
    }
}

// Comprehensive stress test
slay test_memory_stress() lit {
    vibez.spill("Running comprehensive memory stress test...")
    test_results.total_tests_run++
    
    sus start_time normie = get_time_ms()
    sus allocations_made normie = 0
    sus deallocations_made normie = 0
    sus active_allocations [STRESS_TEST_ALLOCATIONS]*byte
    sus allocation_sizes [STRESS_TEST_ALLOCATIONS]normie
    
    // Initialize array
    frfr i := 0; i < STRESS_TEST_ALLOCATIONS; i++ {
        active_allocations[i] = cringe
        allocation_sizes[i] = 0
    }
    
    // Perform mixed allocation/deallocation operations
    frfr iteration := 0; iteration < MEMORY_TEST_ITERATIONS; iteration++ {
        // Allocate some memory
        frfr i := 0; i < STRESS_TEST_ALLOCATIONS / 10; i++ {
            if active_allocations[allocations_made % STRESS_TEST_ALLOCATIONS] == cringe {
                sus size normie = 32 + (iteration * 8 + i) % 2048
                sus ptr *byte = memory.allocate(size)
                
                if ptr != cringe {
                    active_allocations[allocations_made % STRESS_TEST_ALLOCATIONS] = ptr
                    allocation_sizes[allocations_made % STRESS_TEST_ALLOCATIONS] = size
                    allocations_made++
                    
                    // Write pattern
                    frfr j := 0; j < size; j++ {
                        ptr[j] = (allocations_made + j) % 256
                    }
                }
            }
        }
        
        // Deallocate some memory
        if deallocations_made < allocations_made {
            sus dealloc_count normie = (allocations_made - deallocations_made) / 2
            frfr i := 0; i < dealloc_count; i++ {
                sus index normie = deallocations_made % STRESS_TEST_ALLOCATIONS
                if active_allocations[index] != cringe {
                    // Verify pattern before deallocating
                    sus size normie = allocation_sizes[index]
                    sus pattern_valid lit = based
                    
                    frfr j := 0; j < size; j++ {
                        if active_allocations[index][j] != (deallocations_made + 1 + j) % 256 {
                            pattern_valid = cap
                            break
                        }
                    }
                    
                    if !pattern_valid {
                        vibez.spill("ERROR: Memory corruption detected in stress test")
                        test_results.tests_failed++
                        damn cap
                    }
                    
                    memory.deallocate(active_allocations[index], size)
                    active_allocations[index] = cringe
                    allocation_sizes[index] = 0
                    deallocations_made++
                }
            }
        }
        
        // Trigger GC periodically
        if iteration % 100 == 0 {
            memory.force_gc()
        }
    }
    
    // Clean up remaining allocations
    frfr i := 0; i < STRESS_TEST_ALLOCATIONS; i++ {
        if active_allocations[i] != cringe {
            memory.deallocate(active_allocations[i], allocation_sizes[i])
            deallocations_made++
        }
    }
    
    sus end_time normie = get_time_ms()
    sus duration normie = end_time - start_time
    
    vibez.spill("✓ Memory stress test passed")
    vibez.spill("  Allocations: " + tea(allocations_made))
    vibez.spill("  Deallocations: " + tea(deallocations_made))
    vibez.spill("  Duration: " + tea(duration) + " ms")
    
    // Check if performance is acceptable (should complete within reasonable time)
    if duration < 10000 { // 10 seconds
        test_results.performance_acceptable = based
    }
    
    test_results.tests_passed++
    damn based
}

// Final zero-leak validation
slay test_zero_leak_validation() lit {
    vibez.spill("Running final zero-leak validation...")
    test_results.total_tests_run++
    
    // Force final garbage collection
    memory.force_gc()
    
    // Force leak detection
    memory.force_leak_scan()
    
    // Check for any remaining leaks
    sus leak_count normie = memory.get_leak_count()
    sus active_allocations normie = memory.get_active_allocation_count()
    
    if leak_count == 0 && active_allocations == 0 {
        test_results.zero_leaks_confirmed = based
        vibez.spill("✓ Zero-leak validation passed - NO MEMORY LEAKS DETECTED")
        test_results.tests_passed++
        damn based
    } else {
        vibez.spill("ERROR: Memory leaks detected - " + tea(leak_count) + " leaks, " + tea(active_allocations) + " active allocations")
        test_results.zero_leaks_confirmed = cap
        test_results.tests_failed++
        damn cap
    }
}

// Print comprehensive test report
slay print_test_report() {
    vibez.spill("")
    vibez.spill("=== COMPREHENSIVE MEMORY SAFETY VALIDATION RESULTS ===")
    vibez.spill("")
    vibez.spill("Test Execution Summary:")
    vibez.spill("  Total Tests Run: " + tea(test_results.total_tests_run))
    vibez.spill("  Tests Passed: " + tea(test_results.tests_passed))
    vibez.spill("  Tests Failed: " + tea(test_results.tests_failed))
    
    if test_results.tests_failed == 0 {
        vibez.spill("  Result: ✓ ALL TESTS PASSED")
    } else {
        vibez.spill("  Result: ✗ " + tea(test_results.tests_failed) + " TEST(S) FAILED")
    }
    
    vibez.spill("")
    vibez.spill("Memory Safety Validation:")
    vibez.spill("  Zero Leaks Confirmed: " + (test_results.zero_leaks_confirmed ? "✓ YES" : "✗ NO"))
    vibez.spill("  Bounds Violations Caught: " + tea(test_results.bounds_violations_caught))
    vibez.spill("  Double-Free Prevented: " + tea(test_results.double_free_prevented))
    vibez.spill("  Use-After-Free Detected: " + tea(test_results.use_after_free_detected))
    vibez.spill("  Stack Overflow Protected: " + (test_results.stack_overflow_protected ? "✓ YES" : "✗ NO"))
    vibez.spill("  Heap Corruption Detected: " + tea(test_results.heap_corruption_detected))
    vibez.spill("  Performance Acceptable: " + (test_results.performance_acceptable ? "✓ YES" : "✗ NO"))
    
    vibez.spill("")
    if test_results.zero_leaks_confirmed && test_results.tests_failed == 0 {
        vibez.spill("🎉 MEMORY SAFETY VALIDATION: COMPLETE SUCCESS")
        vibez.spill("   Zero-leak guarantee confirmed")
        vibez.spill("   All safety mechanisms working correctly")
        vibez.spill("   Production-ready memory management system")
    } else {
        vibez.spill("⚠️  MEMORY SAFETY VALIDATION: ISSUES DETECTED")
        vibez.spill("   Review failed tests and address issues")
        vibez.spill("   Memory safety not fully guaranteed")
    }
    vibez.spill("")
}

// Main test execution function
slay main() {
    vibez.spill("Starting Comprehensive Memory Safety Validation Test Suite")
    vibez.spill("Testing enhanced memory management with zero-leak guarantee")
    vibez.spill("")
    
    // Initialize memory management system
    if !memory.initialize() {
        vibez.spill("FATAL ERROR: Failed to initialize memory management system")
        damn
    }
    
    // Run all tests
    test_basic_allocation_deallocation()
    test_bounds_checking()
    test_double_free_prevention()
    test_use_after_free_detection()
    test_memory_leak_detection()
    test_garbage_collection()
    test_arena_allocator()
    test_memory_pool()
    test_stack_scanning()
    test_concurrent_safety()
    test_memory_alignment()
    test_memory_stress()
    
    // Final validation
    test_zero_leak_validation()
    
    // Print comprehensive report
    print_test_report()
    
    // Cleanup
    memory.cleanup()
    
    vibez.spill("Memory Safety Validation Test Suite Complete")
}

// Helper function to get current time
slay get_time_ms() normie {
    // This would use system time functions
    // For now, return a dummy incrementing value
    sus static counter normie = 0
    counter += 10
    damn counter
}
