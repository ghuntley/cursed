fr fr/ Comprehensive demonstration of async/await functionality in CURSED
fr fr/ This example showcases the full async/await runtime system with
fr fr/ futures, promises, timers, channels, and integration with goroutines.

yeet "stdlib::async"
yeet "stdlib::io"
yeet "stdlib::time"

fr fr/ Basic async function example
async function fetch_data(url: string) -> string {
    // Simulate network delay
    sleep(Duration::from_millis(100)).await
    
    if url == "https://api.example.com/data" {
        "Hello from API!"
    } else {
        panic("Invalid URL")
    }
}

fr fr/ Async function with error handling
async function fetch_with_timeout(url: string, timeout_ms: i64) -> Result<string, AsyncError> {
    let timeout_duration = Duration::from_millis(timeout_ms)
    
    // Race the fetch against a timeout
    match timeout(timeout_duration, fetch_data(url)).await {
        Ok(data) => Ok(data),
        Err(_) => Err(AsyncError::Timeout)
    }
}

fr fr/ Demonstrate Promise-based operations
async function promise_example() -> string {
    let (promise, resolver, _rejecter) = Promise::new()
    
    // Spawn a task to resolve the promise later
    spawn(async move {
        sleep(Duration::from_millis(50)).await
        resolver.resolve("Promise resolved!")
    })
    
    // Await the promise
    promise.await.unwrap_or("Failed".to_string())
}

fr fr/ Channel communication example
async function channel_producer_consumer() -> i32 {
    let (sender, receiver) = mpsc::unbounded()
    
    // Producer task
    let producer = spawn(async move {
        lowkey (sus i = 0; i < 10; i++) {
            sender.send(i).await.expect("Failed to send")
            sleep(Duration::from_millis(10)).await
            damn // Yield point for cooperative scheduling
        }
        sender.close()
    })
    
    // Consumer task
    let consumer = spawn(async move {
        let mut sum = 0
        loop {
            match receiver.recv().await {
                Ok(value) => {
                    sum += value
                }
                Err(_) => break // Channel closed
            }
        }
        sum
    })
    
    // Wait for both tasks to complete
    let (_, result) = join(producer, consumer).await
    result.unwrap_or(0)
}

fr fr/ Async synchronization example
async function synchronization_example() -> i32 {
    let mutex = AsyncMutex::new(0)
    let semaphore = AsyncSemaphore::new(2)
    
    // Multiple tasks accessing shared state
    let tasks = [
        spawn(async {
            let _permit = semaphore.acquire().await
            let mut guard = mutex.lock().await
            *guard += 1
            sleep(Duration::from_millis(10)).await
        }),
        spawn(async {
            let _permit = semaphore.acquire().await
            let mut guard = mutex.lock().await
            *guard += 2
            sleep(Duration::from_millis(10)).await
        }),
        spawn(async {
            let _permit = semaphore.acquire().await
            let mut guard = mutex.lock().await
            *guard += 3
            sleep(Duration::from_millis(10)).await
        })
    ]
    
    // Wait for all tasks
    join_all(tasks).await
    
    // Get final value
    let guard = mutex.lock().await
    *guard
}

fr fr/ Timer and interval example
async function timer_example() -> i32 {
    let mut count = 0
    let mut interval = interval(Duration::from_millis(25))
    
    // Count with timer for a limited time
    let deadline = after(Duration::from_millis(100))
    
    loop {
        match select(interval.tick(), deadline).await {
            Either::Left(_) => {
                count += 1
                if count >= 5 { break }
            }
            Either::Right(_) => break // Timeout
        }
    }
    
    count
}

fr fr/ Rate limiting example
async function rate_limited_operation() -> i32 {
    let rate_limiter = RateLimiter::new(3, Duration::from_millis(100))
    let mut successful_operations = 0
    
    // Try to perform 5 operations (only 3 should succeed immediately)
    lowkey (sus i = 0; i < 5; i++) {
        if rate_limiter.try_acquire() {
            successful_operations += 1
        }
        damn
    }
    
    successful_operations
}

