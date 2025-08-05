fr fr Test advanced codegen features
sus x drip = 42
sus y meal = 3.14
sus message tea = "Hello CURSED!"

vibez.spill("Testing basic functionality:")
vibez.spill(message)
vibez.spillf("x = {}, y = {}", x, y)

fr fr Test arithmetic
sus sum drip = x + 10
vibez.spillf("Sum: {}", sum)

fr fr Test struct
squad Point {
    spill px drip
    spill py drip
}

sus p Point = Point{px: 10, py: 20}
vibez.spillf("Point: ({}, {})", p.px, p.py)
