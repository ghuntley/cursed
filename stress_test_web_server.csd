fr fr ========================================
fr fr CURSED Web Server Simulator - Comprehensive Stress Test
fr fr Uses: net, json, fs, time, stringz, crypto modules
fr fr ========================================

yeet "stdlib/net"
yeet "stdlib/json" 
yeet "stdlib/fs"
yeet "stdlib/time"
yeet "stdlib/stringz"
yeet "stdlib/crypto"

fr fr Web server configuration
be_like ServerConfig squad {
    host tea
    port normie
    root_dir tea
    enable_auth lit
    log_file tea
}

fr fr Request statistics
be_like RequestStats squad {
    total_requests normie
    successful_requests normie
    failed_requests normie
    bytes_served normie
}

fr fr Authentication token
be_like AuthToken squad {
    token tea
    user_id tea
    expires normie
}

fr fr HTTP request context
be_like RequestContext squad {
    method tea
    path tea
    headers tea
    body tea
    timestamp normie
    auth_token AuthToken
}

sus server_stats RequestStats = RequestStats{
    total_requests: 0,
    successful_requests: 0,
    failed_requests: 0,
    bytes_served: 0
}

sus server_config ServerConfig = ServerConfig{
    host: "127.0.0.1",
    port: 8080,
    root_dir: "www",
    enable_auth: based,
    log_file: "server.log"
}

fr fr Initialize crypto system
crypto_secure_init(1234567890, 987654321, 543216789)

slay log_request(context RequestContext, status_code normie, message tea) {
    sus current_time Time = now()
    sus timestamp_str tea = current_time.format("2006-01-02 15:04:05")
    
    sus log_entry tea = timestamp_str + " - " + context.method + " " + context.path + 
                       " - Status: " + status_code + " - " + message + "\n"
    
    append_log(server_config.log_file, log_entry)
}

slay generate_auth_token(user_id tea) AuthToken {
    sus current_time Time = now()
    sus expires normie = current_time.seconds + 3600  fr fr 1 hour expiry
    
    sus token_data tea = user_id + ":" + expires + ":" + secure_random_string(32)
    sus token_hash tea = sha256(token_data)
    
    sus token AuthToken = AuthToken{
        token: token_hash,
        user_id: user_id,
        expires: expires
    }
    
    damn token
}

slay verify_auth_token(token_str tea) (lit, tea) {
    fr fr Simulate token verification
    vibes string_length(token_str) < 10 {
        damn (cap, "Invalid token format")
    }
    
    vibes token_str == "invalid_token" {
        damn (cap, "Token verification failed")
    }
    
    fr fr Simple token validation
    vibes string_contains(token_str, "admin") {
        damn (based, "admin")
    }
    
    vibes string_contains(token_str, "user") {
        damn (based, "user")
    }
    
    damn (based, "guest")
}

slay handle_authentication(context RequestContext) (lit, tea) {
    vibes !server_config.enable_auth {
        damn (based, "Authentication disabled")
    }
    
    fr fr Extract auth header
    vibes string_contains(context.headers, "Authorization:") {
        sus token tea = "valid_user_token"  fr fr Simplified extraction
        (is_valid, user_id) := verify_auth_token(token)
        
        vibes is_valid {
            damn (based, user_id)
        } else {
            damn (cap, "Invalid authentication")
        }
    }
    
    damn (cap, "Missing authorization header")
}

slay serve_static_file(file_path tea, context RequestContext) (tea, normie, tea) {
    fr fr Security check - prevent directory traversal
    vibes string_contains(file_path, "..") {
        damn ("", 403, "Forbidden")
    }
    
    sus full_path tea = join_path(server_config.root_dir, file_path)
    
    vibes !exists(full_path) {
        damn ("", 404, "File not found")
    }
    
    (content, read_err) := read_file(full_path)
    vibes read_err != "" {
        damn ("", 500, "Internal server error")
    }
    
    fr fr Determine content type
    sus content_type tea = "text/plain"
    vibes string_ends_with(file_path, ".html") {
        content_type = "text/html"
    } nah vibes string_ends_with(file_path, ".json") {
        content_type = "application/json"
    } nah vibes string_ends_with(file_path, ".css") {
        content_type = "text/css"
    } nah vibes string_ends_with(file_path, ".js") {
        content_type = "application/javascript"
    }
    
    fr fr Update statistics
    server_stats.bytes_served = server_stats.bytes_served + get_size(full_path)
    
    damn (content, 200, content_type)
}

