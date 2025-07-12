vibez.spill("Testing runtime execution features")

# Test basic goroutine creation
yolo vibez.spill("Hello from goroutine")

# Test channel operations
sus channel dm normie
channel <- 42
sus value := <-channel
vibez.spill("Received from channel: ", value)

# Test defer statement
defer vibez.spill("Defer executed")
vibez.spill("Before defer")

# Test tuple creation and access
sus tuple := (1, 2, 3)
vibez.spill("Tuple first element: ", tuple.0)
vibez.spill("Tuple second element: ", tuple.1)

vibez.spill("Runtime test complete")
