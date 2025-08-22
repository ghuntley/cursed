// Simple environment variable test

// Test runtime_get_env directly
(value, err) := runtime_get_env("HOME")

lowkey err == "" {
    vibez.spill("HOME directory found:", value)
} otherwise {
    vibez.spill("Error getting HOME:", err)
}

// Test basic functionality
vibez.spill("Testing basic environment variable functions...")

// Try setting a test variable
test_err := runtime_set_env("TEST_VAR", "test_value")
lowkey test_err == "" {
    vibez.spill("Successfully set TEST_VAR")
    
    // Try getting it back
    (test_val, get_err) := runtime_get_env("TEST_VAR")
    lowkey get_err == "" {
        vibez.spill("Retrieved TEST_VAR:", test_val)
    } otherwise {
        vibez.spill("Error retrieving TEST_VAR:", get_err)
    }
} otherwise {
    vibez.spill("Error setting TEST_VAR:", test_err)
}

vibez.spill("Environment test completed")
