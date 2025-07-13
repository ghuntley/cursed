vibez.spill("Testing validation module import...")

slay main() {
    vibez.spill("Starting validation test")
    
    // Try to create ValidationResult directly
    be_like ValidationResult squad {
        is_valid lit
        errors []tea
        warnings []tea
    }
    
    sus result ValidationResult = ValidationResult{
        is_valid: based,
        errors: []tea{},
        warnings: []tea{}
    }
    
    vibez.spill("ValidationResult created successfully")
}
