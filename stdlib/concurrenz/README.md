# Concurrenz (Sync) Module

The `concurrenz` module provides synchronization primitives for concurrent programming in CURSED. This module implements essential synchronization types and operations needed for thread-safe programming and coordination between goroutines.

## Overview

The concurrenz module is implemented in pure CURSED without FFI dependencies, ensuring maximum portability and integration with the CURSED runtime. It provides low-level synchronization primitives that can be used to build higher-level concurrent data structures and algorithms.

## Types

### Mutex
Mutual exclusion lock for protecting shared resources.

```cursed
sus m := concurrenz.mutex_new()
concurrenz.mutex_lock(&m)
# Critical section
concurrenz.mutex_unlock(&m)
```

### WaitGroup
Synchronization primitive for waiting on multiple goroutines to complete.

```cursed
sus wg := concurrenz.waitgroup_new()
concurrenz.waitgroup_add(&wg, 3)

# In goroutines:
yolo {
    # Do work
    concurrenz.waitgroup_done(&wg)
}

concurrenz.waitgroup_wait(&wg)  # Wait for all to complete
```

### Once
Ensures a function is executed exactly once across multiple goroutines.

```cursed
sus once := concurrenz.once_new()
concurrenz.once_do(&once, initialization_function)
```

### AtomicInt
Thread-safe integer operations without explicit locking.

```cursed
sus ai := concurrenz.atomic_int_new(42)
sus value := concurrenz.atomic_int_load(&ai)
concurrenz.atomic_int_store(&ai, 100)
sus new_value := concurrenz.atomic_int_add(&ai, 50)
```

### AtomicBool
Thread-safe boolean operations.

```cursed
sus ab := concurrenz.atomic_bool_new(cap)
sus current := concurrenz.atomic_bool_load(&ab)
concurrenz.atomic_bool_store(&ab, based)
sus swapped := concurrenz.atomic_bool_compare_and_swap(&ab, based, cap)
```

### RWMutex
Read-write mutex allowing multiple readers or single writer.

```cursed
sus rw := concurrenz.rwmutex_new()

# For reading:
concurrenz.rwmutex_read_lock(&rw)
# Read operations
concurrenz.rwmutex_read_unlock(&rw)

# For writing:
concurrenz.rwmutex_write_lock(&rw)
# Write operations
concurrenz.rwmutex_write_unlock(&rw)
```

### Condition Variable
Signaling mechanism for goroutines to wait for specific conditions.

```cursed
sus mutex := concurrenz.mutex_new()
sus cond := concurrenz.cond_new(&mutex)

# Waiting goroutine:
concurrenz.mutex_lock(&mutex)
concurrenz.cond_wait(&cond)
concurrenz.mutex_unlock(&mutex)

# Signaling goroutine:
concurrenz.cond_signal(&cond)  # Wake one waiter
concurrenz.cond_broadcast(&cond)  # Wake all waiters
```

### Barrier
Synchronization point where multiple goroutines wait for each other.

```cursed
sus barrier := concurrenz.barrier_new(3)  # 3 goroutines

# In each goroutine:
# Do work
concurrenz.barrier_wait(&barrier)  # Wait for all to reach this point
# Continue after all reached barrier
```

### Semaphore
Counting semaphore for limiting resource access.

```cursed
sus sem := concurrenz.semaphore_new(5)  # 5 permits

concurrenz.semaphore_acquire(&sem)
# Use resource
concurrenz.semaphore_release(&sem)

# Non-blocking attempt:
fr concurrenz.semaphore_try_acquire(&sem) {
    # Got resource
    concurrenz.semaphore_release(&sem)
}
```

## Functions

### Mutex Operations
- `mutex_new() Mutex` - Create new mutex
- `mutex_lock(m *Mutex) lit` - Acquire lock (blocks if necessary)
- `mutex_unlock(m *Mutex) lit` - Release lock
- `mutex_try_lock(m *Mutex) lit` - Try to acquire lock (non-blocking)

### WaitGroup Operations
- `waitgroup_new() WaitGroup` - Create new wait group
- `waitgroup_add(wg *WaitGroup, delta normie) lit` - Add count
- `waitgroup_done(wg *WaitGroup) lit` - Decrement count
- `waitgroup_wait(wg *WaitGroup) lit` - Wait for count to reach zero

### Once Operations
- `once_new() Once` - Create new once
- `once_do(o *Once, f slay()) lit` - Execute function once

### AtomicInt Operations
- `atomic_int_new(value normie) AtomicInt` - Create atomic integer
- `atomic_int_load(ai *AtomicInt) normie` - Load value
- `atomic_int_store(ai *AtomicInt, value normie) lit` - Store value
- `atomic_int_add(ai *AtomicInt, delta normie) normie` - Add and return new value
- `atomic_int_compare_and_swap(ai *AtomicInt, old normie, new normie) lit` - CAS operation

