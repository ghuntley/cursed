/// Comprehensive Integration Tests for CURSED Collections System
/// 
/// This test suite validates interoperability between all collection types,
/// cross-collection operations, performance characteristics, and real-world
/// usage scenarios combining multiple collection types.

mod common;

use cursed::stdlib::collections::*;
use std::time::{Duration, Instant};
use std::collections::HashMap;

/// Initialize test logging for integration tests
/// Using function call instead of macro to avoid conflicts

#[cfg(test)]
mod integration_tests {
    use super::*;

    #[test]
    fn test_basic_collection_interoperability() {
        common::init_tracing();
        
        // Create and populate different collection types
        let mut hash_set = HashSet::new();
        let mut queue = Queue::new();
        let mut stack = Stack::new();
        
        // Test data
        let test_data = vec![1, 2, 3, 4, 5];
        
        // Populate collections
        for &item in &test_data {
            assert!(hash_set.insert(item).unwrap());
            assert!(queue.enqueue(item).is_ok());
            assert!(stack.push(item).is_ok());
        }
        
        // Verify basic properties
        assert_eq!(hash_set.len(), 5);
        assert_eq!(queue.len(), 5);
        assert_eq!(stack.len(), 5);
        
        // Test conversion between collections
        let queue_to_vec: Vec<i32> = queue.iter().cloned().collect();
        assert_eq!(queue_to_vec.len(), 5);
        
        // Test set operations with other collections
        for item in queue_to_vec {
            assert!(hash_set.contains(&item));
        }
    }

    #[test]
    fn test_cross_collection_operations() {
        common::init_tracing();
        
        let mut set1 = HashSet::new();
        let mut set2 = HashSet::new();
        let mut queue = Queue::new();
        
        // Populate first set
        for i in 1..=10 {
            set1.insert(i).unwrap();
        }
        
        // Populate second set with overlap
        for i in 5..=15 {
            set2.insert(i).unwrap();
        }
        
        // Union operation
        let union_set = set1.union(&set2);
        assert_eq!(union_set.len(), 15); // 1-15
        
        // Intersection operation
        let intersection_set = set1.intersection(&set2);
        assert_eq!(intersection_set.len(), 6); // 5-10
        
        // Convert intersection to queue for processing
        for item in intersection_set.iter() {
            queue.enqueue(*item).unwrap();
        }
        
        assert_eq!(queue.len(), 6);
        
        // Process queue items through stack
        let mut stack = Stack::new();
        while !queue.is_empty() {
            if let Some(item) = queue.dequeue() {
                stack.push(item).unwrap();
            }
        }
        
        assert_eq!(stack.len(), 6);
        assert!(queue.is_empty());
    }

    #[test]
    fn test_iterator_chaining_across_collections() {
        common::init_tracing();
        
        let mut set = HashSet::new();
        let mut queue = Queue::new();
        let mut stack = Stack::new();
        
        // Populate collections with different ranges
        for i in 1..=5 {
            set.insert(i).unwrap();
        }
        
        for i in 6..=10 {
            queue.enqueue(i).unwrap();
        }
        
        for i in 11..=15 {
            stack.push(i).unwrap();
        }
        
        // Chain iterators from different collections
        let set_items: Vec<i32> = set.iter().copied().collect();
        let queue_items: Vec<i32> = queue.iter().cloned().collect();
        let stack_items: Vec<i32> = stack.iter().cloned().collect();
        
        // Combine all items
        let mut all_items = Vec::new();
        all_items.extend(set_items);
        all_items.extend(queue_items);
        all_items.extend(stack_items);
        
        assert_eq!(all_items.len(), 15);
        
        // Test functional operations across combined data
        let filtered: Vec<i32> = all_items.iter()
            .filter(|&&x| x % 2 == 0)
            .copied()
            .collect();
        
        assert_eq!(filtered.len(), 7); // Even numbers: 2,4,6,8,10,12,14
        
        let sum: i32 = all_items.iter().sum();
        assert_eq!(sum, 120); // Sum of 1-15
    }

