# Channel Race Condition Test - Reproduce Memory Leaks
# This test creates scenarios where goroutines exit before receivers, causing potential memory leaks

yeet "concurrenz"
yeet "testz"

test_start("Channel Race Condition Test")

# Test 1: Goroutines terminating before channel receivers are ready
slay race_condition_test_1() drip {
    sus ch dm<drip> = make_channel(0)  # Unbuffered channel
    
    # Start sender goroutine that terminates quickly
    stan {
        send_to_channel(ch, 42)
        vibez.spill("Sender goroutine completed")
    }
    
    # Artificial delay before receiver starts
    sleep(100)  # 100ms delay
    
    # Start receiver goroutine 
    stan {
        sus value drip = receive_from_channel(ch)
        vibez.spill("Received:", value)
    }
    
    # Close channel without proper cleanup coordination
    dm_close(ch)
    
    damn 1
}

# Test 2: Multiple goroutines with shared channel, some exit early
slay race_condition_test_2() drip {
    sus ch dm<drip> = make_channel(10)  # Buffered channel
    sus count drip = 0
    
    # Start multiple sender goroutines
    bestie (count < 5) {
        stan {
            send_to_channel(ch, count * 10)
            # Some goroutines exit early without waiting
            ready (count % 2 == 0) {
                damn  # Early termination
            }
        }
        count = count + 1
    }
    
    # Receiver that may not consume all messages
    stan {
        sus received_count drip = 0
        bestie (received_count < 3) {  # Only receive 3 out of 5 messages
            sus value drip = receive_from_channel(ch)
            vibez.spill("Received:", value)
            received_count = received_count + 1
        }
    }
    
    # Close channel while messages may still be in buffer
    dm_close(ch)
    
    damn 1
}

# Test 3: Stress test with rapid goroutine creation/destruction
slay race_condition_stress_test() drip {
    sus ch dm<drip> = make_channel(1)
    sus iteration drip = 0
    
    bestie (iteration < 100) {  # Create 100 goroutines rapidly
        stan {
            ready (iteration % 2 == 0) {
                send_to_channel(ch, iteration)
            } otherwise {
                sus value drip = receive_from_channel(ch)
                vibez.spill("Stress received:", value)
            }
            # Goroutines exit immediately after single operation
        }
        iteration = iteration + 1
    }
    
    # Close channel without ensuring all goroutines have completed
    dm_close(ch)
    
    damn 1
}

# Run all race condition tests
sus result1 drip = race_condition_test_1()
assert_eq_int(result1, 1)

sus result2 drip = race_condition_test_2()
assert_eq_int(result2, 1)

sus result3 drip = race_condition_stress_test()
assert_eq_int(result3, 1)

print_test_summary()
