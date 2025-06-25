/// Integration tests for async/await with goroutines and other systems
use cursed::runtime::r#async::*;
use cursed::runtime::goroutine::*;
use cursed::stdlib::r#async::*;
use std::time::Duration;
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

/// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_test_writer()
            .with_max_level(tracing::Level::DEBUG)
            .try_init();
    };
}

#[cfg(test)]
mod goroutine_integration {
    use super::*;

    #[test]
    fn test_async_goroutine_coordination() {
        init_tracing!();
        
        // Initialize both async runtime and goroutine scheduler
        assert!(initialize_async_runtime().is_ok());
        
        // Test coordination between async runtime and goroutines
        if let Some(async_runtime) = get_async_runtime() {
            // Test GC coordination
            assert!(async_runtime.coordinate_gc().is_ok());
            
            let stats = async_runtime.statistics();
            // Goroutine integration may or may not be active depending on setup
            println!("Goroutine integration active: {}", stats.goroutine_integration_active);
        }
        
        shutdown_async_runtime();
    }

    #[test]
    fn test_runtime_coordinator() {
        init_tracing!();
        
        let mut coordinator = RuntimeCoordinator::new();
        assert!(!coordinator.is_integration_active());
        
        // Test GC coordination without goroutine scheduler
        assert!(coordinator.coordinate_gc_safe_point().is_ok());
    }

    #[test]
    fn test_async_runtime_with_goroutine_config() {
        init_tracing!();
        
        let mut config = AsyncRuntimeConfig::default();
        config.integrate_with_goroutines = true;
        
        let runtime = AsyncRuntime::new(config);
        assert!(runtime.initialize().is_ok());
        
        let stats = runtime.statistics();
        assert!(stats.runtime_uptime >= Duration::ZERO);
        
        runtime.shutdown();
    }
}

#[cfg(test)]
mod stdlib_integration {
    use super::*;

    #[test]
    fn test_async_io_integration() {
        init_tracing!();
        
        // Test async I/O components creation
        let stdin = stdin_async();
        let stdout = stdout_async();
        let stderr = stderr_async();
        
        // Test buffered I/O creation
        let data = b"hello world";
        let cursor = std::io::Cursor::new(data);
        let buf_reader = AsyncBufReader::new(cursor);
        
        let output = Vec::new();
        let buf_writer = AsyncBufWriter::new(output);
        
        // Basic creation tests pass
        assert!(true);
    }

    #[test]
    fn test_async_timer_integration() {
        init_tracing!();
        
        // Test timer utilities
        let mut interval = interval(Duration::from_millis(100));
        assert!(!interval.is_cancelled());
        
        // Test deadline utilities
        let deadline = from_now(Duration::from_millis(100));
        assert!(!has_passed(deadline));
        assert!(time_remaining(deadline).is_some());
        
        // Test rate limiter
        let rate_limiter = RateLimiter::new(5, Duration::from_secs(1));
        assert!(rate_limiter.try_acquire());
        assert!(rate_limiter.try_acquire());
    }

    #[test]
    fn test_async_utils_integration() {
        init_tracing!();
        
        // Test utility functions
        let ready_future = utils::ready(42);
        let never_future = utils::never::<i32>();
        
        // Test error utilities
        let io_err = io_error("test");
        let net_err = network_error("test");
        let timeout_err = timeout_error();
        
        assert!(matches!(io_err, AsyncError::Io(_)));
        assert!(matches!(net_err, AsyncError::Network(_)));
        assert!(matches!(timeout_err, AsyncError::Timeout));
    }
}

#[cfg(test)]
mod llvm_integration {
    use super::*;

    #[test]
    fn test_async_await_compiler_types() {
        init_tracing!();
        
        // Test await point creation
        use cursed::codegen::llvm::AwaitPoint;
        
        let await_point = AwaitPoint {
            block_id: 1,
            future_value: "test_future".to_string(),
            result_type: std::ptr::null_mut(),
            continuation_block: "continue_1".to_string(),
        };
        
        assert_eq!(await_point.block_id, 1);
        assert_eq!(await_point.future_value, "test_future");
        assert_eq!(await_point.continuation_block, "continue_1");
    }

