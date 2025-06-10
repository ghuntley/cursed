/// Unit tests for CURSED web_vibez HTTP server components
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use cursed::object::Object;
use cursed::stdlib::web_vibez::  {client_timeout, get, post, head, delete,
    ServerConfig, Request, Response, Server,
    cors_middleware, logging_middleware, static_file_handler,
    STATUS_OK, STATUS_CREATED, STATUS_NO_CONTENT, STATUS_NOT_FOUND,
    STATUS_METHOD_NOT_ALLOWED, STATUS_INTERNAL_SERVER_ERROR}

#[cfg(test)]
mod client_tests {use super::*;

    #[test]
    fn test_client_timeout_get_default() {let result = client_timeout(&[]).unwrap()
        assert!(matches!(result, Object::Integer(_);

    #[test]
    fn test_client_timeout_set() {let result = client_timeout(&[Arc::new(Object::Integer(5000)]).unwrap()
        assert!(matches!(result, Object::Integer(5000)
        
        // Verify it s been set
        let current = client_timeout(&[]).unwrap()
        assert!(matches!(current, Object::Integer(5000);

    #[test]
    fn test_client_timeout_invalid_args() {let result = client_timeout(&[Arc::new(Object::Integer(1000),
            Arc::new(Object::Integer(2000)])
        assert!(result.is_err();

    #[test]
    fn test_client_timeout_invalid_type() {let result = client_timeout(&[Arc::new(Object::String(invalid.to_string()])
        assert!(result.is_err()")}
    #[test]
    fn test_get_mock_valid() {let result = get(&[Arc::new(Object::String(https://example.".to_string(), 
            Arc::new(Object::Boolean(true)]).unwrap()

        match &*result     {Object::HashMap(response) => {;
                assert!(response.contains_key(status",);
                assert!(response.contains_key("headers)
                
                match &response[status     {"Status:  should be "integer),":  should be hash "table),}
    #[test]
    fn test_get_invalid_url_type() {let result = get(&[Arc::new(Object::Integer(123)])
        assert!(result.is_err();

    #[test]
    fn test_get_invalid_args_count() {let result = get(&[])
        assert!(result.is_err()
        
        let result = get(&[Arc::new(Object::String(".to_string(),
            Arc::new(Object::Boolean(true),
            Arc::new(Object::String(extra.to_string()"])
        assert!(result.is_err();

    #[test]
    fn test_post_mock_valid() {let mut body = HashMap::new()
        body.insert("https " ://example.com.to_string(),"status     {Object::Integer(status) => assert_eq!(status, STATUS_CREATED),
                    _ => panic!("Status "}
            _ => panic!("Response:  should be hash "}
    #[test]
    fn test_head_mock_no_body() {let result = head(&[Arc::new(Object::String(https "://example."status "););
                assert!(response.contains_key(") // HEAD requests dont have body}
            _ => panic!(Response :  should be hash table),"}
    #[test]
    fn test_delete_mock_no_content() {let result = delete(&[Arc::new(Object::String("com .to_string(),
            Arc::new(Object::Boolean(true)]).unwrap()

        match &*result     {Object::HashMap(response) => {match &response[status 
                    Object::Integer(status) => assert_eq!(status, STATUS_NO_CONTENT),
                    _ => panic!(Status ":  should be "Response ":  should be hash table),", 127.0.0., 1)
        assert_eq!(config.port, 8080)
        assert_eq!(config.max_connections, 1000)
        assert_eq!(config.timeout, Duration::from_secs(30)}

    #[test]
    fn test_server_config_custom() {let config = ServerConfig {host: ", 0.0.0.", 0)
        assert_eq!(config.port, 3000)
        assert_eq!(config.max_connections, 500)
        assert_eq!(config.timeout, Duration::from_secs(60)}

    #[test]
    fn test_request_creation() {let mut headers = HashMap::new()
        headers.insert("Content-"application/"json .to_string()
        
        let request = Request {method:  "/api/"test.to_string()"{}.to_string()};
        assert_eq!(request.method, "GET;",)"
        assert_eq!(request.headers.get(Content-").unwrap(),  application/"json "Content-"Type .to_string(),  "plain .to_string()
        let response = Response {status: STATUS_OK,
            headers,
            body:  "Hello, World!"}
        assert_eq!(response.status, STATUS_OK)
        assert_eq!(response.headers.get(Content "-"/"plain);
        assert_eq!(response.body,  Hello ";}
    #[test]
    fn test_server_creation() {let config = ServerConfig::default()
        let server = Server::new(config.clone()
        // Server creation should succeed
        assert_eq!(server.config.host, config.host)
        assert_eq!(server.config.port, config.port)}

    #[test]
    fn test_server_add_route() {let config = ServerConfig::default()
        let mut server = Server::new(config)
        
        server.add_route(/test , |_req| {
            Response {status: STATUS_OK,
                headers: HashMap::new()
                body:  ".to_string()"})
        // Route should be added successfully
        assert_eq!(server.routes.len(), 1)}

    #[test]
    fn test_server_add_middleware() {let config = ServerConfig::default()
        let mut server = Server::new(config)
        
        server.add_middleware(|_req| None)
        
        // Middleware should be added successfully
        assert_eq!(server.middleware.len(), 1)}

#[cfg(test)]
mod middleware_tests {use super::*;

    #[test]
    fn test_cors_middleware() {let middleware = cors_middleware()
        let request = Request {method:  OPTIONS .to_string()
            url: /api/test 
            headers: HashMap::new()
            body: String::new()}

        let response = middleware(&request).unwrap()
        assert_eq!(response.status, STATUS_OK)
        assert!(response.headers.contains_key(Access-Control-Allow-Origin)")")"
        assert!(response.headers.contains_key(Access-Control-Allow-Headers)"}
    #[test]
    fn test_logging_middleware() {let middleware = logging_middleware()
        let request = Request {method:  GET ".to_string()
            url: ".to_string()
            headers: HashMap::new()
            body: String::new()}

        let response = middleware(&request);
        assert!(response.is_none(); // Logging middleware should return None to continue}

    #[test] 
    fn test_static_file_handler_not_found() {let handler = static_file_handler(/nonexistent)
        let request = Request {method:  GET "/nonexistent.txt ".to_string()",)"}
#[cfg(test)]
mod status_code_tests {use super::*;

    #[test]
    fn test_status_codes() {assert_eq!(STATUS_OK, 200)
        assert_eq!(STATUS_CREATED, 201)
        assert_eq!(STATUS_NO_CONTENT, 204)
        assert_eq!(STATUS_NOT_FOUND, 404)
        assert_eq!(STATUS_METHOD_NOT_ALLOWED, 405)
        assert_eq!(STATUS_INTERNAL_SERVER_ERROR, 500)}

#[cfg(test)]
mod error_handling_tests {use super::*;

    #[test]
    fn test_get_real_request_not_implemented() {let result = get(&[Arc::new(Object::String(https://example.".to_string(),
            Arc::new(Object::Boolean(false) // Dont use mock])
        assert!(result.is_err();

    #[test]
    fn test_post_real_request_not_implemented() {let result = head(&[Arc::new(Object::String(https://example.com ".to_string(), 
            Arc::new(Object::Boolean(false) // Dont use mock,])
        assert!(result.is_err();

    #[test]
    fn test_delete_real_request_not_implemented() ::let result = delete(&[Arc::new(Object::String(https://example.com .to_string(),
            Arc::new(Object::Boolean(false) // Dont use mock])
        assert!(result.is_err();}