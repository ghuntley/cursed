# Test 3: Structs and Interfaces
# Struct definition
squad Point {
    spill x normie
    spill y normie
}

# Create struct instance
sus p drip = Point{x: 10, y: 20}
vibez.spill("Point created:")
vibez.spill(p.x)
vibez.spill(p.y)

# Interface definition
collab Drawable {
    slay draw()
}

# Struct that implements interface
squad Circle {
    spill radius normie
}

slay (c Circle) draw() {
    vibez.spill("Drawing circle with radius:")
    vibez.spill(c.radius)
}

sus circle drip = Circle{radius: 5}
circle.draw()
