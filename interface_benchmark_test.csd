collab Shape {
    slay draw()
    slay area() drip
}

squad Rectangle {
    spill width drip
    spill height drip

    slay draw() {
        vibez.spill("Drawing rectangle", width, "x", height)
    }

    slay area() drip {
        damn width * height
    }
}

squad Circle {
    spill radius drip

    slay draw() {
        vibez.spill("Drawing circle with radius", radius)
    }

    slay area() drip {
        damn 3 * radius * radius  // simplified pi * r^2
    }
}

sus rect Rectangle = Rectangle{width: 10, height: 5}
sus circle Circle = Circle{radius: 3}

rect.draw()
vibez.spill("Rectangle area:", rect.area())

circle.draw()
vibez.spill("Circle area:", circle.area())
