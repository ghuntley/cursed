// Struct test 21
squad Point21 {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p21 Point21 = Point21{x: 21, y: 42, id: 21}
p21.move(1, 1)
vibez.spill("Point 21:", p21.x, p21.y, p21.id)
