# Real Garbage Collection Implementation Plan for CURSED

## Overview
This document outlines the implementation plan for moving from the current placeholder GC system to a production-ready garbage collector suitable for CURSED's Go-like concurrent programming model.

## Phase 1: Real Goroutine Foundation (2-3 weeks)

### 1.1 Core Goroutine Runtime System
**File: `src/runtime/real_goroutine.rs`**

```rust
pub struct RealGoroutineScheduler {
    /// Work-stealing scheduler with per-processor queues
    processors: Vec<Processor>,
    /// Global run queue for load balancing
    global_queue: Arc<Mutex<VecDeque<GoroutineHandle>>>,
    /// GC-aware safe point coordination
    gc_coordinator: Arc<GcSafePointCoordinator>,
    /// Stack allocator for goroutine stacks
    stack_allocator: StackAllocator,
}

pub struct Goroutine {
    /// Unique goroutine ID
    id: GoroutineId,
    /// Current execution state
    state: GoroutineState,
    /// Stack memory region
    stack: GoroutineStack,
    /// Local GC roots (stack references)
    local_roots: HashSet<ObjectPtr>,
    /// Last safe point timestamp
    last_safe_point: Instant,
}
```

**Key Features:**
- M:N threading model (M goroutines on N OS threads)
- Stack allocation with guard pages
- Safe point coordination for GC
- Work-stealing scheduler for load balancing

### 1.2 Stack Management System
**File: `src/runtime/goroutine_stack.rs`**

```rust
pub struct GoroutineStack {
    /// Stack memory region (typically 2KB-1MB)
    memory: NonNull<u8>,
    /// Stack size in bytes
    size: usize,
    /// Current stack pointer
    sp: *mut u8,
    /// Stack bounds for overflow detection
    guard_page: *mut u8,
    /// GC stack scanning metadata
    scan_info: StackScanInfo,
}

pub struct StackScanInfo {
    /// Frame pointers for precise scanning
    frames: Vec<StackFrame>,
    /// Conservative scan regions
    conservative_regions: Vec<MemoryRegion>,
    /// Last GC scan timestamp
    last_scan: Instant,
}
```

### 1.3 Safe Point Coordination
**File: `src/runtime/gc_safe_points.rs`**

```rust
pub struct GcSafePointCoordinator {
    /// Active goroutines registered for GC coordination
    active_goroutines: RwLock<HashMap<GoroutineId, GoroutineGcState>>,
    /// Global GC state
    gc_state: AtomicU8, // None, Requesting, Active, Finishing
    /// Safe point barrier for synchronization
    safe_point_barrier: Barrier,
}

impl GcSafePointCoordinator {
    /// Register goroutine for GC coordination
    pub fn register_goroutine(&self, id: GoroutineId, stack_info: StackInfo);
    
    /// Goroutine checks in at safe point
    pub fn check_safe_point(&self, goroutine_id: GoroutineId) -> SafePointAction;
    
    /// Initiate GC safe point across all goroutines
    pub fn initiate_gc_safe_point(&self) -> Result<Vec<GoroutineSnapshot>, String>;
}
```

## Phase 2: Enhanced Object Layout & Metadata (1-2 weeks)

### 2.1 Object Headers with GC Metadata
**File: `src/memory/object_layout.rs`**

```rust
/// Standard object header for all heap-allocated objects
#[repr(C)]
pub struct ObjectHeader {
    /// Object type ID for runtime type information
    type_id: u32,
    /// Object size including header
    size: u32,
    /// GC marking bits (2 bits: white=00, gray=01, black=10)
    gc_mark: AtomicU8,
    /// Reference count for cycle detection
    ref_count: AtomicU32,
    /// Forward pointer for copying collection
    forward_ptr: AtomicPtr<ObjectHeader>,
}

/// Object layout with header + data
#[repr(C)]
pub struct GcObject<T> {
    header: ObjectHeader,
    data: T,
}

impl<T> GcObject<T> {
    /// Get object data from header pointer
    pub unsafe fn from_header(header: *mut ObjectHeader) -> *mut T {
        (header.add(1)) as *mut T
    }
    
    /// Get header from object data pointer
    pub unsafe fn to_header(data: *mut T) -> *mut ObjectHeader {
        (data as *mut ObjectHeader).sub(1)
    }
}
```

### 2.2 Precise Object Scanning
**File: `src/memory/object_scanner.rs`**

