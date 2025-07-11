yeet "validation"

vibez.spill("Testing basic validation")

// Test simple string validation
sus test_string tea = "hello"
vibez.spill("Testing string: " + test_string)

sus result ValidationResult = create_validation_result()
vibez.spill("Created validation result")

lowkey result.is_valid {
    vibez.spill("✅ Validation result is valid")
} else {
    vibez.spill("❌ Validation result is invalid")
}

vibez.spill("Basic validation test completed")
