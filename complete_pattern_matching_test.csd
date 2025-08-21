// Complete Pattern Matching Test Suite for CURSED
// Tests all advanced pattern matching features including compilation to LLVM IR

yeet "vibez"
yeet "mathz"
yeet "stringz"

// Test 1: Enum pattern matching with exhaustiveness checking
ghosted Color {
    Red
    Green  
    Blue
    Custom(drip) // Enum variant with data
}

slay test_enum_patterns() {
    sus color Color = Color.Red
    
    // Exhaustive enum pattern matching
    vibe_check color {
        mood Color.Red:
            vibez.spill("✅ Red color matched!")
            damn 1
        mood Color.Green:
            vibez.spill("✅ Green color matched!")
            damn 2
        mood Color.Blue:
            vibez.spill("✅ Blue color matched!")
            damn 3
        mood Color.Custom(value):
            vibez.spill("✅ Custom color with value:", value)
            damn value
        // No default case needed - exhaustive
    }
}

// Test 2: Struct destructuring patterns
squad Person {
    name tea
    age drip
    active lit
}

slay test_struct_patterns() {
    sus person Person = Person{name: "Alice", age: 25, active: based}
    
    vibe_check person {
        mood Person{name: "Alice", age}:
            vibez.spill("✅ Found Alice, age:", age)
            damn age
        mood Person{name, age} when age >= 18:
            vibez.spill("✅ Adult:", name, "age:", age)
            damn age
        mood Person{name, age, active} when active:
            vibez.spill("✅ Active person:", name)
            damn 1
        basic:
            vibez.spill("✅ Other person pattern")
            damn 0
    }
}

// Test 3: Array/slice pattern matching with rest elements
slay test_array_patterns() {
    sus numbers []drip = [1, 2, 3, 4, 5]
    
    vibe_check numbers {
        mood []:
            vibez.spill("✅ Empty array")
            damn 0
        mood [head]:
            vibez.spill("✅ Single element:", head)
            damn head
        mood [first, second]:
            vibez.spill("✅ Two elements:", first, second)
            damn first + second
        mood [head, ...tail]:
            vibez.spill("✅ Head:", head, "Tail length:", len(tail))
            damn head + len(tail)
        basic:
            vibez.spill("✅ Other array pattern")
            damn -1
    }
}

// Test 4: Guard clauses with complex conditions
slay test_guard_patterns() {
    sus num drip = 15
    
    vibe_check num {
        mood x when x > 10 and x < 20 and x % 5 == 0:
            vibez.spill("✅ Multiple of 5 between 10 and 20:", x)
            damn x
        mood x when x % 2 == 0:
            vibez.spill("✅ Even number:", x)
            damn x * 2
        mood x when x % 3 == 0:
            vibez.spill("✅ Multiple of 3:", x)
            damn x * 3
        basic:
            vibez.spill("✅ Other number:", num)
            damn num
    }
}

// Test 5: Nested pattern matching with tuples
slay test_nested_patterns() {
    sus data (Color, drip, tea) = (Color.Blue, 42, "hello")
    
    vibe_check data {
        mood (Color.Red, value, message):
            vibez.spill("✅ Red with:", value, message)
            damn value
        mood (Color.Blue, 42, message):
            vibez.spill("✅ Blue 42 with message:", message)
            damn 42
        mood (color, num, "world"):
            vibez.spill("✅ Any color with world:", color, num)
            damn num
        basic:
            vibez.spill("✅ Other tuple pattern")
            damn 0
    }
}

// Test 6: OR patterns (multiple alternatives)
slay test_or_patterns() {
    sus letter tea = "a"
    
    vibe_check letter {
        mood "a" | "e" | "i" | "o" | "u":
            vibez.spill("✅ Vowel found:", letter)
            damn 1
        mood "y":
            vibez.spill("✅ Sometimes vowel:", letter)
            damn 2
        basic:
            vibez.spill("✅ Consonant:", letter)
            damn 0
    }
}

