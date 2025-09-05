# Async Core Module (async_core)

A comprehensive asynchronous runtime system implemented entirely in pure CURSED, providing task management, executors, promises, and async primitives without FFI dependencies.

## Overview

The `async_core` module provides the foundation for asynchronous programming in CURSED, including task spawning, execution management, promise handling, and runtime coordination. It replaces FFI-dependent async functionality with a pure CURSED implementation.

## Features

- **Task Management**: Spawn, execute, complete, cancel, and monitor async tasks
- **Executor System**: Multi-threaded task execution with configurable thread pools
- **Promise Support**: JavaScript-style promises with resolution and rejection
- **Runtime Coordination**: Global async runtime with lifecycle management
- **Blocking Operations**: Support for blocking tasks and async/await patterns
- **Timeout & Delay**: Built-in timeout and delay primitives
- **Statistics & Monitoring**: Comprehensive runtime and task statistics
- **Priority System**: Task priority levels for execution ordering

## Core Concepts

### Task States
- `TASK_PENDING (0)` - Task created but not yet started
- `TASK_RUNNING (1)` - Task currently executing
- `TASK_COMPLETED (2)` - Task finished successfully
- `TASK_CANCELLED (3)` - Task was cancelled before completion
- `TASK_FAILED (4)` - Task failed with an error

### Priority Levels
- `PRIORITY_LOW (0)` - Background tasks
- `PRIORITY_NORMAL (1)` - Standard task priority
- `PRIORITY_HIGH (2)` - High-priority tasks

## API Reference

### Runtime Management

#### `async_runtime_init() normie`
Initialize the async runtime system.

```cursed
yeet "async_core"

sus result normie = async_runtime_init()
lowkey result == 0 {
    vibez.spill("Async runtime initialized")
} else {
    vibez.spill("Runtime already initialized")
}
```

#### `async_runtime_shutdown() normie`
Shutdown the async runtime and cleanup resources.

```cursed
yeet "async_core"

sus remaining_tasks normie = async_runtime_shutdown()
vibez.spill("Shutdown complete, cancelled tasks: " + string_from_int(remaining_tasks))
```

### Task Management

#### `async_spawn_task(priority normie) normie`
Spawn a new async task with the specified priority.

```cursed
yeet "async_core"

async_runtime_init()

sus task_id normie = async_spawn_task(PRIORITY_NORMAL)
lowkey task_id > 0 {
    vibez.spill("Task spawned with ID: " + string_from_int(task_id))
}
```

#### `async_complete_task(task_id normie, result tea) normie`
Mark a task as completed with a result value.

```cursed
yeet "async_core"

sus task_id normie = async_spawn_task(PRIORITY_HIGH)
sus result normie = async_complete_task(task_id, "Task finished successfully")
lowkey result == 0 {
    vibez.spill("Task completed successfully")
}
```

#### `async_fail_task(task_id normie, error tea) normie`
Mark a task as failed with an error message.

```cursed
yeet "async_core"

sus task_id normie = async_spawn_task(PRIORITY_NORMAL)
sus result normie = async_fail_task(task_id, "Connection timeout")
lowkey result == 0 {
    vibez.spill("Task marked as failed")
}
```

#### `async_cancel_task(task_id normie) normie`
Cancel a pending or running task.

```cursed
yeet "async_core"

sus task_id normie = async_spawn_task(PRIORITY_LOW)
sus result normie = async_cancel_task(task_id)
lowkey result == 0 {
    vibez.spill("Task cancelled")
}
```

### Task State Queries

#### `async_get_task_state(task_id normie) normie`
Get the current state of a task.

```cursed
yeet "async_core"

sus task_id normie = async_spawn_task(PRIORITY_NORMAL)
sus state normie = async_get_task_state(task_id)
match state {
    TASK_PENDING => vibez.spill("Task is pending")
    TASK_RUNNING => vibez.spill("Task is running") 
    TASK_COMPLETED => vibez.spill("Task completed")
    TASK_FAILED => vibez.spill("Task failed")
    TASK_CANCELLED => vibez.spill("Task cancelled")
    _ => vibez.spill("Unknown task state")
}
```

#### `async_get_task_result(task_id normie) tea`
Get the result of a completed task.

```cursed
yeet "async_core"

sus task_id normie = async_spawn_task(PRIORITY_NORMAL)
async_complete_task(task_id, "Hello from async task")

sus result tea = async_get_task_result(task_id)
vibez.spill("Task result: " + result)
```

