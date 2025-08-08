// Test concurrent operations to validate race condition fixes
yeet "concurrenz"

// Test 1: Channel stress test
sus ch = make_channel(drip, 10)

// Spawn multiple senders
sus i drip = 0
bestie (i < 5) {
    stan {
        sus j drip = 0
        bestie (j < 100) {
            send_channel(ch, j)
            j = j + 1
        }
    }
    i = i + 1
}

// Spawn multiple receivers  
sus k drip = 0
bestie (k < 3) {
    stan {
        sus count drip = 0
        bestie (count < 50) {
            sus value = recv_channel(ch)
            ready (value != -1) {
                vibez.spill("Received:", value)
                count = count + 1
            }
        }
    }
    k = k + 1
}

// Wait for operations to complete
sus timeout drip = 0
bestie (timeout < 100) {
    timeout = timeout + 1
}

vibez.spill("Concurrent test completed")
