// Struct test 14
squad Point14 {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p14 Point14 = Point14{x: 14, y: 28, id: 14}
p14.move(1, 1)
vibez.spill("Point 14:", p14.x, p14.y, p14.id)
