# Memory Management Module

Pure CURSED implementation of comprehensive memory management system for compiler self-hosting.

## Overview

The memory module provides essential memory management capabilities needed for the CURSED compiler to manage memory safely and efficiently during self-hosting. This includes allocation, deallocation, garbage collection, memory pools, and safety checking - all implemented in pure CURSED.

## Key Components

### Core Memory Operations

#### Basic Allocation
```cursed
# Allocate and deallocate memory
sus pointer normie = memory_allocate(1024)
sus success lit = memory_deallocate(pointer)
sus new_pointer normie = memory_reallocate(pointer, 2048)
```

#### Memory Manipulation
```cursed
# Copy, zero, and compare memory
memory_copy(dest, src, size)
memory_zero(buffer, size)
sus result normie = memory_compare(ptr1, ptr2, size)
```

### Garbage Collection System

#### GCState - Garbage Collector
- **Automatic Collection**: Tracks allocations and performs collection
- **Statistics**: Monitors total allocated memory and collection frequency
- **Configurable**: Can be enabled/disabled for performance tuning

```cursed
sus gc GCState = memory_get_gc()
sus pointer normie = gc_allocate(gc, 512)
sus freed_bytes normie = gc_collect(gc)
sus total normie = gc_get_total_allocated(gc)
```

### Memory Allocator

#### MemoryAllocator - Low-level Allocation
- **System Interface**: Bridges to system malloc/free
- **Tracking**: Monitors all allocations and deallocations
- **Reallocation**: Efficient memory resizing

```cursed
sus allocator MemoryAllocator = allocator_create()
sus ptr normie = allocator_malloc(256)
sus new_ptr normie = allocator_realloc(ptr, 512)
allocator_free(new_ptr)
```

### Memory Pools

#### MemoryPool - Efficient Block Allocation
- **Fixed-size Blocks**: Pre-allocated blocks for frequent allocations
- **Fast Allocation**: O(1) allocation and deallocation
- **Pool Management**: Tracks available and used blocks

```cursed
sus pool MemoryPool = memory_pool_create(64, 100)  # 100 blocks of 64 bytes
sus block normie = memory_pool_acquire(pool)
memory_pool_release(pool, block)
lowkey memory_pool_is_empty(pool) {
    vibez.spill("Pool exhausted")
}
```

### Memory Safety

#### MemorySafety - Runtime Safety Checks
- **Bounds Checking**: Validate memory access within allocated bounds
- **Null Pointer Detection**: Prevent null pointer dereferences
- **Double-free Protection**: Detect attempts to free memory twice

```cursed
sus safety MemorySafety = memory_safety_create()
assert_true(memory_check_bounds(safety, pointer, access_size))
assert_true(memory_check_null(safety, pointer))
assert_true(memory_check_double_free(safety, pointer))
```

### Memory Block Metadata

#### MemoryBlock - Allocation Information
- **Size Tracking**: Track allocated block sizes
- **Type Information**: Categorize allocations by purpose
- **Validity Checking**: Ensure blocks are still valid

```cursed
sus block MemoryBlock = memory_block_create(1024, "ast_node")
sus size normie = memory_block_get_size(block)
sus type tea = memory_block_get_type(block)
sus valid lit = memory_block_is_valid(block)
```

## Compiler-Specific Utilities

### AST Memory Management
```cursed
# Allocate memory for specific compiler data structures
sus ast_ptr normie = memory_allocate_ast_node("expression")
sus symbol_table normie = memory_allocate_symbol_table(100)
sus string_buf normie = memory_allocate_string_buffer(256)
```

### Memory Statistics and Debugging
```cursed
# Monitor memory usage during compilation
sus stats tea = memory_get_stats()
memory_print_stats()
sus freed normie = memory_force_gc()
```

## Advanced Features

### Efficient String Operations
```cursed
# Memory-efficient string handling
sus original normie = memory_allocate_string_buffer(50)
sus duplicate normie = memory_string_duplicate(original)
sus length normie = string_pointer_length(original)
```

### Memory Pool Strategies
```cursed
# Different pool sizes for different allocation patterns
sus small_pool MemoryPool = memory_pool_create(32, 1000)   # Small objects
sus medium_pool MemoryPool = memory_pool_create(128, 500)  # Medium objects  
sus large_pool MemoryPool = memory_pool_create(512, 100)   # Large objects

# Use appropriate pool based on allocation size
slay allocate_by_size(size normie) normie {
    lowkey size <= 32 {
        damn memory_pool_acquire(small_pool)
    } elseif size <= 128 {
        damn memory_pool_acquire(medium_pool)
    } elseif size <= 512 {
        damn memory_pool_acquire(large_pool)
    } else {
        damn memory_allocate(size)  # Use general allocator
    }
}
```

