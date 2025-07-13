yeet "testz"

# Simple Web Framework Test
test_start("Simple Web Framework Test")

# Test basic constants
assert_eq_int(HTTP_GET, 1)
assert_eq_int(HTTP_POST, 2)
assert_eq_int(HTTP_OK, 200)
assert_eq_int(HTTP_NOT_FOUND, 404)

# Test server creation
sus server_id normie = web_server_create(8080)
assert_eq_int(server_id, 1)

sus server_start lit = web_server_start(server_id)
assert_true(server_start)

# Test routing
sus route_add lit = web_route_add(server_id, HTTP_GET, "/test", "test_handler")
assert_true(route_add)

sus matched_handler tea = web_route_match(server_id, HTTP_GET, "/test")
assert_eq_string(matched_handler, "default_handler")

# Test request/response
sus request_id normie = web_request_create(HTTP_GET, "/test", "{}", "")
assert_eq_int(request_id, 1)

sus response_id normie = web_response_create(HTTP_OK, "{}", "Hello World")
assert_eq_int(response_id, 1)

# Test session management
sus session_created lit = web_session_create("session123")
assert_true(session_created)

sus session_value tea = web_session_get("session123", "key1")
assert_eq_string(session_value, "session_value")

# Test WebSocket upgrade
sus ws_upgrade lit = web_websocket_upgrade(request_id, response_id)
assert_true(ws_upgrade)

sus ws_send lit = web_websocket_send(1, "Hello WebSocket")
assert_true(ws_send)

sus ws_message tea = web_websocket_receive(1)
assert_eq_string(ws_message, "websocket_message")

print_test_summary()

vibez.spill("✅ Simple Web Framework test completed successfully!")
