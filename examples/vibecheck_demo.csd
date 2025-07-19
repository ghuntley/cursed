fr fr CURSED Runtime Introspection (vibecheck) Demo
fr fr This example demonstrates the runtime introspection capabilities

yeet "stdlib::vibecheck"
yeet "stdlib::io"

fr fr Memory statistics example
slay show_memory_stats() {
    var m vibecheck.MemStats;
    vibecheck.read_mem_stats(&m);
    
    io.println(&format!("=== Memory Statistics ==="));
    io.println(&format!("Allocated: {} KB", m.alloc / 1024));
    io.println(&format!("Total Allocated: {} KB", m.total_alloc / 1024));
    io.println(&format!("System Memory: {} KB", m.sys / 1024));
    io.println(&format!("Garbage Collections: {}", m.num_gc));
    io.println(&format!("GC CPU Fraction: {:.4}", m.gc_cpu_fraction));
    io.println(&format!(""));
}

fr fr Goroutine information example
slay show_goroutine_info() {
    io.println(&format!("=== Goroutine Information ==="));
    io.println(&format!("Number of goroutines: {}", vibecheck.num_goroutine()));
    io.println(&format!("Current goroutine ID: {}", vibecheck.go_id()));
    io.println(&format!("Max PROCS: {}", vibecheck.gomaxprocs(0)));
    io.println(&format!("Number of CPUs: {}", vibecheck.num_cpu()));
    io.println(&format!(""));
}

fr fr Version and platform information
slay show_version_info() {
    io.println(&format!("=== Version Information ==="));
    io.println(&format!("CURSED Version: {}", vibecheck.version()));
    io.println(&format!("Compiler: {}", vibecheck.compiler()));
    io.println(&format!("Architecture: {}", vibecheck.goarch()));
    io.println(&format!("Operating System: {}", vibecheck.goos()));
    io.println(&format!(""));
}

fr fr Stack trace example
slay show_stack_trace() {
    io.println(&format!("=== Stack Trace ==="));
    let stack = vibecheck.stack();
    io.println(&format!("{}", String.from_utf8_lossy(&stack)));
}

fr fr Garbage collection control example
slay gc_control_demo() {
    io.println(&format!("=== Garbage Collection Control ==="));
    
    fr fr Show current GC settings
    let current_percent = vibecheck.get_gc_percent();
    io.println(&format!("Current GC percent: {}", current_percent));
    
    fr fr Allocate some memory
    io.println(&format!("Allocating 10MB of memory..."));
    let data = vec![0u8; 10 * 1024 * 1024];
    
    fr fr Show memory before GC
    show_memory_stats();
    
    fr fr Trigger garbage collection
    io.println(&format!("Triggering garbage collection..."));
    vibecheck.run_gc();
    
    fr fr Show memory after GC
    show_memory_stats();
    
    fr fr Set GC percentage
    let old_percent = vibecheck.set_gc_percent(200);
    io.println(&format!("Changed GC percent from {} to 200", old_percent));
    
    fr fr Restore original setting
    vibecheck.set_gc_percent(old_percent);
    io.println(&format!("Restored GC percent to {}", old_percent));
    io.println(&format!(""));
}

fr fr Runtime metrics example
slay show_runtime_metrics() {
    io.println(&format!("=== Runtime Metrics ==="));
    
    let metrics = vibecheck.get_metrics();
    io.println(&format!("Memory used: {} KB", metrics.memory_used / 1024));
    io.println(&format!("Goroutine count: {}", metrics.goroutine_count));
    io.println(&format!("CPU utilization: {:.2}%", metrics.cpu_utilization * 100.0));
    io.println(&format!("GC cycles: {}", metrics.gc_stats.cycles));
    io.println(&format!(""));
}

fr fr Memory profiling example
slay memory_profiling_demo() {
    io.println(&format!("=== Memory Profiling ==="));
    
    fr fr Create memory profile
    let profile = vibecheck.memory_profile();
    let profile_text = vibecheck.write_profile(&profile);
    
    io.println(&format!("{}", profile_text));
}

fr fr Platform information example
slay show_platform_info() {
    io.println(&format!("=== Platform Information ==="));
    
    let build = vibecheck.build_info();
    io.println(&format!("Build Version: {}", build.version));
    io.println(&format!("Build Architecture: {}", build.architecture));
    io.println(&format!("Build OS: {}", build.os));
    io.println(&format!("LLVM Version: {}", build.llvm_version));
    io.println(&format!("Rust Version: {}", build.rust_version));
    
    let features = vibecheck.runtime_features();
    io.println(&format!("GC Enabled: {}", features.gc_enabled));
    io.println(&format!("JIT Enabled: {}", features.jit_enabled));
    io.println(&format!("Goroutines Enabled: {}", features.goroutines_enabled));
    
    let layout = vibecheck.memory_layout();
    io.println(&format!("Pointer Size: {} bytes", layout.pointer_size));
    io.println(&format!("Page Size: {} bytes", layout.page_size));
    io.println(&format!("Endianness: {}", layout.endianness));
    io.println(&format!(""));
}

