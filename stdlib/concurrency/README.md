# Concurrency Module

The concurrency module provides advanced concurrency primitives and patterns for the CURSED language. It enables building highly concurrent applications with thread management, synchronization primitives, and async communication patterns.

## Features

- **Thread Management**: Create, start, join, and manage threads
- **Synchronization Primitives**: Mutex, semaphore, condition variables, barriers
- **Read-Write Locks**: Efficient reader-writer synchronization
- **Atomic Operations**: Lock-free atomic operations
- **Worker Pools**: Task-based parallel execution
- **Channel Communication**: Message passing between threads
- **Future/Promise**: Asynchronous computation results
- **Performance Monitoring**: Context switches, lock contention tracking
- **Pure CURSED Implementation**: No external dependencies

## Thread States

```cursed
THREAD_STATE_READY = 0
THREAD_STATE_RUNNING = 1
THREAD_STATE_BLOCKED = 2
THREAD_STATE_TERMINATED = 3
```

## Mutex States

```cursed
MUTEX_UNLOCKED = 0
MUTEX_LOCKED = 1
```

## Semaphore States

```cursed
SEMAPHORE_AVAILABLE = 0
SEMAPHORE_BLOCKED = 1
```

## Channel Types

```cursed
CHANNEL_BUFFERED = 1
CHANNEL_UNBUFFERED = 2
CHANNEL_CLOSED = 3
```

## Worker Pool States

```cursed
POOL_ACTIVE = 1
POOL_SHUTTING_DOWN = 2
POOL_SHUTDOWN = 3
```

## Basic Usage

### Thread Management

```cursed
yeet "concurrency"

# Create and start thread
sus thread_id normie = concurrency_thread_create("worker_function")
concurrency_thread_start(thread_id)

# Set thread priority
concurrency_thread_set_priority(thread_id, 8)

# Join thread
concurrency_thread_join(thread_id)

# Thread utilities
concurrency_thread_yield()
concurrency_thread_sleep(1000)
sus current_id normie = concurrency_thread_get_id()
```

### Mutex Operations

```cursed
# Create mutex
sus mutex_id normie = concurrency_mutex_create()

# Lock and unlock
concurrency_mutex_lock(mutex_id)
# Critical section
concurrency_mutex_unlock(mutex_id)

# Try lock
vibe_if concurrency_mutex_try_lock(mutex_id) {
    # Got lock
    concurrency_mutex_unlock(mutex_id)
}

# Cleanup
concurrency_mutex_destroy(mutex_id)
```

### Semaphore Operations

```cursed
# Create semaphore with initial count
sus semaphore_id normie = concurrency_semaphore_create(5)

# Wait and signal
concurrency_semaphore_wait(semaphore_id)
# Resource access
concurrency_semaphore_signal(semaphore_id)

# Try wait
vibe_if concurrency_semaphore_try_wait(semaphore_id) {
    # Got resource
    concurrency_semaphore_signal(semaphore_id)
}

# Check count
sus count normie = concurrency_semaphore_get_count(semaphore_id)
```

### Condition Variables

```cursed
# Create condition variable
sus condition_id normie = concurrency_condition_create()
sus mutex_id normie = concurrency_mutex_create()

# Wait for condition
concurrency_mutex_lock(mutex_id)
concurrency_condition_wait(condition_id, mutex_id)
concurrency_mutex_unlock(mutex_id)

# Signal condition
concurrency_condition_signal(condition_id)
concurrency_condition_broadcast(condition_id)

# Timed wait
concurrency_condition_timed_wait(condition_id, mutex_id, 5000)
```

### Read-Write Locks

```cursed
# Create read-write lock
sus rwlock_id normie = concurrency_rwlock_create()

# Read lock
concurrency_rwlock_read_lock(rwlock_id)
# Read operations
concurrency_rwlock_read_unlock(rwlock_id)

# Write lock
concurrency_rwlock_write_lock(rwlock_id)
# Write operations
concurrency_rwlock_write_unlock(rwlock_id)

# Try locks
vibe_if concurrency_rwlock_try_read_lock(rwlock_id) {
    # Read access
    concurrency_rwlock_read_unlock(rwlock_id)
}
```

