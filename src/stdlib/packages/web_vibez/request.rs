use crate::error::CursedError;
/// fr fr HTTP request handling for web_vibez - comprehensive request processing
use std::collections::HashMap;
use std::net::SocketAddr;
use std::fmt;
use url::Url;

// use crate::stdlib::packages::web_vibez::{
    method::HttpMethod,
    types::{Headers, QueryParams, FormData, RequestBody, Cookie},
    error::{WebError, WebResult},
};

/// fr fr HTTP request representation - everything about an incoming request
#[derive(Debug, Clone)]
pub struct HttpRequest {
    /// fr fr HTTP method (GET, POST, etc.)
    pub method: HttpMethod,
    /// fr fr Request URI path
    pub path: String,
    /// fr fr Query parameters from URL
    pub query: QueryParams,
    /// fr fr HTTP headers
    pub headers: Headers,
    /// fr fr Request body content
    pub body: RequestBody,
    /// fr fr HTTP version string
    pub version: String,
    /// fr fr Client's IP address and port
    pub remote_addr: Option<SocketAddr>,
    /// fr fr Cookies from the request
    pub cookies: Vec<Cookie>,
    /// fr fr Custom extensions for middleware data
    pub extensions: HashMap<String, serde_json::Value>,
}

impl HttpRequest {
    /// fr fr Create new HTTP request - basic setup
    pub fn new(method: HttpMethod, path: String) -> Self {
        Self {
            method,
            path,
            query: QueryParams::new(),
            headers: Headers::new(),
            body: RequestBody::Empty,
            version: "HTTP/1.1".to_string(),
            remote_addr: None,
            cookies: Vec::new(),
            extensions: HashMap::new(),
        }
    }

    /// fr fr Get header value by name - case insensitive lookup
    pub fn header(&self, name: &str) -> Option<&String> {
        // HTTP headers are case-insensitive
        self.headers
            .iter()
            .find(|(key, _)| key.to_lowercase() == name.to_lowercase())
            .map(|(_, value)| value)
    }

    /// fr fr Get all header values for a name - handles multiple values
    pub fn headers_all(&self, name: &str) -> Vec<&String> {
        self.headers
            .iter()
            .filter(|(key, _)| key.to_lowercase() == name.to_lowercase())
            .map(|(_, value)| value)
            .collect()
    }

    /// fr fr Get query parameter value - URL parameters (first value for backward compatibility)
    pub fn query_param(&self, name: &str) -> Option<&String> {
        self.query.get_first(name)
    }

    /// fr fr Get all query parameter values - handles array parameters like ?tags=rust&tags=web
    pub fn query_params_all(&self, name: &str) -> Vec<&String> {
        self.query.get_all(name)
    }

    /// fr fr Get query parameter as array - specifically for multi-value parameters
    pub fn query_array(&self, name: &str) -> Vec<&String> {
        self.query.get_all(name)
    }

    /// fr fr Get cookie by name - find specific cookie
    pub fn cookie(&self, name: &str) -> Option<&Cookie> {
        self.cookies.iter().find(|c| c.name == name)
    }

    /// fr fr Get cookie value by name - just the value
    pub fn cookie_value(&self, name: &str) -> Option<&String> {
        self.cookie(name).map(|c| &c.value)
    }

    /// fr fr Check if request has body content - quick validation
    pub fn has_body(&self) -> bool {
        !self.body.is_empty()
    }

    /// fr fr Get content type from headers - what kind of data
    pub fn content_type(&self) -> Option<&String> {
        self.header("content-type")
    }

    /// fr fr Get content length from headers - how much data
    pub fn content_length(&self) -> Option<usize> {
        self.header("content-length")?.parse().ok()
    }

    /// fr fr Check if request expects JSON response - accept header check
    pub fn expects_json(&self) -> bool {
        if let Some(accept) = self.header("accept") {
            accept.contains("application/json") || accept.contains("*/*")
        } else {
            false
        }
    }

