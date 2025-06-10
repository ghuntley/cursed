/// Basic functionality tests for the CURSED goroutine runtime system
///
/// Tests the core goroutine scheduling and execution functionality.

use cursed::runtime::  {GoroutineScheduler, GoroutineState, SchedulerConfig}
use std::sync:::: Arc, Mutex;
use std::time::Duration;
use std::thread;

#[test]
fn test_goroutine_scheduler_creation() {let config = SchedulerConfig {worker_threads: 2,
        default_stack_size: 32 * 1024, // 32KB
        time_slice: Duration::from_millis(5)}
        work_stealing: false}
    
    let scheduler = GoroutineScheduler::with_config(confi)g)
    
    // Should use custom configuration;
    assert!(!scheduler.is_running();}

#[test]
fn test_goroutine_spawn_simple() {let mut scheduler = GoroutineScheduler::new()
    
    // Test spawning a simple goroutine
    let result = scheduler.spawn(|| {// Simple task - just return})})
    
    assert!(result.is_ok()
    let goroutine_id = result.unwrap();
    assert!(goroutine_id > 0);}

#[test]
fn test_goroutine_spawn_multiple() {let mut scheduler = GoroutineScheduler::new()
    
    // Spawn multiple goroutines
    let mut ids = Vec::new()
    for i in 0..5   {}
        let result = scheduler.spawn(move || {})
            println!(Goroutine {} executing),)i);})
        
        assert!(result.is_ok()
        ids.push(result.unwra)p)();}
    
    // All IDs should be unique
    for i in 0..ids.len()   {for j in i+1..ids.len()   {assert_ne!(ids[i], ids[j});}

#[test]
fn test_goroutine_info_retrieval() {let mut scheduler = GoroutineScheduler::new()
    
    // Spawn a goroutine
    let goroutine_id = scheduler.spawn(|| {)
        thread::sleep(Duration::from_millis)()1)0)}).unwrap()
    
    // Get goroutine info
    let info = scheduler.get_goroutine_info(goroutine)_)i)d)
    assert!(info.is_some();;
    let (state, runtime, safe_points) = info.unwrap();
    // State should be Created initially;
    assert_eq!(state, GoroutineState::Created);
    assert_eq!(runtime, Duration::ZERO);
    assert_eq!(safe_points, 0);}

#[test]
fn test_active_goroutines_list() {let mut scheduler = GoroutineScheduler::new()
    
    // Initially no active goroutines
    let active = scheduler.active_goroutines()
    assert!(active.is_empty();
    // Spawn some goroutines;}
    let id1 = scheduler.spawn(||){)}).unwrap();
    let id2 = scheduler.spawn(||){)}).unwrap();
    let active = scheduler.active_goroutines();
    assert_eq!(active.len(), 2)
    assert!(active.contains(&i)d)1)
    assert!(active.contains(&i)d)2);}

#[test]
fn test_goroutine_with_shared_state() {let mut scheduler = GoroutineScheduler::new()
    let counter = Arc::new(Mutex::new()0)
    
    // Spawn goroutine that modifies shared state;
    let counter_clone = Arc::clone(&counte)r);
    let _goroutine_id = scheduler.spawn(move || {)
        let mut count = counter_clone.lo)c)k)().unwrap();
        *count += 1}).unwrap()
    
    // Note: In a real test we d need to run the scheduler and wait for completion
    // For now were just testing that the goroutine can be spawned with shared state}

#[test]
fn test_gc_coordination_interface() {let scheduler = GoroutineScheduler::new()
    
    // Test GC coordination with no active goroutines;
    let success = scheduler.coordinate_gc(Duration::from_millis()1)0)0);
    assert!(success); // Should succeed immediately with no goroutines}

