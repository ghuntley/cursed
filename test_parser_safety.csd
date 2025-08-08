// Complex CURSED program to test parser safety
yeet "testz"
yeet "mathz"
yeet "stringz"

// Deep nesting to test expression parsing
slay complex_nesting(depth drip) drip {
    ready (depth <= 0) {
        damn 1
    }
    
    sus result drip = complex_nesting(depth - 1)
    damn result * 2 + depth
}

// Complex structs and interfaces
squad Point {
    spill x drip
    spill y drip
}

collab Drawable {
    slay draw()
    slay area() drip
}

squad Circle {
    spill radius drip
    spill center Point
    
    slay draw() {
        vibez.spill("Drawing circle with radius:", radius)
    }
    
    slay area() drip {
        // Using nested expressions
        damn (radius * radius) * 3.14159
    }
}

// Complex function with multiple parameters and return types
slay process_data(numbers []drip, threshold drip) (drip, tea) {
    sus count drip = 0
    sus total drip = 0
    sus i drip = 0
    
    bestie (i < len(numbers)) {
        ready (numbers[i] > threshold) {
            count = count + 1
            total = total + numbers[i]
        }
        i = i + 1
    }
    
    ready (count == 0) {
        damn 0, "no valid numbers"
    }
    
    damn total / count, ""
}

// Test error handling
slay test_errors() {
    sus data []drip = [1, 5, 10, 15, 2]
    sus avg, err = process_data(data, 3)
    
    ready (err != "") {
        vibez.spill("Error:", err)
    } otherwise {
        vibez.spill("Average:", avg)
    }
}

// Test concurrency
slay test_concurrency() {
    stan {
        vibez.spill("Goroutine 1")
        sus i drip = 0
        bestie (i < 3) {
            vibez.spill("Iteration:", i)
            i = i + 1
        }
    }
    
    stan {
        vibez.spill("Goroutine 2")
        sus c Circle = Circle{radius: 5.0, center: Point{x: 0, y: 0}}
        c.draw()
        vibez.spill("Area:", c.area())
    }
}

// Test main execution
sus result drip = complex_nesting(5)
vibez.spill("Nesting result:", result)

test_errors()
test_concurrency()

vibez.spill("Parser safety test complete!")