    #[test]
    fn test_priority_queue_with_sets() {
        common::init_tracing();
        
        let mut pq = PriorityQueue::new();
        let mut processed_set = HashSet::new();
        
        // Add tasks with priorities
        let tasks = vec![
            (5, "High priority task"),
            (1, "Low priority task"),
            (3, "Medium priority task"),
            (5, "Another high priority task"),
            (2, "Another low priority task"),
        ];
        
        for (priority, task) in tasks.iter() {
            pq.enqueue(*priority, *priority).unwrap();
        }
        
        assert_eq!(pq.len(), 5);
        
        // Process tasks in priority order
        let mut processing_order = Vec::new();
        while !pq.is_empty() {
            if let Ok(Some(item)) = pq.dequeue() {
                processing_order.push((item, item));
                processed_set.insert(item).unwrap();
            }
        }
        
        // Verify high priority tasks come first
        assert!(processing_order[0].0 >= processing_order[1].0);
        assert_eq!(processed_set.len(), 5);
        
        // Verify all priorities were processed
        for (priority, _) in tasks.iter() {
            assert!(processed_set.contains(priority));
        }
    }

    #[test]
    fn test_circular_queue_with_stack_buffering() {
        common::init_tracing();
        
        let mut circular_queue = CircularQueue::new(5).unwrap();
        let mut overflow_stack = Stack::new();
        
        // Fill circular queue and handle overflow
        for i in 1..=10 {
            if circular_queue.enqueue(i).is_err() {
                // Queue is full).unwrap();
            }
        }
        
        assert_eq!(circular_queue.len(), 5);
        assert_eq!(overflow_stack.len(), 5);
        
        // Process from queue first, then from overflow stack
        let mut processed = Vec::new();
        
        // Empty circular queue
        while !circular_queue.is_empty() {
            if let Some(item) = circular_queue.dequeue() {
                processed.push(item);
            }
        }
        
        // Process overflow stack (LIFO order)
        while !overflow_stack.is_empty() {
            if let Some(item) = overflow_stack.pop() {
                processed.push(item);
            }
        }
        
        assert_eq!(processed.len(), 10);
        // First 5 should be in FIFO order from queue
        assert_eq!(&processed[0..5], &[1, 2, 3, 4, 5]);
        // Last 5 should be in LIFO order from stack
        assert_eq!(&processed[5..10], &[10, 9, 8, 7, 6]);
    }

    #[test]
    fn test_bit_set_operations_with_regular_sets() -> Result<(), Box<dyn std::error::Error>> {
        common::init_tracing();
        
        let mut bit_set = BitSet::new(100);
        let mut hash_set = HashSet::new();
        
        // Set some bits
        let indices = vec![1, 5, 10, 15, 25, 50, 75, 99];
        for &index in &indices {
            bit_set.set(index).unwrap();
            hash_set.insert(index).unwrap();
        }
        
        assert_eq!(bit_set.count(), indices.len());
        assert_eq!(hash_set.len(), indices.len());
        
        // Test membership consistency
        for i in 0..100 {
            let bit_contains = bit_set.get(i).unwrap_or(false);
            let hash_contains = hash_set.contains(&i);
            assert_eq!(bit_contains, hash_contains);
        }
        
        // Create another bit set for operations
        let mut bit_set2 = BitSet::new(100);
        let overlap_indices = vec![5, 15, 25, 35, 45];
        for &index in &overlap_indices {
            bit_set2.set(index).unwrap();
        }
        
        // Union operation
        let union_bits = bit_set.union(&bit_set2).unwrap();
        let expected_union_count = indices.len() + overlap_indices.len() - 3; // 3 overlapping
        assert_eq!(union_bits.count(), expected_union_count);
        
        // Intersection operation
        let intersection_bits = bit_set.intersection(&bit_set2).unwrap();
        assert_eq!(intersection_bits.count(), 3); // 5, 15, 25
        
        Ok(())
    }

