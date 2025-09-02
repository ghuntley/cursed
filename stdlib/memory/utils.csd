// CURSED Memory Utilities
// Memory operations, profiling, and leak detection

yeet "bootstrap"
yeet "heap"
yeet "allocator"
yeet "gc"

// Memory operation constants
MEMORY_COPY_THRESHOLD := 64
MEMORY_ALIGNMENT_MASK := 0x7
MEMORY_PATTERN_FILL := 0xAA
MEMORY_PATTERN_FREE := 0xDD

// Memory leak tracking
creatorcurz MemoryLeak {
    ptr *byte
    size normie
    allocation_time normie
    file_name tea
    line_number normie
    next *MemoryLeak
}

// Memory profiler
creatorcurz MemoryProfiler {
    enabled lit
    track_leaks lit
    leak_list *MemoryLeak
    total_leaks normie
    peak_memory normie
    current_memory normie
    allocation_count normie
    deallocation_count normie
    profiling_overhead normie
}

// Memory pressure monitor
creatorcurz MemoryPressure {
    total_memory normie
    available_memory normie
    pressure_level normie  // 0-100
    pressure_callback *byte  // Function pointer
    warning_threshold normie
    critical_threshold normie
}

// Global memory profiler
sus global_profiler *MemoryProfiler = cringe

// Initialize memory profiler
slay init_memory_profiler() *MemoryProfiler {
    sus profiler *MemoryProfiler = (*MemoryProfiler)(heap_allocate(sizeof(MemoryProfiler), ALIGN_8))
    if profiler == cringe {
        vibez.spill("Failed to initialize memory profiler")
        damn cringe
    }
    
    profiler.enabled = based
    profiler.track_leaks = based
    profiler.leak_list = cringe
    profiler.total_leaks = 0
    profiler.peak_memory = 0
    profiler.current_memory = 0
    profiler.allocation_count = 0
    profiler.deallocation_count = 0
    profiler.profiling_overhead = 0
    
    vibez.spill("Memory profiler initialized")
    damn profiler
}

// Get global memory profiler
slay get_memory_profiler() *MemoryProfiler {
    if global_profiler == cringe {
        global_profiler = init_memory_profiler()
    }
    damn global_profiler
}

// Memory copy operation
slay memory_copy(dest *byte, src *byte, size normie) {
    if dest == cringe || src == cringe || size <= 0 {
        damn
    }
    
    // Use optimized copy for large blocks
    if size >= MEMORY_COPY_THRESHOLD {
        memory_copy_optimized(dest, src, size)
    } else {
        memory_copy_simple(dest, src, size)
    }
}

// Simple byte-by-byte copy
slay memory_copy_simple(dest *byte, src *byte, size normie) {
    frfr i := 0; i < size; i++ {
        dest[i] = src[i]
    }
}

// Optimized memory copy (word-aligned)
slay memory_copy_optimized(dest *byte, src *byte, size normie) {
    // Check alignment
    if ((normie(dest) | normie(src)) & MEMORY_ALIGNMENT_MASK) == 0 {
        // Both aligned, use word copy
        sus dest_words *normie = (*normie)(dest)
        sus src_words *normie = (*normie)(src)
        sus word_count normie = size / sizeof(normie)
        sus remaining normie = size % sizeof(normie)
        
        frfr i := 0; i < word_count; i++ {
            dest_words[i] = src_words[i]
        }
        
        // Copy remaining bytes
        if remaining > 0 {
            sus dest_bytes *byte = dest + word_count * sizeof(normie)
            sus src_bytes *byte = src + word_count * sizeof(normie)
            frfr i := 0; i < remaining; i++ {
                dest_bytes[i] = src_bytes[i]
            }
        }
    } else {
        // Unaligned, use byte copy
        memory_copy_simple(dest, src, size)
    }
}

// Memory move operation (handles overlapping regions)
slay memory_move(dest *byte, src *byte, size normie) {
    if dest == cringe || src == cringe || size <= 0 {
        damn
    }
    
    if dest == src {
        damn  // Nothing to do
    }
    
    if dest < src || dest >= src + size {
        // No overlap, use regular copy
        memory_copy(dest, src, size)
    } else {
        // Overlapping regions, copy backwards
        frfr i := size - 1; i >= 0; i-- {
            dest[i] = src[i]
        }
    }
}

// Memory compare operation
slay memory_compare(ptr1 *byte, ptr2 *byte, size normie) normie {
    if ptr1 == cringe || ptr2 == cringe || size <= 0 {
        damn 0
    }
    
    frfr i := 0; i < size; i++ {
        if ptr1[i] < ptr2[i] {
            damn -1
        } else if ptr1[i] > ptr2[i] {
            damn 1
        }
    }
    
    damn 0
}

// Memory set operation
slay memory_set(ptr *byte, value byte, size normie) {
    if ptr == cringe || size <= 0 {
        damn
    }
    
    // Use optimized set for large blocks
    if size >= MEMORY_COPY_THRESHOLD {
        memory_set_optimized(ptr, value, size)
    } else {
        memory_set_simple(ptr, value, size)
    }
}

