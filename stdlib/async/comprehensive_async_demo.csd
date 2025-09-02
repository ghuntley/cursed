// Comprehensive CURSED Async System Demo
// Demonstrates all features of the native async/future system

yeet "async"

slay main() {
    vibez.spill("=== CURSED Native Async System Demo ===")
    
    // Initialize async system
    async_init()
    
    // Demonstrate basic async operations
    demo_basic_async()
    
    // Demonstrate future combinators
    demo_future_combinators()
    
    // Demonstrate task management
    demo_task_management()
    
    // Demonstrate async primitives
    demo_async_primitives()
    
    // Demonstrate async pipeline
    demo_async_pipeline()
    
    // Demonstrate concurrent operations
    demo_concurrent_operations()
    
    // Demonstrate error handling
    demo_error_handling()
    
    // Show final metrics
    demo_metrics()
    
    vibez.spill("=== Demo Complete ===")
}

slay demo_basic_async() {
    vibez.spill("\n--- Basic Async Operations ---")
    
    // Create a simple async operation
    sus future *Future = async_delay("Hello Async World", 100)
    vibez.spill("Created async operation")
    
    // Create and execute task
    sus task *Task = Task.new("basic_demo", future)
    vibez.spill("Task created: " + task.name)
    vibez.spill("Task state: " + tea(task.state))
    
    // Spawn task
    sus spawn_success lit = spawn(task)
    if spawn_success {
        vibez.spill("Task spawned successfully")
    }
    
    // Simple async function
    sus fn_future *Future = async_fn(slay() extra {
        vibez.spill("Inside async function")
        damn "Function result"
    })
    
    vibez.spill("Async function created")
}

slay demo_future_combinators() {
    vibez.spill("\n--- Future Combinators ---")
    
    // Create base futures
    sus future1 *Future = resolved(10)
    sus future2 *Future = resolved(20)
    sus future3 *Future = resolved(30)
    
    // Test 'then' combinator
    sus then_future *Future = then(future1, slay(x extra) extra {
        vibez.spill("Processing value: " + tea(x))
        damn normie(x) * 2
    })
    
    vibez.spill("Created 'then' combinator")
    
    // Test 'and_then' combinator
    sus and_then_future *Future = and_then(future1, future2)
    vibez.spill("Created 'and_then' combinator")
    
    // Test 'or_else' combinator
    sus or_else_future *Future = or_else(future1, future2)
    vibez.spill("Created 'or_else' combinator")
    
    // Test 'join' combinator
    sus futures []*Future = []
    futures = append(futures, future1)
    futures = append(futures, future2)
    futures = append(futures, future3)
    
    sus joined_future *Future = join(futures)
    vibez.spill("Created 'join' combinator for " + tea(len(futures)) + " futures")
    
    // Test resolved and rejected utilities
    sus resolved_future *Future = resolved(42)
    sus rejected_future *Future = rejected("Demo error")
    
    vibez.spill("Resolved future ready: " + tea(resolved_future.is_ready()))
    vibez.spill("Rejected future has error: " + tea(rejected_future.has_error()))
}

slay demo_task_management() {
    vibez.spill("\n--- Task Management ---")
    
    // Create tasks with different priorities
    sus high_future *Future = resolved("High priority task")
    sus high_task *Task = Task.new_with_priority("high_priority", high_future, TaskPriority.High)
    
    sus normal_future *Future = resolved("Normal priority task")
    sus normal_task *Task = Task.new_with_priority("normal_priority", normal_future, TaskPriority.Normal)
    
    sus low_future *Future = resolved("Low priority task")
    sus low_task *Task = Task.new_with_priority("low_priority", low_future, TaskPriority.Low)
    
    vibez.spill("Created tasks with priorities:")
    vibez.spill("  High: " + high_task.name)
    vibez.spill("  Normal: " + normal_task.name)
    vibez.spill("  Low: " + low_task.name)
    
    // Test task builder
    sus builder *TaskBuilder = TaskBuilder.new()
    sus built_task *Task = builder
        .with_name("built_task")
        .with_priority(TaskPriority.Critical)
        .with_future(resolved("Built task result"))
        .build()
    
    vibez.spill("Built task: " + built_task.name + " with priority " + tea(built_task.priority))
    
    // Test parent-child relationships
    high_task.add_child(normal_task)
    vibez.spill("Added child task relationship")
    vibez.spill("High task children: " + tea(len(high_task.children_ids)))
    
    // Test task cancellation
    sus cancel_task *Task = Task.new("cancel_demo", resolved("Will be cancelled"))
    cancel_task.cancel()
    vibez.spill("Task cancelled: " + tea(cancel_task.cancelled))
    vibez.spill("Task state after cancel: " + tea(cancel_task.state))
}

