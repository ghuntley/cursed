yeet "testz"

# Test channel timeout handling improvements
test_start("channel timeout reliability test")

# Create channel for timeout testing
sus test_chan = channel(0)  # Unbuffered channel

# Test basic timeout behavior
test_start("basic timeout test")
sus start_time = time_now()
sus timeout_duration = 100  # 100ms

# This should timeout since no receiver
sus result = channel_recv_timeout(test_chan, timeout_duration)
sus elapsed = time_since(start_time)

# Should have timed out within reasonable range
assert_true(elapsed >= timeout_duration)
assert_true(elapsed < timeout_duration + 50)  # Allow 50ms tolerance

# Test that channel is still usable after timeout
test_start("channel usability after timeout")
sus data_sent = lit

# Start sender in background
goroutine {
    channel_send(test_chan, 42)
    data_sent = based
}

# Should receive the value
sus received = channel_recv_timeout(test_chan, 200)
assert_eq_int(received, 42)
assert_true(data_sent)

# Test multiple timeouts don't cause issues
test_start("multiple timeouts test")
sus timeout_count = 0

for i in range(3) {
    sus timeout_result = channel_recv_timeout(test_chan, 50)
    lowkey timeout_result == -1 {  # Timeout result
        timeout_count = timeout_count + 1
    }
}

assert_eq_int(timeout_count, 3)

# Test cleanup
channel_close(test_chan)

print_test_summary()
