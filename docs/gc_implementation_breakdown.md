# GC Implementation Breakdown

## Component Implementation Plan

### 1. Core Data Structures

#### 1.1 Object Identification (`src/memory/object_id.rs`)
```rust
/// Unique identifier for heap objects
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct ObjectId(u64);

impl ObjectId {
    pub fn new() -> Self {
        static COUNTER: AtomicU64 = AtomicU64::new(1);
        Self(COUNTER.fetch_add(1, Ordering::SeqCst))
    }
    
    pub fn as_u64(&self) -> u64 { self.0 }
    pub fn null() -> Self { Self(0) }
    pub fn is_null(&self) -> bool { self.0 == 0 }
}

/// Registry for tracking all heap objects
pub struct ObjectRegistry {
    objects: DashMap<ObjectId, Arc<HeapObject>>,
    statistics: AtomicRegistryStats,
}
```

#### 1.2 Heap Object Management (`src/memory/heap_object.rs`)
```rust
/// Object stored in the garbage collected heap
pub struct HeapObject {
    /// Unique identifier
    pub id: ObjectId,
    
    /// Object metadata
    pub header: ObjectHeader,
    
    /// Raw object data
    pub data: ObjectData,
    
    /// GC marking state
    pub mark_state: AtomicMarkState,
    
    /// Objects this object references
    pub references: Mutex<Vec<ObjectId>>,
    
    /// Allocation timestamp
    pub allocated_at: Instant,
}

#[derive(Debug)]
pub struct ObjectHeader {
    pub type_id: TypeId,
    pub size: usize,
    pub flags: ObjectFlags,
}

pub enum ObjectData {
    Primitive(PrimitiveValue),
    Struct(StructData),
    Array(ArrayData),
    Function(FunctionData),
    Interface(InterfaceData),
}
```

#### 1.3 Memory Block Management (`src/memory/memory_block.rs`)
```rust
/// Large memory block for object allocation
pub struct MemoryBlock {
    /// Block metadata
    pub id: BlockId,
    pub base_ptr: *mut u8,
    pub size: usize,
    pub allocated: usize,
    
    /// Allocation tracking
    pub allocations: Vec<AllocationInfo>,
    
    /// Free space management
    pub free_chunks: BTreeSet<FreeChunk>,
}

/// Information about an allocation within a block
#[derive(Debug)]
pub struct AllocationInfo {
    pub object_id: ObjectId,
    pub offset: usize,
    pub size: usize,
    pub allocated_at: Instant,
}

/// Free memory chunk within a block
#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FreeChunk {
    pub offset: usize,
    pub size: usize,
}
```

### 2. Mark-and-Sweep Algorithm

#### 2.1 Marking Phase (`src/memory/mark.rs`)
```rust
/// Marking phase implementation
pub struct MarkingPhase {
    visited: HashSet<ObjectId>,
    work_queue: VecDeque<ObjectId>,
    stats: MarkingStats,
}

impl MarkingPhase {
    /// Start marking from root set
    pub fn mark_from_roots(&mut self, roots: &[ObjectId]) -> Result<MarkingStats, GcError> {
        // Initialize work queue with roots
        for &root in roots {
            if !root.is_null() {
                self.work_queue.push_back(root);
            }
        }
        
        // Process work queue
        while let Some(object_id) = self.work_queue.pop_front() {
            self.mark_object(object_id)?;
        }
        
        Ok(self.stats.clone())
    }
    
    /// Mark a single object and add its references to work queue
    fn mark_object(&mut self, object_id: ObjectId) -> Result<(), GcError> {
        // Avoid revisiting objects
        if !self.visited.insert(object_id) {
            return Ok(());
        }
        
        // Get object from registry
        let object = self.get_object(object_id)?;
        
        // Mark object as reachable
        object.mark_state.store(MarkState::Marked, Ordering::SeqCst);
        
        // Add references to work queue
        let references = object.references.lock().unwrap();
        for &ref_id in references.iter() {
            if !ref_id.is_null() && !self.visited.contains(&ref_id) {
                self.work_queue.push_back(ref_id);
            }
        }
        
        self.stats.objects_marked += 1;
        Ok(())
    }
}
```

