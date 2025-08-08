// Test deep nesting to verify scope stack overflow protection
yeet "testz"

slay deep_nesting_test() {
    vibez.spill("Testing deep scope nesting...")
    
    // Create nested scopes with defer statements to test bounds checking
    sus level drip = 0
    
    bestie (level < 50) {
        defer { vibez.spill("Cleanup level:", level) }
        
        ready (level < 25) {
            defer { vibez.spill("Inner cleanup:", level) }
            
            ready (level < 10) {
                defer { vibez.spill("Deep cleanup:", level) }
                vibez.spill("Processing level:", level)
            }
        }
        
        level = level + 1
    }
    
    vibez.spill("Deep nesting test completed")
}

test_start("Scope Stack Overflow Protection")
deep_nesting_test()
print_test_summary()
