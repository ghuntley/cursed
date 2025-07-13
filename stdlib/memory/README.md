# Memory Management Module

Pure CURSED implementation of comprehensive memory management functions for self-hosting compiler support.

## Overview

The memory module provides essential memory management capabilities including heap allocation, garbage collection, memory tracking, stack operations, and pool allocation. This module is critical for the CURSED compiler's self-hosting capabilities.

## Core Functions

### Heap Allocation
- `malloc(size normie) thicc` - Allocate memory block of specified size
- `free(ptr thicc) lit` - Free previously allocated memory
- `realloc(ptr thicc, new_size normie) thicc` - Resize allocated memory block

### Garbage Collection
- `gc_collect() normie` - Run garbage collection, returns freed bytes
- `gc_stats() tea` - Get garbage collection statistics
- `gc_pressure() normie` - Calculate memory pressure (0-100%)

### Memory Tracking
- `track_allocation(size normie, tag tea) lit` - Track allocation with descriptive tag
- `memory_report() tea` - Generate comprehensive memory usage report
- `get_memory_usage() thicc` - Get current total memory usage

### Stack Operations
- `get_stack_size() normie` - Get current stack size
- `check_stack_overflow() lit` - Check for potential stack overflow

### Pool Allocation
- `create_pool(block_size normie, block_count normie) thicc` - Create memory pool
- `pool_alloc(pool_id thicc, size normie) thicc` - Allocate from specific pool
- `pool_free(pool_id thicc, ptr thicc) lit` - Free memory back to pool

## Utility Functions

### Memory Operations
- `zero_memory(ptr thicc, size normie) lit` - Zero out memory region
- `copy_memory(dest thicc, src thicc, size normie) lit` - Copy memory between regions
- `compare_memory(ptr1 thicc, ptr2 thicc, size normie) normie` - Compare memory contents

### Memory Alignment
- `align_size(size normie, alignment normie) normie` - Align size to boundary
- `is_aligned(ptr thicc, alignment normie) lit` - Check pointer alignment

### Advanced Management
- `set_memory_limit(limit thicc) lit` - Set maximum memory allocation limit
- `memory_compact() normie` - Compact/defragment memory, returns compacted bytes
- `reset_memory_stats() lit` - Reset all memory tracking statistics

## Usage Examples

### Basic Heap Allocation
```cursed
yeet "memory"

// Allocate 1KB of memory
sus ptr thicc = malloc(1024)
if ptr > 0 {
    vibez.spill("Allocation successful")
    
    // Use the memory...
    
    // Free when done
    free(ptr)
}
```

### Garbage Collection
```cursed
yeet "memory"

// Check memory pressure
sus pressure normie = gc_pressure()
if pressure > 80 {
    // Run garbage collection
    sus freed normie = gc_collect()
    vibez.spill("Freed " + freed.tea + " bytes")
}

// Get statistics
sus stats tea = gc_stats()
vibez.spill(stats)
```

### Memory Pool Management
```cursed
yeet "memory"

// Create pool for 64-byte blocks
sus pool thicc = create_pool(64, 100)

// Allocate from pool
sus ptr thicc = pool_alloc(pool, 64)
if ptr > 0 {
    // Use memory...
    
    // Return to pool
    pool_free(pool, ptr)
}
```

### Memory Tracking and Reporting
```cursed
yeet "memory"

// Track allocations with tags
track_allocation(2048, "parser_buffer")
track_allocation(1024, "lexer_tokens")

// Generate report
sus report tea = memory_report()
vibez.spill(report)
```

### Stack Safety
```cursed
yeet "memory"

// Check stack before recursive operations
if check_stack_overflow() {
    vibez.spill("Warning: Stack overflow risk")
    damn cap
}

sus stack_size normie = get_stack_size()
vibez.spill("Available stack: " + stack_size.tea + " bytes")
```

## Memory Alignment
```cursed
yeet "memory"

// Align allocation size to 8-byte boundary
sus raw_size normie = 100
sus aligned_size normie = align_size(raw_size, 8)
sus ptr thicc = malloc(aligned_size)

// Check alignment
if is_aligned(ptr, 8) {
    vibez.spill("Properly aligned allocation")
}
```

## Integration with Self-Hosting

This module is designed specifically to support the CURSED compiler's self-hosting requirements:

1. **Compiler Memory Management**: Heap allocation for AST nodes, symbol tables, and code generation
2. **Garbage Collection**: Automatic memory cleanup during compilation phases
3. **Memory Tracking**: Monitor memory usage during large compilation tasks
4. **Pool Allocation**: Efficient allocation for frequently created/destroyed objects
5. **Stack Safety**: Prevent stack overflow during deep recursive parsing

## Performance Considerations

- **Pool Allocation**: Use for objects with similar lifetimes and sizes
- **Garbage Collection**: Monitor pressure and run collection strategically
- **Memory Alignment**: Properly aligned memory improves performance
- **Tracking Overhead**: Use allocation tracking judiciously in production

## Testing

Run comprehensive tests:
```bash
cargo run --bin cursed stdlib/memory/test_memory.csd
```

Test both interpretation and compilation modes:
```bash
cargo run --bin cursed stdlib/memory/test_memory.csd
cargo run --bin cursed -- compile stdlib/memory/test_memory.csd
./test_memory
```

## Implementation Notes

- Pure CURSED implementation without FFI dependencies
- Simulation-based approach suitable for compiler self-hosting
- Thread-safe design for concurrent compilation
- Comprehensive error handling and validation
- Compatible with both interpretation and native compilation modes

This module is essential for the CURSED compiler's transition to full self-hosting capability.
