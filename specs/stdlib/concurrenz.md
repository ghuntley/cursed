# concurrenz (Synchronization Primitives)

## Overview
`concurrenz` provides comprehensive synchronization primitives and concurrent programming utilities for CURSED. This module implements thread-safe data structures, coordination primitives, and atomic operations essential for building robust concurrent applications. All functions are implemented in pure CURSED without external dependencies.

## Core Synchronization Types

### Type Definitions
```cursed
be_like Mutex = mid          # Mutual exclusion lock
be_like WaitGroup = mid      # Goroutine synchronization
be_like SyncChannel = mid    # Synchronous communication channel
```

These type aliases provide clear semantic meaning for synchronization primitives.

## Mutex Operations

### Basic Mutex Functions
```cursed
slay create_mutex() Mutex
slay mutex_lock(mutex Mutex) lit
slay mutex_unlock(mutex Mutex) lit
slay mutex_trylock(mutex Mutex) lit
```

#### Create Mutex
```cursed
slay create_mutex() Mutex
```
Creates a new mutex for exclusive access synchronization.

**Returns:**
- `Mutex`: Initialized mutex in unlocked state

#### Lock Mutex
```cursed
slay mutex_lock(mutex Mutex) lit
```
Acquires exclusive lock (blocking operation).

**Parameters:**
- `mutex Mutex`: The mutex to lock

**Returns:**
- `lit`: `based` if lock acquired, `cap` if already locked

#### Unlock Mutex
```cursed
slay mutex_unlock(mutex Mutex) lit
```
Releases exclusive lock.

**Parameters:**
- `mutex Mutex`: The mutex to unlock

**Returns:**
- `lit`: `based` on successful unlock

#### Try Lock
```cursed
slay mutex_trylock(mutex Mutex) lit
```
Attempts to acquire lock without blocking.

**Parameters:**
- `mutex Mutex`: The mutex to try locking

**Returns:**
- `lit`: `based` if lock acquired, `cap` if busy

**Examples:**
```cursed
sus m := create_mutex()
sus locked := mutex_lock(m)        # Returns based
sus try_fail := mutex_trylock(m)   # Returns cap (already locked)
sus unlocked := mutex_unlock(m)    # Returns based
sus try_success := mutex_trylock(m) # Returns based (now available)
```

## Wait Group Operations

### Wait Group Functions
```cursed
slay create_waitgroup() WaitGroup
slay waitgroup_add(wg WaitGroup, count normie) lit
slay waitgroup_done(wg WaitGroup) lit
slay waitgroup_wait(wg WaitGroup) lit
```

#### Create Wait Group
```cursed
slay create_waitgroup() WaitGroup
```
Creates a new wait group for goroutine coordination.

**Returns:**
- `WaitGroup`: Initialized wait group with zero count

#### Add to Wait Group
```cursed
slay waitgroup_add(wg WaitGroup, count normie) lit
```
Adds count to the wait group counter.

**Parameters:**
- `wg WaitGroup`: The wait group to modify
- `count normie`: Number to add to counter

**Returns:**
- `lit`: `based` on successful addition

#### Mark Done
```cursed
slay waitgroup_done(wg WaitGroup) lit
```
Decrements wait group counter by one.

**Parameters:**
- `wg WaitGroup`: The wait group to decrement

**Returns:**
- `lit`: `based` if decremented, `cap` if already zero

#### Wait for Completion
```cursed
slay waitgroup_wait(wg WaitGroup) lit
```
Blocks until wait group counter reaches zero.

**Parameters:**
- `wg WaitGroup`: The wait group to wait on

**Returns:**
- `lit`: `based` when all tasks complete

**Examples:**
```cursed
sus wg := create_waitgroup()
waitgroup_add(wg, 3)           # Add 3 tasks
# ... spawn 3 goroutines ...
# Each goroutine calls waitgroup_done(wg)
waitgroup_wait(wg)             # Wait for all 3 to complete
```

## Channel Operations

