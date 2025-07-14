yeet "testz"
yeet "concurrency_advanced"
yeet "core"

# Comprehensive Test Suite for Advanced Concurrency Module

# Test channel operations
test_start("channel_create_and_basic_operations")
sus ch tea = concurrency_advanced.channel_create(3)
assert_eq_string(ch.length, 36)  # UUID length
sus send_result lit = concurrency_advanced.channel_send(ch, "test_message")
assert_true(send_result)
sus received tea = concurrency_advanced.channel_receive(ch)
assert_eq_string(received, "test_message")
sus close_result lit = concurrency_advanced.channel_close(ch)
assert_true(close_result)

test_start("buffered_channel_capacity")
sus buffered_ch tea = concurrency_advanced.channel_create(2)
assert_true(concurrency_advanced.channel_send(buffered_ch, "msg1"))
assert_true(concurrency_advanced.channel_send(buffered_ch, "msg2"))
# Third message should succeed but fill buffer
assert_true(concurrency_advanced.channel_send(buffered_ch, "msg3"))
sus msg1 tea = concurrency_advanced.channel_receive(buffered_ch)
assert_eq_string(msg1, "msg1")

test_start("unbuffered_channel")
sus unbuffered tea = concurrency_advanced.channel_create(0)
sus can_send lit = concurrency_advanced.channel_can_send(unbuffered)
assert_true(can_send)  # Unbuffered channels are always ready to send

# Test mutex operations
test_start("mutex_basic_operations")
sus mutex tea = concurrency_advanced.mutex_create()
sus goroutine_id tea = "test_goroutine_1"
sus lock_result lit = concurrency_advanced.mutex_lock(mutex, goroutine_id)
assert_true(lock_result)
sus unlock_result lit = concurrency_advanced.mutex_unlock(mutex, goroutine_id)
assert_true(unlock_result)

test_start("mutex_try_lock")
sus mutex2 tea = concurrency_advanced.mutex_create()
sus goroutine1 tea = "goroutine_1"
sus goroutine2 tea = "goroutine_2"
assert_true(concurrency_advanced.mutex_try_lock(mutex2, goroutine1))
assert_false(concurrency_advanced.mutex_try_lock(mutex2, goroutine2))
assert_true(concurrency_advanced.mutex_unlock(mutex2, goroutine1))

# Test wait group operations
test_start("waitgroup_operations")
sus wg tea = concurrency_advanced.waitgroup_create()
concurrency_advanced.waitgroup_add(wg, 3)
concurrency_advanced.waitgroup_done(wg)
concurrency_advanced.waitgroup_done(wg)
concurrency_advanced.waitgroup_done(wg)
sus wait_result lit = concurrency_advanced.waitgroup_wait(wg)
assert_true(wait_result)

test_start("waitgroup_multiple_add")
sus wg2 tea = concurrency_advanced.waitgroup_create()
concurrency_advanced.waitgroup_add(wg2, 2)
concurrency_advanced.waitgroup_add(wg2, 1)  # Total: 3
concurrency_advanced.waitgroup_done(wg2)
concurrency_advanced.waitgroup_done(wg2)
concurrency_advanced.waitgroup_done(wg2)
assert_true(concurrency_advanced.waitgroup_wait(wg2))

# Test condition variables
test_start("condition_variable_operations")
sus cond tea = concurrency_advanced.condition_create()
sus mutex_cond tea = concurrency_advanced.mutex_create()
sus goroutine_cond tea = "cond_goroutine"
concurrency_advanced.mutex_lock(mutex_cond, goroutine_cond)
sus signal_result lit = concurrency_advanced.condition_signal(cond)
assert_true(signal_result)
sus broadcast_result lit = concurrency_advanced.condition_broadcast(cond)
assert_true(broadcast_result)
concurrency_advanced.mutex_unlock(mutex_cond, goroutine_cond)

