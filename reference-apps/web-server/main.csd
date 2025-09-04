# CURSED Web Server - HTTP Server with Routing and Middleware
# Demonstrates: Network programming, HTTP handling, JSON, concurrent request processing

yeet "vibez"
yeet "networkz"
yeet "stringz"
yeet "jsonz"
yeet "timez"
yeet "concurrenz"
yeet "mathz"

# HTTP request structure
squad HttpRequest {
    method tea
    path tea
    headers map<tea, tea>
    body tea
    query_params map<tea, tea>
}

# HTTP response structure
squad HttpResponse {
    status_code drip
    headers map<tea, tea>
    body tea
}

# Route handler function type
collab RouteHandler {
    slay handle(request HttpRequest) HttpResponse
}

# Simple handler implementation
squad SimpleHandler {
    handler_func slay(HttpRequest) HttpResponse
}

# Implement RouteHandler for SimpleHandler
impl RouteHandler for SimpleHandler {
    slay handle(request HttpRequest) HttpResponse {
        damn self.handler_func(request)
    }
}

# Web server configuration
squad ServerConfig {
    host tea
    port drip
    max_connections drip
    request_timeout drip
}

# Web server state
squad WebServer {
    config ServerConfig
    routes map<tea, RouteHandler>
    middleware []slay(HttpRequest, HttpResponse) HttpResponse
    active_connections drip
    stats ServerStats
}

# Server statistics
squad ServerStats {
    requests_handled drip
    errors_count drip
    start_time drip
    bytes_sent drip
    bytes_received drip
}

# Create new web server
slay new_server(config ServerConfig) WebServer {
    damn WebServer{
        config: config,
        routes: {},
        middleware: [],
        active_connections: 0,
        stats: ServerStats{
            requests_handled: 0,
            errors_count: 0,
            start_time: timez.now(),
            bytes_sent: 0,
            bytes_received: 0
        }
    }
}

# Parse HTTP request
slay parse_request(raw_request tea) HttpRequest {
    sus lines []tea = stringz.split(raw_request, "\r\n")
    ready (len(lines) == 0) {
        damn HttpRequest{method: "GET", path: "/", headers: {}, body: "", query_params: {}}
    }
    
    # Parse request line
    sus request_parts []tea = stringz.split(lines[0], " ")
    ready (len(request_parts) < 2) {
        damn HttpRequest{method: "GET", path: "/", headers: {}, body: "", query_params: {}}
    }
    
    sus method tea = request_parts[0]
    sus path_with_query tea = request_parts[1]
    
    # Split path and query parameters
    sus path tea = path_with_query
    sus query_params map<tea, tea> = {}
    
    ready (stringz.contains(path_with_query, "?")) {
        sus parts []tea = stringz.split(path_with_query, "?")
        path = parts[0]
        ready (len(parts) > 1) {
            sus query_string tea = parts[1]
            sus param_pairs []tea = stringz.split(query_string, "&")
            bestie (pair in param_pairs) {
                sus kv []tea = stringz.split(pair, "=")
                ready (len(kv) == 2) {
                    query_params[kv[0]] = kv[1]
                }
            }
        }
    }
    
    # Parse headers
    sus headers map<tea, tea> = {}
    sus body tea = ""
    sus in_body lit = false
    sus i drip = 1
    
    bestie (i < len(lines)) {
        sus line tea = lines[i]
        
        ready (stringz.len(line) == 0) {
            in_body = based
        } ready (!in_body) {
            ready (stringz.contains(line, ":")) {
                sus header_parts []tea = stringz.split(line, ":")
                ready (len(header_parts) >= 2) {
                    sus key tea = stringz.trim(header_parts[0])
                    sus value tea = stringz.trim(header_parts[1])
                    headers[key] = value
                }
            }
        } otherwise {
            body = body + line + "\n"
        }
        
        i = i + 1
    }
    
    damn HttpRequest{
        method: method,
        path: path,
        headers: headers,
        body: body,
        query_params: query_params
    }
}

