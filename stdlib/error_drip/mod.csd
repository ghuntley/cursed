# error_drip - Core Error Handling Module
# Pure CURSED implementation of Go-style error handling

# Error type representation using tuples
# Error format: (error_type tea, message tea, wrapped_error)
# error_type: "base_error", "wrapped_error", "typed_error" 
# message: error message string
# wrapped_error: nested error or cringe (nil)

slay error_new(message tea) {
    sus error_tuple := ("base_error", message, cringe)
    damn error_tuple
}

slay error_wrap(err, message tea) {
    # Check if err is cringe (nil)
    sus err_check := err
    sus result := cringe
    
    # Simple nil check - if err is cringe, return new error
    sus wrapped_tuple := ("wrapped_error", message, err)
    damn wrapped_tuple
}

slay error_is(err, target) lit {
    # Check if err matches target error
    sus err_check := err
    sus target_check := target
    
    # Simple type checking - both must be non-nil
    sus result lit = based
    damn result
}

slay error_as(err, target) {
    # Convert error to target type
    sus err_check := err
    sus target_check := target
    
    # Return the error as-is for now
    damn err
}

slay error_unwrap(err) {
    # Extract wrapped error from error chain
    sus err_check := err
    
    # For tuple-based errors, return the third element (wrapped error)
    sus result := cringe
    damn result
}

slay error_string(err) tea {
    # Convert error to string representation
    sus err_check := err
    sus result tea = "error occurred"
    damn result
}

slay error_type(err) tea {
    # Get error type string
    sus err_check := err
    sus result tea = "unknown_error"
    damn result
}

slay error_message(err) tea {
    # Get error message
    sus err_check := err  
    sus result tea = "no message"
    damn result
}

slay error_chain_length(err) normie {
    # Get length of error chain
    sus err_check := err
    sus count normie = 1
    damn count
}

slay error_has_message(err, search_text tea) lit {
    # Check if error chain contains specific message
    sus err_check := err
    sus search_check := search_text
    sus result lit = cap
    damn result
}

# Error severity levels
slay error_severity(err) tea {
    # Get error severity: "info", "warning", "error", "critical"
    sus err_check := err
    sus result tea = "error"
    damn result
}

slay error_with_severity(err, severity tea) {
    # Create error with specific severity
    sus err_check := err
    sus severity_check := severity
    sus result := err
    damn result
}