### Synchronous Channel Functions
```cursed
slay create_sync_channel() SyncChannel
slay channel_send(channel SyncChannel, data normie) lit
slay channel_receive(channel SyncChannel) normie
```

#### Create Channel
```cursed
slay create_sync_channel() SyncChannel
```
Creates a synchronous communication channel.

**Returns:**
- `SyncChannel`: Initialized synchronous channel

#### Send Data
```cursed
slay channel_send(channel SyncChannel, data normie) lit
```
Sends data through channel (blocking until received).

**Parameters:**
- `channel SyncChannel`: The channel to send through
- `data normie`: The data value to send

**Returns:**
- `lit`: `based` on successful send

#### Receive Data
```cursed
slay channel_receive(channel SyncChannel) normie
```
Receives data from channel (blocking until available).

**Parameters:**
- `channel SyncChannel`: The channel to receive from

**Returns:**
- `normie`: The received data value

**Examples:**
```cursed
sus ch := create_sync_channel()
# In goroutine 1:
channel_send(ch, 42)
# In goroutine 2:
sus value := channel_receive(ch)  # Receives 42
```

## Read-Write Mutex Operations

### RWMutex Functions
```cursed
slay create_rwmutex() Mutex
slay rwmutex_rlock(rwmutex Mutex) lit
slay rwmutex_runlock(rwmutex Mutex) lit
slay rwmutex_lock(rwmutex Mutex) lit
slay rwmutex_unlock(rwmutex Mutex) lit
```

#### Create RWMutex
```cursed
slay create_rwmutex() Mutex
```
Creates a read-write mutex for shared resource access.

**Returns:**
- `Mutex`: Initialized RWMutex

#### Read Lock Operations
```cursed
slay rwmutex_rlock(rwmutex Mutex) lit    # Acquire read lock
slay rwmutex_runlock(rwmutex Mutex) lit  # Release read lock
```

Multiple readers can hold read locks simultaneously.

#### Write Lock Operations
```cursed
slay rwmutex_lock(rwmutex Mutex) lit     # Acquire write lock
slay rwmutex_unlock(rwmutex Mutex) lit   # Release write lock
```

Write lock is exclusive - no other readers or writers allowed.

**Examples:**
```cursed
sus rw := create_rwmutex()
# Multiple readers:
rwmutex_rlock(rw)   # Reader 1
rwmutex_rlock(rw)   # Reader 2 (allowed)
rwmutex_runlock(rw) # Reader 1 done
rwmutex_runlock(rw) # Reader 2 done

# Exclusive writer:
rwmutex_lock(rw)    # Writer (exclusive)
rwmutex_unlock(rw)  # Writer done
```

## Condition Variables

### Condition Variable Functions
```cursed
slay create_condition() Mutex
slay condition_wait(condition Mutex, mutex Mutex) lit
slay condition_signal(condition Mutex) lit
slay condition_broadcast(condition Mutex) lit
```

#### Create Condition
```cursed
slay create_condition() Mutex
```
Creates a condition variable for thread coordination.

**Returns:**
- `Mutex`: Initialized condition variable

#### Wait on Condition
```cursed
slay condition_wait(condition Mutex, mutex Mutex) lit
```
Waits for condition signal while releasing mutex.

**Parameters:**
- `condition Mutex`: The condition variable
- `mutex Mutex`: Associated mutex (released during wait)

**Returns:**
- `lit`: `based` when signaled and mutex reacquired

#### Signal Condition
```cursed
slay condition_signal(condition Mutex) lit     # Wake one waiter
slay condition_broadcast(condition Mutex) lit  # Wake all waiters
```

**Examples:**
```cursed
sus cond := create_condition()
sus m := create_mutex()

# Waiter:
mutex_lock(m)
condition_wait(cond, m)  # Wait for signal
mutex_unlock(m)

# Signaler:
condition_signal(cond)   # Wake one waiter
condition_broadcast(cond) # Wake all waiters
```

## Atomic Operations

