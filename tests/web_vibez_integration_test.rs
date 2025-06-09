/// Integration tests for the CURSED web_vibez framework
/// 
/// Tests the complete routing and middleware system working together

use cursed::stdlib::web_vibez::{
    Router, RouterConfig, Route, RouteGroup, HttpMethod, StatusCode,
    RouteMatcher, RoutePattern, ChainBuilder, MiddlewareOrdering, ChainExecution,
    RequestContext, ResponseContext, ContextData,
    LoggingMiddleware, CorsMiddleware, AuthMiddleware, RateLimitMiddleware, StaticFileMiddleware,
    StaticHandler, JsonApiHandler, TemplateHandler, RedirectHandler, CompositeHandler,
    AuthScheme, LogLevel, MiddlewareDependency
};

use std::sync::Arc;
use std::time::Duration;
use std::path::PathBuf;
use std::collections::HashMap;
use tokio;
use tracing_test::traced_test;

// Common test utilities
mod common;

#[traced_test]
#[tokio::test]
async fn test_basic_routing_functionality() {
    common::tracing::setup();
    
    let mut router = Router::new();
    let static_handler = Arc::new(StaticHandler::new("Hello World"));
    
    // Add basic routes
    router.get("/", static_handler.clone()).unwrap();
    router.get("/hello", static_handler.clone()).unwrap();
    router.post("/api/users", static_handler.clone()).unwrap();
    
    // Test route matching
    let matched = router.find_route(HttpMethod::GET, "/").unwrap();
    assert_eq!(matched.route.method, HttpMethod::GET);
    assert_eq!(matched.route.pattern, "/");
    
    let matched = router.find_route(HttpMethod::POST, "/api/users").unwrap();
    assert_eq!(matched.route.method, HttpMethod::POST);
    assert_eq!(matched.route.pattern, "/api/users");
    
    // Test non-existent route
    let no_match = router.find_route(HttpMethod::GET, "/nonexistent");
    assert!(no_match.is_none());
    
    println!("✅ Basic routing functionality tests passed");
}

#[traced_test]
#[tokio::test]
async fn test_route_parameters_and_wildcards() {
    common::tracing::setup();
    
    let mut router = Router::new();
    let handler = Arc::new(StaticHandler::new("Parameter handler"));
    
    // Add parameterized routes
    router.get("/users/:id", handler.clone()).unwrap();
    router.get("/users/:id/posts/:post_id", handler.clone()).unwrap();
    router.get("/files/*", handler.clone()).unwrap();
    router.get("/api/**", handler.clone()).unwrap();
    
    // Test parameter extraction
    let matched = router.find_route(HttpMethod::GET, "/users/123").unwrap();
    assert_eq!(matched.route_match.param("id"), Some("123"));
    
    let matched = router.find_route(HttpMethod::GET, "/users/456/posts/789").unwrap();
    assert_eq!(matched.route_match.param("id"), Some("456"));
    assert_eq!(matched.route_match.param("post_id"), Some("789"));
    
    // Test wildcard matching
    let matched = router.find_route(HttpMethod::GET, "/files/document.pdf").unwrap();
    assert!(matched.route.pattern.contains("*"));
    
    let matched = router.find_route(HttpMethod::GET, "/api/v1/users/123").unwrap();
    assert!(matched.route.pattern.contains("**"));
    
    println!("✅ Route parameters and wildcards tests passed");
}

#[traced_test]
#[tokio::test]
async fn test_route_groups_and_nesting() {
    common::tracing::setup();
    
    let mut router = Router::new();
    let handler = Arc::new(StaticHandler::new("API response"));
    
    // Create API v1 group
    let mut api_v1_group = RouteGroup::new("/api/v1");
    api_v1_group.add_route(
        Route::new(HttpMethod::GET, "/users", handler.clone(), vec![]).unwrap()
    );
    api_v1_group.add_route(
        Route::new(HttpMethod::POST, "/users", handler.clone(), vec![]).unwrap()
    );
    
    // Create users subgroup
    let mut users_subgroup = RouteGroup::new("/users");
    users_subgroup.add_route(
        Route::new(HttpMethod::GET, "/:id", handler.clone(), vec![]).unwrap()
    );
    users_subgroup.add_route(
        Route::new(HttpMethod::PUT, "/:id", handler.clone(), vec![]).unwrap()
    );
    
    api_v1_group.add_subgroup("users", users_subgroup);
    router.add_group("api_v1", api_v1_group);
    
    // Test grouped routes
    let matched = router.find_route(HttpMethod::GET, "/api/v1/users").unwrap();
    assert!(matched.route.pattern.starts_with("/api/v1"));
    
    let matched = router.find_route(HttpMethod::GET, "/api/v1/users/123").unwrap();
    assert!(matched.route.pattern.contains("/api/v1/users"));
    assert_eq!(matched.route_match.param("id"), Some("123"));
    
    println!("✅ Route groups and nesting tests passed");
}

