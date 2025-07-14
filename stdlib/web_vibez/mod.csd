# web_vibez - Comprehensive HTTP Client and Server Module
# Pure CURSED implementation of HTTP functionality
yeet "vibez"
yeet "timez"
yeet "stringz"
yeet "dropz"
yeet "encode_mood"

# HTTP Constants
fact HTTP_VERSION tea = "HTTP/1.1"
fact DEFAULT_PORT normie = 80
fact DEFAULT_HTTPS_PORT normie = 443
fact DEFAULT_TIMEOUT normie = 30000000000  # 30 seconds in nanoseconds
fact MAX_HEADER_SIZE normie = 8192
fact MAX_BODY_SIZE normie = 10485760  # 10MB
fact CRLF tea = "\r\n"
fact HTTP_SEPARATOR tea = ": "

# HTTP Status Codes
fact STATUS_OK normie = 200
fact STATUS_CREATED normie = 201
fact STATUS_ACCEPTED normie = 202
fact STATUS_NO_CONTENT normie = 204
fact STATUS_MOVED_PERMANENTLY normie = 301
fact STATUS_FOUND normie = 302
fact STATUS_NOT_MODIFIED normie = 304
fact STATUS_BAD_REQUEST normie = 400
fact STATUS_UNAUTHORIZED normie = 401
fact STATUS_FORBIDDEN normie = 403
fact STATUS_NOT_FOUND normie = 404
fact STATUS_METHOD_NOT_ALLOWED normie = 405
fact STATUS_INTERNAL_SERVER_ERROR normie = 500
fact STATUS_NOT_IMPLEMENTED normie = 501
fact STATUS_BAD_GATEWAY normie = 502
fact STATUS_SERVICE_UNAVAILABLE normie = 503

# HTTP Methods
fact METHOD_GET tea = "GET"
fact METHOD_POST tea = "POST"
fact METHOD_PUT tea = "PUT"
fact METHOD_DELETE tea = "DELETE"
fact METHOD_PATCH tea = "PATCH"
fact METHOD_HEAD tea = "HEAD"
fact METHOD_OPTIONS tea = "OPTIONS"

# Common Headers
fact HEADER_CONTENT_TYPE tea = "Content-Type"
fact HEADER_CONTENT_LENGTH tea = "Content-Length"
fact HEADER_AUTHORIZATION tea = "Authorization"
fact HEADER_ACCEPT tea = "Accept"
fact HEADER_USER_AGENT tea = "User-Agent"
fact HEADER_HOST tea = "Host"
fact HEADER_CONNECTION tea = "Connection"
fact HEADER_LOCATION tea = "Location"
fact HEADER_SET_COOKIE tea = "Set-Cookie"
fact HEADER_COOKIE tea = "Cookie"

# Content Types
fact CONTENT_TYPE_JSON tea = "application/json"
fact CONTENT_TYPE_TEXT tea = "text/plain"
fact CONTENT_TYPE_HTML tea = "text/html"
fact CONTENT_TYPE_FORM tea = "application/x-www-form-urlencoded"
fact CONTENT_TYPE_MULTIPART tea = "multipart/form-data"

# Core HTTP Structures
struct HttpHeader {
    name tea,
    value tea
}

struct HttpHeaders {
    headers [50]HttpHeader,  # Fixed array for simplicity
    count normie
}

struct HttpUrl {
    scheme tea,
    host tea,
    port normie,
    path tea,
    query tea,
    fragment tea
}

struct HttpCookie {
    name tea,
    value tea,
    domain tea,
    path tea,
    expires thicc,
    secure lit,
    http_only lit
}

struct HttpRequest {
    method tea,
    url HttpUrl,
    proto tea,
    proto_major normie,
    proto_minor normie,
    headers HttpHeaders,
    body tea,
    content_length thicc,
    host tea,
    remote_addr tea
}

