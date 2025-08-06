# Test 6: Interface definitions (collab)
collab Drawable {
    slay draw()
}

squad Circle {
    spill radius drip
}

slay (c Circle) draw() {
    vibez.spill("Drawing a circle with radius: " + c.radius)
}

sus c Circle = Circle{radius: 5}
c.draw()
