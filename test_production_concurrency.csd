// CURSED Production Concurrency Test Suite
// Tests enterprise-grade goroutine and channel features

yeet "testz"

// Test basic goroutine spawning and execution
test_start("Basic goroutine spawning")
sus goroutine_result drip = 0

// Spawn a goroutine using "stan" keyword
stan {
    goroutine_result = 42
}

// Wait for goroutine to complete
yolo  // Yield to allow goroutine to execute

assert_eq_int(goroutine_result, 42)

// Test multiple goroutines
test_start("Multiple goroutines")
sus counter drip = 0

// Spawn multiple goroutines
stan {
    counter = counter + 1
}

stan {
    counter = counter + 10
}

stan {
    counter = counter + 100
}

// Yield to allow all goroutines to execute
yolo
yolo
yolo

assert_eq_int(counter, 111)

// Test channel communication using "dm" keyword
test_start("Basic channel communication")
sus channel_received drip = 0

// Create a channel
dm msg_channel := make_channel(drip)

// Spawn sender goroutine
stan {
    msg_channel <- 99  // Send value
}

// Spawn receiver goroutine
stan {
    channel_received = <- msg_channel  // Receive value
}

// Yield to allow goroutines to execute
yolo
yolo

assert_eq_int(channel_received, 99)

// Test buffered channels
test_start("Buffered channel communication")
sus buffered_result drip = 0

// Create buffered channel with capacity 3
dm buffered_channel := make_buffered_channel(drip, 3)

// Send multiple values without blocking
buffered_channel <- 1
buffered_channel <- 2
buffered_channel <- 3

// Receive values
sus val1 drip = <- buffered_channel
sus val2 drip = <- buffered_channel
sus val3 drip = <- buffered_channel

buffered_result = val1 + val2 + val3

assert_eq_int(buffered_result, 6)

// Test select statement using "ready" keyword
test_start("Select statement communication")
sus select_result drip = 0

dm ch1 := make_channel(drip)
dm ch2 := make_channel(drip)

// Spawn senders
stan {
    ch1 <- 100
}

stan {
    ch2 <- 200
}

// Use select to receive from first available channel
ready {
    case val := <- ch1:
        select_result = val
    case val := <- ch2:
        select_result = val
}

assert_true(select_result == 100 || select_result == 200)

// Test goroutine error handling
test_start("Goroutine error isolation")
sus error_handled lit = cap

stan {
    yikes "test error"  // This should be isolated
} shook {
    error_handled = based
}

assert_true(error_handled)

// Test channel closing
test_start("Channel closing")
sus close_result lit = cap

dm close_channel := make_channel(drip)

// Close the channel
close(close_channel)

// Try to receive from closed channel
ready {
    case val := <- close_channel:
        close_result = cap  // Should not reach here
    case closed := <- close_channel:
        close_result = based  // Channel is closed
}

assert_true(close_result)

// Test goroutine priorities
test_start("Goroutine priority scheduling")
sus priority_order tea = ""

// High priority goroutine
stan high_priority {
    priority_order = priority_order + "H"
}

// Normal priority goroutine
stan normal_priority {
    priority_order = priority_order + "N"
}

// Low priority goroutine
stan low_priority {
    priority_order = priority_order + "L"
}

// Yield to allow execution
yolo
yolo
yolo

// High priority should execute first
assert_true(priority_order[0] == "H")

// Test channel with timeout
test_start("Channel timeout operations")
sus timeout_result lit = cap

dm timeout_channel := make_channel(drip)

// Try to receive with timeout
ready {
    case val := <- timeout_channel:
        timeout_result = cap  // Should not reach here
    case timeout(1000):  // 1 second timeout
        timeout_result = based  // Timeout occurred
}

assert_true(timeout_result)

// Test concurrent data structures
test_start("Concurrent data structures")
sus concurrent_result drip = 0

// Create a concurrent map
dm concurrent_map := make_concurrent_map(tea, drip)

// Spawn multiple goroutines to write to map
stan {
    concurrent_map.put("key1", 10)
}

stan {
    concurrent_map.put("key2", 20)
}

stan {
    concurrent_map.put("key3", 30)
}

// Yield to allow writes
yolo
yolo
yolo

// Read values
sus val1 drip = concurrent_map.get("key1")
sus val2 drip = concurrent_map.get("key2")
sus val3 drip = concurrent_map.get("key3")

