fr fr Test the memory module in interpreter mode
yeet "memory"

fr fr Test basic memory allocation
vibez.spill("Testing memory allocation...")
sus ptr normie = memory.malloc(1024)
yo ptr != 0 {
    vibez.spill("✓ Memory allocation successful")
} otherwise {
    vibez.spill("✗ Memory allocation failed")
}

fr fr Test memory setting
yo ptr != 0 {
    sus success lit = memory.memset(ptr, 42, 1024)
    yo success {
        vibez.spill("✓ Memory set successful")
    } otherwise {
        vibez.spill("✗ Memory set failed")
    }
}

fr fr Test memory reallocation
sus new_ptr normie = memory.realloc(ptr, 2048)
yo new_ptr != 0 {
    vibez.spill("✓ Memory reallocation successful")
    ptr = new_ptr
} otherwise {
    vibez.spill("✗ Memory reallocation failed")
}

fr fr Test memory copy
sus src_ptr normie = memory.malloc(512)
yo src_ptr != 0 && ptr != 0 {
    fr fr Set source memory
    memory.memset(src_ptr, 99, 512)
    
    fr fr Copy from src to dest
    sus copy_success lit = memory.memcpy(ptr, src_ptr, 512)
    yo copy_success {
        vibez.spill("✓ Memory copy successful")
    } otherwise {
        vibez.spill("✗ Memory copy failed")
    }
    
    fr fr Free source memory
    memory.free(src_ptr)
}

fr fr Test garbage collection
sus gc_result lit = memory.gc_collect()
yo gc_result {
    vibez.spill("✓ Garbage collection triggered")
} otherwise {
    vibez.spill("✗ Garbage collection failed")
}

fr fr Get memory statistics
sus stats vibe = memory.get_memory_stats()
vibez.spill("Memory Statistics:")
vibez.spill(stats)

fr fr Clean up
yo ptr != 0 {
    memory.free(ptr)
    vibez.spill("✓ Memory freed")
}

vibez.spill("Memory module test complete!")
