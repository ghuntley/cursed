vibe main
yeet "vibez"
yeet "collections"

// Test various memory allocation patterns
slay create_large_array() []normie {
    sus large_array []normie = []normie{}
    
    // Allocate progressively larger chunks
    finna i normie = 0; i < 1000; i++ {
        large_array = append(large_array, i * i)
    }
    
    damn large_array
}

slay create_nested_structures() [][]normie {
    sus nested [][]normie = [][]normie{}
    
    finna i normie = 0; i < 10; i++ {
        sus inner_array []normie = []normie{}
        finna j normie = 0; j < 100; j++ {
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
    finna i normie = 0; i < 50; i++ {
        sus temp = create_large_array()
        // temp goes out of scope, should be collected
    }
    
    vibez.spill("Memory intensive operations completed")
}

slay main() {
    vibez.spill("=== Memory Allocation Pattern Tests ===")
    
    memory_intensive_operations()
    
    // Test repeated allocations
    finna cycle normie = 0; cycle < 5; cycle++ {
        vibez.spill("Allocation cycle:", cycle)
        sus data = create_large_array()
        vibez.spill("Cycle", cycle, "allocation size:", len(data))
    }
    
    vibez.spill("All memory allocation tests completed")
}
