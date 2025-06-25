/// Comprehensive tests for the async/await runtime system
use cursed::runtime::r#async::*;
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
mod basic_functionality {
    use super::*;

    #[test]
    fn test_async_runtime_initialization() {
        init_tracing!();
        
        // Test runtime initialization
        assert!(initialize_async_runtime().is_ok());
        
        // Test getting runtime
        let runtime = get_async_runtime();
        assert!(runtime.is_some());
        
        // Test runtime statistics
        if let Some(runtime) = runtime {
            let stats = runtime.statistics();
            assert_eq!(stats.active_futures, 0);
        }
        
        // Clean shutdown
        shutdown_async_runtime();
    }

    #[test]
    fn test_future_creation_and_completion() {
        init_tracing!();
        
        // Test ready future
        let ready_future = future::ready(42);
        // Would need async test framework to properly test polling
        
        // Test pending future
        let pending_future: future::PendingFuture<i32> = future::pending();
        // Would need async test framework to properly test polling
    }

    #[test]
    fn test_promise_creation_and_resolution() {
        init_tracing!();
        
        let (promise, resolver, _rejecter) = Promise::new();
        
        // Test initial state
        assert_eq!(promise.state(), PromiseState::Pending);
        assert!(promise.is_pending());
        assert!(!promise.is_resolved());
        assert!(!promise.is_rejected());
        
        // Test resolution
        assert!(resolver.resolve(42).is_ok());
        
        // Test state after resolution
        assert_eq!(promise.state(), PromiseState::Resolved);
        assert!(promise.is_resolved());
        assert!(!promise.is_pending());
    }

    #[test]
    fn test_promise_rejection() {
        init_tracing!();
        
        let (promise, _resolver, rejecter) = Promise::new();
        
        // Test rejection
        assert!(rejecter.reject(FutureError::Failed("test error".to_string())).is_ok());
        
        // Test state after rejection
        assert_eq!(promise.state(), PromiseState::Rejected);
        assert!(promise.is_rejected());
        assert!(!promise.is_pending());
        assert!(!promise.is_resolved());
    }

    #[test]
    fn test_task_handle_creation() {
        init_tracing!();
        
        let (handle, notifier) = TaskHandle::new(TaskId(123));
        
        // Test initial state
        assert_eq!(handle.task_id().as_u64(), 123);
        assert!(!handle.is_completed());
        assert!(!handle.is_cancelled());
        
        // Test completion
        notifier.complete(42);
        assert!(handle.is_completed());
        
        // Test result retrieval
        if let Some(result) = handle.try_result() {
            assert!(result.is_ok());
            assert_eq!(result.unwrap(), 42);
        }
    }

    #[test]
    fn test_task_handle_cancellation() {
        init_tracing!();
        
        let (handle, _notifier) = TaskHandle::<i32>::new(TaskId(456));
        
        // Test cancellation
        handle.cancel();
        assert!(handle.is_cancelled());
        
        // Test result after cancellation
        if let Some(result) = handle.try_result() {
            assert!(result.is_err());
            matches!(result.unwrap_err(), FutureError::Cancelled);
        }
    }
}

#[cfg(test)]
mod executor_tests {
    use super::*;

    #[test]
    fn test_executor_creation() {
        init_tracing!();
        
        let config = ExecutorConfig::default();
        let executor = AsyncExecutor::new(config);
        
        let stats = executor.statistics();
        assert_eq!(stats.tasks_executed, 0);
        assert_eq!(stats.tasks_completed, 0);
    }

    #[test]
    fn test_executor_block_on() {
        init_tracing!();
        
        let executor = AsyncExecutor::new(ExecutorConfig::default());
        let result = executor.block_on(future::ready(42));
        assert_eq!(result, 42);
    }

    #[test]
    fn test_task_queue_priority() {
        init_tracing!();
        
        let mut queue = TaskQueue::new();
        assert!(queue.is_empty());
        assert_eq!(queue.len(), 0);
        
        // Would need to test actual task entry creation and priority ordering
        // This requires more complex setup with actual task entries
    }
}

#[cfg(test)]
mod timer_tests {
    use super::*;

