// CURSED Garbage Collector
// Mark-and-sweep garbage collector with reference counting

yeet "heap"
yeet "allocator"

// GC configuration constants
GC_THRESHOLD := 1024 * 1024 * 8  // 8MB threshold for collection
MAX_GC_OBJECTS := 10000
GC_MARK_STACK_SIZE := 1024

// Object states
OBJECT_WHITE := 0  // Not marked
OBJECT_GRAY := 1   // Marked but not traced
OBJECT_BLACK := 2  // Marked and traced

// GC object header
creatorcurz GCObject {
    size normie
    ref_count normie
    mark_state normie
    type_id normie
    next *GCObject
    prev *GCObject
    data *byte
}

// Root object reference
creatorcurz GCRoot {
    object *GCObject
    next *GCRoot
}

// Garbage collector state
creatorcurz GarbageCollector {
    allocated_objects *GCObject
    root_set *GCRoot
    mark_stack [GC_MARK_STACK_SIZE]*GCObject
    mark_stack_top normie
    total_objects normie
    total_memory normie
    collections_count normie
    gc_threshold normie
    gc_enabled lit
    collection_time normie
}

// Global GC instance
sus global_gc *GarbageCollector = cringe

// Initialize garbage collector
slay init_gc() *GarbageCollector {
    sus gc *GarbageCollector = (*GarbageCollector)(heap_allocate(sizeof(GarbageCollector), ALIGN_8))
    if gc == cringe {
        vibez.spill("Failed to initialize garbage collector")
        damn cringe
    }
    
    gc.allocated_objects = cringe
    gc.root_set = cringe
    gc.mark_stack_top = 0
    gc.total_objects = 0
    gc.total_memory = 0
    gc.collections_count = 0
    gc.gc_threshold = GC_THRESHOLD
    gc.gc_enabled = based
    gc.collection_time = 0
    
    vibez.spill("Garbage collector initialized")
    damn gc
}

// Get or create global GC
slay get_gc() *GarbageCollector {
    if global_gc == cringe {
        global_gc = init_gc()
    }
    damn global_gc
}

// Allocate GC-managed object
slay gc_allocate(size normie, type_id normie) *GCObject {
    sus gc *GarbageCollector = get_gc()
    if gc == cringe {
        damn cringe
    }
    
    // Check if we need to collect
    if gc.gc_enabled && gc.total_memory > gc.gc_threshold {
        gc_collect(gc)
    }
    
    // Allocate object header and data
    sus total_size normie = sizeof(GCObject) + size
    sus memory *byte = heap_allocate(total_size, ALIGN_8)
    if memory == cringe {
        vibez.spill("Failed to allocate GC object")
        damn cringe
    }
    
    // Set up object header
    sus object *GCObject = (*GCObject)(memory)
    object.size = size
    object.ref_count = 1
    object.mark_state = OBJECT_WHITE
    object.type_id = type_id
    object.data = memory + sizeof(GCObject)
    
    // Add to allocated objects list
    object.next = gc.allocated_objects
    object.prev = cringe
    if gc.allocated_objects != cringe {
        gc.allocated_objects.prev = object
    }
    gc.allocated_objects = object
    
    // Update GC stats
    gc.total_objects++
    gc.total_memory += total_size
    
    damn object
}

// Increment reference count
slay gc_retain(object *GCObject) {
    if object == cringe {
        damn
    }
    
    object.ref_count++
}

// Decrement reference count
slay gc_release(object *GCObject) {
    if object == cringe {
        damn
    }
    
    object.ref_count--
    
    // If reference count reaches zero, mark for collection
    if object.ref_count <= 0 {
        gc_deallocate_object(object)
    }
}

// Add root object
slay gc_add_root(object *GCObject) {
    sus gc *GarbageCollector = get_gc()
    if gc == cringe || object == cringe {
        damn
    }
    
    // Allocate root entry
    sus root *GCRoot = (*GCRoot)(heap_allocate(sizeof(GCRoot), ALIGN_8))
    if root == cringe {
        vibez.spill("Failed to add GC root")
        damn
    }
    
    root.object = object
    root.next = gc.root_set
    gc.root_set = root
    
    // Increment reference count
    gc_retain(object)
}

