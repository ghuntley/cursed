use crate::error::Error;
// Web module for CURSED HTTP server functionality
use std::collections::HashMap;
use std::fmt;

/// HTTP status codes
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum StatusCode {
    // 1xx Informational
    Continue = 100,
    SwitchingProtocols = 101,
    Processing = 102,
    
    // 2xx Success
    OK = 200,
    Created = 201,
    Accepted = 202,
    NonAuthoritativeInformation = 203,
    NoContent = 204,
    ResetContent = 205,
    PartialContent = 206,
    
    // 3xx Redirection
    MultipleChoices = 300,
    MovedPermanently = 301,
    Found = 302,
    SeeOther = 303,
    NotModified = 304,
    UseProxy = 305,
    TemporaryRedirect = 307,
    PermanentRedirect = 308,
    
    // 4xx Client Error
    BadRequest = 400,
    Unauthorized = 401,
    PaymentRequired = 402,
    Forbidden = 403,
    NotFound = 404,
    MethodNotAllowed = 405,
    NotAcceptable = 406,
    ProxyAuthenticationRequired = 407,
    RequestTimeout = 408,
    Conflict = 409,
    Gone = 410,
    LengthRequired = 411,
    PreconditionFailed = 412,
    PayloadTooLarge = 413,
    UriTooLong = 414,
    UnsupportedMediaType = 415,
    RangeNotSatisfiable = 416,
    ExpectationFailed = 417,
    ImATeapot = 418,
    UnprocessableEntity = 422,
    Locked = 423,
    FailedDependency = 424,
    UpgradeRequired = 426,
    PreconditionRequired = 428,
    TooManyRequests = 429,
    RequestHeaderFieldsTooLarge = 431,
    UnavailableForLegalReasons = 451,
    
    // 5xx Server Error
    InternalServerError = 500,
    NotImplemented = 501,
    BadGateway = 502,
    ServiceUnavailable = 503,
    GatewayTimeout = 504,
    HttpVersionNotSupported = 505,
    VariantAlsoNegotiates = 506,
    InsufficientStorage = 507,
    LoopDetected = 508,
    NotExtended = 510,
    NetworkAuthenticationRequired = 511,
}

impl StatusCode {
    pub fn as_u16(&self) -> u16 {
        *self as u16
    }

    /// Create StatusCode from u16 value
    pub fn from_u16(code: u16) -> Option<Self> {
        match code {
            100 => Some(StatusCode::Continue),
            101 => Some(StatusCode::SwitchingProtocols),
            102 => Some(StatusCode::Processing),
            200 => Some(StatusCode::OK),
            201 => Some(StatusCode::Created),
            202 => Some(StatusCode::Accepted),
            203 => Some(StatusCode::NonAuthoritativeInformation),
            204 => Some(StatusCode::NoContent),
            205 => Some(StatusCode::ResetContent),
            206 => Some(StatusCode::PartialContent),
            300 => Some(StatusCode::MultipleChoices),
            301 => Some(StatusCode::MovedPermanently),
            302 => Some(StatusCode::Found),
            303 => Some(StatusCode::SeeOther),
            304 => Some(StatusCode::NotModified),
            305 => Some(StatusCode::UseProxy),
            307 => Some(StatusCode::TemporaryRedirect),
            308 => Some(StatusCode::PermanentRedirect),
            400 => Some(StatusCode::BadRequest),
            401 => Some(StatusCode::Unauthorized),
            402 => Some(StatusCode::PaymentRequired),
            403 => Some(StatusCode::Forbidden),
            404 => Some(StatusCode::NotFound),
            405 => Some(StatusCode::MethodNotAllowed),
            406 => Some(StatusCode::NotAcceptable),
            407 => Some(StatusCode::ProxyAuthenticationRequired),
            408 => Some(StatusCode::RequestTimeout),
            409 => Some(StatusCode::Conflict),
            410 => Some(StatusCode::Gone),
            411 => Some(StatusCode::LengthRequired),
            412 => Some(StatusCode::PreconditionFailed),
            413 => Some(StatusCode::PayloadTooLarge),
            414 => Some(StatusCode::UriTooLong),
            415 => Some(StatusCode::UnsupportedMediaType),
            416 => Some(StatusCode::RangeNotSatisfiable),
            417 => Some(StatusCode::ExpectationFailed),
            418 => Some(StatusCode::ImATeapot),
            422 => Some(StatusCode::UnprocessableEntity),
            429 => Some(StatusCode::TooManyRequests),
            500 => Some(StatusCode::InternalServerError),
            501 => Some(StatusCode::NotImplemented),
            502 => Some(StatusCode::BadGateway),
            503 => Some(StatusCode::ServiceUnavailable),
            504 => Some(StatusCode::GatewayTimeout),
            505 => Some(StatusCode::HttpVersionNotSupported),
            _ => None,
        }
    }

