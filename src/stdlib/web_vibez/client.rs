/// HTTP client functionality for making requests
use std::collections::HashMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use reqwest;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;
use chrono::{DateTime, Utc, TimeZone};
use httpdate;
use url::Url;
use urlencoding;

/// Parse HTTP date format (RFC 7231)
/// Supports formats: RFC 1123, RFC 850, and ANSI C's asctime()
fn parse_http_date(date_str: &str) -> Option<u64> {
    // Try parsing with httpdate crate first (most efficient)
    if let Ok(system_time) = httpdate::parse_http_date(date_str) {
        return system_time.duration_since(UNIX_EPOCH).ok().map(|d| d.as_secs());
    }
    
    // Fall back to chrono parsing for additional formats
    let formats = [
        "%a, %d %b %Y %H:%M:%S GMT",         // RFC 1123: "Sun, 06 Nov 1994 08:49:37 GMT"
        "%A, %d-%b-%y %H:%M:%S GMT",         // RFC 850: "Sunday, 06-Nov-94 08:49:37 GMT"  
        "%a %b  %d %H:%M:%S %Y",             // asctime(): "Sun Nov  6 08:49:37 1994"
        "%a %b %d %H:%M:%S %Y",              // asctime() variant: "Sun Nov 6 08:49:37 1994"
        "%Y-%m-%d %H:%M:%S UTC",             // ISO-like format: "1994-11-06 08:49:37 UTC"
        "%Y-%m-%dT%H:%M:%SZ",                // ISO 8601: "1994-11-06T08:49:37Z"
        "%Y-%m-%dT%H:%M:%S%.fZ",             // ISO 8601 with fractional seconds
    ];
    
    for format in &formats {
        if let Ok(parsed) = DateTime::parse_from_str(date_str, format) {
            return Some(parsed.timestamp() as u64);
        }
        
        // Try parsing as UTC
        if let Ok(parsed) = chrono::NaiveDateTime::parse_from_str(date_str, format) {
            return Some(parsed.and_utc().timestamp() as u64);
        }
    }
    
    None
}



/// HTTP client for making requests
pub struct HttpClient {
    base_url: Option<String>,
    default_headers: HashMap<String, String>,
    timeout: Duration,
    follow_redirects: bool,
    max_redirects: usize,
    user_agent: String,
}

impl HttpClient {
    /// Create new HTTP client
    pub fn new() -> Self {
        Self {
            base_url: None,
            default_headers: HashMap::new(),
            timeout: Duration::from_secs(30),
            follow_redirects: true,
            max_redirects: 10,
            user_agent: "CURSED-WebVibez/1.0".to_string(),
        }
    }

    /// Set base URL for all requests
    pub fn with_base_url(mut self, base_url: String) -> Self {
        self.base_url = Some(base_url);
        self
    }