fr fr Caller information example
slay caller_info_example() {
    io.println(&format!("=== Caller Information ==="));
    
    let (pc, file, line, ok) = vibecheck.caller(0);
    if ok {
        let func_info = vibecheck.func_for_pc(pc);
        io.println(&format!("Called from: {}", func_info.name()));
        io.println(&format!("File: {} (line {})", file, line));
        io.println(&format!("Entry point: 0x{:x}", func_info.entry()));
    } else {
        io.println(&format!("No caller information available"));
    }
    io.println(&format!(""));
}

fr fr Resource control example
slay resource_control_demo() {
    io.println(&format!("=== Resource Control ==="));
    
    fr fr Set memory limit (1GB)
    vibecheck.set_memory_limit(1024 * 1024 * 1024);
    let limit = vibecheck.get_memory_limit();
    if let Some(limit_bytes) = limit {
        io.println(&format!("Memory limit set to: {} MB", limit_bytes / (1024 * 1024)));
    }
    
    fr fr Set CPU profiling rate
    vibecheck.set_cpu_profile_rate(100);
    let rate = vibecheck.get_cpu_profile_rate();
    if let Some(samples_per_sec) = rate {
        io.println(&format!("CPU profile rate: {} samples/sec", samples_per_sec));
    }
    
    io.println(&format!(""));
}

fr fr Performance measurement example
slay performance_demo() {
    io.println(&format!("=== Performance Measurement ==="));
    
    fr fr Start CPU profiling
    let profile = vibecheck.cpu_profile();
    
    fr fr Do some CPU-intensive work
    io.println(&format!("Running CPU-intensive task..."));
    for i in 0..1000000 {
        let _ = i * i;
    }
    
    fr fr Stop profiling and show results
    let elapsed = profile.stop();
    io.println(&format!("Task completed in: {:?}", elapsed));
    
    io.println(&format!(""));
}

fr fr Main demo function
slay main() {
    io.println(&format!("CURSED Runtime Introspection (vibecheck) Demo"));
    io.println(&format!("============================================="));
    io.println(&format!(""));
    
    fr fr Initialize scheduler for goroutine features
    vibecheck.init_scheduler();
    
    fr fr Run all demonstrations
    show_version_info();
    show_platform_info();
    show_memory_stats();
    show_goroutine_info();
    show_runtime_metrics();
    caller_info_example();
    gc_control_demo();
    memory_profiling_demo();
    resource_control_demo();
    performance_demo();
    show_stack_trace();
    
    io.println(&format!("Demo completed successfully!"));
}

fr fr Advanced goroutine monitoring example
slay goroutine_monitoring_demo() {
    io.println(&format!("=== Goroutine Monitoring ==="));
    
    fr fr Get information about all goroutines
    let goroutines = vibecheck.get_all_goroutine_info();
    for (id, info) in goroutines {
        io.println(&format!("Goroutine {}: {:?} (runtime: {:?})", 
                          id, info.state, info.runtime));
    }
    
    fr fr Enable blocking profile
    vibecheck.block_profile(based);
    
    fr fr Get specific goroutine info
    let current_id = vibecheck.go_id() as u64;
    if let Some(info) = vibecheck.goroutine_info(current_id) {
        io.println(&format!("Current goroutine info: stack_size={}", info.stack_size));
    }
    
    fr fr Coordinate with GC
    let success = vibecheck.coordinate_gc(1000); // 1 second timeout
    io.println(&format!("GC coordination successful: {}", success));
    
    io.println(&format!(""));
}

fr fr JIT compiler introspection example
slay jit_introspection_demo() {
    io.println(&format!("=== JIT Compiler Introspection ==="));
    
    fr fr Get JIT statistics
    let jit_stats = vibecheck.jit_stats();
    io.println(&format!("Functions compiled: {}", jit_stats.functions_compiled));
    io.println(&format!("Compilation time: {} ns", jit_stats.compilation_time));
    io.println(&format!("Code cache size: {} KB", jit_stats.code_cache_size / 1024));
    
    fr fr Set optimization level
    let old_level = vibecheck.set_jit_opt_level(2);
    io.println(&format!("Changed JIT optimization level from {} to 2", old_level));
    
    io.println(&format!(""));
}

fr fr Free memory to OS example
slay memory_cleanup_demo() {
    io.println(&format!("=== Memory Cleanup ==="));
    
    fr fr Show memory before cleanup
    show_memory_stats();
    
    fr fr Free memory to OS
    io.println(&format!("Freeing memory to operating system..."));
    vibecheck.free_os_memory();
    
    fr fr Show memory after cleanup
    show_memory_stats();
}