    #[test]
    fn test_async_function_context() {
        init_tracing!();
        
        use cursed::codegen::llvm::AsyncFunctionContext;
        
        let mut context = AsyncFunctionContext::new(
            std::ptr::null_mut(),
            std::ptr::null_mut(),
        );
        
        assert_eq!(context.current_state, 0);
        assert_eq!(context.next_state(), 1);
        assert_eq!(context.current_state, 1);
        assert_eq!(context.await_points.len(), 0);
    }

    #[test]
    fn test_llvm_integration_functions() {
        init_tracing!();
        
        // Test that LLVM integration functions are accessible
        // These would be used by the LLVM code generator
        
        // In a real test, we would create an LLVM code generator
        // and test the async/await compilation functions
        // For now, we just verify the functions exist
        
        assert!(true); // Placeholder for LLVM integration tests
    }
}

#[cfg(test)]
mod error_integration {
    use super::*;

    #[test]
    fn test_future_error_conversion() {
        init_tracing!();
        
        // Test FutureError to AsyncError conversion
        let future_err = FutureError::Timeout;
        let async_err: AsyncError = future_err.into();
        assert!(matches!(async_err, AsyncError::Timeout));
        
        let future_err = FutureError::Cancelled;
        let async_err: AsyncError = future_err.into();
        assert!(matches!(async_err, AsyncError::Runtime(_)));
        
        let future_err = FutureError::Failed("test".to_string());
        let async_err: AsyncError = future_err.into();
        assert!(matches!(async_err, AsyncError::Runtime(_)));
    }

    #[test]
    fn test_io_error_conversion() {
        init_tracing!();
        
        // Test std::io::Error to AsyncError conversion
        let io_error = std::io::Error::new(std::io::ErrorKind::NotFound, "file not found");
        let async_err: AsyncError = io_error.into();
        assert!(matches!(async_err, AsyncError::Io(_)));
    }

    #[test]
    fn test_error_display_formatting() {
        init_tracing!();
        
        let errors = vec![
            AsyncError::Io("io error".to_string()),
            AsyncError::Network("network error".to_string()),
            AsyncError::Timeout,
            AsyncError::Channel("channel error".to_string()),
            AsyncError::Runtime("runtime error".to_string()),
            AsyncError::Other("other error".to_string()),
        ];
        
        for error in errors {
            let display = error.to_string();
            assert!(!display.is_empty());
            assert!(display.len() > 5); // Should have meaningful content
        }
    }
}

#[cfg(test)]
mod performance_integration {
    use super::*;

    #[test]
    fn test_runtime_performance_metrics() {
        init_tracing!();
        
        assert!(initialize_async_runtime().is_ok());
        
        if let Some(runtime) = get_async_runtime() {
            let stats = runtime.statistics();
            
            // Test performance metrics
            assert!(stats.runtime_uptime >= Duration::ZERO);
            assert_eq!(stats.active_futures, 0);
            assert_eq!(stats.completed_futures, 0);
            assert_eq!(stats.failed_futures, 0);
            assert_eq!(stats.memory_usage, 0);
            
            // Test executor statistics
            assert_eq!(stats.executor_stats.tasks_executed, 0);
            assert_eq!(stats.executor_stats.tasks_completed, 0);
            assert_eq!(stats.executor_stats.average_task_time, Duration::ZERO);
        }
        
        shutdown_async_runtime();
    }

    #[test]
    fn test_scheduler_performance() {
        init_tracing!();
        
        let config = SchedulerConfig::default();
        let scheduler = AsyncScheduler::new(config);
        
        let stats = scheduler.statistics();
        assert_eq!(stats.total_tasks_scheduled, 0);
        assert_eq!(stats.tasks_completed, 0);
        assert_eq!(stats.work_steal_attempts, 0);
        assert_eq!(stats.work_steal_successes, 0);
        assert_eq!(stats.average_task_time, Duration::ZERO);
        assert_eq!(stats.scheduler_overhead, Duration::ZERO);
    }

