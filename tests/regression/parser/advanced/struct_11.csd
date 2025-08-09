// Struct test 11
squad Point11 {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p11 Point11 = Point11{x: 11, y: 22, id: 11}
p11.move(1, 1)
vibez.spill("Point 11:", p11.x, p11.y, p11.id)