```rust
pub struct ObjectScanner {
    /// Type information for precise scanning
    type_registry: Arc<TypeRegistry>,
    /// GC visitor for object traversal
    visitor: Box<dyn GcVisitor>,
}

pub trait GcVisitor {
    /// Visit a pointer field in an object
    fn visit_pointer(&mut self, ptr: *mut ObjectHeader);
    
    /// Visit a range of potential pointers (conservative)
    fn visit_range(&mut self, start: *const u8, end: *const u8);
}

/// Type information for precise object scanning
pub struct TypeInfo {
    /// Type ID
    id: u32,
    /// Object size
    size: usize,
    /// Pointer field offsets for precise scanning
    pointer_offsets: Vec<usize>,
    /// Finalizer function (if any)
    finalizer: Option<fn(*mut u8)>,
}
```

## Phase 3: Tri-Color Concurrent Collector (2-3 weeks)

### 3.1 Concurrent Mark-Sweep Algorithm
**File: `src/memory/concurrent_mark_sweep.rs`**

```rust
pub struct ConcurrentMarkSweepCollector {
    /// Current GC state
    state: AtomicU8, // Idle, Marking, Sweeping
    /// Tri-color marking state
    marking_state: MarkingState,
    /// Write barrier state
    write_barriers: WriteBarrierState,
    /// Sweep progress tracking
    sweep_state: SweepState,
}

pub struct MarkingState {
    /// Gray objects waiting to be scanned
    gray_queue: ConcurrentQueue<*mut ObjectHeader>,
    /// Marking work distributed across threads
    work_stealing_queues: Vec<WorkStealingQueue<*mut ObjectHeader>>,
    /// Mark bits (separate from object headers for cache efficiency)
    mark_bitmap: AtomicBitmap,
}

impl ConcurrentMarkSweepCollector {
    /// Start concurrent marking phase
    pub fn start_marking(&self, roots: Vec<*mut ObjectHeader>) -> Result<(), String>;
    
    /// Process marking work (called by background threads)
    pub fn process_marking_work(&self) -> bool;
    
    /// Start concurrent sweeping phase
    pub fn start_sweeping(&self) -> Result<SweepStats, String>;
}
```

### 3.2 Write Barriers for Concurrent Safety
**File: `src/memory/write_barriers.rs`**

```rust
pub struct WriteBarrierManager {
    /// Current barrier type based on GC phase
    current_barrier: AtomicU8,
    /// Write barrier log for processing
    write_log: ConcurrentQueue<WriteBarrierEntry>,
    /// Remembered set for generational collection
    remembered_set: ConcurrentHashSet<*mut ObjectHeader>,
}

pub struct WriteBarrierEntry {
    /// Object being modified
    object: *mut ObjectHeader,
    /// Field offset being updated
    field_offset: usize,
    /// Old pointer value (for incremental barriers)
    old_value: *mut ObjectHeader,
    /// New pointer value
    new_value: *mut ObjectHeader,
    /// Timestamp
    timestamp: Instant,
}

/// Write barrier functions called from compiled code
extern "C" {
    pub fn cursed_write_barrier_slow(
        object: *mut ObjectHeader,
        field_offset: usize,
        old_value: *mut ObjectHeader,
        new_value: *mut ObjectHeader,
    );
}
```

## Phase 4: LLVM Integration (2-3 weeks)

### 4.1 Allocation Runtime Functions
**File: `src/runtime/gc_runtime.rs`**

```rust
/// Runtime functions called from compiled CURSED code
extern "C" {
    /// Allocate object with GC integration
    pub fn cursed_gc_alloc(size: usize, type_id: u32) -> *mut u8;
    
    /// Safe point check for goroutines
    pub fn cursed_gc_safe_point(goroutine_id: u64);
    
    /// Write barrier for pointer updates
    pub fn cursed_gc_write_barrier(
        object: *mut u8,
        field_offset: usize,
        new_value: *mut u8,
    );
    
    /// Trigger garbage collection
    pub fn cursed_gc_collect(force: bool) -> u64; // returns bytes collected
}

#[no_mangle]
pub extern "C" fn cursed_gc_alloc(size: usize, type_id: u32) -> *mut u8 {
    // Get thread-local GC state
    let gc = get_thread_local_gc();
    
    // Check allocation pressure
    if gc.should_trigger_collection() {
        gc.maybe_trigger_collection();
    }
    
    // Allocate with object header
    let total_size = size + size_of::<ObjectHeader>();
    let ptr = gc.heap_allocator.allocate(total_size)
        .expect("Failed to allocate memory");
    
    // Initialize object header
    let header = ptr as *mut ObjectHeader;
    unsafe {
        (*header).type_id = type_id;
        (*header).size = total_size as u32;
        (*header).gc_mark.store(0, Ordering::Relaxed); // White
        (*header).ref_count.store(0, Ordering::Relaxed);
        (*header).forward_ptr.store(null_mut(), Ordering::Relaxed);
    }
    
    // Return pointer to object data
    unsafe { (header.add(1)) as *mut u8 }
}
```

