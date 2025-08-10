use std::fmt;

/// HTTP method enumeration
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum HttpMethod {
    Get,
    Post,
    Put,
    Delete,
    Head,
    Options,
    Patch,
    Trace,
    Connect,
}

impl fmt::Display for HttpMethod {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            HttpMethod::Get => write!(f, "GET"),
            HttpMethod::Post => write!(f, "POST"),
            HttpMethod::Put => write!(f, "PUT"),
            HttpMethod::Delete => write!(f, "DELETE"),
            HttpMethod::Head => write!(f, "HEAD"),
            HttpMethod::Options => write!(f, "OPTIONS"),
            HttpMethod::Patch => write!(f, "PATCH"),
            HttpMethod::Trace => write!(f, "TRACE"),
            HttpMethod::Connect => write!(f, "CONNECT"),
        }
    }
}

/// Set of HTTP methods
pub struct MethodSet {
    methods: Vec<HttpMethod>,
}

impl MethodSet {
    pub fn new() -> Self {
        Self {
            methods: Vec::new(),
        }
    }
    
    pub fn add(mut self, method: HttpMethod) -> Self {
        self.methods.push(method);
        self
    }
}

impl Default for MethodSet {
    fn default() -> Self {
        Self::new()
    }
}

/// Invalid method error
#[derive(Debug)]
pub struct InvalidMethodError {
    pub method: String,
}

impl fmt::Display for InvalidMethodError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid HTTP method: {}", self.method)
    }
}

impl std::error::Error for InvalidMethodError {}
