// Struct test 2
squad Point2 {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p2 Point2 = Point2{x: 2, y: 4, id: 2}
p2.move(1, 1)
vibez.spill("Point 2:", p2.x, p2.y, p2.id)
