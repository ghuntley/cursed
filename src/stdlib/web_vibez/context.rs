use crate::web::StatusCode;
/// Request and response context for HTTP request processing
/// 
/// Provides thread-safe context passing through middleware chains
/// with support for request parameters, headers, body, and metadata

use crate::stdlib::web_vibez::{HttpMethod, StatusCode};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};
use std::time::{Duration, Instant};
use tracing::{debug, instrument};

/// Data that can be stored in context
#[derive(Debug, Clone)]
pub enum ContextData {
    String(String),
    Integer(i64),
    Float(f64),
    Boolean(bool),
    Bytes(Vec<u8>),
    Map(HashMap<String, ContextData>),
    List(Vec<ContextData>),
}

impl ContextData {
    /// Get as string if possible
    pub fn as_string(&self) -> Option<&str> {
        match self {
            ContextData::String(s) => Some(s),
            _ => None,
        }
    }

    /// Get as integer if possible
    pub fn as_integer(&self) -> Option<i64> {
        match self {
            ContextData::Integer(i) => Some(*i),
            _ => None,
        }
    }

    /// Get as boolean if possible
    pub fn as_boolean(&self) -> Option<bool> {
        match self {
            ContextData::Boolean(b) => Some(*b),
            _ => None,
        }
    }

    /// Get as bytes if possible
    pub fn as_bytes(&self) -> Option<&[u8]> {
        match self {
            ContextData::Bytes(b) => Some(b),
            _ => None,
        }
    }
}

impl From<String> for ContextData {
    fn from(s: String) -> Self {
        ContextData::String(s)
    }
}

impl From<&str> for ContextData {
    fn from(s: &str) -> Self {
        ContextData::String(s.to_string())
    }
}

impl From<i64> for ContextData {
    fn from(i: i64) -> Self {
        ContextData::Integer(i)
    }
}

impl From<bool> for ContextData {
    fn from(b: bool) -> Self {
        ContextData::Boolean(b)
    }
}

impl From<Vec<u8>> for ContextData {
    fn from(b: Vec<u8>) -> Self {
        ContextData::Bytes(b)
    }
}

/// HTTP request context containing all request information
#[derive(Debug, Clone)]
pub struct RequestContext {
    /// HTTP method
    pub method: HttpMethod,
    /// Request path
    pub path: String,
    /// Query string parameters
    pub query_params: HashMap<String, String>,
    /// Route parameters (extracted from path)
    pub route_params: HashMap<String, String>,
    /// Request headers
    pub headers: HashMap<String, String>,
    /// Request body
    pub body: Vec<u8>,
    /// Custom context data
    pub data: Arc<RwLock<HashMap<String, ContextData>>>,
    /// Request ID for tracing
    pub request_id: String,
    /// Request start time
    pub start_time: Instant,
    /// Client IP address
    pub client_ip: Option<String>,
    /// User agent
    pub user_agent: Option<String>,
    /// Request timeout
    pub timeout: Option<Duration>,
}

impl RequestContext {
    /// Create a new request context
    #[instrument(skip(method, path))]
    pub fn new(method: String, path: String) -> Self {
        let parsed_method = method.parse().unwrap_or(HttpMethod::GET);
        let request_id = Self::generate_request_id();
        
        debug!(method = %method, path = %path, request_id = %request_id, "Creating request context");
        
        Self {
            method: parsed_method,
            path,
            query_params: HashMap::new(),
            route_params: HashMap::new(),
            headers: HashMap::new(),
            body: Vec::new(),
            data: Arc::new(RwLock::new(HashMap::new())),
            request_id,
            start_time: Instant::now(),
            client_ip: None,
            user_agent: None,
            timeout: None,
        }
    }

    /// Generate a unique request ID
    fn generate_request_id() -> String {
        use std::sync::atomic::{AtomicU64, Ordering};
        static REQUEST_COUNTER: AtomicU64 = AtomicU64::new(0);
        
        let count = REQUEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        let timestamp = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis();
        
        format!("req_{}_{}", timestamp, count)
    }