# Format HTTP response
slay format_response(response HttpResponse) tea {
    sus status_text tea = sick (response.status_code) {
        when 200 -> "OK"
        when 201 -> "Created"
        when 400 -> "Bad Request"
        when 404 -> "Not Found"
        when 500 -> "Internal Server Error"
        when _ -> "Unknown"
    }
    
    sus result tea = stringz.format("HTTP/1.1 %d %s\r\n", response.status_code, status_text)
    
    # Add headers
    bestie (key, value in response.headers) {
        result = result + stringz.format("%s: %s\r\n", key, value)
    }
    
    # Add content length if not present
    ready (!("Content-Length" in response.headers)) {
        result = result + stringz.format("Content-Length: %d\r\n", stringz.len(response.body))
    }
    
    result = result + "\r\n" + response.body
    damn result
}

# JSON response helper
slay json_response(data tea, status_code drip) HttpResponse {
    damn HttpResponse{
        status_code: status_code,
        headers: {
            "Content-Type": "application/json",
            "Access-Control-Allow-Origin": "*"
        },
        body: data
    }
}

# HTML response helper  
slay html_response(html tea, status_code drip) HttpResponse {
    damn HttpResponse{
        status_code: status_code,
        headers: {
            "Content-Type": "text/html; charset=utf-8"
        },
        body: html
    }
}

# Add route to server
slay add_route(server *WebServer, path tea, handler RouteHandler) {
    server.routes[path] = handler
}

# Add middleware to server
slay add_middleware(server *WebServer, middleware slay(HttpRequest, HttpResponse) HttpResponse) {
    server.middleware = arrayz.append(server.middleware, middleware)
}

# Logging middleware
slay logging_middleware(request HttpRequest, response HttpResponse) HttpResponse {
    sus timestamp tea = timez.format_time(timez.now(), "2006-01-02 15:04:05")
    vibez.spill(stringz.format("[%s] %s %s - %d", timestamp, request.method, request.path, response.status_code))
    damn response
}

# CORS middleware
slay cors_middleware(request HttpRequest, response HttpResponse) HttpResponse {
    response.headers["Access-Control-Allow-Origin"] = "*"
    response.headers["Access-Control-Allow-Methods"] = "GET, POST, PUT, DELETE, OPTIONS"
    response.headers["Access-Control-Allow-Headers"] = "Content-Type, Authorization"
    damn response
}

# Handle client connection
slay handle_connection(server *WebServer, conn networkz.Connection) {
    server.active_connections = server.active_connections + 1
    
    go {
        # Read request
        sus raw_request tea = networkz.read_all(conn, 8192) fam {
            when _ -> {
                server.stats.errors_count = server.stats.errors_count + 1
                networkz.close(conn)
                server.active_connections = server.active_connections - 1
                damn
            }
        }
        
        server.stats.bytes_received = server.stats.bytes_received + stringz.len(raw_request)
        
        # Parse request
        sus request HttpRequest = parse_request(raw_request)
        
        # Find matching route
        sus response HttpResponse = ready (request.path in server.routes) {
            sus handler RouteHandler = server.routes[request.path]
            damn handler.handle(request)
        } otherwise {
            damn HttpResponse{
                status_code: 404,
                headers: {"Content-Type": "text/plain"},
                body: "Not Found"
            }
        }
        
        # Apply middleware
        bestie (middleware in server.middleware) {
            response = middleware(request, response)
        }
        
        # Send response
        sus response_text tea = format_response(response)
        networkz.write(conn, response_text) fam {
            when _ -> server.stats.errors_count = server.stats.errors_count + 1
        }
        
        server.stats.bytes_sent = server.stats.bytes_sent + stringz.len(response_text)
        server.stats.requests_handled = server.stats.requests_handled + 1
        
        networkz.close(conn)
        server.active_connections = server.active_connections - 1
    }
}

# Start the web server
slay start_server(server *WebServer) {
    vibez.spill("Starting CURSED Web Server...")
    vibez.spill("Host:", server.config.host)
    vibez.spill("Port:", server.config.port)
    vibez.spill("Max connections:", server.config.max_connections)
    vibez.spill("")
    
    sus listener networkz.Listener = networkz.listen(server.config.host, server.config.port) fam {
        when _ -> {
            vibez.spill("Failed to start server")
            damn
        }
    }
    
    vibez.spill("Server started successfully!")
    vibez.spill("Visit http://", server.config.host, ":", server.config.port)
    
    bestie (based) {
        sus conn networkz.Connection = networkz.accept(listener) fam {
            when _ -> skip
        }
        
        ready (server.active_connections < server.config.max_connections) {
            handle_connection(server, conn)
        } otherwise {
            # Server overloaded
            sus overload_response tea = format_response(HttpResponse{
                status_code: 503,
                headers: {"Content-Type": "text/plain"},
                body: "Server Overloaded"
            })
            networkz.write(conn, overload_response)
            networkz.close(conn)
        }
    }
}