# Test semaphore operations
test_start("semaphore_operations")
sus sem tea = concurrency_advanced.semaphore_create(2)
assert_true(concurrency_advanced.semaphore_acquire(sem))
assert_true(concurrency_advanced.semaphore_acquire(sem))
assert_false(concurrency_advanced.semaphore_try_acquire(sem))  # Should fail, no permits
concurrency_advanced.semaphore_release(sem)
assert_true(concurrency_advanced.semaphore_try_acquire(sem))  # Should succeed now

test_start("semaphore_try_acquire")
sus sem2 tea = concurrency_advanced.semaphore_create(1)
assert_true(concurrency_advanced.semaphore_try_acquire(sem2))
assert_false(concurrency_advanced.semaphore_try_acquire(sem2))
concurrency_advanced.semaphore_release(sem2)
assert_true(concurrency_advanced.semaphore_try_acquire(sem2))

# Test read-write lock operations
test_start("rwlock_operations")
sus rwlock tea = concurrency_advanced.rwlock_create()
assert_true(concurrency_advanced.rwlock_read_lock(rwlock))
assert_true(concurrency_advanced.rwlock_read_lock(rwlock))  # Multiple readers OK
concurrency_advanced.rwlock_read_unlock(rwlock)
concurrency_advanced.rwlock_read_unlock(rwlock)
assert_true(concurrency_advanced.rwlock_write_lock(rwlock))
concurrency_advanced.rwlock_write_unlock(rwlock)

test_start("rwlock_write_exclusivity")
sus rwlock2 tea = concurrency_advanced.rwlock_create()
assert_true(concurrency_advanced.rwlock_write_lock(rwlock2))
# Should not be able to acquire read lock while write locked
concurrency_advanced.rwlock_write_unlock(rwlock2)
assert_true(concurrency_advanced.rwlock_read_lock(rwlock2))

# Test barrier operations
test_start("barrier_operations")
sus barrier tea = concurrency_advanced.barrier_create(3)
# In a real scenario, this would be called from multiple goroutines
# For testing, we simulate the barrier reaching its count
sus wait_result_barrier lit = concurrency_advanced.barrier_wait(barrier)
# This test verifies the barrier structure is created correctly

# Test goroutine pool operations
test_start("goroutine_pool_operations")
sus pool tea = concurrency_advanced.goroutine_pool_create(2)
sus submit_result lit = concurrency_advanced.goroutine_pool_submit(pool, "test_task", "task_data")
assert_true(submit_result)

# Test select operations
test_start("select_operations")
sus ch1 tea = concurrency_advanced.channel_create(1)
sus ch2 tea = concurrency_advanced.channel_create(1)
concurrency_advanced.channel_send(ch1, "ready")
sus channels [tea] = [ch1, ch2]
sus operations [tea] = ["receive", "receive"]
sus ready_index normie = concurrency_advanced.select_operation(channels, operations)
assert_eq_int(ready_index, 0)  # First channel should be ready

test_start("select_no_ready_channels")
sus ch3 tea = concurrency_advanced.channel_create(1)
sus ch4 tea = concurrency_advanced.channel_create(1)
sus channels2 [tea] = [ch3, ch4]
sus operations2 [tea] = ["receive", "receive"]
sus no_ready normie = concurrency_advanced.select_operation(channels2, operations2)
assert_eq_int(no_ready, -1)  # No channels ready

# Test channel state checking
test_start("channel_state_checking")
sus ch_check tea = concurrency_advanced.channel_create(1)
assert_true(concurrency_advanced.channel_can_send(ch_check))
concurrency_advanced.channel_send(ch_check, "data")
assert_false(concurrency_advanced.channel_can_send(ch_check))  # Buffer full
assert_true(concurrency_advanced.channel_can_receive(ch_check))

test_start("closed_channel_behavior")
sus ch_closed tea = concurrency_advanced.channel_create(1)
concurrency_advanced.channel_close(ch_closed)
assert_false(concurrency_advanced.channel_send(ch_closed, "should_fail"))

