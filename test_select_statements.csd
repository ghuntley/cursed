// Select Statement Comprehensive Test
// Tests: ready keyword, timeout, default, channel multiplexing

yeet "testz"

// Test 1: Basic Select with Two Channels
test_start("basic select with two channels")
sus (ch1_sender, ch1_receiver) := make_channel()
sus (ch2_sender, ch2_receiver) := make_channel()

// Send on first channel
yolo {
    ch1_sender.send(100)
}

sus selected := cap
ready {
    ch1_receiver -> {
        sus val := ch1_receiver.recv()
        assert_eq_int(val, 100)
        selected = based
    }
    ch2_receiver -> {
        vibez.spill("Should not select ch2")
    }
}

assert_true(selected)

// Test 2: Select with Timeout
test_start("select with timeout")
sus (timeout_sender, timeout_receiver) := make_channel()

sus timeout_executed := cap
ready {
    timeout_receiver -> {
        vibez.spill("Should not receive")
    }
    timeout(100) -> {
        timeout_executed = based
    }
}

assert_true(timeout_executed)

// Test 3: Select with Default Case
test_start("select with default case")
sus (default_sender, default_receiver) := make_channel()

sus default_executed := cap
ready {
    default_receiver -> {
        vibez.spill("Should not receive")
    }
    default -> {
        default_executed = based
    }
}

assert_true(default_executed)

// Test 4: Select with Multiple Ready Channels
test_start("select with multiple ready channels")
sus (multi1_sender, multi1_receiver) := make_buffered_channel(1)
sus (multi2_sender, multi2_receiver) := make_buffered_channel(1)

// Send on both channels
multi1_sender.send(111)
multi2_sender.send(222)

sus first_selected := 0
sus second_selected := 0

// First select
ready {
    multi1_receiver -> {
        sus val := multi1_receiver.recv()
        first_selected = val
    }
    multi2_receiver -> {
        sus val := multi2_receiver.recv()
        first_selected = val
    }
}

// Second select
ready {
    multi1_receiver -> {
        sus val := multi1_receiver.recv()
        second_selected = val
    }
    multi2_receiver -> {
        sus val := multi2_receiver.recv()
        second_selected = val
    }
}

assert_true(first_selected == 111 || first_selected == 222)
assert_true(second_selected == 111 || second_selected == 222)
assert_true(first_selected != second_selected)

// Test 5: Select with Send Operations
test_start("select with send operations")
sus (send_ch1_sender, send_ch1_receiver) := make_buffered_channel(1)
sus (send_ch2_sender, send_ch2_receiver) := make_buffered_channel(1)

sus send_completed := cap
ready {
    send_ch1_sender.send(333) -> {
        send_completed = based
    }
    send_ch2_sender.send(444) -> {
        send_completed = based
    }
}

assert_true(send_completed)

// Test 6: Select with Mixed Send/Receive
test_start("select with mixed send/receive")
sus (mixed_sender, mixed_receiver) := make_buffered_channel(1)
sus (mixed_sender2, mixed_receiver2) := make_channel()

// Pre-fill one channel
mixed_sender.send(555)

sus mixed_result := 0
ready {
    mixed_receiver -> {
        sus val := mixed_receiver.recv()
        mixed_result = val
    }
    mixed_sender2.send(666) -> {
        mixed_result = 999
    }
}

assert_eq_int(mixed_result, 555)

// Test 7: Select in Loop (Channel Multiplexing)
test_start("select in loop (channel multiplexing)")
sus (loop_ch1_sender, loop_ch1_receiver) := make_buffered_channel(5)
sus (loop_ch2_sender, loop_ch2_receiver) := make_buffered_channel(5)
sus (loop_ch3_sender, loop_ch3_receiver) := make_buffered_channel(5)

// Send values on all channels
bestie i := 1; i <= 5; i++ {
    loop_ch1_sender.send(i)
    loop_ch2_sender.send(i * 10)
    loop_ch3_sender.send(i * 100)
}

sus loop_results := []
sus loop_count := 0

// Multiplex channels in loop
while loop_count < 15 {
    ready {
        loop_ch1_receiver -> {
            sus val := loop_ch1_receiver.recv()
            loop_results = append(loop_results, val)
            loop_count++
        }
        loop_ch2_receiver -> {
            sus val := loop_ch2_receiver.recv()
            loop_results = append(loop_results, val)
            loop_count++
        }
        loop_ch3_receiver -> {
            sus val := loop_ch3_receiver.recv()
            loop_results = append(loop_results, val)
            loop_count++
        }
    }
}

assert_eq_int(len(loop_results), 15)

// Test 8: Select with Channel Closing
test_start("select with channel closing")
sus (close_sender, close_receiver) := make_channel()
sus (close_sender2, close_receiver2) := make_channel()

// Close first channel
close_sender.close()

sus close_detected := cap
ready {
    close_receiver -> {
        // Should detect closed channel
        close_detected = based
    }
    close_receiver2 -> {
        vibez.spill("Should not select open channel")
    }
    timeout(100) -> {
        vibez.spill("Should not timeout")
    }
}

assert_true(close_detected)

// Test 9: Select with Goroutine Coordination
test_start("select with goroutine coordination")
sus (coord_work_sender, coord_work_receiver) := make_buffered_channel(10)
sus (coord_result_sender, coord_result_receiver) := make_buffered_channel(10)
sus (coord_done_sender, coord_done_receiver) := make_channel()