    /// fr fr Check if request is AJAX - common indicators
    pub fn is_ajax(&self) -> bool {
        if let Some(requested_with) = self.header("x-requested-with") {
            requested_with.to_lowercase() == "xmlhttprequest"
        } else {
            false
        }
    }

    /// fr fr Get client IP address - real IP handling
    pub fn client_ip(&self) -> Option<std::net::IpAddr> {
        // Check X-Forwarded-For header first (proxy/load balancer)
        if let Some(forwarded) = self.header("x-forwarded-for") {
            if let Some(ip_str) = forwarded.split(',').next() {
                if let Ok(ip) = ip_str.trim().parse() {
                    return Some(ip);
                }
            }
        }

        // Check X-Real-IP header (nginx)
        if let Some(real_ip) = self.header("x-real-ip") {
            if let Ok(ip) = real_ip.parse() {
                return Some(ip);
            }
        }

        // Fall back to remote address
        self.remote_addr.map(|addr| addr.ip())
    }

    /// fr fr Get user agent string - browser/client info
    pub fn user_agent(&self) -> Option<&String> {
        self.header("user-agent")
    }

    /// fr fr Get referer URL - where they came from
    pub fn referer(&self) -> Option<&String> {
        self.header("referer").or_else(|| self.header("referrer"))
    }

    /// fr fr Parse body as JSON - structured data extraction
    pub fn json<T>(&self) -> WebResult<T>
    where
        T: serde::de::DeserializeOwned,
    {
        match &self.body {
            RequestBody::Json(value) => {
                serde_json::from_value(value.clone()).map_err(|e| {
                    WebError::Json {
                        message: format!("Failed to deserialize JSON: {}", e),
                        path: None,
                    }
                })
            }
            RequestBody::Text(text) => {
                serde_json::from_str(text).map_err(|e| {
                    WebError::Json {
                        message: format!("Failed to parse JSON from text: {}", e),
                        path: None,
                    }
                })
            }
            _ => Err(WebError::bad_request("Request body is not JSON")),
        }
    }

    /// fr fr Parse body as form data - URL-encoded forms
    pub fn form(&self) -> WebResult<&FormData> {
        match &self.body {
            RequestBody::Form(form) => Ok(form),
            _ => Err(WebError::bad_request("Request body is not form data")),
        }
    }

    /// fr fr Get body as text - string representation
    pub fn text(&self) -> WebResult<String> {
        self.body.to_string().map_err(|e| {
            WebError::RequestParsing {
                message: format!("Failed to convert body to text: {}", e),
                field: Some("body".to_string()),
            }
        })
    }

    /// fr fr Get body as bytes - raw binary data
    pub fn bytes(&self) -> WebResult<Vec<u8>> {
        match &self.body {
            RequestBody::Binary(bytes) => Ok(bytes.clone()),
            RequestBody::Text(text) => Ok(text.as_bytes().to_vec()),
            _ => {
                let text = self.text()?;
                Ok(text.into_bytes())
            }
        }
    }

    /// fr fr Set extension data - middleware storage
    pub fn set_extension(&mut self, key: String, value: serde_json::Value) {
        self.extensions.insert(key, value);
    }

    /// fr fr Get extension data - retrieve middleware data
    pub fn extension(&self, key: &str) -> Option<&serde_json::Value> {
        self.extensions.get(key)
    }

    /// fr fr Parse full URL with host - reconstruct complete URL
    pub fn url(&self) -> WebResult<Url> {
        let host = self
            .header("host")
            .ok_or_else(|| WebError::bad_request("Missing Host header"))?;

        let scheme = if self.is_secure() { "https" } else { "http" };
        let url_str = format!("{}://{}{}", scheme, host, self.path);

        if !self.query.is_empty() {
            let query_string = self
                .query
                .iter()
                .flat_map(|(k, values)| values.iter().map(move |v| format!("{}={}", k, v)))
                .collect::<Vec<_>>()
                .join("&");
            let url_str = format!("{}?{}", url_str, query_string);
            Url::parse(&url_str)
        } else {
            Url::parse(&url_str)
        }
        .map_err(|e| {
            WebError::RequestParsing {
                message: format!("Failed to parse URL: {}", e),
                field: Some("url".to_string()),
            }
        })
    }