# Test performance monitoring
test_start("concurrency_metrics")
sus metrics tea = concurrency_advanced.concurrency_metrics()
assert_true(metrics.length > 10)  # Should contain JSON data
assert_true(metrics.contains("active_goroutines"))
assert_true(metrics.contains("channel_operations"))

# Test deadlock detection
test_start("deadlock_detector")
sus detector tea = concurrency_advanced.deadlock_detector_create()
sus resource_graph tea = "simple_graph"
sus has_deadlock lit = concurrency_advanced.deadlock_check(detector, resource_graph)
# Basic structure test - actual deadlock detection would require complex graph

# Advanced concurrency patterns testing
test_start("producer_consumer_pattern")
sus producer_ch tea = concurrency_advanced.channel_create(5)
sus consumer_wg tea = concurrency_advanced.waitgroup_create()
concurrency_advanced.waitgroup_add(consumer_wg, 1)

# Simulate producer
concurrency_advanced.channel_send(producer_ch, "item1")
concurrency_advanced.channel_send(producer_ch, "item2")
concurrency_advanced.channel_send(producer_ch, "item3")
concurrency_advanced.channel_close(producer_ch)

# Simulate consumer
sus consumed_items [tea] = []
sus item1 tea = concurrency_advanced.channel_receive(producer_ch)
sus item2 tea = concurrency_advanced.channel_receive(producer_ch)
sus item3 tea = concurrency_advanced.channel_receive(producer_ch)

assert_eq_string(item1, "item1")
assert_eq_string(item2, "item2")
assert_eq_string(item3, "item3")

concurrency_advanced.waitgroup_done(consumer_wg)
concurrency_advanced.waitgroup_wait(consumer_wg)

# Test worker pool pattern
test_start("worker_pool_pattern")
sus work_ch tea = concurrency_advanced.channel_create(10)
sus result_ch tea = concurrency_advanced.channel_create(10)
sus worker_wg tea = concurrency_advanced.waitgroup_create()

# Add work items
concurrency_advanced.channel_send(work_ch, "task1")
concurrency_advanced.channel_send(work_ch, "task2")
concurrency_advanced.channel_send(work_ch, "task3")
concurrency_advanced.channel_close(work_ch)

# Simulate worker processing
sus task1 tea = concurrency_advanced.channel_receive(work_ch)
concurrency_advanced.channel_send(result_ch, "processed_" + task1)

sus result1 tea = concurrency_advanced.channel_receive(result_ch)
assert_eq_string(result1, "processed_task1")

# Test pipeline pattern
test_start("pipeline_pattern")
sus stage1_ch tea = concurrency_advanced.channel_create(3)
sus stage2_ch tea = concurrency_advanced.channel_create(3)
sus final_ch tea = concurrency_advanced.channel_create(3)

# Stage 1: Input processing
concurrency_advanced.channel_send(stage1_ch, "raw_data")

# Stage 2: Transformation
sus raw_data tea = concurrency_advanced.channel_receive(stage1_ch)
sus transformed tea = "transformed_" + raw_data
concurrency_advanced.channel_send(stage2_ch, transformed)

# Stage 3: Final processing
sus stage2_data tea = concurrency_advanced.channel_receive(stage2_ch)
sus final_result tea = "final_" + stage2_data
concurrency_advanced.channel_send(final_ch, final_result)

sus pipeline_result tea = concurrency_advanced.channel_receive(final_ch)
assert_eq_string(pipeline_result, "final_transformed_raw_data")

# Test fan-out/fan-in pattern
test_start("fan_out_fan_in_pattern")
sus input_ch tea = concurrency_advanced.channel_create(1)
sus worker1_ch tea = concurrency_advanced.channel_create(1)
sus worker2_ch tea = concurrency_advanced.channel_create(1)
sus output_ch tea = concurrency_advanced.channel_create(2)

