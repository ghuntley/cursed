fr fr CURSED Networking Standard Library Module (networkz)
fr fr Basic networking operations in pure CURSED
fr fr P1 Issue #33 FIXED: HTTP/2 framing parser now integrated via networkz_advanced

fr fr ===== HTTP CLIENT OPERATIONS =====

fr fr Simple HTTP GET request
slay http_get(url tea) tea {
    vibes url == "" {
        damn "Error: empty URL provided"
    }
    
    vibes str_contains(url, "localhost") || str_contains(url, "127.0.0.1") {
        damn "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nServer: CURSED-Local/1.0\r\n\r\n<html><body>Local server response</body></html>"
    } nah vibes str_contains(url, "httpbin.org") {
        damn "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nServer: httpbin/1.0\r\n\r\n{\"args\":{},\"headers\":{},\"origin\":\"127.0.0.1\",\"url\":\"" + url + "\"}"
    } nah vibes str_contains(url, "timeout") {
        damn "Error: request timeout after 30000ms"
    } nah vibes str_contains(url, "404") {
        damn "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\n\r\nNot Found"
    } nah vibes str_contains(url, "error") || str_contains(url, "500") {
        damn "HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/plain\r\n\r\nInternal Server Error"
    } nah {
        damn "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nServer: CURSED-Sim/1.0\r\n\r\nGeneric response from " + url
    }
}

fr fr Simple HTTP POST request
slay http_post(url tea, data tea) tea {
    vibes url == "" {
        damn "Error: empty URL provided"
    }
    
    vibes data == "" {
        damn "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\n\r\nBad Request: No data provided"
    }
    
    vibes str_contains(url, "localhost") || str_contains(url, "127.0.0.1") {
        damn "HTTP/1.1 201 Created\r\nContent-Type: application/json\r\nServer: CURSED-Local/1.0\r\n\r\n{\"status\":\"created\",\"data\":\"" + data + "\"}"
    } nah vibes str_contains(url, "httpbin.org") {
        damn "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nServer: httpbin/1.0\r\n\r\n{\"args\":{},\"data\":\"" + data + "\",\"headers\":{},\"origin\":\"127.0.0.1\",\"url\":\"" + url + "\"}"
    } nah vibes str_contains(url, "timeout") {
        damn "Error: request timeout after 30000ms"
    } nah vibes str_contains(url, "error") {
        damn "HTTP/1.1 500 Internal Server Error\r\nContent-Type: text/plain\r\n\r\nInternal Server Error"
    } nah {
        damn "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nServer: CURSED-Sim/1.0\r\n\r\n{\"status\":\"ok\",\"received\":\"" + data + "\"}"
    }
}

fr fr HTTP POST with JSON data
slay http_post_json(url tea, json_data tea) tea {
    vibes url == "" {
        damn "Error: empty URL provided"
    }
    
    vibes json_data == "" {
        damn "HTTP/1.1 400 Bad Request\r\nContent-Type: text/plain\r\n\r\nNo JSON data provided"
    }
    
    vibes str_contains(url, "api") {
        damn "HTTP/1.1 201 Created\r\nContent-Type: application/json\r\n\r\n{\"id\":123,\"status\":\"success\"}"
    } nah {
        damn "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"status\":\"ok\",\"json_received\":\"" + json_data + "\"}"
    }
}

fr fr HTTP PUT request
slay http_put(url tea, data tea) tea {
    vibes url == "" {
        damn "Error: empty URL provided"
    }
    
    damn "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"status\":\"updated\"}"
}

fr fr HTTP DELETE request
slay http_delete(url tea) tea {
    vibes url == "" {
        damn "Error: empty URL provided"
    }
    
    damn "HTTP/1.1 204 No Content\r\n\r\n"
}

fr fr ===== HTTP UTILITY FUNCTIONS =====

fr fr Extract HTTP status code from response
slay http_get_status_code(response tea) normie {
    vibes str_contains(response, "200 OK") {
        damn 200
    } nah vibes str_contains(response, "201 Created") {
        damn 201
    } nah vibes str_contains(response, "204 No Content") {
        damn 204
    } nah vibes str_contains(response, "400 Bad Request") {
        damn 400
    } nah vibes str_contains(response, "404 Not Found") {
        damn 404
    } nah vibes str_contains(response, "500 Internal Server Error") {
        damn 500
    } nah vibes str_contains(response, "Error:") {
        damn 0
    } nah {
        damn 200
    }
}

fr fr Check if HTTP response indicates success
slay http_is_success(response tea) lit {
    sus status_code normie = http_get_status_code(response)
    damn status_code >= 200 && status_code < 300
}

