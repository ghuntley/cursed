// Struct test 18
squad Point18 {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p18 Point18 = Point18{x: 18, y: 36, id: 18}
p18.move(1, 1)
vibez.spill("Point 18:", p18.x, p18.y, p18.id)
