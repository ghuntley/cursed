yeet "vibez"
yeet "web_vibez"

vibez.spill("🌐 CURSED HTTP Client/Server Demo")
vibez.spill("================================")

# Demonstrate HTTP client functionality
vibez.spill("\n📱 HTTP Client Demo:")

sus client HttpClient = web_vibez.create_client()
vibez.spill("✅ Created HTTP client with timeout: " + tea(client.timeout))

sus get_response HttpResponse = web_vibez.http_get("https://api.example.com/data")
vibez.spill("📥 GET Response - Status: " + tea(get_response.status_code) + " " + get_response.status)
vibez.spill("📄 Response body: " + get_response.body)

sus post_response HttpResponse = web_vibez.http_post_json("https://api.example.com/users", '{"name":"Alice","age":30}')
vibez.spill("📤 POST Response - Status: " + tea(post_response.status_code) + " " + post_response.status)

# Demonstrate HTTP server functionality
vibez.spill("\n🖥️ HTTP Server Demo:")

sus server HttpServer = web_vibez.create_server("localhost", 8080)
vibez.spill("✅ Created HTTP server on " + server.addr + ":" + tea(server.port))

web_vibez.handle_get("/", "home_handler")
web_vibez.handle_post("/api/users", "create_user_handler")
web_vibez.handle_get("/api/users/:id", "get_user_handler")
vibez.spill("📋 Registered route handlers")

web_vibez.enable_logging_middleware()
web_vibez.enable_cors_middleware()
web_vibez.enable_compression_middleware()
vibez.spill("🔧 Enabled middleware: logging, CORS, compression")

sus start_success lit = web_vibez.server_start(server)
fr fr start_success {
    vibez.spill("🚀 Server started successfully!")
    sus stop_success lit = web_vibez.server_stop(server)
    fr fr stop_success {
        vibez.spill("🛑 Server stopped successfully!")
    }
}

# Demonstrate HTTP utilities
vibez.spill("\n🛠️ HTTP Utilities Demo:")

sus status_ok tea = web_vibez.status_text(200)
sus status_not_found tea = web_vibez.status_text(404)
vibez.spill("📊 Status codes: " + status_ok + " (200), " + status_not_found + " (404)")

sus json_resp tea = web_vibez.create_json_response("Hello World")
vibez.spill("📄 JSON response: " + json_resp)

sus error_resp tea = web_vibez.create_error_response("Resource not found", 404)
vibez.spill("❌ Error response: " + error_resp)

# Demonstrate security features
vibez.spill("\n🔒 Security Features Demo:")

sus clean_header tea = web_vibez.sanitize_header_value("normal header")
sus malicious_header tea = web_vibez.sanitize_header_value("evil\r\nSet-Cookie: hack=true")
vibez.spill("🛡️ Header sanitization working")
vibez.spill("   Clean: '" + clean_header + "'")
vibez.spill("   Sanitized: '" + malicious_header + "'")

fr fr web_vibez.validate_method("GET") {
    vibez.spill("✅ GET method validation passed")
}

fr fr !web_vibez.validate_method("INVALID") {
    vibez.spill("✅ Invalid method correctly rejected")
}

# Demonstrate performance monitoring
vibez.spill("\n📊 Performance Monitoring Demo:")

web_vibez.record_request(based, 150, 1024, 2048)
web_vibez.record_request(based, 200, 512, 1024)
web_vibez.record_request(cap, 500, 256, 512)

sus metrics HttpMetrics = web_vibez.get_metrics()
vibez.spill("📈 Total requests: " + tea(metrics.total_requests))
vibez.spill("📈 Successful: " + tea(metrics.successful_requests))
vibez.spill("📈 Failed: " + tea(metrics.failed_requests))
vibez.spill("📈 Avg response time: " + tea(metrics.average_response_time) + "ms")
vibez.spill("📈 Bytes sent: " + tea(metrics.bytes_sent))
vibez.spill("📈 Bytes received: " + tea(metrics.bytes_received))

vibez.spill("\n🎉 CURSED HTTP Module Demo Complete!")
vibez.spill("✅ Client functionality: HTTP GET, POST, PUT, DELETE")
vibez.spill("✅ Server functionality: Routes, handlers, middleware")
vibez.spill("✅ Security features: Header sanitization, method validation")
vibez.spill("✅ Utilities: JSON responses, form encoding, status codes")
vibez.spill("✅ Performance monitoring: Request metrics and analytics")
vibez.spill("✅ Pure CURSED implementation - no FFI dependencies")
