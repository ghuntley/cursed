# Concurrenz Module

Pure CURSED implementation of synchronization primitives essential for async programming and concurrent systems.

## Overview

The concurrenz module provides thread-safe synchronization primitives for coordinating goroutines and shared resource access in CURSED programs. All implementations are FFI-free and use pure CURSED language features.

## Core Synchronization Primitives

### Mutex Operations
```cursed
yeet "concurrenz"

# Create and use mutex for exclusive access
sus mutex = create_mutex()
mutex_lock(mutex)
# Critical section
mutex_unlock(mutex)

# Non-blocking try-lock
lowkey mutex_trylock(mutex) {
    # Got lock, do work
    mutex_unlock(mutex)
}
```

### WaitGroup Coordination
```cursed
# Coordinate multiple goroutines
sus wg = create_waitgroup()
waitgroup_add(wg, 3)  # Expect 3 goroutines

# In each goroutine:
# do work...
waitgroup_done(wg)

# Wait for all to complete
waitgroup_wait(wg)
```

### Channel Communication
```cursed
# Synchronous channel for goroutine communication
sus channel = create_sync_channel()

# Producer goroutine
channel_send(channel, 42)

# Consumer goroutine  
sus data = channel_receive(channel)
```

## Advanced Synchronization

### Read-Write Mutex
```cursed
# Multiple readers, single writer pattern
sus rwmutex = create_rwmutex()

# Reader access (multiple allowed)
rwmutex_rlock(rwmutex)
sus data = shared_resource
rwmutex_runlock(rwmutex)

# Writer access (exclusive)
rwmutex_lock(rwmutex)
shared_resource = new_data
rwmutex_unlock(rwmutex)
```

### Condition Variables
```cursed
# Thread coordination with conditions
sus condition = create_condition()
sus mutex = create_mutex()

# Wait for condition
mutex_lock(mutex)
condition_wait(condition, mutex)
mutex_unlock(mutex)

# Signal waiting threads
condition_signal(condition)    # Signal one
condition_broadcast(condition) # Signal all
```

### Atomic Operations
```cursed
# Lock-free atomic operations
sus counter = 0
atomic_increment(counter)      # Thread-safe increment
atomic_decrement(counter)      # Thread-safe decrement

# Compare and swap
sus old_val = 10
sus new_val = 20
lowkey atomic_cas(counter, old_val, new_val) {
    # Swap succeeded
}
```

## Synchronization Patterns

### Barrier Synchronization
```cursed
# Wait for all participants
sus barrier = create_barrier(4)  # 4 participants

# Each participant calls:
barrier_wait(barrier)
# All continue together after barrier
```

### Semaphore Resource Counting
```cursed
# Control access to limited resources
sus semaphore = create_semaphore(3)  # 3 resources

# Acquire resource
lowkey semaphore_acquire(semaphore) {
    # Use resource
    semaphore_release(semaphore)
}
```

### Once Initialization
```cursed
# Execute initialization exactly once
sus once = create_once()

lowkey once_do(once, "init_function") {
    # Initialization code runs only once
}
```

## Common Patterns

### Producer-Consumer
```cursed
sus buffer = create_sync_channel()
sus mutex = create_mutex()

# Producer
stan {
    mutex_lock(mutex)
    channel_send(buffer, data)
    mutex_unlock(mutex)
}

# Consumer
stan {
    mutex_lock(mutex)
    sus item = channel_receive(buffer)
    mutex_unlock(mutex)
}
```

### Reader-Writer Coordination
```cursed
sus rwmutex = create_rwmutex()
sus shared_data = initial_value

# Reader goroutines
stan {
    rwmutex_rlock(rwmutex)
    sus local_copy = shared_data
    rwmutex_runlock(rwmutex)
}

# Writer goroutine
stan {
    rwmutex_lock(rwmutex)
    shared_data = new_value
    rwmutex_unlock(rwmutex)
}
```

### Worker Pool Coordination
```cursed
sus wg = create_waitgroup()
sus job_channel = create_sync_channel()

# Start workers
bestie i := 0; i < worker_count; i++ {
    waitgroup_add(wg, 1)
    stan {
        bestie {
            sus job = channel_receive(job_channel)
            # Process job
        }
        waitgroup_done(wg)
    }
}

# Send jobs
bestie job in jobs {
    channel_send(job_channel, job)
}

# Wait for completion
waitgroup_wait(wg)
```

## Function Reference

### Mutex Functions
- `create_mutex() Mutex` - Create new mutex
- `mutex_lock(mutex Mutex) lit` - Lock mutex (blocking)
- `mutex_unlock(mutex Mutex) lit` - Unlock mutex
- `mutex_trylock(mutex Mutex) lit` - Try lock (non-blocking)

### WaitGroup Functions
- `create_waitgroup() WaitGroup` - Create wait group
- `waitgroup_add(wg WaitGroup, count normie) lit` - Add count
- `waitgroup_done(wg WaitGroup) lit` - Mark done
- `waitgroup_wait(wg WaitGroup) lit` - Wait for completion

### Channel Functions
- `create_sync_channel() SyncChannel` - Create channel
- `channel_send(channel SyncChannel, data normie) lit` - Send data
- `channel_receive(channel SyncChannel) normie` - Receive data

### Read-Write Mutex Functions
- `create_rwmutex() Mutex` - Create RW mutex
- `rwmutex_rlock(rwmutex Mutex) lit` - Read lock
- `rwmutex_runlock(rwmutex Mutex) lit` - Read unlock
- `rwmutex_lock(rwmutex Mutex) lit` - Write lock
- `rwmutex_unlock(rwmutex Mutex) lit` - Write unlock

### Condition Variable Functions
- `create_condition() Mutex` - Create condition variable
- `condition_wait(condition Mutex, mutex Mutex) lit` - Wait on condition
- `condition_signal(condition Mutex) lit` - Signal one waiter
- `condition_broadcast(condition Mutex) lit` - Signal all waiters

### Atomic Functions
- `atomic_cas(addr Mutex, old normie, new normie) lit` - Compare and swap
- `atomic_increment(addr Mutex) normie` - Atomic increment
- `atomic_decrement(addr Mutex) normie` - Atomic decrement

### Other Primitives
- `create_barrier(count normie) WaitGroup` - Create barrier
- `barrier_wait(barrier WaitGroup) lit` - Wait at barrier
- `create_semaphore(initial normie) Mutex` - Create semaphore
- `semaphore_acquire(semaphore Mutex) lit` - Acquire resource
- `semaphore_release(semaphore Mutex) lit` - Release resource
- `create_once() lit` - Create once primitive
- `once_do(once lit, func tea) lit` - Execute once

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/concurrenz/test_concurrenz.csd

# Test compilation mode  
cargo run --bin cursed -- compile stdlib/concurrenz/test_concurrenz.csd
./test_concurrenz

# Both-mode verification
test_both_modes stdlib/concurrenz/test_concurrenz.csd
```

## Notes

- All primitives use pure CURSED implementations for maximum portability
- Synchronization semantics are simplified for demonstration
- Real-world implementations would use OS-specific primitives
- Thread safety is achieved through careful state management
- All functions are designed for use with CURSED's `stan` goroutines

## Integration

Import the module in your CURSED programs:

```cursed
yeet "concurrenz"

# Use synchronization primitives
sus mutex = create_mutex()
sus wg = create_waitgroup()
sus channel = create_sync_channel()
```

Perfect for building concurrent CURSED applications with proper synchronization and thread safety.
