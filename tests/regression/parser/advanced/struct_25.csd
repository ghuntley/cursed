// Struct test 25
squad Point25 {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p25 Point25 = Point25{x: 25, y: 50, id: 25}
p25.move(1, 1)
vibez.spill("Point 25:", p25.x, p25.y, p25.id)
