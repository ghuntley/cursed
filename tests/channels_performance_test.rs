//! Performance tests for CURSED channel implementation
//! 
//! These tests focus on benchmarking channel operations, testing performance
//! under high load, memory usage analysis, and select operation performance.

use cursed::runtime::channels::{Channel, ChannelRegistry, ChannelError};
use cursed::runtime::value::Value;
use cursed::types::Type;
use cursed::memory::gc::GarbageCollector;
use std::sync::{Arc, Barrier, Mutex};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::thread;
use std::time::{Duration, Instant};

#[path = "common/mod.rs"]
pub mod common;

#[test]
fn test_basic_send_receive_performance() {
    init_tracing!();
    
    let channel = Channel::new(Type::Integer, 1000).unwrap();
    let num_operations = 10_000;
    
    let _timer = common::timing::Timer::new("basic_send_receive");
    
    let start = Instant::now();
    
    // Send operations
    for i in 0..num_operations {
        channel.send(Value::Integer(i)).unwrap();
    }
    
    let send_duration = start.elapsed();
    
    // Receive operations
    let receive_start = Instant::now();
    
    for _ in 0..num_operations {
        let _ = channel.receive().unwrap();
    }
    
    let receive_duration = receive_start.elapsed();
    let total_duration = start.elapsed();
    
    let send_throughput = num_operations as f64 / send_duration.as_secs_f64();
    let receive_throughput = num_operations as f64 / receive_duration.as_secs_f64();
    let total_throughput = (num_operations * 2) as f64 / total_duration.as_secs_f64();
    
    tracing::info!(
        send_throughput = %format!("{:.0} ops/sec", send_throughput),
        receive_throughput = %format!("{:.0} ops/sec", receive_throughput),
        total_throughput = %format!("{:.0} ops/sec", total_throughput),
        "Basic send/receive performance"
    );
    
    // Assert reasonable performance (adjust thresholds as needed)
    assert!(send_throughput > 1000.0, "Send throughput too low: {}", send_throughput);
    assert!(receive_throughput > 1000.0, "Receive throughput too low: {}", receive_throughput);
    
    tracing::info!("✓ Basic send/receive performance test passed");
}

#[test]
fn test_buffered_vs_unbuffered_performance() {
    init_tracing!();
    
    let num_operations = 1_000;
    
    // Test unbuffered channel performance
    let unbuffered = Arc::new(Channel::new(Type::Integer, 0).unwrap());
    let unbuffered_barrier = Arc::new(Barrier::new(2));
    
    let unbuffered_start = Instant::now();
    
    let sender_handle = {
        let channel = Arc::clone(&unbuffered);
        let barrier = Arc::clone(&unbuffered_barrier);
        
        thread::spawn(move || {
            barrier.wait();
            for i in 0..num_operations {
                channel.send(Value::Integer(i)).unwrap();
            }
        })
    };
    
    let receiver_handle = {
        let channel = Arc::clone(&unbuffered);
        let barrier = Arc::clone(&unbuffered_barrier);
        
        thread::spawn(move || {
            barrier.wait();
            for _ in 0..num_operations {
                let _ = channel.receive().unwrap();
            }
        })
    };
    
    sender_handle.join().unwrap();
    receiver_handle.join().unwrap();
    
    let unbuffered_duration = unbuffered_start.elapsed();
    let unbuffered_throughput = (num_operations * 2) as f64 / unbuffered_duration.as_secs_f64();
    
    // Test buffered channel performance
    let buffered = Channel::new(Type::Integer, num_operations).unwrap();
    let buffered_start = Instant::now();
    
    // Send all values (should not block)
    for i in 0..num_operations {
        buffered.send(Value::Integer(i)).unwrap();
    }
    
    // Receive all values
    for _ in 0..num_operations {
        let _ = buffered.receive().unwrap();
    }
    
    let buffered_duration = buffered_start.elapsed();
    let buffered_throughput = (num_operations * 2) as f64 / buffered_duration.as_secs_f64();
    
    tracing::info!(
        unbuffered_throughput = %format!("{:.0} ops/sec", unbuffered_throughput),
        buffered_throughput = %format!("{:.0} ops/sec", buffered_throughput),
        "Buffered vs unbuffered performance comparison"
    );
    
    // Buffered should generally be faster for this test pattern
    // (though this may vary based on implementation)
    
    tracing::info!("✓ Buffered vs unbuffered performance test passed");
}

