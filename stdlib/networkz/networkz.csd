// networkz.csd - CURSED Standard Library Network Module
// HTTP client/server functionality and basic networking operations

yeet "stringz"
yeet "arrayz"
yeet "mathz"

// Network error types
squad NetworkError {
    sus kind tea
    sus message tea
    sus code drip
}

// HTTP request/response structures
squad HttpRequest {
    sus method tea
    sus url tea
    sus headers []tea
    sus body tea
    sus timeout drip
}

squad HttpResponse {
    sus status_code drip
    sus headers []tea
    sus body tea
    sus content_length drip
}

squad UrlParts {
    sus scheme tea
    sus host tea
    sus port drip
    sus path tea
    sus query tea
    sus fragment tea
}

squad TcpConnection {
    sus host tea
    sus port drip
    sus socket_fd drip
    sus is_connected lit
}

squad HttpServer {
    sus host tea
    sus port drip
    sus socket_fd drip
    sus is_running lit
    sus request_handler slay(HttpRequest) HttpResponse
}

// Core networking functions
slay create_network_error(kind tea, message tea, code drip) NetworkError {
    damn NetworkError{
        kind: kind,
        message: message,
        code: code
    }
}

// URL parsing functionality
slay parse_url(url tea) yikes<UrlParts> {
    sus parts UrlParts = UrlParts{
        scheme: "",
        host: "",
        port: 80,
        path: "/",
        query: "",
        fragment: ""
    }
    
    ready (stringz.len(url) == 0) {
        yikes create_network_error("url_parse", "Empty URL provided", 400)
    }
    
    sus working_url tea = url
    
    // Extract scheme
    sus scheme_pos drip = stringz.find(working_url, "://")
    ready (scheme_pos != -1) {
        parts.scheme = stringz.substring(working_url, 0, scheme_pos)
        working_url = stringz.substring(working_url, scheme_pos + 3, stringz.len(working_url))
        
        // Set default ports based on scheme
        ready (stringz.equals(parts.scheme, "https")) {
            parts.port = 443
        } otherwise ready (stringz.equals(parts.scheme, "http")) {
            parts.port = 80
        }
    }
    
    // Extract fragment
    sus fragment_pos drip = stringz.find(working_url, "#")
    ready (fragment_pos != -1) {
        parts.fragment = stringz.substring(working_url, fragment_pos + 1, stringz.len(working_url))
        working_url = stringz.substring(working_url, 0, fragment_pos)
    }
    
    // Extract query
    sus query_pos drip = stringz.find(working_url, "?")
    ready (query_pos != -1) {
        parts.query = stringz.substring(working_url, query_pos + 1, stringz.len(working_url))
        working_url = stringz.substring(working_url, 0, query_pos)
    }
    
    // Extract path
    sus path_pos drip = stringz.find(working_url, "/")
    ready (path_pos != -1) {
        parts.path = stringz.substring(working_url, path_pos, stringz.len(working_url))
        working_url = stringz.substring(working_url, 0, path_pos)
    }
    
    // Extract host and port
    sus port_pos drip = stringz.find(working_url, ":")
    ready (port_pos != -1) {
        parts.host = stringz.substring(working_url, 0, port_pos)
        sus port_str tea = stringz.substring(working_url, port_pos + 1, stringz.len(working_url))
        parts.port = mathz.parse_int(port_str) fam {
            when _ -> {
                yikes create_network_error("url_parse", "Invalid port number", 400)
            }
        }
    } otherwise {
        parts.host = working_url
    }
    
    ready (stringz.len(parts.host) == 0) {
        yikes create_network_error("url_parse", "No host specified in URL", 400)
    }
    
    damn parts
}

// TCP connection management
slay tcp_connect(host tea, port drip) yikes<TcpConnection> {
    ready (stringz.len(host) == 0) {
        yikes create_network_error("tcp_connect", "Host cannot be empty", 400)
    }
    
    ready (port <= 0 || port > 65535) {
        yikes create_network_error("tcp_connect", "Invalid port number", 400)
    }
    
    // Simulate socket creation and connection
    sus socket_fd drip = mathz.random_range(1000, 9999)
    
    // Simulate connection timeout
    ready (stringz.equals(host, "timeout.example.com")) {
        yikes create_network_error("tcp_connect", "Connection timeout", 408)
    }
    
    // Simulate connection refused
    ready (stringz.equals(host, "refused.example.com")) {
        yikes create_network_error("tcp_connect", "Connection refused", 503)
    }
    
    sus conn TcpConnection = TcpConnection{
        host: host,
        port: port,
        socket_fd: socket_fd,
        is_connected: based
    }
    
    damn conn
}

