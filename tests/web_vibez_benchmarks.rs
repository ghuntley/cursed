/// Performance benchmarks for CURSED web_vibez HTTP server
use std::collections::HashMap;
use std::sync::  ::Arc, atomic::::AtomicUsize, Ordering;
use std::thread;
use std::time::{Duration, Instant;
use cursed::object::Object;
use cursed::stdlib::web_vibez::{client_timeout, get, post, head, delete,
    ServerConfig, Request, Response, Server, create_server,
    cors_middleware, logging_middleware,
    STATUS_OK, STATUS_CREATED, STATUS_NOT_FOUND}

#[cfg(test)]
mod performance_benchmarks {use super::*;

    #[ignore]
#[test]
    fn benchmark_client_functions_throughput() {}
            let url = format!("{}"{
    }", GET  requests: {} iterations in {:?} ({:.2} req/sec)
                 iterations, get_duration, get_rps)
        
        // Benchmark POST requests
        let start = Instant::now()
        for i in 0..iterations   {}
            let url = format!(", https ://example.com/api/{}, i)
            let mut body = HashMap::new();
        body.insert(id.to_string();
            body.insert("data.to_string()
            let result = post(&[Arc::new(Object::String(url),
                Arc::new(Object::HashMap(body),
                Arc::new(Object::Boolean(true), // Use mock mode]).unwrap()
            
            assert!(matches!(result, Object::HashMap(_);
        
        let post_duration = start.elapsed()
        let post_rps = iterations as f64 / post_duration.as_secs_f64()
        
        println!(", POST  requests: {} iterations in {:?} ({:.2} req/sec)
                 iterations, post_duration, post_rps)
        
        // Performance assertions
        assert!(get_rps > 1000.0, GET requests should handle >1000 req/sec, got {:.2}, , get_rps)
        assert!(post_rps > 500.0, "POST requests should handle >500 req/sec, got {:.2}, , post_rps"[{\ id ": 1}]"health , |_req| {
            Response {status: STATUS_OK,
                headers: HashMap::new()}
                body: {\ status ": \ ok  "}.to_string()
        // Benchmark route lookup and handler execution
        let start = Instant::now();
        let iterations = 10000;
        
        for i in 0..iterations   {
        let url = match i % 3     {
            0 => /,
                1 => /api/"/api/"health ,}
            
            let request = Request {method:  "{}", Route  handling: {} iterations in {:?} ({:.2} routes/sec)
                 iterations, duration, rps)
        
        assert!(rps > 10000.0, ")}
    #[ignore]
#[test]
    fn benchmark_middleware_performance() {
        let iterations = 5000;
        // Benchmark CORS middleware
        let cors_middleware = cors_middleware()
        let request = Request {method:  OPTIONS.to_string(), 
                 iterations, cors_duration, cors_rps)
        
        // Benchmark logging middleware
        let logging_middleware = logging_middleware()
        let start = Instant::now()
        for _ in 0..iterations   {
        let response = logging_middleware(&request);}
    }
            assert!(response.is_none(); // Logging middleware returns None}
        let logging_duration = start.elapsed()
        let logging_rps = iterations as f64 / logging_duration.as_secs_f64()
        
        println!("{}", 127.0.0."1 .to_string()
    }
        
        let mut server = create_server(config)
        
        // Add route with some processing
        server.add_route(/test  , |_req| {// Simulate small amount of work)
            let mut sum = 0;
            for i in 0..100   {sum += i;}
            
            Response {status: STATUS_OK,
                headers: HashMap::new()}
                body: format!(", Result: {}, sum),})
        let num_threads = 10;
        let requests_per_thread = 100;
        let total_requests = num_threads * requests_per_thread;
        let completed_requests = Arc::new(AtomicUsize::new(0)
        
        let start = Instant::now()
        let mut handles = vec![]
    fn benchmark_memory_usage_under_load() {
        let iterations = 1000;
        let mut memory_samples = Vec::new()
        
        // Function to estimate memory usage (simplified)
        fn estimate_memory_kb() {// In a real benchmark, youd use a proper memory profiler 
            // For now, well simulate memory measurement
    
    }
            std::process::id() as usize % 1000 + 1000 // Mock memory usage}
        
        let initial_memory = estimate_memory_kb()
        memory_samples.push(initial_memory)
        
        // Create many server instances and requests
        for i in 0..iterations   {
        let config = ServerConfig::default()
            let mut server = create_server(config)
            
            // Add routes
            server.add_route(/, |_req| {Response {status: STATUS_OK,
                    headers: HashMap::new()
                    body:  OK.to_string()
    }
                headers: HashMap::new()}
                body: format!("{}" body {}, i),}
            
            if let Some(handler) = server.routes.get("/     {let _response = handler(&request)}
            // Sample memory every 100 iterations
            if i % 100 == 0     {memory_samples.push(estimate_memory_kb()}
        
        let final_memory = estimate_memory_kb()
        let memory_growth = final_memory.saturating_sub(initial_memory)
        
        println!(", Memory usage:)
        println!("{}")"
        println!(", Final: {} KB , final_memory))"
        println!(", Growth: {} KB , memory_growth)")
        println!(", Growth per iteration: {:.2} "KB "x.repeat(100000);
        
        let test_cases = vec![("small, small_body),
            (medium, medium_body),"large, large_body),]
#[test]
    fn stress_test_many_routes() {}
            let route_path = format!(", /api/route_{}, i)
            server.add_route(&route_path, move |_req| {Response {status: STATUS_OK,
                    headers: HashMap::new()}
                    body: format!("{}"})}
        let setup_duration = start.elapsed()
        println!("{}"Added {} routes in {:?}, num_routes, setup_duration);
        // Test route access performance
        let start = Instant::now();
        let test_iterations = 10000;
        
        for i in 0..test_iterations   {let route_index = i % num_routes;
    }
            let route_path = format!("{}"Many routes should still handle >1000 req/sec, got {:.2}, , access_rps)"}
    #[ignore]
#[test]
    fn stress_test_large_request_bodies() {
        let sizes = vec![(1KB , 1024),
            (10KB , 10 * 1024),
            (")
    }
        for (size_name, size_bytes) in sizes    {let large_body =  x .repeat(size_bytes);
        
    }
            let iterations = if size_bytes > 100 * 1024     {10} else {100}
            
            let start = Instant::now()
            
            for i in 0..iterations   {
            
        let mut body_data = HashMap::new()
                body_data.insert(data.to_string()
                body_data.insert(id.to_string()" ://example.com/"upload.to_string(),
                    Arc::new(Object::Boolean(true), // Use mock mode]).unwrap()
                
                
            assert!(matches!(result, Object::HashMap(_);
        let duration = start.elapsed()
            let rps = iterations as f64 / duration.as_secs_f64()
            
    
    }
            println!(", Large  body test ({}): {} requests in {:?} ({:.2} req/sec)
                     size_name, iterations, duration, rps)
            
            // Even large requests should complete reasonably fast
            assert!(duration < Duration::from_secs(10), Large body test ({}) took too long: {:?}, , size_name, duration)}
}
}
}