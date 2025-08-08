# Comprehensive struct functionality test

vibez.spill("=== CURSED Struct Implementation Test ===")

# Basic struct definition
squad Point {
    spill x drip
    spill y drip
}

# Nested struct definition
squad Rectangle {
    spill top_left Point
    spill bottom_right Point
    spill area drip
}

# Create basic struct
sus p1 Point = Point{x: 10, y: 20}
vibez.spill("Point p1:", p1.x, p1.y)

# Create second point
sus p2 Point = Point{x: 30, y: 40}
vibez.spill("Point p2:", p2.x, p2.y)

# Create nested struct using variable references
sus rect Rectangle = Rectangle{top_left: p1, bottom_right: p2, area: 0}
vibez.spill("Rectangle created")

# Access nested fields
vibez.spill("Top-left corner:", rect.top_left.x, rect.top_left.y)
vibez.spill("Bottom-right corner:", rect.bottom_right.x, rect.bottom_right.y)

# Field modification
rect.area = 100
vibez.spill("Rectangle area:", rect.area)

# Struct in expressions
sus width drip = rect.bottom_right.x - rect.top_left.x
sus height drip = rect.bottom_right.y - rect.top_left.y
vibez.spill("Calculated width:", width, "height:", height)

# Function with struct parameters
slay calculateArea(r Rectangle) drip {
    sus w drip = r.bottom_right.x - r.top_left.x
    sus h drip = r.bottom_right.y - r.top_left.y
    damn w * h
}

sus calculated_area drip = calculateArea(rect)
vibez.spill("Function calculated area:", calculated_area)

vibez.spill("=== All struct tests passed! ===")