    /// Alias for reason_phrase for compatibility
    pub fn canonical_reason(&self) -> &'static str {
        self.reason_phrase()
    }
    
    pub fn reason_phrase(&self) -> &'static str {
        match self {
            StatusCode::Continue => "Continue",
            StatusCode::SwitchingProtocols => "Switching Protocols",
            StatusCode::Processing => "Processing",
            StatusCode::OK => "OK",
            StatusCode::Created => "Created",
            StatusCode::Accepted => "Accepted",
            StatusCode::NonAuthoritativeInformation => "Non-Authoritative Information",
            StatusCode::NoContent => "No Content",
            StatusCode::ResetContent => "Reset Content",
            StatusCode::PartialContent => "Partial Content",
            StatusCode::MultipleChoices => "Multiple Choices",
            StatusCode::MovedPermanently => "Moved Permanently",
            StatusCode::Found => "Found",
            StatusCode::SeeOther => "See Other",
            StatusCode::NotModified => "Not Modified",
            StatusCode::UseProxy => "Use Proxy",
            StatusCode::TemporaryRedirect => "Temporary Redirect",
            StatusCode::PermanentRedirect => "Permanent Redirect",
            StatusCode::BadRequest => "Bad Request",
            StatusCode::Unauthorized => "Unauthorized",
            StatusCode::PaymentRequired => "Payment Required",
            StatusCode::Forbidden => "Forbidden",
            StatusCode::NotFound => "Not Found",
            StatusCode::MethodNotAllowed => "Method Not Allowed",
            StatusCode::NotAcceptable => "Not Acceptable",
            StatusCode::ProxyAuthenticationRequired => "Proxy Authentication Required",
            StatusCode::RequestTimeout => "Request Timeout",
            StatusCode::Conflict => "Conflict",
            StatusCode::Gone => "Gone",
            StatusCode::LengthRequired => "Length Required",
            StatusCode::PreconditionFailed => "Precondition Failed",
            StatusCode::PayloadTooLarge => "Payload Too Large",
            StatusCode::UriTooLong => "URI Too Long",
            StatusCode::UnsupportedMediaType => "Unsupported Media Type",
            StatusCode::RangeNotSatisfiable => "Range Not Satisfiable",
            StatusCode::ExpectationFailed => "Expectation Failed",
            StatusCode::ImATeapot => "I'm a teapot",
            StatusCode::UnprocessableEntity => "Unprocessable Entity",
            StatusCode::Locked => "Locked",
            StatusCode::FailedDependency => "Failed Dependency",
            StatusCode::UpgradeRequired => "Upgrade Required",
            StatusCode::PreconditionRequired => "Precondition Required",
            StatusCode::TooManyRequests => "Too Many Requests",
            StatusCode::RequestHeaderFieldsTooLarge => "Request Header Fields Too Large",
            StatusCode::UnavailableForLegalReasons => "Unavailable For Legal Reasons",
            StatusCode::InternalServerError => "Internal Server Error",
            StatusCode::NotImplemented => "Not Implemented",
            StatusCode::BadGateway => "Bad Gateway",
            StatusCode::ServiceUnavailable => "Service Unavailable",
            StatusCode::GatewayTimeout => "Gateway Timeout",
            StatusCode::HttpVersionNotSupported => "HTTP Version Not Supported",
            StatusCode::VariantAlsoNegotiates => "Variant Also Negotiates",
            StatusCode::InsufficientStorage => "Insufficient Storage",
            StatusCode::LoopDetected => "Loop Detected",
            StatusCode::NotExtended => "Not Extended",
            StatusCode::NetworkAuthenticationRequired => "Network Authentication Required",
        }
    }
    
    pub fn is_success(&self) -> bool {
        matches!(self.as_u16(), 200..=299)
    }
    
    pub fn is_redirection(&self) -> bool {
        matches!(self.as_u16(), 300..=399)
    }
    
    pub fn is_client_error(&self) -> bool {
        matches!(self.as_u16(), 400..=499)
    }
    
    pub fn is_server_error(&self) -> bool {
        matches!(self.as_u16(), 500..=599)
    }
}

