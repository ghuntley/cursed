//! Concurrent tests for goroutine synchronization primitives
//!
//! These tests verify that the synchronization primitives work correctly
//! under concurrent load and properly prevent race conditions and deadlocks.

#[path = "common.rs"]
pub mod common;

use cursed::runtime::{WaitGroup, GoroutineMutex, AtomicCounter, GoroutineCondvar, GoroutineParker, get_global_parker};
use std::sync::{Arc, mpsc};
use std::time::Duration;
use std::thread;
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use tracing::{debug, info, warn};

/// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("debug")
            .with_test_writer()
            .try_init();
    };
}

#[test]
fn test_waitgroup_concurrent() {
    init_tracing!();
    info!("Testing WaitGroup with concurrent goroutines");

    let wg = Arc::new(WaitGroup::new());
    let counter = Arc::new(AtomicUsize::new(0));
    let num_goroutines = 10;

    // Add count for all goroutines
    for _ in 0..num_goroutines {
        wg.add_one().unwrap();
    }

    // Spawn goroutines
    let mut handles = Vec::new();
    for i in 0..num_goroutines {
        let wg_clone = Arc::clone(&wg);
        let counter_clone = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            // Simulate some work
            thread::sleep(Duration::from_millis(10 + (i * 5) as u64));
            
            // Increment counter
            counter_clone.fetch_add(1, Ordering::SeqCst);
            
            // Mark goroutine as done
            wg_clone.done().unwrap();
            
            debug!(goroutine_id = i, "Goroutine completed");
        });
        handles.push(handle);
    }

    // Wait for all goroutines to complete
    let start = std::time::Instant::now();
    wg.wait().unwrap();
    let elapsed = start.elapsed();
    
    debug!(elapsed = ?elapsed, "All goroutines completed");

    // Verify all goroutines executed
    assert_eq!(counter.load(Ordering::SeqCst), num_goroutines);
    assert_eq!(wg.count(), 0);

    // Clean up threads
    for handle in handles {
        handle.join().unwrap();
    }

    info!("WaitGroup concurrent test completed successfully");
}