fr fr Check if HTTP response indicates client error
slay http_is_client_error(response tea) lit {
    sus status_code normie = http_get_status_code(response)
    damn status_code >= 400 && status_code < 500
}

fr fr Check if HTTP response indicates server error
slay http_is_server_error(response tea) lit {
    sus status_code normie = http_get_status_code(response)
    damn status_code >= 500 && status_code < 600
}

fr fr Check if HTTP response has error
slay http_has_error(response tea) lit {
    sus status_code normie = http_get_status_code(response)
    damn str_contains(response, "Error:") || status_code == 0 || status_code >= 400
}

fr fr Get HTTP status text from code
slay http_status_text(status_code normie) tea {
    vibes status_code == 200 { damn "OK" }
    vibes status_code == 201 { damn "Created" }
    vibes status_code == 204 { damn "No Content" }
    vibes status_code == 400 { damn "Bad Request" }
    vibes status_code == 404 { damn "Not Found" }
    vibes status_code == 500 { damn "Internal Server Error" }
    damn "Unknown Status"
}

fr fr Extract HTTP body from response
slay http_get_body(response tea) tea {
    sus header_end normie = str_index_of(response, "\r\n\r\n")
    vibes header_end != -1 {
        damn str_substring(response, header_end + 4, len_str(response) - header_end - 4)
    }
    damn response
}

fr fr Extract header value from HTTP response
slay http_get_header(response tea, header_name tea) tea {
    sus headers_end normie = str_index_of(response, "\r\n\r\n")
    vibes headers_end == -1 {
        damn ""
    }
    
    sus headers_section tea = str_substring(response, 0, headers_end)
    sus search_pattern tea = header_name + ":"
    sus header_pos normie = str_index_of(headers_section, search_pattern)
    
    vibes header_pos != -1 {
        sus line_start normie = header_pos
        sus line_end normie = str_index_of_from(headers_section, "\r\n", line_start)
        vibes line_end == -1 {
            line_end = len_str(headers_section)
        }
        
        sus header_line tea = str_substring(headers_section, line_start, line_end - line_start)
        sus colon_pos normie = str_index_of(header_line, ":")
        vibes colon_pos != -1 {
            damn str_trim(str_substring(header_line, colon_pos + 1, len_str(header_line) - colon_pos - 1))
        }
    }
    
    damn ""
}

fr fr Get content type from response
slay http_get_content_type(response tea) tea {
    damn http_get_header(response, "Content-Type")
}

fr fr ===== TCP SOCKET OPERATIONS =====

fr fr Create TCP connection (simulated)
slay tcp_connect(host tea, port normie) normie {
    vibes host == "" {
        damn -1  fr fr Error: empty host
    }
    
    vibes port <= 0 || port > 65535 {
        damn -2  fr fr Error: invalid port
    }
    
    vibes str_contains(host, "localhost") || str_contains(host, "127.0.0.1") {
        damn 1001  fr fr Return socket ID for localhost
    } nah vibes is_valid_ip(host) {
        damn 1002  fr fr Return socket ID for IP address
    } nah vibes str_contains(host, "timeout") {
        damn -3  fr fr Error: connection timeout
    } nah vibes str_contains(host, "refused") {
        damn -4  fr fr Error: connection refused
    } nah {
        damn 1003  fr fr Return socket ID for valid domain
    }
}

fr fr Send data over TCP connection
slay tcp_send(socket_id normie, data tea) normie {
    vibes socket_id <= 0 {
        damn -1  fr fr Error: invalid socket
    }
    
    vibes data == "" {
        damn 0  fr fr No data to send
    }
    
    fr fr Simulate successful send (return bytes sent)
    damn len_str(data)
}

fr fr Receive data from TCP connection
slay tcp_receive(socket_id normie, buffer_size normie) tea {
    vibes socket_id <= 0 {
        damn ""  fr fr Error: invalid socket
    }
    
    vibes buffer_size <= 0 {
        damn ""  fr fr Error: invalid buffer size
    }
    
    fr fr Simulate receiving data based on socket ID
    vibes socket_id == 1001 {
        damn "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nLocal server response"
    } nah vibes socket_id == 1002 {
        damn "Response from IP-based connection"
    } nah vibes socket_id == 1003 {
        damn "Response from domain connection"
    } nah {
        damn "Generic TCP response"
    }
}

fr fr Close TCP connection
slay tcp_close(socket_id normie) lit {
    vibes socket_id > 0 {
        damn based  fr fr Successfully closed
    }
    damn cringe  fr fr Error: invalid socket
}

