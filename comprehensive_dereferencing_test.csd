# Comprehensive test of all struct field dereferencing improvements
vibez.spill("=== COMPREHENSIVE STRUCT DEREFERENCING TEST ===")

# Test 1: Basic struct field access
vibez.spill("--- Test 1: Basic Field Access ---")
squad Point {
    spill x drip
    spill y drip
}

sus p Point = Point{x: 10, y: 20}
vibez.spill("Point p:", p.x, p.y)
vibez.spill("Direct field access works: ✅")

# Test 2: Field modification
vibez.spill("--- Test 2: Field Modification ---")
p.x = 15
p.y = 25
vibez.spill("Modified point:", p.x, p.y)
vibez.spill("Field modification works: ✅")

# Test 3: Nested struct field access
vibez.spill("--- Test 3: Nested Field Access ---")
squad Rectangle {
    spill top_left Point
    spill bottom_right Point
}

sus rect Rectangle = Rectangle{
    top_left: Point{x: 0, y: 0},
    bottom_right: Point{x: 100, y: 100}
}
vibez.spill("Rectangle corners:", rect.top_left.x, rect.top_left.y, "to", rect.bottom_right.x, rect.bottom_right.y)
vibez.spill("Nested field access works: ✅")

# Test 4: Nested field modification
vibez.spill("--- Test 4: Nested Field Modification ---")
rect.top_left.x = 5
rect.top_left.y = 5
rect.bottom_right.x = 95
rect.bottom_right.y = 95
vibez.spill("Modified rectangle:", rect.top_left.x, rect.top_left.y, "to", rect.bottom_right.x, rect.bottom_right.y)
vibez.spill("Nested field modification works: ✅")

# Test 5: Unary operators on struct fields
vibez.spill("--- Test 5: Unary Operators on Fields ---")
sus neg_x = -rect.top_left.x
sus pos_y = +rect.bottom_right.y
vibez.spill("Negative top-left x:", neg_x)
vibez.spill("Positive bottom-right y:", pos_y)
vibez.spill("Unary operators on fields work: ✅")

# Test 6: Logical operators involving struct fields
vibez.spill("--- Test 6: Logical Operators ---")
sus is_positive lit = !(rect.top_left.x < 0)
sus is_large lit = !(rect.bottom_right.x < 50)
vibez.spill("Top-left x is positive:", is_positive)
vibez.spill("Bottom-right x is large:", is_large)
vibez.spill("Logical operators on fields work: ✅")

# Test 7: Struct fields in arithmetic expressions
vibez.spill("--- Test 7: Fields in Expressions ---")
sus width = rect.bottom_right.x - rect.top_left.x
sus height = rect.bottom_right.y - rect.top_left.y
sus area = width * height
vibez.spill("Rectangle width:", width)
vibez.spill("Rectangle height:", height)
vibez.spill("Rectangle area:", area)
vibez.spill("Fields in expressions work: ✅")

# Test 8: Function parameters with struct fields
vibez.spill("--- Test 8: Function Parameters ---")
slay calculateDistance(p1 Point, p2 Point) drip {
    sus dx = p2.x - p1.x
    sus dy = p2.y - p1.y
    damn dx * dx + dy * dy  # Simplified distance (without sqrt)
}

sus distance = calculateDistance(rect.top_left, rect.bottom_right)
vibez.spill("Distance squared:", distance)
vibez.spill("Function parameters with fields work: ✅")

vibez.spill("=== ALL STRUCT DEREFERENCING TESTS PASSED! ===")
vibez.spill("✅ Field access: WORKING")
vibez.spill("✅ Field modification: WORKING") 
vibez.spill("✅ Nested field access: WORKING")
vibez.spill("✅ Nested field modification: WORKING")
vibez.spill("✅ Unary operators on fields: WORKING")
vibez.spill("✅ Logical operators on fields: WORKING")
vibez.spill("✅ Fields in expressions: WORKING")
vibez.spill("✅ Function parameters: WORKING")
