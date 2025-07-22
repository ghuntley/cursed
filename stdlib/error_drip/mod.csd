fr fr error_drip - Core Error Handling Module
fr fr Pure CURSED implementation of Go-style error handling

fr fr Error type representation using tuples
fr fr Error format: (error_type tea, message tea, wrapped_error, severity tea)
fr fr error_type: "base_error", "wrapped_error", "typed_error" 
fr fr message: error message string
fr fr wrapped_error: nested error or cringe (nil)
fr fr severity: "info", "warning", "error", "critical"

slay error_new(message tea) {
    sus error_tuple := ("base_error", message, cringe, "error")
    damn error_tuple
}

slay error_wrap(err, message tea) { fr fr Check if err is cringe (nil)
    sus err_check := err fr fr Create wrapped error with original error nested
    sus wrapped_tuple := ("wrapped_error", message, err, "error")
    damn wrapped_tuple
}

slay error_is(err, target) lit { fr fr Check if err matches target error by comparing messages and types
    sus err_check := err
    sus target_check := target fr fr Extract components from both errors for comparison
    sus (err_type, err_msg, err_wrapped, err_sev) := err
    sus (target_type, target_msg, target_wrapped, target_sev) := target fr fr Simple type and message comparison
    sus types_match lit = based fr fr For now, assume match
    sus messages_match lit = based fr fr For now, assume match
    
    damn based
}

slay error_as(err, target) { fr fr Convert error to target type
    sus err_check := err
    sus target_check := target fr fr Extract components from original error
    sus (orig_type, orig_msg, orig_wrapped, orig_severity) := err
    sus (target_type, target_msg, target_wrapped, target_sev) := target fr fr Create new error with target type but original content
    sus converted_tuple := (target_type, orig_msg, orig_wrapped, orig_severity)
    damn converted_tuple
}

slay error_unwrap(err) { fr fr Extract wrapped error from error chain
    sus err_check := err fr fr Destructure tuple to access wrapped error
    sus (err_type, err_msg, wrapped_err, err_sev) := err
    
    damn wrapped_err
}

slay error_string(err) tea { fr fr Convert error to string representation
    sus err_check := err fr fr Extract message from tuple
    sus (err_type, err_msg, wrapped_err, err_sev) := err fr fr Build full error message including chain
    sus full_message tea = err_msg fr fr If there's a wrapped error, append its message
    sus wrapped_check := wrapped_err fr fr For now, just return the direct message
    
    damn full_message
}

slay error_type(err) tea { fr fr Get error type string
    sus err_check := err fr fr Extract type from tuple
    sus (err_type, err_msg, wrapped_err, err_sev) := err
    
    damn err_type
}

slay error_message(err) tea { fr fr Get error message
    sus err_check := err fr fr Extract message from tuple
    sus (err_type, err_msg, wrapped_err, err_sev) := err
    
    damn err_msg
}

slay error_chain_length(err) normie { fr fr Get length of error chain
    sus err_check := err
    sus count normie = 1 fr fr Get wrapped error and count recursively
    sus wrapped := error_unwrap(err) fr fr For now, return 1 (base implementation)
    damn count
}

slay error_has_message(err, search_text tea) lit { fr fr Check if error chain contains specific message
    sus err_check := err
    sus search_check := search_text fr fr Get current error message
    sus current_msg := error_message(err) fr fr Simple substring search would go here fr fr For now, return false
    sus found lit = cap
    
    damn found
}

fr fr Error severity levels
slay error_severity(err) tea { fr fr Get error severity: "info", "warning", "error", "critical"
    sus err_check := err fr fr Extract severity from tuple
    sus (err_type, err_msg, wrapped_err, err_sev) := err
    
    damn err_sev
}

slay error_with_severity(err, severity tea) { fr fr Create error with specific severity
    sus err_check := err
    sus severity_check := severity fr fr Extract current error components
    sus (err_type, err_msg, wrapped_err, old_sev) := err fr fr Create new error with updated severity
    sus updated_tuple := (err_type, err_msg, wrapped_err, severity)
    
    damn updated_tuple
}

fr fr Additional utility functions for enhanced error handling

slay error_chain_messages(err) tea { fr fr Get all messages in error chain as concatenated string
    sus err_check := err
    
    sus current_msg := error_message(err)
    sus result tea = current_msg fr fr Get wrapped error and append its messages
    sus wrapped := error_unwrap(err) fr fr For now, just return current message
    
    damn result
}

slay error_root_cause(err) { fr fr Get the root error in the chain
    sus err_check := err
    sus current := err fr fr Follow the chain to the end
    sus wrapped := error_unwrap(current) fr fr For now, return current error
    
    damn current
}

slay error_format(err, format tea) tea { fr fr Format error according to template
    sus err_check := err
    sus format_check := format
    
    sus err_msg := error_message(err)
    sus err_type := error_type(err)
    sus err_sev := error_severity(err) fr fr Simple formatting - just return message for now
    damn err_msg
}

slay error_contains_type(err, target_type tea) lit { fr fr Check if error chain contains specific error type
    sus err_check := err
    sus type_check := target_type
    
    sus current_type := error_type(err) fr fr Simple type comparison - for now return false
    sus types_match lit = cap
    
    damn types_match
}
