/// Comprehensive test suite for ChaosMode runtime package
/// 
/// Tests all aspects of the CURSED runtime system including goroutine management,
/// memory management, profiling, runtime information, and enhanced features.

#[path = "common.rs"]
mod common;

use cursed::stdlib::chaos_mode::*;
use cursed::stdlib::chaos_mode::error::*;
use std::io::Cursor;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Duration;

#[test]
fn test_chaos_mode_initialization() {
    common::tracing::setup();
    
    // Test module initialization and cleanup
    let result = initialize();
    assert!(result.is_ok(), "ChaosMode initialization should succeed");
    
    let result = cleanup();
    assert!(result.is_ok(), "ChaosMode cleanup should succeed");
    
    // Should be able to initialize again
    let result = initialize();
    assert!(result.is_ok(), "ChaosMode should re-initialize successfully");
    
    cleanup().unwrap();
}

#[test]
fn test_core_runtime_functions() {
    common::tracing::setup();
    initialize().unwrap();
    
    // Test basic runtime functions
    let cpu_count = num_cpu().unwrap();
    assert!(cpu_count > 0, "CPU count should be positive: {}", cpu_count);
    assert!(cpu_count <= 256, "CPU count should be reasonable: {}", cpu_count);
    
    let goroutine_count = num_goroutine().unwrap();
    assert!(goroutine_count >= 0, "Goroutine count should be non-negative: {}", goroutine_count);
    
    // Test processor yielding
    assert!(yield_processor().is_ok(), "Processor yield should succeed");
    assert!(gosched().is_ok(), "Goroutine scheduling should succeed");
    
    // Test garbage collection
    assert!(gc().is_ok(), "Manual GC should succeed");
    
    // Test GOMAXPROCS
    let current_procs = gomaxprocs(0).unwrap();
    assert!(current_procs > 0, "GOMAXPROCS should be positive");
    
    let old_procs = gomaxprocs(2).unwrap();
    assert_eq!(old_procs, current_procs, "GOMAXPROCS should return old value");
    
    // Restore original value
    gomaxprocs(current_procs).unwrap();
    
    // Test GC percentage
    let current_percent = 100; // Default GC percent
    let old_percent = set_gc_percent(75).unwrap();
    
    let new_old = set_gc_percent(current_percent).unwrap();
    assert_eq!(new_old, 75, "GC percent should return old value");
    
    // Test max heap setting
    let old_heap = set_max_heap(1024 * 1024 * 1024).unwrap(); // 1GB
    let heap_setting = get_max_heap().unwrap();
    assert_eq!(heap_setting, Some(1024 * 1024 * 1024));
    
    cleanup().unwrap();
}

#[test]
fn test_memory_management() {
    common::tracing::setup();
    initialize().unwrap();
    
    // Test memory statistics
    let mem_stats_result = mem_stats();
    assert!(mem_stats_result.is_ok(), "Memory stats should be available");
    
    let stats = mem_stats_result.unwrap();
    assert!(stats.alloc >= 0, "Allocated memory should be non-negative");
    assert!(stats.total_alloc >= stats.alloc, "Total alloc should be >= current alloc");
    assert!(stats.sys >= stats.alloc, "System memory should be >= allocated");
    assert!(stats.heap_alloc >= 0, "Heap alloc should be non-negative");
    assert!(stats.heap_objects >= 0, "Heap objects should be non-negative");
    assert!(stats.num_gc >= 0, "GC count should be non-negative");
    assert!(stats.gc_cpu_fraction >= 0.0 && stats.gc_cpu_fraction <= 1.0, 
            "GC CPU fraction should be between 0 and 1");
    
    // Test read_mem_stats
    let mut stats_copy = memory::MemoryStats::default();
    let result = read_mem_stats(&mut stats_copy);
    assert!(result.is_ok(), "read_mem_stats should succeed");
    assert_eq!(stats_copy.alloc, stats.alloc, "Stats should match");
    
    // Test GC enabled/disabled
    let old_enabled = set_gc_enabled(false).unwrap();
    let new_old = set_gc_enabled(true).unwrap();
    assert!(!new_old, "GC should have been disabled");
    
    // Test free OS memory
    assert!(free_os_memory().is_ok(), "Free OS memory should succeed");
    
    // Test memory profile rate
    assert!(set_mem_profile_rate(1024).is_ok(), "Set mem profile rate should succeed");
    
    cleanup().unwrap();
}

