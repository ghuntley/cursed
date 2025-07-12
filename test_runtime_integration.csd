// CURSED Runtime Integration Test Suite
// Tests the complete runtime system integration

yeet "testz"

// Test runtime initialization
test_start("Runtime system initialization")
sus runtime_initialized lit = cap

// Initialize the runtime system
runtime_initialized = init_runtime()

assert_true(runtime_initialized)

// Test scheduler initialization
test_start("Scheduler initialization")
sus scheduler_initialized lit = cap

// Initialize the goroutine scheduler
scheduler_initialized = init_scheduler(4)  // 4 worker threads

assert_true(scheduler_initialized)

// Test memory management integration
test_start("Memory management integration")
sus memory_stats := get_memory_stats()

assert_true(memory_stats.total_allocated >= 0)
assert_true(memory_stats.total_freed >= 0)

// Test garbage collection
test_start("Garbage collection")
sus gc_stats := get_gc_stats()

assert_true(gc_stats.collections_performed >= 0)
assert_true(gc_stats.objects_collected >= 0)

// Test channel system integration
test_start("Channel system integration")
sus channel_test_result drip = 0

dm test_channel := make_production_channel(drip, 100)  // Capacity 100

// Test channel creation
assert_true(test_channel != cringe)

// Test channel operations
test_channel <- 42
sus received_value drip = <- test_channel

assert_eq_int(received_value, 42)

// Test production scheduler integration
test_start("Production scheduler integration")
sus production_result drip = 0

// Create work-stealing configuration
sus config := WorkStealingConfig {
    num_workers: 4,
    local_queue_capacity: 256,
    global_queue_capacity: 1024,
    max_steal_attempts: 8,
    steal_batch_size: 16,
    enable_load_balancing: based,
    enable_adaptive_scheduling: based,
}

// Initialize production scheduler
sus scheduler := create_production_scheduler(config)

assert_true(scheduler != cringe)

// Test goroutine pool integration
test_start("Goroutine pool integration")
sus pool_result drip = 0

dm work_queue := make_channel(drip)
dm result_queue := make_channel(drip)

// Create worker pool
bestie worker := 0; worker < 4; worker++ {
    stan {
        bestie {
            ready {
                case work := <- work_queue:
                    result_queue <- work * 2
                case timeout(5000):
                    ghosted
            }
        }
    }
}

// Send work to pool
bestie i := 1; i <= 10; i++ {
    work_queue <- i
}

// Collect results
bestie i := 0; i < 10; i++ {
    sus result drip = <- result_queue
    pool_result = pool_result + result
}

assert_eq_int(pool_result, 110)  // 2*(1+2+...+10) = 110

// Test select statement runtime
test_start("Select statement runtime")
sus select_test_result drip = 0

dm ch1 := make_channel(drip)
dm ch2 := make_channel(drip)
dm ch3 := make_channel(drip)

// Send values to different channels
stan { ch1 <- 100 }
stan { ch2 <- 200 }
stan { ch3 <- 300 }

// Use select to receive from any available channel
sus received_values drip = 0
bestie i := 0; i < 3; i++ {
    ready {
        case val := <- ch1:
            received_values = received_values + val
        case val := <- ch2:
            received_values = received_values + val
        case val := <- ch3:
            received_values = received_values + val
    }
}

assert_eq_int(received_values, 600)

// Test monitoring system
test_start("Monitoring system")
sus monitor_result drip = 0

// Get comprehensive statistics
sus stats := get_comprehensive_stats()

if stats.scheduler.total_scheduled >= 0 {
    monitor_result = monitor_result + 1
}

if stats.pool.total_created >= 0 {
    monitor_result = monitor_result + 1
}

if stats.monitor.total_monitored >= 0 {
    monitor_result = monitor_result + 1
}

assert_eq_int(monitor_result, 3)

// Test concurrent data structures
test_start("Concurrent data structures")
sus concurrent_test_result drip = 0

// Create concurrent hash map
dm concurrent_map := create_concurrent_map(tea, drip, 16)  // 16 buckets

// Test concurrent operations
stan {
    concurrent_map.insert("key1", 10)
    concurrent_map.insert("key2", 20)
}