    /// Add a route parameter
    pub fn add_param(&mut self, key: &str, value: &str) {
        self.route_params.insert(key.to_string(), value.to_string());
    }

    /// Get a route parameter
    pub fn param(&self, key: &str) -> Option<&str> {
        self.route_params.get(key).map(|s| s.as_str())
    }

    /// Add a query parameter
    pub fn add_query_param(&mut self, key: &str, value: &str) {
        self.query_params.insert(key.to_string(), value.to_string());
    }

    /// Get a query parameter
    pub fn query_param(&self, key: &str) -> Option<&str> {
        self.query_params.get(key).map(|s| s.as_str())
    }

    /// Add a header
    pub fn add_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_lowercase(), value.to_string());
    }

    /// Get a header
    pub fn header(&self, key: &str) -> Option<&str> {
        self.headers.get(&key.to_lowercase()).map(|s| s.as_str())
    }

    /// Set request body
    pub fn set_body(&mut self, body: Vec<u8>) {
        self.body = body;
    }

    /// Get request body as bytes
    pub fn body(&self) -> &[u8] {
        &self.body
    }

    /// Get request body as string
    pub fn body_string(&self) -> Result<(), Error> {
        String::from_utf8(self.body.clone())
    }

    /// Set custom context data
    #[instrument(skip(self, value))]
    pub fn set_data(&self, key: &str, value: ContextData) {
        if let Ok(mut data) = self.data.write() {
            data.insert(key.to_string(), value);
            debug!(key = %key, "Set context data");
        }
    }

    /// Get custom context data
    pub fn get_data(&self, key: &str) -> Option<ContextData> {
        if let Ok(data) = self.data.read() {
            data.get(key).cloned()
        } else {
            None
        }
    }

    /// Check if context has data for key
    pub fn has_data(&self, key: &str) -> bool {
        if let Ok(data) = self.data.read() {
            data.contains_key(key)
        } else {
            false
        }
    }

    /// Set client IP address
    pub fn set_client_ip(&mut self, ip: String) {
        self.client_ip = Some(ip);
    }

    /// Set user agent
    pub fn set_user_agent(&mut self, user_agent: String) {
        self.user_agent = Some(user_agent);
    }

    /// Set request timeout
    pub fn set_timeout(&mut self, timeout: Duration) {
        self.timeout = Some(timeout);
    }

    /// Get elapsed time since request start
    pub fn elapsed(&self) -> Duration {
        self.start_time.elapsed()
    }

    /// Check if request has timed out
    pub fn is_timed_out(&self) -> bool {
        if let Some(timeout) = self.timeout {
            self.elapsed() > timeout
        } else {
            false
        }
    }

    /// Get content type from headers
    pub fn content_type(&self) -> Option<&str> {
        self.header("content-type")
    }

    /// Check if request is JSON
    pub fn is_json(&self) -> bool {
        if let Some(content_type) = self.content_type() {
            content_type.contains("application/json")
        } else {
            false
        }
    }

    /// Check if request is form data
    pub fn is_form(&self) -> bool {
        if let Some(content_type) = self.content_type() {
            content_type.contains("application/x-www-form-urlencoded") ||
            content_type.contains("multipart/form-data")
        } else {
            false
        }
    }

    /// Parse JSON body
    pub fn json<T>(&self) -> Result<(), Error>
    where
        T: serde::de::DeserializeOwned,
    {
        serde_json::from_slice(&self.body)
    }

    /// Clone context for middleware chain passing
    pub fn clone_for_middleware(&self) -> Self {
        Self {
            method: self.method,
            path: self.path.clone(),
            query_params: self.query_params.clone(),
            route_params: self.route_params.clone(),
            headers: self.headers.clone(),
            body: self.body.clone(),
            data: Arc::clone(&self.data),
            request_id: self.request_id.clone(),
            start_time: self.start_time,
            client_ip: self.client_ip.clone(),
            user_agent: self.user_agent.clone(),
            timeout: self.timeout,
        }
    }
}