impl fmt::Display for StatusCode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {}", self.as_u16(), self.reason_phrase())
    }
}

/// HTTP methods
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Method {
    GET,
    POST,
    PUT,
    DELETE,
    HEAD,
    OPTIONS,
    PATCH,
    TRACE,
    CONNECT,
}

impl fmt::Display for Method {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

/// HTTP headers
#[derive(Debug, Clone)]
pub struct Headers {
    inner: HashMap<String, String>,
}

impl Headers {
    pub fn new() -> Self {
        Self {
            inner: HashMap::new(),
        }
    }
    
    pub fn insert(&mut self, key: String, value: String) {
        self.inner.insert(key.to_lowercase(), value);
    }
    
    pub fn get(&self, key: &str) -> Option<&String> {
        self.inner.get(&key.to_lowercase())
    }
    
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.inner.remove(&key.to_lowercase())
    }
    
    pub fn iter(&self) -> impl Iterator<Item = (&String, &String)> {
        self.inner.iter()
    }
}

impl Default for Headers {
    fn default() -> Self {
        Self::new()
    }
}

/// HTTP request
#[derive(Debug, Clone)]
pub struct Request {
    pub method: Method,
    pub path: String,
    pub headers: Headers,
    pub body: Vec<u8>,
    pub query_params: HashMap<String, String>,
}

impl Request {
    pub fn new(method: Method, path: String) -> Self {
        Self {
            method,
            path,
            headers: Headers::new(),
            body: Vec::new(),
            query_params: HashMap::new(),
        }
    }
    
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }
    
    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self
    }
    
    pub fn with_query_param(mut self, key: String, value: String) -> Self {
        self.query_params.insert(key, value);
        self
    }
    
    pub fn body_as_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.body.clone())
    }
}

/// HTTP response
#[derive(Debug, Clone)]
pub struct Response {
    pub status: StatusCode,
    pub headers: Headers,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(status: StatusCode) -> Self {
        Self {
            status,
            headers: Headers::new(),
            body: Vec::new(),
        }
    }
    
    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.insert(key, value);
        self
    }
    
    pub fn with_body(mut self, body: Vec<u8>) -> Self {
        self.body = body;
        self
    }
    
    pub fn with_text(mut self, text: String) -> Self {
        self.body = text.into_bytes();
        self.headers.insert("Content-Type".to_string(), "text/plain".to_string());
        self
    }
    
    pub fn with_json(mut self, json: String) -> Self {
        self.body = json.into_bytes();
        self.headers.insert("Content-Type".to_string(), "application/json".to_string());
        self
    }
    
    pub fn with_html(mut self, html: String) -> Self {
        self.body = html.into_bytes();
        self.headers.insert("Content-Type".to_string(), "text/html".to_string());
        self
    }
    
    pub fn set_status(&mut self, status: StatusCode) {
        self.status = status;
    }
    
    pub fn body_as_string(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.body.clone())
    }
}

