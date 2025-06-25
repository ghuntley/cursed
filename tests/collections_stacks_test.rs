/// Comprehensive test suite for CURSED stack collections
/// 
/// This test suite validates:
/// - Stack<T>: LIFO stack with dynamic resizing
/// - FixedStack<T>: Fixed-capacity stack
/// - ThreadSafeStack<T>: Concurrent stack
/// - StackWithMin<T>: Stack with O(1) minimum tracking
/// - All operations, edge cases, and performance characteristics

use cursed::stdlib::collections::stacks::*;
use cursed::stdlib::collections::{CollectionsError, CollectionsResult};
use std::sync::{Arc, Barrier};
use std::thread;
use std::time::Duration;

// ==================== Stack Tests ====================

#[test]
fn test_stack_creation() {
    let stack: Stack<i32> = Stack::new();
    assert!(stack.is_empty());
    assert_eq!(stack.len(), 0);
    assert_eq!(stack.peek(), None);

    let stack_with_capacity: Stack<i32> = Stack::with_capacity(100);
    assert!(stack_with_capacity.capacity() >= 100);
    assert!(stack_with_capacity.is_empty());
}

#[test]
fn test_stack_basic_operations() {
    let mut stack = Stack::new();
    
    // Test push and basic properties
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    assert_eq!(stack.len(), 3);
    assert!(!stack.is_empty());
    assert_eq!(stack.peek(), Some(&3));
    
    // Test LIFO behavior
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.pop(), Some(2));
    assert_eq!(stack.len(), 1);
    assert_eq!(stack.peek(), Some(&1));
    
    assert_eq!(stack.pop(), Some(1));
    assert!(stack.is_empty());
    assert_eq!(stack.pop(), None);
}

#[test]
fn test_stack_peek_mut() {
    let mut stack = Stack::new();
    stack.push(1);
    stack.push(2);
    
    if let Some(top) = stack.peek_mut() {
        *top = 10;
    }
    
    assert_eq!(stack.peek(), Some(&10));
    assert_eq!(stack.pop(), Some(10));
    assert_eq!(stack.pop(), Some(1));
}

#[test]
fn test_stack_clear_and_capacity() {
    let mut stack = Stack::with_capacity(50);
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    assert_eq!(stack.len(), 3);
    stack.clear();
    assert!(stack.is_empty());
    assert_eq!(stack.len(), 0);
    
    // Test reserve and shrink
    stack.reserve(100);
    assert!(stack.capacity() >= 100);
    
    stack.shrink_to_fit();
    // Capacity should be reduced but exact value depends on implementation
}

#[test]
fn test_stack_bulk_operations() {
    let mut stack = Stack::new();
    
    // Test push_many
    stack.push_many(vec![1, 2, 3, 4, 5]);
    assert_eq!(stack.len(), 5);
    assert_eq!(stack.peek(), Some(&5));
    
    // Test pop_many
    let popped = stack.pop_many(3);
    assert_eq!(popped, vec![5, 4, 3]);
    assert_eq!(stack.len(), 2);
    
    // Test peek_many
    stack.push_many(vec![6, 7, 8]);
    let peeked = stack.peek_many(3);
    assert_eq!(peeked, vec![&8, &7, &6]);
    assert_eq!(stack.len(), 5); // Should not remove elements
}

#[test]
fn test_stack_special_operations() {
    let mut stack = Stack::new();
    
    // Test dup with empty stack
    assert!(stack.dup().is_err());
    
    stack.push(5);
    assert!(stack.dup().is_ok());
    assert_eq!(stack.len(), 2);
    assert_eq!(stack.pop(), Some(5));
    assert_eq!(stack.pop(), Some(5));
    
    // Test swap with insufficient elements
    stack.push(1);
    assert!(stack.swap().is_err());
    
    stack.push(2);
    assert!(stack.swap().is_ok());
    assert_eq!(stack.pop(), Some(1));
    assert_eq!(stack.pop(), Some(2));
}

#[test]
fn test_stack_conversions() {
    let vec = vec![1, 2, 3, 4, 5];
    let stack = Stack::from_vec(vec.clone());
    
    assert_eq!(stack.len(), 5);
    assert_eq!(stack.peek(), Some(&5)); // Last element becomes top
    
    let converted_vec = stack.into_vec();
    assert_eq!(converted_vec, vec);
    
    // Test From trait
    let stack2: Stack<i32> = vec![10, 20, 30].into();
    assert_eq!(stack2.len(), 3);
    assert_eq!(stack2.peek(), Some(&30));
}

