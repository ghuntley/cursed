use crate::error::CursedError;
/// HTTP client implementation for CURSED networking
/// 
/// This module provides a comprehensive HTTP client with support for various
/// HTTP methods, authentication, cookies, connection pooling, and more.

use std::time::Duration;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
// use crate::stdlib::net::error::{NetError, NetResult, http_error, http_error_with_status, timeout_error};
// use crate::stdlib::net::socket::TcpSocket;
// use crate::stdlib::net::http::{
    HttpRequest, HttpResponse, HttpHeaders, HttpAuth, CookieJar, ConnectionPool,
    HttpConfig, Method, Status, HttpVersion, mime
};

/// HTTP client for making requests
#[derive(Debug, Clone)]
pub struct HttpClient {
    config: HttpConfig,
    default_headers: HttpHeaders,
    cookie_jar: Arc<Mutex<CookieJar>>,
    connection_pool: Arc<Mutex<ConnectionPool>>,
    auth: Option<HttpAuth>,
}

impl HttpClient {
    /// Create a new HTTP client with default configuration
    pub fn new() -> NetResult<Self> {
        Self::builder().build()
    }
    
    /// Create a builder for configuring the HTTP client
    pub fn builder() -> HttpClientBuilder {
        HttpClientBuilder::new()
    }
    
    /// Make a GET request
    pub fn get(&self, url: &str) -> NetResult<HttpResponse> {
        self.request(Method::GET, url)
            .send()
    }
    
    /// Make a POST request with body
    pub fn post(&self, url: &str) -> RequestBuilder {
        self.request(Method::POST, url)
    }
    
    /// Make a PUT request with body
    pub fn put(&self, url: &str) -> RequestBuilder {
        self.request(Method::PUT, url)
    }
    
    /// Make a DELETE request
    pub fn delete(&self, url: &str) -> NetResult<HttpResponse> {
        self.request(Method::DELETE, url)
            .send()
    }
    
    /// Make a PATCH request with body
    pub fn patch(&self, url: &str) -> RequestBuilder {
        self.request(Method::PATCH, url)
    }
    
    /// Make a HEAD request
    pub fn head(&self, url: &str) -> NetResult<HttpResponse> {
        self.request(Method::HEAD, url)
            .send()
    }
    
    /// Create a request builder for any HTTP method
    pub fn request(&self, method: Method, url: &str) -> RequestBuilder {
        RequestBuilder::new(self, method, url)
    }
    
    /// Execute an HttpRequest
    pub fn execute(&self, request: &HttpRequest) -> NetResult<HttpResponse> {
        // Parse URL to extract host, port, and path
        let url = self.parse_url(&request.url)?;
        
        // Create or reuse connection
        let mut socket = self.get_connection(&url.host, url.port)?;
        
        // Build HTTP request string
        let request_str = self.build_request_string(request, &url)?;
        
        // Send request
        socket.write_string(&request_str)?;
        
        // Read response
        let response = self.read_response(&mut socket)?;
        
        // Handle cookies
        self.handle_response_cookies(&response)?;
        
        // Handle redirects if configured
        if self.config.follow_redirects && response.status.is_redirection() {
            return self.handle_redirect(request, &response);
        }
        
        Ok(response)
    }
    
    /// Parse URL into components
    fn parse_url(&self, url: &str) -> NetResult<UrlComponents> {
        if !url.starts_with("http://") && !url.starts_with("https://") {
            return Err(http_error(&format!("Invalid URL scheme: {}", url)));
        }
        
        let is_https = url.starts_with("https://");
        let without_scheme = if is_https {
            &url[8..] // Remove "https://"
        } else {
            &url[7..] // Remove "http://"
        };
        
        let (host_port, path) = if let Some(slash_pos) = without_scheme.find('/') {
            (&without_scheme[..slash_pos], &without_scheme[slash_pos..])
        } else {
            (without_scheme, "/")
        };
        
        let (host, port) = if let Some(colon_pos) = host_port.rfind(':') {
            let host = &host_port[..colon_pos];
            let port_str = &host_port[colon_pos + 1..];
            let port = port_str.parse::<u16>()
                .map_err(|_| http_error(&format!("Invalid port: {}", port_str)))?;
            (host.to_string(), port)
        } else {
            (host_port.to_string(), if is_https { 443 } else { 80 })
        };
        
        Ok(UrlComponents {
            scheme: if is_https { "https" } else { "http" }.to_string(),
            host,
            port,
            path: path.to_string(),
            is_https,
        })
    }
    