#### `async_get_task_error(task_id normie) tea`
Get the error message of a failed task.

```cursed
yeet "async_core"

sus task_id normie = async_spawn_task(PRIORITY_NORMAL)
async_fail_task(task_id, "Network connection failed")

sus error tea = async_get_task_error(task_id)
vibez.spill("Task error: " + error)
```

#### `async_is_task_ready(task_id normie) lit`
Check if a task has finished (completed, failed, or cancelled).

```cursed
yeet "async_core"

sus task_id normie = async_spawn_task(PRIORITY_NORMAL)
async_complete_task(task_id, "done")

sus ready lit = async_is_task_ready(task_id)
lowkey ready {
    vibez.spill("Task is ready")
}
```

### Executor Management

#### `async_create_executor(num_threads normie) normie`
Create a new task executor with specified thread count.

```cursed
yeet "async_core"

sus executor_id normie = async_create_executor(4)  # 4 threads
lowkey executor_id > 0 {
    vibez.spill("Executor created with ID: " + string_from_int(executor_id))
}
```

#### `async_start_executor(executor_id normie) normie`
Start an executor to begin processing tasks.

```cursed
yeet "async_core"

sus executor_id normie = async_create_executor(2)
sus result normie = async_start_executor(executor_id)
lowkey result == 0 {
    vibez.spill("Executor started")
}
```

#### `async_stop_executor(executor_id normie) normie`
Stop an executor from processing new tasks.

```cursed
yeet "async_core"

sus result normie = async_stop_executor(executor_id)
lowkey result == 0 {
    vibez.spill("Executor stopped")
}
```

#### `async_execute_task(executor_id normie, task_id normie) normie`
Execute a specific task on an executor.

```cursed
yeet "async_core"

sus executor_id normie = async_create_executor(2)
async_start_executor(executor_id)

sus task_id normie = async_spawn_task(PRIORITY_HIGH)
sus result normie = async_execute_task(executor_id, task_id)
lowkey result == 0 {
    vibez.spill("Task executing on executor")
}
```

### Blocking Operations

#### `async_spawn_blocking_task(priority normie) normie`
Spawn a task that will run on a blocking thread pool.

```cursed
yeet "async_core"

sus blocking_task_id normie = async_spawn_blocking_task(PRIORITY_HIGH)
lowkey blocking_task_id > 0 {
    vibez.spill("Blocking task spawned")
}
```

#### `async_block_on_task(task_id normie) tea`
Block current execution until a task completes.

```cursed
yeet "async_core"

sus task_id normie = async_spawn_task(PRIORITY_NORMAL)
async_complete_task(task_id, "blocking result")

sus result tea = async_block_on_task(task_id)
vibez.spill("Blocking operation result: " + result)
```

### Time-Based Operations

#### `async_sleep(duration_ms normie) normie`
Create an async sleep operation.

```cursed
yeet "async_core"

sus sleep_task_id normie = async_sleep(1000)  # Sleep for 1 second
sus result tea = async_get_task_result(sleep_task_id)
vibez.spill("Sleep completed: " + result)
```

#### `async_timeout(task_id normie, timeout_ms normie) normie`
Create a timeout wrapper for a task.

```cursed
yeet "async_core"

sus task_id normie = async_spawn_task(PRIORITY_NORMAL)
sus timeout_task_id normie = async_timeout(task_id, 5000)  # 5 second timeout
```

#### `async_create_delay(duration_ms normie) normie`
Create a delayed execution task.

```cursed
yeet "async_core"

sus delay_task_id normie = async_create_delay(2000)  # 2 second delay
sus result tea = async_get_task_result(delay_task_id)
vibez.spill("Delay completed: " + result)
```

### Promise System

#### `async_create_promise() normie`
Create a new promise.

```cursed
yeet "async_core"

sus promise_id normie = async_create_promise()
lowkey promise_id > 0 {
    vibez.spill("Promise created")
}
```

#### `async_resolve_promise(promise_id normie, value tea) normie`
Resolve a promise with a value.

```cursed
yeet "async_core"

sus promise_id normie = async_create_promise()
sus result normie = async_resolve_promise(promise_id, "Promise resolved!")
lowkey result == 0 {
    sus value tea = async_get_task_result(promise_id)
    vibez.spill("Promise value: " + value)
}
```

#### `async_reject_promise(promise_id normie, error tea) normie`
Reject a promise with an error.

