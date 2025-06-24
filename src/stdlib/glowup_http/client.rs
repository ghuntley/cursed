use crate::web::StatusCode;
// HTTP client implementation for GlowUpHTTP

use crate::stdlib::glowup_http::error::{GlowUpError, GlowUpResult};
use crate::stdlib::glowup_http::request::{VibeRequest, Method};
use crate::error::Error;
pub use crate::stdlib::glowup_http::response::VibeResponse;
use reqwest;
#[cfg(feature = "multipart")]
use reqwest::multipart;
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;
use tokio::runtime::Runtime;

/// Authentication types for HTTP requests
#[derive(Debug, Clone)]
pub enum AuthType {
    /// No authentication
    None,
    /// Basic authentication with username and password
    Basic { username: String, password: String },
    /// Bearer token authentication
    Bearer(String),
    /// Custom header authentication
    Custom { header: String, value: String },
}

/// Cookie jar management
#[derive(Debug, Clone)]
pub struct CookieJar {
    cookies: HashMap<String, String>,
}

impl CookieJar {
    pub fn new() -> Self {
        Self {
            cookies: HashMap::new(),
        }
    }
    
    pub fn add_cookie(&mut self, name: String, value: String) {
        self.cookies.insert(name, value);
    }
    
    pub fn get_cookies(&self) -> &HashMap<String, String> {
        &self.cookies
    }
}

/// Builder for configuring HTTP requests
#[derive(Debug)]
pub struct RequestBuilder<'a> {
    client: &'a VibeClient,
    method: Method,
    url: String,
    headers: HashMap<String, String>,
    query_params: HashMap<String, String>,
    body: Option<Vec<u8>>,
    json_body: Option<Value>,
    form_data: Option<HashMap<String, String>>,
    multipart_data: Option<HashMap<String, Vec<u8>>>,
    auth: AuthType,
    timeout: Option<Duration>,
}

impl<'a> RequestBuilder<'a> {
    pub fn new(client: &'a VibeClient, method: Method, url: impl Into<String>) -> Self {
        Self {
            client,
            method,
            url: url.into(),
            headers: HashMap::new(),
            query_params: HashMap::new(),
            body: None,
            json_body: None,
            form_data: None,
            multipart_data: None,
            auth: AuthType::None,
            timeout: None,
        }
    }
    
