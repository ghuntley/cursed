/// HTTP Client Integration Tests
/// 
/// Integration tests using real HTTP endpoints (httpbin.org) to test:
/// - Actual HTTP requests and responses
/// - Network error handling
/// - SSL/TLS functionality
/// - Real authentication scenarios
/// - Performance characteristics
/// - Concurrent request handling

use std::collections::HashMap;
use std::time::{Duration, Instant};
use std::thread;
use std::sync::{Arc, Mutex};

// Import the HTTP client from the web_vibez module
use cursed::stdlib::web_vibez::client::{
    HttpClient, HttpError, HttpResponse, RequestBuilder, Cookie, ConnectionPool
};

/// Test real GET request
#[test]
#[ignore] // Mark as ignored for CI - requires network access
fn test_real_get_request() {
    let client = HttpClient::new()
        .with_timeout(Duration::from_secs(10));
    
    let response = client.get("https://httpbin.org/get")
        .header("X-Test-Header".to_string(), "integration-test".to_string())
        .send();
    
    assert!(response.is_ok());
    let resp = response.unwrap();
    
    assert!(resp.is_success());
    assert_eq!(resp.status, 200);
    assert!(resp.content_length().unwrap_or(0) > 0);
    
    // Parse the JSON response
    let json_result = resp.json();
    assert!(json_result.is_ok());
    let json_text = json_result.unwrap();
    assert!(json_text.contains("httpbin.org"));
}

/// Test real POST request with JSON body
#[test]
#[ignore] // Mark as ignored for CI - requires network access
fn test_real_post_json() {
    let client = HttpClient::new()
        .with_timeout(Duration::from_secs(10));
    
    let json_data = r#"{"name": "CURSED", "type": "programming_language", "features": ["gen_z_slang", "go_like_syntax"]}"#;
    
    let response = client.post("https://httpbin.org/post")
        .json(json_data)
        .header("X-Language".to_string(), "CURSED".to_string())
        .send();
    
    assert!(response.is_ok());
    let resp = response.unwrap();
    
    assert!(resp.is_success());
    assert_eq!(resp.status, 200);
    
    let json_result = resp.json();
    assert!(json_result.is_ok());
    let response_text = json_result.unwrap();
    assert!(response_text.contains("CURSED"));
    assert!(response_text.contains("programming_language"));
}

/// Test real POST request with form data
#[test]
#[ignore] // Mark as ignored for CI - requires network access
fn test_real_post_form() {
    let client = HttpClient::new()
        .with_timeout(Duration::from_secs(10));
    
    let mut form_data = HashMap::new();
    form_data.insert("language".to_string(), "CURSED".to_string());
    form_data.insert("author".to_string(), "Geoffrey Huntley".to_string());
    form_data.insert("slang".to_string(), "periodt bestie".to_string());
    
    let response = client.post("https://httpbin.org/post")
        .form(&form_data)
        .send();
    
    assert!(response.is_ok());
    let resp = response.unwrap();
    
    assert!(resp.is_success());
    assert_eq!(resp.status, 200);
    
    let response_text = resp.text().unwrap();
    assert!(response_text.contains("CURSED"));
    assert!(response_text.contains("Geoffrey Huntley"));
}