slay handle_api_request(context RequestContext) (tea, normie, tea) {
    vibes context.method == "GET" && context.path == "/api/status" {
        sus status_data tea = "{\"status\": \"healthy\", \"uptime\": 3600, \"requests\": " + 
                             server_stats.total_requests + "}"
        damn (status_data, 200, "application/json")
    }
    
    vibes context.method == "GET" && context.path == "/api/stats" {
        sus stats_json tea = "{\"total\": " + server_stats.total_requests + 
                            ", \"successful\": " + server_stats.successful_requests +
                            ", \"failed\": " + server_stats.failed_requests +
                            ", \"bytes_served\": " + server_stats.bytes_served + "}"
        damn (stats_json, 200, "application/json")
    }
    
    vibes context.method == "POST" && context.path == "/api/data" {
        fr fr Validate JSON input
        vibes !is_valid_json(context.body) {
            damn ("{\"error\": \"Invalid JSON\"}", 400, "application/json")
        }
        
        fr fr Process data
        sus processed_data tea = "{\"received\": true, \"processed_at\": \"" + 
                                now().format("RFC3339") + "\", \"status\": \"success\"}"
        damn (processed_data, 201, "application/json")
    }
    
    vibes context.method == "PUT" && string_starts_with(context.path, "/api/config/") {
        fr fr Configuration update endpoint
        sus config_key tea = string_substring(context.path, 12, string_length(context.path))
        
        vibes !is_valid_json(context.body) {
            damn ("{\"error\": \"Invalid JSON configuration\"}", 400, "application/json")
        }
        
        fr fr Save configuration to file
        sus config_file tea = join_path("config", config_key + ".json")
        write_err := write_file(config_file, context.body)
        
        vibes write_err != "" {
            damn ("{\"error\": \"Failed to save configuration\"}", 500, "application/json")
        }
        
        sus response tea = "{\"message\": \"Configuration updated\", \"key\": \"" + config_key + "\"}"
        damn (response, 200, "application/json")
    }
    
    damn ("{\"error\": \"Endpoint not found\"}", 404, "application/json")
}

slay parse_http_request(request_data tea) RequestContext {
    sus context RequestContext
    sus lines [tea] = string_split(request_data, "\n")
    
    vibes len(lines) > 0 {
        sus request_line [tea] = string_split(lines[0], " ")
        vibes len(request_line) >= 2 {
            context.method = request_line[0]
            context.path = request_line[1]
        }
    }
    
    fr fr Parse headers (simplified)
    context.headers = "Content-Type: application/json\nAuthorization: Bearer admin_token"
    context.body = "{\"test\": \"data\"}"
    context.timestamp = now().seconds
    
    damn context
}

slay handle_request(context RequestContext) HTTPResponse {
    sus response HTTPResponse
    
    fr fr Update request counter
    server_stats.total_requests = server_stats.total_requests + 1
    
    fr fr Authenticate if required
    (is_authenticated, user_id) := handle_authentication(context)
    
    vibes server_config.enable_auth && !is_authenticated {
        response.status_code = 401
        response.body = "{\"error\": \"Unauthorized\"}"
        response.headers = "Content-Type: application/json\nWWW-Authenticate: Bearer"
        
        log_request(context, 401, "Authentication failed")
        server_stats.failed_requests = server_stats.failed_requests + 1
        damn response
    }
    
    fr fr Route request
    vibes string_starts_with(context.path, "/api/") {
        (response_body, status, content_type) := handle_api_request(context)
        response.status_code = status
        response.body = response_body
        response.headers = "Content-Type: " + content_type
        
        log_request(context, status, "API request handled")
    } else {
        fr fr Serve static files
        (file_content, status, content_type) := serve_static_file(context.path, context)
        response.status_code = status
        response.body = file_content
        response.headers = "Content-Type: " + content_type
        
        log_request(context, status, "Static file served")
    }
    
    fr fr Update success counter
    vibes response.status_code < 400 {
        server_stats.successful_requests = server_stats.successful_requests + 1
    } else {
        server_stats.failed_requests = server_stats.failed_requests + 1
    }
    
    damn response
}