### Atomic Functions
```cursed
slay atomic_cas(addr Mutex, old normie, new normie) lit
slay atomic_increment(addr Mutex) normie
slay atomic_decrement(addr Mutex) normie
```

#### Compare and Swap
```cursed
slay atomic_cas(addr Mutex, old normie, new normie) lit
```
Atomically compares and swaps value.

**Parameters:**
- `addr Mutex`: Memory address to modify
- `old normie`: Expected current value
- `new normie`: New value to set

**Returns:**
- `lit`: `based` if swap occurred, `cap` if value didn't match

#### Atomic Increment/Decrement
```cursed
slay atomic_increment(addr Mutex) normie
slay atomic_decrement(addr Mutex) normie
```

**Parameters:**
- `addr Mutex`: Memory address to modify

**Returns:**
- `normie`: Previous value before operation

**Examples:**
```cursed
sus counter := create_mutex()  # Use as atomic counter
sus old_val := atomic_increment(counter)  # Returns 0, counter now 1
sus swapped := atomic_cas(counter, 1, 5)  # Returns based, counter now 5
sus prev := atomic_decrement(counter)     # Returns 5, counter now 4
```

## Barrier Synchronization

### Barrier Functions
```cursed
slay create_barrier(count normie) WaitGroup
slay barrier_wait(barrier WaitGroup) lit
```

#### Create Barrier
```cursed
slay create_barrier(count normie) WaitGroup
```
Creates a barrier for coordinating multiple goroutines.

**Parameters:**
- `count normie`: Number of participants required

**Returns:**
- `WaitGroup`: Initialized barrier

#### Wait at Barrier
```cursed
slay barrier_wait(barrier WaitGroup) lit
```
Waits until all participants reach the barrier.

**Parameters:**
- `barrier WaitGroup`: The barrier to wait at

**Returns:**
- `lit`: `based` when all participants arrive

**Examples:**
```cursed
sus barrier := create_barrier(3)  # 3 goroutines must synchronize

# In each of 3 goroutines:
# ... do work ...
barrier_wait(barrier)  # All 3 will proceed together
# ... continue work ...
```

## Semaphore Operations

### Semaphore Functions
```cursed
slay create_semaphore(initial normie) Mutex
slay semaphore_acquire(semaphore Mutex) lit
slay semaphore_release(semaphore Mutex) lit
```

#### Create Semaphore
```cursed
slay create_semaphore(initial normie) Mutex
```
Creates a counting semaphore for resource management.

**Parameters:**
- `initial normie`: Initial resource count

**Returns:**
- `Mutex`: Initialized semaphore

#### Acquire/Release Semaphore
```cursed
slay semaphore_acquire(semaphore Mutex) lit  # Decrement count
slay semaphore_release(semaphore Mutex) lit  # Increment count
```

**Examples:**
```cursed
sus sem := create_semaphore(2)  # Allow 2 concurrent accesses
semaphore_acquire(sem)          # Count: 1 remaining
semaphore_acquire(sem)          # Count: 0 remaining
# semaphore_acquire(sem)        # Would block (no resources)
semaphore_release(sem)          # Count: 1 available
```

## Once Primitive

### Once Functions
```cursed
slay create_once() lit
slay once_do(once lit, func tea) lit
```

#### Create Once
```cursed
slay create_once() lit
```
Creates a once primitive for one-time initialization.

**Returns:**
- `lit`: Initialized once primitive

#### Execute Once
```cursed
slay once_do(once lit, func tea) lit
```
Executes function exactly once across all goroutines.

**Parameters:**
- `once lit`: The once primitive
- `func tea`: Function identifier (placeholder)

**Returns:**
- `lit`: `based` if function was executed, `cap` if already called

**Examples:**
```cursed
sus once := create_once()
# Multiple goroutines can call this:
once_do(once, "initialize")  # Only first call executes
```

## Error Handling

### Error Conditions
- Operations on invalid synchronization objects fail gracefully
- Blocking operations in this implementation use busy-waiting
- All functions return boolean success indicators
- No panics or exceptions thrown