struct HttpResponse {
    status tea,
    status_code normie,
    proto tea,
    proto_major normie,
    proto_minor normie,
    headers HttpHeaders,
    body tea,
    content_length thicc
}

struct HttpClient {
    timeout normie,
    user_agent tea,
    max_redirects normie
}

struct HttpServer {
    addr tea,
    port normie,
    timeout normie,
    max_connections normie,
    running lit
}

struct HttpHandler {
    pattern tea,
    handler_name tea
}

struct HttpMiddleware {
    name tea,
    enabled lit
}

# Global state for server management
sus server_handlers [100]HttpHandler
sus handler_count normie = 0
sus middleware_stack [10]HttpMiddleware
sus middleware_count normie = 0
sus server_running lit = cap

# ===============================
# HTTP Header Functions
# ===============================

slay init_headers() HttpHeaders {
    sus headers HttpHeaders
    headers.count = 0
    damn headers
}

slay add_header(headers HttpHeaders, name tea, value tea) HttpHeaders {
    fr fr headers.count < 50 {
        headers.headers[headers.count].name = name
        headers.headers[headers.count].value = value
        headers.count = headers.count + 1
    }
    damn headers
}

slay get_header(headers HttpHeaders, name tea) tea {
    sus i normie = 0
    bestie i = 0; i < headers.count; i = i + 1 {
        fr fr stringz.to_lower(headers.headers[i].name) == stringz.to_lower(name) {
            damn headers.headers[i].value
        }
    }
    damn ""
}

slay set_header(headers HttpHeaders, name tea, value tea) HttpHeaders {
    sus i normie = 0
    bestie i = 0; i < headers.count; i = i + 1 {
        fr fr stringz.to_lower(headers.headers[i].name) == stringz.to_lower(name) {
            headers.headers[i].value = value
            damn headers
        }
    }
    # If not found, add new header
    damn add_header(headers, name, value)
}

slay remove_header(headers HttpHeaders, name tea) HttpHeaders {
    sus i normie = 0
    bestie i = 0; i < headers.count; i = i + 1 {
        fr fr stringz.to_lower(headers.headers[i].name) == stringz.to_lower(name) {
            # Shift remaining headers
            sus j normie = i
            bestie j = i; j < headers.count - 1; j = j + 1 {
                headers.headers[j] = headers.headers[j + 1]
            }
            headers.count = headers.count - 1
            ghosted
        }
    }
    damn headers
}

slay headers_to_string(headers HttpHeaders) tea {
    sus result tea = ""
    sus i normie = 0
    bestie i = 0; i < headers.count; i = i + 1 {
        result = result + headers.headers[i].name + HTTP_SEPARATOR + headers.headers[i].value + CRLF
    }
    damn result
}

# ===============================
# HTTP URL Functions
# ===============================

slay parse_url(url_string tea) HttpUrl {
    sus url HttpUrl
    url.scheme = "http"
    url.host = "localhost"
    url.port = DEFAULT_PORT
    url.path = "/"
    url.query = ""
    url.fragment = ""
    
    # Simple URL parsing (basic implementation)
    fr fr stringz.starts_with(url_string, "https://") {
        url.scheme = "https"
        url.port = DEFAULT_HTTPS_PORT
        url_string = stringz.substring(url_string, 8, stringz.length(url_string))
    } vr fr stringz.starts_with(url_string, "http://") {
        url_string = stringz.substring(url_string, 7, stringz.length(url_string))
    }
    
    # Extract host and path
    sus slash_pos normie = stringz.index_of(url_string, "/")
    fr fr slash_pos > 0 {
        url.host = stringz.substring(url_string, 0, slash_pos)
        url.path = stringz.substring(url_string, slash_pos, stringz.length(url_string))
    } else {
        url.host = url_string
    }
    
    # Extract port if present
    sus colon_pos normie = stringz.index_of(url.host, ":")
    fr fr colon_pos > 0 {
        sus port_str tea = stringz.substring(url.host, colon_pos + 1, stringz.length(url.host))
        url.port = stringz.to_int(port_str)
        url.host = stringz.substring(url.host, 0, colon_pos)
    }
    
    damn url
}

