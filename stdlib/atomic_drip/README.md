# CURSED Atomic Operations Module (atomic_drip)

## Overview

The `atomic_drip` module provides atomic operations and memory ordering primitives for concurrent programming in CURSED. This module implements thread-safe atomic operations using pure CURSED language features with simulated spinlocks and memory barriers.

## Features

### Atomic Data Types
- **AtomicI32**: 32-bit atomic integer operations
- **AtomicI64**: 64-bit atomic integer operations  
- **AtomicFlag**: Atomic boolean flag operations
- **AtomicCounter**: Thread-safe counter with atomic operations

### Memory Ordering
- **MEMORY_ORDER_RELAXED**: No synchronization or ordering constraints
- **MEMORY_ORDER_ACQUIRE**: Acquire memory ordering
- **MEMORY_ORDER_RELEASE**: Release memory ordering
- **MEMORY_ORDER_ACQ_REL**: Acquire-release memory ordering
- **MEMORY_ORDER_SEQ_CST**: Sequential consistency ordering

## Core Functions

### Atomic I32 Operations
```cursed
# Create atomic i32 variable
sus atomic *AtomicI32 = atomic_i32_new(42)

# Atomic load/store
sus value normie = atomic_load_i32(atomic)
atomic_store_i32(atomic, 100)

# Compare-and-swap
sus success lit = atomic_cas_i32(atomic, 50, 75)

# Atomic swap
sus old normie = atomic_swap_i32(atomic, 60)

# Atomic arithmetic
sus old normie = atomic_add_i32(atomic, 5)
sus old normie = atomic_sub_i32(atomic, 3)
```

### Atomic I64 Operations
```cursed
# Create atomic i64 variable
sus atomic *AtomicI64 = atomic_i64_new(1000000)

# Atomic load/store
sus value thicc = atomic_load_i64(atomic)
atomic_store_i64(atomic, 9876543210)

# Compare-and-swap
sus success lit = atomic_cas_i64(atomic, 5000000, 7500000)

# Atomic arithmetic
sus old thicc = atomic_add_i64(atomic, 500000)
sus old thicc = atomic_sub_i64(atomic, 250000)
```

### Atomic Flag Operations
```cursed
# Create atomic flag
sus flag *AtomicFlag = atomic_flag_new()

# Test-and-set operation
sus was_set lit = atomic_flag_test_and_set(flag)

# Clear flag
atomic_flag_clear(flag)
```

### Atomic Counter Operations
```cursed
# Create atomic counter
sus counter *AtomicCounter = atomic_counter_new(100)

# Counter operations
sus old normie = atomic_counter_increment(counter)
sus old normie = atomic_counter_decrement(counter)
sus value normie = atomic_counter_get(counter)
atomic_counter_set(counter, 500)
atomic_counter_reset(counter)
```

### Memory Barriers
```cursed
# Memory fence/barrier
memory_fence()
```

## API Reference

### AtomicI32 Functions
- `atomic_i32_new(initial_value normie) *AtomicI32` - Create new atomic i32
- `atomic_load_i32(ptr *AtomicI32) normie` - Atomic load operation
- `atomic_store_i32(ptr *AtomicI32, val normie)` - Atomic store operation
- `atomic_cas_i32(ptr *AtomicI32, old normie, new normie) lit` - Compare-and-swap
- `atomic_swap_i32(ptr *AtomicI32, new normie) normie` - Atomic swap
- `atomic_add_i32(ptr *AtomicI32, delta normie) normie` - Atomic add
- `atomic_sub_i32(ptr *AtomicI32, delta normie) normie` - Atomic subtract

### AtomicI64 Functions
- `atomic_i64_new(initial_value thicc) *AtomicI64` - Create new atomic i64
- `atomic_load_i64(ptr *AtomicI64) thicc` - Atomic load operation
- `atomic_store_i64(ptr *AtomicI64, val thicc)` - Atomic store operation
- `atomic_cas_i64(ptr *AtomicI64, old thicc, new thicc) lit` - Compare-and-swap
- `atomic_swap_i64(ptr *AtomicI64, new thicc) thicc` - Atomic swap
- `atomic_add_i64(ptr *AtomicI64, delta thicc) thicc` - Atomic add
- `atomic_sub_i64(ptr *AtomicI64, delta thicc) thicc` - Atomic subtract

### AtomicFlag Functions
- `atomic_flag_new() *AtomicFlag` - Create new atomic flag
- `atomic_flag_test_and_set(ptr *AtomicFlag) lit` - Test-and-set operation
- `atomic_flag_clear(ptr *AtomicFlag)` - Clear flag