#[test]
fn test_high_contention_performance() {
    init_tracing!();
    
    let channel = Arc::new(Channel::new(Type::Integer, 1000).unwrap());
    let num_threads = 8;
    let operations_per_thread = 1_000;
    let barrier = Arc::new(Barrier::new(num_threads));
    
    let _timer = common::timing::Timer::new("high_contention");
    
    let start = Instant::now();
    
    // Create mixed sender/receiver threads
    let handles: Vec<_> = (0..num_threads).map(|thread_id| {
        let channel = Arc::clone(&channel);
        let barrier = Arc::clone(&barrier);
        
        thread::spawn(move || {
            barrier.wait(); // Synchronize start
            
            let is_sender = thread_id % 2 == 0;
            let mut operations_completed = 0;
            
            if is_sender {
                for i in 0..operations_per_thread {
                    let value = Value::Integer((thread_id * 1000 + i) as i64);
                    if channel.send_timeout(value, Duration::from_millis(100)).is_ok() {
                        operations_completed += 1;
                    }
                }
            } else {
                for _ in 0..operations_per_thread {
                    if channel.receive_timeout(Duration::from_millis(100)).is_ok() {
                        operations_completed += 1;
                    }
                }
            }
            
            operations_completed
        })
    }).collect();
    
    // Wait for all threads to complete
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    let total_operations: usize = results.iter().sum();
    
    let duration = start.elapsed();
    let throughput = total_operations as f64 / duration.as_secs_f64();
    
    tracing::info!(
        num_threads,
        total_operations,
        throughput = %format!("{:.0} ops/sec", throughput),
        duration = ?duration,
        "High contention performance"
    );
    
    assert!(total_operations > 0, "No operations completed");
    assert!(throughput > 100.0, "Throughput too low under contention: {}", throughput);
    
    tracing::info!("✓ High contention performance test passed");
}

#[test]
fn test_memory_usage_large_channels() {
    init_tracing!();
    
    let _timer = common::timing::Timer::new("memory_usage_large_channels");
    
    // Create large channels with different buffer sizes
    let buffer_sizes = vec![1_000, 10_000, 100_000];
    let mut channels = Vec::new();
    
    for &buffer_size in &buffer_sizes {
        let channel = Channel::new(Type::String, buffer_size).unwrap();
        
        // Fill channel partially
        let fill_ratio = 0.8;
        let num_items = (buffer_size as f64 * fill_ratio) as usize;
        
        let start = Instant::now();
        
        for i in 0..num_items {
            let large_string = format!("Large string item {} with content {}", i, "x".repeat(100));
            channel.send(Value::String(large_string)).unwrap();
        }
        
        let fill_duration = start.elapsed();
        let fill_throughput = num_items as f64 / fill_duration.as_secs_f64();
        
        tracing::info!(
            buffer_size,
            num_items,
            fill_throughput = %format!("{:.0} items/sec", fill_throughput),
            "Large channel fill performance"
        );
        
        channels.push(channel);
    }
    
    // Test memory efficiency by accessing channels
    for (i, channel) in channels.iter().enumerate() {
        let buffer_size = buffer_sizes[i];
        assert!(channel.len() > 0);
        assert!(channel.len() <= buffer_size);
        
        // Sample receive to test access performance
        let start = Instant::now();
        for _ in 0..10 {
            if let Ok(Some(_)) = channel.receive() {
                // Successfully received
            }
        }
        let access_time = start.elapsed();
        
        tracing::debug!(
            buffer_size,
            access_time = ?access_time,
            "Channel access time"
        );
    }
    
    tracing::info!("✓ Memory usage large channels test passed");
}