/// HTTP response context for building responses
#[derive(Debug, Clone)]
pub struct ResponseContext {
    /// HTTP status code
    pub status: StatusCode,
    /// Response headers
    pub headers: HashMap<String, String>,
    /// Response body
    pub body: Vec<u8>,
    /// Custom response data
    pub data: Arc<RwLock<HashMap<String, ContextData>>>,
    /// Whether response has been sent
    pub sent: bool,
    /// Response creation time
    pub created_at: Instant,
}

impl ResponseContext {
    /// Create a new response context
    pub fn new() -> Self {
        Self {
            status: StatusCode::OK,
            headers: HashMap::new(),
            body: Vec::new(),
            data: Arc::new(RwLock::new(HashMap::new())),
            sent: false,
            created_at: Instant::now(),
        }
    }

    /// Set status code
    pub fn set_status(&mut self, status: StatusCode) {
        self.status = status;
    }

    /// Set a header
    pub fn set_header(&mut self, key: &str, value: &str) {
        self.headers.insert(key.to_string(), value.to_string());
    }

    /// Add a header (append if exists)
    pub fn add_header(&mut self, key: &str, value: &str) {
        if let Some(existing) = self.headers.get_mut(key) {
            existing.push_str(", ");
            existing.push_str(value);
        } else {
            self.headers.insert(key.to_string(), value.to_string());
        }
    }

    /// Get a header
    pub fn header(&self, key: &str) -> Option<&str> {
        self.headers.get(key).map(|s| s.as_str())
    }

    /// Set response body
    pub fn set_body(&mut self, body: Vec<u8>) {
        self.body = body;
    }

    /// Set response body from string
    pub fn set_body_string(&mut self, body: &str) {
        self.body = body.as_bytes().to_vec();
    }

    /// Set JSON response body
    pub fn set_json<T>(&mut self, data: &T) -> Result<(), Error>
    where
        T: serde::Serialize,
    {
        let json_data = serde_json::to_vec(data)?;
        self.set_body(json_data);
        self.set_header("Content-Type", "application/json");
        Ok(())
    }

    /// Set HTML response body
    pub fn set_html(&mut self, html: &str) {
        self.set_body_string(html);
        self.set_header("Content-Type", "text/html; charset=utf-8");
    }

    /// Set plain text response body
    pub fn set_text(&mut self, text: &str) {
        self.set_body_string(text);
        self.set_header("Content-Type", "text/plain; charset=utf-8");
    }

    /// Set redirect response
    pub fn set_redirect(&mut self, location: &str, permanent: bool) {
        self.status = if permanent {
            StatusCode(301) // Moved Permanently
        } else {
            StatusCode(302) // Found
        };
        self.set_header("Location", location);
    }

    /// Set custom response data
    #[instrument(skip(self, value))]
    pub fn set_data(&self, key: &str, value: ContextData) {
        if let Ok(mut data) = self.data.write() {
            data.insert(key.to_string(), value);
            debug!(key = %key, "Set response data");
        }
    }

    /// Get custom response data
    pub fn get_data(&self, key: &str) -> Option<ContextData> {
        if let Ok(data) = self.data.read() {
            data.get(key).cloned()
        } else {
            None
        }
    }

    /// Mark response as sent
    pub fn mark_sent(&mut self) {
        self.sent = true;
    }

    /// Check if response has been sent
    pub fn is_sent(&self) -> bool {
        self.sent
    }

    /// Get response size in bytes
    pub fn size(&self) -> usize {
        self.body.len()
    }