### Atomic Operations

```cursed
# Create atomic variable
sus atomic_id normie = concurrency_atomic_create(42)

# Load and store
sus value normie = concurrency_atomic_load(atomic_id)
concurrency_atomic_store(atomic_id, 100)

# Exchange
sus old_value normie = concurrency_atomic_exchange(atomic_id, 200)

# Compare and swap
concurrency_atomic_compare_and_swap(atomic_id, 200, 300)

# Fetch and add/sub
sus prev_value normie = concurrency_atomic_fetch_add(atomic_id, 10)
sus prev_value2 normie = concurrency_atomic_fetch_sub(atomic_id, 5)
```

### Worker Pools

```cursed
# Create worker pool
sus pool_id normie = concurrency_pool_create(4)

# Submit tasks
sus task_id normie = concurrency_pool_submit_task(pool_id, "process_data")
sus task_id2 normie = concurrency_pool_submit_task(pool_id, "compute_result")

# Wait for task completion
concurrency_pool_wait_for_task(pool_id, task_id)

# Get pool statistics
sus active_tasks normie = concurrency_pool_get_active_tasks(pool_id)
sus completed_tasks normie = concurrency_pool_get_completed_tasks(pool_id)
sus state smol = concurrency_pool_get_state(pool_id)

# Shutdown pool
concurrency_pool_shutdown(pool_id)
```

### Channel Communication

```cursed
# Create channel
sus channel_id normie = concurrency_channel_create(10)  # Buffered channel

# Send and receive
concurrency_channel_send(channel_id, "Hello")
sus message tea = concurrency_channel_receive(channel_id)

# Try operations
vibe_if concurrency_channel_try_send(channel_id, "World") {
    # Message sent
}

sus try_message tea = concurrency_channel_try_receive(channel_id)

# Channel status
sus buffer_size normie = concurrency_channel_get_buffer_size(channel_id)
sus message_count normie = concurrency_channel_get_message_count(channel_id)
sus is_closed lit = concurrency_channel_is_closed(channel_id)

# Close channel
concurrency_channel_close(channel_id)
```

### Future/Promise

```cursed
# Create future
sus future_id normie = concurrency_future_create()

# Set value (from another thread)
concurrency_future_set_value(future_id, "result")

# Get value (blocking)
sus result tea = concurrency_future_get_value(future_id)

# Check if ready
vibe_if concurrency_future_is_ready(future_id) {
    sus value tea = concurrency_future_get_value(future_id)
}

# Wait for completion
concurrency_future_wait(future_id)
concurrency_future_wait_timeout(future_id, 5000)
```

### Barriers

```cursed
# Create barrier for 3 threads
sus barrier_id normie = concurrency_barrier_create(3)

# Wait at barrier (all threads must reach this point)
concurrency_barrier_wait(barrier_id)

# Cleanup
concurrency_barrier_destroy(barrier_id)
```

### Thread Utilities

```cursed
# System information
sus cpu_count normie = concurrency_get_cpu_count()
sus thread_count normie = concurrency_get_thread_count()
sus is_main lit = concurrency_is_main_thread()

# Thread naming
concurrency_set_thread_name(thread_id, "worker_thread")
sus name tea = concurrency_get_thread_name(thread_id)
```

### Performance Monitoring

```cursed
# Get performance metrics
sus context_switches normie = concurrency_get_context_switches()
sus lock_contention normie = concurrency_get_lock_contention()
sus deadlocks normie = concurrency_get_deadlock_count()

# Reset counters
concurrency_reset_performance_counters()
```

## Functions