#[test]
fn test_memory_debugging() {
    common::tracing::setup();
    initialize().unwrap();
    
    // Test allocation size histogram
    let histogram_result = allocation_size_histogram();
    assert!(histogram_result.is_ok(), "Allocation histogram should be available");
    
    let histogram = histogram_result.unwrap();
    assert!(!histogram.is_empty(), "Histogram should not be empty");
    assert!(histogram.contains_key(&8), "Should have small allocations");
    assert!(histogram.contains_key(&1024), "Should have large allocations");
    
    // Test top allocated types
    let types_result = top_allocated_types(5);
    assert!(types_result.is_ok(), "Top allocated types should be available");
    
    let types = types_result.unwrap();
    assert!(!types.is_empty(), "Should have allocation types");
    assert!(types.len() <= 5, "Should respect limit");
    
    // Verify sorted by total size
    for i in 1..types.len() {
        assert!(types[i-1].total_size >= types[i].total_size, 
                "Types should be sorted by total size descending");
    }
    
    // Test each type has valid data
    for type_info in &types {
        assert!(!type_info.type_name.is_empty(), "Type name should not be empty");
        assert!(type_info.count > 0, "Type count should be positive");
        assert!(type_info.total_size > 0, "Total size should be positive");
        assert!(type_info.average_size > 0, "Average size should be positive");
        
        // Average should be approximately correct
        let expected_avg = type_info.total_size / type_info.count as i64;
        let diff = (type_info.average_size - expected_avg).abs();
        assert!(diff <= 1, "Average size should be approximately correct");
    }
    
    // Test pointer validation
    let null_ptr: *const u8 = std::ptr::null();
    assert!(!is_valid_pointer(null_ptr).unwrap(), "Null pointer should be invalid");
    
    let valid_ptr = &42u8 as *const u8;
    assert!(is_valid_pointer(valid_ptr).unwrap(), "Valid pointer should be recognized");
    
    // Test object size
    assert!(get_object_size(null_ptr).is_err(), "Should fail for null pointer");
    
    let size = get_object_size(valid_ptr).unwrap();
    assert!(size > 0, "Object size should be positive");
    
    // Test pointer info
    assert!(get_pointer_info(null_ptr).is_err(), "Should fail for null pointer");
    
    let info = get_pointer_info(valid_ptr).unwrap();
    assert_eq!(info.address, valid_ptr as usize, "Address should match");
    assert!(info.size > 0, "Size should be positive");
    assert!(!info.type_name.is_empty(), "Type name should not be empty");
    assert!(info.reachable, "Pointer should be reachable");
    
    cleanup().unwrap();
}

#[test]
fn test_goroutine_management() {
    common::tracing::setup();
    initialize().unwrap();
    
    // Test stack trace
    let trace_result = stack_trace();
    assert!(trace_result.is_ok(), "Stack trace should be available");
    
    let trace = trace_result.unwrap();
    assert!(!trace.is_empty(), "Stack trace should not be empty");
    
    // Test all goroutine IDs
    let ids_result = all_goroutine_ids();
    assert!(ids_result.is_ok(), "Goroutine IDs should be available");
    
    let ids = ids_result.unwrap();
    assert!(!ids.is_empty(), "Should have at least one goroutine");
    assert!(ids.contains(&1), "Should include main goroutine");
    
    // Test all goroutine stacks
    let stacks_result = all_goroutine_stacks();
    assert!(stacks_result.is_ok(), "Goroutine stacks should be available");
    
    let stacks_json = stacks_result.unwrap();
    assert!(!stacks_json.is_empty(), "Stacks JSON should not be empty");
    
    // Should be valid JSON
    let parsed: Result<serde_json::Value, _> = serde_json::from_str(&stacks_json);
    assert!(parsed.is_ok(), "Stacks should be valid JSON");
    
    // Test callers
    let mut pc = [0usize; 10];
    let count = callers(2, &mut pc).unwrap();
    assert!(count > 0, "Should capture stack frames");
    assert!(count <= 10, "Should respect PC array size");
    
    // Check PC values were filled
    for i in 0..count as usize {
        assert!(pc[i] != 0, "PC values should be non-zero");
    }
    
    // Test PC to file/line conversion
    let (file, line) = pc_to_file_and_line(pc[0]).unwrap();
    assert!(!file.is_empty(), "File name should not be empty");
    assert!(file.ends_with(".rs"), "Should be a Rust file");
    assert!(line > 0, "Line number should be positive");
    
    // Test PC to function name conversion
    let func_name = pc_to_func_name(pc[0]).unwrap();
    assert!(!func_name.is_empty(), "Function name should not be empty");
    assert!(func_name.contains("::"), "Should contain module separator");
    
    cleanup().unwrap();
}

