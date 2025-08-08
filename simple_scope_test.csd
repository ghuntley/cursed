// Simple scope overflow test
yeet "testz"

slay test_scopes() {
    vibez.spill("Testing scope stack")
    
    ready (based) {
        defer { vibez.spill("Outer defer") }
        
        ready (based) {
            defer { vibez.spill("Inner defer") }
            vibez.spill("Deep scope")
        }
    }
    
    vibez.spill("Scope test done")
}

test_start("Simple Scope Test")
test_scopes()
assert_true(based)
print_test_summary()
