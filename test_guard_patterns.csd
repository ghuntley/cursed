// Test guard patterns (when conditions)
yeet "testz"

slay test_guard_patterns() lit {
    test_start("Guard Patterns")
    
    // Test guards with literals
    sus x drip = 5
    ready (x) {
        n when n > 0 && n < 10 => vibez.spill("Small positive number")
        n when n >= 10 => vibez.spill("Large number")
        n when n <= 0 => vibez.spill("Zero or negative")
        _ => vibez.spill("Fallback")
    }
    
    // Test guards with destructuring
    sus point = [3, 4]
    ready (point) {
        [x, y] when x * x + y * y == 25 => vibez.spill("Point on circle of radius 5")
        [x, y] when x == y => vibez.spill("Point on diagonal")
        [x, y] when x > 0 && y > 0 => vibez.spill("Point in first quadrant")
        _ => vibez.spill("Other point")
    }
    
    // Test guards with struct patterns
    squad Person {
        spill name tea
        spill age drip
    }
    
    sus person = Person{ name: "Alice", age: 25 }
    ready (person) {
        Person{name: n, age: a} when a >= 18 => vibez.spill("Adult: " + n)
        Person{name: n, age: a} when a < 18 => vibez.spill("Minor: " + n)
        _ => vibez.spill("Unknown person")
    }
    
    print_test_summary()
    damn based
}

test_guard_patterns()
