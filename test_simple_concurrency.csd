// Simple CURSED Concurrency Test
// Tests basic goroutine and channel functionality

yeet "testz"

// Test basic goroutine spawning
test_start("Basic goroutine spawning")
sus result drip = 0

// Spawn a goroutine
stan {
    result = 42
}

// Give goroutine time to execute
yolo

assert_eq_int(result, 42)

// Test channel communication
test_start("Channel communication")
sus channel_value drip = 0

// Create channel
dm ch := make_channel(drip)

// Send value in goroutine
stan {
    ch <- 100
}

// Receive value
channel_value = <- ch

assert_eq_int(channel_value, 100)

// Test multiple goroutines
test_start("Multiple goroutines")
sus counter drip = 0

// Spawn multiple goroutines
stan {
    counter = counter + 1
}

stan {
    counter = counter + 2
}

stan {
    counter = counter + 3
}

// Yield to allow execution
yolo
yolo
yolo

assert_eq_int(counter, 6)

// Test buffered channel
test_start("Buffered channel")
sus buffered_result drip = 0

// Create buffered channel
dm buffered_ch := make_buffered_channel(drip, 3)

// Send without blocking
buffered_ch <- 1
buffered_ch <- 2
buffered_ch <- 3

// Receive all values
sus val1 drip = <- buffered_ch
sus val2 drip = <- buffered_ch
sus val3 drip = <- buffered_ch

buffered_result = val1 + val2 + val3

assert_eq_int(buffered_result, 6)

// Test simple select
test_start("Simple select")
sus select_result drip = 0

dm ch1 := make_channel(drip)
dm ch2 := make_channel(drip)

// Send to first channel
stan {
    ch1 <- 99
}

// Select from channels
ready {
    case val := <- ch1:
        select_result = val
    case val := <- ch2:
        select_result = val * 2
}

assert_eq_int(select_result, 99)

print_test_summary()
