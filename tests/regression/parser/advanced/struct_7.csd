// Struct test 7
squad Point7 {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p7 Point7 = Point7{x: 7, y: 14, id: 7}
p7.move(1, 1)
vibez.spill("Point 7:", p7.x, p7.y, p7.id)
