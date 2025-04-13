# Garbage Collector Implementation Documentation

## Overview

The garbage collector in the CURSED language is based on a mark-and-sweep algorithm with incremental collection support. It uses a combination of reference counting and tracing to manage memory efficiently.

## Core Components

1. **GarbageCollector** (`src/memory/gc.rs`): The main GC implementation that tracks objects, performs collection cycles, and provides memory statistics.

2. **Gc<T>** (`src/memory/mod.rs`): A smart pointer for garbage-collected objects, similar to Rust's `Arc<T>` but integrated with the GC system.

3. **Weak<T>** (`src/memory/weak.rs`): A weak reference to break reference cycles, similar to Rust's `Weak<T>`.

4. **Traceable** trait: Interface for objects that can be traversed by the GC during mark phase.

5. **Visitor** trait: Interface for objects that perform the traversal during mark phase.

## Circular Reference Fix

The GC implementation was redesigned to fix circular reference issues that could cause deadlocks. The key changes:

1. **Weak References to GC**: `Gc<T>` now stores a `Weak<GarbageCollector>` (Rust's standard weak reference) instead of an `Arc<GarbageCollector>`, breaking potential reference cycles between the GC and the objects it manages.

2. **Root Management**: The GC maintains strong references to root objects, while allowing those objects to have weak references back to the GC.

3. **Reference Lifecycle**: When `Gc<T>` objects are created or cloned, they register with the GC root set. When dropped, they unregister.

4. **Self-Reference Handling**: The GarbageCollector maintains an optional weak reference to itself to facilitate operations that need to create new objects.

## Memory Management Process

1. **Allocation**: Objects are allocated through the GC, which tracks them and returns a `Gc<T>` smart pointer.

2. **Root Tracking**: `Gc<T>` instances register themselves as roots when created or cloned.

3. **Mark Phase**: When collection runs, the GC marks all objects reachable from roots.

4. **Sweep Phase**: Unmarked objects are reclaimed, and their memory is freed.

## Advanced Features

- **Incremental Collection**: To reduce pause times, the GC can perform collection in small steps.
- **Statistics and Debugging**: The GC provides detailed memory usage statistics and debugging information.
- **Type-based Memory Analysis**: Track memory usage by object type.
- **Custom Collection Triggers**: Collection can be triggered manually, on allocation, or based on memory pressure.

## Usage Examples

```rust
// Create a garbage collector
let gc = Arc::new(GarbageCollector::new());

// Allocate an object
let obj = gc.allocate(MyObject::new());

// Use the object
if let Some(inner) = obj.inner() {
    inner.do_something();
}

// Create a weak reference
let weak_ref = obj.downgrade();

// Try to upgrade weak reference
if let Some(strong_ref) = weak_ref.upgrade() {
    // Use the object again
}

// Force garbage collection
gc.collect_garbage();

// Get memory statistics
let stats = gc.stats();
println!("Objects in memory: {}", stats.object_count);
```

## Implementation Notes

- Safety is enforced through careful management of raw pointers and thread-safe reference counting.
- The GC is designed to work with CURSED's concurrency model, including goroutines and channels.
- Proper support for user-defined types is provided through the `Traceable` trait.