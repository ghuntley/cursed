/// Performance tests for the CURSED web_vibez HTTP server
/// 
/// Tests server performance under various load conditions:
/// - High connection count
/// - Large request/response bodies  
/// - Sustained request load
/// - Memory usage patterns
/// - Connection pooling efficiency

use cursed::stdlib::web_vibez::{
    HttpServer, HttpMethod, StatusCode, Router, MiddlewareChain, WebVibezConfig
};
use cursed::stdlib::web_vibez::config::{
    ServerConfig, SecurityConfig, PerformanceConfig, SessionConfig, TemplateConfig,
    StaticFileConfig, LoggingConfig, DevelopmentConfig
};
use cursed::stdlib::web_vibez::handlers::{RequestHandler, HandlerResult};
use cursed::stdlib::web_vibez::context::{RequestContext, ResponseContext};
use cursed::stdlib::web_vibez::server::{Connection, ConnectionPool};

use std::collections::HashMap;
use std::net::SocketAddr;
use std::time::{Duration, Instant};
use std::thread;
use std::sync::{Arc, atomic::{AtomicU64, AtomicUsize, Ordering}};

/// High-performance handler for load testing
#[derive(Debug)]
pub struct FastHandler {
    response_size: usize,
    processing_delay: Duration,
}

impl FastHandler {
    pub fn new(response_size: usize, processing_delay: Duration) -> Self {
        Self { response_size, processing_delay }
    }
}

impl RequestHandler for FastHandler {
    fn handle(&self, _request: &mut RequestContext) -> HandlerResult {
        // Simulate processing time
        if self.processing_delay > Duration::from_nanos(0) {
            thread::sleep(self.processing_delay);
        }
        
        let mut response = ResponseContext::new();
        response.set_status(StatusCode::OK);
        
        // Generate response of specified size
        let body = vec![b'A'; self.response_size];
        response.set_body(body);
        response.add_header("Content-Type", "text/plain");
        
        Ok(response)
    }
}

/// Slow handler for testing timeout behavior
#[derive(Debug)]
pub struct SlowHandler {
    delay: Duration,
}

impl SlowHandler {
    pub fn new(delay: Duration) -> Self {
        Self { delay }
    }
}

impl RequestHandler for SlowHandler {
    fn handle(&self, _request: &mut RequestContext) -> HandlerResult {
        thread::sleep(self.delay);
        
        let mut response = ResponseContext::new();
        response.set_status(StatusCode::OK);
        response.set_body(b"Slow response".to_vec());
        
        Ok(response)
    }
}

/// Create high-performance test configuration
fn create_performance_config(port: u16) -> WebVibezConfig {
    WebVibezConfig {
        server: ServerConfig {
            host: "127.0.0.1".to_string(),
            port,
            max_connections: 1000,
            request_timeout: Duration::from_secs(30),
            keep_alive_timeout: Duration::from_secs(60),
            header_timeout: Duration::from_secs(10),
            connection_timeout: Duration::from_secs(300),
            max_header_size: 16384,
            max_body_size: 10 * 1024 * 1024, // 10MB
        },
        security: SecurityConfig {
            csrf_secret: "perf_test_csrf".to_string(),
            session_secret: "perf_test_session".to_string(),
            enable_xss_protection: false,
            enable_csrf_protection: false,
            allowed_origins: vec!["*".to_string()],
            content_security_policy: None,
            hsts_max_age: None,
            enable_secure_headers: false,
        },
        performance: PerformanceConfig {
            enable_compression: false, // Disable for pure performance testing
            compression_level: 1,
            max_request_size: 10 * 1024 * 1024,
            worker_threads: 8,
            connection_pool_size: 100,
            enable_http2: false,
            enable_request_id: false,
        },
        session: SessionConfig {
            cookie_name: "perf_session".to_string(),
            secret_key: "perf_secret".to_string(),
            max_age: Duration::from_secs(3600),
            secure: false,
            http_only: false,
            same_site: "None".to_string(),
            domain: None,
            path: "/".to_string(),
        },
        template: TemplateConfig {
            template_dir: "templates".to_string(),
            cache_templates: true,
            auto_reload: false,
        },
        static_files: StaticFileConfig {
            static_dir: "static".to_string(),
            enable_directory_listing: false,
            cache_control: "public, max-age=3600".to_string(),
        },
        logging: LoggingConfig {
            level: "error".to_string(), // Minimal logging for performance
            format: "json".to_string(),
            enable_request_logging: false,
            log_file: None,
        },
        development: DevelopmentConfig {
            hot_reload: false,
            debug_mode: false,
            profiling: false,
        },
    }
}

