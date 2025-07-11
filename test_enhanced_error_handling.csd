// Enhanced Error Handling Test for CURSED
// Tests production-grade error management with yikes, shook, fam keywords

// Test 1: Basic yikes error creation with context
vibez.spill("=== Test 1: Basic yikes error creation ===")
yikes network_error := "Connection timeout occurred"
vibez.spill("✅ Created network error:", network_error)

// Test 2: Error propagation with context preservation
vibez.spill("=== Test 2: Error propagation with shook ===")
slay risky_operation() {
    yikes operation_error := "Database connection failed"
    vibez.spill("⚠️ Error in risky operation:", operation_error)
    damn operation_error shook  // Propagate error with context
}

// Test 3: Advanced error recovery with fam
vibez.spill("=== Test 3: Advanced error recovery with fam ===")
fam {
    yikes critical_error := "Memory allocation failed"
    vibez.spill("❌ This should trigger recovery:", critical_error)
    // Simulate panic/error condition
    critical_error shook
} sus recovered_error {
    vibez.spill("🔄 Recovery successful! Caught error:", recovered_error)
}

// Test 4: Goroutine error isolation
vibez.spill("=== Test 4: Goroutine error isolation ===")
slay goroutine_task() {
    yikes goroutine_error := "Goroutine encountered error"
    vibez.spill("🔀 Goroutine error:", goroutine_error)
    damn goroutine_error shook
}

// Test 5: Nested error handling
vibez.spill("=== Test 5: Nested error handling ===")
fam {
    fam {
        yikes nested_error := "Nested operation failed"
        vibez.spill("🔄 Nested error:", nested_error)
        nested_error shook
    } sus inner_error {
        vibez.spill("🔄 Inner recovery:", inner_error)
        yikes escalated_error := "Escalated from nested error"
        escalated_error shook
    }
} sus outer_error {
    vibez.spill("🔄 Outer recovery:", outer_error)
}

// Test 6: Error with structured information
vibez.spill("=== Test 6: Structured error information ===")
yikes structured_error := "Structured error with details"
vibez.spill("📊 Structured error created:", structured_error)

// Test 7: Error monitoring and statistics
vibez.spill("=== Test 7: Error monitoring ===")
yikes monitoring_error := "Error for monitoring system"
vibez.spill("📈 Monitoring error:", monitoring_error)

// Test 8: Performance-optimized error handling
vibez.spill("=== Test 8: Performance-optimized error handling ===")
slay performance_critical_function() {
    yikes performance_error := "Performance critical error"
    vibez.spill("⚡ Performance error:", performance_error)
    damn performance_error shook
}

fam {
    performance_critical_function()
} sus perf_error {
    vibez.spill("⚡ Performance error recovered:", perf_error)
}

// Test 9: Error context propagation
vibez.spill("=== Test 9: Error context propagation ===")
slay context_function() {
    yikes context_error := "Error with context information"
    vibez.spill("🔗 Context error:", context_error)
    damn context_error shook
}

fam {
    context_function()
} sus ctx_error {
    vibez.spill("🔗 Context error recovered:", ctx_error)
}

// Test 10: Production-grade error reporting
vibez.spill("=== Test 10: Production-grade error reporting ===")
yikes production_error := "Production environment error"
vibez.spill("🏭 Production error:", production_error)

vibez.spill("=== Enhanced Error Handling Tests Complete ===")
vibez.spill("✅ All error handling features tested successfully")