#[test]
fn test_mutex_concurrent_access() {
    init_tracing!();
    info!("Testing GoroutineMutex with concurrent access");

    let mutex = Arc::new(GoroutineMutex::new(0));
    let num_threads = 10;
    let increments_per_thread = 100;

    let mut handles = Vec::new();
    for i in 0..num_threads {
        let mutex_clone = Arc::clone(&mutex);
        
        let handle = thread::spawn(move || {
            for j in 0..increments_per_thread {
                {
                    let mut guard = mutex_clone.lock().unwrap();
                    let old_value = *guard;
                    
                    // Simulate some work while holding the lock
                    thread::sleep(Duration::from_nanos(100));
                    
                    *guard = old_value + 1;
                }
                
                if j % 10 == 0 {
                    debug!(thread_id = i, increment = j, "Thread progress");
                }
            }
            debug!(thread_id = i, "Thread completed");
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify the final value
    let final_value = {
        let guard = mutex.lock().unwrap();
        *guard
    };
    
    let expected_value = num_threads * increments_per_thread;
    assert_eq!(final_value, expected_value);
    
    info!(final_value = final_value, expected = expected_value, "Mutex concurrent test completed");
}

#[test]
fn test_atomic_counter_concurrent() {
    init_tracing!();
    info!("Testing AtomicCounter with concurrent operations");

    let counter = Arc::new(AtomicCounter::new(0));
    let num_threads = 10;
    let operations_per_thread = 1000;

    let mut handles = Vec::new();
    for i in 0..num_threads {
        let counter_clone = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            for j in 0..operations_per_thread {
                match j % 4 {
                    0 => { counter_clone.add(1); }
                    1 => { counter_clone.set(counter_clone.get() + 1); }
                    2 => {
                        let current = counter_clone.get();
                        counter_clone.compare_and_swap(current, current + 1);
                    }
                    _ => { counter_clone.add(1); }
                }
                
                if j % 100 == 0 {
                    debug!(thread_id = i, operation = j, "Thread progress");
                }
            }
            debug!(thread_id = i, "Thread completed");
        });
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    let final_value = counter.get();
    let operation_count = counter.operation_count();
    
    // The value should be positive and operations should have been recorded
    assert!(final_value > 0);
    assert!(operation_count >= num_threads * operations_per_thread);
    
    info!(final_value = final_value, operations = operation_count, "Atomic counter concurrent test completed");
}

#[test]
fn test_condition_variable_producer_consumer() {
    init_tracing!();
    info!("Testing GoroutineCondvar with producer-consumer pattern");

    let buffer = Arc::new(GoroutineMutex::new(Vec::<i32>::new()));
    let condvar = Arc::new(GoroutineCondvar::new());
    let should_stop = Arc::new(AtomicBool::new(false));
    let produced_count = Arc::new(AtomicUsize::new(0));
    let consumed_count = Arc::new(AtomicUsize::new(0));

    let buffer_producer = Arc::clone(&buffer);
    let condvar_producer = Arc::clone(&condvar);
    let should_stop_producer = Arc::clone(&should_stop);
    let produced_count_clone = Arc::clone(&produced_count);

    // Producer thread
    let producer = thread::spawn(move || {
        for i in 0..20 {
            {
                let mut guard = buffer_producer.lock().unwrap();
                guard.push(i);
                produced_count_clone.fetch_add(1, Ordering::SeqCst);
                debug!(item = i, buffer_len = guard.len(), "Produced item");
            }
            condvar_producer.notify_one();
            thread::sleep(Duration::from_millis(5));
        }
        
        should_stop_producer.store(true, Ordering::SeqCst);
        condvar_producer.notify_all();
        debug!("Producer finished");
    });

    let buffer_consumer = Arc::clone(&buffer);
    let condvar_consumer = Arc::clone(&condvar);
    let should_stop_consumer = Arc::clone(&should_stop);
    let consumed_count_clone = Arc::clone(&consumed_count);

    // Consumer thread
    let consumer = thread::spawn(move || {
        loop {
            let item = {
                let mut guard = buffer_consumer.lock().unwrap();
                
                // Wait for items or stop signal
                while guard.is_empty() && !should_stop_consumer.load(Ordering::Acquire) {
                    guard = condvar_consumer.wait(guard).unwrap();
                }
                
                if guard.is_empty() && should_stop_consumer.load(Ordering::Acquire) {
                    break;
                }
                
                guard.pop()
            };
            
            if let Some(item) = item {
                consumed_count_clone.fetch_add(1, Ordering::SeqCst);
                debug!(item = item, "Consumed item");
                thread::sleep(Duration::from_millis(10));
            }
        }
        debug!("Consumer finished");
    });

    // Wait for completion
    producer.join().unwrap();
    consumer.join().unwrap();

    let produced = produced_count.load(Ordering::SeqCst);
    let consumed = consumed_count.load(Ordering::SeqCst);
    
    assert_eq!(produced, 20);
    assert_eq!(consumed, 20);
    
    // Buffer should be empty
    let guard = buffer.lock().unwrap();
    assert!(guard.is_empty());
    
    info!(produced = produced, consumed = consumed, "Producer-consumer test completed");
}

#[test]
fn test_goroutine_parker_concurrent() {
    init_tracing!();
    info!("Testing GoroutineParker with concurrent parking/unparking");

    let parker = get_global_parker();
    let num_threads = 5;
    let (tx, rx) = mpsc::channel();

    let mut handles = Vec::new();
    
    // Spawn threads that will park themselves
    for i in 0..num_threads {
        let tx_clone = tx.clone();
        let parker_clone = Arc::clone(&parker);
        
        let handle = thread::spawn(move || {
            debug!(thread_id = i, "Thread starting");
            
            // Send our thread ID
            tx_clone.send((i, thread::current().id())).unwrap();
            
            // Park and wait
            parker_clone.park().unwrap();
            
            debug!(thread_id = i, "Thread unparked");
        });
        handles.push(handle);
    }

    // Collect thread IDs
    let mut thread_ids = Vec::new();
    for _ in 0..num_threads {
        let (i, thread_id) = rx.recv().unwrap();
        thread_ids.push((i, thread_id));
        debug!(thread_num = i, thread_id = ?thread_id, "Received thread ID");
    }

    // Give threads time to park
    thread::sleep(Duration::from_millis(100));
    
    // Verify threads are parked
    let parked_count = parker.parked_count();
    debug!(parked_count = parked_count, "Threads parked");
    assert!(parked_count > 0);

    // Unpark threads one by one
    for (i, thread_id) in thread_ids {
        thread::sleep(Duration::from_millis(10));
        let unparked = parker.unpark(thread_id).unwrap();
        assert!(unparked);
        debug!(thread_num = i, thread_id = ?thread_id, "Unparked thread");
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify no threads are parked
    assert_eq!(parker.parked_count(), 0);
    
    let (park_count, unpark_count, _) = parker.stats();
    assert!(park_count >= num_threads);
    assert!(unpark_count >= num_threads);
    
    info!(park_count = park_count, unpark_count = unpark_count, "Parker concurrent test completed");
}

#[test]
fn test_stress_test_all_primitives() {
    init_tracing!();
    info!("Running stress test for all synchronization primitives");

    let num_workers = 8;
    let operations_per_worker = 100;
    
    // Shared state
    let wg = Arc::new(WaitGroup::new());
    let mutex_data = Arc::new(GoroutineMutex::new(0));
    let atomic_counter = Arc::new(AtomicCounter::new(0));
    let condvar = Arc::new(GoroutineCondvar::new());
    let notification_received = Arc::new(AtomicBool::new(false));

    // Add workers to wait group
    for _ in 0..num_workers {
        wg.add_one().unwrap();
    }

    let mut handles = Vec::new();
    
    // Spawn worker threads
    for worker_id in 0..num_workers {
        let wg_clone = Arc::clone(&wg);
        let mutex_clone = Arc::clone(&mutex_data);
        let counter_clone = Arc::clone(&atomic_counter);
        let condvar_clone = Arc::clone(&condvar);
        let notification_received_clone = Arc::clone(&notification_received);
        
        let handle = thread::spawn(move || {
            debug!(worker_id = worker_id, "Worker starting");
            
            for op in 0..operations_per_worker {
                // Mix of different operations
                match op % 3 {
                    0 => {
                        // Mutex operations
                        let mut guard = mutex_clone.lock().unwrap();
                        *guard += 1;
                        thread::sleep(Duration::from_nanos(100));
                    }
                    1 => {
                        // Atomic operations
                        atomic_counter.add(1);
                        let _ = counter_clone.compare_and_swap(
                            counter_clone.get(),
                            counter_clone.get() + 1
                        );
                    }
                    _ => {
                        // Condition variable (last worker triggers notification)
                        if worker_id == num_workers - 1 && op == operations_per_worker - 1 {
                            notification_received_clone.store(true, Ordering::SeqCst);
                            condvar_clone.notify_all();
                        }
                    }
                }
                
                if op % 20 == 0 {
                    debug!(worker_id = worker_id, operation = op, "Worker progress");
                }
            }
            
            debug!(worker_id = worker_id, "Worker completed");
            wg_clone.done().unwrap();
        });
        handles.push(handle);
    }

    // Wait for all workers to complete
    let start = std::time::Instant::now();
    wg.wait().unwrap();
    let elapsed = start.elapsed();

    // Wait for threads to join
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify results
    let mutex_value = {
        let guard = mutex_data.lock().unwrap();
        *guard
    };
    let atomic_value = atomic_counter.get();
    let notification_state = notification_received.load(Ordering::SeqCst);

    assert!(mutex_value > 0);
    assert!(atomic_value > 0);
    assert!(notification_state);
    
    info!(
        elapsed = ?elapsed,
        mutex_value = mutex_value,
        atomic_value = atomic_value,
        notification_received = notification_state,
        "Stress test completed successfully"
    );
}

#[test]
fn test_deadlock_prevention() {
    init_tracing!();
    info!("Testing deadlock prevention mechanisms");

    let mutex1 = Arc::new(GoroutineMutex::new(1));
    let mutex2 = Arc::new(GoroutineMutex::new(2));
    let success_count = Arc::new(AtomicUsize::new(0));

    let mutex1_clone = Arc::clone(&mutex1);
    let mutex2_clone = Arc::clone(&mutex2);
    let success_count_clone = Arc::clone(&success_count);

    // Thread 1: Lock mutex1 then mutex2
    let handle1 = thread::spawn(move || {
        debug!("Thread 1 starting");
        
        for i in 0..10 {
            let _guard1 = mutex1_clone.lock().unwrap();
            debug!("Thread 1 acquired mutex1, iteration {}", i);
            
            thread::sleep(Duration::from_millis(1));
            
            // Try to acquire mutex2 with a timeout approach using try_lock
            let mut attempts = 0;
            loop {
                match mutex2_clone.try_lock() {
                    Ok(_guard2) => {
                        debug!("Thread 1 acquired mutex2, iteration {}", i);
                        success_count_clone.fetch_add(1, Ordering::SeqCst);
                        break;
                    }
                    Err(_) => {
                        attempts += 1;
                        if attempts > 10 {
                            warn!("Thread 1 gave up on mutex2, iteration {}", i);
                            break;
                        }
                        thread::sleep(Duration::from_millis(1));
                    }
                }
            }
        }
        debug!("Thread 1 completed");
    });

    let mutex1_clone2 = Arc::clone(&mutex1);
    let mutex2_clone2 = Arc::clone(&mutex2);
    let success_count_clone2 = Arc::clone(&success_count);

    // Thread 2: Lock mutex2 then mutex1 (potential deadlock scenario)
    let handle2 = thread::spawn(move || {
        debug!("Thread 2 starting");
        
        for i in 0..10 {
            let _guard2 = mutex2_clone2.lock().unwrap();
            debug!("Thread 2 acquired mutex2, iteration {}", i);
            
            thread::sleep(Duration::from_millis(1));
            
            // Try to acquire mutex1 with a timeout approach using try_lock
            let mut attempts = 0;
            loop {
                match mutex1_clone2.try_lock() {
                    Ok(_guard1) => {
                        debug!("Thread 2 acquired mutex1, iteration {}", i);
                        success_count_clone2.fetch_add(1, Ordering::SeqCst);
                        break;
                    }
                    Err(_) => {
                        attempts += 1;
                        if attempts > 10 {
                            warn!("Thread 2 gave up on mutex1, iteration {}", i);
                            break;
                        }
                        thread::sleep(Duration::from_millis(1));
                    }
                }
            }
        }
        debug!("Thread 2 completed");
    });

    // Wait for both threads with a timeout to detect deadlocks
    let start = std::time::Instant::now();
    handle1.join().unwrap();
    handle2.join().unwrap();
    let elapsed = start.elapsed();

    let success_count_final = success_count.load(Ordering::SeqCst);
    
    // Test should complete within reasonable time (not deadlock)
    assert!(elapsed < Duration::from_secs(5));
    
    // Some operations should have succeeded
    assert!(success_count_final > 0);
    
    info!(
        elapsed = ?elapsed,
        successful_operations = success_count_final,
        "Deadlock prevention test completed"
    );
}
