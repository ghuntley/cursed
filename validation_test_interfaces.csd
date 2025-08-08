// Test interface definitions and dispatch
collab Drawable {
    slay draw()
    slay area() drip
}

squad Circle {
    spill radius drip
    
    slay draw() {
        vibez.spill("Drawing circle with radius", radius)
    }
    
    slay area() drip {
        damn 3.14159 * radius * radius
    }
}

squad Square {
    spill side drip
    
    slay draw() {
        vibez.spill("Drawing square with side", side)
    }
    
    slay area() drip {
        damn side * side
    }
}

slay main() {
    sus circle Circle = Circle{radius: 5}
    sus square Square = Square{side: 4}
    
    circle.draw()
    vibez.spill("Circle area:", circle.area())
    
    square.draw()
    vibez.spill("Square area:", square.area())
}

main()