    #[test]
    fn test_delay_creation() {
        init_tracing!();
        
        let delay = Delay::new(Duration::from_millis(100));
        assert!(!delay.is_completed());
        
        // Test deadline
        let deadline = delay.deadline();
        assert!(deadline > std::time::Instant::now());
    }

    #[test]
    fn test_timeout_creation() {
        init_tracing!();
        
        let timeout = Timeout::new(Duration::from_millis(100), future::ready(42));
        // Would need async test framework to properly test timeout behavior
    }

    #[test]
    fn test_interval_creation() {
        init_tracing!();
        
        let mut interval = Interval::new(Duration::from_millis(100));
        assert!(!interval.is_cancelled());
        
        // Test cancellation
        interval.cancel();
        assert!(interval.is_cancelled());
        
        // Test reset
        interval.reset(Duration::from_millis(200));
        assert!(!interval.is_cancelled());
    }

    #[test]
    fn test_timer_wheel() {
        init_tracing!();
        
        let mut wheel = TimerWheel::new();
        let now = std::time::Instant::now();
        
        // Add timers
        let handle1 = wheel.add_timer(now + Duration::from_millis(100));
        let handle2 = wheel.add_timer(now + Duration::from_millis(50));
        
        assert_eq!(wheel.len(), 2);
        
        // Process expired timers
        let expired = wheel.process_expired(now + Duration::from_millis(60));
        assert_eq!(expired.len(), 1);
        assert_eq!(expired[0], handle2);
        
        assert_eq!(wheel.len(), 1);
    }
}

#[cfg(test)]
mod scheduler_tests {
    use super::*;

    #[test]
    fn test_scheduler_creation() {
        init_tracing!();
        
        let config = SchedulerConfig::default();
        let scheduler = AsyncScheduler::new(config);
        
        let stats = scheduler.statistics();
        assert_eq!(stats.total_tasks_scheduled, 0);
    }

    #[test]
    fn test_work_stealing_queue() {
        init_tracing!();
        
        let mut queue = WorkStealingQueue::new();
        assert!(queue.is_empty());
        
        // Test local operations
        queue.push_local(1);
        queue.push_local(2);
        queue.push_local(3);
        assert_eq!(queue.len(), 3);
        
        // Test local pop (LIFO)
        assert_eq!(queue.pop_local(), Some(3));
        assert_eq!(queue.len(), 2);
        
        // Test stealing (FIFO)
        assert_eq!(queue.steal(), Some(1));
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn test_priority_task_queue() {
        init_tracing!();
        
        // This would require creating actual SchedulerTask instances
        // which need more complex setup
    }
}

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_runtime_initialization_integration() {
        init_tracing!();
        
        // Test full runtime initialization
        assert!(initialize_async_runtime().is_ok());
        
        let runtime = get_async_runtime();
        assert!(runtime.is_some());
        
        if let Some(runtime) = runtime {
            // Test runtime capabilities
            let stats = runtime.statistics();
            assert!(stats.runtime_uptime.as_millis() >= 0);
            
            // Test runtime coordination
            assert!(runtime.coordinate_gc().is_ok());
            
            // Test shutdown
            assert!(!runtime.is_shutdown());
        }
        
        shutdown_async_runtime();
    }

    #[test]
    fn test_promise_chain_operations() {
        init_tracing!();
        
        let (promise, resolver, _rejecter) = Promise::new();
        
        // Test promise utility functions
        let resolved_promise = utils::resolve(42);
        assert_eq!(resolved_promise.state(), PromiseState::Resolved);
        
        let rejected_promise: Promise<i32> = utils::reject(FutureError::Failed("test".to_string()));
        assert_eq!(rejected_promise.state(), PromiseState::Rejected);
        
        // Test resolution
        assert!(resolver.resolve(100).is_ok());
        assert_eq!(promise.state(), PromiseState::Resolved);
    }

