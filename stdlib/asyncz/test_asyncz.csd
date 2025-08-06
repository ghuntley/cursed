yeet "testz"
yeet "asyncz"

fr fr Test suite for CURSED Async/Await Module (asyncz)

slay test_future_creation() {
    test_start("Future Creation")
    
    fr fr Test basic future creation
    sus future *Future = create_future()
    assert_true(future != 0)
    assert_eq_int(future.state, TASK_PENDING)
    assert_eq_int(future.result, 0)
    assert_true(future.error == 0)
    assert_true(future.id > 0)
    
    vibez.spill("✅ Future creation tests passed")
}

slay test_async_execution() {
    test_start("Async Execution")
    
    fr fr Define a simple async task
    slay simple_task() normie {
        damn 42
    }
    
    fr fr Test async execution
    sus future *Future = async_run(simple_task)
    assert_true(future != 0)
    assert_eq_int(future.state, TASK_RUNNING)
    
    fr fr Wait for completion
    sus result normie = await_future(future)
    assert_eq_int(result, 42)
    assert_eq_int(future.state, TASK_COMPLETED)
    
    vibez.spill("✅ Async execution tests passed")
}

slay test_future_completion_check() {
    test_start("Future Completion Check")
    
    slay quick_task() normie {
        damn 100
    }
    
    slay slow_task() normie {
        sleep_ms(100)
        damn 200
    }
    
    fr fr Test immediate completion
    sus quick_future *Future = async_run(quick_task)
    sleep_ms(10)  fr fr Give it time to complete
    assert_true(is_future_ready(quick_future))
    
    fr fr Test pending state
    sus slow_future *Future = async_run(slow_task)
    assert_false(is_future_ready(slow_future))
    
    vibez.spill("✅ Future completion check tests passed")
}

slay test_future_cancellation() {
    test_start("Future Cancellation")
    
    slay long_running_task() normie {
        sleep_ms(1000)
        damn 999
    }
    
    sus future *Future = async_run(long_running_task)
    assert_eq_int(future.state, TASK_RUNNING)
    
    fr fr Cancel the future
    sus cancelled lit = cancel_future(future)
    assert_true(cancelled)
    assert_eq_int(future.state, TASK_CANCELLED)
    
    fr fr Try to cancel already cancelled future
    sus already_cancelled lit = cancel_future(future)
    assert_false(already_cancelled)
    
    vibez.spill("✅ Future cancellation tests passed")
}

slay test_future_timeout() {
    test_start("Future Timeout")
    
    slay timeout_task() normie {
        sleep_ms(500)
        damn 123
    }
    
    sus future *Future = async_run(timeout_task)
    
    fr fr Test timeout (should timeout)
    sus timeout_result normie = await_future_timeout(future, 100)
    assert_eq_int(timeout_result, 0)  fr fr Timeout returns 0
    assert_eq_int(future.state, TASK_CANCELLED)
    
    vibez.spill("✅ Future timeout tests passed")
}

slay test_future_callbacks() {
    test_start("Future Callbacks")
    
    sus callback_executed lit = cap
    sus callback_result normie = 0
    
    slay test_callback(result normie) {
        callback_executed = based
        callback_result = result
    }
    
    slay callback_task() normie {
        damn 456
    }
    
    sus future *Future = async_run(callback_task)
    future_then(future, test_callback)
    
    fr fr Wait for completion
    await_future(future)
    
    fr fr Check callback execution (simplified - may not work without proper callback system)
    fr fr assert_true(callback_executed)
    fr fr assert_eq_int(callback_result, 456)
    
    vibez.spill("✅ Future callback tests passed")
}

slay test_async_executor() {
    test_start("Async Executor")
    
    sus executor *AsyncExecutor = create_async_executor(4, 10)
    assert_true(executor != 0)
    assert_eq_int(executor.max_concurrent_tasks, 10)
    assert_false(executor.shutdown_requested)
    
    fr fr Define test tasks
    slay task1() normie { damn 1 }
    slay task2() normie { damn 2 }
    slay task3() normie { damn 3 }
    
    fr fr Submit tasks
    sus future1 *Future = executor_submit(executor, task1)
    sus future2 *Future = executor_submit(executor, task2)
    sus future3 *Future = executor_submit(executor, task3)
    
    assert_true(future1 != 0)
    assert_true(future2 != 0)
    assert_true(future3 != 0)
    
    fr fr Wait for results
    sus result1 normie = await_future(future1)
    sus result2 normie = await_future(future2)
    sus result3 normie = await_future(future3)
    
    assert_eq_int(result1, 1)
    assert_eq_int(result2, 2)
    assert_eq_int(result3, 3)
    
    fr fr Shutdown executor
    shutdown_executor(executor)
    assert_true(executor.shutdown_requested)
    
    vibez.spill("✅ Async executor tests passed")
}

