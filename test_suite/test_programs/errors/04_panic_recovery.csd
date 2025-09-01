vibe main
yeet "vibez"

// Test panic scenarios and recovery mechanisms
slay cause_panic() {
    shook("Critical system failure - panic triggered")
}

slay safe_operation_with_recovery() yikes {
    fam {
        vibez.spill("Starting potentially dangerous operation...")
        cause_panic()
        vibez.spill("This line should not execute")
        damn cringe
    } sus panic_value {
        vibez.spill("Panic recovered successfully:", panic_value)
        damn yikes("Operation failed due to panic")
    }
}

slay main() {
    vibez.spill("=== Panic Recovery Tests ===")
    
    // Test 1: Basic panic recovery
    fam {
        safe_operation_with_recovery()
    } sus error {
        vibez.spill("Handled converted panic:", error.message())
    }
    
    // Test 2: Direct panic handling
    fam {
        vibez.spill("About to trigger direct panic")
        shook("Direct panic test")
    } sus panic_val {
        vibez.spill("Direct panic caught:", panic_val)
    }
    
    // Test 3: Nested panic scenarios
    fam {
        fam {
            vibez.spill("Nested operation starting")
            shook("Nested panic")
        } sus inner_panic {
            vibez.spill("Inner panic handled:", inner_panic)
            shook("Re-panic from handler")
        }
    } sus outer_panic {
        vibez.spill("Outer panic handled:", outer_panic)
    }
    
    vibez.spill("Panic recovery tests completed successfully")
}