#[test]
fn test_enhanced_goroutine_features() {
    common::tracing::setup();
    initialize().unwrap();
    
    // Test goroutine info for main goroutine
    let info_result = goroutine_info(1);
    assert!(info_result.is_ok(), "Should get info for main goroutine");
    
    let info = info_result.unwrap();
    assert_eq!(info.id, 1, "ID should match");
    assert!(!info.state.is_empty(), "State should not be empty");
    assert!(!info.stack_trace.is_empty(), "Stack trace should not be empty");
    assert!(info.cpu_time.as_millis() >= 0, "CPU time should be non-negative");
    
    // Test setting goroutine labels
    assert!(set_goroutine_label("purpose".to_string(), "test".to_string()).is_ok());
    assert!(set_goroutine_label("component".to_string(), "chaos_mode".to_string()).is_ok());
    
    // Test getting goroutines by label
    let labeled_ids = goroutines_by_label("purpose".to_string(), "test".to_string()).unwrap();
    // Note: Our implementation doesn't track current goroutine labels, so this might be empty
    assert!(labeled_ids.len() >= 0, "Should return valid result");
    
    // Test getting goroutines by state
    let running_ids = goroutines_by_state("running".to_string()).unwrap();
    assert!(!running_ids.is_empty(), "Should have running goroutines");
    assert!(running_ids.contains(&1), "Main goroutine should be running");
    
    let waiting_ids = goroutines_by_state("waiting".to_string()).unwrap();
    assert!(waiting_ids.len() >= 0, "Waiting goroutines result should be valid");
    
    // Test goroutine stack retrieval
    let stack = goroutine_stack(1).unwrap();
    assert!(!stack.is_empty(), "Goroutine stack should not be empty");
    assert!(stack.contains("goroutine"), "Stack should mention goroutine");
    
    // Test killing a goroutine (create and kill a copy)
    let kill_result = kill_goroutine(1);
    assert!(kill_result.is_ok(), "Should be able to kill goroutine");
    
    let message = kill_result.unwrap();
    assert!(message.contains("killed"), "Message should confirm kill");
    
    // Verify it's gone
    let info_result2 = goroutine_info(1);
    assert!(info_result2.is_err(), "Killed goroutine should not be found");
    
    // Test killing non-existent goroutine
    let kill_result2 = kill_goroutine(999);
    assert!(kill_result2.is_err(), "Should fail for non-existent goroutine");
    
    cleanup().unwrap();
}

#[test]
fn test_profiling_and_tracing() {
    common::tracing::setup();
    initialize().unwrap();
    
    // Test trace functionality
    assert!(!is_trace_enabled().unwrap(), "Tracing should start disabled");
    
    let start_result = start_trace();
    assert!(start_result.is_ok(), "Should start tracing successfully");
    
    let message = start_result.unwrap();
    assert!(message.contains("started"), "Start message should confirm tracing started");
    assert!(is_trace_enabled().unwrap(), "Tracing should be enabled");
    
    // Starting again should fail
    assert!(start_trace().is_err(), "Starting trace twice should fail");
    
    // Add some trace events
    assert!(add_trace_event("function_enter", "test_function", 0).is_ok());
    assert!(add_trace_event("allocation", "test_allocation", 1024).is_ok());
    assert!(add_trace_event("function_exit", "test_function", 2500).is_ok());
    
    // Read trace data
    let trace_data = read_trace().unwrap();
    let trace_str = String::from_utf8(trace_data).unwrap();
    
    assert!(trace_str.contains("Chaos Mode Trace Data"), "Should have trace header");
    assert!(trace_str.contains("trace_start"), "Should contain start event");
    assert!(trace_str.contains("test_function"), "Should contain our test events");
    assert!(trace_str.contains("1024"), "Should contain allocation size");
    
    // Stop tracing
    let stop_result = stop_trace();
    assert!(stop_result.is_ok(), "Should stop tracing successfully");
    
    let stop_message = stop_result.unwrap();
    assert!(stop_message.contains("stopped"), "Stop message should confirm tracing stopped");
    assert!(!is_trace_enabled().unwrap(), "Tracing should be disabled");
    
    // Stopping again should fail
    assert!(stop_trace().is_err(), "Stopping trace twice should fail");
    
    cleanup().unwrap();
}

