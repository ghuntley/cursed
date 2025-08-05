fr fr Comprehensive test of all new codegen features

vibez.spill("=== CURSED Comprehensive Codegen Test ===")

fr fr Test 1: Short declarations and basic operations
x := 42
y := 3.14
name := "CURSED"
vibez.spillf("Short declarations: x={}, y={}, name={}", x, y, name)

fr fr Test 2: Increment/Decrement operations  
counter := 10
vibez.spillf("Initial counter: {}", counter)
counter++
vibez.spillf("After increment: {}", counter)
counter--
vibez.spillf("After decrement: {}", counter)

fr fr Test 3: Arrays and indexing
numbers := [10, 20, 30, 40, 50]
vibez.spillf("First element: {}", numbers[0])
vibez.spillf("Third element: {}", numbers[2])

fr fr Test 4: Tuples and access
coordinates := (100, 200, 300)
vibez.spillf("First coordinate: {}", coordinates.0)
vibez.spillf("Second coordinate: {}", coordinates.1)

fr fr Test 5: Map creation (basic)
config := {"timeout": 30, "retries": 3}
vibez.spill("Map created successfully")

fr fr Test 6: Struct creation and access
squad Point {
    spill x drip
    spill y drip
}

point := Point{x: 15, y: 25}
vibez.spillf("Point coordinates: ({}, {})", point.x, point.y)

fr fr Test 7: Functions with return values
slay multiply(a drip, b drip) drip {
    damn a * b
}

result := multiply(6, 7)
vibez.spillf("Multiplication result: {}", result)

fr fr Test 8: Conditional statements
if result > 40 {
    vibez.spill("Result is large!")
} else {
    vibez.spill("Result is small!")
}

fr fr Test 9: Loop operations
vibez.spill("Loop test:")
i := 0
bestie i < 3 {
    vibez.spillf("  Iteration {}", i)
    i = i + 1
}

fr fr Test 10: Type operations and constants
const PI meal = 3.14159
area := PI * 5.0 * 5.0
vibez.spillf("Circle area (r=5): {}", area)

vibez.spill("=== All Codegen Features Working! ===")
