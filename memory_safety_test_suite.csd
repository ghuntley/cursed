# CURSED v1.0 Memory Safety Test Suite
# Oracle Quality Gate 3 - Comprehensive Memory Safety Testing

yeet "vibez"
yeet "arrayz"
yeet "stringz"

# Test 1: Basic variable operations and stack management
slay test_basic_variables() drip {
    sus x drip = 42
    sus y drip = x * 2
    sus z drip = y + x
    vibez.spill("Basic variables test: x =", x, "y =", y, "z =", z)
    damn z
}

# Test 2: Array operations and heap memory
slay test_array_memory() []drip {
    sus arr []drip = [1, 2, 3, 4, 5]
    sus i drip = 0
    bestie (i < 5) {
        arr[i] = arr[i] * 2
        i = i + 1
    }
    vibez.spill("Array memory test:", arr)
    damn arr
}

# Test 3: String operations and memory
slay test_string_memory() tea {
    sus str1 tea = "Hello"
    sus str2 tea = "World"
    sus combined tea = str1 + " " + str2
    vibez.spill("String memory test:", combined)
    damn combined
}

# Test 4: Recursive function memory usage
slay factorial(n drip) drip {
    ready (n <= 1) {
        damn 1
    }
    damn n * factorial(n - 1)
}

slay test_recursive_memory() drip {
    sus result drip = factorial(10)
    vibez.spill("Recursive memory test - factorial(10):", result)
    damn result
}

# Test 5: Loop operations with memory allocation
slay test_loop_memory() drip {
    sus sum drip = 0
    sus i drip = 0
    bestie (i < 100) {
        sus temp_array []drip = [i, i+1, i+2]
        sum = sum + temp_array[0]
        i = i + 1
    }
    vibez.spill("Loop memory test - sum:", sum)
    damn sum
}

# Test 6: Complex nested operations
slay test_complex_operations() lit {
    sus data [][]drip = [[1, 2], [3, 4], [5, 6]]
    sus total drip = 0
    sus row drip = 0
    bestie (row < 3) {
        sus col drip = 0
        bestie (col < 2) {
            total = total + data[row][col]
            col = col + 1
        }
        row = row + 1
    }
    vibez.spill("Complex operations test - total:", total)
    damn based
}

# Test 7: Memory stress test with repeated allocations
slay test_memory_stress() lit {
    sus i drip = 0
    bestie (i < 50) {
        sus temp_str tea = "Iteration " + stringz.from_int(i)
        sus temp_arr []drip = [i, i*2, i*3, i*4, i*5]
        vibez.spill("Stress test iteration:", i)
        i = i + 1
    }
    damn based
}

# Main test runner
vibez.spill("Starting CURSED v1.0 Memory Safety Test Suite")
vibez.spill("=====================================")

vibez.spill("Test 1: Basic Variables")
test_basic_variables()

vibez.spill("\nTest 2: Array Memory")
test_array_memory()

vibez.spill("\nTest 3: String Memory") 
test_string_memory()

vibez.spill("\nTest 4: Recursive Memory")
test_recursive_memory()

vibez.spill("\nTest 5: Loop Memory")
test_loop_memory()

vibez.spill("\nTest 6: Complex Operations")
test_complex_operations()

vibez.spill("\nTest 7: Memory Stress Test")
test_memory_stress()

vibez.spill("\n=====================================")
vibez.spill("Memory Safety Test Suite Completed")
