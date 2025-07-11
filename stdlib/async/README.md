# Async Module

The async module provides comprehensive asynchronous programming support for the CURSED language. It enables building non-blocking, event-driven applications with async/await patterns, promises, and asynchronous I/O operations.

## Features

- **Event Loop**: Asynchronous event loop management
- **Async Tasks**: Task creation and execution with async/await
- **Promises**: Promise-based asynchronous operations
- **Async I/O**: Non-blocking file and network operations
- **Timers**: Asynchronous timer operations
- **Combinators**: Async task combination patterns (all, any, race, sequence)
- **Streams**: Asynchronous data stream processing
- **Context Management**: Async execution context handling
- **Error Handling**: Comprehensive async error management
- **Scheduler**: Task scheduling and delay management
- **Pure CURSED Implementation**: No external dependencies

## Async Task States

```cursed
ASYNC_TASK_PENDING = 0
ASYNC_TASK_RUNNING = 1
ASYNC_TASK_COMPLETED = 2
ASYNC_TASK_FAILED = 3
ASYNC_TASK_CANCELLED = 4
```

## Event Loop States

```cursed
EVENT_LOOP_IDLE = 0
EVENT_LOOP_RUNNING = 1
EVENT_LOOP_STOPPING = 2
EVENT_LOOP_STOPPED = 3
```

## Promise States

```cursed
PROMISE_PENDING = 0
PROMISE_RESOLVED = 1
PROMISE_REJECTED = 2
```

## Async I/O States

```cursed
ASYNC_IO_READY = 0
ASYNC_IO_WAITING = 1
ASYNC_IO_COMPLETED = 2
ASYNC_IO_ERROR = 3
```

## Basic Usage

### Event Loop Management

```cursed
yeet "async"

# Create and run event loop
sus loop_id normie = async_event_loop_create()
async_event_loop_run(loop_id)

# Get loop state
sus state smol = async_event_loop_get_state(loop_id)

# Stop event loop
async_event_loop_stop(loop_id)
async_event_loop_destroy(loop_id)
```

### Async Task Management

```cursed
# Create async task
sus task_id normie = async_task_create("async_function")

# Run task
async_task_run(task_id)

# Check task state
sus state smol = async_task_get_state(task_id)
sus is_completed lit = async_task_is_completed(task_id)

# Wait for task completion
async_task_wait(task_id)
async_task_wait_timeout(task_id, 5000)

# Get task result
sus result tea = async_task_get_result(task_id)
sus error tea = async_task_get_error(task_id)

# Cancel task
async_task_cancel(task_id)
```

### Promise Operations

```cursed
# Create promise
sus promise_id normie = async_promise_create()

# Resolve or reject promise
async_promise_resolve(promise_id, "success_value")
async_promise_reject(promise_id, "error_message")

# Chain promises
sus then_promise normie = async_promise_then(promise_id, "success_callback")
sus catch_promise normie = async_promise_catch(promise_id, "error_callback")
sus finally_promise normie = async_promise_finally(promise_id, "cleanup_callback")

# Get promise state and value
sus state smol = async_promise_get_state(promise_id)
sus value tea = async_promise_get_value(promise_id)
sus error tea = async_promise_get_error(promise_id)
```

### Async I/O Operations

```cursed
# Async file I/O
sus read_op normie = async_io_read(file_handle, 1024)
sus write_op normie = async_io_write(file_handle, "data")

# Async network I/O
sus connect_op normie = async_io_connect("localhost", 8080)
sus listen_op normie = async_io_listen(8080)
sus accept_op normie = async_io_accept(listener_id)

# Check I/O operation status
sus io_state smol = async_io_get_state(read_op)
sus io_result tea = async_io_get_result(read_op)
sus io_error tea = async_io_get_error(read_op)

# Cancel I/O operation
async_io_cancel(read_op)
```

### Timer Operations

```cursed
# Create and start timer
sus timer_id normie = async_timer_create(1000)  # 1 second
async_timer_start(timer_id)

# Check timer status
sus is_expired lit = async_timer_is_expired(timer_id)
sus remaining normie = async_timer_get_remaining_time(timer_id)

# Control timer
async_timer_stop(timer_id)
async_timer_reset(timer_id)
```

### Async Utilities

```cursed
# Async sleep and delay
sus sleep_task normie = async_sleep(1000)
sus delay_task normie = async_delay(500)

# Yield control
sus yield_task normie = async_yield()
```

### Async Combinators

```cursed
# Wait for all tasks to complete
sus all_task normie = async_all("[1,2,3]")  # JSON array of task IDs

# Wait for any task to complete
sus any_task normie = async_any("[1,2,3]")

# Race tasks (first to complete wins)
sus race_task normie = async_race("[1,2,3]")

# Execute tasks in sequence
sus sequence_task normie = async_sequence("[1,2,3]")

# Execute tasks in parallel
sus parallel_task normie = async_parallel("[1,2,3]")
```

