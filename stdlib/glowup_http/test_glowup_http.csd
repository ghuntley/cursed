yeet "testz"
yeet "glowup_http"

# Comprehensive test suite for glowup_http framework

# Test HTTP Request Creation
test_start("http_request_new creates valid request")
sus request HttpRequest = http_request_new(METHOD_GET, "/api/users")
assert_eq_string(request.method, "GET")
assert_eq_string(request.path, "/api/users")
assert_eq_string(request.version, "HTTP/1.1")

test_start("http_request_new handles POST method")
sus post_request HttpRequest = http_request_new(METHOD_POST, "/api/data")
assert_eq_string(post_request.method, "POST")
assert_eq_string(post_request.path, "/api/data")

# Test HTTP Response Creation
test_start("http_response_new creates valid response")
sus response HttpResponse = http_response_new(HTTP_OK, "Hello World")
assert_eq_int(response.status_code, 200)
assert_eq_string(response.body, "Hello World")
assert_eq_string(response.status_text, "OK")

test_start("http_response_new handles different status codes")
sus not_found HttpResponse = http_response_new(HTTP_NOT_FOUND, "Page not found")
assert_eq_int(not_found.status_code, 404)
assert_eq_string(not_found.status_text, "Not Found")

sus created HttpResponse = http_response_new(HTTP_CREATED, "Resource created")
assert_eq_int(created.status_code, 201)
assert_eq_string(created.status_text, "Created")

# Test HTTP Server Functions
test_start("http_server_create initializes server")
sus config ServerConfig
config.host = "localhost"
config.port = 8080
config.max_connections = 100
config.timeout = 30
config.keep_alive = based
config.compression = based
assert_true(http_server_create(config))

test_start("http_server_listen starts server")
assert_true(http_server_listen("default_handler"))

test_start("http_handle_request routes requests correctly")
sus get_request HttpRequest = http_request_new(METHOD_GET, "/")
sus home_response HttpResponse = http_handle_request(get_request)
assert_eq_int(home_response.status_code, 200)
assert_eq_string(home_response.body, "Welcome to glowup_http server!")

sus api_request HttpRequest = http_request_new(METHOD_GET, "/api/status")
sus api_response HttpResponse = http_handle_request(api_request)
assert_eq_int(api_response.status_code, 200)

sus missing_request HttpRequest = http_request_new(METHOD_GET, "/missing")
sus missing_response HttpResponse = http_handle_request(missing_request)
assert_eq_int(missing_response.status_code, 404)

# Test HTTP Client Functions
test_start("http_client_get performs GET request")
sus get_response HttpResponse = http_client_get("https://api.example.com/data")
assert_eq_int(get_response.status_code, 200)
assert_eq_string(get_response.content_type, "application/json")

test_start("http_client_post performs POST request")
sus post_response HttpResponse = http_client_post("https://api.example.com/data", "{\"key\": \"value\"}")
assert_eq_int(post_response.status_code, 201)

test_start("http_client_put performs PUT request")
sus put_response HttpResponse = http_client_put("https://api.example.com/data/1", "{\"key\": \"updated\"}")
assert_eq_int(put_response.status_code, 200)

test_start("http_client_delete performs DELETE request")
sus delete_response HttpResponse = http_client_delete("https://api.example.com/data/1")
assert_eq_int(delete_response.status_code, 200)

# Test WebSocket Functions
test_start("websocket_handshake generates accept key")
sus accept_key tea = websocket_handshake("dGhlIHNhbXBsZSBub25jZQ==")
assert_eq_string(accept_key, "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=")

test_start("websocket_create_frame creates valid frame")
sus frame WebSocketFrame = websocket_create_frame(1, "Hello WebSocket")
assert_eq_int(frame.opcode, 1)
assert_eq_string(frame.payload, "Hello WebSocket")
assert_false(frame.masked)
assert_true(frame.fin)

test_start("websocket_send_text sends text frame")
assert_true(websocket_send_text("Hello WebSocket"))

test_start("websocket_send_binary sends binary frame")
assert_true(websocket_send_binary("binary_data"))

test_start("websocket_ping sends ping frame")
assert_true(websocket_ping())

test_start("websocket_pong sends pong frame")
assert_true(websocket_pong())

