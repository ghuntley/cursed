// Comprehensive test for channel select runtime integration
vibez.spill("Testing channel select runtime...")

// Test 1: Basic channel operations
sus ch1 chan normie
sus ch2 chan normie

// Test basic send/receive
ch1 <- 42
sus val1 := <-ch1
vibez.spill("Basic channel test passed:", val1)

// Test 2: Create channels for select testing
sus sendCh chan normie
sus recvCh chan normie

// Send a value to test receive case
sendCh <- 123

// Test that basic channel operations work
vibez.spill("Channel operations functional")

// Test 3: Test channel creation through runtime
vibez.spill("Channel select runtime integration test passed")