impl Default for Response {
    fn default() -> Self {
        Self::new(StatusCode::OK)
    }
}

/// Session management
#[derive(Debug, Clone)]
pub enum SameSitePolicy {
    Strict,
    Lax,
    None,
}

#[derive(Debug, Clone)]
pub enum SessionStoreType {
    Memory,
    File(String),
    Redis(String),
}

#[derive(Debug, Clone)]
pub struct SessionConfig {
    pub store_type: SessionStoreType,
    pub cookie_name: String,
    pub max_age: Option<u64>,
    pub secure: bool,
    pub http_only: bool,
    pub same_site: SameSitePolicy,
}

impl Default for SessionConfig {
    fn default() -> Self {
        Self {
            store_type: SessionStoreType::Memory,
            cookie_name: "cursed_session".to_string(),
            max_age: Some(3600), // 1 hour
            secure: false,
            http_only: true,
            same_site: SameSitePolicy::Lax,
        }
    }
}

/// Session data
#[derive(Debug, Clone)]
pub struct Session {
    pub id: String,
    pub data: HashMap<String, String>,
    pub created_at: std::time::SystemTime,
    pub expires_at: Option<std::time::SystemTime>,
}

impl Session {
    pub fn new(id: String) -> Self {
        Self {
            id,
            data: HashMap::new(),
            created_at: std::time::SystemTime::now(),
            expires_at: None,
        }
    }
    
    pub fn set(&mut self, key: String, value: String) {
        self.data.insert(key, value);
    }
    
    pub fn get(&self, key: &str) -> Option<&String> {
        self.data.get(key)
    }
    
    pub fn remove(&mut self, key: &str) -> Option<String> {
        self.data.remove(key)
    }
    
    pub fn is_expired(&self) -> bool {
        if let Some(expires) = self.expires_at {
            std::time::SystemTime::now() > expires
        } else {
            false
        }
    }
}

/// Server configuration
#[derive(Debug, Clone)]
pub struct ServerConfig {
    pub host: String,
    pub port: u16,
    pub max_connections: usize,
    pub read_timeout: Option<std::time::Duration>,
    pub write_timeout: Option<std::time::Duration>,
    pub keep_alive: bool,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            host: "127.0.0.1".to_string(),
            port: 8080,
            max_connections: 1000,
            read_timeout: Some(std::time::Duration::from_secs(30)),
            write_timeout: Some(std::time::Duration::from_secs(30)),
            keep_alive: true,
        }
    }
}

/// Timeout configuration
#[derive(Debug, Clone)]
pub struct TimeoutConfig {
    pub request_timeout: std::time::Duration,
    pub response_timeout: std::time::Duration,
    pub idle_timeout: std::time::Duration,
    pub keep_alive_timeout: std::time::Duration,
}

impl Default for TimeoutConfig {
    fn default() -> Self {
        Self {
            request_timeout: std::time::Duration::from_secs(30),
            response_timeout: std::time::Duration::from_secs(30),
            idle_timeout: std::time::Duration::from_secs(60),
            keep_alive_timeout: std::time::Duration::from_secs(120),
        }
    }
}

/// Timeout errors
#[derive(Debug, Clone)]
pub enum TimeoutError {
    RequestTimeout { elapsed: std::time::Duration, timeout: std::time::Duration },
    ResponseTimeout { elapsed: std::time::Duration, timeout: std::time::Duration },
    DatabaseTimeout { elapsed: std::time::Duration, timeout: std::time::Duration, operation: String },
    ConnectionTimeout { elapsed: std::time::Duration, timeout: std::time::Duration },
}

impl fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            TimeoutError::RequestTimeout { elapsed, timeout } => {
                write!(f, "Request timeout: elapsed {:?}, timeout {:?}", elapsed, timeout)
            }
            TimeoutError::ResponseTimeout { elapsed, timeout } => {
                write!(f, "Response timeout: elapsed {:?}, timeout {:?}", elapsed, timeout)
            }
            TimeoutError::DatabaseTimeout { elapsed, timeout, operation } => {
                write!(f, "Database timeout for {}: elapsed {:?}, timeout {:?}", operation, elapsed, timeout)
            }
            TimeoutError::ConnectionTimeout { elapsed, timeout } => {
                write!(f, "Connection timeout: elapsed {:?}, timeout {:?}", elapsed, timeout)
            }
        }
    }
}