#[test]
fn test_stack_iterators() {
    let mut stack = Stack::new();
    stack.push_many(vec![1, 2, 3, 4, 5]);
    
    // Test iterator (top to bottom)
    let items: Vec<i32> = stack.iter().cloned().collect();
    assert_eq!(items, vec![5, 4, 3, 2, 1]);
    
    // Test into_iter
    let items: Vec<i32> = stack.into_iter().collect();
    assert_eq!(items, vec![5, 4, 3, 2, 1]);
}

#[test]
fn test_stack_from_iterator() {
    let stack: Stack<i32> = vec![1, 2, 3, 4, 5].into_iter().collect();
    assert_eq!(stack.len(), 5);
    assert_eq!(stack.peek(), Some(&5));
}

// ==================== FixedStack Tests ====================

#[test]
fn test_fixed_stack_creation() {
    let stack = FixedStack::<i32>::new(10).unwrap();
    assert!(stack.is_empty());
    assert_eq!(stack.len(), 0);
    assert_eq!(stack.capacity(), 10);
    assert_eq!(stack.remaining_capacity(), 10);
    
    // Test invalid capacity
    assert!(FixedStack::<i32>::new(0).is_err());
}

#[test]
fn test_fixed_stack_capacity_limits() {
    let mut stack = FixedStack::new(3).unwrap();
    
    // Fill to capacity
    assert!(stack.push(1).is_ok());
    assert!(stack.push(2).is_ok());
    assert!(stack.push(3).is_ok());
    
    assert!(stack.is_full());
    assert_eq!(stack.remaining_capacity(), 0);
    
    // Should fail when full
    assert!(stack.push(4).is_err());
    
    // Should work after pop
    assert_eq!(stack.pop(), Some(3));
    assert!(!stack.is_full());
    assert_eq!(stack.remaining_capacity(), 1);
    assert!(stack.push(4).is_ok());
}

#[test]
fn test_fixed_stack_bulk_operations() {
    let mut stack = FixedStack::new(5).unwrap();
    
    // Test push_many with partial fill
    let pushed = stack.push_many(vec![1, 2, 3, 4, 5, 6, 7]).unwrap();
    assert_eq!(pushed, 5); // Only 5 could be pushed
    assert!(stack.is_full());
    
    // Test pop_many
    let popped = stack.pop_many(3);
    assert_eq!(popped, vec![5, 4, 3]);
    assert_eq!(stack.len(), 2);
    
    // Test clear
    stack.clear();
    assert!(stack.is_empty());
    assert_eq!(stack.remaining_capacity(), 5);
}

#[test]
fn test_fixed_stack_iterator() {
    let mut stack = FixedStack::new(10).unwrap();
    stack.push_many(vec![1, 2, 3, 4, 5]).unwrap();
    
    let items: Vec<i32> = stack.iter().cloned().collect();
    assert_eq!(items, vec![5, 4, 3, 2, 1]);
}

// ==================== ThreadSafeStack Tests ====================

#[test]
fn test_thread_safe_stack_basic() {
    let stack = ThreadSafeStack::new();
    
    assert!(stack.push(1).is_ok());
    assert!(stack.push(2).is_ok());
    assert!(stack.push(3).is_ok());
    
    assert_eq!(stack.len().unwrap(), 3);
    assert!(!stack.is_empty().unwrap());
    
    assert_eq!(stack.pop().unwrap(), Some(3));
    assert_eq!(stack.pop().unwrap(), Some(2));
    assert_eq!(stack.len().unwrap(), 1);
    
    assert!(stack.clear().is_ok());
    assert!(stack.is_empty().unwrap());
}

#[test]
fn test_thread_safe_stack_peek() {
    let stack = ThreadSafeStack::new();
    stack.push(42).unwrap();
    
    let result = stack.peek(|x| *x * 2).unwrap();
    assert_eq!(result, Some(84));
    
    // Stack should be unchanged
    assert_eq!(stack.len().unwrap(), 1);
}

