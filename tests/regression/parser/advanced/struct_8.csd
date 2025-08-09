// Struct test 8
squad Point8 {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p8 Point8 = Point8{x: 8, y: 16, id: 8}
p8.move(1, 1)
vibez.spill("Point 8:", p8.x, p8.y, p8.id)