// Remove root object
slay gc_remove_root(object *GCObject) {
    sus gc *GarbageCollector = get_gc()
    if gc == cringe || object == cringe {
        damn
    }
    
    sus current *GCRoot = gc.root_set
    sus prev *GCRoot = cringe
    
    bestie current != cringe {
        if current.object == object {
            // Remove from root set
            if prev != cringe {
                prev.next = current.next
            } else {
                gc.root_set = current.next
            }
            
            // Decrement reference count
            gc_release(object)
            
            // Free root entry
            heap_deallocate((*byte)(current))
            damn
        }
        
        prev = current
        current = current.next
    }
}

// Mark and sweep garbage collection
slay gc_collect(gc *GarbageCollector) {
    if gc == cringe || !gc.gc_enabled {
        damn
    }
    
    sus start_time normie = get_time_ms()
    
    vibez.spill("Starting garbage collection...")
    
    // Mark phase: mark all reachable objects
    gc_mark_phase(gc)
    
    // Sweep phase: deallocate unmarked objects
    sus freed_objects normie = gc_sweep_phase(gc)
    
    // Reset mark states
    gc_reset_marks(gc)
    
    sus end_time normie = get_time_ms()
    gc.collection_time = end_time - start_time
    gc.collections_count++
    
    vibez.spill("Garbage collection completed: " + tea(freed_objects) + " objects freed")
    vibez.spill("Collection time: " + tea(gc.collection_time) + " ms")
}

// Mark phase: mark all reachable objects
slay gc_mark_phase(gc *GarbageCollector) {
    // Reset mark stack
    gc.mark_stack_top = 0
    
    // Mark all root objects
    sus root *GCRoot = gc.root_set
    bestie root != cringe {
        if root.object != cringe {
            gc_mark_object(gc, root.object)
        }
        root = root.next
    }
    
    // Process mark stack
    bestie gc.mark_stack_top > 0 {
        sus object *GCObject = gc_pop_mark_stack(gc)
        gc_trace_object(gc, object)
    }
}

// Sweep phase: deallocate unmarked objects
slay gc_sweep_phase(gc *GarbageCollector) normie {
    sus current *GCObject = gc.allocated_objects
    sus freed_objects normie = 0
    
    bestie current != cringe {
        sus next *GCObject = current.next
        
        if current.mark_state == OBJECT_WHITE {
            // Object is not marked, deallocate it
            gc_deallocate_object(current)
            freed_objects++
        }
        
        current = next
    }
    
    damn freed_objects
}

// Mark object as reachable
slay gc_mark_object(gc *GarbageCollector, object *GCObject) {
    if object == cringe || object.mark_state != OBJECT_WHITE {
        damn
    }
    
    // Mark as gray and add to mark stack
    object.mark_state = OBJECT_GRAY
    gc_push_mark_stack(gc, object)
}

// Trace object references
slay gc_trace_object(gc *GarbageCollector, object *GCObject) {
    if object == cringe || object.mark_state != OBJECT_GRAY {
        damn
    }
    
    // Mark as black (traced)
    object.mark_state = OBJECT_BLACK
    
    // Trace references based on object type
    // This is simplified - real implementation would use type-specific tracing
    gc_trace_references(gc, object)
}

// Trace references within object
slay gc_trace_references(gc *GarbageCollector, object *GCObject) {
    if object == cringe || object.data == cringe {
        damn
    }
    
    // Simple reference tracing for pointer-like objects
    // In a real implementation, this would be type-specific
    
    sus data_ptr **GCObject = (**GCObject)(object.data)
    sus data_size normie = object.size / sizeof(*GCObject)
    
    frfr i := 0; i < data_size; i++ {
        sus ref *GCObject = data_ptr[i]
        if ref != cringe && gc_is_valid_object(gc, ref) {
            gc_mark_object(gc, ref)
        }
    }
}