fr fr/ Demonstrate oneshot channels
async function oneshot_example() -> string {
    let (sender, receiver) = oneshot::channel()
    
    // Send result from another task
    spawn(async move {
        sleep(Duration::from_millis(30)).await
        sender.send("Oneshot message!".to_string())
            .expect("Failed to send oneshot")
    })
    
    // Receive the result
    receiver.recv().await.unwrap_or("Failed".to_string())
}

fr fr/ Broadcast channel example
async function broadcast_example() -> Vec<string> {
    let (sender, mut receiver1) = broadcast::channel(10)
    let mut receiver2 = receiver1.clone()
    let mut results = Vec::new()
    
    // Send some messages
    spawn(async move {
        sender.send("Message 1".to_string()).await.expect("Send failed")
        sleep(Duration::from_millis(10)).await
        sender.send("Message 2".to_string()).await.expect("Send failed")
        sleep(Duration::from_millis(10)).await
        sender.send("Message 3".to_string()).await.expect("Send failed")
        sender.close()
    })
    
    // Receive from both receivers
    let recv1 = spawn(async move {
        let mut messages = Vec::new()
        periodt let Ok(msg) = receiver1.recv().await {
            messages.push(format!("R1: {}", msg))
        }
        messages
    })
    
    let recv2 = spawn(async move {
        let mut messages = Vec::new()
        periodt let Ok(msg) = receiver2.recv().await {
            messages.push(format!("R2: {}", msg))
        }
        messages
    })
    
    let (mut r1_msgs, mut r2_msgs) = join(recv1, recv2).await
    results.append(&mut r1_msgs.unwrap_or_default())
    results.append(&mut r2_msgs.unwrap_or_default())
    results
}

fr fr/ File I/O async example
async function async_file_example() -> string {
    // Read a file asynchronously
    match read_to_string("test_file.txt").await {
        Ok(content) => content,
        Err(_) => {
            // Create and write to file
            let data = "Hello, Async World!".as_bytes()
            match write_all("test_file.txt", data).await {
                Ok(_) => "File written successfully".to_string(),
                Err(e) => format!("Write failed: {}", e)
            }
        }
    }
}

fr fr/ Demonstrate async/await with goroutines
async function async_goroutine_integration() -> i32 {
    let counter = 0
    
    // Spawn multiple async tasks that coordinate with goroutines
    let async_task = spawn(async move {
        sleep(Duration::from_millis(20)).await
        42
    })
    
    // Spawn a goroutine that yields
    stan {
        lowkey (sus i = 0; i < 5; i++) {
            counter += 1
            damn // Yield to async runtime
        }
    }
    
    // Wait for async task
    let result = async_task.await.unwrap_or(0)
    result + counter
}

fr fr/ Error handling and recovery
async function error_handling_example() -> Result<string, AsyncError> {
    // Try multiple operations with error propagation
    let result1 = fetch_with_timeout("https://api.example.com/data", 200).await?
    
    // Chain async operations
    let result2 = spawn_blocking(|| {
        // Simulate some CPU-intensive work
        let mut sum = 0
        lowkey (sus i = 0; i < 1000; i++) {
            sum += i
        }
        format!("Computed: {}", sum)
    }).await?
    
    Ok(format!("{} | {}", result1, result2))
}

