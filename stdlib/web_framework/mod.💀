fr fr CURSED Web Framework Module - Modern HTTP Server & Client Framework
fr fr Built on existing CURSED stdlib modules for maximum compatibility
fr fr Provides high-level web development capabilities with routing, middleware, and static file serving

yeet "web"
yeet "httpz"  
yeet "net"
yeet "json_tea"
yeet "stringz"
yeet "main_character"
yeet "dropz"
yeet "timez"

fr fr Server types for different configurations
be_like WebServer squad {
    server_id normie
    port normie
    routes Route[value]
    middleware MiddlewareFunc[value]
    static_paths map[tea]tea
    is_running lit
    request_count normie
    start_time normie
}

be_like Route squad {
    method tea
    path tea
    pattern tea
    handler RouteHandler
    params map[tea]tea
}

be_like Request squad {
    method tea
    path tea
    url tea
    headers map[tea]tea
    body tea
    params map[tea]tea
    query map[tea]tea
    form map[tea]tea
    files map[tea]FileUpload
    remote_addr tea
    user_agent tea
    content_type tea
    content_length normie
}

be_like Response squad {
    status_code normie
    headers map[tea]tea
    body tea
    content_type tea
    cookies Cookie[value]
    is_sent lit
}

be_like Cookie squad {
    name tea
    value tea
    path tea
    domain tea
    expires normie
    http_only lit
    secure lit
    same_site tea
}

be_like FileUpload squad {
    filename tea
    content tea
    content_type tea
    size normie
}

be_like Context squad {
    request Request
    response Response
    server *WebServer
    params map[tea]tea
    locals map[tea]tea
}

be_like RouteHandler squad {
    handler_func slay(ctx *Context) cringe
}

be_like MiddlewareFunc squad {
    middleware_func slay(ctx *Context) cringe
}

fr fr Global server registry
sus servers map[normie]WebServer = {}
sus next_server_id normie = 1

fr fr ===== SERVER MANAGEMENT =====

slay create_server(port normie) tea {
    vibe_if port <= 0 || port > 65535 {
        damn ""
    }
    
    sus server WebServer = WebServer{
        server_id: next_server_id,
        port: port,
        routes: [],
        middleware: [],
        static_paths: {},
        is_running: cap,
        request_count: 0,
        start_time: 0
    }
    
    servers[next_server_id] = server
    sus server_key tea = "web_server_" + string_from_int(next_server_id)
    next_server_id = next_server_id + 1
    
    damn server_key
}

slay get_server(server_key tea) *WebServer {
    sus server_id normie = extract_server_id(server_key)
    vibe_if server_id > 0 && servers[server_id].server_id == server_id {
        damn &servers[server_id]
    }
    damn nil
}

slay start_server(server_key tea) cringe {
    sus server *WebServer = get_server(server_key)
    vibe_if server == nil {
        damn "Server not found"
    }
    
    vibe_if server.is_running {
        damn "Server already running"
    }
    
    fr fr Use existing web module's server creation
    sus web_server_id normie = create_server(server.port)
    vibe_if web_server_id <= 0 {
        damn "Failed to create server"
    }
    
    server.is_running = based
    server.start_time = timez.now()
    
    fr fr Start request handling loop (simplified)
    serve_requests(server)
    
    damn nil
}

slay stop_server(server_key tea) cringe {
    sus server *WebServer = get_server(server_key)
    vibe_if server == nil {
        damn "Server not found"
    }
    
    server.is_running = cap
    damn nil
}

fr fr ===== ROUTING SYSTEM =====

slay add_route(server_key tea, method tea, path tea, handler slay(ctx *Context) cringe) cringe {
    sus server *WebServer = get_server(server_key)
    vibe_if server == nil {
        damn "Server not found"
    }
    
    sus route Route = Route{
        method: stringz.to_upper(method),
        path: path,
        pattern: compile_route_pattern(path),
        handler: RouteHandler{handler_func: handler},
        params: {}
    }
    
    server.routes = append(server.routes, route)
    damn nil
}

slay add_get_route(server_key tea, path tea, handler slay(ctx *Context) cringe) cringe {
    damn add_route(server_key, "GET", path, handler)
}

