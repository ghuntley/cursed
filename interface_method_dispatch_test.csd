# Test interface method dispatch issue
# Should call actual methods, not display strings

# Define interface
collab Drawable {
    slay draw() void
    slay get_area() drip
}

# Define struct implementing interface  
squad Circle {
    spill radius drip
}

# Implementation block
flex Circle => Drawable {
    slay draw() void {
        vibez.spill("Drawing circle with radius:", radius)
    }
    
    slay get_area() drip {
        damn 3 * radius * radius  # Simplified pi calculation
    }
}

# Test the interface dispatch
sus circle Circle = Circle{ radius: 5 }
sus drawable Drawable = circle

# These should execute methods, not show method names as strings
drawable.draw()                    # Should print "Drawing circle..." 
sus area drip = drawable.get_area() # Should calculate area
vibez.spill("Area:", area)
