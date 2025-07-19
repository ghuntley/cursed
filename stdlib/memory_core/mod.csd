# Memory Core - Pure CURSED Memory Management System
# Garbage collection and heap management without FFI dependencies  
# Replaces src/runtime/memory.rs and src/runtime/gc.rs

yeet "runtime_core"
yeet "error_drip"
yeet "testz"

# Memory allocation types
sus ALLOC_STACK normie = 1
sus ALLOC_HEAP normie = 2
sus ALLOC_GLOBAL normie = 3

# GC states
sus GC_IDLE normie = 0
sus GC_MARKING normie = 1
sus GC_SWEEPING normie = 2
sus GC_COMPACTING normie = 3

# Memory object representation
vibe MemoryObject = smash {
    id normie,
    size normie,
    alloc_type normie,
    is_marked lit,
    ref_count normie,
    data tea,
    allocated_at normie,
    last_accessed normie
}

# Heap management
vibe Heap = smash {
    objects map[normie]MemoryObject,
    free_list []normie,
    total_allocated normie,
    total_freed normie,
    next_id normie,
    gc_threshold normie,
    gc_state normie
}

# GC statistics
vibe GCStats = smash {
    collections_run normie,
    objects_collected normie,
    bytes_freed normie,
    collection_time normie,
    heap_size normie,
    live_objects normie
}

# Global memory management state
sus global_heap Heap
sus gc_stats GCStats
sus gc_enabled lit = based
sus gc_pressure_threshold normie = 1048576  # 1MB

# Memory configuration
sus DEFAULT_GC_THRESHOLD normie = 524288    # 512KB
sus MAX_HEAP_SIZE normie = 134217728        # 128MB
sus GC_COLLECTION_INTERVAL normie = 1000    # Objects allocated between GC runs

# ==============================================================================
# MEMORY MANAGEMENT INITIALIZATION
# ==============================================================================

# Initialize memory management system
slay init_memory_system() lit {
    global_heap.objects = {}
    global_heap.free_list = []
    global_heap.total_allocated = 0
    global_heap.total_freed = 0
    global_heap.next_id = 1
    global_heap.gc_threshold = DEFAULT_GC_THRESHOLD
    global_heap.gc_state = GC_IDLE
    
    gc_stats.collections_run = 0
    gc_stats.objects_collected = 0
    gc_stats.bytes_freed = 0
    gc_stats.collection_time = 0
    gc_stats.heap_size = 0
    gc_stats.live_objects = 0
    
    vibez.spill("Memory management system initialized")
    damn based
}

# ==============================================================================
# MEMORY ALLOCATION AND DEALLOCATION
# ==============================================================================

# Allocate memory object
slay allocate_memory(size normie, alloc_type normie) normie {
    lowkey size <= 0 {
        damn -1  # Invalid size
    }
    
    lowkey global_heap.total_allocated + size > MAX_HEAP_SIZE {
        # Try garbage collection first
        lowkey gc_enabled {
            run_garbage_collection()
        }
        
        # Check again after GC
        lowkey global_heap.total_allocated + size > MAX_HEAP_SIZE {
            damn -1  # Out of memory
        }
    }
    
    sus object_id normie = global_heap.next_id
    global_heap.next_id = global_heap.next_id + 1
    
    sus memory_obj MemoryObject
    memory_obj.id = object_id
    memory_obj.size = size
    memory_obj.alloc_type = alloc_type
    memory_obj.is_marked = cap
    memory_obj.ref_count = 1
    memory_obj.data = ""  # Simulate allocated data
    memory_obj.allocated_at = get_current_timestamp()
    memory_obj.last_accessed = memory_obj.allocated_at
    
    global_heap.objects[object_id] = memory_obj
    global_heap.total_allocated = global_heap.total_allocated + size
    gc_stats.heap_size = gc_stats.heap_size + size
    gc_stats.live_objects = gc_stats.live_objects + 1
    
    # Check if GC should run
    lowkey should_run_gc() {
        run_garbage_collection()
    }
    
    damn object_id
}

