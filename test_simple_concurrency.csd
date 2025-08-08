# Test basic concurrency without complex syntax
vibez.spill("Testing simple concurrency")

# Test 1: Multiple goroutines
stan {
    vibez.spill("First goroutine running")
}

stan {
    vibez.spill("Second goroutine running")
}

stan {
    vibez.spill("Third goroutine running")
}

vibez.spill("Main thread executing")
vibez.spill("All goroutines spawned")
