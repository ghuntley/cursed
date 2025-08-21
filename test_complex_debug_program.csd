//! Complex CURSED program for debugger testing
//! Tests various language features including functions, loops, arrays, and error handling

yeet "vibez"
yeet "mathz"
yeet "concurrenz"

// Global variables for debugging
sus global_counter drip = 0
sus global_data []drip = [1, 2, 3, 4, 5]

// Test function with parameters and local variables
slay calculate_fibonacci(n drip) drip {
    sus a drip = 0
    sus b drip = 1
    
    ready (n <= 0) {
        damn 0
    }
    
    ready (n == 1) {
        damn 1
    }
    
    bestie (sus i drip = 2; i <= n; i += 1) {
        sus temp drip = a + b
        a = b
        b = temp
        global_counter += 1
    }
    
    damn b
}

// Function with array operations
slay process_array(arr []drip) []drip {
    sus result []drip = []
    
    bestie (sus i drip = 0; i < len(arr); i += 1) {
        sus value drip = arr[i] * 2
        result.push(value)
        
        // Conditional logic for debugging
        ready (value > 5) {
            vibez.spill("Large value:", value)
        }
    }
    
    damn result
}

// Function with error handling
slay divide_safely(a drip, b drip) yikes<tea> {
    ready (b == 0) {
        yikes "division by zero"
    }
    
    sus result drip = a / b
    damn result
}

// Concurrent function for advanced debugging
slay concurrent_worker(id drip, data []drip) {
    vibez.spill("Worker", id, "starting...")
    
    bestie (sus i drip = 0; i < len(data); i += 1) {
        sus value drip = data[i] * id
        global_counter += value
        
        // Simulate some work
        ready (i % 2 == 0) {
            vibez.spill("Worker", id, "processed:", value)
        }
    }
    
    vibez.spill("Worker", id, "completed")
}

// Struct for complex data debugging
squad DataContainer {
    id drip,
    name tea,
    values []drip,
    active lit
}

// Main function - entry point for debugging
slay main() {
    vibez.spill("=== Complex CURSED Program Debug Test ===")
    
    // Test basic variable debugging
    sus counter drip = 0
    sus message tea = "Debug test starting"
    sus active lit = based
    
    vibez.spill("Initial values:")
    vibez.spill("  counter:", counter)
    vibez.spill("  message:", message)
    vibez.spill("  active:", active)
    
    // Test function calls and stepping
    vibez.spill("\n=== Testing Fibonacci calculation ===")
    bestie (sus i drip = 1; i <= 8; i += 1) {
        sus fib_result drip = calculate_fibonacci(i)
        vibez.spill("fib({}) = {}", i, fib_result)
        counter += fib_result
    }
    
    // Test array operations
    vibez.spill("\n=== Testing array processing ===")
    sus test_array []drip = [1, 2, 3, 4, 5, 6, 7, 8]
    sus processed_array []drip = process_array(test_array)
    
    vibez.spill("Original:", test_array)
    vibez.spill("Processed:", processed_array)
    
    // Test error handling
    vibez.spill("\n=== Testing error handling ===")
    sus safe_division drip = divide_safely(10, 2) fam {
        when _ -> {
            vibez.spill("Division error caught")
            damn 0
        }
    }
    vibez.spill("Safe division result:", safe_division)
    
    // Test error case
    sus unsafe_division drip = divide_safely(10, 0) fam {
        when "division by zero" -> {
            vibez.spill("Caught division by zero!")
            damn -1
        }
    }
    
    // Test struct creation and manipulation
    vibez.spill("\n=== Testing struct operations ===")
    sus container DataContainer = DataContainer {
        id: 1001,
        name: "Test Container",
        values: [10, 20, 30, 40, 50],
        active: based
    }
    
    vibez.spill("Container ID:", container.id)
    vibez.spill("Container name:", container.name)
    vibez.spill("Container values:", container.values)
    vibez.spill("Container active:", container.active)
    
    // Test concurrent operations (commented out for basic debugging)
    // vibez.spill("\n=== Testing concurrent operations ===")
    // go concurrent_worker(1, [1, 2, 3])
    // go concurrent_worker(2, [4, 5, 6])
    
    // Test loops with breakpoint opportunities
    vibez.spill("\n=== Testing complex loops ===")
    sus nested_counter drip = 0
    
    bestie (sus outer drip = 1; outer <= 3; outer += 1) {
        vibez.spill("Outer loop iteration:", outer)
        
        bestie (sus inner drip = 1; inner <= 4; inner += 1) {
            nested_counter += outer * inner
            
            ready (nested_counter > 10) {
                vibez.spill("  Inner loop - nested_counter:", nested_counter)
            }
        }
    }
    
    // Final summary
    vibez.spill("\n=== Debug test summary ===")
    vibez.spill("Final counter:", counter)
    vibez.spill("Global counter:", global_counter)
    vibez.spill("Nested counter:", nested_counter)
    vibez.spill("Program execution completed successfully!")
}

// Entry point
main()
