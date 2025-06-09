/// Test utilities for HTTP testing in CURSED applications
/// Provides helper functions and mocks for testing web_vibez functionality
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use cursed::object::Object;
use cursed::stdlib::web_vibez::{
    ServerConfig, Request, Response, Server,
    STATUS_OK, STATUS_CREATED, STATUS_NOT_FOUND, STATUS_INTERNAL_SERVER_ERROR
};

/// Mock HTTP client for testing
pub struct MockHttpClient {
    responses: Arc<Mutex<HashMap<String, Response>>>,
    call_log: Arc<Mutex<Vec<MockHttpCall>>>,
}

#[derive(Debug, Clone)]
pub struct MockHttpCall {
    pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub timestamp: std::time::SystemTime,
}

impl MockHttpClient {
    pub fn new() -> Self {
        Self {
            responses: Arc::new(Mutex::new(HashMap::new())),
            call_log: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Set a mock response for a specific URL
    pub fn set_response(&self, url: &str, response: Response) {
        let mut responses = self.responses.lock().unwrap();
        responses.insert(url.to_string(), response);
    }

    /// Set a default response for all unmatched URLs
    pub fn set_default_response(&self, response: Response) {
        self.set_response("*", response);
    }

    /// Make a mock HTTP request
    pub fn request(&self, method: &str, url: &str, headers: HashMap<String, String>, body: String) -> Response {
        // Log the call
        let call = MockHttpCall {
            method: method.to_string(),
            url: url.to_string(),
            headers: headers.clone(),
            body: body.clone(),
            timestamp: std::time::SystemTime::now(),
        };
        
        self.call_log.lock().unwrap().push(call);

        // Find response
        let responses = self.responses.lock().unwrap();
        if let Some(response) = responses.get(url) {
            response.clone()
        } else if let Some(default_response) = responses.get("*") {
            default_response.clone()
        } else {
            Response {
                status: STATUS_NOT_FOUND,
                headers: HashMap::new(),
                body: "Not Found".to_string(),
            }
        }
    }

    /// Get all recorded calls
    pub fn get_calls(&self) -> Vec<MockHttpCall> {
        self.call_log.lock().unwrap().clone()
    }

    /// Get calls filtered by method
    pub fn get_calls_by_method(&self, method: &str) -> Vec<MockHttpCall> {
        self.call_log.lock().unwrap()
            .iter()
            .filter(|call| call.method == method)
            .cloned()
            .collect()
    }

    /// Get calls filtered by URL pattern
    pub fn get_calls_by_url_pattern(&self, pattern: &str) -> Vec<MockHttpCall> {
        self.call_log.lock().unwrap()
            .iter()
            .filter(|call| call.url.contains(pattern))
            .cloned()
            .collect()
    }

    /// Clear call log
    pub fn clear_calls(&self) {
        self.call_log.lock().unwrap().clear();
    }

    /// Assert that a specific call was made
    pub fn assert_call_made(&self, method: &str, url: &str) {
        let calls = self.get_calls();
        let found = calls.iter().any(|call| call.method == method && call.url == url);
        assert!(found, "Expected call {} {} not found in: {:?}", method, url, calls);
    }

    /// Assert number of calls made
    pub fn assert_call_count(&self, expected: usize) {
        let actual = self.get_calls().len();
        assert_eq!(actual, expected, "Expected {} calls, got {}", expected, actual);
    }
}

/// Test server builder for easy server setup in tests
pub struct TestServerBuilder {
    config: ServerConfig,
    routes: Vec<(String, Box<dyn Fn(&Request) -> Response + Send + Sync>)>,
    middleware: Vec<Box<dyn Fn(&Request) -> Option<Response> + Send + Sync>>,
}

impl TestServerBuilder {
    pub fn new() -> Self {
        Self {
            config: ServerConfig {
                host: "127.0.0.1".to_string(),
                port: 0, // Let OS choose port for testing
                max_connections: 10,
                timeout: Duration::from_secs(5),
            },
            routes: Vec::new(),
            middleware: Vec::new(),
        }
    }

    pub fn with_port(mut self, port: u16) -> Self {
        self.config.port = port;
        self
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.config.timeout = timeout;
        self
    }

    pub fn with_route<F>(mut self, path: &str, handler: F) -> Self
    where
        F: Fn(&Request) -> Response + Send + Sync + 'static,
    {
        self.routes.push((path.to_string(), Box::new(handler)));
        self
    }

    pub fn with_json_route(self, path: &str, json_response: &str) -> Self {
        let json = json_response.to_string();
        self.with_route(path, move |_req| {
            let mut headers = HashMap::new();
            headers.insert("Content-Type".to_string(), "application/json".to_string());
            Response {
                status: STATUS_OK,
                headers,
                body: json.clone(),
            }
        })
    }

    pub fn with_static_route(self, path: &str, content: &str, content_type: &str) -> Self {
        let content = content.to_string();
        let content_type = content_type.to_string();
        self.with_route(path, move |_req| {
            let mut headers = HashMap::new();
            headers.insert("Content-Type".to_string(), content_type.clone());
            Response {
                status: STATUS_OK,
                headers,
                body: content.clone(),
            }
        })
    }

    pub fn with_middleware<F>(mut self, middleware: F) -> Self
    where
        F: Fn(&Request) -> Option<Response> + Send + Sync + 'static,
    {
        self.middleware.push(Box::new(middleware));
        self
    }

    pub fn build(self) -> Server {
        let mut server = Server::new(self.config);
        
        for (path, handler) in self.routes {
            server.add_route(&path, handler);
        }
        
        for middleware in self.middleware {
            server.add_middleware(middleware);
        }
        
        server
    }
}

/// HTTP request builder for testing
pub struct TestRequestBuilder {
    method: String,
    url: String,
    headers: HashMap<String, String>,
    body: String,
}

impl TestRequestBuilder {
    pub fn new() -> Self {
        Self {
            method: "GET".to_string(),
            url: "/".to_string(),
            headers: HashMap::new(),
            body: String::new(),
        }
    }

    pub fn get(url: &str) -> Self {
        Self::new().with_method("GET").with_url(url)
    }

    pub fn post(url: &str) -> Self {
        Self::new().with_method("POST").with_url(url)
    }

    pub fn put(url: &str) -> Self {
        Self::new().with_method("PUT").with_url(url)
    }

    pub fn delete(url: &str) -> Self {
        Self::new().with_method("DELETE").with_url(url)
    }

    pub fn with_method(mut self, method: &str) -> Self {
        self.method = method.to_string();
        self
    }

    pub fn with_url(mut self, url: &str) -> Self {
        self.url = url.to_string();
        self
    }

    pub fn with_header(mut self, key: &str, value: &str) -> Self {
        self.headers.insert(key.to_string(), value.to_string());
        self
    }

    pub fn with_content_type(self, content_type: &str) -> Self {
        self.with_header("Content-Type", content_type)
    }

    pub fn with_authorization(self, token: &str) -> Self {
        self.with_header("Authorization", &format!("Bearer {}", token))
    }

    pub fn with_body(mut self, body: &str) -> Self {
        self.body = body.to_string();
        self
    }

    pub fn with_json_body(self, json: &str) -> Self {
        self.with_content_type("application/json").with_body(json)
    }

    pub fn with_form_body(self, form_data: &[(&str, &str)]) -> Self {
        let body = form_data
            .iter()
            .map(|(k, v)| format!("{}={}", k, v))
            .collect::<Vec<_>>()
            .join("&");
        self.with_content_type("application/x-www-form-urlencoded").with_body(&body)
    }

    pub fn build(self) -> Request {
        Request {
            method: self.method,
            url: self.url,
            headers: self.headers,
            body: self.body,
        }
    }
}

/// HTTP response assertions
pub struct ResponseAssertions {
    response: Response,
}

impl ResponseAssertions {
    pub fn new(response: Response) -> Self {
        Self { response }
    }

    pub fn assert_status(self, expected_status: i64) -> Self {
        assert_eq!(self.response.status, expected_status, 
                   "Expected status {}, got {}", expected_status, self.response.status);
        self
    }

    pub fn assert_ok(self) -> Self {
        self.assert_status(STATUS_OK)
    }

    pub fn assert_created(self) -> Self {
        self.assert_status(STATUS_CREATED)
    }

    pub fn assert_not_found(self) -> Self {
        self.assert_status(STATUS_NOT_FOUND)
    }

    pub fn assert_header(self, key: &str, expected_value: &str) -> Self {
        match self.response.headers.get(key) {
            Some(actual_value) => {
                assert_eq!(actual_value, expected_value,
                          "Expected header {} to be '{}', got '{}'", key, expected_value, actual_value);
            }
            None => panic!("Expected header '{}' not found", key),
        }
        self
    }

    pub fn assert_content_type(self, expected: &str) -> Self {
        self.assert_header("Content-Type", expected)
    }

    pub fn assert_body_contains(self, expected: &str) -> Self {
        assert!(self.response.body.contains(expected),
                "Expected body to contain '{}', got: {}", expected, self.response.body);
        self
    }

    pub fn assert_body_equals(self, expected: &str) -> Self {
        assert_eq!(self.response.body, expected,
                   "Expected body to equal '{}', got: {}", expected, self.response.body);
        self
    }

    pub fn assert_json_body(self, expected_json: &str) -> Self {
        // In a real implementation, you'd parse and compare JSON
        // For now, just compare strings
        self.assert_body_equals(expected_json)
    }

    pub fn assert_body_length(self, expected_length: usize) -> Self {
        assert_eq!(self.response.body.len(), expected_length,
                   "Expected body length {}, got {}", expected_length, self.response.body.len());
        self
    }

    pub fn assert_body_not_empty(self) -> Self {
        assert!(!self.response.body.is_empty(), "Expected body to not be empty");
        self
    }

    pub fn get_response(self) -> Response {
        self.response
    }
}

/// Common test fixtures and data
pub struct TestFixtures;

impl TestFixtures {
    /// Sample user data for testing
    pub fn sample_user_json() -> &'static str {
        r#"{"id": 1, "name": "Test User", "email": "test@example.com"}"#
    }

    /// Sample error response
    pub fn error_response(message: &str) -> Response {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        Response {
            status: STATUS_INTERNAL_SERVER_ERROR,
            headers,
            body: format!(r#"{{"error": "{}"}}"#, message),
        }
    }

    /// Sample success response
    pub fn success_response(data: &str) -> Response {
        let mut headers = HashMap::new();
        headers.insert("Content-Type".to_string(), "application/json".to_string());
        Response {
            status: STATUS_OK,
            headers,
            body: data.to_string(),
        }
    }

    /// Authentication token for testing
    pub fn test_auth_token() -> &'static str {
        "test_token_12345"
    }

    /// Sample HTML response
    pub fn sample_html() -> &'static str {
        r#"<html><body><h1>Test Page</h1></body></html>"#
    }