### Thread Management Functions
- `concurrency_thread_create(function_name tea) normie` - Create thread
- `concurrency_thread_start(thread_id normie) lit` - Start thread
- `concurrency_thread_join(thread_id normie) lit` - Join thread
- `concurrency_thread_detach(thread_id normie) lit` - Detach thread
- `concurrency_thread_get_state(thread_id normie) smol` - Get thread state
- `concurrency_thread_set_priority(thread_id normie, priority normie) lit` - Set priority
- `concurrency_thread_get_priority(thread_id normie) normie` - Get priority
- `concurrency_thread_yield() lit` - Yield execution
- `concurrency_thread_sleep(milliseconds normie) lit` - Sleep thread
- `concurrency_thread_get_id() normie` - Get current thread ID

### Mutex Functions
- `concurrency_mutex_create() normie` - Create mutex
- `concurrency_mutex_lock(mutex_id normie) lit` - Lock mutex
- `concurrency_mutex_unlock(mutex_id normie) lit` - Unlock mutex
- `concurrency_mutex_try_lock(mutex_id normie) lit` - Try lock mutex
- `concurrency_mutex_destroy(mutex_id normie) lit` - Destroy mutex
- `concurrency_mutex_is_locked(mutex_id normie) lit` - Check if locked

### Semaphore Functions
- `concurrency_semaphore_create(initial_count normie) normie` - Create semaphore
- `concurrency_semaphore_wait(semaphore_id normie) lit` - Wait on semaphore
- `concurrency_semaphore_signal(semaphore_id normie) lit` - Signal semaphore
- `concurrency_semaphore_try_wait(semaphore_id normie) lit` - Try wait
- `concurrency_semaphore_get_count(semaphore_id normie) normie` - Get count
- `concurrency_semaphore_destroy(semaphore_id normie) lit` - Destroy semaphore

### Condition Variable Functions
- `concurrency_condition_create() normie` - Create condition variable
- `concurrency_condition_wait(condition_id normie, mutex_id normie) lit` - Wait on condition
- `concurrency_condition_signal(condition_id normie) lit` - Signal condition
- `concurrency_condition_broadcast(condition_id normie) lit` - Broadcast condition
- `concurrency_condition_timed_wait(condition_id normie, mutex_id normie, timeout_ms normie) lit` - Timed wait
- `concurrency_condition_destroy(condition_id normie) lit` - Destroy condition

### Read-Write Lock Functions
- `concurrency_rwlock_create() normie` - Create read-write lock
- `concurrency_rwlock_read_lock(rwlock_id normie) lit` - Read lock
- `concurrency_rwlock_write_lock(rwlock_id normie) lit` - Write lock
- `concurrency_rwlock_read_unlock(rwlock_id normie) lit` - Read unlock
- `concurrency_rwlock_write_unlock(rwlock_id normie) lit` - Write unlock
- `concurrency_rwlock_try_read_lock(rwlock_id normie) lit` - Try read lock
- `concurrency_rwlock_try_write_lock(rwlock_id normie) lit` - Try write lock
- `concurrency_rwlock_destroy(rwlock_id normie) lit` - Destroy lock

### Barrier Functions
- `concurrency_barrier_create(thread_count normie) normie` - Create barrier
- `concurrency_barrier_wait(barrier_id normie) lit` - Wait at barrier
- `concurrency_barrier_destroy(barrier_id normie) lit` - Destroy barrier

### Atomic Functions
- `concurrency_atomic_create(initial_value normie) normie` - Create atomic variable
- `concurrency_atomic_load(atomic_id normie) normie` - Load atomic value
- `concurrency_atomic_store(atomic_id normie, value normie) lit` - Store atomic value
- `concurrency_atomic_exchange(atomic_id normie, value normie) normie` - Exchange values
- `concurrency_atomic_compare_and_swap(atomic_id normie, expected normie, new_value normie) lit` - Compare and swap
- `concurrency_atomic_fetch_add(atomic_id normie, value normie) normie` - Fetch and add
- `concurrency_atomic_fetch_sub(atomic_id normie, value normie) normie` - Fetch and subtract
- `concurrency_atomic_destroy(atomic_id normie) lit` - Destroy atomic variable

