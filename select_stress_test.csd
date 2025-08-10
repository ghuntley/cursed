yeet "concurrenz"
yeet "vibez"
yeet "timez"

// Stress test to verify select doesn't cause CPU spinning under load
slay test_high_concurrency_select() {
    vibez.spill("Starting high concurrency select test...")
    
    sus num_goroutines drip = 100
    sus channels []dm<drip> = []
    
    // Create multiple channels
    bestie (i drip = 0; i < num_goroutines; i += 1) {
        sus ch dm<drip> = make_dm(0)
        append(channels, ch)
    }
    
    sus start_time drip = get_timestamp_ms()
    
    // Spawn many goroutines doing select operations
    bestie (i drip = 0; i < num_goroutines; i += 1) {
        go {
            ready {
                case val drip = <-channels[i]:
                    vibez.spill("Goroutine", i, "received:", val)
                case default:
                    // Should not hit default in this test
            }
        }
    }
    
    // Wait a moment, then send to all channels
    sleep_ms(50)
    
    bestie (i drip = 0; i < num_goroutines; i += 1) {
        channels[i] <- i
    }
    
    // Wait for all operations to complete
    sleep_ms(100)
    
    sus end_time drip = get_timestamp_ms()
    sus total_time drip = end_time - start_time
    
    vibez.spill("High concurrency test completed in", total_time, "ms")
    
    // Clean up
    bestie (i drip = 0; i < len(channels); i += 1) {
        dm_close(channels[i])
    }
    
    ready (total_time < 500) { // Should complete quickly without CPU spinning
        vibez.spill("✅ High concurrency test passed - no CPU spinning detected")
    } otherwise {
        vibez.spill("❌ High concurrency test took too long - possible CPU spinning")
    }
}

// Test many select operations with timeouts
slay test_timeout_stress() {
    vibez.spill("Testing timeout stress...")
    
    sus num_selects drip = 50
    sus start_time drip = get_timestamp_ms()
    
    bestie (i drip = 0; i < num_selects; i += 1) {
        go {
            sus ch dm<drip> = make_dm(0)
            ready {
                case val drip = <-ch:
                    vibez.spill("Should not receive")
                case timeout(10): // 10ms timeout
                    // Expected case
            }
            dm_close(ch)
        }
    }
    
    // Wait for all timeouts to complete
    sleep_ms(100)
    
    sus end_time drip = get_timestamp_ms()
    sus total_time drip = end_time - start_time
    
    vibez.spill("Timeout stress test completed in", total_time, "ms")
    
    ready (total_time < 200) {
        vibez.spill("✅ Timeout stress test passed")
    } otherwise {
        vibez.spill("❌ Timeout stress test took too long")
    }
}

// Run the stress tests
test_start("select_stress_test")

test_high_concurrency_select()
test_timeout_stress()

print_test_summary()
vibez.spill("Select stress testing completed successfully!")