#[traced_test]
#[tokio::test]
async fn test_middleware_chain_execution() {
    common::tracing::setup();
    
    // Create middleware chain
    let chain = ChainBuilder::new()
        .add(Arc::new(LoggingMiddleware::new()))
        .add(Arc::new(CorsMiddleware::new()))
        .with_ordering(MiddlewareOrdering::Priority)
        .with_execution(ChainExecution::FailFast)
        .build();
    
    let handler = Arc::new(StaticHandler::new("Test response"));
    let context = RequestContext::new("GET".to_string(), "/test".to_string());
    let response = ResponseContext::new();
    
    // Execute middleware chain
    let result = chain.execute(context, response, handler).await;
    assert!(result.is_ok());
    
    let final_response = result.unwrap();
    assert_eq!(final_response.status, StatusCode::OK);
    
    // Check that CORS headers were added
    assert!(final_response.header("Access-Control-Allow-Origin").is_some());
    
    println!("✅ Middleware chain execution tests passed");
}

#[traced_test]
#[tokio::test]
async fn test_authentication_middleware() {
    common::tracing::setup();
    
    let auth_middleware = AuthMiddleware::new(vec![AuthScheme::Bearer])
        .with_skip_paths(vec!["/public".to_string()]);
    
    // Test with missing auth header
    let mut context = RequestContext::new("GET".to_string(), "/protected".to_string());
    let mut response = ResponseContext::new();
    
    let result = auth_middleware.before_request(&mut context, &mut response).await;
    assert!(result.is_err());
    
    // Test with valid auth header
    context.add_header("Authorization", "Bearer valid_token");
    let result = auth_middleware.before_request(&mut context, &mut response).await;
    assert!(result.is_ok());
    assert_eq!(context.get_data("authenticated").unwrap().as_boolean(), Some(true));
    
    // Test skip path
    let mut public_context = RequestContext::new("GET".to_string(), "/public/resource".to_string());
    let mut public_response = ResponseContext::new();
    let result = auth_middleware.before_request(&mut public_context, &mut public_response).await;
    assert!(result.is_ok());
    
    println!("✅ Authentication middleware tests passed");
}

#[traced_test]
#[tokio::test]
async fn test_rate_limiting_middleware() {
    common::tracing::setup();
    
    let rate_limiter = RateLimitMiddleware::new(2) // 2 requests per window
        .with_window(2, Duration::from_secs(60));
    
    let mut context = RequestContext::new("GET".to_string(), "/api/test".to_string());
    context.set_client_ip("127.0.0.1".to_string());
    
    // First request should pass
    let mut response = ResponseContext::new();
    let result = rate_limiter.before_request(&mut context, &mut response).await;
    assert!(result.is_ok());
    
    // Second request should pass
    let mut response = ResponseContext::new();
    let result = rate_limiter.before_request(&mut context, &mut response).await;
    assert!(result.is_ok());
    
    // Third request should be rate limited
    let mut response = ResponseContext::new();
    let result = rate_limiter.before_request(&mut context, &mut response).await;
    assert!(result.is_err());
    assert_eq!(response.status, StatusCode::TOO_MANY_REQUESTS);
    
    println!("✅ Rate limiting middleware tests passed");
}

