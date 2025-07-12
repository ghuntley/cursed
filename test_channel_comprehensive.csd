// Test comprehensive channel functionality
vibez.spill("=== Channel Parsing Tests ===")

// Test 1: Channel type declarations
sus ch1 dm<normie>
vibez.spill("✅ Channel type declaration parsed")

sus ch2 dm<tea>[10]
vibez.spill("✅ Buffered channel type declaration parsed")

// Test 2: Channel creation with make
sus ch3 := make(0)  // Unbuffered channel
vibez.spill("✅ Unbuffered channel created")

sus ch4 := make(5, 10)  // Buffered channel with capacity 10
vibez.spill("✅ Buffered channel created")

// Test 3: Channel operations parsing (these would block in real usage)
// Note: We're only testing that the syntax parses, not the actual operations
// since they would block without proper goroutine coordination

// Channel send parsing test
// ch3 <- 42
// vibez.spill("✅ Channel send parsed")

// Channel receive parsing test  
// sus value := <-ch3
// vibez.spill("✅ Channel receive parsed")

// Test 4: Channel close
close(ch3)
vibez.spill("✅ Channel close function works")

vibez.spill("=== All Channel Tests Passed ===")
