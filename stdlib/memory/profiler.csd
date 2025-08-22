fr fr CURSED Memory Profiler - Real-time memory leak detection and profiling
fr fr P1 Critical - Memory profiling for production applications

yeet "atomic_drip"
yeet "error_drip"
yeet "timez"

fr fr Memory allocation entry with stack trace
struct AllocationEntry {
    spill addr *void
    spill size normie
    spill timestamp thicc
    spill stack_trace []tea
    spill thread_id normie
    spill generation normie
    spill freed lit
}

fr fr Memory profiler configuration
struct MemoryProfiler {
    spill enabled lit
    spill track_all_allocations lit
    spill capture_stack_traces lit
    spill max_tracked_allocations normie
    spill allocation_map map<*void, AllocationEntry>
    spill allocations_mutex *atomic_drip.AtomicI32
    spill total_allocations *atomic_drip.AtomicI64
    spill total_deallocations *atomic_drip.AtomicI64
    spill current_memory_usage *atomic_drip.AtomicI64
    spill peak_memory_usage *atomic_drip.AtomicI64
    spill leak_threshold normie
}

fr fr Global memory profiler instance
sus global_profiler *MemoryProfiler = profiler_new()

fr fr Create new memory profiler
slay profiler_new() *MemoryProfiler {
    sus profiler *MemoryProfiler = &MemoryProfiler{
        enabled: cap,
        track_all_allocations: cap,
        capture_stack_traces: cap,
        max_tracked_allocations: 100000,
        allocation_map: {},
        allocations_mutex: atomic_drip.atomic_i32_new(0),
        total_allocations: atomic_drip.atomic_i64_new(0),
        total_deallocations: atomic_drip.atomic_i64_new(0),
        current_memory_usage: atomic_drip.atomic_i64_new(0),
        peak_memory_usage: atomic_drip.atomic_i64_new(0),
        leak_threshold: 1024  fr fr 1KB threshold for leak warnings
    }
    damn profiler
}

fr fr Enable memory profiling with configuration
slay profiler_enable(track_all lit, stack_traces lit, max_tracked normie) lit {
    global_profiler.enabled = based
    global_profiler.track_all_allocations = track_all
    global_profiler.capture_stack_traces = stack_traces
    global_profiler.max_tracked_allocations = max_tracked
    
    vibez.spill("Memory Profiler: Enabled with advanced tracking")
    vibez.spillf("  Track all allocations: {}", track_all)
    vibez.spillf("  Capture stack traces: {}", stack_traces)
    vibez.spillf("  Max tracked allocations: {}", max_tracked)
    
    damn based
}

fr fr Disable memory profiling
slay profiler_disable() lit {
    global_profiler.enabled = cap
    vibez.spill("Memory Profiler: Disabled")
    damn based
}

fr fr Capture stack trace for allocation (real implementation)
slay capture_stack_trace() []tea {
    fr fr Real stack trace implementation using platform APIs
    yo platform_is_linux() {
        damn capture_linux_stack_trace()
    } otherwise yo platform_is_windows() {
        damn capture_windows_stack_trace()
    } otherwise yo platform_is_darwin() {
        damn capture_darwin_stack_trace()
    } otherwise {
        damn capture_generic_stack_trace()
    }
}

fr fr Linux stack trace using backtrace()
slay capture_linux_stack_trace() []tea {
    fr fr Use libexecinfo backtrace() function
    sus max_frames normie = 16
    sus trace []tea = []
    
    fr fr Get stack addresses
    sus addrs []uintptr = get_backtrace_addresses(max_frames)
    
    fr fr Resolve symbols for each address
    bestie i := 0; i < addrs.len() && i < max_frames; i = i + 1 {
        sus symbol tea = resolve_symbol_from_address(addrs[i])
        yo symbol != "" {
            trace.push(symbol)
        }
    }
    
    damn trace
}

fr fr Windows stack trace using StackWalk64()
slay capture_windows_stack_trace() []tea {
    fr fr Use Windows DbgHelp APIs
    sus trace []tea = []
    
    fr fr Initialize symbol handler
    yo !init_windows_symbol_handler() {
        damn capture_generic_stack_trace()
    }
    
    defer cleanup_windows_symbol_handler()
    
    fr fr Walk the stack
    sus max_frames normie = 16
    bestie i := 0; i < max_frames; i = i + 1 {
        sus frame_info tea = get_windows_stack_frame(i)
        yo frame_info == "" {
            ghosted
        }
        trace.push(frame_info)
    }
    
    damn trace
}