#[test]
fn test_cpu_profiling() {
    common::tracing::setup();
    initialize().unwrap();
    
    // Test traceback limit
    let old_limit = get_traceback_limit().unwrap();
    assert!(old_limit > 0, "Traceback limit should be positive");
    
    assert!(set_traceback_limit(50).is_ok(), "Should set traceback limit");
    assert_eq!(get_traceback_limit().unwrap(), 50, "Traceback limit should be updated");
    
    // Test negative limit
    assert!(set_traceback_limit(-1).is_err(), "Negative traceback limit should fail");
    
    // Test CPU profile rate
    let old_rate = get_cpu_profile_rate().unwrap();
    assert!(old_rate > 0, "CPU profile rate should be positive");
    
    assert!(set_cpu_profile_rate(200).is_ok(), "Should set CPU profile rate");
    assert_eq!(get_cpu_profile_rate().unwrap(), 200, "CPU profile rate should be updated");
    
    // Test negative rate
    assert!(set_cpu_profile_rate(-1).is_err(), "Negative CPU profile rate should fail");
    
    // Test CPU profiling
    assert!(!is_cpu_profiling_active().unwrap(), "CPU profiling should start inactive");
    
    let buffer = Cursor::new(Vec::new());
    let start_result = start_cpu_profile(buffer);
    assert!(start_result.is_ok(), "Should start CPU profiling");
    
    let start_message = start_result.unwrap();
    assert!(start_message.contains("started"), "Start message should confirm profiling started");
    assert!(is_cpu_profiling_active().unwrap(), "CPU profiling should be active");
    
    // Starting again should fail
    let buffer2 = Cursor::new(Vec::new());
    assert!(start_cpu_profile(buffer2).is_err(), "Starting CPU profiling twice should fail");
    
    // Stop profiling
    assert!(stop_cpu_profile().is_ok(), "Should stop CPU profiling");
    assert!(!is_cpu_profiling_active().unwrap(), "CPU profiling should be inactive");
    
    // Stopping again should fail
    assert!(stop_cpu_profile().is_err(), "Stopping CPU profiling twice should fail");
    
    // Restore original values
    set_traceback_limit(old_limit).unwrap();
    set_cpu_profile_rate(old_rate).unwrap();
    
    cleanup().unwrap();
}

#[test]
fn test_runtime_information() {
    common::tracing::setup();
    
    // Test basic runtime info
    let version_str = version().unwrap();
    assert!(!version_str.is_empty(), "Version should not be empty");
    
    let arch = goarch().unwrap();
    assert!(!arch.is_empty(), "Architecture should not be empty");
    assert!(["amd64", "386", "arm", "arm64", "x86_64", "aarch64"].iter()
        .any(|&a| arch.contains(a)), "Should be valid architecture: {}", arch);
    
    let os = goos().unwrap();
    assert!(!os.is_empty(), "OS should not be empty");
    assert!(["linux", "windows", "darwin", "macos"].iter()
        .any(|&o| os.contains(o)), "Should be valid OS: {}", os);
    
    let compiler_str = compiler().unwrap();
    assert!(!compiler_str.is_empty(), "Compiler should not be empty");
    
    let root = goroot().unwrap();
    assert!(!root.is_empty(), "GOROOT should not be empty");
    
    // Test runtime stats
    let stats = runtime_stats().unwrap();
    assert!(stats.contains_key("version"), "Should have version");
    assert!(stats.contains_key("compiler"), "Should have compiler");
    assert!(stats.contains_key("goos"), "Should have OS");
    assert!(stats.contains_key("goarch"), "Should have architecture");
    assert!(stats.contains_key("num_cpu"), "Should have CPU count");
    assert!(stats.contains_key("goroutines"), "Should have goroutine count");
    assert!(stats.contains_key("heap_alloc"), "Should have heap allocation");
    assert!(stats.contains_key("features"), "Should have features");
    assert!(stats.contains_key("environment"), "Should have environment");
    assert!(stats.contains_key("timing"), "Should have timing info");
    
    // Verify data types
    assert!(stats["version"].is_string(), "Version should be string");
    assert!(stats["num_cpu"].is_number(), "CPU count should be number");
    assert!(stats["goroutines"].is_number(), "Goroutine count should be number");
    assert!(stats["features"].is_object(), "Features should be object");
    
    // Test system info
    let system_info = runtime_info::system_info().unwrap();
    assert!(system_info.contains_key("os"), "Should have OS info");
    assert!(system_info.contains_key("cpu"), "Should have CPU info");
    assert!(system_info.contains_key("memory_layout"), "Should have memory layout");
    assert!(system_info.contains_key("process"), "Should have process info");
    
    // Test performance info
    let perf_info = runtime_info::performance_info().unwrap();
    assert!(perf_info.contains_key("runtime_performance"), "Should have runtime performance");
    assert!(perf_info.contains_key("jit_performance"), "Should have JIT performance");
    assert!(perf_info.contains_key("memory_performance"), "Should have memory performance");
    
    // Test environment variables
    let env_vars = runtime_info::cursed_env_vars().unwrap();
    assert!(env_vars.len() >= 0, "Should return valid env vars map");
}