slay url_to_string(url HttpUrl) tea {
    sus result tea = url.scheme + "://" + url.host
    fr fr (url.scheme == "http" && url.port != DEFAULT_PORT) || 
          (url.scheme == "https" && url.port != DEFAULT_HTTPS_PORT) {
        result = result + ":" + tea(url.port)
    }
    result = result + url.path
    fr fr url.query != "" {
        result = result + "?" + url.query
    }
    fr fr url.fragment != "" {
        result = result + "#" + url.fragment
    }
    damn result
}

# ===============================
# HTTP Request Functions
# ===============================

slay create_request(method tea, url_string tea) HttpRequest {
    sus request HttpRequest
    request.method = method
    request.url = parse_url(url_string)
    request.proto = HTTP_VERSION
    request.proto_major = 1
    request.proto_minor = 1
    request.headers = init_headers()
    request.body = ""
    request.content_length = 0
    request.host = request.url.host
    request.remote_addr = ""
    
    # Add default headers
    request.headers = add_header(request.headers, HEADER_HOST, request.host)
    request.headers = add_header(request.headers, HEADER_USER_AGENT, "CURSED-HTTP/1.0")
    
    damn request
}

slay set_request_body(request HttpRequest, body tea) HttpRequest {
    request.body = body
    request.content_length = stringz.length(body)
    request.headers = set_header(request.headers, HEADER_CONTENT_LENGTH, tea(request.content_length))
    damn request
}

slay set_request_json(request HttpRequest, json_body tea) HttpRequest {
    request.headers = set_header(request.headers, HEADER_CONTENT_TYPE, CONTENT_TYPE_JSON)
    damn set_request_body(request, json_body)
}

slay add_request_header(request HttpRequest, name tea, value tea) HttpRequest {
    request.headers = add_header(request.headers, name, value)
    damn request
}

slay request_to_string(request HttpRequest) tea {
    sus result tea = request.method + " " + request.url.path
    fr fr request.url.query != "" {
        result = result + "?" + request.url.query
    }
    result = result + " " + request.proto + CRLF
    result = result + headers_to_string(request.headers)
    result = result + CRLF
    fr fr request.body != "" {
        result = result + request.body
    }
    damn result
}

# ===============================
# HTTP Response Functions
# ===============================

slay create_response(status_code normie) HttpResponse {
    sus response HttpResponse
    response.status_code = status_code
    response.status = get_status_text(status_code)
    response.proto = HTTP_VERSION
    response.proto_major = 1
    response.proto_minor = 1
    response.headers = init_headers()
    response.body = ""
    response.content_length = 0
    
    # Add default headers
    response.headers = add_header(response.headers, HEADER_CONNECTION, "close")
    response.headers = add_header(response.headers, "Server", "CURSED-HTTP/1.0")
    
    damn response
}

slay set_response_body(response HttpResponse, body tea) HttpResponse {
    response.body = body
    response.content_length = stringz.length(body)
    response.headers = set_header(response.headers, HEADER_CONTENT_LENGTH, tea(response.content_length))
    damn response
}

slay set_response_json(response HttpResponse, json_body tea) HttpResponse {
    response.headers = set_header(response.headers, HEADER_CONTENT_TYPE, CONTENT_TYPE_JSON)
    damn set_response_body(response, json_body)
}

slay set_response_html(response HttpResponse, html_body tea) HttpResponse {
    response.headers = set_header(response.headers, HEADER_CONTENT_TYPE, CONTENT_TYPE_HTML)
    damn set_response_body(response, html_body)
}

slay add_response_header(response HttpResponse, name tea, value tea) HttpResponse {
    response.headers = add_header(response.headers, name, value)
    damn response
}

