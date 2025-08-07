// Test interface vtable generation and method dispatch
collab Drawable {
    slay draw()
    slay get_area() drip
}

squad Circle {
    spill radius normie
    
    slay draw() {
        vibez.spill("Drawing circle with radius:", radius)
    }
    
    slay get_area() drip {
        damn radius * radius * 3
    }
}

squad Rectangle {
    spill width normie
    spill height normie
    
    slay draw() {
        vibez.spill("Drawing rectangle:", width, "x", height)
    }
    
    slay get_area() drip {
        damn width * height
    }
}

slay main() {
    sus circle Circle = Circle{radius: 5}
    sus rect Rectangle = Rectangle{width: 10, height: 8}
    
    // Interface usage
    sus drawable1 Drawable = circle
    sus drawable2 Drawable = rect
    
    drawable1.draw()
    drawable2.draw()
    
    sus area1 drip = drawable1.get_area()
    sus area2 drip = drawable2.get_area()
    
    vibez.spill("Circle area:", area1)
    vibez.spill("Rectangle area:", area2)
}