impl std::error::Error for TimeoutError {}

/// Middleware trait
pub trait Middleware: Send + Sync {
    fn handle(&self, request: &mut Request, response: &mut Response) -> Result<(), Box<dyn std::error::Error>>;
}

/// Middleware chain
#[derive(Debug)]
pub struct MiddlewareChain {
    middlewares: Vec<Box<dyn Middleware>>,
}

impl MiddlewareChain {
    pub fn new(middlewares: Vec<Box<dyn Middleware>>) -> Self {
        Self { middlewares }
    }
    
    pub fn add_middleware(&mut self, middleware: Box<dyn Middleware>) {
        self.middlewares.push(middleware);
    }
    
    pub fn handle(&self, request: &mut Request, response: &mut Response) -> Result<(), Box<dyn std::error::Error>> {
        for middleware in &self.middlewares {
            middleware.handle(request, response)?;
        }
        Ok(())
    }
}

/// Basic middleware implementations
#[derive(Debug)]
pub struct LoggingMiddleware {
    enabled: bool,
}

impl LoggingMiddleware {
    pub fn new() -> Self {
        Self { enabled: true }
    }
}

impl Middleware for LoggingMiddleware {
    fn handle(&self, request: &mut Request, _response: &mut Response) -> Result<(), Box<dyn std::error::Error>> {
        if self.enabled {
            println!("[{}] {} {}", 
                chrono::Utc::now().format("%Y-%m-%d %H:%M:%S"),
                request.method, 
                request.path
            );
        }
        Ok(())
    }
}

#[derive(Debug)]
pub struct CorsMiddleware {
    allow_origins: Vec<String>,
    allow_methods: Vec<Method>,
    allow_headers: Vec<String>,
}

impl CorsMiddleware {
    pub fn new() -> Self {
        Self {
            allow_origins: vec!["*".to_string()],
            allow_methods: vec![Method::GET, Method::POST, Method::PUT, Method::DELETE],
            allow_headers: vec![
                "Content-Type".to_string(),
                "Authorization".to_string(),
                "X-Requested-With".to_string(),
            ],
        }
    }
}

impl Middleware for CorsMiddleware {
    fn handle(&self, _request: &mut Request, response: &mut Response) -> Result<(), Box<dyn std::error::Error>> {
        response.headers.insert("Access-Control-Allow-Origin".to_string(), self.allow_origins.join(", "));
        response.headers.insert("Access-Control-Allow-Methods".to_string(), 
            self.allow_methods.iter().map(|m| m.to_string()).collect::<Vec<_>>().join(", "));
        response.headers.insert("Access-Control-Allow-Headers".to_string(), self.allow_headers.join(", "));
        Ok(())
    }
}

#[derive(Debug)]
pub struct RateLimitMiddleware {
    requests_per_minute: usize,
    window: std::time::Duration,
}

impl RateLimitMiddleware {
    pub fn new(requests_per_minute: usize) -> Self {
        Self {
            requests_per_minute,
            window: std::time::Duration::from_secs(60),
        }
    }
}

impl Middleware for RateLimitMiddleware {
    fn handle(&self, _request: &mut Request, response: &mut Response) -> Result<(), Box<dyn std::error::Error>> {
        // Simple rate limiting - in production this would use a proper store
        response.headers.insert("X-RateLimit-Limit".to_string(), self.requests_per_minute.to_string());
        response.headers.insert("X-RateLimit-Remaining".to_string(), self.requests_per_minute.to_string());
        Ok(())
    }
}

#[derive(Debug)]
pub struct TimeoutMiddleware {
    config: TimeoutConfig,
}

