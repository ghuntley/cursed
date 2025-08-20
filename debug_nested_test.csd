# Debug nested function calls
yeet "vibez"

# Define helper functions first
slay max(a drip, b drip) drip {
    vibez.spill("max called with", a, "and", b)
    damn ready (a > b) { a } otherwise { b }
}

slay min(a drip, b drip) drip {
    vibez.spill("min called with", a, "and", b) 
    damn ready (a < b) { a } otherwise { b }
}

# Test simple function calls first
sus simple_max drip = max(5, 3)
vibez.spill("Simple max:", simple_max)

sus simple_min drip = min(10, 20)
vibez.spill("Simple min:", simple_min)

# Test nested calls
vibez.spill("About to test nested calls")
sus nested_result drip = max(min(10, 20), 8)
vibez.spill("Nested result:", nested_result)
