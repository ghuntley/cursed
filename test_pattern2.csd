// Test tuple destructuring patterns
sus tuple (drip, tea) = (42, "hello")

match tuple {
    (a, b) => vibez.spill("Got:", a, b)
    _ => vibez.spill("No match")
}

// Test array patterns
sus array [drip] = [1, 2, 3, 4, 5]

match array {
    [first, second, ...rest] => {
        vibez.spill("First:", first)
        vibez.spill("Second:", second)
        vibez.spill("Rest length:", rest.length)
    }
    [single] => vibez.spill("Single element:", single)
    [] => vibez.spill("Empty array")
    _ => vibez.spill("Other")
}

// Test struct patterns  
squad Point {
    spill x drip
    spill y drip
}

sus point Point = Point{x: 10, y: 20}

match point {
    Point{x: 0, y: 0} => vibez.spill("Origin")
    Point{x: a, y: b} ready (a > 0 && b > 0) => vibez.spill("Positive quadrant")
    Point{x, y} => vibez.spill("Point at:", x, y)
    _ => vibez.spill("Not a point")
}
