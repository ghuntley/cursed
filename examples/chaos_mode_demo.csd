fr fr ChaosMode Demo - CURSED Runtime System Package
fr fr Demonstrates comprehensive runtime management, profiling, and system introspection

yeet "stdlib::chaos_mode"
yeet "stdlib::io"
yeet "stdlib::json_tea"

slay main_character() {
    println("🔥 CURSED ChaosMode Runtime Demo 🔥")?;
    println("=====================================")?;
    
    fr fr Initialize the ChaosMode system
    initialize_chaos_mode()?;
    
    fr fr Basic runtime information
    spill_basic_info()?;
    
    fr fr Memory management demonstration
    demo_memory_management()?;
    
    fr fr Goroutine management demonstration
    demo_goroutine_management()?;
    
    fr fr Profiling and tracing demonstration
    demo_profiling_tracing()?;
    
    fr fr Enhanced features demonstration
    demo_enhanced_features()?;
    
    fr fr Performance tuning demonstration
    demo_performance_tuning()?;
    
    fr fr Comprehensive stats summary
    spill_chaos_stats()?;
    
    fr fr Cleanup
    cleanup_chaos_mode()?;
    
    println("\n✨ ChaosMode demo complete! ✨")?;
}

slay spill_basic_info() {
    println("\n📊 Basic Runtime Information")?;
    println("----------------------------")?;
    
    sus version_info = version()?;
    sus arch_info = goarch()?;
    sus os_info = goos()?;
    sus compiler_info = compiler()?;
    sus root_path = goroot()?;
    sus cpu_count = num_cpu()?;
    sus goroutine_count = num_goroutine()?;
    sus current_procs = gomaxprocs(0)?;
    
    printf("🏗️  CURSED Version: {}\n", &[version_info])?;
    printf("💻 Architecture: {}\n", &[arch_info])?;
    printf("🖥️  Operating System: {}\n", &[os_info])?;
    printf("⚙️  Compiler: {}\n", &[compiler_info])?;
    printf("📁 CURSED Root: {}\n", &[root_path])?;
    printf("🔢 CPU Cores: {}\n", &[cpu_count])?;
    printf("🚀 Goroutines: {}\n", &[goroutine_count])?;
    printf("⚡ GOMAXPROCS: {}\n", &[current_procs])?;
}

slay demo_memory_management() {
    println("\n🧠 Memory Management Demo")?;
    println("-------------------------")?;
    
    fr fr Get memory statistics
    sus mem_stats = mem_stats()?;
    
    printf("💾 Allocated Memory: {} bytes\n", &[mem_stats.alloc])?;
    printf("📈 Total Allocated: {} bytes\n", &[mem_stats.total_alloc])?;
    printf("🏠 Heap Objects: {}\n", &[mem_stats.heap_objects])?;
    printf("🗑️  GC Cycles: {}\n", &[mem_stats.num_gc])?;
    printf("⏱️  GC CPU Usage: {:.2}%\n", &[mem_stats.gc_cpu_fraction * 100.0])?;
    
    fr fr Trigger garbage collection
    println("🧹 Running garbage collection...")?;
    gc()?;
    
    fr fr Get allocation histogram
    sus histogram = allocation_size_histogram()?;
    println("📊 Allocation Size Histogram (top 5):")?;
    sus count = 0;
    lowkey (sus size, sus alloc_count) in histogram {
        if count < 5 {
            printf("  {} bytes: {} allocations\n", &[size, alloc_count])?;
            count = count + 1;
        }
    }
    
    fr fr Get top allocated types
    sus top_types = top_allocated_types(3)?;
    println("🏆 Top Allocated Types:")?;
    lowkey (sus i, sus type_info) in top_types.enumerate() {
        printf("  {}. {} - {} bytes in {} objects\n", 
               &[i + 1, type_info.type_name, type_info.total_size, type_info.count])?;
    }
    
    fr fr Test memory debugging
    sus test_ptr = &42u8 as *const u8;
    sus is_valid = is_valid_pointer(test_ptr)?;
    printf("🔍 Pointer validity check: {}\n", &[is_valid])?;
    
    if is_valid {
        sus obj_size = get_object_size(test_ptr)?;
        sus ptr_info = get_pointer_info(test_ptr)?;
        printf("📏 Object size: {} bytes\n", &[obj_size])?;
        printf("🎯 Pointer address: 0x{:x}\n", &[ptr_info.address])?;
    }
}