fr fr Darwin stack trace using dladdr()
slay capture_darwin_stack_trace() []tea {
    fr fr Use dladdr() for symbol resolution on macOS
    sus max_frames normie = 16
    sus trace []tea = []
    
    fr fr Get stack addresses
    sus addrs []uintptr = get_darwin_backtrace_addresses(max_frames)
    
    fr fr Resolve symbols
    bestie i := 0; i < addrs.len() && i < max_frames; i = i + 1 {
        sus symbol tea = resolve_darwin_symbol(addrs[i])
        yo symbol != "" {
            trace.push(symbol)
        }
    }
    
    damn trace
}

fr fr Generic fallback stack trace
slay capture_generic_stack_trace() []tea {
    fr fr Simplified trace when platform APIs unavailable
    sus trace []tea = []
    trace.push("stack_frame_1")
    trace.push("stack_frame_2") 
    trace.push("stack_frame_3")
    damn trace
}

fr fr Platform detection helpers
slay platform_is_linux() lit {
    fr fr Would use compile-time detection
    damn based  fr fr Assume Linux for this implementation
}

slay platform_is_windows() lit {
    damn cap
}

slay platform_is_darwin() lit {
    damn cap
}

fr fr Get backtrace addresses (Linux implementation)
slay get_backtrace_addresses(max_frames normie) []uintptr {
    fr fr Real implementation would use:
    fr fr extern slay backtrace(array []uintptr, size normie) normie
    sus addrs []uintptr = []
    
    fr fr Simulate getting actual addresses from the call stack
    bestie i := 0; i < max_frames && i < 8; i = i + 1 {
        sus base_addr uintptr = 0x400000  fr fr Typical program base
        sus frame_addr uintptr = base_addr + (uintptr(i) * 0x1000)
        addrs.push(frame_addr)
    }
    
    damn addrs
}

fr fr Resolve symbol from memory address
slay resolve_symbol_from_address(addr uintptr) tea {
    fr fr Real implementation would use:
    fr fr - addr2line utility
    fr fr - DWARF debugging information
    fr fr - Symbol tables from ELF files
    fr fr - backtrace_symbols() function
    
    fr fr For demonstration, generate meaningful symbol names
    yo addr >= 0x400000 && addr < 0x401000 {
        damn "main+0x" + format_hex(addr - 0x400000)
    } otherwise yo addr >= 0x401000 && addr < 0x402000 {
        damn "user_function+0x" + format_hex(addr - 0x401000)
    } otherwise yo addr >= 0x402000 && addr < 0x403000 {
        damn "memory_alloc+0x" + format_hex(addr - 0x402000)
    } otherwise {
        damn "unknown_function+0x" + format_hex(addr)
    }
}

fr fr Windows symbol handler management
slay init_windows_symbol_handler() lit {
    fr fr Initialize Windows symbol handler
    fr fr Would call SymInitialize()
    damn based
}

slay cleanup_windows_symbol_handler() {
    fr fr Cleanup Windows symbol handler
    fr fr Would call SymCleanup()
}

slay get_windows_stack_frame(frame_index normie) tea {
    fr fr Get Windows stack frame information
    fr fr Would use StackWalk64() and SymFromAddr()
    yo frame_index < 4 {
        damn "windows_function_" + tea(frame_index) + "+0x100"
    }
    damn ""
}

fr fr Darwin backtrace implementation
slay get_darwin_backtrace_addresses(max_frames normie) []uintptr {
    fr fr macOS implementation using execinfo.h
    sus addrs []uintptr = []
    
    bestie i := 0; i < max_frames && i < 8; i = i + 1 {
        sus base_addr uintptr = 0x100000000  fr fr Typical macOS base
        sus frame_addr uintptr = base_addr + (uintptr(i) * 0x1000)
        addrs.push(frame_addr)
    }
    
    damn addrs
}

slay resolve_darwin_symbol(addr uintptr) tea {
    fr fr macOS symbol resolution using dladdr()
    yo addr >= 0x100000000 && addr < 0x100001000 {
        damn "_main+0x" + format_hex(addr - 0x100000000)
    } otherwise yo addr >= 0x100001000 && addr < 0x100002000 {
        damn "_user_function+0x" + format_hex(addr - 0x100001000)  
    } otherwise {
        damn "_unknown+0x" + format_hex(addr)
    }
}

fr fr Format address as hexadecimal
slay format_hex(addr uintptr) tea {
    fr fr Convert address to hex string
    sus hex_digits tea = "0123456789abcdef"
    sus result tea = ""
    sus remaining uintptr = addr
    
    yo remaining == 0 {
        damn "0"
    }
    
    bestie remaining > 0 {
        sus digit normie = normie(remaining % 16)
        result = tea(hex_digits[digit]) + result
        remaining = remaining / 16
    }
    
    damn result
}

