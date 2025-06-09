# CURSED web_vibez Quick Start Guide

The `web_vibez` package is your gateway to building slaying HTTP servers and clients in CURSED! 🔥

## Installation

Add `web_vibez` to your CURSED project:

```cursed
yeet "web_vibez"
```

## Hello, World! Server

Let's create your first HTTP server:

```cursed
yeet "web_vibez"
yeet "vibez"

slay main() {
    fr fr Create server configuration
    sus config = web_vibez.ServerConfig{
        host: "127.0.0.1",
        port: 8080,
        max_connections: 100,
        timeout: 30000
    }
    
    fr fr Create server instance
    sus server = web_vibez.create_server(config)
    
    fr fr Add a simple route
    server.add_route("/", slay(request) {
        yolo web_vibez.Response{
            status: 200,
            headers: {"Content-Type": "text/html"},
            body: "<h1>Hello, World! 🔥</h1>"
        }
    })
    
    fr fr Start the server
    vibez.spill("Server starting on http://127.0.0.1:8080")
    server.listen_and_serve()
}
```

Run your server:
```bash
cursed run hello_server.csd
```

Visit `http://127.0.0.1:8080` in your browser to see your slaying server! ✨

## HTTP Client Requests

Making HTTP requests is straightforward:

```cursed
yeet "web_vibez"
yeet "vibez"

slay main() {
    fr fr Set client timeout (optional)
    web_vibez.client_timeout(5000) fr fr 5 seconds
    
    fr fr Make a GET request
    sus response = web_vibez.get("https://api.example.com/data", facts)
    vibez.spill("Status: " + response.status.to_string())
    vibez.spill("Body: " + response.body)
    
    fr fr Make a POST request
    sus post_data = {
        "name": "CURSED User",
        "message": "This API is slaying!"
    }
    
    sus post_response = web_vibez.post("https://api.example.com/submit", post_data, facts)
    vibez.spill("Created: " + post_response.body)
}
```

## JSON API Server

Building a JSON API is easy:

```cursed
yeet "web_vibez"
yeet "json_tea"
yeet "vibez"

squad User {
    id: numo,
    name: tea,
    email: tea
}

sus users = []User{}

slay main() {
    sus server = web_vibez.create_server(web_vibez.ServerConfig{
        host: "0.0.0.0",
        port: 3000,
        max_connections: 1000,
        timeout: 60000
    })
    
    fr fr Add CORS middleware
    server.add_middleware(web_vibez.cors_middleware())
    
    fr fr GET /api/users - List users
    server.add_route("/api/users", slay(request) {
        vibe_check request.method {
            mood "GET": {
                yolo web_vibez.Response{
                    status: 200,
                    headers: {"Content-Type": "application/json"},
                    body: json_tea.encode(users)
                }
            }
            mood "POST": {
                sus user_data = json_tea.decode(request.body)
                sus new_user = User{
                    id: users.len() + 1,
                    name: user_data.name,
                    email: user_data.email
                }
                users.append(new_user)
                
                yolo web_vibez.Response{
                    status: 201,
                    headers: {"Content-Type": "application/json"},
                    body: json_tea.encode(new_user)
                }
            }
            basic: {
                yolo web_vibez.Response{
                    status: 405,
                    headers: {"Content-Type": "application/json"},
                    body: '{"error": "Method not allowed"}'
                }
            }
        }
    })
    
    vibez.spill("API server starting on http://0.0.0.0:3000")
    server.listen_and_serve()
}
```

## Static File Server

Serve static files with style:

```cursed
yeet "web_vibez"
yeet "vibez"

slay main() {
    sus server = web_vibez.create_server(web_vibez.ServerConfig{
        host: "127.0.0.1",
        port: 8000,
        max_connections: 200,
        timeout: 30000
    })
    
    fr fr Add security middleware
    server.add_middleware(slay(request) {
        lowkey request.url.contains("..") {
            yolo web_vibez.Response{
                status: 403,
                headers: {},
                body: "Access denied"
            }
        }
        yolo cap fr fr Continue
    })
    
    fr fr Serve static files
    server.add_route("/*", web_vibez.static_file_handler("./public"))
    
    vibez.spill("Static server starting on http://127.0.0.1:8000")
    server.listen_and_serve()
}
```

## Authentication & Middleware

Add authentication to your API:

