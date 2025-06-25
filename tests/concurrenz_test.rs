/// Tests for ConcurrenZ module - Synchronization primitives with Gen Z flair
/// 
/// This test suite validates the comprehensive concurrency functionality
/// provided by the ConcurrenZ module, ensuring all synchronization primitives
/// work correctly with Gen Z naming conventions.

#[cfg(test)]
mod tests {
    use cursed::stdlib::concurrenz::*;
    use std::time::Duration;
    use std::sync::Arc;

    #[test]
    fn test_mutex_vibes() {
        let mutex = new_mutex_vibes(42);
        
        // Test basic locking
        {
            let guard = mutex.lock_it().unwrap();
            assert_eq!(*guard, 42);
        }
        
        // Test mutation
        {
            let mut guard = mutex.lock_it().unwrap();
            *guard = 100;
        }
        
        // Verify mutation
        {
            let guard = mutex.lock_it().unwrap();
            assert_eq!(*guard, 100);
        }
        
        // Test try_lock
        {
            let _guard = mutex.lock_it().unwrap();
            // This should fail because mutex is already locked
            assert!(mutex.try_lock_it().is_err());
        }
        
        // Now try_lock should succeed
        let guard = mutex.try_lock_it().unwrap();
        assert_eq!(*guard, 100);
    }

    #[test]
    fn test_rwlock_vibes() {
        let rwlock = new_rwlock_vibes(42);
        
        // Test multiple readers
        {
            let read_guard1 = rwlock.read_it().unwrap();
            let read_guard2 = rwlock.read_it().unwrap();
            assert_eq!(*read_guard1, 42);
            assert_eq!(*read_guard2, 42);
        }
        
        // Test writer exclusivity
        {
            let mut write_guard = rwlock.write_it().unwrap();
            *write_guard = 100;
        }
        
        // Verify write
        {
            let read_guard = rwlock.read_it().unwrap();
            assert_eq!(*read_guard, 100);
        }
        
        // Test try operations
        {
            let read_guard = rwlock.try_read_it().unwrap();
            assert_eq!(*read_guard, 100);
            
            // Should be able to get another read lock
            let read_guard2 = rwlock.try_read_it().unwrap();
            assert_eq!(*read_guard2, 100);
        }
    }

    #[test]
    fn test_atomic_bool_vibes() {
        let atomic = new_atomic_bool_vibes(false);
        
        // Test basic operations
        assert_eq!(atomic.load_it(), false);
        
        atomic.store_it(true);
        assert_eq!(atomic.load_it(), true);
        
        // Test swap
        let old_value = atomic.swap_it(false);
        assert_eq!(old_value, true);
        assert_eq!(atomic.load_it(), false);
        
        // Test compare and swap
        let old_value = atomic.compare_and_swap_it(false, true);
        assert_eq!(old_value, false);
        assert_eq!(atomic.load_it(), true);
        
        // Test failed compare and swap
        let old_value = atomic.compare_and_swap_it(false, false);
        assert_eq!(old_value, true); // Should return current value
        assert_eq!(atomic.load_it(), true); // Should remain unchanged
    }

    #[test]
    fn test_atomic_int_vibes() {
        let atomic = new_atomic_int_vibes(10);
        
        // Test basic operations
        assert_eq!(atomic.load_it(), 10);
        
        atomic.store_it(20);
        assert_eq!(atomic.load_it(), 20);
        
        // Test swap
        let old_value = atomic.swap_it(30);
        assert_eq!(old_value, 20);
        assert_eq!(atomic.load_it(), 30);
        
        // Test compare and swap
        let old_value = atomic.compare_and_swap_it(30, 40);
        assert_eq!(old_value, 30);
        assert_eq!(atomic.load_it(), 40);
        
        // Test arithmetic operations
        let old_value = atomic.fetch_add_it(5);
        assert_eq!(old_value, 40);
        assert_eq!(atomic.load_it(), 45);
        
        let old_value = atomic.fetch_sub_it(10);
        assert_eq!(old_value, 45);
        assert_eq!(atomic.load_it(), 35);
        
        // Test increment/decrement
        let new_value = atomic.increment_it();
        assert_eq!(new_value, 36);
        assert_eq!(atomic.load_it(), 36);
        
        let new_value = atomic.decrement_it();
        assert_eq!(new_value, 35);
        assert_eq!(atomic.load_it(), 35);
    }

    #[test]
    fn test_channel_vibes() {
        let (sender, receiver) = channel_vibes();
        
        // Test basic send and receive
        sender.send_it(42).unwrap();
        let value = receiver.receive_it().unwrap();
        assert_eq!(value, 42);
        
        // Test try operations
        sender.try_send_it(100).unwrap();
        let value = receiver.try_receive_it().unwrap();
        assert_eq!(value, 100);
        
        // Test empty channel
        assert!(receiver.try_receive_it().is_err());
        
        // Test multiple sends
        for i in 0..5 {
            sender.send_it(i).unwrap();
        }
        
        for i in 0..5 {
            let value = receiver.receive_it().unwrap();
            assert_eq!(value, i);
        }
    }

