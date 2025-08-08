# Test simple field access in struct methods
squad Circle {
    spill radius drip
    
    slay show_radius() {
        vibez.spill("Radius is:", radius)
    }
    
    slay show_self_radius() {
        vibez.spill("Self radius is:", self.radius)
    }
}

sus c Circle = Circle{radius: 42}
vibez.spill("Direct field access:", c.radius)
c.show_radius()
c.show_self_radius()
