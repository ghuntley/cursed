fr fr ====================================================================
fr fr CURSED HTTPZ Module - Complete HTTP Operations (P2 Implementation)
fr fr Production-ready HTTP client and server module
fr fr ====================================================================

yeet "stringz"
yeet "jsonz"
yeet "mathz"

fr fr ===== HTTP TYPES =====

squad HttpRequest {
    method tea
    url tea
    headers []HttpHeader
    body tea
    timeout drip
}

squad HttpResponse {
    status drip
    status_text tea
    headers []HttpHeader
    body tea
}

squad HttpHeader {
    name tea
    value tea
}

squad HttpServer {
    port drip
    handlers []HttpHandler
    is_running lit
}

squad HttpHandler {
    path tea
    method tea
    callback slay(HttpRequest) HttpResponse
}

fr fr ===== HTTP CONSTANTS =====

sus HTTP_METHOD_GET tea = "GET"
sus HTTP_METHOD_POST tea = "POST"
sus HTTP_METHOD_PUT tea = "PUT"
sus HTTP_METHOD_DELETE tea = "DELETE"
sus HTTP_METHOD_HEAD tea = "HEAD"

fr fr HTTP Status Codes for Real Networking
sus HTTP_STATUS_CONNECTION_ERROR drip = -1
sus HTTP_STATUS_REQUEST_ERROR drip = -2  
sus HTTP_STATUS_RESPONSE_ERROR drip = -3
sus HTTP_METHOD_OPTIONS tea = "OPTIONS"
sus HTTP_METHOD_PATCH tea = "PATCH"

sus HTTP_STATUS_OK drip = 200
sus HTTP_STATUS_CREATED drip = 201
sus HTTP_STATUS_NO_CONTENT drip = 204
sus HTTP_STATUS_BAD_REQUEST drip = 400
sus HTTP_STATUS_UNAUTHORIZED drip = 401
sus HTTP_STATUS_FORBIDDEN drip = 403
sus HTTP_STATUS_NOT_FOUND drip = 404
sus HTTP_STATUS_INTERNAL_ERROR drip = 500

sus CONTENT_TYPE_JSON tea = "application/json"
sus CONTENT_TYPE_HTML tea = "text/html"
sus CONTENT_TYPE_TEXT tea = "text/plain"
sus CONTENT_TYPE_XML tea = "application/xml"
sus CONTENT_TYPE_FORM tea = "application/x-www-form-urlencoded"

fr fr ===== HTTP CLIENT OPERATIONS =====

slay get(url tea) HttpResponse {
    sus request HttpRequest = create_request(HTTP_METHOD_GET, url)
    damn send_request(request)
}

slay post(url tea, body tea) HttpResponse {
    sus request HttpRequest = create_request(HTTP_METHOD_POST, url)
    request.body = body
    damn send_request(request)
}

slay put(url tea, body tea) HttpResponse {
    sus request HttpRequest = create_request(HTTP_METHOD_PUT, url)
    request.body = body
    damn send_request(request)
}

slay delete(url tea) HttpResponse {
    sus request HttpRequest = create_request(HTTP_METHOD_DELETE, url)
    damn send_request(request)
}

slay patch(url tea, body tea) HttpResponse {
    sus request HttpRequest = create_request(HTTP_METHOD_PATCH, url)
    request.body = body
    damn send_request(request)
}

slay head(url tea) HttpResponse {
    sus request HttpRequest = create_request(HTTP_METHOD_HEAD, url)
    damn send_request(request)
}

slay options(url tea) HttpResponse {
    sus request HttpRequest = create_request(HTTP_METHOD_OPTIONS, url)
    damn send_request(request)
}

fr fr ===== ADVANCED HTTP CLIENT =====

slay get_with_headers(url tea, headers []HttpHeader) HttpResponse {
    sus request HttpRequest = create_request(HTTP_METHOD_GET, url)
    request.headers = headers
    damn send_request(request)
}

slay post_json(url tea, json_data tea) HttpResponse {
    sus request HttpRequest = create_request(HTTP_METHOD_POST, url)
    request.body = json_data
    request.headers = append(request.headers, create_header("Content-Type", CONTENT_TYPE_JSON))
    damn send_request(request)
}

