# Simple JIT Test Program
vibez.spill("Testing CURSED JIT functionality")

# Test basic function with parameters
slay greet(name tea) {
    vibez.spill("Hello,", name)
    damn "greeting complete"
}

# Test array operations
sus numbers drip = [1, 2, 3, 4, 5]
vibez.spill("Array created with elements")

# Test tuple operations 
sus coordinates drip = (10, 20)
vibez.spill("Tuple created:", coordinates)

# Test struct
squad Point {
    spill x normie
    spill y normie
}

sus p drip = Point{ x: 100, y: 200 }
vibez.spill("Point created:", p.x, p.y)

# Call test function
result := greet("CURSED")
vibez.spill("Function result:", result)

vibez.spill("JIT test completed successfully!")