### Async Streams

```cursed
# Create async stream
sus stream_id normie = async_stream_create()

# Push and pull data
async_stream_push(stream_id, "data1")
async_stream_push(stream_id, "data2")
sus value tea = async_stream_pull(stream_id)

# Stream transformations
sus mapped_stream normie = async_stream_map(stream_id, "transform_function")
sus filtered_stream normie = async_stream_filter(stream_id, "filter_function")
sus reduced_task normie = async_stream_reduce(stream_id, "reduce_function", "initial")

# Stream control
sus is_closed lit = async_stream_is_closed(stream_id)
async_stream_close(stream_id)
```

### Async Context Management

```cursed
# Create async context
sus context_id normie = async_context_create()

# Set context values
async_context_set_value(context_id, "user_id", "123")
async_context_set_value(context_id, "request_id", "req_456")

# Get context values
sus user_id tea = async_context_get_value(context_id, "user_id")
sus request_id tea = async_context_get_value(context_id, "request_id")

# Run task with context
async_context_run_with(context_id, task_id)

# Cleanup context
async_context_destroy(context_id)
```

### Async Error Handling

```cursed
# Create async error
sus error_id normie = async_error_create("Operation failed")

# Get error information
sus message tea = async_error_get_message(error_id)
sus stack_trace tea = async_error_get_stack_trace(error_id)

# Check error type
sus is_timeout lit = async_error_is_timeout(error_id)
sus is_cancelled lit = async_error_is_cancellation(error_id)
```

### Performance Monitoring

```cursed
# Get async runtime statistics
sus pending normie = async_get_pending_tasks()
sus completed normie = async_get_completed_tasks()
sus failed normie = async_get_failed_tasks()
sus avg_time normie = async_get_average_execution_time()
sus memory normie = async_get_memory_usage()

# Reset statistics
async_reset_statistics()
```

### Async Scheduler

```cursed
# Create scheduler
sus scheduler_id normie = async_scheduler_create()

# Schedule task with delay
async_scheduler_schedule(scheduler_id, task_id, 1000)  # 1 second delay

# Cancel scheduled task
async_scheduler_cancel(scheduler_id, task_id)

# Get scheduled task count
sus scheduled_count normie = async_scheduler_get_scheduled_count(scheduler_id)

# Cleanup scheduler
async_scheduler_destroy(scheduler_id)
```

## Functions

### Event Loop Functions
- `async_event_loop_create() normie` - Create event loop
- `async_event_loop_run(loop_id normie) lit` - Run event loop
- `async_event_loop_stop(loop_id normie) lit` - Stop event loop
- `async_event_loop_get_state(loop_id normie) smol` - Get loop state
- `async_event_loop_destroy(loop_id normie) lit` - Destroy event loop

### Task Management Functions
- `async_task_create(function_name tea) normie` - Create async task
- `async_task_run(task_id normie) lit` - Run task
- `async_task_cancel(task_id normie) lit` - Cancel task
- `async_task_get_state(task_id normie) smol` - Get task state
- `async_task_get_result(task_id normie) tea` - Get task result
- `async_task_get_error(task_id normie) tea` - Get task error
- `async_task_is_completed(task_id normie) lit` - Check if completed
- `async_task_is_cancelled(task_id normie) lit` - Check if cancelled
- `async_task_wait(task_id normie) lit` - Wait for completion
- `async_task_wait_timeout(task_id normie, timeout_ms normie) lit` - Wait with timeout

### Promise Functions
- `async_promise_create() normie` - Create promise
- `async_promise_resolve(promise_id normie, value tea) lit` - Resolve promise
- `async_promise_reject(promise_id normie, error tea) lit` - Reject promise
- `async_promise_then(promise_id normie, callback_name tea) normie` - Chain success callback
- `async_promise_catch(promise_id normie, error_callback tea) normie` - Chain error callback
- `async_promise_finally(promise_id normie, finally_callback tea) normie` - Chain finally callback
- `async_promise_get_state(promise_id normie) smol` - Get promise state
- `async_promise_get_value(promise_id normie) tea` - Get promise value
- `async_promise_get_error(promise_id normie) tea` - Get promise error

### Async I/O Functions
- `async_io_read(file_handle normie, buffer_size normie) normie` - Async read
- `async_io_write(file_handle normie, data tea) normie` - Async write
- `async_io_connect(address tea, port normie) normie` - Async connect
- `async_io_listen(port normie) normie` - Async listen
- `async_io_accept(listener_id normie) normie` - Async accept
- `async_io_get_state(operation_id normie) smol` - Get I/O state
- `async_io_get_result(operation_id normie) tea` - Get I/O result
- `async_io_get_error(operation_id normie) tea` - Get I/O error
- `async_io_cancel(operation_id normie) lit` - Cancel I/O operation

