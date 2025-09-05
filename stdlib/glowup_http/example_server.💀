yeet "glowup_http"

fr fr Example HTTP Server using glowup_http framework

fr fr Create server configuration
sus config ServerConfig
config.host = "localhost"
config.port = 8080
config.max_connections = 100
config.timeout = 30
config.keep_alive = based
config.compression = based

fr fr Initialize the framework
glowup_http_main()

fr fr Create and start server
vibez.spill("Creating HTTP server...")
http_server_create(config)

fr fr Register routes
vibez.spill("Registering routes...")
http_route_get("/", "home_handler")
http_route_get("/api/status", "status_handler")
http_route_get("/api/users", "users_handler")
http_route_post("/api/users", "create_user_handler")
http_route_put("/api/users/:id", "update_user_handler")
http_route_delete("/api/users/:id", "delete_user_handler")

fr fr Test route handling
vibez.spill("Testing route handlers...")
sus home_request HttpRequest = http_request_new(METHOD_GET, "/")
sus home_response HttpResponse = http_handle_request(home_request)
vibez.spill("Home route response: " + home_response.body)

sus status_request HttpRequest = http_request_new(METHOD_GET, "/api/status")
sus status_response HttpResponse = http_handle_request(status_request)
vibez.spill("Status route response: " + status_response.body)

sus health_request HttpRequest = http_request_new(METHOD_GET, "/api/health")
sus health_response HttpResponse = http_handle_request(health_request)
vibez.spill("Health route response: " + health_response.body)

fr fr Test middleware
vibez.spill("Testing middleware...")
sus cors_response HttpResponse = http_middleware_cors(home_request, home_response)
vibez.spill("CORS middleware applied")

http_middleware_logging(home_request)
vibez.spill("Logging middleware applied")

fr fr Test session management
vibez.spill("Testing session management...")
session_create("user_session_123")
sus session_data tea = session_get("user_session_123")
vibez.spill("Session data: " + session_data)

fr fr Test cookies
vibez.spill("Testing cookies...")
sus cookie_header tea = cookie_set("session_id", "abc123xyz")
vibez.spill("Cookie header: " + cookie_header)

fr fr Test template rendering
vibez.spill("Testing template engine...")
sus rendered_content tea = template_render("user_template", "user_data")
vibez.spill("Rendered content: " + rendered_content)

fr fr Start server listener
vibez.spill("Starting server listener...")
http_server_listen("request_handler")

vibez.spill("Server example completed successfully!")
