# Memory Management Specification

## Overview

The CURSED memory management system provides a comprehensive, production-ready memory management solution combining advanced garbage collection, heap management, and memory safety guarantees.

## Architecture

### Core Components

1. **Garbage Collector (GC)** - Advanced mark-and-sweep collector with generational optimization
2. **Memory Manager** - Primary interface integrating GC with runtime
3. **Heap Manager** - Direct heap allocation and deallocation
4. **Stack Manager** - Goroutine stack allocation and management
5. **Memory Tracking** - Statistics, debugging, and performance monitoring

### System Integration

```
┌─────────────────┐    ┌─────────────────┐    ┌─────────────────┐
│   Runtime       │    │ Memory Manager  │    │ Garbage         │
│   Allocator     │◄──►│                 │◄──►│ Collector       │
│                 │    │                 │    │                 │
└─────────────────┘    └─────────────────┘    └─────────────────┘
                                │
                                ▼
                       ┌─────────────────┐
                       │   Heap Manager  │
                       │                 │
                       └─────────────────┘
```

## Memory Model

### Heap Structure

The CURSED heap is organized as a multi-generational structure:

```
┌─────────────────────────────────────────────────────────────────┐
│                      CURSED Heap                                │
├─────────────────────────────────────────────────────────────────┤
│  Young Generation (33% of heap)                                │
│  ┌─────────────────┐  ┌─────────────────┐                     │
│  │  Eden Space     │  │  Survivor Space │                     │
│  │                 │  │                 │                     │
│  └─────────────────┘  └─────────────────┘                     │
├─────────────────────────────────────────────────────────────────┤
│  Old Generation (67% of heap)                                  │
│  ┌─────────────────────────────────────────────────────────────┐ │
│  │  Tenured Space                                            │ │
│  │                                                           │ │
│  └─────────────────────────────────────────────────────────────┘ │
└─────────────────────────────────────────────────────────────────┘
```

### Memory Allocation Strategy

1. **New Object Allocation**
   - Small objects (< 32KB): Allocated in Eden space
   - Large objects (≥ 32KB): Directly allocated in old generation
   - Huge objects (≥ 1MB): Special large object space

2. **Object Promotion**
   - Objects surviving young GC: Promoted to survivor space
   - Objects surviving multiple GCs: Promoted to old generation
   - Long-lived objects: Promoted based on allocation patterns

3. **Memory Layout**
   - Object headers: 16 bytes (type info, GC metadata, reference count)
   - Alignment: 8-byte alignment for all objects
   - Padding: Automatic padding for proper alignment

## Garbage Collection

### Collection Algorithms

#### 1. Mark-and-Sweep (Primary Algorithm)

**Mark Phase:**
```rust
fn mark_phase(&mut self) -> Result<(), GcError> {
    // Mark all reachable objects from roots
    for root in self.get_roots() {
        self.mark_object(root)?;
    }
    
    // Mark objects reachable from goroutine stacks
    for stack in self.get_goroutine_stacks() {
        self.mark_stack_objects(stack)?;
    }
    
    // Mark JIT-compiled code references
    self.mark_jit_roots()?;
    
    Ok(())
}
```

**Sweep Phase:**
```rust
fn sweep_phase(&mut self) -> Result<SweepStats, GcError> {
    let mut freed_objects = 0;
    let mut freed_bytes = 0;
    
    for object in self.heap.iter_objects() {
        if !object.is_marked() {
            freed_bytes += object.size();
            freed_objects += 1;
            self.deallocate_object(object)?;
        } else {
            object.clear_mark();
        }
    }
    
    Ok(SweepStats { freed_objects, freed_bytes })
}
```

#### 2. Generational Collection

**Young Generation Collection:**
- Frequency: When Eden space is full
- Algorithm: Copying collector with survivor spaces
- Pause time: Target < 5ms for applications

**Old Generation Collection:**
- Frequency: When old space reaches threshold
- Algorithm: Mark-and-sweep with optional compaction
- Pause time: Target < 50ms for full collection

#### 3. Incremental Collection

**Time-budgeted Collection:**
```rust
pub struct IncrementalCollector {
    time_budget: Duration,
    work_units: VecDeque<GcWorkUnit>,
    current_phase: GcPhase,
}

impl IncrementalCollector {
    pub fn collect_increment(&mut self) -> Result<bool, GcError> {
        let start_time = Instant::now();
        
        while start_time.elapsed() < self.time_budget {
            if let Some(work_unit) = self.work_units.pop_front() {
                self.process_work_unit(work_unit)?;
            } else {
                return Ok(true); // Collection complete
            }
        }
        
        Ok(false) // More work remaining
    }
}
```

### GC Triggering

#### 1. Allocation-based Triggering
- **Threshold Mode**: Trigger when heap usage exceeds threshold
- **Adaptive Mode**: Adjust thresholds based on allocation patterns
- **Pressure Mode**: Trigger based on memory pressure indicators

#### 2. Periodic Triggering
- **Fixed Intervals**: Collect every N milliseconds
- **Adaptive Intervals**: Adjust based on application behavior
- **Idle Collection**: Collect during application idle periods