/// Create router optimized for performance testing
fn create_performance_router() -> Router {
    let mut router = Router::new();
    
    // Fast endpoints with different response sizes
    router.get("/fast/small", Arc::new(FastHandler::new(
        100, Duration::from_nanos(0)
    ))).unwrap();
    
    router.get("/fast/medium", Arc::new(FastHandler::new(
        1024, Duration::from_nanos(0)
    ))).unwrap();
    
    router.get("/fast/large", Arc::new(FastHandler::new(
        10 * 1024, Duration::from_nanos(0)
    ))).unwrap();
    
    // Endpoints with processing delays
    router.get("/slow/1ms", Arc::new(FastHandler::new(
        100, Duration::from_millis(1)
    ))).unwrap();
    
    router.get("/slow/10ms", Arc::new(FastHandler::new(
        100, Duration::from_millis(10)
    ))).unwrap();
    
    router.get("/slow/100ms", Arc::new(FastHandler::new(
        100, Duration::from_millis(100)
    ))).unwrap();
    
    // Very slow endpoint for timeout testing
    router.get("/timeout", Arc::new(SlowHandler::new(
        Duration::from_secs(5)
    ))).unwrap();
    
    router
}

/// Performance metrics collector
#[derive(Debug, Default)]
pub struct PerformanceMetrics {
    pub total_requests: AtomicU64,
    pub successful_requests: AtomicU64,
    pub failed_requests: AtomicU64,
    pub total_response_time: AtomicU64, // in microseconds
    pub min_response_time: AtomicU64,
    pub max_response_time: AtomicU64,
    pub total_bytes_sent: AtomicU64,
    pub total_bytes_received: AtomicU64,
}

impl PerformanceMetrics {
    pub fn record_request(&self, response_time: Duration, bytes_sent: u64, bytes_received: u64, success: bool) {
        let response_time_us = response_time.as_micros() as u64;
        
        self.total_requests.fetch_add(1, Ordering::Relaxed);
        if success {
            self.successful_requests.fetch_add(1, Ordering::Relaxed);
        } else {
            self.failed_requests.fetch_add(1, Ordering::Relaxed);
        }
        
        self.total_response_time.fetch_add(response_time_us, Ordering::Relaxed);
        self.total_bytes_sent.fetch_add(bytes_sent, Ordering::Relaxed);
        self.total_bytes_received.fetch_add(bytes_received, Ordering::Relaxed);
        
        // Update min/max response times
        let current_min = self.min_response_time.load(Ordering::Relaxed);
        if current_min == 0 || response_time_us < current_min {
            self.min_response_time.store(response_time_us, Ordering::Relaxed);
        }
        
        let current_max = self.max_response_time.load(Ordering::Relaxed);
        if response_time_us > current_max {
            self.max_response_time.store(response_time_us, Ordering::Relaxed);
        }
    }
    
    pub fn get_average_response_time(&self) -> Duration {
        let total = self.total_response_time.load(Ordering::Relaxed);
        let count = self.total_requests.load(Ordering::Relaxed);
        
        if count > 0 {
            Duration::from_micros(total / count)
        } else {
            Duration::from_micros(0)
        }
    }
    
    pub fn get_requests_per_second(&self, duration: Duration) -> f64 {
        let total = self.total_requests.load(Ordering::Relaxed) as f64;
        total / duration.as_secs_f64()
    }
    
    pub fn get_success_rate(&self) -> f64 {
        let total = self.total_requests.load(Ordering::Relaxed) as f64;
        let successful = self.successful_requests.load(Ordering::Relaxed) as f64;
        
        if total > 0.0 {
            successful / total
        } else {
            0.0
        }
    }
}

#[test]
fn test_server_creation_performance() {
    let start = Instant::now();
    
    // Test creating multiple servers quickly
    let mut servers = Vec::new();
    for i in 0..10 {
        let config = create_performance_config(8000 + i);
        let router = create_performance_router();
        let middleware = MiddlewareChain::new();
        
        let server = HttpServer::new(config, router, middleware);
        assert!(server.is_ok());
        servers.push(server.unwrap());
    }
    
    let creation_time = start.elapsed();
    println!("Created 10 servers in {:?}", creation_time);
    
    // Should be able to create servers quickly
    assert!(creation_time < Duration::from_millis(100));
}

