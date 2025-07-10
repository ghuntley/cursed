// Simple channel test
vibez.spill("Testing basic channel functionality...")

// Create a channel
sus ch chan normie

// Test basic channel operations (without select for now)
// This should work with the existing channel implementation
ch <- 42

sus value := <-ch
vibez.spill("Received value:", value)

vibez.spill("Channel test complete")