#[test]
fn test_thread_safe_stack_bulk_operations() {
    let stack = ThreadSafeStack::new();
    
    // Test push_many
    assert!(stack.push_many(vec![1, 2, 3, 4, 5]).is_ok());
    assert_eq!(stack.len().unwrap(), 5);
    
    // Test pop_many
    let popped = stack.pop_many(3).unwrap();
    assert_eq!(popped, vec![5, 4, 3]);
    assert_eq!(stack.len().unwrap(), 2);
}

#[test]
fn test_thread_safe_stack_snapshot() {
    let stack = ThreadSafeStack::new();
    stack.push_many(vec![1, 2, 3]).unwrap();
    
    let snapshot = stack.snapshot().unwrap();
    assert_eq!(snapshot, vec![1, 2, 3]);
    
    // Original stack should be unchanged
    assert_eq!(stack.len().unwrap(), 3);
}

#[test]
fn test_thread_safe_stack_concurrent_access() {
    let stack = Arc::new(ThreadSafeStack::new());
    let barrier = Arc::new(Barrier::new(4));
    
    let mut handles = vec![];
    
    // Spawn threads that push elements
    for thread_id in 0..3 {
        let stack_clone = Arc::clone(&stack);
        let barrier_clone = Arc::clone(&barrier);
        
        let handle = thread::spawn(move || {
            barrier_clone.wait();
            for i in 0..10 {
                let value = thread_id * 100 + i;
                stack_clone.push(value).unwrap();
            }
        });
        handles.push(handle);
    }
    
    // Main thread also participates
    barrier.wait();
    for i in 0..10 {
        let value = 1000 + i;
        stack.push(value).unwrap();
    }
    
    // Wait for all threads
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify total count
    assert_eq!(stack.len().unwrap(), 40);
    
    // Pop all elements and verify we get all unique values
    let mut all_values = vec![];
    while let Some(value) = stack.pop().unwrap() {
        all_values.push(value);
    }
    
    assert_eq!(all_values.len(), 40);
    all_values.sort();
    
    let mut expected = vec![];
    for thread_id in 0..3 {
        for i in 0..10 {
            expected.push(thread_id * 100 + i);
        }
    }
    for i in 0..10 {
        expected.push(1000 + i);
    }
    expected.sort();
    
    assert_eq!(all_values, expected);
}

#[test]
fn test_thread_safe_stack_clone() {
    let stack1 = ThreadSafeStack::new();
    stack1.push_many(vec![1, 2, 3]).unwrap();
    
    let stack2 = stack1.clone();
    
    // Both stacks should have the same content
    assert_eq!(stack1.len().unwrap(), stack2.len().unwrap());
    assert_eq!(stack1.snapshot().unwrap(), stack2.snapshot().unwrap());
    
    // But should be independent
    stack1.push(4).unwrap();
    assert_ne!(stack1.len().unwrap(), stack2.len().unwrap());
}

// ==================== StackWithMin Tests ====================

#[test]
fn test_stack_with_min_basic() {
    let mut stack = StackWithMin::new();
    
    assert!(stack.is_empty());
    assert_eq!(stack.min(), None);
    
    stack.push(3);
    assert_eq!(stack.min(), Some(&3));
    assert_eq!(stack.peek(), Some(&3));
    
    stack.push(1);
    assert_eq!(stack.min(), Some(&1));
    assert_eq!(stack.peek(), Some(&1));
    
    stack.push(4);
    assert_eq!(stack.min(), Some(&1));
    assert_eq!(stack.peek(), Some(&4));
    
    stack.push(1); // Duplicate minimum
    assert_eq!(stack.min(), Some(&1));
}

#[test]
fn test_stack_with_min_pop_behavior() {
    let mut stack = StackWithMin::new();
    stack.push_many(vec![3, 1, 4, 1, 5]);
    
    assert_eq!(stack.min(), Some(&1));
    
    // Pop elements and verify minimum tracking
    assert_eq!(stack.pop(), Some(5));
    assert_eq!(stack.min(), Some(&1));
    
    assert_eq!(stack.pop(), Some(1)); // Remove one of the minimums
    assert_eq!(stack.min(), Some(&1)); // Still have another minimum
    
    assert_eq!(stack.pop(), Some(4));
    assert_eq!(stack.min(), Some(&1));
    
    assert_eq!(stack.pop(), Some(1)); // Remove last minimum
    assert_eq!(stack.min(), Some(&3)); // Should update to next minimum
    
    assert_eq!(stack.pop(), Some(3));
    assert_eq!(stack.min(), None);
}