### Garbage Collection Tuning
```cursed
# Configure GC for compiler workloads
slay setup_compiler_gc() GCState {
    sus gc GCState = memory_get_gc()
    gc.enabled = based
    
    # Tune for compilation workload
    sus initial_heap normie = memory_allocate(1024 * 1024)  # 1MB initial heap
    
    damn gc
}

# Force collection at strategic points
slay compile_with_gc_management(source tea) tea {
    sus initial_memory normie = gc_get_total_allocated(memory_get_gc())
    
    # Perform compilation
    sus result tea = compile_source(source)
    
    # Clean up after compilation
    memory_force_gc()
    
    sus final_memory normie = gc_get_total_allocated(memory_get_gc())
    sus leaked normie = final_memory - initial_memory
    
    lowkey leaked > 0 {
        vibez.spill("Memory leak detected: " + integer_to_string(leaked) + " bytes")
    }
    
    damn result
}
```

## Error Handling and Recovery

### Memory Allocation Failures
```cursed
slay safe_allocate(size normie) normie {
    sus pointer normie = memory_allocate(size)
    lowkey pointer == 0 {
        # Try garbage collection and retry
        memory_force_gc()
        pointer = memory_allocate(size)
        
        lowkey pointer == 0 {
            # Still failed - use emergency pool or reduce size
            pointer = try_emergency_allocation(size)
        }
    }
    damn pointer
}

slay try_emergency_allocation(size normie) normie {
    # Try smaller allocation
    sus reduced_size normie = size / 2
    lowkey reduced_size > 0 {
        damn memory_allocate(reduced_size)
    }
    damn 0
}
```

### Memory Leak Detection
```cursed
slay detect_memory_leaks() lit {
    sus stats tea = memory_get_stats()
    sus gc GCState = memory_get_gc()
    
    # Force full collection
    memory_force_gc()
    memory_force_gc()  # Second pass
    
    sus remaining normie = gc_get_total_allocated(gc)
    lowkey remaining > expected_baseline() {
        vibez.spill("Potential memory leak: " + integer_to_string(remaining) + " bytes remain")
        damn cap
    }
    
    damn based
}

slay expected_baseline() normie {
    # Expected memory usage for compiler infrastructure
    damn 4096  # 4KB baseline
}
```

## Testing

Comprehensive test suite validates all memory operations:

```bash
cargo run --bin cursed stdlib/memory/test_memory.💀
```

Tests cover:
- Basic allocation/deallocation cycles
- Memory operations (copy, zero, compare)
- Garbage collection functionality
- Memory pool management
- Safety checking systems
- Compiler-specific utilities
- Error handling and edge cases
- Memory leak detection
- Performance characteristics

## Self-Hosting Significance

Critical for compiler self-hosting:

1. **AST Management**: Efficient allocation for syntax trees
2. **Symbol Tables**: Memory management for compiler state
3. **String Handling**: Efficient string allocation and manipulation
4. **Garbage Collection**: Automatic cleanup during compilation
5. **Memory Safety**: Prevent crashes during self-compilation
6. **Performance**: Optimized allocation patterns for compilation workloads

## Performance Considerations

### Allocation Patterns
- **Small Allocations**: Use memory pools for AST nodes, tokens
- **Large Allocations**: Direct allocation for source buffers, symbol tables
- **Temporary Allocations**: Stack-like allocation for parser state
- **Long-lived Allocations**: Managed allocation for compiler infrastructure

### Memory Pool Sizing
```cursed
# Compiler-optimized pool configuration
sus token_pool MemoryPool = memory_pool_create(16, 10000)    # Tokens
sus ast_pool MemoryPool = memory_pool_create(64, 5000)      # AST nodes
sus symbol_pool MemoryPool = memory_pool_create(128, 1000)  # Symbol entries
sus string_pool MemoryPool = memory_pool_create(256, 2000)  # String literals
```

### Garbage Collection Strategy
- **Generational Collection**: Young generation for temporary compilation data
- **Incremental Collection**: Avoid long pauses during compilation
- **Collection Triggers**: Trigger collection at phase boundaries
- **Memory Pressure**: Aggressive collection when memory is low

## Integration Points

Works with other stdlib modules:
- **compiler_core**: Provides memory for AST, symbol tables, compilation state
- **runtime_core**: Manages runtime value allocation
- **error_core**: Allocates memory for error context and messages
- **fs**: Buffers for file I/O operations
- **process**: Memory for inter-process communication

This comprehensive memory management system ensures the CURSED compiler can efficiently and safely manage memory during self-hosting, providing both performance and reliability for production compilation workloads.
