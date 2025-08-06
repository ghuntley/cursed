# Advanced Select Statement Implementation for CURSED
# This implements CSP-style concurrent programming with comprehensive select statement support

yeet "testz"
yeet "concurrenz"

# Test basic select with channel operations
slay test_basic_select() {
    test_start("Basic Select Statement")
    
    sus ch1 dm<normie> = dm<normie>(5)
    sus ch2 dm<normie> = dm<normie>(5)
    
    # Send values to channels
    dm_send(ch1, 42)
    dm_send(ch2, 84)
    
    sus received lit = cringe
    sus value normie = 0
    
    # Basic select statement
    ready {
        mood val := dm_recv(ch1): {
            value = val
            received = based
        }
        mood val := dm_recv(ch2): {
            value = val + 100
            received = based
        }
        basic: {
            vibez.spill("No channels ready")
        }
    }
    
    assert_true(received)
    vibez.spill("Received value: ", value)
    
    print_test_summary()
}

# Test select with timeout
slay test_select_timeout() {
    test_start("Select with Timeout")
    
    sus ch dm<normie> = dm<normie>(0)  # Unbuffered channel
    sus timeout_hit lit = cringe
    
    ready {
        mood val := dm_recv(ch): {
            vibez.spill("Received: ", val)
        }
        timeout(100ms): {
            timeout_hit = based
            vibez.spill("Timeout occurred")
        }
    }
    
    assert_true(timeout_hit)
    print_test_summary()
}

# Test select with multiple send operations
slay test_select_multi_send() {
    test_start("Select Multiple Send")
    
    sus ch1 dm<normie> = dm<normie>(1)
    sus ch2 dm<normie> = dm<normie>(1)
    sus sent_to normie = 0
    
    ready {
        mood dm_send(ch1, 100): {
            sent_to = 1
            vibez.spill("Sent to channel 1")
        }
        mood dm_send(ch2, 200): {
            sent_to = 2
            vibez.spill("Sent to channel 2")
        }
        basic: {
            vibez.spill("No channels ready for send")
        }
    }
    
    assert_true(sent_to > 0)
    print_test_summary()
}

# Test select with mixed send/receive
slay test_select_mixed_operations() {
    test_start("Select Mixed Send/Receive")
    
    sus send_ch dm<normie> = dm<normie>(1)
    sus recv_ch dm<normie> = dm<normie>(1)
    
    # Pre-fill receive channel
    dm_send(recv_ch, 999)
    
    sus operation tea = ""
    sus value normie = 0
    
    ready {
        mood dm_send(send_ch, 555): {
            operation = "sent"
            value = 555
        }
        mood val := dm_recv(recv_ch): {
            operation = "received"
            value = val
        }
        basic: {
            operation = "default"
        }
    }
    
    assert_true(operation != "")
    vibez.spill("Operation: ", operation, " Value: ", value)
    print_test_summary()
}

# Test non-blocking select (all default cases)
slay test_non_blocking_select() {
    test_start("Non-blocking Select")
    
    sus ch dm<normie> = dm<normie>(0)  # Unbuffered, empty channel
    sus default_executed lit = cringe
    
    ready {
        mood val := dm_recv(ch): {
            vibez.spill("Should not receive")
        }
        mood dm_send(ch, 123): {
            vibez.spill("Should not send")
        }
        basic: {
            default_executed = based
            vibez.spill("Default case executed")
        }
    }
    
    assert_true(default_executed)
    print_test_summary()
}

# Test select with goroutines
slay test_select_with_goroutines() {
    test_start("Select with Goroutines")
    
    sus ch dm<normie> = dm<normie>(0)
    sus result normie = 0
    
    # Start goroutine that sends after delay
    stan {
        yolo()  # Yield to ensure select starts first
        dm_send(ch, 777)
    }
    
    ready {
        mood val := dm_recv(ch): {
            result = val
            vibez.spill("Received from goroutine: ", val)
        }
        timeout(1000ms): {
            vibez.spill("Timeout waiting for goroutine")
        }
    }
    
    assert_eq_int(result, 777)
    print_test_summary()
}

