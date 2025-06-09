//! Stress tests for goroutine synchronization primitives
//!
//! These tests are designed to detect race conditions, memory corruption,
//! and performance issues under high concurrent load. They are critical
//! for ensuring the reliability of the synchronization primitives in
//! production environments.

mod common;

use cursed::runtime::{WaitGroup, GoroutineMutex, AtomicCounter, GoroutineCondvar, GoroutineParker, get_global_parker};
use std::sync::{Arc, mpsc, Barrier};
use std::time::{Duration, Instant};
use std::thread;
use std::sync::atomic::{AtomicUsize, AtomicBool, AtomicU64, Ordering};
use tracing::{debug, info, warn, error};

/// Initialize tracing for tests
macro_rules! init_tracing {
    () => {
        let _ = tracing_subscriber::fmt()
            .with_env_filter("info")
            .with_test_writer()
            .try_init();
    };
}

/// Timer utility for measuring test performance
struct TestTimer {
    start: Instant,
    name: String,
}

impl TestTimer {
    fn new(name: &str) -> Self {
        Self {
            start: Instant::now(),
            name: name.to_string(),
        }
    }
}

impl Drop for TestTimer {
    fn drop(&mut self) {
        let elapsed = self.start.elapsed();
        info!(test = %self.name, elapsed = ?elapsed, "Test timing");
    }
}

#[test]
fn test_waitgroup_high_concurrency() {
    init_tracing!();
    info!("Running WaitGroup high concurrency stress test");
    let _timer = TestTimer::new("waitgroup_high_concurrency");

    let num_groups = 100;
    let goroutines_per_group = 50;
    let total_goroutines = num_groups * goroutines_per_group;
    
    let global_counter = Arc::new(AtomicUsize::new(0));
    let mut group_handles = Vec::new();

    for group_id in 0..num_groups {
        let wg = Arc::new(WaitGroup::new());
        let counter_clone = Arc::clone(&global_counter);
        
        // Add all goroutines to the wait group
        for _ in 0..goroutines_per_group {
            wg.add_one().unwrap();
        }

        let wg_clone = Arc::clone(&wg);
        
        // Spawn goroutines for this group
        let group_handle = thread::spawn(move || {
            let mut goroutine_handles = Vec::new();
            
            for goroutine_id in 0..goroutines_per_group {
                let wg_goroutine = Arc::clone(&wg_clone);
                let counter_goroutine = Arc::clone(&counter_clone);
                
                let handle = thread::spawn(move || {
                    // Simulate varying amounts of work
                    let work_duration = Duration::from_nanos(100 + (goroutine_id as u64 * 10));
                    thread::sleep(work_duration);
                    
                    // Increment global counter
                    counter_goroutine.fetch_add(1, Ordering::SeqCst);
                    
                    // Mark as done
                    wg_goroutine.done().unwrap();
                });
                goroutine_handles.push(handle);
            }
            
            // Wait for all goroutines in this group to complete
            wg_clone.wait().unwrap();
            
            // Join all goroutine threads
            for handle in goroutine_handles {
                handle.join().unwrap();
            }
            
            debug!(group_id = group_id, "Group completed");
        });
        
        group_handles.push(group_handle);
    }

    // Wait for all groups to complete
    for handle in group_handles {
        handle.join().unwrap();
    }

    // Verify all goroutines executed
    let final_count = global_counter.load(Ordering::SeqCst);
    assert_eq!(final_count, total_goroutines);
    
    info!(
        total_goroutines = total_goroutines,
        final_count = final_count,
        "WaitGroup high concurrency test completed"
    );
}

