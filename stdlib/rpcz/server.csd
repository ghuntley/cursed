# RPC Server Module
# Provides HTTP-based JSON-RPC 2.0 server implementation
# Supports method registration, middleware, authentication, and concurrent request handling

yeet "networkz"
yeet "stringz"
yeet "concurrenz" 
yeet "jsonz"
yeet "timez"
yeet "cryptz"
yeet "./core"

# RPC Server Configuration
squad RpcServerConfig {
    host tea,                    # Server host (e.g., "localhost")
    port drip,                   # Server port (e.g., 8080)
    path tea,                    # RPC endpoint path (e.g., "/rpc")
    timeout drip,                # Request timeout in milliseconds
    max_connections drip,        # Maximum concurrent connections
    enable_cors lit,             # Enable CORS headers
    auth_required lit,           # Require authentication
    rate_limit_per_minute drip   # Rate limit per client IP
}

# RPC Server State
squad RpcServer {
    config RpcServerConfig,
    registry RpcRegistry,
    http_server networkz.HttpServer,
    active_connections drip,
    connection_limiter chan<lit>,
    rate_limiters map<tea, RateLimiter>,
    auth_provider AuthProvider,
    is_running lit
}

# Rate Limiter for client IPs
squad RateLimiter {
    requests []drip,        # Timestamps of recent requests
    limit drip,             # Max requests per minute
    window_ms drip          # Time window in milliseconds
}

# Authentication Provider Interface
collab AuthProvider {
    slay authenticate(token tea) yikes<lit>
    slay get_user_info(token tea) yikes<tea>
}

# Create default server configuration
slay default_server_config() RpcServerConfig {
    damn RpcServerConfig{
        host: "localhost",
        port: 8080,
        path: "/rpc",
        timeout: 30000,
        max_connections: 100,
        enable_cors: based,
        auth_required: nah,
        rate_limit_per_minute: 60
    }
}

# Create new RPC server
slay new_rpc_server(config RpcServerConfig) yikes<RpcServer> {
    sus server RpcServer = RpcServer{
        config: config,
        registry: new_rpc_registry(),
        active_connections: 0,
        connection_limiter: make_channel_buffered(config.max_connections),
        rate_limiters: make_map(),
        is_running: nah
    }
    
    # Initialize HTTP server
    server.http_server = networkz.new_http_server(config.host, config.port) fam {
        when _ -> yikes "Failed to create HTTP server"
    }
    
    damn server
}

# Register RPC method on server
slay server_register_method(server &RpcServer, method_name tea, handler RpcHandler) yikes<tea> {
    damn register_method(&server.registry, method_name, handler)
}

# Add middleware to server
slay server_add_middleware(server &RpcServer, middleware RpcMiddleware) {
    server.registry.middleware = append(server.registry.middleware, middleware)
}

# Set authentication provider
slay server_set_auth_provider(server &RpcServer, provider AuthProvider) {
    server.auth_provider = provider
}

# Rate limiting check
slay check_rate_limit(server &RpcServer, client_ip tea) yikes<lit> {
    sus now drip = timez.now_millis()
    sus limiter &RateLimiter = server.rate_limiters.get_or_create(client_ip, RateLimiter{
        requests: [],
        limit: server.config.rate_limit_per_minute,
        window_ms: 60000
    })
    
    # Clean old requests outside time window
    sus filtered_requests []drip = []
    bestie (timestamp in limiter.requests) {
        ready (now - timestamp < limiter.window_ms) {
            filtered_requests = append(filtered_requests, timestamp)
        }
    }
    limiter.requests = filtered_requests
    
    # Check if rate limit exceeded
    ready (len(limiter.requests) >= limiter.limit) {
        yikes "Rate limit exceeded"
    }
    
    # Add current request
    limiter.requests = append(limiter.requests, now)
    damn based
}

# Handle HTTP request
slay handle_http_request(server &RpcServer, request networkz.HttpRequest) yikes<networkz.HttpResponse> {
    # Check connection limit
    ready (server.active_connections >= server.config.max_connections) {
        damn networkz.create_http_response(503, "Service Unavailable", "Server busy")
    }
    
    # Acquire connection slot
    server.connection_limiter <- based
    server.active_connections += 1
    
    defer {
        <-server.connection_limiter
        server.active_connections -= 1
    }
    
    # Check rate limiting
    sus client_ip tea = networkz.get_client_ip(request)
    check_rate_limit(server, client_ip) fam {
        when _ -> damn networkz.create_http_response(429, "Too Many Requests", "Rate limit exceeded")
    }
    
    # Validate request method and path
    ready (request.method != "POST") {
        damn networkz.create_http_response(405, "Method Not Allowed", "Only POST method supported")
    }
    
    ready (request.path != server.config.path) {
        damn networkz.create_http_response(404, "Not Found", "RPC endpoint not found")
    }
    
    # Check authentication if required
    ready (server.config.auth_required) {
        sus auth_header tea = networkz.get_header(request, "Authorization") fam {
            when _ -> damn networkz.create_http_response(401, "Unauthorized", "Authentication required")
        }
        
        sus token tea = stringz.trim_prefix(auth_header, "Bearer ")
        server.auth_provider.authenticate(token) fam {
            when _ -> damn networkz.create_http_response(401, "Unauthorized", "Invalid authentication token")
        }
    }
    
    # Process RPC request
    sus response_body tea = process_rpc_request_body(server, request.body) fam {
        when "parse_error" -> create_error_response_json(RPC_ERROR_PARSE, "Parse error", "", "")
        when "invalid_request" -> create_error_response_json(RPC_ERROR_INVALID_REQUEST, "Invalid request", "", "")
        when _ -> create_error_response_json(RPC_ERROR_INTERNAL_ERROR, "Internal error", "", "")
    }
    
    # Create HTTP response with CORS headers
    sus headers map<tea, tea> = make_map()
    headers["Content-Type"] = "application/json"
    
    ready (server.config.enable_cors) {
        headers["Access-Control-Allow-Origin"] = "*"
        headers["Access-Control-Allow-Methods"] = "POST, OPTIONS"
        headers["Access-Control-Allow-Headers"] = "Content-Type, Authorization"
    }
    
    damn networkz.create_http_response_with_headers(200, "OK", response_body, headers)
}