    /// Set CORS headers
    pub fn set_cors_headers(&mut self, allowed_origins: &[String], allowed_methods: &[String]) {
        let origins = if allowed_origins.contains(&"*".to_string()) {
            "*".to_string()
        } else {
            allowed_origins.join(", ")
        };
        
        self.set_header("Access-Control-Allow-Origin", &origins);
        self.set_header("Access-Control-Allow-Methods", &allowed_methods.join(", "));
        self.set_header("Access-Control-Allow-Headers", "Content-Type, Authorization");
    }

    /// Set cache control headers
    pub fn set_cache_control(&mut self, max_age: u32, public: bool) {
        let cache_control = if public {
            format!("public, max-age={}", max_age)
        } else {
            format!("private, max-age={}", max_age)
        };
        self.set_header("Cache-Control", &cache_control);
    }

    /// Set security headers
    pub fn set_security_headers(&mut self) {
        self.set_header("X-Content-Type-Options", "nosniff");
        self.set_header("X-Frame-Options", "DENY");
        self.set_header("X-XSS-Protection", "1; mode=block");
        self.set_header("Strict-Transport-Security", "max-age=31536000; includeSubDomains");
    }

    /// Clone response for middleware chain passing
    pub fn clone_for_middleware(&self) -> Self {
        Self {
            status: self.status,
            headers: self.headers.clone(),
            body: self.body.clone(),
            data: Arc::clone(&self.data),
            sent: self.sent,
            created_at: self.created_at,
        }
    }
}

impl Default for ResponseContext {
    fn default() -> Self {
        Self::new()
    }
}

/// Context utilities for middleware and handlers
pub struct ContextUtils;

impl ContextUtils {
    /// Extract client IP from various headers
    pub fn extract_client_ip(context: &RequestContext) -> Option<String> {
        // Try various headers in order of preference
        let headers_to_check = [
            "x-forwarded-for",
            "x-real-ip",
            "cf-connecting-ip",
            "x-cluster-client-ip",
            "x-forwarded",
            "forwarded-for",
            "forwarded",
        ];
        
        for header in &headers_to_check {
            if let Some(value) = context.header(header) {
                // X-Forwarded-For can contain multiple IPs, take the first
                let ip = value.split(',').next().unwrap_or(value).trim();
                if !ip.is_empty() && ip != "unknown" {
                    return Some(ip.to_string());
                }
            }
        }
        
        context.client_ip.clone()
    }

    /// Parse basic authentication from Authorization header
    pub fn parse_basic_auth(context: &RequestContext) -> Option<(String, String)> {
        if let Some(auth_header) = context.header("authorization") {
            if auth_header.starts_with("Basic ") {
                let encoded = &auth_header[6..];
                if let Ok(decoded) = base64::decode(encoded) {
                    if let Ok(decoded_str) = String::from_utf8(decoded) {
                        if let Some(colon_pos) = decoded_str.find(':') {
                            let username = decoded_str[..colon_pos].to_string();
                            let password = decoded_str[colon_pos + 1..].to_string();
                            return Some((username, password));
                        }
                    }
                }
            }
        }
        None
    }

    /// Extract bearer token from Authorization header
    pub fn extract_bearer_token(context: &RequestContext) -> Option<String> {
        if let Some(auth_header) = context.header("authorization") {
            if auth_header.starts_with("Bearer ") {
                return Some(auth_header[7..].to_string());
            }
        }
        None
    }

    /// Parse query string into parameters
    pub fn parse_query_string(query: &str) -> HashMap<String, String> {
        let mut params = HashMap::new();
        
        for pair in query.split('&') {
            if let Some(eq_pos) = pair.find('=') {
                let key = &pair[..eq_pos];
                let value = &pair[eq_pos + 1..];
                
                // URL decode key and value
                if let (Ok(decoded_key), Ok(decoded_value)) = 
                    (url_decode(key), url_decode(value)) {
                    params.insert(decoded_key, decoded_value);
                }
            } else if !pair.is_empty() {
                // Key without value
                if let Ok(decoded_key) = url_decode(pair) {
                    params.insert(decoded_key, String::new());
                }
            }
        }
        
        params
    }
}

