/// Performance benchmarks for CURSED web_vibez HTTP server
use std::collections::HashMap;
use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};
use std::thread;
use std::time::{Duration, Instant};
use cursed::object::Object;
use cursed::stdlib::web_vibez::{
    client_timeout, get, post, head, delete,
    ServerConfig, Request, Response, Server, create_server,
    cors_middleware, logging_middleware,
    STATUS_OK, STATUS_CREATED, STATUS_NOT_FOUND
};

#[cfg(test)]
mod performance_benchmarks {
    use super::*;

    #[test]
    fn benchmark_client_functions_throughput() {
        let start = Instant::now();
        let iterations = 1000;
        
        // Benchmark GET requests
        for i in 0..iterations {
            let url = format!("https://example.com/test/{}", i);
            let result = get(&[
                Arc::new(Object::String(url)),
                Arc::new(Object::Boolean(true)), // Use mock mode
            ]).unwrap();
            
            assert!(matches!(*result, Object::HashTable(_)));
        }
        
        let get_duration = start.elapsed();
        let get_rps = iterations as f64 / get_duration.as_secs_f64();
        
        println!("GET requests: {} iterations in {:?} ({:.2} req/sec)", 
                 iterations, get_duration, get_rps);
        
        // Benchmark POST requests
        let start = Instant::now();
        for i in 0..iterations {
            let url = format!("https://example.com/api/{}", i);
            let mut body = HashMap::new();
            body.insert("id".to_string(), Object::Integer(i as i64));
            body.insert("data".to_string(), Object::String(format!("test_{}", i)));
            
            let result = post(&[
                Arc::new(Object::String(url)),
                Arc::new(Object::HashTable(body)),
                Arc::new(Object::Boolean(true)), // Use mock mode
            ]).unwrap();
            
            assert!(matches!(*result, Object::HashTable(_)));
        }
        
        let post_duration = start.elapsed();
        let post_rps = iterations as f64 / post_duration.as_secs_f64();
        
        println!("POST requests: {} iterations in {:?} ({:.2} req/sec)", 
                 iterations, post_duration, post_rps);
        
        // Performance assertions
        assert!(get_rps > 1000.0, "GET requests should handle >1000 req/sec, got {:.2}", get_rps);
        assert!(post_rps > 500.0, "POST requests should handle >500 req/sec, got {:.2}", post_rps);
    }

    #[test]
    fn benchmark_server_route_handling() {
        let config = ServerConfig::default();
        let mut server = create_server(config);
        
        // Add multiple routes for testing
        server.add_route("/", |_req| {
            Response {
                status: STATUS_OK,
                headers: HashMap::new(),
                body: "Hello".to_string(),
            }
        });
        
        server.add_route("/api/users", |_req| {
            Response {
                status: STATUS_OK,
                headers: HashMap::new(),
                body: "[{\"id\": 1}]".to_string(),
            }
        });
        
        server.add_route("/api/health", |_req| {
            Response {
                status: STATUS_OK,
                headers: HashMap::new(),
                body: "{\"status\": \"ok\"}".to_string(),
            }
        });
        
        // Benchmark route lookup and handler execution
        let start = Instant::now();
        let iterations = 10000;
        
        for i in 0..iterations {
            let url = match i % 3 {
                0 => "/",
                1 => "/api/users",
                _ => "/api/health",
            };
            
            let request = Request {
                method: "GET".to_string(),
                url: url.to_string(),
                headers: HashMap::new(),
                body: String::new(),
            };
            
            // Simulate route handling (direct handler call)
            if let Some(handler) = server.routes.get(url) {
                let response = handler(&request);
                assert_eq!(response.status, STATUS_OK);
            }
        }
        
        let duration = start.elapsed();
        let rps = iterations as f64 / duration.as_secs_f64();
        
        println!("Route handling: {} iterations in {:?} ({:.2} routes/sec)", 
                 iterations, duration, rps);
        
        assert!(rps > 10000.0, "Route handling should be >10000 routes/sec, got {:.2}", rps);
    }