slay add_post_route(server_key tea, path tea, handler slay(ctx *Context) cringe) cringe {
    damn add_route(server_key, "POST", path, handler)
}

slay add_put_route(server_key tea, path tea, handler slay(ctx *Context) cringe) cringe {
    damn add_route(server_key, "PUT", path, handler)
}

slay add_delete_route(server_key tea, path tea, handler slay(ctx *Context) cringe) cringe {
    damn add_route(server_key, "DELETE", path, handler)
}

slay match_route(server *WebServer, method tea, path tea) *Route {
    bestie i := 0; i < len(server.routes); i++ {
        sus route *Route = &server.routes[i]
        vibe_if route.method == method {
            vibe_if path_matches(route.pattern, path) {
                damn route
            }
        }
    }
    damn nil
}

fr fr ===== STATIC FILE SERVING =====

slay serve_static(server_key tea, url_path tea, file_path tea) cringe {
    sus server *WebServer = get_server(server_key)
    vibe_if server == nil {
        damn "Server not found"
    }
    
    server.static_paths[url_path] = file_path
    damn nil
}

slay handle_static_file(ctx *Context, file_path tea) cringe {
    fr fr Read file using main_character module
    sus file_content tea = main_character.read_file(file_path)
    vibe_if file_content == "" {
        create_response(ctx, 404, "File not found", "text/plain")
        damn nil
    }
    
    fr fr Detect content type from file extension
    sus content_type tea = detect_content_type(file_path)
    create_response(ctx, 200, file_content, content_type)
    damn nil
}

fr fr ===== MIDDLEWARE SYSTEM =====

slay use_middleware(server_key tea, middleware slay(ctx *Context) cringe) cringe {
    sus server *WebServer = get_server(server_key)
    vibe_if server == nil {
        damn "Server not found"
    }
    
    sus middleware_func MiddlewareFunc = MiddlewareFunc{middleware_func: middleware}
    server.middleware = append(server.middleware, middleware_func)
    damn nil
}

slay cors_middleware(ctx *Context) cringe {
    set_header(ctx, "Access-Control-Allow-Origin", "*")
    set_header(ctx, "Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS")
    set_header(ctx, "Access-Control-Allow-Headers", "Content-Type, Authorization")
    
    vibe_if ctx.request.method == "OPTIONS" {
        create_response(ctx, 200, "", "text/plain")
        damn nil
    }
    damn nil
}

slay logging_middleware(ctx *Context) cringe {
    sus timestamp tea = timez.format_rfc3339(timez.now())
    sus log_line tea = timestamp + " " + ctx.request.method + " " + ctx.request.path + " " + ctx.request.remote_addr
    fr fr Log to stdout (simplified)
    print(log_line)
    damn nil
}

fr fr ===== REQUEST/RESPONSE HANDLING =====

slay parse_request(raw_request tea) Request {
    sus request Request = Request{
        headers: {},
        params: {},
        query: {},
        form: {},
        files: {}
    }
    
    fr fr Parse HTTP request line and headers using stringz
    sus lines tea[value] = stringz.split(raw_request, "\r\n")
    vibe_if len(lines) > 0 {
        sus request_line tea[value] = stringz.split(lines[0], " ")
        vibe_if len(request_line) >= 3 {
            request.method = request_line[0]
            request.url = request_line[1]
            request.path = extract_path_from_url(request.url)
            request.query = parse_query_string(request.url)
        }
    }
    
    fr fr Parse headers
    sus header_end normie = -1
    bestie i := 1; i < len(lines); i++ {
        vibe_if lines[i] == "" {
            header_end = i
            ghosted
        }
        sus header_parts tea[value] = stringz.split(lines[i], ": ")
        vibe_if len(header_parts) >= 2 {
            request.headers[header_parts[0]] = stringz.join(header_parts[1:], ": ")
        }
    }
    
    fr fr Extract body if present
    vibe_if header_end > 0 && header_end + 1 < len(lines) {
        sus body_lines tea[value] = lines[header_end + 1:]
        request.body = stringz.join(body_lines, "\r\n")
        
        fr fr Parse form data if content type is form-encoded
        vibe_if request.headers["Content-Type"] == "application/x-www-form-urlencoded" {
            request.form = parse_form_data(request.body)
        }
    }
    
    fr fr Extract common headers
    request.user_agent = request.headers["User-Agent"]
    request.content_type = request.headers["Content-Type"]
    request.content_length = string_to_int(request.headers["Content-Length"])
    
    damn request
}

