# Concurrenz Module - Complete Synchronization Primitives

The `concurrenz` module provides a comprehensive suite of concurrency primitives for CURSED programs, implementing thread-safe synchronization with hardware-level atomic operations.

## Features

### Core Synchronization Primitives

- **Mutexes** - Thread-safe locking with atomic compare-and-swap operations
- **WaitGroups** - Goroutine synchronization with atomic counters  
- **Channels** - Buffered and synchronous communication primitives
- **Read-Write Mutexes** - Shared/exclusive locking for reader-writer scenarios

### Atomic Operations

- **32-bit & 64-bit Atomics** - Lock-free integer operations with memory ordering
- **Atomic Booleans** - Thread-safe boolean values
- **Compare-and-Swap** - Hardware-level atomic compare-and-exchange
- **Fetch-and-Add/Sub** - Atomic arithmetic operations

### Advanced Primitives

- **Semaphores** - Resource counting with atomic permits
- **Barriers** - Multi-goroutine synchronization points
- **Condition Variables** - Thread coordination with signal/broadcast
- **Thread Pools** - Concurrent task execution management
- **Once Primitives** - One-time initialization guarantees

### Memory Ordering

All atomic operations support proper memory ordering semantics:
- `RELAXED` - No ordering constraints
- `ACQUIRE` - Acquire semantics for loads
- `RELEASE` - Release semantics for stores  
- `ACQREL` - Combined acquire-release
- `SEQCST` - Sequential consistency

## Core Types

### Mutex Operations
- `create_mutex()` - Create mutex with atomic state
- `mutex_lock(mutex *Mutex)` - Lock mutex (blocking)
- `mutex_unlock(mutex *Mutex)` - Unlock mutex  
- `mutex_trylock(mutex *Mutex)` - Try to lock (non-blocking)

### WaitGroup Operations
- `create_waitgroup()` - Create waitgroup with atomic counters
- `waitgroup_add(wg *WaitGroup, count normie)` - Add count atomically
- `waitgroup_done(wg *WaitGroup)` - Mark one task done
- `waitgroup_wait(wg *WaitGroup)` - Wait for all tasks

### Channel Operations
- `create_channel(capacity normie)` - Create buffered channel
- `create_sync_channel()` - Create synchronous channel
- `channel_send(ch *Channel, data normie)` - Send data (blocking)
- `channel_receive(ch *Channel)` - Receive data (blocking)
- `channel_close(ch *Channel)` - Close channel
- `channel_is_closed(ch *Channel)` - Check if closed

### Atomic Operations
- `atomic_i32_new(initial normie)` - Create 32-bit atomic
- `atomic_i64_new(initial thicc)` - Create 64-bit atomic
- `atomic_bool_new(initial lit)` - Create atomic boolean
- `atomic_cas_i32(atomic, expected, desired)` - Compare-and-swap
- `atomic_increment(atomic)` - Atomic increment
- `atomic_decrement(atomic)` - Atomic decrement
- `atomic_load_i32(atomic)` - Load value atomically
- `atomic_store_i32(atomic, value)` - Store value atomically

### Semaphore Operations
- `create_semaphore(initial normie)` - Create counting semaphore
- `semaphore_acquire(sem *Semaphore)` - Acquire permit (blocking)
- `semaphore_release(sem *Semaphore)` - Release permit
- `semaphore_try_acquire(sem *Semaphore)` - Try acquire (non-blocking)

### Barrier Operations
- `create_barrier(count normie)` - Create barrier for N participants
- `barrier_wait(barrier *Barrier)` - Wait at barrier

### Read-Write Mutex Operations
- `create_rwmutex()` - Create read-write mutex
- `rwmutex_rlock(rwmutex *RWMutex)` - Acquire read lock
- `rwmutex_runlock(rwmutex *RWMutex)` - Release read lock
- `rwmutex_lock(rwmutex *RWMutex)` - Acquire write lock (exclusive)
- `rwmutex_unlock(rwmutex *RWMutex)` - Release write lock

