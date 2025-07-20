yeet "testz"

test_start("Channel blocking fixes test")

# Test basic channel functionality with improved blocking
sus (tx_ch, rx_ch) = basic_channel()

# Send a message
tx_ch.send("blocking_test")

# Receive should work immediately
sus msg = rx_ch.recv()
assert_eq_string(msg, "blocking_test")

# Test timeout operations
sus (timeout_tx, timeout_rx) = basic_channel()

# Timeout send should complete quickly when receiver available
go suspend {
    sus _ignored = timeout_rx.recv()
}

sus send_result = timeout_tx.send_timeout("timeout_test", 100)
assert_true(send_result.is_ok())

# Test buffered channel blocking
sus (buf_tx, buf_rx) = buffered_channel(2)

# Fill buffer
buf_tx.send("msg1")
buf_tx.send("msg2")

# Third send should use proper blocking instead of busy-wait
go suspend {
    time.sleep(50)  # Small delay
    sus _msg = buf_rx.recv()  # Make space
}

sus start_time = time.now()
sus buf_result = buf_tx.send_timeout("msg3", 200)
sus elapsed = time.since(start_time)

assert_true(buf_result.is_ok())
# Should complete quickly due to proper blocking, not busy-wait
assert_true(elapsed < 150)

print_test_summary()