#[test]
fn test_router_performance() {
    let router = create_performance_router();
    let start = Instant::now();
    let num_requests = 1000;
    
    // Test routing performance
    for i in 0..num_requests {
        let path = match i % 6 {
            0 => "/fast/small",
            1 => "/fast/medium", 
            2 => "/fast/large",
            3 => "/slow/1ms",
            4 => "/slow/10ms",
            5 => "/slow/100ms",
            _ => "/fast/small",
        };
        
        let mut context = RequestContext::new(HttpMethod::GET, path, "127.0.0.1");
        let result = router.route(&mut context);
        assert!(result.is_ok());
        assert!(result.unwrap().is_some());
    }
    
    let routing_time = start.elapsed();
    let requests_per_second = num_requests as f64 / routing_time.as_secs_f64();
    
    println!("Routed {} requests in {:?} ({:.0} req/s)", 
             num_requests, routing_time, requests_per_second);
    
    // Should be able to route at least 10,000 requests per second
    assert!(requests_per_second > 10_000.0);
}

#[test]
fn test_handler_performance() {
    let handler = FastHandler::new(1024, Duration::from_nanos(0));
    let start = Instant::now();
    let num_requests = 1000;
    
    // Test handler execution performance
    for _ in 0..num_requests {
        let mut context = RequestContext::new(HttpMethod::GET, "/test", "127.0.0.1");
        let result = handler.handle(&mut context);
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert_eq!(response.status(), StatusCode::OK);
        assert_eq!(response.body().unwrap().len(), 1024);
    }
    
    let handling_time = start.elapsed();
    let requests_per_second = num_requests as f64 / handling_time.as_secs_f64();
    
    println!("Handled {} requests in {:?} ({:.0} req/s)", 
             num_requests, handling_time, requests_per_second);
    
    // Should be able to handle at least 50,000 requests per second
    assert!(requests_per_second > 50_000.0);
}

#[test]
fn test_memory_usage_patterns() {
    let router = create_performance_router();
    let initial_memory = get_memory_usage_estimate();
    
    // Create many contexts and responses
    let mut contexts = Vec::new();
    let mut responses = Vec::new();
    
    for i in 0..1000 {
        let mut context = RequestContext::new(
            HttpMethod::GET, 
            "/fast/medium", 
            &format!("127.0.0.1:{}", 12345 + i)
        );
        
        // Add some data to context
        context.add_header("Authorization", "Bearer token123456789");
        context.add_query_param("param1", "value1");
        context.set_body(vec![b'X'; 100]);
        
        let result = router.route(&mut context).unwrap().unwrap();
        
        contexts.push(context);
        responses.push(result);
    }
    
    let peak_memory = get_memory_usage_estimate();
    
    // Clean up
    contexts.clear();
    responses.clear();
    
    let final_memory = get_memory_usage_estimate();
    
    println!("Memory usage: initial={}, peak={}, final={}", 
             initial_memory, peak_memory, final_memory);
    
    // Memory should increase during the test but not leak
    assert!(peak_memory > initial_memory);
    // Allow some memory increase due to allocator behavior
    assert!(final_memory <= initial_memory + (peak_memory - initial_memory) / 2);
}

#[test]
fn test_connection_pool_performance() {
    let pool = ConnectionPool::new(100, Duration::from_secs(60));
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    
    let start = Instant::now();
    let num_connections = 100;
    
    // Add connections
    let mut connections = Vec::new();
    for i in 0..num_connections {
        let connection = Arc::new(Connection::new(i, addr, addr));
        pool.add_connection(connection.clone());
        connections.push(connection);
    }
    
    let add_time = start.elapsed();
    
    // Remove connections
    let remove_start = Instant::now();
    for i in 0..num_connections {
        pool.remove_connection(i);
    }
    let remove_time = remove_start.elapsed();
    
    println!("Connection pool: add {} in {:?}, remove {} in {:?}", 
             num_connections, add_time, num_connections, remove_time);
    
    // Pool operations should be fast
    assert!(add_time < Duration::from_millis(10));
    assert!(remove_time < Duration::from_millis(10));
    assert_eq!(pool.connection_count(), 0);
}

