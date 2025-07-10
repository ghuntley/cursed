// Goroutine Scheduler Advanced Features Test
// Tests: work-stealing, load balancing, preemption, priority scheduling

yeet "testz"

// Test 1: Work-Stealing Scheduler
test_start("work-stealing scheduler")

// Initialize scheduler with multiple workers
sus scheduler := GoroutineScheduler.new_with_config(SchedulerConfig{
    num_workers: 4,
    enable_work_stealing: based,
    max_goroutines_per_worker: 100,
    quantum_ms: 10,
})

scheduler.start()

// Spawn many goroutines to test work distribution
sus results := make_buffered_channel(100)
bestie i := 0; i < 100; i++ {
    yolo {
        sus work_result := i * 2
        results.send(work_result)
    }
}

// Collect results
sus collected := 0
bestie i := 0; i < 100; i++ {
    sus result := results.recv()
    collected++
}

assert_eq_int(collected, 100)
scheduler.stop()

// Test 2: Goroutine Priority Scheduling
test_start("goroutine priority scheduling")

sus priority_results := make_buffered_channel(10)

// Spawn high-priority goroutine
yolo_with_priority(GoroutinePriority::High) {
    priority_results.send(1)
}

// Spawn normal-priority goroutine
yolo_with_priority(GoroutinePriority::Normal) {
    priority_results.send(2)
}

// Spawn low-priority goroutine
yolo_with_priority(GoroutinePriority::Low) {
    priority_results.send(3)
}

// High-priority should run first
sus first_result := priority_results.recv()
assert_eq_int(first_result, 1)

// Test 3: Goroutine Preemption
test_start("goroutine preemption")

sus preemption_scheduler := GoroutineScheduler.new_with_config(SchedulerConfig{
    num_workers: 2,
    preemptive_scheduling: based,
    quantum_ms: 5,
})

preemption_scheduler.start()

sus preemption_results := make_buffered_channel(2)

// Long-running goroutine
yolo {
    sus counter := 0
    bestie i := 0; i < 1000000; i++ {
        counter++
        if counter % 100000 == 0 {
            yield_now()
        }
    }
    preemption_results.send(1)
}

// Short goroutine that should preempt
yolo {
    preemption_results.send(2)
}

// Short goroutine should complete first due to preemption
sus first_done := preemption_results.recv()
assert_eq_int(first_done, 2)

preemption_scheduler.stop()

// Test 4: Load Balancing
test_start("load balancing")

sus load_balancer := GoroutineScheduler.new_with_config(SchedulerConfig{
    num_workers: 4,
    enable_load_balancing: based,
    load_balance_interval_ms: 10,
})

load_balancer.start()

sus load_results := make_buffered_channel(20)

// Create uneven load
bestie i := 0; i < 20; i++ {
    yolo {
        if i < 10 {
            // Heavy work
            sus sum := 0
            bestie j := 0; j < 10000; j++ {
                sum += j
            }
            load_results.send(sum)
        } else {
            // Light work
            load_results.send(i)
        }
    }
}

// Collect results
sus load_collected := 0
bestie i := 0; i < 20; i++ {
    sus result := load_results.recv()
    load_collected++
}

assert_eq_int(load_collected, 20)
load_balancer.stop()

// Test 5: Goroutine Lifecycle Management
test_start("goroutine lifecycle management")

sus lifecycle_results := make_buffered_channel(3)

// Parent goroutine
sus parent_id := yolo {
    // Spawn child goroutines
    sus child1_id := yolo {
        lifecycle_results.send(1)
    }
    
    sus child2_id := yolo {
        lifecycle_results.send(2)
    }
    
    // Wait for children (simplified)
    yield_now()
    lifecycle_results.send(3)
}

// Collect results
sus lifecycle_collected := 0
bestie i := 0; i < 3; i++ {
    sus result := lifecycle_results.recv()
    lifecycle_collected++
}

assert_eq_int(lifecycle_collected, 3)

// Test 6: Scheduler Statistics
test_start("scheduler statistics")

sus stats_scheduler := GoroutineScheduler.new_with_config(SchedulerConfig{
    num_workers: 2,
    enable_metrics: based,
})

stats_scheduler.start()

// Spawn some goroutines
bestie i := 0; i < 10; i++ {
    yolo {
        sus work := i * i
    }
}

yield_now()

sus stats := stats_scheduler.get_stats()
assert_true(stats.total_goroutines_spawned >= 10)
assert_true(stats.current_active_goroutines >= 0)

stats_scheduler.stop()

// Test 7: Goroutine Memory Management
test_start("goroutine memory management")

sus memory_results := make_buffered_channel(5)

// Spawn goroutines with different stack sizes
bestie i := 0; i < 5; i++ {
    yolo_with_stack_size(1024 * (i + 1)) {
        sus stack_data := make_array(100)
        bestie j := 0; j < 100; j++ {
            stack_data[j] = j
        }
        memory_results.send(len(stack_data))
    }
}

// Collect results
bestie i := 0; i < 5; i++ {
    sus result := memory_results.recv()
    assert_eq_int(result, 100)
}