    /// Get or create a connection to the specified host
    fn get_connection(&self, host: &str, port: u16) -> NetResult<TcpSocket> {
        // For now, create a new connection each time
        // In a real implementation, this would use the connection pool
        let addr = format!("{}:{}", host, port);
        TcpSocket::connect_timeout(&addr, self.config.timeout.connect)
    }
    
    /// Build the HTTP request string
    fn build_request_string(&self, request: &HttpRequest, url: &UrlComponents) -> NetResult<String> {
        let mut request_str = String::new();
        
        // Request line: METHOD /path HTTP/1.1
        request_str.push_str(&format!("{} {} HTTP/1.1\r\n", request.method, url.path));
        
        // Host header (required for HTTP/1.1)
        request_str.push_str(&format!("Host: {}\r\n", url.host));
        
        // Default headers from client
        for (name, value) in &self.default_headers.headers {
            request_str.push_str(&format!("{}: {}\r\n", name, value));
        }
        
        // Request-specific headers
        for (name, value) in &request.headers.headers {
            request_str.push_str(&format!("{}: {}\r\n", name, value));
        }
        
        // Authentication header
        if let Some(ref auth) = self.auth {
            if let Some(auth_header) = auth.to_header_value() {
                request_str.push_str(&format!("Authorization: {}\r\n", auth_header));
            }
        }
        
        // Cookies
        let cookie_header = self.get_cookie_header(&url.host, &url.path)?;
        if !cookie_header.is_empty() {
            request_str.push_str(&format!("Cookie: {}\r\n", cookie_header));
        }
        
        // Content-Length for requests with body
        if let Some(ref body) = request.body {
            request_str.push_str(&format!("Content-Length: {}\r\n", body.len()));
        }
        
        // End of headers
        request_str.push_str("\r\n");
        
        // Request body
        if let Some(ref body) = request.body {
            request_str.push_str(body);
        }
        
        Ok(request_str)
    }
    
    /// Read HTTP response from socket
    fn read_response(&self, socket: &mut TcpSocket) -> NetResult<HttpResponse> {
        // Set read timeout
        socket.set_timeout(Some(self.config.timeout.read), None)?;
        
        // Read status line
        let status_line = socket.read_line()?;
        let (version, status) = self.parse_status_line(&status_line)?;
        
        // Read headers
        let mut headers = HttpHeaders::new();
        loop {
            let header_line = socket.read_line()?;
            if header_line.trim().is_empty() {
                break; // End of headers
            }
            
            if let Some(colon_pos) = header_line.find(':') {
                let name = header_line[..colon_pos].trim().to_lowercase();
                let value = header_line[colon_pos + 1..].trim().to_string();
                headers.set(&name, &value);
            }
        }
        
        // Read body
        let body = self.read_response_body(socket, &headers)?;
        
        Ok(HttpResponse {
            status,
            version,
            headers,
            body,
            url: String::new(), // Will be set by caller
        })
    }
    
    /// Parse HTTP status line
    fn parse_status_line(&self, line: &str) -> NetResult<(HttpVersion, Status)> {
        let parts: Vec<&str> = line.split_whitespace().collect();
        if parts.len() < 2 {
            return Err(http_error(&format!("Invalid status line: {}", line)));
        }
        
        let version = match parts[0] {
            "HTTP/1.0" => HttpVersion::Http10,
            "HTTP/1.1" => HttpVersion::Http11,
            "HTTP/2" => HttpVersion::Http2,
            _ => return Err(http_error(&format!("Unsupported HTTP version: {}", parts[0]))),
        };
        
        let status_code = parts[1].parse::<u16>()
            .map_err(|_| http_error(&format!("Invalid status code: {}", parts[1])))?;
        
        Ok((version, Status(status_code)))
    }
    
    /// Read response body based on headers
    fn read_response_body(&self, socket: &mut TcpSocket, headers: &HttpHeaders) -> NetResult<String> {
        if let Some(content_length) = headers.get("content-length") {
            // Read exact number of bytes
            let length = content_length.parse::<usize>()
                .map_err(|_| http_error("Invalid Content-Length header"))?;
            
            let mut buffer = vec![0u8; length];
            socket.read_exact(&mut buffer)?;
            
            String::from_utf8(buffer)
                .map_err(|e| http_error(&format!("Invalid UTF-8 in response body: {}", e)))
        } else if headers.get("transfer-encoding").map_or(false, |te| te.contains("chunked")) {
            // Read chunked encoding
            self.read_chunked_body(socket)
        } else {
            // Read until connection closes
            let mut body = String::new();
            loop {
                match socket.read_string(8192) {
                    Ok(chunk) if chunk.is_empty() => break,
                    Ok(chunk) => body.push_str(&chunk),
                    Err(_) => break, // Connection closed or error
                }
            }
            Ok(body)
        }
    }
    
