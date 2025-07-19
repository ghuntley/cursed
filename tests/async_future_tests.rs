use cursed::runtime::r#async::future::*;
use cursed::runtime::r#async::future::util;
use std::time::{Duration, Instant};
use std::future::Future;
use std::pin::Pin;

#[tokio::test]
async fn test_memory_safety_no_unsafe_zeroed() {
    // Test that we can safely handle various types without unsafe zeroing
    let string_future = ReadyFuture::new("test string".to_string());
    let result = string_future.await;
    assert_eq!(result, "test string");

    let vec_future = ReadyFuture::new(vec![1, 2, 3, 4, 5]);
    let result = vec_future.await;
    assert_eq!(result, vec![1, 2, 3, 4, 5]);

    // Test with Option types (which are not zero-safe for Some variants)
    let option_future = ReadyFuture::new(Some(42));
    let result = option_future.await;
    assert_eq!(result, Some(42));
}

#[tokio::test]
async fn test_future_chaining_safety() {
    // Test proper future chaining without unsafe memory hacks
    let first_future = ReadyFuture::new(42);
    
    let chained = util::and_then(first_future, |x| {
        Box::pin(async move { 
            format!("Result: {}", x * 2) 
        }) as Pin<Box<dyn Future<Output = String> + Send>>
    });
    
    let result = chained.await;
    assert_eq!(result, "Result: 84");
}

#[tokio::test]
async fn test_map_future_type_safety() {
    // Test mapping with different types
    let int_future = ReadyFuture::new(42);
    let string_mapped = int_future.map(|x| format!("Number: {}", x));
    let result = string_mapped.await;
    assert_eq!(result, "Number: 42");

    // Test mapping with complex types
    let vec_future = ReadyFuture::new(vec![1, 2, 3]);
    let sum_mapped = vec_future.map(|v| v.iter().sum::<i32>());
    let result = sum_mapped.await;
    assert_eq!(result, 6);
}

#[tokio::test]
async fn test_async_result_safety() {
    // Test AsyncResult with various types
    let success: AsyncResult<String> = AsyncResult::Success("test".to_string());
    assert!(success.is_success());
    assert_eq!(success.unwrap(), "test");

    let error: AsyncResult<i32> = AsyncResult::Error("test error".to_string());
    assert!(error.is_error());

    let timeout: AsyncResult<Vec<i32>> = AsyncResult::Timeout;
    assert!(timeout.is_timeout());
}

#[tokio::test]
async fn test_complex_future_composition() {
    // Test complex composition without memory safety issues
    let base_future = ReadyFuture::new(10);
    
    let mapped = base_future.map(|x| x * 2);
    let complex_chain = util::and_then(mapped, |x| {
        Box::pin(async move {
            vec![x, x + 1, x + 2]
        }) as Pin<Box<dyn Future<Output = Vec<i32>> + Send>>
    });
    
    let result = complex_chain.await;
    assert_eq!(result, vec![20, 21, 22]);
}

#[tokio::test]
async fn test_timeout_with_various_types() {
    // Test timeout functionality with different types
    let string_future = ReadyFuture::new("quick response".to_string());
    let timeout_future = string_future.timeout(Duration::from_millis(100));
    let result = timeout_future.await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "quick response");

    // Test timeout with pending future
    let pending_future = PendingFuture::<Vec<i32>>::new();
    let timeout_future = pending_future.timeout(Duration::from_millis(10));
    let result = timeout_future.await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_join_future_with_different_futures() {
    // Test joining futures with proper memory management
    let futures: Vec<Pin<Box<dyn Future<Output = String> + Send>>> = vec![
        Box::pin(ReadyFuture::new("first".to_string())),
        Box::pin(async { "second".to_string() }),
        Box::pin(ReadyFuture::new("third".to_string())),
    ];
    
    let join_future = JoinFuture::new(futures);
    let results = join_future.await;
    assert_eq!(results, vec!["first", "second", "third"]);
}

#[tokio::test]
async fn test_select_future_safety() {
    // Test select with different completion times
    let futures: Vec<Pin<Box<dyn Future<Output = i32> + Send>>> = vec![
        Box::pin(async {
            tokio::time::sleep(Duration::from_millis(50)).await;
            1
        }),
        Box::pin(ReadyFuture::new(2)), // This should complete first
        Box::pin(async {
            tokio::time::sleep(Duration::from_millis(100)).await;
            3
        }),
    ];
    
    let select_future = SelectFuture::new(futures);
    let (result, index) = select_future.await;
    
    // The ready future should complete first
    assert_eq!(result, 2);
    assert_eq!(index, 1);
}