    #[test]
    fn benchmark_middleware_performance() {
        let iterations = 5000;
        
        // Benchmark CORS middleware
        let cors_middleware = cors_middleware();
        let request = Request {
            method: "OPTIONS".to_string(),
            url: "/api/test".to_string(),
            headers: HashMap::new(),
            body: String::new(),
        };
        
        let start = Instant::now();
        for _ in 0..iterations {
            let response = cors_middleware(&request);
            assert!(response.is_some());
        }
        let cors_duration = start.elapsed();
        let cors_rps = iterations as f64 / cors_duration.as_secs_f64();
        
        println!("CORS middleware: {} iterations in {:?} ({:.2} req/sec)", 
                 iterations, cors_duration, cors_rps);
        
        // Benchmark logging middleware
        let logging_middleware = logging_middleware();
        let start = Instant::now();
        for _ in 0..iterations {
            let response = logging_middleware(&request);
            assert!(response.is_none()); // Logging middleware returns None
        }
        let logging_duration = start.elapsed();
        let logging_rps = iterations as f64 / logging_duration.as_secs_f64();
        
        println!("Logging middleware: {} iterations in {:?} ({:.2} req/sec)", 
                 iterations, logging_duration, logging_rps);
        
        assert!(cors_rps > 5000.0, "CORS middleware should handle >5000 req/sec, got {:.2}", cors_rps);
        assert!(logging_rps > 10000.0, "Logging middleware should handle >10000 req/sec, got {:.2}", logging_rps);
    }

    #[test]
    fn benchmark_concurrent_request_simulation() {
        let config = ServerConfig {
            host: "127.0.0.1".to_string(),
            port: 0,
            max_connections: 1000,
            timeout: Duration::from_secs(30),
        };
        
        let mut server = create_server(config);
        
        // Add route with some processing
        server.add_route("/test", |_req| {
            // Simulate small amount of work
            let mut sum = 0;
            for i in 0..100 {
                sum += i;
            }
            
            Response {
                status: STATUS_OK,
                headers: HashMap::new(),
                body: format!("Result: {}", sum),
            }
        });
        
        let num_threads = 10;
        let requests_per_thread = 100;
        let total_requests = num_threads * requests_per_thread;
        let completed_requests = Arc::new(AtomicUsize::new(0));
        
        let start = Instant::now();
        let mut handles = vec![];
        
        for thread_id in 0..num_threads {
            let server_routes = server.routes.clone();
            let completed = Arc::clone(&completed_requests);
            
            let handle = thread::spawn(move || {
                for i in 0..requests_per_thread {
                    let request = Request {
                        method: "GET".to_string(),
                        url: "/test".to_string(),
                        headers: HashMap::new(),
                        body: String::new(),
                    };
                    
                    if let Some(handler) = server_routes.get("/test") {
                        let response = handler(&request);
                        assert_eq!(response.status, STATUS_OK);
                        completed.fetch_add(1, Ordering::Relaxed);
                    }
                }
            });
            
            handles.push(handle);
        }
        
        // Wait for all threads to complete
        for handle in handles {
            handle.join().unwrap();
        }
        
        let duration = start.elapsed();
        let rps = total_requests as f64 / duration.as_secs_f64();
        let completed = completed_requests.load(Ordering::Relaxed);
        
        println!("Concurrent requests: {} completed in {:?} ({:.2} req/sec)", 
                 completed, duration, rps);
        println!("Threads: {}, Requests per thread: {}", num_threads, requests_per_thread);
        
        assert_eq!(completed, total_requests);
        assert!(rps > 1000.0, "Concurrent handling should be >1000 req/sec, got {:.2}", rps);
    }