### Root Set Management

#### Stack Scanning
```rust
pub fn scan_stack(&self, stack: &RuntimeStack) -> Result<Vec<GcRoot>, GcError> {
    let mut roots = Vec::new();
    
    for frame in stack.iter_frames() {
        // Scan local variables
        for local in frame.locals.iter() {
            if self.is_heap_pointer(local) {
                roots.push(GcRoot::new(local, RootType::Local));
            }
        }
        
        // Scan function parameters
        for param in frame.parameters.iter() {
            if self.is_heap_pointer(param) {
                roots.push(GcRoot::new(param, RootType::Parameter));
            }
        }
    }
    
    Ok(roots)
}
```

#### Global Root Registration
```rust
pub fn register_global_root(&mut self, ptr: *mut u8, root_type: RootType) -> Result<RootHandle, GcError> {
    let handle = RootHandle::new();
    self.global_roots.insert(handle, GcRoot::new(ptr, root_type));
    Ok(handle)
}
```

## Memory Safety

### Safety Guarantees

1. **Memory Safety**
   - No dangling pointers through GC management
   - Automatic memory leak prevention
   - Stack overflow detection and prevention
   - Heap corruption detection

2. **Thread Safety**
   - Atomic reference counting for shared objects
   - Thread-safe allocation and deallocation
   - Concurrent GC with proper synchronization
   - Lock-free data structures where possible

3. **Type Safety**
   - Type-tagged heap objects
   - Runtime type checking for casts
   - Prevent use-after-free through GC
   - Buffer overflow protection

### Memory Corruption Detection

```rust
pub struct MemoryCorruptionDetector {
    canaries: HashMap<*mut u8, u64>,
    checksums: HashMap<*mut u8, u64>,
}

impl MemoryCorruptionDetector {
    pub fn check_object(&self, ptr: *mut u8) -> Result<(), MemoryError> {
        // Check canary values
        if let Some(expected_canary) = self.canaries.get(&ptr) {
            let actual_canary = unsafe { *(ptr as *const u64) };
            if actual_canary != *expected_canary {
                return Err(MemoryError::Corruption(
                    format!("Canary mismatch at {:p}", ptr)
                ));
            }
        }
        
        // Check object checksums
        if let Some(expected_checksum) = self.checksums.get(&ptr) {
            let actual_checksum = self.calculate_checksum(ptr);
            if actual_checksum != *expected_checksum {
                return Err(MemoryError::Corruption(
                    format!("Checksum mismatch at {:p}", ptr)
                ));
            }
        }
        
        Ok(())
    }
}
```

## Performance Characteristics

### Allocation Performance

1. **Fast Path Allocation**
   - Bump pointer allocation in Eden space
   - Thread-local allocation buffers (TLABs)
   - Lock-free allocation for small objects

2. **Allocation Latency**
   - Small objects: < 50 nanoseconds
   - Large objects: < 1 microsecond
   - Huge objects: < 10 microseconds

### Collection Performance

1. **Pause Time Targets**
   - Young generation: < 5ms
   - Old generation: < 50ms
   - Concurrent collection: < 1ms pause

2. **Throughput Targets**
   - > 95% application time (< 5% GC overhead)
   - > 99% allocation success rate
   - > 1 million allocations/second

### Memory Utilization

1. **Heap Utilization**
   - Target: 70-80% heap occupancy
   - Fragmentation: < 10% of heap space
   - Overhead: < 5% for GC metadata

2. **Stack Utilization**
   - Default stack size: 2MB per goroutine
   - Minimum stack size: 4KB
   - Maximum stack size: 16MB

## Configuration

### Memory Configuration

```rust
pub struct MemoryConfig {
    /// Garbage collector configuration
    pub gc_config: GcConfig,
    /// Enable memory tracking and debugging
    pub enable_tracking: bool,
    /// Memory limit per goroutine stack
    pub stack_memory_limit: Option<usize>,
    /// Global memory limit
    pub global_memory_limit: Option<usize>,
    /// Enable memory pressure detection
    pub enable_pressure_detection: bool,
    /// Memory pressure threshold (0.0-1.0)
    pub pressure_threshold: f64,
}
```

### GC Configuration

```rust
pub struct GcConfig {
    /// Initial heap size in bytes
    pub initial_heap_size: usize,
    /// Maximum heap size in bytes
    pub max_heap_size: Option<usize>,
    /// Young generation size ratio
    pub young_generation_ratio: f64,
    /// Collection thresholds
    pub young_collection_threshold: usize,
    pub old_collection_threshold: usize,
    /// Incremental collection settings
    pub incremental_collection: bool,
    pub incremental_time_budget: u64,
    /// Concurrent collection settings
    pub concurrent_collection: bool,
    pub concurrent_threads: usize,
    /// Trigger mode
    pub trigger_mode: GcTriggerMode,
    /// Compaction settings
    pub enable_compaction: bool,
    pub compaction_threshold: f64,
}
```

## Error Handling

### Memory Errors