// Test 8: Goroutine Synchronization Primitives
test_start("goroutine synchronization primitives")

sus mutex := Mutex.new()
sus shared_counter := 0
sus sync_results := make_buffered_channel(10)

// Spawn competing goroutines
bestie i := 0; i < 10; i++ {
    yolo {
        mutex.lock()
        shared_counter++
        sus current_value := shared_counter
        mutex.unlock()
        sync_results.send(current_value)
    }
}

// Collect results
bestie i := 0; i < 10; i++ {
    sus result := sync_results.recv()
    assert_true(result > 0 && result <= 10)
}

assert_eq_int(shared_counter, 10)

// Test 9: Goroutine Error Handling
test_start("goroutine error handling")

sus error_results := make_buffered_channel(2)

// Goroutine that panics
yolo {
    defer {
        error_results.send(1)
    }
    
    panic("Test panic")
}

// Goroutine that handles error
yolo {
    yikes err := risky_operation() {
        error_results.send(2)
    } shook {
        error_results.send(3)
    }
}

// Collect error handling results
sus panic_handled := error_results.recv()
sus error_handled := error_results.recv()

assert_eq_int(panic_handled, 1)
assert_true(error_handled == 2 || error_handled == 3)

// Test 10: Goroutine Deadlock Detection
test_start("goroutine deadlock detection")

sus deadlock_detector := DeadlockDetector.new()
deadlock_detector.start()

sus (deadlock_ch1_sender, deadlock_ch1_receiver) := make_channel()
sus (deadlock_ch2_sender, deadlock_ch2_receiver) := make_channel()

// Potential deadlock scenario
yolo {
    deadlock_ch1_sender.send(1)
    sus val := deadlock_ch2_receiver.recv()
}

yolo {
    deadlock_ch2_sender.send(2)
    sus val := deadlock_ch1_receiver.recv()
}

// Wait for deadlock detection
sus deadlock_detected := deadlock_detector.wait_for_detection(1000)
assert_true(deadlock_detected)

deadlock_detector.stop()

// Test 11: Goroutine Performance Profiling
test_start("goroutine performance profiling")

sus profiler := GoroutineProfiler.new()
profiler.start()

sus profile_results := make_buffered_channel(3)

// CPU-intensive goroutine
yolo {
    sus start_time := time.now()
    sus sum := 0
    bestie i := 0; i < 1000000; i++ {
        sum += i
    }
    sus end_time := time.now()
    profile_results.send(end_time - start_time)
}

// I/O-intensive goroutine
yolo {
    sus start_time := time.now()
    bestie i := 0; i < 100; i++ {
        sleep(1)
    }
    sus end_time := time.now()
    profile_results.send(end_time - start_time)
}

// Memory-intensive goroutine
yolo {
    sus start_time := time.now()
    sus big_array := make_array(10000)
    bestie i := 0; i < 10000; i++ {
        big_array[i] = i
    }
    sus end_time := time.now()
    profile_results.send(end_time - start_time)
}

// Collect profiling results
bestie i := 0; i < 3; i++ {
    sus duration := profile_results.recv()
    assert_true(duration > 0)
}

sus profile_report := profiler.get_report()
assert_true(profile_report.total_goroutines > 0)

profiler.stop()

// Test 12: Advanced Channel Patterns
test_start("advanced channel patterns")

// Fan-out pattern
sus (fan_out_input, fan_out_output) := make_channel()
sus fan_out_results := make_buffered_channel(10)

// Fan-out to multiple workers
bestie i := 0; i < 3; i++ {
    yolo {
        bestie work := range fan_out_output {
            sus result := work * 2
            fan_out_results.send(result)
        }
    }
}

// Send work
bestie i := 1; i <= 9; i++ {
    fan_out_input.send(i)
}
fan_out_input.close()

// Collect fan-out results
sus fan_out_collected := 0
bestie i := 0; i < 9; i++ {
    sus result := fan_out_results.recv()
    fan_out_collected++
}

assert_eq_int(fan_out_collected, 9)

// Fan-in pattern
sus fan_in_inputs := []
bestie i := 0; i < 3; i++ {
    sus (sender, receiver) := make_channel()
    fan_in_inputs = append(fan_in_inputs, receiver)
    
    yolo {
        bestie j := 0; j < 3; j++ {
            sender.send(i * 10 + j)
        }
        sender.close()
    }
}

sus fan_in_results := make_buffered_channel(9)

// Fan-in from multiple sources
yolo {
    bestie input := range fan_in_inputs {
        bestie value := range input {
            fan_in_results.send(value)
        }
    }
}

// Collect fan-in results
sus fan_in_collected := 0
bestie i := 0; i < 9; i++ {
    sus result := fan_in_results.recv()
    fan_in_collected++
}

assert_eq_int(fan_in_collected, 9)

print_test_summary()

// Helper functions
slay risky_operation() error {
    // Simulate risky operation
    damn Error.new("Simulated error")
}

slay make_array(size drip) []drip {
    sus arr := []
    bestie i := 0; i < size; i++ {
        arr = append(arr, i)
    }
    damn arr
}