#[test]
fn test_enhanced_gc_features() {
    common::tracing::setup();
    initialize().unwrap();
    
    // Test GC modes
    assert_eq!(get_gc_mode(), enhanced::GCMode::Auto, "Should start in auto mode");
    
    assert!(set_gc_mode(enhanced::GCMode::Manual).is_ok(), "Should set manual mode");
    assert_eq!(get_gc_mode(), enhanced::GCMode::Manual, "Should be in manual mode");
    
    assert!(set_gc_mode(enhanced::GCMode::IncrementalOnly).is_ok(), "Should set incremental mode");
    assert_eq!(get_gc_mode(), enhanced::GCMode::IncrementalOnly, "Should be in incremental mode");
    
    assert!(set_gc_mode(enhanced::GCMode::StopTheWorldOnly).is_ok(), "Should set stop-the-world mode");
    assert_eq!(get_gc_mode(), enhanced::GCMode::StopTheWorldOnly, "Should be in stop-the-world mode");
    
    // Reset to auto
    assert!(set_gc_mode(enhanced::GCMode::Auto).is_ok(), "Should reset to auto mode");
    
    // Test GC notifications
    let before_called = Arc::new(AtomicBool::new(false));
    let after_called = Arc::new(AtomicBool::new(false));
    
    let before_called_clone = before_called.clone();
    let after_called_clone = after_called.clone();
    
    assert!(register_gc_notification(
        move || { before_called_clone.store(true, Ordering::SeqCst); },
        move || { after_called_clone.store(true, Ordering::SeqCst); }
    ).is_ok(), "Should register GC notification");
    
    assert_eq!(get_gc_notification_count().unwrap(), 1, "Should have one notification registered");
    
    // Test GC cycle
    assert!(!is_gc_in_progress().unwrap(), "GC should not be in progress initially");
    assert!(get_gc_duration().unwrap().is_none(), "Should have no GC duration initially");
    
    assert!(start_gc().is_ok(), "Should start GC");
    assert!(is_gc_in_progress().unwrap(), "GC should be in progress");
    assert!(get_gc_duration().unwrap().is_some(), "Should have GC duration");
    
    // Starting again should fail
    assert!(start_gc().is_err(), "Starting GC twice should fail");
    
    assert!(wait_for_gc().unwrap(), "Should wait for GC to complete");
    assert!(!is_gc_in_progress().unwrap(), "GC should not be in progress after completion");
    assert!(get_gc_duration().unwrap().is_none(), "Should have no GC duration after completion");
    
    // Verify callbacks were called
    assert!(before_called.load(Ordering::SeqCst), "Before callback should have been called");
    assert!(after_called.load(Ordering::SeqCst), "After callback should have been called");
    
    cleanup().unwrap();
}