#[test]
fn test_stack_with_min_complex_scenario() {
    let mut stack = StackWithMin::new();
    
    // Complex sequence with multiple minimums
    let sequence = vec![5, 2, 8, 2, 1, 3, 1, 9, 1];
    stack.push_many(sequence);
    
    assert_eq!(stack.min(), Some(&1));
    assert_eq!(stack.len(), 9);
    
    // Pop until we change the minimum
    while stack.min() == Some(&1) && !stack.is_empty() {
        stack.pop();
    }
    
    // Should now have minimum of 2
    assert_eq!(stack.min(), Some(&2));
    
    // Continue popping
    while stack.min() == Some(&2) && !stack.is_empty() {
        stack.pop();
    }
    
    // Should now have minimum of 5
    assert_eq!(stack.min(), Some(&5));
}

#[test]
fn test_stack_with_min_capacity_operations() {
    let mut stack = StackWithMin::with_capacity(100);
    assert!(stack.capacity() >= 100);
    
    stack.push_many(vec![10, 5, 8, 3, 9]);
    assert_eq!(stack.min(), Some(&3));
    
    stack.reserve(200);
    assert!(stack.capacity() >= 200);
    assert_eq!(stack.min(), Some(&3)); // Should be unchanged
    
    stack.clear();
    assert!(stack.is_empty());
    assert_eq!(stack.min(), None);
}

#[test]
fn test_stack_with_min_bulk_operations() {
    let mut stack = StackWithMin::new();
    
    // Test bulk operations
    stack.push_many(vec![5, 2, 8, 1, 9]);
    assert_eq!(stack.min(), Some(&1));
    
    let popped = stack.pop_many(2);
    assert_eq!(popped, vec![9, 1]);
    assert_eq!(stack.min(), Some(&2));
    
    // Test iterator
    let items: Vec<i32> = stack.iter().cloned().collect();
    assert_eq!(items, vec![8, 2, 5]);
}

#[test]
fn test_stack_with_min_from_iterator() {
    let data = vec![8, 3, 1, 7, 2, 9, 1, 4];
    let stack: StackWithMin<i32> = data.into_iter().collect();
    
    assert_eq!(stack.len(), 8);
    assert_eq!(stack.min(), Some(&1));
    assert_eq!(stack.peek(), Some(&4));
}

// ==================== Performance and Stress Tests ====================

#[test]
fn test_stack_large_dataset() {
    let mut stack = Stack::new();
    
    // Push large number of elements
    let size = 10000;
    for i in 0..size {
        stack.push(i);
    }
    
    assert_eq!(stack.len(), size);
    assert_eq!(stack.peek(), Some(&(size - 1)));
    
    // Pop all elements and verify order
    for expected in (0..size).rev() {
        assert_eq!(stack.pop(), Some(expected));
    }
    
    assert!(stack.is_empty());
}

#[test]
fn test_fixed_stack_memory_efficiency() {
    let capacity = 1000;
    let mut stack = FixedStack::new(capacity).unwrap();
    
    // Fill completely
    for i in 0..capacity {
        assert!(stack.push(i).is_ok());
    }
    
    assert!(stack.is_full());
    assert_eq!(stack.len(), capacity);
    
    // Verify LIFO order
    for expected in (0..capacity).rev() {
        assert_eq!(stack.pop(), Some(expected));
    }
    
    assert!(stack.is_empty());
}

#[test]
fn test_stack_with_min_performance() {
    let mut stack = StackWithMin::new();
    let size = 100;
    
    // Push elements in reverse order so minimum keeps changing
    for i in (0..size).rev() {
        stack.push(i);
        assert_eq!(stack.min(), Some(&i));
    }
    
    // Pop elements - the minimum should stay 0 until we pop the 0
    while !stack.is_empty() {
        let current_min_before = stack.min().copied();
        let item = stack.pop().unwrap();
        let current_min_after = stack.min().copied();
        
        if item == 0 {
            // When we pop 0, min should become None (if stack is empty) or next minimum
            assert!(current_min_before == Some(0));
            if stack.is_empty() {
                assert_eq!(current_min_after, None);
            } else {
                assert!(current_min_after > Some(0));
            }
            break;
        } else {
            // While 0 is still in the stack, it should remain the minimum
            assert_eq!(current_min_before, Some(0));
            assert_eq!(current_min_after, Some(0));
        }
    }
}

