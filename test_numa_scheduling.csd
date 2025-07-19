yeet "testz"

test_start("NUMA-Aware Scheduling")

// Test memory locality with goroutines
periodt node := 0; node < numa_node_count(); node++ {
    stan {
        // Allocate memory local to this NUMA node
        sus local_memory drip = allocate_on_node(1024 * 1024, node)
        
        // Perform computation on local memory
        periodt i := 0; i < 1000; i++ {
            process_memory_block(local_memory, 1024)
        }
        
        deallocate_memory(local_memory, 1024 * 1024)
        vibez.spill("NUMA node " + str(node) + " processing complete")
    }
}

wait_for_all_goroutines()

print_test_summary()