// Worker goroutine
yolo {
    bestie {
        ready {
            coord_work_receiver -> {
                sus work := coord_work_receiver.recv()
                if work == 0 {
                    coord_done_sender.send(based)
                    ghosted
                }
                sus result := work * 2
                coord_result_sender.send(result)
            }
        }
    }
}

// Send work
bestie i := 1; i <= 5; i++ {
    coord_work_sender.send(i)
}

// Collect results
sus coord_results := []
bestie i := 0; i < 5; i++ {
    sus result := coord_result_receiver.recv()
    coord_results = append(coord_results, result)
}

// Send termination signal
coord_work_sender.send(0)

// Wait for worker to finish
sus done := coord_done_receiver.recv()
assert_true(done)
assert_eq_int(len(coord_results), 5)

// Test 10: Select with Priority Channels
test_start("select with priority channels")
sus (priority_high_sender, priority_high_receiver) := make_buffered_channel(1)
sus (priority_low_sender, priority_low_receiver) := make_buffered_channel(1)

// Send on both channels
priority_high_sender.send(1000)
priority_low_sender.send(1)

sus priority_order := []

// Use nested selects to implement priority
ready {
    priority_high_receiver -> {
        sus val := priority_high_receiver.recv()
        priority_order = append(priority_order, val)
    }
    default -> {
        ready {
            priority_low_receiver -> {
                sus val := priority_low_receiver.recv()
                priority_order = append(priority_order, val)
            }
        }
    }
}

assert_eq_int(priority_order[0], 1000)

// Test 11: Select with Error Handling
test_start("select with error handling")
sus (error_sender, error_receiver) := make_channel()
sus (error_sender2, error_receiver2) := make_channel()

sus error_handled := cap
yikes {
    ready {
        error_receiver -> {
            sus val := error_receiver.recv()
            if val == 0 {
                panic("Error condition")
            }
        }
        error_receiver2 -> {
            sus val := error_receiver2.recv()
        }
        timeout(100) -> {
            error_handled = based
        }
    }
} shook err {
    error_handled = based
}

assert_true(error_handled)

// Test 12: Select with Channel Buffering Strategies
test_start("select with channel buffering strategies")
sus (buf_unbuffered_sender, buf_unbuffered_receiver) := make_channel()
sus (buf_buffered_sender, buf_buffered_receiver) := make_buffered_channel(5)

// Fill buffered channel
bestie i := 0; i < 5; i++ {
    buf_buffered_sender.send(i)
}

sus buffer_strategy_result := 0
ready {
    buf_unbuffered_receiver -> {
        buffer_strategy_result = 1
    }
    buf_buffered_receiver -> {
        sus val := buf_buffered_receiver.recv()
        buffer_strategy_result = val
    }
}

assert_true(buffer_strategy_result >= 0 && buffer_strategy_result < 5)

// Test 13: Select with Dynamic Channel Set
test_start("select with dynamic channel set")
sus dynamic_channels := []
sus dynamic_results := make_buffered_channel(10)

// Create dynamic set of channels
bestie i := 0; i < 3; i++ {
    sus (sender, receiver) := make_buffered_channel(1)
    dynamic_channels = append(dynamic_channels, receiver)
    
    // Send data
    sender.send(i * 111)
}

// Select from dynamic channel set
sus dynamic_collected := 0
while dynamic_collected < 3 {
    ready {
        dynamic_channels[0] -> {
            sus val := dynamic_channels[0].recv()
            dynamic_results.send(val)
            dynamic_collected++
        }
        dynamic_channels[1] -> {
            sus val := dynamic_channels[1].recv()
            dynamic_results.send(val)
            dynamic_collected++
        }
        dynamic_channels[2] -> {
            sus val := dynamic_channels[2].recv()
            dynamic_results.send(val)
            dynamic_collected++
        }
    }
}

assert_eq_int(dynamic_collected, 3)

// Test 14: Select with Backpressure Control
test_start("select with backpressure control")
sus (backpressure_sender, backpressure_receiver) := make_buffered_channel(2)
sus (backpressure_overflow_sender, backpressure_overflow_receiver) := make_unbounded_channel()

// Fill main channel
backpressure_sender.send(1)
backpressure_sender.send(2)

sus backpressure_handled := cap
ready {
    backpressure_sender.send(3) -> {
        vibez.spill("Should not send to full channel")
    }
    backpressure_overflow_sender.send(3) -> {
        backpressure_handled = based
    }
}

assert_true(backpressure_handled)

// Test 15: Select Performance Benchmark
test_start("select performance benchmark")
sus (perf_sender, perf_receiver) := make_buffered_channel(1000)
sus (perf_sender2, perf_receiver2) := make_buffered_channel(1000)

// Fill channels
bestie i := 0; i < 1000; i++ {
    perf_sender.send(i)
    perf_sender2.send(i * 2)
}

sus perf_start_time := time.now()
sus perf_count := 0

// High-performance select loop
while perf_count < 2000 {
    ready {
        perf_receiver -> {
            sus val := perf_receiver.recv()
            perf_count++
        }
        perf_receiver2 -> {
            sus val := perf_receiver2.recv()
            perf_count++
        }
    }
}

sus perf_end_time := time.now()
sus perf_duration := perf_end_time - perf_start_time

assert_eq_int(perf_count, 2000)
assert_true(perf_duration < 1000) // Should complete within 1 second

print_test_summary()
