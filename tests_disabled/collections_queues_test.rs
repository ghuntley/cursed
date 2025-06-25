/// Comprehensive test suite for CURSED Queue collections
/// 
/// Tests all queue types: Queue, Deque, PriorityQueue, CircularQueue
/// Covers functionality, performance, error handling, and edge cases

use cursed::stdlib::collections::queues::*;
use cursed::stdlib::collections::{CollectionsError, CollectionsResult};
use std::sync::{Arc, Barrier, Mutex};
use std::thread;
use std::time::{Duration, Instant};

// ==================== Queue Tests ====================

#[test]
fn test_queue_fifo_behavior() {
    let mut queue = Queue::new();
    
    // Test FIFO ordering
    for i in 0..10 {
        queue.enqueue(i);
    }
    
    for i in 0..10 {
        assert_eq!(queue.dequeue(), Some(i));
    }
    
    assert!(queue.is_empty());
    assert_eq!(queue.dequeue(), None);
}

#[test]
fn test_queue_capacity_management() {
    let mut queue = Queue::with_capacity(100);
    assert!(queue.capacity() >= 100);
    
    // Test reserve and shrink
    queue.reserve(200);
    assert!(queue.capacity() >= 200);
    
    // Add some elements
    for i in 0..50 {
        queue.enqueue(i);
    }
    
    queue.shrink_to_fit();
    assert_eq!(queue.len(), 50);
}

#[test]
fn test_queue_peek_operations() {
    let mut queue = Queue::new();
    
    assert_eq!(queue.peek(), None);
    assert_eq!(queue.peek_back(), None);
    
    queue.enqueue(1);
    queue.enqueue(2);
    queue.enqueue(3);
    
    assert_eq!(queue.peek(), Some(&1));
    assert_eq!(queue.peek_back(), Some(&3));
    
    // Peek should not modify queue
    assert_eq!(queue.len(), 3);
    assert_eq!(queue.dequeue(), Some(1));
}

#[test]
fn test_queue_bulk_operations() {
    let mut queue = Queue::new();
    
    // Bulk enqueue
    queue.enqueue_all(vec![1, 2, 3, 4, 5]);
    assert_eq!(queue.len(), 5);
    
    // Bulk dequeue
    let items = queue.dequeue_many(3);
    assert_eq!(items, vec![1, 2, 3]);
    assert_eq!(queue.len(), 2);
    
    // Peek many
    let peeked = queue.peek_many(2);
    assert_eq!(peeked, vec![&4, &5]);
    assert_eq!(queue.len(), 2); // Should not modify queue
    
    // Dequeue more than available
    let remaining = queue.dequeue_many(5);
    assert_eq!(remaining, vec![4, 5]);
    assert!(queue.is_empty());
}