impl TimeoutMiddleware {
    pub fn new(_server_config: ServerConfig, _session_config: SessionConfig) -> Self {
        Self {
            config: TimeoutConfig::default(),
        }
    }
}

impl Middleware for TimeoutMiddleware {
    fn handle(&self, _request: &mut Request, response: &mut Response) -> Result<(), Box<dyn std::error::Error>> {
        response.headers.insert("X-Timeout-Config".to_string(), 
            format!("request: {:?}, response: {:?}", 
                self.config.request_timeout, 
                self.config.response_timeout));
        Ok(())
    }
}

/// Session manager
#[derive(Debug)]
pub struct TimeoutSessionManager {
    config: SessionConfig,
    sessions: HashMap<String, Session>,
}

impl TimeoutSessionManager {
    pub fn new(config: SessionConfig) -> Result<Self, Box<dyn std::error::Error>> {
        Ok(Self {
            config,
            sessions: HashMap::new(),
        })
    }
    
    pub fn create_session(&mut self) -> String {
        let id = uuid::Uuid::new_v4().to_string();
        let session = Session::new(id.clone());
        self.sessions.insert(id.clone(), session);
        id
    }
    
    pub fn get_session(&self, id: &str) -> Option<&Session> {
        self.sessions.get(id)
    }
    
    pub fn get_session_mut(&mut self, id: &str) -> Option<&mut Session> {
        self.sessions.get_mut(id)
    }
    
    pub fn remove_session(&mut self, id: &str) -> Option<Session> {
        self.sessions.remove(id)
    }
    
    pub fn cleanup_expired(&mut self) {
        let expired_ids: Vec<String> = self.sessions
            .iter()
            .filter(|(_, session)| session.is_expired())
            .map(|(id, _)| id.clone())
            .collect();
        
        for id in expired_ids {
            self.sessions.remove(&id);
        }
    }
}

/// Router
#[derive(Debug)]
pub struct Router {
    routes: HashMap<(Method, String), Box<dyn Fn(&Request) -> Response + Send + Sync>>,
}

impl Router {
    pub fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }
    
    pub fn add_route<F>(&mut self, method: Method, path: String, handler: F)
    where
        F: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        self.routes.insert((method, path), Box::new(handler));
    }
    
    pub fn handle(&self, request: &Request) -> Option<Response> {
        if let Some(handler) = self.routes.get(&(request.method.clone(), request.path.clone())) {
            Some(handler(request))
        } else {
            None
        }
    }
}

impl Default for Router {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_status_code_properties() {
        assert!(StatusCode::OK.is_success());
        assert!(StatusCode::Found.is_redirection());
        assert!(StatusCode::NotFound.is_client_error());
        assert!(StatusCode::InternalServerError.is_server_error());
    }
    
    #[test]
    fn test_request_creation() {
        let request = Request::new(Method::GET, "/test".to_string())
            .with_header("User-Agent".to_string(), "Test".to_string())
            .with_query_param("q".to_string(), "value".to_string());
        
        assert_eq!(request.method, Method::GET);
        assert_eq!(request.path, "/test");
        assert_eq!(request.headers.get("user-agent"), Some(&"Test".to_string()));
        assert_eq!(request.query_params.get("q"), Some(&"value".to_string()));
    }
    
    #[test]
    fn test_response_creation() {
        let response = Response::new(StatusCode::OK)
            .with_text("Hello, World!".to_string());
        
        assert_eq!(response.status, StatusCode::OK);
        assert_eq!(response.body_as_string().unwrap(), "Hello, World!");
        assert_eq!(response.headers.get("content-type"), Some(&"text/plain".to_string()));
    }
    
    #[test]
    fn test_session_management() {
        let mut session = Session::new("test-id".to_string());
        session.set("user_id".to_string(), "123".to_string());
        
        assert_eq!(session.get("user_id"), Some(&"123".to_string()));
        assert_eq!(session.remove("user_id"), Some("123".to_string()));
        assert_eq!(session.get("user_id"), None);
    }
}
