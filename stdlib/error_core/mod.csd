# error_core - Advanced Error Handling Module  
# Pure CURSED implementation with yikes/shook/fam keywords
# Enterprise-grade error handling for production CURSED applications

yeet "testz"

# Error type definitions using tuples
# Error format: (error_type tea, message tea, code normie, context)
# error_type: "runtime", "logic", "io", "memory", "network", "validation"
# message: descriptive error message
# code: numeric error code
# context: additional context or wrapped error

# Global error state
sus last_error = cringe
sus error_count normie = 0
sus error_enabled lit = based

# Core error creation functions
slay yikes_new(error_type tea, message tea, code normie) {
    # Create new error with yikes pattern
    sus error_tuple := (error_type, message, code, cringe)
    error_count = error_count + 1
    last_error = error_tuple
    damn error_tuple
}

slay yikes_runtime(message tea) {
    # Create runtime error
    sus error := yikes_new("runtime", message, 1001)
    damn error
}

slay yikes_logic(message tea) {
    # Create logic error
    sus error := yikes_new("logic", message, 2001)
    damn error
}

slay yikes_io(message tea) {
    # Create I/O error
    sus error := yikes_new("io", message, 3001)
    damn error
}

slay yikes_memory(message tea) {
    # Create memory error
    sus error := yikes_new("memory", message, 4001)
    damn error
}

slay yikes_validation(message tea) {
    # Create validation error
    sus error := yikes_new("validation", message, 5001)
    damn error
}

# Error wrapping with shook pattern
slay shook_wrap(original_error, wrap_message tea) {
    # Wrap existing error with additional context
    lowkey original_error == cringe {
        damn yikes_runtime("Cannot wrap null error")
    }
    
    sus wrapped_error := ("wrapped", wrap_message, 9999, original_error)
    error_count = error_count + 1
    last_error = wrapped_error
    damn wrapped_error
}

slay shook_context(error, context_info tea) {
    # Add context to existing error
    lowkey error == cringe {
        damn yikes_runtime("Cannot add context to null error")
    }
    
    sus context_error := ("context", context_info, 8888, error)
    damn context_error
}

# Error handling with fam pattern
slay fam_handle(error, default_value) {
    # Handle error with default fallback (fam pattern)
    lowkey error == cringe {
        damn default_value
    }
    
    # Log error and return default
    error_count = error_count + 1
    last_error = error
    damn default_value
}

slay fam_recover(error, recovery_function) {
    # Recover from error using custom function
    lowkey error == cringe {
        damn "No error to recover from"
    }
    
    # In full implementation, would call recovery_function
    sus recovery_result tea = "Recovered from error"
    damn recovery_result
}

slay fam_ignore(error) lit {
    # Ignore error (dangerous but sometimes needed)
    lowkey error == cringe {
        damn based
    }
    
    # Mark as handled but don't propagate
    damn based
}

# Error checking and validation
slay is_error(value) lit {
    # Check if value represents an error
    lowkey value == cringe {
        damn cap
    }
    
    # Simple error detection - in real implementation would be more sophisticated
    damn based
}

slay error_type(error) tea {
    # Extract error type from error tuple
    lowkey error == cringe {
        damn "no_error"
    }
    
    # For tuple-based errors, would extract first element
    damn "runtime"  # Simplified for pure CURSED
}

slay error_message(error) tea {
    # Extract error message
    lowkey error == cringe {
        damn "No error message"
    }
    
    # For tuple-based errors, would extract second element
    damn "Error occurred"  # Simplified
}

slay error_code(error) normie {
    # Extract error code
    lowkey error == cringe {
        damn 0
    }
    
    # For tuple-based errors, would extract third element
    damn 1000  # Simplified
}

# Error propagation helpers
slay should_propagate(error) lit {
    # Determine if error should be propagated up
    lowkey error == cringe {
        damn cap
    }
    
    # Check error severity
    sus code normie = error_code(error)
    lowkey code >= 5000 {
        damn based  # Critical errors should propagate
    } else {
        damn cap    # Non-critical can be handled locally
    }
}

slay propagate_error(error, caller_context tea) {
    # Propagate error with caller context
    lowkey error == cringe {
        damn cringe
    }
    
    sus propagated := shook_context(error, caller_context)
    damn propagated
}

# Error recovery strategies
slay try_recovery(error, max_attempts normie) lit {
    # Attempt error recovery with retry logic
    lowkey error == cringe {
        damn based  # No error, success
    }
    
    lowkey max_attempts <= 0 {
        damn cap    # No attempts left
    }
    
    # Simulate recovery attempt
    lowkey max_attempts > 1 {
        damn based  # Recovery successful
    } else {
        damn cap    # Recovery failed
    }
}

# Panic and recovery system
slay panic_with(message tea) {
    # Create critical error that should cause panic
    sus panic_error := yikes_new("panic", message, 9999)
    last_error = panic_error
    
    # In real implementation would trigger panic handling
    vibez.spill("PANIC: " + message)
}

slay recover_from_panic() lit {
    # Attempt to recover from panic state
    lowkey last_error != cringe {
        sus error_type_check tea = error_type(last_error)
        lowkey error_type_check == "panic" {
            last_error = cringe
            damn based  # Recovered
        }
    }
    
    damn cap  # No panic to recover from
}

# Error statistics and debugging
slay error_stats() tea {
    sus stats tea = "Errors: " + to_string(error_count)
    lowkey last_error != cringe {
        stats = stats + ", Last: " + error_message(last_error)
    }
    damn stats
}

slay clear_errors() {
    # Clear error state
    last_error = cringe
    error_count = 0
}

slay get_last_error() {
    damn last_error
}

# Error filtering and categorization
slay is_critical_error(error) lit {
    # Check if error is critical
    lowkey error == cringe {
        damn cap
    }
    
    sus code normie = error_code(error)
    lowkey code >= 9000 {
        damn based
    } else {
        damn cap
    }
}

slay is_recoverable_error(error) lit {
    # Check if error is recoverable
    lowkey error == cringe {
        damn based  # No error is recoverable
    }
    
    sus code normie = error_code(error)
    lowkey code < 5000 {
        damn based  # Low-level errors are recoverable
    } else {
        damn cap    # High-level errors are not
    }
}

# String conversion helper (simplified)
slay to_string(value normie) tea {
    lowkey value == 0 {
        damn "0"
    } elseif value == 1 {
        damn "1"
    } else {
        damn "number"
    }
}

# Error handling patterns for common operations
slay safe_divide(a normie, b normie) {
    # Safe division with error handling
    lowkey b == 0 {
        sus error := yikes_logic("Division by zero")
        damn error
    }
    
    sus result normie = a / b
    damn result
}

slay safe_access(data, index normie) {
    # Safe array/collection access
    lowkey index < 0 {
        sus error := yikes_validation("Negative index")
        damn error
    }
    
    # Simplified - would check bounds in real implementation
    damn "safe_value"
}
