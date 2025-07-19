yeet "testz"

// Basic goroutine functionality test
test_start("Basic goroutine test")

// Simple goroutine spawn test
sus test_result drip = 0

spawn {
    test_result = 42
}

// Let goroutine execute
yield()

assert_eq_int(test_result, 42)
print_test_summary()

vibez.spill("Basic goroutine test completed")