    /// Read chunked response body
    fn read_chunked_body(&self, socket: &mut TcpSocket) -> NetResult<String> {
        let mut body = String::new();
        
        loop {
            // Read chunk size line
            let size_line = socket.read_line()?;
            let chunk_size = usize::from_str_radix(size_line.trim(), 16)
                .map_err(|_| http_error("Invalid chunk size"))?;
            
            if chunk_size == 0 {
                // Last chunk, read trailing headers
                loop {
                    let line = socket.read_line()?;
                    if line.trim().is_empty() {
                        break;
                    }
                }
                break;
            }
            
            // Read chunk data
            let mut chunk_buffer = vec![0u8; chunk_size];
            socket.read_exact(&mut chunk_buffer)?;
            
            let chunk_str = String::from_utf8(chunk_buffer)
                .map_err(|e| http_error(&format!("Invalid UTF-8 in chunk: {}", e)))?;
            body.push_str(&chunk_str);
            
            // Read trailing CRLF
            socket.read_line()?;
        }
        
        Ok(body)
    }
    
    /// Get cookie header for request
    fn get_cookie_header(&self, host: &str, path: &str) -> NetResult<String> {
        let jar = self.cookie_jar.lock().unwrap();
        Ok(jar.get_cookies_for_request(host, path))
    }
    
    /// Handle cookies from response
    fn handle_response_cookies(&self, response: &HttpResponse) -> NetResult<()> {
        let mut jar = self.cookie_jar.lock().unwrap();
        
        // Process Set-Cookie headers
        for (name, value) in &response.headers.headers {
            if name.to_lowercase() == "set-cookie" {
                jar.add_cookie_from_header(value)?;
            }
        }
        
        Ok(())
    }
    
    /// Handle HTTP redirects
    fn handle_redirect(&self, original_request: &HttpRequest, response: &HttpResponse) -> NetResult<HttpResponse> {
        if let Some(location) = response.headers.get("location") {
            // Create new request for redirect
            let mut new_request = original_request.clone();
            new_request.url = location.to_string();
            
            // Change POST to GET for certain redirect codes
            if matches!(response.status.as_u16(), 301 | 302 | 303) && original_request.method == Method::POST {
                new_request.method = Method::GET;
                new_request.body = None;
            }
            
            self.execute(&new_request)
        } else {
            Err(http_error("Redirect response missing Location header"))
        }
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new().unwrap()
    }
}

/// Builder for configuring HTTP client
#[derive(Debug)]
pub struct HttpClientBuilder {
    config: HttpConfig,
    default_headers: HttpHeaders,
    auth: Option<HttpAuth>,
}

impl HttpClientBuilder {
    /// Create a new builder
    pub fn new() -> Self {
        let mut default_headers = HttpHeaders::new();
        default_headers.set("User-Agent", "CURSED-HTTP-Client/1.0");
        default_headers.set("Accept", "*/*");
        default_headers.set("Connection", "keep-alive");
        
        Self {
            config: HttpConfig::default(),
            default_headers,
            auth: None,
        }
    }
    
    /// Set connection timeout
    pub fn connect_timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout.connect = timeout;
        self
    }
    
    /// Set read timeout
    pub fn read_timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout.read = timeout;
        self
    }
    
    /// Set write timeout
    pub fn write_timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout.write = timeout;
        self
    }
    
    /// Set User-Agent header
    pub fn user_agent(mut self, user_agent: &str) -> Self {
        self.default_headers.set("User-Agent", user_agent);
        self
    }
    
    /// Add default header
    pub fn default_header(mut self, name: &str, value: &str) -> Self {
        self.default_headers.set(name, value);
        self
    }
    
    /// Set authentication
    pub fn auth(mut self, auth: HttpAuth) -> Self {
        self.auth = Some(auth);
        self
    }
    
    /// Enable/disable following redirects
    pub fn follow_redirects(mut self, follow: bool) -> Self {
        self.config.follow_redirects = follow;
        self
    }
    
    /// Set maximum number of redirects to follow
    pub fn max_redirects(mut self, max: usize) -> Self {
        self.config.max_redirects = max;
        self
    }
    
    /// Build the HTTP client
    pub fn build(self) -> NetResult<HttpClient> {
        Ok(HttpClient {
            config: self.config,
            default_headers: self.default_headers,
            cookie_jar: Arc::new(Mutex::new(CookieJar::new())),
            connection_pool: Arc::new(Mutex::new(ConnectionPool::new())),
            auth: self.auth,
        })
    }
}

impl Default for HttpClientBuilder {
    fn default() -> Self {
        Self::new()
    }
}