slay demo_goroutine_management() {
    println("\n🚀 Goroutine Management Demo")?;
    println("----------------------------")?;
    
    fr fr Get current stack trace
    sus trace = stack_trace()?;
    printf("📜 Current stack trace:\n{}\n", &[trace])?;
    
    fr fr Get all goroutine IDs
    sus goroutine_ids = all_goroutine_ids()?;
    printf("🆔 Active goroutine IDs: {:?}\n", &[goroutine_ids])?;
    
    fr fr Get stack traces for all goroutines
    sus all_stacks = all_goroutine_stacks()?;
    println("📚 All goroutine stacks (JSON):")?;
    println(all_stacks)?;
    
    fr fr Capture call stack
    sus pc = [0usize; 5];
    sus frame_count = callers(1, &mut pc)?;
    printf("🔗 Captured {} stack frames\n", &[frame_count])?;
    
    lowkey sus i in 0..frame_count {
        sus (file, line) = pc_to_file_and_line(pc[i as usize])?;
        sus func_name = pc_to_func_name(pc[i as usize])?;
        printf("  Frame {}: {}() at {}:{}\n", &[i, func_name, file, line])?;
    }
    
    fr fr Set goroutine labels
    set_goroutine_label("purpose".to_string(), "demo".to_string())?;
    set_goroutine_label("component".to_string(), "chaos_mode".to_string())?;
    println("🏷️  Set goroutine labels: purpose=demo, component=chaos_mode")?;
    
    fr fr Get goroutines by state
    sus running_goroutines = goroutines_by_state("running".to_string())?;
    printf("🏃 Running goroutines: {:?}\n", &[running_goroutines])?;
    
    fr fr Get detailed info for first goroutine
    if !running_goroutines.is_empty() {
        sus goroutine_id = running_goroutines[0];
        sus info = goroutine_info(goroutine_id)?;
        printf("📝 Goroutine {} details:\n", &[goroutine_id])?;
        printf("  State: {}\n", &[info.state])?;
        printf("  CPU Time: {:?}\n", &[info.cpu_time])?;
        printf("  Created By: {}\n", &[info.created_by])?;
    }
}

slay demo_profiling_tracing() {
    println("\n📊 Profiling & Tracing Demo")?;
    println("---------------------------")?;
    
    fr fr Configure traceback limit
    sus old_limit = get_traceback_limit()?;
    set_traceback_limit(20)?;
    printf("🔢 Set traceback limit to 20 (was {})\n", &[old_limit])?;
    
    fr fr Start tracing
    sus trace_msg = start_trace()?;
    printf("▶️  {}\n", &[trace_msg])?;
    
    fr fr Add some trace events
    add_trace_event("demo_start", "demo_profiling_tracing", 0)?;
    add_trace_event("computation", "heavy_calculation", 1500)?;
    add_trace_event("io_operation", "file_write", 2300)?;
    add_trace_event("demo_end", "demo_profiling_tracing", 5000)?;
    
    println("📝 Added trace events for demo operations")?;
    
    fr fr Read trace data
    sus trace_data = read_trace()?;
    sus trace_str = String::from_utf8(trace_data)?;
    println("📋 Trace data:")?;
    println(trace_str)?;
    
    fr fr Stop tracing
    sus stop_msg = stop_trace()?;
    printf("⏹️  {}\n", &[stop_msg])?;
    
    fr fr CPU profiling demo
    sus cpu_rate = get_cpu_profile_rate()?;
    set_cpu_profile_rate(250)?; fr fr 250Hz sampling
    printf("🔄 Set CPU profile rate to 250Hz (was {}Hz)\n", &[cpu_rate])?;
    
    printf("⚡ CPU profiling capabilities configured\n")?;
    
    fr fr Restore original settings
    set_traceback_limit(old_limit)?;
    set_cpu_profile_rate(cpu_rate)?;
}

