# Test the struct field dereferencing fixes we implemented
vibez.spill("=== Fixed Struct Field Access ===")

# Basic struct definition
squad Point {
    spill x drip
    spill y drip
}

# Create a struct
sus p Point = Point{x: 100, y: 200}
vibez.spill("Point:", p.x, p.y)

# These should now work due to our fixes:

# 1. Unary operators on struct fields
sus neg_x = -p.x
vibez.spill("Negative x:", neg_x)

# 2. Logical not on comparisons involving struct fields
sus x_is_small lit = !(p.x > 150)
vibez.spill("X is small?", x_is_small)

# 3. Basic arithmetic with struct fields
sus sum = p.x + p.y
vibez.spill("Sum:", sum)

# 4. Field modification
p.x = 300
vibez.spill("Modified point:", p.x, p.y)

# 5. Nested struct field access
squad Rectangle {
    spill corner Point
    spill width drip  
}

sus rect Rectangle = Rectangle{corner: p, width: 50}
vibez.spill("Rectangle corner:", rect.corner.x, rect.corner.y)

# 6. Nested field modification
rect.corner.x = 500
vibez.spill("Modified rectangle corner:", rect.corner.x, rect.corner.y)

vibez.spill("=== Field dereferencing works correctly! ===")
