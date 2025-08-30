yeet "memory"

vibez.spill("=== CURSED Memory Module Test ===")
vibez.spill("")

fr fr Test 1: Basic allocation
vibez.spill("1. Testing memory allocation...")
sus ptr1 normie = memory.malloc(1024)
yo ptr1 != 0 {
    vibez.spill("✓ Allocated 1024 bytes successfully")
} otherwise {
    vibez.spill("✗ Allocation failed")
}

fr fr Test 2: Memory setting
yo ptr1 != 0 {
    vibez.spill("2. Testing memory set...")
    sus set_result lit = memory.memset(ptr1, 42, 100)
    yo set_result {
        vibez.spill("✓ Memory set to value 42 successfully")
    } otherwise {
        vibez.spill("✗ Memory set failed")
    }
}

fr fr Test 3: Reallocation
vibez.spill("3. Testing memory reallocation...")
sus ptr2 normie = memory.realloc(ptr1, 2048)
yo ptr2 != 0 {
    vibez.spill("✓ Reallocated to 2048 bytes successfully")
    ptr1 = ptr2
} otherwise {
    vibez.spill("✗ Reallocation failed")
}

fr fr Test 4: Second allocation for copy test
vibez.spill("4. Testing second allocation...")
sus src_ptr normie = memory.malloc(512)
yo src_ptr != 0 {
    vibez.spill("✓ Second allocation successful")
    
    fr fr Set source memory to different value
    memory.memset(src_ptr, 99, 512)
    
    fr fr Test 5: Memory copy
    vibez.spill("5. Testing memory copy...")
    sus copy_result lit = memory.memcpy(ptr1, src_ptr, 512)
    yo copy_result {
        vibez.spill("✓ Memory copy successful")
    } otherwise {
        vibez.spill("✗ Memory copy failed")
    }
    
    fr fr Free source memory
    memory.free(src_ptr)
    vibez.spill("✓ Source memory freed")
}

fr fr Test 6: Garbage collection
vibez.spill("6. Testing garbage collection...")
sus gc_result lit = memory.gc_collect()
yo gc_result {
    vibez.spill("✓ Garbage collection completed")
} otherwise {
    vibez.spill("✗ Garbage collection failed")
}

fr fr Test 7: Memory statistics
vibez.spill("7. Getting memory statistics...")
sus stats vibe = memory.get_memory_stats()
vibez.spill(stats)

fr fr Clean up
yo ptr1 != 0 {
    memory.free(ptr1)
    vibez.spill("✓ Main memory freed")
}

vibez.spill("")
vibez.spill("=== Memory Module Test Complete ===")
