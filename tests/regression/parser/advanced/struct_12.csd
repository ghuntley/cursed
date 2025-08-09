// Struct test 12
squad Point12 {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p12 Point12 = Point12{x: 12, y: 24, id: 12}
p12.move(1, 1)
vibez.spill("Point 12:", p12.x, p12.y, p12.id)