slay response_to_string(response HttpResponse) tea {
    sus result tea = response.proto + " " + tea(response.status_code) + " " + response.status + CRLF
    result = result + headers_to_string(response.headers)
    result = result + CRLF
    fr fr response.body != "" {
        result = result + response.body
    }
    damn result
}

# ===============================
# HTTP Status Functions
# ===============================

slay get_status_text(status_code normie) tea {
    fr fr status_code == STATUS_OK {
        damn "OK"
    } vr fr status_code == STATUS_CREATED {
        damn "Created"
    } vr fr status_code == STATUS_ACCEPTED {
        damn "Accepted"
    } vr fr status_code == STATUS_NO_CONTENT {
        damn "No Content"
    } vr fr status_code == STATUS_MOVED_PERMANENTLY {
        damn "Moved Permanently"
    } vr fr status_code == STATUS_FOUND {
        damn "Found"
    } vr fr status_code == STATUS_NOT_MODIFIED {
        damn "Not Modified"
    } vr fr status_code == STATUS_BAD_REQUEST {
        damn "Bad Request"
    } vr fr status_code == STATUS_UNAUTHORIZED {
        damn "Unauthorized"
    } vr fr status_code == STATUS_FORBIDDEN {
        damn "Forbidden"
    } vr fr status_code == STATUS_NOT_FOUND {
        damn "Not Found"
    } vr fr status_code == STATUS_METHOD_NOT_ALLOWED {
        damn "Method Not Allowed"
    } vr fr status_code == STATUS_INTERNAL_SERVER_ERROR {
        damn "Internal Server Error"
    } vr fr status_code == STATUS_NOT_IMPLEMENTED {
        damn "Not Implemented"
    } vr fr status_code == STATUS_BAD_GATEWAY {
        damn "Bad Gateway"
    } vr fr status_code == STATUS_SERVICE_UNAVAILABLE {
        damn "Service Unavailable"
    } else {
        damn "Unknown Status"
    }
}

# ===============================
# HTTP Client Functions
# ===============================

slay create_client() HttpClient {
    sus client HttpClient
    client.timeout = DEFAULT_TIMEOUT
    client.user_agent = "CURSED-HTTP/1.0"
    client.max_redirects = 10
    damn client
}

slay client_get(client HttpClient, url tea) HttpResponse {
    sus request HttpRequest = create_request(METHOD_GET, url)
    request.headers = set_header(request.headers, HEADER_USER_AGENT, client.user_agent)
    damn send_request(request)
}

slay client_post(client HttpClient, url tea, body tea) HttpResponse {
    sus request HttpRequest = create_request(METHOD_POST, url)
    request = set_request_body(request, body)
    request.headers = set_header(request.headers, HEADER_USER_AGENT, client.user_agent)
    damn send_request(request)
}

slay client_post_json(client HttpClient, url tea, json_body tea) HttpResponse {
    sus request HttpRequest = create_request(METHOD_POST, url)
    request = set_request_json(request, json_body)
    request.headers = set_header(request.headers, HEADER_USER_AGENT, client.user_agent)
    damn send_request(request)
}

slay client_put(client HttpClient, url tea, body tea) HttpResponse {
    sus request HttpRequest = create_request(METHOD_PUT, url)
    request = set_request_body(request, body)
    request.headers = set_header(request.headers, HEADER_USER_AGENT, client.user_agent)
    damn send_request(request)
}

slay client_delete(client HttpClient, url tea) HttpResponse {
    sus request HttpRequest = create_request(METHOD_DELETE, url)
    request.headers = set_header(request.headers, HEADER_USER_AGENT, client.user_agent)
    damn send_request(request)
}

# ===============================
# HTTP Request Sending (Simplified)
# ===============================

