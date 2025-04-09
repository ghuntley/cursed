# Garbage Collector Implementation for CURSED

## Overview

The CURSED language uses a mark-and-sweep garbage collector for automatic memory management. This document describes the implementation of the garbage collector and how it integrates with the rest of the CURSED language.

## Key Components

### Memory Manager

The `MemoryManager` is the main entry point for memory management operations. It uses the garbage collector internally and provides a simplified API for allocation and garbage collection.

```rust
pub struct MemoryManager {
    gc: Arc<GarbageCollector>,
}
```

### Garbage Collector

The `GarbageCollector` implements a mark-and-sweep garbage collection algorithm. It tracks all allocated objects and periodically collects objects that are no longer reachable.

```rust
pub struct GarbageCollector {
    inner: Arc<RwLock<GcState>>,
    allocation_threshold: usize,
}
```

### Garbage Collector State

The `GcState` holds the internal state of the garbage collector, including a map of all allocated objects, set of root objects, and statistics about memory usage.

```rust
struct GcState {
    allocator: BumpAllocator,
    objects: HashMap<usize, GcObject>,
    roots: HashSet<usize>,
    total_size: usize,
    collection_count: usize,
}
```

### Traceable Objects

Objects that can be managed by the garbage collector must implement the `Traceable` trait, which allows the GC to trace references between objects.

```rust
pub trait Traceable: 'static {
    fn trace(&self, visitor: &mut dyn Visitor);
    fn size(&self) -> usize;
    fn tag(&self) -> Tag;
}
```

### Garbage Collected References

The `Gc<T>` type represents a garbage-collected reference to an object. It wraps a raw pointer to the object and provides safe access to the object.

```rust
pub struct Gc<T: Traceable + Clone> {
    ptr: NonNull<T>,
    collector: Arc<GarbageCollector>,
    _marker: PhantomData<T>,
}
```

## Garbage Collection Algorithm

The garbage collector uses a mark-and-sweep algorithm:

1. **Mark Phase**: Starting from root objects, the GC traverses the object graph and marks all reachable objects. This is done in multiple steps:
   - First, all objects are marked as "White" (unreachable)
   - Root objects are marked as "Gray" (reachable but not yet processed)
   - Gray objects are processed one by one, marking their references as Gray, and then marking themselves as "Black" (fully processed)
   - This process continues until no Gray objects remain

2. **Sweep Phase**: After the mark phase, any objects still marked as White are unreachable and can be safely deallocated. The GC scans the object map and removes these objects.

## Usage

To allocate an object with garbage collection:

```rust
let mm = MemoryManager::new();
let obj = mm.allocate(MyObject { ... });
```

To force garbage collection:

```rust
mm.collect_garbage();
```

To get statistics about memory usage:

```rust
let stats = mm.stats();
println!("Objects: {}, Total size: {} bytes", stats.object_count, stats.total_size);
```

## Implementing Traceable for Custom Types

To make a custom type garbage collectable, implement the `Traceable` and `Trace` traits:

```rust
#[derive(Clone)]
struct MyObject {
    value: i64,
    next: Option<Gc<MyObject>>,
}

impl Traceable for MyObject {
    fn trace(&self, visitor: &mut dyn Visitor) {
        if let Some(next) = &self.next {
            visitor.visit_ptr(next.ptr.as_ptr() as usize, Tag::Object);
        }
    }
    
    fn size(&self) -> usize {
        std::mem::size_of::<MyObject>()
    }
    
    fn tag(&self) -> Tag {
        Tag::Object
    }
}

impl Trace for MyObject {}
```

## Limitations

- This implementation does not handle cyclic references optimally - cycles will be collected only if all objects in the cycle become unreachable
- The GC is non-generational and non-incremental, which may cause longer pause times
- The current implementation uses unsafe Rust code for memory management, which requires careful handling

## Advanced Features

### Incremental Garbage Collection

The garbage collector supports incremental collection to reduce pause times:

```rust
// Create a memory manager with incremental GC
let mm = MemoryManager::with_incremental_gc(
    1024 * 1024 * 10, // 10MB threshold
    100,              // Process 100 objects per step
    10                // Spend up to 10% of time in GC
);
```

Incremental collection works by dividing the mark and sweep phases into smaller steps that can be executed incrementally, reducing the maximum pause time at the cost of slightly higher total GC overhead.

### Weak References

Weak references allow referencing objects without preventing them from being collected:

```rust
// Create a normal (strong) reference
let obj = mm.allocate(MyObject { ... });

// Create a weak reference
let weak_ref = mm.downgrade(&obj);

// Later, try to upgrade back to a strong reference
if let Some(strong_ref) = weak_ref.upgrade() {
    // Object still exists, use strong_ref
} else {
    // Object has been collected
}
```

Weak references are particularly useful for breaking reference cycles that would otherwise prevent objects from being collected.

### Memory Layout Optimizations

The garbage collector includes several optimizations for memory layout:

1. **Object Regions**: Objects allocated close in time are grouped into regions for better cache locality
2. **Generational Collection**: Objects that survive collections are promoted to higher generations, allowing the GC to focus on younger objects that are more likely to be garbage
3. **Cache-Friendly Object Metadata**: The GC object metadata is laid out in memory to optimize cache access patterns

### Detailed Memory Statistics and Debugging

The garbage collector provides detailed statistics and debugging information:

```rust
// Get basic statistics
let stats = mm.stats();
println!("Objects: {}, Total size: {} bytes", stats.object_count, stats.total_size);

// Get detailed debugging information
let debug_info = mm.debug_info();

// Display memory usage by type
for type_usage in &debug_info.type_usage {
    println!("Type: {}, Bytes: {}, Objects: {}", 
        type_usage.type_name, type_usage.bytes, type_usage.object_count);
}

// Display objects by generation
for (i, &count) in debug_info.generations.iter().enumerate() {
    println!("Generation {}: {} objects", i, count);
}

// Display debug logs
for log in &debug_info.debug_logs {
    println!("Log: {}", log);
}
```

## Future Improvements

- Further optimize incremental collection to reduce pause times even more
- Implement concurrent garbage collection using background threads
- Add specialized handling for large objects
- Develop tools for visualizing memory usage patterns