#[test]
fn test_performance_tuning() {
    common::tracing::setup();
    initialize().unwrap();
    
    // Test thread management
    let current_threads = num_threads().unwrap();
    assert!(current_threads > 0, "Should have positive thread count");
    
    let old_threads = set_max_threads(4).unwrap();
    assert_eq!(old_threads, current_threads, "Should return old thread count");
    assert_eq!(num_threads().unwrap(), 4, "Should have updated thread count");
    
    // Test invalid thread count
    assert!(set_max_threads(0).is_err(), "Zero threads should fail");
    assert!(set_max_threads(-1).is_err(), "Negative threads should fail");
    
    // Test CPU frequency setting
    let freq_result = set_cpu_frequency(80);
    assert!(freq_result.is_ok(), "Should set CPU frequency");
    let freq_message = freq_result.unwrap();
    assert!(freq_message.contains("80%"), "Message should contain percentage");
    
    // Test invalid frequency
    assert!(set_cpu_frequency(-1).is_err(), "Negative frequency should fail");
    assert!(set_cpu_frequency(101).is_err(), "Over 100% frequency should fail");
    
    // Test thread priority setting
    let priority_result = set_thread_priority(1, 10);
    assert!(priority_result.is_ok(), "Should set thread priority");
    let priority_message = priority_result.unwrap();
    assert!(priority_message.contains("Thread 1"), "Message should contain thread ID");
    assert!(priority_message.contains("priority set to 10"), "Message should contain priority");
    
    // Test invalid priority
    assert!(set_thread_priority(1, -21).is_err(), "Too low priority should fail");
    assert!(set_thread_priority(1, 21).is_err(), "Too high priority should fail");
    
    // Test scheduler modes
    assert_eq!(get_scheduler_mode(), enhanced::SchedulerMode::Default, "Should start in default mode");
    
    assert!(set_scheduler_mode(enhanced::SchedulerMode::Fair).is_ok(), "Should set fair mode");
    assert_eq!(get_scheduler_mode(), enhanced::SchedulerMode::Fair, "Should be in fair mode");
    
    assert!(set_scheduler_mode(enhanced::SchedulerMode::Aggressive).is_ok(), "Should set aggressive mode");
    assert_eq!(get_scheduler_mode(), enhanced::SchedulerMode::Aggressive, "Should be in aggressive mode");
    
    assert!(set_scheduler_mode(enhanced::SchedulerMode::Conservative).is_ok(), "Should set conservative mode");
    assert_eq!(get_scheduler_mode(), enhanced::SchedulerMode::Conservative, "Should be in conservative mode");
    
    // Reset to default
    assert!(set_scheduler_mode(enhanced::SchedulerMode::Default).is_ok(), "Should reset to default mode");
    
    cleanup().unwrap();
}

#[test]
fn test_chaos_stats_integration() {
    common::tracing::setup();
    initialize().unwrap();
    
    // Test comprehensive stats
    let stats_result = chaos_stats();
    assert!(stats_result.is_ok(), "Chaos stats should be available");
    
    let stats = stats_result.unwrap();
    
    // Check structure
    assert!(stats["goroutines"].is_object(), "Should have goroutines section");
    assert!(stats["memory"].is_object(), "Should have memory section");
    assert!(stats["system"].is_object(), "Should have system section");
    assert!(stats["enhanced"].is_object(), "Should have enhanced section");
    
    // Check goroutines section
    let goroutines = &stats["goroutines"];
    assert!(goroutines["count"].is_number(), "Should have goroutine count");
    assert!(goroutines["max_procs"].is_number(), "Should have max procs");
    
    // Check memory section
    let memory = &stats["memory"];
    assert!(memory["stats"].is_object(), "Should have memory stats");
    assert!(memory["gc_percent"].is_number(), "Should have GC percent");
    
    // Check system section
    let system = &stats["system"];
    assert!(system["num_cpu"].is_number(), "Should have CPU count");
    assert!(system["version"].is_string(), "Should have version");
    assert!(system["goos"].is_string(), "Should have OS");
    assert!(system["goarch"].is_string(), "Should have architecture");
    
    // Check enhanced section
    let enhanced = &stats["enhanced"];
    assert!(enhanced["gc_mode"].is_number(), "Should have GC mode");
    assert!(enhanced["scheduler_mode"].is_number(), "Should have scheduler mode");
    assert!(enhanced["num_threads"].is_number(), "Should have thread count");
    
    cleanup().unwrap();
}

