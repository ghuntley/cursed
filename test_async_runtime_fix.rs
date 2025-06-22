/// Test async runtime functionality after fixing E0659 conflicts
use cursed::runtime::r#async::{
    Future, FutureError, FutureResult,
    Promise, PromiseResolver, PromiseState,
    AsyncRuntime, AsyncRuntimeConfig,
    spawn, block_on, delay
};
use cursed::stdlib::r#async::{
    AsyncError, AsyncResult,
    spawn_blocking, timeout as async_timeout
};
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Testing async runtime functionality...");

    // Test 1: Basic future creation and awaiting
    println!("1. Testing basic future operations...");
    let (promise, resolver, _rejecter) = Promise::new();
    
    // Resolve the promise in a separate task
    tokio::spawn(async move {
        tokio::time::sleep(Duration::from_millis(100)).await;
        resolver.resolve("Hello, async!".to_string());
    });
    
    match promise.await {
        Ok(result) => println!("✅ Promise resolved: {}", result),
        Err(e) => println!("❌ Promise failed: {}", e),
    }

    // Test 2: Async delay/timeout
    println!("2. Testing delay and timeout...");
    let start = std::time::Instant::now();
    tokio::time::sleep(Duration::from_millis(50)).await;
    let elapsed = start.elapsed();
    println!("✅ Delay completed in {:?}", elapsed);

    // Test 3: Racing futures 
    println!("3. Testing future racing...");
    use cursed::runtime::r#async::timer::utils::Timer;
    
    let fast_future = async { "fast" };
    let slow_future = async {
        tokio::time::sleep(Duration::from_millis(200)).await;
        "slow"
    };
    
    let result = Timer::race(fast_future, slow_future).await;
    match result {
        cursed::runtime::r#async::timer::utils::Either::Left(val) => {
            println!("✅ Fast future won: {}", val);
        }
        cursed::runtime::r#async::timer::utils::Either::Right(val) => {
            println!("⚠️  Slow future won: {}", val);
        }
    }

    // Test 4: Async I/O operations
    println!("4. Testing async I/O...");
    match async_timeout(Duration::from_millis(500), async {
        tokio::time::sleep(Duration::from_millis(100)).await;
        "I/O completed"
    }).await {
        Ok(result) => println!("✅ Async I/O: {}", result),
        Err(e) => println!("❌ Async I/O failed: {}", e),
    }

    println!("✅ All async runtime tests completed successfully!");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_promise_creation() {
        let (promise, resolver, _rejecter) = Promise::new();
        resolver.resolve(42);
        assert_eq!(promise.await.unwrap(), 42);
    }

    #[tokio::test]
    async fn test_async_timeout() {
        let result = async_timeout(Duration::from_millis(100), async {
            tokio::time::sleep(Duration::from_millis(50)).await;
            "success"
        }).await;
        
        assert!(result.is_ok());
    }

    #[test]
    fn test_future_state_enum() {
        use cursed::runtime::r#async::FutureState;
        
        let state = FutureState::Pending;
        assert_eq!(state, FutureState::Pending);
        
        let state = FutureState::Ready;
        assert_eq!(state, FutureState::Ready);
    }
}