#[test]
fn test_channel_creation_performance() {
    init_tracing!();
    
    let num_channels = 1_000;
    let _timer = common::timing::Timer::new("channel_creation");
    
    let start = Instant::now();
    
    let mut channels = Vec::with_capacity(num_channels);
    
    for i in 0..num_channels {
        let buffer_size = (i % 10) + 1; // Vary buffer sizes
        let channel_type = if i % 2 == 0 { Type::Integer } else { Type::String };
        
        let channel = Channel::new(channel_type, buffer_size).unwrap();
        channels.push(channel);
    }
    
    let creation_duration = start.elapsed();
    let creation_throughput = num_channels as f64 / creation_duration.as_secs_f64();
    
    tracing::info!(
        num_channels,
        creation_throughput = %format!("{:.0} channels/sec", creation_throughput),
        creation_duration = ?creation_duration,
        "Channel creation performance"
    );
    
    // Test that all channels work
    for (i, channel) in channels.iter().enumerate() {
        let test_value = if i % 2 == 0 {
            Value::Integer(i as i64)
        } else {
            Value::String(format!("test{}", i))
        };
        
        channel.send(test_value.clone()).unwrap();
        let received = channel.receive().unwrap().unwrap();
        assert_eq!(received, test_value);
    }
    
    assert!(creation_throughput > 100.0, "Channel creation too slow: {}", creation_throughput);
    
    tracing::info!("✓ Channel creation performance test passed");
}

#[test]
fn test_select_operation_performance() {
    init_tracing!();
    
    let num_channels = 10;
    let operations_per_channel = 100;
    
    // Create multiple channels
    let channels: Vec<_> = (0..num_channels).map(|i| {
        Arc::new(Channel::new(Type::Integer, 10).unwrap())
    }).collect();
    
    let _timer = common::timing::Timer::new("select_operations");
    
    // Populate channels with different timing
    for (i, channel) in channels.iter().enumerate() {
        let channel = Arc::clone(channel);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(i as u64 * 10)); // Stagger sends
            
            for j in 0..operations_per_channel {
                let value = Value::Integer((i * 1000 + j) as i64);
                if channel.send_timeout(value, Duration::from_millis(50)).is_err() {
                    break;
                }
            }
        });
    }
    
    // Simulate select operations by trying to receive from any channel
    let start = Instant::now();
    let mut total_received = 0;
    let timeout = Duration::from_millis(1);
    
    for _ in 0..(num_channels * operations_per_channel) {
        let mut received_from_any = false;
        
        // Try each channel (simulate select behavior)
        for channel in &channels {
            if let Ok(Some(_)) = channel.receive_timeout(timeout) {
                total_received += 1;
                received_from_any = true;
                break;
            }
        }
        
        if !received_from_any {
            thread::sleep(Duration::from_millis(1));
        }
    }
    
    let select_duration = start.elapsed();
    let select_throughput = total_received as f64 / select_duration.as_secs_f64();
    
    tracing::info!(
        num_channels,
        total_received,
        select_throughput = %format!("{:.0} ops/sec", select_throughput),
        "Select operation performance"
    );
    
    assert!(total_received > 0, "No messages received in select test");
    
    tracing::info!("✓ Select operation performance test passed");
}

#[test]
fn test_gc_impact_on_channels() {
    init_tracing!();
    
    let gc = Arc::new(Mutex::new(GarbageCollector::new()));
    let channel = Channel::new_with_gc(Type::String, 1000, Arc::clone(&gc)).unwrap();
    
    let num_operations = 5_000;
    let gc_interval = 500; // Run GC every 500 operations
    
    let _timer = common::timing::Timer::new("gc_impact");
    
    let start = Instant::now();
    
    for i in 0..num_operations {
        // Send large string values
        let large_value = format!("Large string value {} with lots of content {}", i, "x".repeat(200));
        channel.send(Value::String(large_value)).unwrap();
        
        // Trigger GC periodically
        if i % gc_interval == 0 {
            let gc_start = Instant::now();
            {
                let mut gc_guard = gc.lock().unwrap();
                gc_guard.collect();
            }
            let gc_duration = gc_start.elapsed();
            
            tracing::debug!(
                operation = i,
                gc_duration = ?gc_duration,
                "GC collection during channel operations"
            );
        }
        
        // Receive some values to prevent channel from filling up
        if i % 2 == 0 {
            let _ = channel.receive().unwrap();
        }
    }
    
    let total_duration = start.elapsed();
    let throughput = num_operations as f64 / total_duration.as_secs_f64();
    
    tracing::info!(
        num_operations,
        throughput = %format!("{:.0} ops/sec", throughput),
        total_duration = ?total_duration,
        "Channel performance with GC"
    );
    
    // Clean up remaining values
    while let Ok(Some(_)) = channel.receive() {
        // Drain channel
    }
    
    assert!(throughput > 100.0, "GC impact too severe on channel performance: {}", throughput);
    
    tracing::info!("✓ GC impact on channels test passed");
}