concurrent_result = val1 + val2 + val3

assert_eq_int(concurrent_result, 60)

// Test work-stealing scheduler
test_start("Work-stealing scheduler")
sus work_stealing_result drip = 0

// Spawn many goroutines to test work distribution
bestie i := 0; i < 100; i++ {
    stan {
        work_stealing_result = work_stealing_result + 1
    }
}

// Yield to allow all goroutines to execute
bestie j := 0; j < 10; j++ {
    yolo
}

assert_eq_int(work_stealing_result, 100)

// Test channel multiplexing
test_start("Channel multiplexing")
sus multiplex_result drip = 0

dm in_channel := make_channel(drip)
dm out_channel1 := make_channel(drip)
dm out_channel2 := make_channel(drip)

// Multiplexer goroutine
stan {
    bestie {
        sus val drip = <- in_channel
        if val == 0 {
            ghosted  // Break on sentinel value
        }
        out_channel1 <- val
        out_channel2 <- val * 2
    }
}

// Send values
in_channel <- 5
in_channel <- 10
in_channel <- 0  // Sentinel

// Receive from multiplexed channels
sus result1 drip = <- out_channel1
sus result2 drip = <- out_channel1
sus result3 drip = <- out_channel2
sus result4 drip = <- out_channel2

multiplex_result = result1 + result2 + result3 + result4

assert_eq_int(multiplex_result, 45)  // 5 + 10 + 10 + 20

// Test goroutine pools
test_start("Goroutine pool management")
sus pool_result drip = 0

// Create a pool of worker goroutines
dm work_channel := make_buffered_channel(drip, 100)
dm result_channel := make_buffered_channel(drip, 100)

// Spawn worker pool
bestie worker := 0; worker < 10; worker++ {
    stan {
        bestie {
            ready {
                case work := <- work_channel:
                    result_channel <- work * 2
                case timeout(5000):
                    ghosted  // Exit on timeout
            }
        }
    }
}

// Send work to pool
bestie task := 1; task <= 20; task++ {
    work_channel <- task
}

// Collect results
bestie result := 0; result < 20; result++ {
    sus value drip = <- result_channel
    pool_result = pool_result + value
}

assert_eq_int(pool_result, 420)  // Sum of 2*1 + 2*2 + ... + 2*20

// Test deadlock detection
test_start("Deadlock detection")
sus deadlock_detected lit = cap

dm deadlock_ch1 := make_channel(drip)
dm deadlock_ch2 := make_channel(drip)

// Spawn potentially deadlocking goroutines
stan {
    deadlock_ch1 <- 1
    <- deadlock_ch2  // This could deadlock
} shook {
    deadlock_detected = based
}

stan {
    deadlock_ch2 <- 2
    <- deadlock_ch1  // This could deadlock
} shook {
    deadlock_detected = based
}

// Yield to trigger deadlock detection
yolo
yolo

assert_true(deadlock_detected)

// Test channel backpressure
test_start("Channel backpressure handling")
sus backpressure_result lit = cap

dm backpressure_channel := make_buffered_channel(drip, 2)

// Fill channel to capacity
backpressure_channel <- 1
backpressure_channel <- 2

// This should trigger backpressure
ready {
    case backpressure_channel <- 3:
        backpressure_result = cap  // Should not reach here
    case timeout(100):
        backpressure_result = based  // Backpressure triggered
}

assert_true(backpressure_result)

// Test goroutine monitoring
test_start("Goroutine monitoring")
sus monitor_result drip = 0

// Get scheduler statistics
sus scheduler_stats := get_scheduler_stats()

// Check if statistics are available
if scheduler_stats.active_goroutines >= 0 {
    monitor_result = 1
}

if scheduler_stats.total_goroutines_spawned > 0 {
    monitor_result = monitor_result + 1
}

assert_eq_int(monitor_result, 2)

// Test memory efficient channels
test_start("Memory efficient channels")
sus memory_result drip = 0

// Create large number of channels to test memory efficiency
dm channels := make_array(Channel(drip), 1000)

bestie i := 0; i < 1000; i++ {
    channels[i] = make_channel(drip)
}

// Use some channels
bestie i := 0; i < 100; i++ {
    channels[i] <- i
    sus val drip = <- channels[i]
    memory_result = memory_result + val
}

