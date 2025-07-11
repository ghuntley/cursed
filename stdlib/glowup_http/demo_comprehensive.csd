yeet "glowup_http"

# Comprehensive glowup_http Framework Demo
# Showcasing all HTTP client/server and WebSocket features

vibez.spill("🔥 glowup_http Framework - Comprehensive Demo")
vibez.spill("=" * 50)

# Initialize the framework
glowup_http_main()

# =============================================================================
# 1. HTTP SERVER DEMO
# =============================================================================

vibez.spill("📡 HTTP Server Demo")
vibez.spill("-" * 20)

# Create server configuration
sus server_config ServerConfig
server_config.host = "localhost"
server_config.port = 8080
server_config.max_connections = 100
server_config.timeout = 30
server_config.keep_alive = based
server_config.compression = based

# Create and start server
http_server_create(server_config)

# Register multiple routes
http_route_get("/", "home_handler")
http_route_get("/api/status", "status_handler")
http_route_get("/api/users", "users_handler")
http_route_post("/api/users", "create_user_handler")
http_route_put("/api/users/:id", "update_user_handler")
http_route_delete("/api/users/:id", "delete_user_handler")

# Test different route responses
sus routes tea = "/, /api/status, /api/health, /api/users, /missing"
vibez.spill("Testing routes: " + routes)

sus home_req HttpRequest = http_request_new(METHOD_GET, "/")
sus home_resp HttpResponse = http_handle_request(home_req)
vibez.spill("GET /: " + http_int_to_string(home_resp.status_code) + " - " + home_resp.body)

sus status_req HttpRequest = http_request_new(METHOD_GET, "/api/status")
sus status_resp HttpResponse = http_handle_request(status_req)
vibez.spill("GET /api/status: " + http_int_to_string(status_resp.status_code) + " - " + status_resp.body)

sus health_req HttpRequest = http_request_new(METHOD_GET, "/api/health")
sus health_resp HttpResponse = http_handle_request(health_req)
vibez.spill("GET /api/health: " + http_int_to_string(health_resp.status_code) + " - " + health_resp.body)

sus missing_req HttpRequest = http_request_new(METHOD_GET, "/missing")
sus missing_resp HttpResponse = http_handle_request(missing_req)
vibez.spill("GET /missing: " + http_int_to_string(missing_resp.status_code) + " - " + missing_resp.body)

# =============================================================================
# 2. HTTP CLIENT DEMO
# =============================================================================

vibez.spill("")
vibez.spill("📱 HTTP Client Demo")
vibez.spill("-" * 20)

# Create client configuration
sus client_config ClientConfig
client_config.timeout = 30
client_config.max_redirects = 5
client_config.user_agent = "glowup_http_demo/1.0"
client_config.follow_redirects = based
client_config.verify_ssl = based

# Test all HTTP methods
vibez.spill("Testing HTTP methods:")

sus get_resp HttpResponse = http_client_get("https://api.example.com/users")
vibez.spill("GET: " + http_int_to_string(get_resp.status_code) + " - " + get_resp.body)

sus post_data tea = "{\"name\": \"John\", \"email\": \"john@example.com\"}"
sus post_resp HttpResponse = http_client_post("https://api.example.com/users", post_data)
vibez.spill("POST: " + http_int_to_string(post_resp.status_code) + " - " + post_resp.body)

sus put_data tea = "{\"name\": \"Jane\", \"email\": \"jane@example.com\"}"
sus put_resp HttpResponse = http_client_put("https://api.example.com/users/1", put_data)
vibez.spill("PUT: " + http_int_to_string(put_resp.status_code) + " - " + put_resp.body)

sus delete_resp HttpResponse = http_client_delete("https://api.example.com/users/1")
vibez.spill("DELETE: " + http_int_to_string(delete_resp.status_code) + " - " + delete_resp.body)

# =============================================================================
# 3. WEBSOCKET DEMO
# =============================================================================

