# Test 5: Interface definitions and method dispatch
collab Drawable {
    slay draw()
    slay area() drip
}

squad Circle {
    spill radius drip
    
    slay draw() {
        vibez.spill("Drawing a circle with radius", self.radius)
    }
    
    slay area() drip {
        damn 3.14 * self.radius * self.radius
    }
}

squad Rectangle {
    spill width drip
    spill height drip
    
    slay draw() {
        vibez.spill("Drawing a rectangle", self.width, "x", self.height)
    }
    
    slay area() drip {
        damn self.width * self.height
    }
}

vibez.spill("Interfaces test:")

sus circle Circle = Circle{radius: 5}
circle.draw()
sus circle_area drip = circle.area()
vibez.spill("Circle area:", circle_area)

sus rect Rectangle = Rectangle{width: 10, height: 6}
rect.draw()
sus rect_area drip = rect.area()
vibez.spill("Rectangle area:", rect_area)