```rust
pub enum MemoryError {
    /// Out of memory
    OutOfMemory { requested: usize, available: usize },
    /// Memory limit exceeded
    LimitExceeded { limit: usize, current: usize },
    /// Invalid allocation size
    InvalidSize(usize),
    /// Memory corruption detected
    Corruption(String),
    /// GC error
    GcError(String),
    /// Stack overflow
    StackOverflow { stack_id: StackId, size: usize },
    /// Channel allocation error
    ChannelError(String),
    /// Initialization failed
    InitializationFailed(String),
}
```

### Recovery Strategies

1. **Out of Memory Recovery**
   - Force garbage collection
   - Reduce heap size temporarily
   - Fail gracefully with error message

2. **Memory Pressure Response**
   - Increase GC frequency
   - Reduce allocation rate
   - Notify application of memory pressure

3. **Corruption Recovery**
   - Isolate corrupted objects
   - Trigger emergency collection
   - Log corruption details for debugging

## Monitoring and Debugging

### Memory Statistics

```rust
pub struct MemoryStats {
    /// Heap statistics
    pub heap_allocations: u64,
    pub heap_deallocations: u64,
    pub heap_usage: usize,
    pub peak_heap_usage: usize,
    
    /// Stack statistics
    pub stack_allocations: u64,
    pub stack_deallocations: u64,
    pub stack_usage: usize,
    pub peak_stack_usage: usize,
    
    /// GC statistics
    pub gc_stats: GcStats,
    
    /// Memory pressure
    pub pressure_level: f64,
    pub last_pressure_check: Option<Instant>,
}
```

### Debugging Support

1. **Memory Leak Detection**
   - Object reference tracking
   - Allocation stack traces
   - Leak pattern analysis

2. **Performance Profiling**
   - Allocation profiling
   - GC pause time analysis
   - Memory usage trends

3. **Diagnostic Tools**
   - Heap dumps
   - Object graph analysis
   - Memory usage reports

## Integration Points

### LLVM Integration

1. **Compiled Code Memory Management**
   - Automatic GC root registration
   - Safe stack scanning
   - Optimized allocation calls

2. **JIT Compilation Memory**
   - Compiled code memory tracking
   - JIT object root management
   - Dynamic code cache management

### Runtime Integration

1. **Goroutine Integration**
   - Stack allocation per goroutine
   - Automatic stack scanning
   - Stack overflow detection

2. **Channel Integration**
   - Channel buffer allocation
   - Message object management
   - Lock-free channel operations

## Testing Strategy

### Unit Tests

```rust
#[test]
fn test_allocation_performance() {
    let mut gc = create_test_gc();
    let start = Instant::now();
    
    for _ in 0..1000000 {
        let obj = gc.allocate(64).unwrap();
        // Use object to prevent optimization
        unsafe { *(obj as *mut u64) = 42; }
    }
    
    let duration = start.elapsed();
    assert!(duration < Duration::from_millis(100));
}

#[test]
fn test_gc_correctness() {
    let mut gc = create_test_gc();
    let mut objects = Vec::new();
    
    // Allocate many objects
    for _ in 0..10000 {
        objects.push(gc.allocate(64).unwrap());
    }
    
    // Clear references
    objects.clear();
    
    // Force collection
    gc.collect().unwrap();
    
    // Verify objects were collected
    assert_eq!(gc.get_stats().live_objects, 0);
}
```

### Integration Tests

```rust
#[test]
fn test_memory_manager_integration() {
    let memory_manager = MemoryManager::new(MemoryConfig::default()).unwrap();
    
    // Test allocation
    let obj = memory_manager.allocate(1024).unwrap();
    assert!(!obj.is_null());
    
    // Test deallocation
    memory_manager.deallocate(obj).unwrap();
    
    // Test statistics
    let stats = memory_manager.get_stats();
    assert_eq!(stats.heap_allocations, 1);
    assert_eq!(stats.heap_deallocations, 1);
}
```

### Performance Tests

```rust
#[test]
fn test_gc_pause_times() {
    let mut gc = create_test_gc();
    let mut pause_times = Vec::new();
    
    for _ in 0..100 {
        // Allocate until GC is triggered
        while !gc.needs_collection() {
            gc.allocate(1024).unwrap();
        }
        
        // Measure GC pause time
        let start = Instant::now();
        gc.collect().unwrap();
        let pause_time = start.elapsed();
        
        pause_times.push(pause_time);
    }
    
    // Verify pause times are within acceptable limits
    let max_pause = pause_times.iter().max().unwrap();
    assert!(max_pause < &Duration::from_millis(50));
}
```

## Compliance

### Memory Safety Standards

1. **RAII Compliance**
   - Automatic resource management
   - Deterministic cleanup
   - Exception safety

2. **Memory Model Compliance**
   - Sequential consistency
   - Atomic operations
   - Memory ordering guarantees

### Performance Standards

1. **Allocation Performance**
   - Sub-microsecond allocation latency
   - High allocation throughput
   - Low memory overhead

2. **Collection Performance**
   - Predictable pause times
   - Low GC overhead
   - Good scaling characteristics

This specification provides a comprehensive foundation for implementing and maintaining the CURSED memory management system with production-grade performance and reliability.
