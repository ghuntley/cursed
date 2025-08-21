# CURSED Complete Concurrency Integration Test
# Tests goroutines, channels, and select statements

# Test basic goroutine spawning
stan {
    vibez.spill("Goroutine 1 executing")
}

stan {
    vibez.spill("Goroutine 2 executing") 
}

# Test buffered channel operations
sus ch dm<drip>[5] = make_channel(drip, 5)

# Test sending to channel
dm_send(ch, 42)
dm_send(ch, 43)
dm_send(ch, 44)

# Test receiving from channel
sus val1 drip = dm_recv(ch)
sus val2 drip = dm_recv(ch)
sus val3 drip = dm_recv(ch)

vibez.spill("Received values:", val1, val2, val3)

# Test goroutine with channel communication
stan {
    dm_send(ch, 100)
    vibez.spill("Goroutine sent 100 to channel")
}

stan {
    sus received drip = dm_recv(ch) 
    vibez.spill("Goroutine received:", received)
}

# Test select statement
sus ch1 dm<drip>[1] = make_channel(drip, 1)
sus ch2 dm<drip>[1] = make_channel(drip, 1)

stan {
    dm_send(ch1, 200)
}

stan {
    dm_send(ch2, 300)
}

# Select from multiple channels
ready {
    case dm_recv(ch1) -> sus val drip:
        vibez.spill("Received from ch1:", val)
    case dm_recv(ch2) -> sus val drip:
        vibez.spill("Received from ch2:", val)
    default:
        vibez.spill("No channel ready")
}

# Test channel closing
dm_close(ch1)
dm_close(ch2)
dm_close(ch)

vibez.spill("Concurrency test completed successfully")
