# Direct interface method call test
# Define interface
collab Drawable {
    slay draw() void
}

# Define struct with method
squad Circle {
    spill radius drip
    
    slay draw() void {
        vibez.spill("Drawing circle with radius:", radius)
    }
}

# Test: create struct and call method directly (this should work)
sus circle Circle = Circle{ radius: 5 }
circle.draw()  # This should call the struct method

vibez.spill("Direct method call test completed")