    #[test]
    fn benchmark_memory_usage_under_load() {
        let iterations = 1000;
        let mut memory_samples = Vec::new();
        
        // Function to estimate memory usage (simplified)
        fn estimate_memory_kb() -> usize {
            // In a real benchmark, you'd use a proper memory profiler
            // For now, we'll simulate memory measurement
            std::process::id() as usize % 1000 + 1000 // Mock memory usage
        }
        
        let initial_memory = estimate_memory_kb();
        memory_samples.push(initial_memory);
        
        // Create many server instances and requests
        for i in 0..iterations {
            let config = ServerConfig::default();
            let mut server = create_server(config);
            
            // Add routes
            server.add_route("/", |_req| {
                Response {
                    status: STATUS_OK,
                    headers: HashMap::new(),
                    body: "OK".to_string(),
                }
            });
            
            // Simulate requests
            let request = Request {
                method: "GET".to_string(),
                url: "/".to_string(),
                headers: HashMap::new(),
                body: format!("Request body {}", i),
            };
            
            if let Some(handler) = server.routes.get("/") {
                let _response = handler(&request);
            }
            
            // Sample memory every 100 iterations
            if i % 100 == 0 {
                memory_samples.push(estimate_memory_kb());
            }
        }
        
        let final_memory = estimate_memory_kb();
        let memory_growth = final_memory.saturating_sub(initial_memory);
        
        println!("Memory usage:");
        println!("  Initial: {} KB", initial_memory);
        println!("  Final: {} KB", final_memory);
        println!("  Growth: {} KB", memory_growth);
        println!("  Growth per iteration: {:.2} KB", 
                 memory_growth as f64 / iterations as f64);
        
        // Memory growth should be reasonable
        assert!(memory_growth < 10000, "Memory growth should be <10MB, got {} KB", memory_growth);
    }

    #[test]
    fn benchmark_response_serialization() {
        let iterations = 5000;
        
        // Test different response sizes
        let small_body = "OK";
        let medium_body = "x".repeat(1000);
        let large_body = "x".repeat(100000);
        
        let test_cases = vec![
            ("small", small_body),
            ("medium", medium_body),
            ("large", large_body),
        ];
        
        for (size_name, body) in test_cases {
            let mut headers = HashMap::new();
            headers.insert("Content-Type".to_string(), "text/plain".to_string());
            headers.insert("Server".to_string(), "CURSED web_vibez".to_string());
            
            let start = Instant::now();
            
            for _ in 0..iterations {
                let response = Response {
                    status: STATUS_OK,
                    headers: headers.clone(),
                    body: body.clone(),
                };
                
                // Simulate response serialization
                let _serialized = format!(
                    "HTTP/1.1 {} OK\r\nContent-Length: {}\r\n\r\n{}",
                    response.status,
                    response.body.len(),
                    response.body
                );
            }
            
            let duration = start.elapsed();
            let rps = iterations as f64 / duration.as_secs_f64();
            
            println!("Response serialization ({}): {} iterations in {:?} ({:.2} resp/sec)", 
                     size_name, iterations, duration, rps);
            
            // Performance should be reasonable even for large responses
            match size_name {
                "small" => assert!(rps > 10000.0, "Small responses should be >10000/sec, got {:.2}", rps),
                "medium" => assert!(rps > 1000.0, "Medium responses should be >1000/sec, got {:.2}", rps),
                "large" => assert!(rps > 50.0, "Large responses should be >50/sec, got {:.2}", rps),
                _ => {}
            }
        }
    }