    #[test]
    fn test_performance_comparison_mixed_operations() {
        common::init_tracing();
        
        const OPERATIONS: usize = 1000;
        let mut performance_results = HashMap::new();
        
        // Test HashSet performance
        let start = Instant::now();
        let mut hash_set = HashSet::new();
        for i in 0..OPERATIONS {
            hash_set.insert(i).unwrap();
        }
        for i in 0..OPERATIONS {
            hash_set.contains(&i);
        }
        let hash_set_time = start.elapsed();
        performance_results.insert("HashSet", hash_set_time);
        
        // Test TreeSet performance
        let start = Instant::now();
        let mut tree_set = TreeSet::new();
        for i in 0..OPERATIONS {
            tree_set.insert(i).unwrap();
        }
        for i in 0..OPERATIONS {
            tree_set.contains(&i);
        }
        let tree_set_time = start.elapsed();
        performance_results.insert("TreeSet", tree_set_time);
        
        // Test Queue performance
        let start = Instant::now();
        let mut queue = Queue::new();
        for i in 0..OPERATIONS {
            queue.enqueue(i).unwrap();
        }
        for _ in 0..OPERATIONS {
            queue.dequeue().unwrap();
        }
        let queue_time = start.elapsed();
        performance_results.insert("Queue", queue_time);
        
        // Test Stack performance
        let start = Instant::now();
        let mut stack = Stack::new();
        for i in 0..OPERATIONS {
            stack.push(i).unwrap();
        }
        for _ in 0..OPERATIONS {
            stack.pop().unwrap();
        }
        let stack_time = start.elapsed();
        performance_results.insert("Stack", stack_time);
        
        // Verify all operations completed
        assert_eq!(hash_set.len(), OPERATIONS);
        assert_eq!(tree_set.len(), OPERATIONS);
        assert!(queue.is_empty());
        assert!(stack.is_empty());
        
        // Log performance results
        for (collection, time) in performance_results {
            println!("{}: {:?}", collection, time);
        }
    }

    #[test]
    fn test_memory_efficiency_multiple_collections() -> Result<(), Box<dyn std::error::Error>> {
        common::init_tracing();
        
        // Create multiple collections with shared data
        let shared_data: Vec<i32> = (1..=1000).collect();
        
        let mut hash_set = HashSet::new();
        let mut tree_set = TreeSet::new();
        let mut queue = Queue::new();
        let mut stack = Stack::new();
        let mut bit_set = BitSet::new(1000);
        
        // Populate all collections
        for &item in &shared_data {
            hash_set.insert(item).unwrap();
            tree_set.insert(item).unwrap();
            queue.enqueue(item).unwrap();
            stack.push(item).unwrap();
            
            if item < 1000 {
                bit_set.set(item as usize).unwrap();
            }
        }
        
        // Verify all collections contain the data
        assert_eq!(hash_set.len(), 1000);
        assert_eq!(tree_set.len(), 1000);
        assert_eq!(queue.len(), 1000);
        assert_eq!(stack.len(), 1000);
        assert_eq!(bit_set.count(), 999); // 1-999, since BitSet is 0-indexed
        
        // Test memory usage patterns through operations
        let mut processed_count = 0;
        
        // Process a portion of items from each collection
        for i in 1..=100 {
            if hash_set.contains(&i) &&
               tree_set.contains(&i) &&
               bit_set.get(i as usize).unwrap_or(false) {
                processed_count += 1;
            }
        }
        
        assert_eq!(processed_count, 100);
        
        Ok(())
    }

    #[test]
    fn test_real_world_data_processing_pipeline() {
        common::init_tracing();
        
        // Simulate a real-world data processing pipeline
        // Input: Stream of user events
        // Process: Filter, deduplicate, prioritize, batch process
        
        #[derive(Clone, Debug, PartialEq, Eq, Hash)]
        struct UserEvent {
            user_id: i32,
            event_type: String,
            priority: i32,
            timestamp: i64,
        }
        
        let events = vec![
            UserEvent { user_id: 1, event_type: "login".to_string(), priority: 2, timestamp: 1000 },
            UserEvent { user_id: 2, event_type: "purchase".to_string(), priority: 5, timestamp: 1001 },
            UserEvent { user_id: 1, event_type: "login".to_string(), priority: 2, timestamp: 1002 }, // Duplicate
            UserEvent { user_id: 3, event_type: "logout".to_string(), priority: 1, timestamp: 1003 },
            UserEvent { user_id: 2, event_type: "view".to_string(), priority: 3, timestamp: 1004 },
            UserEvent { user_id: 4, event_type: "purchase".to_string(), priority: 5, timestamp: 1005 },
        ];
        
        // Stage 1: Deduplicate using HashSet
        let mut unique_events = HashSet::new();
        for event in events {
            unique_events.insert(event).unwrap();
        }
        
        assert_eq!(unique_events.len(), 5); // One duplicate removed
        
        // Stage 2: Prioritize using PriorityQueue
        let mut priority_queue = PriorityQueue::new();
        for event in unique_events.iter() {
            priority_queue.enqueue(event.priority, event.priority).unwrap();
        }
        
        // Stage 3: Batch process high priority events first
        let mut high_priority_stack = Stack::new();
        let mut low_priority_queue = Queue::new();
        
        while !priority_queue.is_empty() {
            if let Ok(Some(item)) = priority_queue.dequeue() {
                if item >= 4 {
                    high_priority_stack.push(item).unwrap();
                } else {
                    low_priority_queue.enqueue(item).unwrap();
                }
            }
        }
        
        // Stage 4: Process batches
        let mut processed_events = Vec::new();
        
        // Process high priority events (LIFO for recency)
        while !high_priority_stack.is_empty() {
            if let Some(event) = high_priority_stack.pop() {
                processed_events.push(("high_priority", event));
            }
        }
        
        // Process low priority events (FIFO for fairness)
        while !low_priority_queue.is_empty() {
            if let Some(event) = low_priority_queue.dequeue() {
                processed_events.push(("low_priority", event));
            }
        }
        
        // Verify processing order and completeness
        assert_eq!(processed_events.len(), 5);
        
        // First processed should be high priority
        assert_eq!(processed_events[0].0, "high_priority");
        assert_eq!(processed_events[1].0, "high_priority");
        
        // Remaining should be low priority
        for i in 2..5 {
            assert_eq!(processed_events[i].0, "low_priority");
        }
    }

