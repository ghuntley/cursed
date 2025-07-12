// Test channel operations with properly created channel
sus ch := make(42)
vibez.spill("Channel created")

// Test channel send and receive (won't work due to blocking, but tests parsing)
// Note: These operations would block in real usage, but we're testing the parser
ch <- 123
vibez.spill("Send parsed (would block)")

sus value := <-ch
vibez.spill("Receive parsed (would block)")

// Test channel close
close(ch)
vibez.spill("Channel closed")
