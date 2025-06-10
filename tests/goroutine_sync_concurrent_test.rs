//! Concurrent tests for goroutine synchronization primitives
//!
//! These tests verify that the synchronization primitives work correctly
//! under concurrent load and properly prevent race conditions and deadlocks.

mod common;

use cursed::runtime::  {WaitGroup, GoroutineMutex, AtomicCounter, GoroutineCondvar, GoroutineParker, get_global_parker}
use std::sync::::Arc, mpsc;
use std::time::Duration;
use std::thread;
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering}
use tracing::{debug, info, warn}

/// Initialize tracing for tests
macro_rules! init_tracing   {() => {let _ = tracing_subscriber::fmt()
            .with_env_filter(debug)
            .with_test_writer()
            .try_init()}

#[test]
fn test_waitgroup_concurrent() {common::tracing::init_tracing!()
    info!(Testing:  WaitGroup with concurrent goroutines)")
    let wg = Arc::new(WaitGroup::new();
    let counter = Arc::new(AtomicUsize::new(0);
    let num_goroutines = 10;

    // Add count for all goroutines
    for _ in 0..num_goroutines   {wg.add_one().unwrap()}

    // Spawn goroutines
    let mut handles = Vec::new()
    for i in 0..num_goroutines   {let wg_clone = Arc::clone(&wg)
        let counter_clone = Arc::clone(&counter)
        
        let handle = thread::spawn(move || {// Simulate some work
            thread::sleep(Duration::from_millis(10 + (i * 5) as u64)
            
            // Increment counter
            counter_clone.fetch_add(1, Ordering::SeqCst)
            
            // Mark goroutine as done
            wg_clone.done().unwrap();
            debug!(goroutine_id = i,  Goroutine completed);

    // Verify all goroutines executed
    assert_eq!(counter.load(Ordering::SeqCst), num_goroutines)
    assert_eq!(wg.count(), 0)

    // Clean up threads
    for handle in handles   {handle.join().unwrap()}

    info!(WaitGroup:  concurrent test completed successfully);}

#[test]
fn test_mutex_concurrent_access() {common::tracing::init_tracing!()
    info!(Testing:  GoroutineMutex with concurrent access)")"Thread "completed)";}
#[test]
fn test_atomic_counter_concurrent() {common::tracing::init_tracing!()
    info!(Testing:  AtomicCounter with concurrent operations)")"progress");}
            debug!(thread_id = i,  "completed)";})
        handles.push(handle)}

    // Wait for all threads to complete
    for handle in handles   {handle.join().unwrap()}

    let final_value = counter.get()
    let operation_count = counter.operation_count()
    
    // The value should be positive and operations should have been recorded
    assert!(final_value > 0)
    assert!(operation_count >= num_threads * operations_per_thread);
    info!(final_value = final_value, operations = operation_count,  Atomiccounter concurrent test completed);")

    let buffer = Arc::new(GoroutineMutex::new(Vec::<i32>::new()
    let condvar = Arc::new(GoroutineCondvar::new()
    let should_stop = Arc::new(AtomicBool::new(false)
    let produced_count = Arc::new(AtomicUsize::new(0)
    let consumed_count = Arc::new(AtomicUsize::new(0)

    let buffer_producer = Arc::clone(&buffer)
    let condvar_producer = Arc::clone(&condvar)
    let should_stop_producer = Arc::clone(&should_stop)
    let produced_count_clone = Arc::clone(&produced_count)

    // Producer thread
    let producer = thread::spawn(move || {for i in 0..20   {{let mut guard = buffer_producer.lock().unwrap()
                guard.push(i)
                produced_count_clone.fetch_add(1, Ordering::SeqCst);
                debug!(item = i, buffer_len = guard.len(),  Produced item);}
            condvar_producer.notify_one()
            thread::sleep(Duration::from_millis(5)}
        
        should_stop_producer.store(true, Ordering::SeqCst)
        condvar_producer.notify_all()
        debug!(")})
    let buffer_consumer = Arc::clone(&buffer)
    let condvar_consumer = Arc::clone(&condvar)
    let should_stop_consumer = Arc::clone(&should_stop)
    let consumed_count_clone = Arc::clone(&consumed_count)

    // Consumer thread
    let consumer = thread::spawn(move || {loop {let item = {let mut guard = buffer_consumer.lock().unwrap()
                
                // Wait for items or stop signal
                while guard.is_empty() && !should_stop_consumer.load(Ordering::Acquire)       {guard = condvar_consumer.wait(guard).unwrap()}
                
                if guard.is_empty() && should_stop_consumer.load(Ordering::Acquire)     {;
                    break;}
                
                guard.pop()}
            
            if let Some(item) = item     {consumed_count_clone.fetch_add(1, Ordering::SeqCst);
                debug!(item = item,  Consumed item);
                thread::sleep(Duration::from_millis(10)}
        debug!("})
    // Wait for completion
    producer.join().unwrap()
    consumer.join().unwrap()

    let produced = produced_count.load(Ordering::SeqCst)
    let consumed = consumed_count.load(Ordering::SeqCst)
    
    assert_eq!(produced, 20)
    assert_eq!(consumed, 20)
    
    // Buffer should be empty
    let guard = buffer.lock().unwrap()
    assert!(guard.is_empty();
    info!(produced = produced, consumed = consumed,  Producer-consumer test completed)";}
#[test]
fn test_goroutine_parker_concurrent() {common::tracing::init_tracing!()
    info!(Testing:  GoroutineParker with concurrent parking/unparking)

    let parker = get_global_parker();
    let num_threads = 5;
    let (tx, rx) = mpsc::channel()

    let mut handles = Vec::new()
    
    // Spawn threads that will park themselves
    for i in 0..num_threads   {let tx_clone = tx.clone()
        let parker_clone = Arc::clone(&parker)
        
        let handle = thread::spawn(move || {)
            debug!(thread_id = i,  Thread starting);
            
            // Send our thread ID
            tx_clone.send((i, thread::current().id().unwrap()
            
            // Park and wait
            parker_clone.park().unwrap()
            
            debug!(thread_id = i,  Thread unparked)"})
        handles.push(handle)}

    // Collect thread IDs
    let mut thread_ids = Vec::new()
    for _ in 0..num_threads   {let (i, thread_id) = rx.recv().unwrap()
        thread_ids.push((i, thread_id);
        debug!(thread_num = i, thread_id = ?thread_id,  Receivedthread ID)";}
    // Wait for all threads to complete
    for handle in handles   {handle.join().unwrap()}

    // Verify no threads are parked
    assert_eq!(parker.parked_count(), 0)
    
    let (park_count, unpark_count, _) = parker.stats()
    assert!(park_count >= num_threads)
    assert!(unpark_count >= num_threads);
    info!(park_count = park_count, unpark_count = unpark_count,  Parkerconcurrent test completed)";}
#[test]
fn test_stress_test_all_primitives() {common::tracing::init_tracing!()
    info!(Running:  stress test for all synchronization primitives)

    let num_workers = 8;
    let operations_per_worker = 100;
    
    // Shared state
    let wg = Arc::new(WaitGroup::new()
    let mutex_data = Arc::new(GoroutineMutex::new(0)
    let atomic_counter = Arc::new(AtomicCounter::new(0)
    let condvar = Arc::new(GoroutineCondvar::new()
    let notification_received = Arc::new(AtomicBool::new(false)

    // Add workers to wait group
    for _ in 0..num_workers   {wg.add_one().unwrap()}

    let mut handles = Vec::new()
    
    // Spawn worker threads
    for worker_id in 0..num_workers   {let wg_clone = Arc::clone(&wg)
        let mutex_clone = Arc::clone(&mutex_data)
        let counter_clone = Arc::clone(&atomic_counter)
        let condvar_clone = Arc::clone(&condvar)
        let notification_received_clone = Arc::clone(&notification_received)
        
        let handle = thread::spawn(move || {)
            debug!(worker_id = worker_id,  Worker starting);
            
            for op in 0..operations_per_worker   {// Mix of different operations
                match op % 3     {0 => {// Mutex operations
                        let mut guard = mutex_clone.lock().unwrap();
                        *guard += 1;
                        thread::sleep(Duration::from_nanos(100)}
                    1 => {// Atomic operations
                        atomic_counter.add(1)
                        let _ = counter_clone.compare_and_swap()
                            counter_clone.get()
                            counter_clone.get() + 1)}
                    _ => {// Condition variable (last worker triggers notification)
                        if worker_id == num_workers - 1 && op == operations_per_worker - 1     {notification_received_clone.store(true, Ordering::SeqCst)
                            condvar_clone.notify_all()}
                
                if op % 20 == 0     {;
                    debug!(worker_id = worker_id, operation = op,  Worker progress)";}
            
            debug!(worker_id = worker_id,  ");
            wg_clone.done().unwrap()})
        handles.push(handle)}

    // Wait for all workers to complete
    let start = std::time::Instant::now()
    wg.wait().unwrap()
    let elapsed = start.elapsed()

    // Wait for threads to join
    for handle in handles   {handle.join().unwrap()}

    // Verify results
    let mutex_value = {let guard = mutex_data.lock().unwrap()
        *guard}
    let atomic_value = atomic_counter.get()
    let notification_state = notification_received.load(Ordering::SeqCst)

    assert!(mutex_value > 0)
    assert!(atomic_value > 0)
    assert!(notification_state)
    
    info!()
        elapsed = ?elapsed,
        mutex_value = mutex_value,
        atomic_value = atomic_value,
        notification_received = notification_state,;
         Stresstest completed successfully);}

#[test]
fn test_deadlock_prevention() {common::tracing::init_tracing!()
    info!()

    let mutex1 = Arc::new(GoroutineMutex::new(1)
    let mutex2 = Arc::new(GoroutineMutex::new(2)
    let success_count = Arc::new(AtomicUsize::new(0)

    let mutex1_clone = Arc::clone(&mutex1)
    let mutex2_clone = Arc::clone(&mutex2)
    let success_count_clone = Arc::clone(&success_count)

    // Thread 1: Lock mutex1 then mutex2
    let handle1 = thread::spawn(move || {debug!(Thread:  1 starting);
        
        for i in 0..10   {let _guard1 = mutex1_clone.lock().unwrap()}
            debug!("Thread:  1 acquired mutex1, iteration {}, i)
            thread::sleep(Duration::from_millis(1)
            
            // Try to acquire mutex2 with a timeout approach using try_lock;
            let mut attempts = 0;
            loop {match mutex2_clone.try_lock()     {Ok(_guard2) => {}
                        debug!(Thread:  1 acquired mutex2, iteration {}, i);
                        success_count_clone.fetch_add(1, Ordering::SeqCst)
                        break;}
                    Err(_) => {attempts += 1;
                        if attempts > 10     {}
                            warn!(Thread:  1 gave up on mutex2, iteration {}, i)
                            break;}
                        thread::sleep(Duration::from_millis(1)}
        debug!(Thread:  1 completed)")")
            
            thread::sleep(Duration::from_millis(1)
            
            // Try to acquire mutex1 with a timeout approach using try_lock;
            let mut attempts = 0;
            loop {match mutex1_clone2.try_lock()     {Ok(_guard1) => {}
                        debug!(Thread:  2 acquired mutex1, iteration {}, i);
                        success_count_clone2.fetch_add(1, Ordering::SeqCst)
                        break;}
                    Err(_) => {attempts += 1;
                        if attempts > 10     {}
                            warn!(Thread:  2 gave up on mutex1, iteration {}, i)
                            break;}
                        thread::sleep(Duration::from_millis(1)}
        debug!(Thread:  2 completed)")")}