/// Request builder for fluent API
#[derive(Debug)]
pub struct RequestBuilder<'a> {
    client: &'a HttpClient,
    request: HttpRequest,
}

impl<'a> RequestBuilder<'a> {
    /// Create a new request builder
    pub fn new(client: &'a HttpClient, method: Method, url: &str) -> Self {
        Self {
            client,
            request: HttpRequest {
                method,
                url: url.to_string(),
                headers: HttpHeaders::new(),
                body: None,
            },
        }
    }
    
    /// Set request header
    pub fn header(mut self, name: &str, value: &str) -> Self {
        self.request.headers.set(name, value);
        self
    }
    
    /// Set Content-Type header
    pub fn content_type(self, content_type: &str) -> Self {
        self.header("Content-Type", content_type)
    }
    
    /// Set Accept header
    pub fn accept(self, accept: &str) -> Self {
        self.header("Accept", accept)
    }
    
    /// Set request body as string
    pub fn body(mut self, body: String) -> Self {
        self.request.body = Some(body);
        self
    }
    
    /// Set request body as JSON
    pub fn json<T: json::JsonSerialize>(self, data: &T) -> NetResult<Self> {
        let json_body = json::to_string(data)
            .map_err(|e| http_error(&format!("Failed to serialize JSON: {}", e)))?;
        
        Ok(self
            .content_type(mime::APPLICATION_JSON)
            .body(json_body))
    }
    
    /// Add query parameter
    pub fn query(mut self, key: &str, value: &str) -> Self {
        let mut params = HashMap::new();
        if let Some(query_start) = self.request.url.find('?') {
            let query_part = &self.request.url[query_start + 1..];
            params = query::parse_query_string(query_part);
            self.request.url = self.request.url[..query_start].to_string();
        }
        
        params.insert(key.to_string(), value.to_string());
        self.request.url = query::add_query_params(&self.request.url, &params);
        self
    }
    
    /// Add multiple query parameters
    pub fn query_params(mut self, params: &HashMap<String, String>) -> Self {
        let mut existing_params = HashMap::new();
        if let Some(query_start) = self.request.url.find('?') {
            let query_part = &self.request.url[query_start + 1..];
            existing_params = query::parse_query_string(query_part);
            self.request.url = self.request.url[..query_start].to_string();
        }
        
        // Merge new params with existing ones
        for (key, value) in params {
            existing_params.insert(key.clone(), value.clone());
        }
        
        self.request.url = query::add_query_params(&self.request.url, &existing_params);
        self
    }
    
    /// Set Bearer token authorization
    pub fn bearer_token(self, token: &str) -> Self {
        self.header("Authorization", &format!("Bearer {}", token))
    }
    
    /// Set Basic authentication
    pub fn basic_auth(self, username: &str, password: Option<&str>) -> Self {
        let credentials = match password {
            Some(pwd) => format!("{}:{}", username, pwd),
            None => username.to_string(),
        };
        let encoded = base64_encode(credentials.as_bytes());
        self.header("Authorization", &format!("Basic {}", encoded))
    }
    
    /// Set request body as form data
    pub fn form(self, data: &HashMap<String, String>) -> Self {
        let form_body = data.iter()
            .map(|(k, v)| format!("{}={}", urlencoding::encode(k), urlencoding::encode(v)))
            .collect::<Vec<_>>()
            .join("&");
        
        self.content_type(mime::APPLICATION_FORM_URLENCODED)
            .body(form_body)
    }
    
    /// Send the request
    pub fn send(self) -> NetResult<HttpResponse> {
        self.client.execute(&self.request)
    }
}

/// URL components for parsing
#[derive(Debug)]
struct UrlComponents {
    scheme: String,
    host: String,
    port: u16,
    path: String,
    is_https: bool,
}

/// Enhanced JSON serialization support
mod json {
//     use crate::stdlib::net::error::{NetError, NetResult};
    use std::collections::HashMap;
    
    /// JSON serialization trait
    pub trait JsonSerialize {
        fn to_json(&self) -> NetResult<String>;
    }
    
    /// JSON value representation
    #[derive(Debug, Clone, PartialEq)]
    pub enum JsonValue {
        Null,
        Bool(bool),
        Number(f64),
        String(String),
        Array(Vec<JsonValue>),
        Object(HashMap<String, JsonValue>),
    }
    
