// Test for shook (try/catch) and fam (finally) blocks
// These should be advanced error handling constructs

// Test basic shook block
shook {
    sus risky_value drip = 42 / 0  // Should cause division by zero error
    vibez.spill("This should not execute")
} fam {
    vibez.spill("Error caught in fam block")
}

// Test shook with error propagation
slay risky_function() drip {
    shook {
        yikes "Something went wrong"
        damn 0
    } fam {
        vibez.spill("Handling error in function")
        damn -1
    }
}

// Test nested shook/fam blocks
shook {
    shook {
        yikes "Inner error"
    } fam {
        vibez.spill("Inner fam block")
        yikes "Re-throwing error"
    }
} fam {
    vibez.spill("Outer fam block caught re-thrown error")
}

vibez.spill("Error handling test complete")
