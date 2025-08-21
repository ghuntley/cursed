// Comprehensive Pattern Matching Test for CURSED
// Tests all pattern matching features including vibe_check/mood syntax

sus test_value drip = 42

// Test 1: Basic literal patterns with vibe_check
vibe_check test_value {
    mood 42: 
        vibez.spill("✅ Literal pattern matched!")
    mood 0, 1, 2:
        vibez.spill("Multiple literal patterns")
    basic:
        vibez.spill("Default case")
}

// Test 2: Variable binding patterns
sus x drip = 100
vibe_check x {
    mood value:
        vibez.spill("Variable bound to:", value)
}

// Test 3: Range patterns (when implemented in parser)
sus score drip = 85
vibe_check score {
    mood 90..100:
        vibez.spill("Excellent score!")
    mood 80..89:
        vibez.spill("Good score!")  
    mood 0..79:
        vibez.spill("Needs improvement")
}

// Test 4: Guard patterns (when implemented)
sus num drip = 15
vibe_check num {
    mood n when n > 10:
        vibez.spill("Number is greater than 10:", n)
    mood n when n < 5:
        vibez.spill("Number is less than 5:", n)
    basic:
        vibez.spill("Number is between 5 and 10")
}

// Test 5: Or patterns (multiple alternatives)
sus letter tea = "a"
vibe_check letter {
    mood "a" | "e" | "i" | "o" | "u":
        vibez.spill("Vowel found!")
    basic:
        vibez.spill("Consonant")
}

// Test 6: Wildcard pattern
sus anything drip = 999
vibe_check anything {
    mood _:
        vibez.spill("Wildcard matches everything")
}

// Test 7: Array pattern matching (simplified)
sus arr []drip = [1, 2, 3]
vibe_check arr {
    mood []:
        vibez.spill("Empty array")
    mood [head, ...tail]:
        vibez.spill("Head:", head, "Tail length:", len(tail))
    basic:
        vibez.spill("Other array pattern")
}

// Test 8: Struct pattern matching
squad Person {
    name tea
    age drip
}

sus person Person = Person{name: "Alice", age: 25}
vibe_check person {
    mood Person{name: "Alice", age}:
        vibez.spill("Found Alice, age:", age)
    mood Person{name, age} when age >= 18:
        vibez.spill("Adult:", name)
    basic:
        vibez.spill("Other person")
}

// Test 9: Enum pattern matching
ghosted Status {
    Success(drip)
    Error(tea)
    Loading
}

sus status Status = Status.Success(200)
vibe_check status {
    mood Status.Success(code):
        vibez.spill("Success with code:", code)
    mood Status.Error(msg):
        vibez.spill("Error:", msg)
    mood Status.Loading:
        vibez.spill("Still loading...")
}

// Test 10: Nested pattern matching
sus nested_data (drip, tea) = (42, "hello")
vibe_check nested_data {
    mood (42, message):
        vibez.spill("Found 42 with message:", message)
    mood (num, "world"):
        vibez.spill("Found world with number:", num)
    basic:
        vibez.spill("Other tuple pattern")
}

// Test 11: Complex guard with multiple conditions
sus complex_value drip = 50
vibe_check complex_value {
    mood x when x > 0 and x < 100 and x % 2 == 0:
        vibez.spill("Even number between 0 and 100:", x)
    mood x when x % 3 == 0:
        vibez.spill("Multiple of 3:", x)
    basic:
        vibez.spill("Other number:", complex_value)
}

// Test 12: Exhaustiveness checking
ghosted Color {
    Red
    Green
    Blue
}

sus color Color = Color.Red
vibe_check color {
    mood Color.Red:
        vibez.spill("Red color")
    mood Color.Green:
        vibez.spill("Green color") 
    mood Color.Blue:
        vibez.spill("Blue color")
    // No basic case needed - exhaustive
}

// Test 13: Non-exhaustive pattern (should generate warning/error)
vibe_check color {
    mood Color.Red:
        vibez.spill("Only red handled")
    // Missing Green and Blue - should warn about non-exhaustive patterns
}

vibez.spill("🎯 Pattern matching tests completed!")
