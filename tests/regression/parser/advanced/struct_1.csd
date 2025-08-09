// Struct test 1
squad Point1 {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p1 Point1 = Point1{x: 1, y: 2, id: 1}
p1.move(1, 1)
vibez.spill("Point 1:", p1.x, p1.y, p1.id)
