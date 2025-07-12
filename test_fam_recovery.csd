// Test fam recovery specifically
vibez.spill("Testing fam recovery")

// Test 1: Simple recovery block
sus recovery_worked lit = cap
fam {
    yikes panic_error := "Test panic"
    vibez.spill("This should be caught")
} sus caught {
    vibez.spill("Caught in recovery:", caught)
    recovery_worked = based
}

vibez.spill("Recovery worked:", recovery_worked)

// Test 2: Recovery with cleanup
sus cleanup_ran lit = cap
fam {
    defer {
        cleanup_ran = based
        vibez.spill("Cleanup executed")
    }
    yikes cleanup_error := "Error with cleanup"
    vibez.spill("This should trigger cleanup")
} sus caught_cleanup {
    vibez.spill("Caught cleanup error:", caught_cleanup)
}

vibez.spill("Cleanup ran:", cleanup_ran)
vibez.spill("Fam recovery test completed")
