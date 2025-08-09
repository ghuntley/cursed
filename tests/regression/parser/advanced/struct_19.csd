// Struct test 19
squad Point19 {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p19 Point19 = Point19{x: 19, y: 38, id: 19}
p19.move(1, 1)
vibez.spill("Point 19:", p19.x, p19.y, p19.id)