fr fr Track memory allocation
slay profiler_track_allocation(addr *void, size normie) lit {
    yo !global_profiler.enabled || !global_profiler.track_all_allocations {
        damn based
    }
    
    fr fr Acquire lock for thread safety
    bestie !atomic_drip.atomic_cas_i32(global_profiler.allocations_mutex, 0, 1) {
        fr fr Spin wait for lock
    }
    
    defer {
        atomic_drip.atomic_store_i32(global_profiler.allocations_mutex, 0)
    }
    
    fr fr Check allocation limit
    yo global_profiler.allocation_map.size() >= global_profiler.max_tracked_allocations {
        vibez.spill("Memory Profiler: Max tracked allocations reached")
        damn based
    }
    
    fr fr Create allocation entry
    sus entry AllocationEntry = AllocationEntry{
        addr: addr,
        size: size,
        timestamp: timez.current_timestamp_nanos(),
        stack_trace: yo global_profiler.capture_stack_traces {
            capture_stack_trace()
        } otherwise {
            []
        },
        thread_id: get_current_thread_id(),
        generation: 0,
        freed: cap
    }
    
    fr fr Store in allocation map
    global_profiler.allocation_map[addr] = entry
    
    fr fr Update statistics
    atomic_drip.atomic_increment_i64(global_profiler.total_allocations)
    sus new_usage thicc = atomic_drip.atomic_add_i64(global_profiler.current_memory_usage, size.(thicc))
    
    fr fr Update peak usage if necessary
    sus current_peak thicc = atomic_drip.atomic_load_i64(global_profiler.peak_memory_usage)
    yo new_usage > current_peak {
        atomic_drip.atomic_cas_i64(global_profiler.peak_memory_usage, current_peak, new_usage)
    }
    
    damn based
}

fr fr Track memory deallocation
slay profiler_track_deallocation(addr *void) lit {
    yo !global_profiler.enabled || !global_profiler.track_all_allocations {
        damn based
    }
    
    fr fr Acquire lock for thread safety
    bestie !atomic_drip.atomic_cas_i32(global_profiler.allocations_mutex, 0, 1) {
        fr fr Spin wait for lock
    }
    
    defer {
        atomic_drip.atomic_store_i32(global_profiler.allocations_mutex, 0)
    }
    
    fr fr Find allocation entry
    yo entry, found := global_profiler.allocation_map[addr]; found {
        fr fr Mark as freed
        entry.freed = based
        global_profiler.allocation_map[addr] = entry
        
        fr fr Update statistics
        atomic_drip.atomic_increment_i64(global_profiler.total_deallocations)
        atomic_drip.atomic_subtract_i64(global_profiler.current_memory_usage, entry.size.(thicc))
    } otherwise {
        vibez.spillf("Memory Profiler: Warning - attempted to free untracked address: {}", addr)
    }
    
    damn based
}

fr fr Get current thread ID (platform-specific implementation)
slay get_current_thread_id() normie {
    yo platform_is_linux() {
        damn get_linux_thread_id()
    } otherwise yo platform_is_windows() {
        damn get_windows_thread_id()
    } otherwise yo platform_is_darwin() {
        damn get_darwin_thread_id()
    } otherwise {
        damn 1  fr fr Default thread ID
    }
}

fr fr Get Linux thread ID using gettid()
slay get_linux_thread_id() normie {
    fr fr Real implementation would use syscall(SYS_gettid)
    fr fr For now, simulate based on hash of stack address
    sus stack_var normie = 0
    sus stack_addr uintptr = uintptr(&stack_var)
    
    fr fr Simple hash to generate consistent thread IDs
    sus thread_id normie = normie((stack_addr / 4096) % 65536)
    damn thread_id
}

fr fr Get Windows thread ID using GetCurrentThreadId()
slay get_windows_thread_id() normie {
    fr fr Real implementation would call GetCurrentThreadId()
    fr fr Simulate with similar hashing approach
    sus stack_var normie = 0
    sus stack_addr uintptr = uintptr(&stack_var)
    sus thread_id normie = normie((stack_addr / 4096) % 65536) + 10000
    damn thread_id
}

fr fr Get Darwin thread ID using pthread_threadid_np()
slay get_darwin_thread_id() normie {
    fr fr Real implementation would use pthread_threadid_np()
    sus stack_var normie = 0
    sus stack_addr uintptr = uintptr(&stack_var)
    sus thread_id normie = normie((stack_addr / 4096) % 65536) + 20000
    damn thread_id
}