/// Simple URL decoding function
fn url_decode(input: &str) -> Result<(), Error> {
    let mut result = Vec::new();
    let mut chars = input.chars();
    
    while let Some(c) = chars.next() {
        match c {
            '%' => {
                // Try to decode hex sequence
                let hex: String = chars.by_ref().take(2).collect();
                if hex.len() == 2 {
                    if let Ok(byte) = u8::from_str_radix(&hex, 16) {
                        result.push(byte);
                    } else {
                        // Invalid hex, treat as literal
                        result.push(b'%');
                        result.extend(hex.bytes());
                    }
                } else {
                    // Incomplete hex sequence
                    result.push(b'%');
                    result.extend(hex.bytes());
                }
            }
            '+' => result.push(b' '), // Space encoded as +
            _ => {
                let mut buf = [0; 4];
                let encoded = c.encode_utf8(&mut buf);
                result.extend(encoded.bytes());
            }
        }
    }
    
    String::from_utf8(result)
}

/// Base64 decoding implementation for HTTP authentication and data parsing
mod base64 {
    use std::collections::HashMap;
    
    /// Base64 decoding errors
    #[derive(Debug, Clone)]
    pub enum Base64Error {
        InvalidCharacter(char, usize),
        InvalidLength,
        InvalidPadding,
        Empty,
    }
    
