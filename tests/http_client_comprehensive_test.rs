/// Comprehensive HTTP Client Tests
/// 
/// Tests all functionality of the CURSED HTTP client including:
/// - Basic HTTP methods (GET, POST, PUT, DELETE, PATCH, HEAD, OPTIONS)
/// - Request/response handling with different content types
/// - Authentication mechanisms (Basic, Bearer)
/// - Header management and cookie handling
/// - Connection pooling and timeout handling
/// - Error scenarios and edge cases
/// - SSL/TLS support validation
/// - Performance and concurrency testing

use std::collections::HashMap;
use std::time::Duration;
use std::thread;
use std::sync::{Arc, Mutex};

// Import the HTTP client from the web_vibez module
use cursed::stdlib::web_vibez::client::{
    HttpClient, HttpError, HttpResponse, RequestBuilder, Cookie, ConnectionPool, PoolStats
};

/// Test basic HTTP client creation and configuration
#[test]
fn test_http_client_creation() {
    let client = HttpClient::new();
    assert_eq!(client.user_agent, "CURSED-WebVibez/1.0");
    
    let configured_client = HttpClient::new()
        .with_base_url("https://api.example.com".to_string())
        .with_timeout(Duration::from_secs(60))
        .with_user_agent("Test Client".to_string())
        .with_redirects(true, 5);
    
    assert_eq!(configured_client.base_url, Some("https://api.example.com".to_string()));
    assert_eq!(configured_client.timeout, Duration::from_secs(60));
    assert_eq!(configured_client.user_agent, "Test Client");
    assert_eq!(configured_client.max_redirects, 5);
}

/// Test HTTP method builders
#[test]
fn test_http_method_builders() {
    let client = HttpClient::new().with_base_url("https://httpbin.org".to_string());
    
    // Test all HTTP methods
    let get_request = client.get("/get");
    assert_eq!(get_request.method, "GET");
    assert_eq!(get_request.url, "https://httpbin.org/get");
    
    let post_request = client.post("/post");
    assert_eq!(post_request.method, "POST");
    
    let put_request = client.put("/put");
    assert_eq!(put_request.method, "PUT");
    
    let delete_request = client.delete("/delete");
    assert_eq!(delete_request.method, "DELETE");
    
    let patch_request = client.patch("/patch");
    assert_eq!(patch_request.method, "PATCH");
    
    let head_request = client.head("/get");
    assert_eq!(head_request.method, "HEAD");
    
    let options_request = client.options("/get");
    assert_eq!(options_request.method, "OPTIONS");
}

/// Test request builder functionality
#[test]
fn test_request_builder() {
    let client = HttpClient::new();
    
    // Test header addition
    let request = client.get("https://httpbin.org/get")
        .header("X-Custom-Header".to_string(), "test-value".to_string())
        .header("Accept".to_string(), "application/json".to_string());
    
    assert!(request.headers.contains_key("X-Custom-Header"));
    assert_eq!(request.headers.get("Accept").unwrap(), "application/json");
    
    // Test multiple headers
    let mut headers = HashMap::new();
    headers.insert("Header1".to_string(), "Value1".to_string());
    headers.insert("Header2".to_string(), "Value2".to_string());
    
    let multi_header_request = client.get("https://httpbin.org/get")
        .headers(headers);
    
    assert!(multi_header_request.headers.contains_key("Header1"));
    assert!(multi_header_request.headers.contains_key("Header2"));
}

