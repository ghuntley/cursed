# vibe_lock - CURSED Synchronization Primitives

A comprehensive synchronization library for CURSED providing thread-safe primitives for concurrent programming.

## Overview

The `vibe_lock` module provides essential synchronization primitives implemented in pure CURSED using atomic operations. This module enables safe concurrent programming with mutexes, read-write locks, semaphores, and once-initialization patterns.

## Features

- **Mutex**: Mutual exclusion locks for protecting shared resources
- **Read-Write Locks**: Allow multiple readers or exclusive writers
- **Semaphores**: Counting semaphores for resource management
- **Once Initialization**: Ensure functions execute exactly once
- **Atomic Operations**: Low-level atomic primitives for lock-free programming

## API Reference

### Mutex Operations

```cursed
// Create a new mutex
sus mu *Mutex = mutex_new()

// Lock the mutex (blocks until acquired)
mutex_lock(mu)

// Unlock the mutex
mutex_unlock(mu)

// Try to lock (non-blocking)
sus success lit = mutex_try_lock(mu)
vibe success {
    // Critical section
    mutex_unlock(mu)
}
```

### Read-Write Lock Operations

```cursed
// Create a new read-write lock
sus rw *RWLock = rwlock_new()

// Read lock (multiple readers allowed)
rwlock_rlock(rw)
// ... read operations ...
rwlock_runlock(rw)

// Write lock (exclusive access)
rwlock_lock(rw)
// ... write operations ...
rwlock_unlock(rw)
```

### Semaphore Operations

```cursed
// Create semaphore with initial count
sus sem *Semaphore = semaphore_new(3)

// Acquire a permit (blocks if none available)
semaphore_acquire(sem)
// ... use resource ...
semaphore_release(sem)
```

### Once Initialization

```cursed
// Create once initializer
sus once_obj *Once = once_new()

// Execute function exactly once
once_do(once_obj, initialization_function)
```

### Atomic Operations

```cursed
// Create atomic variable
sus counter atomic_drip = atomic_drip_new(0)

// Load value atomically
sus value atomic_drip = atomic_drip_load(&counter)

// Store value atomically
atomic_drip_store(&counter, 42)

// Compare and swap
sus old_value atomic_drip = atomic_drip_compare_and_swap(&counter, 0, 1)

// Fetch and add
sus previous atomic_drip = atomic_drip_fetch_and_add(&counter, 5)
```

## Usage Examples

### Producer-Consumer with Mutex

```cursed
yeet "vibe_lock"

sus buffer_mutex *Mutex = mutex_new()
sus buffer [100]normie
sus buffer_size normie = 0

slay producer() normie {
    bestie i := 0; i < 10; i++ {
        mutex_lock(buffer_mutex)
        buffer[buffer_size] = i
        buffer_size = buffer_size + 1
        mutex_unlock(buffer_mutex)
    }
    damn 0
}

slay consumer() normie {
    bestie buffer_size > 0 {
        mutex_lock(buffer_mutex)
        vibe buffer_size > 0 {
            sus item normie = buffer[buffer_size - 1]
            buffer_size = buffer_size - 1
            vibez.spill("Consumed: ", item)
        }
        mutex_unlock(buffer_mutex)
    }
    damn 0
}
```

### Shared Resource Pool with Semaphore

```cursed
yeet "vibe_lock"

sus resource_pool *Semaphore = semaphore_new(5)  // 5 resources available

slay use_resource() normie {
    // Acquire resource
    semaphore_acquire(resource_pool)
    
    // Use resource
    vibez.spill("Using resource...")
    
    // Release resource
    semaphore_release(resource_pool)
    
    damn 0
}
```

### Read-Write Lock for Configuration

```cursed
yeet "vibe_lock"

sus config_lock *RWLock = rwlock_new()
sus config_value normie = 0

slay read_config() normie {
    rwlock_rlock(config_lock)
    sus value normie = config_value
    rwlock_runlock(config_lock)
    damn value
}

slay update_config(new_value normie) normie {
    rwlock_lock(config_lock)
    config_value = new_value
    rwlock_unlock(config_lock)
    damn 0
}
```

### Singleton Pattern with Once

```cursed
yeet "vibe_lock"

sus instance_once *Once = once_new()
sus singleton_instance *MyClass = cringe

slay get_instance() *MyClass {
    once_do(instance_once, initialize_singleton)
    damn singleton_instance
}

slay initialize_singleton() normie {
    singleton_instance = create_my_class()
    damn 0
}
```

## Thread Safety

All operations in this module are thread-safe and designed for concurrent use:

- **Mutex**: Provides mutual exclusion with blocking semantics
- **RWLock**: Allows concurrent readers, exclusive writers
- **Semaphore**: Counting semaphore with configurable capacity
- **Once**: Guarantees single execution across all threads
- **Atomic Operations**: Lock-free operations for high-performance scenarios

## Performance Considerations

- **Spinlocks**: Current implementation uses simple spinlocks for demonstration
- **Atomic Operations**: Built on compare-and-swap primitives
- **Memory Ordering**: Provides sequential consistency
- **Contention**: Performance degrades under high contention

## Implementation Details

### Architecture

The module is implemented in pure CURSED using:
- Atomic integer operations (`atomic_drip`)
- Compare-and-swap primitives
- Memory barriers for synchronization
- Spinlock-based waiting (yield-based)

### Memory Model

- **Sequential Consistency**: All operations appear to execute in program order
- **Atomic Variables**: Provide thread-safe access to shared data
- **Memory Barriers**: Ensure proper ordering of operations
- **Lock-Free Algorithms**: Used where possible for performance

### Error Handling

- **Robust Design**: Handles edge cases and contention gracefully
- **Deadlock Prevention**: Careful lock ordering and try-lock operations
- **Resource Management**: Proper cleanup and state management
- **Validation**: Comprehensive test coverage for correctness

## Testing

The module includes comprehensive tests covering:
- Basic functionality of all primitives
- Concurrent access patterns
- Edge cases and error conditions
- Performance characteristics
- Memory safety

```bash
# Run tests in interpretation mode
cargo run --bin cursed stdlib/vibe_lock/test_vibe_lock.csd

# Run tests in compilation mode
cargo run --bin cursed -- compile stdlib/vibe_lock/test_vibe_lock.csd
./test_vibe_lock
```

## Best Practices

1. **Always Release Locks**: Use proper cleanup patterns
2. **Avoid Deadlocks**: Maintain consistent lock ordering
3. **Minimize Critical Sections**: Keep locked regions small
4. **Use Appropriate Primitives**: Choose the right synchronization tool
5. **Test Thoroughly**: Concurrent code requires extensive testing

## Future Enhancements

- Condition variables for complex synchronization
- Barrier synchronization for coordinated execution
- Lock-free data structures for high performance
- Timeout-based operations for responsiveness
- Priority-aware scheduling for real-time systems

## Compatibility

- **CURSED Language**: Compatible with all CURSED features
- **Execution Modes**: Works in both interpretation and compilation
- **Platforms**: Portable across all supported platforms
- **Memory Management**: Integrates with CURSED's GC system

## License

This module is part of the CURSED standard library and follows the same licensing terms.