    /// Add a header to the request
    pub fn header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }
    
    /// Add multiple headers
    pub fn headers(mut self, headers: HashMap<String, String>) -> Self {
        self.headers.extend(headers);
        self
    }
    
    /// Add query parameter
    pub fn query(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.query_params.insert(key.into(), value.into());
        self
    }
    
    /// Add multiple query parameters
    pub fn queries(mut self, params: HashMap<String, String>) -> Self {
        self.query_params.extend(params);
        self
    }
    
    /// Set request body as bytes
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.body = Some(body);
        self
    }
    
    /// Set request body as text
    pub fn text(mut self, text: impl Into<String>) -> Self {
        self.body = Some(text.into().into_bytes());
        self
    }
    
    /// Set request body as JSON
    pub fn json(mut self, json: &Value) -> Self {
        self.json_body = Some(json.clone());
        self
    }
    
    /// Set form data
    pub fn form(mut self, form: HashMap<String, String>) -> Self {
        self.form_data = Some(form);
        self
    }
    
    /// Set multipart form data
    pub fn multipart(mut self, data: HashMap<String, Vec<u8>>) -> Self {
        self.multipart_data = Some(data);
        self
    }
    
    /// Set authentication
    pub fn auth(mut self, auth: AuthType) -> Self {
        self.auth = auth;
        self
    }
    
    /// Set basic authentication
    pub fn basic_auth(self, username: impl Into<String>, password: impl Into<String>) -> Self {
        self.auth(AuthType::Basic {
            username: username.into(),
            password: password.into(),
        })
    }
    
    /// Set bearer token authentication
    pub fn bearer_auth(self, token: impl Into<String>) -> Self {
        self.auth(AuthType::Bearer(token.into()))
    }
    
    /// Set timeout for this request
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = Some(timeout);
        self
    }
    
    /// Execute the request synchronously
    pub fn send_sync(self) -> GlowUpResult<VibeResponse> {
        let rt = Runtime::new().map_err(|e| GlowUpError::internal_error(&format!("Failed to create tokio runtime: {}", e)))?;
        rt.block_on(self.send())
    }
    
    /// Execute the request asynchronously
    pub async fn send(self) -> GlowUpResult<VibeResponse> {
        let mut req_builder = match self.method {
            Method::GET => self.client.inner_client.get(&self.url),
            Method::POST => self.client.inner_client.post(&self.url),
            Method::PUT => self.client.inner_client.put(&self.url),
            Method::DELETE => self.client.inner_client.delete(&self.url),
            Method::PATCH => self.client.inner_client.patch(&self.url),
            Method::HEAD => self.client.inner_client.head(&self.url),
            Method::OPTIONS => self.client.inner_client.request(reqwest::Method::OPTIONS, &self.url),
        };

        // Add query parameters
        if !self.query_params.is_empty() {
            let query: Vec<(String, String)> = self.query_params.into_iter().collect();
            req_builder = req_builder.query(&query);
        }

        // Add headers
        for (key, value) in &self.headers {
            req_builder = req_builder.header(key, value);
        }
        
        // Add default headers
        for (key, value) in &self.client.default_headers {
            if !self.headers.contains_key(key) {
                req_builder = req_builder.header(key, value);
            }
        }

        // Add authentication
        match &self.auth {
            AuthType::Basic { username, password } => {
                req_builder = req_builder.basic_auth(username, Some(password));
            },
            AuthType::Bearer(token) => {
                req_builder = req_builder.bearer_auth(token);
            },
            AuthType::Custom { header, value } => {
                req_builder = req_builder.header(header, value);
            },
            AuthType::None => {},
        }

        // Set body
        if let Some(json) = &self.json_body {
            req_builder = req_builder.json(json);
        } else if let Some(form) = &self.form_data {
            req_builder = req_builder.form(form);
        } else if let Some(body) = &self.body {
            req_builder = req_builder.body(body.clone());
        } else if let Some(multipart) = &self.multipart_data {
            #[cfg(feature = "multipart")]
            {
                let mut form = reqwest::multipart::Form::new();
                for (key, data) in multipart {
                    let part = reqwest::multipart::Part::bytes(data.clone())
                        .file_name(key.clone());
                    form = form.part(key.clone(), part);
                }
                req_builder = req_builder.multipart(form);
            }
            #[cfg(not(feature = "multipart"))]
            {
                return Err(GlowUpError::Other("Multipart support not available".to_string()));
            }
        }

        // Set timeout
        let timeout = self.timeout.unwrap_or(self.client.timeout);
        req_builder = req_builder.timeout(timeout);

        // Execute request
        let response = req_builder.send().await
            .map_err(|e| convert_reqwest_error(e))?;

        // Convert response
        convert_response(response).await
    }
}

/// HTTP client for making requests
/// This follows the CURSED spec's `VibeClient` naming
#[derive(Debug, Clone)]
pub struct VibeClient {
    /// Internal reqwest client
    inner_client: reqwest::Client,
    /// Request timeout
    pub timeout: Duration,
    /// User agent string
    pub user_agent: String,
    /// Default headers
    pub default_headers: HashMap<String, String>,
    /// Cookie jar
    pub cookie_jar: CookieJar,
    /// Follow redirects
    pub follow_redirects: bool,
    /// Maximum number of redirects
    pub max_redirects: u32,
    /// Proxy URL
    pub proxy: Option<String>,
}

impl VibeClient {
    /// Create a new HTTP client
    pub fn new() -> GlowUpResult<Self> {
        let client = reqwest::Client::builder()
            .user_agent("GlowUpHTTP/1.0")
            .timeout(Duration::from_secs(30))
            .redirect(reqwest::redirect::Policy::limited(10))
            .gzip(true)
            .build()
            .map_err(|e| GlowUpError::internal_error(&format!("Failed to create HTTP client: {}", e)))?;

        Ok(Self {
            inner_client: client,
            timeout: Duration::from_secs(30),
            user_agent: "GlowUpHTTP/1.0".to_string(),
            default_headers: HashMap::new(),
            cookie_jar: CookieJar::new(),
            follow_redirects: true,
            max_redirects: 10,
            proxy: None,
        })
    }
    