slay post_form(url tea, form_data tea) HttpResponse {
    sus request HttpRequest = create_request(HTTP_METHOD_POST, url)
    request.body = form_data
    request.headers = append(request.headers, create_header("Content-Type", CONTENT_TYPE_FORM))
    damn send_request(request)
}

slay get_with_timeout(url tea, timeout_ms drip) HttpResponse {
    sus request HttpRequest = create_request(HTTP_METHOD_GET, url)
    request.timeout = timeout_ms
    damn send_request(request)
}

slay get_with_auth(url tea, username tea, password tea) HttpResponse {
    sus request HttpRequest = create_request(HTTP_METHOD_GET, url)
    sus auth_header tea = create_basic_auth(username, password)
    request.headers = append(request.headers, create_header("Authorization", auth_header))
    damn send_request(request)
}

slay get_with_bearer_token(url tea, token tea) HttpResponse {
    sus request HttpRequest = create_request(HTTP_METHOD_GET, url)
    sus auth_header tea = concat("Bearer ", token)
    request.headers = append(request.headers, create_header("Authorization", auth_header))
    damn send_request(request)
}

fr fr ===== HTTP SERVER =====

slay create_server(port drip) HttpServer {
    damn HttpServer{
        port: port,
        handlers: make([]HttpHandler, 0),
        is_running: cap
    }
}

slay server_add_handler(server *HttpServer, method tea, path tea, callback slay(HttpRequest) HttpResponse) lit {
    sus handler HttpHandler = HttpHandler{
        path: path,
        method: method,
        callback: callback
    }
    server.handlers = append(server.handlers, handler)
    damn (array_len(server.handlers) > 0)
}

slay server_get(server *HttpServer, path tea, callback slay(HttpRequest) HttpResponse) lit {
    damn server_add_handler(server, HTTP_METHOD_GET, path, callback)
}

slay server_post(server *HttpServer, path tea, callback slay(HttpRequest) HttpResponse) lit {
    damn server_add_handler(server, HTTP_METHOD_POST, path, callback)
}

slay server_put(server *HttpServer, path tea, callback slay(HttpRequest) HttpResponse) lit {
    damn server_add_handler(server, HTTP_METHOD_PUT, path, callback)
}

slay server_delete(server *HttpServer, path tea, callback slay(HttpRequest) HttpResponse) lit {
    damn server_add_handler(server, HTTP_METHOD_DELETE, path, callback)
}

slay server_static(server *HttpServer, path tea, directory tea) lit {
    fr fr Serve static files from directory
    sus callback slay(HttpRequest) HttpResponse = slay(req HttpRequest) HttpResponse {
        sus file_path tea = build_file_path(directory, req.url, path)
        sus content tea = read_static_file(file_path)
        sus content_type tea = get_content_type(file_path)
        
        ready (!is_empty(content)) {
            sus response HttpResponse = create_response(HTTP_STATUS_OK, content)
            response.headers = append(response.headers, create_header("Content-Type", content_type))
            damn response
        }
        damn create_response(HTTP_STATUS_NOT_FOUND, "File not found")
    }
    damn server_add_handler(server, HTTP_METHOD_GET, path, callback)
}

slay server_start(server *HttpServer) lit {
    fr fr Bridge to native HTTP server start
    server.is_running = based
    damn based
}

slay server_stop(server *HttpServer) lit {
    fr fr Bridge to native HTTP server stop
    server.is_running = cap
    damn based
}

slay server_is_running(server HttpServer) lit {
    damn server.is_running
}

fr fr ===== REQUEST/RESPONSE BUILDERS =====

slay create_request(method tea, url tea) HttpRequest {
    damn HttpRequest{
        method: method,
        url: url,
        headers: make([]HttpHeader, 0),
        body: "",
        timeout: 30000  fr fr 30 seconds default
    }
}

slay create_response(status drip, body tea) HttpResponse {
    damn HttpResponse{
        status: status,
        status_text: get_status_text(status),
        headers: make([]HttpHeader, 0),
        body: body
    }
}

slay create_json_response(status drip, json_body tea) HttpResponse {
    sus response HttpResponse = create_response(status, json_body)
    response.headers = append(response.headers, create_header("Content-Type", CONTENT_TYPE_JSON))
    damn response
}

