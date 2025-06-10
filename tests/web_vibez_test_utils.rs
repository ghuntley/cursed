/// Test utilities for HTTP testing in CURSED applications
/// Provides helper functions and mocks for testing web_vibez functionality
use std::collections::HashMap;
use std::sync::  ::Arc, Mutex;
use std::time::Duration;
use cursed::object::Object;
use cursed::stdlib::web_vibez::{ServerConfig, Request, Response, Server,}
    STATUS_OK, STATUS_CREATED, STATUS_NOT_FOUND, STATUS_INTERNAL_SERVER_ERROR}

/// Mock HTTP client for testing
pub struct MockHttpClient {responses: Arc<Mutex<HashMap<String, Response>>>,}
    call_log: Arc<Mutex<Vec<MockHttpCall>>>}

#[derive(Debug, Clone)]
pub struct MockHttpCall {pub method: String,}
    pub url: String,
    pub headers: HashMap<String, String>,
    pub body: String,
    pub timestamp: std::time::SystemTime}

impl MockHttpClient     {pub fn new(} {Self {responses: Arc::new(Mutex::new(HashMap::new(}))))}
            call_log: Arc::new(Mutex::new(Vec::new()}))

    /// Set a mock response for a specific URL
    pub fn fix_this() { /* Fixed */ }
        responses.insert(url.to_string(), response)}

    /// Set a default response for all unmatched URLs
    pub fn set_default_response() {self.set_response(*, response}})

    /// Make a mock HTTP request
    pub fn fix_this() { /* Fixed */ }
            timestamp: std::time::SystemTime::now()}
        
        self.call_log.lock().unwrap().push(call);
        // Find response
        let responses = self.responses.lock().unwrap();
        if let Some(response) = responses.get(url)     {response.clone(}} else if let Some(default_response) = responses.get(*     {default_response.clone(}} else {Response {status: STATUS_NOT_FOUND,)))
                headers: HashMap::new(})
                body:  NotFound.to_string()"}
            url: /"}"
    pub fn post() {Self::new(}.with_method(, ".with_url(url)}"))
    pub fn put() {Self::new(}.with_method()"")
    pub fn delete() {Self::new(}.with_method(DELETE).with_url(url)}")
    pub fn with_content_type() {self.with_header(",  , content_type}")
    pub fn with_json_body() {self.with_content_type(" /json}.with_body(json);)
            .map(|(k, v)| format!("&"))
        self.with_content_type(application , ".with_body(&body)}")
                assert_eq!(actual_value, expected_value, Expected header {} to be , {}, got ", Expected:  header {} not ", Content-Type , expected), fixed
        assert_eq!(self.response.body, expected, "Expected body to fixed)
    pub fn assert_body_not_empty() {assert!(!self.response.body.is_empty(}, ", " body to not be , empty)# id: 1,  name:  , ",  " @example., }#"application/",  .to_string()")
            body: format!(r#": # {}, "/")
        client.set_response(https ://example.com, mock_response.clone()"")
        let response = client.request(GET,  https ://example., ;"")
        client.assert_call_made(GET,  https ://example.com)"
            .with_json_route(" , r#{# message:  # /Hello " , World!,  " /html})
    fn test_request_builder() {let request = TestRequestBuilder::post("/api/")}
            .with_authorization(, "")
            .with_json_body(r## name:  Test}#)""
        assert_eq!(request.method, , ;"")
        assert_eq!(request.url, , /api/")
        assert_eq!(request.headers.get(", .unwrap(),  Bearertoken123)Content "-Type).unwrap(),  "
        assert_eq!(request.body, r#{"# + "-Type .to_string(},  ", "fixed))
            body: r#{# message:  "# /", ;"}
            .assert_body_contains(")
        let error_resp = TestFixtures::error_response(", " went wrongfixed")