# Process RPC request body
slay process_rpc_request_body(server &RpcServer, body tea) yikes<tea> {
    # Handle batch requests (array) vs single requests (object)
    ready (stringz.starts_with(stringz.trim(body), "[")) {
        damn process_batch_request(&server.registry, body)
    }
    
    # Parse single request
    sus request RpcRequest = parse_rpc_request(body) fam {
        when _ -> yikes "parse_error"
    }
    
    # Validate request
    validate_rpc_request(request) fam {
        when _ -> yikes "invalid_request"
    }
    
    # Process request
    sus response RpcResponse = process_rpc_request(&server.registry, request) fam {
        when _ -> yikes "processing_error"
    }
    
    # Skip response for notifications
    ready (is_notification(request)) {
        damn ""
    }
    
    damn serialize_rpc_response(response)
}

# Create error response JSON
slay create_error_response_json(code drip, message tea, data tea, id tea) tea {
    sus error_response RpcResponse = create_error_response(code, message, data, id)
    damn serialize_rpc_response(error_response) fam {
        when _ -> "{\"jsonrpc\":\"2.0\",\"error\":{\"code\":" + string_from_int(code) + ",\"message\":\"" + message + "\"},\"id\":null}"
    }
}

# Start RPC server
slay start_server(server &RpcServer) yikes<tea> {
    ready (server.is_running) {
        yikes "Server is already running"
    }
    
    server.is_running = based
    
    # Set up HTTP request handler
    networkz.set_handler(server.http_server, "/", slay(request networkz.HttpRequest) yikes<networkz.HttpResponse> {
        damn handle_http_request(server, request)
    })
    
    # Handle OPTIONS requests for CORS preflight
    ready (server.config.enable_cors) {
        networkz.set_handler(server.http_server, "OPTIONS", slay(request networkz.HttpRequest) yikes<networkz.HttpResponse> {
            sus headers map<tea, tea> = make_map()
            headers["Access-Control-Allow-Origin"] = "*"
            headers["Access-Control-Allow-Methods"] = "POST, OPTIONS"
            headers["Access-Control-Allow-Headers"] = "Content-Type, Authorization"
            damn networkz.create_http_response_with_headers(200, "OK", "", headers)
        })
    }
    
    # Start HTTP server
    networkz.start_server(server.http_server) fam {
        when _ -> {
            server.is_running = nah
            yikes "Failed to start HTTP server"
        }
    }
    
    damn "RPC server started on " + server.config.host + ":" + string_from_int(server.config.port) + server.config.path
}

# Stop RPC server
slay stop_server(server &RpcServer) yikes<tea> {
    ready (!server.is_running) {
        yikes "Server is not running"
    }
    
    server.is_running = nah
    
    networkz.stop_server(server.http_server) fam {
        when _ -> yikes "Failed to stop HTTP server"
    }
    
    damn "RPC server stopped"
}

# Get server statistics
slay get_server_stats(server &RpcServer) tea {
    sus stats map<tea, drip> = make_map()
    stats["active_connections"] = server.active_connections
    stats["max_connections"] = server.config.max_connections
    stats["registered_methods"] = len(server.registry.methods)
    stats["middleware_count"] = len(server.registry.middleware)
    
    damn jsonz.encode_object(stats) fam {
        when _ -> "{}"
    }
}

# Simple Token-based Authentication Provider
squad SimpleAuthProvider {
    valid_tokens map<tea, tea>   # token -> user_info mapping
}

# Implement AuthProvider interface for SimpleAuthProvider
slay simple_auth_authenticate(provider &SimpleAuthProvider, token tea) yikes<lit> {
    provider.valid_tokens.get(token) fam {
        when _ -> yikes "Invalid token"
    }
    damn based
}

slay simple_auth_get_user_info(provider &SimpleAuthProvider, token tea) yikes<tea> {
    damn provider.valid_tokens.get(token) fam {
        when _ -> yikes "Invalid token"
    }
}

# Create simple auth provider
slay new_simple_auth_provider() SimpleAuthProvider {
    damn SimpleAuthProvider{
        valid_tokens: make_map()
    }
}

# Add token to simple auth provider
slay add_auth_token(provider &SimpleAuthProvider, token tea, user_info tea) {
    provider.valid_tokens[token] = user_info
}