slay demo_enhanced_features() {
    println("\n✨ Enhanced Features Demo")?;
    println("-------------------------")?;
    
    fr fr GC mode management
    sus current_gc_mode = get_gc_mode();
    printf("🗑️  Current GC mode: {:?}\n", &[current_gc_mode])?;
    
    fr fr Test different GC modes
    set_gc_mode(GCMode::IncrementalOnly)?;
    printf("🔄 Switched to incremental-only GC\n")?;
    
    set_gc_mode(GCMode::Manual)?;
    printf("✋ Switched to manual GC\n")?;
    
    fr fr Register GC notification
    register_gc_notification(
        || println("🚀 GC cycle starting..."),
        || println("✅ GC cycle completed!")
    )?;
    printf("📬 Registered GC notification callbacks\n")?;
    
    fr fr Start and wait for GC
    start_gc()?;
    printf("🏁 Started GC cycle\n")?;
    sus gc_completed = wait_for_gc()?;
    printf("⏳ GC completed: {}\n", &[gc_completed])?;
    
    fr fr Restore original GC mode
    set_gc_mode(current_gc_mode)?;
    
    fr fr Scheduler mode management
    sus current_scheduler = get_scheduler_mode();
    printf("📅 Current scheduler mode: {:?}\n", &[current_scheduler])?;
    
    set_scheduler_mode(SchedulerMode::Aggressive)?;
    printf("🚀 Switched to aggressive scheduler\n")?;
    
    set_scheduler_mode(SchedulerMode::Fair)?;
    printf("⚖️  Switched to fair scheduler\n")?;
    
    fr fr Restore original scheduler
    set_scheduler_mode(current_scheduler)?;
}

slay demo_performance_tuning() {
    println("\n⚡ Performance Tuning Demo")?;
    println("--------------------------")?;
    
    fr fr Thread management
    sus current_threads = num_threads()?;
    printf("🧵 Current thread count: {}\n", &[current_threads])?;
    
    sus old_threads = set_max_threads(6)?;
    printf("🔧 Set max threads to 6 (was {})\n", &[old_threads])?;
    
    sus new_threads = num_threads()?;
    printf("✅ New thread count: {}\n", &[new_threads])?;
    
    fr fr CPU frequency scaling (platform-dependent)
    sus freq_msg = set_cpu_frequency(85)?;
    printf("🏃 {}\n", &[freq_msg])?;
    
    fr fr Thread priority setting (platform-dependent)
    sus priority_msg = set_thread_priority(1, 5)?;
    printf("⭐ {}\n", &[priority_msg])?;
    
    fr fr Memory optimization
    free_os_memory()?;
    printf("🧹 Released memory back to OS\n")?;
    
    fr fr Restore original thread count
    set_max_threads(old_threads)?;
}

slay spill_chaos_stats() {
    println("\n📈 Comprehensive Chaos Stats")?;
    println("-----------------------------")?;
    
    sus stats = chaos_stats()?;
    sus pretty_stats = json_tea::marshal_indent(stats, "", "  ")?;
    println(pretty_stats)?;
}

fr fr Helper function to handle errors gracefully
slay handle_error(err: ChaosError) {
    eprintln("❌ ChaosMode Error: {}", err);
}

fr fr Example of advanced usage patterns
slay advanced_usage_example() {
    fr fr This would demonstrate more complex scenarios in a real application
    
    fr fr 1. Continuous performance monitoring
    fr fr 2. Adaptive GC tuning based on workload
    fr fr 3. Goroutine leak detection
    fr fr 4. Memory pressure response
    fr fr 5. Real-time profiling integration
    
    println("🔬 Advanced usage patterns available for production applications")?;
}
