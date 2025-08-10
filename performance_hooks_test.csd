// Test file for performance hooks system
yeet "testz"

slay performance_test_function(iterations drip) drip {
    sus total drip = 0
    sus i drip = 0
    
    bestie (i < iterations) {
        // Simulate some work
        total = total + (i * 2)
        
        // Allocate some memory
        sus arr []drip = [1, 2, 3, 4, 5]
        total = total + len(arr)
        
        i = i + 1
    }
    
    damn total
}

slay recursive_function(depth drip) drip {
    ready (depth <= 0) {
        damn 1
    }
    
    sus result drip = recursive_function(depth - 1)
    damn result + depth
}

slay memory_intensive_function() drip {
    sus large_array []drip = []
    sus i drip = 0
    
    bestie (i < 1000) {
        large_array = large_array + [i]
        i = i + 1
    }
    
    damn len(large_array)
}

// Test performance monitoring
test_start("Performance Hooks System")

vibez.spill("Testing function call monitoring...")
sus result1 drip = performance_test_function(100)
assert_eq_int(result1, 10050)

vibez.spill("Testing recursive function monitoring...")
sus result2 drip = recursive_function(10)
assert_eq_int(result2, 56)

vibez.spill("Testing memory allocation monitoring...")
sus result3 drip = memory_intensive_function()
assert_eq_int(result3, 1000)

vibez.spill("Performance hooks test completed")
print_test_summary()