slay demo_async_primitives() {
    vibez.spill("\n--- Async Primitives ---")
    
    // Test async channel
    sus channel *AsyncChannel = AsyncChannel.new(3)
    vibez.spill("Created async channel with capacity: " + tea(channel.capacity))
    
    // Test channel operations
    sus send_future *Future = channel.send("Message 1")
    vibez.spill("Sent message to channel, size: " + tea(channel.len()))
    
    sus recv_future *Future = channel.receive()
    vibez.spill("Receiving from channel")
    
    // Test async mutex
    sus mutex *AsyncMutex = AsyncMutex.new()
    vibez.spill("Created async mutex")
    
    sus try_lock_success lit = mutex.try_lock()
    vibez.spill("Mutex try_lock success: " + tea(try_lock_success))
    vibez.spill("Mutex is locked: " + tea(mutex.is_locked()))
    
    mutex.unlock()
    vibez.spill("Mutex unlocked: " + tea(!mutex.is_locked()))
    
    // Test async semaphore
    sus semaphore *AsyncSemaphore = AsyncSemaphore.new(2)
    vibez.spill("Created semaphore with " + tea(semaphore.available_permits()) + " permits")
    
    sus acquire_future *Future = semaphore.acquire()
    vibez.spill("Acquired semaphore permit, remaining: " + tea(semaphore.available_permits()))
    
    semaphore.release()
    vibez.spill("Released semaphore permit, available: " + tea(semaphore.available_permits()))
    
    // Test async barrier
    sus barrier *AsyncBarrier = AsyncBarrier.new(3)
    vibez.spill("Created barrier for " + tea(barrier.total) + " tasks")
    
    sus wait_future *Future = barrier.wait()
    vibez.spill("Waiting on barrier, count: " + tea(barrier.get_count()))
    
    // Test async select
    sus select_op *AsyncSelect = AsyncSelect.new()
    sus channel1 *AsyncChannel = AsyncChannel.new(1)
    sus channel2 *AsyncChannel = AsyncChannel.new(1)
    
    select_op.add_send_case(channel1, "Select message 1")
    select_op.add_receive_case(channel2)
    select_op.add_default_case()
    
    vibez.spill("Created select with " + tea(len(select_op.cases)) + " cases")
    
    // Test async timeout
    sus timeout *AsyncTimeout = AsyncTimeout.new(1000)
    vibez.spill("Created timeout with duration: " + tea(timeout.duration))
    vibez.spill("Timeout expired: " + tea(timeout.is_expired()))
}

slay demo_async_pipeline() {
    vibez.spill("\n--- Async Pipeline ---")
    
    // Create processing pipeline
    sus pipeline *AsyncPipeline = AsyncPipeline.new()
    
    // Add stages
    pipeline.add_stage("input", slay(x extra) *Future {
        vibez.spill("Pipeline input stage: " + tea(x))
        damn resolved(tea(x) + " -> input")
    })
    
    pipeline.add_stage("process", slay(x extra) *Future {
        vibez.spill("Pipeline process stage: " + tea(x))
        damn resolved(tea(x) + " -> processed")
    })
    
    pipeline.add_stage_with_timeout("output", slay(x extra) *Future {
        vibez.spill("Pipeline output stage: " + tea(x))
        damn resolved(tea(x) + " -> output")
    }, 5000)
    
    // Add error handler
    pipeline.with_error_handler(slay(error tea) extra {
        vibez.spill("Pipeline error handler: " + error)
        damn "Error handled: " + error
    })
    
    vibez.spill("Created pipeline with " + tea(len(pipeline.stages)) + " stages")
    
    // Execute pipeline
    sus pipeline_future *Future = pipeline.execute("test data")
    vibez.spill("Pipeline execution started")
}