# Test select with channel direction (send-only, receive-only)
slay test_select_channel_directions() {
    test_start("Select Channel Directions")
    
    sus ch dm<normie> = dm<normie>(1)
    sus send_only dm<-normie = ch  # Send-only channel
    sus recv_only <-dm<normie> = ch  # Receive-only channel
    
    sus operation tea = ""
    
    ready {
        mood dm_send(send_only, 111): {
            operation = "sent"
        }
        basic: {
            operation = "default"
        }
    }
    
    ready {
        mood val := dm_recv(recv_only): {
            operation = "received"
            vibez.spill("Received: ", val)
        }
        basic: {
            operation = "no_receive"
        }
    }
    
    assert_true(operation != "")
    print_test_summary()
}

# Test select priority and fairness
slay test_select_fairness() {
    test_start("Select Fairness")
    
    sus ch1 dm<normie> = dm<normie>(1)
    sus ch2 dm<normie> = dm<normie>(1)
    
    # Fill both channels
    dm_send(ch1, 1)
    dm_send(ch2, 2)
    
    sus counts [2]normie = [0, 0]
    
    # Run select multiple times to test fairness
    bestie i drip = 0; i < 10; i++ {
        ready {
            mood val := dm_recv(ch1): {
                counts[0]++
                dm_send(ch1, val)  # Put it back
            }
            mood val := dm_recv(ch2): {
                counts[1]++
                dm_send(ch2, val)  # Put it back
            }
        }
    }
    
    vibez.spill("Channel 1 selected: ", counts[0])
    vibez.spill("Channel 2 selected: ", counts[1])
    
    # Both should be selected at least once for fairness
    assert_true(counts[0] > 0)
    assert_true(counts[1] > 0)
    print_test_summary()
}

# Test select with closed channels
slay test_select_closed_channels() {
    test_start("Select with Closed Channels")
    
    sus ch dm<normie> = dm<normie>(1)
    
    # Send value then close
    dm_send(ch, 123)
    dm_close(ch)
    
    sus received_before_close lit = cringe
    sus received_after_close lit = cringe
    sus value normie = 0
    
    # Should receive the value
    ready {
        mood val := dm_recv(ch): {
            received_before_close = based
            value = val
        }
        basic: {
            vibez.spill("Should have received value")
        }
    }
    
    # Should receive zero value and closed indication
    ready {
        mood val := dm_recv(ch): {
            received_after_close = based
            assert_eq_int(val, 0)  # Zero value for closed channel
        }
        basic: {
            vibez.spill("Should have received from closed channel")
        }
    }
    
    assert_true(received_before_close)
    assert_true(received_after_close)
    assert_eq_int(value, 123)
    print_test_summary()
}

# Test complex select with error handling
slay test_select_error_handling() {
    test_start("Select Error Handling")
    
    sus ch dm<normie> = dm<normie>(0)
    sus error_handled lit = cringe
    
    ready {
        mood val := dm_recv(ch): {
            vibez.spill("Received: ", val)
        } yikes err: {
            error_handled = based
            vibez.spill("Handled channel error: ", err)
        }
        basic: {
            vibez.spill("Default case")
        }
    }
    
    # For this test, we expect default case since channel is empty
    print_test_summary()
}

# Main test runner
slay main() {
    vibez.spill("=== Advanced Select Statement Tests ===")
    
    test_basic_select()
    test_select_timeout()
    test_select_multi_send()
    test_select_mixed_operations()
    test_non_blocking_select()
    test_select_with_goroutines()
    test_select_channel_directions()
    test_select_fairness()
    test_select_closed_channels()
    test_select_error_handling()
    
    vibez.spill("=== All Select Tests Complete ===")
}
