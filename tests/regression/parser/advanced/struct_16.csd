// Struct test 16
squad Point16 {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p16 Point16 = Point16{x: 16, y: 32, id: 16}
p16.move(1, 1)
vibez.spill("Point 16:", p16.x, p16.y, p16.id)
