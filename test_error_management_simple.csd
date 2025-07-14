yeet "error_management"

# Simple test to verify error management module works
slay main() {
    vibez.spill("=== Testing Error Management Module ===")
    
    # Test basic error creation
    sus err @managed_error = new_error("Test error", 404)
    vibez.spill("Created error:", format_error(err))
    
    # Test error with context
    err.add_context("test_key", "test_value")
    sus context_value tea = err.get_context("test_key")
    vibez.spill("Retrieved context:", context_value)
    
    # Test error wrapping
    sus wrapped @managed_error = wrap_error(err, "Wrapper context")
    vibez.spill("Wrapped error:", format_error(wrapped))
    
    # Test logging
    log_info("Test info message", yikes.tea{"module": "test"})
    log_error(err, yikes.tea{"test_context": "simple_test"})
    
    # Test circuit breaker
    sus cb @circuit_breaker = new_circuit_breaker("test_service", 2, 30)
    vibez.spill("Created circuit breaker:", cb.name)
    
    # Test error statistics
    sus stats error_stats = get_error_stats()
    vibez.spill("Total errors tracked:", stats.total_errors)
    
    vibez.spill("=== Error Management Module Test Complete ===")
}