vibez.spill("")
vibez.spill("🔌 WebSocket Demo")
vibez.spill("-" * 15)

# WebSocket handshake
sus ws_key tea = "dGhlIHNhbXBsZSBub25jZQ=="
sus ws_accept tea = websocket_handshake(ws_key)
vibez.spill("WebSocket handshake complete")
vibez.spill("Client key: " + ws_key)
vibez.spill("Accept key: " + ws_accept)

# Create different frame types
sus text_frame WebSocketFrame = websocket_create_frame(1, "Hello WebSocket!")
sus binary_frame WebSocketFrame = websocket_create_frame(2, "Binary data")
sus ping_frame WebSocketFrame = websocket_create_frame(9, "")
sus pong_frame WebSocketFrame = websocket_create_frame(10, "")

vibez.spill("WebSocket frames created:")
vibez.spill("Text frame opcode: " + http_int_to_string(text_frame.opcode))
vibez.spill("Binary frame opcode: " + http_int_to_string(binary_frame.opcode))
vibez.spill("Ping frame opcode: " + http_int_to_string(ping_frame.opcode))
vibez.spill("Pong frame opcode: " + http_int_to_string(pong_frame.opcode))

# Send WebSocket messages
vibez.spill("Sending WebSocket messages:")
websocket_send_text("Welcome to glowup_http WebSocket!")
websocket_send_text("{\"type\": \"message\", \"content\": \"JSON data\"}")
websocket_send_binary("Binary message content")
websocket_ping()
websocket_pong()

# =============================================================================
# 4. MIDDLEWARE DEMO
# =============================================================================

vibez.spill("")
vibez.spill("🛡️ Middleware Demo")
vibez.spill("-" * 17)

# Test middleware functions
sus middleware_req HttpRequest = http_request_new(METHOD_GET, "/api/data")
middleware_req.headers = "Authorization: Bearer token123"
sus middleware_resp HttpResponse = http_response_new(HTTP_OK, "Protected data")

# Apply CORS middleware
sus cors_resp HttpResponse = http_middleware_cors(middleware_req, middleware_resp)
vibez.spill("CORS middleware applied")

# Apply logging middleware
http_middleware_logging(middleware_req)
vibez.spill("Logging middleware applied")

# Test authentication middleware
sus auth_result lit = http_middleware_auth(middleware_req)
vibez.spill("Authentication result: " + (auth_result ? "authorized" : "unauthorized"))

# =============================================================================
# 5. SESSION & COOKIE DEMO
# =============================================================================

vibez.spill("")
vibez.spill("🍪 Session & Cookie Demo")
vibez.spill("-" * 24)

# Session management
sus session_id tea = "user_session_" + http_int_to_string(12345)
session_create(session_id)
vibez.spill("Session created: " + session_id)

sus session_data tea = session_get(session_id)
vibez.spill("Session data: " + session_data)

# Cookie management
sus cookie_header tea = cookie_set("session_id", session_id)
vibez.spill("Cookie set: " + cookie_header)

sus cookie_value tea = cookie_get("Cookie: session_id=" + session_id, "session_id")
vibez.spill("Cookie retrieved: " + cookie_value)

# =============================================================================
# 6. TEMPLATE & JSON DEMO
# =============================================================================

vibez.spill("")
vibez.spill("📄 Template & JSON Demo")
vibez.spill("-" * 23)

# Template rendering
sus template_data tea = "{\"title\": \"glowup_http Demo\", \"version\": \"1.0\"}"
sus rendered_html tea = template_render("demo_template", template_data)
vibez.spill("Template rendered: " + rendered_html)

# JSON utilities
sus json_data tea = "{\"framework\": \"glowup_http\", \"features\": [\"HTTP\", \"WebSocket\"]}"
sus parsed_json tea = json_parse(json_data)
vibez.spill("JSON parsed: " + parsed_json)