### Worker Pool Functions
- `concurrency_pool_create(worker_count normie) normie` - Create worker pool
- `concurrency_pool_submit_task(pool_id normie, task_function tea) normie` - Submit task
- `concurrency_pool_wait_for_task(pool_id normie, task_id normie) lit` - Wait for task
- `concurrency_pool_shutdown(pool_id normie) lit` - Shutdown pool
- `concurrency_pool_get_active_tasks(pool_id normie) normie` - Get active task count
- `concurrency_pool_get_completed_tasks(pool_id normie) normie` - Get completed task count
- `concurrency_pool_get_state(pool_id normie) smol` - Get pool state

### Channel Functions
- `concurrency_channel_create(buffer_size normie) normie` - Create channel
- `concurrency_channel_send(channel_id normie, message tea) lit` - Send message
- `concurrency_channel_receive(channel_id normie) tea` - Receive message
- `concurrency_channel_try_send(channel_id normie, message tea) lit` - Try send
- `concurrency_channel_try_receive(channel_id normie) tea` - Try receive
- `concurrency_channel_close(channel_id normie) lit` - Close channel
- `concurrency_channel_is_closed(channel_id normie) lit` - Check if closed
- `concurrency_channel_get_buffer_size(channel_id normie) normie` - Get buffer size
- `concurrency_channel_get_message_count(channel_id normie) normie` - Get message count

### Future/Promise Functions
- `concurrency_future_create() normie` - Create future
- `concurrency_future_set_value(future_id normie, value tea) lit` - Set future value
- `concurrency_future_get_value(future_id normie) tea` - Get future value
- `concurrency_future_is_ready(future_id normie) lit` - Check if ready
- `concurrency_future_wait(future_id normie) lit` - Wait for completion
- `concurrency_future_wait_timeout(future_id normie, timeout_ms normie) lit` - Wait with timeout
- `concurrency_future_destroy(future_id normie) lit` - Destroy future

### Utility Functions
- `concurrency_get_cpu_count() normie` - Get CPU count
- `concurrency_get_thread_count() normie` - Get thread count
- `concurrency_is_main_thread() lit` - Check if main thread
- `concurrency_get_current_thread_id() normie` - Get current thread ID
- `concurrency_set_thread_name(thread_id normie, name tea) lit` - Set thread name
- `concurrency_get_thread_name(thread_id normie) tea` - Get thread name

### Performance Functions
- `concurrency_get_context_switches() normie` - Get context switch count
- `concurrency_get_lock_contention() normie` - Get lock contention count
- `concurrency_get_deadlock_count() normie` - Get deadlock count
- `concurrency_reset_performance_counters() lit` - Reset performance counters

## Testing

Run the comprehensive test suite:

```bash
cargo run --bin cursed stdlib/concurrency/test_concurrency.💀
```

Test both interpretation and compilation modes:

```bash
cargo run --bin cursed stdlib/concurrency/test_concurrency.💀
cargo run --bin cursed -- compile stdlib/concurrency/test_concurrency.💀
./test_concurrency
```

## Error Handling

All functions return appropriate error values:
- Boolean functions return `cap` (false) on error
- Integer functions return -1 on error
- String functions return empty string on error

## Performance

- Lock-free atomic operations where possible
- Efficient thread pool implementation
- Optimized channel communication
- Minimal memory allocation
- Fast synchronization primitives
- Optimized for both interpretation and compilation modes

## Use Cases

- **Parallel Processing**: CPU-intensive tasks across multiple threads
- **Producer-Consumer**: Channel-based communication patterns
- **Resource Management**: Semaphores and mutexes for shared resources
- **Async Programming**: Futures and promises for non-blocking operations
- **Task Scheduling**: Worker pools for task-based parallelism
- **Data Pipeline**: Multi-stage processing with barriers and synchronization

## Dependencies

- `testz` - Testing framework
- `string` - String manipulation
- `collections` - Data structures
- `time` - Time utilities

## License

Part of the CURSED language standard library.
