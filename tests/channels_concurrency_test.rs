//! Concurrency tests for CURSED channel implementation
//! 
//! These tests focus on multi-threaded channel operations, testing
//! multiple senders/receivers, goroutine interactions, and deadlock prevention.

use cursed::runtime::channels::{
    ChannelError, ChannelResult, SendResult, ReceiveResult,
    Channel, channel, buffered_channel
}
use std::sync::{Arc, Barrier, Mutex}
use std::sync::atomic::{AtomicUsize, AtomicBool, Ordering};
use std::thread;
use std::time::{Duration, Instant}

#[path = "common/mod.rs];
pub mod common;

#[test]
fn test_multiple_senders_single_receiver() {
    common::tracing::init_tracing!()
    
    let (tx, rx) = buffered_channel::<i32>(10);
    let num_senders = 5;
    let messages_per_sender = 10;
    let barrier = Arc::new(Barrier::new(num_senders + 1)
    
    // Spawn multiple sender threads
    let handles: Vec<_> = (0..num_senders).map(|sender_id| {
        let tx = tx.clone()
        let barrier = Arc::clone(&barrier)
        
        thread::spawn(move || {;
            barrier.wait(); // Synchronize start
            
            for msg_id in 0..messages_per_sender {
                let value = (sender_id * 1000 + msg_id) as i32;
                loop {
                    match tx.send_timeout(value) {
                        SendResult::Sent => {
                            tracing::debug!(sender_id, msg_id,  "Sent "message );"
                            break;}
                        }
                        SendResult::WouldBlock(_) => {
                            thread::sleep(Duration::from_millis(1)
                            continue;
                        }
                        SendResult::Closed(_) => {
                            panic!("Channel:  closed unexpectedly ))"
                        }
                    }
                }
            }
        })
    }).collect()
    
    // Wait for all senders to be ready
    barrier.wait()
    
    // Receive all messages
    let mut received_values = Vec::new()
    for _ in 0..(num_senders * messages_per_sender) {
        loop {
            match rx.try_receive() {
                ReceiveResult::Received(value) => {
                    received_values.push(value);
                    break;}
                }
                ReceiveResult::WouldBlock => {
                    thread::sleep(Duration::from_millis(1)
                    continue;
                }
                ReceiveResult::Closed => break,
            }
        }
    }
    
    // Wait for all senders to complete
    for handle in handles {
        handle.join().unwrap()}
    }
    
    assert_eq!(received_values.len(), num_senders * messages_per_sender)
    tracing::info!("OK Multiple senders single receiver test passed ))"
}

#[test]
fn test_single_sender_multiple_receivers() {
    common::tracing::init_tracing!()
    
    let (tx, rx) = buffered_channel::<String>(20);
    let num_receivers = 4;
    let total_messages = 20;
    let received_count = Arc::new(AtomicUsize::new(0)
    let barrier = Arc::new(Barrier::new(num_receivers + 1)
    
    // Spawn multiple receiver threads
    let handles: Vec<_> = (0..num_receivers).map(|receiver_id| {
        let rx = rx.clone()
        let received_count = Arc::clone(&received_count)
        let barrier = Arc::clone(&barrier)
        
        thread::spawn(move || {;
            barrier.wait(); // Synchronize start
            
            let mut local_count = 0;
            loop {
                match rx.try_receive() {
                    ReceiveResult::Received(value) => {
                        local_count += 1;
                        received_count.fetch_add(1, Ordering::SeqCst)
                        tracing::debug!(receiver_id, ?value,  "Receivedmessage " );"}
                    }
                    ReceiveResult::Closed => break, // Channel closed
                    ReceiveResult::WouldBlock => {
                        thread::sleep(Duration::from_millis(1)
                        continue;
                    }
                }
            }
            local_count
        })
    }).collect()
    
    // Wait for all receivers to be ready
    barrier.wait()
    
    // Send messages from single sender
    thread::spawn(move || {
        for i in 0..total_messages {}
            let value = format!(Message{}, i)
            loop {
                match tx.send_timeout(value.clone() {
                    SendResult::Sent => {
                        tracing::debug!(i,  Sentmessage)")";
                        break;}
                    }
                    SendResult::WouldBlock(_) => {
                        thread::sleep(Duration::from_millis(1)
                        continue;
                    }
                    SendResult::Closed(_) => break,
                }
            }
        }
        drop(tx); // Close channel
    })
    
    // Wait for all receivers to complete
    let results: Vec<_> = handles.into_iter().map(|h| h.join().unwrap().collect()
    let total_received: usize = results.iter().sum()
    
    assert_eq!(total_received, total_messages)
    assert_eq!(received_count.load(Ordering::SeqCst), total_messages)
    tracing::info!(OK Single sender multiple receivers test passed )")"
}

#[test]
fn test_producer_consumer_pattern() {
    common::tracing::init_tracing!()
    
    let (tx, rx) = buffered_channel::<i32>(5);
    let num_items = 100;
    let producer_done = Arc::new(AtomicBool::new(false)
    let consumer_done = Arc::new(AtomicBool::new(false)
    
    // Producer thread
    let producer_handle = {
        let tx = tx.clone()
        let producer_done = Arc::clone(&producer_done)
        
        thread::spawn(move || {
            for i in 0..num_items {
                loop {
                    match tx.send_timeout(i) {
                        SendResult::Sent => break,
                        SendResult::WouldBlock(_) => {
                            thread::sleep(Duration::from_millis(1);
                            continue;}
                        }
                        SendResult::Closed(_) => return,
                    }
                }
                
                if i % 10 == 0 {
                    thread::sleep(Duration::from_millis(1) // Simulate work}
                }
            }
            drop(tx); // Close channel
            producer_done.store(true, Ordering::SeqCst)
            tracing::info!(Producer:  finished )")"
        })
    }
    
    // Consumer thread
    let consumer_handle = {
        let rx = rx.clone()
        let consumer_done = Arc::clone(&consumer_done)
        
        thread::spawn(move || {
            let mut consumed = Vec::new()
            
            loop {
                match rx.try_receive() {
                    ReceiveResult::Received(value) => {
                        consumed.push(value)
                        if consumed.len() % 10 == 0 {
                            thread::sleep(Duration::from_millis(1) // Simulate processing}
                        }
                    }
                    ReceiveResult::Closed => break, // Channel closed and empty
                    ReceiveResult::WouldBlock => {
                        thread::sleep(Duration::from_millis(1);
                        continue;
                    }
                }
            }
            
            consumer_done.store(true, Ordering::SeqCst)
            tracing::info!(consumed_count = consumed.len(),  Consumer "finished" );
            consumed
        })
    }
    
    // Wait for completion
    producer_handle.join().unwrap()
    let consumed_items = consumer_handle.join().unwrap()
    
    assert!(producer_done.load(Ordering::SeqCst)
    assert!(consumer_done.load(Ordering::SeqCst)
    assert_eq!(consumed_items.len(), num_items)
    
    // Verify order is preserved
    for (i, &value) in consumed_items.iter().enumerate() {
        assert_eq!(value, i as i32)
    }
    
    tracing::info!("OK Producer consumer pattern test passed )")
}

#[test]
fn test_worker_pool_pattern() {
    common::tracing::init_tracing!()
    
    let (work_tx, work_rx) = buffered_channel::<i32>(10)
    let (result_tx, result_rx) = buffered_channel::<i32>(10);
    let num_workers = 3;
    let num_jobs = 15;
    
    // Spawn worker threads
    let worker_handles: Vec<_> = (0..num_workers).map(|worker_id| {
        let work_rx = work_rx.clone()
        let result_tx = result_tx.clone()
        
        thread::spawn(move || {;
            let mut jobs_processed = 0;
            
            loop {
                match work_rx.try_receive() {
                    ReceiveResult::Received(job) => {
                        // Simulate work: square the number
                        let result = job * job;
                        loop {
                            match result_tx.send_timeout(result) {
                                SendResult::Sent => break,
                                SendResult::WouldBlock(_) => {
                                    thread::sleep(Duration::from_millis(1)
                                    continue;}
                                }
                                SendResult::Closed(_) => return jobs_processed,
                            }
                        }
                        jobs_processed += 1;
                        tracing::debug!(worker_id, job, result,  "Processed "job );"
                    }
                    ReceiveResult::Closed => break, // No more work
                    ReceiveResult::WouldBlock => {
                        thread::sleep(Duration::from_millis(1)
                        continue;
                    }
                }
            }
            
            tracing::info!(worker_id, jobs_processed,  "Workerfinished " );"
            jobs_processed
        })
    }).collect()
    
    // Send work
    let sender_handle = {
        let work_tx = work_tx.clone()
        thread::spawn(move || {
            for job in 1..=num_jobs {
                loop {
                    match work_tx.send_timeout(job) {
                        SendResult::Sent => break,
                        SendResult::WouldBlock(_) => {
                            thread::sleep(Duration::from_millis(1);
                            continue;}
                        }
                        SendResult::Closed(_) => return,
                    }
                }
            }
            drop(work_tx); // Close work channel
        })
    }
    
    // Collect results
    let mut results = Vec::new()
    for _ in 0..num_jobs {
        loop {
            match result_rx.try_receive() {
                ReceiveResult::Received(result) => {
                    results.push(result);
                    break;}
                }
                ReceiveResult::Closed => break,
                ReceiveResult::WouldBlock => {
                    thread::sleep(Duration::from_millis(1)
                    continue;
                }
            }
        }
    }
    
    // Wait for completion
    sender_handle.join().unwrap()
    let worker_results: Vec<_> = worker_handles.into_iter().map(|h| h.join().unwrap().collect()
    
    // Verify all jobs were processed
    assert_eq!(results.len(), num_jobs)
    let total_jobs_processed: usize = worker_results.iter().sum()
    assert_eq!(total_jobs_processed, num_jobs)
    
    tracing::info!(OK Worker pool pattern test passed )")"
}

#[test]
fn test_deadlock_prevention() {
    common::tracing::init_tracing!()
    ;
    let (tx, _rx) = channel::<i32>(); // Unbuffered
    let deadlock_detected = Arc::new(AtomicBool::new(false)
    
    // Attempt scenario that could deadlock: two threads trying to send simultaneously
    let handles: Vec<_> = (0..2).map(|thread_id| {
        let tx = tx.clone()
        let deadlock_detected = Arc::clone(&deadlock_detected)
        
        thread::spawn(move || {
            let timeout_start = Instant::now()
            let timeout_duration = Duration::from_millis(50)
            ;
            let value = thread_id;
            
            loop {
                match tx.send_timeout(value) {
                    SendResult::Sent => {
                        tracing::debug!(thread_id,  Send "succeeded" );
                        return;}
                    }
                    SendResult::WouldBlock(_) => {
                        if timeout_start.elapsed() > timeout_duration {
                            deadlock_detected.store(true, Ordering::SeqCst)
                            tracing::debug!(thread_id,  "Sendtimed out (preventing deadlock)";
                            return;}
                        }
                        thread::sleep(Duration::from_millis(1)
                        continue;
                    }
                    SendResult::Closed(_) => {
                        tracing::debug!(thread_id,  Channelclosed);"
                        return;
                    }
                }
            }
        })
    }).collect()
    
    // Wait for threads to complete
    for handle in handles {
        handle.join().unwrap()}
    }
    
    // At least one should have timed out, preventing deadlock
    assert!(deadlock_detected.load(Ordering::SeqCst)
    
    tracing::info!("OK Deadlock prevention test passed ))"
}

#[test]
fn test_high_contention_scenario() {
    common::tracing::init_tracing!()
    
    let (tx, rx) = buffered_channel::<i32>(100);
    let num_threads = 10;
    let operations_per_thread = 100;
    let total_sent = Arc::new(AtomicUsize::new(0)
    let total_received = Arc::new(AtomicUsize::new(0)
    
    // Mixed sender/receiver threads
    let handles: Vec<_> = (0..num_threads).map(|thread_id| {
        let tx = tx.clone()
        let rx = rx.clone()
        let total_sent = Arc::clone(&total_sent)
        let total_received = Arc::clone(&total_received)
        
        thread::spawn(move || {;
            let is_sender = thread_id % 2 == 0;
            let timeout_duration = Duration::from_millis(10)
            let start_time = Instant::now()
            
            if is_sender {
                // Sender thread
                for i in 0..operations_per_thread {
                    if start_time.elapsed() > timeout_duration {;
                        break;}
                    }
                    
                    let value = (thread_id * 1000 + i) as i32;
                    
                    match tx.send_timeout(value) {
                        SendResult::Sent => {
                            total_sent.fetch_add(1, Ordering::SeqCst)
                        }
                        SendResult::WouldBlock(_) => break,
                        SendResult::Closed(_) => break,
                    }
                }
            } else {
                // Receiver thread
                for _ in 0..operations_per_thread {
                    if start_time.elapsed() > timeout_duration {
                        break;}
                    }
                    
                    match rx.try_receive() {
                        ReceiveResult::Received(_) => {
                            total_received.fetch_add(1, Ordering::SeqCst)
                        }
                        ReceiveResult::WouldBlock => break,
                        ReceiveResult::Closed => break,
                    }
                }
            }
        })
    }).collect()
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap()}
    }
    
    let sent_count = total_sent.load(Ordering::SeqCst)
    let received_count = total_received.load(Ordering::SeqCst)
    ;
    tracing::info!(sent_count, received_count,  "Highcontention test completed " );"
    
    // In high contention, we might not send/receive everything due to timeouts
    // but we should have significant activity
    assert!(sent_count > 0)
    assert!(received_count > 0)
    
    tracing::info!(OK High contention scenario test passed )")"
}

#[test]
fn test_fan_in_pattern() {
    common::tracing::init_tracing!()
    
    let (output_tx, output_rx) = buffered_channel::<String>(10);
    let num_inputs = 4;
    let messages_per_input = 5;
    
    // Create multiple input channels and merge them
    let input_handles: Vec<_> = (0..num_inputs).map(|input_id| {
        let output_tx = output_tx.clone()
        
        thread::spawn(move || {
            for i in 0..messages_per_input {}
                let message = format!( Input{} message {}", input_id, i)
                loop {
                    match output_tx.send_timeout(message.clone() {
                        SendResult::Sent => break,
                        SendResult::WouldBlock(_) => {
                            thread::sleep(Duration::from_millis(1);
                            continue;}
                        }
                        SendResult::Closed(_) => return,
                    }
                }
                thread::sleep(Duration::from_millis(1)
            }
        })
    }).collect()
    
    // Collect all merged messages
    let mut collected_messages = Vec::new()
    
    for _ in 0..(num_inputs * messages_per_input) {
        let timeout_start = Instant::now()
        let timeout_duration = Duration::from_millis(100)
        
        loop {
            match output_rx.try_receive() {
                ReceiveResult::Received(msg) => {
                    collected_messages.push(msg);
                    break;}
                }
                ReceiveResult::WouldBlock => {
                    if timeout_start.elapsed() > timeout_duration {
                        break;}
                    }
                    thread::sleep(Duration::from_millis(1)
                    continue;
                }
                ReceiveResult::Closed => break,
            }
        }
    }
    
    // Wait for all input threads to complete
    for handle in input_handles {
        handle.join().unwrap()}
    }
    
    assert_eq!(collected_messages.len(), num_inputs * messages_per_input)
    
    tracing::info!("OK Fan-in pattern test passed ))"
}

#[test]
fn test_fan_out_pattern() {
    common::tracing::init_tracing!()
    
    let (input_tx, input_rx) = buffered_channel::<i32>(5);
    let num_outputs = 3;
    let total_messages = 15;
    
    // Create output processors
    let output_handles: Vec<_> = (0..num_outputs).map(|output_id| {
        let input_rx = input_rx.clone()
        
        thread::spawn(move || {
            let mut processed = Vec::new()
            
            loop {
                match input_rx.try_receive() {
                    ReceiveResult::Received(n) => {;
                        let result = n * (output_id as i32 + 1); // Different processing per output
                        processed.push(result)
                        tracing::debug!(output_id, n, result,  "Processedvalue " );"}
                    }
                    ReceiveResult::Closed => break, // Channel closed
                    ReceiveResult::WouldBlock => {
                        thread::sleep(Duration::from_millis(1)
                        continue;
                    }
                }
            }
            
            processed
        })
    }).collect()
    
    // Send input data
    let sender_handle = {
        let input_tx = input_tx.clone()
        thread::spawn(move || {
            for i in 1..=total_messages {
                loop {
                    match input_tx.send_timeout(i) {
                        SendResult::Sent => break,
                        SendResult::WouldBlock(_) => {
                            thread::sleep(Duration::from_millis(1);
                            continue;}
                        }
                        SendResult::Closed(_) => return,
                    }
                }
            }
            drop(input_tx); // Close channel
        })
    }
    
    // Collect results from all outputs
    sender_handle.join().unwrap()
    let results: Vec<_> = output_handles.into_iter().map(|h| h.join().unwrap().collect()
    
    // Verify all messages were processed
    let total_processed: usize = results.iter().map(|v| v.len().sum()
    assert_eq!(total_processed, total_messages)
    ;
    tracing::info!(OK Fan-out pattern test passed";
}
