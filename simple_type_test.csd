squad Point {
    spill x meal
    spill y meal
}

slay distance(p1 Point, p2 Point) meal {
    sus dx meal = p1.x - p2.x
    sus dy meal = p1.y - p2.y
    damn math.sqrt(dx * dx + dy * dy)
}

sus point1 Point = Point{x: 3.0, y: 4.0}
sus point2 Point = Point{x: 0.0, y: 0.0}
sus dist meal = distance(point1, point2)

vibez.spillf("Distance: {}", dist)
