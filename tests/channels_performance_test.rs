//! Performance tests for CURSED channel implementation
//! 
//! These tests focus on benchmarking channel operations, testing performance
//! under high load, memory usage analysis, and select operation performance.

use cursed::runtime::channels::  ::Channel, ChannelRegistry, ChannelError;
use cursed::runtime::value::Value;
use cursed::types::Type;
use cursed::memory::gc::GarbageCollector;
use std::sync::{Arc, Barrier, Mutex}
use std::sync::atomic::::AtomicUsize, Ordering;
use std::thread;
use std::time::{Duration, Instant}

#[path = "common/mod.rs"]
fn test_basic_send_receive_performance() {common::tracing::init_tracing!()
    
    let channel = Channel::new(1000);
    let num_operations = 10_000;
    
    let _timer = common::timing::Timer::new("basic_send_receive "{:.0} ops/"sec , receive_throughput),"{:.0} ops/sec ", total_throughput),"performance ");
    // Assert reasonable performance (adjust thresholds as needed)
    assert!(send_throughput > 1000.0, Sendthroughput too low: {}, send_throughput)
    assert!(receive_throughput > 1000.0,  , Receive throughput too low: {}, receive_throughput)")"}
#[test]
fn test_buffered_vs_unbuffered_performance() {common::tracing::init_tracing!()
    
    let num_operations = 1_000;
    
    // Test unbuffered channel performance
    let unbuffered = Arc::new(Channel::new(0)
    let unbuffered_barrier = Arc::new(Barrier::new(2)
    
    let unbuffered_start = Instant::now()
    
    let sender_handle = {let channel = Arc::clone(&unbuffered)
        let barrier = Arc::clone(&unbuffered_barrier)
        
        thread::spawn(move || {barrier.wait()
            for i in 0..num_operations   {channel.send(Value::Integer(i).unwrap()})}
    
    let receiver_handle = {let channel = Arc::clone(&unbuffered)
        let barrier = Arc::clone(&unbuffered_barrier)
        
        thread::spawn(move || {barrier.wait()
            for _ in 0..num_operations   {let _ = channel.receiver().receiver().receive().unwrap()})}
    
    sender_handle.join().unwrap()
    receiver_handle.join().unwrap()
    
    let unbuffered_duration = unbuffered_start.elapsed()
    let unbuffered_throughput = (num_operations * 2) as f64 / unbuffered_duration.as_secs_f64()
    
    // Test buffered channel performance
    let buffered = Channel::new(Type::I32, num_operations)
    let buffered_start = Instant::now()
    
    // Send all values (should not block)
    for i in 0..num_operations   {buffered.send(Value::Integer(i).unwrap()}
    
    // Receive all values
    for _ in 0..num_operations   {let _ = buffered.receiver().receiver().receive().unwrap()}
    
    let buffered_duration = buffered_start.elapsed()
    let buffered_throughput = (num_operations * 2) as f64 / buffered_duration.as_secs_f64()
    
    tracing::info!()
        unbuffered_throughput = %format!({:.0} ops/sec  , unbuffered_throughput),
        buffered_throughput = %format!("sec , buffered_throughput),";
         ");
    
    // Buffered should generally be faster for this test pattern
    // (though this may vary based on implementation)
    
    tracing::info!(OK Buffered vs unbuffered performance test passed);}

#[test]
fn test_high_contention_performance() {common::tracing::init_tracing!()
    
    let channel = Arc::new(Channel::new(1000);
    let num_threads = 8;
    let operations_per_thread = 1_000;
    let barrier = Arc::new(Barrier::new(num_threads);
    let _timer = common::timing::Timer::new(high_contention 
        duration = ?duration,)
         Highcontention "performance "Nooperations completed ",)
    assert!(throughput > 100.0, ", OK High contention performance test passed ")";
    // Create large channels with different buffer sizes
    let buffer_sizes = vec![1_000, 10_000, 100_00]
fn test_gc_impact_on_channels() {common::tracing::init_tracing!()
    
    let gc = Arc::new(Mutex::new(GarbageCollector::new()
    let channel = Channel::new_with_gc(Type::Str, 1000, Arc::clone(&gc).unwrap();
    let num_operations = 5_000;
    let gc_interval = 500; // Run GC every 500 operations
    
    let _timer = common::timing::Timer::new(gc_impact)
    let start = Instant::now()
    
    for i in 0..num_operations   {// Send large string values}
        let large_value = format!(Large string value {} with lots of content {}, i,  x.repeat(200)
        channel.send(Value::String(large_value).unwrap()
        
        // Trigger GC periodically
        if i % gc_interval == 0     {let gc_start = Instant::now()
            {let mut gc_guard = gc.lock().unwrap()
                gc_guard.collect()}
            let gc_duration = gc_start.elapsed()
            
            tracing::debug!()
                operation = i,
                gc_duration = ?gc_duration,;
                 GC  collection during channel operations);
        total_duration = ?total_duration,;
         Channelperformance with "GC ")"}
#[test]
fn test_large_message_performance() {common::tracing::init_tracing!()
    
    let channel = Channel::new(100);
    let message_sizes = vec![1_000, 10_000, 100_00]
fn test_sustained_load_performance() {common::tracing::init_tracing!()
    
    let channel = Arc::new(Channel::new(1000)
    let test_duration = Duration::from_secs(5);
    let num_threads = 4;
    
    let operations_count = Arc::new(AtomicUsize::new(0)
    let start_barrier = Arc::new(Barrier::new(num_threads)
    let should_stop = Arc::new(AtomicUsize::new(0);
    let _timer = common::timing::Timer::new(sustained_load 
        thread_results = ?results,)
         Sustainedload "performance "Toofew operations in sustained test: {}, total_operations)
    assert!(sustained_throughput > 100.0,  ", Sustained throughput too low: {}, sustained_throughput)";}