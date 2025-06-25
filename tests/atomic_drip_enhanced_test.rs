use cursed::stdlib::atomic_drip::*;
use std::sync::Arc;
use std::thread;
use std::time::Duration;

#[test]
fn test_atomic_wait_group_basic() {
    let wg = WaitGroup::new();
    assert_eq!(wg.count(), 0);
    
    wg.add(3).unwrap();
    assert_eq!(wg.count(), 3);
    
    wg.done().unwrap();
    assert_eq!(wg.count(), 2);
    
    wg.done().unwrap();
    wg.done().unwrap();
    assert_eq!(wg.count(), 0);
}

#[test]
fn test_atomic_wait_group_concurrent() {
    let wg = Arc::new(WaitGroup::new());
    let wg_waiter = Arc::clone(&wg);
    
    wg.add(3).unwrap();
    
    let start_time = std::time::Instant::now();
    
    // Spawn goroutines that will signal done after delays
    for i in 0..3 {
        let wg_worker = Arc::clone(&wg);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(50 + i * 10));
            wg_worker.done().unwrap();
        });
    }
    
    // Wait for all to complete
    wg_waiter.wait().unwrap();
    
    let elapsed = start_time.elapsed();
    assert!(elapsed >= Duration::from_millis(70)); // Should take at least 70ms
    assert_eq!(wg.count(), 0);
}

#[test]
fn test_atomic_bitfield32_operations() {
    let bf = Bitfield32::new(0);
    
    // Test basic bit operations
    bf.set_bit(0).unwrap();
    bf.set_bit(2).unwrap();
    bf.set_bit(5).unwrap();
    
    assert!(bf.test_bit(0).unwrap());
    assert!(!bf.test_bit(1).unwrap());
    assert!(bf.test_bit(2).unwrap());
    assert!(!bf.test_bit(3).unwrap());
    assert!(!bf.test_bit(4).unwrap());
    assert!(bf.test_bit(5).unwrap());
    
    assert_eq!(bf.count_set_bits(), 3);
    assert_eq!(bf.find_first_set(), Some(0));
    assert_eq!(bf.find_last_set(), Some(5));
    
    bf.clear_bit(2).unwrap();
    assert!(!bf.test_bit(2).unwrap());
    assert_eq!(bf.count_set_bits(), 2);
}

#[test]
fn test_atomic_bitfield_concurrent() {
    let bf = Arc::new(Bitfield32::new(0));
    let mut handles = vec![];
    
    // Each thread sets a different bit
    for i in 0..16 {
        let bf_clone = Arc::clone(&bf);
        let handle = thread::spawn(move || {
            bf_clone.set_bit(i).unwrap();
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify all bits are set
    for i in 0..16 {
        assert!(bf.test_bit(i).unwrap());
    }
    assert_eq!(bf.count_set_bits(), 16);
}

#[test]
fn test_atomic_queue_fifo() {
    let queue = Queue::new();
    assert!(queue.is_empty());
    
    // Push items
    queue.push(1);
    queue.push(2);
    queue.push(3);
    
    assert!(!queue.is_empty());
    assert_eq!(queue.len(), 3);
    
    // Pop items in FIFO order
    assert_eq!(queue.pop(), Some(1));
    assert_eq!(queue.pop(), Some(2));
    assert_eq!(queue.pop(), Some(3));
    assert_eq!(queue.pop(), None);
    
    assert!(queue.is_empty());
}

#[test]
fn test_atomic_queue_concurrent() {
    let queue = Arc::new(Queue::new());
    let mut producer_handles = vec![];
    let mut consumer_handles = vec![];
    let consumed_items = Arc::new(std::sync::Mutex::new(Vec::new()));
    
    // Spawn producers
    for producer_id in 0..4 {
        let q = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            for i in 0..100 {
                q.push(producer_id * 100 + i);
            }
        });
        producer_handles.push(handle);
    }
    
    // Spawn consumers
    for _ in 0..4 {
        let q = Arc::clone(&queue);
        let items = Arc::clone(&consumed_items);
        let handle = thread::spawn(move || {
            let mut local_items = Vec::new();
            for _ in 0..100 {
                while let Some(item) = q.pop() {
                    local_items.push(item);
                    break;
                }
                if local_items.len() < 100 {
                    thread::sleep(Duration::from_millis(1));
                }
            }
            if let Ok(mut items_guard) = items.lock() {
                items_guard.extend(local_items);
            }
        });
        consumer_handles.push(handle);
    }
    
    // Wait for all producers
    for handle in producer_handles {
        handle.join().unwrap();
    }
    
    // Wait for all consumers
    for handle in consumer_handles {
        handle.join().unwrap();
    }
    
    // Verify all items were consumed
    let final_items = consumed_items.lock().unwrap();
    assert_eq!(final_items.len(), 400);
}

#[test]
fn test_atomic_stack_lifo() {
    let stack = Stack::new();
    assert!(stack.is_empty());
    
    // Push items
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    assert!(!stack.is_empty());
    assert_eq!(stack.len(), 3);
    
    // Pop items in LIFO order
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), None);
    
    assert!(stack.is_empty());
}

