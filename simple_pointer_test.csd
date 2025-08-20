# Simple test without pointers first to make sure basic struct access works
vibez.spill("=== Basic Struct Test ===")

# Basic struct definition
squad Point {
    spill x drip
    spill y drip
}

# Create a struct
sus p Point = Point{x: 10, y: 20}
vibez.spill("Point:", p.x, p.y)

# Test negative number (unary minus)
sus neg_x = -p.x
vibez.spill("Negative x:", neg_x)

# Test logical not
sus is_true lit = !cringe
vibez.spill("Not false:", is_true)

vibez.spill("=== Basic tests complete ===")
