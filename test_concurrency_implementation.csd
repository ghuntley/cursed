sus ch = make_channel<drip>()

stan {
    ch <- 42
    vibez.spill("Sent value to channel")
}

stan {
    sus value drip = <-ch
    vibez.spill("Received value:", value)
}

stan {
    wait(100) fr fr Sleep for 100ms
    vibez.spill("Background goroutine finished")
}

wait_all() fr fr Wait for all goroutines to complete
vibez.spill("Main thread finished")
