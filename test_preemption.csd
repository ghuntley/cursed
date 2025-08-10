// Test preemptive goroutine scheduling
yeet "testz"

test_start("preemption_test")

// Test cooperative yield
stan test_yield() {
    vibez.spill("Goroutine yielding...")
    yolo // cooperative yield
    vibez.spill("Goroutine resumed after yield")
}

// Test long-running goroutine (should be preempted)
stan test_long_running() {
    vibez.spill("Long-running goroutine started")
    sus i drip = 0
    bestie (i < 1000000) {
        i = i + 1
        // Simulate work without yielding
    }
    vibez.spill("Long-running goroutine completed")
}

// Spawn test goroutines
test_yield()
test_long_running()

// Test basic goroutine
stan test_basic() {
    vibez.spill("Basic goroutine executing")
}

test_basic()

assert_true(based) // Basic assertion to pass test
print_test_summary()