    impl JsonValue {
        /// Convert JsonValue to string representation
        pub fn to_string(&self) -> String {
            match self {
                JsonValue::Null => "null".to_string(),
                JsonValue::Bool(b) => b.to_string(),
                JsonValue::Number(n) => {
                    if n.fract() == 0.0 && n.is_finite() {
                        format!("{}", *n as i64)
                    } else {
                        n.to_string()
                    }
                },
                JsonValue::String(s) => format!("\"{}\"", escape_json_string(s)),
                JsonValue::Array(arr) => {
                    let items: Vec<String> = arr.iter().map(|v| v.to_string()).collect();
                    format!("[{}]", items.join(","))
                },
                JsonValue::Object(obj) => {
                    let items: Vec<String> = obj.iter()
                        .map(|(k, v)| format!("\"{}\":{}", escape_json_string(k), v.to_string()))
                        .collect();
                    format!("{{{}}}", items.join(","))
                },
            }
        }
        
        /// Parse JSON string into JsonValue
        pub fn parse(json_str: &str) -> NetResult<JsonValue> {
            let mut parser = JsonParser::new(json_str);
            parser.parse_value()
        }
    }
    
    /// Simple JSON parser
    struct JsonParser {
        input: Vec<char>,
        pos: usize,
    }
    
    impl JsonParser {
        fn new(input: &str) -> Self {
            Self {
                input: input.chars().collect(),
                pos: 0,
            }
        }
        
        fn parse_value(&mut self) -> NetResult<JsonValue> {
            self.skip_whitespace();
            
            if self.pos >= self.input.len() {
                return Err(NetError::General("Unexpected end of JSON".to_string()));
            }
            
            match self.input[self.pos] {
                'n' => self.parse_null(),
                't' | 'f' => self.parse_bool(),
                '"' => self.parse_string(),
                '[' => self.parse_array(),
                '{' => self.parse_object(),
                c if c.is_ascii_digit() || c == '-' => self.parse_number(),
                _ => Err(NetError::General(format!("Unexpected character: {}", self.input[self.pos]))),
            }
        }
        
        fn parse_null(&mut self) -> NetResult<JsonValue> {
            if self.consume_literal("null") {
                Ok(JsonValue::Null)
            } else {
                Err(NetError::General("Invalid null literal".to_string()))
            }
        }
        
        fn parse_bool(&mut self) -> NetResult<JsonValue> {
            if self.consume_literal("true") {
                Ok(JsonValue::Bool(true))
            } else if self.consume_literal("false") {
                Ok(JsonValue::Bool(false))
            } else {
                Err(NetError::General("Invalid boolean literal".to_string()))
            }
        }
        
        fn parse_string(&mut self) -> NetResult<JsonValue> {
            if self.input[self.pos] != '"' {
                return Err(NetError::General("Expected '\"'".to_string()));
            }
            self.pos += 1; // Skip opening quote
            
            let mut result = String::new();
            while self.pos < self.input.len() && self.input[self.pos] != '"' {
                if self.input[self.pos] == '\\' {
                    self.pos += 1;
                    if self.pos >= self.input.len() {
                        return Err(NetError::General("Unterminated string escape".to_string()));
                    }
                    match self.input[self.pos] {
                        '"' => result.push('"'),
                        '\\' => result.push('\\'),
                        '/' => result.push('/'),
                        'b' => result.push('\u{0008}'),
                        'f' => result.push('\u{000C}'),
                        'n' => result.push('\n'),
                        'r' => result.push('\r'),
                        't' => result.push('\t'),
                        _ => return Err(NetError::General("Invalid escape sequence".to_string())),
                    }
                } else {
                    result.push(self.input[self.pos]);
                }
                self.pos += 1;
            }
            
            if self.pos >= self.input.len() {
                return Err(NetError::General("Unterminated string".to_string()));
            }
            self.pos += 1; // Skip closing quote
            
            Ok(JsonValue::String(result))
        }
        
        fn parse_array(&mut self) -> NetResult<JsonValue> {
            if self.input[self.pos] != '[' {
                return Err(NetError::General("Expected '['".to_string()));
            }
            self.pos += 1; // Skip opening bracket
            
            let mut array = Vec::new();
            self.skip_whitespace();
            
            if self.pos < self.input.len() && self.input[self.pos] == ']' {
                self.pos += 1;
                return Ok(JsonValue::Array(array));
            }
            
            loop {
                array.push(self.parse_value()?);
                self.skip_whitespace();
                
                if self.pos >= self.input.len() {
                    return Err(NetError::General("Unterminated array".to_string()));
                }
                
                match self.input[self.pos] {
                    ',' => {
                        self.pos += 1;
                        self.skip_whitespace();
                    },
                    ']' => {
                        self.pos += 1;
                        break;
                    },
                    _ => return Err(NetError::General("Expected ',' or ']'".to_string())),
                }
            }
            
            Ok(JsonValue::Array(array))
        }
        
