collab Drawable {
    slay draw()
}

squad Circle {
    slay draw() {
        vibez.spill("Circle")
    }
}

# Benchmark repeated method calls
sus c Circle = Circle{}
sus i drip = 0
bestie (i < 5) {
    c.draw()
    i = i + 1
}
