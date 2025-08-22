# CURSED Concurrency Test
# Testing goroutines (stan) and channels (dm)

# Test basic goroutine
slay worker(id drip) {
    spill("Worker", id, "starting")
    # Simulate work
    bestie (sus i drip = 0; i < 3; i = i + 1) {
        spill("Worker", id, "step", i)
    }
    spill("Worker", id, "finished")
}

# Test channel communication
slay channel_sender(ch chan<drip>, id drip) {
    bestie (sus i drip = 0; i < 3; i = i + 1) {
        sus value drip = id * 10 + i
        ch <- value
        spill("Sender", id, "sent:", value)
    }
}

slay channel_receiver(ch chan<drip>, id drip) {
    bestie (sus i drip = 0; i < 3; i = i + 1) {
        sus value drip = <-ch
        spill("Receiver", id, "got:", value)
    }
}

spill("=== CONCURRENCY TEST ===")

# Test basic goroutines
spill("Testing basic goroutines:")
stan worker(1)
stan worker(2)

# Wait a bit (simplified)
bestie (sus wait drip = 0; wait < 1000000; wait = wait + 1) {
    # Simple busy wait
}

# Test channels
spill("Testing channels:")
sus ch chan<drip> = make_channel()
stan channel_sender(ch, 1)
stan channel_receiver(ch, 1)

# Wait for completion
bestie (sus wait2 drip = 0; wait2 < 1000000; wait2 = wait2 + 1) {
    # Simple busy wait
}

# Test select statement if implemented
# select {
#     case value := <-ch1:
#         spill("Got from ch1:", value)
#     case ch2 <- 42:
#         spill("Sent to ch2")
#     default:
#         spill("No channel ready")
# }

spill("Concurrency test completed")
