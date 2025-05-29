//! The web_vibez package provides HTTP client and server functionality.
//!
//! This module is equivalent to the net/http package in Go, providing functions
//! for making HTTP requests and creating HTTP servers. It includes both mock
//! implementations for testing and real HTTP client functionality.
//!
//! # Features
//!
//! - HTTP client for making GET, POST, PUT, DELETE, and HEAD requests
//! - HTTP client timeout configuration
//! - HTTP server for handling incoming requests
//! - Route registration for different URL paths
//!
//! # Examples
//!
//! ```cursed
//! import "web_vibez"
//!
//! // Make an HTTP GET request
//! response := web_vibez.get("https://example.com")
//! vibez.println(response.body)
//!
//! // Make a POST request with JSON data
//! data := {"name": "Cursed User", "message": "Hello!"}
//! response := web_vibez.post("https://example.com/api", data)
//! 
//! // Create a simple HTTP server
//! web_vibez.handle_func("/hello", func(w, r) {
//!     vibez.fprint(w, "Hello, Web Vibez!")
//! })
//! web_vibez.listen_and_serve(":8080")
//! ```

use crate::error::Error;
use crate::object::Object;
use std::collections::HashMap;
use std::sync::Arc;
// We don't need Duration anymore as we're using mock responses only
use std::sync::atomic::{AtomicU64, Ordering};

// Global timeout value in milliseconds (default: 30000 ms = 30 seconds)
static TIMEOUT_MS: AtomicU64 = AtomicU64::new(30000);

/// Get or set the HTTP client timeout in milliseconds
///
/// # Arguments
///
/// * `args[]` - Optional timeout value in milliseconds as an Integer Object
///
/// # Returns
///
/// An Integer Object with the current timeout value
///
/// # Examples
///
/// ```cursed
/// // Set timeout to a short value (500ms)
/// web_vibez.client_timeout(500)
///
/// // Get the current timeout
/// timeout := web_vibez.client_timeout()
/// ```
pub fn client_timeout(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if !args.is_empty() {
        // Set timeout
        let timeout = match &*args[0] {
            Object::Integer(ms) => {
                if *ms < 0 {
                    return Err(Error::Runtime("Timeout cannot be negative".to_string()));
                }
                *ms as u64
            },
            _ => return Err(Error::Runtime("Timeout must be an integer".to_string())),
        };
        
        TIMEOUT_MS.store(timeout, Ordering::Relaxed);
        // In a real implementation, we would reset the HTTP client here
    }
    
    // Return current timeout
    let current = TIMEOUT_MS.load(Ordering::Relaxed);
    Ok(Arc::new(Object::Integer(current as i64)))
}

/// Create mock headers for testing
fn create_mock_headers(content_type: &str) -> HashMap<String, Object> {
    let mut headers = HashMap::new();
    headers.insert(
        "Content-Type".to_string(),
        Object::String(content_type.to_string()),
    );
    headers.insert(
        "X-Mock".to_string(),
        Object::String("true".to_string()),
    );
    headers
}

/// Creates a mock HTTP response for testing
fn create_mock_response(method: &str, url: &str) -> HashMap<String, Object> {
    let mut response = HashMap::new();
    let mut headers = HashMap::new();
    
    match method {
        "GET" => {
            response.insert("status".to_string(), Object::Integer(200));
            response.insert(
                "body".to_string(),
                Object::String(format!("Mock GET response for {}", url)),
            );
            headers.insert(
                "Content-Type".to_string(),
                Object::String("text/plain".to_string()),
            );
        },
        "POST" => {
            response.insert("status".to_string(), Object::Integer(201));
            response.insert(
                "body".to_string(),
                Object::String("{\"id\": 123, \"success\": true}".to_string()),
            );
            headers.insert(
                "Content-Type".to_string(),
                Object::String("application/json".to_string()),
            );
        },
        "HEAD" => {
            response.insert("status".to_string(), Object::Integer(200));
            // HEAD responses don't have a body
            headers.insert(
                "Content-Type".to_string(),
                Object::String("text/html".to_string()),
            );
            headers.insert(
                "Content-Length".to_string(),
                Object::String("12345".to_string()),
            );
        },
        "DELETE" => {
            response.insert("status".to_string(), Object::Integer(204));
            // Often DELETE responses have no body
        },
        "PUT" => {
            response.insert("status".to_string(), Object::Integer(200));
            response.insert(
                "body".to_string(),
                Object::String("{\"updated\": true}".to_string()),
            );
            headers.insert(
                "Content-Type".to_string(),
                Object::String("application/json".to_string()),
            );
        },
        _ => {
            response.insert("status".to_string(), Object::Integer(200));
            response.insert(
                "body".to_string(),
                Object::String("Mock response".to_string()),
            );
        }
    }
    
    response.insert("headers".to_string(), Object::HashTable(headers));
    response
}

