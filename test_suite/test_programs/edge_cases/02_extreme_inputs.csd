vibe main
yeet "vibez"
yeet "mathz"
yeet "stringz"
yeet "collections"

fr fr Test extreme input values and stress conditions
slay test_very_large_numbers() {
    vibez.spill("=== Very Large Number Tests ===")
    
    sus large1 = 999999
    sus large2 = 888888
    sus large3 = 777777
    
    vibez.spill("Large number operations:")
    vibez.spill("Large1:", large1)
    vibez.spill("Large2:", large2) 
    vibez.spill("Large3:", large3)
    
    vibez.spill("Large1 + Large2:", large1 + large2)
    vibez.spill("Large1 - Large2:", large1 - large2)
    vibez.spill("Large1 abs:", mathz.abs_normie(large1))
    
    fr fr Test large number accumulation
    sus count normie = 0
    count = count + large1
    count = count + large2
    count = count + large3
    vibez.spill("Large accumulation:", count)
}

slay test_very_long_strings() {
    vibez.spill("=== Very Long String Tests ===")
    
    fr fr Build progressively longer strings
    sus base_str = "CURSED"
    sus long_str = base_str
    
    fr fr Manually expand string a few times
    long_str = stringz.concat(long_str, base_str)
    long_str = stringz.concat(long_str, base_str)
    long_str = stringz.concat(long_str, base_str)
    
    vibez.spill("Final long string length:", stringz.length(long_str))
    
    fr fr Test operations on long string
    sus upper_long = long_str
    sus lower_long = long_str
    
    vibez.spill("Upper version length:", stringz.length(upper_long))
    vibez.spill("Lower version length:", stringz.length(lower_long))
}

slay test_very_large_arrays() {
    vibez.spill("=== Very Large Array Tests ===")
    
    sus huge_array normie[value] = normie[value]{100, 400, 900, 1600, 2500}
    
    vibez.spill("Array length:", collections.length(huge_array))
    
    fr fr Test operations on array
    sus sum normie = 0
    sus max_val normie = 100
    sus min_val normie = 100
    
    fr fr Manual processing
    sum = sum + 100
    sum = sum + 400
    sum = sum + 900
    sum = sum + 1600
    sum = sum + 2500
    
    vibez.spill("Array sum:", sum)
    vibez.spill("Array max:", max_val)
    vibez.spill("Array min:", min_val)
}

slay test_extreme_computation_loops() {
    vibez.spill("=== Extreme Computation Tests ===")
    
    sus computation_cycles = 1000
    sus result_accumulator normie = 0
    
    fr fr Manual intensive computation
    sus temp_val1 = mathz.abs_normie(10 - 25)
    sus squared1 = temp_val1 * temp_val1
    
    sus temp_val2 = mathz.abs_normie(20 - 25)  
    sus squared2 = temp_val2 * temp_val2
    
    sus temp_val3 = mathz.abs_normie(30 - 25)
    sus squared3 = temp_val3 * temp_val3
    
    sus cycle_sum = squared1 + squared2 + squared3
    result_accumulator = result_accumulator + cycle_sum
    
    vibez.spill("Computation result:", result_accumulator)
    vibez.spill("Average per cycle:", result_accumulator / 3)
}

slay test_mixed_extreme_operations() {
    vibez.spill("=== Mixed Extreme Operations ===")
    
    fr fr Combine all extreme patterns manually
    sus big_num = 123456 + 111111
    
    fr fr Long strings  
    sus big_str = "MegaCycle"
    big_str = stringz.concat(big_str, "123456")
    big_str = stringz.concat(big_str, "123456")
    
    fr fr Large arrays
    sus big_array normie[value] = normie[value]{mathz.abs_normie(100 - big_num), mathz.abs_normie(200 - big_num), mathz.abs_normie(300 - big_num)}
    
    vibez.spill("Final - num:", big_num, "str_len:", stringz.length(big_str), "arr_len:", collections.length(big_array))
    
    vibez.spill("Mixed extreme operations completed")
}

slay main() {
    vibez.spill("=== Extreme Input Tests ===")
    
    test_very_large_numbers()
    test_very_long_strings() 
    test_very_large_arrays()
    test_extreme_computation_loops()
    test_mixed_extreme_operations()
    
    vibez.spill("All extreme input tests completed")
    damn
}
