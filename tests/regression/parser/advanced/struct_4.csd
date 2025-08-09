// Struct test 4
squad Point4 {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p4 Point4 = Point4{x: 4, y: 8, id: 4}
p4.move(1, 1)
vibez.spill("Point 4:", p4.x, p4.y, p4.id)
