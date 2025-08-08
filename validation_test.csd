// Comprehensive test of type unification fixes
yeet "testz"
yeet "mathz"
yeet "stringz"

test_start("Type Unification Validation Tests")

// Test 1: Basic type inference should work
slay identity[T](x T) T {
    damn x
}

sus number drip = identity(42)
assert_eq_int(number, 42)

sus text tea = identity("hello")
assert_eq_string(text, "hello")

// Test 2: Complex generic function with inference
slay process_array[T](arr []T, func slay(T) drip) drip {
    sus sum drip = 0
    sus i drip = 0
    bestie (i < len(arr)) {
        sum = sum + func(arr[i])
        i = i + 1
    }
    damn sum
}

sus numbers []drip = [1, 2, 3, 4, 5]
sus result drip = process_array(numbers, slay(x drip) drip { damn x * 2 })
assert_eq_int(result, 30)  // (1+2+3+4+5) * 2 = 30

// Test 3: Nested generics should resolve correctly
slay nested_access[T](matrix [][]T, row drip, col drip) T {
    damn matrix[row][col]
}

sus matrix [][]drip = [[1, 2], [3, 4]]
sus value drip = nested_access(matrix, 1, 0)
assert_eq_int(value, 3)

// Test 4: Function return type inference
slay infer_return(condition lit) {
    ready (condition) {
        damn 42
    } otherwise {
        damn 84
    }
}

sus inferred_result = infer_return(based)
assert_eq_int(inferred_result, 42)

vibez.spill("Type unification tests completed successfully!")
print_test_summary()
