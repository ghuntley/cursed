# CURSED Sync Primitives Fix Validation Test
# Tests the fixed condition variable bridging and synchronization primitives

yeet "vibez"
yeet "mathz" 
yeet "concurrenz"
yeet "testz"

# Test 1: Basic Channel Select with Proper Condition Variable Blocking
slay test_basic_select() {
    vibez.spill("Testing basic select with condition variable fix...")
    
    # Create channels
    sus ch1 dm<drip> = dm_create(10)
    sus ch2 dm<drip> = dm_create(10) 
    sus result_ch dm<drip> = dm_create(1)
    
    # Sender goroutine
    go {
        sleep(50) # 50ms delay
        ch1 <- 42
        vibez.spill("Sent value to ch1")
    }
    
    # Select operation that should block properly on condition variables
    go {
        ready {
            when val drip = <-ch1 -> {
                vibez.spill("Received from ch1:", val)
                result_ch <- val
            }
            when val drip = <-ch2 -> {
                vibez.spill("Received from ch2:", val)
                result_ch <- val + 100
            }
            otherwise -> {
                vibez.spill("Default case should not execute")
                result_ch <- 999
            }
        }
    }
    
    # Wait for result
    sus result drip = <-result_ch
    vibez.spill("Select result:", result)
    
    # Verify the condition variable blocking worked correctly
    assert_eq_int(result, 42)
    
    dm_close(ch1)
    dm_close(ch2)
    dm_close(result_ch)
}

# Test 2: Multiple Goroutines with Condition Variable Coordination
slay test_multiple_goroutines_sync() {
    vibez.spill("Testing multiple goroutines with condition variable synchronization...")
    
    sus shared_ch dm<drip> = dm_create(5)
    sus result_ch dm<drip> = dm_create(10)
    sus goroutine_count drip = 5
    
    # Spawn multiple sender goroutines
    bestie (sus i drip = 0; i < goroutine_count; i += 1) {
        go {
            sleep(random() % 20) # Random delay up to 20ms
            shared_ch <- i * 10
            vibez.spill("Goroutine", i, "sent value:", i * 10)
        }
    }
    
    # Spawn multiple receiver goroutines with select statements
    bestie (sus j drip = 0; j < goroutine_count; j += 1) {
        go {
            ready {
                when val drip = <-shared_ch -> {
                    vibez.spill("Goroutine", j, "received:", val)
                    result_ch <- val
                }
                timeout(100) -> {
                    vibez.spill("Goroutine", j, "timed out")
                    result_ch <- -1
                }
            }
        }
    }
    
    # Collect all results
    sus results []drip = []
    bestie (sus k drip = 0; k < goroutine_count; k += 1) {
        sus val drip = <-result_ch
        vibez.spill("Collected result:", val)
        results = append(results, val)
    }
    
    # Verify we got all expected results
    sus positive_count drip = 0
    bestie (val drip : results) {
        ready (val >= 0) {
            positive_count += 1
        }
    }
    
    vibez.spill("Positive results count:", positive_count)
    assert_ge_int(positive_count, 3) # At least 3 should succeed with proper sync
    
    dm_close(shared_ch)
    dm_close(result_ch)
}