#[test]
fn test_thread_safe_stack_high_concurrency() {
    let stack = Arc::new(ThreadSafeStack::new());
    let num_threads = 8;
    let operations_per_thread = 100;
    
    let mut handles = vec![];
    
    // Spawn producer threads
    for thread_id in 0..num_threads {
        let stack_clone = Arc::clone(&stack);
        let handle = thread::spawn(move || {
            for i in 0..operations_per_thread {
                let value = thread_id * 1000 + i;
                stack_clone.push(value).unwrap();
            }
        });
        handles.push(handle);
    }
    
    // Wait for all producers
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify total count
    let expected_total = num_threads * operations_per_thread;
    assert_eq!(stack.len().unwrap(), expected_total);
    
    // Spawn consumer threads
    let mut handles = vec![];
    for _ in 0..num_threads {
        let stack_clone = Arc::clone(&stack);
        let handle = thread::spawn(move || {
            let mut consumed = 0;
            while let Some(_) = stack_clone.pop().unwrap() {
                consumed += 1;
                if consumed >= operations_per_thread {
                    break;
                }
            }
            consumed
        });
        handles.push(handle);
    }
    
    // Wait for all consumers and sum consumed items
    let mut total_consumed = 0;
    for handle in handles {
        total_consumed += handle.join().unwrap();
    }
    
    // Should have consumed most or all items
    assert!(total_consumed >= expected_total - num_threads);
}

// ==================== Error Handling Tests ====================

#[test]
fn test_collections_error_scenarios() {
    // Test FixedStack capacity error
    let result = FixedStack::<i32>::new(0);
    assert!(result.is_err());
    if let Err(CollectionsError::InvalidCapacity { capacity }) = result {
        assert_eq!(capacity, 0);
    } else {
        panic!("Expected InvalidCapacity error");
    }
    
    // Test Stack dup error
    let mut stack = Stack::<i32>::new();
    let result = stack.dup();
    assert!(result.is_err());
    if let Err(CollectionsError::IndexOutOfBounds { index, size }) = result {
        assert_eq!(index, 0);
        assert_eq!(size, 0);
    } else {
        panic!("Expected IndexOutOfBounds error");
    }
    
    // Test Stack swap error
    stack.push(1);
    let result = stack.swap();
    assert!(result.is_err());
    if let Err(CollectionsError::IndexOutOfBounds { index, size }) = result {
        assert_eq!(index, 1);
        assert_eq!(size, 1);
    } else {
        panic!("Expected IndexOutOfBounds error");
    }
}

// ==================== Display and Debug Tests ====================

#[test]
fn test_stack_display() {
    let mut stack = Stack::new();
    stack.push_many(vec![1, 2, 3]);
    
    let display = format!("{}", stack);
    assert!(display.contains("Stack["));
    assert!(display.contains("3"));
    assert!(display.contains("2"));
    assert!(display.contains("1"));
}

#[test]
fn test_fixed_stack_display() {
    let mut stack = FixedStack::new(10).unwrap();
    stack.push_many(vec![1, 2, 3]).unwrap();
    
    let display = format!("{}", stack);
    assert!(display.contains("FixedStack[3/10]"));
    assert!(display.contains("3"));
    assert!(display.contains("2"));
    assert!(display.contains("1"));
}

#[test]
fn test_stack_with_min_display() {
    let mut stack = StackWithMin::new();
    stack.push_many(vec![3, 1, 4]);
    
    let display = format!("{}", stack);
    assert!(display.contains("StackWithMin["));
    assert!(display.contains("min: 1"));
    assert!(display.contains("4"));
    assert!(display.contains("1"));
    assert!(display.contains("3"));
}

#[test]
fn test_thread_safe_stack_display() {
    let stack = ThreadSafeStack::new();
    stack.push_many(vec![1, 2, 3]).unwrap();
    
    let display = format!("{}", stack);
    assert!(display.contains("ThreadSafeStack["));
    assert!(display.contains("3"));
    assert!(display.contains("2"));
    assert!(display.contains("1"));
}
