// Test pattern matching compilation with switch/match statements
vibe Color = Red | Green | Blue | Custom(drip, drip, drip)

vibe Status = Success | Error(tea) | Pending

slay test_simple_patterns() {
    sus x drip = 5
    
    // Test basic switch pattern
    switch (x) {
        case 1: vibez.spill("One")
        case 2: vibez.spill("Two")
        case 5: vibez.spill("Five - matched!")
        default: vibez.spill("Other")
    }
    
    sus name tea = "Alice"
    switch (name) {
        case "Bob": vibez.spill("Hello Bob")
        case "Alice": vibez.spill("Hello Alice - matched!")
        case _: vibez.spill("Hello stranger")
    }
}

slay test_enum_patterns() {
    sus color Color = Red
    sus custom_color Color = Custom(255, 128, 0)
    
    // Test enum pattern matching
    match (color) {
        Red => vibez.spill("Red color")
        Green => vibez.spill("Green color")
        Blue => vibez.spill("Blue color")
        Custom(r, g, b) => vibez.spill("Custom color:", r, g, b)
    }
    
    match (custom_color) {
        Red => vibez.spill("Red")
        Green => vibez.spill("Green")
        Blue => vibez.spill("Blue")
        Custom(r, g, b) => {
            vibez.spill("Custom RGB:", r, g, b)
            ready (r > 200) {
                vibez.spill("High red component")
            }
        }
    }
}

slay test_tuple_patterns() {
    sus point (drip, drip) = (10, 20)
    sus pair (tea, drip) = ("value", 42)
    
    // Test tuple destructuring
    match (point) {
        (0, 0) => vibez.spill("Origin")
        (x, 0) => vibez.spill("On X-axis:", x)
        (0, y) => vibez.spill("On Y-axis:", y)
        (x, y) => vibez.spill("Point:", x, y)
    }
    
    match (pair) {
        ("test", n) => vibez.spill("Test with number:", n)
        ("value", n) => vibez.spill("Value with number:", n)
        (s, 0) => vibez.spill("String with zero:", s)
        (_, _) => vibez.spill("Other pair")
    }
}

slay test_guard_patterns() {
    sus numbers []drip = [1, 2, 3, 10, 15, 20]
    
    // Test pattern matching with guards
    bestie (i < len(numbers)) {
        sus num drip = numbers[i]
        
        match (num) {
            n if (n < 5) => vibez.spill("Small number:", n)
            n if (n >= 5 && n < 15) => vibez.spill("Medium number:", n)
            n if (n >= 15) => vibez.spill("Large number:", n)
            _ => vibez.spill("Unknown pattern")
        }
        
        i = i + 1
    }
}

slay test_array_patterns() {
    sus empty_array []drip = []
    sus single_array []drip = [42]
    sus multi_array []drip = [1, 2, 3, 4, 5]
    
    // Test array pattern matching
    match (empty_array) {
        [] => vibez.spill("Empty array")
        [x] => vibez.spill("Single element:", x)
        [first, ...rest] => vibez.spill("Multiple elements, first:", first)
    }
    
    match (single_array) {
        [] => vibez.spill("Empty")
        [x] => vibez.spill("Single:", x)
        [first, ...rest] => vibez.spill("Multiple, first:", first)
    }
    
    match (multi_array) {
        [] => vibez.spill("Empty")
        [x] => vibez.spill("Single:", x)
        [first, second, ...rest] => {
            vibez.spill("First two:", first, second)
            vibez.spill("Rest length:", len(rest))
        }
    }
}

slay test_range_patterns() {
    sus score drip = 85
    
    // Test range patterns
    match (score) {
        0..49 => vibez.spill("Fail")
        50..69 => vibez.spill("Pass")
        70..84 => vibez.spill("Good")
        85..94 => vibez.spill("Great - matched!")
        95..100 => vibez.spill("Excellent")
        _ => vibez.spill("Invalid score")
    }
}

slay main() {
    vibez.spill("=== Testing Simple Patterns ===")
    test_simple_patterns()
    
    vibez.spill("\n=== Testing Enum Patterns ===")
    test_enum_patterns()
    
    vibez.spill("\n=== Testing Tuple Patterns ===")
    test_tuple_patterns()
    
    vibez.spill("\n=== Testing Guard Patterns ===")
    test_guard_patterns()
    
    vibez.spill("\n=== Testing Array Patterns ===")
    test_array_patterns()
    
    vibez.spill("\n=== Testing Range Patterns ===")
    test_range_patterns()
    
    vibez.spill("\n=== Pattern Matching Tests Complete ===")
}
