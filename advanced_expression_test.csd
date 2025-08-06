# Advanced Expression Test Suite
# Tests P0 critical expression features: arrays, structs, function calls, method calls, complex expressions

# Test 1: Array indexing and array literals
sus numbers [normie] = [1, 2, 3, 4, 5]
sus first_num normie = numbers[0]
sus last_num normie = numbers[4]
vibez.spill("Array test:", first_num, last_num)

# Test 2: Struct field access and struct literals
squad Point {
    spill x normie
    spill y normie
}

sus p1 Point = Point{x: 10, y: 20}
sus x_coord normie = p1.x
sus y_coord normie = p1.y
vibez.spill("Struct test:", x_coord, y_coord)

# Test 3: Function call expressions with proper argument passing
slay add(a normie, b normie) normie {
    damn a + b
}

slay multiply(x normie, y normie) normie {
    damn x * y
}

sus sum normie = add(5, 3)
sus product normie = multiply(4, 7)
vibez.spill("Function calls:", sum, product)

# Test 4: Complex nested expressions with operator precedence
sus complex_expr normie = add(multiply(2, 3), add(4, 5))
sus precedence_test normie = 2 + 3 * 4 + 5
vibez.spill("Complex expressions:", complex_expr, precedence_test)

# Test 5: Array of structs with mixed access patterns
sus points [Point] = [
    Point{x: 1, y: 2},
    Point{x: 3, y: 4},
    Point{x: 5, y: 6}
]

sus second_point Point = points[1]
sus second_x normie = points[1].x
vibez.spill("Array of structs:", second_point.x, second_x)

# Test 6: Nested function calls with array/struct arguments
slay distance(p1 Point, p2 Point) normie {
    sus dx normie = p1.x - p2.x
    sus dy normie = p1.y - p2.y
    damn dx * dx + dy * dy  # Simplified distance (no sqrt)
}

sus dist normie = distance(points[0], points[2])
vibez.spill("Distance calculation:", dist)

# Test 7: Complex assignment and array modification
numbers[0] = add(numbers[1], numbers[2])
p1.x = multiply(p1.x, 2)
vibez.spill("After modification:", numbers[0], p1.x)

# Test 8: Method-style calls (if interfaces are working)
# This tests interface method dispatch
collab Drawable {
    slay draw() normie
}

squad Circle {
    spill radius normie
}

# Method implementation for Circle
slay draw_circle(c Circle) normie {
    vibez.spill("Drawing circle with radius:", c.radius)
    damn c.radius * 2  # Return diameter
}

# Test 9: Tuple expressions and access
sus coords (normie, normie) = (100, 200)
sus tuple_x normie = coords.0
sus tuple_y normie = coords.1
vibez.spill("Tuple test:", tuple_x, tuple_y)

# Test 10: Type casting expressions
sus float_val meal = 3.14
sus int_from_float normie = float_val.(normie)
sus float_from_int meal = 42.(meal)
vibez.spill("Type casts:", int_from_float, float_from_int)

vibez.spill("Advanced expression tests completed!")
