// Struct test 13
squad Point13 {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p13 Point13 = Point13{x: 13, y: 26, id: 13}
p13.move(1, 1)
vibez.spill("Point 13:", p13.x, p13.y, p13.id)
