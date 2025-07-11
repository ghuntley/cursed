yeet "testz"
yeet "async"

# Async Module Comprehensive Test Suite
# Testing all asynchronous programming functionality

test_start("Event Loop Management Tests")

# Test event loop creation and management
assert_eq_int(async_event_loop_create(), 1)
assert_true(async_event_loop_run(1))
assert_false(async_event_loop_run(-1))
assert_true(async_event_loop_stop(1))
assert_false(async_event_loop_stop(-1))
assert_eq_int(async_event_loop_get_state(1), EVENT_LOOP_IDLE)
assert_eq_int(async_event_loop_get_state(-1), -1)
assert_true(async_event_loop_destroy(1))
assert_false(async_event_loop_destroy(-1))

print_test_summary()

test_start("Async Task Management Tests")

# Test task creation and management
assert_eq_int(async_task_create("test_function"), 1)
assert_eq_int(async_task_create(""), -1)
assert_true(async_task_run(1))
assert_false(async_task_run(-1))
assert_true(async_task_cancel(1))
assert_false(async_task_cancel(-1))
assert_eq_int(async_task_get_state(1), ASYNC_TASK_PENDING)
assert_eq_int(async_task_get_state(-1), -1)
assert_eq_string(async_task_get_result(1), "task_result")
assert_eq_string(async_task_get_result(-1), "")
assert_eq_string(async_task_get_error(1), "task_error")
assert_eq_string(async_task_get_error(-1), "")
assert_true(async_task_is_completed(1))
assert_false(async_task_is_completed(-1))
assert_false(async_task_is_cancelled(1))
assert_true(async_task_is_cancelled(-1))
assert_true(async_task_wait(1))
assert_false(async_task_wait(-1))
assert_true(async_task_wait_timeout(1, 1000))
assert_false(async_task_wait_timeout(-1, 1000))
assert_false(async_task_wait_timeout(1, -1))

print_test_summary()

test_start("Promise Operations Tests")

# Test promise creation and operations
assert_eq_int(async_promise_create(), 1)
assert_true(async_promise_resolve(1, "resolved_value"))
assert_false(async_promise_resolve(-1, "resolved_value"))
assert_true(async_promise_reject(1, "error_message"))
assert_false(async_promise_reject(-1, "error_message"))
assert_eq_int(async_promise_then(1, "success_callback"), 1)
assert_eq_int(async_promise_then(-1, "success_callback"), -1)
assert_eq_int(async_promise_then(1, ""), -1)
assert_eq_int(async_promise_catch(1, "error_callback"), 1)
assert_eq_int(async_promise_catch(-1, "error_callback"), -1)
assert_eq_int(async_promise_catch(1, ""), -1)
assert_eq_int(async_promise_finally(1, "finally_callback"), 1)
assert_eq_int(async_promise_finally(-1, "finally_callback"), -1)
assert_eq_int(async_promise_finally(1, ""), -1)
assert_eq_int(async_promise_get_state(1), PROMISE_PENDING)
assert_eq_int(async_promise_get_state(-1), -1)
assert_eq_string(async_promise_get_value(1), "promise_value")
assert_eq_string(async_promise_get_value(-1), "")
assert_eq_string(async_promise_get_error(1), "promise_error")
assert_eq_string(async_promise_get_error(-1), "")

print_test_summary()

test_start("Async I/O Operations Tests")

# Test async I/O operations
assert_eq_int(async_io_read(1, 1024), 1)
assert_eq_int(async_io_read(-1, 1024), -1)
assert_eq_int(async_io_read(1, 0), -1)
assert_eq_int(async_io_read(1, -1), -1)
assert_eq_int(async_io_write(1, "test_data"), 1)
assert_eq_int(async_io_write(-1, "test_data"), -1)
assert_eq_int(async_io_write(1, ""), -1)
assert_eq_int(async_io_connect("localhost", 8080), 1)
assert_eq_int(async_io_connect("", 8080), -1)
assert_eq_int(async_io_connect("localhost", 0), -1)
assert_eq_int(async_io_connect("localhost", 70000), -1)
assert_eq_int(async_io_listen(8080), 1)
assert_eq_int(async_io_listen(0), -1)
assert_eq_int(async_io_listen(70000), -1)
assert_eq_int(async_io_accept(1), 1)
assert_eq_int(async_io_accept(-1), -1)
assert_eq_int(async_io_get_state(1), ASYNC_IO_READY)
assert_eq_int(async_io_get_state(-1), -1)
assert_eq_string(async_io_get_result(1), "io_result")
assert_eq_string(async_io_get_result(-1), "")
assert_eq_string(async_io_get_error(1), "io_error")
assert_eq_string(async_io_get_error(-1), "")
assert_true(async_io_cancel(1))
assert_false(async_io_cancel(-1))

print_test_summary()

test_start("Timer Operations Tests")

# Test timer operations
assert_eq_int(async_timer_create(1000), 1)
assert_eq_int(async_timer_create(-1), -1)
assert_true(async_timer_start(1))
assert_false(async_timer_start(-1))
assert_true(async_timer_stop(1))
assert_false(async_timer_stop(-1))
assert_true(async_timer_reset(1))
assert_false(async_timer_reset(-1))
assert_true(async_timer_is_expired(1))
assert_false(async_timer_is_expired(-1))
assert_eq_int(async_timer_get_remaining_time(1), 1000)
assert_eq_int(async_timer_get_remaining_time(-1), -1)

print_test_summary()

test_start("Async Utilities Tests")

# Test async utility functions
assert_eq_int(async_sleep(1000), 1)
assert_eq_int(async_sleep(-1), -1)
assert_eq_int(async_yield(), 1)
assert_eq_int(async_delay(500), 1)
assert_eq_int(async_delay(-1), -1)

