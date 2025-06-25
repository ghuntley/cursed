/// Channels Performance Test Suite
/// 
/// Comprehensive performance validation for channel operations including:
/// - Channel creation/destruction throughput
/// - Message passing throughput (send/receive)
/// - Multi-producer multi-consumer scalability  
/// - Buffer management efficiency
/// - Backpressure handling performance
/// - Channel closure and cleanup performance

use std::sync::{Arc, Barrier, mpsc};
use std::thread;
use std::time::{Duration, Instant};
use std::collections::HashMap;

// Mock channel structures for performance testing
#[derive(Clone, Debug)]
struct Message {
    id: usize,
    data: Vec<u8>,
    timestamp: Instant,
}

#[derive(Debug)]
struct ChannelMetrics {
    messages_sent: usize,
    messages_received: usize,
    send_throughput: f64,        // messages per second
    receive_throughput: f64,     // messages per second
    average_latency: Duration,   // send to receive time
    peak_buffer_usage: usize,
    channel_operations: usize,   // create/close operations
    memory_allocated: usize,
    failed_operations: usize,
}

impl ChannelMetrics {
    fn new() -> Self {
        Self {
            messages_sent: 0,
            messages_received: 0,
            send_throughput: 0.0,
            receive_throughput: 0.0,
            average_latency: Duration::from_nanos(0),
            peak_buffer_usage: 0,
            channel_operations: 0,
            memory_allocated: 0,
            failed_operations: 0,
        }
    }

    fn record_send(&mut self) {
        self.messages_sent += 1;
    }

    fn record_receive(&mut self, latency: Duration) {
        self.messages_received += 1;
        
        // Update average latency
        let total_nanos = self.average_latency.as_nanos() * (self.messages_received - 1) as u128;
        self.average_latency = Duration::from_nanos(
            ((total_nanos + latency.as_nanos()) / self.messages_received as u128) as u64
        );
    }

    fn record_channel_operation(&mut self) {
        self.channel_operations += 1;
    }

    fn record_failed_operation(&mut self) {
        self.failed_operations += 1;
    }

    fn calculate_throughput(&mut self, duration: Duration) {
        if duration.as_secs_f64() > 0.0 {
            self.send_throughput = self.messages_sent as f64 / duration.as_secs_f64();
            self.receive_throughput = self.messages_received as f64 / duration.as_secs_f64();
        }
    }
}

// Mock buffered channel implementation
struct MockBufferedChannel<T> {
    sender: mpsc::Sender<T>,
    receiver: mpsc::Receiver<T>,
    buffer_size: usize,
    current_size: Arc<std::sync::atomic::AtomicUsize>,
}

impl<T> MockBufferedChannel<T> {
    fn new(buffer_size: usize) -> Self {
        let (sender, receiver) = mpsc::channel();
        Self {
            sender,
            receiver,
            buffer_size,
            current_size: Arc::new(std::sync::atomic::AtomicUsize::new(0)),
        }
    }

    fn send(&self, item: T) -> Result<(), mpsc::SendError<T>> {
        self.current_size.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
        self.sender.send(item)
    }

    fn try_recv(&self) -> Result<T, mpsc::TryRecvError> {
        let result = self.receiver.try_recv();
        if result.is_ok() {
            self.current_size.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
        }
        result
    }

    fn recv_timeout(&self, timeout: Duration) -> Result<T, mpsc::RecvTimeoutError> {
        let result = self.receiver.recv_timeout(timeout);
        if result.is_ok() {
            self.current_size.fetch_sub(1, std::sync::atomic::Ordering::Relaxed);
        }
        result
    }

