// Struct and interface definitions
squad Point {
    spill x drip
    spill y drip
}

collab Drawable {
    slay draw()
    slay move(x drip, y drip)
}

squad Circle {
    spill center Point
    spill radius drip
    
    slay draw() {
        vibez.spill("Drawing circle")
    }
    
    slay move(x drip, y drip) {
        center.x = x
        center.y = y
    }
}