    /// Set default headers
    pub fn with_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.default_headers = headers;
        self
    }

    /// Add default header
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.default_headers.insert(key, value);
        self
    }

    /// Set request timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set redirect behavior
    pub fn with_redirects(mut self, follow: bool, max_redirects: usize) -> Self {
        self.follow_redirects = follow;
        self.max_redirects = max_redirects;
        self
    }

    /// Set user agent
    pub fn with_user_agent(mut self, user_agent: String) -> Self {
        self.user_agent = user_agent;
        self
    }

    /// Make GET request
    pub fn get(&self, url: &str) -> RequestBuilder {
        self.request("GET", url)
    }

    /// Make POST request
    pub fn post(&self, url: &str) -> RequestBuilder {
        self.request("POST", url)
    }

    /// Make PUT request
    pub fn put(&self, url: &str) -> RequestBuilder {
        self.request("PUT", url)
    }

    /// Make DELETE request
    pub fn delete(&self, url: &str) -> RequestBuilder {
        self.request("DELETE", url)
    }

    /// Make PATCH request
    pub fn patch(&self, url: &str) -> RequestBuilder {
        self.request("PATCH", url)
    }

    /// Make HEAD request
    pub fn head(&self, url: &str) -> RequestBuilder {
        self.request("HEAD", url)
    }

    /// Make OPTIONS request
    pub fn options(&self, url: &str) -> RequestBuilder {
        self.request("OPTIONS", url)
    }

    /// Create request builder
    pub fn request(&self, method: &str, url: &str) -> RequestBuilder {
        let full_url = if let Some(base_url) = &self.base_url {
            if url.starts_with("http://") || url.starts_with("https://") {
                url.to_string()
            } else {
                // Use proper URL joining
                if let Ok(base) = Url::parse(base_url) {
                    if let Ok(joined) = base.join(url) {
                        joined.to_string()
                    } else {
                        // Fallback to simple concatenation
                        format!("{}/{}", base_url.trim_end_matches('/'), url.trim_start_matches('/'))
                    }
                } else {
                    // Invalid base URL, use as-is
                    format!("{}/{}", base_url.trim_end_matches('/'), url.trim_start_matches('/'))
                }
            }
        } else {
            url.to_string()
        };

        let mut headers = self.default_headers.clone();
        headers.insert("User-Agent".to_string(), self.user_agent.clone());

        RequestBuilder {
            method: method.to_string(),
            url: full_url,
            headers,
            body: None,
            timeout: self.timeout,
            follow_redirects: self.follow_redirects,
            max_redirects: self.max_redirects,
        }
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

/// HTTP request builder
pub struct RequestBuilder {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
    timeout: Duration,
    follow_redirects: bool,
    max_redirects: usize,
}

impl RequestBuilder {
    /// Add header to request
    pub fn header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }

    /// Add multiple headers
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        for (key, value) in headers {
            self.headers.insert(key, value);
        }
        self
    }

    /// Set request body (raw bytes)
    pub fn body(mut self, body: Vec<u8>) -> Self {
        let body_len = body.len();
        self.body = Some(body);
        if !self.headers.contains_key("Content-Type") {
            self.headers.insert("Content-Type".to_string(), "application/octet-stream".to_string());
        }
        self.headers.insert("Content-Length".to_string(), body_len.to_string());
        self
    }

    /// Set JSON body
    pub fn json(mut self, json: &str) -> Self {
        self.body = Some(json.as_bytes().to_vec());
        self.headers.insert("Content-Type".to_string(), "application/json".to_string());
        self.headers.insert("Content-Length".to_string(), json.len().to_string());
        self
    }

    /// Set form data body
    pub fn form(mut self, form_data: &HashMap<String, String>) -> Self {
        let body = self.encode_form_data(form_data);
        self.body = Some(body.as_bytes().to_vec());
        self.headers.insert("Content-Type".to_string(), "application/x-www-form-urlencoded".to_string());
        self.headers.insert("Content-Length".to_string(), body.len().to_string());
        self
    }

    /// Set text body
    pub fn text(mut self, text: &str) -> Self {
        self.body = Some(text.as_bytes().to_vec());
        self.headers.insert("Content-Type".to_string(), "text/plain".to_string());
        self.headers.insert("Content-Length".to_string(), text.len().to_string());
        self
    }

    /// Set request timeout
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Set basic authentication
    pub fn basic_auth(mut self, username: &str, password: &str) -> Self {
        let credentials = format!("{}:{}", username, password);
        let encoded = base64_encode(credentials.as_bytes());
        self.headers.insert("Authorization".to_string(), format!("Basic {}", encoded));
        self
    }

    /// Set bearer token authentication
    pub fn bearer_token(mut self, token: &str) -> Self {
        self.headers.insert("Authorization".to_string(), format!("Bearer {}", token));
        self
    }

    /// Execute the request
    pub fn send(self) -> Result<HttpResponse, HttpError> {
        // Use async runtime to execute the HTTP request
        let runtime = tokio::runtime::Runtime::new()
            .map_err(|e| HttpError::Other(format!("Failed to create async runtime: {}", e)))?;
        
        runtime.block_on(self.send_async())
    }

    /// Execute the request asynchronously
    async fn send_async(self) -> Result<HttpResponse, HttpError> {
        // Validate URL
        Url::parse(&self.url)
            .map_err(|_| HttpError::InvalidUrl(self.url.clone()))?;
        
        let client = reqwest::Client::builder()
            .timeout(self.timeout)
            .redirect(if self.follow_redirects {
                reqwest::redirect::Policy::limited(self.max_redirects)
            } else {
                reqwest::redirect::Policy::none()
            })
            .build()
            .map_err(|e| HttpError::NetworkError(format!("Failed to build client: {}", e)))?;

        let method = reqwest::Method::from_str(&self.method)
            .map_err(|e| HttpError::Other(format!("Invalid HTTP method: {}", e)))?;

        let mut request_builder = client.request(method, &self.url);

        // Add headers
        let mut header_map = HeaderMap::new();
        for (key, value) in &self.headers {
            let header_name = HeaderName::from_str(key)
                .map_err(|_| HttpError::Other(format!("Invalid header name: {}", key)))?;
            let header_value = HeaderValue::from_str(value)
                .map_err(|_| HttpError::Other(format!("Invalid header value: {}", value)))?;
            header_map.insert(header_name, header_value);
        }
        request_builder = request_builder.headers(header_map);

        // Add body if present
        if let Some(body) = self.body {
            request_builder = request_builder.body(body);
        }

        // Send the request
        let start_time = SystemTime::now();
        let response = request_builder.send().await
            .map_err(|e| {
                if e.is_timeout() {
                    HttpError::TimeoutError
                } else if e.is_connect() {
                    HttpError::ConnectionError(format!("Connection failed: {}", e))
                } else if e.is_request() {
                    HttpError::RequestError(format!("Request failed: {}", e))
                } else if e.is_decode() {
                    HttpError::ResponseError(format!("Response decode failed: {}", e))
                } else if e.is_redirect() {
                    HttpError::TooManyRedirects
                } else if e.to_string().contains("tls") || e.to_string().contains("ssl") {
                    HttpError::TlsError(format!("TLS/SSL error: {}", e))
                } else {
                    HttpError::NetworkError(format!("Request failed: {}", e))
                }
            })?;

        let request_duration = start_time.elapsed().unwrap_or_default();
        let status = response.status().as_u16();
        let url = response.url().to_string();

        // Extract headers
        let mut response_headers = HashMap::new();
        for (name, value) in response.headers() {
            if let Ok(value_str) = value.to_str() {
                response_headers.insert(name.to_string(), value_str.to_string());
            }
        }

        // Get response body
        let body = response.bytes().await
            .map_err(|e| HttpError::ResponseError(format!("Failed to read response body: {}", e)))?
            .to_vec();

        Ok(HttpResponse {
            status,
            headers: response_headers,
            body,
            url,
            request_duration,
        })
    }

    /// Encode form data as URL-encoded string
    fn encode_form_data(&self, form_data: &HashMap<String, String>) -> String {
        form_data
            .iter()
            .map(|(key, value)| format!("{}={}", url_encode(key), url_encode(value)))
            .collect::<Vec<_>>()
            .join("&")
    }


}

