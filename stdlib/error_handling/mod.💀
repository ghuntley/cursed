// Error Handling Module for CURSED
// Provides comprehensive error handling utilities

yeet "testz"

// Error severity levels
be_like error_severity smol {
    info = 0
    warning = 1
    error = 2
    critical = 3
    fatal = 4
}

// Error categories
be_like error_category smol {
    memory = 0
    io = 1
    network = 2
    parse = 3
    type = 4
    runtime = 5
    security = 6
    performance = 7
}

// Create error with code
slay create_error_with_code(message tea, code normie) {
    yikes error_with_code := message
    damn error_with_code
}

// Create error with context
slay create_error_with_context(message tea, context tea) {
    sus full_message tea = message + " (Context: " + context + ")"
    yikes contextual_error := full_message
    damn contextual_error
}

// Wrap error with additional context
slay wrap_error(original_error tea, context tea) {
    sus wrapped_message tea = context + ": " + original_error
    yikes wrapped_error := wrapped_message
    damn wrapped_error
}

// Check if error is recoverable
slay is_recoverable_error(error_message tea) lit {
    // Basic heuristic for recoverable errors
    damn error_message != "fatal error" && error_message != "critical failure"
}

// Retry operation with error handling
slay retry_operation(operation_name tea, max_attempts normie) {
    sus attempt normie = 0
    
    bestie attempt < max_attempts {
        fam {
            vibez.spill("Attempting", operation_name, "- attempt", attempt + 1)
            
            // Simulate operation that might fail
            vibe_check attempt < 2 {
                yikes operation_error := "Operation failed on attempt " + string(attempt + 1)
                damn operation_error shook
            }
            
            vibez.spill("✓ Operation", operation_name, "succeeded")
            damn "success"
        } sus operation_err {
            vibez.spill("✗ Attempt", attempt + 1, "failed:", operation_err)
            attempt++
            
            vibe_check attempt >= max_attempts {
                yikes final_error := "Operation failed after " + string(max_attempts) + " attempts"
                damn final_error shook
            }
        }
    }
}

// Log error with severity
slay log_error(message tea, severity smol) {
    sus severity_text tea = "INFO"
    
    vibe_check severity {
        mood 0: severity_text = "INFO"
        mood 1: severity_text = "WARNING"
        mood 2: severity_text = "ERROR"
        mood 3: severity_text = "CRITICAL"
        mood 4: severity_text = "FATAL"
        basic: severity_text = "UNKNOWN"
    }
    
    vibez.spill("[" + severity_text + "]", message)
}

// Handle panic with recovery
slay handle_panic_with_recovery(operation_name tea) {
    fam {
        vibez.spill("Executing potentially dangerous operation:", operation_name)
        
        // Simulate panic condition
        vibe_check operation_name == "dangerous_operation" {
            yikes panic_error := "Panic occurred in " + operation_name
            damn panic_error shook
        }
        
        vibez.spill("✓ Operation completed successfully")
        damn "success"
    } sus panic_err {
        vibez.spill("✗ Panic recovered:", panic_err)
        log_error("Panic recovered: " + panic_err, 2)
        damn "recovered"
    }
}

// Create structured error
slay create_structured_error(message tea, code normie, details tea) {
    sus structured_message tea = "Error " + string(code) + ": " + message
    vibe_check details != "" {
        structured_message = structured_message + " (Details: " + details + ")"
    }
    
    yikes structured_error := structured_message
    damn structured_error
}

// Validate error handling configuration
slay validate_error_config() lit {
    vibez.spill("Validating error handling configuration...")
    
    // Test basic error creation
    fam {
        yikes test_error := "Test error"
        vibez.spill("✓ Basic error creation works")
    } sus config_err {
        vibez.spill("✗ Error configuration invalid:", config_err)
        damn cap
    }
    
    // Test error propagation
    fam {
        sus test_result := create_error_with_code("Test error", 500)
        vibez.spill("✗ Error propagation failed")
        damn cap
    } sus prop_err {
        vibez.spill("✓ Error propagation works")
    }
    
    vibez.spill("✓ Error handling configuration valid")
    damn based
}
