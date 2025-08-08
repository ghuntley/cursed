# Test interface definition and method calls
collab Drawable {
    slay draw()
    slay area() normie
}

squad Circle {
    spill radius normie
    
    slay draw() {
        vibez.spill("Drawing circle with radius", this.radius)
    }
    
    slay area() normie {
        damn 3.14159 * this.radius * this.radius
    }
}

squad Rectangle {
    spill width normie
    spill height normie
    
    slay draw() {
        vibez.spill("Drawing rectangle", this.width, "x", this.height)
    }
    
    slay area() normie {
        damn this.width * this.height
    }
}

# Test basic interface usage
sus circle Circle = Circle{radius: 5.0}
sus rect Rectangle = Rectangle{width: 4.0, height: 3.0}

# Test method calls directly on structs
circle.draw()
vibez.spill("Circle area:", circle.area())

rect.draw()
vibez.spill("Rectangle area:", rect.area())

# Test polymorphic usage
slay test_drawable(d Drawable) {
    d.draw()
    vibez.spill("Area:", d.area())
}

test_drawable(circle)
test_drawable(rect)
