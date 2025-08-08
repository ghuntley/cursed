vibez.spill("=== Goroutine Concurrency Test ===")

sus counter drip = 0

vibez.spill("Starting concurrent goroutines...")

stan {
    vibez.spill("Goroutine A: Starting work")
    vibez.spill("Goroutine A: Processing...")
    vibez.spill("Goroutine A: Completed")
}

stan {
    vibez.spill("Goroutine B: Starting work")
    vibez.spill("Goroutine B: Processing...")
    vibez.spill("Goroutine B: Completed")
}

stan {
    vibez.spill("Goroutine C: Starting work")
    vibez.spill("Goroutine C: Processing...")
    vibez.spill("Goroutine C: Completed")
}

vibez.spill("Main: All goroutines started")
vibez.spill("Main: Test completed")
vibez.spill("=== Test completed ===")
