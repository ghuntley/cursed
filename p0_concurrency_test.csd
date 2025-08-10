# Test basic concurrency without stdlib
sus counter drip = 0

# Simple goroutine test
go {
    counter = 42
}

# Give it time to execute
sus delay drip = 0
bestie (delay < 1000000) {
    delay = delay + 1
}

vibez.spill("Concurrency test - counter:", counter)
