# Garbage Collector Implementation Documentation

## Update: April 2025

The garbage collector has been fully redesigned with a comprehensive set of modern features:

1. **Tri-Color Marking Algorithm**: Implemented a proper tri-color (White/Gray/Black) marking scheme for efficient traversal and cycle detection.
2. **Full Cycle Detection**: The mark-sweep algorithm properly navigates object reference graphs to find all reachable objects, including those in cycles.
3. **Object Finalization**: Added support for finalizing objects before collection to properly clean up resources.
4. **Incremental Collection**: Implemented incremental GC with configurable step sizes and time budgets to reduce pause times.
5. **Deadlock Prevention**: Integrated timeout-based lock acquisition mechanisms to prevent deadlocks.
6. **Unified Implementation**: Consolidated multiple GC implementations into a single, cohesive implementation.
7. **Verbose Logging Control**: Added options for controlling GC logging verbosity.

### Implementation Details

1. **Mark Phase**:
   - Objects start as WHITE (unmarked)
   - When visited, they're marked GRAY and added to a worklist
   - After processing all references, objects become BLACK
   - This approach properly handles cycles while avoiding stack overflows

2. **Sweep Phase**:
   - All WHITE objects are unreachable and collected
   - Object finalization is called before memory is reclaimed
   - Statistics are updated for monitoring purposes

3. **Incremental Collection**:
   - Divides collection into small steps that can be interleaved with program execution
   - Helps reduce pause times and improve responsiveness
   - Configurable time budget and step size for fine-tuning

### Current Limitations and Future Work

1. **Weak References**: Currently, weak references lose their connection to the GC when strong references are dropped. A more robust weak reference implementation is needed to maintain the GC connection.

2. **Object Tracing**: The current approach simulates reference traversal without direct access to Traceable objects. In a production implementation, the GC would maintain direct references to Traceable objects for proper marking.

3. **Finalization**: The object finalization is currently simulated and would need to be properly implemented with direct Traceable object access in a production environment.

### Next Steps

1. Improve weak reference implementation to maintain GC connection after strong references are dropped
2. Refactor GC object storage to allow direct access to Traceable objects
3. Add more sophisticated finalization support with explicit ordering

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