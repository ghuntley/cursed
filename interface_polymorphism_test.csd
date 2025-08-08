# Test interface definition and polymorphism
collab Drawable {
    slay draw()
    slay area() drip
}

# Circle implementing Drawable interface
squad Circle {
    spill radius drip
    
    slay draw() {
        vibez.spill("Drawing circle")
    }
    
    slay area() drip {
        damn 3 * radius * radius
    }
}

# Rectangle implementing Drawable interface  
squad Rectangle {
    spill width drip
    spill height drip
    
    slay draw() {
        vibez.spill("Drawing rectangle")
    }
    
    slay area() drip {
        damn width * height
    }
}

# Test basic struct method calls
sus circle Circle = Circle{radius: 5}
circle.draw()
vibez.spill("Circle area:", circle.area())

sus rect Rectangle = Rectangle{width: 4, height: 6}
rect.draw()
vibez.spill("Rectangle area:", rect.area())

# Test interface polymorphism (if supported)
# sus shape Drawable = Circle{radius: 3}
# shape.draw()
# vibez.spill("Polymorphic area:", shape.area())
