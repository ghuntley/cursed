vibe main

yeet "vibez"

// Feature test validates proper error handling and recovery mechanisms
// Tests various error scenarios to ensure graceful handling between modes
// Includes division handling, variable scope, and function calls

slay main_character() {
    vibez.spill("=== Error Recovery Feature Test ===")
    
    // Test safe operations first
    sus safe_value normie = 10
    sus safe_result normie = safe_divide(safe_value, 2)
    vibez.spill("Safe division result:")
    vibez.spill(safe_result)
    
    // Test boundary conditions
    sus boundary_result normie = safe_divide(100, 1)
    vibez.spill("Boundary test result:")
    vibez.spill(boundary_result)
    
    // Test negative number handling
    sus negative_test normie = -15
    sus abs_result normie = manual_abs(negative_test)
    vibez.spill("Absolute value test:")
    vibez.spill(abs_result)
    
    // Test with positive numbers
    sus positive_test normie = 25
    sus pos_result normie = manual_abs(positive_test)
    vibez.spill("Positive test:")
    vibez.spill(pos_result)
    
    // Test variable scope and recovery
    ready based {
        sus local_var normie = 25
        vibez.spill("Local scope test:")
        vibez.spill(local_var)
    }
    
    // Test function call error recovery
    sus func_result tea = error_prone_function("valid_input")
    vibez.spill("Function result:")
    vibez.spill(func_result)
    
    vibez.spill("=== Error Recovery Tests Complete ===")
}

slay safe_divide(a normie, b normie) normie {
    ready b {
        damn a / b
    } otherwise {
        vibez.spill("Warning: Division by zero avoided")
        damn 0
    }
}

slay manual_abs(x normie) normie {
    ready x < 0 {
        damn 0 - x
    } otherwise {
        damn x
    }
}

slay error_prone_function(input tea) tea {
    damn "Success: Input processed"
}
