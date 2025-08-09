// Comprehensive test for advanced pattern matching features
yeet "testz"

// Test enum for exhaustiveness checking
enum Color {
    Red,
    Green, 
    Blue,
    Custom
}

enum ResponseCode {
    Success,
    NotFound,
    ServerError,
    Unauthorized
}

slay test_advanced_pattern_matching() lit {
    test_start("Advanced Pattern Matching")
    
    // Test 1: Range patterns with different syntax
    vibez.spill("=== Range Pattern Tests ===")
    
    sus score drip = 85
    ready (score) {
        0..59 => vibez.spill("F grade")
        60..69 => vibez.spill("D grade")  
        70..79 => vibez.spill("C grade")
        80..89 => vibez.spill("B grade")
        90..100 => vibez.spill("A grade")
        _ => vibez.spill("Invalid score")
    }
    
    // Test 2: Guard patterns with complex conditions
    vibez.spill("=== Guard Pattern Tests ===")
    
    sus data = [1, 2, 3, 4, 5]
    ready (data) {
        arr when len(arr) > 0 && arr[0] > 0 => vibez.spill("Non-empty array with positive first element")
        arr when len(arr) == 0 => vibez.spill("Empty array")
        _ => vibez.spill("Other array")
    }
    
    // Test 3: OR patterns with ranges and guards
    vibez.spill("=== OR Pattern Tests ===")
    
    sus value drip = 15
    ready (value) {
        0..10 | 20..30 | 40..50 => vibez.spill("In specific ranges")
        n when n > 100 => vibez.spill("Large number")
        _ => vibez.spill("Other value")
    }
    
    // Test 4: Exhaustive enum matching
    vibez.spill("=== Exhaustive Enum Tests ===")
    
    sus color = Color.Red
    ready (color) {
        Color.Red => vibez.spill("Red color")
        Color.Green => vibez.spill("Green color") 
        Color.Blue => vibez.spill("Blue color")
        Color.Custom => vibez.spill("Custom color")
        // This should be detected as exhaustive
    }
    
    // Test 5: Non-exhaustive pattern (should generate warning)
    vibez.spill("=== Non-Exhaustive Test (should warn) ===")
    
    sus response = ResponseCode.Success
    ready (response) {
        ResponseCode.Success => vibez.spill("Success response")
        ResponseCode.NotFound => vibez.spill("Not found")
        // Missing ServerError and Unauthorized - should warn
    }
    
    // Test 6: Complex nested patterns with guards
    vibez.spill("=== Complex Nested Patterns ===")
    
    squad Point {
        spill x drip
        spill y drip
    }
    
    sus point = Point{ x: 3, y: 4 }
    ready (point) {
        Point{x: 0, y: 0} => vibez.spill("Origin point")
        Point{x: a, y: b} when a == b => vibez.spill("Diagonal point")
        Point{x: a, y: b} when a > 0 && b > 0 => vibez.spill("First quadrant")
        Point{x: a, y: b} when a * a + b * b <= 25 => vibez.spill("Within unit circle") 
        _ => vibez.spill("Other point")
    }
    
    // Test 7: Array patterns with rest elements
    vibez.spill("=== Array Pattern Tests ===")
    
    sus numbers = [1, 2, 3, 4, 5]
    ready (numbers) {
        [] => vibez.spill("Empty array")
        [x] => vibez.spill("Single element: " + x)
        [first, second, ...rest] when len(rest) > 0 => vibez.spill("Multiple elements with rest")
        [a, b] => vibez.spill("Exactly two elements")
        _ => vibez.spill("Other array pattern")
    }
    
    // Test 8: String range patterns
    vibez.spill("=== String Range Tests ===")
    
    sus letter tea = "m"
    ready (letter) {
        "a".."m" => vibez.spill("First half of alphabet")
        "n".."z" => vibez.spill("Second half of alphabet")
        "A".."Z" => vibez.spill("Uppercase letter")
        "0".."9" => vibez.spill("Digit character")
        _ => vibez.spill("Other character")
    }
    
    print_test_summary()
    damn based
}

test_advanced_pattern_matching()
