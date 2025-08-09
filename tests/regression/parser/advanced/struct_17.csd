// Struct test 17
squad Point17 {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p17 Point17 = Point17{x: 17, y: 34, id: 17}
p17.move(1, 1)
vibez.spill("Point 17:", p17.x, p17.y, p17.id)