// Test 7: Range patterns with different syntaxes
slay test_range_patterns() {
    sus score drip = 85
    
    vibe_check score {
        mood 90..100:
            vibez.spill("✅ Excellent score!")
            damn 5
        mood 80..89:
            vibez.spill("✅ Good score!")
            damn 4
        mood 70..79:
            vibez.spill("✅ Fair score!")
            damn 3
        mood 60..69:
            vibez.spill("✅ Passing score!")
            damn 2
        mood 0..59:
            vibez.spill("✅ Needs improvement!")
            damn 1
        basic:
            vibez.spill("✅ Invalid score")
            damn 0
    }
}

// Test 8: Wildcard patterns
slay test_wildcard_patterns() {
    sus anything drip = 999
    
    vibe_check anything {
        mood 42:
            vibez.spill("✅ The answer!")
            damn 42
        mood _:
            vibez.spill("✅ Wildcard matches everything:", anything)
            damn anything
    }
}

// Test 9: Complex nested structures
ghosted Result<T> {
    Success(T)
    Error(tea)
}

slay test_complex_patterns() {
    sus result Result<Person> = Result.Success(Person{name: "Bob", age: 30, active: based})
    
    vibe_check result {
        mood Result.Success(Person{name: "Alice", age}):
            vibez.spill("✅ Alice success with age:", age)
            damn age
        mood Result.Success(Person{name, age}) when age >= 18:
            vibez.spill("✅ Adult success:", name, age)
            damn age
        mood Result.Error(msg):
            vibez.spill("✅ Error occurred:", msg)
            damn 0
        basic:
            vibez.spill("✅ Other result pattern")
            damn -1
    }
}

// Test 10: Non-exhaustive pattern checking (should generate warnings)
slay test_non_exhaustive() {
    sus color Color = Color.Red
    
    // This should trigger exhaustiveness warnings since Blue and Custom are missing
    vibe_check color {
        mood Color.Red:
            vibez.spill("⚠️ Only Red handled")
            damn 1
        mood Color.Green:
            vibez.spill("⚠️ Only Green handled")
            damn 2
        // Missing Color.Blue and Color.Custom - should warn
    }
}

// Main test runner
slay main() drip {
    vibez.spill("🎯 Starting Complete Pattern Matching Test Suite")
    
    vibez.spill("1. Testing enum patterns...")
    sus enum_result drip = test_enum_patterns()
    vibez.spill("   Result:", enum_result)
    
    vibez.spill("2. Testing struct patterns...")
    sus struct_result drip = test_struct_patterns()
    vibez.spill("   Result:", struct_result)
    
    vibez.spill("3. Testing array patterns...")
    sus array_result drip = test_array_patterns()
    vibez.spill("   Result:", array_result)
    
    vibez.spill("4. Testing guard patterns...")
    sus guard_result drip = test_guard_patterns()
    vibez.spill("   Result:", guard_result)
    
    vibez.spill("5. Testing nested patterns...")
    sus nested_result drip = test_nested_patterns()
    vibez.spill("   Result:", nested_result)
    
    vibez.spill("6. Testing OR patterns...")
    sus or_result drip = test_or_patterns()
    vibez.spill("   Result:", or_result)
    
    vibez.spill("7. Testing range patterns...")
    sus range_result drip = test_range_patterns()
    vibez.spill("   Result:", range_result)
    
    vibez.spill("8. Testing wildcard patterns...")
    sus wildcard_result drip = test_wildcard_patterns()
    vibez.spill("   Result:", wildcard_result)
    
    vibez.spill("9. Testing complex patterns...")
    sus complex_result drip = test_complex_patterns()
    vibez.spill("   Result:", complex_result)
    
    vibez.spill("10. Testing non-exhaustive patterns...")
    sus non_exhaustive_result drip = test_non_exhaustive()
    vibez.spill("   Result:", non_exhaustive_result)
    
    vibez.spill("🚀 Complete Pattern Matching Test Suite finished!")
    vibez.spill("   All advanced pattern matching features tested successfully")
    
    damn 0
}
