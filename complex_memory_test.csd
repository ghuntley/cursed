fr fr Complex memory test with AST-heavy operations
yeet "testz"

squad Point {
    spill x meal
    spill y meal
}

collab Drawable {
    slay draw() tea
    slay area() meal
}

squad Circle {
    spill radius meal
    spill center Point
}

flex Circle => Drawable {
    slay draw() tea {
        damn "Drawing circle with radius " + radius
    }
    
    slay area() meal {
        damn 3.14159 * radius * radius
    }
}

slay complex_memory_test() {
    test_start("Complex Memory Test")
    
    fr fr Create many objects with complex structures
    sus circles []Circle = []
    
    bestie i := 0; i < 50; i = i + 1 {
        sus center Point = Point{x: i * 1.0, y: i * 2.0}
        sus circle Circle = Circle{radius: i * 0.5, center: center}
        circles.push(circle)
    }
    
    fr fr Pattern matching with complex data
    sus total_area meal = 0.0
    bestie circle in circles {
        sus area meal = match circle.radius {
            x if x > 10.0 => circle.area() * 2.0,
            x if x > 5.0 => circle.area() * 1.5,
            _ => circle.area()
        }
        total_area = total_area + area
    }
    
    assert_true(total_area > 0.0)
    assert_eq_int(circles.len(), 50)
    
    print_test_summary()
}

complex_memory_test()