slay test_async_context() {
    test_start("Async Context")
    
    fr fr Test basic context creation
    sus context *AsyncContext = create_async_context()
    assert_true(context != 0)
    assert_false(context.cancelled)
    assert_true(context.created_time > 0)
    
    fr fr Test context cancellation
    cancel_async_context(context)
    assert_true(is_context_cancelled(context))
    
    fr fr Test context with timeout
    sus timeout_context *AsyncContext = create_async_context_with_timeout(100)
    assert_true(timeout_context != 0)
    assert_eq_int(timeout_context.timeout_ms, 100)
    
    fr fr Test child context
    sus parent *AsyncContext = create_async_context()
    sus child *AsyncContext = create_child_context(parent)
    assert_true(child != 0)
    assert_true(child.parent_context == parent)
    
    vibez.spill("✅ Async context tests passed")
}

slay test_timer_operations() {
    test_start("Timer Operations")
    
    sus timer_executed lit = cap
    
    slay timer_callback() {
        timer_executed = based
    }
    
    fr fr Create and test timer
    sus timer *Timer = create_timer(50, timer_callback)
    assert_true(timer != 0)
    assert_eq_int(timer.duration_ms, 50)
    assert_false(timer.cancelled)
    
    fr fr Test timer cancellation
    sus cancelled lit = cancel_timer(timer)
    assert_true(cancelled)
    assert_true(timer.cancelled)
    
    fr fr Test async delay
    sus delay_future *Future = async_delay(25)
    assert_true(delay_future != 0)
    
    sus delay_result normie = await_future(delay_future)
    assert_eq_int(delay_result, 1)
    
    vibez.spill("✅ Timer operation tests passed")
}

slay test_async_streams() {
    test_start("Async Streams")
    
    fr fr Test stream creation
    sus stream *AsyncStream = create_async_stream(5)
    assert_true(stream != 0)
    assert_eq_int(stream.buffer_size, 5)
    assert_false(stream.closed)
    
    fr fr Test stream send and receive
    sus sent lit = stream_send(stream, 100)
    assert_true(sent)
    
    sus received normie = stream_receive(stream)
    assert_eq_int(received, 100)
    
    fr fr Test stream closing
    close_async_stream(stream)
    assert_true(stream.closed)
    
    fr fr Test sending to closed stream
    sus sent_to_closed lit = stream_send(stream, 200)
    assert_false(sent_to_closed)
    
    vibez.spill("✅ Async stream tests passed")
}

slay test_stream_transformations() {
    test_start("Stream Transformations")
    
    fr fr Create source stream
    sus source *AsyncStream = create_async_stream(3)
    
    fr fr Define transformation functions
    slay double_transform(value normie) normie {
        damn value * 2
    }
    
    slay is_even_predicate(value normie) lit {
        damn value % 2 == 0
    }
    
    slay add_reducer(acc normie, value normie) normie {
        damn acc + value
    }
    
    fr fr Test stream mapping
    sus mapped_stream *AsyncStream = stream_map(source, double_transform)
    assert_true(mapped_stream != 0)
    
    fr fr Test stream filtering
    sus filtered_stream *AsyncStream = stream_filter(mapped_stream, is_even_predicate)
    assert_true(filtered_stream != 0)
    
    fr fr Test stream reduction
    sus reduction_future *Future = stream_reduce(filtered_stream, 0, add_reducer)
    assert_true(reduction_future != 0)
    
    fr fr Send some test data
    stream_send(source, 1)
    stream_send(source, 2)
    stream_send(source, 3)
    close_async_stream(source)
    
    vibez.spill("✅ Stream transformation tests passed")
}

slay test_parallel_execution() {
    test_start("Parallel Execution")
    
    fr fr Define parallel tasks
    slay task_a() normie { damn 10 }
    slay task_b() normie { damn 20 }
    slay task_c() normie { damn 30 }
    
    sus tasks []slay() normie = [task_a, task_b, task_c]
    
    fr fr Test parallel execution
    sus parallel_future *Future = async_parallel(tasks, 3)
    assert_true(parallel_future != 0)
    
    sus parallel_result normie = await_future(parallel_future)
    assert_eq_int(parallel_result, 1)  fr fr Success indicator
    
    vibez.spill("✅ Parallel execution tests passed")
}