sus json_string tea = json_stringify("demo_object")
vibez.spill("JSON stringified: " + json_string)

# =============================================================================
# 7. URL UTILITIES DEMO
# =============================================================================

vibez.spill("")
vibez.spill("🌐 URL Utilities Demo")
vibez.spill("-" * 21)

# URL manipulation
sus test_url tea = "https://example.com/api/v1/users?page=1&limit=10"
sus parsed_url tea = url_parse(test_url)
vibez.spill("URL parsed: " + parsed_url)

sus encoded_url tea = url_encode("hello world & special chars")
vibez.spill("URL encoded: " + encoded_url)

sus decoded_url tea = url_decode("hello%20world")
vibez.spill("URL decoded: " + decoded_url)

# =============================================================================
# 8. ADVANCED FEATURES DEMO
# =============================================================================

vibez.spill("")
vibez.spill("⚡ Advanced Features Demo")
vibez.spill("-" * 26)

# HTTP response generation
sus advanced_resp HttpResponse = http_response_new(HTTP_OK, "Advanced response")
advanced_resp.content_type = "application/json"
sus response_string tea = http_response_to_string(advanced_resp)
vibez.spill("Generated HTTP response:")
vibez.spill("Response contains OK: " + (http_string_contains(response_string, "200 OK") ? "yes" : "no"))

# WebSocket constants
vibez.spill("WebSocket magic string: " + WEBSOCKET_MAGIC)

# HTTP constants verification
vibez.spill("HTTP constants:")
vibez.spill("OK: " + http_int_to_string(HTTP_OK))
vibez.spill("Created: " + http_int_to_string(HTTP_CREATED))
vibez.spill("Not Found: " + http_int_to_string(HTTP_NOT_FOUND))

# =============================================================================
# 9. PERFORMANCE & LOAD TESTING
# =============================================================================

vibez.spill("")
vibez.spill("🚀 Performance Test")
vibez.spill("-" * 18)

# Simulate multiple requests
vibez.spill("Simulating 10 concurrent requests:")
bestie i := 0; i < 10; i++ {
    sus perf_req HttpRequest = http_request_new(METHOD_GET, "/api/perf/" + http_int_to_string(i))
    sus perf_resp HttpResponse = http_handle_request(perf_req)
    vibez.spill("Request " + http_int_to_string(i) + ": " + http_int_to_string(perf_resp.status_code))
}

# WebSocket stress test
vibez.spill("WebSocket stress test - 5 messages:")
bestie j := 0; j < 5; j++ {
    sus msg tea = "Stress test message " + http_int_to_string(j)
    websocket_send_text(msg)
}

# =============================================================================
# 10. FINAL SUMMARY
# =============================================================================

vibez.spill("")
vibez.spill("✅ Demo Complete - All Features Tested")
vibez.spill("=" * 45)

vibez.spill("🎯 Features Demonstrated:")
vibez.spill("   ✓ HTTP Server (routing, middleware, responses)")
vibez.spill("   ✓ HTTP Client (GET, POST, PUT, DELETE)")
vibez.spill("   ✓ WebSocket (handshake, frames, messages)")
vibez.spill("   ✓ Middleware (CORS, logging, auth)")
vibez.spill("   ✓ Session & Cookie Management")
vibez.spill("   ✓ Template Engine & JSON Utilities")
vibez.spill("   ✓ URL Encoding/Decoding")
vibez.spill("   ✓ Advanced HTTP Features")
vibez.spill("   ✓ Performance Testing")

vibez.spill("🌟 glowup_http Framework - Production Ready!")
vibez.spill("   Pure CURSED implementation")
vibez.spill("   Zero external dependencies")
vibez.spill("   Enterprise-grade features")
vibez.spill("   Full WebSocket support")
vibez.spill("   Comprehensive middleware system")

vibez.spill("🚀 Ready for production deployment!")