    /// Create a new client with custom configuration
    pub fn builder() -> VibeClientBuilder {
        VibeClientBuilder::new()
    }
    
    /// Set timeout for requests
    pub fn timeout(mut self, timeout: Duration) -> GlowUpResult<Self> {
        self.timeout = timeout;
        
        // Rebuild client with new timeout
        let mut client_builder = reqwest::Client::builder()
            .user_agent(&self.user_agent)
            .timeout(timeout)
            .gzip(true);
            
        if self.follow_redirects {
            client_builder = client_builder.redirect(reqwest::redirect::Policy::limited(self.max_redirects as usize));
        } else {
            client_builder = client_builder.redirect(reqwest::redirect::Policy::none());
        }
        
        if let Some(proxy) = &self.proxy {
            let proxy = reqwest::Proxy::all(proxy)
                .map_err(|e| GlowUpError::invalid_input(&format!("Invalid proxy URL: {}", e)))?;
            client_builder = client_builder.proxy(proxy);
        }
        
        self.inner_client = client_builder.build()
            .map_err(|e| GlowUpError::internal_error(&format!("Failed to rebuild HTTP client: {}", e)))?;
        
        Ok(self)
    }
    
    /// Set user agent
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> GlowUpResult<Self> {
        self.user_agent = user_agent.into();
        
        // Rebuild client with new user agent
        let mut client_builder = reqwest::Client::builder()
            .user_agent(&self.user_agent)
            .timeout(self.timeout)
            .gzip(true);
            
        if self.follow_redirects {
            client_builder = client_builder.redirect(reqwest::redirect::Policy::limited(self.max_redirects as usize));
        } else {
            client_builder = client_builder.redirect(reqwest::redirect::Policy::none());
        }
        
        if let Some(proxy) = &self.proxy {
            let proxy = reqwest::Proxy::all(proxy)
                .map_err(|e| GlowUpError::invalid_input(&format!("Invalid proxy URL: {}", e)))?;
            client_builder = client_builder.proxy(proxy);
        }
        
        self.inner_client = client_builder.build()
            .map_err(|e| GlowUpError::internal_error(&format!("Failed to rebuild HTTP client: {}", e)))?;
        
        Ok(self)
    }
    