#[test]
fn test_large_message_performance() {
    init_tracing!();
    
    let channel = Channel::new(Type::String, 100).unwrap();
    let message_sizes = vec![1_000, 10_000, 100_000]; // Different message sizes
    let messages_per_size = 100;
    
    for &message_size in &message_sizes {
        let large_message = "x".repeat(message_size);
        
        let _timer = common::timing::Timer::new(&format!("large_message_{}_bytes", message_size));
        
        let start = Instant::now();
        
        // Send large messages
        for i in 0..messages_per_size {
            let message = format!("{}-{}", large_message, i);
            channel.send(Value::String(message)).unwrap();
        }
        
        let send_duration = start.elapsed();
        
        // Receive large messages
        let receive_start = Instant::now();
        
        for _ in 0..messages_per_size {
            let _ = channel.receive().unwrap();
        }
        
        let receive_duration = receive_start.elapsed();
        let total_duration = start.elapsed();
        
        let bytes_sent = (message_size + 10) * messages_per_size; // +10 for suffix
        let throughput_mbps = (bytes_sent as f64 / (1024.0 * 1024.0)) / total_duration.as_secs_f64();
        
        tracing::info!(
            message_size,
            messages_per_size,
            throughput_mbps = %format!("{:.2} MB/s", throughput_mbps),
            send_duration = ?send_duration,
            receive_duration = ?receive_duration,
            "Large message performance"
        );
        
        assert!(throughput_mbps > 1.0, "Large message throughput too low: {} MB/s", throughput_mbps);
    }
    
    tracing::info!("✓ Large message performance test passed");
}

#[test]
fn test_sustained_load_performance() {
    init_tracing!();
    
    let channel = Arc::new(Channel::new(Type::Integer, 1000).unwrap());
    let test_duration = Duration::from_secs(5);
    let num_threads = 4;
    
    let operations_count = Arc::new(AtomicUsize::new(0));
    let start_barrier = Arc::new(Barrier::new(num_threads));
    let should_stop = Arc::new(AtomicUsize::new(0));
    
    let _timer = common::timing::Timer::new("sustained_load");
    
    let start = Instant::now();
    
    // Start the timer thread
    {
        let should_stop = Arc::clone(&should_stop);
        thread::spawn(move || {
            thread::sleep(test_duration);
            should_stop.store(1, Ordering::SeqCst);
        });
    }
    
    // Worker threads
    let handles: Vec<_> = (0..num_threads).map(|thread_id| {
        let channel = Arc::clone(&channel);
        let operations_count = Arc::clone(&operations_count);
        let start_barrier = Arc::clone(&start_barrier);
        let should_stop = Arc::clone(&should_stop);
        
        thread::spawn(move || {
            start_barrier.wait();
            
            let mut local_ops = 0;
            let is_sender = thread_id % 2 == 0;
            
            while should_stop.load(Ordering::SeqCst) == 0 {
                if is_sender {
                    let value = Value::Integer(local_ops as i64);
                    if channel.send_timeout(value, Duration::from_millis(1)).is_ok() {
                        local_ops += 1;
                        operations_count.fetch_add(1, Ordering::SeqCst);
                    }
                } else {
                    if channel.receive_timeout(Duration::from_millis(1)).is_ok() {
                        local_ops += 1;
                        operations_count.fetch_add(1, Ordering::SeqCst);
                    }
                }
            }
            
            local_ops
        })
    }).collect();
    
    // Wait for completion
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap()).collect();
    let actual_duration = start.elapsed();
    
    let total_operations = operations_count.load(Ordering::SeqCst);
    let sustained_throughput = total_operations as f64 / actual_duration.as_secs_f64();
    
    tracing::info!(
        test_duration = ?test_duration,
        actual_duration = ?actual_duration,
        total_operations,
        sustained_throughput = %format!("{:.0} ops/sec", sustained_throughput),
        thread_results = ?results,
        "Sustained load performance"
    );
    
    assert!(total_operations > 1000, "Too few operations in sustained test: {}", total_operations);
    assert!(sustained_throughput > 100.0, "Sustained throughput too low: {}", sustained_throughput);
    
    tracing::info!("✓ Sustained load performance test passed");
}
