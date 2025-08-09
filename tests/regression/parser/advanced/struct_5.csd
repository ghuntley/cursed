// Struct test 5
squad Point5 {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p5 Point5 = Point5{x: 5, y: 10, id: 5}
p5.move(1, 1)
vibez.spill("Point 5:", p5.x, p5.y, p5.id)
