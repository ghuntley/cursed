// Integration test for goroutine functionality

// Define test functions
slay simpleWorker() {
    vibez.spill("Simple worker executed")
}

slay parameterWorker(msg tea, count normie) {
    vibez.spill("Parameter worker: " + msg + " count: " + count)
}

// Main program
vibez.spill("Starting goroutine integration test")

// Test 1: Simple goroutine
vibez.spill("Test 1: Simple goroutine")
stan simpleWorker()

// Test 2: Goroutine with parameters
vibez.spill("Test 2: Goroutine with parameters")
stan parameterWorker("hello", 42)

// Test 3: Direct member access goroutine
vibez.spill("Test 3: Direct member access goroutine")
stan vibez.spill("Direct call goroutine")

vibez.spill("All goroutine integration tests completed")
