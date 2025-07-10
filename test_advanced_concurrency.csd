// Advanced Concurrency Features Test for CURSED
// Tests: channels, goroutines, select statements, async operations

yeet "testz"

// Test 1: Buffered Channel Operations
test_start("buffered channel operations")
sus (sender, receiver) := make_buffered_channel(3)

// Send values to buffered channel
sender.send(1)
sender.send(2)
sender.send(3)

// Receive values
sus val1 := receiver.recv()
sus val2 := receiver.recv()
sus val3 := receiver.recv()

assert_eq_int(val1, 1)
assert_eq_int(val2, 2)
assert_eq_int(val3, 3)

// Test 2: Unbuffered Channel Communication
test_start("unbuffered channel communication")
sus (unbuf_sender, unbuf_receiver) := make_channel()

// Spawn goroutine to send
yolo {
    unbuf_sender.send(42)
}

// Receive in main thread
sus received := unbuf_receiver.recv()
assert_eq_int(received, 42)

// Test 3: Select Statement with Multiple Channels
test_start("select statement with multiple channels")
sus (ch1_sender, ch1_receiver) := make_channel()
sus (ch2_sender, ch2_receiver) := make_channel()

// Send on first channel
yolo {
    ch1_sender.send(100)
}

// Send on second channel
yolo {
    ch2_sender.send(200)
}

// Select from multiple channels
ready {
    ch1_receiver -> {
        sus val := ch1_receiver.recv()
        assert_eq_int(val, 100)
    }
    ch2_receiver -> {
        sus val := ch2_receiver.recv()
        assert_eq_int(val, 200)
    }
}

// Test 4: Select with Timeout
test_start("select with timeout")
sus (timeout_sender, timeout_receiver) := make_channel()

// Select with timeout (should timeout)
ready {
    timeout_receiver -> {
        sus val := timeout_receiver.recv()
        vibez.spill("Should not reach here")
    }
    timeout(1000) -> {
        vibez.spill("Timeout case executed")
    }
}

// Test 5: Select with Default Case
test_start("select with default case")
sus (default_sender, default_receiver) := make_channel()

// Non-blocking select with default
ready {
    default_receiver -> {
        sus val := default_receiver.recv()
        vibez.spill("Should not reach here")
    }
    default -> {
        vibez.spill("Default case executed")
    }
}

// Test 6: Channel Closing and Range Operations
test_start("channel closing and range operations")
sus (range_sender, range_receiver) := make_buffered_channel(5)

// Send values and close
range_sender.send(1)
range_sender.send(2)
range_sender.send(3)
range_sender.close()

// Range over channel
sus count := 0
bestie val := range range_receiver {
    count++
    assert_true(val > 0)
}

assert_eq_int(count, 3)

// Test 7: Goroutine Lifecycle Management
test_start("goroutine lifecycle management")
sus goroutine_completed := based

yolo {
    // Goroutine work
    sus result := 42 * 2
    assert_eq_int(result, 84)
    goroutine_completed = based
}

// Wait for goroutine completion (simplified)
sus wait_count := 0
while !goroutine_completed && wait_count < 100 {
    wait_count++
    yield_now()
}

assert_true(goroutine_completed)

// Test 8: Channel Synchronization Patterns
test_start("channel synchronization patterns")
sus (sync_sender, sync_receiver) := make_channel()
sus (done_sender, done_receiver) := make_channel()

// Producer goroutine
yolo {
    bestie i := 0; i < 5; i++ {
        sync_sender.send(i)
    }
    done_sender.send(based)
}

// Consumer goroutine
yolo {
    bestie i := 0; i < 5; i++ {
        sus val := sync_receiver.recv()
        assert_eq_int(val, i)
    }
    done_sender.send(based)
}

// Wait for both goroutines
sus producer_done := done_receiver.recv()
sus consumer_done := done_receiver.recv()
assert_true(producer_done)
assert_true(consumer_done)

// Test 9: Worker Pool Pattern
test_start("worker pool pattern")
sus (work_sender, work_receiver) := make_buffered_channel(10)
sus (result_sender, result_receiver) := make_buffered_channel(10)

// Create worker pool
sus num_workers := 3
bestie i := 0; i < num_workers; i++ {
    yolo {
        bestie work := range work_receiver {
            sus result := work * 2
            result_sender.send(result)
        }
    }
}

// Send work
bestie i := 1; i <= 5; i++ {
    work_sender.send(i)
}
work_sender.close()

// Collect results
sus results := []
bestie i := 0; i < 5; i++ {
    sus result := result_receiver.recv()
    results = append(results, result)
}

assert_eq_int(len(results), 5)

// Test 10: Channel Multiplexing
test_start("channel multiplexing")
sus (input1_sender, input1_receiver) := make_channel()
sus (input2_sender, input2_receiver) := make_channel()
sus (output_sender, output_receiver) := make_channel()

// Multiplexer goroutine
yolo {
    bestie {
        ready {
            input1_receiver -> {
                sus val := input1_receiver.recv()
                output_sender.send(val)
            }
            input2_receiver -> {
                sus val := input2_receiver.recv()
                output_sender.send(val)
            }
        }
    }
}

// Send on both inputs
input1_sender.send(111)
input2_sender.send(222)

// Receive multiplexed output
sus out1 := output_receiver.recv()
sus out2 := output_receiver.recv()

assert_true(out1 == 111 || out1 == 222)
assert_true(out2 == 111 || out2 == 222)
assert_true(out1 != out2)

// Test 11: Channel Capacity and Backpressure
test_start("channel capacity and backpressure")
sus (backpressure_sender, backpressure_receiver) := make_buffered_channel(2)

// Fill channel to capacity
backpressure_sender.send(1)
backpressure_sender.send(2)

// Try non-blocking send (should fail)
sus send_result := backpressure_sender.try_send(3)
assert_false(send_result.is_ok())

// Receive one value
sus val := backpressure_receiver.recv()
assert_eq_int(val, 1)

// Now send should succeed
sus send_result2 := backpressure_sender.try_send(3)
assert_true(send_result2.is_ok())

// Test 12: Async/Await Integration
test_start("async/await integration")
sus future_result := await async_task()
assert_eq_int(future_result, 42)

// Test 13: Goroutine Panic Recovery
test_start("goroutine panic recovery")
sus (panic_sender, panic_receiver) := make_channel()

yolo {
    defer {
        panic_sender.send(based)
    }
    
    // This should panic and trigger defer
    sus result := 1 / 0
}

sus panic_handled := panic_receiver.recv()
assert_true(panic_handled)

// Test 14: Channel Deadlock Detection
test_start("channel deadlock detection")
sus (deadlock_sender, deadlock_receiver) := make_channel()

// This should not deadlock due to timeout
ready {
    deadlock_receiver -> {
        sus val := deadlock_receiver.recv()
        vibez.spill("Should not reach here")
    }
    timeout(100) -> {
        vibez.spill("Deadlock prevented by timeout")
    }
}

// Test 15: High-Performance Channel Operations
test_start("high-performance channel operations")
sus (perf_sender, perf_receiver) := make_buffered_channel(1000)

// High-volume send/receive
sus start_time := time.now()
bestie i := 0; i < 1000; i++ {
    perf_sender.send(i)
}

bestie i := 0; i < 1000; i++ {
    sus val := perf_receiver.recv()
    assert_eq_int(val, i)
}
sus end_time := time.now()

sus duration := end_time - start_time
assert_true(duration < 1000) // Should complete within 1 second

print_test_summary()

// Helper function for async test
slay async_task() async drip {
    await sleep(10)
    damn 42
}
