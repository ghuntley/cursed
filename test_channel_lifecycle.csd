// Test comprehensive channel lifecycle management
yeet "testz"

test_start("Channel Lifecycle Management Test")

// Test channel creation and basic operations
sus chan_tx dm<normie> = dm::<normie>()
sus chan_rx dm<normie> = dm::<normie>()

// Test channel send/receive
chan_tx <- 42
sus value normie = <-chan_rx
assert_eq_int(value, 42)

// Test buffered channel
sus buffered_tx dm<tea> = dm_buffered::<tea>(10)
sus buffered_rx dm<tea> = dm_buffered::<tea>(10)

// Test buffer operations
buffered_tx <- "hello"
buffered_tx <- "world"
sus msg1 tea = <-buffered_rx
sus msg2 tea = <-buffered_rx
assert_eq_string(msg1, "hello")
assert_eq_string(msg2, "world")

// Test channel lifecycle monitoring
vibez.spill("Channel lifecycle test completed")

print_test_summary()
