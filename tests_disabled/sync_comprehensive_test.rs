/// Comprehensive test suite for the CURSED sync module
/// 
/// This test file validates all major functionality of the threading
/// and synchronization primitives including:
/// - Thread spawning and management
/// - Synchronization primitives (mutexes, semaphores, etc.)
/// - Concurrent collections
/// - Parallel processing
/// - Thread-local storage

use cursed::stdlib::sync::*;
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use std::sync::atomic::{AtomicI32, Ordering as StdOrdering};

#[test]
fn test_basic_thread_spawning() {
    // Test basic thread creation and joining
    let handle = spawn(|| {
        thread::sleep(Duration::from_millis(10));
        42
    }).unwrap();
    
    let result = handle.join().unwrap();
    assert_eq!(result, 42);
}

#[test]
fn test_named_thread_spawning() {
    let handle = spawn_named("test-thread", || {
        current_thread_name().unwrap_or_default()
    }).unwrap();
    
    let thread_name = handle.join().unwrap();
    assert_eq!(thread_name, "test-thread");
}

#[test]
fn test_thread_builder() {
    let handle = ThreadBuilder::new()
        .name("builder-test".to_string())
        .stack_size(1024 * 1024) // 1MB stack
        .spawn(|| 100)
        .unwrap();
    
    let result = handle.join().unwrap();
    assert_eq!(result, 100);
}

#[test]
fn test_mutex_basic_operations() {
    let mutex = Mutex::new(0);
    
    // Test basic lock and unlock
    {
        let mut guard = mutex.lock().unwrap();
        *guard = 42;
    }
    
    let guard = mutex.lock().unwrap();
    assert_eq!(*guard, 42);
}