### 4.2 LLVM Code Generation Updates
**File: `src/codegen/llvm/gc_integration.rs`**

```rust
pub struct LlvmGcIntegration<'ctx> {
    context: &'ctx Context,
    module: &'ctx Module<'ctx>,
    builder: &'ctx Builder<'ctx>,
    /// Runtime GC function declarations
    gc_functions: GcRuntimeFunctions<'ctx>,
}

pub struct GcRuntimeFunctions<'ctx> {
    /// cursed_gc_alloc function
    alloc_fn: FunctionValue<'ctx>,
    /// cursed_gc_safe_point function
    safe_point_fn: FunctionValue<'ctx>,
    /// cursed_gc_write_barrier function
    write_barrier_fn: FunctionValue<'ctx>,
    /// cursed_gc_collect function
    collect_fn: FunctionValue<'ctx>,
}

impl<'ctx> LlvmGcIntegration<'ctx> {
    /// Generate allocation code for CURSED objects
    pub fn generate_allocation(
        &self,
        type_id: u32,
        size: IntValue<'ctx>,
    ) -> Result<PointerValue<'ctx>, String> {
        let type_id_val = self.context.i32_type().const_int(type_id as u64, false);
        
        let call = self.builder.build_call(
            self.gc_functions.alloc_fn,
            &[size.into(), type_id_val.into()],
            "gc_alloc",
        )?;
        
        Ok(call.try_as_basic_value().left().unwrap().into_pointer_value())
    }
    
    /// Insert safe point check
    pub fn insert_safe_point(&self, goroutine_id: IntValue<'ctx>) -> Result<(), String> {
        self.builder.build_call(
            self.gc_functions.safe_point_fn,
            &[goroutine_id.into()],
            "safe_point",
        )?;
        Ok(())
    }
    
    /// Insert write barrier for pointer assignment
    pub fn insert_write_barrier(
        &self,
        object: PointerValue<'ctx>,
        field_offset: IntValue<'ctx>,
        new_value: PointerValue<'ctx>,
    ) -> Result<(), String> {
        self.builder.build_call(
            self.gc_functions.write_barrier_fn,
            &[object.into(), field_offset.into(), new_value.into()],
            "write_barrier",
        )?;
        Ok(())
    }
}
```

## Phase 5: Stack Scanning & Root Set Management (1-2 weeks)

### 5.1 Conservative Stack Scanning
**File: `src/memory/stack_scanner.rs`**

```rust
pub struct StackScanner {
    /// Heap bounds for pointer validation
    heap_bounds: Vec<MemoryRegion>,
    /// Object header validation
    header_validator: ObjectHeaderValidator,
}

impl StackScanner {
    /// Scan goroutine stack conservatively
    pub fn scan_stack(
        &self,
        stack_base: *const u8,
        stack_top: *const u8,
        visitor: &mut dyn GcVisitor,
    ) -> Result<usize, String> {
        let mut potential_pointers = 0;
        
        // Scan stack in pointer-sized chunks
        let mut current = stack_base as *const usize;
        let end = stack_top as *const usize;
        
        while current < end {
            unsafe {
                let value = *current;
                
                // Check if value could be a heap pointer
                if self.is_potential_heap_pointer(value as *const u8) {
                    // Validate object header
                    if let Some(header) = self.validate_object_header(value as *mut ObjectHeader) {
                        visitor.visit_pointer(header);
                        potential_pointers += 1;
                    }
                }
                
                current = current.add(1);
            }
        }
        
        Ok(potential_pointers)
    }
    
    /// Check if pointer could point to heap
    fn is_potential_heap_pointer(&self, ptr: *const u8) -> bool {
        // Check alignment
        if (ptr as usize) % size_of::<usize>() != 0 {
            return false;
        }
        
        // Check if within heap bounds
        self.heap_bounds.iter().any(|region| region.contains(ptr))
    }
}
```

### 5.2 Precise Root Set Scanning
**File: `src/memory/root_scanner.rs`**

