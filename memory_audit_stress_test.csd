# Comprehensive Memory Safety Stress Test
# Tests loops, functions, arrays, strings, and recursive operations

yeet "vibez"
yeet "mathz"
yeet "stringz"
yeet "arrayz"

# Test 1: Large array operations with memory allocation
slay test_large_arrays() {
    sus large_array []drip = []
    bestie (sus i drip = 0; i < 1000; i++) {
        large_array = append(large_array, i * i)
    }
    
    sus sum drip = 0
    bestie (sus i drip = 0; i < len(large_array); i++) {
        sum = sum + large_array[i]
    }
    
    vibez.spill("Large array sum:", sum)
}

# Test 2: Recursive function with memory allocation
slay fibonacci_recursive(n drip) drip {
    ready (n <= 1) {
        damn n
    }
    damn fibonacci_recursive(n - 1) + fibonacci_recursive(n - 2)
}

# Test 3: String manipulation stress test
slay test_string_operations() {
    sus base_string tea = "CURSED Memory Test "
    sus result tea = ""
    
    bestie (sus i drip = 0; i < 100; i++) {
        result = result + base_string + mathz.to_string(i)
    }
    
    vibez.spill("String operations completed, length:", len(result))
}

# Test 4: Nested function calls with local variables
slay nested_function_test(depth drip) drip {
    ready (depth <= 0) {
        damn 1
    }
    
    sus local_array []drip = [1, 2, 3, 4, 5]
    sus local_string tea = "depth " + mathz.to_string(depth)
    
    damn nested_function_test(depth - 1) * len(local_array)
}

# Test 5: Complex data structure manipulation
slay test_complex_structures() {
    sus matrix [][]drip = []
    bestie (sus i drip = 0; i < 10; i++) {
        sus row []drip = []
        bestie (sus j drip = 0; j < 10; j++) {
            row = append(row, i * j)
        }
        matrix = append(matrix, row)
    }
    
    sus total drip = 0
    bestie (sus i drip = 0; i < len(matrix); i++) {
        bestie (sus j drip = 0; j < len(matrix[i]); j++) {
            total = total + matrix[i][j]
        }
    }
    
    vibez.spill("Matrix sum:", total)
}

# Main execution
vibez.spill("Starting comprehensive memory safety stress test...")

test_large_arrays()
vibez.spill("Fibonacci(20):", fibonacci_recursive(20))
test_string_operations()
vibez.spill("Nested function result:", nested_function_test(5))
test_complex_structures()

vibez.spill("Memory safety stress test completed successfully!")
