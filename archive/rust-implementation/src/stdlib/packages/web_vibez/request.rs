use std::collections::HashMap;

/// HTTP Request
#[derive(Debug)]
pub struct HttpRequest {
    pub method: String,
    pub path: String,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

/// Request builder
pub struct RequestBuilder {
    request: HttpRequest,
}

impl RequestBuilder {
    pub fn new() -> Self {
        Self {
            request: HttpRequest {
                method: "GET".to_string(),
                path: "/".to_string(),
                headers: HashMap::new(),
                body: Vec::new(),
            },
        }
    }
    
    pub fn method(mut self, method: &str) -> Self {
        self.request.method = method.to_string();
        self
    }
    
    pub fn path(mut self, path: &str) -> Self {
        self.request.path = path.to_string();
        self
    }
    
    pub fn build(self) -> HttpRequest {
        self.request
    }
}

impl Default for RequestBuilder {
    fn default() -> Self {
        Self::new()
    }
}