```cursed
yeet "async_core"

sus promise_id normie = async_create_promise()
sus result normie = async_reject_promise(promise_id, "Promise failed")
lowkey result == 0 {
    sus error tea = async_get_task_error(promise_id)
    vibez.spill("Promise error: " + error)
}
```

### Statistics and Monitoring

#### `async_get_runtime_stats() map[tea]tea`
Get comprehensive runtime statistics.

```cursed
yeet "async_core"

sus stats map[tea]tea = async_get_runtime_stats()
vibez.spill("Tasks spawned: " + string_from_int(stats["tasks_spawned"]))
vibez.spill("Tasks completed: " + string_from_int(stats["tasks_completed"]))
vibez.spill("Tasks failed: " + string_from_int(stats["tasks_failed"]))
```

#### `async_get_executor_stats(executor_id normie) map[tea]tea`
Get statistics for a specific executor.

```cursed
yeet "async_core"

sus executor_stats map[tea]tea = async_get_executor_stats(executor_id)
vibez.spill("Executor running: " + executor_stats["running"])
vibez.spill("Tasks executed: " + string_from_int(executor_stats["tasks_executed"]))
```

#### Task Count Functions
```cursed
yeet "async_core"

sus total_tasks normie = async_get_task_count()
sus active_tasks normie = async_get_active_task_count() 
sus completed_tasks normie = async_get_completed_task_count()

vibez.spill("Total: " + string_from_int(total_tasks))
vibez.spill("Active: " + string_from_int(active_tasks))
vibez.spill("Completed: " + string_from_int(completed_tasks))
```

## Usage Examples

### Basic Async Task Processing

```cursed
yeet "async_core"

slay basic_async_example() lit {
    # Initialize runtime
    async_runtime_init()
    
    # Create executor
    sus executor_id normie = async_create_executor(2)
    async_start_executor(executor_id)
    
    # Spawn multiple tasks
    sus task1 normie = async_spawn_task(PRIORITY_HIGH)
    sus task2 normie = async_spawn_task(PRIORITY_NORMAL)
    sus task3 normie = async_spawn_task(PRIORITY_LOW)
    
    # Execute tasks
    async_execute_task(executor_id, task1)
    async_execute_task(executor_id, task2)
    async_execute_task(executor_id, task3)
    
    # Simulate task completion
    async_complete_task(task1, "Task 1 result")
    async_complete_task(task2, "Task 2 result")
    async_fail_task(task3, "Task 3 failed")
    
    # Check results
    vibez.spill("Task 1: " + async_get_task_result(task1))
    vibez.spill("Task 2: " + async_get_task_result(task2))
    vibez.spill("Task 3 error: " + async_get_task_error(task3))
    
    # Cleanup
    async_stop_executor(executor_id)
    async_runtime_shutdown()
    
    damn based
}

basic_async_example()
```

### Promise-Based Operations

```cursed
yeet "async_core"

slay promise_example() lit {
    async_runtime_init()
    
    # Create promises
    sus promise1 normie = async_create_promise()
    sus promise2 normie = async_create_promise()
    
    # Resolve one, reject another
    async_resolve_promise(promise1, "Success!")
    async_reject_promise(promise2, "Network error")
    
    # Handle results
    lowkey async_is_task_ready(promise1) {
        sus result tea = async_get_task_result(promise1)
        vibez.spill("Promise 1 resolved: " + result)
    }
    
    lowkey async_is_task_ready(promise2) {
        sus error tea = async_get_task_error(promise2)
        vibez.spill("Promise 2 rejected: " + error)
    }
    
    async_runtime_shutdown()
    damn based
}

promise_example()
```

### Time-Based Operations

```cursed
yeet "async_core"

slay time_based_example() lit {
    async_runtime_init()
    
    # Sleep operation
    vibez.spill("Starting sleep...")
    sus sleep_task normie = async_sleep(1000)
    sus sleep_result tea = async_get_task_result(sleep_task)
    vibez.spill("Sleep finished: " + sleep_result)
    
    # Delay operation
    vibez.spill("Creating delay...")
    sus delay_task normie = async_create_delay(500)
    sus delay_result tea = async_get_task_result(delay_task)
    vibez.spill("Delay finished: " + delay_result)
    
    # Timeout operation
    sus long_task normie = async_spawn_task(PRIORITY_NORMAL)
    sus timeout_task normie = async_timeout(long_task, 2000)
    vibez.spill("Timeout task created")
    
    async_runtime_shutdown()
    damn based
}

time_based_example()
```

