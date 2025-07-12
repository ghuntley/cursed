// Test complete channel functionality
sus ch := make(dm<normie>)
vibez.spill("Channel created")

// Test channel send (in a separate test due to blocking)
// ch <- 42
// vibez.spill("Channel send completed")

// Test channel receive (in a separate test due to blocking)
// sus value := <-ch
// vibez.spill("Channel receive completed")

// Test channel close
close(ch)
vibez.spill("Channel closed")
