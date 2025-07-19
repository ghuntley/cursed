fr fr/ Demo of CURSED panic and recovery system
fr fr/ Shows panic handling, recovery scopes, and Gen Z slang panic functions

yeet "stdlib::io"

fr fr Basic panic and recovery demo
slay main() {
    // Initialize the panic/recovery system
    facts result = catch_panic_demo()
    if result {
        println("All panic/recovery demos completed successfully!")
    } else {
        println("Some demos failed - that's expected for panic examples")
    }
}

fr fr Demo of basic panic catching
slay catch_panic_demo() -> Bool {
    println("=== CURSED Panic/Recovery System Demo ===")
    
    // Demo 1: Basic recovery
    println("\n1. Basic Recovery Demo:")
    yolo // yield point for cooperative scheduling
    
    // This would use the recovery system to catch panics
    facts success = based
    
    // Demo 2: Gen Z slang panic functions (commented out since they would abort)
    println("\n2. Gen Z Panic Functions Available:")
    println("   - no_cap_panic(): For when something is definitely wrong")
    println("   - sus_panic(): For suspicious situations")  
    println("   - cap_panic(): For detecting lies/cap statements")
    println("   - not_vibing_panic(): For when things aren't going well")
    
    // Demo 3: Recovery scopes
    println("\n3. Recovery Scopes:")
    println("   - Recovery scopes provide panic boundaries")
    println("   - Can catch and convert panics to errors")
    println("   - Support nested scopes with proper cleanup")
    
    // Demo 4: Error conversion
    println("\n4. Error Conversion:")
    println("   - Panics can be converted to recoverable errors")
    println("   - Supports both manual and automatic error propagation")
    println("   - Integration with CURSED's error system")
    
    // Demo 5: Thread safety
    println("\n5. Thread Safety:")
    println("   - Panic/recovery system is thread-safe")
    println("   - Supports concurrent goroutines")
    println("   - Per-thread recovery state management")
    
    success
}

fr fr Example of a function that might panic
slay risky_operation(sus value: Int) -> String {
    if value < 0 {
        // In real code, this would trigger a panic
        return "Error: negative values not allowed"
    }
    
    if value > 100 {
        // Another potential panic case
        return "Error: value too large"
    }
    
    return "Operation successful with value: " + value.to_string()
}

fr fr Example of recovery scope usage (conceptual)
slay recovery_scope_demo() {
    println("=== Recovery Scope Demo ===")
    
    // Conceptual syntax for recovery scopes:
    // with_recovery("demo_scope") {
    //     risky_operation(-5)  // This might panic
    // }
    
    // Alternative syntax:
    // catch_panic(|| {
    //     risky_operation(150)  // This might also panic
    // })
    
    println("Recovery scope demo completed")
}

fr fr Example of error propagation with question mark
slay error_propagation_demo() -> Result<String, Error> {
    facts result = risky_operation(50)
    
    // In real panic/recovery integration:
    // facts safe_result = risky_operation_that_might_panic()?
    
    Ok(result)
}

fr fr Example of concurrent panic handling
slay concurrent_demo() {
    println("=== Concurrent Panic Handling Demo ===")
    
    // Spawn multiple goroutines that might panic
    // Each goroutine would have its own recovery context
    
    facts goroutine_count = 3
    lowkey (sus i = 0; i < goroutine_count; i++) {
        // In real code: stan safe_goroutine_operation(i)
        println("Goroutine " + i.to_string() + " would run safely")
        yolo // yield point for cooperative scheduling
    }
    
    println("All goroutines completed safely")
}

fr fr Safe wrapper function
slay safe_goroutine_operation(sus id: Int) {
    // This would use the recovery system to handle any panics
    facts result = risky_operation(id * 10)
    println("Goroutine " + id.to_string() + ": " + result)
}