#[test]
fn test_concurrent_routing_performance() {
    let router = Arc::new(create_performance_router());
    let metrics = Arc::new(PerformanceMetrics::default());
    let num_threads = 8;
    let requests_per_thread = 100;
    
    let start = Instant::now();
    
    let handles: Vec<_> = (0..num_threads)
        .map(|thread_id| {
            let router = router.clone();
            let metrics = metrics.clone();
            
            thread::spawn(move || {
                for i in 0..requests_per_thread {
                    let request_start = Instant::now();
                    
                    let path = match (thread_id + i) % 3 {
                        0 => "/fast/small",
                        1 => "/fast/medium",
                        2 => "/fast/large",
                        _ => "/fast/small",
                    };
                    
                    let mut context = RequestContext::new(
                        HttpMethod::GET, 
                        path, 
                        &format!("127.0.0.1:{}", 12345 + thread_id * 1000 + i)
                    );
                    
                    let result = router.route(&mut context);
                    let request_time = request_start.elapsed();
                    
                    let success = result.is_ok() && result.unwrap().is_some();
                    metrics.record_request(request_time, 100, 1024, success);
                }
            })
        })
        .collect();
    
    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let total_time = start.elapsed();
    let total_requests = num_threads * requests_per_thread;
    
    println!("Concurrent performance: {} threads, {} req/thread, {} total req in {:?}",
             num_threads, requests_per_thread, total_requests, total_time);
    println!("Average response time: {:?}", metrics.get_average_response_time());
    println!("Requests per second: {:.0}", metrics.get_requests_per_second(total_time));
    println!("Success rate: {:.2}%", metrics.get_success_rate() * 100.0);
    
    // Performance expectations
    assert!(metrics.get_requests_per_second(total_time) > 1000.0);
    assert!(metrics.get_success_rate() > 0.99);
    assert!(metrics.get_average_response_time() < Duration::from_millis(10));
}

#[test]
fn test_large_response_performance() {
    let handler = FastHandler::new(1024 * 1024, Duration::from_nanos(0)); // 1MB response
    let start = Instant::now();
    let num_requests = 10;
    
    for _ in 0..num_requests {
        let mut context = RequestContext::new(HttpMethod::GET, "/large", "127.0.0.1");
        let result = handler.handle(&mut context);
        assert!(result.is_ok());
        
        let response = result.unwrap();
        assert_eq!(response.body().unwrap().len(), 1024 * 1024);
    }
    
    let handling_time = start.elapsed();
    let throughput_mbps = (num_requests as f64 * 1.0) / handling_time.as_secs_f64();
    
    println!("Large response performance: {} x 1MB responses in {:?} ({:.1} MB/s)",
             num_requests, handling_time, throughput_mbps);
    
    // Should be able to generate at least 100 MB/s
    assert!(throughput_mbps > 100.0);
}

#[test]
fn test_request_processing_pipeline_performance() {
    let router = create_performance_router();
    let middleware = MiddlewareChain::new();
    let start = Instant::now();
    let num_requests = 100;
    
    for i in 0..num_requests {
        let mut context = RequestContext::new(
            HttpMethod::GET, 
            "/fast/small", 
            &format!("127.0.0.1:{}", 12345 + i)
        );
        
        // Simulate middleware processing
        let _middleware_result = middleware.process(&mut context);
        
        // Route request
        let route_result = router.route(&mut context);
        assert!(route_result.is_ok());
        assert!(route_result.unwrap().is_some());
    }
    
    let pipeline_time = start.elapsed();
    let requests_per_second = num_requests as f64 / pipeline_time.as_secs_f64();
    
    println!("Pipeline performance: {} requests in {:?} ({:.0} req/s)",
             num_requests, pipeline_time, requests_per_second);
    
    // Should be able to process at least 5,000 requests per second through pipeline
    assert!(requests_per_second > 5_000.0);
}

#[test]
#[ignore] // Ignored by default as it's a stress test
fn test_stress_concurrent_connections() {
    let config = create_performance_config(0);
    let max_connections = config.server.max_connections;
    let pool = ConnectionPool::new(max_connections, Duration::from_secs(60));
    
    let addr: SocketAddr = "127.0.0.1:8080".parse().unwrap();
    let start = Instant::now();
    
    // Create maximum number of connections
    let mut handles = Vec::new();
    for i in 0..max_connections {
        let pool = &pool;
        let handle = thread::spawn(move || {
            let connection = Arc::new(Connection::new(i as u64, addr, addr));
            pool.add_connection(connection);
            
            // Simulate some work
            thread::sleep(Duration::from_millis(10));
            
            pool.remove_connection(i as u64);
        });
        handles.push(handle);
    }
    
    // Wait for all connections to complete
    for handle in handles {
        handle.join().unwrap();
    }
    
    let stress_time = start.elapsed();
    
    println!("Stress test: {} concurrent connections in {:?}",
             max_connections, stress_time);
    
    assert_eq!(pool.connection_count(), 0);
    assert!(stress_time < Duration::from_secs(30));
}

/// Estimate memory usage (simplified)
fn get_memory_usage_estimate() -> usize {
    // This is a simplified memory estimation
    // In a real implementation, we'd use system APIs to get actual memory usage
    std::mem::size_of::<usize>() * 1000 // Placeholder value
}