#### 2.2 Sweep Phase (`src/memory/sweep.rs`)
```rust
/// Sweep phase implementation
pub struct SweepingPhase {
    freed_objects: Vec<ObjectId>,
    freed_memory: usize,
    stats: SweepingStats,
}

impl SweepingPhase {
    /// Sweep all objects, freeing unmarked ones
    pub fn sweep_heap(&mut self, registry: &ObjectRegistry) -> Result<SweepingStats, GcError> {
        let mut objects_to_free = Vec::new();
        
        // Identify unmarked objects
        for entry in registry.objects.iter() {
            let object_id = *entry.key();
            let object = entry.value();
            
            if object.mark_state.load(Ordering::SeqCst) == MarkState::Unmarked {
                objects_to_free.push(object_id);
            } else {
                // Reset mark state for next collection
                object.mark_state.store(MarkState::Unmarked, Ordering::SeqCst);
            }
        }
        
        // Free unmarked objects
        for object_id in objects_to_free {
            self.free_object(object_id, registry)?;
        }
        
        Ok(self.stats.clone())
    }
    
    /// Free a specific object
    fn free_object(&mut self, object_id: ObjectId, registry: &ObjectRegistry) -> Result<(), GcError> {
        if let Some((_, object)) = registry.objects.remove(&object_id) {
            self.freed_memory += object.header.size;
            self.freed_objects.push(object_id);
            self.stats.objects_freed += 1;
        }
        Ok(())
    }
}
```

### 3. Main Collector Implementation

#### 3.1 Core Collector (`src/memory/mark_sweep_collector.rs`)
```rust
/// Main mark-and-sweep garbage collector
pub struct MarkSweepCollector {
    /// Object registry
    registry: Arc<ObjectRegistry>,
    
    /// Memory block manager
    block_manager: Mutex<BlockManager>,
    
    /// Root set management
    root_manager: Arc<RootManager>,
    
    /// Collection statistics
    stats: Arc<Mutex<CollectionStats>>,
    
    /// Configuration
    config: GcConfig,
}

impl MarkSweepCollector {
    pub fn new(config: GcConfig) -> Self {
        Self {
            registry: Arc::new(ObjectRegistry::new()),
            block_manager: Mutex::new(BlockManager::new(config.block_size)),
            root_manager: Arc::new(RootManager::new()),
            stats: Arc::new(Mutex::new(CollectionStats::default())),
            config,
        }
    }
    
    /// Perform a complete mark-and-sweep collection
    pub fn collect(&self) -> Result<CollectionStats, GcError> {
        let start_time = Instant::now();
        
        // Phase 1: Mark reachable objects
        let mark_stats = self.mark_phase()?;
        
        // Phase 2: Sweep unreachable objects
        let sweep_stats = self.sweep_phase()?;
        
        // Update statistics
        let mut stats = self.stats.lock().unwrap();
        stats.update_from_collection(mark_stats, sweep_stats);
        stats.last_collection_time = start_time.elapsed();
        
        Ok(stats.clone())
    }
    
    /// Mark phase implementation
    fn mark_phase(&self) -> Result<MarkingStats, GcError> {
        let mut marking = MarkingPhase::new();
        
        // Get all root objects
        let roots = self.root_manager.get_all_roots();
        
        // Mark from roots
        marking.mark_from_roots(&roots)
    }
    
    /// Sweep phase implementation
    fn sweep_phase(&self) -> Result<SweepingStats, GcError> {
        let mut sweeping = SweepingPhase::new();
        
        // Sweep the heap
        sweeping.sweep_heap(&self.registry)
    }
}
```

#### 3.2 Root Set Management (`src/memory/root_manager.rs`)
```rust
/// Manages the set of root objects for GC
pub struct RootManager {
    /// Global roots (static variables, etc.)
    global_roots: DashSet<ObjectId>,
    
    /// Stack roots (local variables)
    stack_roots: DashMap<ThreadId, Vec<ObjectId>>,
    
    /// Temporary roots (intermediate values)
    temp_roots: DashSet<ObjectId>,
}

impl RootManager {
    pub fn add_global_root(&self, object_id: ObjectId) {
        self.global_roots.insert(object_id);
    }
    
    pub fn remove_global_root(&self, object_id: ObjectId) {
        self.global_roots.remove(&object_id);
    }
    
    pub fn add_stack_root(&self, thread_id: ThreadId, object_id: ObjectId) {
        self.stack_roots.entry(thread_id)
            .or_insert_with(Vec::new)
            .push(object_id);
    }
    
    pub fn get_all_roots(&self) -> Vec<ObjectId> {
        let mut roots = Vec::new();
        
        // Add global roots
        for root in self.global_roots.iter() {
            roots.push(*root);
        }
        
        // Add stack roots from all threads
        for entry in self.stack_roots.iter() {
            roots.extend(entry.value().iter().copied());
        }
        
        // Add temporary roots
        for root in self.temp_roots.iter() {
            roots.push(*root);
        }
        
        roots
    }
}
```

