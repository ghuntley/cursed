// Test advanced pattern matching in vibe_check statements
yeet "vibez"

slay test_pattern_matching() {
    // Test literal pattern matching
    sus value lit = based
    
    vibe_check value {
        mood based:
            vibez.spill("Pattern matched: true")
        mood cap:
            vibez.spill("Pattern matched: false")
        basic:
            vibez.spill("Default case")
    }
    
    // Test variable pattern matching
    sus number normie = 42
    
    vibe_check number {
        mood 42:
            vibez.spill("Number is 42")
        mood x:
            vibez.spill("Number is something else")
    }
    
    // Test string pattern matching
    sus name tea = "CURSED"
    
    vibe_check name {
        mood "CURSED":
            vibez.spill("Language is CURSED")
        mood "Go":
            vibez.spill("Language is Go")
        mood _:
            vibez.spill("Unknown language")
    }
    
    // Test tuple pattern matching
    sus pair := (1, 2)
    
    vibe_check pair {
        mood (1, 2):
            vibez.spill("Pair is (1, 2)")
        mood (x, y):
            vibez.spill("Pair is something else")
    }
}

// Test type pattern matching
slay test_type_patterns() {
    sus value any // Interface{} equivalent
    
    vibe_check value {
        mood x tea:
            vibez.spill("Value is a string")
        mood x normie:
            vibez.spill("Value is an integer")
        mood x lit:
            vibez.spill("Value is a boolean")
        basic:
            vibez.spill("Value is unknown type")
    }
}

// Test exhaustiveness checking
slay test_exhaustive_patterns() {
    sus flag lit = based
    
    // This should compile (exhaustive)
    vibe_check flag {
        mood based:
            vibez.spill("True case")
        mood cap:
            vibez.spill("False case")
    }
    
    // This should also compile (has wildcard)
    vibe_check flag {
        mood based:
            vibez.spill("True case")
        mood _:
            vibez.spill("Other case")
    }
}

// Test pattern destructuring
slay test_destructuring() {
    sus person := Person{name: "Alice", age: 30}
    
    vibe_check person {
        mood Person{name: "Alice", age: x}:
            vibez.spill("Alice with age")
        mood Person{name: n, age: 30}:
            vibez.spill("30 year old person")
        mood Person{name: n, age: a}:
            vibez.spill("Some person")
    }
}

slay main() {
    test_pattern_matching()
    test_type_patterns()
    test_exhaustive_patterns()
    test_destructuring()
}
