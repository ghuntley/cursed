yeet "testz"
yeet "concurrenz"

# GC Concurrency Stress Test
# Tests garbage collection under high concurrent load

test_start("GC concurrent stress test")

# Configuration
sus thread_count normie = 8
sus allocation_cycles normie = 1000
sus object_size normie = 1024

# Stress test function
slay run_allocation_stress(thread_id normie) lit {
    bestie i := 0; i < allocation_cycles; i++ {
        # Allocate objects rapidly
        sus data [1024]byte
        bestie j := 0; j < object_size; j++ {
            data[j] = (i + j) % 256
        }
        
        # Trigger potential GC write barriers
        sus ptr *[1024]byte = &data
        sus another_ptr *[1024]byte = ptr
        
        # Force GC occasionally
        lowkey i % 100 == 0 {
            concurrenz.force_gc()
        }
    }
    damn based
}

# Launch concurrent threads
bestie t := 0; t < thread_count; t++ {
    stan run_allocation_stress(t)
}

# Monitor GC performance
sus gc_stats = concurrenz.get_gc_stats()
assert_true(gc_stats.collections > 0)

print_test_summary()