    /// Add default header
    pub fn default_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.default_headers.insert(key.into(), value.into());
        self
    }
    
    /// Set proxy
    pub fn proxy(mut self, proxy_url: impl Into<String>) -> GlowUpResult<Self> {
        self.proxy = Some(proxy_url.into());
        
        // Rebuild client with proxy
        let proxy = reqwest::Proxy::all(self.proxy.as_ref().unwrap())
            .map_err(|e| GlowUpError::invalid_input(&format!("Invalid proxy URL: {}", e)))?;
            
        let mut client_builder = reqwest::Client::builder()
            .user_agent(&self.user_agent)
            .timeout(self.timeout)
            .proxy(proxy)
            .gzip(true);
            
        if self.follow_redirects {
            client_builder = client_builder.redirect(reqwest::redirect::Policy::limited(self.max_redirects as usize));
        } else {
            client_builder = client_builder.redirect(reqwest::redirect::Policy::none());
        }
        
        self.inner_client = client_builder.build()
            .map_err(|e| GlowUpError::internal_error(&format!("Failed to rebuild HTTP client: {}", e)))?;
        
        Ok(self)
    }
    
    /// Enable/disable following redirects
    pub fn follow_redirects(mut self, follow: bool, max_redirects: u32) -> GlowUpResult<Self> {
        self.follow_redirects = follow;
        self.max_redirects = max_redirects;
        
        // Rebuild client with redirect policy
        let mut client_builder = reqwest::Client::builder()
            .user_agent(&self.user_agent)
            .timeout(self.timeout)
            .gzip(true);
            
        if follow {
            client_builder = client_builder.redirect(reqwest::redirect::Policy::limited(max_redirects as usize));
        } else {
            client_builder = client_builder.redirect(reqwest::redirect::Policy::none());
        }
        
        if let Some(proxy) = &self.proxy {
            let proxy = reqwest::Proxy::all(proxy)
                .map_err(|e| GlowUpError::invalid_input(&format!("Invalid proxy URL: {}", e)))?;
            client_builder = client_builder.proxy(proxy);
        }
        
        self.inner_client = client_builder.build()
            .map_err(|e| GlowUpError::internal_error(&format!("Failed to rebuild HTTP client: {}", e)))?;
        
        Ok(self)
    }
    
    /// Create a request builder
    pub fn request(&self, method: Method, url: impl Into<String>) -> RequestBuilder {
        RequestBuilder::new(self, method, url)
    }
    
    /// Perform HTTP request (legacy method for compatibility)
    pub fn do_request(&self, req: &VibeRequest) -> GlowUpResult<VibeResponse> {
        let rt = Runtime::new().map_err(|e| GlowUpError::internal_error(&format!("Failed to create tokio runtime: {}", e)))?;
        rt.block_on(self.do_request_async(req))
    }
    
    /// Perform HTTP request asynchronously
    pub async fn do_request_async(&self, req: &VibeRequest) -> GlowUpResult<VibeResponse> {
        let mut builder = self.request(req.method.clone(), &req.url);
        
        // Add headers
        builder = builder.headers(req.header.clone());
        
        // Add body if present
        if !req.body.is_empty() {
            builder = builder.body(req.body.clone());
        }
        
        builder.send().await
    }
    
    /// GET request
    pub fn get(&self, url: &str) -> RequestBuilder {
        self.request(Method::GET, url)
    }
    
    /// POST request  
    pub fn post(&self, url: &str) -> RequestBuilder {
        self.request(Method::POST, url)
    }
    
    /// PUT request
    pub fn put(&self, url: &str) -> RequestBuilder {
        self.request(Method::PUT, url)
    }
    
    /// DELETE request
    pub fn delete(&self, url: &str) -> RequestBuilder {
        self.request(Method::DELETE, url)
    }
    
    /// PATCH request
    pub fn patch(&self, url: &str) -> RequestBuilder {
        self.request(Method::PATCH, url)
    }
    
    /// HEAD request
    pub fn head(&self, url: &str) -> RequestBuilder {
        self.request(Method::HEAD, url)
    }
    
    /// OPTIONS request
    pub fn options(&self, url: &str) -> RequestBuilder {
        self.request(Method::OPTIONS, url)
    }
    
    /// Simple GET request that returns the response directly
    pub fn get_simple(&self, url: &str) -> GlowUpResult<VibeResponse> {
        self.get(url).send_sync()
    }
    
    /// Simple POST request with JSON body
    pub fn post_json(&self, url: &str, json: &Value) -> GlowUpResult<VibeResponse> {
        self.post(url).json(json).send_sync()
    }
    
    /// Simple POST request with form data
    pub fn post_form(&self, url: &str, form: HashMap<String, String>) -> GlowUpResult<VibeResponse> {
        self.post(url).form(form).send_sync()
    }
}

/// Builder for creating VibeClient with custom configuration
pub struct VibeClientBuilder {
    timeout: Duration,
    user_agent: String,
    default_headers: HashMap<String, String>,
    follow_redirects: bool,
    max_redirects: u32,
    proxy: Option<String>,
    gzip: bool,
    cookie_store: bool,
}

impl VibeClientBuilder {
    pub fn new() -> Self {
        Self {
            timeout: Duration::from_secs(30),
            user_agent: "GlowUpHTTP/1.0".to_string(),
            default_headers: HashMap::new(),
            follow_redirects: true,
            max_redirects: 10,
            proxy: None,
            gzip: true,
            cookie_store: false,
        }
    }
    