#[test]
fn test_atomic_counter_operations() {
    let counter = Counter::new(10);
    
    assert_eq!(counter.get(), 10);
    assert_eq!(counter.min(), 10);
    assert_eq!(counter.max(), 10);
    
    assert_eq!(counter.increment(), 11);
    assert_eq!(counter.add(5), 16);
    assert_eq!(counter.max(), 16);
    
    assert_eq!(counter.decrement(), 15);
    assert_eq!(counter.subtract(3), 12);
    assert_eq!(counter.min(), 10);
    assert_eq!(counter.max(), 16);
    
    // Test compare and swap
    assert_eq!(counter.compare_and_swap(12, 20), Ok(12));
    assert_eq!(counter.get(), 20);
    assert_eq!(counter.compare_and_swap(12, 25), Err(20));
    assert_eq!(counter.get(), 20);
}

#[test]
fn test_atomic_counter_concurrent() {
    let counter = Arc::new(Counter::new(0));
    let mut handles = vec![];
    
    // Multiple threads incrementing
    for _ in 0..8 {
        let c = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                c.increment();
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    assert_eq!(counter.get(), 800);
    assert_eq!(counter.max(), 800);
}

#[test]
fn test_atomic_flag_basic() {
    let flag = Flag::new(false);
    
    assert!(!flag.load());
    assert_eq!(flag.set_count(), 0);
    assert!(!flag.has_been_set());
    
    flag.set();
    assert!(flag.load());
    assert_eq!(flag.set_count(), 1);
    assert!(flag.has_been_set());
    
    flag.clear();
    assert!(!flag.load());
    assert_eq!(flag.set_count(), 1); // Count doesn't reset
    assert!(flag.has_been_set());
}

#[test]
fn test_atomic_flag_set_if_unset() {
    let flag = Arc::new(Flag::new(false));
    let mut handles = vec![];
    let winners = Arc::new(std::sync::Mutex::new(Vec::new()));
    
    // Multiple threads trying to set the flag
    for i in 0..10 {
        let flag_clone = Arc::clone(&flag);
        let winners_clone = Arc::clone(&winners);
        let handle = thread::spawn(move || {
            let won = flag_clone.set_if_unset();
            if won {
                if let Ok(mut w) = winners_clone.lock() {
                    w.push(i);
                }
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Exactly one thread should have won
    let winner_list = winners.lock().unwrap();
    assert_eq!(winner_list.len(), 1);
    assert!(flag.load());
    assert_eq!(flag.set_count(), 1);
}

#[test]
fn test_atomic_flag_toggle() {
    let flag = Flag::new(false);
    
    let old = flag.toggle();
    assert!(!old); // Previous value was false
    assert!(flag.load()); // Current value is true
    
    let old = flag.toggle();
    assert!(old); // Previous value was true
    assert!(!flag.load()); // Current value is false
}

#[test]
fn test_state_flag_basic() {
    let flag = new_state_flag(4).unwrap(); // States 0, 1, 2, 3
    
    assert_eq!(flag.get(), 0);
    assert!(flag.is_min_state());
    assert!(!flag.is_max_state());
    
    assert!(flag.advance());
    assert_eq!(flag.get(), 1);
    
    assert!(flag.advance());
    assert_eq!(flag.get(), 2);
    
    flag.set(3).unwrap();
    assert!(flag.is_max_state());
    
    // Can't advance past max
    assert!(!flag.advance());
    assert_eq!(flag.get(), 3);
    
    assert!(flag.retreat());
    assert_eq!(flag.get(), 2);
}

#[test]
fn test_state_flag_concurrent() {
    let flag = Arc::new(new_state_flag(20).unwrap());
    let mut handles = vec![];
    
    // Multiple threads advancing
    for _ in 0..8 {
        let flag_clone = Arc::clone(&flag);
        let handle = thread::spawn(move || {
            for _ in 0..5 {
                flag_clone.advance();
                thread::sleep(Duration::from_millis(1));
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Should have advanced but not exceeded max
    let final_state = flag.get();
    assert!(final_state > 0);
    assert!(final_state <= 19);
}

#[test]
fn test_atomic_value_operations() {
    let atomic = Value::new(42i32);
    
    assert_eq!(atomic.load(), Some(42));
    assert!(atomic.is_set());
    
    assert!(atomic.store(100));
    assert_eq!(atomic.load(), Some(100));
    
    let old = atomic.swap(200);
    assert_eq!(old, Some(100));
    assert_eq!(atomic.load(), Some(200));
    
    assert!(atomic.compare_and_swap(200, 300));
    assert_eq!(atomic.load(), Some(300));
    
    assert!(!atomic.compare_and_swap(200, 400));
    assert_eq!(atomic.load(), Some(300));
}

#[test]
fn test_atomic_string_operations() {
    let atomic = AtomicString::from_str("hello");
    
    assert_eq!(atomic.len(), Some(5));
    assert!(!atomic.is_empty());
    assert_eq!(atomic.load(), Some("hello".to_string()));
    
    assert!(atomic.push_str(" world"));
    assert_eq!(atomic.load(), Some("hello world".to_string()));
    assert_eq!(atomic.len(), Some(11));
    
    let old = atomic.swap("goodbye".to_string());
    assert_eq!(old, Some("hello world".to_string()));
    assert_eq!(atomic.load(), Some("goodbye".to_string()));
}

#[test]
fn test_atomic_vec_operations() {
    let atomic = AtomicVec::new_vec();
    
    assert_eq!(atomic.len(), Some(0));
    assert!(atomic.is_empty());
    
    assert!(atomic.push(1));
    assert!(atomic.push(2));
    assert!(atomic.push(3));
    
    assert_eq!(atomic.len(), Some(3));
    assert!(!atomic.is_empty());
    
    assert_eq!(atomic.pop(), Some(3));
    assert_eq!(atomic.len(), Some(2));
    
    let vec_copy = atomic.load();
    assert_eq!(vec_copy, Some(vec![1, 2]));
}

#[test]
fn test_memory_ordering_operations() {
    let flag = Flag::new(false);
    let counter = Int32::new(0);
    
    // Test different memory orderings
    flag.set_ordered(MemoryOrder::Release);
    assert!(flag.load_ordered(MemoryOrder::Acquire));
    
    counter.store_ordered(42, MemoryOrder::Release);
    assert_eq!(counter.load_ordered(MemoryOrder::Acquire), 42);
    
    let old = counter.add_ordered(10, MemoryOrder::Relaxed);
    assert_eq!(old, 42);
    assert_eq!(counter.load(), 52);
    
    // Test memory barriers
    fence::memory_fence(MemoryOrder::AcquireRelease);
    fence::compiler_fence(MemoryOrder::Relaxed);
}

#[test]
fn test_enhanced_atomic_features_integration() {
    // Test integration of all enhanced features
    let wg = Arc::new(WaitGroup::new());
    let counter = Arc::new(Counter::new(0));
    let flag = Arc::new(Flag::new(false));
    let queue = Arc::new(Queue::new());
    let bitfield = Arc::new(Bitfield32::new(0));
    
    // Setup work
    wg.add(4).unwrap();
    
    // Worker 1: Counter operations
    {
        let wg_ref = Arc::clone(&wg);
        let counter_ref = Arc::clone(&counter);
        thread::spawn(move || {
            for _ in 0..100 {
                counter_ref.increment();
            }
            wg_ref.done().unwrap();
        });
    }
    
    // Worker 2: Queue operations
    {
        let wg_ref = Arc::clone(&wg);
        let queue_ref = Arc::clone(&queue);
        thread::spawn(move || {
            for i in 0..50 {
                queue_ref.push(i);
            }
            wg_ref.done().unwrap();
        });
    }
    
    // Worker 3: Bitfield operations
    {
        let wg_ref = Arc::clone(&wg);
        let bitfield_ref = Arc::clone(&bitfield);
        thread::spawn(move || {
            for i in 0..16 {
                bitfield_ref.set_bit(i).unwrap();
            }
            wg_ref.done().unwrap();
        });
    }
    
    // Worker 4: Flag operations
    {
        let wg_ref = Arc::clone(&wg);
        let flag_ref = Arc::clone(&flag);
        thread::spawn(move || {
            thread::sleep(Duration::from_millis(50));
            flag_ref.set();
            wg_ref.done().unwrap();
        });
    }
    
    // Wait for all workers to complete
    wg.wait().unwrap();
    
    // Verify results
    assert_eq!(counter.get(), 100);
    assert_eq!(queue.len(), 50);
    assert_eq!(bitfield.count_set_bits(), 16);
    assert!(flag.load());
    assert!(flag.has_been_set());
    
    // Clean up queue
    let mut items = Vec::new();
    while let Some(item) = queue.pop() {
        items.push(item);
    }
    assert_eq!(items.len(), 50);
}

#[test]
fn test_atomic_module_initialization() {
    // Test that module initializes correctly
    assert!(atomic_drip::init().is_ok());
}

#[test]
fn test_error_handling() {
    // Test error cases
    let bf = Bitfield32::new(0);
    assert!(bf.set_bit(32).is_err()); // Out of range
    assert!(bf.test_bit(32).is_err()); // Out of range
    
    let wg = WaitGroup::new();
    assert!(wg.done().is_err()); // Counter would go negative
    
    assert!(new_state_flag(0).is_err()); // Invalid state count
}