slay send_request(request HttpRequest) HttpResponse {
    # Simplified HTTP request sending
    # In a real implementation, this would create TCP connections
    vibez.spill("Sending HTTP request:")
    vibez.spill("Method: " + request.method)
    vibez.spill("URL: " + url_to_string(request.url))
    vibez.spill("Headers: " + tea(request.headers.count))
    
    # Simulate HTTP response
    sus response HttpResponse = create_response(STATUS_OK)
    response = set_response_json(response, '{"message":"Success","timestamp":"' + tea(timez.now_unix()) + '"}')
    
    damn response
}

# ===============================
# HTTP Server Functions
# ===============================

slay create_server(addr tea, port normie) HttpServer {
    sus server HttpServer
    server.addr = addr
    server.port = port
    server.timeout = DEFAULT_TIMEOUT
    server.max_connections = 100
    server.running = cap
    damn server
}

slay server_start(server HttpServer) lit {
    fr fr server_running == based {
        vibez.spill("Server already running")
        damn cap
    }
    
    vibez.spill("Starting HTTP server on " + server.addr + ":" + tea(server.port))
    server_running = based
    server.running = based
    
    # Simulate server startup
    vibez.spill("Server started successfully")
    damn based
}

slay server_stop(server HttpServer) lit {
    fr fr server_running == cap {
        vibez.spill("Server not running")
        damn cap
    }
    
    vibez.spill("Stopping HTTP server")
    server_running = cap
    server.running = cap
    
    vibez.spill("Server stopped successfully")
    damn based
}

slay server_handle_func(pattern tea, handler_name tea) {
    fr fr handler_count < 100 {
        server_handlers[handler_count].pattern = pattern
        server_handlers[handler_count].handler_name = handler_name
        handler_count = handler_count + 1
        vibez.spill("Registered handler for pattern: " + pattern)
    }
}

slay server_handle_get(pattern tea, handler_name tea) {
    server_handle_func("GET " + pattern, handler_name)
}

slay server_handle_post(pattern tea, handler_name tea) {
    server_handle_func("POST " + pattern, handler_name)
}

slay server_handle_put(pattern tea, handler_name tea) {
    server_handle_func("PUT " + pattern, handler_name)
}

slay server_handle_delete(pattern tea, handler_name tea) {
    server_handle_func("DELETE " + pattern, handler_name)
}

# ===============================
# HTTP Middleware Functions
# ===============================

slay add_middleware(name tea) {
    fr fr middleware_count < 10 {
        middleware_stack[middleware_count].name = name
        middleware_stack[middleware_count].enabled = based
        middleware_count = middleware_count + 1
        vibez.spill("Added middleware: " + name)
    }
}

slay remove_middleware(name tea) {
    sus i normie = 0
    bestie i = 0; i < middleware_count; i = i + 1 {
        fr fr middleware_stack[i].name == name {
            # Shift remaining middleware
            sus j normie = i
            bestie j = i; j < middleware_count - 1; j = j + 1 {
                middleware_stack[j] = middleware_stack[j + 1]
            }
            middleware_count = middleware_count - 1
            vibez.spill("Removed middleware: " + name)
            ghosted
        }
    }
}

slay enable_logging_middleware() {
    add_middleware("logging")
}

slay enable_cors_middleware() {
    add_middleware("cors")
}

slay enable_compression_middleware() {
    add_middleware("compression")
}

slay enable_rate_limit_middleware() {
    add_middleware("rate_limit")
}

# ===============================
# HTTP Cookie Functions
# ===============================

slay create_cookie(name tea, value tea) HttpCookie {
    sus cookie HttpCookie
    cookie.name = name
    cookie.value = value
    cookie.domain = ""
    cookie.path = "/"
    cookie.expires = 0
    cookie.secure = cap
    cookie.http_only = cap
    damn cookie
}

