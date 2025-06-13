/// HTTP client implementation for CURSED networking
/// 
/// This module provides a comprehensive HTTP client with support for various
/// HTTP methods, authentication, cookies, connection pooling, and more.

use std::time::Duration;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use crate::stdlib::net::error::{NetError, NetResult, http_error, http_error_with_status, timeout_error};
use crate::stdlib::net::socket::TcpSocket;
use crate::stdlib::net::http::{
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
    pub fn json<T: serde::Serialize>(self, data: &T) -> NetResult<Self> {
        let json_body = serde_json::to_string(data)
            .map_err(|e| http_error(&format!("Failed to serialize JSON: {}", e)))?;
        
        Ok(self
            .content_type(mime::APPLICATION_JSON)
            .body(json_body))
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

// Placeholder for external dependencies
mod serde {
    pub trait Serialize {}
}

mod serde_json {
    use super::serde::Serialize;
    use crate::stdlib::net::error::NetError;
    
    pub fn to_string<T: Serialize>(_data: &T) -> Result<String, NetError> {
        // Placeholder implementation
        Ok("{}".to_string())
    }
}

mod urlencoding {
    pub fn encode(input: &str) -> String {
        // Simple URL encoding implementation
        input.chars()
            .map(|c| match c {
                'A'..='Z' | 'a'..='z' | '0'..='9' | '-' | '_' | '.' | '~' => c.to_string(),
                ' ' => "+".to_string(),
                _ => format!("%{:02X}", c as u8),
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_http_client_creation() {
        let client = HttpClient::new();
        assert!(client.is_ok());
    }

    #[test]
    fn test_http_client_builder() {
        let client = HttpClient::builder()
            .connect_timeout(Duration::from_secs(10))
            .user_agent("Test-Agent/1.0")
            .follow_redirects(true)
            .build();
        
        assert!(client.is_ok());
    }

    #[test]
    fn test_request_builder() {
        let client = HttpClient::new().unwrap();
        let mut headers = HashMap::new();
        headers.insert("key".to_string(), "value".to_string());
        
        let request_builder = client.post("http://example.com")
            .header("Custom-Header", "value")
            .content_type("application/json")
            .form(&headers);
        
        // The request builder should be created successfully
        assert_eq!(request_builder.request.method, Method::POST);
        assert_eq!(request_builder.request.url, "http://example.com");
    }

    #[test]
    fn test_url_parsing() {
        let client = HttpClient::new().unwrap();
        
        let url1 = client.parse_url("http://example.com/path").unwrap();
        assert_eq!(url1.host, "example.com");
        assert_eq!(url1.port, 80);
        assert_eq!(url1.path, "/path");
        assert!(!url1.is_https);
        
        let url2 = client.parse_url("https://example.com:8080/api").unwrap();
        assert_eq!(url2.host, "example.com");
        assert_eq!(url2.port, 8080);
        assert_eq!(url2.path, "/api");
        assert!(url2.is_https);
    }

    #[test]
    fn test_status_line_parsing() {
        let client = HttpClient::new().unwrap();
        
        let (version, status) = client.parse_status_line("HTTP/1.1 200 OK").unwrap();
        assert_eq!(version, HttpVersion::Http11);
        assert_eq!(status.as_u16(), 200);
        
        let result = client.parse_status_line("INVALID");
        assert!(result.is_err());
    }

    #[test]
    fn test_url_encoding() {
        assert_eq!(urlencoding::encode("hello world"), "hello+world");
        assert_eq!(urlencoding::encode("test@example.com"), "test%40example.com");
        assert_eq!(urlencoding::encode("normal"), "normal");
    }
}
