// Test struct definitions and operations
squad Point {
    spill x drip
    spill y drip
}

squad Rectangle {
    spill width drip
    spill height drip
    spill center Point
}

slay main() {
    // Basic struct creation
    sus p1 Point = Point{x: 10, y: 20}
    vibez.spill("Point 1:", p1.x, p1.y)
    
    // Nested struct
    sus center Point = Point{x: 5, y: 5}
    sus rect Rectangle = Rectangle{
        width: 100,
        height: 50,
        center: center
    }
    vibez.spill("Rectangle center:", rect.center.x, rect.center.y)
    
    // Struct field modification
    rect.center.x = 15
    vibez.spill("Modified center:", rect.center.x)
}

main()
