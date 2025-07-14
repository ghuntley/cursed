# Advanced Concurrency Module

A comprehensive concurrency module for CURSED implementing channels, goroutines, mutexes, wait groups, condition variables, and advanced synchronization primitives in pure CURSED.

## Features

### Core Synchronization Primitives

- **Channels**: Buffered and unbuffered channels for goroutine communication
- **Mutexes**: Exclusive locks with owner tracking and deadlock prevention
- **Wait Groups**: Synchronization for multiple goroutines
- **Condition Variables**: Thread-safe signaling between goroutines
- **Semaphores**: Resource limiting and counting semaphores
- **Read-Write Locks**: Multiple readers, single writer synchronization
- **Barriers**: Synchronization points for multiple goroutines

### Advanced Features

- **Goroutine Pool**: Managed worker thread pools
- **Select Operations**: Non-blocking channel operations
- **Deadlock Detection**: Runtime deadlock detection utilities
- **Performance Monitoring**: Metrics and performance tracking
- **Atomic Operations**: Lock-free atomic primitives

## Channel Operations

```cursed
# Create buffered channel
sus ch tea = concurrency_advanced.channel_create(10)

# Send and receive
concurrency_advanced.channel_send(ch, "message")
sus msg tea = concurrency_advanced.channel_receive(ch)

# Close channel
concurrency_advanced.channel_close(ch)

# Check channel state
sus can_send lit = concurrency_advanced.channel_can_send(ch)
sus can_receive lit = concurrency_advanced.channel_can_receive(ch)
```

## Mutex Operations

```cursed
# Create mutex
sus mutex tea = concurrency_advanced.mutex_create()

# Lock and unlock
sus goroutine_id tea = "worker_1"
concurrency_advanced.mutex_lock(mutex, goroutine_id)
# Critical section
concurrency_advanced.mutex_unlock(mutex, goroutine_id)

# Try lock (non-blocking)
sus acquired lit = concurrency_advanced.mutex_try_lock(mutex, goroutine_id)
```

## Wait Groups

```cursed
# Create wait group
sus wg tea = concurrency_advanced.waitgroup_create()

# Add goroutines to wait for
concurrency_advanced.waitgroup_add(wg, 3)

# Mark goroutine as done
concurrency_advanced.waitgroup_done(wg)

# Wait for all goroutines to complete
concurrency_advanced.waitgroup_wait(wg)
```

## Condition Variables

```cursed
# Create condition variable and mutex
sus cond tea = concurrency_advanced.condition_create()
sus mutex tea = concurrency_advanced.mutex_create()

# Wait for condition
concurrency_advanced.condition_wait(cond, mutex, "goroutine_id")

# Signal one waiting goroutine
concurrency_advanced.condition_signal(cond)

# Signal all waiting goroutines
concurrency_advanced.condition_broadcast(cond)
```

## Semaphores

```cursed
# Create semaphore with 5 permits
sus sem tea = concurrency_advanced.semaphore_create(5)

# Acquire permit (blocking)
concurrency_advanced.semaphore_acquire(sem)

# Try acquire (non-blocking)
sus acquired lit = concurrency_advanced.semaphore_try_acquire(sem)

# Release permit
concurrency_advanced.semaphore_release(sem)
```

## Read-Write Locks

```cursed
# Create read-write lock
sus rwlock tea = concurrency_advanced.rwlock_create()

# Multiple readers
concurrency_advanced.rwlock_read_lock(rwlock)
# Read operations
concurrency_advanced.rwlock_read_unlock(rwlock)

# Exclusive writer
concurrency_advanced.rwlock_write_lock(rwlock)
# Write operations
concurrency_advanced.rwlock_write_unlock(rwlock)
```

## Barriers

```cursed
# Create barrier for 3 goroutines
sus barrier tea = concurrency_advanced.barrier_create(3)

# Wait at barrier (called by each goroutine)
concurrency_advanced.barrier_wait(barrier)
```

## Goroutine Pool

