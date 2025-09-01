vibe main
yeet "vibez"

fr fr Memory Allocation Test - Simple Patterns
fr fr Tests: Basic array allocation and deallocation patterns
fr fr Expected: Successful memory operations without leaks

slay main() {
    vibez.spill("=== Memory Allocation Test ===")
    
    vibez.spill("Creating small array...")
    sus small_array []normie = []normie{1, 2, 3, 4, 5}
    vibez.spill("Small array length:", len(small_array))
    
    vibez.spill("Creating larger array...")
    sus large_array []normie = []normie{}
    
    finna i normie = 0; i < 100; i++ {
        large_array = append(large_array, i)
    }
    
    vibez.spill("Large array length:", len(large_array))
    
    vibez.spill("Creating multiple arrays...")
    finna j normie = 0; j < 10; j++ {
        sus temp_array []normie = []normie{}
        
        finna k normie = 0; k < 50; k++ {
            temp_array = append(temp_array, j * k)
        }
        
        vibez.spill("Temp array", j, "length:", len(temp_array))
    }
    
    vibez.spill("Memory allocation test completed")
}
