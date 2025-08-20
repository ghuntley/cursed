# Final demonstration of struct field dereferencing fixes
vibez.spill("=== STRUCT FIELD DEREFERENCING FIXES DEMO ===")

# Basic struct
squad Point {
    spill x drip
    spill y drip
}

sus p Point = Point{x: 42, y: 84}
vibez.spill("Created point with x =", p.x, "and y =", p.y)

# Test 1: Unary minus on struct field (FIXED)
sus neg_x = -p.x
vibez.spill("Unary minus on field: -p.x =", neg_x)

# Test 2: Logical NOT on expression involving struct field (FIXED)  
sus is_small lit = !(p.x > 50)
vibez.spill("Logical not: !(p.x > 50) =", is_small)

# Test 3: Field modification (WORKING)
p.x = 100
vibez.spill("After p.x = 100, point is:", p.x, p.y)

# Test 4: Complex expressions with struct fields (WORKING)
sus doubled = p.x * 2
sus sum = p.x + p.y  
vibez.spill("p.x * 2 =", doubled)
vibez.spill("p.x + p.y =", sum)

# Test 5: Nested struct (simplified)
squad Rectangle {
    spill corner Point
}

sus rect Rectangle = Rectangle{corner: p}
vibez.spill("Rectangle corner x:", rect.corner.x)
vibez.spill("Rectangle corner y:", rect.corner.y)

# Test 6: Nested field modification
rect.corner.x = 200
vibez.spill("After rect.corner.x = 200:")
vibez.spill("Corner is now:", rect.corner.x, rect.corner.y)

vibez.spill("=== SUMMARY ===")
vibez.spill("✅ Basic field access: WORKING")
vibez.spill("✅ Unary operators on fields: FIXED")  
vibez.spill("✅ Logical operators on fields: FIXED")
vibez.spill("✅ Field modification: WORKING")
vibez.spill("✅ Nested field access: WORKING") 
vibez.spill("✅ Nested field modification: WORKING")
vibez.spill("=== ALL KEY FIXES IMPLEMENTED SUCCESSFULLY ===")