```cursed
# Create pool with 4 workers
sus pool tea = concurrency_advanced.goroutine_pool_create(4)

# Submit task to pool
concurrency_advanced.goroutine_pool_submit(pool, "task_function", "task_data")
```

## Select Operations

```cursed
# Create channels
sus ch1 tea = concurrency_advanced.channel_create(1)
sus ch2 tea = concurrency_advanced.channel_create(1)

# Set up select operation
sus channels [tea] = [ch1, ch2]
sus operations [tea] = ["receive", "send"]

# Find ready channel
sus ready_index normie = concurrency_advanced.select_operation(channels, operations)
```

## Concurrency Patterns

### Producer-Consumer

```cursed
# Create shared channel
sus work_ch tea = concurrency_advanced.channel_create(100)

# Producer goroutine
stan {
    concurrency_advanced.channel_send(work_ch, "work_item")
}

# Consumer goroutine
stan {
    sus item tea = concurrency_advanced.channel_receive(work_ch)
    # Process item
}
```

### Worker Pool

```cursed
# Create channels for work distribution
sus work_ch tea = concurrency_advanced.channel_create(1000)
sus result_ch tea = concurrency_advanced.channel_create(1000)

# Start workers
sus wg tea = concurrency_advanced.waitgroup_create()
concurrency_advanced.waitgroup_add(wg, 4)

bestie i := 0; i < 4; i++ {
    stan {
        bestie {
            sus work tea = concurrency_advanced.channel_receive(work_ch)
            when work == "" {
                ghosted  # Channel closed
            }
            # Process work and send result
            concurrency_advanced.channel_send(result_ch, "processed_" + work)
        }
        concurrency_advanced.waitgroup_done(wg)
    }
}
```

### Pipeline Processing

```cursed
# Three-stage pipeline
sus stage1_ch tea = concurrency_advanced.channel_create(10)
sus stage2_ch tea = concurrency_advanced.channel_create(10)
sus output_ch tea = concurrency_advanced.channel_create(10)

# Stage 1: Data input
stan {
    concurrency_advanced.channel_send(stage1_ch, "raw_data")
    concurrency_advanced.channel_close(stage1_ch)
}

# Stage 2: Processing
stan {
    bestie {
        sus data tea = concurrency_advanced.channel_receive(stage1_ch)
        when data == "" { ghosted }
        concurrency_advanced.channel_send(stage2_ch, "processed_" + data)
    }
    concurrency_advanced.channel_close(stage2_ch)
}

# Stage 3: Output
stan {
    bestie {
        sus data tea = concurrency_advanced.channel_receive(stage2_ch)
        when data == "" { ghosted }
        concurrency_advanced.channel_send(output_ch, "final_" + data)
    }
    concurrency_advanced.channel_close(output_ch)
}
```

## Performance Monitoring

```cursed
# Get concurrency metrics
sus metrics tea = concurrency_advanced.concurrency_metrics()
vibez.spill("Metrics: " + metrics)

# Create deadlock detector
sus detector tea = concurrency_advanced.deadlock_detector_create()
sus has_deadlock lit = concurrency_advanced.deadlock_check(detector, resource_graph)
```

## Testing

```bash
# Run all concurrency tests
cargo run --bin cursed stdlib/concurrency_advanced/test_concurrency_advanced.csd

# Test both interpretation and compilation modes
cargo run --bin cursed -- compile stdlib/concurrency_advanced/test_concurrency_advanced.csd
./test_concurrency_advanced
```

## Implementation Details

- **Pure CURSED**: No FFI dependencies, implemented entirely in CURSED
- **Atomic Operations**: Uses atomic_drip module for lock-free operations
- **Memory Safe**: All operations are memory safe with proper cleanup
- **Deadlock Prevention**: Built-in deadlock detection and prevention
- **Performance Optimized**: Efficient implementation suitable for production use

## Thread Safety

All operations are thread-safe and can be used concurrently from multiple goroutines without additional synchronization.

## Error Handling

Functions return `lit` (boolean) values indicating success/failure. Failed operations return `cap` (false).

## Memory Management

The module properly manages memory and cleans up resources automatically. No manual cleanup required.
