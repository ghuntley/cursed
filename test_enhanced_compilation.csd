yeet "mathz"
yeet "stringz"
yeet "arrayz"

// Test comprehensive CURSED features for LLVM compilation
squad Point {
    spill x drip
    spill y drip
}

collab Drawable {
    slay draw()
}

squad Circle {
    spill radius drip
    
    slay draw() {
        vibez.spill("Drawing circle with radius:", self.radius)
    }
}

slay add(a drip, b drip) drip {
    damn a + b
}

slay factorial(n drip) drip {
    ready (n <= 1) {
        damn 1
    }
    damn n * factorial(n - 1)
}

slay test_pattern_matching(value drip) tea {
    ready (value) {
        0 => damn "zero"
        1..10 => damn "small"
        11..100 => damn "medium"
        _ => damn "large"
    }
}

slay test_error_handling() (drip, tea) {
    ready (based) {
        damn 42, ""
    }
    damn 0, "error occurred"
}

slay test_defer_statements() {
    defer {
        vibez.spill("Cleanup executed")
    }
    
    vibez.spill("Main function executing")
}

slay test_goroutines() {
    stan {
        vibez.spill("Goroutine executing")
    }
    
    vibez.spill("Main thread continues")
}

slay main() {
    // Test basic variables and expressions
    sus x drip = 42
    sus y drip = add(x, 8)
    vibez.spill("Addition result:", y)
    
    // Test arrays
    sus numbers []drip = [1, 2, 3, 4, 5]
    sus total drip = 0
    sus i drip = 0
    bestie (i < len(numbers)) {
        total = total + numbers[i]
        i = i + 1
    }
    vibez.spill("Array sum:", total)
    
    // Test structs
    sus point Point = Point{x: 10, y: 20}
    vibez.spill("Point:", point.x, point.y)
    
    // Test interfaces
    sus circle Circle = Circle{radius: 5}
    circle.draw()
    
    // Test mathematical functions
    sus fact_result drip = factorial(5)
    vibez.spill("Factorial of 5:", fact_result)
    
    // Test pattern matching
    sus pattern_result tea = test_pattern_matching(50)
    vibez.spill("Pattern result:", pattern_result)
    
    // Test error handling
    sus val, err = test_error_handling()
    ready (err == "") {
        vibez.spill("Success:", val)
    } otherwise {
        vibez.spill("Error:", err)
    }
    
    // Test defer statements
    test_defer_statements()
    
    // Test concurrency
    test_goroutines()
    
    vibez.spill("All tests completed!")
}