slay cookie_to_string(cookie HttpCookie) tea {
    sus result tea = cookie.name + "=" + cookie.value
    fr fr cookie.path != "" {
        result = result + "; Path=" + cookie.path
    }
    fr fr cookie.domain != "" {
        result = result + "; Domain=" + cookie.domain
    }
    fr fr cookie.expires > 0 {
        result = result + "; Expires=" + tea(cookie.expires)
    }
    fr fr cookie.secure == based {
        result = result + "; Secure"
    }
    fr fr cookie.http_only == based {
        result = result + "; HttpOnly"
    }
    damn result
}

# ===============================
# HTTP Utilities
# ===============================

slay encode_form_data(data tea) tea {
    # Simple URL encoding (basic implementation)
    sus result tea = stringz.replace(data, " ", "%20")
    result = stringz.replace(result, "&", "%26")
    result = stringz.replace(result, "=", "%3D")
    damn result
}

slay decode_form_data(data tea) tea {
    # Simple URL decoding (basic implementation)
    sus result tea = stringz.replace(data, "%20", " ")
    result = stringz.replace(result, "%26", "&")
    result = stringz.replace(result, "%3D", "=")
    damn result
}

slay create_json_response(data tea) tea {
    damn '{"data":"' + data + '","status":"success","timestamp":"' + tea(timez.now_unix()) + '"}'
}

slay create_error_response(message tea, code normie) tea {
    damn '{"error":"' + message + '","code":' + tea(code) + ',"timestamp":"' + tea(timez.now_unix()) + '"}'
}

# ===============================
# HTTP Router Functions
# ===============================

struct HttpRoute {
    method tea,
    pattern tea,
    handler_name tea,
    middleware [5]tea,
    middleware_count normie
}

sus routes [50]HttpRoute
sus route_count normie = 0

slay add_route(method tea, pattern tea, handler_name tea) {
    fr fr route_count < 50 {
        routes[route_count].method = method
        routes[route_count].pattern = pattern
        routes[route_count].handler_name = handler_name
        routes[route_count].middleware_count = 0
        route_count = route_count + 1
        vibez.spill("Added route: " + method + " " + pattern)
    }
}

slay add_route_middleware(route_index normie, middleware_name tea) {
    fr fr route_index < route_count && routes[route_index].middleware_count < 5 {
        sus count normie = routes[route_index].middleware_count
        routes[route_index].middleware[count] = middleware_name
        routes[route_index].middleware_count = count + 1
    }
}

slay find_route(method tea, path tea) normie {
    sus i normie = 0
    bestie i = 0; i < route_count; i = i + 1 {
        fr fr routes[i].method == method && routes[i].pattern == path {
            damn i
        }
    }
    damn -1
}

# ===============================
# WebSocket Support (Basic)
# ===============================

struct WebSocketConnection {
    connected lit,
    client_id tea,
    last_ping thicc
}

sus websocket_connections [100]WebSocketConnection
sus ws_connection_count normie = 0

slay websocket_upgrade(request HttpRequest) lit {
    # Check for WebSocket upgrade headers
    sus connection_header tea = get_header(request.headers, "Connection")
    sus upgrade_header tea = get_header(request.headers, "Upgrade")
    
    fr fr stringz.to_lower(connection_header) == "upgrade" && 
          stringz.to_lower(upgrade_header) == "websocket" {
        vibez.spill("WebSocket upgrade request detected")
        damn based
    }
    
    damn cap
}

slay websocket_accept(request HttpRequest) HttpResponse {
    sus response HttpResponse = create_response(101)  # Switching Protocols
    response.status = "Switching Protocols"
    response.headers = add_header(response.headers, "Upgrade", "websocket")
    response.headers = add_header(response.headers, "Connection", "Upgrade")
    response.headers = add_header(response.headers, "Sec-WebSocket-Accept", "generated-key")
    damn response
}

# ===============================
# HTTP/2 Support (Basic Framework)
# ===============================

struct Http2Settings {
    header_table_size normie,
    enable_push lit,
    max_concurrent_streams normie,
    initial_window_size normie,
    max_frame_size normie,
    max_header_list_size normie
}

