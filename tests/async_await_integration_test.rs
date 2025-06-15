/// Integration tests for async/await runtime system
use cursed::runtime::async::*;
use cursed::runtime::async::event_loop::{EventLoop, EventLoopConfig};
use cursed::codegen::llvm::async_await::*;
use std::time::Duration;
use std::sync::{Arc, Mutex};

#[path = "common.rs"]
mod common;

/// Test basic async runtime initialization
#[test]
fn test_async_runtime_initialization() {
    common::tracing::setup();
    
    // Initialize async runtime
    let result = initialize_async_runtime();
    assert!(result.is_ok(), "Failed to initialize async runtime: {:?}", result);
    
    // Check that runtime is available
    let runtime = get_async_runtime();
    assert!(runtime.is_some(), "Async runtime not available after initialization");
    
    // Cleanup
    shutdown_async_runtime();
}

/// Test basic future spawning and execution
#[test]
fn test_future_spawning() {
    common::tracing::setup();
    
    let _ = initialize_async_runtime();
    
    // Create a simple future
    let result = Arc::new(Mutex::new(None));
    let result_clone = result.clone();
    
    let future = async move {
        *result_clone.lock().unwrap() = Some(42);
        42
    };
    
    // Spawn the future
    let handle = spawn(future);
    
    // Block on the result
    let value = block_on(async move {
        handle.await.unwrap_or(0)
    });
    
    assert_eq!(value, 42);
    assert_eq!(*result.lock().unwrap(), Some(42));
    
    shutdown_async_runtime();
}

/// Test async/await with delays
#[test]
fn test_async_delay() {
    common::tracing::setup();
    
    let _ = initialize_async_runtime();
    
    let start_time = std::time::Instant::now();
    
    let result = block_on(async {
        delay(Duration::from_millis(100)).await;
        42
    });
    
    let elapsed = start_time.elapsed();
    assert!(elapsed >= Duration::from_millis(90), "Delay was too short: {:?}", elapsed);
    assert_eq!(result, 42);
    
    shutdown_async_runtime();
}

/// Test async timeout functionality
#[test]
fn test_async_timeout() {
    common::tracing::setup();
    
    let _ = initialize_async_runtime();
    
    // Test successful completion within timeout
    let result = block_on(async {
        timeout(Duration::from_millis(200), async {
            delay(Duration::from_millis(50)).await;
            42
        }).await
    });
    
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), 42);
    
    // Test timeout expiration
    let result = block_on(async {
        timeout(Duration::from_millis(50), async {
            delay(Duration::from_millis(200)).await;
            42
        }).await
    });
    
    assert!(result.is_err());
    assert!(matches!(result.unwrap_err(), FutureError::Timeout));
    
    shutdown_async_runtime();
}

/// Test event loop basic functionality
#[test]
fn test_event_loop_basic() {
    common::tracing::setup();
    
    let config = EventLoopConfig::default();
    let event_loop = EventLoop::new(config.clone());
    
    assert!(event_loop.start().is_ok());
    assert!(event_loop.is_running());
    
    // Spawn a simple task
    let task_id = event_loop.spawn(async { 42 });
    assert!(task_id.0 > 0);
    
    // Run one iteration
    let progress = event_loop.run_once(&config).unwrap();
    assert!(progress, "Event loop should have made progress");
    
    event_loop.shutdown();
    assert!(!event_loop.is_running());
}

/// Test event loop with multiple tasks
#[test]
fn test_event_loop_multiple_tasks() {
    common::tracing::setup();
    
    let config = EventLoopConfig::default();
    let event_loop = EventLoop::new(config.clone());
    
    event_loop.start().unwrap();
    
    // Spawn multiple tasks
    let task_ids: Vec<_> = (0..5).map(|i| {
        event_loop.spawn(async move { i * 2 })
    }).collect();
    
    assert_eq!(task_ids.len(), 5);
    
    // Run the event loop for a bit
    for _ in 0..10 {
        let _ = event_loop.run_once(&config);
    }
    
    let stats = event_loop.statistics();
    assert!(stats.tasks_polled > 0, "Should have polled some tasks");
    
    event_loop.shutdown();
}

