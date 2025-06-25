//! Sample package demonstrating comprehensive documentation features
//! 
//! This package provides examples of well-documented CURSED code including
//! functions, structs, interfaces, and proper documentation formatting.
//! 
//! # Examples
//! 
//! ```cursed
//! facts client = new HttpClient("https://api.example.com")
//! facts response = client.get("/users")
//! ```

/// HTTP client for making web requests
/// 
/// Provides a high-level interface for HTTP operations with automatic
/// error handling and response parsing.
/// 
/// # Examples
/// 
/// ```cursed
/// facts client = new HttpClient("https://api.example.com")
/// facts response = client.get("/users")
/// lowkey response.status == 200 {
///     vibe_check response.data {
///         mood User[] { parse_users(response.data) }
///         basic { [] }
///     }
/// }
/// ```
squad HttpClient {
    /// Base URL for all requests
    base_url: String,
    /// Request timeout in seconds  
    timeout: Int,
    /// HTTP headers to include with requests
    headers: Map[String, String],
}

/// Create a new HTTP client with the specified base URL
/// 
/// # Arguments
/// * `base_url` - The base URL for all HTTP requests
/// * `timeout` - Optional timeout in seconds (default: 30)
/// 
/// # Returns
/// A new HttpClient instance configured with the provided settings
/// 
/// # Examples
/// 
/// ```cursed
/// facts client = new HttpClient("https://api.example.com")
/// facts client_with_timeout = new HttpClient("https://api.example.com", 60)
/// ```
yolo new HttpClient(base_url: String, timeout: Int = 30) -> HttpClient {
    HttpClient {
        base_url: base_url,
        timeout: timeout,
        headers: new Map[String, String](),
    }
}

/// Perform GET request to the specified endpoint
/// 
/// # Arguments
/// * `endpoint` - The API endpoint to request (relative to base_url)
/// 
/// # Returns
/// HttpResponse containing the server response data
/// 
/// # Errors
/// Returns HttpError if the request fails or times out
yolo slay get(self, endpoint: String) -> HttpResponse {
    // Implementation details...
    HttpResponse { status: 200, data: "mock response", headers: new Map[String, String]() }
}

/// HTTP response containing server data
/// 
/// Represents the complete response from an HTTP request including
/// status code, headers, and response body.
squad HttpResponse {
    /// HTTP status code (200, 404, 500, etc.)
    status: Int,
    /// Response body as string
    data: String,
    /// Response headers
    headers: Map[String, String],
}

/// Interface for objects that can be serialized to HTTP requests
/// 
/// Implement this interface for custom types that need to be sent
/// in HTTP request bodies.
collab HttpSerializable {
    /// Convert object to JSON string for HTTP transmission
    /// 
    /// # Returns
    /// JSON representation of the object
    /// 
    /// # Errors
    /// Returns SerializationError if object cannot be serialized
    yolo to_json(self) -> String
}

/// User data structure for API responses
/// 
/// Represents user information returned from user management APIs.
squad User {
    /// Unique user identifier
    id: Int,
    /// User's display name
    name: String,
    /// User's email address
    email: String,
    /// User creation timestamp
    created_at: String,
}

/// Parse user data from JSON response
/// 
/// # Arguments
/// * `json_data` - Raw JSON string from API response
/// 
/// # Returns
/// Array of User objects parsed from the JSON data
/// 
/// # Errors
/// Returns ParseError if JSON is malformed or missing required fields
yolo parse_users(json_data: String) -> User[] {
    // JSON parsing implementation...
    []
}

/// Error types for HTTP operations
/// 
/// Comprehensive error handling for all HTTP-related failures.
squad HttpError {
    /// Error message describing the failure
    message: String,
    /// HTTP status code if available
    status_code: Int?,
    /// Whether the error is retryable
    retryable: Bool,
}
