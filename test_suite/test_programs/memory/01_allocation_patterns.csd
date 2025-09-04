vibe main
yeet "vibez"
yeet "collections"

// Test various memory allocation patterns
slay create_large_array() normie[value] {
    sus large_array normie[value] = normie[value]{}
    
    // Allocate progressively larger chunks
    bestie i := 0; i < 1000; i++ {
        large_array = append(large_array, i * i)
    }
    
    damn large_array
}

slay create_nested_structures() normie[value][value] {
    sus nested normie[value][value] = normie[value][value]{}
    
    bestie i := 0; i < 10; i++ {
        sus inner_array normie[value] = normie[value]{}
        bestie j := 0; j < 100; j++ {
            inner_array = append(inner_array, i * j)
        }
        nested = append(nested, inner_array)
    }
    
    damn nested
}

slay memory_intensive_operations() {
    vibez.spill("Creating multiple large allocations...")
    
    sus array1 = create_large_array()
    vibez.spill("First array created, length:", len(array1))
    
    sus array2 = create_large_array() 
    vibez.spill("Second array created, length:", len(array2))
    
    sus nested = create_nested_structures()
    vibez.spill("Nested structure created, outer length:", len(nested))
    
    // Force some garbage collection pressure
    bestie i := 0; i < 50; i++ {
        sus temp = create_large_array()
        // temp goes out of scope, should be collected
    }
    
    vibez.spill("Memory intensive operations completed")
}

slay main_character() {
    vibez.spill("=== Memory Allocation Pattern Tests ===")
    
    memory_intensive_operations()
    
    // Test repeated allocations
    bestie cycle := 0; cycle < 5; cycle++ {
        vibez.spill("Allocation cycle:", cycle)
        sus data = create_large_array()
        vibez.spill("Cycle", cycle, "allocation size:", len(data))
    }
    
    vibez.spill("All memory allocation tests completed")
}