/// HTTP response
#[derive(Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
    pub url: String,
    pub request_duration: Duration,
}

impl HttpResponse {
    /// Check if response is successful (2xx status)
    pub fn is_success(&self) -> bool {
        (200..300).contains(&self.status)
    }

    /// Check if response is client error (4xx status)
    pub fn is_client_error(&self) -> bool {
        (400..500).contains(&self.status)
    }

    /// Check if response is server error (5xx status)
    pub fn is_server_error(&self) -> bool {
        (500..600).contains(&self.status)
    }

    /// Get response body as string
    pub fn text(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.body.clone())
    }

    /// Get response body as bytes
    pub fn bytes(&self) -> &[u8] {
        &self.body
    }

    /// Get header value
    pub fn header(&self, name: &str) -> Option<&String> {
        self.headers.get(name)
    }

    /// Get content type
    pub fn content_type(&self) -> Option<&String> {
        self.header("Content-Type")
    }

    /// Get content length
    pub fn content_length(&self) -> Option<usize> {
        self.header("Content-Length")
            .and_then(|s| s.parse().ok())
    }

    /// Parse JSON response (simplified)
    pub fn json(&self) -> Result<String, HttpError> {
        // In a real implementation, this would parse JSON using a proper parser
        // For now, just return the text if it looks like JSON
        let text = self.text().map_err(|_| HttpError::InvalidResponse)?;
        
        if text.trim().starts_with('{') || text.trim().starts_with('[') {
            Ok(text)
        } else {
            Err(HttpError::InvalidJson)
        }
    }

    /// Check if response has specific header
    pub fn has_header(&self, name: &str) -> bool {
        self.headers.contains_key(name)
    }

    /// Get all header names
    pub fn header_names(&self) -> Vec<&String> {
        self.headers.keys().collect()
    }

    /// Get response cookies (simplified)
    pub fn cookies(&self) -> Vec<Cookie> {
        let mut cookies = Vec::new();
        
        for (name, value) in &self.headers {
            if name.eq_ignore_ascii_case("set-cookie") {
                if let Some(cookie) = Cookie::parse(value) {
                    cookies.push(cookie);
                }
            }
        }
        
        cookies
    }
}