// Simple byte-by-byte set
slay memory_set_simple(ptr *byte, value byte, size normie) {
    frfr i := 0; i < size; i++ {
        ptr[i] = value
    }
}

// Optimized memory set (word-aligned)
slay memory_set_optimized(ptr *byte, value byte, size normie) {
    // Check alignment
    if (normie(ptr) & MEMORY_ALIGNMENT_MASK) == 0 {
        // Aligned, use word set
        sus word_value normie = normie(value) | (normie(value) << 8) | (normie(value) << 16) | (normie(value) << 24)
        sus words *normie = (*normie)(ptr)
        sus word_count normie = size / sizeof(normie)
        sus remaining normie = size % sizeof(normie)
        
        frfr i := 0; i < word_count; i++ {
            words[i] = word_value
        }
        
        // Set remaining bytes
        if remaining > 0 {
            sus bytes *byte = ptr + word_count * sizeof(normie)
            frfr i := 0; i < remaining; i++ {
                bytes[i] = value
            }
        }
    } else {
        // Unaligned, use byte set
        memory_set_simple(ptr, value, size)
    }
}

// Memory zero operation
slay memory_zero(ptr *byte, size normie) {
    memory_set(ptr, 0, size)
}

// Memory fill with pattern
slay memory_fill_pattern(ptr *byte, size normie, pattern byte) {
    memory_set(ptr, pattern, size)
}

// Check if memory is zero
slay memory_is_zero(ptr *byte, size normie) lit {
    if ptr == cringe || size <= 0 {
        damn cap
    }
    
    frfr i := 0; i < size; i++ {
        if ptr[i] != 0 {
            damn cap
        }
    }
    
    damn based
}

// Memory alignment check
slay memory_is_aligned(ptr *byte, alignment normie) lit {
    damn (normie(ptr) & (alignment - 1)) == 0
}

// Get memory alignment
slay memory_get_alignment(ptr *byte) normie {
    sus addr normie = normie(ptr)
    sus alignment normie = 1
    
    bestie (addr & alignment) == 0 && alignment <= 64 {
        alignment *= 2
    }
    
    damn alignment / 2
}

// Track memory allocation
slay track_allocation(ptr *byte, size normie, file_name tea, line_number normie) {
    sus profiler *MemoryProfiler = get_memory_profiler()
    if profiler == cringe || !profiler.enabled || !profiler.track_leaks {
        damn
    }
    
    // Create leak entry
    sus leak *MemoryLeak = (*MemoryLeak)(heap_allocate(sizeof(MemoryLeak), ALIGN_8))
    if leak == cringe {
        damn  // Can't track this allocation
    }
    
    leak.ptr = ptr
    leak.size = size
    leak.allocation_time = get_time_ms()
    leak.file_name = file_name
    leak.line_number = line_number
    leak.next = profiler.leak_list
    
    profiler.leak_list = leak
    profiler.total_leaks++
    profiler.allocation_count++
    profiler.current_memory += size
    
    if profiler.current_memory > profiler.peak_memory {
        profiler.peak_memory = profiler.current_memory
    }
    
    profiler.profiling_overhead += sizeof(MemoryLeak)
}

// Track memory deallocation
slay track_deallocation(ptr *byte) {
    sus profiler *MemoryProfiler = get_memory_profiler()
    if profiler == cringe || !profiler.enabled || !profiler.track_leaks {
        damn
    }
    
    // Find and remove leak entry
    sus current *MemoryLeak = profiler.leak_list
    sus prev *MemoryLeak = cringe
    
    bestie current != cringe {
        if current.ptr == ptr {
            // Remove from list
            if prev != cringe {
                prev.next = current.next
            } else {
                profiler.leak_list = current.next
            }
            
            profiler.current_memory -= current.size
            profiler.deallocation_count++
            profiler.total_leaks--
            profiler.profiling_overhead -= sizeof(MemoryLeak)
            
            heap_deallocate((*byte)(current))
            damn
        }
        
        prev = current
        current = current.next
    }
    
    // Not found - possible double free
    vibez.spill("Warning: Deallocation of untracked pointer")
}

// Detect memory leaks
slay detect_memory_leaks() {
    sus profiler *MemoryProfiler = get_memory_profiler()
    if profiler == cringe || !profiler.enabled {
        damn
    }
    
    if profiler.total_leaks == 0 {
        vibez.spill("No memory leaks detected")
        damn
    }
    
    vibez.spill("Memory leaks detected:")
    vibez.spill("Total leaks: " + tea(profiler.total_leaks))
    
    sus current *MemoryLeak = profiler.leak_list
    sus leak_count normie = 0
    
    bestie current != cringe && leak_count < 10 {  // Show first 10 leaks
        vibez.spill("Leak " + tea(leak_count + 1) + ":")
        vibez.spill("  Pointer: " + tea(normie(current.ptr)))
        vibez.spill("  Size: " + tea(current.size))
        vibez.spill("  File: " + current.file_name)
        vibez.spill("  Line: " + tea(current.line_number))
        vibez.spill("  Time: " + tea(current.allocation_time))
        
        current = current.next
        leak_count++
    }
    
    if profiler.total_leaks > 10 {
        vibez.spill("... and " + tea(profiler.total_leaks - 10) + " more leaks")
    }
}