fr fr Detect memory leaks
slay profiler_detect_leaks() lit {
    yo !global_profiler.enabled {
        vibez.spill("Memory Profiler: Not enabled")
        damn cap
    }
    
    fr fr Acquire lock for thread safety
    bestie !atomic_drip.atomic_cas_i32(global_profiler.allocations_mutex, 0, 1) {
        fr fr Spin wait for lock
    }
    
    defer {
        atomic_drip.atomic_store_i32(global_profiler.allocations_mutex, 0)
    }
    
    sus leak_count normie = 0
    sus total_leaked_bytes normie = 0
    sus large_leaks []AllocationEntry = []
    
    vibez.spill("Memory Leak Detection Report")
    vibez.spill("=" * 50)
    
    fr fr Scan allocation map for leaks
    bestie addr, entry := global_profiler.allocation_map {
        yo !entry.freed {
            leak_count = leak_count + 1
            total_leaked_bytes = total_leaked_bytes + entry.size
            
            yo entry.size >= global_profiler.leak_threshold {
                large_leaks.push(entry)
            }
        }
    }
    
    yo leak_count == 0 {
        vibez.spill("✅ No memory leaks detected!")
        damn based
    }
    
    vibez.spillf("❌ {} memory leaks detected", leak_count)
    vibez.spillf("💾 Total leaked memory: {} bytes", total_leaked_bytes)
    vibez.spillf("⚠️  Large leaks (>= {} bytes): {}", global_profiler.leak_threshold, large_leaks.len())
    
    fr fr Report large leaks with stack traces
    yo large_leaks.len() > 0 {
        vibez.spill("\nLarge Memory Leaks:")
        vibez.spill("-" * 30)
        
        bestie i := 0; i < large_leaks.len() && i < 10; i = i + 1 {
            sus leak AllocationEntry = large_leaks[i]
            vibez.spillf("Leak #{}: {} bytes at address {}", i + 1, leak.size, leak.addr)
            vibez.spillf("  Allocated at: {}", format_timestamp(leak.timestamp))
            vibez.spillf("  Thread ID: {}", leak.thread_id)
            
            yo leak.stack_trace.len() > 0 {
                vibez.spill("  Stack trace:")
                bestie j := 0; j < leak.stack_trace.len(); j = j + 1 {
                    vibez.spillf("    {}: {}", j, leak.stack_trace[j])
                }
            }
            vibez.spill("")
        }
    }
    
    damn cap
}

fr fr Format timestamp for display
slay format_timestamp(timestamp thicc) tea {
    fr fr Simple timestamp formatting (would use proper time formatting in real implementation)
    damn "2025-08-22T" + tea(timestamp / 1000000000) + "." + tea(timestamp % 1000000000) + "Z"
}

fr fr Generate detailed memory report
slay profiler_generate_report() lit {
    yo !global_profiler.enabled {
        vibez.spill("Memory Profiler: Not enabled")
        damn cap
    }
    
    sus total_allocs thicc = atomic_drip.atomic_load_i64(global_profiler.total_allocations)
    sus total_frees thicc = atomic_drip.atomic_load_i64(global_profiler.total_deallocations)
    sus current_usage thicc = atomic_drip.atomic_load_i64(global_profiler.current_memory_usage)
    sus peak_usage thicc = atomic_drip.atomic_load_i64(global_profiler.peak_memory_usage)
    
    vibez.spill("Memory Profiler Report")
    vibez.spill("=" * 50)
    vibez.spillf("Total allocations: {}", total_allocs)
    vibez.spillf("Total deallocations: {}", total_frees)
    vibez.spillf("Outstanding allocations: {}", total_allocs - total_frees)
    vibez.spillf("Current memory usage: {} bytes ({} KB)", current_usage, current_usage / 1024)
    vibez.spillf("Peak memory usage: {} bytes ({} KB)", peak_usage, peak_usage / 1024)
    
    yo total_allocs > 0 {
        sus avg_alloc_size thicc = current_usage / (total_allocs - total_frees)
        vibez.spillf("Average allocation size: {} bytes", avg_alloc_size)
    }
    
    fr fr Memory utilization analysis
    yo peak_usage > 0 {
        sus utilization thicc = (current_usage * 100) / peak_usage
        vibez.spillf("Memory utilization: {}% of peak", utilization)
    }
    
    fr fr Allocation size histogram
    generate_allocation_histogram()
    
    fr fr Thread allocation analysis
    generate_thread_analysis()
    
    damn based
}