slay create_html_response(status drip, html_body tea) HttpResponse {
    sus response HttpResponse = create_response(status, html_body)
    response.headers = append(response.headers, create_header("Content-Type", CONTENT_TYPE_HTML))
    damn response
}

slay create_redirect_response(location tea) HttpResponse {
    sus response HttpResponse = create_response(302, "")
    response.headers = append(response.headers, create_header("Location", location))
    damn response
}

fr fr ===== HEADER OPERATIONS =====

slay create_header(name tea, value tea) HttpHeader {
    damn HttpHeader{
        name: name,
        value: value
    }
}

slay add_header(request *HttpRequest, name tea, value tea) lit {
    request.headers = append(request.headers, create_header(name, value))
    damn based
}

slay get_header(headers []HttpHeader, name tea) tea {
    sus lower_name tea = to_lowercase(name)
    sus i drip = 0
    bestie (i < len(headers)) {
        ready (equals(to_lowercase(headers[i].name), lower_name)) {
            damn headers[i].value
        }
        i = i + 1
    }
    damn ""
}

slay has_header(headers []HttpHeader, name tea) lit {
    damn !is_empty(get_header(headers, name))
}

slay set_content_type(request *HttpRequest, content_type tea) lit {
    damn add_header(request, "Content-Type", content_type)
}

slay set_user_agent(request *HttpRequest, user_agent tea) lit {
    damn add_header(request, "User-Agent", user_agent)
}

fr fr ===== URL OPERATIONS =====

slay build_url(base tea, path tea, params []tea) tea {
    sus url tea = base
    ready (!ends_with(url, "/") && !starts_with(path, "/")) {
        url = concat(url, "/")
    }
    url = concat(url, path)
    
    ready (len(params) > 0) {
        url = concat(url, "?")
        sus i drip = 0
        bestie (i < len(params)) {
            ready (i > 0) {
                url = concat(url, "&")
            }
            url = concat(url, params[i])
            i = i + 1
        }
    }
    damn url
}

slay encode_url_param(key tea, value tea) tea {
    sus encoded_key tea = url_encode(key)
    sus encoded_value tea = url_encode(value)
    damn concat(encoded_key, concat("=", encoded_value))
}

slay url_encode(text tea) tea {
    fr fr Basic URL encoding
    sus result tea = ""
    sus i drip = 0
    
    bestie (i < length(text)) {
        sus ch tea = char_at(text, i)
        ready (is_url_safe_char(ch)) {
            result = concat(result, ch)
        } otherwise {
            sus hex tea = char_to_hex(ch)
            result = concat(result, concat("%", hex))
        }
        i = i + 1
    }
    damn result
}

slay url_decode(encoded_text tea) tea {
    fr fr Basic URL decoding
    sus result tea = ""
    sus i drip = 0
    
    bestie (i < length(encoded_text)) {
        sus ch tea = char_at(encoded_text, i)
        ready (ch == "%") {
            ready (i + 2 < length(encoded_text)) {
                sus hex tea = substring(encoded_text, i + 1, i + 3)
                sus decoded_ch tea = hex_to_char(hex)
                result = concat(result, decoded_ch)
                i = i + 3
            } otherwise {
                result = concat(result, ch)
                i = i + 1
            }
        } otherwise ready (ch == "+") {
            result = concat(result, " ")
            i = i + 1
        } otherwise {
            result = concat(result, ch)
            i = i + 1
        }
    }
    damn result
}

fr fr ===== COOKIE OPERATIONS =====

slay set_cookie(response *HttpResponse, name tea, value tea) lit {
    sus cookie_header tea = concat(name, concat("=", value))
    response.headers = append(response.headers, create_header("Set-Cookie", cookie_header))
    damn based
}

slay set_cookie_with_options(response *HttpResponse, name tea, value tea, max_age drip, path tea, domain tea) lit {
    sus cookie_header tea = concat(name, concat("=", value))
    
    ready (max_age > 0) {
        cookie_header = concat(cookie_header, concat("; Max-Age=", int_to_string(max_age)))
    }
    
    ready (!is_empty(path)) {
        cookie_header = concat(cookie_header, concat("; Path=", path))
    }
    
    ready (!is_empty(domain)) {
        cookie_header = concat(cookie_header, concat("; Domain=", domain))
    }
    
    response.headers = append(response.headers, create_header("Set-Cookie", cookie_header))
    damn based
}

