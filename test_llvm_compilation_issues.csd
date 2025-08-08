// Test cases for LLVM compilation issues

yeet "arrayz"
yeet "mathz"
yeet "testz"

// Test 1: Recursive function calls
slay factorial(n drip) drip {
    ready (n <= 1) {
        damn 1
    }
    damn n * factorial(n - 1)
}

// Test 2: Complex expression evaluation
slay complex_expr_test() drip {
    sus a drip = 5
    sus b drip = 3
    sus result drip = (a + b) * 2 - (a - b) / 2
    damn result
}

// Test 3: Array length function usage
slay array_length_test() drip {
    sus numbers []drip = [1, 2, 3, 4, 5]
    sus count drip = len(numbers)
    damn count
}

// Test 4: Struct field access
squad Point {
    spill x drip
    spill y drip
}

slay struct_field_test() drip {
    sus p Point = Point{x: 10, y: 20}
    sus sum drip = p.x + p.y
    damn sum
}

// Test 5: Combined complex expression with recursive calls
slay fib(n drip) drip {
    ready (n <= 0) {
        damn 0
    }
    ready (n == 1) {
        damn 1
    }
    damn fib(n - 1) + fib(n - 2)
}

// Test 6: Array operations with complex expressions
slay array_operations_test() drip {
    sus data []drip = [1, 2, 3, 4]
    sus total drip = 0
    sus i drip = 0
    bestie (i < len(data)) {
        total = total + data[i] * 2
        i = i + 1
    }
    damn total
}

// Main test function
slay main() {
    test_start("LLVM Compilation Issues Test")
    
    // Test recursive functions
    sus fact_result drip = factorial(5)
    vibez.spill("Factorial(5):", fact_result)
    assert_eq_int(fact_result, 120)
    
    // Test complex expressions
    sus expr_result drip = complex_expr_test()
    vibez.spill("Complex expression result:", expr_result)
    assert_eq_int(expr_result, 15)
    
    // Test array length
    sus length_result drip = array_length_test()
    vibez.spill("Array length:", length_result)
    assert_eq_int(length_result, 5)
    
    // Test struct field access
    sus struct_result drip = struct_field_test()
    vibez.spill("Struct field sum:", struct_result)
    assert_eq_int(struct_result, 30)
    
    // Test fibonacci
    sus fib_result drip = fib(6)
    vibez.spill("Fibonacci(6):", fib_result)
    assert_eq_int(fib_result, 8)
    
    // Test array operations
    sus array_result drip = array_operations_test()
    vibez.spill("Array operations result:", array_result)
    assert_eq_int(array_result, 20)
    
    print_test_summary()
}
