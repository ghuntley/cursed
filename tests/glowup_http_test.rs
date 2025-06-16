//! Tests for GlowUpHTTP implementation

use cursed::stdlib::glowup_http::*;
use cursed::stdlib::glowup_http::error::GlowUpError;
use cursed::stdlib::glowup_http::handler::{Handler, HandlerFunc, handler_func, StaticHandler};
use cursed::stdlib::glowup_http::request::{VibeRequest, Method, HttpVersion};
use cursed::stdlib::glowup_http::response::{ResponderVibe, StatusCode};
use cursed::stdlib::glowup_http::router::VibeRouter;
use cursed::stdlib::glowup_http::middleware::*;
use cursed::stdlib::glowup_http::server::VibeServer;
use cursed::stdlib::glowup_http::client::VibeClient;
use cursed::stdlib::glowup_http::websocket::{WebSocketUpgrader, MessageType};
use std::time::Duration;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vibe_request_creation() {
        let request = VibeRequest::new(Method::GET, "/test");
        assert_eq!(request.method, Method::GET);
        assert_eq!(request.url, "/test");
        assert_eq!(request.proto, HttpVersion::Http1_1);
        assert!(request.header.is_empty());
        assert!(request.body.is_empty());
    }

    #[test]
    fn test_vibe_request_cookies() {
        let mut request = VibeRequest::new(Method::GET, "/test");
        
        // Add cookies via header
        request.header.insert("cookie".to_string(), "session=abc123; theme=dark".to_string());
        
        let cookies = request.cookies();
        assert_eq!(cookies.len(), 2);
        assert_eq!(cookies[0].name, "session");
        assert_eq!(cookies[0].value, "abc123");
        assert_eq!(cookies[1].name, "theme");
        assert_eq!(cookies[1].value, "dark");
    }

    #[test]
    fn test_vibe_request_form_parsing() {
        let mut request = VibeRequest::new(Method::POST, "/submit?name=John&age=25");
        request.header.insert("content-type".to_string(), "application/x-www-form-urlencoded".to_string());
        request.body = b"email=john@example.com&city=NYC".to_vec();
        
        request.parse_form().unwrap();
        
        // Query parameters
        assert_eq!(request.form_value("name"), "John");
        assert_eq!(request.form_value("age"), "25");
        
        // POST form data
        assert_eq!(request.post_form_value("email"), "john@example.com");
        assert_eq!(request.post_form_value("city"), "NYC");
    }

    #[test]
    fn test_responder_vibe_basic() {
        let response = ResponderVibe::new();
        
        response.write_header(StatusCode::OK);
        response.write(b"Hello, World!").unwrap();
        
        assert_eq!(response.get_status(), Some(StatusCode::OK));
        assert_eq!(response.get_body(), b"Hello, World!");
        assert!(response.is_written());
    }

    #[test]
    fn test_responder_vibe_json() {
        let response = ResponderVibe::new();
        
        #[derive(serde::Serialize)]
        struct TestData {
            message: String,
            code: u32,
        }
        
        let data = TestData {
            message: "success".to_string(),
            code: 200,
        };
        
        response.write_json(&data).unwrap();
        
        let headers = response.get_headers();
        assert_eq!(headers.get("content-type"), Some(&"application/json".to_string()));
        
        let body_str = String::from_utf8(response.get_body()).unwrap();
        assert!(body_str.contains("success"));
        assert!(body_str.contains("200"));
    }

    #[test]
    fn test_responder_vibe_fluent_interface() {
        let response = ResponderVibe::new();
        
        let result = response
            .status(StatusCode::CREATED)
            .text("Resource created");
        
        assert!(result.is_ok());
        let response = result.unwrap();
        
        assert_eq!(response.get_status(), Some(StatusCode::CREATED));
        assert_eq!(response.get_body(), b"Resource created");
        
        let headers = response.get_headers();
        assert_eq!(headers.get("content-type"), Some(&"text/plain".to_string()));
    }

    #[test]
    fn test_static_handler() {
        let handler = StaticHandler::text("Hello from static handler");
        let request = VibeRequest::new(Method::GET, "/static");
        let response = ResponderVibe::new();
        
        handler.handle_vibe(&response, &request).unwrap();
        
        assert_eq!(response.get_body(), b"Hello from static handler");
        let headers = response.get_headers();
        assert_eq!(headers.get("content-type"), Some(&"text/plain".to_string()));
    }

    #[test]
    fn test_handler_func() {
        let handler = handler_func(|w: &ResponderVibe, r: &VibeRequest| {
            w.write(format!("Hello from {}", r.url).as_bytes())?;
            Ok(())
        });
        
        let request = VibeRequest::new(Method::GET, "/dynamic");
        let response = ResponderVibe::new();
        
        handler.handle_vibe(&response, &request).unwrap();
        
        assert_eq!(response.get_body(), b"Hello from /dynamic");
    }

    #[test]
    fn test_vibe_router_basic() {
        let mut router = VibeRouter::new();
        
        let handler = handler_func(|w: &ResponderVibe, _r: &VibeRequest| {
            w.write(b"Route matched")?;
            Ok(())
        });
        
        router.get("/test", handler);
        
        let request = VibeRequest::new(Method::GET, "/test");
        let response = ResponderVibe::new();
        
        router.handle_vibe(&response, &request).unwrap();
        
        assert_eq!(response.get_body(), b"Route matched");
    }

    #[test]
    fn test_vibe_router_not_found() {
        let router = VibeRouter::new();
        
        let request = VibeRequest::new(Method::GET, "/nonexistent");
        let response = ResponderVibe::new();
        
        router.handle_vibe(&response, &request).unwrap();
        
        assert_eq!(response.get_status(), Some(StatusCode::NOT_FOUND));
        assert_eq!(response.get_body(), b"Not Found");
    }

    #[test]
    fn test_vibe_router_different_methods() {
        let mut router = VibeRouter::new();
        
        router.get("/resource", handler_func(|w, _r| {
            w.write(b"GET response")?;
            Ok(())
        }));
        
        router.post("/resource", handler_func(|w, _r| {
            w.write(b"POST response")?;
            Ok(())
        }));
        
        // Test GET
        let get_request = VibeRequest::new(Method::GET, "/resource");
        let get_response = ResponderVibe::new();
        router.handle_vibe(&get_response, &get_request).unwrap();
        assert_eq!(get_response.get_body(), b"GET response");
        
        // Test POST
        let post_request = VibeRequest::new(Method::POST, "/resource");
        let post_response = ResponderVibe::new();
        router.handle_vibe(&post_response, &post_request).unwrap();
        assert_eq!(post_response.get_body(), b"POST response");
    }

    #[test]
    fn test_logging_middleware() {
        let handler = handler_func(|w: &ResponderVibe, _r: &VibeRequest| {
            w.write(b"Handler response")?;
            Ok(())
        });
        
        let middleware_handler = logging_middleware(handler);
        
        let request = VibeRequest::new(Method::GET, "/logged");
        let response = ResponderVibe::new();
        
        middleware_handler(&response, &request).unwrap();
        
        assert_eq!(response.get_body(), b"Handler response");
    }

    #[test]
    fn test_cors_middleware() {
        let handler = handler_func(|w: &ResponderVibe, _r: &VibeRequest| {
            w.write(b"CORS enabled")?;
            Ok(())
        });
        
        let middleware_handler = cors_middleware(handler);
        
        let request = VibeRequest::new(Method::GET, "/api/data");
        let response = ResponderVibe::new();
        
        middleware_handler(&response, &request).unwrap();
        
        let headers = response.get_headers();
        assert!(headers.contains_key("access-control-allow-origin"));
        assert_eq!(headers.get("access-control-allow-origin"), Some(&"*".to_string()));
        assert_eq!(response.get_body(), b"CORS enabled");
    }

    #[test]
    fn test_cors_preflight() {
        let handler = handler_func(|w: &ResponderVibe, _r: &VibeRequest| {
            w.write(b"Should not reach here")?;
            Ok(())
        });
        
        let middleware_handler = cors_middleware(handler);
        
        let request = VibeRequest::new(Method::OPTIONS, "/api/data");
        let response = ResponderVibe::new();
        
        middleware_handler(&response, &request).unwrap();
        
        assert_eq!(response.get_status(), Some(StatusCode::NO_CONTENT));
        assert_eq!(response.get_body(), b"");
    }

    #[test]
    fn test_jwt_auth_middleware() {
        let handler = handler_func(|w: &ResponderVibe, _r: &VibeRequest| {
            w.write(b"Authenticated")?;
            Ok(())
        });
        
        let middleware_func = jwt_auth_middleware("secret".to_string());
        let middleware_handler = middleware_func(handler);
        
        // Test without token
        let request_no_token = VibeRequest::new(Method::GET, "/protected");
        let response_no_token = ResponderVibe::new();
        
        middleware_handler(&response_no_token, &request_no_token).unwrap();
        
        assert_eq!(response_no_token.get_status(), Some(StatusCode::UNAUTHORIZED));
        
        // Test with token
        let mut request_with_token = VibeRequest::new(Method::GET, "/protected");
        request_with_token.header.insert("authorization".to_string(), "Bearer valid-token".to_string());
        let response_with_token = ResponderVibe::new();
        
        middleware_handler(&response_with_token, &request_with_token).unwrap();
        
        assert_eq!(response_with_token.get_body(), b"Authenticated");
    }

    #[test]
    fn test_vibe_server_creation() {
        let server = VibeServer::new()
            .addr("127.0.0.1:0")
            .read_timeout(Duration::from_secs(10))
            .write_timeout(Duration::from_secs(10));
        
        assert!(!server.is_running());
        
        let stats = server.stats();
        assert_eq!(stats.active_connections, 0);
        assert_eq!(stats.total_requests, 0);
    }

    #[test]
    fn test_vibe_client_creation() {
        let client = VibeClient::new()
            .timeout(Duration::from_secs(30))
            .user_agent("Test Client");
        
        assert_eq!(client.timeout, Duration::from_secs(30));
        assert_eq!(client.user_agent, "Test Client");
    }

    #[test]
    fn test_websocket_upgrader() {
        let upgrader = WebSocketUpgrader::new();
        
        // Test upgrade with missing headers
        let request_bad = VibeRequest::new(Method::GET, "/ws");
        let response_bad = ResponderVibe::new();
        
        let result = upgrader.upgrade(&response_bad, &request_bad);
        assert!(result.is_err());
        
        // Test upgrade with valid headers
        let mut request_good = VibeRequest::new(Method::GET, "/ws");
        request_good.header.insert("connection".to_string(), "Upgrade".to_string());
        request_good.header.insert("upgrade".to_string(), "websocket".to_string());
        request_good.header.insert("sec-websocket-key".to_string(), "dGhlIHNhbXBsZSBub25jZQ==".to_string());
        request_good.header.insert("sec-websocket-version".to_string(), "13".to_string());
        
        let response_good = ResponderVibe::new();
        
        let result = upgrader.upgrade(&response_good, &request_good);
        assert!(result.is_ok());
        
        assert_eq!(response_good.get_status(), Some(StatusCode::SWITCHING_PROTOCOLS));
        let headers = response_good.get_headers();
        assert!(headers.contains_key("sec-websocket-accept"));
    }

    #[test]
    fn test_websocket_connection() {
        let upgrader = WebSocketUpgrader::new();
        let mut request = VibeRequest::new(Method::GET, "/ws");
        
        // Add required headers
        request.header.insert("connection".to_string(), "Upgrade".to_string());
        request.header.insert("upgrade".to_string(), "websocket".to_string());
        request.header.insert("sec-websocket-key".to_string(), "test-key".to_string());
        request.header.insert("sec-websocket-version".to_string(), "13".to_string());
        
        let response = ResponderVibe::new();
        let conn = upgrader.upgrade(&response, &request).unwrap();
        
        assert!(conn.is_connected());
        
        // Test write message
        let result = conn.write_message(MessageType::Text, b"Hello WebSocket");
        assert!(result.is_ok());
        
        // Test close
        conn.close().unwrap();
        assert!(!conn.is_connected());
        
        // Test write after close
        let result = conn.write_message(MessageType::Text, b"Should fail");
        assert!(result.is_err());
    }

    #[test]
    fn test_status_codes() {
        assert_eq!(StatusCode::OK.as_u16(), 200);
        assert_eq!(StatusCode::NOT_FOUND.as_u16(), 404);
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR.as_u16(), 500);
        
        assert_eq!(StatusCode::OK.canonical_reason(), "OK");
        assert_eq!(StatusCode::NOT_FOUND.canonical_reason(), "Not Found");
        assert_eq!(StatusCode::INTERNAL_SERVER_ERROR.canonical_reason(), "Internal Server Error");
    }

    #[test]
    fn test_http_methods() {
        use std::str::FromStr;
        
        assert_eq!(Method::from_str("GET").unwrap(), Method::GET);
        assert_eq!(Method::from_str("POST").unwrap(), Method::POST);
        assert_eq!(Method::from_str("PUT").unwrap(), Method::PUT);
        assert_eq!(Method::from_str("DELETE").unwrap(), Method::DELETE);
        
        assert!(Method::from_str("INVALID").is_err());
        
        assert_eq!(Method::GET.to_string(), "GET");
        assert_eq!(Method::POST.to_string(), "POST");
    }

    #[test]
    fn test_http_versions() {
        use std::str::FromStr;
        
        assert_eq!(HttpVersion::from_str("HTTP/1.0").unwrap(), HttpVersion::Http1_0);
        assert_eq!(HttpVersion::from_str("HTTP/1.1").unwrap(), HttpVersion::Http1_1);
        assert_eq!(HttpVersion::from_str("HTTP/2.0").unwrap(), HttpVersion::Http2_0);
        
        assert!(HttpVersion::from_str("HTTP/3.0").is_err());
        
        assert_eq!(HttpVersion::Http1_1.to_string(), "HTTP/1.1");
    }

    #[test]
    fn test_error_types() {
        let io_error = GlowUpError::io_error("Test IO error");
        assert!(matches!(io_error, GlowUpError::Io(_)));
        
        let parse_error = GlowUpError::parse_error("Test parse error");
        assert!(matches!(parse_error, GlowUpError::Parse(_)));
        
        let http_error = GlowUpError::http_error(404, "Not found");
        assert!(matches!(http_error, GlowUpError::Http(404, _)));
    }

    #[test]
    fn test_integration_server_router_middleware() {
        let mut router = VibeRouter::new();
        
        // Add middleware
        router.use_middleware(logging_middleware);
        router.use_middleware(cors_middleware);
        
        // Add routes
        router.get("/", handler_func(|w, _r| {
            w.write_json(&serde_json::json!({"message": "Hello, GlowUpHTTP!"}))?;
            Ok(())
        }));
        
        router.post("/echo", handler_func(|w, r| {
            w.write(&r.body)?;
            Ok(())
        }));
        
        // Test GET route
        let get_request = VibeRequest::new(Method::GET, "/");
        let get_response = ResponderVibe::new();
        router.handle_vibe(&get_response, &get_request).unwrap();
        
        let body_str = String::from_utf8(get_response.get_body()).unwrap();
        assert!(body_str.contains("Hello, GlowUpHTTP!"));
        
        // Test POST route
        let mut post_request = VibeRequest::new(Method::POST, "/echo");
        post_request.body = b"Echo this message".to_vec();
        let post_response = ResponderVibe::new();
        router.handle_vibe(&post_response, &post_request).unwrap();
        
        assert_eq!(post_response.get_body(), b"Echo this message");
    }
}
