# Comprehensive JIT Test - All CURSED Features
# Tests all the newly implemented JIT functionality

# Function parameter handling test
slay add_numbers(x normie, y normie) normie {
    damn x + y
}

# Struct creation and member access test
squad Point {
    spill x normie
    spill y normie
}

slay struct_test() {
    p := Point{ x: 10, y: 20 }
    vibez.spill("Point coordinates:", p.x, p.y)
    damn p.x + p.y
}

# Array creation and access test
slay array_test() {
    arr := [1, 2, 3, 4, 5]
    vibez.spill("Array length:", arr.length)
    vibez.spill("First element:", arr.element_0)
    vibez.spill("Second element:", arr.element_1)
    damn arr.element_0 + arr.element_1
}

# Tuple creation and access test
slay tuple_test() {
    tup := (42, "hello", based)
    vibez.spill("Tuple length:", tup.length)
    vibez.spill("First element:", tup._0)
    vibez.spill("Second element:", tup._1)
    vibez.spill("Third element:", tup._2)
    damn tup._0
}

# Interface test
collab Drawable {
    slay draw()
}

squad Circle {
    spill radius normie
}

# Lambda expression test (simplified)
slay lambda_test() {
    # Create a lambda that adds two numbers
    add_lambda := lambda(a, b) { a + b }
    vibez.spill("Lambda created with param count:", add_lambda.param_count)
    damn 100
}

# Type conversion test
slay type_conversion_test() {
    x := 42
    y := x.(meal)  # Convert to float
    s := y.(tea)   # Convert to string
    vibez.spill("Type conversions:", x, y, s)
    damn 1
}

# String concatenation test
slay string_concat_test() {
    name := "CURSED"
    version := "1.0"
    result := name + " " + version
    vibez.spill("Concatenated string:", result)
    damn result
}

# Main test function that exercises all features
slay main() {
    vibez.spill("🧪 JIT Comprehensive Test Starting")
    
    # Test function parameters
    result1 := add_numbers(10, 20)
    vibez.spill("add_numbers(10, 20) =", result1)
    
    # Test structs
    result2 := struct_test()
    vibez.spill("struct_test() =", result2)
    
    # Test arrays
    result3 := array_test()
    vibez.spill("array_test() =", result3)
    
    # Test tuples
    result4 := tuple_test()
    vibez.spill("tuple_test() =", result4)
    
    # Test lambdas
    result5 := lambda_test()
    vibez.spill("lambda_test() =", result5)
    
    # Test type conversions
    result6 := type_conversion_test()
    vibez.spill("type_conversion_test() =", result6)
    
    # Test string concatenation
    result7 := string_concat_test()
    vibez.spill("string_concat_test() =", result7)
    
    vibez.spill("✅ All JIT tests completed!")
    damn 0
}

main()
