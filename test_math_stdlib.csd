// Test basic math stdlib functions
slay main() {
    vibez.spill("Testing math functions...")
    
    // Test basic math operations
    sus x drip = 4.0
    sus y drip = 9.0
    
    vibez.spill("sqrt(9) = %f", math.sqrt(y))
    vibez.spill("abs(-5) = %f", math.abs(-5.0))
    vibez.spill("max(4, 9) = %f", math.max(x, y))
    vibez.spill("min(4, 9) = %f", math.min(x, y))
    vibez.spill("pow(2, 3) = %f", math.pow(2.0, 3.0))
    vibez.spill("sin(0) = %f", math.sin(0.0))
    vibez.spill("cos(0) = %f", math.cos(0.0))
    vibez.spill("floor(3.7) = %f", math.floor(3.7))
    vibez.spill("ceil(3.2) = %f", math.ceil(3.2))
    
    vibez.spill("All math tests completed!")
}
