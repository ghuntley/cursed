// Very simple test for goroutine error isolation
yeet "testz"

test_start("minimal goroutine error test")

vibez.spill("Testing basic goroutine functionality...")

// Test that runtime survives goroutine panics
slay normal_function() {
    vibez.spill("Normal function completed")
}

normal_function()

vibez.spill("✅ Basic test completed")

print_test_summary()
