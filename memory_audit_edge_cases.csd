# Memory Safety Edge Cases Test
# Tests extreme scenarios that could cause memory leaks

yeet "vibez"
yeet "stringz"
yeet "mathz"

# Test 1: Deep recursion with large stack frames
slay deep_recursion_test(depth drip, data []drip) drip {
    ready (depth <= 0) {
        damn len(data)
    }
    
    sus new_data []drip = []
    bestie (sus i drip = 0; i < 100; i++) {
        new_data = append(new_data, depth * i)
    }
    
    damn deep_recursion_test(depth - 1, new_data)
}

# Test 2: Massive string concatenation
slay test_massive_strings() {
    sus result tea = ""
    bestie (sus i drip = 0; i < 1000; i++) {
        result = result + "Memory test iteration " + mathz.to_string(i) + " "
    }
    vibez.spill("Massive string length:", len(result))
}

# Test 3: Array growth and shrinking
slay test_dynamic_arrays() {
    sus dynamic []drip = []
    
    # Grow array
    bestie (sus i drip = 0; i < 5000; i++) {
        dynamic = append(dynamic, i)
    }
    
    # Access random elements
    bestie (sus i drip = 0; i < 100; i++) {
        sus index drip = i * 50
        ready (index < len(dynamic)) {
            vibez.spill("Element at", index, ":", dynamic[index])
        }
    }
}

# Test 4: Interleaved allocations
slay test_interleaved_allocations() {
    sus arrays [][]drip = []
    sus strings []tea = []
    
    bestie (sus i drip = 0; i < 50; i++) {
        sus new_array []drip = []
        bestie (sus j drip = 0; j < 10; j++) {
            new_array = append(new_array, i * j)
        }
        arrays = append(arrays, new_array)
        
        sus new_string tea = "Allocation " + mathz.to_string(i)
        strings = append(strings, new_string)
    }
    
    vibez.spill("Created", len(arrays), "arrays and", len(strings), "strings")
}

# Test 5: Function scope cleanup
slay function_scope_test() {
    bestie (sus outer drip = 0; outer < 10; outer++) {
        sus outer_array []drip = []
        bestie (sus inner drip = 0; inner < 100; inner++) {
            sus inner_string tea = "Scope test " + mathz.to_string(outer) + "_" + mathz.to_string(inner)
            outer_array = append(outer_array, len(inner_string))
        }
        # outer_array and inner_string should be cleaned up here
    }
}

# Main execution
vibez.spill("Starting memory safety edge cases test...")

vibez.spill("Deep recursion result:", deep_recursion_test(50, [1, 2, 3]))
test_massive_strings()
test_dynamic_arrays()
test_interleaved_allocations()
function_scope_test()

vibez.spill("Edge cases test completed!")
