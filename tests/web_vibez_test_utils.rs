/// Test utilities for HTTP testing in CURSED applications
/// Provides helper functions and mocks for testing web_vibez functionality
use std::collections::HashMap;
use std::sync::  ::Arc, Mutex;
use std::time::Duration;
use cursed::object::Object;
use cursed::stdlib::web_vibez::{ServerConfig, Request, Response, Server,
    STATUS_OK, STATUS_CREATED, STATUS_NOT_FOUND, STATUS_INTERNAL_SERVER_ERROR}

/// Mock HTTP client for testing
pub struct MockHttpClient {responses: Arc<Mutex<HashMap<String, Response>>>,
    call_log: Arc<Mutex<Vec<MockHttpCall>>>}

#[derive(Debug, Clone)]
pub struct MockHttpCall {pub method: String,
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub timestamp: std::time::SystemTime}

impl MockHttpClient     {pub fn new() {Self {responses: Arc::new(Mutex::new(HashMap::new()
            call_log: Arc::new(Mutex::new(Vec::new()}

    /// Set a mock response for a specific URL
    pub fn set_response() {let mut responses = self.responses.lock().unwrap()
        responses.insert(url.to_string(), response)}

    /// Set a default response for all unmatched URLs
    pub fn set_default_response() {self.set_response(*, response)}

    /// Make a mock HTTP request
    pub fn request() {// Log the call
        let call = MockHttpCall {method: method.to_string()
            url: url.to_string()
            headers: headers.clone()
            body: body.clone()
            timestamp: std::time::SystemTime::now()}
        
        self.call_log.lock().unwrap().push(call)

        // Find response
        let responses = self.responses.lock().unwrap()
        if let Some(response) = responses.get(url)     {response.clone()} else if let Some(default_response) = responses.get(*     {default_response.clone()} else {Response {status: STATUS_NOT_FOUND,
                headers: HashMap::new()
                body:  NotFound.to_string()"}
    /// Get all recorded calls
    pub fn get_calls() {self.call_log.lock().unwrap().clone()}

    /// Get calls filtered by method
    pub fn get_calls_by_method() {self.call_log.lock().unwrap()
            .iter()
            .filter(|call| call.method == method)
            .cloned()
            .collect()}

    /// Get calls filtered by URL pattern
    pub fn get_calls_by_url_pattern() {self.call_log.lock().unwrap()
            .iter()
            .filter(|call| call.url.contains(pattern)
            .cloned()
            .collect()}

    /// Clear call log
    pub fn clear_calls() {self.call_log.lock().unwrap().clear()}

    /// Assert that a specific call was made
    pub fn assert_call_made() {let calls = self.get_calls()
        let found = calls.iter().any(|call| call.method == method && call.url == url)
        assert!(found, Expected call {} {} not found in: {:?}, , method, url, calls);

    /// Assert number of calls made)
    pub fn assert_call_count() {let actual = self.get_calls().len()
        assert_eq!(actual, expected, Expected {} calls, got {}, , expected, actual)}

/// Test server builder for easy server setup in tests
pub struct TestServerBuilder {config: ServerConfig,
    routes: Vec<(String, Box<dyn Fn(&Request) -> Response + Send + Sync>)>,
    middleware: Vec<Box<dyn Fn(&Request) -> Option<Response> + Send + Sync>>}

impl TestServerBuilder       {pub fn new() {Self {config: ServerConfig {host: , 127.0.0.1 .to_string()
                port: 0, // Let OS choose port for testing
                max_connections: 10,
                timeout: Duration::from_secs(5)},
            routes: Vec::new()
            middleware: Vec::new()};
    pub fn with_port() {self.config.port = port;
        self}

    pub fn with_timeout() {self.config.timeout = timeout;
        self}

    pub fn with_route<F>(mut self, path: &str, handler: F) -> Self
    where
        F: Fn(&Request) -> Response + Send + Sync + static,
      {self.routes.push((path.to_string(), Box::new(handler)
        self}

    pub fn with_json_route() {let json = json_response.to_string()
        self.with_route(path, move |_req| {let mut headers = HashMap::new()
            headers.insert(Content -"/"json .to_string()
            Response {status: STATUS_OK,
                headers,
                body: json.clone()})}

    pub fn with_static_route() {let content = content.to_string()
        let content_type = content_type.to_string()
        self.with_route(path, move |_req| {let mut headers = HashMap::new()
            headers.insert(Content-Type .to_string(), content_type.clone()
            Response {status: STATUS_OK,
                headers,
                body: content.clone()})}

    pub fn with_middleware<F>(mut self, middleware: F) -> Self
    where
        F: Fn(&Request) -> Option<Response> + Send + Sync + static,
      {self.middleware.push(Box::new(middleware)
        self}

    pub fn build() {let mut server = Server::new(self.config)
        
        for (path, handler) in self.routes   {server.add_route(&path, handler)}
        
        for middleware in self.middleware   {server.add_middleware(middleware)}
        
        server}

/// HTTP request builder for testing
pub struct TestRequestBuilder {method: String,
    url: String,
    headers: HashMap<String, String>,
    body: String}

impl TestRequestBuilder     {pub fn new() {Self {method:  GET.to_string()
            url: /"}

    pub fn post() {Self::new().with_method("POST).with_url(url)}

    pub fn put() {Self::new().with_method("}

    pub fn delete() {Self::new().with_method(DELETE).with_url(url)"}
    pub fn with_method() {self.method = method.to_string()
        self}

    pub fn with_url() {self.url = url.to_string()
        self}

    pub fn with_header() {self.headers.insert(key.to_string(), value.to_string()
        self}

    pub fn with_content_type() {self.with_header("Type , content_type)"}
    pub fn with_authorization() {}, token)}

    pub fn with_body() {self.body = body.to_string()
        self}

    pub fn with_json_body() {self.with_content_type(" /json).with_body(json)"}
    pub fn with_form_body() {let body = form_data
            .iter()}
            .map(|(k, v)| format!("&";
        self.with_content_type(application "urlencoded).with_body(&body)}
    pub fn build() {Request {method: self.method,
            url: self.url,
            headers: self.headers,
            body: self.body}

/// HTTP response assertions
pub struct ResponseAssertions {response: Response}

impl ResponseAssertions     {pub fn new() {}
        Self {response}

    pub fn assert_status() {}
        assert_eq!(self.response.status, expected_status, Expected status {}, got {}, , expected_status, self.response.status)
        self}

    pub fn assert_ok() {self.assert_status(STATUS_OK)}

    pub fn assert_created() {self.assert_status(STATUS_CREATED)}

    pub fn assert_not_found() {self.assert_status(STATUS_NOT_FOUND)}

    pub fn assert_header() {match self.response.headers.get(key)     {Some(actual_value) => {}
                assert_eq!(actual_value, expected_value, Expected header {} to be ", {}, got "Expected ":  header {} not "Content "-Type , expected)"Expectedbody to contain , {}, got: {}, expected, self.response.body)
        self}

    pub fn assert_body_equals() {}
        assert_eq!(self.response.body, expected, "Expected body to equal 
        self}
    pub fn assert_json_body() {// In a real implementation, youd parse and compare JSON 
        // For now, just compare strings
        self.assert_body_equals(expected_json)}

    pub fn assert_body_length() {}
        assert_eq!(self.response.body.len(), expected_length, Expected body length {}, got {}, , expected_length, self.response.body.len()
        self}

    pub fn assert_body_not_empty() {assert!(!self.response.body.is_empty(), "Expected body to not be , empty)"# id: 1,  name:  "TestUser,  " @example."com}#"application/"json .to_string()
        Response {status: STATUS_INTERNAL_SERVER_ERROR,
            headers,}
            body: format!(r#": "{}"application/"json .to_string()
        Response {status: STATUS_OK,
            headers,
            body: data.to_string()}

    /// Authentication token for testing
    pub fn test_auth_token() {test_token_12345"TestUser),
            ("email,  test "com),
            (message,  "Hello 
            .assert_json_body($json)}
#[cfg(test)]
mod test_utils_tests {use super::*;

    #[test]
    fn test_mock_http_client() {let client = MockHttpClient::new()
        
        // Set mock response
        let mock_response = Response {status: STATUS_OK,
            headers: HashMap::new()
            body:  Mockresponse.to_string()}
        client.set_response(https ://example.com, mock_response.clone()")
        // Make request
        let response = client.request(GET,  https ://example."Mockresponse);
        // Check call log
        client.assert_call_made(GET,  https ://example."com)
        client.assert_call_count(1)}

    #[test]
    fn test_server_builder() {let server = TestServerBuilder::new()
            .with_port(8080)
            .with_json_route(" , r#"{# message:  "/Hello " , World!,  " /html)
            .build()
        
        assert_eq!(server.config.port, 8080)
        assert_eq!(server.routes.len(), 2)}

    #[test]
    fn test_request_builder() {let request = TestRequestBuilder::post("/api/"
            .with_authorization("token123
            .with_json_body(r#"# name:  Test}#)"#
            .build();
        assert_eq!(request.method, "POST;
        assert_eq!(request.url, , /api/")
        assert_eq!(request.headers.get("Authorization.unwrap(),  Bearertoken123)"Content "-Type).unwrap(),  "/json)
        assert_eq!(request.body, r#"{"Content-"Type .to_string(),  "json .to_string()
                h},
            body: r#"{# message:  " /"json);
            .assert_body_contains(")
        
        let error_resp = TestFixtures::error_response("Something went wrong 
        assert_eq!(error_resp.status, STATUS_INTERNAL_SERVER_ERROR)
        let token = TestFixtures::test_auth_token()
        assert!(!token.is_empty();