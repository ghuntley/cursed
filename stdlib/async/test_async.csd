// CURSED Async System Tests
// Comprehensive tests for the async/future system

yeet "testz"

slay main() {
    vibez.spill("=== CURSED Async System Tests ===")
    
    test_future_basic()
    test_future_combinators()
    test_task_lifecycle()
    test_task_execution()
    test_executor_basic()
    test_executor_scheduling()
    test_async_channel()
    test_async_mutex()
    test_async_select()
    test_async_timeout()
    test_async_primitives()
    test_async_pipeline()
    test_async_utilities()
    test_async_integration()
    test_async_error_handling()
    test_async_cancellation()
    
    print_test_summary()
}

// Future Tests
slay test_future_basic() {
    test_start("Future Basic Operations")
    
    // Test basic future creation
    sus future *BasicFuture = BasicFuture.new()
    assert_eq_int(future.state, PollState.Pending)
    assert_false(future.is_ready())
    assert_false(future.has_error())
    
    // Test setting ready
    future.set_ready(42)
    assert_eq_int(future.state, PollState.Ready)
    assert_true(future.is_ready())
    assert_eq_int(future.get_result(), 42)
    
    // Test error state
    sus error_future *BasicFuture = BasicFuture.new()
    error_future.set_error("test error")
    assert_eq_int(error_future.state, PollState.Error)
    assert_true(error_future.has_error())
    assert_eq_string(error_future.get_error(), "test error")
    
    // Test resolved utility
    sus resolved_future *Future = resolved(123)
    assert_true(resolved_future.is_ready())
    assert_eq_int(resolved_future.get_result(), 123)
    
    // Test rejected utility
    sus rejected_future *Future = rejected("error message")
    assert_true(rejected_future.has_error())
    assert_eq_string(rejected_future.get_error(), "error message")
}

slay test_future_combinators() {
    test_start("Future Combinators")
    
    // Test 'then' combinator
    sus base_future *Future = resolved(10)
    sus then_future *Future = then(base_future, slay(x extra) extra {
        damn normie(x) * 2
    })
    
    // Note: Would need proper async execution here
    assert_true(then_future != cringe)
    
    // Test 'and_then' combinator
    sus future1 *Future = resolved(1)
    sus future2 *Future = resolved(2)
    sus and_then_future *Future = and_then(future1, future2)
    assert_true(and_then_future != cringe)
    
    // Test 'or_else' combinator
    sus or_else_future *Future = or_else(future1, future2)
    assert_true(or_else_future != cringe)
    
    // Test 'join' combinator
    sus futures []*Future = []
    futures = append(futures, resolved(1))
    futures = append(futures, resolved(2))
    futures = append(futures, resolved(3))
    
    sus joined_future *Future = join(futures)
    assert_true(joined_future != cringe)
}

// Task Tests
slay test_task_lifecycle() {
    test_start("Task Lifecycle")
    
    sus future *Future = resolved(42)
    sus task *Task = Task.new("test_task", future)
    
    assert_eq_string(task.name, "test_task")
    assert_eq_int(task.state, TaskState.Created)
    assert_false(task.is_completed())
    assert_true(task.is_ready())
    assert_false(task.has_error())
    
    // Test task completion
    task.state = TaskState.Completed
    task.result = 42
    assert_true(task.is_completed())
    assert_eq_int(task.get_result(), 42)
    
    // Test task cancellation
    sus cancel_task *Task = Task.new("cancel_test", future)
    cancel_task.cancel()
    assert_eq_int(cancel_task.state, TaskState.Cancelled)
    assert_true(cancel_task.cancelled)
    assert_true(cancel_task.is_completed())
}

slay test_task_execution() {
    test_start("Task Execution")
    
    sus future *BasicFuture = BasicFuture.new()
    future.set_ready(100)
    
    sus task *Task = Task.new("exec_test", future)
    sus executor *SingleThreadedExecutor = SingleThreadedExecutor.new()
    sus context *TaskContext = TaskContext.new(executor)
    
    // Execute task
    sus result_state TaskState = task.execute(context)
    assert_eq_int(result_state, TaskState.Completed)
    assert_eq_int(task.get_result(), 100)
    
    // Test task with error
    sus error_future *BasicFuture = BasicFuture.new()
    error_future.set_error("execution error")
    
    sus error_task *Task = Task.new("error_test", error_future)
    sus error_state TaskState = error_task.execute(context)
    assert_eq_int(error_state, TaskState.Error)
    assert_eq_string(error_task.error_msg, "execution error")
}