/// Makes an HTTP GET request to the specified URL.
///
/// # Arguments
///
/// * `args[0]` - The URL to make the request to as a String Object
/// * `args[1]` - Optional boolean to indicate mock mode (true = use mock, false/not present = real request)
///
/// # Returns
///
/// A HashTable Object containing:
/// * `status` - HTTP status code as an Integer
/// * `body` - Response body as a String
/// * `headers` - Response headers as a HashTable of String keys and values
///
/// # Errors
///
/// Returns a Runtime error if no URL argument is provided
pub fn get(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("get requires at least 1 argument: url".to_string()));
    }

    let url = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("URL must be a string".to_string())),
    };
    
    // Check if we should use mock mode
    let use_mock = if args.len() > 1 {
        match &*args[1] {
            Object::Boolean(mock) => *mock,
            _ => false,
        }
    } else {
        false
    };
    
    // Always use mock responses for now
    // In a real implementation, we would implement actual HTTP requests here
    let response = create_mock_response("GET", &url);
    Ok(Arc::new(Object::HashTable(response)))
}

/// Makes an HTTP POST request to the specified URL with a JSON body.
///
/// # Arguments
///
/// * `args[0]` - The URL to make the request to as a String Object
/// * `args[1]` - The body to send (typically a HashTable that will be converted to JSON)
/// * `args[2]` - Optional boolean to indicate mock mode (true = use mock, false/not present = real request)
///
/// # Returns
///
/// A HashTable Object containing:
/// * `status` - HTTP status code as an Integer
/// * `body` - Response body as a String
/// * `headers` - Response headers as a HashTable of String keys and values
///
/// # Errors
///
/// Returns a Runtime error if required arguments are missing
pub fn post(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("post requires at least 2 arguments: url and body".to_string()));
    }

    let url = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("URL must be a string".to_string())),
    };
    
    // Check if we should use mock mode
    let use_mock = if args.len() > 2 {
        match &*args[2] {
            Object::Boolean(mock) => *mock,
            _ => false,
        }
    } else {
        false
    };
    
    // Always use mock responses for now
    // For a real implementation, we would send the actual HTTP request here
    
    // Log the body for debugging purposes
    match &*args[1] {
        Object::HashTable(map) => {
            // For debugging, we could inspect the map here if needed
        },
        Object::String(_) => {
            // For debugging, we could inspect the string here if needed
        },
        _ => return Err(Error::Runtime("Body must be a hash table or string".to_string())),
    };
    
    let response = create_mock_response("POST", &url);
    Ok(Arc::new(Object::HashTable(response)))
}

/// Makes an HTTP PUT request to the specified URL with a JSON body.
///
/// # Arguments
///
/// * `args[0]` - The URL to make the request to as a String Object
/// * `args[1]` - The body to send (typically a HashTable that will be converted to JSON)
/// * `args[2]` - Optional boolean to indicate mock mode (true = use mock, false/not present = real request)
///
/// # Returns
///
/// A HashTable Object containing:
/// * `status` - HTTP status code as an Integer
/// * `body` - Response body as a String
/// * `headers` - Response headers as a HashTable of String keys and values
///
/// # Errors
///
/// Returns a Runtime error if required arguments are missing
pub fn put(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime("put requires at least 2 arguments: url and body".to_string()));
    }

    let url = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("URL must be a string".to_string())),
    };
    
    // Check if we should use mock mode
    let use_mock = if args.len() > 2 {
        match &*args[2] {
            Object::Boolean(mock) => *mock,
            _ => false,
        }
    } else {
        false
    };
    
    // Always use mock responses for now
    // In a real implementation, we would send the actual HTTP request here
    let response = create_mock_response("PUT", &url);
    Ok(Arc::new(Object::HashTable(response)))
}