#[test]
fn test_mutex_contention_stress() {
    init_tracing!();
    info!("Running Mutex contention stress test");
    let _timer = TestTimer::new("mutex_contention_stress");

    let num_threads = 20;
    let operations_per_thread = 1000;
    let shared_data = Arc::new(GoroutineMutex::new(Vec::<u64>::new()));
    let operation_counter = Arc::new(AtomicUsize::new(0));

    let barrier = Arc::new(Barrier::new(num_threads));
    let mut handles = Vec::new();

    for thread_id in 0..num_threads {
        let data_clone = Arc::clone(&shared_data);
        let counter_clone = Arc::clone(&operation_counter);
        let barrier_clone = Arc::clone(&barrier);
        
        let handle = thread::spawn(move || {
            // Wait for all threads to be ready
            barrier_clone.wait();
            
            for op in 0..operations_per_thread {
                {
                    let mut guard = data_clone.lock().unwrap();
                    
                    // Perform various operations on the shared data
                    match op % 4 {
                        0 => guard.push(thread_id as u64 * 1000 + op as u64),
                        1 => { guard.pop(); }
                        2 => guard.extend_from_slice(&[thread_id as u64; 3]),
                        _ => guard.retain(|&x| x % 2 == 0),
                    }
                    
                    // Simulate some work while holding the lock
                    if op % 100 == 0 {
                        thread::sleep(Duration::from_nanos(500));
                    }
                }
                
                counter_clone.fetch_add(1, Ordering::SeqCst);
                
                // Occasionally yield to increase contention
                if op % 50 == 0 {
                    thread::yield_now();
                }
            }
            
            debug!(thread_id = thread_id, "Thread completed");
        });
        
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify operation count
    let total_operations = operation_counter.load(Ordering::SeqCst);
    assert_eq!(total_operations, num_threads * operations_per_thread);
    
    // Check final data state
    let final_data = {
        let guard = shared_data.lock().unwrap();
        guard.len()
    };
    
    info!(
        total_operations = total_operations,
        final_data_size = final_data,
        "Mutex contention stress test completed"
    );
}

#[test]
fn test_atomic_operations_intensive() {
    init_tracing!();
    info!("Running intensive atomic operations stress test");
    let _timer = TestTimer::new("atomic_operations_intensive");

    let num_threads = 16;
    let operations_per_thread = 10000;
    let counter = Arc::new(AtomicCounter::new(0));
    let cas_successes = Arc::new(AtomicUsize::new(0));
    let cas_failures = Arc::new(AtomicUsize::new(0));

    let barrier = Arc::new(Barrier::new(num_threads));
    let mut handles = Vec::new();

    for thread_id in 0..num_threads {
        let counter_clone = Arc::clone(&counter);
        let successes_clone = Arc::clone(&cas_successes);
        let failures_clone = Arc::clone(&cas_failures);
        let barrier_clone = Arc::clone(&barrier);
        
        let handle = thread::spawn(move || {
            // Wait for all threads to start simultaneously
            barrier_clone.wait();
            
            for op in 0..operations_per_thread {
                match op % 5 {
                    0 => { counter_clone.add(1); }
                    1 => { counter_clone.add(-1); }
                    2 => { counter_clone.set(counter_clone.get() + thread_id as i64); }
                    3 | 4 => {
                        // Compare and swap operations with retry logic
                        let mut attempts = 0;
                        loop {
                            let current = counter_clone.get();
                            let new_value = current + 1;
                            let (_, success) = counter_clone.compare_and_swap(current, new_value);
                            
                            if success {
                                successes_clone.fetch_add(1, Ordering::SeqCst);
                                break;
                            } else {
                                failures_clone.fetch_add(1, Ordering::SeqCst);
                                attempts += 1;
                                if attempts > 100 {
                                    break; // Prevent infinite loops
                                }
                            }
                        }
                    }
                }
                
                // Occasionally yield to increase contention
                if op % 1000 == 0 {
                    thread::yield_now();
                }
            }
            
            debug!(thread_id = thread_id, "Thread completed");
        });
        
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify results
    let final_value = counter.get();
    let total_operations = counter.operation_count();
    let total_cas_successes = cas_successes.load(Ordering::SeqCst);
    let total_cas_failures = cas_failures.load(Ordering::SeqCst);
    
    assert!(total_operations >= num_threads * operations_per_thread);
    assert!(total_cas_successes > 0);
    
    info!(
        final_value = final_value,
        total_operations = total_operations,
        cas_successes = total_cas_successes,
        cas_failures = total_cas_failures,
        "Atomic operations intensive test completed"
    );
}

#[test]
fn test_condition_variable_broadcast_storm() {
    init_tracing!();
    info!("Running condition variable broadcast storm test");
    let _timer = TestTimer::new("condition_variable_broadcast_storm");

    let num_waiters = 50;
    let num_notifiers = 5;
    let notifications_per_notifier = 100;
    
    let mutex = Arc::new(GoroutineMutex::new(0));
    let condvar = Arc::new(GoroutineCondvar::new());
    let notifications_received = Arc::new(AtomicUsize::new(0));
    let should_stop = Arc::new(AtomicBool::new(false));

    let mut handles = Vec::new();

    // Spawn waiter threads
    for waiter_id in 0..num_waiters {
        let mutex_clone = Arc::clone(&mutex);
        let condvar_clone = Arc::clone(&condvar);
        let notifications_clone = Arc::clone(&notifications_received);
        let stop_clone = Arc::clone(&should_stop);
        
        let handle = thread::spawn(move || {
            while !stop_clone.load(Ordering::Acquire) {
                let mut guard = mutex_clone.lock().unwrap();
                
                while *guard == 0 && !stop_clone.load(Ordering::Acquire) {
                    guard = condvar_clone.wait(guard).unwrap();
                }
                
                if *guard > 0 {
                    *guard -= 1;
                    notifications_clone.fetch_add(1, Ordering::SeqCst);
                }
            }
            
            debug!(waiter_id = waiter_id, "Waiter completed");
        });
        
        handles.push(handle);
    }

    // Give waiters time to start waiting
    thread::sleep(Duration::from_millis(100));

    // Spawn notifier threads
    for notifier_id in 0..num_notifiers {
        let mutex_clone = Arc::clone(&mutex);
        let condvar_clone = Arc::clone(&condvar);
        
        let handle = thread::spawn(move || {
            for i in 0..notifications_per_notifier {
                {
                    let mut guard = mutex_clone.lock().unwrap();
                    *guard += 1;
                }
                
                // Mix of notify_one and notify_all
                if i % 10 == 0 {
                    condvar_clone.notify_all();
                } else {
                    condvar_clone.notify_one();
                }
                
                // Small delay to create notification patterns
                thread::sleep(Duration::from_millis(1));
            }
            
            debug!(notifier_id = notifier_id, "Notifier completed");
        });
        
        handles.push(handle);
    }

    // Let the test run for a while
    thread::sleep(Duration::from_secs(2));

    // Signal stop and wake up all waiters
    should_stop.store(true, Ordering::Release);
    {
        let mut guard = mutex.lock().unwrap();
        *guard = num_waiters; // Wake up all waiters
    }
    condvar.notify_all();

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    let total_notifications = notifications_received.load(Ordering::SeqCst);
    let expected_notifications = num_notifiers * notifications_per_notifier;
    
    // We should have received most notifications (allowing for some timing issues)
    assert!(total_notifications >= expected_notifications / 2);
    
    info!(
        total_notifications = total_notifications,
        expected = expected_notifications,
        "Condition variable broadcast storm test completed"
    );
}

#[test]
fn test_parker_mass_parking() {
    init_tracing!();
    info!("Running mass parking/unparking stress test");
    let _timer = TestTimer::new("parker_mass_parking");

    let num_threads = 100;
    let parker = get_global_parker();
    let (tx, rx) = mpsc::channel();
    let completion_counter = Arc::new(AtomicUsize::new(0));

    let mut handles = Vec::new();

    // Spawn threads that will park themselves
    for thread_id in 0..num_threads {
        let tx_clone = tx.clone();
        let completion_clone = Arc::clone(&completion_counter);
        
        let handle = thread::spawn(move || {
            // Send thread ID
            tx_clone.send(thread::current().id()).unwrap();
            
            // Park and wait
            parker.park().unwrap();
            
            // Mark completion
            completion_clone.fetch_add(1, Ordering::SeqCst);
            
            debug!(thread_id = thread_id, "Thread unparked and completed");
        });
        
        handles.push(handle);
    }

    // Collect all thread IDs
    let mut thread_ids = Vec::new();
    for _ in 0..num_threads {
        thread_ids.push(rx.recv().unwrap());
    }

    // Give threads time to park
    thread::sleep(Duration::from_millis(200));
    
    let parked_count = parker.parked_count();
    assert_eq!(parked_count, num_threads);
    info!(parked_count = parked_count, "All threads parked");

    // Unpark threads in batches
    let batch_size = 10;
    for batch in thread_ids.chunks(batch_size) {
        for &thread_id in batch {
            parker.unpark(thread_id).unwrap();
        }
        
        // Small delay between batches
        thread::sleep(Duration::from_millis(10));
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify all threads completed
    let completed = completion_counter.load(Ordering::SeqCst);
    assert_eq!(completed, num_threads);
    
    // Verify no threads are still parked
    assert_eq!(parker.parked_count(), 0);
    
    let (park_count, unpark_count, current_parked) = parker.stats();
    
    info!(
        completed_threads = completed,
        park_count = park_count,
        unpark_count = unpark_count,
        current_parked = current_parked,
        "Mass parking test completed"
    );
}

#[test]
fn test_memory_pressure_synchronization() {
    init_tracing!();
    info!("Running memory pressure synchronization test");
    let _timer = TestTimer::new("memory_pressure_synchronization");

    let num_threads = 20;
    let allocations_per_thread = 1000;
    let sync_primitives = Arc::new((
        GoroutineMutex::new(Vec::<Box<[u8; 1024]>>::new()),
        AtomicCounter::new(0),
        GoroutineCondvar::new(),
    ));

    let total_allocations = Arc::new(AtomicUsize::new(0));
    let mut handles = Vec::new();

    for thread_id in 0..num_threads {
        let primitives_clone = Arc::clone(&sync_primitives);
        let allocations_clone = Arc::clone(&total_allocations);
        
        let handle = thread::spawn(move || {
            let (mutex, counter, condvar) = &*primitives_clone;
            
            for i in 0..allocations_per_thread {
                // Allocate memory
                let data = Box::new([thread_id as u8; 1024]);
                
                // Use synchronization primitives with the allocated data
                {
                    let mut guard = mutex.lock().unwrap();
                    guard.push(data);
                    
                    // Periodically clean up to prevent excessive memory usage
                    if guard.len() > 100 {
                        guard.clear();
                    }
                }
                
                // Update atomic counter
                counter.add(1);
                allocations_clone.fetch_add(1, Ordering::SeqCst);
                
                // Occasionally trigger condition variable
                if i % 100 == 0 {
                    condvar.notify_one();
                }
                
                // Create memory pressure by frequently allocating/deallocating
                if i % 10 == 0 {
                    let _temp_allocs: Vec<Box<[u8; 512]>> = (0..10)
                        .map(|_| Box::new([0u8; 512]))
                        .collect();
                    // _temp_allocs is dropped here
                }
            }
            
            debug!(thread_id = thread_id, "Thread completed");
        });
        
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    // Verify results
    let total_allocs = total_allocations.load(Ordering::SeqCst);
    let counter_value = sync_primitives.1.get();
    let final_data_size = {
        let guard = sync_primitives.0.lock().unwrap();
        guard.len()
    };

    assert_eq!(total_allocs, num_threads * allocations_per_thread);
    assert_eq!(counter_value, total_allocs as i64);
    
    info!(
        total_allocations = total_allocs,
        counter_value = counter_value,
        final_data_size = final_data_size,
        "Memory pressure synchronization test completed"
    );
}

#[test]
fn test_timeout_stress() {
    init_tracing!();
    info!("Running timeout operations stress test");
    let _timer = TestTimer::new("timeout_stress");

    let num_threads = 20;
    let timeout_operations_per_thread = 100;
    let wg = Arc::new(WaitGroup::new());
    let timeout_successes = Arc::new(AtomicUsize::new(0));
    let timeout_failures = Arc::new(AtomicUsize::new(0));

    let mut handles = Vec::new();

    for thread_id in 0..num_threads {
        let wg_clone = Arc::clone(&wg);
        let successes_clone = Arc::clone(&timeout_successes);
        let failures_clone = Arc::clone(&timeout_failures);
        
        let handle = thread::spawn(move || {
            for i in 0..timeout_operations_per_thread {
                // Create a temporary waitgroup that will timeout
                let temp_wg = WaitGroup::new();
                temp_wg.add_one().unwrap();
                
                // Attempt to wait with short timeout
                let timeout_duration = Duration::from_millis(1 + (i % 10) as u64);
                match temp_wg.wait_timeout(timeout_duration) {
                    Ok(_) => successes_clone.fetch_add(1, Ordering::SeqCst),
                    Err(_) => failures_clone.fetch_add(1, Ordering::SeqCst),
                }
                
                // Occasionally test parker timeout
                if i % 10 == 0 {
                    let parker = get_global_parker();
                    let _was_unparked = parker.park_timeout(Duration::from_millis(1)).unwrap();
                }
            }
            
            debug!(thread_id = thread_id, "Thread completed");
        });
        
        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    let total_successes = timeout_successes.load(Ordering::SeqCst);
    let total_failures = timeout_failures.load(Ordering::SeqCst);
    let total_operations = total_successes + total_failures;

    // Most operations should timeout (since we don't complete the waitgroups)
    assert_eq!(total_operations, num_threads * timeout_operations_per_thread);
    assert!(total_failures > total_successes);
    
    info!(
        total_operations = total_operations,
        successes = total_successes,
        failures = total_failures,
        "Timeout stress test completed"
    );
}