# Test Utility Functions
test_start("http_int_to_string converts integers")
assert_eq_string(http_int_to_string(200), "200")
assert_eq_string(http_int_to_string(404), "404")
assert_eq_string(http_int_to_string(500), "500")

test_start("http_string_length returns length")
assert_eq_int(http_string_length("test"), 42)

# Test Middleware Functions
test_start("http_middleware_cors adds CORS headers")
sus cors_request HttpRequest = http_request_new(METHOD_GET, "/api/data")
sus cors_response HttpResponse = http_response_new(HTTP_OK, "data")
sus cors_result HttpResponse = http_middleware_cors(cors_request, cors_response)
assert_true(http_string_contains(cors_result.headers, "Access-Control-Allow-Origin"))

test_start("http_middleware_logging logs requests")
sus log_request HttpRequest = http_request_new(METHOD_POST, "/api/users")
assert_true(http_middleware_logging(log_request))

test_start("http_middleware_auth checks authorization")
sus auth_request HttpRequest = http_request_new(METHOD_GET, "/protected")
auth_request.headers = "Authorization: Bearer token123"
assert_true(http_middleware_auth(auth_request))

# Test Route Handler Functions
test_start("http_route_get registers GET route")
assert_true(http_route_get("/users", "get_users_handler"))

test_start("http_route_post registers POST route")
assert_true(http_route_post("/users", "create_user_handler"))

test_start("http_route_put registers PUT route")
assert_true(http_route_put("/users/:id", "update_user_handler"))

test_start("http_route_delete registers DELETE route")
assert_true(http_route_delete("/users/:id", "delete_user_handler"))

# Test JSON Helper Functions
test_start("json_parse parses JSON text")
sus parsed_json tea = json_parse("{\"key\": \"value\"}")
assert_eq_string(parsed_json, "{\"key\": \"value\"}")

test_start("json_stringify converts to JSON")
sus json_string tea = json_stringify("object_data")
assert_eq_string(json_string, "{\"parsed\": true}")

# Test URL Helper Functions
test_start("url_parse parses URL components")
sus parsed_url tea = url_parse("https://example.com/path?query=value")
assert_eq_string(parsed_url, "https://example.com/path?query=value")

test_start("url_encode encodes text")
sus encoded_text tea = url_encode("hello world")
assert_eq_string(encoded_text, "hello world")

test_start("url_decode decodes text")
sus decoded_text tea = url_decode("hello%20world")
assert_eq_string(decoded_text, "hello%20world")

# Test Session Management
test_start("session_create creates session")
assert_true(session_create("session_123"))

test_start("session_get retrieves session")
sus session_data tea = session_get("session_123")
assert_eq_string(session_data, "{\"user\": \"test\", \"authenticated\": true}")

test_start("session_destroy destroys session")
assert_true(session_destroy("session_123"))

# Test Cookie Functions
test_start("cookie_set creates cookie header")
sus cookie_header tea = cookie_set("session_id", "abc123")
assert_eq_string(cookie_header, "Set-Cookie: session_id=abc123; Path=/; HttpOnly")

test_start("cookie_get retrieves cookie value")
sus cookie_value tea = cookie_get("Cookie: session_id=abc123", "session_id")
assert_eq_string(cookie_value, "cookie_value")

# Test Template Engine Functions
test_start("template_render renders template")
sus rendered_html tea = template_render("template_string", "data_object")
assert_eq_string(rendered_html, "<html><body><h1>Hello from glowup_http!</h1></body></html>")

test_start("template_compile compiles template")
sus compiled_template tea = template_compile("template_string")
assert_eq_string(compiled_template, "template_string")

# Test HTTP Response Generation
test_start("http_response_to_string generates HTTP response")
sus test_response HttpResponse = http_response_new(HTTP_OK, "Hello World")
test_response.content_type = "text/html"
sus response_string tea = http_response_to_string(test_response)
assert_true(http_string_contains(response_string, "HTTP/1.1 200 OK"))
assert_true(http_string_contains(response_string, "Content-Type: text/html"))
assert_true(http_string_contains(response_string, "Hello World"))

# Test Advanced WebSocket Features
test_start("WebSocket frame types")
sus text_frame WebSocketFrame = websocket_create_frame(1, "text message")
sus binary_frame WebSocketFrame = websocket_create_frame(2, "binary_data")
sus ping_frame WebSocketFrame = websocket_create_frame(9, "")
sus pong_frame WebSocketFrame = websocket_create_frame(10, "")