        fn parse_object(&mut self) -> NetResult<JsonValue> {
            if self.input[self.pos] != '{' {
                return Err(NetError::General("Expected '{'".to_string()));
            }
            self.pos += 1; // Skip opening brace
            
            let mut object = HashMap::new();
            self.skip_whitespace();
            
            if self.pos < self.input.len() && self.input[self.pos] == '}' {
                self.pos += 1;
                return Ok(JsonValue::Object(object));
            }
            
            loop {
                // Parse key
                if self.input[self.pos] != '"' {
                    return Err(NetError::General("Expected string key".to_string()));
                }
                let key = match self.parse_string()? {
                    JsonValue::String(s) => s,
                    _ => unreachable!(),
                };
                
                self.skip_whitespace();
                if self.pos >= self.input.len() || self.input[self.pos] != ':' {
                    return Err(NetError::General("Expected ':'".to_string()));
                }
                self.pos += 1; // Skip colon
                self.skip_whitespace();
                
                // Parse value
                let value = self.parse_value()?;
                object.insert(key, value);
                
                self.skip_whitespace();
                if self.pos >= self.input.len() {
                    return Err(NetError::General("Unterminated object".to_string()));
                }
                
                match self.input[self.pos] {
                    ',' => {
                        self.pos += 1;
                        self.skip_whitespace();
                    },
                    '}' => {
                        self.pos += 1;
                        break;
                    },
                    _ => return Err(NetError::General("Expected ',' or '}'".to_string())),
                }
            }
            
            Ok(JsonValue::Object(object))
        }
        
        fn parse_number(&mut self) -> NetResult<JsonValue> {
            let start = self.pos;
            
            if self.input[self.pos] == '-' {
                self.pos += 1;
            }
            
            if self.pos >= self.input.len() || !self.input[self.pos].is_ascii_digit() {
                return Err(NetError::General("Invalid number".to_string()));
            }
            
            // Integer part
            if self.input[self.pos] == '0' {
                self.pos += 1;
            } else {
                while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
                    self.pos += 1;
                }
            }
            
            // Fractional part
            if self.pos < self.input.len() && self.input[self.pos] == '.' {
                self.pos += 1;
                if self.pos >= self.input.len() || !self.input[self.pos].is_ascii_digit() {
                    return Err(NetError::General("Invalid number".to_string()));
                }
                while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
                    self.pos += 1;
                }
            }
            
            // Exponent part
            if self.pos < self.input.len() && (self.input[self.pos] == 'e' || self.input[self.pos] == 'E') {
                self.pos += 1;
                if self.pos < self.input.len() && (self.input[self.pos] == '+' || self.input[self.pos] == '-') {
                    self.pos += 1;
                }
                if self.pos >= self.input.len() || !self.input[self.pos].is_ascii_digit() {
                    return Err(NetError::General("Invalid number".to_string()));
                }
                while self.pos < self.input.len() && self.input[self.pos].is_ascii_digit() {
                    self.pos += 1;
                }
            }
            
            let number_str: String = self.input[start..self.pos].iter().collect();
            let number = number_str.parse::<f64>()
                .map_err(|_| NetError::General("Invalid number format".to_string()))?;
            
