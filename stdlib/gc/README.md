# Garbage Collection (gc)

The `gc` module provides automatic memory management and garbage collection for CURSED programs.

## Purpose

This module implements a concurrent, generational garbage collector that automatically manages memory allocation and deallocation, preventing memory leaks and reducing manual memory management overhead.

## Main Functions

- `gc.collect()` - Force garbage collection cycle
- `gc.set_threshold(size)` - Configure GC trigger threshold  
- `gc.get_stats()` - Get garbage collection statistics
- `gc.enable()` - Enable automatic garbage collection
- `gc.disable()` - Disable automatic garbage collection
- `gc.is_enabled()` - Check if GC is currently enabled

## Usage Examples

### Basic Garbage Collection

```cursed
yeet "gc"

fr fr Force garbage collection
gc.collect()

fr fr Check if GC is enabled
if gc.is_enabled() {
    vibez.spill("GC is active")
}
```

### Memory Management with Statistics

```cursed
yeet "gc"

fr fr Get initial stats
sus initial_stats = gc.get_stats()
vibez.spillf("Initial memory: {} bytes", initial_stats.total_memory)

fr fr Allocate objects
sus objects []normie = []
bestie i := 0; i < 10000; i = i + 1 {
    objects.push(i)
}

fr fr Force collection and check stats
gc.collect()
sus final_stats = gc.get_stats()
vibez.spillf("Final memory: {} bytes", final_stats.total_memory)
vibez.spillf("Collections: {}", final_stats.collection_count)
```

### Configuration and Tuning

```cursed
yeet "gc"

fr fr Configure GC threshold (in bytes)
gc.set_threshold(1024 * 1024)  # 1MB threshold

fr fr Temporarily disable GC for performance-critical section
gc.disable()
fr fr Performance-critical code here
gc.enable()
```

## Compilation Examples

### Interpretation Mode
```bash
echo 'yeet "gc"
gc.collect()
vibez.spill("GC collection complete")' > gc_test.💀

./cursed-unified gc_test.💀
```

### Compilation Mode
```bash
./cursed-unified --compile gc_test.💀
./gc_test
```

## Implementation Notes

- Uses concurrent mark-and-sweep algorithm
- Generational collection for improved performance
- Thread-safe for concurrent CURSED programs
- Automatically triggered based on memory thresholds
- Integrates with CURSED's ownership system

## Dependencies

- Part of core runtime system
- No external dependencies
- Works with `memory` module for allocation tracking

## Performance Considerations

- GC pauses are minimized through concurrent collection
- Tune threshold based on application memory patterns
- Monitor stats to optimize collection frequency
- Consider disabling GC for real-time critical sections