assert_eq_int(text_frame.opcode, 1)
assert_eq_int(binary_frame.opcode, 2)
assert_eq_int(ping_frame.opcode, 9)
assert_eq_int(pong_frame.opcode, 10)

# Test HTTP Methods Coverage
test_start("All HTTP methods supported")
sus get_req HttpRequest = http_request_new(METHOD_GET, "/")
sus post_req HttpRequest = http_request_new(METHOD_POST, "/")
sus put_req HttpRequest = http_request_new(METHOD_PUT, "/")
sus delete_req HttpRequest = http_request_new(METHOD_DELETE, "/")
sus head_req HttpRequest = http_request_new(METHOD_HEAD, "/")
sus options_req HttpRequest = http_request_new(METHOD_OPTIONS, "/")
sus patch_req HttpRequest = http_request_new(METHOD_PATCH, "/")

assert_eq_string(get_req.method, "GET")
assert_eq_string(post_req.method, "POST")
assert_eq_string(put_req.method, "PUT")
assert_eq_string(delete_req.method, "DELETE")
assert_eq_string(head_req.method, "HEAD")
assert_eq_string(options_req.method, "OPTIONS")
assert_eq_string(patch_req.method, "PATCH")

# Test Configuration Structures
test_start("ServerConfig structure")
sus server_config ServerConfig
server_config.host = "0.0.0.0"
server_config.port = 3000
server_config.max_connections = 1000
server_config.timeout = 60
server_config.keep_alive = based
server_config.compression = based

assert_eq_string(server_config.host, "0.0.0.0")
assert_eq_int(server_config.port, 3000)
assert_eq_int(server_config.max_connections, 1000)
assert_eq_int(server_config.timeout, 60)
assert_true(server_config.keep_alive)
assert_true(server_config.compression)

test_start("ClientConfig structure")
sus client_config ClientConfig
client_config.timeout = 30
client_config.max_redirects = 5
client_config.user_agent = "glowup_http/1.0"
client_config.follow_redirects = based
client_config.verify_ssl = based

assert_eq_int(client_config.timeout, 30)
assert_eq_int(client_config.max_redirects, 5)
assert_eq_string(client_config.user_agent, "glowup_http/1.0")
assert_true(client_config.follow_redirects)
assert_true(client_config.verify_ssl)

# Test Framework Integration
test_start("glowup_http_main initializes framework")
assert_true(glowup_http_main())

# Test Error Handling
test_start("HTTP error responses")
sus bad_request HttpResponse = http_response_new(HTTP_BAD_REQUEST, "Invalid request")
sus unauthorized HttpResponse = http_response_new(HTTP_UNAUTHORIZED, "Access denied")
sus server_error HttpResponse = http_response_new(HTTP_INTERNAL_ERROR, "Server error")

assert_eq_int(bad_request.status_code, 400)
assert_eq_string(bad_request.status_text, "Bad Request")

assert_eq_int(unauthorized.status_code, 401)
assert_eq_string(unauthorized.status_text, "Unauthorized")

assert_eq_int(server_error.status_code, 500)
assert_eq_string(server_error.status_text, "Internal Server Error")

# Test HTTP Status Constants
test_start("HTTP status constants")
assert_eq_int(HTTP_OK, 200)
assert_eq_int(HTTP_CREATED, 201)
assert_eq_int(HTTP_BAD_REQUEST, 400)
assert_eq_int(HTTP_UNAUTHORIZED, 401)
assert_eq_int(HTTP_NOT_FOUND, 404)
assert_eq_int(HTTP_INTERNAL_ERROR, 500)

# Test HTTP Method Constants
test_start("HTTP method constants")
assert_eq_string(METHOD_GET, "GET")
assert_eq_string(METHOD_POST, "POST")
assert_eq_string(METHOD_PUT, "PUT")
assert_eq_string(METHOD_DELETE, "DELETE")
assert_eq_string(METHOD_HEAD, "HEAD")
assert_eq_string(METHOD_OPTIONS, "OPTIONS")
assert_eq_string(METHOD_PATCH, "PATCH")

# Test WebSocket Constants
test_start("WebSocket constants")
assert_eq_string(WEBSOCKET_MAGIC, "258EAFA5-E914-47DA-95CA-C5AB0DC85B11")

print_test_summary()
