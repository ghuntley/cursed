// Struct test 6
squad Point6 {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p6 Point6 = Point6{x: 6, y: 12, id: 6}
p6.move(1, 1)
vibez.spill("Point 6:", p6.x, p6.y, p6.id)