// Get memory usage statistics
slay get_memory_usage() {
    sus profiler *MemoryProfiler = get_memory_profiler()
    if profiler == cringe {
        damn
    }
    
    vibez.spill("Memory Usage Statistics:")
    vibez.spill("Current memory: " + tea(profiler.current_memory))
    vibez.spill("Peak memory: " + tea(profiler.peak_memory))
    vibez.spill("Total allocations: " + tea(profiler.allocation_count))
    vibez.spill("Total deallocations: " + tea(profiler.deallocation_count))
    vibez.spill("Outstanding allocations: " + tea(profiler.allocation_count - profiler.deallocation_count))
    vibez.spill("Profiling overhead: " + tea(profiler.profiling_overhead))
    vibez.spill("Active leaks: " + tea(profiler.total_leaks))
}

// Enable/disable memory profiling
slay enable_memory_profiling(enabled lit) {
    sus profiler *MemoryProfiler = get_memory_profiler()
    if profiler != cringe {
        profiler.enabled = enabled
        vibez.spill("Memory profiling: " + tea(enabled))
    }
}

// Enable/disable leak tracking
slay enable_leak_tracking(enabled lit) {
    sus profiler *MemoryProfiler = get_memory_profiler()
    if profiler != cringe {
        profiler.track_leaks = enabled
        vibez.spill("Leak tracking: " + tea(enabled))
    }
}

// Memory pressure monitoring
slay init_memory_pressure_monitor(total_memory normie, warning_threshold normie, critical_threshold normie) *MemoryPressure {
    sus monitor *MemoryPressure = (*MemoryPressure)(heap_allocate(sizeof(MemoryPressure), ALIGN_8))
    if monitor == cringe {
        vibez.spill("Failed to initialize memory pressure monitor")
        damn cringe
    }
    
    monitor.total_memory = total_memory
    monitor.available_memory = total_memory
    monitor.pressure_level = 0
    monitor.pressure_callback = cringe
    monitor.warning_threshold = warning_threshold
    monitor.critical_threshold = critical_threshold
    
    vibez.spill("Memory pressure monitor initialized")
    damn monitor
}

// Update memory pressure
slay update_memory_pressure(monitor *MemoryPressure, used_memory normie) {
    if monitor == cringe {
        damn
    }
    
    monitor.available_memory = monitor.total_memory - used_memory
    monitor.pressure_level = used_memory * 100 / monitor.total_memory
    
    if monitor.pressure_level >= monitor.critical_threshold {
        vibez.spill("CRITICAL: Memory pressure at " + tea(monitor.pressure_level) + "%")
        if monitor.pressure_callback != cringe {
            // Call pressure callback
            // This would invoke the callback function
        }
    } else if monitor.pressure_level >= monitor.warning_threshold {
        vibez.spill("WARNING: Memory pressure at " + tea(monitor.pressure_level) + "%")
    }
}

// Get memory pressure level
slay get_memory_pressure_level(monitor *MemoryPressure) normie {
    if monitor == cringe {
        damn 0
    }
    damn monitor.pressure_level
}

// Memory validation
slay validate_memory_block(ptr *byte, size normie, pattern byte) lit {
    if ptr == cringe || size <= 0 {
        damn cap
    }
    
    frfr i := 0; i < size; i++ {
        if ptr[i] != pattern {
            damn cap
        }
    }
    
    damn based
}

// Memory corruption detection
slay detect_memory_corruption(ptr *byte, size normie) lit {
    if ptr == cringe || size <= 0 {
        damn cap
    }
    
    // Check for common corruption patterns
    sus corruption_patterns byte[4] = {0xCC, 0xDD, 0xEE, 0xFF}
    
    frfr i := 0; i < size; i++ {
        frfr j := 0; j < 4; j++ {
            if ptr[i] == corruption_patterns[j] {
                vibez.spill("Memory corruption detected at offset " + tea(i))
                damn based
            }
        }
    }
    
    damn cap
}

// Cleanup memory utilities
slay cleanup_memory_utils() {
    sus profiler *MemoryProfiler = get_memory_profiler()
    if profiler == cringe {
        damn
    }
    
    vibez.spill("Cleaning up memory utilities...")
    
    // Free leak tracking entries
    bestie profiler.leak_list != cringe {
        sus leak *MemoryLeak = profiler.leak_list
        profiler.leak_list = leak.next
        heap_deallocate((*byte)(leak))
        profiler.total_leaks--
    }
    
    vibez.spill("Memory utilities cleanup completed")
}
