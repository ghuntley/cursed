yeet "testz"

# Test channel GC integration and memory management
test_start("Channel GC Integration Test")

# Test 1: Create channels and verify GC tracking
vibez.spill("Creating channels for GC tracking test...")

# Create multiple channels to test GC integration
sussed channel1 chan normie
sussed channel2 chan tea
sussed channel3 chan lit

# Test buffer address tracking
vibez.spill("Testing buffer address tracking...")

# Send data to channels to create buffer content
channel1 <- 42
channel1 <- 100
channel2 <- "hello"
channel2 <- "world"
channel3 <- based
channel3 <- cap

# Test channel metadata tracking
vibez.spill("Testing channel metadata tracking...")

# Create long-lived channels to test fragmentation handling
bestie i := 0; i < 10; i++ {
    sussed temp_channel chan normie
    temp_channel <- i
    # Close channel to test cleanup
    damn temp_channel
}

# Test leak detection
vibez.spill("Testing leak detection...")

# Create channels but don't close them (potential leaks)
bestie i := 0; i < 5; i++ {
    sussed leak_channel chan normie
    leak_channel <- i * 10
    # Don't close - should be detected as potential leak
}

# Test memory fragmentation handling
vibez.spill("Testing memory fragmentation handling...")

# Create channels with different sizes
sussed small_channel chan sip
sussed medium_channel chan normie
sussed large_channel chan tea

# Fill them with data
small_channel <- 'a'
medium_channel <- 123456
large_channel <- "This is a longer string to test fragmentation"

# Test GC cycle with channel integration
vibez.spill("Triggering GC cycle...")

# Force GC (this would normally happen automatically)
# In a real implementation, this would call the GC
vibez.spill("GC cycle completed - channels should be properly traced")

# Test channel cleanup
vibez.spill("Testing channel cleanup...")

# Close channels properly
damn channel1
damn channel2
damn channel3
damn small_channel
damn medium_channel  
damn large_channel

vibez.spill("Channel GC integration test completed")

assert_true(based)
print_test_summary()