slay demo_concurrent_operations() {
    vibez.spill("\n--- Concurrent Operations ---")
    
    // Test async map
    sus values extra[value] = []
    values = append(values, 1)
    values = append(values, 2)
    values = append(values, 3)
    values = append(values, 4)
    
    sus map_future *Future = async_map(values, slay(x extra) *Future {
        vibez.spill("Processing value: " + tea(x))
        damn resolved(normie(x) * 2)
    })
    
    vibez.spill("Created async map for " + tea(len(values)) + " values")
    
    // Test async reduce
    sus reduce_future *Future = async_reduce(values, 0, slay(acc extra, x extra) *Future {
        vibez.spill("Reducing: " + tea(acc) + " + " + tea(x))
        damn resolved(normie(acc) + normie(x))
    })
    
    vibez.spill("Created async reduce operation")
    
    // Test async filter
    sus filter_future *Future = async_filter(values, slay(x extra) *Future {
        sus is_even lit = (normie(x) % 2) == 0
        vibez.spill("Filtering value " + tea(x) + ", even: " + tea(is_even))
        damn resolved(is_even)
    })
    
    vibez.spill("Created async filter operation")
    
    // Test async race
    sus race_futures []*Future = []
    race_futures = append(race_futures, async_delay("First", 100))
    race_futures = append(race_futures, async_delay("Second", 200))
    race_futures = append(race_futures, async_delay("Third", 300))
    
    sus race_future *Future = async_race(race_futures)
    vibez.spill("Created async race with " + tea(len(race_futures)) + " futures")
    
    // Test async collect
    sus collect_future *Future = async_collect(race_futures)
    vibez.spill("Created async collect operation")
}

slay demo_error_handling() {
    vibez.spill("\n--- Error Handling ---")
    
    // Test basic error future
    sus error_future *Future = rejected("Demo error message")
    vibez.spill("Created error future: " + tea(error_future.has_error()))
    
    if error_future.has_error() {
        vibez.spill("Error message: " + error_future.get_error())
    }
    
    // Test retry mechanism
    sus retry_count normie = 0
    sus retry_future *Future = async_retry(slay() *Future {
        retry_count++
        vibez.spill("Retry attempt: " + tea(retry_count))
        
        if retry_count < 3 {
            damn rejected("Retry attempt " + tea(retry_count) + " failed")
        } else {
            damn resolved("Success after " + tea(retry_count) + " attempts")
        }
    }, 5, 100)
    
    vibez.spill("Created retry operation with max 5 attempts")
    
    // Test task error handling
    sus error_task_future *Future = rejected("Task error")
    sus error_task *Task = Task.new("error_demo", error_task_future)
    
    // Simulate task execution
    error_task.state = TaskState.Error
    error_task.error_msg = "Simulated task error"
    
    vibez.spill("Task has error: " + tea(error_task.has_error()))
    if error_task.has_error() {
        vibez.spill("Task error: " + error_task.error_msg)
    }
}

slay demo_metrics() {
    vibez.spill("\n--- Async Metrics ---")
    
    // Get and display metrics
    sus metrics *AsyncMetrics = get_async_metrics()
    vibez.spill("Total tasks: " + tea(metrics.total_tasks))
    vibez.spill("Completed tasks: " + tea(metrics.completed_tasks))
    vibez.spill("Failed tasks: " + tea(metrics.failed_tasks))
    vibez.spill("Average execution time: " + tea(metrics.average_execution_time))
    vibez.spill("Max execution time: " + tea(metrics.max_execution_time))
    vibez.spill("Min execution time: " + tea(metrics.min_execution_time))
    
    // Create some tasks for metrics
    sus demo_task1 *Task = Task.new("metrics_demo_1", resolved("Task 1"))
    sus demo_task2 *Task = Task.new("metrics_demo_2", resolved("Task 2"))
    sus demo_task3 *Task = Task.new("metrics_demo_3", rejected("Task 3 error"))
    
    // Simulate task completion
    demo_task1.state = TaskState.Completed
    demo_task2.state = TaskState.Completed
    demo_task3.state = TaskState.Error
    
    // Update metrics
    update_async_metrics(demo_task1)
    update_async_metrics(demo_task2)
    update_async_metrics(demo_task3)
    
    vibez.spill("Updated metrics:")
    vibez.spill("  Total: " + tea(metrics.total_tasks))
    vibez.spill("  Completed: " + tea(metrics.completed_tasks))
    vibez.spill("  Failed: " + tea(metrics.failed_tasks))
    
    // Test executor stats
    sus executor *SingleThreadedExecutor = SingleThreadedExecutor.new()
    sus executor_stats *TaskStats = executor.get_stats()
    vibez.spill("Executor queue size: " + tea(executor.get_queue_size()))
    vibez.spill("Executor is running: " + tea(executor.is_running()))
    vibez.spill("Executor is shutdown: " + tea(executor.is_shutdown()))
}

// Helper functions
slay async_delay(value extra, duration normie) *Future {
    sus future *BasicFuture = BasicFuture.new()
    // For demo purposes, immediately resolve
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