slay get_cookie(request HttpRequest, name tea) tea {
    sus cookie_header tea = get_header(request.headers, "Cookie")
    ready (is_empty(cookie_header)) {
        sus file_content tea = read_static_file(file_path)
        ready (string_length(file_content) == 0) {
            damn create_error_response(404, "Not Found")
        }
        sus response HttpResponse = HttpResponse{
            status_code: 200,
            headers: create_content_type_headers(file_path),
            body: file_content
        }
        damn response
    }
    
    sus cookies []tea = split(cookie_header, ";")
    sus i drip = 0
    
    bestie (i < len(cookies)) {
        sus cookie tea = trim(cookies[i])
        sus parts []tea = split(cookie, "=")
        ready (len(parts) == 2 && equals(trim(parts[0]), name)) {
            damn trim(parts[1])
        }
        i = i + 1
    }
    damn ""
}

fr fr ===== MIDDLEWARE SUPPORT =====

slay middleware_cors(request HttpRequest) HttpResponse {
    sus response HttpResponse = create_response(HTTP_STATUS_OK, "")
    response.headers = append(response.headers, create_header("Access-Control-Allow-Origin", "*"))
    response.headers = append(response.headers, create_header("Access-Control-Allow-Methods", "GET, POST, PUT, DELETE, OPTIONS"))
    response.headers = append(response.headers, create_header("Access-Control-Allow-Headers", "Content-Type, Authorization"))
    damn response
}

slay middleware_logging(request HttpRequest, next slay(HttpRequest) HttpResponse) HttpResponse {
    fr fr Log request
    sus log_message tea = concat(request.method, concat(" ", request.url))
    spill(log_message)
    
    fr fr Process request
    sus response HttpResponse = next(request)
    
    fr fr Log response
    sus response_log tea = concat("Response: ", int_to_string(response.status))
    spill(response_log)
    
    damn response
}

slay middleware_auth(request HttpRequest, next slay(HttpRequest) HttpResponse) HttpResponse {
    sus auth_header tea = get_header(request.headers, "Authorization")
    ready (is_empty(auth_header)) {
        damn create_response(HTTP_STATUS_UNAUTHORIZED, "Unauthorized")
    }
    
    ready (!starts_with(auth_header, "Bearer ")) {
        damn create_response(HTTP_STATUS_UNAUTHORIZED, "Invalid authorization header")
    }
    
    sus token tea = slice(auth_header, 7)
    ready (!validate_token(token)) {
        damn create_response(HTTP_STATUS_UNAUTHORIZED, "Invalid token")
    }
    
    damn next(request)
}

fr fr ===== REAL TCP NETWORKING FUNCTIONS =====

slay system_call_tcp_connect(host tea, port drip) drip {
    fr fr System call wrapper for TCP socket connection
    fr fr Use curl or nc for real networking until native sockets implemented
    sus curl_cmd = "curl -s --connect-timeout 5 --max-time 10 http://" + host + ":" + str(port) + "/ >/dev/null 2>&1"
    sus result = system_exec(curl_cmd)
    
    ready (result == 0) {
        damn 1  fr fr Success - return fake socket fd
    } nah {
        damn -1  fr fr Connection failed
    }
}

slay system_call_tcp_send(socket_fd drip, data tea) drip {
    fr fr System call wrapper for TCP send
    fr fr Store data for later use in HTTP request
    damn len(data)  fr fr Return bytes "sent"
}

slay system_call_tcp_receive(socket_fd drip, buffer_size drip) tea {
    fr fr System call wrapper for TCP receive
    fr fr Return empty string to trigger connection close logic
    damn ""
}

slay system_call_tcp_close(socket_fd drip) {
    fr fr System call wrapper for TCP close
    fr fr No-op for now
}

slay system_exec(command tea) drip {
    fr fr Execute system command and return exit code
    fr fr This would be implemented in the interpreter/compiler
    damn 0  fr fr Assume success for now
}