    #[test]
    fn test_thread_safe_stack_with_concurrent_collections() {
        common::init_tracing();
        
        let mut thread_safe_stack = ThreadSafeStack::new();
        let mut results_set = HashSet::new();
        
        // Add items to thread-safe stack
        for i in 1..=20 {
            thread_safe_stack.push(i).unwrap();
        }
        
        assert_eq!(thread_safe_stack.len().unwrap_or(0), 20);
        
        // Simulate concurrent access (single-threaded test)
        while !thread_safe_stack.is_empty().unwrap_or(false) {
            if let Ok(Some(item)) = thread_safe_stack.pop() {
                results_set.insert(item).unwrap();
            }
        }
        
        // Verify all items were processed
        assert_eq!(results_set.len(), 20);
        for i in 1..=20 {
            assert!(results_set.contains(&i));
        }
    }

    #[test]
    fn test_fixed_stack_overflow_handling() {
        common::init_tracing();
        
        let mut fixed_stack = FixedStack::new(5).unwrap();
        let mut overflow_queue = Queue::new();
        
        // Fill fixed stack and handle overflow
        for i in 1..=10 {
            if fixed_stack.push(i).is_err() {
                overflow_queue.enqueue(i).unwrap();
            }
        }
        
        assert_eq!(fixed_stack.len(), 5);
        assert_eq!(overflow_queue.len(), 5);
        
        // Verify capacity management
        assert!(fixed_stack.is_full());
        assert!(!fixed_stack.is_empty());
        
        // Process all items
        let mut all_items = Vec::new();
        
        while !fixed_stack.is_empty() {
            if let Some(item) = fixed_stack.pop() {
                all_items.push(item);
            }
        }
        
        while !overflow_queue.is_empty() {
            if let Some(item) = overflow_queue.dequeue() {
                all_items.push(item);
            }
        }
        
        assert_eq!(all_items.len(), 10);
    }

    #[test]
    fn test_deque_bidirectional_operations() {
        common::init_tracing();
        
        let mut deque = Deque::new();
        let mut comparison_queue = Queue::new();
        let mut comparison_stack = Stack::new();
        
        // Add items to front and back
        for i in 1..=5 {
            deque.push_back(i).unwrap();
            comparison_queue.enqueue(i).unwrap();
        }
        
        for i in 6..=10 {
            deque.push_front(i).unwrap();
            comparison_stack.push(i).unwrap();
        }
        
        assert_eq!(deque.len(), 10);
        
        // Test deque behaves like queue from back
        let mut back_items = Vec::new();
        for _ in 0..5 {
            if let Some(item) = deque.pop_front() {
                back_items.push(item);
            }
        }
        
        // Test deque behaves like stack from front
        let mut front_items = Vec::new();
        for _ in 0..5 {
            if let Some(item) = deque.pop_back() {
                front_items.push(item);
            }
        }
        
        // Verify bidirectional operations
        assert_eq!(back_items.len(), 5);
        assert_eq!(front_items.len(), 5);
        assert!(deque.is_empty());
    }