    /// Sample form data
    pub fn sample_form_data() -> Vec<(&'static str, &'static str)> {
        vec![
            ("name", "Test User"),
            ("email", "test@example.com"),
            ("message", "Hello, World!"),
        ]
    }
}

/// Test helper macros
#[macro_export]
macro_rules! assert_response_ok {
    ($response:expr) => {
        crate::web_vibez_test_utils::ResponseAssertions::new($response).assert_ok()
    };
}

#[macro_export]
macro_rules! assert_response_status {
    ($response:expr, $status:expr) => {
        crate::web_vibez_test_utils::ResponseAssertions::new($response).assert_status($status)
    };
}

#[macro_export]
macro_rules! assert_response_json {
    ($response:expr, $json:expr) => {
        crate::web_vibez_test_utils::ResponseAssertions::new($response)
            .assert_content_type("application/json")
            .assert_json_body($json)
    };
}

#[cfg(test)]
mod test_utils_tests {
    use super::*;

    #[test]
    fn test_mock_http_client() {
        let client = MockHttpClient::new();
        
        // Set mock response
        let mock_response = Response {
            status: STATUS_OK,
            headers: HashMap::new(),
            body: "Mock response".to_string(),
        };
        client.set_response("https://example.com", mock_response.clone());
        
        // Make request
        let response = client.request("GET", "https://example.com", HashMap::new(), String::new());
        assert_eq!(response.status, STATUS_OK);
        assert_eq!(response.body, "Mock response");
        
        // Check call log
        client.assert_call_made("GET", "https://example.com");
        client.assert_call_count(1);
    }