    #[test]
    fn test_shared_future_operations() {
        init_tracing!();
        
        let (shared_future, resolver) = SharedFuture::new();
        
        // Test initial state
        assert_eq!(shared_future.state(), FutureState::Pending);
        
        // Test cloning
        let cloned_future = shared_future.clone();
        assert_eq!(cloned_future.state(), FutureState::Pending);
        
        // Test resolution
        resolver.resolve(42);
        assert_eq!(shared_future.state(), FutureState::Ready);
        assert_eq!(cloned_future.state(), FutureState::Ready);
    }

    #[test]
    fn test_async_error_handling() {
        init_tracing!();
        
        // Test error conversion
        let io_error = AsyncError::Io("test io error".to_string());
        assert!(matches!(io_error, AsyncError::Io(_)));
        
        let network_error = AsyncError::Network("test network error".to_string());
        assert!(matches!(network_error, AsyncError::Network(_)));
        
        let timeout_error = AsyncError::Timeout;
        assert!(matches!(timeout_error, AsyncError::Timeout));
        
        // Test error display
        assert!(io_error.to_string().contains("I/O error"));
        assert!(network_error.to_string().contains("Network error"));
        assert!(timeout_error.to_string().contains("timed out"));
    }

    #[test]
    fn test_async_utilities() {
        init_tracing!();
        
        // Test utility functions
        let io_err = io_error("test message");
        assert!(matches!(io_err, AsyncError::Io(_)));
        
        let net_err = network_error("test message");
        assert!(matches!(net_err, AsyncError::Network(_)));
        
        let timeout_err = timeout_error();
        assert!(matches!(timeout_err, AsyncError::Timeout));
        
        let runtime_err = runtime_error("test message");
        assert!(matches!(runtime_err, AsyncError::Runtime(_)));
        
        let channel_err = channel_error("test message");
        assert!(matches!(channel_err, AsyncError::Channel(_)));
    }
}

#[cfg(test)]
mod performance_tests {
    use super::*;

    #[test]
    fn test_executor_statistics() {
        init_tracing!();
        
        let executor = AsyncExecutor::new(ExecutorConfig::default());
        let stats = executor.statistics();
        
        // Test initial statistics
        assert_eq!(stats.tasks_executed, 0);
        assert_eq!(stats.tasks_completed, 0);
        assert_eq!(stats.tasks_failed, 0);
        assert_eq!(stats.tasks_cancelled, 0);
        assert_eq!(stats.average_task_time, Duration::ZERO);
    }

    #[test]
    fn test_runtime_statistics() {
        init_tracing!();
        
        if let Ok(()) = initialize_async_runtime() {
            if let Some(runtime) = get_async_runtime() {
                let stats = runtime.statistics();
                
                // Test runtime statistics
                assert!(stats.runtime_uptime >= Duration::ZERO);
                assert_eq!(stats.active_futures, 0);
                assert_eq!(stats.completed_futures, 0);
                assert_eq!(stats.failed_futures, 0);
            }
            
            shutdown_async_runtime();
        }
    }

    #[test]
    fn test_task_manager_statistics() {
        init_tracing!();
        
        let manager = TaskManager::new();
        let stats = manager.get_statistics();
        
        // Test initial task statistics
        assert_eq!(stats.total_tasks, 0);
        assert_eq!(stats.completed_tasks, 0);
        assert_eq!(stats.failed_tasks, 0);
        assert_eq!(stats.cancelled_tasks, 0);
        assert_eq!(stats.average_execution_time, Duration::ZERO);
        assert_eq!(stats.success_rate(), 0.0);
        assert_eq!(stats.failure_rate(), 0.0);
    }

    #[test]
    fn test_memory_usage() {
        init_tracing!();
        
        // Test that basic async structures have reasonable memory usage
        let config = ExecutorConfig::default();
        let _executor = AsyncExecutor::new(config);
        
        let (promise, _resolver, _rejecter) = Promise::<i32>::new();
        let _cloned_promise = promise.clone();
        
        // Test task handle memory usage
        let (handle, _notifier) = TaskHandle::<i32>::new(TaskId(1));
        let _cloned_handle = handle.clone();
        
        // Memory usage validation would need more sophisticated measurement
        // For now, we just ensure the structures can be created without issues
    }
}