### Condition Variable Operations
- `create_condition()` - Create condition variable
- `condition_wait(cond *CondVar, mutex *Mutex)` - Wait on condition
- `condition_signal(cond *CondVar)` - Signal one waiting goroutine
- `condition_broadcast(cond *CondVar)` - Signal all waiting goroutines

### Thread Pool Operations
- `create_thread_pool(workers, queue_size normie)` - Create thread pool
- `thread_pool_submit(pool *ThreadPool, task normie)` - Submit task
- `thread_pool_wait_all(pool *ThreadPool)` - Wait for all tasks
- `thread_pool_shutdown(pool *ThreadPool)` - Shutdown pool

### Once Operations
- `create_once()` - Create one-time execution primitive
- `once_do(once *Once, func_id normie)` - Execute exactly once

## Usage Examples

### Basic Mutex Usage
```cursed
yeet "concurrenz"

sus mutex := concurrenz.create_mutex()
concurrenz.mutex_lock(mutex)
# Critical section
concurrenz.mutex_unlock(mutex)
```

### Atomic Operations
```cursed
yeet "concurrenz"

sus atomic := concurrenz.atomic_i32_new(0)
concurrenz.atomic_increment(atomic)
sus value := concurrenz.atomic_load_i32(atomic)
```

### Buffered Channel Communication
```cursed
yeet "concurrenz"

sus ch := concurrenz.create_channel(5)  # Buffered channel
concurrenz.channel_send(ch, 42)
sus data := concurrenz.channel_receive(ch)
```

### WaitGroup Synchronization
```cursed
yeet "concurrenz"

sus wg := concurrenz.create_waitgroup()
concurrenz.waitgroup_add(wg, 2)
# Start goroutines...
concurrenz.waitgroup_done(wg)
concurrenz.waitgroup_wait(wg)
```

### Semaphore Resource Management
```cursed
yeet "concurrenz"

sus sem := concurrenz.create_semaphore(3)
concurrenz.semaphore_acquire(sem)
# Use resource
concurrenz.semaphore_release(sem)
```

## Implementation Details

- **Pure CURSED** - No external dependencies except atomic_drip
- **Hardware Atomics** - Leverages platform-specific atomic instructions
- **Memory Safe** - Proper error handling and null pointer checks
- **Performance Optimized** - Lock-free algorithms where possible
- **Cross-Platform** - Works on x86_64, ARM64, and WebAssembly

## Integration

The module integrates with:
- `atomic_drip` - Hardware atomic operations with memory ordering
- `memory` - Safe memory allocation and management
- `error_drip` - Comprehensive error handling
- `concurrency.zig` - Runtime goroutine scheduling system

## Testing

Run comprehensive tests with:
```bash
./zig-out/bin/cursed stdlib/concurrenz/test_concurrenz.csd
```

The test suite covers:
- All synchronization primitives
- Atomic operations with different memory orderings
- Complex multi-primitive scenarios
- Stress testing and edge cases
- Error conditions and boundary cases

## Performance Characteristics

- **O(1)** - Mutex operations with atomic CAS
- **O(1)** - Atomic operations (hardware-level)
- **O(n)** - WaitGroup operations (n = waiters)
- **O(1)** - Channel operations (amortized)
- **Lock-free** - Atomic primitives with proper memory ordering

## Memory Ordering Guarantees

All atomic operations provide:
- **Acquire-Release** - Proper synchronization semantics
- **Sequential Consistency** - Default strong ordering
- **Memory Barriers** - Platform-specific fence operations
- **Cross-Platform** - Consistent behavior across architectures

## Thread Safety

All functions in this module are thread-safe and can be used safely from multiple goroutines. The implementations use hardware atomic operations and memory barriers to ensure correct behavior in highly concurrent environments.

## Current Status: ✅ COMPLETE

This implementation provides a production-ready concurrency library with:
- ✅ Full atomic operations suite
- ✅ All major synchronization primitives  
- ✅ Hardware-level performance optimizations
- ✅ Comprehensive test coverage
- ✅ Cross-platform compatibility
- ✅ Memory-safe implementations
