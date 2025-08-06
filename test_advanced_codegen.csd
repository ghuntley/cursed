squad Point {
    spill x normie
    spill y normie
}

collab Drawable {
    slay draw()
}

impl Point for Drawable {
    slay draw() {
        vibez.spill("Drawing point")
    }
}

slay add(a normie, b normie) normie {
    damn a + b
}

slay main() normie {
    sus p Point = { x: 10, y: 20 }
    sus result normie = add(5, 3)
    
    vibez.spill("Advanced features test")
    p.draw()
    
    damn result
}
