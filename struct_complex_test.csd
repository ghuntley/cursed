squad Point {
    spill x drip
    spill y drip
}

squad Rectangle {
    spill width drip  
    spill height drip
    spill top_left Point
}

sus p1 Point = Point{x: 10, y: 20}
sus p2 Point = Point{x: 30, y: 40}

vibez.spill("Point 1:", p1.x, p1.y)
vibez.spill("Point 2:", p2.x, p2.y)

p1.x = 15
vibez.spill("Updated Point 1:", p1.x, p1.y)

sus rect Rectangle = Rectangle{width: 100, height: 50, top_left: p1}
vibez.spill("Rectangle:", rect.width, rect.height)