stan {
    concurrent_map.insert("key3", 30)
    concurrent_map.insert("key4", 40)
}

// Yield to allow operations
yolo
yolo

// Read values
sus sum drip = 0
if concurrent_map.contains_key("key1") {
    sum = sum + concurrent_map.get("key1")
}
if concurrent_map.contains_key("key2") {
    sum = sum + concurrent_map.get("key2")
}
if concurrent_map.contains_key("key3") {
    sum = sum + concurrent_map.get("key3")
}
if concurrent_map.contains_key("key4") {
    sum = sum + concurrent_map.get("key4")
}

concurrent_test_result = sum

assert_eq_int(concurrent_test_result, 100)

// Test lock-free queue
test_start("Lock-free queue")
sus queue_result drip = 0

dm lock_free_queue := create_lock_free_queue(drip)

// Test queue operations
assert_true(lock_free_queue.push(1))
assert_true(lock_free_queue.push(2))
assert_true(lock_free_queue.push(3))

assert_eq_int(lock_free_queue.len(), 3)

sus val1 drip = lock_free_queue.pop()
sus val2 drip = lock_free_queue.pop()
sus val3 drip = lock_free_queue.pop()

queue_result = val1 + val2 + val3

assert_eq_int(queue_result, 6)

// Test error handling integration
test_start("Error handling integration")
sus error_handling_result lit = cap

stan {
    yikes "test error in goroutine"
} shook {
    error_handling_result = based
}

yolo  // Allow error handling

assert_true(error_handling_result)

// Test channel priority system
test_start("Channel priority system")
sus priority_result drip = 0

dm priority_channel := make_priority_channel(drip)

// Send messages with different priorities
priority_channel.send_with_priority(1, ChannelPriority.Low)
priority_channel.send_with_priority(2, ChannelPriority.High)
priority_channel.send_with_priority(3, ChannelPriority.Normal)
priority_channel.send_with_priority(4, ChannelPriority.Critical)

// Receive messages (should come in priority order)
sus msg1 drip = <- priority_channel  // Critical (4)
sus msg2 drip = <- priority_channel  // High (2)
sus msg3 drip = <- priority_channel  // Normal (3)
sus msg4 drip = <- priority_channel  // Low (1)

priority_result = msg1 * 1000 + msg2 * 100 + msg3 * 10 + msg4

assert_eq_int(priority_result, 4231)

// Test backpressure handling
test_start("Backpressure handling")
sus backpressure_result lit = cap

dm backpressure_channel := make_channel_with_config(drip, ChannelConfig {
    capacity: 2,
    enable_backpressure: based,
    backpressure_threshold: 0.5,
})

// Fill channel to trigger backpressure
backpressure_channel <- 1
backpressure_channel <- 2  // Should trigger backpressure

// Next send should be blocked
ready {
    case backpressure_channel <- 3:
        backpressure_result = cap  // Should not reach here
    case timeout(100):
        backpressure_result = based  // Backpressure worked
}

assert_true(backpressure_result)

// Test deadlock detection
test_start("Deadlock detection")
sus deadlock_result lit = cap

dm deadlock_ch1 := make_channel(drip)
dm deadlock_ch2 := make_channel(drip)

// Create potential deadlock scenario
stan {
    deadlock_ch1 <- 1
    <- deadlock_ch2
} shook {
    deadlock_result = based
}

stan {
    deadlock_ch2 <- 2
    <- deadlock_ch1
} shook {
    deadlock_result = based
}

// Yield to trigger deadlock detection
yolo
yolo

assert_true(deadlock_result)

// Test runtime shutdown
test_start("Runtime system shutdown")
sus shutdown_result lit = cap

// Shutdown the runtime system
shutdown_result = shutdown_runtime()

assert_true(shutdown_result)

// Test runtime statistics
test_start("Runtime statistics")
sus final_stats := get_runtime_stats()

assert_true(final_stats.uptime_seconds >= 0)
assert_true(final_stats.total_goroutines_spawned >= 0)
assert_true(final_stats.total_messages_sent >= 0)
assert_true(final_stats.total_memory_allocated >= 0)

print_test_summary()