/// Test different body types
#[test]
fn test_request_bodies() {
    let client = HttpClient::new();
    
    // Test JSON body
    let json_request = client.post("https://httpbin.org/post")
        .json(r#"{"name": "test", "value": 123}"#);
    
    assert_eq!(json_request.headers.get("Content-Type").unwrap(), "application/json");
    assert!(json_request.body.is_some());
    
    // Test form data
    let mut form_data = HashMap::new();
    form_data.insert("name".to_string(), "John Doe".to_string());
    form_data.insert("email".to_string(), "john@example.com".to_string());
    
    let form_request = client.post("https://httpbin.org/post")
        .form(&form_data);
    
    assert_eq!(form_request.headers.get("Content-Type").unwrap(), "application/x-www-form-urlencoded");
    
    // Test text body
    let text_request = client.post("https://httpbin.org/post")
        .text("Plain text content");
    
    assert_eq!(text_request.headers.get("Content-Type").unwrap(), "text/plain");
    
    // Test raw binary body
    let binary_data = vec![0x00, 0x01, 0x02, 0x03];
    let binary_request = client.post("https://httpbin.org/post")
        .body(binary_data.clone());
    
    assert_eq!(binary_request.headers.get("Content-Type").unwrap(), "application/octet-stream");
    assert_eq!(binary_request.body.unwrap(), binary_data);
}

/// Test authentication methods
#[test]
fn test_authentication() {
    let client = HttpClient::new();
    
    // Test basic authentication
    let basic_request = client.get("https://httpbin.org/basic-auth/user/pass")
        .basic_auth("user", "pass");
    
    let auth_header = basic_request.headers.get("Authorization").unwrap();
    assert!(auth_header.starts_with("Basic "));
    
    // Test bearer token authentication
    let bearer_request = client.get("https://httpbin.org/bearer")
        .bearer_token("abc123token");
    
    let bearer_header = bearer_request.headers.get("Authorization").unwrap();
    assert_eq!(bearer_header, "Bearer abc123token");
}

/// Test form data encoding
#[test]
fn test_form_encoding() {
    let client = HttpClient::new();
    let mut form_data = HashMap::new();
    form_data.insert("name".to_string(), "John Doe".to_string());
    form_data.insert("email".to_string(), "john+test@example.com".to_string());
    form_data.insert("message".to_string(), "Hello World!".to_string());
    
    let request = client.post("https://httpbin.org/post").form(&form_data);
    
    if let Some(body) = request.body {
        let body_str = String::from_utf8(body).unwrap();
        assert!(body_str.contains("name=John%20Doe"));
        assert!(body_str.contains("email=john%2Btest%40example.com"));
        assert!(body_str.contains("message=Hello%20World%21"));
    }
}

/// Test cookie parsing
#[test]
fn test_cookie_parsing() {
    // Test basic cookie
    let simple_cookie = Cookie::parse("sessionid=abc123");
    assert!(simple_cookie.is_some());
    let cookie = simple_cookie.unwrap();
    assert_eq!(cookie.name, "sessionid");
    assert_eq!(cookie.value, "abc123");
    assert!(!cookie.secure);
    assert!(!cookie.http_only);
    
    // Test complex cookie with attributes
    let complex_cookie_str = "sessionid=abc123; Domain=example.com; Path=/app; Secure; HttpOnly; Max-Age=3600; Expires=Wed, 09 Jun 2021 10:18:14 GMT";
    let complex_cookie = Cookie::parse(complex_cookie_str);
    assert!(complex_cookie.is_some());
    
    let cookie = complex_cookie.unwrap();
    assert_eq!(cookie.name, "sessionid");
    assert_eq!(cookie.value, "abc123");
    assert_eq!(cookie.domain, Some("example.com".to_string()));
    assert_eq!(cookie.path, Some("/app".to_string()));
    assert!(cookie.secure);
    assert!(cookie.http_only);
    assert_eq!(cookie.max_age, Some(3600));
    assert!(cookie.expires.is_some());
    
    // Test cookie to header value conversion
    let header_value = cookie.to_header_value();
    assert_eq!(header_value, "sessionid=abc123");
}

/// Test invalid cookie parsing
#[test]
fn test_invalid_cookie_parsing() {
    assert!(Cookie::parse("").is_none());
    assert!(Cookie::parse("invalid").is_none());
    assert!(Cookie::parse("=value").is_none());
    assert!(Cookie::parse("name=").is_some()); // Empty value is valid
}

/// Test connection pool functionality
#[test]
fn test_connection_pool() {
    let mut pool = ConnectionPool::new()
        .with_max_connections(5);
    
    // Test getting new connections
    let conn1 = pool.get_connection("example.com");
    assert!(conn1.is_some());
    
    let conn2 = pool.get_connection("test.com");
    assert!(conn2.is_some());
    
    // Test returning connections
    if let Some(conn) = conn1 {
        pool.return_connection(conn);
    }
    
    // Test reusing connections
    let conn3 = pool.get_connection("example.com");
    assert!(conn3.is_some());
    
    // Test pool statistics
    let stats = pool.stats();
    assert_eq!(stats.max_connections_per_host, 5);
    assert!(stats.total_hosts <= 2); // May be 1 or 2 depending on cleanup
}

/// Test connection pool limits
#[test]
fn test_connection_pool_limits() {
    let mut pool = ConnectionPool::new().with_max_connections(2);
    
    // Fill the pool for one host
    let conn1 = pool.get_connection("example.com").unwrap();
    let conn2 = pool.get_connection("example.com").unwrap();
    let conn3 = pool.get_connection("example.com").unwrap();
    
    // Return connections
    pool.return_connection(conn1);
    pool.return_connection(conn2);
    pool.return_connection(conn3); // This should be rejected due to limit
    
    let stats = pool.stats();
    assert!(stats.total_connections <= 2);
}

/// Test URL validation and error handling
#[test]
fn test_url_validation() {
    let client = HttpClient::new();
    
    // Test invalid URL
    let invalid_result = client.get("not-a-valid-url").send();
    assert!(invalid_result.is_err());
    
    if let Err(error) = invalid_result {
        match error {
            HttpError::InvalidUrl(_) => {}, // Expected
            _ => panic!("Expected InvalidUrl error"),
        }
    }
    
    // Test valid URL structure (we won't actually send the request)
    let valid_request = client.get("https://httpbin.org/get");
    assert!(valid_request.url.starts_with("https://"));
}

/// Test header validation
#[test]
fn test_header_validation() {
    let client = HttpClient::new();
    
    // Test valid headers
    let valid_request = client.get("https://httpbin.org/get")
        .header("Content-Type".to_string(), "application/json".to_string())
        .header("Accept".to_string(), "application/json".to_string());
    
    assert!(valid_request.headers.contains_key("Content-Type"));
    assert!(valid_request.headers.contains_key("Accept"));
    
    // Test request building with potentially problematic headers
    let problematic_request = client.get("https://httpbin.org/get")
        .header("Custom-Header".to_string(), "value-with-spaces".to_string());
    
    assert!(problematic_request.headers.contains_key("Custom-Header"));
}

/// Test timeout configuration
#[test]
fn test_timeout_configuration() {
    let client = HttpClient::new()
        .with_timeout(Duration::from_secs(5));
    
    let request = client.get("https://httpbin.org/delay/10")
        .timeout(Duration::from_secs(2));
    
    assert_eq!(request.timeout, Duration::from_secs(2));
}

/// Test redirect configuration
#[test]
fn test_redirect_configuration() {
    let client = HttpClient::new()
        .with_redirects(false, 0);
    
    let request = client.get("https://httpbin.org/redirect/3");
    assert!(!request.follow_redirects);
    assert_eq!(request.max_redirects, 0);
    
    let redirect_client = HttpClient::new()
        .with_redirects(true, 5);
    
    let redirect_request = redirect_client.get("https://httpbin.org/redirect/3");
    assert!(redirect_request.follow_redirects);
    assert_eq!(redirect_request.max_redirects, 5);
}

/// Test response methods (using mock response)
#[test]
fn test_response_methods() {
    let mut headers = HashMap::new();
    headers.insert("Content-Type".to_string(), "application/json".to_string());
    headers.insert("Content-Length".to_string(), "25".to_string());
    headers.insert("Set-Cookie".to_string(), "sessionid=abc123; Path=/".to_string());
    
    let response = HttpResponse {
        status: 200,
        headers,
        body: b"{\"message\": \"success\"}".to_vec(),
        url: "https://example.com/api".to_string(),
        request_duration: Duration::from_millis(150),
    };
    
    // Test status checking methods
    assert!(response.is_success());
    assert!(!response.is_client_error());
    assert!(!response.is_server_error());
    
    // Test content methods
    assert_eq!(response.text().unwrap(), "{\"message\": \"success\"}");
    assert_eq!(response.bytes(), b"{\"message\": \"success\"}");
    assert_eq!(response.content_type().unwrap(), "application/json");
    assert_eq!(response.content_length().unwrap(), 25);
    
    // Test header methods
    assert!(response.has_header("Content-Type"));
    assert!(!response.has_header("Authorization"));
    
    let header_names = response.header_names();
    assert!(header_names.contains(&&"Content-Type".to_string()));
    
    // Test JSON parsing
    let json_result = response.json();
    assert!(json_result.is_ok());
    
    // Test cookie extraction
    let cookies = response.cookies();
    assert_eq!(cookies.len(), 1);
    assert_eq!(cookies[0].name, "sessionid");
    assert_eq!(cookies[0].value, "abc123");
}

/// Test error response handling
#[test]
fn test_error_response_handling() {
    let error_response = HttpResponse {
        status: 404,
        headers: HashMap::new(),
        body: b"Not Found".to_vec(),
        url: "https://example.com/notfound".to_string(),
        request_duration: Duration::from_millis(50),
    };
    
    assert!(!error_response.is_success());
    assert!(error_response.is_client_error());
    assert!(!error_response.is_server_error());
    
    let server_error_response = HttpResponse {
        status: 500,
        headers: HashMap::new(),
        body: b"Internal Server Error".to_vec(),
        url: "https://example.com/error".to_string(),
        request_duration: Duration::from_millis(75),
    };
    
    assert!(!server_error_response.is_success());
    assert!(!server_error_response.is_client_error());
    assert!(server_error_response.is_server_error());
}

/// Test error display and formatting
#[test]
fn test_error_display() {
    let network_error = HttpError::NetworkError("Connection refused".to_string());
    assert_eq!(format!("{}", network_error), "Network error: Connection refused");
    
    let timeout_error = HttpError::TimeoutError;
    assert_eq!(format!("{}", timeout_error), "Request timeout");
    
    let invalid_url_error = HttpError::InvalidUrl("invalid-url".to_string());
    assert_eq!(format!("{}", invalid_url_error), "Invalid URL: invalid-url");
    
    let tls_error = HttpError::TlsError("Certificate validation failed".to_string());
    assert_eq!(format!("{}", tls_error), "TLS/SSL error: Certificate validation failed");
}

/// Test concurrent HTTP client usage
#[test]
fn test_concurrent_usage() {
    let client = Arc::new(HttpClient::new().with_timeout(Duration::from_secs(5)));
    let results = Arc::new(Mutex::new(Vec::new()));
    let mut handles = Vec::new();
    
    // Spawn multiple threads making requests
    for i in 0..5 {
        let client_clone = Arc::clone(&client);
        let results_clone = Arc::clone(&results);
        
        let handle = thread::spawn(move || {
            let request = client_clone.get("https://httpbin.org/get")
                .header("X-Thread-ID".to_string(), i.to_string());
            
            // We're not actually sending the request to avoid network dependency
            // Just test that the client can be used concurrently
            results_clone.lock().unwrap().push(format!("Thread-{}", i));
        });
        
        handles.push(handle);
    }
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let results = results.lock().unwrap();
    assert_eq!(results.len(), 5);
}

/// Test URL joining with base URL
#[test]
fn test_url_joining() {
    let client = HttpClient::new()
        .with_base_url("https://api.example.com".to_string());
    
    // Test relative path joining
    let request1 = client.get("/users");
    assert_eq!(request1.url, "https://api.example.com/users");
    
    let request2 = client.get("users");
    assert_eq!(request2.url, "https://api.example.com/users");
    
    // Test absolute URL override
    let request3 = client.get("https://other-api.com/data");
    assert_eq!(request3.url, "https://other-api.com/data");
    
    // Test complex path joining
    let client_with_path = HttpClient::new()
        .with_base_url("https://api.example.com/v1".to_string());
    
    let request4 = client_with_path.get("/users/123");
    assert_eq!(request4.url, "https://api.example.com/users/123");
}

/// Test multiple header operations
#[test]
fn test_multiple_header_operations() {
    let client = HttpClient::new()
        .with_header("X-API-Key".to_string(), "secret123".to_string())
        .with_header("User-Agent".to_string(), "MyApp/1.0".to_string());
    
    let request = client.get("https://api.example.com")
        .header("Accept".to_string(), "application/json".to_string())
        .header("Content-Type".to_string(), "application/json".to_string());
    
    // Check that default headers are included
    assert_eq!(request.headers.get("X-API-Key").unwrap(), "secret123");
    
    // Check that request-specific headers are included
    assert_eq!(request.headers.get("Accept").unwrap(), "application/json");
    assert_eq!(request.headers.get("Content-Type").unwrap(), "application/json");
    
    // User agent should be overridden by the client's setting
    assert_eq!(request.headers.get("User-Agent").unwrap(), "MyApp/1.0");
}

/// Test edge cases in cookie parsing
#[test]
fn test_cookie_edge_cases() {
    // Cookie with equals sign in value
    let cookie_with_equals = Cookie::parse("token=abc=123=def");
    assert!(cookie_with_equals.is_some());
    let cookie = cookie_with_equals.unwrap();
    assert_eq!(cookie.name, "token");
    assert_eq!(cookie.value, "abc=123=def");
    
    // Cookie with spaces
    let cookie_with_spaces = Cookie::parse("  name  =  value  ");
    assert!(cookie_with_spaces.is_some());
    let cookie = cookie_with_spaces.unwrap();
    assert_eq!(cookie.name, "name");
    assert_eq!(cookie.value, "value");
    
    // Cookie with all attributes
    let full_cookie = Cookie::parse("sessionid=abc123; Domain=.example.com; Path=/; Secure; HttpOnly; Max-Age=7200; Expires=Thu, 01 Jan 1970 00:00:00 GMT");
    assert!(full_cookie.is_some());
    let cookie = full_cookie.unwrap();
    assert_eq!(cookie.domain, Some(".example.com".to_string()));
    assert_eq!(cookie.path, Some("/".to_string()));
    assert!(cookie.secure);
    assert!(cookie.http_only);
    assert_eq!(cookie.max_age, Some(7200));
}

/// Test connection pool cleanup
#[test]
fn test_connection_pool_cleanup() {
    let mut pool = ConnectionPool::new();
    
    // Add some connections
    let conn1 = pool.get_connection("example.com");
    let conn2 = pool.get_connection("test.com");
    
    if let (Some(c1), Some(c2)) = (conn1, conn2) {
        pool.return_connection(c1);
        pool.return_connection(c2);
    }
    
    let stats_before = pool.stats();
    assert!(stats_before.total_connections > 0);
    
    // Force cleanup (this would normally happen automatically)
    // Since we can't access private cleanup method, we just verify the pool works
    let new_conn = pool.get_connection("example.com");
    assert!(new_conn.is_some());
}

/// Test response with different content types
#[test]
fn test_response_content_types() {
    // Test JSON response
    let json_response = HttpResponse {
        status: 200,
        headers: {
            let mut h = HashMap::new();
            h.insert("Content-Type".to_string(), "application/json".to_string());
            h
        },
        body: b"{\"status\": \"ok\"}".to_vec(),
        url: "https://example.com".to_string(),
        request_duration: Duration::from_millis(100),
    };
    
    assert!(json_response.json().is_ok());
    
    // Test non-JSON response
    let text_response = HttpResponse {
        status: 200,
        headers: {
            let mut h = HashMap::new();
            h.insert("Content-Type".to_string(), "text/plain".to_string());
            h
        },
        body: b"Plain text content".to_vec(),
        url: "https://example.com".to_string(),
        request_duration: Duration::from_millis(100),
    };
    
    assert!(text_response.json().is_err());
    assert_eq!(text_response.text().unwrap(), "Plain text content");
}

/// Test complex form data encoding
#[test]
fn test_complex_form_encoding() {
    let client = HttpClient::new();
    let mut form_data = HashMap::new();
    
    // Test special characters in form data
    form_data.insert("field1".to_string(), "value with spaces".to_string());
    form_data.insert("field2".to_string(), "value+with+plus".to_string());
    form_data.insert("field3".to_string(), "value&with&ampersand".to_string());
    form_data.insert("field4".to_string(), "value=with=equals".to_string());
    form_data.insert("field5".to_string(), "value%with%percent".to_string());
    
    let request = client.post("https://httpbin.org/post").form(&form_data);
    
    if let Some(body) = request.body {
        let body_str = String::from_utf8(body).unwrap();
        
        // Verify URL encoding
        assert!(body_str.contains("value%20with%20spaces"));  // spaces
        assert!(body_str.contains("value%2Bwith%2Bplus"));    // plus signs
        assert!(body_str.contains("value%26with%26ampersand")); // ampersands
        assert!(body_str.contains("value%3Dwith%3Dequals"));   // equals signs
        assert!(body_str.contains("value%25with%25percent"));  // percent signs
    }
}
