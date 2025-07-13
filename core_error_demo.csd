yeet "core"
yeet "error_core"

# Demo: Core Runtime with Advanced Error Handling

vibez.spill("=== CURSED Core Runtime & Error Handling Demo ===")

# Initialize core runtime
vibez.spill("\n1. Initializing Core Runtime...")
sus init_result lit = runtime_init()
lowkey init_result == based {
    vibez.spill("✓ Core runtime initialized successfully")
} else {
    vibez.spill("✗ Core runtime initialization failed")
}

vibez.spill("Runtime info: " + core_info())

# Demonstrate type conversions with error handling
vibez.spill("\n2. Type Conversions with Error Handling...")
sus number_str tea = "42"
sus converted_int normie = to_int(number_str)
vibez.spill("Converted '" + number_str + "' to integer: " + to_string(converted_int))

sus float_str tea = "3.14"
sus converted_float meal = to_float(float_str)
vibez.spill("Converted '" + float_str + "' to float")

# Error handling demonstration
vibez.spill("\n3. Error Handling with yikes/shook/fam...")

# Create an error using yikes pattern
sus validation_error = yikes_validation("Invalid user input detected")
vibez.spill("Created validation error using yikes pattern")

# Wrap error using shook pattern
sus wrapped_error = shook_wrap(validation_error, "During form processing")
vibez.spill("Wrapped error using shook pattern")

# Handle error using fam pattern
sus handled_result = fam_handle(wrapped_error, "Using default safe value")
vibez.spill("Handled error using fam pattern: " + handled_result)

# Safe operations demonstration
vibez.spill("\n4. Safe Operations...")

# Safe division
sus safe_result = safe_divide(100, 5)
lowkey is_error(safe_result) {
    vibez.spill("Division failed: " + error_message(safe_result))
} else {
    vibez.spill("Safe division successful: " + to_string(safe_result))
}

# Unsafe division (divide by zero)
sus unsafe_result = safe_divide(100, 0)
lowkey is_error(unsafe_result) {
    vibez.spill("Division by zero caught: " + error_message(unsafe_result))
    sus recovered_value = fam_handle(unsafe_result, 999)
    vibez.spill("Recovered with fallback value: " + to_string(recovered_value))
}

# Memory operations with error checking
vibez.spill("\n5. Memory Operations...")
sus mem_alloc lit = memory_allocate(1024)
lowkey mem_alloc == based {
    vibez.spill("✓ Memory allocation successful")
} else {
    sus mem_error = yikes_memory("Memory allocation failed")
    vibez.spill("✗ Memory allocation failed")
    fam_ignore(mem_error)  # Ignore non-critical memory error
}

# Data processing with error recovery
vibez.spill("\n6. Data Processing with Recovery...")
sus test_data tea = "important_data"
sus processed_data tea = safe_process(test_data)
vibez.spill("Processed data: " + processed_data)

# Error statistics
vibez.spill("\n7. Error Statistics...")
sus stats tea = error_stats()
vibez.spill("Error statistics: " + stats)

# Cleanup and finalization
vibez.spill("\n8. Cleanup...")
clear_errors()
vibez.spill("Errors cleared")

sus final_stats tea = error_stats()
vibez.spill("Final error statistics: " + final_stats)

vibez.spill("\n=== Demo Complete: Core + Error Handling Working ===")
