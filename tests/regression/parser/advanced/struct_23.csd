// Struct test 23
squad Point23 {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p23 Point23 = Point23{x: 23, y: 46, id: 23}
p23.move(1, 1)
vibez.spill("Point 23:", p23.x, p23.y, p23.id)