```cursed
yeet "web_vibez"
yeet "cryptz"
yeet "vibez"

fr fr Authentication middleware
slay auth_middleware() {
    yolo slay(request) {
        fr fr Skip auth for login endpoint
        lowkey request.url == "/api/login" {
            yolo cap
        }
        
        sus auth_header = request.headers.get("Authorization")
        lowkey auth_header == cap || !auth_header.starts_with("Bearer ") {
            yolo web_vibez.Response{
                status: 401,
                headers: {"Content-Type": "application/json"},
                body: '{"error": "Unauthorized"}'
            }
        }
        
        sus token = auth_header.substring(7)
        lowkey !cryptz.verify_jwt_token(token) {
            yolo web_vibez.Response{
                status: 401,
                headers: {"Content-Type": "application/json"},
                body: '{"error": "Invalid token"}'
            }
        }
        
        yolo cap fr fr Continue
    }
}

slay main() {
    sus server = web_vibez.create_server(web_vibez.ServerConfig{
        host: "0.0.0.0",
        port: 4000,
        max_connections: 500,
        timeout: 60000
    })
    
    fr fr Add global middleware
    server.add_middleware(web_vibez.cors_middleware())
    server.add_middleware(web_vibez.logging_middleware())
    
    fr fr Public login endpoint
    server.add_route("/api/login", slay(request) {
        fr fr Handle login logic
        sus token = cryptz.generate_jwt_token("user123", "user")
        yolo web_vibez.Response{
            status: 200,
            headers: {"Content-Type": "application/json"},
            body: json_tea.encode({"token": token})
        }
    })
    
    fr fr Add auth middleware for protected routes
    server.add_middleware(auth_middleware())
    
    fr fr Protected endpoint
    server.add_route("/api/profile", slay(request) {
        yolo web_vibez.Response{
            status: 200,
            headers: {"Content-Type": "application/json"},
            body: '{"user": "Protected data"}'
        }
    })
    
    vibez.spill("Auth API starting on http://0.0.0.0:4000")
    server.listen_and_serve()
}
```

## Configuration Options

### ServerConfig

```cursed
sus config = web_vibez.ServerConfig{
    host: "0.0.0.0",           fr fr Bind address
    port: 8080,                fr fr Port number
    max_connections: 1000,     fr fr Max concurrent connections
    timeout: 30000             fr fr Request timeout in milliseconds
}
```

### Client Settings

```cursed
fr fr Set global client timeout
web_vibez.client_timeout(10000) fr fr 10 seconds

fr fr Get current timeout
sus current_timeout = web_vibez.client_timeout()
vibez.spill("Current timeout: " + current_timeout.to_string() + "ms")
```

## Testing Your Server

Use the built-in test utilities:

```cursed
yeet "web_vibez_test_utils"

slay test_my_api() {
    fr fr Create test server
    sus server = TestServerBuilder.new()
        .with_port(0) fr fr Random port
        .with_json_route("/api/test", '{"message": "success"}')
        .build()
    
    fr fr Create test request
    sus request = TestRequestBuilder.get("/api/test")
        .with_header("Accept", "application/json")
        .build()
    
    fr fr Make request and assert response
    sus response = server.handle_request(request)
    ResponseAssertions.new(response)
        .assert_ok()
        .assert_content_type("application/json")
        .assert_body_contains("success")
}
```

## Common Patterns

### Error Handling

```cursed
server.add_route("/api/users", slay(request) {
    yolo vibe_check {
        sus result = process_user_request(request)
        web_vibez.Response{
            status: 200,
            headers: {"Content-Type": "application/json"},
            body: json_tea.encode(result)
        }
    } catch err {
        vibez.spill("Error: " + err.to_string())
        web_vibez.Response{
            status: 500,
            headers: {"Content-Type": "application/json"},
            body: '{"error": "Internal server error"}'
        }
    }
})
```

### Request Validation

```cursed
slay validate_user_data(data) {
    lowkey data.name == cap || data.name.trim().is_empty() {
        throw "Name is required"
    }
    lowkey data.email == cap || !data.email.contains("@") {
        throw "Valid email is required"
    }
}

server.add_route("/api/users", slay(request) {
    yolo vibe_check {
        sus user_data = json_tea.decode(request.body)
        validate_user_data(user_data)
        
        fr fr Process valid data
        sus new_user = create_user(user_data)
        web_vibez.Response{
            status: 201,
            headers: {"Content-Type": "application/json"},
            body: json_tea.encode(new_user)
        }
    } catch validation_error {
        web_vibez.Response{
            status: 400,
            headers: {"Content-Type": "application/json"},
            body: json_tea.encode({"error": validation_error})
        }
    }
})
```

### Rate Limiting

```cursed
slay rate_limit_middleware(max_requests: numo, window_seconds: numo) {
    sus request_counts = {}
    
    yolo slay(request) {
        sus client_ip = request.headers.get("X-Real-IP") || "127.0.0.1"
        sus now = time_utils.unix_timestamp()
        
        fr fr Check rate limit
        lowkey check_rate_limit(client_ip, now, max_requests, window_seconds) {
            yolo web_vibez.Response{
                status: 429,
                headers: {"Content-Type": "application/json"},
                body: '{"error": "Rate limit exceeded"}'
            }
        }
        
        yolo cap fr fr Continue
    }
}

fr fr Apply rate limiting
server.add_middleware(rate_limit_middleware(100, 3600)) fr fr 100 requests per hour
```

## Next Steps

- 📖 Check out the [complete examples](../examples/) directory
- 🔧 Read the [API reference](./web_vibez_api_reference.md)
- 🧪 Explore [testing patterns](./web_vibez_testing_guide.md)
- 🚀 Learn about [performance optimization](./web_vibez_performance_guide.md)
- 🔒 Dive into [security best practices](./web_vibez_security_guide.md)

Happy coding! Your CURSED web server is about to be absolutely slaying! 🔥✨
