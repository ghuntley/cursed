collab Drawable { slay draw(); slay area() drip; }
squad Circle { 
    spill radius drip
    slay draw() { vibez.spill("Drawing circle with radius", radius) }
    slay area() drip { damn 3 * radius * radius }
}
squad Rectangle {
    spill width drip
    spill height drip
    slay draw() { vibez.spill("Drawing rectangle", width, "x", height) }
    slay area() drip { damn width * height }
}
sus c Circle = Circle{radius: 5}
sus r Rectangle = Rectangle{width: 4, height: 6}
c.draw()
r.draw()
vibez.spill("Circle area:", c.area())
vibez.spill("Rectangle area:", r.area())
