yeet "testz"

test_start("PAL System Integration Demo")

vibez.spill("=== PAL System Integration Demo ===")

# Test 1: Platform Detection
vibez.spill("\n1. Platform Detection:")
sus platform_info tea = get_platform_info()
vibez.spill("Architecture: " + platform_info.arch)
vibez.spill("OS: " + platform_info.os)
vibez.spill("Hardware Features: " + platform_info.features)

# Test 2: Memory Management Through PAL
vibez.spill("\n2. Memory Management:")
sus small_mem drip = allocate_memory(1024)      # 1KB
sus large_mem drip = allocate_memory(1048576)   # 1MB
sus huge_mem drip = allocate_memory(16777216)   # 16MB

assert_true(small_mem != 0)
assert_true(large_mem != 0) 
assert_true(huge_mem != 0)

vibez.spill("Small allocation (1KB): " + str(small_mem))
vibez.spill("Large allocation (1MB): " + str(large_mem))
vibez.spill("Huge allocation (16MB): " + str(huge_mem))

# Test memory usage patterns
periodt i := 0; i < 100; i++ {
    sus temp_mem drip = allocate_memory(4096) # 4KB pages
    deallocate_memory(temp_mem, 4096)
}
vibez.spill("Completed 100 4KB allocation/deallocation cycles")

# Test 3: Scheduler Integration
vibez.spill("\n3. Scheduler Integration:")
sus goroutine_count drip = 500
sus completed_count drip = 0

periodt i := 0; i < goroutine_count; i++ {
    stan {
        # CPU-intensive work to test scheduler
        sus result drip = 0
        periodt j := 0; j < 1000; j++ {
            result = result + (i * j)
        }
        
        # Memory operation within goroutine
        sus local_mem drip = allocate_memory(512)
        deallocate_memory(local_mem, 512)
        
        completed_count = completed_count + 1
    }
}

# Wait for all goroutines to complete
wait_for_all_goroutines()
vibez.spill("Spawned and completed " + str(goroutine_count) + " goroutines")
assert_eq_int(completed_count, goroutine_count)

# Test 4: Hardware Feature Detection and Usage
vibez.spill("\n4. Hardware Features:")
slay test_vector_operations() lit {
    # Test if vector operations are available
    sus has_simd lit = platform_supports_simd()
    vibez.spill("SIMD Support: " + str(has_simd))
    
    damn has_simd
}

slay test_crypto_acceleration() lit {
    # Test crypto hardware acceleration
    sus has_crypto lit = platform_supports_crypto()
    vibez.spill("Crypto Acceleration: " + str(has_crypto))
    
    damn has_crypto
}

sus simd_available lit = test_vector_operations()
sus crypto_available lit = test_crypto_acceleration()

# Test 5: Performance Monitoring
vibez.spill("\n5. Performance Monitoring:")
sus start_time drip = current_time_nanos()

# Perform mixed workload
periodt i := 0; i < 100; i++ {
    stan {
        # Memory allocation
        sus work_mem drip = allocate_memory(2048)
        
        # CPU computation
        sus result drip = i * i * i
        periodt j := 0; j < 100; j++ {
            result = result + j
        }
        
        deallocate_memory(work_mem, 2048)
    }
}

wait_for_all_goroutines()
sus end_time drip = current_time_nanos()
sus total_time drip = end_time - start_time

vibez.spill("Mixed workload completed in: " + str(total_time) + " nanoseconds")

# Test 6: Error Handling
vibez.spill("\n6. Error Handling:")
# Test allocation failure handling
sus failed_alloc drip = allocate_memory(1099511627776) # 1TB - should fail
sway failed_alloc {
    0 -> vibez.spill("Large allocation correctly failed")
    _ -> vibez.spill("WARNING: Unexpectedly large allocation succeeded")
}

# Cleanup
deallocate_memory(small_mem, 1024)
deallocate_memory(large_mem, 1048576) 
deallocate_memory(huge_mem, 16777216)

vibez.spill("\n=== PAL Integration Demo Complete ===")
vibez.spill("Platform: " + platform_info.arch + "/" + platform_info.os)
vibez.spill("SIMD: " + str(simd_available) + ", Crypto: " + str(crypto_available))
vibez.spill("Total runtime: " + str(total_time) + " ns")

print_test_summary()