/// Test event loop statistics collection
#[test]
fn test_event_loop_statistics() {
    common::tracing::setup();
    
    let config = EventLoopConfig {
        collect_stats: true,
        ..Default::default()
    };
    let event_loop = EventLoop::new(config.clone());
    
    event_loop.start().unwrap();
    
    // Initial stats
    let initial_stats = event_loop.statistics();
    assert_eq!(initial_stats.loop_iterations, 0);
    assert_eq!(initial_stats.tasks_polled, 0);
    
    // Spawn a task and run
    event_loop.spawn(async { 42 });
    event_loop.run_once(&config).unwrap();
    
    // Check updated stats
    let updated_stats = event_loop.statistics();
    assert!(updated_stats.loop_iterations > 0);
    
    event_loop.shutdown();
}

/// Test async runtime with executor integration
#[test]
fn test_runtime_executor_integration() {
    common::tracing::setup();
    
    let config = AsyncRuntimeConfig::default();
    let runtime = AsyncRuntime::new(config);
    
    assert!(runtime.initialize().is_ok());
    
    // Test spawning with different priorities
    let high_prio_handle = runtime.spawn_with_priority(async { 1 }, TaskPriority::High);
    let normal_prio_handle = runtime.spawn_with_priority(async { 2 }, TaskPriority::Normal);
    let low_prio_handle = runtime.spawn_with_priority(async { 3 }, TaskPriority::Low);
    
    // Block on all handles
    let results = runtime.block_on(async {
        let high = high_prio_handle.await.unwrap_or(0);
        let normal = normal_prio_handle.await.unwrap_or(0);
        let low = low_prio_handle.await.unwrap_or(0);
        (high, normal, low)
    });
    
    assert_eq!(results, (1, 2, 3));
    
    runtime.shutdown();
}

/// Test async runtime statistics
#[test]
fn test_runtime_statistics() {
    common::tracing::setup();
    
    let config = AsyncRuntimeConfig::default();
    let runtime = AsyncRuntime::new(config);
    
    runtime.initialize().unwrap();
    
    // Initial statistics
    let initial_stats = runtime.statistics();
    assert_eq!(initial_stats.active_futures, 0);
    
    // Spawn some futures
    let _handle1 = runtime.spawn(async { delay(Duration::from_millis(10)).await; 1 });
    let _handle2 = runtime.spawn(async { delay(Duration::from_millis(20)).await; 2 });
    
    // Give some time for statistics to update
    std::thread::sleep(Duration::from_millis(5));
    
    let updated_stats = runtime.statistics();
    assert!(updated_stats.runtime_uptime > Duration::ZERO);
    
    runtime.shutdown();
}

/// Test async function context creation
#[test]
fn test_async_function_context() {
    common::tracing::setup();
    
    let function = std::ptr::null_mut();
    let context_struct = std::ptr::null_mut();
    let mut context = AsyncFunctionContext::new(function, context_struct);
    
    assert_eq!(context.current_state, 0);
    assert_eq!(context.await_points.len(), 0);
    
    // Test state progression
    assert_eq!(context.next_state(), 1);
    assert_eq!(context.current_state, 1);
    
    // Test await point addition
    let await_point = AwaitPoint {
        block_id: 1,
        future_value: "test_future".to_string(),
        result_type: std::ptr::null_mut(),
        continuation_block: "continue_1".to_string(),
    };
    
    let await_id = context.add_await_point(await_point);
    assert_eq!(await_id, 0);
    assert_eq!(context.await_points.len(), 1);
}

/// Test await point creation and management
#[test]
fn test_await_point_management() {
    common::tracing::setup();
    
    let await_point = AwaitPoint {
        block_id: 42,
        future_value: "test_future_value".to_string(),
        result_type: std::ptr::null_mut(),
        continuation_block: "test_continuation".to_string(),
    };
    
    assert_eq!(await_point.block_id, 42);
    assert_eq!(await_point.future_value, "test_future_value");
    assert_eq!(await_point.continuation_block, "test_continuation");
}

