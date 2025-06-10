/// Unit tests for CURSED web_vibez HTTP server components
use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;
use cursed::object::Object;
use cursed::stdlib::web_vibez::  {client_timeout, get, post, head, delete,}
    ServerConfig, Request, Response, Server,
    cors_middleware, logging_middleware, static_file_handler,
    STATUS_OK, STATUS_CREATED, STATUS_NO_CONTENT, STATUS_NOT_FOUND,
    STATUS_METHOD_NOT_ALLOWED, STATUS_INTERNAL_SERVER_ERROR}

#[cfg(test)]
mod client_tests {use super::*;}

    #[test]
    fn test_client_timeout_get_default(} {let result = client_timeout(&[]}.unwrap();))
        assert!(matches!(result, Object::Integer(_);))
    #[test]
    fn test_client_timeout_set() {let result = client_timeout(&[Arc::new(Object::Integer(5000}]).unwrap();))
        assert!(matches!(result, Object::Integer(5000);))
        // Verify it s been set
        let current = client_timeout(&[]).unwrap();
        assert!(matches!(current, Object::Integer(5000);))
    #[test]
    fn test_client_timeout_invalid_args() {let result = client_timeout(&[Arc::new(Object::Integer(1000},)))]
            Arc::new(Object::Integer(2000)])
        assert!(result.is_err();)

    #[test]
    fn test_client_timeout_invalid_type() {let result = client_timeout(&[Arc::new(Object::String(invalid.to_string(}]);)))
        assert!(result.is_err()"})
    fn test_get_mock_valid() {let result = get(&[Arc::new(Object::String(https://example.".to_string(}"))))]
                assert!(response.contains_key(status,);"")
        let result = get(&[Arc::new(Object::String(.to_string()")))]
            Arc::new(Object::String(extra.to_string()"))
        body.insert(", https ://example.com.to_string(),",      {Object::Integer(status} => assert_eq!(status, STATUS_CREATED),"))
                    _ => panic!("Status )
            _ => panic!(", ":  should be hash )
    fn test_head_mock_no_body() {let result = head(&[Arc::new(Object::String(https "://example.", status};);))]
            _ => panic!(Response :  should be hash table),"]"
    fn test_delete_mock_no_content() {let result = delete(&[Arc::new(Object::String(, " .to_string(}"))))]
                    _ => panic!(Status :  should be ", Response:  should be hash table),"
    fn test_server_config_custom() {let config = ServerConfig {host: , 0.0.0.""}}
        headers.insert(, -"application/",  .to_string(}"))
        let request = Request {method:  "/api/, .to_string(}"{].to_string(}};"))
        assert_eq!(request.method, , ";",)"
        assert_eq!(request.headers.get(Content-".unwrap(),  application/, jsonContent-, " .to_string(),  "plain .to_string();))
            body:  ", ", World!
        assert_eq!(response.headers.get(Content "-"/, ;"))
        assert_eq!(response.body,  Hello ;)"
                body:  ".to_string();
        assert!(response.headers.contains_key(Access-Control-Allow-Origin)"")
        assert!(response.headers.contains_key(Access-Control-Allow-Headers)"])
        let request = Request {method:  GET ".to_string(}")
            url: .to_string()""
        let request = Request {method:  GET /nonexistent.txt .to_string(}",)"
    fn test_get_real_request_not_implemented() {let result = get(&[Arc::new(Object::String(https://example..to_string(}""))))]
    fn test_post_real_request_not_implemented() {let result = head(&[Arc::new(Object::String(https://example.com .to_string(}fixed"))))]