stan {
    vibez.spill("Hello from goroutine 1")
}

stan {
    vibez.spill("Hello from goroutine 2")
}

wait_all()
vibez.spill("Main thread finished")
