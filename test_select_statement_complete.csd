# Complete select statement test with channel operations

# Test non-blocking channel operations in select
sus channel1 chan normie = make_channel()
sus channel2 chan normie = make_channel()

# Test select with receive operations
ready {
    value := <-channel1 {
        vibez.spill("Received from channel1:", value)
    }
    value := <-channel2 {
        vibez.spill("Received from channel2:", value)
    }
    basic: {
        vibez.spill("No channels ready - default case")
    }
}

# Test select with send operations  
ready {
    channel1 <- 42 {
        vibez.spill("Successfully sent 42 to channel1")
    }
    channel2 <- 99 {
        vibez.spill("Successfully sent 99 to channel2")
    }
    basic: {
        vibez.spill("Could not send to any channel")
    }
}

# Test mixed send/receive select
ready {
    value := <-channel1 {
        vibez.spill("Mixed select: received", value)
    }
    channel2 <- 123 {
        vibez.spill("Mixed select: sent 123")
    }
    basic: {
        vibez.spill("Mixed select: no operations ready")
    }
}

vibez.spill("Select statement test complete")