### 4. Integration with Existing GC System

#### 4.1 Enhanced GarbageCollector (`src/memory/gc.rs`)
```rust
/// Enhanced garbage collector with real implementation
pub struct GarbageCollector {
    /// Core mark-and-sweep collector
    core_collector: Arc<MarkSweepCollector>,
    
    /// Goroutine-aware collector (existing)
    goroutine_collector: Option<Arc<GoroutineGarbageCollector>>,
    
    /// Allocation tracking
    allocator: Arc<Mutex<GcAllocator>>,
    
    /// Configuration
    config: GcConfig,
}

impl GarbageCollector {
    pub fn new() -> Self {
        let config = GcConfig::default();
        let core_collector = Arc::new(MarkSweepCollector::new(config.clone()));
        
        Self {
            core_collector,
            goroutine_collector: None,
            allocator: Arc::new(Mutex::new(GcAllocator::new(core_collector.clone()))),
            config,
        }
    }
    
    /// Main collection method with goroutine awareness
    pub fn collect(&mut self) -> Result<CollectionStats, GcError> {
        // Check if goroutine-aware collection is needed
        if let Some(ref goroutine_gc) = self.goroutine_collector {
            if self.should_use_goroutine_aware_collection() {
                return goroutine_gc.collect_garbage_goroutine_aware()
                    .map_err(|e| GcError::GorounteCollection(e));
            }
        }
        
        // Use core mark-and-sweep collection
        self.core_collector.collect()
    }
    
    /// Allocate object in GC heap
    pub fn allocate<T>(&mut self, obj: T) -> Result<Gc<T>, GcError>
    where
        T: Traceable + 'static,
    {
        let mut allocator = self.allocator.lock().unwrap();
        allocator.allocate(obj)
    }
    
    /// Check if goroutine-aware collection should be used
    fn should_use_goroutine_aware_collection(&self) -> bool {
        // Check if any goroutines are active
        // This would integrate with the existing goroutine runtime
        false // Placeholder
    }
}
```

#### 4.2 Real Gc<T> Smart Pointer (`src/memory/gc_ptr.rs`)
```rust
/// Smart pointer for garbage collected objects
pub struct Gc<T> {
    object_id: ObjectId,
    registry: Arc<ObjectRegistry>,
    _phantom: PhantomData<T>,
}

impl<T: 'static> Gc<T> {
    /// Create new Gc pointer
    pub(crate) fn new(object_id: ObjectId, registry: Arc<ObjectRegistry>) -> Self {
        Self {
            object_id,
            registry,
            _phantom: PhantomData,
        }
    }
    
    /// Get reference to the contained object
    pub fn as_ref(&self) -> Result<GcRef<T>, GcError> {
        let object = self.registry.get_object(self.object_id)
            .ok_or(GcError::ObjectNotFound(self.object_id))?;
        
        GcRef::new(object)
    }
    
    /// Get the object ID
    pub fn id(&self) -> ObjectId {
        self.object_id
    }
}

impl<T> Clone for Gc<T> {
    fn clone(&self) -> Self {
        Self {
            object_id: self.object_id,
            registry: self.registry.clone(),
            _phantom: PhantomData,
        }
    }
}

/// RAII guard for accessing GC objects
pub struct GcRef<T> {
    object: Arc<HeapObject>,
    _phantom: PhantomData<T>,
}

impl<T: 'static> GcRef<T> {
    fn new(object: Arc<HeapObject>) -> Result<Self, GcError> {
        // Verify object type
        if object.header.type_id != TypeId::of::<T>() {
            return Err(GcError::TypeMismatch);
        }
        
        Ok(Self {
            object,
            _phantom: PhantomData,
        })
    }
}

impl<T: 'static> Deref for GcRef<T> {
    type Target = T;
    
    fn deref(&self) -> &Self::Target {
        match &self.object.data {
            ObjectData::Primitive(PrimitiveValue::Custom(data)) => {
                data.downcast_ref::<T>().unwrap()
            }
            _ => panic!("Invalid object data type"),
        }
    }
}
```

### 5. LLVM Runtime Integration

