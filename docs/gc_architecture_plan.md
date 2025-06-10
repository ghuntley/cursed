# Complete Garbage Collection Architecture Plan

## Executive Summary

This document outlines the comprehensive implementation plan for replacing the current placeholder garbage collection (GC) system in the CURSED language with a production-ready mark-and-sweep garbage collector. The implementation will build upon the existing goroutine-aware GC infrastructure while providing the missing core GC functionality.

## Current State Analysis

### Existing Infrastructure ✅
- **Goroutine-aware GC**: Fully implemented in tests with comprehensive integration
- **Memory safety framework**: Traceable trait and Visitor pattern established
- **Testing infrastructure**: Comprehensive test suites for goroutine integration
- **Profiling integration**: Memory profiling capabilities exist
- **Type system integration**: GC roots tracking in place

### Missing Core Components ❌
- **Heap management**: No actual memory allocation/deallocation
- **Mark-and-sweep algorithm**: Basic GC collection logic missing
- **Object storage**: No heap data structures
- **Memory reclamation**: No actual memory freeing
- **Root set management**: Global root tracking incomplete
- **LLVM integration**: Runtime allocation calls missing

## Architecture Design

### 1. Core GC Components

#### 1.1 Heap Management System
```rust
pub struct Heap {
    // Object storage with metadata
    objects: BTreeMap<ObjectId, HeapObject>,
    
    // Memory blocks for efficient allocation
    blocks: Vec<MemoryBlock>,
    
    // Free list for recycled memory
    free_list: Vec<FreeBlock>,
    
    // Heap statistics
    stats: HeapStatistics,
}

pub struct HeapObject {
    // Object metadata
    header: ObjectHeader,
    
    // Raw object data
    data: Box<dyn Any>,
    
    // GC state information
    mark: MarkState,
    
    // Reference information
    references: Vec<ObjectId>,
}
```

#### 1.2 Mark-and-Sweep Algorithm
```rust
pub struct MarkAndSweepCollector {
    // Core heap management
    heap: Heap,
    
    // Root set tracking
    global_roots: HashSet<ObjectId>,
    
    // Collection statistics
    stats: CollectionStats,
    
    // Configuration
    config: GcConfig,
}
```

#### 1.3 Integration with Existing Goroutine GC
```rust
impl GarbageCollector {
    pub fn should_use_goroutine_aware_collection(&self) -> bool {
        // Detect active goroutines
    }
    
    pub fn collect_garbage_with_goroutine_awareness(&mut self) -> CollectionResult {
        if self.should_use_goroutine_aware_collection() {
            // Use existing goroutine-aware collection
            self.goroutine_gc.collect_garbage_goroutine_aware()
        } else {
            // Use new core collection
            self.core_collector.collect()
        }
    }
}
```

### 2. Memory Management Strategy

#### 2.1 Object Identification System
- **ObjectId**: Unique 64-bit identifier for each heap object
- **Reference tracking**: Direct reference counting for immediate cleanup
- **Weak references**: Support for cycles via weak reference patterns

#### 2.2 Allocation Strategy
- **Block-based allocation**: Large memory blocks subdivided for objects
- **Size classes**: Different block sizes for different object categories
- **Free list management**: Efficient recycling of deallocated memory

#### 2.3 Collection Triggers
- **Memory pressure**: Collect when heap usage exceeds thresholds
- **Allocation pressure**: Collect after N allocations
- **Explicit requests**: Manual collection calls
- **Time-based**: Periodic collection for long-running programs

### 3. Integration Points

#### 3.1 LLVM Runtime Integration
```rust
// FFI functions for LLVM-generated code
#[no_mangle]
pub extern "C" fn cursed_gc_allocate(size: usize, type_id: u32) -> *mut c_void;

#[no_mangle]
pub extern "C" fn cursed_gc_add_root(ptr: *mut c_void);

#[no_mangle]
pub extern "C" fn cursed_gc_remove_root(ptr: *mut c_void);

#[no_mangle]
pub extern "C" fn cursed_gc_collect();
```

#### 3.2 Goroutine System Integration
- **Existing infrastructure**: Build on implemented goroutine-aware GC
- **Stack scanning**: Leverage existing conservative stack scanning
- **Safe points**: Use existing safe point coordination
- **Root management**: Extend existing per-goroutine root tracking

### 4. Performance Characteristics

#### 4.1 Collection Performance
- **Target pause time**: < 10ms for most collections
- **Throughput**: > 95% application time (< 5% GC overhead)
- **Scalability**: Linear performance with heap size up to 1GB

#### 4.2 Memory Efficiency
- **Overhead**: < 10% memory overhead for GC metadata
- **Fragmentation**: < 20% memory fragmentation after collection
- **Reclamation**: > 90% garbage memory reclaimed per collection

## Implementation Plan

### Phase 1: Core Heap Management (Week 1)
1. **Heap data structures** (`src/memory/heap.rs`)
   - MemoryBlock and FreeBlock implementation
   - ObjectHeader and HeapObject structures
   - Basic allocation and deallocation

2. **Object identification** (`src/memory/object_id.rs`)
   - ObjectId generation and management
   - Object registry with lookup capabilities
   - Reference tracking infrastructure

