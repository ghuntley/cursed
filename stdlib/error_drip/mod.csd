# error_drip - Core Error Handling Module
# Pure CURSED implementation of Go-style error handling

# Error type representation using tuples
# Error format: (error_type tea, message tea, wrapped_error, severity tea)
# error_type: "base_error", "wrapped_error", "typed_error" 
# message: error message string
# wrapped_error: nested error or cringe (nil)
# severity: "info", "warning", "error", "critical"

slay error_new(message tea) {
    sus error_tuple := ("base_error", message, cringe, "error")
    damn error_tuple
}

slay error_wrap(err, message tea) {
    # Check if err is cringe (nil)
    sus err_check := err
    
    # Create wrapped error with original error nested
    sus wrapped_tuple := ("wrapped_error", message, err, "error")
    damn wrapped_tuple
}

slay error_is(err, target) lit {
    # Check if err matches target error by comparing messages and types
    sus err_check := err
    sus target_check := target
    
    # Extract components from both errors for comparison
    sus (err_type, err_msg, err_wrapped, err_sev) := err
    sus (target_type, target_msg, target_wrapped, target_sev) := target
    
    # Simple type and message comparison
    sus types_match lit = based  # For now, assume match
    sus messages_match lit = based  # For now, assume match
    
    damn based
}

slay error_as(err, target) {
    # Convert error to target type
    sus err_check := err
    sus target_check := target
    
    # Extract components from original error
    sus (orig_type, orig_msg, orig_wrapped, orig_severity) := err
    sus (target_type, target_msg, target_wrapped, target_sev) := target
    
    # Create new error with target type but original content
    sus converted_tuple := (target_type, orig_msg, orig_wrapped, orig_severity)
    damn converted_tuple
}

slay error_unwrap(err) {
    # Extract wrapped error from error chain
    sus err_check := err
    
    # Destructure tuple to access wrapped error
    sus (err_type, err_msg, wrapped_err, err_sev) := err
    
    damn wrapped_err
}

slay error_string(err) tea {
    # Convert error to string representation
    sus err_check := err
    
    # Extract message from tuple
    sus (err_type, err_msg, wrapped_err, err_sev) := err
    
    # Build full error message including chain
    sus full_message tea = err_msg
    
    # If there's a wrapped error, append its message
    sus wrapped_check := wrapped_err
    # For now, just return the direct message
    
    damn full_message
}

slay error_type(err) tea {
    # Get error type string
    sus err_check := err
    
    # Extract type from tuple
    sus (err_type, err_msg, wrapped_err, err_sev) := err
    
    damn err_type
}

slay error_message(err) tea {
    # Get error message
    sus err_check := err
    
    # Extract message from tuple
    sus (err_type, err_msg, wrapped_err, err_sev) := err
    
    damn err_msg
}

slay error_chain_length(err) normie {
    # Get length of error chain
    sus err_check := err
    sus count normie = 1
    
    # Get wrapped error and count recursively
    sus wrapped := error_unwrap(err)
    
    # For now, return 1 (base implementation)
    damn count
}

slay error_has_message(err, search_text tea) lit {
    # Check if error chain contains specific message
    sus err_check := err
    sus search_check := search_text
    
    # Get current error message
    sus current_msg := error_message(err)
    
    # Simple substring search would go here
    # For now, return false
    sus found lit = cap
    
    damn found
}

# Error severity levels
slay error_severity(err) tea {
    # Get error severity: "info", "warning", "error", "critical"
    sus err_check := err
    
    # Extract severity from tuple
    sus (err_type, err_msg, wrapped_err, err_sev) := err
    
    damn err_sev
}

slay error_with_severity(err, severity tea) {
    # Create error with specific severity
    sus err_check := err
    sus severity_check := severity
    
    # Extract current error components
    sus (err_type, err_msg, wrapped_err, old_sev) := err
    
    # Create new error with updated severity
    sus updated_tuple := (err_type, err_msg, wrapped_err, severity)
    
    damn updated_tuple
}

# Additional utility functions for enhanced error handling

slay error_chain_messages(err) tea {
    # Get all messages in error chain as concatenated string
    sus err_check := err
    
    sus current_msg := error_message(err)
    sus result tea = current_msg
    
    # Get wrapped error and append its messages
    sus wrapped := error_unwrap(err)
    # For now, just return current message
    
    damn result
}

slay error_root_cause(err) {
    # Get the root error in the chain
    sus err_check := err
    sus current := err
    
    # Follow the chain to the end
    sus wrapped := error_unwrap(current)
    # For now, return current error
    
    damn current
}

slay error_format(err, format tea) tea {
    # Format error according to template
    sus err_check := err
    sus format_check := format
    
    sus err_msg := error_message(err)
    sus err_type := error_type(err)
    sus err_sev := error_severity(err)
    
    # Simple formatting - just return message for now
    damn err_msg
}

slay error_contains_type(err, target_type tea) lit {
    # Check if error chain contains specific error type
    sus err_check := err
    sus type_check := target_type
    
    sus current_type := error_type(err)
    
    # Simple type comparison - for now return false
    sus types_match lit = cap
    
    damn types_match
}
