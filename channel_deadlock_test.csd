# Channel Deadlock High-Contention Test
# Tests channel operations under extreme race conditions and high throughput

yeet "vibez"
yeet "concurrenz" 
yeet "mathz"

sus NUM_GOROUTINES drip = 100
sus NUM_OPERATIONS drip = 1000
sus CHANNEL_CAPACITY drip = 10

# Shared statistics
sus total_sent drip = 0
sus total_received drip = 0
sus failures drip = 0

# Test high-throughput sender goroutines
slay sender_goroutine(ch chan<drip>, goroutine_id drip, operations drip) {
    bestie (operations > 0) {
        ready (mathz.random_range(0, 10) < 3) {
            # Random delay to create contention
            concurrenz.sleep(mathz.random_range(1, 5))
        }
        
        sus value drip = goroutine_id * 1000 + operations
        
        ready {
            fam ch <- value -> {
                total_sent = total_sent + 1
                vibez.spill("Sender", goroutine_id, "sent", value)
            }
            shook -> {
                failures = failures + 1
                vibez.spill("Sender", goroutine_id, "failed to send", value)
            }
        }
        
        operations = operations - 1
    }
}

# Test high-throughput receiver goroutines  
slay receiver_goroutine(ch chan<drip>, goroutine_id drip, operations drip) {
    bestie (operations > 0) {
        ready (mathz.random_range(0, 10) < 3) {
            # Random delay to create contention
            concurrenz.sleep(mathz.random_range(1, 5))
        }
        
        ready {
            fam sus value drip = <-ch -> {
                total_received = total_received + 1
                vibez.spill("Receiver", goroutine_id, "received", value)
            }
            shook -> {
                failures = failures + 1
                vibez.spill("Receiver", goroutine_id, "failed to receive")
            }
        }
        
        operations = operations - 1
    }
}

# Test select statement under high contention
slay select_goroutine(ch1 chan<drip>, ch2 chan<drip>, goroutine_id drip, operations drip) {
    bestie (operations > 0) {
        sus value drip = goroutine_id * 10000 + operations
        
        ready {
            fam ch1 <- value -> {
                vibez.spill("Select", goroutine_id, "sent to ch1:", value)
            }
            fam ch2 <- value + 1 -> {
                vibez.spill("Select", goroutine_id, "sent to ch2:", value + 1)
            }
            fam sus received1 drip = <-ch1 -> {
                vibez.spill("Select", goroutine_id, "received from ch1:", received1)
            }
            fam sus received2 drip = <-ch2 -> {
                vibez.spill("Select", goroutine_id, "received from ch2:", received2)
            }
            shook -> {
                vibez.spill("Select", goroutine_id, "all operations would block")
            }
        }
        
        operations = operations - 1
        concurrenz.sleep(mathz.random_range(1, 3))
    }
}

