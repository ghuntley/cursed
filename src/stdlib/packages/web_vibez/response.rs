use std::collections::HashMap;

/// HTTP Response
#[derive(Debug)]
pub struct HttpResponse {
    pub status: u16,
    pub headers: HashMap<String, String>,
    pub body: Vec<u8>,
}

/// Response builder
pub struct ResponseBuilder {
    response: HttpResponse,
}

impl ResponseBuilder {
    pub fn new() -> Self {
        Self {
            response: HttpResponse {
                status: 200,
                headers: HashMap::new(),
                body: Vec::new(),
            },
        }
    }
    
    pub fn status(mut self, status: u16) -> Self {
        self.response.status = status;
        self
    }
    
    pub fn header(mut self, key: &str, value: &str) -> Self {
        self.response.headers.insert(key.to_string(), value.to_string());
        self
    }
    
    pub fn body(mut self, body: Vec<u8>) -> Self {
        self.response.body = body;
        self
    }
    
    pub fn build(self) -> HttpResponse {
        self.response
    }
}

impl Default for ResponseBuilder {
    fn default() -> Self {
        Self::new()
    }
}
