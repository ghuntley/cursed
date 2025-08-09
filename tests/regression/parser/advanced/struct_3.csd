// Struct test 3
squad Point3 {
    spill x drip
    spill y drip
    spill id drip
    
    slay move(dx drip, dy drip) {
        x = x + dx
        y = y + dy
    }
}

sus p3 Point3 = Point3{x: 3, y: 6, id: 3}
p3.move(1, 1)
vibez.spill("Point 3:", p3.x, p3.y, p3.id)