    #[test]
    fn test_server_builder() {
        let server = TestServerBuilder::new()
            .with_port(8080)
            .with_json_route("/api/test", r#"{"message": "test"}"#)
            .with_static_route("/", "Hello, World!", "text/html")
            .build();
        
        assert_eq!(server.config.port, 8080);
        assert_eq!(server.routes.len(), 2);
    }

    #[test]
    fn test_request_builder() {
        let request = TestRequestBuilder::post("/api/users")
            .with_authorization("token123")
            .with_json_body(r#"{"name": "Test"}"#)
            .build();
        
        assert_eq!(request.method, "POST");
        assert_eq!(request.url, "/api/users");
        assert_eq!(request.headers.get("Authorization").unwrap(), "Bearer token123");
        assert_eq!(request.headers.get("Content-Type").unwrap(), "application/json");
        assert_eq!(request.body, r#"{"name": "Test"}"#);
    }

    #[test]
    fn test_response_assertions() {
        let response = Response {
            status: STATUS_OK,
            headers: {
                let mut h = HashMap::new();
                h.insert("Content-Type".to_string(), "application/json".to_string());
                h
            },
            body: r#"{"message": "success"}"#.to_string(),
        };
        
        ResponseAssertions::new(response)
            .assert_ok()
            .assert_content_type("application/json")
            .assert_body_contains("success");
    }

    #[test]
    fn test_fixtures() {
        let user_json = TestFixtures::sample_user_json();
        assert!(user_json.contains("Test User"));
        
        let error_resp = TestFixtures::error_response("Something went wrong");
        assert_eq!(error_resp.status, STATUS_INTERNAL_SERVER_ERROR);
        
        let token = TestFixtures::test_auth_token();
        assert!(!token.is_empty());
    }
}