/// HTTP errors
#[derive(Debug)]
pub enum HttpError {
    NetworkError(String),
    TimeoutError,
    InvalidUrl(String),
    InvalidResponse,
    InvalidJson,
    TooManyRedirects,
    AuthenticationFailed,
    TlsError(String),
    ConnectionError(String),
    RequestError(String),
    ResponseError(String),
    Other(String),
}

impl std::fmt::Display for HttpError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            HttpError::NetworkError(msg) => write!(f, "Network error: {}", msg),
            HttpError::TimeoutError => write!(f, "Request timeout"),
            HttpError::InvalidUrl(url) => write!(f, "Invalid URL: {}", url),
            HttpError::InvalidResponse => write!(f, "Invalid response"),
            HttpError::InvalidJson => write!(f, "Invalid JSON response"),
            HttpError::TooManyRedirects => write!(f, "Too many redirects"),
            HttpError::AuthenticationFailed => write!(f, "Authentication failed"),
            HttpError::TlsError(msg) => write!(f, "TLS/SSL error: {}", msg),
            HttpError::ConnectionError(msg) => write!(f, "Connection error: {}", msg),
            HttpError::RequestError(msg) => write!(f, "Request error: {}", msg),
            HttpError::ResponseError(msg) => write!(f, "Response error: {}", msg),
            HttpError::Other(msg) => write!(f, "HTTP error: {}", msg),
        }
    }
}

impl std::error::Error for HttpError {}

/// Cookie structure
#[derive(Debug, Clone)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub secure: bool,
    pub http_only: bool,
    pub expires: Option<u64>,
    pub max_age: Option<u64>,
}

impl Cookie {
    /// Parse cookie from Set-Cookie header value
    pub fn parse(cookie_str: &str) -> Option<Self> {
        let parts: Vec<&str> = cookie_str.split(';').collect();
        if parts.is_empty() {
            return None;
        }

        // Parse name=value
        let name_value: Vec<&str> = parts[0].trim().splitn(2, '=').collect();
        if name_value.len() != 2 {
            return None;
        }

        let mut cookie = Cookie {
            name: name_value[0].trim().to_string(),
            value: name_value[1].trim().to_string(),
            domain: None,
            path: None,
            secure: false,
            http_only: false,
            expires: None,
            max_age: None,
        };

        // Parse attributes
        for part in &parts[1..] {
            let part = part.trim();
            if part.eq_ignore_ascii_case("secure") {
                cookie.secure = true;
            } else if part.eq_ignore_ascii_case("httponly") {
                cookie.http_only = true;
            } else if let Some((key, value)) = part.split_once('=') {
                match key.trim().to_lowercase().as_str() {
                    "domain" => cookie.domain = Some(value.trim().to_string()),
                    "path" => cookie.path = Some(value.trim().to_string()),
                    "max-age" => cookie.max_age = value.trim().parse().ok(),
                    "expires" => {
                        cookie.expires = parse_http_date(value.trim());
                    }
                    _ => {}
                }
            }
        }

        Some(cookie)
    }