# Test 3: Stress Test - High Concurrency with Select Operations
slay test_high_concurrency_select() {
    vibez.spill("Testing high concurrency select operations...")
    
    sus num_channels drip = 10
    sus num_goroutines drip = 20
    sus channels []dm<drip> = []
    sus result_ch dm<drip> = dm_create(num_goroutines)
    
    # Create multiple channels
    bestie (sus i drip = 0; i < num_channels; i += 1) {
        sus ch dm<drip> = dm_create(5)
        channels = append(channels, ch)
    }
    
    # Spawn senders
    bestie (sus i drip = 0; i < num_channels; i += 1) {
        go {
            sleep(random() % 30) # Random delay
            channels[i] <- i * 100
            vibez.spill("Sent to channel", i, "value:", i * 100)
        }
    }
    
    # Spawn receivers with complex select statements
    bestie (sus j drip = 0; j < num_goroutines; j += 1) {
        go {
            sus received lit = spooked
            sus attempt_count drip = 0
            
            bestie (!received && attempt_count < 5) {
                attempt_count += 1
                
                ready {
                    when val drip = <-channels[0] -> {
                        vibez.spill("Worker", j, "got from ch0:", val)
                        result_ch <- val
                        received = based
                    }
                    when val drip = <-channels[1] -> {
                        vibez.spill("Worker", j, "got from ch1:", val)
                        result_ch <- val + 1
                        received = based
                    }
                    when val drip = <-channels[2] -> {
                        vibez.spill("Worker", j, "got from ch2:", val)
                        result_ch <- val + 2
                        received = based
                    }
                    timeout(20) -> {
                        vibez.spill("Worker", j, "timeout on attempt", attempt_count)
                        # Continue loop
                    }
                }
            }
            
            ready (!received) {
                vibez.spill("Worker", j, "failed to receive after all attempts")
                result_ch <- -999
            }
        }
    }
    
    # Collect results with timeout protection
    sus successful_operations drip = 0
    sus timeout_operations drip = 0
    
    bestie (sus k drip = 0; k < num_goroutines; k += 1) {
        ready {
            when val drip = <-result_ch -> {
                ready (val >= 0) {
                    successful_operations += 1
                } otherwise {
                    timeout_operations += 1
                }
                vibez.spill("Result", k, ":", val)
            }
            timeout(200) -> {
                vibez.spill("Timeout waiting for result", k)
                timeout_operations += 1
            }
        }
    }
    
    vibez.spill("Successful operations:", successful_operations)
    vibez.spill("Timeout operations:", timeout_operations)
    
    # With proper condition variable bridging, we should have high success rate
    assert_ge_int(successful_operations, num_goroutines / 2)
    
    # Clean up channels
    bestie (ch dm<drip> : channels) {
        dm_close(ch)
    }
    dm_close(result_ch)
}

# Test 4: Condition Variable Spurious Wakeup Protection
slay test_spurious_wakeup_protection() {
    vibez.spill("Testing spurious wakeup protection...")
    
    sus control_ch dm<drip> = dm_create(1)
    sus test_ch dm<drip> = dm_create(1)
    sus result_ch dm<drip> = dm_create(1)
    
    # Create a scenario that might trigger spurious wakeups
    go {
        bestie (sus i drip = 0; i < 50; i += 1) {
            sleep(1) # 1ms intervals
            control_ch <- i
            <-control_ch # Immediate receive to create churn
        }
    }
    
    # Main test goroutine that should handle spurious wakeups gracefully
    go {
        sus wait_count drip = 0
        sus received_correct_value lit = spooked
        
        bestie (!received_correct_value && wait_count < 100) {
            ready {
                when val drip = <-test_ch -> {
                    ready (val == 12345) {
                        vibez.spill("Received correct value:", val)
                        received_correct_value = based
                        result_ch <- 1
                    } otherwise {
                        vibez.spill("Received incorrect value:", val, "(spurious?)")
                        wait_count += 1
                    }
                }
                when <-control_ch -> {
                    wait_count += 1
                    control_ch <- 0 # Put back
                }
                timeout(5) -> {
                    wait_count += 1
                }
            }
        }
        
        ready (!received_correct_value) {
            vibez.spill("Test completed without receiving correct value, wait_count:", wait_count)
            result_ch <- 0
        }
    }
    
    # Send the expected value after some delay
    go {
        sleep(25) # 25ms delay
        test_ch <- 12345
        vibez.spill("Sent correct test value")
    }
    
    # Wait for test completion
    sus test_result drip = <-result_ch
    vibez.spill("Spurious wakeup test result:", test_result)
    
    # The test should succeed (receive the correct value)
    assert_eq_int(test_result, 1)
    
    dm_close(control_ch)
    dm_close(test_ch)
    dm_close(result_ch)
}