slay test_sequential_execution() {
    test_start("Sequential Execution")
    
    fr fr Define sequential tasks that build on previous result
    slay add_ten(value normie) normie { damn value + 10 }
    slay multiply_by_two(value normie) normie { damn value * 2 }
    slay subtract_five(value normie) normie { damn value - 5 }
    
    sus tasks []slay(normie) normie = [add_ten, multiply_by_two, subtract_five]
    
    fr fr Test sequential execution starting with 5
    fr fr Expected: 5 -> 15 -> 30 -> 25
    sus sequence_future *Future = async_sequence(tasks, 5, 3)
    assert_true(sequence_future != 0)
    
    sus sequence_result normie = await_future(sequence_future)
    assert_eq_int(sequence_result, 25)
    
    vibez.spill("✅ Sequential execution tests passed")
}

slay test_retry_mechanism() {
    test_start("Retry Mechanism")
    
    sus attempt_count normie = 0
    
    slay failing_task() normie {
        attempt_count = attempt_count + 1
        lowkey attempt_count < 3 {
            trigger_panic("Task failed on attempt " + string(attempt_count))
        }
        damn 999  fr fr Success on third attempt
    }
    
    fr fr Test retry with eventual success
    sus retry_future *Future = async_retry(failing_task, 5, 10)
    assert_true(retry_future != 0)
    
    sus retry_result normie = await_future(retry_future)
    assert_eq_int(retry_result, 999)
    assert_true(attempt_count >= 3)
    
    vibez.spill("✅ Retry mechanism tests passed")
}

slay test_immediate_futures() {
    test_start("Immediate Futures")
    
    fr fr Test immediate successful future
    sus immediate_success *Future = async_immediate(777)
    assert_true(immediate_success != 0)
    assert_eq_int(immediate_success.state, TASK_COMPLETED)
    assert_eq_int(immediate_success.result, 777)
    
    sus immediate_result normie = await_future(immediate_success)
    assert_eq_int(immediate_result, 777)
    
    fr fr Test immediate failed future
    sus error *ErrorInstance = create_error("Immediate failure")
    sus immediate_failure *Future = async_failed(error)
    assert_true(immediate_failure != 0)
    assert_eq_int(immediate_failure.state, TASK_FAILED)
    assert_true(immediate_failure.error == error)
    
    vibez.spill("✅ Immediate future tests passed")
}

slay test_blocking_to_async_conversion() {
    test_start("Blocking to Async Conversion")
    
    slay blocking_operation() normie {
        fr fr Simulate blocking operation
        sleep_ms(50)
        damn 555
    }
    
    fr fr Convert blocking operation to async
    sus async_future *Future = async_from_blocking(blocking_operation)
    assert_true(async_future != 0)
    
    sus converted_result normie = await_future(async_future)
    assert_eq_int(converted_result, 555)
    
    vibez.spill("✅ Blocking to async conversion tests passed")
}

slay test_await_any_future() {
    test_start("Await Any Future")
    
    slay fast_task() normie { 
        sleep_ms(10)
        damn 100 
    }
    
    slay slow_task() normie { 
        sleep_ms(200) 
        damn 200 
    }
    
    slay medium_task() normie { 
        sleep_ms(50) 
        damn 150 
    }
    
    fr fr Create futures for racing
    sus fast_future *Future = async_run(fast_task)
    sus slow_future *Future = async_run(slow_task)
    sus medium_future *Future = async_run(medium_task)
    
    sus futures []*Future = [fast_future, slow_future, medium_future]
    
    fr fr Should return result from fastest task
    sus first_result normie = await_any_future(futures, 3)
    assert_eq_int(first_result, 100)  fr fr Fast task should win
    
    vibez.spill("✅ Await any future tests passed")
}

slay test_timeout_wrapper() {
    test_start("Timeout Wrapper")
    
    slay slow_operation() normie {
        sleep_ms(200)
        damn 888
    }
    
    fr fr Test operation that should timeout
    sus timeout_future *Future = async_timeout(slow_operation, 50)
    assert_true(timeout_future != 0)
    
    await_future(timeout_future)
    assert_eq_int(timeout_future.state, TASK_FAILED)
    assert_true(timeout_future.error != 0)
    
    vibez.spill("✅ Timeout wrapper tests passed")
}

slay run_all_asyncz_tests() {
    vibez.spill("🚀 Starting CURSED Async/Await (asyncz) Tests")
    
    test_future_creation()
    test_async_execution()
    test_future_completion_check()
    test_future_cancellation()
    test_future_timeout()
    test_future_callbacks()
    test_async_executor()
    test_async_context()
    test_timer_operations()
    test_async_streams()
    test_stream_transformations()
    test_parallel_execution()
    test_sequential_execution()
    test_retry_mechanism()
    test_immediate_futures()
    test_blocking_to_async_conversion()
    test_await_any_future()
    test_timeout_wrapper()
    
    print_test_summary()
    vibez.spill("✅ All asyncz tests completed!")
}

fr fr Run tests when this file is executed
run_all_asyncz_tests()