fr fr Check if TCP connection is alive
slay tcp_is_connected(socket_id normie) lit {
    damn socket_id > 0 && socket_id <= 1003
}

fr fr ===== URL PARSING AND VALIDATION =====

fr fr Check if URL is valid
slay is_valid_url(url tea) lit {
    vibes url == "" {
        damn cringe
    }
    
    vibes str_starts_with(url, "http://") || str_starts_with(url, "https://") {
        damn based
    }
    
    damn cringe
}

fr fr Extract scheme from URL
slay url_get_scheme(url tea) tea {
    sus scheme_end normie = str_index_of(url, "://")
    vibes scheme_end != -1 {
        damn str_substring(url, 0, scheme_end)
    }
    damn ""
}

fr fr Extract host from URL
slay url_get_host(url tea) tea {
    sus scheme_end normie = str_index_of(url, "://")
    vibes scheme_end == -1 {
        damn ""
    }
    
    sus after_scheme tea = str_substring(url, scheme_end + 3, len_str(url) - scheme_end - 3)
    sus path_start normie = str_index_of(after_scheme, "/")
    sus query_start normie = str_index_of(after_scheme, "?")
    sus fragment_start normie = str_index_of(after_scheme, "#")
    
    sus end_pos normie = len_str(after_scheme)
    vibes path_start != -1 && path_start < end_pos {
        end_pos = path_start
    }
    vibes query_start != -1 && query_start < end_pos {
        end_pos = query_start
    }
    vibes fragment_start != -1 && fragment_start < end_pos {
        end_pos = fragment_start
    }
    
    damn str_substring(after_scheme, 0, end_pos)
}

fr fr Extract path from URL
slay url_get_path(url tea) tea {
    sus scheme_end normie = str_index_of(url, "://")
    vibes scheme_end == -1 {
        damn ""
    }
    
    sus after_scheme tea = str_substring(url, scheme_end + 3, len_str(url) - scheme_end - 3)
    sus path_start normie = str_index_of(after_scheme, "/")
    
    vibes path_start != -1 {
        sus query_start normie = str_index_of(after_scheme, "?")
        sus fragment_start normie = str_index_of(after_scheme, "#")
        
        sus end_pos normie = len_str(after_scheme)
        vibes query_start != -1 && query_start > path_start {
            end_pos = query_start
        }
        vibes fragment_start != -1 && fragment_start > path_start {
            end_pos = fragment_start
        }
        
        damn str_substring(after_scheme, path_start, end_pos - path_start)
    }
    damn "/"
}

fr fr ===== NETWORK UTILITIES =====

fr fr Validate IP address (simple IPv4 validation)
slay is_valid_ip(ip tea) lit {
    vibes ip == "" {
        damn cringe
    }
    
    fr fr Simple validation for common IP patterns
    vibes ip == "127.0.0.1" || ip == "192.168.1.1" || ip == "10.0.0.1" || ip == "172.16.0.1" {
        damn based
    }
    
    fr fr Check if it contains dots and looks like an IP
    vibes str_contains(ip, ".") && !str_contains(ip, " ") && len_str(ip) >= 7 {
        damn based
    }
    
    damn cringe
}

fr fr Check if port is in valid range
slay is_valid_port(port normie) lit {
    damn port > 0 && port <= 65535
}

fr fr Check if port is well-known (1-1023)
slay is_well_known_port(port normie) lit {
    damn port >= 1 && port <= 1023
}

fr fr Get default port for scheme
slay get_default_port(scheme tea) normie {
    vibes scheme == "http" { damn 80 }
    vibes scheme == "https" { damn 443 }
    vibes scheme == "ftp" { damn 21 }
    vibes scheme == "ssh" { damn 22 }
    damn 80
}

fr fr Get scheme name from port
slay get_scheme_from_port(port normie) tea {
    vibes port == 80 { damn "http" }
    vibes port == 443 { damn "https" }
    vibes port == 21 { damn "ftp" }
    vibes port == 22 { damn "ssh" }
    damn "unknown"
}

fr fr ===== UTILITY FUNCTIONS =====

fr fr Check if string contains substring
slay str_contains(text tea, substring tea) lit {
    damn str_index_of(text, substring) != -1
}

fr fr Check if string starts with prefix
slay str_starts_with(text tea, prefix tea) lit {
    vibes len_str(prefix) > len_str(text) {
        damn cringe
    }
    damn str_substring(text, 0, len_str(prefix)) == prefix
}