// Executor Tests
slay test_executor_basic() {
    test_start("Executor Basic Operations")
    
    sus executor *SingleThreadedExecutor = SingleThreadedExecutor.new()
    assert_eq_int(executor.state, ExecutorState.Created)
    assert_eq_int(executor.get_queue_size(), 0)
    assert_true(executor.current_task == cringe)
    
    // Test task spawning
    sus future *Future = resolved(42)
    sus task *Task = Task.new("spawn_test", future)
    sus spawn_success lit = executor.spawn(task)
    assert_true(spawn_success)
    assert_eq_int(executor.get_queue_size(), 1)
    
    // Test executor state changes
    executor.state = ExecutorState.Running
    assert_true(executor.is_running())
    assert_false(executor.is_shutdown())
    
    executor.state = ExecutorState.Stopped
    assert_false(executor.is_running())
    assert_true(executor.is_shutdown())
}

slay test_executor_scheduling() {
    test_start("Executor Scheduling")
    
    sus executor *SingleThreadedExecutor = SingleThreadedExecutor.new()
    
    // Test priority scheduling
    sus low_future *Future = resolved(1)
    sus low_task *Task = Task.new_with_priority("low", low_future, TaskPriority.Low)
    
    sus high_future *Future = resolved(2)
    sus high_task *Task = Task.new_with_priority("high", high_future, TaskPriority.High)
    
    executor.spawn(low_task)
    executor.spawn(high_task)
    
    // High priority task should be at front of queue
    sus next_task *Task = executor.task_queue.peek()
    assert_eq_string(next_task.name, "high")
    assert_eq_int(next_task.priority, TaskPriority.High)
}

// Async Channel Tests
slay test_async_channel() {
    test_start("Async Channel Operations")
    
    sus channel *AsyncChannel = AsyncChannel.new(2)
    assert_eq_int(channel.capacity, 2)
    assert_eq_int(channel.size, 0)
    assert_false(channel.is_closed())
    
    // Test channel send
    sus send_future *Future = channel.send(42)
    assert_true(send_future != cringe)
    assert_eq_int(channel.size, 1)
    
    // Test channel receive
    sus recv_future *Future = channel.receive()
    assert_true(recv_future != cringe)
    
    // Test channel close
    channel.close()
    assert_true(channel.is_closed())
}

slay test_async_mutex() {
    test_start("Async Mutex Operations")
    
    sus mutex *AsyncMutex = AsyncMutex.new()
    assert_false(mutex.is_locked())
    
    // Test try_lock
    sus try_success lit = mutex.try_lock()
    assert_true(try_success)
    assert_true(mutex.is_locked())
    
    // Test second try_lock fails
    sus try_fail lit = mutex.try_lock()
    assert_false(try_fail)
    
    // Test unlock
    mutex.unlock()
    assert_false(mutex.is_locked())
    
    // Test async lock
    sus lock_future *Future = mutex.lock()
    assert_true(lock_future != cringe)
    assert_true(mutex.is_locked())
}

slay test_async_select() {
    test_start("Async Select Operations")
    
    sus select_op *AsyncSelect = AsyncSelect.new()
    assert_true(select_op != cringe)
    assert_false(select_op.completed)
    
    // Test adding cases
    sus channel1 *AsyncChannel = AsyncChannel.new(1)
    sus channel2 *AsyncChannel = AsyncChannel.new(1)
    
    select_op.add_send_case(channel1, 42)
    select_op.add_receive_case(channel2)
    select_op.add_default_case()
    
    assert_eq_int(len(select_op.cases), 2)
    assert_true(select_op.default_case != cringe)
    
    // Test select execution
    sus select_future *Future = select_op.execute()
    assert_true(select_future != cringe)
}

slay test_async_timeout() {
    test_start("Async Timeout Operations")
    
    sus timeout *AsyncTimeout = AsyncTimeout.new(1000)
    assert_eq_int(timeout.duration, 1000)
    assert_false(timeout.is_expired())
    
    // Test timeout creation
    sus slow_future *Future = async_delay(42, 2000)
    sus timeout_future *Future = with_timeout(slow_future, 1000)
    assert_true(timeout_future != cringe)
    
    // Test timeout cancellation
    timeout.cancel()
    assert_true(timeout.is_expired())
}

slay test_async_primitives() {
    test_start("Async Primitives")
    
    // Test semaphore
    sus semaphore *AsyncSemaphore = AsyncSemaphore.new(2)
    assert_eq_int(semaphore.available_permits(), 2)
    
    sus acquire_future *Future = semaphore.acquire()
    assert_true(acquire_future != cringe)
    assert_eq_int(semaphore.available_permits(), 1)
    
    semaphore.release()
    assert_eq_int(semaphore.available_permits(), 2)
    
    // Test barrier
    sus barrier *AsyncBarrier = AsyncBarrier.new(3)
    assert_eq_int(barrier.total, 3)
    assert_eq_int(barrier.get_count(), 0)
    
    sus wait_future *Future = barrier.wait()
    assert_true(wait_future != cringe)
    assert_eq_int(barrier.get_count(), 1)
}