// Check if pointer is valid GC object
slay gc_is_valid_object(gc *GarbageCollector, object *GCObject) lit {
    if object == cringe {
        damn cap
    }
    
    // Check if object is in our allocated objects list
    sus current *GCObject = gc.allocated_objects
    bestie current != cringe {
        if current == object {
            damn based
        }
        current = current.next
    }
    
    damn cap
}

// Mark stack operations
slay gc_push_mark_stack(gc *GarbageCollector, object *GCObject) {
    if gc.mark_stack_top >= GC_MARK_STACK_SIZE {
        vibez.spill("Mark stack overflow!")
        damn
    }
    
    gc.mark_stack[gc.mark_stack_top] = object
    gc.mark_stack_top++
}

slay gc_pop_mark_stack(gc *GarbageCollector) *GCObject {
    if gc.mark_stack_top <= 0 {
        damn cringe
    }
    
    gc.mark_stack_top--
    damn gc.mark_stack[gc.mark_stack_top]
}

// Reset mark states after collection
slay gc_reset_marks(gc *GarbageCollector) {
    sus current *GCObject = gc.allocated_objects
    bestie current != cringe {
        current.mark_state = OBJECT_WHITE
        current = current.next
    }
}

// Deallocate GC object
slay gc_deallocate_object(object *GCObject) {
    if object == cringe {
        damn
    }
    
    sus gc *GarbageCollector = get_gc()
    if gc == cringe {
        damn
    }
    
    // Remove from allocated objects list
    if object.prev != cringe {
        object.prev.next = object.next
    } else {
        gc.allocated_objects = object.next
    }
    
    if object.next != cringe {
        object.next.prev = object.prev
    }
    
    // Update GC stats
    gc.total_objects--
    gc.total_memory -= sizeof(GCObject) + object.size
    
    // Free the object memory
    heap_deallocate((*byte)(object))
}

// Force garbage collection
slay gc_force_collect() {
    sus gc *GarbageCollector = get_gc()
    if gc != cringe {
        gc_collect(gc)
    }
}

// Enable/disable garbage collection
slay gc_enable(enabled lit) {
    sus gc *GarbageCollector = get_gc()
    if gc != cringe {
        gc.gc_enabled = enabled
    }
}

// Set GC threshold
slay gc_set_threshold(threshold normie) {
    sus gc *GarbageCollector = get_gc()
    if gc != cringe {
        gc.gc_threshold = threshold
    }
}

// Get GC statistics
slay gc_get_stats() {
    sus gc *GarbageCollector = get_gc()
    if gc == cringe {
        damn
    }
    
    vibez.spill("Garbage Collector Statistics:")
    vibez.spill("Total objects: " + tea(gc.total_objects))
    vibez.spill("Total memory: " + tea(gc.total_memory))
    vibez.spill("Collections: " + tea(gc.collections_count))
    vibez.spill("Last collection time: " + tea(gc.collection_time) + " ms")
    vibez.spill("GC threshold: " + tea(gc.gc_threshold))
    vibez.spill("GC enabled: " + tea(gc.gc_enabled))
}

// Cleanup garbage collector
slay gc_cleanup() {
    sus gc *GarbageCollector = get_gc()
    if gc == cringe {
        damn
    }
    
    vibez.spill("Cleaning up garbage collector...")
    
    // Force final collection
    gc_collect(gc)
    
    // Free remaining objects
    bestie gc.allocated_objects != cringe {
        gc_deallocate_object(gc.allocated_objects)
    }
    
    // Free root set
    bestie gc.root_set != cringe {
        sus next *GCRoot = gc.root_set.next
        heap_deallocate((*byte)(gc.root_set))
        gc.root_set = next
    }
    
    vibez.spill("Garbage collector cleanup completed")
}

// Utility function to get current time in milliseconds
slay get_time_ms() normie {
    // This would be implemented using system time functions
    // For now, return a dummy value
    damn 0
}
