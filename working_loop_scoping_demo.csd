fr fr CURSED Variable Scoping Demo - Working Examples

vibez.spill("=== CURSED Variable Scoping in Loops Demo ===")

fr fr Test 1: Basic variable access in loops
sus outer_sum drip = 0
sus outer_count drip = 0

vibez.spill("Test 1: Basic variable access")
bestie (outer_count < 3) {
    vibez.spill("  Iteration:", outer_count)
    vibez.spill("  Current sum:", outer_sum)
    outer_sum = outer_sum + outer_count
    outer_count = outer_count + 1
}
vibez.spill("Final sum:", outer_sum)

fr fr Test 2: Nested scoping
sus global_var drip = 100

ready (global_var > 50) {
    sus block_var drip = 200
    sus inner_counter drip = 0
    
    vibez.spill("Test 2: Nested scoping")
    bestie (inner_counter < 2) {
        vibez.spill("  Inner loop - global_var:", global_var)
        vibez.spill("  Inner loop - block_var:", block_var)
        vibez.spill("  Inner loop - counter:", inner_counter)
        inner_counter = inner_counter + 1
    }
}

fr fr Test 3: Array processing with pre-calculated length
sus data []drip = [10, 20, 30]
sus data_length drip = len(data)
sus index drip = 0
sus total drip = 0

vibez.spill("Test 3: Array processing")
vibez.spill("Array length:", data_length)

bestie (index < data_length) {
    vibez.spill("  Processing index:", index)
    vibez.spill("  Value:", data[index])
    total = total + data[index]
    index = index + 1
}
vibez.spill("Array sum:", total)

fr fr Test 4: Variable isolation - loop variables don't leak
ready (total > 0) {
    sus loop_only_var drip = 999
    bestie (loop_only_var > 995) {
        vibez.spill("  Loop-only var:", loop_only_var)
        loop_only_var = loop_only_var - 1
    }
    fr fr loop_only_var is not accessible outside this block
}

vibez.spill("=== All scoping tests completed successfully ===")