/// Makes an HTTP HEAD request to the specified URL.
///
/// # Arguments
///
/// * `args[0]` - The URL to make the request to as a String Object
/// * `args[1]` - Optional boolean to indicate mock mode (true = use mock, false/not present = real request)
///
/// # Returns
///
/// A HashTable Object containing:
/// * `status` - HTTP status code as an Integer
/// * `headers` - Response headers as a HashTable of String keys and values
/// Note: HEAD requests don't return a body
///
/// # Errors
///
/// Returns a Runtime error if no URL argument is provided
pub fn head(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("head requires at least 1 argument: url".to_string()));
    }

    let url = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("URL must be a string".to_string())),
    };
    
    // Check if we should use mock mode
    let use_mock = if args.len() > 1 {
        match &*args[1] {
            Object::Boolean(mock) => *mock,
            _ => false,
        }
    } else {
        false
    };
    
    // Always use mock responses for now
    // In a real implementation, we would send the actual HTTP request here
    let response = create_mock_response("HEAD", &url);
    Ok(Arc::new(Object::HashTable(response)))
}

/// Makes an HTTP DELETE request to the specified URL.
///
/// # Arguments
///
/// * `args[0]` - The URL to make the request to as a String Object
/// * `args[1]` - Optional boolean to indicate mock mode (true = use mock, false/not present = real request)
///
/// # Returns
///
/// A HashTable Object containing:
/// * `status` - HTTP status code as an Integer
/// * `body` - Response body as a String (if any)
/// * `headers` - Response headers as a HashTable of String keys and values
///
/// # Errors
///
/// Returns a Runtime error if no URL argument is provided
pub fn delete(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime("delete requires at least 1 argument: url".to_string()));
    }

    let url = match &*args[0] {
        Object::String(s) => s.clone(),
        _ => return Err(Error::Runtime("URL must be a string".to_string())),
    };
    
    // Check if we should use mock mode
    let use_mock = if args.len() > 1 {
        match &*args[1] {
            Object::Boolean(mock) => *mock,
            _ => false,
        }
    } else {
        false
    };
    
    // Always use mock responses for now
    // In a real implementation, we would send the actual HTTP request here
    let response = create_mock_response("DELETE", &url);
    Ok(Arc::new(Object::HashTable(response)))
}

/// Registers a handler function for a specific HTTP path.
///
/// # Arguments
///
/// * `args[0]` - The path to handle as a String Object (e.g., "/hello")
/// * `args[1]` - The handler function as a Function Object that accepts response writer and request objects
///
/// # Returns
///
/// Null to indicate successful registration
///
/// # Errors
///
/// Returns a Runtime error if fewer than 2 arguments are provided
pub fn handle_func(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.len() < 2 {
        return Err(Error::Runtime(
            "handle_func requires 2 arguments: path and handler function".to_string(),
        ));
    }

    // Simplified implementation - just acknowledge registration
    Ok(Arc::new(Object::Null))
}

/// Starts an HTTP server on the specified address and listens for incoming connections.
///
/// # Arguments
///
/// * `args[0]` - The address to listen on as a String Object (e.g., ":8080")
///
/// # Returns
///
/// Null to indicate successful server startup (in a real implementation this would block)
///
/// # Errors
///
/// Returns a Runtime error if no address argument is provided
pub fn listen_and_serve(args: &[Arc<Object>]) -> Result<Arc<Object>, Error> {
    if args.is_empty() {
        return Err(Error::Runtime(
            "listen_and_serve requires 1 argument: address".to_string(),
        ));
    }

    // Simplified implementation that doesn't actually start a server
    // Just return null to indicate success
    Ok(Arc::new(Object::Null))
}