# Route handlers

# Home page handler
slay home_handler(request HttpRequest) HttpResponse {
    sus html tea = `
<!DOCTYPE html>
<html>
<head>
    <title>CURSED Web Server</title>
    <style>
        body { font-family: Arial, sans-serif; margin: 40px; }
        .header { background: #f4f4f4; padding: 20px; border-radius: 5px; }
        .nav { margin: 20px 0; }
        .nav a { margin-right: 15px; text-decoration: none; color: #007acc; }
    </style>
</head>
<body>
    <div class="header">
        <h1>🚀 CURSED Web Server v1.0</h1>
        <p>High-performance web server built with CURSED programming language</p>
    </div>
    
    <div class="nav">
        <a href="/api/status">Server Status</a>
        <a href="/api/users">Users API</a>
        <a href="/api/health">Health Check</a>
    </div>
    
    <h2>Features</h2>
    <ul>
        <li>Concurrent request handling with goroutines</li>
        <li>JSON API endpoints</li>
        <li>Request logging middleware</li>
        <li>CORS support</li>
        <li>Error handling and recovery</li>
    </ul>
</body>
</html>`
    
    damn html_response(html, 200)
}

# API status handler
slay status_handler(request HttpRequest) HttpResponse {
    sus uptime drip = timez.now() - server.stats.start_time
    
    sus status tea = jsonz.marshal({
        "server": "CURSED Web Server",
        "version": "1.0.0",
        "status": "running",
        "uptime_seconds": uptime,
        "requests_handled": server.stats.requests_handled,
        "errors_count": server.stats.errors_count,
        "active_connections": server.active_connections,
        "bytes_sent": server.stats.bytes_sent,
        "bytes_received": server.stats.bytes_received
    }) fam {
        when _ -> damn "{\"error\": \"serialization failed\"}"
    }
    
    damn json_response(status, 200)
}

# Users API handler
slay users_handler(request HttpRequest) HttpResponse {
    sick (request.method) {
        when "GET" -> {
            sus users tea = jsonz.marshal([
                {"id": 1, "name": "Alice", "email": "alice@example.com"},
                {"id": 2, "name": "Bob", "email": "bob@example.com"},
                {"id": 3, "name": "Charlie", "email": "charlie@example.com"}
            ]) fam {
                when _ -> damn "{\"error\": \"serialization failed\"}"
            }
            damn json_response(users, 200)
        }
        when "POST" -> {
            # Simple user creation
            sus new_user tea = jsonz.marshal({
                "id": mathz.random(1000, 9999),
                "name": "New User",
                "email": "newuser@example.com",
                "created": timez.format_time(timez.now(), "2006-01-02T15:04:05Z")
            })
            damn json_response(new_user, 201)
        }
        when _ -> {
            damn json_response("{\"error\": \"Method not allowed\"}", 405)
        }
    }
}

# Health check handler
slay health_handler(request HttpRequest) HttpResponse {
    sus health tea = jsonz.marshal({
        "status": "healthy",
        "timestamp": timez.now(),
        "checks": {
            "server": "ok",
            "memory": "ok",
            "connections": "ok"
        }
    })
    
    damn json_response(health, 200)
}

# Main application
slay main_character() {
    sus config ServerConfig = {
        host: "127.0.0.1",
        port: 8080,
        max_connections: 100,
        request_timeout: 30
    }
    
    sus server WebServer = new_server(config)
    
    # Add middleware
    add_middleware(&server, logging_middleware)
    add_middleware(&server, cors_middleware)
    
    # Add routes
    add_route(&server, "/", SimpleHandler{handler_func: home_handler})
    add_route(&server, "/api/status", SimpleHandler{handler_func: status_handler})
    add_route(&server, "/api/users", SimpleHandler{handler_func: users_handler})
    add_route(&server, "/api/health", SimpleHandler{handler_func: health_handler})
    
    # Start server
    start_server(&server)
}

# Run the application
main()
