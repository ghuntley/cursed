// Struct test 22
squad Point22 {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p22 Point22 = Point22{x: 22, y: 44, id: 22}
p22.move(1, 1)
vibez.spill("Point 22:", p22.x, p22.y, p22.id)