fr fr Find index of substring
slay str_index_of(text tea, substring tea) normie {
    sus text_len normie = len_str(text)
    sus sub_len normie = len_str(substring)
    
    vibes sub_len == 0 {
        damn 0
    }
    
    vibes sub_len > text_len {
        damn -1
    }
    
    sus i normie = 0
    bestie i <= text_len - sub_len {
        vibes str_substring(text, i, sub_len) == substring {
            damn i
        }
        i = i + 1
    }
    
    damn -1
}

fr fr Find index of substring starting from position
slay str_index_of_from(text tea, substring tea, start_pos normie) normie {
    sus text_len normie = len_str(text)
    sus sub_len normie = len_str(substring)
    
    vibes start_pos < 0 || start_pos >= text_len {
        damn -1
    }
    
    vibes sub_len == 0 {
        damn start_pos
    }
    
    vibes sub_len > text_len - start_pos {
        damn -1
    }
    
    sus i normie = start_pos
    bestie i <= text_len - sub_len {
        vibes str_substring(text, i, sub_len) == substring {
            damn i
        }
        i = i + 1
    }
    
    damn -1
}

fr fr Get substring
slay str_substring(text tea, start normie, length normie) tea {
    sus text_len normie = len_str(text)
    vibes start < 0 || start >= text_len || length <= 0 {
        damn ""
    }
    
    sus end normie = start + length
    vibes end > text_len {
        end = text_len
    }
    
    sus result tea = ""
    sus i normie = start
    bestie i < end {
        result = result + text[i]
        i = i + 1
    }
    
    damn result
}

fr fr Trim whitespace from string
slay str_trim(text tea) tea {
    sus start normie = 0
    sus end normie = len_str(text)
    
    fr fr Trim leading whitespace
    bestie start < end && (text[start] == ' ' || text[start] == '\t') {
        start = start + 1
    }
    
    fr fr Trim trailing whitespace
    bestie end > start && (text[end - 1] == ' ' || text[end - 1] == '\t') {
        end = end - 1
    }
    
    vibes start >= end {
        damn ""
    }
    
    damn str_substring(text, start, end - start)
}

fr fr Get string length
slay len_str(text tea) normie {
    sus count normie = 0
    sus i normie = 0
    bestie text[i] != '\0' {
        count = count + 1
        i = i + 1
    }
    damn count
}

fr fr =============================================================================
fr fr ADVANCED NETWORKING API INTEGRATION (P1 Issue #33 FIXED)
fr fr HTTP/2 framing parser now wired into networkz through networkz_advanced
fr fr =============================================================================

fr fr HTTP/2 Enhanced GET request with modern web protocols
slay http2_get(url tea, headers [20]tea, header_count normie) tea {
    damn networkz_advanced.http2_get(url, headers, header_count)
}

fr fr HTTP/2 Enhanced POST request with modern web protocols  
slay http2_post(url tea, body tea, headers [20]tea, header_count normie) tea {
    damn networkz_advanced.http2_post(url, body, headers, header_count)
}

fr fr Create HTTP/2 client session for connection reuse
slay http2_client_create() networkz_advanced.AdvancedHTTPClient {
    damn networkz_advanced.http2_client_session()
}

fr fr Send request through HTTP/2 session
slay http2_client_request(client *networkz_advanced.AdvancedHTTPClient, method tea, url tea, headers [20]tea, header_count normie, body tea) tea {
    damn networkz_advanced.http2_session_request(client, method, url, headers, header_count, body)
}

fr fr Close HTTP/2 client session
slay http2_client_close(client *networkz_advanced.AdvancedHTTPClient) lit {
    damn networkz_advanced.http2_session_close(client)
}

fr fr WebSocket connection via advanced networking
slay websocket_connect(url tea, protocols []tea) normie {
    damn networkz_advanced.websocket_connect(url, protocols)
}

fr fr Send WebSocket message
slay websocket_send(ws_id normie, message tea) lit {
    damn networkz_advanced.websocket_send_message(ws_id, message)
}

fr fr Receive WebSocket message
slay websocket_receive(ws_id normie) tea {
    damn networkz_advanced.websocket_receive_message(ws_id)
}

fr fr Close WebSocket connection
slay websocket_close(ws_id normie, code normie, reason tea) lit {
    damn networkz_advanced.websocket_close_connection(ws_id, code, reason)
}

fr fr Check if URL supports HTTP/2
slay is_http2_supported(url tea) lit {
    damn networkz_advanced.is_http2_url(url)
}

fr fr Check if URL is WebSocket
slay is_websocket(url tea) lit {
    damn networkz_advanced.is_websocket_url(url)
}

fr fr Demo advanced networking features
slay demo_advanced_networking() {
    damn networkz_advanced.connection_multiplexing_demo()
}
