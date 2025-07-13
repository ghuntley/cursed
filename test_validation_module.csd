// Test just the validation result structure
be_like ValidationResult squad {
    is_valid lit
    errors []tea
    warnings []tea
}

slay create_validation_result() ValidationResult {
    sus result ValidationResult = ValidationResult{
        is_valid: based,
        errors: []tea{},
        warnings: []tea{}
    }
    damn result
}

slay main() {
    sus result ValidationResult = create_validation_result()
    vibez.spill("Test passed")
}