#[tokio::test]
async fn test_lazy_future_initialization() {
    // Test lazy initialization with proper memory management
    let counter = std::sync::Arc::new(std::sync::atomic::AtomicU32::new(0));
    let counter_clone = counter.clone();
    
    let lazy = LazyFuture::new(move || {
        counter_clone.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        Box::pin(async { 42 }) as Pin<Box<dyn Future<Output = i32> + Send>>
    });
    
    // Counter should not be incremented yet
    assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 0);
    
    let result = lazy.await;
    assert_eq!(result, 42);
    
    // Counter should be incremented now
    assert_eq!(counter.load(std::sync::atomic::Ordering::SeqCst), 1);
}

#[tokio::test]
async fn test_buffered_future_operations() {
    // Test buffered future with proper memory management
    let mut buffered = BufferedFuture::new(3);
    
    // Test with String type (not zero-safe)
    let items = vec!["first".to_string(), "second".to_string(), "third".to_string()];
    
    for item in items.iter() {
        assert!(buffered.push(item.clone()).is_ok());
    }
    
    // Buffer should be full
    assert!(buffered.push("fourth".to_string()).is_err());
    
    // Test draining
    assert_eq!(buffered.pop(), Some("first".to_string()));
    assert_eq!(buffered.pop(), Some("second".to_string()));
    assert_eq!(buffered.pop(), Some("third".to_string()));
    assert_eq!(buffered.pop(), None);
}

#[tokio::test]
async fn test_yield_future_scheduling() {
    // Test that yield future properly yields control
    let start = Instant::now();
    
    let mut counter = 0;
    for _ in 0..10 {
        util::yield_now().await;
        counter += 1;
    }
    
    assert_eq!(counter, 10);
    // Should take some time due to yielding
    assert!(start.elapsed() > Duration::from_nanos(1));
}

#[tokio::test]
async fn test_error_propagation() {
    // Test error propagation through future chains
    use cursed::error::CursedError;
    
    let error_result: Result<i32, CursedError> = Err(CursedError::runtime_error("test error"));
    let async_result = AsyncResult::from(error_result);
    
    assert!(async_result.is_error());
    
    let success_result: Result<String, CursedError> = Ok("success".to_string());
    let async_result = AsyncResult::from(success_result);
    
    assert!(async_result.is_success());
    assert_eq!(async_result.unwrap(), "success");
}

#[tokio::test]
async fn test_concurrent_safety() {
    // Test concurrent access to futures
    let futures = (0..10)
        .map(|i| {
            tokio::spawn(async move {
                let future = ReadyFuture::new(i);
                let result = future.await;
                result * 2
            })
        })
        .collect::<Vec<_>>();
    
    let mut results = Vec::new();
    for handle in futures {
        results.push(handle.await.unwrap());
    }
    
    results.sort();
    assert_eq!(results, vec![0, 2, 4, 6, 8, 10, 12, 14, 16, 18]);
}

#[tokio::test]
async fn test_memory_cleanup() {
    // Test that futures properly clean up memory
    let large_data = vec![0u8; 1024 * 1024]; // 1MB of data
    let future = ReadyFuture::new(large_data);
    let result = future.await;
    
    // Verify the data is correct
    assert_eq!(result.len(), 1024 * 1024);
    assert!(result.iter().all(|&x| x == 0));
    
    // The data should be moved, not copied
    drop(result);
}

#[tokio::test]
async fn test_chain_state_transitions() {
    // Test that AndThenFuture properly transitions through states
    let first = async { 42 };
    let chained = util::and_then(first, |x| {
        Box::pin(async move { 
            format!("Result: {}", x) 
        }) as Pin<Box<dyn Future<Output = String> + Send>>
    });
    
    let result = chained.await;
    assert_eq!(result, "Result: 42");
}

#[tokio::test]
#[should_panic(expected = "polled after completion")]
async fn test_future_polling_after_completion() {
    // Test that futures panic when polled after completion
    let mut future = ReadyFuture::new(42);
    
    // Poll once
    let result1 = (&mut future).await;
    assert_eq!(result1, 42);
    
    // This should panic when polled again
    // Note: This test verifies the panic behavior for safety
    let _result2 = future.await;
}
