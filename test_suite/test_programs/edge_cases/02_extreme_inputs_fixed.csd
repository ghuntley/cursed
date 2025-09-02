vibe main
yeet "vibez"
yeet "mathz"
yeet "stringz"

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
    
    fr fr Test substring operations on long strings
    sus substring1 = stringz.substring(long_str, 0, 10)
    sus substring2 = stringz.substring(long_str, 10, 20)
    
    vibez.spill("Substring1:", substring1)
    vibez.spill("Substring2:", substring2)
}

slay test_very_large_arrays() {
    vibez.spill("=== Very Large Array Tests ===")
    
    sus huge_array []normie = []normie{}
    
    fr fr Build large array manually
    huge_array = append(huge_array, 100)
    huge_array = append(huge_array, 400)
    huge_array = append(huge_array, 900)
    huge_array = append(huge_array, 1600)
    huge_array = append(huge_array, 2500)
    
    vibez.spill("Array length:", len(huge_array))
    
    fr fr Test operations on array
    sus sum normie = 0
    sus max_val normie = huge_array[0]
    sus min_val normie = huge_array[0]
    
    fr fr Manual processing since for loops have issues
    sum = sum + huge_array[0]
    sum = sum + huge_array[1] 
    sum = sum + huge_array[2]
    sum = sum + huge_array[3]
    sum = sum + huge_array[4]
    
    vibez.spill("Array sum:", sum)
    vibez.spill("Array max:", max_val)
    vibez.spill("Array min:", min_val)
}

slay test_nested_extreme_structures() {
    vibez.spill("=== Nested Extreme Structures ===")
    
    fr fr Create simple nested arrays without loops
    sus nested_arrays [][]normie = [][]normie{}
    
    sus inner_array1 []normie = []normie{}
    inner_array1 = append(inner_array1, 10)
    inner_array1 = append(inner_array1, 20)
    inner_array1 = append(inner_array1, 30)
    
    sus inner_array2 []normie = []normie{}
    inner_array2 = append(inner_array2, 40)
    inner_array2 = append(inner_array2, 50)
    
    nested_arrays = append(nested_arrays, inner_array1)
    nested_arrays = append(nested_arrays, inner_array2)
    
    vibez.spill("Total nested layers:", len(nested_arrays))
    
    fr fr Process nested structure manually
    sus total_elements normie = 0
    sus total_sum normie = 0
    
    total_elements = total_elements + len(nested_arrays[0])
    total_elements = total_elements + len(nested_arrays[1])
    
    total_sum = total_sum + nested_arrays[0][0]
    total_sum = total_sum + nested_arrays[0][1]
    total_sum = total_sum + nested_arrays[0][2]
    total_sum = total_sum + nested_arrays[1][0]
    total_sum = total_sum + nested_arrays[1][1]
    
    vibez.spill("Total elements in nested structure:", total_elements)
    vibez.spill("Sum of all nested elements:", total_sum)
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
    sus big_array []normie = []normie{}
    big_array = append(big_array, mathz.abs_normie(100 - big_num))
    big_array = append(big_array, mathz.abs_normie(200 - big_num))
    big_array = append(big_array, mathz.abs_normie(300 - big_num))
    
    vibez.spill("Final - num:", big_num, "str_len:", stringz.length(big_str), "arr_len:", len(big_array))
    
    vibez.spill("Mixed extreme operations completed")
}

slay main() {
    vibez.spill("=== Extreme Input Tests ===")
    
    test_very_large_numbers()
    test_very_long_strings() 
    test_very_large_arrays()
    test_nested_extreme_structures()
    test_extreme_computation_loops()
    test_mixed_extreme_operations()
    
    vibez.spill("All extreme input tests completed")
    damn
}