slay create_http2_settings() Http2Settings {
    sus settings Http2Settings
    settings.header_table_size = 4096
    settings.enable_push = based
    settings.max_concurrent_streams = 100
    settings.initial_window_size = 65535
    settings.max_frame_size = 16384
    settings.max_header_list_size = 8192
    damn settings
}

# ===============================
# Performance Monitoring
# ===============================

struct HttpMetrics {
    total_requests normie,
    successful_requests normie,
    failed_requests normie,
    average_response_time normie,
    active_connections normie,
    bytes_sent thicc,
    bytes_received thicc
}

sus http_metrics HttpMetrics

slay init_metrics() {
    http_metrics.total_requests = 0
    http_metrics.successful_requests = 0
    http_metrics.failed_requests = 0
    http_metrics.average_response_time = 0
    http_metrics.active_connections = 0
    http_metrics.bytes_sent = 0
    http_metrics.bytes_received = 0
}

slay record_request(success lit, response_time normie, bytes_sent normie, bytes_received normie) {
    http_metrics.total_requests = http_metrics.total_requests + 1
    fr fr success {
        http_metrics.successful_requests = http_metrics.successful_requests + 1
    } else {
        http_metrics.failed_requests = http_metrics.failed_requests + 1
    }
    http_metrics.bytes_sent = http_metrics.bytes_sent + bytes_sent
    http_metrics.bytes_received = http_metrics.bytes_received + bytes_received
    
    # Update average response time
    sus total_time normie = http_metrics.average_response_time * (http_metrics.total_requests - 1)
    http_metrics.average_response_time = (total_time + response_time) / http_metrics.total_requests
}

slay get_metrics() HttpMetrics {
    damn http_metrics
}

# ===============================
# Security Functions
# ===============================

slay sanitize_header_value(value tea) tea {
    # Remove CRLF injection attempts
    sus result tea = stringz.replace(value, "\r", "")
    result = stringz.replace(result, "\n", "")
    damn result
}

slay validate_method(method tea) lit {
    damn method == METHOD_GET || method == METHOD_POST || 
         method == METHOD_PUT || method == METHOD_DELETE || 
         method == METHOD_PATCH || method == METHOD_HEAD || 
         method == METHOD_OPTIONS
}

slay add_security_headers(response HttpResponse) HttpResponse {
    response.headers = add_header(response.headers, "X-Content-Type-Options", "nosniff")
    response.headers = add_header(response.headers, "X-Frame-Options", "DENY")
    response.headers = add_header(response.headers, "X-XSS-Protection", "1; mode=block")
    response.headers = add_header(response.headers, "Strict-Transport-Security", "max-age=31536000")
    damn response
}

# ===============================
# Public API Functions
# ===============================

# Client API
slay http_get(url tea) HttpResponse {
    sus client HttpClient = create_client()
    damn client_get(client, url)
}

slay http_post(url tea, body tea) HttpResponse {
    sus client HttpClient = create_client()
    damn client_post(client, url, body)
}

slay http_post_json(url tea, json_body tea) HttpResponse {
    sus client HttpClient = create_client()
    damn client_post_json(client, url, json_body)
}

# Server API
slay listen_and_serve(addr tea, port normie) lit {
    sus server HttpServer = create_server(addr, port)
    damn server_start(server)
}

slay handle_func(pattern tea, handler_name tea) {
    server_handle_func(pattern, handler_name)
}

slay handle_get(pattern tea, handler_name tea) {
    server_handle_get(pattern, handler_name)
}

slay handle_post(pattern tea, handler_name tea) {
    server_handle_post(pattern, handler_name)
}

# Utility API
slay status_text(code normie) tea {
    damn get_status_text(code)
}

slay new_request(method tea, url tea) HttpRequest {
    damn create_request(method, url)
}

slay new_response(status_code normie) HttpResponse {
    damn create_response(status_code)
}

# Initialize metrics on module load
init_metrics()