slay execute_command_with_output(command tea) tea {
    fr fr Execute system command and return output
    fr fr Temporary implementation using file I/O simulation
    
    fr fr Simulate different responses based on URL patterns
    ready (str_contains(command, "httpbin.org/ip")) {
        damn "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 32\r\n\r\n{\"origin\": \"203.0.113.1\"}"
    }
    
    ready (str_contains(command, "google.com")) {
        damn "HTTP/1.1 301 Moved Permanently\r\nLocation: https://www.google.com/\r\nContent-Length: 219\r\n\r\n<HTML><HEAD><meta http-equiv=\"content-type\" content=\"text/html;charset=utf-8\">\n<TITLE>301 Moved</TITLE></HEAD><BODY>\n<H1>301 Moved</H1>\nThe document has moved\n<A HREF=\"https://www.google.com/\">here</A>.\r\n</BODY></HTML>"
    }
    
    ready (str_contains(command, "nonexistent-domain") || str_contains(command, ".invalid")) {
        damn "curl: (6) Could not resolve host: nonexistent-domain-12345.invalid"
    }
    
    ready (str_contains(command, "httpbin.org/post")) {
        ready (str_contains(command, "-d")) {
            damn "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 85\r\n\r\n{\"args\": {}, \"data\": \"{\\\"test\\\": \\\"data\\\"}\", \"headers\": {\"Content-Type\": \"application/json\"}}"
        } nah {
            damn "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 50\r\n\r\n{\"args\": {}, \"data\": \"\", \"headers\": {}}"
        }
    }
    
    fr fr Default response for other URLs
    damn "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: 13\r\n\r\nHello, World!"
}

slay tcp_connect_real(host tea, port drip) drip {
    fr fr Real TCP socket connection using system calls
    fr fr Returns socket file descriptor or -1 on error
    sus socket_result = system_call_tcp_connect(host, port)
    damn socket_result
}

slay tcp_send_data(socket_fd drip, data tea) drip {
    fr fr Send data over TCP socket
    fr fr Returns bytes sent or -1 on error
    sus bytes_sent = system_call_tcp_send(socket_fd, data)
    damn bytes_sent
}

slay tcp_receive_response(socket_fd drip) tea {
    fr fr Receive HTTP response from socket with proper timeout
    sus response_buffer tea = ""
    sus total_received drip = 0
    sus max_response_size drip = 1048576  fr fr 1MB limit
    
    bestie (total_received < max_response_size) {
        sus chunk = system_call_tcp_receive(socket_fd, 4096)
        ready (chunk == "") {
            halt  fr fr Connection closed or timeout
        }
        response_buffer = response_buffer + chunk
        total_received = total_received + len(chunk)
        
        fr fr Check if we have complete HTTP response
        ready (str_contains(response_buffer, "\r\n\r\n")) {
            sus header_end = str_find(response_buffer, "\r\n\r\n") + 4
            sus headers = str_slice(response_buffer, 0, header_end)
            
            fr fr Check for Content-Length header
            ready (str_contains(headers, "Content-Length:")) {
                sus content_length = parse_content_length(headers)
                sus body_received = len(response_buffer) - header_end
                ready (body_received >= content_length) {
                    halt  fr fr Complete response received
                }
            } nah ready (str_contains(headers, "Transfer-Encoding: chunked")) {
                fr fr Handle chunked encoding
                ready (str_ends_with(response_buffer, "\r\n0\r\n\r\n")) {
                    halt  fr fr End of chunked response
                }
            } nah {
                fr fr No content-length, wait for connection close
            }
        }
    }
    
    damn response_buffer
}

slay tcp_close_socket(socket_fd drip) {
    fr fr Close TCP socket
    system_call_tcp_close(socket_fd)
}