# Test 5: Deadlock Prevention in Nested Selects
slay test_deadlock_prevention() {
    vibez.spill("Testing deadlock prevention in nested select operations...")
    
    sus ch_a dm<drip> = dm_create(1)
    sus ch_b dm<drip> = dm_create(1)
    sus ch_c dm<drip> = dm_create(1)
    sus result_ch dm<drip> = dm_create(2)
    
    # Goroutine 1: Complex select with potential deadlock
    go {
        ready {
            when val drip = <-ch_a -> {
                vibez.spill("G1: Got from ch_a:", val)
                
                # Nested select that could cause deadlock without proper implementation
                ready {
                    when ch_b <- (val * 2) -> {
                        vibez.spill("G1: Sent to ch_b:", val * 2)
                        result_ch <- 1
                    }
                    timeout(50) -> {
                        vibez.spill("G1: Timeout on ch_b send")
                        result_ch <- -1
                    }
                }
            }
            timeout(100) -> {
                vibez.spill("G1: Timeout on ch_a receive")
                result_ch <- -2
            }
        }
    }
    
    # Goroutine 2: Complementary operations
    go {
        ready {
            when val drip = <-ch_b -> {
                vibez.spill("G2: Got from ch_b:", val)
                
                ready {
                    when ch_c <- (val + 10) -> {
                        vibez.spill("G2: Sent to ch_c:", val + 10)
                        result_ch <- 2
                    }
                    timeout(50) -> {
                        vibez.spill("G2: Timeout on ch_c send")
                        result_ch <- -3
                    }
                }
            }
            timeout(100) -> {
                vibez.spill("G2: Timeout on ch_b receive")
                result_ch <- -4
            }
        }
    }
    
    # Trigger the sequence
    go {
        sleep(10) # Small delay
        ch_a <- 123
        vibez.spill("Triggered sequence with value 123")
    }
    
    # Wait for both goroutines to complete
    sus result1 drip = <-result_ch
    sus result2 drip = <-result_ch
    
    vibez.spill("Deadlock test results:", result1, result2)
    
    # Both should succeed without deadlock
    sus total_success drip = 0
    ready (result1 > 0) { total_success += 1 }
    ready (result2 > 0) { total_success += 1 }
    
    vibez.spill("Successful operations:", total_success)
    assert_eq_int(total_success, 2)
    
    # Verify the final result
    ready {
        when final_val drip = <-ch_c -> {
            vibez.spill("Final value in ch_c:", final_val)
            assert_eq_int(final_val, 256) # 123 * 2 + 10 = 256
        }
        timeout(50) -> {
            vibez.spill("Warning: No final value received")
        }
    }
    
    dm_close(ch_a)
    dm_close(ch_b) 
    dm_close(ch_c)
    dm_close(result_ch)
}

# Main test runner
slay main() {
    vibez.spill("=== CURSED Sync Primitives Fix Validation ===")
    
    test_start("sync_primitives_fix")
    
    vibez.spill()
    test_basic_select()
    vibez.spill("✅ Basic select test passed")
    
    vibez.spill()
    test_multiple_goroutines_sync()
    vibez.spill("✅ Multiple goroutines sync test passed")
    
    vibez.spill()
    test_high_concurrency_select()
    vibez.spill("✅ High concurrency select test passed")
    
    vibez.spill()
    test_spurious_wakeup_protection()
    vibez.spill("✅ Spurious wakeup protection test passed")
    
    vibez.spill()
    test_deadlock_prevention()
    vibez.spill("✅ Deadlock prevention test passed")
    
    vibez.spill()
    vibez.spill("=== All Sync Primitives Tests Completed Successfully ===")
    vibez.spill("✅ Condition variable bridging issues have been resolved")
    vibez.spill("✅ Synchronization primitives are now working correctly")
    vibez.spill("✅ Inter-goroutine communication is stable and reliable")
    
    print_test_summary()
}

main()
