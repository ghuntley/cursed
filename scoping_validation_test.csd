fr fr Variable scoping validation test

sus global_counter drip = 0
sus numbers []drip = [1, 2, 3, 4, 5]
sus array_size drip = len(numbers)  # Pre-calculate length to avoid expression parsing issue

vibez.spill("=== Variable Scoping Test ===")
vibez.spill("Global counter:", global_counter)
vibez.spill("Array size:", array_size)

ready (array_size > 0) {
    sus block_var drip = 42
    vibez.spill("In if block - block_var:", block_var)
    vibez.spill("In if block - global_counter:", global_counter)
    
    sus i drip = 0
    bestie (i < array_size) {
        vibez.spill("Loop iteration:", i)
        vibez.spill("  - numbers[", i, "]:", numbers[i])
        vibez.spill("  - block_var:", block_var)
        vibez.spill("  - global_counter:", global_counter)
        
        global_counter = global_counter + numbers[i]
        i = i + 1
    }
    
    vibez.spill("After loop - global_counter:", global_counter)
}

vibez.spill("=== Final Results ===")
vibez.spill("Global counter (sum):", global_counter)
vibez.spill("Expected sum: 15")

fr fr Test loop variable scoping - loop variables should not leak
ready (global_counter > 0) {
    sus loop_var drip = 999
    bestie (loop_var > 995) {
        vibez.spill("Loop var:", loop_var)
        loop_var = loop_var - 1
    }
    vibez.spill("Loop var after loop:", loop_var)
}

vibez.spill("=== Scoping Test Complete ===")