    #[test]
    fn test_thread_vibes() {
        // Test basic thread spawning
        let handle = spawn_thread_vibes(|| {
            42
        });
        
        let result = handle.join_it().unwrap();
        assert_eq!(result, 42);
        
        // Test named thread
        let handle = spawn_named_thread_vibes("test_thread", || {
            "hello from thread".to_string()
        });
        
        let result = handle.join_it().unwrap();
        assert_eq!(result, "hello from thread");
        
        // Test thread with computation
        let handle = spawn_thread_vibes(|| {
            let mut sum = 0;
            for i in 1..=10 {
                sum += i;
            }
            sum
        });
        
        let result = handle.join_it().unwrap();
        assert_eq!(result, 55); // Sum of 1..10
    }

    #[test]
    fn test_barrier_vibes() {
        let barrier = new_barrier_vibes(2);
        let barrier_clone = barrier.clone();
        
        let handle = spawn_thread_vibes(move || {
            barrier_clone.wait_it();
            42
        });
        
        // Both threads should reach the barrier
        barrier.wait_it();
        let result = handle.join_it().unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_condvar_vibes() {
        let mutex = Arc::new(new_mutex_vibes(false));
        let condvar = Arc::new(new_condvar_vibes());
        
        let mutex_clone = Arc::clone(&mutex);
        let condvar_clone = Arc::clone(&condvar);
        
        let handle = spawn_thread_vibes(move || {
            let guard = mutex_clone.lock_it().unwrap();
            condvar_clone.wait_it(guard).unwrap();
            42
        });
        
        // Give the other thread time to start waiting
        sleep_vibes(Duration::from_millis(10));
        
        // Signal the waiting thread
        {
            let mut guard = mutex.lock_it().unwrap();
            *guard = true;
        }
        condvar.notify_one_it();
        
        let result = handle.join_it().unwrap();
        assert_eq!(result, 42);
    }

    #[test]
    fn test_once_vibes() {
        let once = new_once_vibes();
        let counter = Arc::new(new_atomic_int_vibes(0));
        
        assert!(!once.is_completed());
        
        // Test that function is called exactly once
        let counter_clone1 = Arc::clone(&counter);
        let counter_clone2 = Arc::clone(&counter);
        let counter_clone3 = Arc::clone(&counter);
        
        once.call_once_it(move || {
            counter_clone1.increment_it();
        });
        
        // Multiple calls should not execute the function again
        once.call_once_it(move || {
            counter_clone2.increment_it();
        });
        
        once.call_once_it(move || {
            counter_clone3.increment_it();
        });
        
        assert!(once.is_completed());
        assert_eq!(counter.load_it(), 1);
    }

    #[test]
    fn test_utility_functions() {
        // Test CPU count
        let cores = num_cpus_vibes();
        assert!(cores > 0);
        assert!(cores <= 1024); // Reasonable upper bound
        
        // Test thread ID
        let thread_id = current_thread_id_vibes();
        assert_eq!(thread_id, std::thread::current().id());
        
        // Test thread name (main thread may or may not have a name)
        let _thread_name = current_thread_name_vibes();
        
        // Test yield
        yield_vibes(); // Should not panic
        
        // Test sleep
        let start = std::time::Instant::now();
        sleep_vibes(Duration::from_millis(10));
        let elapsed = start.elapsed();
        assert!(elapsed >= Duration::from_millis(10));
    }

    #[test]
    fn test_concurrent_operations() {
        // Test concurrent access to shared data
        let counter = Arc::new(new_atomic_int_vibes(0));
        let num_threads = 4;
        let increments_per_thread = 1000;
        
        let mut handles = Vec::new();
        
        for _ in 0..num_threads {
            let counter_clone = Arc::clone(&counter);
            let handle = spawn_thread_vibes(move || {
                for _ in 0..increments_per_thread {
                    counter_clone.increment_it();
                }
            });
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join_it().unwrap();
        }
        
        assert_eq!(counter.load_it(), num_threads * increments_per_thread);
    }

    #[test]
    fn test_producer_consumer() {
        let (sender, receiver) = channel_vibes();
        let num_items = 100;
        
        // Producer thread
        let sender_clone = sender.clone();
        let producer = spawn_thread_vibes(move || {
            for i in 0..num_items {
                sender_clone.send_it(i).unwrap();
            }
        });
        
        // Consumer thread
        let consumer = spawn_thread_vibes(move || {
            let mut sum = 0;
            for _ in 0..num_items {
                let value = receiver.receive_it().unwrap();
                sum += value;
            }
            sum
        });
        
        producer.join_it().unwrap();
        let result = consumer.join_it().unwrap();
        
        // Sum of 0..99 = 99 * 100 / 2 = 4950
        assert_eq!(result, (num_items - 1) * num_items / 2);
    }

    #[test]
    fn test_rwlock_concurrent_readers() {
        let rwlock = Arc::new(new_rwlock_vibes(42));
        let num_readers = 4;
        
        let mut handles = Vec::new();
        
        for i in 0..num_readers {
            let rwlock_clone = Arc::clone(&rwlock);
            let handle = spawn_thread_vibes(move || {
                let guard = rwlock_clone.read_it().unwrap();
                assert_eq!(*guard, 42);
                i // Return thread index
            });
            handles.push(handle);
        }
        
        // All readers should complete successfully
        for (i, handle) in handles.into_iter().enumerate() {
            let result = handle.join_it().unwrap();
            assert_eq!(result, i);
        }
    }

    #[test]
    fn test_module_functions() {
        // Test module initialization
        assert!(init_concurrenz().is_ok());
        
        // Test module statistics
        let stats = get_concurrenz_stats();
        assert!(stats.contains_key("version"));
        assert!(stats.contains_key("primitives"));
        assert!(stats.contains_key("features"));
        assert!(stats.contains_key("cpu_cores"));
        
        assert_eq!(stats.get("version").unwrap(), "1.0.0");
        assert!(stats.get("primitives").unwrap().contains("Mutex"));
        assert!(stats.get("primitives").unwrap().contains("RwLock"));
        assert!(stats.get("primitives").unwrap().contains("Atomic"));
        assert!(stats.get("features").unwrap().contains("Gen Z"));
        
        let cpu_cores_str = stats.get("cpu_cores").unwrap();
        let cpu_cores: usize = cpu_cores_str.parse().unwrap();
        assert!(cpu_cores > 0);
    }

    #[test]
    fn test_parking_operations() {
        // Test parking with timeout
        let handle = spawn_thread_vibes(|| {
            park_timeout_vibes(Duration::from_millis(10));
            42
        });
        
        let result = handle.join_it().unwrap();
        assert_eq!(result, 42);
        
        // Test park and unpark (difficult to test reliably in unit tests)
        // This is more of a smoke test
        let handle = spawn_thread_vibes(|| {
            // Park with a very short timeout to avoid hanging tests
            park_timeout_vibes(Duration::from_millis(1));
            100
        });
        
        let result = handle.join_it().unwrap();
        assert_eq!(result, 100);
    }

    #[test]
    fn test_channel_with_timeout() {
        let (sender, receiver) = channel_vibes();
        
        // Test receive with timeout on empty channel
        let start = std::time::Instant::now();
        let result = receiver.receive_timeout_it(Duration::from_millis(50));
        let elapsed = start.elapsed();
        
        assert!(result.is_err());
        assert!(elapsed >= Duration::from_millis(50));
        assert!(elapsed < Duration::from_millis(100)); // Should not take too long
        
        // Test successful receive within timeout
        sender.send_it(42).unwrap();
        let result = receiver.receive_timeout_it(Duration::from_millis(100));
        assert_eq!(result.unwrap(), 42);
    }

    #[test]
    fn test_barrier_multiple_threads() {
        let barrier = Arc::new(new_barrier_vibes(3));
        let mut handles = Vec::new();
        
        for i in 0..3 {
            let barrier_clone = Arc::clone(&barrier);
            let handle = spawn_thread_vibes(move || {
                // All threads should reach this point before any continue
                barrier_clone.wait_it();
                i * 10
            });
            handles.push(handle);
        }
        
        // Collect results
        let mut results = Vec::new();
        for handle in handles {
            results.push(handle.join_it().unwrap());
        }
        
        results.sort();
        assert_eq!(results, vec![0, 10, 20]);
    }

    #[test]
    fn test_multiple_condvar_notifications() {
        let mutex = Arc::new(new_mutex_vibes(0));
        let condvar = Arc::new(new_condvar_vibes());
        let num_threads = 3;
        
        let mut handles = Vec::new();
        
        for i in 0..num_threads {
            let mutex_clone = Arc::clone(&mutex);
            let condvar_clone = Arc::clone(&condvar);
            
            let handle = spawn_thread_vibes(move || {
                let guard = mutex_clone.lock_it().unwrap();
                condvar_clone.wait_it(guard).unwrap();
                i
            });
            handles.push(handle);
        }
        
        // Give threads time to start waiting
        sleep_vibes(Duration::from_millis(10));
        
        // Notify all waiting threads
        condvar.notify_all_it();
        
        // Collect results
        let mut results = Vec::new();
        for handle in handles {
            results.push(handle.join_it().unwrap());
        }
        
        results.sort();
        assert_eq!(results, vec![0, 1, 2]);
    }
}
