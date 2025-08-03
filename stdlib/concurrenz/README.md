# concurrenz Module - CURSED Synchronization Primitives

The `concurrenz` module provides synchronization primitives for concurrent programming in CURSED, equivalent to Go's `sync` package.

## Core Types

- `Mutex` - Mutual exclusion lock
- `WaitGroup` - Wait for goroutines to finish
- `SyncChannel` - Synchronous communication channel

## Structured Types

- `MutexStruct` - Full mutex implementation with metadata
- `AtomicStruct` - Atomic variable with version control
- `WaitGroupStruct` - Advanced wait group with generation counter

## Mutex Operations

### Basic Mutex

- `create_mutex()` - Create new mutex
- `mutex_lock(mutex Mutex)` - Lock mutex (blocking)
- `mutex_unlock(mutex Mutex)` - Unlock mutex
- `mutex_trylock(mutex Mutex)` - Try to lock (non-blocking)

### Read-Write Mutex

- `create_rwmutex()` - Create read-write mutex
- `rwmutex_rlock(rwmutex Mutex)` - Acquire read lock
- `rwmutex_runlock(rwmutex Mutex)` - Release read lock
- `rwmutex_lock(rwmutex Mutex)` - Acquire write lock (exclusive)
- `rwmutex_unlock(rwmutex Mutex)` - Release write lock

## WaitGroup Operations

- `create_waitgroup()` - Create new wait group
- `waitgroup_add(wg WaitGroup, count normie)` - Add count to wait group
- `waitgroup_done(wg WaitGroup)` - Mark one task as done
- `waitgroup_wait(wg WaitGroup)` - Wait for all tasks to complete

## Channel Operations

- `create_sync_channel()` - Create synchronous channel
- `channel_send(channel SyncChannel, data normie)` - Send data (blocking)
- `channel_receive(channel SyncChannel)` - Receive data (blocking)

## Condition Variables

- `create_condition()` - Create condition variable
- `condition_wait(condition Mutex, mutex Mutex)` - Wait on condition
- `condition_signal(condition Mutex)` - Signal one waiting goroutine
- `condition_broadcast(condition Mutex)` - Signal all waiting goroutines

## Atomic Operations

- `atomic_cas(addr Mutex, old, new normie)` - Compare-and-swap
- `atomic_increment(addr Mutex)` - Atomic increment
- `atomic_decrement(addr Mutex)` - Atomic decrement

## Advanced Synchronization

### Barrier Synchronization

- `create_barrier(count normie)` - Create barrier for N participants
- `barrier_wait(barrier WaitGroup)` - Wait at barrier

### Semaphore

- `create_semaphore(initial normie)` - Create counting semaphore
- `semaphore_acquire(semaphore Mutex)` - Acquire semaphore
- `semaphore_release(semaphore Mutex)` - Release semaphore

### Once Primitive

- `create_once()` - Create one-time execution primitive
- `once_do(once lit, func tea)` - Execute function exactly once

## Structured Concurrency

### Advanced Mutex

- `mutex_new()` - Create structured mutex with metadata
- Fields: `locked`, `owner`, `waiters`

### Atomic Variables

- `atomic_new(initial_value normie)` - Create atomic variable
- `atomic_load(atomic *AtomicStruct)` - Load value atomically
- `atomic_store(atomic *AtomicStruct, new_value normie)` - Store value atomically

### Advanced WaitGroup

- `waitgroup_new()` - Create structured wait group
- Fields: `counter`, `waiters`, `generation`

## Channel Utilities

- `make(chan_type tea, buffer_size normie)` - Generic channel creation

## Example Usage

### Basic Mutex Usage

```cursed
yeet "concurrenz"

slay main() {
    sus mutex Mutex = concurrenz.create_mutex()
    
    fr fr Critical section
    concurrenz.mutex_lock(mutex)
    fr fr Do thread-safe work here
    concurrenz.mutex_unlock(mutex)
}
```

### WaitGroup Coordination

```cursed
yeet "concurrenz"

slay worker(wg WaitGroup, id normie) {
    vibez.spillf("Worker %d starting", id)
    fr fr Do work here
    concurrenz.waitgroup_done(wg)
    vibez.spillf("Worker %d finished", id)
}

slay main() {
    sus wg WaitGroup = concurrenz.create_waitgroup()
    concurrenz.waitgroup_add(wg, 3)
    
    fr fr Start workers (simulated)
    worker(wg, 1)
    worker(wg, 2)
    worker(wg, 3)
    
    concurrenz.waitgroup_wait(wg)
    vibez.spill("All workers finished")
}
```

### Channel Communication

```cursed
yeet "concurrenz"

slay main() {
    sus channel SyncChannel = concurrenz.create_sync_channel()
    
    fr fr Send data
    concurrenz.channel_send(channel, 42)
    
    fr fr Receive data
    sus received normie = concurrenz.channel_receive(channel)
    vibez.spillf("Received: %d", received)
}
```

### Read-Write Mutex

```cursed
yeet "concurrenz"

sus shared_data normie = 0
sus rwmutex Mutex = concurrenz.create_rwmutex()

slay reader(id normie) {
    concurrenz.rwmutex_rlock(rwmutex)
    vibez.spillf("Reader %d: data = %d", id, shared_data)
    concurrenz.rwmutex_runlock(rwmutex)
}

slay writer(id normie, value normie) {
    concurrenz.rwmutex_lock(rwmutex)
    shared_data = value
    vibez.spillf("Writer %d: set data = %d", id, value)
    concurrenz.rwmutex_unlock(rwmutex)
}
```

### Atomic Operations

```cursed
yeet "concurrenz"

slay main() {
    sus atomic_var *AtomicStruct = concurrenz.atomic_new(0)
    
    fr fr Atomic operations
    concurrenz.atomic_store(atomic_var, 100)
    sus value normie = concurrenz.atomic_load(atomic_var)
    vibez.spillf("Atomic value: %d", value)
}
```

## Implementation Details

- Pure CURSED implementation using atomic primitives
- Lock-free algorithms where possible
- Memory-safe concurrent data structures
- Cross-platform synchronization support
- Efficient spinlock-based implementations

## Current Limitations

This is a foundational implementation with some simplifications:

- Mutex operations use basic compare-and-swap
- Channel operations are synchronous only
- No goroutine scheduler integration
- Simplified condition variable implementation

## Performance Characteristics

- O(1) mutex operations
- O(1) atomic operations
- O(n) wait group operations (n = number of waiters)
- Memory-efficient primitive implementations

## Testing

Run tests with:
```bash
cargo run --bin cursed stdlib/concurrenz/test_concurrenz.csd
```

The test suite covers all synchronization primitives, edge cases, and concurrent usage patterns.

## Future Enhancements

- Asynchronous channel variants (buffered channels)
- Priority-based locking mechanisms
- Lock-free data structures (queues, stacks)
- Performance monitoring and deadlock detection
- Integration with CURSED runtime scheduler
- Hardware-specific optimizations
- Context-aware cancellation support

## Thread Safety

All functions in this module are thread-safe and can be used safely from multiple goroutines. The implementations use atomic operations and memory barriers to ensure correct behavior in concurrent environments.