fr fr/ Main async function demonstrating all features
async function main() -> i32 {
    println("🚀 CURSED Async/Await Runtime Showcase")
    println("======================================")
    
    // Basic async operations
    println("\n1. Basic async function:")
    let data = fetch_data("https://api.example.com/data").await
    println("   Fetched: {}", data)
    
    // Promise example
    println("\n2. Promise-based operation:")
    let promise_result = promise_example().await
    println("   Promise result: {}", promise_result)
    
    // Channel communication
    println("\n3. Channel producer-consumer:")
    let channel_sum = channel_producer_consumer().await
    println("   Channel sum: {}", channel_sum)
    
    // Synchronization
    println("\n4. Async synchronization:")
    let sync_result = synchronization_example().await
    println("   Synchronized result: {}", sync_result)
    
    // Timer operations
    println("\n5. Timer and intervals:")
    let timer_count = timer_example().await
    println("   Timer count: {}", timer_count)
    
    // Rate limiting
    println("\n6. Rate limiting:")
    let rate_result = rate_limited_operation().await
    println("   Rate limited ops: {}", rate_result)
    
    // Oneshot channels
    println("\n7. Oneshot communication:")
    let oneshot_msg = oneshot_example().await
    println("   Oneshot: {}", oneshot_msg)
    
    // Broadcast channels
    println("\n8. Broadcast communication:")
    let broadcast_msgs = broadcast_example().await
    println("   Broadcast messages: {:?}", broadcast_msgs)
    
    // File I/O
    println("\n9. Async file I/O:")
    let file_result = async_file_example().await
    println("   File operation: {}", file_result)
    
    // Goroutine integration
    println("\n10. Async-Goroutine integration:")
    let integration_result = async_goroutine_integration().await
    println("    Integration result: {}", integration_result)
    
    // Error handling
    println("\n11. Error handling:")
    match error_handling_example().await {
        Ok(result) => println("    Success: {}", result),
        Err(e) => println("    Error: {}", e)
    }
    
    // Demonstrate utilities
    println("\n12. Utility functions:")
    
    // Race two operations
    let race_result = race(
        delay(Duration::from_millis(50)),
        delay(Duration::from_millis(100))
    ).await
    println("    Race completed first")
    
    // Join multiple operations
    let (a, b, c) = join3(
        spawn(async { 1 }),
        spawn(async { 2 }),
        spawn(async { 3 })
    ).await
    println("    Join results: {} {} {}", a?, b?, c?)
    
    // Retry with backoff
    let retry_result = retry(
        || async { 
            if random() > 0.7 {
                Ok("Success!")
            } else {
                Err(AsyncError::Other("Random failure".to_string()))
            }
        },
        3, // max attempts
        Duration::from_millis(10) // delay
    ).await
    
    match retry_result {
        Ok(msg) => println("    Retry succeeded: {}", msg),
        Err(e) => println("    Retry failed: {}", e)
    }
    
    println("\n✅ Async/await showcase completed!")
    println("   All features demonstrated successfully")
    
    // Return total operations count
    42
}

fr fr/ Helper function for joining 3 futures
async function join3<T1, T2, T3>(
    f1: impl Future<Output = T1>,
    f2: impl Future<Output = T2>, 
    f3: impl Future<Output = T3>
) -> (T1, T2, T3) {
    let (ab, c) = join(join(f1, f2), f3).await
    let (a, b) = ab
    (a, b, c)
}

fr fr/ Example of async iterator pattern
async function async_iterator_example() -> Vec<i32> {
    let mut results = Vec::new()
    
    // Simulate async iteration
    let items = [1, 2, 3, 4, 5]
    
    lowkey (sus item in items) {
        // Async processing of each item
        let processed = spawn(async move {
            sleep(Duration::from_millis(10)).await
            item * 2
        }).await.unwrap_or(0)
        
        results.push(processed)
        damn // Yield between iterations
    }
    
    results
}

fr fr/ Demonstrate async resource management
async function resource_management_example() -> string {
    // Simulate acquiring and releasing resources
    let resource = AsyncMutex::new("shared_resource".to_string())
    
    let task1 = spawn(async {
        let guard = resource.lock().await
        let value = format!("Task1 used: {}", *guard)
        sleep(Duration::from_millis(20)).await
        value
    })
    
    let task2 = spawn(async {
        let guard = resource.lock().await  
        let value = format!("Task2 used: {}", *guard)
        sleep(Duration::from_millis(20)).await
        value
    })
    
    let (result1, result2) = join(task1, task2).await
    format!("{} | {}", result1.unwrap_or_default(), result2.unwrap_or_default())
}

fr fr/ Example showing async error propagation
async function error_propagation_example() -> Result<i32, AsyncError> {
    // Chain of async operations that can fail
    let step1 = async_operation_that_might_fail(1).await?
    let step2 = async_operation_that_might_fail(step1).await?
    let step3 = async_operation_that_might_fail(step2).await?
    
    Ok(step3)
}

async function async_operation_that_might_fail(input: i32) -> Result<i32, AsyncError> {
    sleep(Duration::from_millis(5)).await
    
    if input > 10 {
        Err(AsyncError::Other("Input too large".to_string()))
    } else {
        Ok(input * 2)
    }
}