            Ok(JsonValue::Number(number))
        }
        
        fn consume_literal(&mut self, literal: &str) -> bool {
            let chars: Vec<char> = literal.chars().collect();
            if self.pos + chars.len() <= self.input.len() {
                let slice = &self.input[self.pos..self.pos + chars.len()];
                if slice == chars {
                    self.pos += chars.len();
                    return true;
                }
            }
            false
        }
        
        fn skip_whitespace(&mut self) {
            while self.pos < self.input.len() && self.input[self.pos].is_whitespace() {
                self.pos += 1;
            }
        }
    }
    
    /// Escape string for JSON
    fn escape_json_string(s: &str) -> String {
        let mut result = String::new();
        for c in s.chars() {
            match c {
                '"' => result.push_str("\\\""),
                '\\' => result.push_str("\\\\"),
                '\u{0008}' => result.push_str("\\b"),
                '\u{000C}' => result.push_str("\\f"),
                '\n' => result.push_str("\\n"),
                '\r' => result.push_str("\\r"),
                '\t' => result.push_str("\\t"),
                c if c.is_control() => result.push_str(&format!("\\u{:04x}", c as u32)),
                c => result.push(c),
            }
        }
        result
    }
    
    /// Simple JSON serialization for common types
    impl JsonSerialize for String {
        fn to_json(&self) -> NetResult<String> {
            Ok(JsonValue::String(self.clone()).to_string())
        }
    }
    
    impl JsonSerialize for &str {
        fn to_json(&self) -> NetResult<String> {
            Ok(JsonValue::String(self.to_string()).to_string())
        }
    }
    
    impl JsonSerialize for i32 {
        fn to_json(&self) -> NetResult<String> {
            Ok(JsonValue::Number(*self as f64).to_string())
        }
    }
    
    impl JsonSerialize for i64 {
        fn to_json(&self) -> NetResult<String> {
            Ok(JsonValue::Number(*self as f64).to_string())
        }
    }
    
    impl JsonSerialize for f32 {
        fn to_json(&self) -> NetResult<String> {
            Ok(JsonValue::Number(*self as f64).to_string())
        }
    }
    
    impl JsonSerialize for f64 {
        fn to_json(&self) -> NetResult<String> {
            Ok(JsonValue::Number(*self).to_string())
        }
    }
    
    impl JsonSerialize for bool {
        fn to_json(&self) -> NetResult<String> {
            Ok(JsonValue::Bool(*self).to_string())
        }
    }
    
    impl<T: JsonSerialize> JsonSerialize for Vec<T> {
        fn to_json(&self) -> NetResult<String> {
            let items: crate::error::Result<()> = self.iter()
                .map(|item| item.to_json())
                .collect();
            let items = items?;
            Ok(format!("[{}]", items.join(",")))
        }
    }
    
    impl<T: JsonSerialize> JsonSerialize for HashMap<String, T> {
        fn to_json(&self) -> NetResult<String> {
            let items: crate::error::Result<()> = self.iter()
                .map(|(k, v)| {
                    let value_json = v.to_json()?;
                    Ok(format!("\"{}\":{}", escape_json_string(k), value_json))
                })
                .collect();
            let items = items?;
            Ok(format!("{{{}}}", items.join(",")))
        }
    }
    
    /// Convert JSON serializable type to string
    pub fn to_string<T: JsonSerialize>(data: &T) -> NetResult<String> {
        data.to_json()
    }
}

/// Enhanced URL encoding with RFC 3986 compliance
mod urlencoding {
    /// URL encode string with RFC 3986 compliance
    pub fn encode(input: &str) -> String {
        input.chars()
            .map(|c| match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
                ' ' => "+".to_string(),
                _ => percent_encode_byte(c as u8),
            })
            .collect()
    }
    
    /// URL encode for form data (application/x-www-form-urlencoded)
    pub fn encode_form(input: &str) -> String {
        input.chars()
            .map(|c| match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
                ' ' => "+".to_string(),
                _ => percent_encode_byte(c as u8),
            })
            .collect()
    }
    
    /// URL encode for path segments (preserves slashes)
    pub fn encode_path(input: &str) -> String {
        input.chars()
            .map(|c| match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' | '/' => c.to_string(),
                _ => percent_encode_byte(c as u8),
            })
            .collect()
    }
    
    /// URL decode string
    pub fn decode(input: &str) -> String {
        let mut result = String::new();
        let mut chars = input.chars().peekable();
        
        while let Some(c) = chars.next() {
            match c {
                '+' => result.push(' '),
                '%' => {
                    if let (Some(h1), Some(h2)) = (chars.next(), chars.next()) {
                        if let (Some(d1), Some(d2)) = (hex_digit_value(h1), hex_digit_value(h2)) {
                            let byte = (d1 << 4) | d2;
                            result.push(byte as char);
                        } else {
                            // Invalid hex, keep as literal
                            result.push('%');
                            result.push(h1);
                            result.push(h2);
                        }
                    } else {
                        result.push('%');
                    }
                },
                c => result.push(c),
            }
        }
        
        result
    }
    
    /// Percent encode a single byte
    fn percent_encode_byte(byte: u8) -> String {
        format!("%{:02X}", byte)
    }
    
    /// Convert hex digit to value
    fn hex_digit_value(c: char) -> Option<u8> {
        match c {
            '0'..='9' => Some(c as u8 - b'0'),
            'A'..='F' => Some(c as u8 - b'A' + 10),
            'a'..='f' => Some(c as u8 - b'a' + 10),
            _ => None,
        }
    }
}