    impl std::fmt::Display for Base64Error {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            match self {
                Base64Error::InvalidCharacter(ch, pos) => {
                    write!(f, "Invalid Base64 character '{}' at position {}", ch, pos)
                }
                Base64Error::InvalidLength => {
                    write!(f, "Invalid Base64 string length")
                }
                Base64Error::InvalidPadding => {
                    write!(f, "Invalid Base64 padding")
                }
                Base64Error::Empty => {
                    write!(f, "Empty Base64 string")
                }
            }
        }
    }
    
    impl std::error::Error for Base64Error {}
    
    /// Base64 decoder with standard alphabet
    pub struct Base64Decoder {
        decode_table: HashMap<char, u8>,
    }
    
    impl Base64Decoder {
        /// Create a new Base64 decoder with standard alphabet
        pub fn new() -> Self {
            let alphabet = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
            let mut decode_table = HashMap::new();
            
            for (i, ch) in alphabet.chars().enumerate() {
                decode_table.insert(ch, i as u8);
            }
            
            Self { decode_table }
        }
        
        /// Decode a Base64 string to bytes
        pub fn decode(&self, input: &str) -> Result<(), Error> {
            if input.is_empty() {
                return Err(Base64Error::Empty);
            }
            
            // Remove whitespace and validate characters
            let cleaned: String = input.chars().filter(|c| !c.is_whitespace()).collect();
            
            // Check for valid length (must be multiple of 4)
            if cleaned.len() % 4 != 0 {
                return Err(Base64Error::InvalidLength);
            }
            
            let mut result = Vec::new();
            let chars: Vec<char> = cleaned.chars().collect();
            
            // Process in groups of 4 characters
            for chunk in chars.chunks(4) {
                let mut values = [0u8; 4];
                let mut padding_count = 0;
                
                // Convert each character to its 6-bit value
                for (i, &ch) in chunk.iter().enumerate() {
                    if ch == '=' {
                        padding_count += 1;
                        if i < 2 {
                            return Err(Base64Error::InvalidPadding);
                        }
                        values[i] = 0;
                    } else if let Some(&value) = self.decode_table.get(&ch) {
                        if padding_count > 0 {
                            return Err(Base64Error::InvalidPadding);
                        }
                        values[i] = value;
                    } else {
                        let pos = chunk.as_ptr() as usize - chars.as_ptr() as usize + i;
                        return Err(Base64Error::InvalidCharacter(ch, pos));
                    }
                }
                
                // Validate padding
                if padding_count > 2 {
                    return Err(Base64Error::InvalidPadding);
                }
                
                // Convert 4 6-bit values to 3 8-bit bytes
                let byte1 = (values[0] << 2) | (values[1] >> 4);
                result.push(byte1);
                
                if padding_count < 2 {
                    let byte2 = ((values[1] & 0x0F) << 4) | (values[2] >> 2);
                    result.push(byte2);
                }
                
                if padding_count < 1 {
                    let byte3 = ((values[2] & 0x03) << 6) | values[3];
                    result.push(byte3);
                }
            }
            
            Ok(result)
        }
    }
    
    // Thread-safe static decoder instance
    use std::sync::OnceLock;
    static DECODER: OnceLock<Base64Decoder> = OnceLock::new();
    
    /// Decode a Base64 string to bytes
    pub fn decode(input: &str) -> Result<Vec<u8>, String> {
        let decoder = DECODER.get_or_init(|| Base64Decoder::new());
        decoder.decode(input).map_err(|e| e.to_string())
    }
    
    #[cfg(test)]
    mod tests {
        use super::*;
        
        #[test]
        fn test_basic_decoding() {
            assert_eq!(decode("SGVsbG8=").unwrap(), b"Hello");
            assert_eq!(decode("V29ybGQ=").unwrap(), b"World");
            assert_eq!(decode("").unwrap_err(), "Empty Base64 string");
        }
        
        #[test]
        fn test_padding() {
            assert_eq!(decode("QQ==").unwrap(), b"A");
            assert_eq!(decode("QUI=").unwrap(), b"AB");
            assert_eq!(decode("QUJD").unwrap(), b"ABC");
        }
        
        #[test]
        fn test_invalid_input() {
            assert!(decode("SGVsbG8").is_err()); // Invalid length
            assert!(decode("SGVsbG8@").is_err()); // Invalid character
            assert!(decode("S=VsbG8=").is_err()); // Invalid padding position
        }
        
        #[test]
        fn test_whitespace() {
            assert_eq!(decode("SGVs bG8=").unwrap(), b"Hello");
            assert_eq!(decode("SGVs\nbG8=").unwrap(), b"Hello");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_request_context_creation() {
        let context = RequestContext::new("GET".to_string(), "/test".to_string());
        assert_eq!(context.method, HttpMethod::GET);
        assert_eq!(context.path, "/test");
        assert!(!context.request_id.is_empty());
    }

    #[test]
    fn test_context_parameters() {
        let mut context = RequestContext::new("GET".to_string(), "/users/123".to_string());
        context.add_param("id", "123");
        
        assert_eq!(context.param("id"), Some("123"));
        assert_eq!(context.param("name"), None);
    }

    #[test]
    fn test_response_context() {
        let mut response = ResponseContext::new();
        response.set_status(StatusCode::CREATED);
        response.set_header("Content-Type", "application/json");
        response.set_body_string("test body");
        
        assert_eq!(response.status, StatusCode::CREATED);
        assert_eq!(response.header("Content-Type"), Some("application/json"));
        assert_eq!(response.body, b"test body");
    }

    #[test]
    fn test_context_data() {
        let context = RequestContext::new("GET".to_string(), "/test".to_string());
        context.set_data("user_id", ContextData::Integer(123));
        
        let user_id = context.get_data("user_id").unwrap();
        assert_eq!(user_id.as_integer(), Some(123));
    }

    #[test]
    fn test_url_decode() {
        assert_eq!(url_decode("hello%20world").unwrap(), "hello world");
        assert_eq!(url_decode("test+string").unwrap(), "test string");
        assert_eq!(url_decode("normal").unwrap(), "normal");
    }

    #[test]
    fn test_query_string_parsing() {
        let params = ContextUtils::parse_query_string("name=john&age=30&city=New%20York");
        assert_eq!(params.get("name"), Some(&"john".to_string()));
        assert_eq!(params.get("age"), Some(&"30".to_string()));
        assert_eq!(params.get("city"), Some(&"New York".to_string()));
    }
}