    /// fr fr Check if request is secure (HTTPS) - security validation
    pub fn is_secure(&self) -> bool {
        // Check X-Forwarded-Proto header (load balancer)
        if let Some(proto) = self.header("x-forwarded-proto") {
            return proto.to_lowercase() == "https";
        }

        // Check X-Forwarded-SSL header
        if let Some(ssl) = self.header("x-forwarded-ssl") {
            return ssl.to_lowercase() == "on";
        }

        // Default to false - real implementation would check connection
        false
    }
}

/// fr fr Request builder for creating requests - fluent API
#[derive(Debug)]
pub struct RequestBuilder {
    request: HttpRequest,
}

impl RequestBuilder {
    /// fr fr Create new request builder - start building
    pub fn new(method: HttpMethod, path: String) -> Self {
        Self {
            request: HttpRequest::new(method, path),
        }
    }

    /// fr fr Add header to request - chain building
    pub fn header(mut self, name: String, value: String) -> Self {
        self.request.headers.insert(name, value);
        self
    }

    /// fr fr Add query parameter - URL parameters (replaces existing values)
    pub fn query(mut self, name: String, value: String) -> Self {
        self.request.query.insert_single(name, value);
        self
    }

    /// fr fr Add query parameter value - adds to existing values for array parameters
    pub fn query_add(mut self, name: String, value: String) -> Self {
        self.request.query.add_value(name, value);
        self
    }

    /// fr fr Set query parameter array - multiple values at once
    pub fn query_array(mut self, name: String, values: Vec<String>) -> Self {
        self.request.query.insert(name, values);
        self
    }

    /// fr fr Set request body - data payload
    pub fn body(mut self, body: RequestBody) -> Self {
        self.request.body = body;
        self
    }

    /// fr fr Set JSON body - structured data
    pub fn json<T: serde::Serialize>(mut self, value: &T) -> WebResult<Self> {
        let json_value = serde_json::to_value(value).map_err(|e| {
            WebError::Json {
                message: format!("Failed to serialize JSON: {}", e),
                path: None,
            }
        })?;
        self.request.body = RequestBody::Json(json_value);
        self.request
            .headers
            .insert("content-type".to_string(), "application/json".to_string());
        Ok(self)
    }

    /// fr fr Set form body - URL-encoded data
    pub fn form(mut self, form_data: FormData) -> Self {
        self.request.body = RequestBody::Form(form_data);
        self.request.headers.insert(
            "content-type".to_string(),
            "application/x-www-form-urlencoded".to_string(),
        );
        self
    }

    /// fr fr Set text body - plain text
    pub fn text(mut self, text: String) -> Self {
        self.request.body = RequestBody::Text(text);
        self.request
            .headers
            .insert("content-type".to_string(), "text/plain".to_string());
        self
    }

    /// fr fr Add cookie to request - client cookies
    pub fn cookie(mut self, cookie: Cookie) -> Self {
        self.request.cookies.push(cookie);
        self
    }

    /// fr fr Set remote address - client info
    pub fn remote_addr(mut self, addr: SocketAddr) -> Self {
        self.request.remote_addr = Some(addr);
        self
    }

    /// fr fr Set HTTP version - protocol version
    pub fn version(mut self, version: String) -> Self {
        self.request.version = version;
        self
    }

    /// fr fr Build the final request - get result
    pub fn build(self) -> HttpRequest {
        self.request
    }
}

impl fmt::Display for HttpRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.method, self.path)?;
        if !self.query.is_empty() {
            write!(f, "?")?;
            let query_pairs: Vec<String> = self
                .query
                .iter()
                .flat_map(|(k, values)| values.iter().map(move |v| format!("{}={}", k, v)))
                .collect();
            write!(f, "{}", query_pairs.join("&"))?;
        }
        write!(f, " {}", self.version)
    }
}