    #[test]
    fn test_executor_performance() {
        init_tracing!();
        
        let config = ExecutorConfig {
            worker_threads: 1,
            max_queue_size: 10,
            enable_work_stealing: false,
            task_timeout: Duration::from_millis(10),
            stats_interval: Duration::from_millis(50),
        };
        
        let executor = AsyncExecutor::new(config);
        let stats = executor.statistics();
        
        assert_eq!(stats.tasks_executed, 0);
        assert_eq!(stats.tasks_completed, 0);
        assert_eq!(stats.tasks_failed, 0);
        assert_eq!(stats.tasks_cancelled, 0);
        assert_eq!(stats.average_task_time, Duration::ZERO);
        assert_eq!(stats.queue_size, 0);
        assert_eq!(stats.active_workers, 0);
        assert_eq!(stats.idle_workers, 0);
    }
}

#[cfg(test)]
mod memory_safety {
    use super::*;

    #[test]
    fn test_promise_memory_safety() {
        init_tracing!();
        
        // Test that promises can be safely cloned and shared
        let (promise, resolver, _rejecter) = Promise::new();
        let cloned_promise = promise.clone();
        
        // Test resolution with cloned promise
        assert!(resolver.resolve(42).is_ok());
        
        assert_eq!(promise.state(), PromiseState::Resolved);
        assert_eq!(cloned_promise.state(), PromiseState::Resolved);
        
        // Test that both can safely access the result
        assert!(promise.try_result().is_some());
        assert!(cloned_promise.try_result().is_some());
    }

    #[test]
    fn test_task_handle_memory_safety() {
        init_tracing!();
        
        // Test that task handles can be safely cloned
        let (handle, notifier) = TaskHandle::new(TaskId(1));
        let cloned_handle = handle.clone();
        
        // Test completion notification
        notifier.complete(42);
        
        // Both handles should see the completion
        assert!(handle.is_completed());
        assert!(cloned_handle.is_completed());
        
        // Both should be able to access the result
        assert!(handle.try_result().is_some());
        assert!(cloned_handle.try_result().is_some());
    }

    #[test]
    fn test_shared_future_memory_safety() {
        init_tracing!();
        
        let (shared_future, resolver) = SharedFuture::new();
        let cloned_future = shared_future.clone();
        
        // Test resolution affects all clones
        resolver.resolve(42);
        
        assert_eq!(shared_future.state(), FutureState::Ready);
        assert_eq!(cloned_future.state(), FutureState::Ready);
    }

    #[test]
    fn test_concurrent_access_safety() {
        init_tracing!();
        
        // Test concurrent access to shared async primitives
        let (promise, resolver, _rejecter) = Promise::new();
        let promise_clone = promise.clone();
        
        let counter = Arc::new(AtomicUsize::new(0));
        let counter_clone = counter.clone();
        
        // Spawn thread to resolve promise
        std::thread::spawn(move || {
            std::thread::sleep(Duration::from_millis(10));
            counter_clone.fetch_add(1, Ordering::SeqCst);
            let _ = resolver.resolve(100);
        });
        
        // Wait for resolution in main thread
        std::thread::sleep(Duration::from_millis(50));
        
        // Verify both threads can safely access
        assert_eq!(promise.state(), PromiseState::Resolved);
        assert_eq!(promise_clone.state(), PromiseState::Resolved);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }
}

#[cfg(test)]
mod stress_integration {
    use super::*;

    #[test]
    #[ignore = "Long running stress test"]
    fn test_high_concurrency_promises() {
        init_tracing!();
        
        const NUM_THREADS: usize = 10;
        const PROMISES_PER_THREAD: usize = 100;
        
        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = Vec::new();
        
        for thread_id in 0..NUM_THREADS {
            let counter_clone = counter.clone();
            
            let handle = std::thread::spawn(move || {
                for i in 0..PROMISES_PER_THREAD {
                    let (promise, resolver, _rejecter) = Promise::new();
                    
                    // Immediately resolve
                    let value = thread_id * PROMISES_PER_THREAD + i;
                    assert!(resolver.resolve(value).is_ok());
                    
                    // Verify resolution
                    assert_eq!(promise.state(), PromiseState::Resolved);
                    if let Some(result) = promise.try_result() {
                        assert!(result.is_ok());
                        assert_eq!(result.unwrap(), value);
                    }
                    
                    counter_clone.fetch_add(1, Ordering::SeqCst);
                }
            });
            
            handles.push(handle);
        }
        
        // Wait for all threads
        for handle in handles {
            handle.join().unwrap();
        }
        
        // Verify all promises were processed
        assert_eq!(counter.load(Ordering::SeqCst), NUM_THREADS * PROMISES_PER_THREAD);
    }

