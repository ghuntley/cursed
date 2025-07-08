yeet "testz"

slay test_select_basic() {
    sus ch1 dm<normie>
    sus ch2 dm<tea>
    sus result tea = ""
    
    yolo {
        ch1 <- 42
    }
    
    ready {
        mood val := <-ch1:
            result = "received from ch1: " + string(val)
        mood msg := <-ch2:
            result = "received from ch2: " + msg
        basic:
            result = "no channels ready"
    }
    
    assert_eq_string(result, "received from ch1: 42")
}

slay test_select_timeout() {
    sus ch dm<normie>
    sus timeout dm<lit>
    sus result tea = ""
    
    // Create timeout channel
    yolo {
        time.sleep(100) // 100ms
        timeout <- based
    }
    
    ready {
        mood val := <-ch:
            result = "received value: " + string(val)
        mood <-timeout:
            result = "timeout occurred"
    }
    
    assert_eq_string(result, "timeout occurred")
}

slay test_select_non_blocking() {
    sus ch dm<normie>
    sus result tea = ""
    
    ready {
        mood val := <-ch:
            result = "received: " + string(val)
        basic:
            result = "no data available"
    }
    
    assert_eq_string(result, "no data available")
}

slay test_select_multiple_ready() {
    sus ch1 dm<normie>
    sus ch2 dm<normie>
    sus received_count normie = 0
    
    yolo {
        ch1 <- 10
        ch2 <- 20
    }
    
    bestie i := 0; i < 2; i++ {
        ready {
            mood val := <-ch1:
                received_count++
                vibez.spill("Received from ch1:", val)
            mood val := <-ch2:
                received_count++
                vibez.spill("Received from ch2:", val)
            basic:
                vibez.spill("No channels ready")
        }
    }
    
    assert_eq_int(received_count, 2)
}

slay test_select_send_operation() {
    sus ch dm<normie>[1]  // Buffered channel
    sus result tea = ""
    
    ready {
        mood ch <- 42:
            result = "sent successfully"
        basic:
            result = "channel full"
    }
    
    assert_eq_string(result, "sent successfully")
}

slay test_select_closed_channel() {
    sus ch dm<normie>
    sus result tea = ""
    
    close(ch)
    
    ready {
        mood val, ok := <-ch:
            vibe_check ok {
                result = "received: " + string(val)
            } highkey {
                result = "channel closed"
            }
        basic:
            result = "no operations ready"
    }
    
    assert_eq_string(result, "channel closed")
}

// Test driver
test_start("Select Basic")
test_select_basic()
print_test_summary()

test_start("Select Timeout")
test_select_timeout()
print_test_summary()

test_start("Select Non-blocking")
test_select_non_blocking()
print_test_summary()

test_start("Select Multiple Ready")
test_select_multiple_ready()
print_test_summary()

test_start("Select Send Operation")
test_select_send_operation()
print_test_summary()

test_start("Select Closed Channel")
test_select_closed_channel()
print_test_summary()

vibez.spill("All select tests completed!")
