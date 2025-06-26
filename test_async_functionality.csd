// Test CURSED async functionality
package main

import "stdlib/async"
import "stdlib/time"

// Async function declaration
async fn fetch_data(url: string) -> Result<string, Error> {
    print("Fetching data from: " + url)
    
    // Simulate network delay
    await async::sleep(100ms)
    
    if url == "https://api.example.com/data" {
        return Ok("Data fetched successfully")
    } else {
        return Err(Error::new("Invalid URL"))
    }
}

// Async function with timeout
async fn fetch_with_timeout(url: string) -> Result<string, Error> {
    let future = fetch_data(url)
    let result = await async::timeout(500ms, future)
    
    match result {
        Ok(data) => Ok(data),
        Err(_) => Err(Error::new("Request timed out")),
    }
}

// Main async function
async fn main() -> Result<(), Error> {
    print("Starting async demo...")
    
    // Single async operation
    let result1 = await fetch_data("https://api.example.com/data")
    match result1 {
        Ok(data) => print("Got data: " + data),
        Err(e) => print("Error: " + e.message()),
    }
    
    // Multiple concurrent operations
    let futures = [
        fetch_data("https://api.example.com/data"),
        fetch_data("https://api.example.com/users"),
        fetch_data("https://api.example.com/posts"),
    ]
    
    let results = await async::join_all(futures)
    
    for (i, result) in results.enumerate() {
        match result {
            Ok(data) => print("Task " + i.to_string() + " completed: " + data),
            Err(e) => print("Task " + i.to_string() + " failed: " + e.message()),
        }
    }
    
    // Racing operations
    let race_futures = [
        fetch_data("https://fast-api.com/data"),
        fetch_data("https://slow-api.com/data"),
    ]
    
    let first_result = await async::race_all(race_futures)
    print("First to complete: " + first_result.unwrap_or("None"))
    
    // Timeout example
    let timeout_result = await fetch_with_timeout("https://very-slow-api.com/data")
    match timeout_result {
        Ok(data) => print("Data within timeout: " + data),
        Err(e) => print("Timeout error: " + e.message()),
    }
    
    // Goroutine integration
    stan {
        print("Running in goroutine...")
        let async_result = await fetch_data("https://api.example.com/background")
        print("Background task completed: " + async_result.unwrap_or("Failed"))
    }
    
    // Give goroutine time to complete
    await async::sleep(200ms)
    
    print("Async demo completed!")
    Ok(())
}

// Promise-style async operations
fn demo_promises() {
    let promise = Promise::new()
    
    // Resolve after delay
    stan {
        async::sleep(100ms).await
        promise.resolve("Promise resolved!")
    }
    
    promise.then(|value| {
        print("Promise result: " + value)
        Promise::resolved(value.len())
    }).then(|length| {
        print("Value length: " + length.to_string())
        Promise::resolved(())
    }).catch(|error| {
        print("Promise error: " + error.to_string())
        Promise::resolved(())
    }).finally(|| {
        print("Promise chain completed")
        Promise::resolved(())
    })
}

// Async I/O example
async fn demo_async_io() -> Result<(), Error> {
    // Async file operations
    let content = await async::fs::read_file("test.txt")
    print("File content: " + content)
    
    await async::fs::write_file("output.txt", "Hello, async world!")
    
    // Async network operations
    let response = await async::net::get("https://httpbin.org/json")
    print("HTTP response: " + response.body())
    
    Ok(())
}

// Error handling in async context
async fn demo_error_handling() {
    let result = async {
        let data = await fetch_data("invalid-url")
        data.map_err(|e| format!("Fetch failed: {}", e.message()))
    }
    
    match await result {
        Ok(data) => print("Success: " + data),
        Err(error) => print("Error: " + error),
    }
}

// Async iterators and streams
async fn demo_async_streams() {
    let stream = async_stream! {
        for i in 0..10 {
            await async::sleep(10ms)
            yield i * i
        }
    }
    
    await for value in stream {
        print("Stream value: " + value.to_string())
    }
}
