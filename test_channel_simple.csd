# Test basic channel operations without complex syntax  
vibez.spill("Testing basic channel concept")

# Test producer-consumer pattern with goroutines
sus shared_value drip = 0

stan {
    vibez.spill("Producer goroutine starting")
    shared_value = 42
    vibez.spill("Producer set value to:", shared_value)
}

stan {
    vibez.spill("Consumer goroutine starting")
    vibez.spill("Consumer reading value:", shared_value)
}

vibez.spill("Main thread coordinating")
vibez.spill("Channel concept test complete")
