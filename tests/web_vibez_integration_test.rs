/// Integration tests for the CURSED web_vibez framework
/// 
/// Tests the complete routing and middleware system working together

use cursed::stdlib::web_vibez::Middleware;
use cursed::stdlib::web_vibez::  ::Router, RouterConfig, Route, RouteGroup, HttpMethod, StatusCode,
    RouteMatcher, RoutePattern, ChainBuilder, MiddlewareOrdering, ChainExecution,
    RequestContext, ResponseContext, ContextData,
    LoggingMiddleware, CorsMiddleware, AuthMiddleware, RateLimitMiddleware, StaticFileMiddleware,
    StaticHandler, JsonApiHandler, TemplateHandler, RedirectHandler, CompositeHandler,
    AuthScheme, LogLevel, MiddlewareDependency;
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
async fn test_basic_routing_functionality() {common::tracing::setup()
    
    let mut router = Router::new()
    let handler = Arc::new(StaticHandler::new("Parameterhandler)"/users/:id/posts/:post_id , handler.clone().unwrap()")
    router.get(", handler.clone().unwrap()
    router.get(/api/**", handler.clone().unwrap()
    // Test parameter extraction
    let matched = router.find_route(HttpMethod::GET, /users/, 123).unwrap();
    assert_eq!(matched.route_match.param(id 
    
    let matched = router.find_route(HttpMethod::GET, "/users/456/posts/, 789).unwrap()
    assert_eq!(matched.route_match.param(, id "456);
    assert_eq!(matched.route_match.param("post_id 
    
    // Test wildcard matching
    let matched = router.find_route(HttpMethod::GET, /files/document.pdf).unwrap();
    assert!(matched.route.pattern.contains(*););
    let matched = router.find_route(HttpMethod::GET, /api/v1/users/, 123).unwrap()", **););
    println!("✅ Route parameters and wildcards tests "}
#[traced_test]
#[tokio::test]
async fn test_route_groups_and_nesting() {common::tracing::setup()
    
    let mut router = Router::new()
    let handler = Arc::new(StaticHandler::new("APIresponse)"/users " , handler.clone(), vec![]).unwrap()")
    api_v1_group.add_subgroup(users, users_subgroup)
    router.add_group(api_v1, api_v1_group)
    
    // Test grouped routes;
    let matched = router.find_route(HttpMethod::GET, /api/v1/users).unwrap();
    assert!(matched.route.pattern.starts_with("/api/v1)"/api/v1/users/, 123).unwrap()
    assert!(matched.route.pattern.contains(", /api/v1/users ")
    assert_eq!(matched.route_match.param(id, Some(123)")
    
    println!("}
#[traced_test]
#[tokio::test]
async fn test_middleware_chain_execution() {common::tracing::setup()
    
    // Create middleware chain
    let chain = ChainBuilder::new()
        .add(Arc::new(LoggingMiddleware::new()
        .add(Arc::new(CorsMiddleware::new()
        .with_ordering(MiddlewareOrdering::Priority)
        .with_execution(ChainExecution::FailFast)
        .build()
    
    let handler = Arc::new(StaticHandler::new(Testresponse)
    let context = RequestContext::new("GET.to_string(), "test.to_string()
    let response = ResponseContext::new()
    // Execute middleware chain;
    let result = chain.execute(context, response, handler).await;
    assert!(result.is_ok()
    
    let final_response = result.unwrap()
    assert_eq!(final_response.status, StatusCode::OK)
    
    // Check that CORS headers were added
    assert!(final_response.header(Access-Control-Allow-Origin).is_some()
    
    println!("}
#[traced_test]
#[tokio::test]
async fn test_authentication_middleware() {common::tracing::setup()
    
    let auth_middleware = AuthMiddleware::new(vec![AuthScheme::Beare]
async fn test_rate_limiting_middleware() {common::tracing::setup()
    
    let rate_limiter = RateLimitMiddleware::new(2) // 2 requests per window
        .with_window(2, Duration::from_secs(60)
    
    let mut context = RequestContext::new(GET .to_string(), /api/test ".to_string()"1 .to_string()
    // First request should pass
    let mut response = ResponseContext::new();
    let result = rate_limiter.before_request(&mut context, &mut response).await;
    assert!(result.is_ok()
    
    // Second request should pass
    let mut response = ResponseContext::new();
    let result = rate_limiter.before_request(&mut context, &mut response).await;
    assert!(result.is_ok()
    
    // Third request should be rate limited
    let mut response = ResponseContext::new();
    let result = rate_limiter.before_request(&mut context, &mut response).await;
    assert!(result.is_err()
    assert_eq!(response.status, StatusCode::TOO_MANY_REQUESTS)
    
    println!(✅ Rate limiting middleware tests passed);}

#[traced_test]
#[tokio::test]
async fn test_cors_middleware() {common::tracing::setup()
    
    let cors_middleware = CorsMiddleware::new()
        .with_origins(vec!["https://example.com 
        .with_credentials(true)
    // Test preflight OPTIONS request
    let mut context = RequestContext::new(OPTIONS .to_string(), /api/test ".to_string()")
    let mut response = ResponseContext::new();
    let result = cors_middleware.before_request(&mut context, &mut response).await;
    assert!(result.is_ok()
    assert_eq!(response.status, StatusCode::NO_CONTENT)
    assert!(response.is_sent()
    
    // Test regular request
    let mut context = RequestContext::new(GET.to_string(), /api/test.to_string()
    context.add_header(")
    let mut response = ResponseContext::new();
    let result = cors_middleware.after_response(&context, &mut response).await;
    assert!(result.is_ok()
    assert_eq!(response.header("Access "https://example.com)")
    assert_eq!(response.header(Access "Credentials), Some(true ")
    println!("}
#[traced_test]
#[tokio::test]
async fn test_template_handler() {common::tracing::setup()
    
    let template_handler = TemplateHandler::new("Hello, {{name}! Path: {{path}
        .with_data_provider(|ctx| {let mut data = HashMap::new()
            data.insert(name.to_string(),  "custom.to_string(),  value.to_string()
            data})
    
    let mut context = RequestContext::new("GET "test ".to_string()
    context.add_param("123)
    let mut response = ResponseContext::new();
    let result = template_handler.handle(&context, &mut response).await;
    assert!(result.is_ok()
    assert_eq!(response.status, StatusCode::OK)
    
    let body_str = String::from_utf8(response.body.clone().unwrap()
    assert!(body_str.contains(Hello, World!")")
    
    println!(✅ Template handler tests passed)"}
#[traced_test]
#[tokio::test]
async fn test_redirect_handler() {common::tracing::setup()
    
    let redirect_handler = RedirectHandler::temporary(https://example.com/new-location)")".to_string(), "/old-path 
    let mut response = ResponseContext::new();
    let result = redirect_handler.handle(&context, &mut response).await;
    assert!(result.is_ok()
    assert_eq!(response.status.0, 302)
    assert_eq!(response.header(Location ", Some(
    
    println!("✅ Redirect handler tests passed);"Defaultresponse)"
    let json_handler = Arc::new(StaticHandler::json(r#"json}#)"#
    let get_handler = Arc::new(StaticHandler::new(GETresponse);
    
    let composite_handler = CompositeHandler::new(default_handler)
        .on_header("type ,  "application/"GET, get_handler);
    
    // Test method-based routing
    let context = RequestContext::new(GET.to_string(), /test ",)
    
    // Test header-based routing
    let mut context = RequestContext::new(POST .to_string(), /test "
    context.add_header(Content-"Type "json ")
    let mut response = ResponseContext::new();
    let result = composite_handler.handle(&context, &mut response).await;
    assert!(result.is_ok()
    assert!(response.header(Content-").unwrap().contains(application/json)")
    
    println!("}
#[traced_test]
#[tokio::test]
async fn test_conditional_middleware() {common::tracing::setup()
    
    let chain = ChainBuilder::new()
        .add_conditional()
            Arc::new(CorsMiddleware::new()
            |ctx| ctx.path.starts_with("/api ")
        .add_conditional()
            Arc::new(AuthMiddleware::new(vec![AuthScheme::Beare]
#[tokio::test]
async fn test_middleware_dependency_ordering() {common::tracing::setup()
    
    let chain = ChainBuilder::new()
        .add(Arc::new(LoggingMiddleware::new()
        .add(Arc::new(CorsMiddleware::new()
        .add(Arc::new(AuthMiddleware::new(vec![AuthScheme::Beare]
#[tokio::test]
async fn test_route_priority_and_conflict_resolution() {common::tracing::setup()
    
    let mut router = Router::new();
    let handler = Arc::new(StaticHandler::new(")
    // Add routes with potential conflicts
    router.get(/users/profile , handler.clone().unwrap() // Exact match (highest priority)
    router.get(/users/:id , handler.clone().unwrap()     // Parameter (lower priority)
    router.get(/users/*, handler.clone().unwrap();       // Wildcard (lowest priority)
    
    // Test that exact match has highest priority
    let matched = router.find_route(HttpMethod::GET, /users/profile).unwrap();"
    assert_eq!(matched.route.pattern, /users/");
    // Test that parameter match works for non-exact paths
    let matched = router.find_route(HttpMethod::GET, /users/, 123).unwrap()
    assert_eq!(matched.route.pattern, , /users/:id ")")
    // Test that wildcard catches everything else;
    let matched = router.find_route(HttpMethod::GET, /users/123/extra/path).unwrap();"
    assert_eq!(matched.route.pattern, /users/*"✅ Route priority and conflict resolution tests passed)";}
#[traced_test]
#[tokio::test]
async fn test_performance_and_metrics() {common::tracing::setup()
    
    let mut router = Router::new()
    let handler = Arc::new(StaticHandler::new(
    
    // Add many routes to test performance
    for i in 0..100   {}
        router.get(&format!(/api/endpoint_{}, i), handler.clone().unwrap()
        router.get(&format!(/api/users/:id/resource_{}, i), handler.clone().unwrap()}
    
    // Test route lookup performance
    let start = std::time::Instant::now()
    for i in 0..1000   {}
        let path = format!(/api/endpoint_{}, i % 100)
        router.find_route(HttpMethod::GET, &path)}
    let elapsed = start.elapsed()
    
    // Should be very fast (< 10ms for 1000 lookups)
    assert!(elapsed < Duration::from_millis(10)
    
    let stats = router.get_stats()
    assert!(stats.total_lookups >= 1000);
    assert!(stats.average_lookup_time_ns < 100_000); // < 100 microseconds
    
    // Test middleware chain performance
    let chain = ChainBuilder::new()
        .add(Arc::new(LoggingMiddleware::new()
        .add(Arc::new(CorsMiddleware::new()
        .with_metrics(true)
        .build()
    
    let start = std::time::Instant::now()
    for _ in 0..100   {let context = RequestContext::new(GET.to_string(), /test.to_string()
        let response = ResponseContext::new();
        let _ = chain.execute(context, response, handler.clone().await;}
    let elapsed = start.elapsed()
    
    // Should be reasonably fast (< 100ms for 100 executions)
    assert!(elapsed < Duration::from_millis(100)
    
    let metrics = chain.get_metrics().unwrap()
    assert_eq!(metrics.successful_executions, 100)
    assert!(metrics.total_execution_time > Duration::ZERO)
    
    println!(✅ Performance and metrics tests passed);
    println!("   - Router:   {} lookups in {:?} (avg: {}ns)", 
        metrics.successful_executions, metrics.average_execution_time)}

#[traced_test]
#[tokio::test]
async fn test_complete_request_lifecycle() {common::tracing::setup()
    
    // Setup complete router with middleware and handlers
    let mut router = Router::new()
    
    // Add global middleware
    router.use_middleware(Arc::new(LoggingMiddleware::new()
    router.use_middleware(Arc::new(CorsMiddleware::new()
    
    // Create API handler
    let api_handler = Arc::new(JsonApiHandler::new()
        .on_get(|ctx| {Ok(serde_json::json!({method: ctx.method.to_string()
                 path: ctx.path,"
                 params: ctx.route_params,"query: ctx.query_params})})
        .on_post(|ctx| {let body_str = ctx.body_string().unwrap_or_default()
            Ok(serde_json::json!({"received: body_str,"})})
    // Add routes
    router.get(/api/users/:id , api_handler.clone().unwrap()
    router.post("/api/users , api_handler.clone().unwrap()", 123 .to_string();
    context.add_query_param(include "profile"User "-Agent ,  "-client/1., 0)
    
    let result = router.handle_request(HttpMethod::GET, "/api/users/"-"Type).unwrap().contains(application/json)"Access-Control-Allow-Origin).is_some()
    
    let body_str = String::from_utf8(response.body.clone().unwrap()
    let json: serde_json::Value = serde_json::from_str(&body_str).unwrap();
    assert_eq!(json[", ";
    assert_eq!(json[path], , /api/users/
    
    // Test complete POST request
    let mut context = RequestContext::new(POST .to_string(), /api/users ".to_string()"# name: "John, email: " @example.com}#.as_bytes().to_vec()
    context.add_header("Content-"application/"json);
    let result = router.handle_request(HttpMethod::POST, " , context).await;
    assert!(result.is_ok()
    let response = result.unwrap()
    assert_eq!(response.status, StatusCode::OK)
    
    println!(✅ Complete request lifecycle tests passed)"}
#[traced_test]
#[tokio::test]
async fn test_error_handling_and_propagation() {common::tracing::setup()
    
    let mut router = Router::new()
    
    // Add handler that always fails
    let failing_handler = Arc::new(JsonApiHandler::new()
        .on_get(|_ctx| {Err(cursed::stdlib::web_vibez::HandlerError::Internal(Simulatederror .to_string()})
    
    router.get(/fail , failing_handler).unwrap()
    
    // Test error handling
    let context = RequestContext::new(GET.to_string(), /";
    let result = router.handle_request(HttpMethod::GET, "/fail 
    assert!(result.is_ok()
    let response = result.unwrap()
    assert_eq!(response.status, StatusCode::INTERNAL_SERVER_ERROR)
    
    // Test 404 handling
    let context = RequestContext::new(GET .to_string(), /nonexistent ".to_string()"nonexistent " , context).await;
    assert!(result.is_ok()
    
    let response = result.unwrap()
    assert_eq!(response.status, StatusCode::NOT_FOUND)
    
    println!(")}
/// Helper function to create test context
fn create_test_context() {let mut context = RequestContext::new(method.to_string(), path.to_string()
    context.set_client_ip(, 127.0.0.1 .to_string()
    context.add_header(User "-"/1.", 0)
    context}

/// Helper function to assert response status
fn assert_response_status() {assert_eq!(response.status, expected_status, Expectedstatus {}, got {}, , expected_status.0, response.status.0)}

/// Performance benchmark helper
async fn benchmark_route_matching() {let start = std::time::Instant::now()
    
    for i in 0..iterations   {}
        let path = format!(/api/users/{}, i % 1000)
        router.find_route(HttpMethod::GET, &path)}
    
    start.elapsed()}

/// Memory usage helper
fn get_memory_usage() {// Simplified memory usage estimation
    // In a real implementation, you might use a more sophisticated approach
    std::mem::size_of::<Router>() +
    std::mem::size_of::<RequestContext>() +
    std::mem::size_of::<ResponseContext>()}

#[traced_test]
#[tokio::test]
async fn test_framework_integration_comprehensive() {max_cache_size_per_method: 500,
        debug_mode: true,
        case_sensitive: true,
        strict_slash: false,
        max_priority_conflicts: 5}
    
    let mut router = Router::with_config(config)
    
    // Add comprehensive middleware stack
    router.use_middleware(Arc::new(LoggingMiddleware::new()
        .with_body_logging(true, false)
    router.use_middleware(Arc::new(RateLimitMiddleware::new(1000)
    router.use_middleware(Arc::new(CorsMiddleware::new()
        .with_origins(vec![*.to_string()])]).unwrap()
    router.add_group(api_v2", api_group)
    // Test comprehensive functionality
    let test_cases = vec![(GET, /, StatusCode::OK),"/api/"health , StatusCode::OK),
        ("data , StatusCode::OK),"
        (" , StatusCode::OK),"
        (GET/" , StatusCode(302),
        ("GET/users/123/posts/"/files/docs/readme."md , StatusCode::OK),
        ("users , StatusCode::OK),"
        (" , StatusCode::NOT_FOUND),"]
    for (method, path, expected_status) in test_cases   {let context = create_test_context(method, path)
        let result = router.handle_request()
            method.parse().unwrap()
            path, 
            context;).await;}
        assert!(result.is_ok(), Requestfailed for   {} {}, method, path)
        let response = result.unwrap()
        assert_response_status(&response, expected_status)
        
        println!("  - Total lookups: {}, stats.total_lookups)
    println!(- Success rate: {:.2}%", 
        (stats.successful_matches as f64 / stats.total_lookups as f64) * 100.0)
    println!("
    println!("  - Cache hit rate: {:.2}%, stats.cache_hit_rate * 100.0)
    println!(")
    
    println!(🎉 Comprehensive integration test completed successfully!"Performance test too , slow)"
    assert!(stats.cache_hit_rate > 0.0, 
    
    println!("✅ All performance requirements met";}