print_test_summary()

test_start("Async Combinators Tests")

# Test async combinator functions
assert_eq_int(async_all("[1,2,3]"), 1)
assert_eq_int(async_all(""), -1)
assert_eq_int(async_any("[1,2,3]"), 1)
assert_eq_int(async_any(""), -1)
assert_eq_int(async_race("[1,2,3]"), 1)
assert_eq_int(async_race(""), -1)
assert_eq_int(async_sequence("[1,2,3]"), 1)
assert_eq_int(async_sequence(""), -1)
assert_eq_int(async_parallel("[1,2,3]"), 1)
assert_eq_int(async_parallel(""), -1)

print_test_summary()

test_start("Async Stream Operations Tests")

# Test async stream operations
assert_eq_int(async_stream_create(), 1)
assert_true(async_stream_push(1, "test_value"))
assert_false(async_stream_push(-1, "test_value"))
assert_eq_string(async_stream_pull(1), "stream_value")
assert_eq_string(async_stream_pull(-1), "")
assert_false(async_stream_is_closed(1))
assert_true(async_stream_is_closed(-1))
assert_true(async_stream_close(1))
assert_false(async_stream_close(-1))
assert_eq_int(async_stream_map(1, "transform_function"), 1)
assert_eq_int(async_stream_map(-1, "transform_function"), -1)
assert_eq_int(async_stream_map(1, ""), -1)
assert_eq_int(async_stream_filter(1, "filter_function"), 1)
assert_eq_int(async_stream_filter(-1, "filter_function"), -1)
assert_eq_int(async_stream_filter(1, ""), -1)
assert_eq_int(async_stream_reduce(1, "reduce_function", "initial"), 1)
assert_eq_int(async_stream_reduce(-1, "reduce_function", "initial"), -1)
assert_eq_int(async_stream_reduce(1, "", "initial"), -1)

print_test_summary()

test_start("Async Context Management Tests")

# Test async context management
assert_eq_int(async_context_create(), 1)
assert_true(async_context_set_value(1, "key", "value"))
assert_false(async_context_set_value(-1, "key", "value"))
assert_false(async_context_set_value(1, "", "value"))
assert_eq_string(async_context_get_value(1, "key"), "context_value")
assert_eq_string(async_context_get_value(-1, "key"), "")
assert_eq_string(async_context_get_value(1, ""), "")
assert_true(async_context_run_with(1, 1))
assert_false(async_context_run_with(-1, 1))
assert_false(async_context_run_with(1, -1))
assert_true(async_context_destroy(1))
assert_false(async_context_destroy(-1))

print_test_summary()

test_start("Async Error Handling Tests")

# Test async error handling
assert_eq_int(async_error_create("test_error"), 1)
assert_eq_int(async_error_create(""), -1)
assert_eq_string(async_error_get_message(1), "error_message")
assert_eq_string(async_error_get_message(-1), "")
assert_eq_string(async_error_get_stack_trace(1), "stack_trace")
assert_eq_string(async_error_get_stack_trace(-1), "")
assert_false(async_error_is_timeout(1))
assert_true(async_error_is_timeout(-1))
assert_false(async_error_is_cancellation(1))
assert_true(async_error_is_cancellation(-1))

print_test_summary()

test_start("Performance Monitoring Tests")

# Test performance monitoring
assert_eq_int(async_get_pending_tasks(), 5)
assert_eq_int(async_get_completed_tasks(), 10)
assert_eq_int(async_get_failed_tasks(), 2)
assert_eq_int(async_get_average_execution_time(), 100)
assert_eq_int(async_get_memory_usage(), 1024)
assert_true(async_reset_statistics())

print_test_summary()

test_start("Async Scheduler Tests")

# Test async scheduler
assert_eq_int(async_scheduler_create(), 1)
assert_true(async_scheduler_schedule(1, 1, 1000))
assert_false(async_scheduler_schedule(-1, 1, 1000))
assert_false(async_scheduler_schedule(1, -1, 1000))
assert_false(async_scheduler_schedule(1, 1, -1))
assert_true(async_scheduler_cancel(1, 1))
assert_false(async_scheduler_cancel(-1, 1))
assert_false(async_scheduler_cancel(1, -1))
assert_eq_int(async_scheduler_get_scheduled_count(1), 3)
assert_eq_int(async_scheduler_get_scheduled_count(-1), -1)
assert_true(async_scheduler_destroy(1))
assert_false(async_scheduler_destroy(-1))

print_test_summary()

test_start("Async State Constants Tests")

# Test async task state constants
assert_eq_int(ASYNC_TASK_PENDING, 0)
assert_eq_int(ASYNC_TASK_RUNNING, 1)
assert_eq_int(ASYNC_TASK_COMPLETED, 2)
assert_eq_int(ASYNC_TASK_FAILED, 3)
assert_eq_int(ASYNC_TASK_CANCELLED, 4)

# Test event loop state constants
assert_eq_int(EVENT_LOOP_IDLE, 0)
assert_eq_int(EVENT_LOOP_RUNNING, 1)
assert_eq_int(EVENT_LOOP_STOPPING, 2)
assert_eq_int(EVENT_LOOP_STOPPED, 3)

# Test promise state constants
assert_eq_int(PROMISE_PENDING, 0)
assert_eq_int(PROMISE_RESOLVED, 1)
assert_eq_int(PROMISE_REJECTED, 2)

# Test async I/O state constants
assert_eq_int(ASYNC_IO_READY, 0)
assert_eq_int(ASYNC_IO_WAITING, 1)
assert_eq_int(ASYNC_IO_COMPLETED, 2)
assert_eq_int(ASYNC_IO_ERROR, 3)

print_test_summary()
