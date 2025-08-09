// Struct test 24
squad Point24 {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p24 Point24 = Point24{x: 24, y: 48, id: 24}
p24.move(1, 1)
vibez.spill("Point 24:", p24.x, p24.y, p24.id)
