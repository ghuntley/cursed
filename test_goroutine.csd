slay test_goroutine() {
    stan {
        vibez.spill("Hello from goroutine!")
    }
    vibez.spill("Main thread")
}

test_goroutine()