### AtomicCounter Functions
- `atomic_counter_new(initial normie) *AtomicCounter` - Create new counter
- `atomic_counter_increment(ptr *AtomicCounter) normie` - Increment counter
- `atomic_counter_decrement(ptr *AtomicCounter) normie` - Decrement counter
- `atomic_counter_get(ptr *AtomicCounter) normie` - Get current value
- `atomic_counter_set(ptr *AtomicCounter, val normie)` - Set value
- `atomic_counter_reset(ptr *AtomicCounter)` - Reset to zero

### Memory Ordering
- `memory_fence()` - Memory barrier/fence operation

## Usage Examples

### Basic Atomic Operations
```cursed
yeet "atomic_drip"

# Create atomic variable
sus atomic *AtomicI32 = atomic_i32_new(0)

# Atomic increment
bestie i := 0; i < 10; i++ {
    atomic_add_i32(atomic, 1)
}

# Get final value
sus final normie = atomic_load_i32(atomic)
vibez.spill("Final value:", final)  # Should be 10
```

### Compare-and-Swap Pattern
```cursed
yeet "atomic_drip"

# Create atomic variable
sus atomic *AtomicI32 = atomic_i32_new(100)

# Try to update value using CAS
sus expected normie = 100
sus desired normie = 200

yo atomic_cas_i32(atomic, expected, desired) {
    vibez.spill("Successfully updated to", desired)
} kinda {
    vibez.spill("Update failed, current value:", atomic_load_i32(atomic))
}
```

### Thread-Safe Counter
```cursed
yeet "atomic_drip"

# Create thread-safe counter
sus counter *AtomicCounter = atomic_counter_new(0)

# Simulate concurrent increments
bestie i := 0; i < 1000; i++ {
    atomic_counter_increment(counter)
}

sus final normie = atomic_counter_get(counter)
vibez.spill("Counter value:", final)  # Should be 1000
```

### Atomic Flag for Synchronization
```cursed
yeet "atomic_drip"

# Create atomic flag
sus flag *AtomicFlag = atomic_flag_new()

# Use flag for synchronization
yo atomic_flag_test_and_set(flag) {
    vibez.spill("Flag was already set")
} kinda {
    vibez.spill("Flag was clear, now set")
    # Critical section
    atomic_flag_clear(flag)
}
```

## Implementation Details

### Atomic Operations Simulation
The module simulates atomic operations using:
- **Spinlocks**: Busy-waiting on boolean flags to simulate mutual exclusion
- **Memory Barriers**: Compiler fences to prevent reordering (simulation)
- **Compare-and-Swap**: Atomic read-modify-write operations
- **Memory Ordering**: Constants for different ordering semantics

### Thread Safety
- All atomic operations are implemented with simulated spinlocks
- Memory barriers prevent instruction reordering
- Operations are lock-free where possible

### Performance Considerations
- Pure CURSED implementation without FFI dependencies
- Simulated spinlocks may cause busy-waiting
- Memory barriers provide ordering guarantees
- Lock-free operations where feasible

## Testing

The module includes comprehensive tests covering:
- Basic atomic operations (load, store, CAS, swap)
- Arithmetic operations (add, subtract)
- Flag operations (test-and-set, clear)
- Counter operations (increment, decrement, get, set, reset)
- Memory ordering and barriers
- Thread safety simulation

Run tests with:
```bash
# Test interpretation mode
cargo run --bin cursed stdlib/atomic_drip/test_atomic_drip.csd

# Test compilation mode
cargo run --bin cursed -- compile stdlib/atomic_drip/test_atomic_drip.csd
./test_atomic_drip
```

## Compatibility

- **Pure CURSED**: No FFI dependencies
- **Interpretation Mode**: Full compatibility
- **Compilation Mode**: Full compatibility
- **Cross-Platform**: Works on all supported platforms

## Best Practices

1. **Use Appropriate Ordering**: Choose memory ordering based on synchronization needs
2. **Minimize Contention**: Avoid excessive atomic operations in hot paths
3. **Prefer Lock-Free**: Use atomic operations instead of locks where possible
4. **Memory Barriers**: Use memory_fence() for explicit ordering requirements
5. **Testing**: Test concurrent scenarios thoroughly

## Future Enhancements

- Hardware-specific atomic instructions
- Wait-free data structures
- Atomic pointer operations
- Advanced memory ordering implementations
- Performance optimizations for specific architectures

## License

This module is part of the CURSED standard library and follows the same license terms.