### Timer Functions
- `async_timer_create(delay_ms normie) normie` - Create timer
- `async_timer_start(timer_id normie) lit` - Start timer
- `async_timer_stop(timer_id normie) lit` - Stop timer
- `async_timer_reset(timer_id normie) lit` - Reset timer
- `async_timer_is_expired(timer_id normie) lit` - Check if expired
- `async_timer_get_remaining_time(timer_id normie) normie` - Get remaining time

### Utility Functions
- `async_sleep(milliseconds normie) normie` - Async sleep
- `async_yield() normie` - Yield control
- `async_delay(milliseconds normie) normie` - Async delay

### Combinator Functions
- `async_all(task_ids tea) normie` - Wait for all tasks
- `async_any(task_ids tea) normie` - Wait for any task
- `async_race(task_ids tea) normie` - Race tasks
- `async_sequence(task_ids tea) normie` - Execute in sequence
- `async_parallel(task_ids tea) normie` - Execute in parallel

### Stream Functions
- `async_stream_create() normie` - Create stream
- `async_stream_push(stream_id normie, value tea) lit` - Push to stream
- `async_stream_pull(stream_id normie) tea` - Pull from stream
- `async_stream_close(stream_id normie) lit` - Close stream
- `async_stream_is_closed(stream_id normie) lit` - Check if closed
- `async_stream_map(stream_id normie, transform_function tea) normie` - Map stream
- `async_stream_filter(stream_id normie, filter_function tea) normie` - Filter stream
- `async_stream_reduce(stream_id normie, reduce_function tea, initial_value tea) normie` - Reduce stream

### Context Functions
- `async_context_create() normie` - Create context
- `async_context_set_value(context_id normie, key tea, value tea) lit` - Set context value
- `async_context_get_value(context_id normie, key tea) tea` - Get context value
- `async_context_run_with(context_id normie, task_id normie) lit` - Run with context
- `async_context_destroy(context_id normie) lit` - Destroy context

### Error Handling Functions
- `async_error_create(message tea) normie` - Create error
- `async_error_get_message(error_id normie) tea` - Get error message
- `async_error_get_stack_trace(error_id normie) tea` - Get stack trace
- `async_error_is_timeout(error_id normie) lit` - Check if timeout error
- `async_error_is_cancellation(error_id normie) lit` - Check if cancellation error

### Performance Functions
- `async_get_pending_tasks() normie` - Get pending task count
- `async_get_completed_tasks() normie` - Get completed task count
- `async_get_failed_tasks() normie` - Get failed task count
- `async_get_average_execution_time() normie` - Get average execution time
- `async_get_memory_usage() normie` - Get memory usage
- `async_reset_statistics() lit` - Reset statistics

### Scheduler Functions
- `async_scheduler_create() normie` - Create scheduler
- `async_scheduler_schedule(scheduler_id normie, task_id normie, delay_ms normie) lit` - Schedule task
- `async_scheduler_cancel(scheduler_id normie, task_id normie) lit` - Cancel scheduled task
- `async_scheduler_get_scheduled_count(scheduler_id normie) normie` - Get scheduled count
- `async_scheduler_destroy(scheduler_id normie) lit` - Destroy scheduler

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/async/test_async.csd
```

Test both interpretation and compilation modes:

```bash
cargo run --bin cursed stdlib/async/test_async.csd
cargo run --bin cursed -- compile stdlib/async/test_async.csd
./test_async
```

## Error Handling

All functions return appropriate error values:
- Boolean functions return `cap` (false) on error
- Integer functions return -1 on error
- String functions return empty string on error

## Performance

- Efficient event loop with minimal overhead
- Lock-free task queuing where possible
- Optimized promise chain execution
- Fast async I/O operations
- Memory-efficient stream processing
- Optimized for both interpretation and compilation modes

## Use Cases

- **Web Servers**: Handle HTTP requests asynchronously
- **I/O Operations**: Non-blocking file and network operations
- **Data Processing**: Asynchronous data pipeline processing
- **Real-time Applications**: Event-driven real-time systems
- **Microservices**: Async service communication
- **Background Tasks**: Scheduled and deferred task execution

## Integration

Works seamlessly with other stdlib modules:
- **concurrency**: Thread-safe async operations
- **web**: Async web server and client operations
- **database**: Async database operations
- **net**: Async network communications

## Dependencies

- `testz` - Testing framework
- `string` - String manipulation
- `collections` - Data structures
- `time` - Time utilities
- `concurrency` - Concurrency primitives

## License

Part of the CURSED language standard library.
