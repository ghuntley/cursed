# Test comprehensive struct field access and dereferencing
vibez.spill("=== Struct Field Access Tests ===")

# Basic struct definition
squad Point {
    spill x drip
    spill y drip
}

# Create a struct
sus p Point = Point{x: 10, y: 20}
vibez.spill("Original point:", p.x, p.y)

# Test direct field access
sus point_x = p.x
sus point_y = p.y
vibez.spill("Copied fields:", point_x, point_y)

# Test field modification
p.x = 25
p.y = 35
vibez.spill("Modified point:", p.x, p.y)

# Test nested struct field access
squad Rectangle {
    spill top_left Point
    spill width drip
    spill height drip
}

sus rect Rectangle = Rectangle{top_left: p, width: 100, height: 50}
vibez.spill("Rectangle top-left:", rect.top_left.x, rect.top_left.y)
vibez.spill("Rectangle dimensions:", rect.width, rect.height)

# Test nested field access and modification
rect.top_left.x = 5
rect.top_left.y = 15
vibez.spill("Modified rectangle top-left:", rect.top_left.x, rect.top_left.y)

# Test using field values in expressions
sus area = rect.width * rect.height
vibez.spill("Rectangle area:", area)

# Test field access in conditionals
ready (rect.top_left.x < 10) {
    vibez.spill("Top-left x is less than 10")
} otherwise {
    vibez.spill("Top-left x is 10 or greater")
}

vibez.spill("=== All struct field access tests passed! ===")