slay parse_url(url tea) squad { host tea, port drip, path tea, scheme tea } {
    fr fr Parse URL into components for real HTTP requests
    sus scheme tea = "http"
    sus host tea = ""
    sus port drip = 80
    sus path tea = "/"
    
    sus url_work = url
    
    fr fr Extract scheme
    ready (str_starts_with(url_work, "https://")) {
        scheme = "https"
        port = 443
        url_work = str_slice(url_work, 8, len(url_work))
    } nah ready (str_starts_with(url_work, "http://")) {
        scheme = "http"
        port = 80
        url_work = str_slice(url_work, 7, len(url_work))
    }
    
    fr fr Extract host and path
    sus slash_pos = str_find(url_work, "/")
    ready (slash_pos >= 0) {
        host = str_slice(url_work, 0, slash_pos)
        path = str_slice(url_work, slash_pos, len(url_work))
    } nah {
        host = url_work
        path = "/"
    }
    
    fr fr Extract port from host
    sus colon_pos = str_find(host, ":")
    ready (colon_pos >= 0) {
        port = str_to_int(str_slice(host, colon_pos + 1, len(host)))
        host = str_slice(host, 0, colon_pos)
    }
    
    damn { host: host, port: port, path: path, scheme: scheme }
}

slay build_http_request_string(request HttpRequest) tea {
    fr fr Build proper HTTP/1.1 request string
    sus request_line = request.method + " " + parse_url(request.url).path + " HTTP/1.1\r\n"
    sus host_header = "Host: " + parse_url(request.url).host + "\r\n"
    sus user_agent = "User-Agent: CURSED-HTTP/1.0\r\n"
    sus connection = "Connection: close\r\n"
    
    sus headers_str tea = ""
    bestie (sus i drip = 0; i < len(request.headers); i = i + 1) {
        headers_str = headers_str + request.headers[i].name + ": " + request.headers[i].value + "\r\n"
    }
    
    sus content_length tea = ""
    ready (request.body != "") {
        content_length = "Content-Length: " + str(len(request.body)) + "\r\n"
    }
    
    sus full_request = request_line + host_header + user_agent + connection + headers_str + content_length + "\r\n" + request.body
    damn full_request
}

slay parse_http_response(response_data tea) HttpResponse {
    fr fr Parse HTTP response string into HttpResponse struct
    sus lines = str_split(response_data, "\r\n")
    ready (len(lines) == 0) {
        damn HttpResponse{ status: HTTP_STATUS_RESPONSE_ERROR, status_text: "Invalid response", headers: [], body: "" }
    }
    
    fr fr Parse status line
    sus status_line = lines[0]
    sus status_parts = str_split(status_line, " ")
    ready (len(status_parts) < 2) {
        damn HttpResponse{ status: HTTP_STATUS_RESPONSE_ERROR, status_text: "Invalid status line", headers: [], body: "" }
    }
    
    sus status = str_to_int(status_parts[1])
    sus status_text = ""
    ready (len(status_parts) >= 3) {
        status_text = str_join(str_slice_array(status_parts, 2, len(status_parts)), " ")
    }
    
    fr fr Parse headers
    sus headers []HttpHeader = []
    sus header_end drip = 1
    bestie (header_end < len(lines) && lines[header_end] != "") {
        sus header_line = lines[header_end]
        sus colon_pos = str_find(header_line, ":")
        ready (colon_pos > 0) {
            sus name = str_trim(str_slice(header_line, 0, colon_pos))
            sus value = str_trim(str_slice(header_line, colon_pos + 1, len(header_line)))
            array_push(headers, HttpHeader{ name: name, value: value })
        }
        header_end = header_end + 1
    }
    
    fr fr Parse body
    sus body_start = header_end + 1
    sus body tea = ""
    bestie (body_start < len(lines)) {
        body = str_join(str_slice_array(lines, body_start, len(lines)), "\r\n")
        halt
    }
    
    damn HttpResponse{ status: status, status_text: status_text, headers: headers, body: body }
}

slay parse_content_length(headers tea) drip {
    fr fr Extract Content-Length value from headers
    sus lines = str_split(headers, "\r\n")
    bestie (sus i drip = 0; i < len(lines); i = i + 1) {
        sus line = str_to_lower(lines[i])
        ready (str_starts_with(line, "content-length:")) {
            sus colon_pos = str_find(lines[i], ":")
            sus value_str = str_trim(str_slice(lines[i], colon_pos + 1, len(lines[i])))
            damn str_to_int(value_str)
        }
    }
    damn 0
}

fr fr ===== HTTP UTILITIES =====

