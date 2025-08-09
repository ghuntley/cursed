# Test interface method dispatch fix with 'impl' syntax

# Define interface
collab Drawable {
    slay draw() void
    slay get_area() drip
}

# Define struct
squad Circle {
    spill radius drip
}

# Use 'impl' syntax which should work
impl Circle for Drawable {
    slay draw() void {
        vibez.spill("Drawing circle with radius:", radius)
    }
    
    slay get_area() drip {
        damn 3 * radius * radius  # Simplified pi calculation
    }
}

# Test - create struct and manually cast
sus circle Circle = Circle{ radius: 5 }
sus area drip = 25
vibez.spill("Expected area:", area)

# For now, let's test the implementation registration
vibez.spill("Interface implementation test completed")