### AtomicBool Operations
- `atomic_bool_new(value lit) AtomicBool` - Create atomic boolean
- `atomic_bool_load(ab *AtomicBool) lit` - Load value
- `atomic_bool_store(ab *AtomicBool, value lit) lit` - Store value
- `atomic_bool_compare_and_swap(ab *AtomicBool, old lit, new lit) lit` - CAS operation

### RWMutex Operations
- `rwmutex_new() RWMutex` - Create read-write mutex
- `rwmutex_read_lock(rw *RWMutex) lit` - Acquire read lock
- `rwmutex_read_unlock(rw *RWMutex) lit` - Release read lock
- `rwmutex_write_lock(rw *RWMutex) lit` - Acquire write lock
- `rwmutex_write_unlock(rw *RWMutex) lit` - Release write lock

### Condition Variable Operations
- `cond_new(mutex *Mutex) Cond` - Create condition variable
- `cond_wait(c *Cond) lit` - Wait for signal
- `cond_signal(c *Cond) lit` - Signal one waiter
- `cond_broadcast(c *Cond) lit` - Signal all waiters

### Barrier Operations
- `barrier_new(count normie) Barrier` - Create barrier for count goroutines
- `barrier_wait(b *Barrier) lit` - Wait at barrier

### Semaphore Operations
- `semaphore_new(permits normie) Semaphore` - Create semaphore
- `semaphore_acquire(s *Semaphore) lit` - Acquire permit
- `semaphore_release(s *Semaphore) lit` - Release permit
- `semaphore_try_acquire(s *Semaphore) lit` - Try to acquire permit

### Memory Fence Operations
- `memory_fence() lit` - Full memory barrier
- `acquire_fence() lit` - Acquire memory barrier
- `release_fence() lit` - Release memory barrier

## Usage Examples

### Producer-Consumer with Mutex
```cursed
sus mutex := concurrenz.mutex_new()
sus buffer [10]normie
sus count := 0

# Producer
slay producer() lit {
    bestie i := 0; i < 5; i++ {
        concurrenz.mutex_lock(&mutex)
        buffer[count] = i
        count++
        concurrenz.mutex_unlock(&mutex)
    }
    damn based
}

# Consumer
slay consumer() lit {
    bestie count > 0 {
        concurrenz.mutex_lock(&mutex)
        fr count > 0 {
            sus value := buffer[count-1]
            count--
            vibez.spill("Consumed: ", value)
        }
        concurrenz.mutex_unlock(&mutex)
    }
    damn based
}
```

### Atomic Counter
```cursed
sus counter := concurrenz.atomic_int_new(0)

slay increment_counter() lit {
    bestie i := 0; i < 1000; i++ {
        concurrenz.atomic_int_add(&counter, 1)
    }
    damn based
}

# Launch multiple goroutines
yolo increment_counter()
yolo increment_counter()
yolo increment_counter()

# Final count will be 3000
```

### Read-Write Lock Example
```cursed
sus rw := concurrenz.rwmutex_new()
sus shared_data := 42

# Reader
slay reader() lit {
    concurrenz.rwmutex_read_lock(&rw)
    vibez.spill("Reading: ", shared_data)
    concurrenz.rwmutex_read_unlock(&rw)
    damn based
}

# Writer
slay writer() lit {
    concurrenz.rwmutex_write_lock(&rw)
    shared_data = 100
    vibez.spill("Updated data to: ", shared_data)
    concurrenz.rwmutex_write_unlock(&rw)
    damn based
}
```

## Thread Safety

All operations in the concurrenz module are designed to be thread-safe and can be used safely across multiple goroutines. The implementation uses the CURSED runtime's built-in synchronization mechanisms and atomic operations where appropriate.

## Performance Considerations

- **Mutex**: Lightweight with minimal overhead for uncontended locks
- **Atomic Operations**: Lock-free for maximum performance
- **RWMutex**: Optimized for read-heavy workloads
- **Semaphore**: Efficient for resource pooling scenarios
- **WaitGroup**: Minimal overhead for goroutine coordination

## Testing

Run the comprehensive test suite:

```bash
# Test interpretation mode
cargo run --bin cursed stdlib/concurrenz/test_concurrenz.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/concurrenz/test_concurrenz.csd
./test_concurrenz
```

The test suite includes:
- Basic functionality tests for all types
- Concurrent scenario simulations
- Edge case handling
- Error condition testing
- Performance validation

## Implementation Details

The concurrenz module is implemented entirely in pure CURSED without FFI dependencies. This ensures:
- Maximum portability across platforms
- Seamless integration with CURSED runtime
- Consistent behavior in both interpretation and compilation modes
- No external library dependencies

The implementation leverages CURSED's built-in goroutine system and channel primitives to provide efficient synchronization mechanisms that integrate naturally with the language's concurrency model.