#[traced_test]
#[tokio::test]
async fn test_cors_middleware() {
    common::tracing::setup();
    
    let cors_middleware = CorsMiddleware::new()
        .with_origins(vec!["https://example.com".to_string()])
        .with_credentials(true);
    
    // Test preflight OPTIONS request
    let mut context = RequestContext::new("OPTIONS".to_string(), "/api/test".to_string());
    context.add_header("Origin", "https://example.com");
    let mut response = ResponseContext::new();
    
    let result = cors_middleware.before_request(&mut context, &mut response).await;
    assert!(result.is_ok());
    assert_eq!(response.status, StatusCode::NO_CONTENT);
    assert!(response.is_sent());
    
    // Test regular request
    let mut context = RequestContext::new("GET".to_string(), "/api/test".to_string());
    context.add_header("Origin", "https://example.com");
    let mut response = ResponseContext::new();
    
    let result = cors_middleware.after_response(&context, &mut response).await;
    assert!(result.is_ok());
    assert_eq!(response.header("Access-Control-Allow-Origin"), Some("https://example.com"));
    assert_eq!(response.header("Access-Control-Allow-Credentials"), Some("true"));
    
    println!("✅ CORS middleware tests passed");
}

#[traced_test]
#[tokio::test]
async fn test_static_file_middleware() {
    common::tracing::setup();
    
    // Create temporary directory and file for testing
    let temp_dir = std::env::temp_dir().join("web_vibez_test");
    std::fs::create_dir_all(&temp_dir).unwrap();
    
    let test_file = temp_dir.join("test.html");
    std::fs::write(&test_file, "<html><body>Test content</body></html>").unwrap();
    
    let static_middleware = StaticFileMiddleware::new(temp_dir.clone(), "/static")
        .with_cache_duration(Some(Duration::from_secs(3600)));
    
    // Test serving static file
    let mut context = RequestContext::new("GET".to_string(), "/static/test.html".to_string());
    let mut response = ResponseContext::new();
    
    let result = static_middleware.before_request(&mut context, &mut response).await;
    assert!(result.is_ok());
    assert!(response.is_sent());
    assert_eq!(response.status, StatusCode::OK);
    assert!(response.header("Content-Type").unwrap().contains("text/html"));
    
    // Cleanup
    std::fs::remove_dir_all(&temp_dir).unwrap();
    
    println!("✅ Static file middleware tests passed");
}

#[traced_test]
#[tokio::test]
async fn test_json_api_handler() {
    common::tracing::setup();
    
    let api_handler = JsonApiHandler::new()
        .on_get(|ctx| {
            Ok(serde_json::json!({
                "message": "Hello from GET",
                "path": ctx.path,
                "params": ctx.route_params
            }))
        })
        .on_post(|ctx| {
            Ok(serde_json::json!({
                "message": "Hello from POST",
                "received": "data"
            }))
        });
    
    // Test GET handler
    let context = RequestContext::new("GET".to_string(), "/api/test".to_string());
    let mut response = ResponseContext::new();
    
    let result = api_handler.handle(&context, &mut response).await;
    assert!(result.is_ok());
    assert_eq!(response.status, StatusCode::OK);
    assert!(response.header("Content-Type").unwrap().contains("application/json"));
    
    let body_str = String::from_utf8(response.body.clone()).unwrap();
    assert!(body_str.contains("Hello from GET"));
    
    // Test POST handler
    let context = RequestContext::new("POST".to_string(), "/api/test".to_string());
    let mut response = ResponseContext::new();
    
    let result = api_handler.handle(&context, &mut response).await;
    assert!(result.is_ok());
    assert_eq!(response.status, StatusCode::OK);
    
    // Test unsupported method
    let context = RequestContext::new("PATCH".to_string(), "/api/test".to_string());
    let mut response = ResponseContext::new();
    
    let result = api_handler.handle(&context, &mut response).await;
    assert!(result.is_ok());
    assert_eq!(response.status, StatusCode::METHOD_NOT_ALLOWED);
    
    println!("✅ JSON API handler tests passed");
}

#[traced_test]
#[tokio::test]
async fn test_template_handler() {
    common::tracing::setup();
    
    let template_handler = TemplateHandler::new("Hello, {{name}}! Path: {{path}}")
        .with_data_provider(|ctx| {
            let mut data = HashMap::new();
            data.insert("name".to_string(), "World".to_string());
            data.insert("custom".to_string(), "value".to_string());
            data
        });
    
    let mut context = RequestContext::new("GET".to_string(), "/template/test".to_string());
    context.add_param("user_id", "123");
    let mut response = ResponseContext::new();
    
    let result = template_handler.handle(&context, &mut response).await;
    assert!(result.is_ok());
    assert_eq!(response.status, StatusCode::OK);
    
    let body_str = String::from_utf8(response.body.clone()).unwrap();
    assert!(body_str.contains("Hello, World!"));
    assert!(body_str.contains("Path: /template/test"));
    
    println!("✅ Template handler tests passed");
}