/// Base64 encoding utilities
mod base64 {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    
    /// Encode bytes to base64 string
    pub fn encode(input: &[u8]) -> String {
        let mut result = String::new();
        let mut i = 0;
        
        while i + 2 < input.len() {
            let b1 = input[i];
            let b2 = input[i + 1];
            let b3 = input[i + 2];
            
            let n = ((b1 as u32) << 16) | ((b2 as u32) << 8) | (b3 as u32);
            
            result.push(ALPHABET[((n >> 18) & 63) as usize] as char);
            result.push(ALPHABET[((n >> 12) & 63) as usize] as char);
            result.push(ALPHABET[((n >> 6) & 63) as usize] as char);
            result.push(ALPHABET[(n & 63) as usize] as char);
            
            i += 3;
        }
        
        match input.len() - i {
            1 => {
                let b1 = input[i];
                let n = (b1 as u32) << 16;
                result.push(ALPHABET[((n >> 18) & 63) as usize] as char);
                result.push(ALPHABET[((n >> 12) & 63) as usize] as char);
                result.push('=');
                result.push('=');
            },
            2 => {
                let b1 = input[i];
                let b2 = input[i + 1];
                let n = ((b1 as u32) << 16) | ((b2 as u32) << 8);
                result.push(ALPHABET[((n >> 18) & 63) as usize] as char);
                result.push(ALPHABET[((n >> 12) & 63) as usize] as char);
                result.push(ALPHABET[((n >> 6) & 63) as usize] as char);
                result.push('=');
            },
            _ => {},
        }
        
        result
    }
    
    /// Decode base64 string to bytes
    pub fn decode(input: &str) -> Result<Vec<u8>, String> {
        let input = input.replace(['\r', '\n', ' ', '\t'], "");
        let input = input.trim_end_matches('=');
        
        let mut result = Vec::new();
        let chars: Vec<u8> = input.bytes().collect();
        let mut i = 0;
        
        while i + 3 < chars.len() {
            let c1 = decode_char(chars[i])?;
            let c2 = decode_char(chars[i + 1])?;
            let c3 = decode_char(chars[i + 2])?;
            let c4 = decode_char(chars[i + 3])?;
            
            let n = (c1 << 18) | (c2 << 12) | (c3 << 6) | c4;
            
            result.push((n >> 16) as u8);
            result.push((n >> 8) as u8);
            result.push(n as u8);
            
            i += 4;
        }
        
        match chars.len() - i {
            2 => {
                let c1 = decode_char(chars[i])?;
                let c2 = decode_char(chars[i + 1])?;
                let n = (c1 << 18) | (c2 << 12);
                result.push((n >> 16) as u8);
            },
            3 => {
                let c1 = decode_char(chars[i])?;
                let c2 = decode_char(chars[i + 1])?;
                let c3 = decode_char(chars[i + 2])?;
                let n = (c1 << 18) | (c2 << 12) | (c3 << 6);
                result.push((n >> 16) as u8);
                result.push((n >> 8) as u8);
            },
            1 => return Err("Invalid base64 padding".to_string()),
            _ => {},
        }
        
        Ok(result)
    }
    
    fn decode_char(c: u8) -> Result<u32, String> {
        match c {
            b'A'..=b'Z' => Ok((c - b'A') as u32),
            b'a'..=b'z' => Ok((c - b'a' + 26) as u32),
            b'0'..=b'9' => Ok((c - b'0' + 52) as u32),
            b'+' => Ok(62),
            b'/' => Ok(63),
            _ => Err(format!("Invalid base64 character: {}", c as char)),
        }
    }
}

/// Helper function for base64 encoding
pub fn base64_encode(input: &[u8]) -> String {
    base64::encode(input)
}

/// Helper function for base64 decoding
pub fn base64_decode(input: &str) -> Result<Vec<u8>, String> {
    base64::decode(input)
}

/// Query parameter building utilities
pub mod query {
    use super::urlencoding;
    use std::collections::HashMap;
    
    /// Build query string from parameters
    pub fn build_query_string(params: &HashMap<String, String>) -> String {
        if params.is_empty() {
            return String::new();
        }
        
        let encoded_params: Vec<String> = params.iter()
            .map(|(key, value)| {
                format!("{}={}", 
                    urlencoding::encode(key), 
                    urlencoding::encode(value))
            })
            .collect();
        
        encoded_params.join("&")
    }
    
    /// Parse query string into parameters
    pub fn parse_query_string(query: &str) -> HashMap<String, String> {
        let mut params = HashMap::new();
        
        if query.is_empty() {
            return params;
        }
        
        for pair in query.split('&') {
            if let Some(eq_pos) = pair.find('=') {
                let key = urlencoding::decode(&pair[..eq_pos]);
                let value = urlencoding::decode(&pair[eq_pos + 1..]);
                params.insert(key, value);
            } else {
                let key = urlencoding::decode(pair);
                params.insert(key, String::new());
            }
        }
        
        params
    }
    
    /// Add parameters to URL
    pub fn add_query_params(url: &str, params: &HashMap<String, String>) -> String {
        if params.is_empty() {
            return url.to_string();
        }
        
        let query_string = build_query_string(params);
        
        if url.contains('?') {
            format!("{}&{}", url, query_string)
        } else {
            format!("{}?{}", url, query_string)
        }
    }
}

