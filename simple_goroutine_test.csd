fr fr Test basic goroutine functionality
vibez.spill("Main thread starting")

stan {
    vibez.spill("Goroutine 1 executing")
}

stan {
    vibez.spill("Goroutine 2 executing")
}

vibez.spill("Main thread completed")
