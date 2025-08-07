// Simple pattern matching test
sus x drip = 5
vibez.spill("Testing value:", x)

// Basic literal patterns work in current implementation
ready (x == 5) {
    vibez.spill("matched five")
} otherwise {
    vibez.spill("no match")
}