### Safety Guarantees
- Memory-safe operations on all primitives
- Consistent state maintenance across operations
- Graceful handling of edge cases

## Performance Characteristics

### Time Complexity
- Mutex operations: O(1) for lock/unlock attempts
- Atomic operations: O(1) for all primitive operations
- Wait operations: O(1) for state checks, blocking for coordination
- Channel operations: O(1) for send/receive

### Memory Usage
- All primitives use constant memory
- No dynamic allocation for synchronization state
- Efficient representation using basic integer types

## Usage Patterns

### Producer-Consumer Pattern
```cursed
yeet "concurrenz"

# Shared resource protection
sus shared_data := create_mutex()
sus data_ready := create_condition()
sus m := create_mutex()

# Producer
slay producer() {
    mutex_lock(m)
    # ... produce data ...
    condition_signal(data_ready)  # Notify consumer
    mutex_unlock(m)
}

# Consumer
slay consumer() {
    mutex_lock(m)
    condition_wait(data_ready, m) # Wait for data
    # ... consume data ...
    mutex_unlock(m)
}
```

### Worker Pool Coordination
```cursed
# Coordinate multiple workers
sus wg := create_waitgroup()
sus job_count normie = 10

waitgroup_add(wg, job_count)

# Start workers
bestie i := 0; i < job_count; i++ {
    yolo worker(i, wg)  # Start goroutine
}

waitgroup_wait(wg)  # Wait for all workers

slay worker(id normie, wg WaitGroup) {
    # ... do work ...
    waitgroup_done(wg)  # Signal completion
}
```

### Resource Pool Management
```cursed
# Limit concurrent access to resources
sus resource_pool := create_semaphore(5)  # Max 5 concurrent users

slay use_resource() {
    lowkey semaphore_acquire(resource_pool) {
        # ... use limited resource ...
        semaphore_release(resource_pool)
    }
}
```

### Thread-Safe Counter
```cursed
# Atomic counter implementation
sus counter := create_mutex()

slay increment_counter() normie {
    damn atomic_increment(counter)
}

slay get_counter() normie {
    damn counter  # Read current value
}
```

## Implementation Notes

### Pure CURSED Implementation
- All synchronization primitives implemented in pure CURSED
- No external threading library dependencies
- Compatible with both interpretation and compilation modes

### Concurrency Model
- Based on goroutines and channels paradigm
- Mutex and atomic operations for shared state
- Message passing preferred over shared memory

### Thread Safety
- All primitive operations are thread-safe by design
- Consistent memory model across all operations
- Safe for use in highly concurrent applications

## Testing Strategy

### Unit Tests
```cursed
yeet "testz"
yeet "concurrenz"

# Test mutex operations
test_start("mutex functionality")
sus m := create_mutex()
assert_true(mutex_lock(m))
assert_false(mutex_trylock(m))  # Should fail when locked
assert_true(mutex_unlock(m))

# Test wait group
test_start("wait group coordination")
sus wg := create_waitgroup()
assert_true(waitgroup_add(wg, 2))
assert_true(waitgroup_done(wg))
assert_true(waitgroup_done(wg))

# Test atomic operations
test_start("atomic operations")
sus addr := create_mutex()
sus old := atomic_increment(addr)
assert_eq_normie(old, 0)
assert_true(atomic_cas(addr, 1, 5))

print_test_summary()
```

### Concurrency Tests
- Race condition detection
- Deadlock prevention verification
- Performance under high contention
- Correctness of synchronization semantics

## Dependencies

- `core`: Basic types and language primitives
- No external threading or synchronization libraries
- No FFI dependencies for thread operations

## Security Considerations

- Memory-safe synchronization operations
- No buffer overflows in primitive implementations
- Deterministic behavior prevents timing attacks
- Safe resource management patterns

## Compatibility

### Language Versions
- Compatible with all CURSED language versions
- Uses only core language synchronization features
- No version-specific threading syntax

### Platform Support
- Consistent synchronization semantics across platforms
- Portable pure CURSED implementation
- No platform-specific threading calls
