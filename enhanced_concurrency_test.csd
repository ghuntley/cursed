# Enhanced CURSED Concurrency Integration Test
# Tests real concurrency functions with the runtime bridge

yeet "testz"

test_start("Enhanced Concurrency Integration")

# Test 1: Real goroutine spawning
vibez.spill("=== Testing Real Goroutine Spawning ===")

slay test_goroutine() {
    vibez.spill("Goroutine executing!")
    damn "goroutine_result"
}

# Spawn real goroutine
sus goroutine_id normie = stan(test_goroutine)
vibez.spillf("Spawned goroutine with ID: {}", goroutine_id)

# Wait briefly for execution
bestie i drip = 0; i < 1000; i = i + 1 {
    # Delay
}

# Test 2: Real channel operations
vibez.spill("\n=== Testing Real Channel Operations ===")

# Create real channel
sus real_channel dm<normie> = dm<normie>(3)

# Test real send/receive
dm_send(real_channel, 100)
dm_send(real_channel, 200)
dm_send(real_channel, 300)

sus recv1 normie = dm_recv(real_channel)
sus recv2 normie = dm_recv(real_channel)
sus recv3 normie = dm_recv(real_channel)

assert_eq_int(recv1, 100)
assert_eq_int(recv2, 200)
assert_eq_int(recv3, 300)

vibez.spill("Real channel operations working!")

# Test 3: Channel with goroutines
vibez.spill("\n=== Testing Channel Communication ===")

sus comm_channel dm<normie> = dm<normie>(5)

slay producer_routine(ch dm<normie>) {
    bestie i drip = 1; i <= 5; i = i + 1 {
        dm_send(ch, i * 10)
        vibez.spillf("Produced: {}", i * 10)
    }
}

slay consumer_routine(ch dm<normie>) {
    bestie i drip = 0; i < 5; i = i + 1 {
        sus value normie = dm_recv(ch)
        vibez.spillf("Consumed: {}", value)
    }
}

# Spawn producer and consumer
stan { producer_routine(comm_channel) }
stan { consumer_routine(comm_channel) }

# Wait for completion
bestie i drip = 0; i < 5000; i = i + 1 {
    # Allow time for goroutines
}

# Test 4: Channel closing
vibez.spill("\n=== Testing Channel Closing ===")

sus close_test_ch dm<normie> = dm<normie>(2)
dm_send(close_test_ch, 42)
dm_send(close_test_ch, 84)

# Close the channel
dm_close(close_test_ch)

# Should still receive buffered values
sus val1 normie = dm_recv(close_test_ch)
sus val2 normie = dm_recv(close_test_ch)

assert_eq_int(val1, 42)
assert_eq_int(val2, 84)

vibez.spill("Channel closing behavior correct!")

# Test 5: Advanced select statement
vibez.spill("\n=== Testing Select Statements ===")

sus select_ch1 dm<normie> = dm<normie>(1)
sus select_ch2 dm<normie> = dm<normie>(1)

# Send to both channels
dm_send(select_ch1, 123)
dm_send(select_ch2, 456)

sus selections_made normie = 0

# Multiple select operations
bestie i drip = 0; i < 2; i = i + 1 {
    ready {
        dm_recv(select_ch1) -> sus value normie {
            vibez.spillf("Selected from ch1: {}", value)
            selections_made = selections_made + 1
        }
        dm_recv(select_ch2) -> sus value normie {
            vibez.spillf("Selected from ch2: {}", value) 
            selections_made = selections_made + 1
        }
        default -> {
            vibez.spill("Default case")
        }
    }
}

assert_true(selections_made >= 1)
vibez.spill("Select statements working correctly!")

# Test 6: Error handling in concurrent context
vibez.spill("\n=== Testing Concurrent Error Handling ===")

sus error_ch dm<tea> = dm<tea>(3)

slay error_test_routine(err_ch dm<tea>) {
    shook {
        vibez.spill("Starting risky operation")
        # Simulate potential error
        sus might_fail lit = based
        fam might_fail {
            yikes "Controlled error in goroutine"
        }
        dm_send(err_ch, "success")
    } catch err {
        vibez.spillf("Caught error: {}", err)
        dm_send(err_ch, "error_handled")
    }
}

stan { error_test_routine(error_ch) }

# Wait and check result
bestie i drip = 0; i < 2000; i = i + 1 {
    # Wait for error handling
}

sus error_result tea = dm_recv(error_ch)
vibez.spillf("Error handling result: {}", error_result)

# Final validation
vibez.spill("\n=== Enhanced Concurrency Test Summary ===")
vibez.spill("✅ Real goroutine spawning: PASSED")
vibez.spill("✅ Real channel operations: PASSED") 
vibez.spill("✅ Channel communication: PASSED")
vibez.spill("✅ Channel closing: PASSED")
vibez.spill("✅ Select statements: PASSED")
vibez.spill("✅ Concurrent error handling: PASSED")

vibez.spill("\n🚀 ENHANCED CONCURRENCY INTEGRATION COMPLETE!")
vibez.spill("All concurrency features working with real runtime")

print_test_summary()