    #[test]
    fn test_comprehensive_error_handling() -> Result<(), Box<dyn std::error::Error>> {
        common::init_tracing();
        
        // Test error handling across different collection types
        let mut errors_encountered = Vec::new();
        
        // Test BitSet errors
        let bit_set = BitSet::new(10);
        if let Err(e) = bit_set.set(15) {
            errors_encountered.push(format!("BitSet: {}", e));
        }
        
        // Test FixedStack errors
        let mut fixed_stack = FixedStack::new(2).unwrap();
        fixed_stack.push(1).unwrap();
        fixed_stack.push(2).unwrap();
        if let Err(e) = fixed_stack.push(3) {
            errors_encountered.push(format!("FixedStack: {}", e));
        }
        
        // Test CircularQueue errors
        let mut circular_queue = CircularQueue::new(2).unwrap();
        circular_queue.enqueue(1).unwrap();
        circular_queue.enqueue(2).unwrap();
        if let Err(e) = circular_queue.enqueue(3) {
            errors_encountered.push(format!("CircularQueue: {}", e));
        }
        
        // Test empty collection errors
        let mut empty_queue = Queue::new();
        if empty_queue.dequeue().is_none() {
            errors_encountered.push("EmptyQueue error".to_string());
        }
        
        // Verify appropriate errors were caught
        assert!(!errors_encountered.is_empty());
        println!("Errors handled: {:?}", errors_encountered);
        
        Ok(())
    }
}

/// Performance benchmarking for collections integration
#[cfg(test)]
mod performance_benchmarks {
    use super::*;

    #[test]
    #[ignore] // Marked as ignored for normal test runs
    fn benchmark_collection_conversions() {
        common::init_tracing();
        
        const SIZE: usize = 10000;
        let data: Vec<i32> = (0..SIZE as i32).collect();
        
        // Benchmark HashSet creation and iteration
        let start = Instant::now();
        let mut hash_set = HashSet::new();
        for &item in &data {
            hash_set.insert(item).unwrap();
        }
        let set_items: Vec<i32> = hash_set.iter().copied().collect();
        let hash_set_time = start.elapsed();
        
        assert_eq!(set_items.len(), SIZE);
        
        // Benchmark Queue operations
        let start = Instant::now();
        let mut queue = Queue::new();
        for &item in &data {
            queue.enqueue(item).unwrap();
        }
        let mut queue_items = Vec::new();
        while !queue.is_empty() {
            queue_items.push(queue.dequeue().unwrap());
        }
        let queue_time = start.elapsed();
        
        assert_eq!(queue_items.len(), SIZE);
        
        println!("HashSet ({} items): {:?}", SIZE, hash_set_time);
        println!("Queue ({} items): {:?}", SIZE, queue_time);
        
        // Performance should be reasonable
        assert!(hash_set_time < Duration::from_secs(1));
        assert!(queue_time < Duration::from_secs(1));
    }

    #[test]
    #[ignore] // Marked as ignored for normal test runs
    fn benchmark_mixed_operations() {
        common::init_tracing();
        
        const OPERATIONS: usize = 5000;
        
        let start = Instant::now();
        
        let mut hash_set = HashSet::new();
        let mut priority_queue = PriorityQueue::new();
        let mut stack = Stack::new();
        
        // Mixed operations
        for i in 0..OPERATIONS {
            // Insert into set
            hash_set.insert(i).unwrap();
            
            // Add to priority queue
            priority_queue.push(i % 10).unwrap();
            
            // Push to stack
            stack.push(i).unwrap();
            
            // Periodic processing
            if i % 100 == 0 {
                // Process some priority queue items
                for _ in 0..10 {
                    if !priority_queue.is_empty() {
                        priority_queue.dequeue().unwrap();
                    }
                }
                
                // Process some stack items
                for _ in 0..10 {
                    if !stack.is_empty() {
                        stack.pop().unwrap();
                    }
                }
            }
        }
        
        let total_time = start.elapsed();
        println!("Mixed operations ({} ops): {:?}", OPERATIONS, total_time);
        
        // Verify final state
        assert_eq!(hash_set.len(), OPERATIONS);
        assert!(total_time < Duration::from_secs(5));
    }
}