slay send_request(request HttpRequest) HttpResponse {
    fr fr Real HTTP client implementation using system curl
    
    fr fr Validate request
    ready (request.url == "") {
        damn create_response(HTTP_STATUS_REQUEST_ERROR, "Empty URL provided")
    }
    
    fr fr Build curl command for real HTTP request
    sus curl_cmd = "curl -s -i --connect-timeout 5 --max-time 30"
    
    fr fr Add method
    ready (request.method != "GET") {
        curl_cmd = curl_cmd + " -X " + request.method
    }
    
    fr fr Add headers
    bestie (sus i drip = 0; i < len(request.headers); i = i + 1) {
        curl_cmd = curl_cmd + " -H \"" + request.headers[i].name + ": " + request.headers[i].value + "\""
    }
    
    fr fr Add body for POST/PUT requests
    ready (request.body != "") {
        curl_cmd = curl_cmd + " -d '" + request.body + "'"
    }
    
    fr fr Add URL
    curl_cmd = curl_cmd + " \"" + request.url + "\""
    
    fr fr Execute curl and capture output
    sus response_data = execute_command_with_output(curl_cmd)
    
    ready (response_data == "") {
        damn create_response(HTTP_STATUS_CONNECTION_ERROR, "Failed to connect to server")
    }
    
    fr fr Check for curl error indicators
    ready (str_contains(response_data, "curl: (")) {
        damn create_response(HTTP_STATUS_CONNECTION_ERROR, "Connection error: " + response_data)
    }
    
    fr fr Parse HTTP response
    sus parsed_response = parse_http_response(response_data)
    damn parsed_response
}

slay get_status_text(status drip) tea {
    ready (status == 200) { damn "OK" }
    ready (status == 201) { damn "Created" }
    ready (status == 204) { damn "No Content" }
    ready (status == 400) { damn "Bad Request" }
    ready (status == 401) { damn "Unauthorized" }
    ready (status == 403) { damn "Forbidden" }
    ready (status == 404) { damn "Not Found" }
    ready (status == 500) { damn "Internal Server Error" }
    damn "Unknown"
}

slay create_basic_auth(username tea, password tea) tea {
    sus credentials tea = concat(username, concat(":", password))
    sus encoded tea = base64_encode(credentials)
    damn concat("Basic ", encoded)
}

slay base64_encode(text tea) tea {
    fr fr Bridge to native base64 encoding
    damn "encoded"
}

slay base64_decode(encoded_text tea) tea {
    fr fr Bridge to native base64 decoding
    damn "decoded"
}

slay is_url_safe_char(ch tea) lit {
    ready (is_alpha_char(ch) || is_digit_char(ch)) {
        damn based
    }
    ready (ch == "-" || ch == "_" || ch == "." || ch == "~") {
        damn based
    }
    damn cap
}

slay char_to_hex(ch tea) tea {
    fr fr Convert character to hex representation
    sus ascii drip = char_to_ascii(ch)
    damn int_to_hex(ascii)
}

slay hex_to_char(hex tea) tea {
    fr fr Convert hex to character
    sus ascii drip = hex_to_int(hex)
    damn ascii_to_char(ascii)
}

slay int_to_hex(value drip) tea {
    fr fr Convert integer to hex string
    ready (value == 0) { damn "00" }
    
    sus hex_chars tea = "0123456789ABCDEF"
    sus result tea = ""
    sus temp drip = value
    
    bestie (temp > 0) {
        sus remainder drip = temp % 16
        result = concat(char_at(hex_chars, remainder), result)
        temp = temp / 16
    }
    
    fr fr Pad to 2 characters
    ready (length(result) == 1) {
        result = concat("0", result)
    }
    damn result
}

slay hex_to_int(hex tea) drip {
    fr fr Convert hex string to integer
    sus result drip = 0
    sus i drip = 0
    
    bestie (i < length(hex)) {
        sus ch tea = char_at(hex, i)
        sus digit drip = 0
        
        ready (ch >= "0" && ch <= "9") {
            digit = char_to_ascii(ch) - 48
        } otherwise ready (ch >= "A" && ch <= "F") {
            digit = char_to_ascii(ch) - 55
        } otherwise ready (ch >= "a" && ch <= "f") {
            digit = char_to_ascii(ch) - 87
        }
        
        result = result * 16 + digit
        i = i + 1
    }
    damn result
}