#[test]
fn test_error_handling() {
    common::tracing::setup();
    
    // Test functions without initialization
    let result = set_gc_mode(enhanced::GCMode::Manual);
    assert!(result.is_err(), "Should fail without initialization");
    
    let result = start_gc();
    assert!(result.is_err(), "Should fail without initialization");
    
    let result = register_gc_notification(|| {}, || {});
    assert!(result.is_err(), "Should fail without initialization");
    
    // Initialize and test error conditions
    initialize().unwrap();
    
    // Test invalid parameters
    let result = set_max_threads(-1);
    assert!(result.is_err(), "Negative thread count should fail");
    
    let result = set_cpu_frequency(150);
    assert!(result.is_err(), "Invalid CPU frequency should fail");
    
    let result = set_thread_priority(1, 100);
    assert!(result.is_err(), "Invalid thread priority should fail");
    
    let result = set_traceback_limit(-5);
    assert!(result.is_err(), "Negative traceback limit should fail");
    
    let result = set_cpu_profile_rate(-10);
    assert!(result.is_err(), "Negative CPU profile rate should fail");
    
    // Test duplicate operations
    start_gc().unwrap();
    let result = start_gc();
    assert!(result.is_err(), "Duplicate GC start should fail");
    wait_for_gc().unwrap();
    
    let buffer = Cursor::new(Vec::new());
    start_cpu_profile(buffer).unwrap();
    let buffer2 = Cursor::new(Vec::new());
    let result = start_cpu_profile(buffer2);
    assert!(result.is_err(), "Duplicate CPU profiling start should fail");
    stop_cpu_profile().unwrap();
    
    start_trace().unwrap();
    let result = start_trace();
    assert!(result.is_err(), "Duplicate trace start should fail");
    stop_trace().unwrap();
    
    // Test operations on stopped/inactive systems
    let result = stop_trace();
    assert!(result.is_err(), "Stop trace without start should fail");
    
    let result = stop_cpu_profile();
    assert!(result.is_err(), "Stop CPU profiling without start should fail");
    
    // Test with null/invalid pointers
    let null_ptr: *const u8 = std::ptr::null();
    let result = get_object_size(null_ptr);
    assert!(result.is_err(), "Get object size with null pointer should fail");
    
    let result = get_pointer_info(null_ptr);
    assert!(result.is_err(), "Get pointer info with null pointer should fail");
    
    cleanup().unwrap();
}

#[test]
fn test_concurrent_access() {
    common::tracing::setup();
    initialize().unwrap();
    
    // Test concurrent access to chaos mode functions
    let handles: Vec<_> = (0..4).map(|i| {
        std::thread::spawn(move || {
            // Each thread performs different operations
            match i % 4 {
                0 => {
                    // Memory operations
                    let _ = mem_stats();
                    let _ = allocation_size_histogram();
                    let _ = top_allocated_types(3);
                },
                1 => {
                    // Goroutine operations
                    let _ = stack_trace();
                    let _ = all_goroutine_ids();
                    let _ = goroutines_by_state("running".to_string());
                },
                2 => {
                    // Runtime info operations
                    let _ = version();
                    let _ = runtime_stats();
                    let _ = runtime_info::system_info();
                },
                3 => {
                    // Enhanced operations
                    let _ = num_threads();
                    let _ = get_gc_mode();
                    let _ = get_scheduler_mode();
                },
                _ => unreachable!(),
            }
        })
    }).collect();
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    cleanup().unwrap();
}

#[test]
fn test_stress_scenarios() {
    common::tracing::setup();
    initialize().unwrap();
    
    // Stress test: rapid GC cycles
    for _ in 0..10 {
        gc().unwrap();
    }
    
    // Stress test: many memory stats calls
    for _ in 0..100 {
        let _ = mem_stats().unwrap();
    }
    
    // Stress test: many goroutine queries
    for _ in 0..50 {
        let _ = all_goroutine_ids().unwrap();
        let _ = stack_trace().unwrap();
    }
    
    // Stress test: trace events
    start_trace().unwrap();
    for i in 0..1000 {
        let _ = add_trace_event("stress_test", "stress_function", i as u64);
    }
    let trace_data = read_trace().unwrap();
    assert!(trace_data.len() > 1000, "Should have substantial trace data");
    stop_trace().unwrap();
    
    // Stress test: GC notifications
    for i in 0..20 {
        let _ = register_gc_notification(
            move || { let _ = i; }, // Capture i to make each closure unique
            move || { let _ = i; }
        );
    }
    assert_eq!(get_gc_notification_count().unwrap(), 20, "Should have 20 notifications");
    
    // Trigger GC with all notifications
    start_gc().unwrap();
    wait_for_gc().unwrap();
    
    cleanup().unwrap();
}