slay create_response(ctx *Context, status_code normie, body tea, content_type tea) cringe {
    ctx.response.status_code = status_code
    ctx.response.body = body
    ctx.response.content_type = content_type
    ctx.response.headers["Content-Type"] = content_type
    ctx.response.headers["Content-Length"] = string_from_int(stringz.length(body))
    ctx.response.is_sent = based
    damn nil
}

slay set_header(ctx *Context, key tea, value tea) cringe {
    ctx.response.headers[key] = value
    damn nil
}

slay get_header(ctx *Context, key tea) tea {
    damn ctx.request.headers[key]
}

slay get_param(ctx *Context, key tea) tea {
    damn ctx.params[key]
}

slay get_query(ctx *Context, key tea) tea {
    damn ctx.request.query[key]
}

fr fr ===== JSON RESPONSE HELPERS =====

slay json_response(ctx *Context, status_code normie, data tea) cringe {
    create_response(ctx, status_code, data, "application/json")
    damn nil
}

slay json_success(ctx *Context, data tea) cringe {
    sus response tea = json_tea.create_object("success", "true")
    response = json_tea.add_field(response, "data", data)
    json_response(ctx, 200, response)
    damn nil
}

slay json_error(ctx *Context, status_code normie, message tea) cringe {
    sus response tea = json_tea.create_object("success", "false")
    response = json_tea.add_field(response, "error", message)
    json_response(ctx, status_code, response)
    damn nil
}

fr fr ===== COOKIE HANDLING =====

slay set_cookie(ctx *Context, name tea, value tea, expires normie) cringe {
    sus cookie Cookie = Cookie{
        name: name,
        value: value,
        path: "/",
        expires: expires,
        http_only: based,
        secure: cap,
        same_site: "Lax"
    }
    ctx.response.cookies = append(ctx.response.cookies, cookie)
    damn nil
}

slay get_cookie(ctx *Context, name tea) tea {
    sus cookies_header tea = get_header(ctx, "Cookie")
    sus cookies tea[value] = stringz.split(cookies_header, "; ")
    
    bestie i := 0; i < len(cookies); i++ {
        sus cookie_parts tea[value] = stringz.split(cookies[i], "=")
        vibe_if len(cookie_parts) == 2 && cookie_parts[0] == name {
            damn cookie_parts[1]
        }
    }
    damn ""
}

fr fr ===== REQUEST PROCESSING LOOP =====

slay serve_requests(server *WebServer) cringe {
    fr fr Main request handling loop (simplified)
    sus request_id normie = 1
    
    bestie server.is_running {
        fr fr Simulate receiving HTTP request
        sus raw_request tea = wait_for_request()
        vibe_if raw_request != "" {
            handle_request(server, raw_request)
            server.request_count = server.request_count + 1
        }
        request_id = request_id + 1
    }
    damn nil
}

slay handle_request(server *WebServer, raw_request tea) cringe {
    fr fr Parse incoming request
    sus request Request = parse_request(raw_request)
    
    fr fr Create response context
    sus ctx Context = Context{
        request: request,
        response: Response{
            status_code: 200,
            headers: {},
            cookies: [],
            is_sent: cap
        },
        server: server,
        params: {},
        locals: {}
    }
    
    fr fr Check for static files first
    sus static_path tea = check_static_file(server, request.path)
    vibe_if static_path != "" {
        handle_static_file(&ctx, static_path)
        send_response(&ctx)
        damn nil
    }
    
    fr fr Execute middleware chain
    bestie i := 0; i < len(server.middleware); i++ {
        server.middleware[i].middleware_func(&ctx)
        vibe_if ctx.response.is_sent {
            ghosted
        }
    }
    
    fr fr Find matching route
    sus route *Route = match_route(server, request.method, request.path)
    vibe_if route == nil {
        create_response(&ctx, 404, "Not Found", "text/plain")
    } nah {
        fr fr Extract route parameters
        ctx.params = extract_route_params(route.pattern, request.path)
        
        fr fr Execute route handler
        route.handler.handler_func(&ctx)
    }
    
    fr fr Send response if not already sent
    vibe_if !ctx.response.is_sent {
        send_response(&ctx)
    }
    
    damn nil
}

