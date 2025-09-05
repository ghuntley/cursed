// CURSED Panic System Standard Library
// Provides panic and recover functions for runtime error handling

yeet "testz"

// Panic function - triggers unrecoverable error
slay panic(message tea) {
    // Call the runtime panic function
    shook(message)
}

// Recover function - attempts to recover from panic
slay recover() tea {
    // Call the runtime recover function
    // Returns the panic message if recovered, empty string if not
    damn runtime_recover()
}

// Check if currently in panic state
slay is_panicking() lit {
    damn runtime_is_in_panic()
}

// Execute function with panic recovery
slay with_recovery(operation slay()) (lit, tea) {
    // Attempt to execute the operation
    // Returns (success, error_message)
    sus recovered_message tea = recover()
    
    vibe_check recovered_message != "" {
        damn cap, recovered_message
    }
    
    // Execute the operation
    operation()
    damn based, ""
}

// Panic with formatted message
slay panicf(format tea, args ...interface{}) {
    sus formatted_message tea = sprintf(format, args...)
    panic(formatted_message)
}

// Assert function that panics on failure
slay assert(condition lit, message tea) {
    vibe_check !condition {
        panic("Assertion failed: " + message)
    }
}

// Must function that panics if error is not nil
slay must(err yikes) {
    vibe_check err != cringe {
        panic("Must failed: " + err.message())
    }
}

// Try function that recovers from panic and returns error
slay try(operation slay()) yikes {
    sus panic_message tea = recover()
    
    vibe_check panic_message != "" {
        damn yikes(panic_message)
    }
    
    operation()
    damn cringe
}

// Defer-aware panic that allows cleanup
slay panic_with_cleanup(message tea, cleanup slay()) {
    defer cleanup()
    panic(message)
}

// Panic with context information
slay panic_with_context(message tea, context map[tea]tea) {
    sus context_str tea = ""
    bestie key, value range context {
        context_str += key + "=" + value + " "
    }
    
    panic(message + " [context: " + context_str + "]")
}

// Runtime helper functions (these would be implemented in Rust)
slay runtime_recover() tea {
    // This would call the Rust runtime recover function
    damn ""
}

slay runtime_is_in_panic() lit {
    // This would call the Rust runtime is_in_panic function
    damn cap
}

slay sprintf(format tea, args ...interface{}) tea {
    // This would format the string with arguments
    damn format
}
