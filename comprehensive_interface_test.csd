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
        damn 3 * radius * radius  // simplified pi * r^2
    }
}

sus c Circle = Circle{radius: 5}
c.draw()
vibez.spill("Area:", c.area())