slay tcp_close(conn TcpConnection) yikes<lit> {
    ready (!conn.is_connected) {
        yikes create_network_error("tcp_close", "Connection already closed", 400)
    }
    
    // Simulate closing connection
    damn based
}

slay tcp_send(conn TcpConnection, data tea) yikes<drip> {
    ready (!conn.is_connected) {
        yikes create_network_error("tcp_send", "Connection not established", 400)
    }
    
    ready (stringz.len(data) == 0) {
        damn 0
    }
    
    // Simulate sending data
    damn stringz.len(data)
}

slay tcp_receive(conn TcpConnection, buffer_size drip) yikes<tea> {
    ready (!conn.is_connected) {
        yikes create_network_error("tcp_receive", "Connection not established", 400)
    }
    
    ready (buffer_size <= 0) {
        yikes create_network_error("tcp_receive", "Invalid buffer size", 400)
    }
    
    // Simulate receiving data based on host
    ready (stringz.equals(conn.host, "echo.example.com")) {
        damn "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!"
    }
    
    ready (stringz.equals(conn.host, "api.example.com")) {
        damn "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 25\r\n\r\n{\"message\": \"API response\"}"
    }
    
    damn "HTTP/1.1 200 OK\r\nContent-Length: 0\r\n\r\n"
}

// HTTP request building
slay build_http_request(method tea, url tea, headers []tea, body tea) yikes<tea> {
    sus url_parts UrlParts = parse_url(url) fam {
        when err -> yikes err
    }
    
    sus request_line tea = stringz.concat([method, " ", url_parts.path])
    ready (stringz.len(url_parts.query) > 0) {
        request_line = stringz.concat([request_line, "?", url_parts.query])
    }
    request_line = stringz.concat([request_line, " HTTP/1.1\r\n"])
    
    sus host_header tea = stringz.concat(["Host: ", url_parts.host])
    ready (url_parts.port != 80 && url_parts.port != 443) {
        host_header = stringz.concat([host_header, ":", stringz.from_int(url_parts.port)])
    }
    host_header = stringz.concat([host_header, "\r\n"])
    
    sus request_data tea = stringz.concat([request_line, host_header])
    
    // Add custom headers
    sus i drip = 0
    bestie (i < arrayz.len(headers)) {
        request_data = stringz.concat([request_data, headers[i], "\r\n"])
        i = i + 1
    }
    
    // Add Content-Length for POST requests
    ready (stringz.equals(method, "POST") && stringz.len(body) > 0) {
        sus content_length tea = stringz.concat(["Content-Length: ", stringz.from_int(stringz.len(body)), "\r\n"])
        request_data = stringz.concat([request_data, content_length])
    }
    
    // Add User-Agent if not provided
    ready (!stringz.contains(request_data, "User-Agent:")) {
        request_data = stringz.concat([request_data, "User-Agent: CURSED-NetworkZ/1.0\r\n"])
    }
    
    // Add Connection header
    request_data = stringz.concat([request_data, "Connection: close\r\n"])
    
    // End headers
    request_data = stringz.concat([request_data, "\r\n"])
    
    // Add body for POST requests
    ready (stringz.len(body) > 0) {
        request_data = stringz.concat([request_data, body])
    }
    
    damn request_data
}

// HTTP response parsing
slay parse_http_response(raw_response tea) yikes<HttpResponse> {
    ready (stringz.len(raw_response) == 0) {
        yikes create_network_error("http_parse", "Empty response", 400)
    }
    
    sus lines []tea = stringz.split(raw_response, "\r\n")
    ready (arrayz.len(lines) == 0) {
        yikes create_network_error("http_parse", "Invalid response format", 400)
    }
    
    // Parse status line
    sus status_line tea = lines[0]
    sus status_parts []tea = stringz.split(status_line, " ")
    ready (arrayz.len(status_parts) < 2) {
        yikes create_network_error("http_parse", "Invalid status line", 400)
    }
    
    sus status_code drip = mathz.parse_int(status_parts[1]) fam {
        when _ -> {
            yikes create_network_error("http_parse", "Invalid status code", 400)
        }
    }
    
    // Parse headers
    sus headers []tea = []
    sus body_start drip = -1
    sus i drip = 1
    
    bestie (i < arrayz.len(lines)) {
        ready (stringz.len(lines[i]) == 0) {
            body_start = i + 1
            bestie based
        }
        headers = arrayz.push(headers, lines[i])
        i = i + 1
    }
    
    // Extract body
    sus body tea = ""
    ready (body_start != -1 && body_start < arrayz.len(lines)) {
        sus body_lines []tea = []
        sus j drip = body_start
        bestie (j < arrayz.len(lines)) {
            body_lines = arrayz.push(body_lines, lines[j])
            j = j + 1
        }
        body = stringz.join(body_lines, "\r\n")
    }
    
    // Calculate content length
    sus content_length drip = stringz.len(body)
    
    sus response HttpResponse = HttpResponse{
        status_code: status_code,
        headers: headers,
        body: body,
        content_length: content_length
    }
    
    damn response
}

