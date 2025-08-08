// Test struct field access

squad Point {
    spill x drip
    spill y drip
}

sus p Point = Point{x: 10, y: 20}
sus sum drip = p.x + p.y
vibez.spill("Sum of x and y:", sum)
