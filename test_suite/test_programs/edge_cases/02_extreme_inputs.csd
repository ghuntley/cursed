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
    
    fr fr Test large number in loops
    sus count normie = 0
    bestie i := 0; i < 100; i++ {
        count = count + large1
        ready (i % 25 == 0) {
            vibez.spill("Large accumulation at", i, ":", count)
        }
    }
}

slay test_very_long_strings() {
    vibez.spill("=== Very Long String Tests ===")
    
    fr fr Build progressively longer strings
    sus base_str = "CURSED"
    sus long_str = base_str
    
    bestie i := 0; i < 10; i++ {
        long_str = stringz.concat(long_str, base_str)
        vibez.spill("Iteration", i, "string length:", stringz.length(long_str))
    }
    
    vibez.spill("Final long string length:", stringz.length(long_str))
    
    fr fr Test operations on long string (simplified)
    sus upper_long = long_str  fr fr Simplified - no to_upper available
    sus lower_long = long_str  fr fr Simplified - no to_lower available
    
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
    
    sus huge_array normie[value] = normie[value]{}
    
    fr fr Build large array progressively  
    bestie i := 0; i < 500; i++ {
        huge_array = append(huge_array, i * i)
        
        ready (i % 100 == 0) {
            vibez.spill("Array size at", i, ":", len(huge_array))
        }
    }
    
    vibez.spill("Final huge array length:", len(huge_array))
    
    fr fr Test operations on large array
    sus sum normie = 0
    sus max_val normie = huge_array[0]
    sus min_val normie = huge_array[0]
    
    bestie i := 0; i < len(huge_array); i++ {
        sus val = huge_array[i]
        sum = sum + val
        
        ready (val > max_val) {
            max_val = val
        }
        
        ready (val < min_val) {
            min_val = val
        }
    }
    
    vibez.spill("Array sum:", sum)
    vibez.spill("Array max:", max_val)
    vibez.spill("Array min:", min_val)
}

slay test_nested_extreme_structures() {
    vibez.spill("=== Nested Extreme Structures ===")
    
    fr fr Create deeply nested arrays
    sus nested_arrays normie[value][value] = normie[value][value]{}
    
    bestie outer := 0; outer < 20; outer++ {
        sus inner_array normie[value] = normie[value]{}
        
        bestie inner := 0; inner < 25; inner++ {
            inner_array = append(inner_array, outer * inner)
        }
        
        nested_arrays = append(nested_arrays, inner_array)
        
        ready (outer % 5 == 0) {
            vibez.spill("Created nested layer", outer, "with", len(inner_array), "elements")
        }
    }
    
    vibez.spill("Total nested layers:", len(nested_arrays))
    
    fr fr Process nested structure
    sus total_elements normie = 0
    sus total_sum normie = 0
    
    bestie i := 0; i < len(nested_arrays); i++ {
        sus layer = nested_arrays[i]
        total_elements = total_elements + len(layer)
        
        bestie j := 0; j < len(layer); j++ {
            total_sum = total_sum + layer[j]
        }
    }
    
    vibez.spill("Total elements in nested structure:", total_elements)
    vibez.spill("Sum of all nested elements:", total_sum)
}

slay test_extreme_computation_loops() {
    vibez.spill("=== Extreme Computation Loops ===")
    
    sus computation_cycles = 1000
    sus result_accumulator normie = 0
    
    bestie cycle := 0; cycle < computation_cycles; cycle++ {
        fr fr Intensive computation per cycle
        sus cycle_sum normie = 0
        
        bestie inner := 0; inner < 50; inner++ {
            sus temp_val = mathz.abs(inner - 25)
            sus squared = mathz.pow(temp_val, 2)
            cycle_sum = cycle_sum + squared
        }
        
        result_accumulator = result_accumulator + cycle_sum
        
        ready (cycle % 200 == 0) {
            vibez.spill("Computation cycle", cycle, "accumulator:", result_accumulator)
        }
    }
    
    vibez.spill("Final computation result:", result_accumulator)
    vibez.spill("Average per cycle:", result_accumulator / computation_cycles)
}

slay test_mixed_extreme_operations() {
    vibez.spill("=== Mixed Extreme Operations ===")
    
    fr fr Combine all extreme patterns
    bestie mega_cycle := 0; mega_cycle < 5; mega_cycle++ {
        vibez.spill("Mega cycle", mega_cycle, "starting...")
        
        fr fr Large numbers
        sus big_num = 123456 + (mega_cycle * 111111)
        
        fr fr Long strings
        sus big_str = "MegaCycle"
        bestie str_expand := 0; str_expand < mega_cycle + 3; str_expand++ {
            big_str = stringz.concat(big_str, stringz.from_int(big_num))
        }
        
        fr fr Large arrays
        sus big_array normie[value] = normie[value]{}
        bestie arr_fill := 0; arr_fill < 100 + (mega_cycle * 20); arr_fill++ {
            big_array = append(big_array, mathz.abs(arr_fill - big_num))
        }
        
        vibez.spill("Cycle", mega_cycle, "- num:", big_num, "str_len:", stringz.length(big_str), "arr_len:", len(big_array))
    }
    
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
}
