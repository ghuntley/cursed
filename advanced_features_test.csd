// Advanced CURSED features test
// Tests pattern matching, channels, defer, error propagation, and goroutines

// Pattern matching test
ready (x) {
    mood 1: vibez.spill("Pattern 1")
    mood 2: vibez.spill("Pattern 2")
    otherwise: vibez.spill("Default")
}

// Channel operations test
sus channel drip = dm_create()
dm_send(channel, 42)
sus result drip = dm_recv(channel)

// Defer statement test
later {
    vibez.spill("Cleanup executed")
}

// Error propagation test
sus risky_value drip = risky_operation()?

// Goroutine spawning test
stan worker_function()

vibez.spill("Advanced features compiled!")