```rust
pub struct RootScanner {
    /// Global roots (static variables)
    global_roots: Vec<*mut ObjectHeader>,
    /// Goroutine local roots
    goroutine_roots: HashMap<GoroutineId, Vec<*mut ObjectHeader>>,
    /// Stack scanners for each goroutine
    stack_scanners: HashMap<GoroutineId, StackScanner>,
}

impl RootScanner {
    /// Scan all roots for GC marking phase
    pub fn scan_all_roots(&self, visitor: &mut dyn GcVisitor) -> Result<RootScanStats, String> {
        let mut stats = RootScanStats::default();
        
        // Scan global roots
        for &root in &self.global_roots {
            visitor.visit_pointer(root);
            stats.global_roots_scanned += 1;
        }
        
        // Scan goroutine roots and stacks
        for (&goroutine_id, roots) in &self.goroutine_roots {
            // Scan registered goroutine roots
            for &root in roots {
                visitor.visit_pointer(root);
                stats.goroutine_roots_scanned += 1;
            }
            
            // Scan goroutine stack
            if let Some(scanner) = self.stack_scanners.get(&goroutine_id) {
                if let Ok(goroutine) = get_goroutine_info(goroutine_id) {
                    let stack_roots = scanner.scan_stack(
                        goroutine.stack_base,
                        goroutine.stack_top,
                        visitor,
                    )?;
                    stats.stack_roots_scanned += stack_roots;
                }
            }
        }
        
        Ok(stats)
    }
}
```

## Phase 6: Testing & Integration (2-3 weeks)

### 6.1 Comprehensive Test Suite
**File: `tests/real_gc_integration_test.rs`**

```rust
/// Test real GC with concurrent goroutines
#[test]
fn test_concurrent_gc_with_goroutines() {
    let gc = RealGarbageCollector::new();
    let scheduler = RealGoroutineScheduler::new();
    
    // Spawn multiple goroutines that allocate objects
    let handles: Vec<_> = (0..10).map(|i| {
        scheduler.spawn(move || {
            for j in 0..100 {
                let obj = allocate_test_object(i * 100 + j);
                // Create some inter-object references
                if j > 0 {
                    link_objects(&obj, &get_previous_object());
                }
                // Yield to allow GC
                yield_goroutine();
            }
        })
    }).collect();
    
    // Trigger multiple GC cycles while goroutines run
    for _ in 0..5 {
        std::thread::sleep(Duration::from_millis(10));
        let stats = gc.collect().unwrap();
        assert!(stats.objects_collected > 0);
    }
    
    // Wait for goroutines to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Final GC should collect remaining objects
    let final_stats = gc.collect().unwrap();
    assert!(final_stats.objects_collected > 0);
}
```

### 6.2 Performance Benchmarks
**File: `benches/real_gc_benchmark.rs`**

```rust
/// Benchmark allocation throughput with real GC
#[bench]
fn bench_allocation_throughput(b: &mut Bencher) {
    let gc = RealGarbageCollector::new();
    
    b.iter(|| {
        // Allocate 1000 objects
        let objects: Vec<_> = (0..1000).map(|i| {
            allocate_test_object(i)
        }).collect();
        
        // Keep objects alive for measurement
        black_box(objects);
    });
}

/// Benchmark GC pause times
#[bench] 
fn bench_gc_pause_times(b: &mut Bencher) {
    let gc = RealGarbageCollector::new();
    
    // Pre-allocate objects to create garbage
    for _ in 0..10000 {
        allocate_test_object(rand::random());
    }
    
    b.iter(|| {
        let start = Instant::now();
        gc.collect().unwrap();
        let duration = start.elapsed();
        
        // Assert pause time is reasonable (< 10ms)
        assert!(duration < Duration::from_millis(10));
        duration
    });
}
```

## Implementation Timeline

| Phase | Duration | Deliverables |
|-------|----------|-------------|
| Phase 1 | 2-3 weeks | Real goroutine runtime with safe points |
| Phase 2 | 1-2 weeks | Object headers and precise scanning |
| Phase 3 | 2-3 weeks | Concurrent mark-sweep collector |
| Phase 4 | 2-3 weeks | LLVM integration and allocation runtime |
| Phase 5 | 1-2 weeks | Stack scanning and root management |
| Phase 6 | 2-3 weeks | Testing, benchmarking, and optimization |

**Total Estimated Time: 10-16 weeks**

## Success Criteria

1. **Functional**: CURSED programs with goroutines run without memory leaks
2. **Performance**: 
   - Allocation throughput > 1M objects/second
   - GC pause times < 10ms (95th percentile)
   - Memory overhead < 25% compared to manual management
3. **Concurrent Safety**: No data races or corruption during concurrent GC
4. **Compatibility**: Existing CURSED code continues to work without changes

## Risks and Mitigation

### High Risk: Concurrent GC Correctness
- **Mitigation**: Extensive testing with ThreadSanitizer and stress tests
- **Fallback**: Start with stop-the-world collection, add concurrency later

### Medium Risk: Stack Scanning Performance
- **Mitigation**: Conservative scanning with optimization for common patterns
- **Fallback**: Require precise root registration if performance insufficient

### Low Risk: LLVM Integration Complexity
- **Mitigation**: Start with simple allocation calls, add optimizations incrementally
- **Fallback**: Use function calls for all GC operations initially