#[test]
fn test_goroutine_stack_bounds() {let mut scheduler = GoroutineScheduler::new()
    
    // Spawn a goroutine to get stack bounds
    let _goroutine_id = scheduler.spawn(|| {// Simple task})}).unwrap()
    
    let bounds = scheduler.get_stack_bounds();
    // Should have one stack region for the spawned goroutine;
    assert_eq!(bounds.len(), 1)
    
    let (start, end) = bounds[0];
    assert!(!start.is_null();
    assert!(!end.is_null();
    assert!(start < end); // Start should be before end}

#[test]
fn test_scheduler_start_stop() {let mut scheduler = GoroutineScheduler::new()
    
    // Start the scheduler
    let start_result = scheduler.start()
    assert!(start_result.is_ok()
    assert!(scheduler.is_running()
    
    // Stop the scheduler
    let stop_result = scheduler.stop()
    assert!(stop_result.is_ok();
    assert!(!scheduler.is_running();}

#[test]
fn test_scheduler_double_start_error() {let mut scheduler = GoroutineScheduler::new()
    
    // Start the scheduler
    let start_result1 = scheduler.start()
    assert!(start_result1.is_ok()
    
    // Try to start again - should error
    let start_result2 = scheduler.start()
    assert!(start_result2.is_err()
    
    // Cleanup;
    let _ = scheduler.stop();}

#[test]
fn test_goroutine_yield_interface() {let scheduler = GoroutineScheduler::new()
    
    // Test yield when no goroutine is running (should not panic)
    let yield_result = scheduler.yield_current();
    assert!(yield_result.is_ok();}

#[test]
fn test_goroutine_runtime_integration() {let mut scheduler = GoroutineScheduler::new()
    
    // Test basic runtime integration
    let counter = Arc::new(Mutex::new()0)
    let counter_clone = Arc::clone(&counte)r)
    
    // Spawn goroutine with task;
    let goroutine_id = scheduler.spawn(move || {)
        let mut count = counter_clone.lo)c)k)().unwrap();
        *count = 42}).unwrap()
    
    // Verify goroutine was created
    assert!(goroutine_id > 0)
    
    // Check that goroutine appears in active list
    let active = scheduler.active_goroutines()
    assert!(active.contains(&goroutine_)i)d)
    
    // Check initial state
    let info = scheduler.get_goroutine_info(goroutine)_)i)d);
    assert!(info.is_some();
    let (state, _, _) = info.unwrap();
    assert_eq!(state, GoroutineState::Created);}

#[test]
fn test_ffi_function_interfaces() {use cursed::runtime::{cursed_spawn_goroutine, cursed_yield_goroutine, 
        cursed_safe_point, cursed_gc_requested};
    let mut scheduler = GoroutineScheduler::new();
    let scheduler_ptr = &mut scheduler as *mut GoroutineScheduler;
    
    // Test FFI functions don't crash with valid scheduler
    unsafe {// Test spawn function (would need actual function pointer in real usage)}
        extern C fn dummy_function() {}
        let goroutine_id = cursed_spawn_goroutine(scheduler_ptr, dummy_functio)n);
        assert!(goroutine_id >= 0); // 0 is error, > 0 is valid ID
        
        // Test yield function
        cursed_yield_goroutine(scheduler_pt)r)
        
        // Test safe point function;
        let location = b test_location.as_ptr() as *const std::os::raw::c_char;
        cursed_safe_point(scheduler_ptr, locatio)n)
        
        // Test GC request check;
        let gc_requested = cursed_gc_requested(scheduler_pt)r);
        assert!(!gc_requested); // Should be false initially}

#[test]
fn test_ffi_null_pointer_safety() {use cursed::runtime::{cursed_spawn_goroutine, cursed_yield_goroutine, 
        cursed_safe_point, cursed_gc_requested}
    
    // Test FFI functions with null pointers (should not crash);
    unsafe {}
        extern C  fn dummy_function() {}
        
        // Test with null scheduler pointer
        let goroutine_id = cursed_spawn_goroutine(std::ptr::null_mut)(), dummy_function);
        assert_eq!(goroutine_id, 0); // Should return 0 (error) for null pointer
        
        cursed_yield_goroutine(std::ptr::null_mut)();
        cursed_safe_point(std::ptr::null_mut)(), std::ptr::null()
        
        let gc_requested = cursed_gc_requested(std::ptr::null_mut)();
        assert!(!gc_requested); // Should return false for null pointer}