#[traced_test]
#[tokio::test]
async fn test_redirect_handler() {
    common::tracing::setup();
    
    let redirect_handler = RedirectHandler::temporary("https://example.com/new-location");
    
    let context = RequestContext::new("GET".to_string(), "/old-path".to_string());
    let mut response = ResponseContext::new();
    
    let result = redirect_handler.handle(&context, &mut response).await;
    assert!(result.is_ok());
    assert_eq!(response.status.0, 302);
    assert_eq!(response.header("Location"), Some("https://example.com/new-location"));
    
    println!("✅ Redirect handler tests passed");
}

#[traced_test]
#[tokio::test]
async fn test_composite_handler() {
    common::tracing::setup();
    
    let default_handler = Arc::new(StaticHandler::new("Default response"));
    let json_handler = Arc::new(StaticHandler::json(r#"{"type":"json"}"#));
    let get_handler = Arc::new(StaticHandler::new("GET response"));
    
    let composite_handler = CompositeHandler::new(default_handler)
        .on_header("content-type", "application/json", json_handler)
        .on_method("GET", get_handler);
    
    // Test method-based routing
    let context = RequestContext::new("GET".to_string(), "/test".to_string());
    let mut response = ResponseContext::new();
    
    let result = composite_handler.handle(&context, &mut response).await;
    assert!(result.is_ok());
    assert_eq!(String::from_utf8(response.body.clone()).unwrap(), "GET response");
    
    // Test header-based routing
    let mut context = RequestContext::new("POST".to_string(), "/test".to_string());
    context.add_header("Content-Type", "application/json");
    let mut response = ResponseContext::new();
    
    let result = composite_handler.handle(&context, &mut response).await;
    assert!(result.is_ok());
    assert!(response.header("Content-Type").unwrap().contains("application/json"));
    
    println!("✅ Composite handler tests passed");
}

#[traced_test]
#[tokio::test]
async fn test_conditional_middleware() {
    common::tracing::setup();
    
    let chain = ChainBuilder::new()
        .add_conditional(
            Arc::new(CorsMiddleware::new()),
            |ctx| ctx.path.starts_with("/api")
        )
        .add_conditional(
            Arc::new(AuthMiddleware::new(vec![AuthScheme::Bearer])),
            |ctx| ctx.path.starts_with("/admin")
        )
        .build();
    
    let handler = Arc::new(StaticHandler::new("Test"));
    
    // Test API path (should trigger CORS)
    let context = RequestContext::new("GET".to_string(), "/api/users".to_string());
    let response = ResponseContext::new();
    let result = chain.execute(context, response, handler.clone()).await;
    assert!(result.is_ok());
    
    // Test admin path (should trigger auth - will fail without token)
    let context = RequestContext::new("GET".to_string(), "/admin/dashboard".to_string());
    let response = ResponseContext::new();
    let result = chain.execute(context, response, handler.clone()).await;
    assert!(result.is_ok()); // Should handle auth error gracefully
    
    // Test regular path (no special middleware)
    let context = RequestContext::new("GET".to_string(), "/public/page".to_string());
    let response = ResponseContext::new();
    let result = chain.execute(context, response, handler).await;
    assert!(result.is_ok());
    
    println!("✅ Conditional middleware tests passed");
}

#[traced_test]
#[tokio::test]
async fn test_middleware_dependency_ordering() {
    common::tracing::setup();
    
    let chain = ChainBuilder::new()
        .add(Arc::new(LoggingMiddleware::new()))
        .add(Arc::new(CorsMiddleware::new()))
        .add(Arc::new(AuthMiddleware::new(vec![AuthScheme::Bearer])))
        .add_dependency(MiddlewareDependency {
            middleware_name: "Auth".to_string(),
            depends_on: vec!["Logging".to_string()],
            runs_before: vec!["CORS".to_string()],
        })
        .with_ordering(MiddlewareOrdering::Dependency)
        .build();
    
    let middleware_names = chain.middleware_names();
    
    // Check that logging comes before auth
    let logging_pos = middleware_names.iter().position(|n| n == "Logging").unwrap();
    let auth_pos = middleware_names.iter().position(|n| n == "Auth").unwrap();
    assert!(logging_pos < auth_pos);
    
    // Check that auth comes before CORS
    let cors_pos = middleware_names.iter().position(|n| n == "CORS").unwrap();
    assert!(auth_pos < cors_pos);
    
    println!("✅ Middleware dependency ordering tests passed");
}

#[traced_test]
#[tokio::test]
async fn test_route_priority_and_conflict_resolution() {
    common::tracing::setup();
    
    let mut router = Router::new();
    let handler = Arc::new(StaticHandler::new("Handler"));
    
    // Add routes with potential conflicts
    router.get("/users/profile", handler.clone()).unwrap(); // Exact match (highest priority)
    router.get("/users/:id", handler.clone()).unwrap();     // Parameter (lower priority)
    router.get("/users/*", handler.clone()).unwrap();       // Wildcard (lowest priority)
    
    // Test that exact match has highest priority
    let matched = router.find_route(HttpMethod::GET, "/users/profile").unwrap();
    assert_eq!(matched.route.pattern, "/users/profile");
    
    // Test that parameter match works for non-exact paths
    let matched = router.find_route(HttpMethod::GET, "/users/123").unwrap();
    assert_eq!(matched.route.pattern, "/users/:id");
    assert_eq!(matched.route_match.param("id"), Some("123"));
    
    // Test that wildcard catches everything else
    let matched = router.find_route(HttpMethod::GET, "/users/123/extra/path").unwrap();
    assert_eq!(matched.route.pattern, "/users/*");
    
    println!("✅ Route priority and conflict resolution tests passed");
}

#[traced_test]
#[tokio::test]
async fn test_performance_and_metrics() {
    common::tracing::setup();
    
    let mut router = Router::new();
    let handler = Arc::new(StaticHandler::new("Performance test"));
    
    // Add many routes to test performance
    for i in 0..100 {
        router.get(&format!("/api/endpoint_{}", i), handler.clone()).unwrap();
        router.get(&format!("/api/users/:id/resource_{}", i), handler.clone()).unwrap();
    }
    
    // Test route lookup performance
    let start = std::time::Instant::now();
    for i in 0..1000 {
        let path = format!("/api/endpoint_{}", i % 100);
        router.find_route(HttpMethod::GET, &path);
    }
    let elapsed = start.elapsed();
    
    // Should be very fast (< 10ms for 1000 lookups)
    assert!(elapsed < Duration::from_millis(10));
    
    let stats = router.get_stats();
    assert!(stats.total_lookups >= 1000);
    assert!(stats.average_lookup_time_ns < 100_000); // < 100 microseconds
    
    // Test middleware chain performance
    let chain = ChainBuilder::new()
        .add(Arc::new(LoggingMiddleware::new()))
        .add(Arc::new(CorsMiddleware::new()))
        .with_metrics(true)
        .build();
    
    let start = std::time::Instant::now();
    for _ in 0..100 {
        let context = RequestContext::new("GET".to_string(), "/test".to_string());
        let response = ResponseContext::new();
        let _ = chain.execute(context, response, handler.clone()).await;
    }
    let elapsed = start.elapsed();
    
    // Should be reasonably fast (< 100ms for 100 executions)
    assert!(elapsed < Duration::from_millis(100));
    
    let metrics = chain.get_metrics().unwrap();
    assert_eq!(metrics.successful_executions, 100);
    assert!(metrics.total_execution_time > Duration::ZERO);
    
    println!("✅ Performance and metrics tests passed");
    println!("   - Router: {} lookups in {:?} (avg: {}ns)", 
        stats.total_lookups, elapsed, stats.average_lookup_time_ns);
    println!("   - Middleware: {} executions (avg: {:?})", 
        metrics.successful_executions, metrics.average_execution_time);
}

#[traced_test]
#[tokio::test]
async fn test_complete_request_lifecycle() {
    common::tracing::setup();
    
    // Setup complete router with middleware and handlers
    let mut router = Router::new();
    
    // Add global middleware
    router.use_middleware(Arc::new(LoggingMiddleware::new()));
    router.use_middleware(Arc::new(CorsMiddleware::new()));
    
    // Create API handler
    let api_handler = Arc::new(JsonApiHandler::new()
        .on_get(|ctx| {
            Ok(serde_json::json!({
                "method": ctx.method.to_string(),
                "path": ctx.path,
                "params": ctx.route_params,
                "query": ctx.query_params
            }))
        })
        .on_post(|ctx| {
            let body_str = ctx.body_string().unwrap_or_default();
            Ok(serde_json::json!({
                "received": body_str,
                "length": ctx.body().len()
            }))
        }));
    
    // Add routes
    router.get("/api/users/:id", api_handler.clone()).unwrap();
    router.post("/api/users", api_handler.clone()).unwrap();
    
    // Test complete GET request
    let mut context = RequestContext::new("GET".to_string(), "/api/users/123".to_string());
    context.add_query_param("include", "profile");
    context.add_header("User-Agent", "test-client/1.0");
    
    let result = router.handle_request(HttpMethod::GET, "/api/users/123", context).await;
    assert!(result.is_ok());
    
    let response = result.unwrap();
    assert_eq!(response.status, StatusCode::OK);
    assert!(response.header("Content-Type").unwrap().contains("application/json"));
    assert!(response.header("Access-Control-Allow-Origin").is_some());
    
    let body_str = String::from_utf8(response.body.clone()).unwrap();
    let json: serde_json::Value = serde_json::from_str(&body_str).unwrap();
    assert_eq!(json["method"], "GET");
    assert_eq!(json["path"], "/api/users/123");
    
    // Test complete POST request
    let mut context = RequestContext::new("POST".to_string(), "/api/users".to_string());
    context.set_body(r#"{"name":"John","email":"john@example.com"}"#.as_bytes().to_vec());
    context.add_header("Content-Type", "application/json");
    
    let result = router.handle_request(HttpMethod::POST, "/api/users", context).await;
    assert!(result.is_ok());
    
    let response = result.unwrap();
    assert_eq!(response.status, StatusCode::OK);
    
    println!("✅ Complete request lifecycle tests passed");
}

#[traced_test]
#[tokio::test]
async fn test_error_handling_and_propagation() {
    common::tracing::setup();
    
    let mut router = Router::new();
    
    // Add handler that always fails
    let failing_handler = Arc::new(JsonApiHandler::new()
        .on_get(|_ctx| {
            Err(cursed::stdlib::web_vibez::HandlerError::Internal("Simulated error".to_string()))
        }));
    
    router.get("/fail", failing_handler).unwrap();
    
    // Test error handling
    let context = RequestContext::new("GET".to_string(), "/fail".to_string());
    let result = router.handle_request(HttpMethod::GET, "/fail", context).await;
    assert!(result.is_ok());
    
    let response = result.unwrap();
    assert_eq!(response.status, StatusCode::INTERNAL_SERVER_ERROR);
    
    // Test 404 handling
    let context = RequestContext::new("GET".to_string(), "/nonexistent".to_string());
    let result = router.handle_request(HttpMethod::GET, "/nonexistent", context).await;
    assert!(result.is_ok());
    
    let response = result.unwrap();
    assert_eq!(response.status, StatusCode::NOT_FOUND);
    
    println!("✅ Error handling and propagation tests passed");
}

/// Helper function to create test context
fn create_test_context(method: &str, path: &str) -> RequestContext {
    let mut context = RequestContext::new(method.to_string(), path.to_string());
    context.set_client_ip("127.0.0.1".to_string());
    context.add_header("User-Agent", "web_vibez_test/1.0");
    context
}

/// Helper function to assert response status
fn assert_response_status(response: &ResponseContext, expected_status: StatusCode) {
    assert_eq!(response.status, expected_status,
        "Expected status {}, got {}", expected_status.0, response.status.0);
}

/// Performance benchmark helper
async fn benchmark_route_matching(router: &mut Router, iterations: usize) -> Duration {
    let start = std::time::Instant::now();
    
    for i in 0..iterations {
        let path = format!("/api/users/{}", i % 1000);
        router.find_route(HttpMethod::GET, &path);
    }
    
    start.elapsed()
}

/// Memory usage helper
fn get_memory_usage() -> usize {
    // Simplified memory usage estimation
    // In a real implementation, you might use a more sophisticated approach
    std::mem::size_of::<Router>() +
    std::mem::size_of::<RequestContext>() +
    std::mem::size_of::<ResponseContext>()
}

#[traced_test]
#[tokio::test]
async fn test_framework_integration_comprehensive() {
    common::tracing::setup();
    
    println!("🚀 Running comprehensive web_vibez framework integration test...");
    
    // Setup complete framework
    let config = RouterConfig {
        max_cache_size_per_method: 500,
        debug_mode: true,
        case_sensitive: true,
        strict_slash: false,
        max_priority_conflicts: 5,
    };
    
    let mut router = Router::with_config(config);
    
    // Add comprehensive middleware stack
    router.use_middleware(Arc::new(LoggingMiddleware::new()
        .with_body_logging(true, false)));
    router.use_middleware(Arc::new(RateLimitMiddleware::new(1000)));
    router.use_middleware(Arc::new(CorsMiddleware::new()
        .with_origins(vec!["*".to_string()])));
    
    // Create various handlers
    let static_handler = Arc::new(StaticHandler::new("Static content"));
    let json_handler = Arc::new(JsonApiHandler::new()
        .on_get(|ctx| Ok(serde_json::json!({"path": ctx.path})))
        .on_post(|ctx| Ok(serde_json::json!({"received": ctx.body().len()}))));
    let template_handler = Arc::new(TemplateHandler::new("Page: {{path}}"));
    let redirect_handler = Arc::new(RedirectHandler::temporary("/new-location"));
    
    // Add comprehensive routes
    router.get("/", static_handler.clone()).unwrap();
    router.get("/api/health", json_handler.clone()).unwrap();
    router.post("/api/data", json_handler.clone()).unwrap();
    router.get("/template/:page", template_handler).unwrap();
    router.get("/redirect", redirect_handler).unwrap();
    router.get("/users/:id/posts/:post_id", static_handler.clone()).unwrap();
    router.get("/files/**", static_handler.clone()).unwrap();
    
    // Create API group
    let mut api_group = RouteGroup::new("/api/v2");
    api_group.add_route(
        Route::new(HttpMethod::GET, "/users", json_handler.clone(), vec![]).unwrap()
    );
    api_group.add_route(
        Route::new(HttpMethod::POST, "/users", json_handler.clone(), 
            vec![Arc::new(AuthMiddleware::new(vec![AuthScheme::Bearer]))]).unwrap()
    );
    router.add_group("api_v2", api_group);
    
    // Test comprehensive functionality
    let test_cases = vec![
        ("GET", "/", StatusCode::OK),
        ("GET", "/api/health", StatusCode::OK),
        ("POST", "/api/data", StatusCode::OK),
        ("GET", "/template/home", StatusCode::OK),
        ("GET", "/redirect", StatusCode(302)),
        ("GET", "/users/123/posts/456", StatusCode::OK),
        ("GET", "/files/docs/readme.md", StatusCode::OK),
        ("GET", "/api/v2/users", StatusCode::OK),
        ("GET", "/nonexistent", StatusCode::NOT_FOUND),
    ];
    
    for (method, path, expected_status) in test_cases {
        let context = create_test_context(method, path);
        let result = router.handle_request(
            method.parse().unwrap(), 
            path, 
            context
        ).await;
        
        assert!(result.is_ok(), "Request failed for {} {}", method, path);
        let response = result.unwrap();
        assert_response_status(&response, expected_status);
        
        println!("  ✅ {} {} -> {}", method, path, response.status.0);
    }
    
    // Performance testing
    let performance_start = std::time::Instant::now();
    let _ = benchmark_route_matching(&mut router, 1000).await;
    let performance_time = performance_start.elapsed();
    
    // Memory usage estimation
    let memory_usage = get_memory_usage();
    
    // Statistics
    let stats = router.get_stats();
    
    println!("📊 Framework Performance Summary:");
    println!("  - Total routes: {}", stats.total_routes);
    println!("  - Total lookups: {}", stats.total_lookups);
    println!("  - Success rate: {:.2}%", 
        (stats.successful_matches as f64 / stats.total_lookups as f64) * 100.0);
    println!("  - Average lookup time: {}ns", stats.average_lookup_time_ns);
    println!("  - Cache hit rate: {:.2}%", stats.cache_hit_rate * 100.0);
    println!("  - Performance test time: {:?}", performance_time);
    println!("  - Memory usage estimate: {} bytes", memory_usage);
    
    println!("🎉 Comprehensive integration test completed successfully!");
    
    // Assertions for performance requirements
    assert!(stats.average_lookup_time_ns < 50_000, "Route lookup too slow");
    assert!(performance_time < Duration::from_millis(50), "Performance test too slow");
    assert!(stats.cache_hit_rate > 0.0, "Cache not being used");
    
    println!("✅ All performance requirements met");
}
