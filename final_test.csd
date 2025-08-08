# Final comprehensive test of all implemented features
vibez.spill("=== CURSED Compiler Feature Test ===")

# Variables and arithmetic
sus x drip = 10
sus y drip = 5
vibez.spill("Variables:", x, y)
vibez.spill("Arithmetic with precedence:", x + y * 2)

# Functions  
slay add(a drip, b drip) drip {
    damn a + b
}
vibez.spill("Function call:", add(3, 7))

# Arrays
sus arr []drip = [10, 20, 30]
vibez.spill("Array length:", len(arr))
vibez.spill("Array elements:", arr[0], arr[1], arr[2])

# Control structures
ready (x > y) {
    vibez.spill("If statement: x is greater")
}

sus i drip = 0
vibez.spill("While loop:")
bestie (i < 3) {
    vibez.spill("  Iteration", i)
    i = i + 1
}

# Structs
squad Point {
    spill x drip
    spill y drip
}
sus p Point = Point{x: 15, y: 25}
vibez.spill("Struct field access:", p.x, p.y)

# Pattern matching
sus value drip = 42
ready (value) {
    1 => vibez.spill("Pattern: one")
    42 => vibez.spill("Pattern: answer to everything")
    _ => vibez.spill("Pattern: other")
}

vibez.spill("✅ All core features working!")