# Deallocate memory object
slay deallocate_memory(object_id normie) lit {
    lowkey !memory_object_exists(object_id) {
        damn cap
    }
    
    sus memory_obj MemoryObject = global_heap.objects[object_id]
    global_heap.total_freed = global_heap.total_freed + memory_obj.size
    gc_stats.heap_size = gc_stats.heap_size - memory_obj.size
    gc_stats.live_objects = gc_stats.live_objects - 1
    
    # Remove from heap
    delete(global_heap.objects, object_id)
    global_heap.free_list = append(global_heap.free_list, object_id)
    
    damn based
}

# Check if memory object exists
slay memory_object_exists(object_id normie) lit {
    damn global_heap.objects[object_id].id == object_id
}

# Get memory object info
slay get_memory_object(object_id normie) MemoryObject {
    lowkey memory_object_exists(object_id) {
        damn global_heap.objects[object_id]
    }
    
    sus empty MemoryObject
    damn empty
}

# ==============================================================================
# REFERENCE COUNTING
# ==============================================================================

# Increment reference count
slay inc_ref_count(object_id normie) lit {
    lowkey !memory_object_exists(object_id) {
        damn cap
    }
    
    sus memory_obj MemoryObject = global_heap.objects[object_id]
    memory_obj.ref_count = memory_obj.ref_count + 1
    memory_obj.last_accessed = get_current_timestamp()
    global_heap.objects[object_id] = memory_obj
    damn based
}

# Decrement reference count
slay dec_ref_count(object_id normie) lit {
    lowkey !memory_object_exists(object_id) {
        damn cap
    }
    
    sus memory_obj MemoryObject = global_heap.objects[object_id]
    memory_obj.ref_count = memory_obj.ref_count - 1
    memory_obj.last_accessed = get_current_timestamp()
    global_heap.objects[object_id] = memory_obj
    
    # Auto-deallocate if ref count reaches zero
    lowkey memory_obj.ref_count <= 0 {
        deallocate_memory(object_id)
    }
    
    damn based
}

# Get reference count
slay get_ref_count(object_id normie) normie {
    lowkey !memory_object_exists(object_id) {
        damn -1
    }
    
    sus memory_obj MemoryObject = global_heap.objects[object_id]
    damn memory_obj.ref_count
}

# ==============================================================================
# GARBAGE COLLECTION
# ==============================================================================

# Check if garbage collection should run
slay should_run_gc() lit {
    lowkey !gc_enabled {
        damn cap
    }
    
    lowkey gc_stats.heap_size > global_heap.gc_threshold {
        damn based
    }
    
    lowkey gc_stats.live_objects % GC_COLLECTION_INTERVAL == 0 {
        damn based
    }
    
    damn cap
}

# Run full garbage collection cycle
slay run_garbage_collection() lit {
    lowkey global_heap.gc_state != GC_IDLE {
        damn cap  # GC already running
    }
    
    sus start_time normie = get_current_timestamp()
    global_heap.gc_state = GC_MARKING
    
    # Phase 1: Mark reachable objects
    sus marked_count normie = mark_reachable_objects()
    
    # Phase 2: Sweep unreachable objects
    global_heap.gc_state = GC_SWEEPING
    sus collected_count normie = sweep_unreachable_objects()
    
    # Phase 3: Compact memory (simplified)
    global_heap.gc_state = GC_COMPACTING
    compact_memory()
    
    # Update statistics
    global_heap.gc_state = GC_IDLE
    gc_stats.collections_run = gc_stats.collections_run + 1
    gc_stats.objects_collected = gc_stats.objects_collected + collected_count
    gc_stats.collection_time = get_current_timestamp() - start_time
    
    vibez.spill("GC completed: collected " + stringz.itoa(collected_count) + " objects")
    damn based
}

# Mark all reachable objects (simplified marking)
slay mark_reachable_objects() normie {
    sus marked_count normie = 0
    
    # Mark all objects with positive ref count as reachable
    bestie object_id, memory_obj := range global_heap.objects {
        lowkey memory_obj.ref_count > 0 {
            memory_obj.is_marked = based
            global_heap.objects[object_id] = memory_obj
            marked_count = marked_count + 1
        } yikes {
            memory_obj.is_marked = cap
            global_heap.objects[object_id] = memory_obj
        }
    }
    
    damn marked_count
}