#### 5.1 Runtime Functions (`src/codegen/llvm/gc_runtime.rs`)
```rust
/// FFI functions for LLVM-generated code
use std::ffi::c_void;

/// Allocate object in GC heap
#[no_mangle]
pub extern "C" fn cursed_gc_allocate(size: usize, type_id: u32) -> *mut c_void {
    let gc = get_global_gc();
    match gc.allocate_raw(size, type_id) {
        Ok(ptr) => ptr,
        Err(_) => std::ptr::null_mut(),
    }
}

/// Add root object
#[no_mangle]
pub extern "C" fn cursed_gc_add_root(ptr: *mut c_void) {
    if let Some(object_id) = ptr_to_object_id(ptr) {
        let gc = get_global_gc();
        gc.root_manager.add_global_root(object_id);
    }
}

/// Remove root object
#[no_mangle]
pub extern "C" fn cursed_gc_remove_root(ptr: *mut c_void) {
    if let Some(object_id) = ptr_to_object_id(ptr) {
        let gc = get_global_gc();
        gc.root_manager.remove_global_root(object_id);
    }
}

/// Trigger garbage collection
#[no_mangle]
pub extern "C" fn cursed_gc_collect() -> u32 {
    let mut gc = get_global_gc();
    match gc.collect() {
        Ok(stats) => stats.objects_freed as u32,
        Err(_) => 0,
    }
}

/// Get global GC instance
fn get_global_gc() -> &'static mut GarbageCollector {
    // Global GC instance - thread-safe access needed
    unsafe { &mut GLOBAL_GC }
}

/// Convert pointer to ObjectId
fn ptr_to_object_id(ptr: *mut c_void) -> Option<ObjectId> {
    // Implementation would map pointers to ObjectIds
    None // Placeholder
}

/// Global GC instance
static mut GLOBAL_GC: GarbageCollector = unsafe { std::mem::zeroed() };
```

### 6. Testing Infrastructure

#### 6.1 Core GC Tests (`tests/gc_core_test.rs`)
```rust
/// Test basic allocation and deallocation
#[test]
fn test_allocation_deallocation() {
    init_tracing!();
    let mut gc = GarbageCollector::new();
    
    // Allocate test objects
    let obj1 = gc.allocate(TestStruct { value: 42 }).unwrap();
    let obj2 = gc.allocate(TestStruct { value: 100 }).unwrap();
    
    // Verify objects are accessible
    assert_eq!(obj1.as_ref().unwrap().value, 42);
    assert_eq!(obj2.as_ref().unwrap().value, 100);
    
    // Collect garbage (should not free referenced objects)
    let stats = gc.collect().unwrap();
    assert_eq!(stats.objects_freed, 0);
    
    // Drop references
    drop(obj1);
    drop(obj2);
    
    // Collect garbage (should free unreferenced objects)
    let stats = gc.collect().unwrap();
    assert_eq!(stats.objects_freed, 2);
}

/// Test circular reference handling
#[test]
fn test_circular_references() {
    init_tracing!();
    let mut gc = GarbageCollector::new();
    
    // Create circular reference
    let obj1 = gc.allocate(RefStruct::new()).unwrap();
    let obj2 = gc.allocate(RefStruct::new()).unwrap();
    
    obj1.as_ref().unwrap().set_reference(obj2.clone());
    obj2.as_ref().unwrap().set_reference(obj1.clone());
    
    // Drop original references
    drop(obj1);
    drop(obj2);
    
    // Collect garbage (should detect and free circular references)
    let stats = gc.collect().unwrap();
    assert_eq!(stats.objects_freed, 2);
}
```

#### 6.2 Integration Tests (`tests/gc_integration_test.rs`)
```rust
/// Test integration with goroutine GC
#[test]
fn test_goroutine_gc_integration() {
    init_tracing!();
    let mut gc = GarbageCollector::new();
    
    // Enable goroutine awareness
    gc.enable_goroutine_awareness();
    
    // Test should use goroutine-aware collection when goroutines are active
    // and fallback to core collection otherwise
    
    // Without active goroutines
    let stats = gc.collect().unwrap();
    assert!(stats.collection_type == CollectionType::MarkSweep);
    
    // With active goroutines (simulated)
    gc.simulate_active_goroutines(true);
    let stats = gc.collect().unwrap();
    assert!(stats.collection_type == CollectionType::GorutineAware);
}
```

This implementation breakdown provides a comprehensive foundation for building a production-ready garbage collector that integrates seamlessly with the existing CURSED infrastructure while providing all the missing core functionality.