### Blocking Operations

```cursed
yeet "async_core"

slay blocking_example() lit {
    async_runtime_init()
    
    # Spawn blocking task
    sus blocking_task normie = async_spawn_blocking_task(PRIORITY_HIGH)
    vibez.spill("Blocking task spawned")
    
    # Complete it
    async_complete_task(blocking_task, "Blocking operation complete")
    
    # Block on result
    vibez.spill("Blocking on task...")
    sus result tea = async_block_on_task(blocking_task)
    vibez.spill("Blocking result: " + result)
    
    async_runtime_shutdown()
    damn based
}

blocking_example()
```

### Runtime Monitoring

```cursed
yeet "async_core"

slay monitoring_example() lit {
    async_runtime_init()
    
    # Create some tasks for monitoring
    sus task1 normie = async_spawn_task(PRIORITY_HIGH)
    sus task2 normie = async_spawn_task(PRIORITY_NORMAL)
    sus task3 normie = async_spawn_task(PRIORITY_LOW)
    
    async_complete_task(task1, "done")
    async_fail_task(task2, "error")
    # task3 remains pending
    
    # Display statistics
    sus stats map[tea]tea = async_get_runtime_stats()
    vibez.spill("=== Runtime Statistics ===")
    vibez.spill("Initialized: " + stats["initialized"])
    vibez.spill("Tasks spawned: " + string_from_int(stats["tasks_spawned"]))
    vibez.spill("Tasks completed: " + string_from_int(stats["tasks_completed"]))
    vibez.spill("Tasks failed: " + string_from_int(stats["tasks_failed"]))
    
    vibez.spill("=== Task Counts ===")
    vibez.spill("Total tasks: " + string_from_int(async_get_task_count()))
    vibez.spill("Active tasks: " + string_from_int(async_get_active_task_count()))
    vibez.spill("Completed tasks: " + string_from_int(async_get_completed_task_count()))
    
    async_runtime_shutdown()
    damn based
}

monitoring_example()
```

## Testing

The module includes a comprehensive test suite covering all functionality:

```bash
# Run the full test suite
./cursed-unified stdlib/async_core/test_async_core.💀

# Or run individual test functions
echo 'yeet "async_core"
test_async_runtime_lifecycle()
test_task_spawning()
test_promises()' > test_custom.💀

./cursed-unified test_custom.💀
```

## Performance Characteristics

- **Task Creation**: O(1) constant time task spawning
- **State Queries**: O(1) task state and result access
- **Statistics**: O(n) where n is the number of tasks for counts
- **Memory Usage**: Linear with number of active tasks and executors
- **Executor Management**: O(1) for executor operations

## Implementation Details

### Task Storage
Tasks are stored in a global map with task IDs as keys and state maps as values. Each task maintains:
- Task ID and creation metadata
- Current state and priority
- Result or error information
- Execution context (executor, timestamps)

### Executor Model
Executors represent thread pools for task execution:
- Configurable thread count per executor
- Independent execution tracking
- Start/stop lifecycle management
- Task assignment and statistics

### Promise Implementation
Promises are implemented as specialized tasks:
- Promise-specific state tracking
- Resolution and rejection handling
- Integration with core task system
- Standard promise semantics

## Dependencies

- **`testz`** - Testing framework for comprehensive test coverage

## Related Modules

- [`concurrency`](../concurrency/README.md) - Higher-level concurrency primitives
- [`channel_core`](../channel_core/README.md) - Channel-based communication
- [`goroutine_core`](../goroutine_core/README.md) - Goroutine implementation
- [`runtime_core`](../runtime_core/README.md) - Core runtime utilities

## Best Practices

1. **Runtime Lifecycle**: Always initialize runtime before using async operations
2. **Resource Cleanup**: Shutdown runtime to properly cleanup resources
3. **Error Handling**: Check return codes from async operations
4. **Task Monitoring**: Use statistics functions for runtime monitoring
5. **Executor Management**: Start executors before executing tasks
6. **Promise Patterns**: Handle both resolution and rejection cases

## Version History

- **v1.0.0** - Complete async runtime with tasks, executors, and promises

## Contributing

When contributing to `async_core`:

1. Maintain pure CURSED implementation (no FFI)
2. Add comprehensive test coverage for new features
3. Follow established task state management patterns
4. Update statistics and monitoring for new operations
5. Ensure thread-safety considerations are documented

## License

Part of the CURSED programming language stdlib - see main project license.