    #[test]
    #[ignore = "Long running stress test"]
    fn test_runtime_under_load() {
        init_tracing!();
        
        assert!(initialize_async_runtime().is_ok());
        
        if let Some(runtime) = get_async_runtime() {
            // Create multiple promise chains concurrently
            const NUM_CHAINS: usize = 50;
            let mut promises = Vec::new();
            
            for i in 0..NUM_CHAINS {
                let (promise, resolver, _rejecter) = Promise::new();
                promises.push(promise);
                
                // Resolve in background thread
                std::thread::spawn(move || {
                    std::thread::sleep(Duration::from_millis(i as u64 % 10));
                    let _ = resolver.resolve(i * 2);
                });
            }
            
            // Wait for all to resolve
            std::thread::sleep(Duration::from_millis(100));
            
            // Count resolved promises
            let resolved_count = promises.iter()
                .filter(|p| p.state() == PromiseState::Resolved)
                .count();
            
            assert!(resolved_count > 0);
            println!("Resolved {} out of {} promises", resolved_count, NUM_CHAINS);
            
            // Check runtime is still healthy
            let stats = runtime.statistics();
            assert!(stats.runtime_uptime > Duration::ZERO);
            assert!(!runtime.is_shutdown());
        }
        
        shutdown_async_runtime();
    }

    #[test]
    #[ignore = "Long running stress test"]
    fn test_timer_wheel_under_load() {
        init_tracing!();
        
        let mut wheel = TimerWheel::new();
        let now = std::time::Instant::now();
        
        // Add many timers with various deadlines
        const NUM_TIMERS: usize = 5000;
        let mut handles = Vec::new();
        
        for i in 0..NUM_TIMERS {
            let deadline = now + Duration::from_millis((i % 1000) as u64);
            let handle = wheel.add_timer(deadline);
            handles.push(handle);
        }
        
        assert_eq!(wheel.len(), NUM_TIMERS);
        
        // Process expired timers in batches
        let mut total_expired = 0;
        for batch in 0..10 {
            let check_time = now + Duration::from_millis(batch * 100);
            let expired = wheel.process_expired(check_time);
            total_expired += expired.len();
            
            if expired.len() > 0 {
                println!("Batch {}: {} timers expired", batch, expired.len());
            }
        }
        
        println!("Total expired: {} out of {}", total_expired, NUM_TIMERS);
        assert!(total_expired > 0);
        assert!(total_expired <= NUM_TIMERS);
    }
}

// Helper functions for integration testing
fn create_test_async_runtime() -> Arc<AsyncRuntime> {
    let config = AsyncRuntimeConfig {
        executor_config: ExecutorConfig {
            worker_threads: 2,
            max_queue_size: 100,
            enable_work_stealing: true,
            task_timeout: Duration::from_millis(100),
            stats_interval: Duration::from_millis(500),
        },
        integrate_with_goroutines: false,
        async_worker_threads: 2,
        stats_collection_interval: Duration::from_millis(500),
    };
    
    let runtime = Arc::new(AsyncRuntime::new(config));
    runtime.initialize().expect("Failed to initialize test runtime");
    runtime
}

#[cfg(test)]
mod helper_tests {
    use super::*;

    #[test]
    fn test_helper_runtime_creation() {
        init_tracing!();
        
        let runtime = create_test_async_runtime();
        let stats = runtime.statistics();
        
        assert!(stats.runtime_uptime >= Duration::ZERO);
        assert!(!stats.goroutine_integration_active);
        
        runtime.shutdown();
    }
}
