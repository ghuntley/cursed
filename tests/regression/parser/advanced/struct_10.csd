// Struct test 10
squad Point10 {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p10 Point10 = Point10{x: 10, y: 20, id: 10}
p10.move(1, 1)
vibez.spill("Point 10:", p10.x, p10.y, p10.id)