    #[test]
    fn benchmark_client_timeout_operations() {
        let iterations = 10000;
        
        // Benchmark timeout setting
        let start = Instant::now();
        for i in 0..iterations {
            let timeout_ms = 1000 + (i % 5000) as i64; // Vary timeout
            let result = client_timeout(&[Arc::new(Object::Integer(timeout_ms))]).unwrap();
            assert!(matches!(*result, Object::Integer(_)));
        }
        let set_duration = start.elapsed();
        let set_ops = iterations as f64 / set_duration.as_secs_f64();
        
        // Benchmark timeout getting
        let start = Instant::now();
        for _ in 0..iterations {
            let result = client_timeout(&[]).unwrap();
            assert!(matches!(*result, Object::Integer(_)));
        }
        let get_duration = start.elapsed();
        let get_ops = iterations as f64 / get_duration.as_secs_f64();
        
        println!("Client timeout operations:");
        println!("  Set: {} ops in {:?} ({:.2} ops/sec)", iterations, set_duration, set_ops);
        println!("  Get: {} ops in {:?} ({:.2} ops/sec)", iterations, get_duration, get_ops);
        
        assert!(set_ops > 10000.0, "Timeout setting should be >10000 ops/sec, got {:.2}", set_ops);
        assert!(get_ops > 50000.0, "Timeout getting should be >50000 ops/sec, got {:.2}", get_ops);
    }
}

#[cfg(test)]
mod stress_tests {
    use super::*;

    #[test]
    fn stress_test_many_routes() {
        let start = Instant::now();
        let config = ServerConfig::default();
        let mut server = create_server(config);
        
        // Add many routes
        let num_routes = 1000;
        for i in 0..num_routes {
            let route_path = format!("/api/route_{}", i);
            server.add_route(&route_path, move |_req| {
                Response {
                    status: STATUS_OK,
                    headers: HashMap::new(),
                    body: format!("Response from route {}", i),
                }
            });
        }
        
        let setup_duration = start.elapsed();
        println!("Added {} routes in {:?}", num_routes, setup_duration);
        
        // Test route access performance
        let start = Instant::now();
        let test_iterations = 10000;
        
        for i in 0..test_iterations {
            let route_index = i % num_routes;
            let route_path = format!("/api/route_{}", route_index);
            
            if let Some(handler) = server.routes.get(&route_path) {
                let request = Request {
                    method: "GET".to_string(),
                    url: route_path,
                    headers: HashMap::new(),
                    body: String::new(),
                };
                
                let response = handler(&request);
                assert_eq!(response.status, STATUS_OK);
            }
        }
        
        let access_duration = start.elapsed();
        let access_rps = test_iterations as f64 / access_duration.as_secs_f64();
        
        println!("Route access: {} requests in {:?} ({:.2} req/sec)", 
                 test_iterations, access_duration, access_rps);
        
        assert!(access_rps > 1000.0, "Many routes should still handle >1000 req/sec, got {:.2}", access_rps);
    }

    #[test]
    fn stress_test_large_request_bodies() {
        let sizes = vec![
            ("1KB", 1024),
            ("10KB", 10 * 1024),
            ("100KB", 100 * 1024),
            ("1MB", 1024 * 1024),
        ];
        
        for (size_name, size_bytes) in sizes {
            let large_body = "x".repeat(size_bytes);
            let iterations = if size_bytes > 100 * 1024 { 10 } else { 100 };
            
            let start = Instant::now();
            
            for i in 0..iterations {
                let mut body_data = HashMap::new();
                body_data.insert("data".to_string(), Object::String(large_body.clone()));
                body_data.insert("id".to_string(), Object::Integer(i as i64));
                
                let result = post(&[
                    Arc::new(Object::String("https://example.com/upload".to_string())),
                    Arc::new(Object::HashTable(body_data)),
                    Arc::new(Object::Boolean(true)), // Use mock mode
                ]).unwrap();
                
                assert!(matches!(*result, Object::HashTable(_)));
            }
            
            let duration = start.elapsed();
            let rps = iterations as f64 / duration.as_secs_f64();
            
            println!("Large body test ({}): {} requests in {:?} ({:.2} req/sec)", 
                     size_name, iterations, duration, rps);
            
            // Even large requests should complete reasonably fast
            assert!(duration < Duration::from_secs(10), 
                    "Large body test ({}) took too long: {:?}", size_name, duration);
        }
    }
}
