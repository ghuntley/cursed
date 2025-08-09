// Struct test 9
squad Point9 {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p9 Point9 = Point9{x: 9, y: 18, id: 9}
p9.move(1, 1)
vibez.spill("Point 9:", p9.x, p9.y, p9.id)
