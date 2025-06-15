/// HTTP Client Performance and Stress Tests
/// 
/// Performance testing for the CURSED HTTP client including:
/// - Throughput measurement
/// - Connection pooling efficiency
/// - Memory usage under load
/// - Concurrent request handling
/// - Large payload processing
/// - Timeout behavior under stress

use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::thread;
use std::sync::{Arc, Mutex, mpsc};

// Import the HTTP client from the web_vibez module
use cursed::stdlib::web_vibez::client::{
    HttpClient, HttpError, HttpResponse, RequestBuilder, Cookie, ConnectionPool
};

/// Test client creation performance
#[test]
fn test_client_creation_performance() {
    let start = Instant::now();
    let iterations = 1000;
    
    for _ in 0..iterations {
        let _client = HttpClient::new()
            .with_timeout(Duration::from_secs(30))
            .with_user_agent("Performance-Test".to_string());
    }
    
    let duration = start.elapsed();
    let per_creation = duration / iterations;
    
    println!("Client creation: {} iterations in {:?} ({:?} per creation)", 
             iterations, duration, per_creation);
    
    // Client creation should be very fast
    assert!(per_creation < Duration::from_millis(1));
}

/// Test request builder performance
#[test]
fn test_request_builder_performance() {
    let client = HttpClient::new()
        .with_base_url("https://api.example.com".to_string());
    
    let start = Instant::now();
    let iterations = 10000;
    
    for i in 0..iterations {
        let _request = client.post("/api/data")
            .header("Content-Type".to_string(), "application/json".to_string())
            .header("Authorization".to_string(), format!("Bearer token-{}", i))
            .json(&format!(r#"{{"id": {}, "data": "test"}}"#, i));
    }
    
    let duration = start.elapsed();
    let per_request = duration / iterations;
    
    println!("Request building: {} iterations in {:?} ({:?} per request)", 
             iterations, duration, per_request);
    
    // Request building should be very fast
    assert!(per_request < Duration::from_micros(100));
}

/// Test concurrent request building
#[test]
fn test_concurrent_request_building() {
    let client = Arc::new(HttpClient::new()
        .with_base_url("https://api.example.com".to_string()));
    
    let num_threads = 8;
    let requests_per_thread = 1000;
    let start = Instant::now();
    
    let mut handles = Vec::new();
    
    for thread_id in 0..num_threads {
        let client_clone = Arc::clone(&client);
        
        let handle = thread::spawn(move || {
            for i in 0..requests_per_thread {
                let _request = client_clone.get(&format!("/data/{}", i))
                    .header("X-Thread-ID".to_string(), thread_id.to_string())
                    .header("X-Request-ID".to_string(), i.to_string());
            }
        });
        
        handles.push(handle);
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let duration = start.elapsed();
    let total_requests = num_threads * requests_per_thread;
    let requests_per_second = total_requests as f64 / duration.as_secs_f64();
    
    println!("Concurrent request building: {} requests across {} threads in {:?} ({:.2} req/sec)", 
             total_requests, num_threads, duration, requests_per_second);
    
    // Should handle at least 10k requests per second
    assert!(requests_per_second > 10000.0);
}

/// Test form data encoding performance
#[test]
fn test_form_encoding_performance() {
    let client = HttpClient::new();
    
    // Create a large form with many fields
    let mut form_data = HashMap::new();
    for i in 0..1000 {
        form_data.insert(
            format!("field_{}", i),
            format!("value_with_special_chars_{}!@#$%^&*()", i)
        );
    }
    
    let start = Instant::now();
    let iterations = 100;
    
    for _ in 0..iterations {
        let _request = client.post("https://example.com/submit")
            .form(&form_data);
    }
    
    let duration = start.elapsed();
    let per_encoding = duration / iterations;
    
    println!("Form encoding (1000 fields): {} iterations in {:?} ({:?} per encoding)", 
             iterations, duration, per_encoding);
    
    // Form encoding should be reasonable even for large forms
    assert!(per_encoding < Duration::from_millis(10));
}

/// Test JSON serialization performance
#[test]
fn test_json_performance() {
    let client = HttpClient::new();
    
    // Create a large JSON string
    let mut json_parts = Vec::new();
    for i in 0..1000 {
        json_parts.push(format!(r#""field_{}": "value_{}""#, i, i));
    }
    let large_json = format!("{{{}}}", json_parts.join(", "));
    
    let start = Instant::now();
    let iterations = 1000;
    
    for _ in 0..iterations {
        let _request = client.post("https://example.com/api")
            .json(&large_json);
    }
    
    let duration = start.elapsed();
    let per_operation = duration / iterations;
    
    println!("JSON request building (large JSON): {} iterations in {:?} ({:?} per operation)", 
             iterations, duration, per_operation);
    
    // JSON handling should be fast
    assert!(per_operation < Duration::from_millis(1));
}

/// Test header manipulation performance
#[test]
fn test_header_performance() {
    let client = HttpClient::new();
    
    let start = Instant::now();
    let iterations = 10000;
    
    for i in 0..iterations {
        let mut headers = HashMap::new();
        for j in 0..10 {
            headers.insert(
                format!("X-Header-{}-{}", i, j),
                format!("value-{}-{}", i, j)
            );
        }
        
        let _request = client.get("https://example.com/test")
            .headers(headers)
            .header("X-Final-Header".to_string(), "final-value".to_string());
    }
    
    let duration = start.elapsed();
    let per_operation = duration / iterations;
    
    println!("Header manipulation: {} iterations in {:?} ({:?} per operation)", 
             iterations, duration, per_operation);
    
    // Header operations should be very fast
    assert!(per_operation < Duration::from_micros(100));
}

/// Test connection pool performance
#[test]
fn test_connection_pool_performance() {
    let mut pool = ConnectionPool::new()
        .with_max_connections(100);
    
    let hosts = vec!["example.com", "test.com", "api.com", "service.com", "backend.com"];
    
    let start = Instant::now();
    let iterations = 10000;
    let mut connections = Vec::new();
    
    // Get many connections
    for i in 0..iterations {
        let host = &hosts[i % hosts.len()];
        if let Some(conn) = pool.get_connection(host) {
            connections.push(conn);
        }
    }
    
    // Return all connections
    for conn in connections {
        pool.return_connection(conn);
    }
    
    let duration = start.elapsed();
    let operations_per_second = (iterations * 2) as f64 / duration.as_secs_f64(); // *2 for get + return
    
    println!("Connection pool: {} get/return cycles in {:?} ({:.2} ops/sec)", 
             iterations, duration, operations_per_second);
    
    // Connection pool should handle many operations per second
    assert!(operations_per_second > 10000.0);
    
    let stats = pool.stats();
    println!("Pool stats: {} connections across {} hosts", 
             stats.total_connections, stats.total_hosts);
}

/// Test cookie parsing performance
#[test]
fn test_cookie_parsing_performance() {
    let cookie_strings = vec![
        "simple=value",
        "session=abc123; Path=/; Secure; HttpOnly",
        "complex=value123; Domain=.example.com; Path=/app; Secure; HttpOnly; Max-Age=3600; Expires=Wed, 09 Jun 2021 10:18:14 GMT",
        "multi_attr=complex_value; Domain=subdomain.example.com; Path=/very/long/path; Secure; HttpOnly; Max-Age=7200; SameSite=Strict",
        "encoded_value=some%20encoded%20value%20with%20spaces; Path=/test; Max-Age=1800",
    ];
    
    let start = Instant::now();
    let iterations = 10000;
    let mut parsed_count = 0;
    
    for i in 0..iterations {
        let cookie_str = &cookie_strings[i % cookie_strings.len()];
        if Cookie::parse(cookie_str).is_some() {
            parsed_count += 1;
        }
    }
    
    let duration = start.elapsed();
    let per_parse = duration / iterations;
    
    println!("Cookie parsing: {} iterations in {:?} ({:?} per parse), {} successful", 
             iterations, duration, per_parse, parsed_count);
    
    // Cookie parsing should be very fast
    assert!(per_parse < Duration::from_micros(10));
    assert_eq!(parsed_count, iterations); // All should parse successfully
}

/// Test URL encoding performance
#[test]
fn test_url_encoding_performance() {
    let test_strings = vec![
        "simple string",
        "string with spaces and special chars: !@#$%^&*()",
        "unicode test: 你好世界 🌍 émojis",
        "long string with many special characters: " + &"!@#$%^&*()".repeat(100),
        "email@example.com?param=value&other=test",
    ];
    
    let start = Instant::now();
    let iterations = 10000;
    
    for i in 0..iterations {
        let test_str = &test_strings[i % test_strings.len()];
        let _encoded = urlencoding::encode(test_str);
    }
    
    let duration = start.elapsed();
    let per_encoding = duration / iterations;
    
    println!("URL encoding: {} iterations in {:?} ({:?} per encoding)", 
             iterations, duration, per_encoding);
    
    // URL encoding should be fast
    assert!(per_encoding < Duration::from_micros(50));
}

/// Test base64 encoding performance
#[test]
fn test_base64_performance() {
    let test_data = vec![
        b"short".to_vec(),
        b"medium length string for testing".to_vec(),
        vec![0u8; 1024], // 1KB of zeros
        (0..255).cycle().take(4096).collect::<Vec<u8>>(), // 4KB of pattern
    ];
    
    let start = Instant::now();
    let iterations = 1000;
    
    for i in 0..iterations {
        let data = &test_data[i % test_data.len()];
        // Use the internal base64_encode function (simulated here)
        let _encoded = base64::encode(data);
    }
    
    let duration = start.elapsed();
    let per_encoding = duration / iterations;
    
    println!("Base64 encoding: {} iterations in {:?} ({:?} per encoding)", 
             iterations, duration, per_encoding);
    
    // Base64 encoding should be reasonably fast
    assert!(per_encoding < Duration::from_millis(1));
}

/// Test memory usage under concurrent load
#[test]
fn test_memory_usage_concurrent() {
    let client = Arc::new(HttpClient::new()
        .with_timeout(Duration::from_secs(1)));
    
    let num_threads = 16;
    let requests_per_thread = 100;
    let start = Instant::now();
    
    let (tx, rx) = mpsc::channel();
    let mut handles = Vec::new();
    
    for thread_id in 0..num_threads {
        let client_clone = Arc::clone(&client);
        let tx_clone = tx.clone();
        
        let handle = thread::spawn(move || {
            let mut request_count = 0;
            
            for i in 0..requests_per_thread {
                // Create and configure requests (don't send to avoid network dependency)
                let _request = client_clone.post("/api/data")
                    .header("Content-Type".to_string(), "application/json".to_string())
                    .header("X-Thread".to_string(), thread_id.to_string())
                    .json(&format!(r#"{{"thread": {}, "request": {}}}"#, thread_id, i));
                
                request_count += 1;
                
                // Simulate some processing time
                thread::sleep(Duration::from_micros(100));
            }
            
            tx_clone.send(request_count).unwrap();
        });
        
        handles.push(handle);
    }
    
    drop(tx); // Close the sender
    
    // Collect results
    let mut total_requests = 0;
    while let Ok(count) = rx.recv() {
        total_requests += count;
    }
    
    for handle in handles {
        handle.join().unwrap();
    }
    
    let duration = start.elapsed();
    let requests_per_second = total_requests as f64 / duration.as_secs_f64();
    
    println!("Memory stress test: {} requests across {} threads in {:?} ({:.2} req/sec)", 
             total_requests, num_threads, duration, requests_per_second);
    
    assert_eq!(total_requests, num_threads * requests_per_thread);
    
    // Should handle concurrent load efficiently
    assert!(requests_per_second > 1000.0);
}

/// Test large payload handling performance
#[test]
fn test_large_payload_performance() {
    let client = HttpClient::new();
    
    // Test different payload sizes
    let payload_sizes = vec![1024, 10240, 102400, 1048576]; // 1KB, 10KB, 100KB, 1MB
    
    for &size in &payload_sizes {
        let large_data = vec![b'A'; size];
        
        let start = Instant::now();
        let iterations = 10;
        
        for _ in 0..iterations {
            let _request = client.post("https://example.com/upload")
                .header("Content-Type".to_string(), "application/octet-stream".to_string())
                .body(large_data.clone());
        }
        
        let duration = start.elapsed();
        let per_request = duration / iterations;
        let throughput_mbps = (size as f64 * iterations as f64) / (1024.0 * 1024.0) / duration.as_secs_f64();
        
        println!("Large payload ({}KB): {} iterations in {:?} ({:?} per request, {:.2} MB/s throughput)", 
                 size / 1024, iterations, duration, per_request, throughput_mbps);
        
        // Should handle large payloads reasonably quickly
        assert!(per_request < Duration::from_millis(100));
    }
}

/// Test authentication performance
#[test]
fn test_authentication_performance() {
    let client = HttpClient::new();
    
    let start = Instant::now();
    let iterations = 10000;
    
    for i in 0..iterations {
        if i % 2 == 0 {
            let _request = client.get("https://example.com/protected")
                .basic_auth(&format!("user{}", i), &format!("pass{}", i));
        } else {
            let _request = client.get("https://example.com/api")
                .bearer_token(&format!("token-{}", i));
        }
    }
    
    let duration = start.elapsed();
    let per_auth = duration / iterations;
    
    println!("Authentication: {} operations in {:?} ({:?} per auth)", 
             iterations, duration, per_auth);
    
    // Authentication setup should be very fast
    assert!(per_auth < Duration::from_micros(10));
}

/// Test response processing performance (mock responses)
#[test]
fn test_response_processing_performance() {
    let iterations = 1000;
    let start = Instant::now();
    
    for i in 0..iterations {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        headers.insert("Content-Length".to_string(), "100".to_string());
        headers.insert("Set-Cookie".to_string(), format!("session{}=value{}; Path=/", i, i));
        
        let response = HttpResponse {
            status: 200,
            headers,
            body: format!(r#"{{"id": {}, "message": "test response {}"}}}"#, i, i).into_bytes(),
            url: format!("https://example.com/api/{}", i),
            request_duration: Duration::from_millis(100),
        };
        
        // Process the response
        let _is_success = response.is_success();
        let _content_type = response.content_type();
        let _text = response.text();
        let _json = response.json();
        let _cookies = response.cookies();
        let _headers = response.header_names();
    }
    
    let duration = start.elapsed();
    let per_response = duration / iterations;
    
    println!("Response processing: {} responses in {:?} ({:?} per response)", 
             iterations, duration, per_response);
    
    // Response processing should be very fast
    assert!(per_response < Duration::from_millis(1));
}

/// Benchmark overall request construction pipeline
#[test]
fn test_request_pipeline_benchmark() {
    let client = HttpClient::new()
        .with_base_url("https://api.example.com".to_string())
        .with_timeout(Duration::from_secs(30))
        .with_user_agent("CURSED-Benchmark/1.0".to_string());
    
    let start = Instant::now();
    let iterations = 5000;
    
    for i in 0..iterations {
        let mut form_data = HashMap::new();
        form_data.insert("id".to_string(), i.to_string());
        form_data.insert("name".to_string(), format!("item-{}", i));
        form_data.insert("description".to_string(), format!("Description for item {}", i));
        
        let _request = client.post(&format!("/api/items/{}", i))
            .header("Authorization".to_string(), format!("Bearer token-{}", i))
            .header("X-Request-ID".to_string(), format!("req-{}", i))
            .form(&form_data)
            .timeout(Duration::from_secs(10));
    }
    
    let duration = start.elapsed();
    let requests_per_second = iterations as f64 / duration.as_secs_f64();
    
    println!("Request pipeline benchmark: {} complete requests in {:?} ({:.2} req/sec)", 
             iterations, duration, requests_per_second);
    
    // Should construct requests very quickly
    assert!(requests_per_second > 5000.0);
}
