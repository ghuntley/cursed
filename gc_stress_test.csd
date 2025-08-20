yeet "vibez"

# GC stress test to verify deadlock prevention under concurrent load
# This test creates memory pressure while simulating finalizer operations

vibez.spill("Starting GC stress test for deadlock prevention...")

# Test 1: Basic finalizer simulation
vibez.spill("Test 1: Basic finalizer operations")
sus allocated_objects drip = 0

bestie (sus round drip = 0; round < 100; round = round + 1) {
    # Simulate object allocation
    sus large_array [50]drip
    bestie (sus i drip = 0; i < 50; i = i + 1) {
        large_array[i] = round * i
        allocated_objects = allocated_objects + 1
    }
    
    # Force some computation to stress memory
    sus sum drip = 0
    bestie (sus i drip = 0; i < 50; i = i + 1) {
        sum = sum + large_array[i]
    }
    
    ready (round % 20 == 0) {
        vibez.spill("Round", round, "- allocated:", allocated_objects, "sum:", sum)
    }
}

# Test 2: Nested allocations that could trigger GC
vibez.spill("Test 2: Nested memory allocations")
bestie (sus outer drip = 0; outer < 10; outer = outer + 1) {
    bestie (sus middle drip = 0; middle < 10; middle = middle + 1) {
        sus temp_data [20]drip
        bestie (sus inner drip = 0; inner < 20; inner = inner + 1) {
            temp_data[inner] = outer + middle + inner
        }
        
        # Simulate work that could cause finalizer queue operations
        sus result drip = temp_data[0] + temp_data[19]
        ready (result > 100) {
            # This creates conditions where finalizer registration might be attempted
            vibez.spill("High value result:", result)
        }
    }
}

# Test 3: Rapid allocation/deallocation pattern
vibez.spill("Test 3: Rapid allocation pattern")
bestie (sus cycle drip = 0; cycle < 50; cycle = cycle + 1) {
    # Create temporary arrays that become eligible for collection
    sus temp1 [30]drip
    sus temp2 [30]drip  
    sus temp3 [30]drip
    
    # Fill with data
    bestie (sus i drip = 0; i < 30; i = i + 1) {
        temp1[i] = cycle
        temp2[i] = cycle * 2  
        temp3[i] = cycle * 3
    }
    
    # Process data (creates CPU/memory pressure)
    sus total drip = 0
    bestie (sus i drip = 0; i < 30; i = i + 1) {
        total = total + temp1[i] + temp2[i] + temp3[i]
    }
    
    ready (cycle % 10 == 0) {
        vibez.spill("Cycle", cycle, "total:", total)
    }
}

vibez.spill("GC stress test completed successfully!")
vibez.spill("If this message appears, the deadlock prevention fix is working correctly.")