    fn current_buffer_size(&self) -> usize {
        self.current_size.load(std::sync::atomic::Ordering::Relaxed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_channel_creation_destruction_throughput() {
        let mut metrics = ChannelMetrics::new();
        let target_throughput = 1000.0; // channel operations per second
        let test_duration = Duration::from_secs(3);
        
        println!("Testing channel creation/destruction throughput...");
        
        let start_time = Instant::now();
        let mut channels_created = 0;
        
        while start_time.elapsed() < test_duration {
            // Create multiple channels in batch
            for _ in 0..10 {
                let _channel: MockBufferedChannel<Message> = MockBufferedChannel::new(100);
                channels_created += 1;
                metrics.record_channel_operation();
                
                // Simulate some work with the channel before dropping
                // (In real scenario, this would test actual channel usage)
            }
            // Channels are automatically dropped here
        }
        
        let total_duration = start_time.elapsed();
        let operation_throughput = channels_created as f64 / total_duration.as_secs_f64();
        
        println!("Channel Creation Results:");
        println!("  Channels created: {}", channels_created);
        println!("  Duration: {:?}", total_duration);
        println!("  Creation throughput: {:.2} channels/sec", operation_throughput);
        println!("  Target: {:.2} channels/sec", target_throughput);
        
        assert!(
            operation_throughput >= target_throughput,
            "Channel creation throughput {:.2} below target {:.2}",
            operation_throughput, target_throughput
        );
    }

    #[test]
    fn test_message_passing_throughput() {
        let mut metrics = ChannelMetrics::new();
        let target_throughput = 10000.0; // messages per second
        let buffer_size = 1000;
        let test_duration = Duration::from_secs(5);
        
        println!("Testing message passing throughput...");
        
        let channel = Arc::new(MockBufferedChannel::new(buffer_size));
        let channel_clone = channel.clone();
        
        // Receiver thread
        let receiver_metrics = Arc::new(std::sync::Mutex::new(ChannelMetrics::new()));
        let receiver_metrics_clone = receiver_metrics.clone();
        
        let receiver_handle = thread::spawn(move || {
            while let Ok(message) = channel_clone.recv_timeout(Duration::from_millis(100)) {
                let latency = message.timestamp.elapsed();
                receiver_metrics_clone.lock().unwrap().record_receive(latency);
            }
        });
        
        // Sender thread  
        let start_time = Instant::now();
        let mut message_id = 0;
        
        while start_time.elapsed() < test_duration {
            let message = Message {
                id: message_id,
                data: vec![0u8; 64], // 64-byte messages
                timestamp: Instant::now(),
            };
            
            if channel.send(message).is_ok() {
                metrics.record_send();
                message_id += 1;
            } else {
                metrics.record_failed_operation();
                break;
            }
            
            // Brief yield to prevent overwhelming the channel
            if message_id % 1000 == 0 {
                thread::yield_now();
            }
        }
        
        // Allow receiver to catch up
        thread::sleep(Duration::from_millis(500));
        drop(channel); // Close sender
        
        receiver_handle.join().unwrap();
        
        let total_duration = start_time.elapsed();
        metrics.calculate_throughput(total_duration);
        
        let receiver_metrics = receiver_metrics.lock().unwrap();
        
        println!("Message Passing Results:");
        println!("  Messages sent: {}", metrics.messages_sent);
        println!("  Messages received: {}", receiver_metrics.messages_received);
        println!("  Send throughput: {:.2} msgs/sec", metrics.send_throughput);
        println!("  Receive throughput: {:.2} msgs/sec", receiver_metrics.receive_throughput);
        println!("  Average latency: {:?}", receiver_metrics.average_latency);
        println!("  Failed operations: {}", metrics.failed_operations);
        
        assert!(
            metrics.send_throughput >= target_throughput,
            "Send throughput {:.2} below target {:.2}",
            metrics.send_throughput, target_throughput
        );
        
        // Verify message delivery reliability
        let delivery_rate = receiver_metrics.messages_received as f64 / metrics.messages_sent as f64;
        assert!(
            delivery_rate >= 0.95,
            "Message delivery rate {:.2}% below 95%",
            delivery_rate * 100.0
        );
        
        // Verify low latency
        assert!(
            receiver_metrics.average_latency <= Duration::from_millis(10),
            "Average latency {:?} exceeds 10ms",
            receiver_metrics.average_latency
        );
    }

    #[test]
    fn test_multi_producer_multi_consumer_scalability() {
        let producer_counts = vec![1, 2, 4];
        let consumer_counts = vec![1, 2, 4];
        let mut scaling_results = HashMap::new();
        
        println!("Testing multi-producer multi-consumer scalability...");
        
        for &producers in &producer_counts {
            for &consumers in &consumer_counts {
                let channel = Arc::new(MockBufferedChannel::new(10000));
                let barrier = Arc::new(Barrier::new(producers + consumers));
                let start_time = Arc::new(std::sync::Mutex::new(None));
                let total_sent = Arc::new(std::sync::atomic::AtomicUsize::new(0));
                let total_received = Arc::new(std::sync::atomic::AtomicUsize::new(0));
                
                // Producer threads
                let mut producer_handles = Vec::new();
                for producer_id in 0..producers {
                    let channel = channel.clone();
                    let barrier = barrier.clone();
                    let start_time = start_time.clone();
                    let total_sent = total_sent.clone();
                    
                    let handle = thread::spawn(move || {
                        barrier.wait();
                        
                        // Record start time from first thread
                        {
                            let mut start = start_time.lock().unwrap();
                            if start.is_none() {
                                *start = Some(Instant::now());
                            }
                        }
                        
                        let mut sent = 0;
                        let end_time = Instant::now() + Duration::from_secs(3);
                        
                        while Instant::now() < end_time {
                            let message = Message {
                                id: producer_id * 100000 + sent,
                                data: vec![0u8; 32],
                                timestamp: Instant::now(),
                            };
                            
                            if channel.send(message).is_ok() {
                                sent += 1;
                            } else {
                                break;
                            }
                            
                            if sent % 100 == 0 {
                                thread::yield_now();
                            }
                        }
                        
                        total_sent.fetch_add(sent, std::sync::atomic::Ordering::Relaxed);
                        sent
                    });
                    
                    producer_handles.push(handle);
                }
                
                // Consumer threads
                let mut consumer_handles = Vec::new();
                for consumer_id in 0..consumers {
                    let channel = channel.clone();
                    let barrier = barrier.clone();
                    let total_received = total_received.clone();
                    
                    let handle = thread::spawn(move || {
                        barrier.wait();
                        
                        let mut received = 0;
                        let end_time = Instant::now() + Duration::from_secs(4); // Extra time for cleanup
                        
                        while Instant::now() < end_time {
                            match channel.recv_timeout(Duration::from_millis(10)) {
                                Ok(_message) => {
                                    received += 1;
                                },
                                Err(mpsc::RecvTimeoutError::Timeout) => {
                                    // Continue trying
                                },
                                Err(mpsc::RecvTimeoutError::Disconnected) => {
                                    break;
                                },
                            }
                        }
                        
                        total_received.fetch_add(received, std::sync::atomic::Ordering::Relaxed);
                        received
                    });
                    
                    consumer_handles.push(handle);
                }
                
                // Wait for all threads to complete
                let mut producer_results = Vec::new();
                for handle in producer_handles {
                    producer_results.push(handle.join().unwrap());
                }
                
                drop(channel); // Close channel to signal consumers
                
                let mut consumer_results = Vec::new();
                for handle in consumer_handles {
                    consumer_results.push(handle.join().unwrap());
                }
                
                let duration = {
                    let start = start_time.lock().unwrap();
                    start.unwrap().elapsed()
                };
                
                let sent = total_sent.load(std::sync::atomic::Ordering::Relaxed);
                let received = total_received.load(std::sync::atomic::Ordering::Relaxed);
                let throughput = sent as f64 / duration.as_secs_f64();
                
                scaling_results.insert((producers, consumers), throughput);
                
                println!("{}P/{}C: sent={}, received={}, throughput={:.2} msgs/sec", 
                        producers, consumers, sent, received, throughput);
                println!("  Producer results: {:?}", producer_results);
                println!("  Consumer results: {:?}", consumer_results);
                
                // Verify message delivery
                let delivery_rate = received as f64 / sent as f64;
                assert!(
                    delivery_rate >= 0.9,
                    "Delivery rate {:.2}% below 90% for {}P/{}C",
                    delivery_rate * 100.0, producers, consumers
                );
            }
        }
        
        // Verify scaling efficiency
        let baseline_throughput = scaling_results[&(1, 1)];
        
        for &producers in &producer_counts[1..] {
            let scaled_throughput = scaling_results[&(producers, 1)];
            let scaling_efficiency = scaled_throughput / (baseline_throughput * producers as f64);
            
            println!("Producer scaling {}P/1C: {:.2}x speedup, {:.2}% efficiency", 
                    producers, scaled_throughput / baseline_throughput, scaling_efficiency * 100.0);
            
            // Allow some overhead, expect at least 60% scaling efficiency
            assert!(
                scaling_efficiency >= 0.6,
                "Producer scaling efficiency {:.2}% below 60% for {}P",
                scaling_efficiency * 100.0, producers
            );
        }
    }

    #[test]
    fn test_buffer_management_efficiency() {
        let buffer_sizes = vec![10, 100, 1000, 10000];
        let mut buffer_results = HashMap::new();
        
        println!("Testing buffer management efficiency...");
        
        for &buffer_size in &buffer_sizes {
            let channel = Arc::new(MockBufferedChannel::new(buffer_size));
            let channel_clone = channel.clone();
            let mut metrics = ChannelMetrics::new();
            
            // Consumer thread (slower than producer to test buffering)
            let receiver_handle = thread::spawn(move || {
                let mut received = 0;
                while let Ok(_message) = channel_clone.recv_timeout(Duration::from_millis(100)) {
                    received += 1;
                    // Simulate slow consumer
                    thread::sleep(Duration::from_micros(100));
                }
                received
            });
            
            // Producer thread
            let start_time = Instant::now();
            let mut sent = 0;
            let mut peak_buffer = 0;
            
            // Send messages until buffer is full or test duration exceeded
            while start_time.elapsed() < Duration::from_secs(5) {
                let message = Message {
                    id: sent,
                    data: vec![0u8; 64],
                    timestamp: Instant::now(),
                };
                
                if channel.send(message).is_ok() {
                    sent += 1;
                    metrics.record_send();
                    
                    let current_buffer = channel.current_buffer_size();
                    peak_buffer = peak_buffer.max(current_buffer);
                    
                    // Brief pause to allow buffer management
                    if sent % 100 == 0 {
                        thread::sleep(Duration::from_micros(50));
                    }
                } else {
                    metrics.record_failed_operation();
                    break;
                }
            }
            
            let total_duration = start_time.elapsed();
            metrics.calculate_throughput(total_duration);
            metrics.peak_buffer_usage = peak_buffer;
            
            drop(channel); // Close sender
            let received = receiver_handle.join().unwrap();
            
            let buffer_utilization = peak_buffer as f64 / buffer_size as f64;
            buffer_results.insert(buffer_size, (metrics.send_throughput, buffer_utilization));
            
            println!("Buffer size {}: sent={}, received={}, peak_buffer={}/{} ({:.1}%), throughput={:.2}", 
                    buffer_size, sent, received, peak_buffer, buffer_size, 
                    buffer_utilization * 100.0, metrics.send_throughput);
            
            // Verify buffer utilization is reasonable
            if buffer_size >= 100 {
                assert!(
                    buffer_utilization >= 0.5,
                    "Buffer utilization {:.1}% below 50% for buffer size {}",
                    buffer_utilization * 100.0, buffer_size
                );
            }
        }
        
        // Verify that larger buffers allow higher throughput
        let small_buffer_throughput = buffer_results[&10].0;
        let large_buffer_throughput = buffer_results[&10000].0;
        
        let throughput_improvement = large_buffer_throughput / small_buffer_throughput;
        println!("Throughput improvement (10K vs 10 buffer): {:.2}x", throughput_improvement);
        
        assert!(
            throughput_improvement >= 1.5,
            "Large buffer throughput improvement {:.2}x below 1.5x minimum",
            throughput_improvement
        );
    }

    #[test]
    fn test_backpressure_handling_performance() {
        let mut metrics = ChannelMetrics::new();
        let buffer_size = 100;
        let channel = Arc::new(MockBufferedChannel::new(buffer_size));
        
        println!("Testing backpressure handling performance...");
        
        // Create a slow consumer to trigger backpressure
        let channel_clone = channel.clone();
        let consumer_handle = thread::spawn(move || {
            let mut received = 0;
            while let Ok(_message) = channel_clone.recv_timeout(Duration::from_millis(1000)) {
                received += 1;
                // Very slow consumer to ensure backpressure
                thread::sleep(Duration::from_millis(10));
            }
            received
        });
        
        // Fast producer that will hit backpressure
        let start_time = Instant::now();
        let mut sent = 0;
        let mut backpressure_events = 0;
        let test_duration = Duration::from_secs(3);
        
        while start_time.elapsed() < test_duration {
            let message = Message {
                id: sent,
                data: vec![0u8; 128],
                timestamp: Instant::now(),
            };
            
            let send_start = Instant::now();
            if channel.send(message).is_ok() {
                sent += 1;
                metrics.record_send();
                
                // Detect backpressure by send time
                if send_start.elapsed() > Duration::from_millis(1) {
                    backpressure_events += 1;
                }
            } else {
                metrics.record_failed_operation();
                backpressure_events += 1;
            }
            
            // Track buffer usage
            let buffer_usage = channel.current_buffer_size();
            metrics.peak_buffer_usage = metrics.peak_buffer_usage.max(buffer_usage);
        }
        
        let total_duration = start_time.elapsed();
        metrics.calculate_throughput(total_duration);
        
        drop(channel); // Close sender
        let received = consumer_handle.join().unwrap();
        
        let backpressure_rate = backpressure_events as f64 / sent as f64;
        
        println!("Backpressure Handling Results:");
        println!("  Messages sent: {}", sent);
        println!("  Messages received: {}", received);
        println!("  Backpressure events: {}", backpressure_events);
        println!("  Backpressure rate: {:.2}%", backpressure_rate * 100.0);
        println!("  Peak buffer usage: {}/{}", metrics.peak_buffer_usage, buffer_size);
        println!("  Send throughput: {:.2} msgs/sec", metrics.send_throughput);
        println!("  Failed operations: {}", metrics.failed_operations);
        
        // Verify backpressure is working (should occur frequently with slow consumer)
        assert!(
            backpressure_rate >= 0.1,
            "Backpressure rate {:.2}% below 10% - backpressure may not be working",
            backpressure_rate * 100.0
        );
        
        // Verify system handles backpressure gracefully
        assert!(
            metrics.failed_operations < sent / 10,
            "Too many failed operations: {} > {}",
            metrics.failed_operations, sent / 10
        );
        
        // Verify buffer is being utilized effectively
        assert!(
            metrics.peak_buffer_usage >= buffer_size / 2,
            "Buffer underutilized: {} < {}",
            metrics.peak_buffer_usage, buffer_size / 2
        );
    }

    #[test]
    fn test_channel_closure_and_cleanup_performance() {
        let mut metrics = ChannelMetrics::new();
        let channel_count = 1000;
        let messages_per_channel = 100;
        
        println!("Testing channel closure and cleanup performance...");
        
        let start_time = Instant::now();
        
        for channel_id in 0..channel_count {
            let channel = Arc::new(MockBufferedChannel::new(50));
            let channel_clone = channel.clone();
            
            // Send some messages
            for msg_id in 0..messages_per_channel {
                let message = Message {
                    id: channel_id * messages_per_channel + msg_id,
                    data: vec![0u8; 32],
                    timestamp: Instant::now(),
                };
                
                if channel.send(message).is_ok() {
                    metrics.record_send();
                }
            }
            
            // Receive some messages (simulating partial consumption)
            let receive_count = messages_per_channel / 2;
            for _ in 0..receive_count {
                if let Ok(message) = channel_clone.try_recv() {
                    let latency = message.timestamp.elapsed();
                    metrics.record_receive(latency);
                }
            }
            
            // Channel is dropped here (automatic cleanup)
            metrics.record_channel_operation();
            
            if channel_id % 100 == 0 {
                println!("Processed {} channels...", channel_id + 1);
            }
        }
        
        let total_duration = start_time.elapsed();
        metrics.calculate_throughput(total_duration);
        
        let channel_ops_per_sec = channel_count as f64 / total_duration.as_secs_f64();
        let cleanup_time_per_channel = total_duration / channel_count as u32;
        
        println!("Channel Cleanup Results:");
        println!("  Channels processed: {}", channel_count);
        println!("  Messages sent: {}", metrics.messages_sent);
        println!("  Messages received: {}", metrics.messages_received);
        println!("  Total duration: {:?}", total_duration);
        println!("  Channel operations/sec: {:.2}", channel_ops_per_sec);
        println!("  Cleanup time per channel: {:?}", cleanup_time_per_channel);
        println!("  Average message latency: {:?}", metrics.average_latency);
        
        // Verify cleanup performance
        assert!(
            channel_ops_per_sec >= 100.0,
            "Channel cleanup throughput {:.2} below 100 ops/sec",
            channel_ops_per_sec
        );
        
        assert!(
            cleanup_time_per_channel <= Duration::from_millis(10),
            "Cleanup time per channel {:?} exceeds 10ms",
            cleanup_time_per_channel
        );
        
        // Verify message processing efficiency during cleanup test
        let message_processing_rate = metrics.messages_received as f64 / metrics.messages_sent as f64;
        assert!(
            message_processing_rate >= 0.4, // At least 40% processed (we only try to receive 50%)
            "Message processing rate {:.2}% below 40%",
            message_processing_rate * 100.0
        );
    }

    #[test]
    fn test_concurrent_channel_operations_stress() {
        let thread_count = 8;
        let operations_per_thread = 1000;
        let barrier = Arc::new(Barrier::new(thread_count));
        let total_operations = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        let failed_operations = Arc::new(std::sync::atomic::AtomicUsize::new(0));
        
        println!("Testing concurrent channel operations stress...");
        
        let start_time = Arc::new(std::sync::Mutex::new(None));
        
        let handles: Vec<_> = (0..thread_count).map(|thread_id| {
            let barrier = barrier.clone();
            let total_operations = total_operations.clone();
            let failed_operations = failed_operations.clone();
            let start_time = start_time.clone();
            
            thread::spawn(move || {
                barrier.wait();
                
                // Record start time from first thread
                {
                    let mut start = start_time.lock().unwrap();
                    if start.is_none() {
                        *start = Some(Instant::now());
                    }
                }
                
                let mut local_operations = 0;
                let mut local_failures = 0;
                
                for op_id in 0..operations_per_thread {
                    // Mix of different channel operations
                    match op_id % 4 {
                        0 => {
                            // Create and immediately drop channel
                            let _channel: MockBufferedChannel<Message> = MockBufferedChannel::new(10);
                            local_operations += 1;
                        },
                        1 => {
                            // Create channel, send messages, drop
                            let channel = MockBufferedChannel::new(20);
                            for i in 0..5 {
                                let message = Message {
                                    id: thread_id * 10000 + op_id * 10 + i,
                                    data: vec![0u8; 16],
                                    timestamp: Instant::now(),
                                };
                                if channel.send(message).is_err() {
                                    local_failures += 1;
                                }
                            }
                            local_operations += 1;
                        },
                        2 => {
                            // Create channel, send and receive
                            let channel = MockBufferedChannel::new(15);
                            let message = Message {
                                id: thread_id * 10000 + op_id,
                                data: vec![0u8; 24],
                                timestamp: Instant::now(),
                            };
                            if channel.send(message).is_ok() {
                                if channel.recv_timeout(Duration::from_millis(1)).is_err() {
                                    // Expected for timeout, not a failure
                                }
                            } else {
                                local_failures += 1;
                            }
                            local_operations += 1;
                        },
                        3 => {
                            // Rapid create/drop cycle
                            for _ in 0..3 {
                                let _channel: MockBufferedChannel<i32> = MockBufferedChannel::new(5);
                            }
                            local_operations += 1;
                        },
                        _ => unreachable!(),
                    }
                    
                    // Brief yield every 50 operations
                    if op_id % 50 == 0 {
                        thread::yield_now();
                    }
                }
                
                total_operations.fetch_add(local_operations, std::sync::atomic::Ordering::Relaxed);
                failed_operations.fetch_add(local_failures, std::sync::atomic::Ordering::Relaxed);
                
                (local_operations, local_failures)
            })
        }).collect();
        
        // Wait for all threads to complete
        let mut thread_results = Vec::new();
        for handle in handles {
            thread_results.push(handle.join().unwrap());
        }
        
        let duration = {
            let start = start_time.lock().unwrap();
            start.unwrap().elapsed()
        };
        
        let total_ops = total_operations.load(std::sync::atomic::Ordering::Relaxed);
        let total_failures = failed_operations.load(std::sync::atomic::Ordering::Relaxed);
        let ops_per_sec = total_ops as f64 / duration.as_secs_f64();
        let failure_rate = total_failures as f64 / total_ops as f64;
        
        println!("Concurrent Stress Test Results:");
        println!("  Threads: {}", thread_count);
        println!("  Total operations: {}", total_ops);
        println!("  Failed operations: {}", total_failures);
        println!("  Failure rate: {:.2}%", failure_rate * 100.0);
        println!("  Duration: {:?}", duration);
        println!("  Operations/sec: {:.2}", ops_per_sec);
        println!("  Thread results: {:?}", thread_results);
        
        // Verify stress test performance
        assert!(
            ops_per_sec >= 1000.0,
            "Concurrent operations throughput {:.2} below 1000 ops/sec",
            ops_per_sec
        );
        
        // Verify system stability under stress
        assert!(
            failure_rate <= 0.05, // Allow up to 5% failures under extreme stress
            "Failure rate {:.2}% exceeds 5% maximum",
            failure_rate * 100.0
        );
        
        // Verify all threads completed successfully
        assert_eq!(thread_results.len(), thread_count);
        let successful_threads = thread_results.iter().filter(|(ops, _)| *ops > 0).count();
        assert_eq!(successful_threads, thread_count, "Not all threads completed successfully");
    }
}
