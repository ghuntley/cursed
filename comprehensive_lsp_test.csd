// Comprehensive LSP feature test for CURSED language
yeet "testz"
yeet "vibez"
yeet "cryptz"
yeet "concurrenz"

// Test struct definitions
squad Person {
    spill name tea
    spill age normie
    spill active lit
}

// Test interface definitions
collab Drawable {
    slay draw() 
    slay get_area() meal
}

// Test function with various parameter types
slay calculate_total(values []normie, multiplier meal) meal {
    sus total meal = 0.0
    
    bestie (i normie := 0; i < len(values); i = i + 1) {
        total = total + values[i] * multiplier
    }
    
    damn total
}

// Test error handling and pattern matching
slay process_data(input tea) tea {
    match input {
        "start" => {
            vibez.spill("Starting process...")
            damn "initiated"
        }
        "stop" => {
            vibez.spill("Stopping process...")
            damn "terminated"
        }
        _ => {
            vibez.spill("Unknown command")
            damn "error"
        }
    }
}

// Test concurrency features
slay concurrent_worker(id normie, ch dm normie) {
    bestie (i normie := 0; i < 5; i = i + 1) {
        vibez.spillf("Worker {} processing item {}", id, i)
        concurrenz.send(ch, i)
    }
}

// Test advanced type usage
slay advanced_example() {
    // Variable declarations with different types
    sus name tea = "CURSED Developer"
    sus count normie = 42
    sus ratio meal = 3.14159
    sus active lit = based
    sus tags []tea = []tea{"programming", "gen-z", "cursed"}
    
    // Struct instantiation
    sus person Person = Person{
        name: name,
        age: count,
        active: active
    }
    
    // Array operations
    sus numbers []normie = make(normie, 5)
    numbers = append(numbers, 1)
    numbers = append(numbers, 2)
    numbers = append(numbers, 3)
    
    // Channel operations
    sus ch dm normie = make(dm normie, 10)
    stan { concurrent_worker(1, ch) }
    stan { concurrent_worker(2, ch) }
    
    // Crypto operations
    sus data []byte = []byte("sensitive information")
    sus key []byte = []byte("encryption-key-32-bytes-long!")
    sus encrypted []byte = cryptz.encrypt(data, key)
    sus hashed []byte = cryptz.hash(data)
    
    // String interpolation and output
    vibez.spillf("Person: {} (Age: {})", person.name, person.age)
    vibez.spillf("Numbers count: {}", len(numbers))
    vibez.spillf("Encrypted size: {} bytes", len(encrypted))
    vibez.spillf("Hash size: {} bytes", len(hashed))
    
    // Mathematical operations
    sus result meal = calculate_total(numbers, 2.5)
    vibez.spillf("Calculation result: {}", result)
    
    // Conditional logic
    facts (person.active) {
        vibez.spill("Person is active!")
    } lowkey {
        vibez.spill("Person is inactive.")
    }
    
    // Pattern matching with result
    sus status tea = process_data("start")
    vibez.spillf("Process status: {}", status)
}

// Main function to test everything
slay main() {
    vibez.spill("=== CURSED LSP Comprehensive Test ===")
    advanced_example()
    vibez.spill("=== Test Completed Successfully ===")
}

// Helper function for testing LSP go-to-definition
slay utility_function(param normie) normie {
    damn param * 2 + 1
}

// Test variable scoping and references
slay scoping_test() {
    sus outer_var normie = 100
    
    facts (outer_var > 50) {
        sus inner_var normie = utility_function(outer_var)
        vibez.spillf("Inner variable: {}", inner_var)
    }
    
    vibez.spillf("Outer variable: {}", outer_var)
}

// Test error conditions for diagnostics
slay diagnostic_test() {
    // This should trigger LSP diagnostics for potential issues
    sus uninitialized_var normie
    // sus unused_variable tea = "not used"  // Commented to avoid warning
    
    vibez.spill("Diagnostic test function")
}
