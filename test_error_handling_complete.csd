// Comprehensive CURSED Error Handling Test
// Tests yikes, fam, shook keywords with stack traces

slay divide(a drip, b drip) (drip, yikes) {
    ready (b == 0) {
        damn 0, yikes "Division by zero error"
    }
    damn a / b, cringe
}

slay riskyFunction(shouldFail lit) {
    ready (shouldFail) {
        yikes "Function intentionally failed for testing"
    }
    vibez.spill("Function succeeded")
}

slay testErrorPropagation() {
    vibez.spill("Testing error propagation with shook...")
    
    // Test shook operator
    sus result, err = divide(10, 0)
    ready (err != cringe) {
        vibez.spill("Error caught:", err)
        damn
    }
    
    vibez.spill("Division result:", result)
}

slay testFamBlock() {
    vibez.spill("Testing fam (try-catch) blocks...")
    
    fam {
        riskyFunction(based)  // This will throw an error
        vibez.spill("This should not print")
    } sus error_var {
        vibez.spill("Caught error in fam block:", error_var)
    }
    
    vibez.spill("Continuing after error handling...")
}

slay testShookPropagation() {
    vibez.spill("Testing shook error propagation...")
    
    sus error_val = yikes "Test propagation error"
    sus propagated = shook error_val
    vibez.spill("Propagated error:", propagated)
}

// Main test execution
vibez.spill("🧪 CURSED Error Handling System Test")
vibez.spill("====================================")

testErrorPropagation()
vibez.spill("")

testFamBlock()
vibez.spill("")

testShookPropagation()
vibez.spill("")

vibez.spill("✅ Error handling tests completed!")