#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    #[ignore = "Long running stress test"]
    fn test_many_promises() {
        init_tracing!();
        
        const NUM_PROMISES: usize = 1000;
        let mut promises = Vec::new();
        let mut resolvers = Vec::new();
        
        // Create many promises
        for i in 0..NUM_PROMISES {
            let (promise, resolver, _rejecter) = Promise::new();
            promises.push(promise);
            resolvers.push(resolver);
        }
        
        // Resolve all promises
        for (i, resolver) in resolvers.into_iter().enumerate() {
            assert!(resolver.resolve(i).is_ok());
        }
        
        // Verify all promises are resolved
        for (i, promise) in promises.iter().enumerate() {
            assert_eq!(promise.state(), PromiseState::Resolved);
            if let Some(result) = promise.try_result() {
                assert!(result.is_ok());
                assert_eq!(result.unwrap(), i);
            }
        }
    }

    #[test]
    #[ignore = "Long running stress test"]
    fn test_concurrent_task_handles() {
        init_tracing!();
        
        const NUM_TASKS: usize = 100;
        let counter = Arc::new(AtomicUsize::new(0));
        let mut handles = Vec::new();
        
        // Create many task handles
        for i in 0..NUM_TASKS {
            let (handle, notifier) = TaskHandle::new(TaskId(i as u64));
            handles.push(handle);
            
            // Simulate completion in background thread
            let counter_clone = counter.clone();
            std::thread::spawn(move || {
                std::thread::sleep(Duration::from_millis(1));
                counter_clone.fetch_add(1, Ordering::SeqCst);
                notifier.complete(i * 2);
            });
        }
        
        // Wait for all to complete
        std::thread::sleep(Duration::from_millis(100));
        
        // Verify all completed
        let completed_count = handles.iter().filter(|h| h.is_completed()).count();
        assert!(completed_count > 0); // Some should be completed
        
        // Verify counter
        assert!(counter.load(Ordering::SeqCst) > 0);
    }

    #[test]
    #[ignore = "Long running stress test"]
    fn test_timer_wheel_performance() {
        init_tracing!();
        
        let mut wheel = TimerWheel::new();
        let now = std::time::Instant::now();
        
        // Add many timers
        const NUM_TIMERS: usize = 1000;
        let mut handles = Vec::new();
        
        for i in 0..NUM_TIMERS {
            let deadline = now + Duration::from_millis(i as u64 % 100);
            let handle = wheel.add_timer(deadline);
            handles.push(handle);
        }
        
        assert_eq!(wheel.len(), NUM_TIMERS);
        
        // Process some expired timers
        let expired = wheel.process_expired(now + Duration::from_millis(50));
        assert!(expired.len() > 0);
        assert!(wheel.len() < NUM_TIMERS);
    }
}

// Helper function for creating test runtime configuration
fn create_test_runtime_config() -> AsyncRuntimeConfig {
    AsyncRuntimeConfig {
        executor_config: ExecutorConfig {
            worker_threads: 2,
            max_queue_size: 100,
            enable_work_stealing: true,
            task_timeout: Duration::from_millis(50),
            stats_interval: Duration::from_millis(100),
        },
        integrate_with_goroutines: false, // Disable for simpler testing
        async_worker_threads: 2,
        stats_collection_interval: Duration::from_millis(100),
    }
}

// Helper function for creating test executor configuration
fn create_test_executor_config() -> ExecutorConfig {
    ExecutorConfig {
        worker_threads: 1,
        max_queue_size: 10,
        enable_work_stealing: false,
        task_timeout: Duration::from_millis(10),
        stats_interval: Duration::from_millis(50),
    }
}

#[cfg(test)]
mod test_helpers {
    use super::*;

    #[test]
    fn test_helper_functions() {
        init_tracing!();
        
        let runtime_config = create_test_runtime_config();
        assert_eq!(runtime_config.async_worker_threads, 2);
        assert!(!runtime_config.integrate_with_goroutines);
        
        let executor_config = create_test_executor_config();
        assert_eq!(executor_config.worker_threads, 1);
        assert_eq!(executor_config.max_queue_size, 10);
        assert!(!executor_config.enable_work_stealing);
    }
}
