yeet "concurrenz"
yeet "vibez"
yeet "timez"

// Test that select doesn't busy-wait and spin CPU
slay test_select_blocking() {
    vibez.spill("Testing select blocking behavior...")
    
    // Create an unbuffered channel
    sus ch dm<drip> = make_dm(0)
    
    // Measure CPU usage before select
    sus start_time drip = get_timestamp_ms()
    
    // Spawn a goroutine that will send after 100ms
    go {
        sleep_ms(100)
        ch <- 42
    }
    
    // Use select to wait for the channel (should block, not spin)
    ready {
        case val drip = <-ch:
            vibez.spill("Received value:", val)
        case default:
            vibez.spill("Should not reach default case")
    }
    
    sus end_time drip = get_timestamp_ms()
    sus elapsed drip = end_time - start_time
    
    vibez.spill("Select operation took", elapsed, "ms")
    
    // If select was spinning CPU, this would complete much faster
    // Proper blocking should take close to 100ms
    ready (elapsed >= 90 && elapsed <= 200) {
        vibez.spill("✅ Select properly blocked (no CPU spin)")
    } otherwise {
        vibez.spill("❌ Select timing suggests CPU spinning issue")
    }
    
    dm_close(ch)
}

// Test select with multiple channels
slay test_select_multiple_channels() {
    vibez.spill("Testing select with multiple channels...")
    
    sus ch1 dm<drip> = make_dm(0)
    sus ch2 dm<tea> = make_dm(0)
    
    // Send to ch2 after 50ms
    go {
        sleep_ms(50)
        ch2 <- "hello"
    }
    
    sus start_time drip = get_timestamp_ms()
    
    ready {
        case val drip = <-ch1:
            vibez.spill("Received from ch1:", val)
        case msg tea = <-ch2:
            vibez.spill("Received from ch2:", msg)
            sus end_time drip = get_timestamp_ms()
            sus elapsed drip = end_time - start_time
            vibez.spill("Multi-channel select took", elapsed, "ms")
    }
    
    dm_close(ch1)
    dm_close(ch2)
}

// Test select timeout without spinning
slay test_select_timeout() {
    vibez.spill("Testing select timeout behavior...")
    
    sus ch dm<drip> = make_dm(0)
    sus start_time drip = get_timestamp_ms()
    
    ready {
        case val drip = <-ch:
            vibez.spill("Should not receive anything")
        case timeout(100): // 100ms timeout
            sus end_time drip = get_timestamp_ms()
            sus elapsed drip = end_time - start_time
            vibez.spill("Select timeout after", elapsed, "ms")
            
            ready (elapsed >= 90 && elapsed <= 150) {
                vibez.spill("✅ Timeout worked correctly")
            } otherwise {
                vibez.spill("❌ Timeout timing issue")
            }
    }
    
    dm_close(ch)
}

// Main test runner
test_start("select_cpu_spin_fix")

test_select_blocking()
test_select_multiple_channels() 
test_select_timeout()

print_test_summary()
vibez.spill("Select fix test completed")