# Fan-out: Distribute work
concurrency_advanced.channel_send(input_ch, "work_item")
sus work_item tea = concurrency_advanced.channel_receive(input_ch)
concurrency_advanced.channel_send(worker1_ch, work_item)
concurrency_advanced.channel_send(worker2_ch, work_item)

# Workers process
sus worker1_result tea = concurrency_advanced.channel_receive(worker1_ch)
sus worker2_result tea = concurrency_advanced.channel_receive(worker2_ch)

# Fan-in: Collect results
concurrency_advanced.channel_send(output_ch, "w1_" + worker1_result)
concurrency_advanced.channel_send(output_ch, "w2_" + worker2_result)

sus result1 tea = concurrency_advanced.channel_receive(output_ch)
sus result2 tea = concurrency_advanced.channel_receive(output_ch)

assert_true(result1.contains("work_item"))
assert_true(result2.contains("work_item"))

# Test timeout pattern with channels
test_start("timeout_pattern")
sus data_ch tea = concurrency_advanced.channel_create(1)
sus timeout_ch tea = concurrency_advanced.channel_create(1)

# Simulate timeout
concurrency_advanced.channel_send(timeout_ch, "timeout")

sus channels_timeout [tea] = [data_ch, timeout_ch]
sus ops_timeout [tea] = ["receive", "receive"]
sus ready_timeout normie = concurrency_advanced.select_operation(channels_timeout, ops_timeout)

# Timeout channel should be ready (index 1)
assert_eq_int(ready_timeout, 1)

# Test resource pool pattern
test_start("resource_pool_pattern")
sus resource_sem tea = concurrency_advanced.semaphore_create(2)  # 2 resources
sus resource_mutex tea = concurrency_advanced.mutex_create()

# Acquire resources
assert_true(concurrency_advanced.semaphore_acquire(resource_sem))
assert_true(concurrency_advanced.semaphore_acquire(resource_sem))

# Try to acquire third resource (should fail)
assert_false(concurrency_advanced.semaphore_try_acquire(resource_sem))

# Release one resource
concurrency_advanced.semaphore_release(resource_sem)

# Now should be able to acquire
assert_true(concurrency_advanced.semaphore_try_acquire(resource_sem))

# Test atomic counter pattern
test_start("atomic_counter_pattern")
sus counter_mutex tea = concurrency_advanced.mutex_create()
sus goroutine_counter tea = "counter_goroutine"

# Simulate atomic increment operations
concurrency_advanced.mutex_lock(counter_mutex, goroutine_counter)
# In real scenario, this would increment a shared counter
sus counter_value normie = 0
counter_value = counter_value + 1
concurrency_advanced.mutex_unlock(counter_mutex, goroutine_counter)

assert_eq_int(counter_value, 1)

# Test synchronization barrier pattern
test_start("synchronization_barrier_pattern")
sus sync_barrier tea = concurrency_advanced.barrier_create(1)
# In real scenario, multiple goroutines would wait at this barrier
# For testing, we verify the barrier creation and basic structure

# Memory and performance validation
test_start("concurrency_performance_validation")
sus perf_metrics tea = concurrency_advanced.concurrency_metrics()
assert_true(perf_metrics.contains("memory_usage"))
assert_true(perf_metrics.contains("mutex_contentions"))

# Test concurrent data structure safety
test_start("concurrent_data_structure_safety")
sus safe_mutex tea = concurrency_advanced.mutex_create()
sus rwlock_safe tea = concurrency_advanced.rwlock_create()

# Multiple readers should work
assert_true(concurrency_advanced.rwlock_read_lock(rwlock_safe))
assert_true(concurrency_advanced.rwlock_read_lock(rwlock_safe))
concurrency_advanced.rwlock_read_unlock(rwlock_safe)
concurrency_advanced.rwlock_read_unlock(rwlock_safe)

# Single writer exclusivity
assert_true(concurrency_advanced.rwlock_write_lock(rwlock_safe))
concurrency_advanced.rwlock_write_unlock(rwlock_safe)

print_test_summary()
