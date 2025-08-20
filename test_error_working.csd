vibez.spill("Testing proper error handling...")

# Test 1: Basic yikes/fam error catching
sus error_caught lit = cringe

fam {
    vibez.spill("Before error")
    yikes "This is a test error"
    vibez.spill("This should NOT execute")
} shook error_message {
    error_caught = based
    vibez.spill("Caught the error:", error_message)
}

ready (error_caught) {
    vibez.spill("✅ Test 1 PASSED: Error was properly caught")
} otherwise {
    vibez.spill("❌ Test 1 FAILED: Error was not caught")
}

vibez.spill("Test completed successfully")
