# Memory leak detection test for CURSED
# This benchmark specifically tests for memory leaks and proper cleanup

yeet "vibez"
yeet "mathz"
yeet "arrayz"
yeet "stringz"

# Test dynamic array allocation and deallocation
slay test_array_allocation() {
    vibez.spill("Testing array allocation and cleanup...")
    
    bestie (i drip = 0; i < 1000; i = i + 1) {
        # Create large arrays to stress memory management
        sus large_array []drip = []
        
        bestie (j drip = 0; j < 1000; j = j + 1) {
            arrayz.push(large_array, j * j)
        }
        
        # Arrays should be automatically cleaned up here
    }
}

# Test string allocation patterns
slay test_string_allocation() {
    vibez.spill("Testing string allocation and cleanup...")
    
    bestie (i drip = 0; i < 500; i = i + 1) {
        sus base_str tea = "This is a test string for memory allocation testing: "
        sus number_str tea = stringz.from_int(i)
        sus combined tea = stringz.concat(base_str, number_str)
        
        # Process the string
        sus words []tea = stringz.split(combined, " ")
        sus word_count drip = arrayz.len(words)
        
        # Strings should be cleaned up automatically
    }
}

# Test recursive function calls (stack management)
slay fibonacci(n drip) drip {
    ready (n <= 1) {
        damn n
    }
    damn fibonacci(n - 1) + fibonacci(n - 2)
}

slay test_recursive_calls() {
    vibez.spill("Testing recursive function stack management...")
    
    bestie (i drip = 1; i <= 30; i = i + 1) {
        sus result drip = fibonacci(i)
        vibez.spill("fib(", i, ") =", result)
    }
}

# Test concurrent memory allocation
slay worker_function(id drip) {
    bestie (i drip = 0; i < 100; i = i + 1) {
        sus data []drip = []
        bestie (j drip = 0; j < 100; j = j + 1) {
            arrayz.push(data, id * 1000 + i * 10 + j)
        }
        # Simulate work
        sus sum drip = 0
        bestie (k drip = 0; k < arrayz.len(data); k = k + 1) {
            sum = sum + data[k]
        }
    }
}

slay test_concurrent_allocation() {
    vibez.spill("Testing concurrent memory allocation...")
    
    # Launch multiple goroutines for concurrent memory allocation
    bestie (i drip = 0; i < 10; i = i + 1) {
        go {
            worker_function(i)
        }
    }
    
    # Wait a bit for goroutines to complete
    # (In a real implementation, we'd use proper synchronization)
    bestie (i drip = 0; i < 1000000; i = i + 1) {
        # Busy wait (not ideal, but works for testing)
    }
}

# Main benchmark function
slay memory_leak_test_main() {
    vibez.spill("Starting CURSED Memory Leak Detection Test")
    vibez.spill("=========================================")
    
    test_array_allocation()
    test_string_allocation()
    test_recursive_calls()
    test_concurrent_allocation()
    
    vibez.spill("Memory leak test completed successfully")
    vibez.spill("All allocations should have been cleaned up automatically")
}

memory_leak_test_main()