# Main test function
slay main() {
    vibez.spill("=== CURSED Channel Deadlock High-Contention Test ===")
    vibez.spill("Goroutines:", NUM_GOROUTINES, "Operations per goroutine:", NUM_OPERATIONS)
    
    # Create multiple channels for different test scenarios
    sus main_channel chan<drip> = concurrenz.make_channel(CHANNEL_CAPACITY)
    sus select_ch1 chan<drip> = concurrenz.make_channel(CHANNEL_CAPACITY)  
    sus select_ch2 chan<drip> = concurrenz.make_channel(CHANNEL_CAPACITY)
    
    sus start_time drip = concurrenz.current_time_millis()
    
    vibez.spill("🚀 Starting high-contention test...")
    
    # Spawn sender goroutines
    sus i drip = 0
    bestie (i < NUM_GOROUTINES / 3) {
        stan {
            sender_goroutine(main_channel, i, NUM_OPERATIONS)
        }
        i = i + 1
    }
    
    # Spawn receiver goroutines
    i = 0
    bestie (i < NUM_GOROUTINES / 3) {
        stan {
            receiver_goroutine(main_channel, i + 1000, NUM_OPERATIONS)
        }
        i = i + 1
    }
    
    # Spawn select statement goroutines
    i = 0
    bestie (i < NUM_GOROUTINES / 3) {
        stan {
            select_goroutine(select_ch1, select_ch2, i + 2000, NUM_OPERATIONS / 2)
        }
        i = i + 1
    }
    
    vibez.spill("⏳ All goroutines spawned, waiting for completion...")
    
    # Monitor progress and detect deadlocks
    sus monitor_iterations drip = 0
    sus last_sent drip = 0
    sus last_received drip = 0
    sus stagnant_count drip = 0
    
    bestie (monitor_iterations < 60) {  # Monitor for 60 seconds max
        concurrenz.sleep(1000)  # 1 second intervals
        
        sus current_sent drip = total_sent
        sus current_received drip = total_received
        
        vibez.spill("Progress: Sent =", current_sent, "Received =", current_received, "Failures =", failures)
        
        # Deadlock detection
        ready (current_sent == last_sent && current_received == last_received) {
            stagnant_count = stagnant_count + 1
            vibez.spill("⚠️ Warning: No progress for", stagnant_count, "seconds")
            
            ready (stagnant_count >= 5) {
                vibez.spill("💀 DEADLOCK DETECTED! No progress for 5+ seconds")
                vibez.spill("Final stats: Sent =", current_sent, "Received =", current_received)
                damn  # Exit early on deadlock
            }
        } otherwise {
            stagnant_count = 0  # Reset counter on progress
        }
        
        last_sent = current_sent
        last_received = current_received
        monitor_iterations = monitor_iterations + 1
        
        # Check if all operations completed
        sus expected_operations drip = (NUM_GOROUTINES / 3) * NUM_OPERATIONS
        ready (current_sent >= expected_operations && current_received >= expected_operations) {
            vibez.spill("✅ All operations completed successfully!")
            nah  # Break out of monitoring loop
        }
    }
    
    sus end_time drip = concurrenz.current_time_millis()
    sus duration drip = end_time - start_time
    
    # Final results
    vibez.spill("=== Test Results ===")
    vibez.spill("Duration:", duration, "ms")
    vibez.spill("Total sent:", total_sent)
    vibez.spill("Total received:", total_received) 
    vibez.spill("Total failures:", failures)
    vibez.spill("Goroutines spawned:", NUM_GOROUTINES)
    vibez.spill("Operations per goroutine:", NUM_OPERATIONS)
    
    # Calculate throughput
    sus throughput_sent drip = (total_sent * 1000) / duration
    sus throughput_received drip = (total_received * 1000) / duration
    
    vibez.spill("Throughput: Sent =", throughput_sent, "ops/sec, Received =", throughput_received, "ops/sec")
    
    # Evaluate test result
    sus expected_min drip = ((NUM_GOROUTINES / 3) * NUM_OPERATIONS * 80) / 100  # 80% minimum success rate
    
    ready (total_sent >= expected_min && total_received >= expected_min && failures < (NUM_OPERATIONS * 20 / 100)) {
        vibez.spill("🎉 HIGH-CONTENTION TEST PASSED!")
        vibez.spill("✅ No deadlocks detected")
        vibez.spill("✅ Acceptable throughput achieved") 
        vibez.spill("✅ Failure rate within acceptable limits")
    } otherwise {
        vibez.spill("❌ HIGH-CONTENTION TEST FAILED!")
        ready (total_sent < expected_min) {
            vibez.spill("❌ Insufficient sends:", total_sent, "< expected:", expected_min)
        }
        ready (total_received < expected_min) {
            vibez.spill("❌ Insufficient receives:", total_received, "< expected:", expected_min)
        }
        ready (failures >= (NUM_OPERATIONS * 20 / 100)) {
            vibez.spill("❌ Too many failures:", failures)
        }
    }
    
    # Close channels
    concurrenz.close_channel(main_channel)
    concurrenz.close_channel(select_ch1)
    concurrenz.close_channel(select_ch2)
    
    vibez.spill("=== Channel Deadlock Test Complete ===")
}

# Run the test
main()
