# Async Module

Pure CURSED implementation of comprehensive async/await functionality with no FFI dependencies.

## Features

### 1. Async Task Management
- **Task Spawning**: Create and manage async tasks with `spawn_async()`
- **Task States**: Complete lifecycle management (pending, running, completed, cancelled, failed)
- **Task Dependencies**: Chain tasks with dependency relationships
- **Task Cancellation**: Cancel tasks with custom reasons
- **Task Timeouts**: Set timeouts for task execution
- **Task Retry**: Automatic retry mechanism for failed tasks

### 2. Future/Promise Implementation
- **Future Creation**: Create futures with `future_new()`
- **Promise Creation**: Create promises with `promise_new()`
- **Promise Resolution**: Resolve promises with `promise_resolve()`
- **Promise Rejection**: Reject promises with `promise_reject()`
- **Promise.all**: Wait for multiple promises with `promise_all()`
- **Promise.race**: Race multiple promises with `promise_race()`
- **Awaiting**: Await future completion with `await_future()`

### 3. Async I/O Operations
- **File Operations**: Async file read/write with `async_file_read()` and `async_file_write()`
- **HTTP Requests**: Async HTTP requests with `async_http_request()`
- **I/O Scheduling**: Efficient I/O operation scheduling
- **Channel-based Communication**: Channel-based async communication

### 4. Task Scheduling
- **Work-stealing Scheduler**: Efficient task distribution across workers
- **Load Balancing**: Automatic load balancing across worker threads
- **Priority Scheduling**: Task priority support
- **Metrics Collection**: Runtime statistics and performance metrics

### 5. Async Error Handling
- **Error Propagation**: Automatic error propagation through async chains
- **Error Recovery**: Retry mechanisms for failed operations
- **Timeout Handling**: Proper timeout error handling
- **Cancellation Support**: Graceful task cancellation

### 6. Coroutine Support
- **Coroutine Creation**: Create coroutines with `coroutine_create()`
- **Coroutine Yield**: Yield control with `coroutine_yield()`
- **Coroutine Resume**: Resume coroutines with `coroutine_resume()`
- **Async/Await Style**: Modern async/await programming patterns

## Core Types

### AsyncRuntime
Main runtime coordinator managing all async operations:
```cursed
struct AsyncRuntime {
    task_counter: thicc,
    task_registry: map[TaskId]Task,
    scheduler: AsyncScheduler,
    event_loop: EventLoop,
    is_running: lit,
    worker_threads: thicc,
    task_queue: Channel[Task],
    completion_queue: Channel[TaskResult]
}
```

### Task
Represents an async task with full lifecycle management:
```cursed
struct Task {
    id: TaskId,
    state: TaskState,
    function_ptr: tea,
    result: AsyncResult,
    priority: normie,
    created_at: thicc,
    started_at: thicc,
    completed_at: thicc,
    dependencies: [TaskId],
    dependents: [TaskId],
    context: map[tea]tea,
    cancellation_token: CancellationToken,
    timeout_ms: thicc,
    retry_count: normie,
    max_retries: normie
}
```

### Future
Future implementation for async operations:
```cursed
struct Future {
    id: TaskId,
    state: TaskState,
    result: AsyncResult,
    awaiter_tasks: [TaskId],
    completion_callbacks: [tea],
    error_callbacks: [tea],
    timeout_duration: thicc,
    created_at: thicc
}
```

### Promise
Promise implementation with resolver/rejector:
```cursed
struct Promise {
    future: Future,
    resolver: PromiseResolver,
    rejector: PromiseRejector
}
```

## Usage Examples

### Basic Task Spawning
```cursed
yeet "async"

# Initialize runtime
init_async_runtime()

# Spawn async task
sus context = {"duration": "100"}
sus task_id = spawn_async("async_sleep", context)

# Wait for completion
sus result = wait_for_task(task_id)
```

### Promise Usage
```cursed
# Create promise
sus promise = promise_new()

# Resolve promise
promise_resolve(promise, "success_value")

# Await result
sus result = await_future(promise.future)
```

### Promise.all Pattern
```cursed
sus promise1 = promise_new()
sus promise2 = promise_new()
sus promise3 = promise_new()

# Resolve all promises
promise_resolve(promise1, "result1")
promise_resolve(promise2, "result2")
promise_resolve(promise3, "result3")

# Wait for all
sus all_promises = [promise1, promise2, promise3]
sus all_result = promise_all(all_promises)
```

### Async I/O Operations
```cursed
# Async file read
sus read_context = {"filename": "input.txt"}
sus read_task = spawn_async("async_file_read", read_context)
sus content = wait_for_task(read_task)

# Async HTTP request
sus http_context = {"url": "https://api.example.com"}
sus http_task = spawn_async("async_http_request", http_context)
sus response = wait_for_task(http_task)
```

### Task Dependencies
```cursed
sus task1 = spawn_async("async_sleep", {"duration": "50"})
sus task2 = spawn_async("async_sleep", {"duration": "30"})

# task1 depends on task2
add_task_dependency(task1, task2)

# task1 will wait for task2 to complete
sus result = wait_for_task(task1)
```

### Coroutine Support
```cursed
# Create coroutine
sus coroutine_id = coroutine_create("async_sleep", {"duration": "100"})

# Yield control
coroutine_yield()

# Resume coroutine
coroutine_resume(coroutine_id)
```

### Error Handling
```cursed
sus task_id = spawn_async("async_operation", context)

# Set timeout
set_task_timeout(task_id, 1000)

# Handle errors
async_error_handler(task_id, "timeout_error")

# Retry failed task
retry_task(task_id)
```

## Testing

Run the comprehensive test suite:
```bash
cargo run --bin cursed stdlib/async/test_async.💀
```

The test suite covers:
- ✅ Runtime initialization and shutdown
- ✅ Task spawning and lifecycle management
- ✅ Future/Promise creation and resolution
- ✅ Async I/O operations
- ✅ Task scheduling and dependencies
- ✅ Error handling and recovery
- ✅ Coroutine support
- ✅ Promise.all and Promise.race patterns
- ✅ Timeout and cancellation mechanisms
- ✅ Load balancing and metrics
- ✅ Complex async workflows

## Both-Mode Support

Both interpretation and compilation modes are fully supported:

```bash
# Interpretation mode
cargo run --bin cursed stdlib/async/test_async.💀

# Compilation mode
cargo run --bin cursed -- compile stdlib/async/test_async.💀
./test_async
```

## Performance Features

- **Work-stealing Scheduler**: Efficient task distribution
- **Lock-free Operations**: Minimize contention
- **Timer Wheel**: Efficient timeout management
- **Load Balancing**: Automatic worker load balancing
- **Metrics Collection**: Runtime performance monitoring

## FFI-Free Implementation

This module is implemented entirely in pure CURSED with no external FFI dependencies:
- No C library calls
- No external runtime dependencies
- Pure CURSED channel-based communication
- Native CURSED data structures and algorithms
- Self-contained event loop and scheduler

## Architecture

The async module consists of several key components:

1. **AsyncRuntime**: Main coordinator and runtime manager
2. **AsyncScheduler**: Task scheduling and load balancing
3. **EventLoop**: Event processing and I/O handling
4. **TimerWheel**: Efficient timeout management
5. **TaskRegistry**: Task lifecycle management
6. **PromiseSystem**: Future/Promise implementation
7. **CoroutineSupport**: Coroutine creation and management

All components work together to provide a complete async/await implementation that's comparable to modern async runtimes but implemented entirely in CURSED.