3. **Basic statistics** (`src/memory/gc_stats.rs`)
   - Collection metrics tracking
   - Memory usage monitoring
   - Performance measurement infrastructure

### Phase 2: Mark-and-Sweep Implementation (Week 2)
1. **Marking phase** (`src/memory/mark.rs`)
   - Root set enumeration
   - Graph traversal with cycle detection
   - Mark bit management

2. **Sweep phase** (`src/memory/sweep.rs`)
   - Unmarked object identification
   - Memory reclamation
   - Free list management

3. **Collection coordination** (`src/memory/collector.rs`)
   - Collection trigger logic
   - Phase coordination
   - Error handling and recovery

### Phase 3: Integration and Optimization (Week 3)
1. **Goroutine integration** (enhance existing `goroutine_gc.rs`)
   - Seamless fallback to core GC
   - Shared statistics and monitoring
   - Unified configuration system

2. **LLVM runtime** (`src/codegen/llvm/gc_runtime.rs`)
   - Runtime allocation functions
   - Root management helpers
   - Collection trigger integration

3. **Performance optimization**
   - Memory layout optimization
   - Collection algorithm tuning
   - Benchmark-driven improvements

### Phase 4: Testing and Validation (Week 4)
1. **Core GC tests** (`tests/gc_core_test.rs`)
   - Basic allocation/deallocation
   - Mark-and-sweep correctness
   - Memory reclamation verification

2. **Integration tests** (enhance existing test suites)
   - Goroutine GC fallback behavior
   - LLVM runtime integration
   - Performance regression testing

3. **Stress testing** (`tests/gc_stress_test.rs`)
   - Large heap stress testing
   - Memory pressure scenarios
   - Long-running collection stability

## Configuration System

### GC Configuration Options
```rust
pub struct GcConfig {
    // Collection triggers
    pub heap_size_threshold: usize,      // Collect when heap > N bytes
    pub allocation_threshold: usize,     // Collect after N allocations
    pub time_threshold: Duration,        // Collect after N seconds
    
    // Performance tuning
    pub block_size: usize,               // Memory block size
    pub free_list_max_size: usize,       // Maximum free list entries
    pub enable_defragmentation: bool,    // Enable memory compaction
    
    // Integration options
    pub goroutine_aware: bool,           // Use goroutine-aware collection
    pub profiling_enabled: bool,         // Enable performance profiling
    pub debug_mode: bool,                // Enable debug logging
}
```

## Testing Strategy

### 1. Unit Tests
- **Heap operations**: Allocation, deallocation, lookup
- **Collection phases**: Mark phase, sweep phase independently
- **Object management**: Reference tracking, lifecycle

### 2. Integration Tests
- **End-to-end collection**: Full mark-and-sweep cycles
- **Goroutine integration**: Seamless fallback behavior
- **LLVM integration**: Runtime allocation and collection

### 3. Performance Tests
- **Collection latency**: Pause time measurement
- **Memory efficiency**: Overhead and fragmentation analysis
- **Scalability**: Performance with varying heap sizes

### 4. Stress Tests
- **Memory pressure**: Large allocation patterns
- **Long-running**: Extended execution with periodic collection
- **Concurrent access**: Thread safety verification

## Risk Mitigation

### 1. Memory Safety Risks
- **Double-free prevention**: Object lifecycle tracking
- **Use-after-free prevention**: Reference validation
- **Memory leak prevention**: Comprehensive marking

### 2. Performance Risks
- **Collection pause time**: Incremental collection support
- **Memory overhead**: Efficient metadata representation
- **Fragmentation**: Periodic defragmentation

### 3. Integration Risks
- **Goroutine compatibility**: Comprehensive fallback testing
- **LLVM integration**: Robust FFI interface design
- **Backward compatibility**: Gradual migration path

## Success Metrics

### 1. Functional Metrics
- **Correctness**: No memory safety violations in test suite
- **Completeness**: 100% of placeholder functionality replaced
- **Integration**: Seamless operation with existing goroutine GC

### 2. Performance Metrics
- **Collection latency**: < 10ms pause time for 95% of collections
- **Memory efficiency**: < 10% overhead, < 20% fragmentation
- **Throughput**: < 5% total execution time spent in GC

### 3. Quality Metrics
- **Test coverage**: > 95% code coverage for GC components
- **Documentation**: Complete API documentation and architecture guide
- **Maintainability**: Clear separation of concerns and modular design

## Future Enhancements

### 1. Advanced Collection Algorithms
- **Generational GC**: Young/old generation partitioning
- **Incremental collection**: Interleaved mark-and-sweep
- **Concurrent collection**: Background collection threads

### 2. Memory Optimization
- **Compacting collection**: Memory defragmentation
- **Compressed pointers**: Reduced pointer overhead
- **Object pooling**: Recycling for common object types

### 3. Monitoring and Debugging
- **Real-time monitoring**: Live heap visualization
- **Collection profiling**: Detailed collection analytics
- **Memory debugging**: Leak detection and analysis tools

This architecture provides a solid foundation for implementing a production-ready garbage collector that builds upon the existing CURSED infrastructure while providing the missing core functionality needed for proper memory management.
