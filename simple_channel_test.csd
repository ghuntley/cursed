yeet "testz"

# Simple channel test
test_start("Simple Channel Test")

# Create a channel
sussed ch chan normie

# Send some data
ch <- 42
ch <- 100

# Receive data
received1 := <- ch
received2 := <- ch

# Verify data
vibez.spill("Received: ", received1, " and ", received2)

# Test passed
assert_true(received1 == 42)
assert_true(received2 == 100)

print_test_summary()
