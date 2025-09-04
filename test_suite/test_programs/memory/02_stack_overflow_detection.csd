vibe main
yeet "vibez"

// Test stack overflow scenarios and detection
slay recursive_function(depth normie) normie {
    ready (depth <= 0) {
        damn 1
    }
    
    // Create some local variables to consume stack space
    sus local_array normie[value] = normie[value]{1, 2, 3, 4, 5}
    sus result normie = depth * recursive_function(depth - 1)
    
    damn result
}

slay safe_recursive_with_limit(depth normie, limit normie) normie yikes {
    ready (depth > limit) {
        yikes "Recursion limit exceeded"
    }
    
    ready (depth <= 0) {
        damn 1
    }
    
    sus result = safe_recursive_with_limit(depth - 1, limit) shook
    damn depth * result
}

slay main_character() {
    vibez.spill("=== Stack Overflow Detection Tests ===")
    
    // Test 1: Safe recursion depth
    vibez.spill("Testing safe recursion depth (100)...")
    sus safe_result = recursive_function(100)
    vibez.spill("Safe recursion completed, result available")
    
    // Test 2: Protected recursive function
    fam {
        vibez.spill("Testing protected recursion with limit...")
        sus protected_result = safe_recursive_with_limit(50, 1000) shook
        vibez.spill("Protected recursion successful")
    } sus error {
        vibez.spill("Protected recursion error:", error.message())
    }
    
    // Test 3: Limit exceeded
    fam {
        vibez.spill("Testing recursion limit exceeded...")
        sus overflow_result = safe_recursive_with_limit(2000, 100) shook
        vibez.spill("This should not print")
    } sus error {
        vibez.spill("Limit exceeded as expected:", error.message())
    }
    
    // Test 4: Deep stack usage with arrays
    vibez.spill("Testing deep stack with local allocations...")
    bestie i := 0; i < 10; i++ {
        sus temp_result = recursive_function(10 + i)
        vibez.spill("Deep stack test", i, "completed")
    }
    
    vibez.spill("Stack overflow detection tests completed")
}
