yeet "testz"

test_start("WASM PAL Performance Test")

// Test memory operations performance (aiming for >100K ops/sec)
sus start_time drip = current_time_nanos()
sus mem_ops drip = 1000

periodt i := 0; i < mem_ops; i++ {
    sus ptr drip = allocate_memory(64)
    deallocate_memory(ptr, 64)
}

sus end_time drip = current_time_nanos()
sus memory_duration drip = end_time - start_time
sus ops_per_sec drip = (mem_ops * 1_000_000_000) / memory_duration

vibez.spill("Memory operations: " + str(ops_per_sec) + " ops/sec")

// Test goroutine spawning performance (aiming for >10K spawns/sec) 
start_time = current_time_nanos()
sus goroutine_count drip = 100

periodt i := 0; i < goroutine_count; i++ {
    stan {
        sus result drip = i * i
    }
}

wait_for_all_goroutines()

end_time = current_time_nanos()
sus goroutine_duration drip = end_time - start_time
sus spawns_per_sec drip = (goroutine_count * 1_000_000_000) / goroutine_duration

vibez.spill("Goroutine spawns: " + str(spawns_per_sec) + " spawns/sec")

// Verify we meet the WASM32 performance requirements
assert_true(ops_per_sec > 50000)        // At least 50K ops/sec for memory
assert_true(spawns_per_sec > 5000)      // At least 5K spawns/sec for goroutines

print_test_summary()
