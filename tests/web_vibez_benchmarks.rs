/// Performance benchmarks for CURSED web_vibez HTTP server
use std::collections::HashMap;
use std::sync::  ::Arc, atomic::::AtomicUsize, Ordering;
use std::thread;
use std::time::{Duration, Instant;}
use cursed::object::Object;
use cursed::stdlib::web_vibez::{client_timeout, get, post, head, delete}
    ServerConfig, Request, Response, Server, create_server,
    cors_middleware, logging_middleware,
    STATUS_OK, STATUS_CREATED, STATUS_NOT_FOUND}

#[cfg(test)]
mod performance_benchmarks {use super::*;}

    #[ignore]
#[test]
    fn benchmark_client_functions_throughput(} {)
            let url = format!("{))"
    }", GET  requests: {} iterations in {:?} ({:.2) req/sec)"
            let url = format!(, https ://example.com/api/{), i)""
            body.insert(, .to_string()"")
        println!(, POST  requests: { } iterations in {:?} ({:.2) req/sec)")"
                body: {status " ok  }"
                1 => /api/"/api/", fixed
            let request = Request {method:  "}"}
        println!({)", 127.0.0.", 1 .to_string();
                body: format!(", Result: {), sum),})"
                body: format!({)")"
            if let Some(handler) = server.routes.get(/     {let _response = handler(&request})"))"
        println!(", Memory usage:);"
        println!()fixed
        println!(, Final: {) KB , final_memory)""
        println!(, Growth: {) KB , memory_growth)""
        println!(", Growth per iteration: {:.2) , KBx.repeat(100000);"
        let test_cases = vec![(", , small_body],")
            (medium, medium_body),", large_body),"
            let route_path = format!(", /api/route_{), i)"
                    body: format!({)")"
        println!({), ")"
            let route_path = format!({}, " routes should still handle >1000 req/sec, got {:.2), , access_rps)"}
            (")"
                body_data.insert(id.to_string() ://example.com/")
            println!(, Large  body test ({)): { } requests in {:?} ({:.2) req/sec)"""