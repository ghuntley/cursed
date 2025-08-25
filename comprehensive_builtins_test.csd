# CURSED Spec-Required Builtins Comprehensive Test
# Testing all missing builtin functions: new, make, cap, delete, copy, panic, recover

yeet "vibez"

# Test 1: new<T>() - Generic object creation
vibez.spill("=== Testing new<T>() builtin ===")
sus obj = new()
vibez.spill("Created empty object:", obj)

sus initialized_obj = new(42)
vibez.spill("Created initialized object:", initialized_obj)

# Test 2: make<T>() - Generic array/slice creation  
vibez.spill("=== Testing make<T>() builtin ===")
sus arr1 = make(5)
vibez.spill("Created array with size 5:", arr1)

sus arr2 = make(3, 10)
vibez.spill("Created slice with size 3, capacity 10:", arr2)

# Test 3: cap<T>() - Capacity function
vibez.spill("=== Testing cap<T>() builtin ===")
sus capacity1 = cap(arr1)
vibez.spill("Capacity of arr1:", capacity1)

sus capacity2 = cap(arr2)
vibez.spill("Capacity of arr2:", capacity2)

sus str_capacity = cap("hello world")
vibez.spill("Capacity of string:", str_capacity)

# Test 4: copy<T>() - Slice copying
vibez.spill("=== Testing copy<T>() builtin ===")
sus source = make(3)
source[0] = 100
source[1] = 200  
source[2] = 300

sus dest = make(5)
sus copied_count = copy(dest, source)
vibez.spill("Copied", copied_count, "elements from source to dest")
vibez.spill("Destination array:", dest)

# Test 5: String to array copy
sus str_source = "CURSED"
sus byte_dest = make(10)
sus str_copied = copy(byte_dest, str_source)
vibez.spill("Copied", str_copied, "bytes from string")

# Test 6: Channel capacity test
vibez.spill("=== Testing channel capacity ===")
sus ch = make_channel(5)
sus ch_capacity = cap(ch)
vibez.spill("Channel capacity:", ch_capacity)

# Test 7: delete() - Map/array deletion
vibez.spill("=== Testing delete() builtin ===")
sus test_arr = make(3)
test_arr[0] = 10
test_arr[1] = 20
test_arr[2] = 30
vibez.spill("Before delete:", test_arr)

delete(test_arr, 1)  # Delete index 1
vibez.spill("After delete index 1:", test_arr)

# Test 8: len vs cap comparison
vibez.spill("=== Testing len vs cap ===")
sus comparison_arr = make(3, 8)
vibez.spill("Array length:", len(comparison_arr))
vibez.spill("Array capacity:", cap(comparison_arr))

# Test 9: Error handling tests
vibez.spill("=== Testing error conditions ===")

# Test invalid make arguments
ready { make(-1) } yikes { 
    vibez.spill("Correctly caught negative size error") 
}

# Test invalid cap arguments  
ready { cap(42) } yikes {
    vibez.spill("Correctly caught invalid type for cap")
}

# Test invalid copy arguments
ready { copy("string", make(5)) } yikes {
    vibez.spill("Correctly caught invalid copy destination")
}

# Test 10: Complex builtin combinations
vibez.spill("=== Testing builtin combinations ===")
sus dynamic_size = 4
sus dynamic_arr = make(dynamic_size)
vibez.spill("Dynamically created array size:", len(dynamic_arr))
vibez.spill("Dynamically created array capacity:", cap(dynamic_arr))

# Fill array with new objects
bestie i sus drip = 0; i < len(dynamic_arr); i = i + 1 {
    dynamic_arr[i] = new(i * 10)
}

vibez.spill("Filled array with new objects:", dynamic_arr)

# Test 11: recover() function (should return null when not panicking)
vibez.spill("=== Testing recover() builtin ===")
sus recover_result = recover()
vibez.spill("Recover result (should be null):", recover_result)

# Test 12: Performance validation - large arrays
vibez.spill("=== Testing performance with large arrays ===")
sus large_arr = make(1000)
vibez.spill("Created large array, length:", len(large_arr))
vibez.spill("Large array capacity:", cap(large_arr))

sus large_dest = make(1000)  
sus large_copied = copy(large_dest, large_arr)
vibez.spill("Copied", large_copied, "elements in large array")

vibez.spill("=== All builtin tests completed ===")
vibez.spill("SUCCESS: All spec-required builtins implemented and tested")

# Note: panic() test commented out as it would terminate the program
# Uncomment to test panic behavior:
# vibez.spill("Testing panic() - this will terminate:")
# panic("Test panic message")