slay run_server_simulation() {
    vibez.spill("🚀 Starting CURSED Web Server Simulator")
    vibez.spill("📍 Listening on " + server_config.host + ":" + server_config.port)
    vibez.spill("📁 Document root: " + server_config.root_dir)
    vibez.spill("🔐 Authentication: " + vibes server_config.enable_auth { "enabled" } else { "disabled" })
    
    fr fr Create initial log entry
    sus startup_time Time = now()
    sus startup_msg tea = "Server started at " + startup_time.format("2006-01-02 15:04:05")
    append_log(server_config.log_file, startup_msg)
    
    fr fr Simulate various requests
    sus requests [tea] = [
        "GET /index.html HTTP/1.1",
        "GET /api/status HTTP/1.1",
        "POST /api/data HTTP/1.1",
        "GET /api/stats HTTP/1.1",
        "PUT /api/config/database HTTP/1.1",
        "GET /favicon.ico HTTP/1.1",
        "GET /static/app.js HTTP/1.1",
        "POST /api/upload HTTP/1.1"
    ]
    
    vibez.spill("\n📊 Processing simulated requests...")
    
    bestie i := 0; i < len(requests); i++ {
        sus context RequestContext = parse_http_request(requests[i])
        sus response HTTPResponse = handle_request(context)
        
        vibez.spill("  Request " + (i + 1) + ": " + context.method + " " + context.path + 
                   " -> " + response.status_code + " (" + string_length(response.body) + " bytes)")
    }
    
    fr fr Generate server statistics report
    sus stats_json tea = "{" +
        "\"total_requests\": " + server_stats.total_requests + ", " +
        "\"successful_requests\": " + server_stats.successful_requests + ", " +
        "\"failed_requests\": " + server_stats.failed_requests + ", " +
        "\"bytes_served\": " + server_stats.bytes_served + ", " +
        "\"uptime_seconds\": " + (now().seconds - startup_time.seconds) +
        "}"
    
    vibez.spill("\n📈 Server Statistics:")
    vibez.spill(pretty_print_json(stats_json, 2))
    
    fr fr Save statistics to file
    write_file("server_stats.json", stats_json)
    
    fr fr Test crypto functionality
    vibez.spill("\n🔐 Testing Authentication System:")
    sus admin_token AuthToken = generate_auth_token("admin")
    sus user_token AuthToken = generate_auth_token("user123")
    
    vibez.spill("  Generated admin token: " + string_substring(admin_token.token, 0, 16) + "...")
    vibez.spill("  Generated user token: " + string_substring(user_token.token, 0, 16) + "...")
    
    fr fr Test file operations
    vibez.spill("\n📁 Testing File System Operations:")
    sus test_content tea = "Web server test content"
    write_err := write_file("test_web_file.txt", test_content)
    vibes write_err == "" {
        vibez.spill("  ✅ File write successful")
        (read_content, read_err) := read_file("test_web_file.txt")
        vibes read_err == "" {
            vibez.spill("  ✅ File read successful: " + string_length(read_content) + " bytes")
        } else {
            vibez.spill("  ❌ File read failed: " + read_err)
        }
    } else {
        vibez.spill("  ❌ File write failed: " + write_err)
    }
    
    vibez.spill("\n🎯 Web Server Simulation Complete!")
}

fr fr Main execution
run_server_simulation()