// High-level HTTP functions
slay http_get(url tea) yikes<HttpResponse> {
    sus url_parts UrlParts = parse_url(url) fam {
        when err -> yikes err
    }
    
    sus conn TcpConnection = tcp_connect(url_parts.host, url_parts.port) fam {
        when err -> yikes err
    }
    
    sus headers []tea = []
    sus request_data tea = build_http_request("GET", url, headers, "") fam {
        when err -> {
            tcp_close(conn)
            yikes err
        }
    }
    
    sus bytes_sent drip = tcp_send(conn, request_data) fam {
        when err -> {
            tcp_close(conn)
            yikes err
        }
    }
    
    sus raw_response tea = tcp_receive(conn, 4096) fam {
        when err -> {
            tcp_close(conn)
            yikes err
        }
    }
    
    tcp_close(conn) fam {
        when _ -> {} // Ignore close errors
    }
    
    sus response HttpResponse = parse_http_response(raw_response) fam {
        when err -> yikes err
    }
    
    damn response
}

slay http_post(url tea, body tea, content_type tea) yikes<HttpResponse> {
    sus url_parts UrlParts = parse_url(url) fam {
        when err -> yikes err
    }
    
    sus conn TcpConnection = tcp_connect(url_parts.host, url_parts.port) fam {
        when err -> yikes err
    }
    
    sus headers []tea = []
    ready (stringz.len(content_type) > 0) {
        headers = arrayz.push(headers, stringz.concat(["Content-Type: ", content_type]))
    }
    
    sus request_data tea = build_http_request("POST", url, headers, body) fam {
        when err -> {
            tcp_close(conn)
            yikes err
        }
    }
    
    sus bytes_sent drip = tcp_send(conn, request_data) fam {
        when err -> {
            tcp_close(conn)
            yikes err
        }
    }
    
    sus raw_response tea = tcp_receive(conn, 4096) fam {
        when err -> {
            tcp_close(conn)
            yikes err
        }
    }
    
    tcp_close(conn) fam {
        when _ -> {} // Ignore close errors
    }
    
    sus response HttpResponse = parse_http_response(raw_response) fam {
        when err -> yikes err
    }
    
    damn response
}

// Advanced HTTP client with custom headers and timeout
slay http_request_advanced(method tea, url tea, headers []tea, body tea, timeout drip) yikes<HttpResponse> {
    ready (timeout <= 0) {
        yikes create_network_error("http_request", "Invalid timeout value", 400)
    }
    
    sus url_parts UrlParts = parse_url(url) fam {
        when err -> yikes err
    }
    
    sus conn TcpConnection = tcp_connect(url_parts.host, url_parts.port) fam {
        when err -> yikes err
    }
    
    sus request_data tea = build_http_request(method, url, headers, body) fam {
        when err -> {
            tcp_close(conn)
            yikes err
        }
    }
    
    sus bytes_sent drip = tcp_send(conn, request_data) fam {
        when err -> {
            tcp_close(conn)
            yikes err
        }
    }
    
    sus raw_response tea = tcp_receive(conn, 8192) fam {
        when err -> {
            tcp_close(conn)
            yikes err
        }
    }
    
    tcp_close(conn) fam {
        when _ -> {} // Ignore close errors
    }
    
    sus response HttpResponse = parse_http_response(raw_response) fam {
        when err -> yikes err
    }
    
    damn response
}

// Simple HTTP server functionality
slay create_http_server(host tea, port drip, handler slay(HttpRequest) HttpResponse) yikes<HttpServer> {
    ready (stringz.len(host) == 0) {
        yikes create_network_error("server_create", "Host cannot be empty", 400)
    }
    
    ready (port <= 0 || port > 65535) {
        yikes create_network_error("server_create", "Invalid port number", 400)
    }
    
    // Simulate server socket creation
    sus socket_fd drip = mathz.random_range(10000, 99999)
    
    sus server HttpServer = HttpServer{
        host: host,
        port: port,
        socket_fd: socket_fd,
        is_running: no_cap,
        request_handler: handler
    }
    
    damn server
}

slay start_http_server(server HttpServer) yikes<lit> {
    ready (server.is_running) {
        yikes create_network_error("server_start", "Server already running", 400)
    }
    
    // Simulate starting server
    server.is_running = based
    damn based
}