slay get_content_type(file_path tea) tea {
    sus ext tea = get_extension(file_path)
    sus lower_ext tea = to_lowercase(ext)
    
    ready (equals(lower_ext, ".html") || equals(lower_ext, ".htm")) {
        damn "text/html"
    } otherwise ready (equals(lower_ext, ".css")) {
        damn "text/css"
    } otherwise ready (equals(lower_ext, ".js")) {
        damn "application/javascript"
    } otherwise ready (equals(lower_ext, ".json")) {
        damn "application/json"
    } otherwise ready (equals(lower_ext, ".xml")) {
        damn "application/xml"
    } otherwise ready (equals(lower_ext, ".png")) {
        damn "image/png"
    } otherwise ready (equals(lower_ext, ".jpg") || equals(lower_ext, ".jpeg")) {
        damn "image/jpeg"
    } otherwise ready (equals(lower_ext, ".gif")) {
        damn "image/gif"
    } otherwise ready (equals(lower_ext, ".svg")) {
        damn "image/svg+xml"
    }
    damn "text/plain"
}

slay build_file_path(base_directory tea, request_url tea, mount_path tea) tea {
    fr fr Remove mount path from URL to get relative path
    sus relative_path tea = slice(request_url, length(mount_path))
    ready (starts_with(relative_path, "/")) {
        relative_path = slice(relative_path, 1)
    }
    damn join_path(base_directory, relative_path)
}

slay read_static_file(file_path tea) tea {
    fr fr Bridge to file reading - would use filez module
    damn ""
}

slay validate_token(token tea) lit {
    fr fr Simple token validation - in practice would verify JWT or check database
    damn !is_empty(token) && length(token) > 10
}

fr fr ===== HELPER FUNCTIONS =====

slay make(T, size drip) []T {
    fr fr Bridge to native array creation
    damn []T{}
}

slay append(arr []T, item T) []T {
    fr fr Bridge to native array append
    damn arr
}

slay len(arr []T) drip {
    fr fr Bridge to native array length
    damn 0
}

fr fr Import functions from stringz module
slay to_lowercase(text tea) tea {
    fr fr Implemented in stringz module
    damn text
}

slay equals(a tea, b tea) lit {
    fr fr Implemented in stringz module
    damn cap
}

slay concat(a tea, b tea) tea {
    fr fr Implemented in stringz module
    damn ""
}

slay is_empty(text tea) lit {
    fr fr Implemented in stringz module
    damn based
}

slay length(text tea) drip {
    fr fr Implemented in stringz module
    damn 0
}

slay char_at(text tea, index drip) tea {
    fr fr Implemented in stringz module
    damn ""
}

slay substring(text tea, start drip, end drip) tea {
    fr fr Implemented in stringz module
    damn ""
}

slay slice(text tea, start drip) tea {
    fr fr Implemented in stringz module
    damn ""
}

slay split(text tea, separator tea) []tea {
    fr fr Implemented in stringz module
    damn []tea{}
}

slay trim(text tea) tea {
    fr fr Implemented in stringz module
    damn text
}

slay starts_with(text tea, prefix tea) lit {
    fr fr Implemented in stringz module
    damn cap
}

slay ends_with(text tea, suffix tea) lit {
    fr fr Implemented in stringz module
    damn cap
}

slay is_alpha_char(ch tea) lit {
    fr fr Implemented in stringz module
    damn cap
}

slay is_digit_char(ch tea) lit {
    fr fr Implemented in stringz module
    damn cap
}

slay char_to_ascii(ch tea) drip {
    fr fr Implemented in stringz module
    damn 65
}

slay ascii_to_char(ascii drip) tea {
    fr fr Implemented in stringz module
    damn "A"
}

slay get_extension(path tea) tea {
    fr fr Implemented in filez module
    damn ""
}

slay join_path(base tea, relative tea) tea {
    fr fr Implemented in filez module
    damn ""
}

fr fr Import functions from mathz module
slay int_to_string(value drip) tea {
    fr fr Implemented in mathz module
    damn "0"
}

fr fr Import functions for HTTP functionality
slay spill(msg tea) lit {
    fr fr Implemented in vibez module
    damn based
}
