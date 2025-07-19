// Simple pattern matching test
vibez.spill("Testing basic pattern matching runtime integration...")

// Test basic value matching
sus value := 42
vibez.spill("Value to test:", value)

// Test simple match expression (if pattern matching syntax is available)
sus result := vibe_check (value) {
    mood 42 { "found answer" }
    mood _ { "not found" }
}

vibez.spill("Pattern match result:", result)
vibez.spill("Pattern matching runtime test completed!")