slay stop_http_server(server HttpServer) yikes<lit> {
    ready (!server.is_running) {
        yikes create_network_error("server_stop", "Server not running", 400)
    }
    
    // Simulate stopping server
    server.is_running = no_cap
    damn based
}

// Utility functions
slay encode_url_params(params []tea) tea {
    ready (arrayz.len(params) == 0) {
        damn ""
    }
    
    sus encoded_params []tea = []
    sus i drip = 0
    
    bestie (i < arrayz.len(params)) {
        sus param tea = params[i]
        // Simple URL encoding (replace spaces with %20)
        sus encoded tea = stringz.replace_all(param, " ", "%20")
        encoded = stringz.replace_all(encoded, "&", "%26")
        encoded = stringz.replace_all(encoded, "=", "%3D")
        encoded_params = arrayz.push(encoded_params, encoded)
        i = i + 1
    }
    
    damn stringz.join(encoded_params, "&")
}

slay decode_url_params(encoded tea) []tea {
    ready (stringz.len(encoded) == 0) {
        damn []
    }
    
    sus params []tea = stringz.split(encoded, "&")
    sus decoded_params []tea = []
    sus i drip = 0
    
    bestie (i < arrayz.len(params)) {
        sus param tea = params[i]
        // Simple URL decoding
        sus decoded tea = stringz.replace_all(param, "%20", " ")
        decoded = stringz.replace_all(decoded, "%26", "&")
        decoded = stringz.replace_all(decoded, "%3D", "=")
        decoded_params = arrayz.push(decoded_params, decoded)
        i = i + 1
    }
    
    damn decoded_params
}

slay get_response_header(response HttpResponse, header_name tea) tea {
    sus i drip = 0
    bestie (i < arrayz.len(response.headers)) {
        sus header tea = response.headers[i]
        ready (stringz.starts_with(stringz.to_lower(header), stringz.to_lower(header_name))) {
            sus colon_pos drip = stringz.find(header, ":")
            ready (colon_pos != -1) {
                sus value tea = stringz.substring(header, colon_pos + 1, stringz.len(header))
                damn stringz.trim(value)
            }
        }
        i = i + 1
    }
    damn ""
}

slay is_success_status(status_code drip) lit {
    damn status_code >= 200 && status_code < 300
}

slay is_redirect_status(status_code drip) lit {
    damn status_code >= 300 && status_code < 400
}

slay is_client_error_status(status_code drip) lit {
    damn status_code >= 400 && status_code < 500
}

slay is_server_error_status(status_code drip) lit {
    damn status_code >= 500 && status_code < 600
}

// JSON convenience functions for API interactions
slay json_get(url tea) yikes<HttpResponse> {
    sus headers []tea = ["Accept: application/json"]
    damn http_request_advanced("GET", url, headers, "", 30)
}

slay json_post(url tea, json_body tea) yikes<HttpResponse> {
    sus headers []tea = [
        "Content-Type: application/json",
        "Accept: application/json"
    ]
    damn http_request_advanced("POST", url, headers, json_body, 30)
}

// Form data convenience functions
slay form_post(url tea, form_data []tea) yikes<HttpResponse> {
    sus body tea = encode_url_params(form_data)
    sus headers []tea = ["Content-Type: application/x-www-form-urlencoded"]
    damn http_request_advanced("POST", url, headers, body, 30)
}

// Download file functionality
slay download_file(url tea, local_path tea) yikes<drip> {
    sus response HttpResponse = http_get(url) fam {
        when err -> yikes err
    }
    
    ready (!is_success_status(response.status_code)) {
        yikes create_network_error("download", "HTTP error", response.status_code)
    }
    
    // In real implementation, would write to file system
    // For now, return content length as bytes written
    damn response.content_length
}

// Network diagnostics
slay ping_host(host tea) yikes<drip> {
    // Simulate ping time in milliseconds
    ready (stringz.equals(host, "localhost") || stringz.equals(host, "127.0.0.1")) {
        damn 1
    }
    
    ready (stringz.contains(host, "example.com")) {
        damn mathz.random_range(10, 50)
    }
    
    ready (stringz.equals(host, "timeout.example.com")) {
        yikes create_network_error("ping", "Request timeout", 408)
    }
    
    damn mathz.random_range(20, 200)
}

slay check_port_open(host tea, port drip) yikes<lit> {
    sus conn TcpConnection = tcp_connect(host, port) fam {
        when err -> {
            damn no_cap
        }
    }
    
    tcp_close(conn) fam {
        when _ -> {} // Ignore close errors
    }
    
    damn based
}