    /// Convert cookie to string for Cookie header
    pub fn to_header_value(&self) -> String {
        format!("{}={}", self.name, self.value)
    }
}

/// Connection pool for reusing connections
pub struct ConnectionPool {
    connections: HashMap<String, Vec<Connection>>,
    max_connections_per_host: usize,
    connection_timeout: Duration,
    idle_timeout: Duration,
}

#[derive(Debug)]
struct Connection {
    host: String,
    created_at: SystemTime,
    last_used: SystemTime,
    is_active: bool,
}

impl ConnectionPool {
    /// Create new connection pool
    pub fn new() -> Self {
        Self {
            connections: HashMap::new(),
            max_connections_per_host: 10,
            connection_timeout: Duration::from_secs(30),
            idle_timeout: Duration::from_secs(300), // 5 minutes
        }
    }

    /// Set maximum connections per host
    pub fn with_max_connections(mut self, max: usize) -> Self {
        self.max_connections_per_host = max;
        self
    }

    /// Get connection for host
    pub fn get_connection(&mut self, host: &str) -> Option<Connection> {
        self.cleanup_idle_connections();
        
        if let Some(connections) = self.connections.get_mut(host) {
            if let Some(connection) = connections.pop() {
                return Some(connection);
            }
        }

        // Create new connection if none available
        Some(Connection {
            host: host.to_string(),
            created_at: SystemTime::now(),
            last_used: SystemTime::now(),
            is_active: true,
        })
    }

    /// Return connection to pool
    pub fn return_connection(&mut self, mut connection: Connection) {
        connection.last_used = SystemTime::now();
        connection.is_active = false;

        let host_connections = self.connections.entry(connection.host.clone()).or_insert_with(Vec::new);
        
        if host_connections.len() < self.max_connections_per_host {
            host_connections.push(connection);
        }
    }

    /// Cleanup idle connections
    fn cleanup_idle_connections(&mut self) {
        let now = SystemTime::now();
        
        for connections in self.connections.values_mut() {
            connections.retain(|conn| {
                now.duration_since(conn.last_used).unwrap_or_default() < self.idle_timeout
            });
        }
        
        self.connections.retain(|_, connections| !connections.is_empty());
    }

    /// Get pool statistics
    pub fn stats(&self) -> PoolStats {
        let total_connections: usize = self.connections.values().map(|v| v.len()).sum();
        let total_hosts = self.connections.len();

        PoolStats {
            total_connections,
            total_hosts,
            max_connections_per_host: self.max_connections_per_host,
            idle_timeout_seconds: self.idle_timeout.as_secs(),
        }
    }
}

impl Default for ConnectionPool {
    fn default() -> Self {
        Self::new()
    }
}

/// Connection pool statistics
#[derive(Debug)]
pub struct PoolStats {
    pub total_connections: usize,
    pub total_hosts: usize,
    pub max_connections_per_host: usize,
    pub idle_timeout_seconds: u64,
}

/// URL encoding using the urlencoding crate
fn url_encode(input: &str) -> String {
    urlencoding::encode(input).to_string()
}

