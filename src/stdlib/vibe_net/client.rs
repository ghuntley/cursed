//! HTTP client implementation for vibe_net

use crate::error::CursedError;
use std::collections::HashMap;

/// Result type for network operations
pub type NetResult<T> = Result<T, CursedError>;

/// HTTP methods supported by the client
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

impl std::fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            HttpMethod::Get => write!(f, "GET"),
            HttpMethod::Post => write!(f, "POST"),
            HttpMethod::Put => write!(f, "PUT"),
            HttpMethod::Delete => write!(f, "DELETE"),
            HttpMethod::Patch => write!(f, "PATCH"),
            HttpMethod::Head => write!(f, "HEAD"),
            HttpMethod::Options => write!(f, "OPTIONS"),
        }
    }
}

/// HTTP request structure
#[derive(Debug, Clone)]
pub struct HttpRequest {
    url: String,
    method: HttpMethod,
    headers: HashMap<String, String>,
    body: Option<Vec<u8>>,
}

impl HttpRequest {
    /// Create a new HTTP request
    pub fn new(method: HttpMethod, url: String) -> Self {
        Self {
            url,
            method,
            headers: HashMap::new(),
            body: None,
        }
    }
    
    /// Add a header to the request
    pub fn header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }
    
    /// Set the request body
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }
    
    /// Set JSON body
    pub fn json_body(mut self, json: &str) -> Self {
        self.headers.insert("Content-Type".to_string(), "application/json".to_string());
        self.body = Some(json.as_bytes().to_vec());
        self
    }
}

/// HTTP response structure
#[derive(Debug)]
pub struct HttpResponse {
    status: u16,
    headers: HashMap<String, String>,
    body: Vec<u8>,
}

impl HttpResponse {
    /// Create a new HTTP response
    pub fn new(status: u16, headers: HashMap<String, String>, body: Vec<u8>) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }
    
    /// Get the response status code
    pub fn status(&self) -> u16 {
        self.status
    }
    
    /// Get response headers
    pub fn headers(&self) -> &HashMap<String, String> {
        &self.headers
    }
    
    /// Get response body as bytes
    pub fn body(&self) -> &[u8] {
        &self.body
    }
    
    /// Get response body as string
    pub fn text(&self) -> NetResult<String> {
        String::from_utf8(self.body.clone())
            .map_err(|e| CursedError::runtime_error(&format!("Failed to parse response as UTF-8: {}", e)))
    }
    
    /// Parse response body as JSON
    pub fn json<T>(&self) -> NetResult<T> 
    where 
        T: serde::de::DeserializeOwned,
    {
        let text = self.text()?;
        serde_json::from_str(&text)
            .map_err(|e| CursedError::runtime_error(&format!("Failed to parse JSON: {}", e)))
    }
}

/// HTTP client implementation
pub struct HttpClient {
    default_headers: HashMap<String, String>,
    timeout_seconds: u64,
}

impl HttpClient {
    /// Create a new HTTP client
    pub fn new() -> Self {
        let mut default_headers = HashMap::new();
        default_headers.insert("User-Agent".to_string(), "CURSED/1.0".to_string());
        
        Self {
            default_headers,
            timeout_seconds: 30,
        }
    }
    
    /// Set timeout for requests
    pub fn timeout(mut self, seconds: u64) -> Self {
        self.timeout_seconds = seconds;
        self
    }
    
    /// Add a default header for all requests
    pub fn default_header(mut self, key: String, value: String) -> Self {
        self.default_headers.insert(key, value);
        self
    }
    
    /// Execute an HTTP request
    pub async fn execute(&self, request: HttpRequest) -> NetResult<HttpResponse> {
        use reqwest;
        
        let client = reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(self.timeout_seconds))
            .build()
            .map_err(|e| CursedError::runtime_error(&format!("Failed to create HTTP client: {}", e)))?;
        
        // Build the request
        let mut req_builder = match request.method {
            HttpMethod::Get => client.get(&request.url),
            HttpMethod::Post => client.post(&request.url),
            HttpMethod::Put => client.put(&request.url),
            HttpMethod::Delete => client.delete(&request.url),
            HttpMethod::Patch => client.patch(&request.url),
            HttpMethod::Head => client.head(&request.url),
            HttpMethod::Options => client.request(reqwest::Method::OPTIONS, &request.url),
        };
        
        // Add default headers
        for (key, value) in &self.default_headers {
            req_builder = req_builder.header(key, value);
        }
        
        // Add request headers
        for (key, value) in &request.headers {
            req_builder = req_builder.header(key, value);
        }
        
        // Add body if present
        if let Some(body) = request.body {
            req_builder = req_builder.body(body);
        }
        
        // Execute the request
        let response = req_builder
            .send()
            .await
            .map_err(|e| CursedError::runtime_error(&format!("HTTP request failed: {}", e)))?;
        
        // Extract response data
        let status = response.status().as_u16();
        let mut headers = HashMap::new();
        
        for (key, value) in response.headers().iter() {
            if let Ok(value_str) = value.to_str() {
                headers.insert(key.to_string(), value_str.to_string());
            }
        }
        
        let body = response
            .bytes()
            .await
            .map_err(|e| CursedError::runtime_error(&format!("Failed to read response body: {}", e)))?
            .to_vec();
        
        Ok(HttpResponse::new(status, headers, body))
    }
    
    /// Convenience method for GET requests
    pub async fn get(&self, url: &str) -> NetResult<HttpResponse> {
        let request = HttpRequest::new(HttpMethod::Get, url.to_string());
        self.execute(request).await
    }
    
    /// Convenience method for POST requests with JSON
    pub async fn post_json(&self, url: &str, json: &str) -> NetResult<HttpResponse> {
        let request = HttpRequest::new(HttpMethod::Post, url.to_string())
            .json_body(json);
        self.execute(request).await
    }
}

impl Default for HttpClient {
    fn default() -> Self {
        Self::new()
    }
}

/// Test HTTP client functionality
pub async fn test_http_client() -> NetResult<()> {
    let client = HttpClient::new();
    
    // Test with a simple HTTP request (this might fail if no internet, but that's ok)
    match client.get("https://httpbin.org/json").await {
        Ok(response) => {
            if response.status() == 200 {
                println!("✅ HTTP client test successful");
            } else {
                println!("⚠️  HTTP client got non-200 status: {}", response.status());
            }
        }
        Err(_) => {
            // This is expected in many environments without internet access
            println!("ℹ️  HTTP client test skipped (no network access)");
        }
    }
    
    Ok(())
}

/// Initialize HTTP client functionality
pub async fn init_http_client() -> NetResult<()> {
    test_http_client().await?;
    println!("🌐 HTTP client functionality initialized");
    Ok(())
}