    pub fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }
    
    pub fn user_agent(mut self, user_agent: impl Into<String>) -> Self {
        self.user_agent = user_agent.into();
        self
    }
    
    pub fn default_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.default_headers.insert(key.into(), value.into());
        self
    }
    
    pub fn proxy(mut self, proxy: impl Into<String>) -> Self {
        self.proxy = Some(proxy.into());
        self
    }
    
    pub fn redirect_policy(mut self, follow: bool, max: u32) -> Self {
        self.follow_redirects = follow;
        self.max_redirects = max;
        self
    }
    
    pub fn gzip(mut self, enabled: bool) -> Self {
        self.gzip = enabled;
        self
    }
    
    pub fn cookie_store(mut self, enabled: bool) -> Self {
        self.cookie_store = enabled;
        self
    }
    
    pub fn build(self) -> GlowUpResult<VibeClient> {
        let mut client_builder = reqwest::Client::builder()
            .user_agent(&self.user_agent)
            .timeout(self.timeout);
            
        if self.gzip {
            client_builder = client_builder.gzip(true);
        }
        
        if self.follow_redirects {
            client_builder = client_builder.redirect(reqwest::redirect::Policy::limited(self.max_redirects as usize));
        } else {
            client_builder = client_builder.redirect(reqwest::redirect::Policy::none());
        }
        
        if let Some(proxy) = &self.proxy {
            let proxy = reqwest::Proxy::all(proxy)
                .map_err(|e| GlowUpError::invalid_input(&format!("Invalid proxy URL: {}", e)))?;
            client_builder = client_builder.proxy(proxy);
        }
        
        if self.cookie_store {
            client_builder = client_builder.cookie_store(true);
        }
        
        let inner_client = client_builder.build()
            .map_err(|e| GlowUpError::internal_error(&format!("Failed to create HTTP client: {}", e)))?;

        Ok(VibeClient {
            inner_client,
            timeout: self.timeout,
            user_agent: self.user_agent,
            default_headers: self.default_headers,
            cookie_jar: CookieJar::new(),
            follow_redirects: self.follow_redirects,
            max_redirects: self.max_redirects,
            proxy: self.proxy,
        })
    }
}

/// Convert reqwest error to GlowUpError
fn convert_reqwest_error(error: reqwest::Error) -> GlowUpError {
    if error.is_timeout() {
        GlowUpError::timeout("Request timed out")
    } else if error.is_connect() {
        GlowUpError::connection_error(&format!("Connection error: {}", error))
    } else if error.is_request() {
        GlowUpError::invalid_request(&format!("Invalid request: {}", error))
    } else if let Some(status) = error.status() {
        match status.as_u16() {
            400 => GlowUpError::bad_request("Bad request"),
            401 => GlowUpError::unauthorized("Unauthorized"),
            403 => GlowUpError::forbidden("Forbidden"),
            404 => GlowUpError::not_found("Not found"),
            429 => GlowUpError::rate_limited("Rate limited"),
            500..=599 => GlowUpError::server_error(&format!("Server error: {}", status)),
            _ => GlowUpError::http_error(status.as_u16(), &format!("HTTP error: {}", status)),
        }
    } else {
        GlowUpError::internal_error(&format!("HTTP client error: {}", error))
    }
}

/// Convert reqwest response to VibeResponse
async fn convert_response(response: reqwest::Response) -> GlowUpResult<VibeResponse> {
    let status = response.status().as_u16();
    let mut headers = HashMap::new();
    
    // Convert headers
    for (name, value) in response.headers() {
        if let Ok(value_str) = value.to_str() {
            headers.insert(name.to_string(), value_str.to_string());
        }
    }
    
    // Get response body
    let body = response.bytes().await
        .map_err(|e| GlowUpError::internal_error(&format!("Failed to read response body: {}", e)))?
        .to_vec();
    
    Ok(VibeResponse {
        status,
        headers,
        body,
        status_line: format!("{} {}", status, StatusCode::from_u16(status).unwrap_or(StatusCode::OK).canonical_reason()),
        status_code: StatusCode::from_u16(status).unwrap_or(StatusCode::OK),
        proto: "HTTP/1.1".to_string(),
        content_length: body.len() as i64,
        transfer_encoding: vec![],
        close: false,
        uncompressed: true,
        trailer: HeaderMap::new(),
    })
}

impl Default for VibeClient {
    fn default() -> Self {
        Self::new().expect("Failed to create default HTTP client")
    }
}