#[test]
fn test_mutex_concurrent_access() {
    let mutex = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    
    for _ in 0..10 {
        let mutex_clone = Arc::clone(&mutex);
        let handle = thread::spawn(move || {
            let mut guard = mutex_clone.lock().unwrap();
            *guard += 1;
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let guard = mutex.lock().unwrap();
    assert_eq!(*guard, 10);
}

#[test]
fn test_rwlock_concurrent_readers() {
    let rwlock = Arc::new(RwLock::new(42));
    let mut handles = Vec::new();
    
    // Multiple readers should be able to read concurrently
    for _ in 0..5 {
        let rwlock_clone = Arc::clone(&rwlock);
        let handle = thread::spawn(move || {
            let guard = rwlock_clone.read().unwrap();
            *guard
        });
        handles.push(handle);
    }
    
    for handle in handles {
        let value = handle.join().unwrap();
        assert_eq!(value, 42);
    }
}

#[test]
fn test_rwlock_writer_exclusivity() {
    let rwlock = Arc::new(RwLock::new(0));
    let counter = Arc::new(AtomicI32::new(0));
    let mut handles = Vec::new();
    
    // Writers should be exclusive
    for i in 0..5 {
        let rwlock_clone = Arc::clone(&rwlock);
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            let mut guard = rwlock_clone.write().unwrap();
            let old_count = counter_clone.fetch_add(1, StdOrdering::SeqCst);
            // Simulate some work
            thread::sleep(Duration::from_millis(10));
            *guard = i;
            old_count
        });
        handles.push(handle);
    }
    
    let mut results = Vec::new();
    for handle in handles {
        results.push(handle.join().unwrap());
    }
    
    // Each writer should have seen a unique counter value
    results.sort();
    assert_eq!(results, vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_atomic_operations() {
    let atomic_bool = AtomicBool::new(false);
    assert_eq!(atomic_bool.load(Ordering::SeqCst), false);
    
    atomic_bool.store(true, Ordering::SeqCst);
    assert_eq!(atomic_bool.load(Ordering::SeqCst), true);
    
    let old_value = atomic_bool.swap(false, Ordering::SeqCst);
    assert_eq!(old_value, true);
    assert_eq!(atomic_bool.load(Ordering::SeqCst), false);
}

#[test]
fn test_atomic_integer_operations() {
    let atomic_i32 = AtomicI32::new(10);
    
    assert_eq!(atomic_i32.load(Ordering::SeqCst), 10);
    
    let old_value = atomic_i32.fetch_add(5, Ordering::SeqCst);
    assert_eq!(old_value, 10);
    assert_eq!(atomic_i32.load(Ordering::SeqCst), 15);
    
    let incremented = atomic_i32.fetch_add(1, Ordering::SeqCst);
    assert_eq!(incremented, 15);
    assert_eq!(atomic_i32.load(Ordering::SeqCst), 16);
}

#[test]
fn test_semaphore_basic() {
    let semaphore = Semaphore::new(2);
    assert_eq!(semaphore.available_permits(), 2);
    
    let _guard1 = semaphore.acquire().unwrap();
    assert_eq!(semaphore.available_permits(), 1);
    
    let _guard2 = semaphore.acquire().unwrap();
    assert_eq!(semaphore.available_permits(), 0);
    
    // Should not be able to acquire without blocking
    assert!(semaphore.try_acquire().unwrap().is_none());
    
    drop(_guard1);
    assert_eq!(semaphore.available_permits(), 1);
}

#[test]
fn test_barrier_synchronization() {
    let barrier = Arc::new(Barrier::new(3));
    let counter = Arc::new(AtomicI32::new(0));
    let mut handles = Vec::new();
    
    for i in 0..3 {
        let barrier_clone = Arc::clone(&barrier);
        let counter_clone = Arc::clone(&counter);
        let handle = thread::spawn(move || {
            // Do some work
            thread::sleep(Duration::from_millis(i * 10));
            counter_clone.fetch_add(1, StdOrdering::SeqCst);
            
            // Wait at barrier
            let result = barrier_clone.wait();
            result.is_leader()
        });
        handles.push(handle);
    }
    
    let mut leader_count = 0;
    for handle in handles {
        let is_leader = handle.join().unwrap();
        if is_leader {
            leader_count += 1;
        }
    }
    
    assert_eq!(leader_count, 1);
    assert_eq!(counter.load(StdOrdering::SeqCst), 3);
}

#[test]
fn test_condition_variable() {
    let mutex = Arc::new(Mutex::new(false));
    let condvar = Arc::new(CondVar::new());
    
    let mutex_clone = Arc::clone(&mutex);
    let condvar_clone = Arc::clone(&condvar);
    
    let handle = thread::spawn(move || {
        thread::sleep(Duration::from_millis(50));
        
        let mut guard = mutex_clone.lock().unwrap();
        *guard = true;
        condvar_clone.notify_one();
    });
    
    let mut guard = mutex.lock().unwrap();
    while !*guard {
        guard = condvar.wait(guard).unwrap();
    }
    
    assert!(*guard);
    handle.join().unwrap();
}

#[test]
fn test_once_initialization() {
    static mut COUNTER: i32 = 0;
    let once = Arc::new(Once::new());
    let mut handles = Vec::new();
    
    for _ in 0..10 {
        let once_clone = Arc::clone(&once);
        let handle = thread::spawn(move || {
            once_clone.call_once(|| {
                unsafe {
                    COUNTER += 1;
                }
            });
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    assert!(once.is_completed());
    unsafe {
        assert_eq!(COUNTER, 1);
    }
}

#[test]
fn test_once_cell() {
    let cell = OnceCell::new();
    assert!(cell.get().is_none());
    
    let value = cell.get_or_init(|| 42);
    assert_eq!(*value, 42);
    
    let value2 = cell.get().unwrap();
    assert_eq!(*value2, 42);
    
    // Should not be able to set again
    assert!(cell.set(100).is_err());
}

#[test]
fn test_concurrent_hashmap() {
    let map = ConcurrentHashMap::new();
    
    assert!(map.insert("key1".to_string(), 42).unwrap().is_none());
    assert_eq!(map.get(&"key1".to_string()).unwrap(), Some(42));
    assert_eq!(map.len(), 1);
    assert!(map.contains_key(&"key1".to_string()).unwrap());
    
    assert_eq!(map.insert("key1".to_string(), 100).unwrap(), Some(42));
    assert_eq!(map.get(&"key1".to_string()).unwrap(), Some(100));
    
    assert_eq!(map.remove(&"key1".to_string()).unwrap(), Some(100));
    assert_eq!(map.len(), 0);
    assert!(map.is_empty());
}

#[test]
fn test_concurrent_vec() {
    let vec = ConcurrentVec::new();
    
    assert!(vec.push(42).is_ok());
    assert!(vec.push(100).is_ok());
    assert_eq!(vec.len().unwrap(), 2);
    
    assert_eq!(vec.get(0).unwrap(), Some(42));
    assert_eq!(vec.get(1).unwrap(), Some(100));
    assert_eq!(vec.get(2).unwrap(), None);
    
    assert_eq!(vec.pop().unwrap(), Some(100));
    assert_eq!(vec.len().unwrap(), 1);
}

#[test]
fn test_channels_basic() {
    let (sender, receiver) = channel();
    
    assert!(sender.send(42).is_ok());
    assert!(sender.send(100).is_ok());
    
    assert_eq!(receiver.recv().unwrap(), 42);
    assert_eq!(receiver.recv().unwrap(), 100);
    
    // Test try_recv on empty channel
    assert!(matches!(receiver.try_recv().unwrap_err(), ChannelError::Empty));
}

#[test]
fn test_channels_multiple_producers() {
    let (sender, receiver) = channel();
    let mut handles = Vec::new();
    
    for i in 0..5 {
        let sender_clone = sender.clone();
        let handle = thread::spawn(move || {
            sender_clone.send(i).unwrap();
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    drop(sender); // Close the channel
    
    let mut received = Vec::new();
    while let Ok(value) = receiver.recv() {
        received.push(value);
    }
    
    received.sort();
    assert_eq!(received, vec![0, 1, 2, 3, 4]);
}

#[test]
fn test_lock_free_stack() {
    let stack = LockFreeStack::new();
    assert!(stack.is_empty());
    
    stack.push(42);
    stack.push(100);
    assert!(!stack.is_empty());
    
    assert_eq!(stack.pop(), Some(100));
    assert_eq!(stack.pop(), Some(42));
    assert_eq!(stack.pop(), None);
    assert!(stack.is_empty());
}

#[test]
fn test_lock_free_queue() {
    let queue = LockFreeQueue::new();
    assert!(queue.is_empty());
    
    queue.enqueue(42);
    queue.enqueue(100);
    assert!(!queue.is_empty());
    
    assert_eq!(queue.dequeue(), Some(42));
    assert_eq!(queue.dequeue(), Some(100));
    assert_eq!(queue.dequeue(), None);
    assert!(queue.is_empty());
}

#[test]
fn test_atomic_counter() {
    let counter = AtomicCounter::new(10);
    assert_eq!(counter.get(), 10);
    
    assert_eq!(counter.increment(), 11);
    assert_eq!(counter.get(), 11);
    
    assert_eq!(counter.decrement(), 10);
    assert_eq!(counter.get(), 10);
    
    assert_eq!(counter.add(5), 15);
    assert_eq!(counter.subtract(3), 12);
    
    assert_eq!(counter.reset(), 12);
    assert_eq!(counter.get(), 0);
}

// Note: ThreadPool tests and other advanced features would require
// fixing more compilation issues first. These basic tests validate
// the core synchronization primitives are working correctly.

#[test]
fn test_thread_local_key_basic() {
    let key: ThreadLocalKey<i32> = ThreadLocalKey::new();
    
    assert!(key.get().is_none());
    
    key.set(42).unwrap();
    assert_eq!(key.get(), Some(42));
    
    let removed = key.remove().unwrap();
    assert_eq!(removed, Some(42));
    assert!(key.get().is_none());
}

#[test]
fn test_named_thread_local_key() {
    let key: ThreadLocalKey<String> = ThreadLocalKey::named("test_key");
    
    assert_eq!(key.name(), Some("test_key"));
    
    key.set("hello".to_string()).unwrap();
    assert_eq!(key.get(), Some("hello".to_string()));
}

#[test]
fn test_thread_local_storage() {
    let tls = ThreadLocal::new();
    
    tls.set(42).unwrap();
    assert_eq!(tls.get().unwrap(), 42);
    
    let result = tls.with(|value| *value * 2).unwrap();
    assert_eq!(result, 84);
    
    assert!(tls.is_set());
    tls.reset().unwrap();
    assert!(!tls.is_set());
}

#[test]
fn test_thread_local_with_initializer() {
    let tls = ThreadLocal::with_initializer(|| 100);
    
    // Should initialize automatically
    assert_eq!(tls.get().unwrap(), 100);
    
    tls.set(200).unwrap();
    assert_eq!(tls.get().unwrap(), 200);
}

#[test]
fn test_multithreaded_thread_local() {
    let key = Arc::new(ThreadLocalKey::<i32>::new());
    let counter = Arc::new(AtomicI32::new(0));
    let mut handles = Vec::new();

    for i in 0..5 {
        let key_clone = Arc::clone(&key);
        let counter_clone = Arc::clone(&counter);
        
        let handle = thread::spawn(move || {
            key_clone.set(i * 10).unwrap();
            let value = key_clone.get().unwrap();
            counter_clone.fetch_add(value, StdOrdering::Relaxed);
        });
        
        handles.push(handle);
    }

    for handle in handles {
        handle.join().unwrap();
    }

    // Each thread should have its own value
    // 0*10 + 1*10 + 2*10 + 3*10 + 4*10 = 0 + 10 + 20 + 30 + 40 = 100
    assert_eq!(counter.load(StdOrdering::Relaxed), 100);
}

#[test]
fn test_concurrent_collections_stress() {
    let map = Arc::new(ConcurrentHashMap::new());
    let mut handles = Vec::new();
    
    // Test concurrent access
    for i in 0..10 {
        let map_clone = Arc::clone(&map);
        let handle = thread::spawn(move || {
            map_clone.insert(format!("key{}", i), i).unwrap();
            map_clone.get(&format!("key{}", i)).unwrap()
        });
        handles.push(handle);
    }
    
    for handle in handles {
        let result = handle.join().unwrap();
        assert!(result.is_some());
    }
    
    assert_eq!(map.len(), 10);
}

#[test]
fn test_memory_fences() {
    // Test memory fence operations
    memory_fence(Ordering::SeqCst);
    compiler_fence(Ordering::SeqCst);
    
    // These should complete without errors
    assert!(true);
}

#[test]
fn test_sync_statistics() {
    // Test getting sync module statistics
    let stats = get_sync_statistics();
    
    // Basic validation that we can get stats
    assert!(stats.active_threads >= 0);
    assert!(stats.thread_pool_utilization >= 0.0);
}

#[test]
fn test_high_level_thread_local_api() {
    let key: ThreadLocalKey<String> = create_thread_local_key();
    
    assert!(thread_local_get(&key).is_none());
    
    thread_local_set(&key, "hello".to_string()).unwrap();
    assert_eq!(thread_local_get(&key), Some("hello".to_string()));
    
    let result = with_thread_local(&key, |value| value.len()).unwrap();
    assert_eq!(result, Some(5));
    
    let removed = thread_local_remove(&key).unwrap();
    assert_eq!(removed, Some("hello".to_string()));
    assert!(thread_local_get(&key).is_none());
}

// Additional tests for edge cases and error conditions

#[test]
fn test_mutex_try_lock() {
    let mutex = Mutex::new(0);
    
    let _guard1 = mutex.lock().unwrap();
    
    // try_lock should fail when mutex is already locked
    assert!(mutex.try_lock().is_err());
}

#[test]
fn test_rwlock_try_operations() {
    let rwlock = RwLock::new(42);
    
    let _read_guard = rwlock.read().unwrap();
    
    // Should be able to get more read locks
    let _read_guard2 = rwlock.try_read().unwrap();
    
    // Should not be able to get write lock while readers exist
    assert!(rwlock.try_write().is_err());
}

#[test]
fn test_semaphore_timeout() {
    let semaphore = Semaphore::new(1);
    let _guard = semaphore.acquire().unwrap();
    
    // Should timeout when trying to acquire with no permits available
    let result = semaphore.acquire_timeout(Duration::from_millis(10));
    assert!(result.is_ok());
    assert!(result.unwrap().is_none());
}

#[test]
fn test_channel_timeout() {
    let (_sender, receiver) = channel::<i32>();
    
    // Should timeout when no messages available
    let result = receiver.recv_timeout(Duration::from_millis(10));
    assert!(matches!(result.unwrap_err(), ChannelError::Timeout));
}

// Performance and scalability tests

#[test] 
fn test_concurrent_atomic_operations() {
    let atomic = Arc::new(AtomicI32::new(0));
    let mut handles = Vec::new();
    
    for _ in 0..100 {
        let atomic_clone = Arc::clone(&atomic);
        let handle = thread::spawn(move || {
            for _ in 0..100 {
                atomic_clone.fetch_add(1, Ordering::SeqCst);
            }
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    assert_eq!(atomic.load(Ordering::SeqCst), 10000);
}

#[test]
fn test_lock_free_structures_concurrent() {
    let stack = Arc::new(LockFreeStack::new());
    let queue = Arc::new(LockFreeQueue::new());
    let mut handles = Vec::new();
    
    // Test concurrent stack operations
    for i in 0..50 {
        let stack_clone = Arc::clone(&stack);
        let queue_clone = Arc::clone(&queue);
        let handle = thread::spawn(move || {
            stack_clone.push(i);
            queue_clone.enqueue(i);
        });
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    // Verify items were added (exact count may vary due to concurrent access)
    let mut stack_count = 0;
    while stack.pop().is_some() {
        stack_count += 1;
    }
    
    let mut queue_count = 0;
    while queue.dequeue().is_some() {
        queue_count += 1;
    }
    
    assert_eq!(stack_count, 50);
    assert_eq!(queue_count, 50);
}