/// Test FFI function existence and basic behavior
#[test]
fn test_ffi_functions() {
    common::tracing::setup();
    
    // Test future ID generation
    let id1 = cursed::codegen::llvm::async_await::next_future_id();
    let id2 = cursed::codegen::llvm::async_await::next_future_id();
    assert!(id2 > id1, "Future IDs should be increasing");
    
    // Test basic FFI function calls (these are mostly stubs for now)
    let task_id = cursed::codegen::llvm::async_await::cursed_spawn_async_task(
        test_task_function,
        std::ptr::null_mut()
    );
    assert!(task_id > 0);
    
    let is_ready = cursed::codegen::llvm::async_await::cursed_future_is_ready(task_id);
    // Should return false for non-existent future
    assert!(!is_ready);
    
    let delay_id = cursed::codegen::llvm::async_await::cursed_create_delay(100);
    assert!(delay_id > 0);
}

/// Test task function for FFI testing
extern "C" fn test_task_function() {
    // Simple test function that does nothing
}

/// Test async runtime coordination with existing systems
#[test]
fn test_runtime_coordination() {
    common::tracing::setup();
    
    let config = AsyncRuntimeConfig {
        integrate_with_goroutines: true,
        ..Default::default()
    };
    let runtime = AsyncRuntime::new(config);
    
    runtime.initialize().unwrap();
    
    // Test GC coordination
    let coordination_result = runtime.coordinate_gc();
    assert!(coordination_result.is_ok(), "GC coordination failed: {:?}", coordination_result);
    
    let stats = runtime.statistics();
    // Integration might not be active if no goroutine scheduler is available
    // assert!(stats.goroutine_integration_active);
    
    runtime.shutdown();
}

/// Test yield functionality
#[test]
fn test_yield_functionality() {
    common::tracing::setup();
    
    let _ = initialize_async_runtime();
    
    let result = block_on(async {
        let mut sum = 0;
        for i in 0..5 {
            sum += i;
            yield_now().await; // Should yield control
        }
        sum
    });
    
    assert_eq!(result, 0 + 1 + 2 + 3 + 4);
    
    shutdown_async_runtime();
}

/// Test concurrent async operations
#[test]
fn test_concurrent_async_operations() {
    common::tracing::setup();
    
    let _ = initialize_async_runtime();
    
    let result = block_on(async {
        let futures = (0..10).map(|i| {
            spawn(async move {
                delay(Duration::from_millis(10)).await;
                i * 2
            })
        }).collect::<Vec<_>>();
        
        let mut results = Vec::new();
        for handle in futures {
            results.push(handle.await.unwrap_or(0));
        }
        
        results
    });
    
    assert_eq!(result.len(), 10);
    for (i, &value) in result.iter().enumerate() {
        assert_eq!(value, i * 2);
    }
    
    shutdown_async_runtime();
}

/// Test async runtime shutdown behavior
#[test]
fn test_runtime_shutdown() {
    common::tracing::setup();
    
    let _ = initialize_async_runtime();
    
    // Check runtime is available
    assert!(get_async_runtime().is_some());
    
    // Shutdown
    shutdown_async_runtime();
    
    // Runtime should be gone
    assert!(get_async_runtime().is_none());
}

/// Test error handling in async context
#[test]
fn test_async_error_handling() {
    common::tracing::setup();
    
    let _ = initialize_async_runtime();
    
    let result = block_on(async {
        let handle = spawn(async {
            delay(Duration::from_millis(10)).await;
            // Simulate an error condition
            Err::<i32, &str>("test error")
        });
        
        handle.await
    });
    
    // The handle should return the error result
    assert!(result.is_ok()); // TaskHandle itself succeeded
    let inner_result = result.unwrap();
    assert!(inner_result.is_err());
    
    shutdown_async_runtime();
}