/// Simple base64 encoding
fn base64_encode(input: &[u8]) -> String {
    const CHARS: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut result = String::new();
    
    for chunk in input.chunks(3) {
        let mut buf = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }
        
        let b = ((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[2] as u32);
        
        result.push(CHARS[((b >> 18) & 0x3F) as usize] as char);
        result.push(CHARS[((b >> 12) & 0x3F) as usize] as char);
        result.push(if chunk.len() > 1 { CHARS[((b >> 6) & 0x3F) as usize] as char } else { '=' });
        result.push(if chunk.len() > 2 { CHARS[(b & 0x3F) as usize] as char } else { '=' });
    }
    
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_client_creation() {
        let client = HttpClient::new()
            .with_base_url("https://api.example.com".to_string())
            .with_timeout(Duration::from_secs(60))
            .with_user_agent("Test Client".to_string());

        let request = client.get("/users");
        assert_eq!(request.url, "https://api.example.com/users");
    }

    #[test]
    fn test_request_builder() {
        let client = HttpClient::new();
        let request = client.post("/api/data")
            .header("Content-Type".to_string(), "application/json".to_string())
            .json(r#"{"test": "data"}"#);

        assert_eq!(request.method, "POST");
        assert!(request.headers.contains_key("Content-Type"));
    }

    #[test]
    fn test_form_encoding() {
        let client = HttpClient::new();
        let mut form_data = HashMap::new();
        form_data.insert("name".to_string(), "John Doe".to_string());
        form_data.insert("email".to_string(), "john@example.com".to_string());

        let request = client.post("/submit").form(&form_data);
        
        if let Some(body) = request.body {
            let body_str = String::from_utf8(body).unwrap();
            assert!(body_str.contains("name=John%20Doe"));
            assert!(body_str.contains("email=john%40example.com"));
        }
    }

    #[test]
    fn test_authentication() {
        let client = HttpClient::new();
        
        // Test basic auth
        let basic_request = client.get("/protected")
            .basic_auth("user", "pass");
        assert!(basic_request.headers.get("Authorization").unwrap().starts_with("Basic"));

        // Test bearer token
        let bearer_request = client.get("/api")
            .bearer_token("abc123");
        assert_eq!(
            bearer_request.headers.get("Authorization").unwrap(),
            "Bearer abc123"
        );
    }

    #[test]
    fn test_cookie_parsing() {
        let cookie_str = "sessionid=abc123; Domain=example.com; Path=/; Secure; HttpOnly; Max-Age=3600";
        let cookie = Cookie::parse(cookie_str).unwrap();

        assert_eq!(cookie.name, "sessionid");
        assert_eq!(cookie.value, "abc123");
        assert_eq!(cookie.domain, Some("example.com".to_string()));
        assert_eq!(cookie.path, Some("/".to_string()));
        assert!(cookie.secure);
        assert!(cookie.http_only);
        assert_eq!(cookie.max_age, Some(3600));
    }

    #[test]
    fn test_connection_pool() {
        let mut pool = ConnectionPool::new().with_max_connections(5);

        // Get connection
        let conn1 = pool.get_connection("example.com");
        assert!(conn1.is_some());

        // Return connection
        if let Some(conn) = conn1 {
            pool.return_connection(conn);
        }

        // Get connection again (should reuse)
        let conn2 = pool.get_connection("example.com");
        assert!(conn2.is_some());

        let stats = pool.stats();
        assert_eq!(stats.max_connections_per_host, 5);
    }

    #[test]
    fn test_url_encoding() {
        assert_eq!(url_encode("hello world"), "hello%20world");
        assert_eq!(url_encode("test@example.com"), "test%40example.com");
        assert_eq!(url_encode("safe-string_123"), "safe-string_123");
    }

    #[test]
    fn test_base64_encoding() {
        assert_eq!(base64_encode(b"hello"), "aGVsbG8=");
        assert_eq!(base64_encode(b"hello world"), "aGVsbG8gd29ybGQ=");
        assert_eq!(base64_encode(b"user:pass"), "dXNlcjpwYXNz");
    }

    #[test]
    fn test_request_validation() {
        let client = HttpClient::new();
        
        // Test URL validation - invalid URLs should be caught before network calls
        let invalid_request = client.get("not-a-valid-url").send();
        assert!(invalid_request.is_err());
        
        // Test request building with invalid header names
        let mut headers = HashMap::new();
        headers.insert("Invalid\nHeader".to_string(), "value".to_string());
        
        let invalid_header_request = client.get("https://httpbin.org/get")
            .headers(headers)
            .send();
        assert!(invalid_header_request.is_err());
        
        // Test that valid request building works (we won't send it)
        let valid_request_builder = client.get("https://httpbin.org/get")
            .header("Content-Type".to_string(), "application/json".to_string())
            .json(r#"{"test": "data"}"#);
        
        // Verify the request builder has correct properties
        assert_eq!(valid_request_builder.method, "GET");
        assert!(valid_request_builder.url.contains("httpbin.org"));
        assert!(valid_request_builder.headers.contains_key("Content-Type"));
    }
}
