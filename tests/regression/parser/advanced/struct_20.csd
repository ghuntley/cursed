// Struct test 20
squad Point20 {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p20 Point20 = Point20{x: 20, y: 40, id: 20}
p20.move(1, 1)
vibez.spill("Point 20:", p20.x, p20.y, p20.id)