#[test]
fn test_queue_drain_filter() {
    let mut queue = Queue::new();
    queue.enqueue_all(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    
    // Remove even numbers
    let evens = queue.drain_filter(|&x| x % 2 == 0);
    assert_eq!(evens, vec![2, 4, 6, 8, 10]);
    
    // Check remaining odds
    let odds: Vec<_> = queue.iter().cloned().collect();
    assert_eq!(odds, vec![1, 3, 5, 7, 9]);
}

#[test]
fn test_queue_from_iterator() {
    let vec = vec![1, 2, 3, 4, 5];
    let queue: Queue<_> = vec.iter().cloned().collect();
    
    assert_eq!(queue.len(), 5);
    assert_eq!(queue.to_vec(), vec);
}

// ==================== Deque Tests ====================

#[test]
fn test_deque_double_ended_operations() {
    let mut deque = Deque::new();
    
    // Test push/pop from both ends
    deque.push_back(2);
    deque.push_front(1);
    deque.push_back(3);
    deque.push_front(0);
    
    // Should be [0, 1, 2, 3]
    assert_eq!(deque.len(), 4);
    assert_eq!(deque.front(), Some(&0));
    assert_eq!(deque.back(), Some(&3));
    
    assert_eq!(deque.pop_front(), Some(0));
    assert_eq!(deque.pop_back(), Some(3));
    assert_eq!(deque.pop_front(), Some(1));
    assert_eq!(deque.pop_back(), Some(2));
    
    assert!(deque.is_empty());
}

#[test]
fn test_deque_indexed_access() {
    let mut deque = Deque::new();
    deque.push_back(10);
    deque.push_back(20);
    deque.push_back(30);
    
    // Test get
    assert_eq!(deque.get(0).unwrap(), &10);
    assert_eq!(deque.get(1).unwrap(), &20);
    assert_eq!(deque.get(2).unwrap(), &30);
    assert!(deque.get(3).is_err());
    
    // Test get_mut
    *deque.get_mut(1).unwrap() = 25;
    assert_eq!(deque.get(1).unwrap(), &25);
    
    // Test insert
    deque.insert(1, 15).unwrap();
    assert_eq!(deque.to_vec(), vec![10, 15, 25, 30]);
    
    // Test remove
    let removed = deque.remove(2).unwrap();
    assert_eq!(removed, 25);
    assert_eq!(deque.to_vec(), vec![10, 15, 30]);
}

#[test]
fn test_deque_rotation() {
    let mut deque = deque_from_vec(vec![1, 2, 3, 4, 5]);
    
    // Rotate left
    deque.rotate_left(2);
    assert_eq!(deque.to_vec(), vec![3, 4, 5, 1, 2]);
    
    // Rotate right
    deque.rotate_right(3);
    assert_eq!(deque.to_vec(), vec![5, 1, 2, 3, 4]);
    
    // Rotate empty deque (should not panic)
    let mut empty_deque = Deque::<i32>::new();
    empty_deque.rotate_left(5);
    empty_deque.rotate_right(3);
    assert!(empty_deque.is_empty());
}

#[test]
fn test_deque_swap() {
    let mut deque = deque_from_vec(vec![1, 2, 3, 4, 5]);
    
    deque.swap(0, 4).unwrap();
    assert_eq!(deque.to_vec(), vec![5, 2, 3, 4, 1]);
    
    deque.swap(1, 3).unwrap();
    assert_eq!(deque.to_vec(), vec![5, 4, 3, 2, 1]);
    
    // Test error cases
    assert!(deque.swap(0, 10).is_err());
    assert!(deque.swap(10, 0).is_err());
}

#[test]
fn test_deque_index_error_handling() {
    let mut deque = Deque::new();
    
    // Empty deque errors
    assert!(matches!(deque.get(0), Err(CollectionsError::IndexOutOfBounds { .. })));
    assert!(matches!(deque.remove(0), Err(CollectionsError::IndexOutOfBounds { .. })));
    assert!(matches!(deque.insert(1, 42), Err(CollectionsError::IndexOutOfBounds { .. })));
    
    deque.push_back(1);
    
    // Valid operations
    assert!(deque.get(0).is_ok());
    assert!(deque.insert(0, 0).is_ok());
    assert!(deque.insert(2, 2).is_ok());
    
    // Invalid indices
    assert!(matches!(deque.get(10), Err(CollectionsError::IndexOutOfBounds { .. })));
}

// ==================== PriorityQueue Tests ====================

#[test]
fn test_priority_queue_max_heap() {
    let mut pq = PriorityQueue::new();
    assert!(!pq.is_min_heap());
    
    let values = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
    for value in values {
        pq.push(value);
    }
    
    assert_eq!(pq.len(), 9);
    assert_eq!(pq.peek(), Some(&9)); // Max element
    
    // Pop in descending order
    let mut result = Vec::new();
    while let Some(item) = pq.pop() {
        result.push(item);
    }
    
    // Should be sorted in descending order
    let mut sorted = result.clone();
    sorted.sort_by(|a, b| b.cmp(a));
    assert_eq!(result, sorted);
    assert_eq!(result[0], 9); // First should be max
}

#[test]
fn test_priority_queue_min_heap() {
    let mut pq = PriorityQueue::new_min();
    assert!(pq.is_min_heap());
    
    let values = vec![3, 1, 4, 1, 5, 9, 2, 6, 5];
    for value in values {
        pq.push(value);
    }
    
    assert_eq!(pq.len(), 9);
    assert_eq!(pq.peek(), Some(&1)); // Min element
    
    // Pop in ascending order
    let mut result = Vec::new();
    while let Some(item) = pq.pop() {
        result.push(item);
    }
    
    // Should be sorted in ascending order
    let mut sorted = result.clone();
    sorted.sort();
    assert_eq!(result, sorted);
    assert_eq!(result[0], 1); // First should be min
}

#[test]
fn test_priority_queue_with_capacity() {
    let pq = PriorityQueue::<i32>::with_capacity(100);
    assert!(pq.capacity() >= 100);
    
    let min_pq = PriorityQueue::<i32>::with_capacity_min(50);
    assert!(min_pq.capacity() >= 50);
    assert!(min_pq.is_min_heap());
}

#[test]
fn test_priority_queue_bulk_operations() {
    let mut pq = PriorityQueue::new();
    
    // Bulk push
    pq.push_all(vec![5, 2, 8, 1, 9, 3]);
    assert_eq!(pq.len(), 6);
    
    // Pop many
    let top_three = pq.pop_many(3);
    assert_eq!(top_three.len(), 3);
    assert_eq!(top_three[0], 9); // Highest priority first
    assert_eq!(pq.len(), 3);
    
    // Pop more than available
    let remaining = pq.pop_many(10);
    assert_eq!(remaining.len(), 3);
    assert!(pq.is_empty());
}

#[test]
fn test_priority_queue_drain_filter() {
    let mut pq = priority_queue_from_vec(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    
    // Remove multiples of 3
    let multiples_of_three = pq.drain_filter(|&x| x % 3 == 0);
    assert_eq!(multiples_of_three.len(), 3); // 3, 6, 9
    assert_eq!(pq.len(), 7);
    
    // Verify remaining elements don't include multiples of 3
    let remaining = pq.to_vec();
    for &item in &remaining {
        assert_ne!(item % 3, 0);
    }
}

#[test]
fn test_priority_queue_sorting() {
    let values = vec![64, 34, 25, 12, 22, 11, 90];
    let pq = priority_queue_from_vec(values.clone());
    
    let sorted = pq.to_sorted_vec();
    let mut expected = values;
    expected.sort_by(|a, b| b.cmp(a)); // Descending for max heap
    assert_eq!(sorted, expected);
}

#[test]
fn test_priority_queue_from_iterator() {
    let vec = vec![3, 1, 4, 1, 5];
    let pq: PriorityQueue<_> = vec.iter().cloned().collect();
    
    assert_eq!(pq.len(), 5);
    assert_eq!(pq.peek(), Some(&5)); // Max element
}

// ==================== CircularQueue Tests ====================

#[test]
fn test_circular_queue_basic_operations() {
    let mut cq = CircularQueue::new(5).unwrap();
    
    assert!(cq.is_empty());
    assert!(!cq.is_full());
    assert_eq!(cq.capacity(), 5);
    assert_eq!(cq.len(), 0);
    
    // Fill the queue
    for i in 1..=5 {
        cq.enqueue(i).unwrap();
    }
    
    assert!(cq.is_full());
    assert!(!cq.is_empty());
    assert_eq!(cq.len(), 5);
    
    // Should fail when full
    assert!(cq.enqueue(6).is_err());
    
    // Dequeue and check order
    for i in 1..=5 {
        assert_eq!(cq.peek(), Some(&i));
        assert_eq!(cq.dequeue(), Some(i));
    }
    
    assert!(cq.is_empty());
    assert_eq!(cq.dequeue(), None);
}

#[test]
fn test_circular_queue_wrap_around() {
    let mut cq = CircularQueue::new(3).unwrap();
    
    // Fill and partially empty
    cq.enqueue(1).unwrap();
    cq.enqueue(2).unwrap();
    cq.enqueue(3).unwrap();
    
    assert_eq!(cq.dequeue(), Some(1));
    assert_eq!(cq.dequeue(), Some(2));
    
    // Add more elements (should wrap around)
    cq.enqueue(4).unwrap();
    cq.enqueue(5).unwrap();
    
    // Check final state
    assert_eq!(cq.to_vec(), vec![3, 4, 5]);
    assert_eq!(cq.len(), 3);
    assert!(cq.is_full());
}

#[test]
fn test_circular_queue_force_enqueue() {
    let mut cq = CircularQueue::new(3).unwrap();
    
    // Fill the queue
    cq.enqueue(1).unwrap();
    cq.enqueue(2).unwrap();
    cq.enqueue(3).unwrap();
    
    // Force enqueue should remove oldest element
    let removed = cq.force_enqueue(4);
    assert_eq!(removed, Some(1));
    assert_eq!(cq.to_vec(), vec![2, 3, 4]);
    
    // Force enqueue on non-full queue
    cq.dequeue();
    let removed = cq.force_enqueue(5);
    assert_eq!(removed, None);
    assert_eq!(cq.to_vec(), vec![3, 4, 5]);
}

#[test]
fn test_circular_queue_indexed_access() {
    let mut cq = CircularQueue::new(5).unwrap();
    
    // Add elements
    for i in 10..=14 {
        cq.enqueue(i).unwrap();
    }
    
    // Test get method
    for i in 0..5 {
        assert_eq!(cq.get(i).unwrap(), &(10 + i));
    }
    
    // Test out of bounds
    assert!(cq.get(5).is_err());
    
    // Test with wrap-around
    cq.dequeue();
    cq.dequeue();
    cq.enqueue(15).unwrap();
    cq.enqueue(16).unwrap();
    
    // Should be [12, 13, 14, 15, 16]
    for (i, &expected) in [12, 13, 14, 15, 16].iter().enumerate() {
        assert_eq!(cq.get(i).unwrap(), &expected);
    }
}

#[test]
fn test_circular_queue_peek_operations() {
    let mut cq = CircularQueue::new(3).unwrap();
    
    assert_eq!(cq.peek(), None);
    assert_eq!(cq.peek_back(), None);
    
    cq.enqueue(1).unwrap();
    assert_eq!(cq.peek(), Some(&1));
    assert_eq!(cq.peek_back(), Some(&1));
    
    cq.enqueue(2).unwrap();
    cq.enqueue(3).unwrap();
    assert_eq!(cq.peek(), Some(&1));
    assert_eq!(cq.peek_back(), Some(&3));
    
    cq.dequeue();
    assert_eq!(cq.peek(), Some(&2));
    assert_eq!(cq.peek_back(), Some(&3));
}

#[test]
fn test_circular_queue_iterator() {
    let mut cq = CircularQueue::new(5).unwrap();
    
    // Empty iterator
    assert_eq!(cq.iter().count(), 0);
    
    // Add elements
    for i in 1..=3 {
        cq.enqueue(i).unwrap();
    }
    
    let collected: Vec<_> = cq.iter().cloned().collect();
    assert_eq!(collected, vec![1, 2, 3]);
    
    // Test with wrap-around
    cq.dequeue();
    cq.enqueue(4).unwrap();
    cq.enqueue(5).unwrap();
    
    let collected: Vec<_> = cq.iter().cloned().collect();
    assert_eq!(collected, vec![2, 3, 4, 5]);
}

#[test]
fn test_circular_queue_clear() {
    let mut cq = CircularQueue::new(3).unwrap();
    
    cq.enqueue(1).unwrap();
    cq.enqueue(2).unwrap();
    cq.enqueue(3).unwrap();
    
    assert_eq!(cq.len(), 3);
    
    cq.clear();
    assert!(cq.is_empty());
    assert_eq!(cq.len(), 0);
    assert_eq!(cq.capacity(), 3);
    
    // Should be able to use after clear
    cq.enqueue(10).unwrap();
    assert_eq!(cq.dequeue(), Some(10));
}

#[test]
fn test_circular_queue_error_handling() {
    // Invalid capacity
    assert!(matches!(
        CircularQueue::<i32>::new(0),
        Err(CollectionsError::InvalidCapacity { .. })
    ));
    
    let mut cq = CircularQueue::new(2).unwrap();
    
    // Overflow when full
    cq.enqueue(1).unwrap();
    cq.enqueue(2).unwrap();
    assert!(matches!(
        cq.enqueue(3),
        Err(CollectionsError::InsufficientMemory { .. })
    ));
    
    // Index out of bounds
    assert!(matches!(
        cq.get(5),
        Err(CollectionsError::IndexOutOfBounds { .. })
    ));
}

// ==================== Thread Safety Tests ====================

#[test]
fn test_thread_safe_queue() {
    let queue = ThreadSafeQueue::new();
    let queue_clone = queue.clone();
    
    let producer = thread::spawn(move || {
        for i in 0..100 {
            queue_clone.enqueue(i).unwrap();
        }
    });
    
    let consumer = thread::spawn(move || {
        let mut received = Vec::new();
        for _ in 0..100 {
            loop {
                if let Ok(Some(item)) = queue.dequeue() {
                    received.push(item);
                    break;
                } else {
                    thread::sleep(Duration::from_millis(1));
                }
            }
        }
        received
    });
    
    producer.join().unwrap();
    let received = consumer.join().unwrap();
    
    assert_eq!(received.len(), 100);
    // Note: Order may not be preserved due to concurrency
}

#[test]
fn test_thread_safe_deque() {
    let deque = ThreadSafeDeque::new();
    let num_threads = 4;
    let items_per_thread = 25;
    
    let barrier = Arc::new(Barrier::new(num_threads));
    let handles: Vec<_> = (0..num_threads)
        .map(|thread_id| {
            let deque_clone = deque.clone();
            let barrier_clone = barrier.clone();
            
            thread::spawn(move || {
                barrier_clone.wait();
                
                for i in 0..items_per_thread {
                    let value = thread_id * items_per_thread + i;
                    if thread_id % 2 == 0 {
                        deque_clone.push_front(value).unwrap();
                    } else {
                        deque_clone.push_back(value).unwrap();
                    }
                }
            })
        })
        .collect();
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    assert_eq!(deque.len().unwrap(), num_threads * items_per_thread);
}

// ==================== Performance Tests ====================

#[test]
#[ignore] // Run with --ignored flag for performance testing
fn test_queue_performance() {
    let mut queue = Queue::new();
    let n = 100_000;
    
    // Enqueue performance
    let start = Instant::now();
    for i in 0..n {
        queue.enqueue(i);
    }
    let enqueue_time = start.elapsed();
    println!("Queue enqueue {} items: {:?}", n, enqueue_time);
    assert!(enqueue_time < Duration::from_millis(100)); // Should be fast
    
    // Dequeue performance
    let start = Instant::now();
    for _ in 0..n {
        queue.dequeue();
    }
    let dequeue_time = start.elapsed();
    println!("Queue dequeue {} items: {:?}", n, dequeue_time);
    assert!(dequeue_time < Duration::from_millis(100)); // Should be fast
}

#[test]
#[ignore] // Run with --ignored flag for performance testing
fn test_priority_queue_performance() {
    let mut pq = PriorityQueue::new();
    let n = 10_000;
    
    // Push performance
    let start = Instant::now();
    for i in 0..n {
        pq.push(n - i); // Reverse order to test heap operations
    }
    let push_time = start.elapsed();
    println!("PriorityQueue push {} items: {:?}", n, push_time);
    assert!(push_time < Duration::from_secs(1)); // Should be reasonable for heap
    
    // Pop performance
    let start = Instant::now();
    for _ in 0..n {
        pq.pop();
    }
    let pop_time = start.elapsed();
    println!("PriorityQueue pop {} items: {:?}", n, pop_time);
    assert!(pop_time < Duration::from_secs(1)); // Should be reasonable for heap
}

#[test]
#[ignore] // Run with --ignored flag for performance testing
fn test_circular_queue_performance() {
    let mut cq = CircularQueue::new(1000).unwrap();
    let n = 100_000;
    
    // Test sustained enqueue/dequeue operations
    let start = Instant::now();
    for i in 0..n {
        // Fill and empty cycle
        for j in 0..1000 {
            cq.enqueue((i * 1000 + j) % 1000000).unwrap();
        }
        for _ in 0..1000 {
            cq.dequeue();
        }
    }
    let cycle_time = start.elapsed();
    println!("CircularQueue {} cycles: {:?}", n, cycle_time);
    assert!(cycle_time < Duration::from_secs(5)); // Should be very fast
}

// ==================== Edge Cases and Stress Tests ====================

#[test]
fn test_queue_edge_cases() {
    let mut queue = Queue::<String>::new();
    
    // Empty operations
    assert_eq!(queue.dequeue(), None);
    assert_eq!(queue.peek(), None);
    assert_eq!(queue.peek_back(), None);
    assert!(queue.is_empty());
    assert_eq!(queue.len(), 0);
    
    // Single element
    queue.enqueue("test".to_string());
    assert_eq!(queue.peek(), Some(&"test".to_string()));
    assert_eq!(queue.peek_back(), Some(&"test".to_string()));
    assert_eq!(queue.len(), 1);
    assert_eq!(queue.dequeue(), Some("test".to_string()));
    assert!(queue.is_empty());
    
    // Clear empty queue
    queue.clear();
    assert!(queue.is_empty());
    
    // Bulk operations on empty
    assert_eq!(queue.dequeue_many(10), Vec::<String>::new());
    assert_eq!(queue.peek_many(5), Vec::<&String>::new());
}

#[test]
fn test_deque_edge_cases() {
    let mut deque = Deque::<i32>::new();
    
    // Empty operations
    assert_eq!(deque.pop_front(), None);
    assert_eq!(deque.pop_back(), None);
    assert_eq!(deque.front(), None);
    assert_eq!(deque.back(), None);
    
    // Rotation on empty
    deque.rotate_left(10);
    deque.rotate_right(5);
    assert!(deque.is_empty());
    
    // Single element rotation
    deque.push_back(42);
    deque.rotate_left(1);
    assert_eq!(deque.front(), Some(&42));
    deque.rotate_right(1);
    assert_eq!(deque.front(), Some(&42));
}

#[test]
fn test_priority_queue_edge_cases() {
    let mut pq = PriorityQueue::<i32>::new();
    
    // Empty operations
    assert_eq!(pq.pop(), None);
    assert_eq!(pq.peek(), None);
    assert!(pq.is_empty());
    
    // Single element
    pq.push(42);
    assert_eq!(pq.peek(), Some(&42));
    assert_eq!(pq.pop(), Some(42));
    assert!(pq.is_empty());
    
    // Duplicate elements in min heap
    let mut min_pq = PriorityQueue::new_min();
    for _ in 0..5 {
        min_pq.push(10);
    }
    assert_eq!(min_pq.len(), 5);
    for _ in 0..5 {
        assert_eq!(min_pq.pop(), Some(10));
    }
    assert!(min_pq.is_empty());
}

#[test]
fn test_circular_queue_edge_cases() {
    let mut cq = CircularQueue::new(1).unwrap();
    
    // Single capacity queue
    cq.enqueue(100).unwrap();
    assert!(cq.is_full());
    assert_eq!(cq.peek(), Some(&100));
    assert_eq!(cq.peek_back(), Some(&100));
    assert_eq!(cq.dequeue(), Some(100));
    assert!(cq.is_empty());
    
    // Force enqueue on single capacity
    cq.enqueue(200).unwrap();
    let removed = cq.force_enqueue(300);
    assert_eq!(removed, Some(200));
    assert_eq!(cq.peek(), Some(&300));
}

// ==================== Convenience Functions Tests ====================

#[test]
fn test_convenience_functions() {
    let vec = vec![5, 2, 8, 1, 9, 3];
    
    // Test queue_from_vec
    let queue = queue_from_vec(vec.clone());
    assert_eq!(queue.len(), 6);
    assert_eq!(queue.to_vec(), vec);
    
    // Test deque_from_vec
    let deque = deque_from_vec(vec.clone());
    assert_eq!(deque.len(), 6);
    assert_eq!(deque.to_vec(), vec);
    
    // Test priority_queue_from_vec
    let pq = priority_queue_from_vec(vec.clone());
    assert_eq!(pq.len(), 6);
    assert_eq!(pq.peek(), Some(&9)); // Max element
    
    // Test min_priority_queue_from_vec
    let min_pq = min_priority_queue_from_vec(vec.clone());
    assert_eq!(min_pq.len(), 6);
    assert_eq!(min_pq.peek(), Some(&1)); // Min element
    
    // Test circular_queue_from_vec
    let cq = circular_queue_from_vec(vec.clone()).unwrap();
    assert_eq!(cq.len(), 6);
    assert_eq!(cq.capacity(), 6);
    assert_eq!(cq.to_vec(), vec);
    
    // Test empty vector
    let empty_cq = circular_queue_from_vec(Vec::<i32>::new()).unwrap();
    assert_eq!(empty_cq.capacity(), 1); // Minimum capacity
    assert!(empty_cq.is_empty());
}

// ==================== Memory and Resource Tests ====================

#[test]
fn test_memory_efficiency() {
    // Test that queues don't hold onto unnecessary memory
    let mut queue = Queue::with_capacity(1000);
    
    // Fill with large data
    for i in 0..1000 {
        queue.enqueue(format!("Large string data item {}", i));
    }
    
    // Empty the queue
    for _ in 0..1000 {
        queue.dequeue();
    }
    
    // Shrink should reduce memory usage
    queue.shrink_to_fit();
    assert!(queue.is_empty());
    
    // Should still be usable
    queue.enqueue("test".to_string());
    assert_eq!(queue.len(), 1);
}

#[test]
fn test_large_queue_operations() {
    let mut queue = Queue::new();
    let large_size = 10_000;
    
    // Test with large number of elements
    for i in 0..large_size {
        queue.enqueue(i);
    }
    
    assert_eq!(queue.len(), large_size);
    
    // Dequeue half
    for i in 0..large_size / 2 {
        assert_eq!(queue.dequeue(), Some(i));
    }
    
    assert_eq!(queue.len(), large_size / 2);
    
    // Add more elements
    for i in large_size..large_size + 1000 {
        queue.enqueue(i);
    }
    
    assert_eq!(queue.len(), large_size / 2 + 1000);
}

#[test]
fn test_queue_with_complex_types() {
    #[derive(Debug, Clone, PartialEq)]
    struct ComplexType {
        id: usize,
        data: Vec<String>,
        nested: Option<Box<ComplexType>>,
    }
    
    let mut queue = Queue::new();
    
    let item1 = ComplexType {
        id: 1,
        data: vec!["test".to_string(), "data".to_string()],
        nested: None,
    };
    
    let item2 = ComplexType {
        id: 2,
        data: vec!["more".to_string(), "test".to_string(), "data".to_string()],
        nested: Some(Box::new(item1.clone())),
    };
    
    queue.enqueue(item1.clone());
    queue.enqueue(item2.clone());
    
    assert_eq!(queue.dequeue(), Some(item1));
    assert_eq!(queue.dequeue(), Some(item2));
    assert!(queue.is_empty());
}