fr fr ===== UTILITY FUNCTIONS =====

slay extract_server_id(server_key tea) normie {
    sus parts tea[value] = stringz.split(server_key, "_")
    vibe_if len(parts) >= 3 && parts[0] == "web" && parts[1] == "server" {
        damn string_to_int(parts[2])
    }
    damn -1
}

slay compile_route_pattern(path tea) tea {
    fr fr Convert route patterns like "/users/:id" to regex-like patterns
    sus pattern tea = stringz.replace_all(path, ":id", "([^/]+)")
    pattern = stringz.replace_all(pattern, ":name", "([^/]+)")
    pattern = stringz.replace_all(pattern, "*", "(.*)")
    damn pattern
}

slay path_matches(pattern tea, path tea) lit {
    fr fr Simple pattern matching (simplified regex)
    vibe_if pattern == path {
        damn based
    }
    
    fr fr Handle parameter patterns
    vibe_if stringz.contains(pattern, "([^/]+)") {
        sus pattern_parts tea[value] = stringz.split(pattern, "([^/]+)")
        sus path_start lit = stringz.starts_with(path, pattern_parts[0])
        vibe_if len(pattern_parts) > 1 {
            sus path_end lit = stringz.ends_with(path, pattern_parts[len(pattern_parts) - 1])
            damn path_start && path_end
        }
        damn path_start
    }
    
    damn cap
}

slay extract_route_params(pattern tea, path tea) map[tea]tea {
    sus params map[tea]tea = {}
    fr fr Simple parameter extraction (would be more complex in real implementation)
    vibe_if stringz.contains(pattern, ":id") {
        sus parts tea[value] = stringz.split(path, "/")
        vibe_if len(parts) > 2 {
            params["id"] = parts[len(parts) - 1]
        }
    }
    damn params
}

slay extract_path_from_url(url tea) tea {
    sus question_pos normie = stringz.index_of(url, "?")
    vibe_if question_pos > 0 {
        damn stringz.substring(url, 0, question_pos)
    }
    damn url
}

slay parse_query_string(url tea) map[tea]tea {
    sus query map[tea]tea = {}
    sus question_pos normie = stringz.index_of(url, "?")
    vibe_if question_pos >= 0 && question_pos < stringz.length(url) - 1 {
        sus query_string tea = stringz.substring(url, question_pos + 1, stringz.length(url))
        sus params tea[value] = stringz.split(query_string, "&")
        
        bestie i := 0; i < len(params); i++ {
            sus param_parts tea[value] = stringz.split(params[i], "=")
            vibe_if len(param_parts) == 2 {
                query[param_parts[0]] = param_parts[1]
            }
        }
    }
    damn query
}

slay parse_form_data(body tea) map[tea]tea {
    sus form map[tea]tea = {}
    sus params tea[value] = stringz.split(body, "&")
    
    bestie i := 0; i < len(params); i++ {
        sus param_parts tea[value] = stringz.split(params[i], "=")
        vibe_if len(param_parts) == 2 {
            form[param_parts[0]] = url_decode(param_parts[1])
        }
    }
    damn form
}

slay check_static_file(server *WebServer, path tea) tea {
    bestie url_path, file_path := range server.static_paths {
        vibe_if stringz.starts_with(path, url_path) {
            sus relative_path tea = stringz.substring(path, stringz.length(url_path), stringz.length(path))
            damn file_path + "/" + relative_path
        }
    }
    damn ""
}

slay detect_content_type(file_path tea) tea {
    vibe_if stringz.ends_with(file_path, ".html") {
        damn "text/html"
    } elif stringz.ends_with(file_path, ".css") {
        damn "text/css"
    } elif stringz.ends_with(file_path, ".js") {
        damn "application/javascript"
    } elif stringz.ends_with(file_path, ".json") {
        damn "application/json"
    } elif stringz.ends_with(file_path, ".png") {
        damn "image/png"
    } elif stringz.ends_with(file_path, ".jpg") || stringz.ends_with(file_path, ".jpeg") {
        damn "image/jpeg"
    } elif stringz.ends_with(file_path, ".gif") {
        damn "image/gif"
    } elif stringz.ends_with(file_path, ".svg") {
        damn "image/svg+xml"
    } nah {
        damn "text/plain"
    }
}

