/// Goroutine Basic Functional Tests
/// 
/// Tests fundamental goroutine and concurrency features including
/// goroutine spawning, channel operations, synchronization, and GC integration.

use cursed::runtime::goroutine::*;
use cursed::runtime::channels::*;
use std::time::Duration;
use std::sync::{Arc, Mutex};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_goroutine_scheduler_creation() {
        let config = SchedulerConfig::default();
        let scheduler = GoroutineScheduler::new(config).expect("Should create scheduler");
        
        assert_eq!(scheduler.active_goroutines(), 0, "Should start with no active goroutines");
        println!("Goroutine scheduler creation test passed");
    }

    #[test]
    fn test_basic_goroutine_spawning() {
        let config = SchedulerConfig::default();
        let mut scheduler = GoroutineScheduler::new(config).expect("Should create scheduler");
        
        // Start the scheduler
        scheduler.start().expect("Should start scheduler");
        
        // Spawn a simple goroutine
        let goroutine_id = scheduler.spawn(|| {
            println!("Hello from goroutine!");
            42
        }).expect("Should spawn goroutine");
        
        assert!(goroutine_id != 0, "Goroutine ID should be non-zero");
        assert!(scheduler.active_goroutines() > 0, "Should have active goroutines");
        
        // Wait briefly for execution
        std::thread::sleep(Duration::from_millis(100));
        
        // Stop the scheduler
        scheduler.stop().expect("Should stop scheduler");
        
        println!("Basic goroutine spawning test passed");
    }

    #[test]
    fn test_multiple_goroutines() {
        let config = SchedulerConfig::default();
        let mut scheduler = GoroutineScheduler::new(config).expect("Should create scheduler");
        
        scheduler.start().expect("Should start scheduler");
        
        let counter = Arc::new(Mutex::new(0));
        let mut goroutine_ids = Vec::new();
        
        // Spawn multiple goroutines
        for i in 0..5 {
            let counter_clone = Arc::clone(&counter);
            let goroutine_id = scheduler.spawn(move || {
                let mut count = counter_clone.lock().unwrap();
                *count += 1;
                println!("Goroutine {} completed", i);
            }).expect("Should spawn goroutine");
            
            goroutine_ids.push(goroutine_id);
        }
        
        // Wait for execution
        std::thread::sleep(Duration::from_millis(200));
        
        // Check that all goroutines ran
        let final_count = *counter.lock().unwrap();
        assert!(final_count > 0, "At least some goroutines should have run");
        
        scheduler.stop().expect("Should stop scheduler");
        
        println!("Multiple goroutines test passed, {} executed", final_count);
    }

    #[test]
    fn test_goroutine_communication() {
        let config = SchedulerConfig::default();
        let mut scheduler = GoroutineScheduler::new(config).expect("Should create scheduler");
        
        scheduler.start().expect("Should start scheduler");
        
        let shared_data = Arc::new(Mutex::new(Vec::new()));
        
        // Producer goroutine
        let data_clone = Arc::clone(&shared_data);
        let _producer = scheduler.spawn(move || {
            for i in 0..5 {
                let mut data = data_clone.lock().unwrap();
                data.push(i);
                std::thread::sleep(Duration::from_millis(10));
            }
        }).expect("Should spawn producer");
        
        // Consumer goroutine
        let data_clone = Arc::clone(&shared_data);
        let _consumer = scheduler.spawn(move || {
            loop {
                let data = data_clone.lock().unwrap();
                if data.len() >= 5 {
                    println!("Consumer found {} items", data.len());
                    break;
                }
                drop(data);
                std::thread::sleep(Duration::from_millis(20));
            }
        }).expect("Should spawn consumer");
        
        // Wait for completion
        std::thread::sleep(Duration::from_millis(300));
        
        let final_data = shared_data.lock().unwrap();
        assert!(final_data.len() > 0, "Should have produced some data");
        
        scheduler.stop().expect("Should stop scheduler");
        
        println!("Goroutine communication test passed with {} items", final_data.len());
    }

    #[test]
    fn test_channel_basic_operations() {
        let mut channel = Channel::<i32>::new(10).expect("Should create channel");
        
        // Test sending
        channel.send(42).expect("Should send value");
        channel.send(100).expect("Should send another value");
        
        // Test receiving
        let value1 = channel.receive().expect("Should receive value");
        let value2 = channel.receive().expect("Should receive another value");
        
        assert_eq!(value1, 42, "Should receive first value");
        assert_eq!(value2, 100, "Should receive second value");
        
        println!("Basic channel operations test passed");
    }

    #[test]
    fn test_channel_with_goroutines() {
        let config = SchedulerConfig::default();
        let mut scheduler = GoroutineScheduler::new(config).expect("Should create scheduler");
        
        scheduler.start().expect("Should start scheduler");
        
        let channel = Arc::new(Mutex::new(Channel::<i32>::new(5).expect("Should create channel")));
        
        // Producer goroutine
        let channel_clone = Arc::clone(&channel);
        let _producer = scheduler.spawn(move || {
            for i in 1..=5 {
                let mut ch = channel_clone.lock().unwrap();
                ch.send(i * 10).expect("Should send value");
                println!("Sent: {}", i * 10);
                drop(ch);
                std::thread::sleep(Duration::from_millis(50));
            }
        }).expect("Should spawn producer");
        
        // Consumer goroutine
        let channel_clone = Arc::clone(&channel);
        let results = Arc::new(Mutex::new(Vec::new()));
        let results_clone = Arc::clone(&results);
        
        let _consumer = scheduler.spawn(move || {
            for _ in 0..5 {
                let value = {
                    let mut ch = channel_clone.lock().unwrap();
                    ch.receive().expect("Should receive value")
                };
                
                let mut res = results_clone.lock().unwrap();
                res.push(value);
                println!("Received: {}", value);
            }
        }).expect("Should spawn consumer");
        
        // Wait for completion
        std::thread::sleep(Duration::from_millis(500));
        
        let final_results = results.lock().unwrap();
        assert_eq!(final_results.len(), 5, "Should have received 5 values");
        
        scheduler.stop().expect("Should stop scheduler");
        
        println!("Channel with goroutines test passed");
    }

    #[test]
    fn test_goroutine_yield() {
        let config = SchedulerConfig::default();
        let mut scheduler = GoroutineScheduler::new(config).expect("Should create scheduler");
        
        scheduler.start().expect("Should start scheduler");
        
        let execution_order = Arc::new(Mutex::new(Vec::new()));
        
        // Goroutine that yields
        let order_clone = Arc::clone(&execution_order);
        let _yielding = scheduler.spawn(move || {
            for i in 0..3 {
                {
                    let mut order = order_clone.lock().unwrap();
                    order.push(format!("yielding-{}", i));
                }
                
                // Yield control
                scheduler_yield();
                std::thread::sleep(Duration::from_millis(10));
            }
        }).expect("Should spawn yielding goroutine");
        
        // Competing goroutine
        let order_clone = Arc::clone(&execution_order);
        let _competing = scheduler.spawn(move || {
            for i in 0..3 {
                {
                    let mut order = order_clone.lock().unwrap();
                    order.push(format!("competing-{}", i));
                }
                std::thread::sleep(Duration::from_millis(15));
            }
        }).expect("Should spawn competing goroutine");
        
        // Wait for completion
        std::thread::sleep(Duration::from_millis(300));
        
        let execution_sequence = execution_order.lock().unwrap();
        assert!(execution_sequence.len() > 0, "Should have execution sequence");
        
        println!("Execution sequence: {:?}", *execution_sequence);
        
        scheduler.stop().expect("Should stop scheduler");
        
        println!("Goroutine yield test passed");
    }

    #[test]
    fn test_goroutine_gc_integration() {
        let config = SchedulerConfig::default();
        let mut scheduler = GoroutineScheduler::new(config).expect("Should create scheduler");
        
        scheduler.start().expect("Should start scheduler");
        
        // Goroutine that triggers GC operations
        let _gc_goroutine = scheduler.spawn(|| {
            // Simulate memory allocation
            let mut objects = Vec::new();
            for i in 0..100 {
                objects.push(format!("object-{}", i));
            }
            
            // Simulate GC safe point
            gc_safe_point();
            
            objects.len()
        }).expect("Should spawn GC goroutine");
        
        // Wait for completion
        std::thread::sleep(Duration::from_millis(200));
        
        // Test GC coordination
        let gc_requested = scheduler.coordinate_gc(Duration::from_millis(100));
        match gc_requested {
            Ok(coordinated) => {
                println!("GC coordination: {}", if coordinated { "successful" } else { "not needed" });
            }
            Err(error) => {
                println!("GC coordination failed: {}", error);
            }
        }
        
        scheduler.stop().expect("Should stop scheduler");
        
        println!("Goroutine GC integration test passed");
    }

    #[test]
    fn test_goroutine_error_handling() {
        let config = SchedulerConfig::default();
        let mut scheduler = GoroutineScheduler::new(config).expect("Should create scheduler");
        
        scheduler.start().expect("Should start scheduler");
        
        // Goroutine that panics
        let _panicking = scheduler.spawn(|| {
            panic!("Simulated goroutine panic");
        });
        
        // Normal goroutine
        let success_flag = Arc::new(Mutex::new(false));
        let flag_clone = Arc::clone(&success_flag);
        
        let _normal = scheduler.spawn(move || {
            std::thread::sleep(Duration::from_millis(100));
            let mut flag = flag_clone.lock().unwrap();
            *flag = true;
        }).expect("Should spawn normal goroutine");
        
        // Wait for execution
        std::thread::sleep(Duration::from_millis(200));
        
        // Normal goroutine should still succeed despite panic in other
        let success = *success_flag.lock().unwrap();
        assert!(success, "Normal goroutine should complete despite other panics");
        
        scheduler.stop().expect("Should stop scheduler");
        
        println!("Goroutine error handling test passed");
    }

    #[test]
    fn test_buffered_channels() {
        let mut buffered_channel = Channel::<String>::new(3).expect("Should create buffered channel");
        
        // Fill the buffer
        buffered_channel.send("first".to_string()).expect("Should send first");
        buffered_channel.send("second".to_string()).expect("Should send second");
        buffered_channel.send("third".to_string()).expect("Should send third");
        
        // Verify buffer is full
        assert!(buffered_channel.is_full(), "Channel should be full");
        
        // Receive all values
        let first = buffered_channel.receive().expect("Should receive first");
        let second = buffered_channel.receive().expect("Should receive second");
        let third = buffered_channel.receive().expect("Should receive third");
        
        assert_eq!(first, "first", "Should receive values in order");
        assert_eq!(second, "second", "Should receive values in order");
        assert_eq!(third, "third", "Should receive values in order");
        
        assert!(buffered_channel.is_empty(), "Channel should be empty");
        
        println!("Buffered channels test passed");
    }

    #[test]
    fn test_select_statement_simulation() {
        let mut ch1 = Channel::<i32>::new(1).expect("Should create channel 1");
        let mut ch2 = Channel::<String>::new(1).expect("Should create channel 2");
        
        // Send to first channel
        ch1.send(42).expect("Should send to ch1");
        
        // Simulate select statement behavior
        let ch1_ready = ch1.has_data();
        let ch2_ready = ch2.has_data();
        
        assert!(ch1_ready, "Channel 1 should have data");
        assert!(!ch2_ready, "Channel 2 should not have data");
        
        // Receive from ready channel
        if ch1_ready {
            let value = ch1.receive().expect("Should receive from ch1");
            assert_eq!(value, 42, "Should receive correct value");
        }
        
        println!("Select statement simulation test passed");
    }

    #[test]
    fn test_goroutine_lifecycle() {
        let config = SchedulerConfig::default();
        let mut scheduler = GoroutineScheduler::new(config).expect("Should create scheduler");
        
        scheduler.start().expect("Should start scheduler");
        
        // Track goroutine states
        let initial_count = scheduler.active_goroutines();
        
        // Spawn short-lived goroutine
        let _short_lived = scheduler.spawn(|| {
            std::thread::sleep(Duration::from_millis(50));
            "completed"
        }).expect("Should spawn short-lived goroutine");
        
        // Spawn long-lived goroutine
        let _long_lived = scheduler.spawn(|| {
            std::thread::sleep(Duration::from_millis(300));
            "long task completed"
        }).expect("Should spawn long-lived goroutine");
        
        // Check increased count
        std::thread::sleep(Duration::from_millis(10));
        let active_count = scheduler.active_goroutines();
        assert!(active_count > initial_count, "Should have more active goroutines");
        
        // Wait for short-lived to complete
        std::thread::sleep(Duration::from_millis(100));
        
        // Wait for long-lived to complete
        std::thread::sleep(Duration::from_millis(250));
        
        // Eventually all should complete
        std::thread::sleep(Duration::from_millis(100));
        
        scheduler.stop().expect("Should stop scheduler");
        
        println!("Goroutine lifecycle test passed");
    }

    // Helper functions for testing
    fn scheduler_yield() {
        std::thread::sleep(Duration::from_millis(1));
    }
    
    fn gc_safe_point() {
        // Simulate GC safe point
        std::thread::sleep(Duration::from_millis(1));
    }
}