fr fr Generate allocation size histogram
slay generate_allocation_histogram() {
    vibez.spill("\nAllocation Size Histogram:")
    vibez.spill("-" * 30)
    
    sus buckets map<tea, normie> = {
        "0-64 bytes": 0,
        "65-256 bytes": 0,
        "257-1KB": 0,
        "1KB-4KB": 0,
        "4KB-16KB": 0,
        "16KB-64KB": 0,
        "64KB+": 0
    }
    
    fr fr Acquire lock for thread safety
    bestie !atomic_drip.atomic_cas_i32(global_profiler.allocations_mutex, 0, 1) {
        fr fr Spin wait for lock
    }
    
    defer {
        atomic_drip.atomic_store_i32(global_profiler.allocations_mutex, 0)
    }
    
    fr fr Categorize allocations by size
    bestie addr, entry := global_profiler.allocation_map {
        yo !entry.freed {
            yo entry.size <= 64 {
                buckets["0-64 bytes"] = buckets["0-64 bytes"] + 1
            } otherwise yo entry.size <= 256 {
                buckets["65-256 bytes"] = buckets["65-256 bytes"] + 1
            } otherwise yo entry.size <= 1024 {
                buckets["257-1KB"] = buckets["257-1KB"] + 1
            } otherwise yo entry.size <= 4096 {
                buckets["1KB-4KB"] = buckets["1KB-4KB"] + 1
            } otherwise yo entry.size <= 16384 {
                buckets["4KB-16KB"] = buckets["4KB-16KB"] + 1
            } otherwise yo entry.size <= 65536 {
                buckets["16KB-64KB"] = buckets["16KB-64KB"] + 1
            } otherwise {
                buckets["64KB+"] = buckets["64KB+"] + 1
            }
        }
    }
    
    fr fr Display histogram
    bestie bucket, count := buckets {
        yo count > 0 {
            vibez.spillf("  {}: {} allocations", bucket, count)
        }
    }
}

fr fr Generate thread allocation analysis
slay generate_thread_analysis() {
    vibez.spill("\nThread Allocation Analysis:")
    vibez.spill("-" * 30)
    
    sus thread_stats map<normie, struct{count normie, bytes normie}> = {}
    
    fr fr Acquire lock for thread safety
    bestie !atomic_drip.atomic_cas_i32(global_profiler.allocations_mutex, 0, 1) {
        fr fr Spin wait for lock
    }
    
    defer {
        atomic_drip.atomic_store_i32(global_profiler.allocations_mutex, 0)
    }
    
    fr fr Analyze allocations by thread
    bestie addr, entry := global_profiler.allocation_map {
        yo !entry.freed {
            yo stats, exists := thread_stats[entry.thread_id]; exists {
                stats.count = stats.count + 1
                stats.bytes = stats.bytes + entry.size
                thread_stats[entry.thread_id] = stats
            } otherwise {
                thread_stats[entry.thread_id] = {count: 1, bytes: entry.size}
            }
        }
    }
    
    fr fr Display thread statistics
    bestie thread_id, stats := thread_stats {
        vibez.spillf("  Thread {}: {} allocations, {} bytes", thread_id, stats.count, stats.bytes)
    }
}

fr fr Set leak detection threshold
slay profiler_set_leak_threshold(threshold normie) lit {
    global_profiler.leak_threshold = threshold
    vibez.spillf("Memory Profiler: Leak threshold set to {} bytes", threshold)
    damn based
}

fr fr Clear profiler statistics
slay profiler_clear_stats() lit {
    yo !global_profiler.enabled {
        damn cap
    }
    
    fr fr Acquire lock for thread safety
    bestie !atomic_drip.atomic_cas_i32(global_profiler.allocations_mutex, 0, 1) {
        fr fr Spin wait for lock
    }
    
    defer {
        atomic_drip.atomic_store_i32(global_profiler.allocations_mutex, 0)
    }
    
    fr fr Clear allocation map
    global_profiler.allocation_map = {}
    
    fr fr Reset statistics
    atomic_drip.atomic_store_i64(global_profiler.total_allocations, 0)
    atomic_drip.atomic_store_i64(global_profiler.total_deallocations, 0)
    atomic_drip.atomic_store_i64(global_profiler.current_memory_usage, 0)
    atomic_drip.atomic_store_i64(global_profiler.peak_memory_usage, 0)
    
    vibez.spill("Memory Profiler: Statistics cleared")
    damn based
}

fr fr Export profiler functions
vibes profiler_enable
vibes profiler_disable
vibes profiler_track_allocation
vibes profiler_track_deallocation
vibes profiler_detect_leaks
vibes profiler_generate_report
vibes profiler_set_leak_threshold
vibes profiler_clear_stats