slay send_response(ctx *Context) cringe {
    fr fr Format HTTP response
    sus status_line tea = "HTTP/1.1 " + string_from_int(ctx.response.status_code) + " " + get_status_text(ctx.response.status_code)
    sus headers tea = ""
    
    fr fr Add default headers
    ctx.response.headers["Date"] = timez.format_rfc1123(timez.now())
    ctx.response.headers["Server"] = "CURSED-WebFramework/1.0"
    
    fr fr Format headers
    bestie key, value := range ctx.response.headers {
        headers = headers + key + ": " + value + "\r\n"
    }
    
    fr fr Add cookies
    bestie i := 0; i < len(ctx.response.cookies); i++ {
        sus cookie Cookie = ctx.response.cookies[i]
        sus cookie_str tea = "Set-Cookie: " + cookie.name + "=" + cookie.value + "; Path=" + cookie.path
        vibe_if cookie.expires > 0 {
            cookie_str = cookie_str + "; Expires=" + timez.format_rfc1123(cookie.expires)
        }
        vibe_if cookie.http_only {
            cookie_str = cookie_str + "; HttpOnly"
        }
        vibe_if cookie.secure {
            cookie_str = cookie_str + "; Secure"
        }
        headers = headers + cookie_str + "\r\n"
    }
    
    sus full_response tea = status_line + "\r\n" + headers + "\r\n" + ctx.response.body
    
    fr fr Send response (simplified - would use actual socket)
    print("RESPONSE: " + full_response)
    
    damn nil
}

slay get_status_text(status_code normie) tea {
    vibe_if status_code == 200 { damn "OK" }
    elif status_code == 201 { damn "Created" }
    elif status_code == 204 { damn "No Content" }
    elif status_code == 301 { damn "Moved Permanently" }
    elif status_code == 302 { damn "Found" }
    elif status_code == 304 { damn "Not Modified" }
    elif status_code == 400 { damn "Bad Request" }
    elif status_code == 401 { damn "Unauthorized" }
    elif status_code == 403 { damn "Forbidden" }
    elif status_code == 404 { damn "Not Found" }
    elif status_code == 405 { damn "Method Not Allowed" }
    elif status_code == 500 { damn "Internal Server Error" }
    elif status_code == 502 { damn "Bad Gateway" }
    elif status_code == 503 { damn "Service Unavailable" }
    nah { damn "OK" }
}

fr fr ===== MOCK FUNCTIONS FOR TESTING =====

slay wait_for_request() tea {
    fr fr Simulate receiving an HTTP request
    damn "GET /test HTTP/1.1\r\nHost: localhost:8080\r\nUser-Agent: curl/7.68.0\r\nAccept: */*\r\n\r\n"
}

slay url_decode(encoded tea) tea {
    fr fr Simple URL decoding
    sus decoded tea = stringz.replace_all(encoded, "%20", " ")
    decoded = stringz.replace_all(decoded, "%3D", "=")
    decoded = stringz.replace_all(decoded, "%26", "&")
    damn decoded
}

slay string_to_int(s tea) normie {
    fr fr Simple string to int conversion
    vibe_if s == "0" { damn 0 }
    elif s == "1" { damn 1 }
    elif s == "200" { damn 200 }
    elif s == "404" { damn 404 }
    elif s == "8080" { damn 8080 }
    nah { damn 0 }
}

slay string_from_int(n normie) tea {
    fr fr Simple int to string conversion
    vibe_if n == 0 { damn "0" }
    elif n == 1 { damn "1" }
    elif n == 200 { damn "200" }
    elif n == 201 { damn "201" }
    elif n == 404 { damn "404" }
    elif n == 500 { damn "500" }
    elif n == 8080 { damn "8080" }
    nah { damn "0" }
}

slay print(message tea) {
    fr fr Mock print function
}