slay test_async_pipeline() {
    test_start("Async Pipeline")
    
    sus pipeline *AsyncPipeline = AsyncPipeline.new()
    assert_true(pipeline != cringe)
    assert_eq_int(len(pipeline.stages), 0)
    
    // Test adding stages
    pipeline.add_stage("stage1", slay(x extra) *Future {
        damn resolved(normie(x) + 1)
    })
    
    pipeline.add_stage_with_timeout("stage2", slay(x extra) *Future {
        damn resolved(normie(x) * 2)
    }, 1000)
    
    assert_eq_int(len(pipeline.stages), 2)
    assert_eq_string(pipeline.stages[0].name, "stage1")
    assert_eq_string(pipeline.stages[1].name, "stage2")
    assert_eq_int(pipeline.stages[1].timeout, 1000)
    
    // Test pipeline execution
    sus execute_future *Future = pipeline.execute(5)
    assert_true(execute_future != cringe)
}

slay test_async_utilities() {
    test_start("Async Utilities")
    
    // Test async_fn
    sus fn_future *Future = async_fn(slay() extra {
        damn 42
    })
    assert_true(fn_future != cringe)
    
    // Test async_sleep
    sus sleep_future *Future = async_sleep(100)
    assert_true(sleep_future != cringe)
    
    // Test async_delay
    sus delay_future *Future = async_delay("test", 100)
    assert_true(delay_future != cringe)
    
    // Test async_map
    sus values []extra = []
    values = append(values, 1)
    values = append(values, 2)
    values = append(values, 3)
    
    sus map_future *Future = async_map(values, slay(x extra) *Future {
        damn resolved(normie(x) * 2)
    })
    assert_true(map_future != cringe)
    
    // Test async_race
    sus race_futures []*Future = []
    race_futures = append(race_futures, async_delay(1, 100))
    race_futures = append(race_futures, async_delay(2, 200))
    
    sus race_future *Future = async_race(race_futures)
    assert_true(race_future != cringe)
}

slay test_async_integration() {
    test_start("Async Integration")
    
    // Test with existing goroutine system
    sus integration_future *Future = resolved(42)
    async_goroutine(integration_future)
    
    // Test async metrics
    sus metrics *AsyncMetrics = get_async_metrics()
    assert_true(metrics != cringe)
    assert_eq_int(metrics.total_tasks, 0)
    
    // Test task builder
    sus builder *TaskBuilder = TaskBuilder.new()
    sus built_task *Task = builder
        .with_name("built_task")
        .with_priority(TaskPriority.High)
        .with_future(resolved(123))
        .build()
    
    assert_eq_string(built_task.name, "built_task")
    assert_eq_int(built_task.priority, TaskPriority.High)
    
    update_async_metrics(built_task)
    assert_eq_int(metrics.total_tasks, 1)
}

slay test_async_error_handling() {
    test_start("Async Error Handling")
    
    // Test error propagation
    sus error_future *Future = rejected("test error")
    assert_true(error_future.has_error())
    assert_eq_string(error_future.get_error(), "test error")
    
    // Test pipeline error handling
    sus pipeline *AsyncPipeline = AsyncPipeline.new()
    pipeline.with_error_handler(slay(error tea) extra {
        damn "handled: " + error
    })
    
    assert_true(pipeline.error_handler != cringe)
    
    // Test retry mechanism
    sus retry_future *Future = async_retry(slay() *Future {
        damn rejected("always fails")
    }, 3, 100)
    
    assert_true(retry_future != cringe)
}

slay test_async_cancellation() {
    test_start("Async Cancellation")
    
    // Test task cancellation
    sus cancel_future *Future = async_delay(42, 10000)
    sus cancel_task *Task = Task.new("cancel_test", cancel_future)
    
    assert_false(cancel_task.cancelled)
    cancel_task.cancel()
    assert_true(cancel_task.cancelled)
    assert_eq_int(cancel_task.state, TaskState.Cancelled)
    
    // Test timeout cancellation
    sus timeout *AsyncTimeout = AsyncTimeout.new(1000)
    assert_false(timeout.is_expired())
    timeout.cancel()
    assert_true(timeout.is_expired())
    
    // Test executor shutdown
    sus executor *SingleThreadedExecutor = SingleThreadedExecutor.new()
    executor.state = ExecutorState.Running
    executor.shutdown()
    assert_true(executor.shutdown_channel != cringe)
}

// Helper functions for testing
slay async_delay(value extra, duration normie) *Future {
    sus future *BasicFuture = BasicFuture.new()
    // Note: In real implementation, would spawn goroutine with delay
    future.set_ready(value)
    damn future
}

slay resolved(value extra) *Future {
    sus future *BasicFuture = BasicFuture.new()
    future.set_ready(value)
    damn future
}

slay rejected(error tea) *Future {
    sus future *BasicFuture = BasicFuture.new()
    future.set_error(error)
    damn future
}