assert_eq_int(memory_result, 4950)  // Sum of 0 to 99

// Test channel statistics
test_start("Channel statistics")
sus stats_result drip = 0

dm stats_channel := make_buffered_channel(drip, 10)

// Send some messages
bestie i := 0; i < 5; i++ {
    stats_channel <- i
}

// Receive some messages
bestie i := 0; i < 3; i++ {
    <- stats_channel
}

// Get channel statistics
sus channel_stats := get_channel_stats(stats_channel)

if channel_stats.total_sent == 5 {
    stats_result = stats_result + 1
}

if channel_stats.total_received == 3 {
    stats_result = stats_result + 1
}

if channel_stats.current_length == 2 {
    stats_result = stats_result + 1
}

assert_eq_int(stats_result, 3)

// Test panic recovery in goroutines
test_start("Panic recovery in goroutines")
sus panic_recovered lit = cap

stan {
    panic("test panic")
} shook {
    panic_recovered = based
}

// Yield to allow panic recovery
yolo

assert_true(panic_recovered)

// Test channel fan-out pattern
test_start("Channel fan-out pattern")
sus fanout_result drip = 0

dm source := make_channel(drip)
dm sink1 := make_channel(drip)
dm sink2 := make_channel(drip)
dm sink3 := make_channel(drip)

// Fan-out goroutine
stan {
    bestie {
        sus val drip = <- source
        if val == 0 {
            ghosted
        }
        sink1 <- val
        sink2 <- val
        sink3 <- val
    }
}

// Send values
source <- 5
source <- 10
source <- 0  // Stop signal

// Receive from all sinks
sus val1 drip = <- sink1
sus val2 drip = <- sink1
sus val3 drip = <- sink2
sus val4 drip = <- sink2
sus val5 drip = <- sink3
sus val6 drip = <- sink3

fanout_result = val1 + val2 + val3 + val4 + val5 + val6

assert_eq_int(fanout_result, 30)  // 5*3 + 10*3

// Test channel fan-in pattern
test_start("Channel fan-in pattern")
sus fanin_result drip = 0

dm source1 := make_channel(drip)
dm source2 := make_channel(drip)
dm source3 := make_channel(drip)
dm sink := make_channel(drip)

// Fan-in goroutine
stan {
    bestie {
        ready {
            case val := <- source1:
                sink <- val
            case val := <- source2:
                sink <- val
            case val := <- source3:
                sink <- val
        }
    }
}

// Send values to different sources
source1 <- 1
source2 <- 2
source3 <- 3

// Receive from sink
sus total drip = 0
bestie i := 0; i < 3; i++ {
    total = total + <- sink
}

fanin_result = total

assert_eq_int(fanin_result, 6)

// Test pipeline pattern
test_start("Pipeline pattern")
sus pipeline_result drip = 0

dm stage1 := make_channel(drip)
dm stage2 := make_channel(drip)
dm stage3 := make_channel(drip)

// Pipeline stage 1: double the value
stan {
    bestie {
        sus val drip = <- stage1
        if val == 0 {
            stage2 <- 0
            ghosted
        }
        stage2 <- val * 2
    }
}

// Pipeline stage 2: add 10
stan {
    bestie {
        sus val drip = <- stage2
        if val == 0 {
            stage3 <- 0
            ghosted
        }
        stage3 <- val + 10
    }
}

// Send values through pipeline
stage1 <- 5
stage1 <- 15
stage1 <- 0  // Stop signal

// Receive processed values
sus result1 drip = <- stage3
sus result2 drip = <- stage3

pipeline_result = result1 + result2

assert_eq_int(pipeline_result, 70)  // (5*2+10) + (15*2+10) = 20 + 40 = 60

// Test error propagation in pipelines
test_start("Error propagation in pipelines")
sus error_propagated lit = cap

dm error_pipeline := make_channel(drip)
dm error_output := make_channel(drip)

stan {
    bestie {
        sus val drip = <- error_pipeline
        if val < 0 {
            yikes "negative value"
        }
        error_output <- val * 2
    }
} shook {
    error_propagated = based
}

// Send negative value to trigger error
error_pipeline <- -5

// Yield to allow error propagation
yolo

assert_true(error_propagated)

print_test_summary()