/// Test PUT, DELETE, and PATCH methods
#[test]
#[ignore] // Mark as ignored for CI - requires network access
fn test_real_other_methods() {
    let client = HttpClient::new()
        .with_timeout(Duration::from_secs(10));
    
    // Test PUT
    let put_response = client.put("https://httpbin.org/put")
        .json(r#"{"operation": "update"}"#)
        .send();
    
    assert!(put_response.is_ok());
    assert!(put_response.unwrap().is_success());
    
    // Test DELETE
    let delete_response = client.delete("https://httpbin.org/delete")
        .send();
    
    assert!(delete_response.is_ok());
    assert!(delete_response.unwrap().is_success());
    
    // Test PATCH
    let patch_response = client.patch("https://httpbin.org/patch")
        .json(r#"{"field": "new_value"}"#)
        .send();
    
    assert!(patch_response.is_ok());
    assert!(patch_response.unwrap().is_success());
}

/// Test basic authentication
#[test]
#[ignore] // Mark as ignored for CI - requires network access
fn test_real_basic_auth() {
    let client = HttpClient::new()
        .with_timeout(Duration::from_secs(10));
    
    // Test successful basic auth
    let success_response = client.get("https://httpbin.org/basic-auth/user/pass")
        .basic_auth("user", "pass")
        .send();
    
    assert!(success_response.is_ok());
    let resp = success_response.unwrap();
    assert!(resp.is_success());
    assert_eq!(resp.status, 200);
    
    // Test failed basic auth
    let fail_response = client.get("https://httpbin.org/basic-auth/user/pass")
        .basic_auth("wrong", "credentials")
        .send();
    
    assert!(fail_response.is_ok());
    let resp = fail_response.unwrap();
    assert_eq!(resp.status, 401);
    assert!(resp.is_client_error());
}

/// Test bearer token authentication
#[test]
#[ignore] // Mark as ignored for CI - requires network access
fn test_real_bearer_auth() {
    let client = HttpClient::new()
        .with_timeout(Duration::from_secs(10));
    
    let response = client.get("https://httpbin.org/bearer")
        .bearer_token("test-token-123")
        .send();
    
    assert!(response.is_ok());
    let resp = response.unwrap();
    assert!(resp.is_success());
    
    let response_text = resp.text().unwrap();
    assert!(response_text.contains("test-token-123"));
}

/// Test redirect handling
#[test]
#[ignore] // Mark as ignored for CI - requires network access
fn test_real_redirects() {
    let client = HttpClient::new()
        .with_timeout(Duration::from_secs(10))
        .with_redirects(true, 10);
    
    // Test following redirects
    let redirect_response = client.get("https://httpbin.org/redirect/3")
        .send();
    
    assert!(redirect_response.is_ok());
    let resp = redirect_response.unwrap();
    assert!(resp.is_success());
    assert_eq!(resp.status, 200);
    
    // The final URL should be different from the original
    assert!(resp.url.contains("httpbin.org/get"));
    
    // Test not following redirects
    let no_redirect_client = HttpClient::new()
        .with_timeout(Duration::from_secs(10))
        .with_redirects(false, 0);
    
    let no_redirect_response = no_redirect_client.get("https://httpbin.org/redirect/1")
        .send();
    
    assert!(no_redirect_response.is_ok());
    let resp = no_redirect_response.unwrap();
    assert_eq!(resp.status, 302); // Should be a redirect status
}

/// Test response headers and cookies
#[test]
#[ignore] // Mark as ignored for CI - requires network access
fn test_real_headers_and_cookies() {
    let client = HttpClient::new()
        .with_timeout(Duration::from_secs(10));
    
    // Test setting and retrieving cookies
    let cookie_response = client.get("https://httpbin.org/cookies/set")
        .header("Cookie".to_string(), "test=value".to_string())
        .send();
    
    assert!(cookie_response.is_ok());
    let resp = cookie_response.unwrap();
    
    // Check headers
    assert!(resp.has_header("Content-Type"));
    assert!(resp.content_type().is_some());
    
    // Test custom headers
    let header_response = client.get("https://httpbin.org/headers")
        .header("X-Custom-Header".to_string(), "custom-value".to_string())
        .header("Accept".to_string(), "application/json".to_string())
        .send();
    
    assert!(header_response.is_ok());
    let resp = header_response.unwrap();
    assert!(resp.is_success());
    
    let response_text = resp.text().unwrap();
    assert!(response_text.contains("X-Custom-Header"));
    assert!(response_text.contains("custom-value"));
}

/// Test error handling with real network errors
#[test]
#[ignore] // Mark as ignored for CI - requires network access
fn test_real_error_scenarios() {
    let client = HttpClient::new()
        .with_timeout(Duration::from_secs(5));
    
    // Test invalid hostname
    let invalid_host_response = client.get("https://this-domain-does-not-exist-12345.com")
        .send();
    
    assert!(invalid_host_response.is_err());
    if let Err(error) = invalid_host_response {
        match error {
            HttpError::NetworkError(_) | HttpError::ConnectionError(_) => {}, // Expected
            _ => panic!("Expected network or connection error, got: {:?}", error),
        }
    }
    
    // Test 404 error (successful request but error status)
    let not_found_response = client.get("https://httpbin.org/status/404")
        .send();
    
    assert!(not_found_response.is_ok());
    let resp = not_found_response.unwrap();
    assert_eq!(resp.status, 404);
    assert!(resp.is_client_error());
    
    // Test 500 error
    let server_error_response = client.get("https://httpbin.org/status/500")
        .send();
    
    assert!(server_error_response.is_ok());
    let resp = server_error_response.unwrap();
    assert_eq!(resp.status, 500);
    assert!(resp.is_server_error());
}

/// Test timeout handling
#[test]
#[ignore] // Mark as ignored for CI - requires network access
fn test_real_timeout() {
    let client = HttpClient::new()
        .with_timeout(Duration::from_secs(2)); // Very short timeout
    
    // This should timeout (httpbin delay is longer than our timeout)
    let timeout_response = client.get("https://httpbin.org/delay/5")
        .send();
    
    assert!(timeout_response.is_err());
    if let Err(error) = timeout_response {
        match error {
            HttpError::TimeoutError => {}, // Expected
            _ => panic!("Expected timeout error, got: {:?}", error),
        }
    }
}

/// Test HTTPS/TLS functionality
#[test]
#[ignore] // Mark as ignored for CI - requires network access
fn test_real_https_tls() {
    let client = HttpClient::new()
        .with_timeout(Duration::from_secs(10));
    
    // Test HTTPS request
    let https_response = client.get("https://httpbin.org/get")
        .send();
    
    assert!(https_response.is_ok());
    let resp = https_response.unwrap();
    assert!(resp.is_success());
    assert!(resp.url.starts_with("https://"));
    
    // Test mixed protocol handling
    let http_response = client.get("http://httpbin.org/get")
        .send();
    
    // This might redirect to HTTPS or work directly
    assert!(http_response.is_ok());
    let resp = http_response.unwrap();
    assert!(resp.is_success());
}

/// Test concurrent requests
#[test]
#[ignore] // Mark as ignored for CI - requires network access
fn test_real_concurrent_requests() {
    let client = Arc::new(HttpClient::new().with_timeout(Duration::from_secs(10)));
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    
    // Make 5 concurrent requests
    for i in 0..5 {
        let client_clone = Arc::clone(&client);
        let results_clone = Arc::clone(&results);
        
        let handle = thread::spawn(move || {
            let response = client_clone.get("https://httpbin.org/get")
                .header("X-Thread-ID".to_string(), i.to_string())
                .send();
            
            let success = response.is_ok() && response.unwrap().is_success();
            results_clone.lock().unwrap().push(success);
        });
        
        handles.push(handle);
    }
    
    // Wait for all requests to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let results = results.lock().unwrap();
    assert_eq!(results.len(), 5);
    
    // All requests should have succeeded
    for &success in results.iter() {
        assert!(success, "One or more concurrent requests failed");
    }
}

/// Test performance characteristics
#[test]
#[ignore] // Mark as ignored for CI - requires network access
fn test_real_performance() {
    let client = HttpClient::new()
        .with_timeout(Duration::from_secs(30));
    
    let start_time = Instant::now();
    
    // Make several requests and measure total time
    for i in 0..5 {
        let response = client.get("https://httpbin.org/get")
            .header("X-Request-Number".to_string(), i.to_string())
            .send();
        
        assert!(response.is_ok());
        let resp = response.unwrap();
        assert!(resp.is_success());
        
        // Check individual request duration
        assert!(resp.request_duration < Duration::from_secs(10));
    }
    
    let total_duration = start_time.elapsed();
    
    // 5 requests should complete in reasonable time (allowing for network variability)
    assert!(total_duration < Duration::from_secs(30));
    
    println!("5 HTTP requests completed in: {:?}", total_duration);
}

/// Test large response handling
#[test]
#[ignore] // Mark as ignored for CI - requires network access
fn test_real_large_response() {
    let client = HttpClient::new()
        .with_timeout(Duration::from_secs(30));
    
    // Request a large response (100KB of JSON data)
    let response = client.get("https://httpbin.org/json")
        .send();
    
    assert!(response.is_ok());
    let resp = response.unwrap();
    assert!(resp.is_success());
    
    // Check that we can handle the large response
    let content_length = resp.content_length().unwrap_or(0);
    assert!(content_length > 0);
    
    let response_text = resp.text();
    assert!(response_text.is_ok());
    
    let text = response_text.unwrap();
    assert!(text.len() > 100); // Should be a substantial response
}

/// Test stream response handling
#[test]
#[ignore] // Mark as ignored for CI - requires network access
fn test_real_streaming_response() {
    let client = HttpClient::new()
        .with_timeout(Duration::from_secs(15));
    
    // Test streaming a response
    let response = client.get("https://httpbin.org/stream/10")
        .send();
    
    assert!(response.is_ok());
    let resp = response.unwrap();
    assert!(resp.is_success());
    
    // The response should contain multiple JSON objects
    let response_text = resp.text().unwrap();
    let line_count = response_text.split("\n").count();
    assert!(line_count >= 10); // Should have at least 10 lines
}

/// Test request with custom user agent
#[test]
#[ignore] // Mark as ignored for CI - requires network access
fn test_real_custom_user_agent() {
    let client = HttpClient::new()
        .with_user_agent("CURSED-HTTP-Client/1.0 (Integration-Test)".to_string())
        .with_timeout(Duration::from_secs(10));
    
    let response = client.get("https://httpbin.org/user-agent")
        .send();
    
    assert!(response.is_ok());
    let resp = response.unwrap();
    assert!(resp.is_success());
    
    let response_text = resp.text().unwrap();
    assert!(response_text.contains("CURSED-HTTP-Client/1.0"));
    assert!(response_text.contains("Integration-Test"));
}

/// Test gzip compression handling
#[test]
#[ignore] // Mark as ignored for CI - requires network access
fn test_real_compression() {
    let client = HttpClient::new()
        .with_timeout(Duration::from_secs(10));
    
    let response = client.get("https://httpbin.org/gzip")
        .header("Accept-Encoding".to_string(), "gzip".to_string())
        .send();
    
    assert!(response.is_ok());
    let resp = response.unwrap();
    assert!(resp.is_success());
    
    // The response should be automatically decompressed
    let response_text = resp.text().unwrap();
    assert!(response_text.contains("gzipped")); // httpbin.org returns this in gzip responses
}

/// Test connection reuse and keep-alive
#[test]
#[ignore] // Mark as ignored for CI - requires network access
fn test_real_connection_reuse() {
    let client = HttpClient::new()
        .with_timeout(Duration::from_secs(10));
    
    let start_time = Instant::now();
    
    // Make multiple requests to the same host
    for i in 0..3 {
        let response = client.get("https://httpbin.org/get")
            .header("X-Request".to_string(), format!("request-{}", i))
            .send();
        
        assert!(response.is_ok());
        let resp = response.unwrap();
        assert!(resp.is_success());
    }
    
    let total_time = start_time.elapsed();
    
    // Subsequent requests should be faster due to connection reuse
    // This is a rough test - exact timing depends on network conditions
    println!("3 requests to same host took: {:?}", total_time);
    assert!(total_time < Duration::from_secs(15));
}

/// Test real-world JSON API interaction
#[test]
#[ignore] // Mark as ignored for CI - requires network access
fn test_real_json_api() {
    let client = HttpClient::new()
        .with_timeout(Duration::from_secs(10));
    
    // Test with a real JSON API endpoint
    let response = client.get("https://httpbin.org/json")
        .header("Accept".to_string(), "application/json".to_string())
        .send();
    
    assert!(response.is_ok());
    let resp = response.unwrap();
    assert!(resp.is_success());
    assert_eq!(resp.content_type().unwrap(), "application/json");
    
    // Parse the JSON response
    let json_result = resp.json();
    assert!(json_result.is_ok());
    
    let json_text = json_result.unwrap();
    // httpbin.org/json returns a sample JSON object
    assert!(json_text.contains("slideshow"));
}
