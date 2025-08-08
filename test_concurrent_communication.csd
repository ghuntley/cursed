# Test 3: Goroutine communication through channels
yeet "concurrenz"

vibez.spill("Testing goroutine communication")

# Create a shared channel
sus shared_ch dm<drip> = make_channel(5)

# Goroutine that sends data
stan {
    vibez.spill("Producer goroutine started")
    send_channel(shared_ch, 1)
    send_channel(shared_ch, 2)
    send_channel(shared_ch, 3)
    vibez.spill("Producer finished sending")
}

# Goroutine that receives data
stan {
    vibez.spill("Consumer goroutine started")
    sus val1 drip = recv_channel(shared_ch)
    sus val2 drip = recv_channel(shared_ch) 
    sus val3 drip = recv_channel(shared_ch)
    vibez.spill("Consumer received:", val1, val2, val3)
}

vibez.spill("Main thread waiting...")
# Give goroutines time to execute
vibez.spill("Concurrent communication test complete")
