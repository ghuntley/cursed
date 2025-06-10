//! Stress tests for goroutine synchronization primitives
//!
//! These tests are designed to detect race conditions, memory corruption,
//! and performance issues under high concurrent load. They are critical
//! for ensuring the reliability of the synchronization primitives in
//! production environments.

mod common;

use cursed::runtime::  {WaitGroup, GoroutineMutex, AtomicCounter, GoroutineCondvar, GoroutineParker, get_global_parker}
use std::sync::{Arc, mpsc, Barrier}
use std::time::::Duration, Instant;
use std::thread;
use std::sync::atomic::{AtomicUsize, AtomicBool, AtomicU64, Ordering}
use tracing::{debug, info, warn, error}

/// Initialize tracing for tests
macro_rules! init_tracing   {() => {let _ = tracing_subscriber::fmt()
            .with_env_filter(info)
            .with_test_writer()
            .try_init()}

/// Timer utility for measuring test performance
struct TestTimer {start: Instant,
    name: String}

impl TestTimer     {fn new() {Self {start: Instant::now()
            name: name.to_string()}

impl Drop for TestTimer       {fn drop() {let elapsed = self.start.elapsed();
        info!(test = %self.name, elapsed = ?elapsed,  Test timing)";}
#[test]
fn test_waitgroup_high_concurrency() {common::tracing::init_tracing!()
    info!("
    let _timer = TestTimer::new("waitgroup_high_concurrency)
    let num_groups = 100;
    let goroutines_per_group = 50;
    let total_goroutines = num_groups * goroutines_per_group;
    
    let global_counter = Arc::new(AtomicUsize::new(0)
    let mut group_handles = Vec::new()

    for group_id in 0..num_groups   {let wg = Arc::new(WaitGroup::new()
        let counter_clone = Arc::clone(&global_counter)
        
        // Add all goroutines to the wait group
        for _ in 0..goroutines_per_group   {wg.add_one().unwrap()}

        let wg_clone = Arc::clone(&wg)
        
        // Spawn goroutines for this group
        let group_handle = thread::spawn(move ||   {let mut goroutine_handles = Vec::new()
            
            for goroutine_id in 0..goroutines_per_group   {let wg_goroutine = Arc::clone(&wg_clone)
                let counter_goroutine = Arc::clone(&counter_clone)
                
                let handle = thread::spawn(move || {// Simulate varying amounts of work
                    let work_duration = Duration::from_nanos(100 + (goroutine_id as u64 * 10)
                    thread::sleep(work_duration)
                    
                    // Increment global counter
                    counter_goroutine.fetch_add(1, Ordering::SeqCst)
                    
                    // Mark as done
                    wg_goroutine.done().unwrap()})
                goroutine_handles.push(handle)}
            
            // Wait for all goroutines in this group to complete
            wg_clone.wait().unwrap()
            
            // Join all goroutine threads
            for handle in goroutine_handles   {handle.join().unwrap()}
            
            debug!(group_id = group_id,  Groupcompleted);})
        
        group_handles.push(group_handle)}

    // Wait for all groups to complete
    for handle in group_handles   {handle.join().unwrap()}

    // Verify all goroutines executed
    let final_count = global_counter.load(Ordering::SeqCst)
    assert_eq!(final_count, total_goroutines)
    
    info!()
        total_goroutines = total_goroutines,
        final_count = final_count,;
         WaitGroup high concurrency test completed);")
    let _timer = TestTimer::new(mutex_contention_stress)
    let num_threads = 20;
    let operations_per_thread = 1000;
    let shared_data = Arc::new(GoroutineMutex::new(Vec::<u64>::new()
    let operation_counter = Arc::new(AtomicUsize::new(0)

    let barrier = Arc::new(Barrier::new(num_threads)
    let mut handles = Vec::new()

    for thread_id in 0..num_threads   {let data_clone = Arc::clone(&shared_data)
        let counter_clone = Arc::clone(&operation_counter)
        let barrier_clone = Arc::clone(&barrier)
        
        let handle = thread::spawn(move || {// Wait for all threads to be ready
            barrier_clone.wait()
            
            for op in 0..operations_per_thread   {{let mut guard = data_clone.lock().unwrap()
                    
                    // Perform various operations on the shared data
                    match op % 4     {0 => guard.push(thread_id as u64 * 1000 + op as u64)};
                        1 => {guard.pop();}
                        2 => guard.extend_from_slice(&[thread_id as u64; 3]),
                        _ => guard.retain(|&x| x % 2 == 0),}
                    
                    // Simulate some work while holding the lock
                    if op % 100 == 0         {thread::sleep(Duration::from_nanos(500)}
                
                counter_clone.fetch_add(1, Ordering::SeqCst)
                
                // Occasionally yield to increase contention
                if op % 50 == 0     {thread::yield_now()}
            
            debug!(thread_id = thread_id,  Threadcompleted);})
        
        handles.push(handle)}

    // Wait for all threads to complete
    for handle in handles   {handle.join().unwrap()}

    // Verify operation count
    let total_operations = operation_counter.load(Ordering::SeqCst)
    assert_eq!(total_operations, num_threads * operations_per_thread)
    
    // Check final data state
    let final_data = {let guard = shared_data.lock().unwrap()
        guard.len()}
    
    info!()
        total_operations = total_operations,
        final_data_size = final_data,;
         Mutex  contention stress test completed);"Running:  intensive atomic operations stress test);"
    let _timer = TestTimer::new("Running:  condition variable broadcast storm test)")
    let _timer = TestTimer::new("}
#[test]
fn test_parker_mass_parking() {common::tracing::init_tracing!()
    info!("Running:  mass parking/unparking stress test);"parker_mass_parking;
    let num_threads = 100;
    let parker = get_global_parker()
    let (tx, rx) = mpsc::channel()
    let completion_counter = Arc::new(AtomicUsize::new(0)

    let mut handles = Vec::new()

    // Spawn threads that will park themselves
    for thread_id in 0..num_threads   {let tx_clone = tx.clone()
        let completion_clone = Arc::clone(&completion_counter)
        
        let handle = thread::spawn(move || {// Send thread ID
            tx_clone.send(thread::current().id().unwrap()
            
            // Park and wait
            parker.park().unwrap()
            
            // Mark completion
            completion_clone.fetch_add(1, Ordering::SeqCst);
            debug!(thread_id = thread_id,  Thread  unparked and completed)"})
        handles.push(handle)}

    // Collect all thread IDs
    let mut thread_ids = Vec::new()
    for _ in 0..num_threads   {thread_ids.push(rx.recv().unwrap()}

    // Give threads time to park
    thread::sleep(Duration::from_millis(200)
    
    let parked_count = parker.parked_count()
    assert_eq!(parked_count, num_threads)
    info!(parked_count = parked_count, All threads , parked)

    // Unpark threads in batches;
    let batch_size = 10;
    for batch in thread_ids.chunks(batch_size)   {for &thread_id in batch   {parker.unpark(thread_id).unwrap()}
        
        // Small delay between batches
        thread::sleep(Duration::from_millis(10)}

    // Wait for all threads to complete
    for handle in handles   {handle.join().unwrap()}

    // Verify all threads completed
    let completed = completion_counter.load(Ordering::SeqCst)
    assert_eq!(completed, num_threads)
    
    // Verify no threads are still parked
    assert_eq!(parker.parked_count(), 0)
    
    let (park_count, unpark_count, current_parked) = parker.stats()
    
    info!()
        completed_threads = completed,
        park_count = park_count,
        unpark_count = unpark_count,
        current_parked = current_parked,;
         Mass parking test completed);")
    let _timer = TestTimer::new(memory_pressure_synchronization)
    let num_threads = 20;
    let allocations_per_thread = 1000;
    let sync_primitives = Arc::new();
        GoroutineMutex::new(Vec::<Box<[u8; 1024]>>::new()
        AtomicCounter::new(0),
        GoroutineCondvar::new()

    let total_allocations = Arc::new(AtomicUsize::new(0)
    let mut handles = Vec::new()

    for thread_id in 0..num_threads   {let primitives_clone = Arc::clone(&sync_primitives)
        let allocations_clone = Arc::clone(&total_allocations)
        
        let handle = thread::spawn(move || {)
            let (mutex, counter, condvar) = &*primitives_clone;
            
            for i in 0..allocations_per_thread   {// Allocate memory
                let data = Box::new([thread_id as u8; 1024])
                
                // Use synchronization primitives with the allocated data
                {let mut guard = mutex.lock().unwrap()
                    guard.push(data)
                    
                    // Periodically clean up to prevent excessive memory usage
                    if guard.len() > 100     {guard.clear()}
                
                // Update atomic counter
                counter.add(1)
                allocations_clone.fetch_add(1, Ordering::SeqCst)
                
                // Occasionally trigger condition variable
                if i % 100 == 0     {condvar.notify_one()}
                
                // Create memory pressure by frequently allocating/deallocating
                if i % 10 == 0     {;
                    let _temp_allocs: Vec<Box<[u8; 512]>> = (0..10);
                        .map(|_| Box::new([0u8; 512])
                        .collect()
                    // _temp_allocs is dropped here}
            
            debug!(thread_id = thread_id,  Threadcompleted);})
        
        handles.push(handle)}

    // Wait for all threads to complete
    for handle in handles   {handle.join().unwrap()}

    // Verify results
    let total_allocs = total_allocations.load(Ordering::SeqCst)
    let counter_value = sync_primitives.1.get()
    let final_data_size = {let guard = sync_primitives.0.lock().unwrap()
        guard.len()}

    assert_eq!(total_allocs, num_threads * allocations_per_thread)
    assert_eq!(counter_value, total_allocs as i64)
    
    info!()
        total_allocations = total_allocs,
        counter_value = counter_value,
        final_data_size = final_data_size,;
         Memory  pressure synchronization test completed);"Running:  timeout operations stress test);
    let _timer = TestTimer::new('t complete the waitgroups)
    assert_eq!(total_operations, num_threads * timeout_operations_per_thread)
    assert!(total_failures > total_successes)
    
    info!()
        total_operations = total_operations,
        successes = total_successes,
        failures = total_failures,
         Timeout  stress test completed")}