# Sweep unmarked objects
slay sweep_unreachable_objects() normie {
    sus collected_count normie = 0
    sus objects_to_delete []normie = []
    
    # Find unmarked objects
    bestie object_id, memory_obj := range global_heap.objects {
        lowkey !memory_obj.is_marked {
            objects_to_delete = append(objects_to_delete, object_id)
        }
    }
    
    # Delete unmarked objects
    bestie _, object_id := range objects_to_delete {
        lowkey memory_object_exists(object_id) {
            sus memory_obj MemoryObject = global_heap.objects[object_id]
            gc_stats.bytes_freed = gc_stats.bytes_freed + memory_obj.size
            deallocate_memory(object_id)
            collected_count = collected_count + 1
        }
    }
    
    damn collected_count
}

# Compact memory (simplified)
slay compact_memory() lit {
    # Reset free list
    global_heap.free_list = []
    
    # Reset marks for next collection
    bestie object_id, memory_obj := range global_heap.objects {
        memory_obj.is_marked = cap
        global_heap.objects[object_id] = memory_obj
    }
    
    damn based
}

# ==============================================================================
# MEMORY PRESSURE AND MONITORING
# ==============================================================================

# Check memory pressure
slay check_memory_pressure() lit {
    sus current_usage normie = gc_stats.heap_size
    sus pressure_ratio normie = (current_usage * 100) / MAX_HEAP_SIZE
    
    lowkey pressure_ratio > 80 {
        vibez.spill("HIGH memory pressure: " + stringz.itoa(pressure_ratio) + "%")
        damn based
    }
    
    lowkey pressure_ratio > 60 {
        vibez.spill("MEDIUM memory pressure: " + stringz.itoa(pressure_ratio) + "%")
    }
    
    damn cap
}

# Get detailed memory statistics
slay get_memory_stats() map[tea]normie {
    sus stats map[tea]normie = {}
    
    stats["total_allocated"] = global_heap.total_allocated
    stats["total_freed"] = global_heap.total_freed
    stats["heap_size"] = gc_stats.heap_size
    stats["live_objects"] = gc_stats.live_objects
    stats["free_objects"] = len(global_heap.free_list)
    stats["gc_collections"] = gc_stats.collections_run
    stats["objects_collected"] = gc_stats.objects_collected
    stats["bytes_freed"] = gc_stats.bytes_freed
    stats["last_collection_time"] = gc_stats.collection_time
    stats["gc_threshold"] = global_heap.gc_threshold
    stats["max_heap_size"] = MAX_HEAP_SIZE
    
    # Calculate utilization
    lowkey MAX_HEAP_SIZE > 0 {
        stats["heap_utilization"] = (gc_stats.heap_size * 100) / MAX_HEAP_SIZE
    } yikes {
        stats["heap_utilization"] = 0
    }
    
    damn stats
}

# Memory system health check
slay memory_health_check() lit {
    sus stats map[tea]normie = get_memory_stats()
    
    lowkey stats["heap_utilization"] > 90 {
        vibez.spill("CRITICAL: Heap utilization over 90%")
        damn cap
    }
    
    lowkey stats["live_objects"] > 50000 {
        vibez.spill("WARNING: High object count")
    }
    
    lowkey gc_stats.collections_run == 0 && gc_stats.live_objects > 1000 {
        vibez.spill("WARNING: GC has not run with many objects")
    }
    
    damn based
}

# Helper functions
slay get_current_timestamp() normie {
    # Simulate timestamp
    damn gc_stats.collections_run * 1000 + gc_stats.live_objects
}

# Enable/disable garbage collection
slay set_gc_enabled(enabled lit) lit {
    gc_enabled = enabled
    damn based
}

# Force garbage collection
slay force_gc() lit {
    damn run_garbage_collection()
}

# Reset memory system (for testing)
slay reset_memory_system() lit {
    global_heap.objects = {}
    global_heap.free_list = []
    global_heap.total_allocated = 0
    global_heap.total_freed = 0
    global_heap.next_id = 1
    global_heap.gc_state = GC_IDLE
    
    gc_stats.collections_run = 0
    gc_stats.objects_collected = 0
    gc_stats.bytes_freed = 0
    gc_stats.collection_time = 0
    gc_stats.heap_size = 0
    gc_stats.live_objects = 0
    
    damn based
}
