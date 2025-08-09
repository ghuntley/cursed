// Struct test 15
squad Point15 {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p15 Point15 = Point15{x: 15, y: 30, id: 15}
p15.move(1, 1)
vibez.spill("Point 15:", p15.x, p15.y, p15.id)
