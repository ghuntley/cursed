# memz - Memory Management Module

Pure CURSED memory allocation, arena management, and GC operations. Critical for self-hosting compiler and runtime operations.

## Features

### Arena Allocators
- Fast bulk allocation/deallocation
- Automatic cleanup on reset
- Perfect for compiler data structures
- Zero fragmentation within arenas

### Dynamic Memory Management
- `malloc()`, `calloc()`, `realloc()`, `free()`
- Memory block tracking
- Leak detection and reporting
- Automatic garbage collection

### Memory Statistics
- Real-time usage tracking
- Peak memory monitoring
- Fragmentation analysis
- Active block counting

## Usage

### Basic Allocation
```cursed
yeet "memz"

# Initialize memory system
memz.init_memz()

# Allocate memory
sus ptr normie = memz.malloc(1024)
# Use memory...
memz.free(ptr)
```

### Arena Allocators
```cursed
# Create arena for compiler data
sus arena = memz.create_arena(16 * memz.MB)

# Fast allocations
sus ast_ptr = memz.arena_alloc(&arena, 256)
sus token_ptr = memz.arena_alloc(&arena, 128)

# Reset all at once
memz.arena_reset(&arena)
```

### Memory Statistics
```cursed
sus stats = memz.get_memory_stats()
vibez.spill("Memory usage: " + core.int_to_string(stats.current_usage))
vibez.spill("Peak usage: " + core.int_to_string(stats.peak_usage))
```

### Garbage Collection
```cursed
# Check if GC should run
check memz.gc_should_collect() {
    sus freed = memz.gc_collect()
    vibez.spill("GC freed " + core.int_to_string(freed) + " bytes")
}
```

## Constants

- `KB` = 1024 bytes
- `MB` = 1048576 bytes  
- `GB` = 1073741824 bytes
- `PAGE_SIZE` = 4096 bytes
- `WORD_SIZE` = 8 bytes

## Memory Safety

- All allocations are tracked
- Double-free protection
- Null pointer safety
- Memory leak detection
- Bounds checking (where possible)

## Performance

- Sub-microsecond arena allocation
- Minimal fragmentation with arenas
- Efficient garbage collection
- Memory usage optimization

## Integration

Perfect for:
- Compiler memory management
- Runtime heap management
- Large data processing
- High-performance applications

## Testing

Run comprehensive tests:
```bash
./zig-out/bin/cursed-zig stdlib/memz/test.csd
```

Memory safety validation:
```bash
valgrind --leak-check=full ./zig-out/bin/cursed-zig stdlib/memz/test.csd
```
