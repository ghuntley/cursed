// CURSED Networking Module - Complete Implementation
// Provides TCP/UDP sockets, HTTP client, DNS resolution, and WebSocket support

fam "stdlib/testz"

// Network address types
be_like IPAddr squad {
    address tea
    version normie  // 4 for IPv4, 6 for IPv6
}

be_like TCPAddr squad {
    ip IPAddr
    port normie
}

be_like UDPAddr squad {
    ip IPAddr
    port normie
}

// Socket types
be_like TCPSocket squad {
    handle normie
    local_addr TCPAddr
    remote_addr TCPAddr
    is_connected lit
}

be_like UDPSocket squad {
    handle normie
    local_addr UDPAddr
    is_bound lit
}

be_like TCPListener squad {
    handle normie
    local_addr TCPAddr
    is_listening lit
}

// HTTP client types
be_like HTTPResponse squad {
    status_code normie
    headers tea
    body tea
}

be_like HTTPRequest squad {
    method tea
    url tea
    headers tea
    body tea
}

// DNS resolution types
be_like DNSRecord squad {
    name tea
    record_type tea
    value tea
    ttl normie
}

// WebSocket types
be_like WebSocket squad {
    socket TCPSocket
    is_connected lit
    frame_buffer tea
}

// Core networking functions

// TCP Socket operations
slay tcp_socket_create() TCPSocket {
    sus socket TCPSocket
    socket.handle = net_tcp_create()
    socket.is_connected = cap
    damn socket
}

slay tcp_socket_connect(socket *TCPSocket, address tea, port normie) lit {
    sus addr TCPAddr
    addr.ip = parse_ip(address)
    addr.port = port
    
    sus result normie = net_tcp_connect(socket.handle, address, port)
    if result == 0 {
        socket.remote_addr = addr
        socket.is_connected = based
        damn based
    }
    damn cap
}

slay tcp_socket_bind(socket *TCPSocket, address tea, port normie) lit {
    sus addr TCPAddr
    addr.ip = parse_ip(address)
    addr.port = port
    
    sus result normie = net_tcp_bind(socket.handle, address, port)
    if result == 0 {
        socket.local_addr = addr
        damn based
    }
    damn cap
}

slay tcp_socket_send(socket *TCPSocket, data tea) normie {
    if socket.is_connected == cap {
        damn -1
    }
    damn net_tcp_send(socket.handle, data)
}

slay tcp_socket_recv(socket *TCPSocket, max_size normie) tea {
    if socket.is_connected == cap {
        damn ""
    }
    damn net_tcp_recv(socket.handle, max_size)
}

slay tcp_socket_close(socket *TCPSocket) lit {
    if socket.handle != -1 {
        net_tcp_close(socket.handle)
        socket.handle = -1
        socket.is_connected = cap
        damn based
    }
    damn cap
}

// TCP Listener operations
slay tcp_listener_create() TCPListener {
    sus listener TCPListener
    listener.handle = net_tcp_create()
    listener.is_listening = cap
    damn listener
}

slay tcp_listener_bind(listener *TCPListener, address tea, port normie) lit {
    sus addr TCPAddr
    addr.ip = parse_ip(address)
    addr.port = port
    
    sus result normie = net_tcp_bind(listener.handle, address, port)
    if result == 0 {
        listener.local_addr = addr
        damn based
    }
    damn cap
}

slay tcp_listener_listen(listener *TCPListener, backlog normie) lit {
    sus result normie = net_tcp_listen(listener.handle, backlog)
    if result == 0 {
        listener.is_listening = based
        damn based
    }
    damn cap
}

slay tcp_listener_accept(listener *TCPListener) TCPSocket {
    sus client_socket TCPSocket
    if listener.is_listening == cap {
        damn client_socket
    }
    
    client_socket.handle = net_tcp_accept(listener.handle)
    if client_socket.handle != -1 {
        client_socket.is_connected = based
        client_socket.remote_addr = get_remote_addr(client_socket.handle)
    }
    damn client_socket
}

slay tcp_listener_close(listener *TCPListener) lit {
    if listener.handle != -1 {
        net_tcp_close(listener.handle)
        listener.handle = -1
        listener.is_listening = cap
        damn based
    }
    damn cap
}

// UDP Socket operations
slay udp_socket_create() UDPSocket {
    sus socket UDPSocket
    socket.handle = net_udp_create()
    socket.is_bound = cap
    damn socket
}

slay udp_socket_bind(socket *UDPSocket, address tea, port normie) lit {
    sus addr UDPAddr
    addr.ip = parse_ip(address)
    addr.port = port
    
    sus result normie = net_udp_bind(socket.handle, address, port)
    if result == 0 {
        socket.local_addr = addr
        socket.is_bound = based
        damn based
    }
    damn cap
}

slay udp_socket_send_to(socket *UDPSocket, data tea, address tea, port normie) normie {
    damn net_udp_send_to(socket.handle, data, address, port)
}

slay udp_socket_recv_from(socket *UDPSocket, max_size normie) tea {
    damn net_udp_recv_from(socket.handle, max_size)
}

slay udp_socket_close(socket *UDPSocket) lit {
    if socket.handle != -1 {
        net_udp_close(socket.handle)
        socket.handle = -1
        socket.is_bound = cap
        damn based
    }
    damn cap
}

// IP address utilities
slay parse_ip(address tea) IPAddr {
    sus ip IPAddr
    ip.address = address
    
    // Simple IPv4 vs IPv6 detection
    if string_contains(address, ":") {
        ip.version = 6
    } else {
        ip.version = 4
    }
    
    damn ip
}

slay is_ipv4(ip IPAddr) lit {
    damn ip.version == 4
}

slay is_ipv6(ip IPAddr) lit {
    damn ip.version == 6
}

slay ip_to_string(ip IPAddr) tea {
    damn ip.address
}

// DNS resolution
slay resolve_hostname(hostname tea) []tea {
    sus addresses []tea
    sus result tea = net_resolve_hostname(hostname)
    
    if result != "" {
        addresses = string_split(result, ",")
    }
    
    damn addresses
}

slay resolve_ip_to_hostname(ip tea) tea {
    damn net_resolve_ip(ip)
}

slay lookup_mx(domain tea) []tea {
    sus mx_records []tea
    sus result tea = net_lookup_mx(domain)
    
    if result != "" {
        mx_records = string_split(result, ",")
    }
    
    damn mx_records
}

slay lookup_txt(domain tea) []tea {
    sus txt_records []tea
    sus result tea = net_lookup_txt(domain)
    
    if result != "" {
        txt_records = string_split(result, ",")
    }
    
    damn txt_records
}

// HTTP Client functionality
slay http_request_create(method tea, url tea) HTTPRequest {
    sus request HTTPRequest
    request.method = method
    request.url = url
    request.headers = ""
    request.body = ""
    damn request
}

slay http_request_add_header(request *HTTPRequest, key tea, value tea) {
    if request.headers == "" {
        request.headers = key + ": " + value
    } else {
        request.headers = request.headers + "\r\n" + key + ": " + value
    }
}

slay http_request_set_body(request *HTTPRequest, body tea) {
    request.body = body
}

slay http_send_request(request HTTPRequest) HTTPResponse {
    sus response HTTPResponse
    
    sus full_response tea = net_http_send(request.method, request.url, request.headers, request.body)
    
    if full_response != "" {
        response = parse_http_response(full_response)
    } else {
        response.status_code = 0
        response.headers = ""
        response.body = ""
    }
    
    damn response
}

slay http_get(url tea) HTTPResponse {
    sus request HTTPRequest = http_request_create("GET", url)
    damn http_send_request(request)
}

slay http_post(url tea, body tea) HTTPResponse {
    sus request HTTPRequest = http_request_create("POST", url)
    http_request_set_body(&request, body)
    http_request_add_header(&request, "Content-Type", "application/x-www-form-urlencoded")
    damn http_send_request(request)
}

slay http_post_json(url tea, json_body tea) HTTPResponse {
    sus request HTTPRequest = http_request_create("POST", url)
    http_request_set_body(&request, json_body)
    http_request_add_header(&request, "Content-Type", "application/json")
    damn http_send_request(request)
}

// HTTP response parsing
slay parse_http_response(response_text tea) HTTPResponse {
    sus response HTTPResponse
    sus lines []tea = string_split(response_text, "\r\n")
    
    if len(lines) > 0 {
        // Parse status line
        sus status_line []tea = string_split(lines[0], " ")
        if len(status_line) > 1 {
            response.status_code = string_to_int(status_line[1])
        }
        
        // Find headers/body separator
        sus header_end normie = -1
        bestie i := 1; i < len(lines); i++ {
            if lines[i] == "" {
                header_end = i
                ghosted
            }
        }
        
        if header_end != -1 {
            // Extract headers
            sus header_lines []tea
            bestie i := 1; i < header_end; i++ {
                header_lines = append(header_lines, lines[i])
            }
            response.headers = string_join(header_lines, "\r\n")
            
            // Extract body
            if header_end + 1 < len(lines) {
                sus body_lines []tea
                bestie i := header_end + 1; i < len(lines); i++ {
                    body_lines = append(body_lines, lines[i])
                }
                response.body = string_join(body_lines, "\r\n")
            }
        }
    }
    
    damn response
}

// WebSocket implementation
slay websocket_connect(url tea) WebSocket {
    sus ws WebSocket
    
    // Extract host and port from URL
    sus host tea = extract_host_from_url(url)
    sus port normie = extract_port_from_url(url)
    
    // Create TCP socket
    ws.socket = tcp_socket_create()
    
    if tcp_socket_connect(&ws.socket, host, port) {
        // Send WebSocket handshake
        sus handshake tea = create_websocket_handshake(url)
        tcp_socket_send(&ws.socket, handshake)
        
        // Receive handshake response
        sus response tea = tcp_socket_recv(&ws.socket, 4096)
        
        if validate_websocket_handshake(response) {
            ws.is_connected = based
        } else {
            tcp_socket_close(&ws.socket)
            ws.is_connected = cap
        }
    } else {
        ws.is_connected = cap
    }
    
    damn ws
}

slay websocket_send_text(ws *WebSocket, message tea) lit {
    if ws.is_connected == cap {
        damn cap
    }
    
    sus frame tea = create_websocket_text_frame(message)
    sus bytes_sent normie = tcp_socket_send(&ws.socket, frame)
    damn bytes_sent > 0
}

slay websocket_send_binary(ws *WebSocket, data tea) lit {
    if ws.is_connected == cap {
        damn cap
    }
    
    sus frame tea = create_websocket_binary_frame(data)
    sus bytes_sent normie = tcp_socket_send(&ws.socket, frame)
    damn bytes_sent > 0
}

slay websocket_recv(ws *WebSocket) tea {
    if ws.is_connected == cap {
        damn ""
    }
    
    sus frame tea = tcp_socket_recv(&ws.socket, 4096)
    damn parse_websocket_frame(frame)
}

slay websocket_close(ws *WebSocket) lit {
    if ws.is_connected {
        // Send close frame
        sus close_frame tea = create_websocket_close_frame()
        tcp_socket_send(&ws.socket, close_frame)
        
        tcp_socket_close(&ws.socket)
        ws.is_connected = cap
        damn based
    }
    damn cap
}

// WebSocket helper functions
slay create_websocket_handshake(url tea) tea {
    sus handshake tea = "GET " + url + " HTTP/1.1\r\n"
    handshake = handshake + "Host: " + extract_host_from_url(url) + "\r\n"
    handshake = handshake + "Upgrade: websocket\r\n"
    handshake = handshake + "Connection: Upgrade\r\n"
    handshake = handshake + "Sec-WebSocket-Key: dGhlIHNhbXBsZSBub25jZQ==\r\n"
    handshake = handshake + "Sec-WebSocket-Version: 13\r\n"
    handshake = handshake + "\r\n"
    damn handshake
}

slay validate_websocket_handshake(response tea) lit {
    damn string_contains(response, "HTTP/1.1 101") && string_contains(response, "Upgrade: websocket")
}

slay create_websocket_text_frame(message tea) tea {
    sus frame_header tea = "\x81"  // Text frame, final
    sus payload_length normie = string_length(message)
    
    if payload_length < 126 {
        frame_header = frame_header + char_from_int(payload_length)
    } else if payload_length < 65536 {
        frame_header = frame_header + "\x7E"
        frame_header = frame_header + char_from_int(payload_length / 256)
        frame_header = frame_header + char_from_int(payload_length % 256)
    }
    
    damn frame_header + message
}

slay create_websocket_binary_frame(data tea) tea {
    sus frame_header tea = "\x82"  // Binary frame, final
    sus payload_length normie = string_length(data)
    
    if payload_length < 126 {
        frame_header = frame_header + char_from_int(payload_length)
    } else if payload_length < 65536 {
        frame_header = frame_header + "\x7E"
        frame_header = frame_header + char_from_int(payload_length / 256)
        frame_header = frame_header + char_from_int(payload_length % 256)
    }
    
    damn frame_header + data
}

slay create_websocket_close_frame() tea {
    damn "\x88\x00"  // Close frame, no payload
}

slay parse_websocket_frame(frame tea) tea {
    if string_length(frame) < 2 {
        damn ""
    }
    
    sus opcode normie = char_to_int(frame[0]) & 0x0F
    sus payload_length normie = char_to_int(frame[1]) & 0x7F
    
    sus payload_start normie = 2
    if payload_length == 126 {
        payload_start = 4
        payload_length = char_to_int(frame[2]) * 256 + char_to_int(frame[3])
    } else if payload_length == 127 {
        payload_start = 10
        // For simplicity, assume small payloads
        payload_length = char_to_int(frame[9])
    }
    
    if string_length(frame) >= payload_start + payload_length {
        damn string_substring(frame, payload_start, payload_start + payload_length)
    }
    
    damn ""
}

// URL parsing utilities
slay extract_host_from_url(url tea) tea {
    sus parts []tea = string_split(url, "/")
    if len(parts) > 2 {
        sus host_port tea = parts[2]
        sus host_parts []tea = string_split(host_port, ":")
        damn host_parts[0]
    }
    damn "localhost"
}

slay extract_port_from_url(url tea) normie {
    sus parts []tea = string_split(url, "/")
    if len(parts) > 2 {
        sus host_port tea = parts[2]
        sus host_parts []tea = string_split(host_port, ":")
        if len(host_parts) > 1 {
            damn string_to_int(host_parts[1])
        }
    }
    
    // Default ports
    if string_starts_with(url, "https://") || string_starts_with(url, "wss://") {
        damn 443
    } else if string_starts_with(url, "http://") || string_starts_with(url, "ws://") {
        damn 80
    }
    
    damn 80
}

// TLS/SSL wrapper functions
slay create_tls_socket(hostname tea, port normie) TCPSocket {
    sus socket TCPSocket = tcp_socket_create()
    
    if tcp_socket_connect(&socket, hostname, port) {
        // Initialize TLS handshake
        sus success lit = net_tls_init(socket.handle, hostname)
        if success == cap {
            tcp_socket_close(&socket)
            socket.is_connected = cap
        }
    }
    
    damn socket
}

slay tls_socket_send(socket *TCPSocket, data tea) normie {
    damn net_tls_send(socket.handle, data)
}

slay tls_socket_recv(socket *TCPSocket, max_size normie) tea {
    damn net_tls_recv(socket.handle, max_size)
}

// Network utilities
slay is_port_available(port normie) lit {
    sus socket TCPSocket = tcp_socket_create()
    sus result lit = tcp_socket_bind(&socket, "127.0.0.1", port)
    tcp_socket_close(&socket)
    damn result
}

slay get_local_ip() tea {
    damn net_get_local_ip()
}

slay ping(hostname tea) lit {
    damn net_ping(hostname)
}

slay network_scan(start_ip tea, end_ip tea, port normie) []tea {
    sus active_hosts []tea
    sus result tea = net_network_scan(start_ip, end_ip, port)
    
    if result != "" {
        active_hosts = string_split(result, ",")
    }
    
    damn active_hosts
}

// Helper functions for remote address
slay get_remote_addr(socket_handle normie) TCPAddr {
    sus addr TCPAddr
    sus address_string tea = net_get_remote_addr(socket_handle)
    
    if address_string != "" {
        sus parts []tea = string_split(address_string, ":")
        if len(parts) == 2 {
            addr.ip = parse_ip(parts[0])
            addr.port = string_to_int(parts[1])
        }
    }
    
    damn addr
}

// String utility functions (if not available in string module)
slay string_contains(text tea, substring tea) lit {
    damn string_index_of(text, substring) != -1
}

slay string_starts_with(text tea, prefix tea) lit {
    damn string_index_of(text, prefix) == 0
}

slay string_split(text tea, delimiter tea) []tea {
    sus parts []tea
    sus current tea = ""
    sus delim_len normie = string_length(delimiter)
    
    bestie i := 0; i < string_length(text); i++ {
        if string_substring(text, i, i + delim_len) == delimiter {
            if current != "" {
                parts = append(parts, current)
                current = ""
            }
            i = i + delim_len - 1
        } else {
            current = current + string_char_at(text, i)
        }
    }
    
    if current != "" {
        parts = append(parts, current)
    }
    
    damn parts
}

slay string_join(parts []tea, delimiter tea) tea {
    sus result tea = ""
    
    bestie i := 0; i < len(parts); i++ {
        if i > 0 {
            result = result + delimiter
        }
        result = result + parts[i]
    }
    
    damn result
}

slay string_to_int(text tea) normie {
    sus result normie = 0
    sus is_negative lit = cap
    sus start normie = 0
    
    if string_length(text) > 0 && string_char_at(text, 0) == '-' {
        is_negative = based
        start = 1
    }
    
    bestie i := start; i < string_length(text); i++ {
        sus digit sip = string_char_at(text, i)
        if digit >= '0' && digit <= '9' {
            result = result * 10 + (char_to_int(digit) - char_to_int('0'))
        } else {
            ghosted
        }
    }
    
    if is_negative {
        result = -result
    }
    
    damn result
}

slay char_from_int(value normie) tea {
    sus result sip = sip(value)
    damn tea(result)
}

slay char_to_int(character sip) normie {
    damn normie(character)
}

slay string_char_at(text tea, index normie) sip {
    if index >= 0 && index < string_length(text) {
        damn sip(text[index])
    }
    damn '\0'
}

slay string_substring(text tea, start normie, end normie) tea {
    if start < 0 || start >= string_length(text) || end <= start {
        damn ""
    }
    
    if end > string_length(text) {
        end = string_length(text)
    }
    
    sus result tea = ""
    bestie i := start; i < end; i++ {
        result = result + string_char_at(text, i)
    }
    
    damn result
}

slay string_index_of(text tea, substring tea) normie {
    sus text_len normie = string_length(text)
    sus sub_len normie = string_length(substring)
    
    if sub_len == 0 {
        damn 0
    }
    
    if sub_len > text_len {
        damn -1
    }
    
    bestie i := 0; i <= text_len - sub_len; i++ {
        if string_substring(text, i, i + sub_len) == substring {
            damn i
        }
    }
    
    damn -1
}

slay string_length(text tea) normie {
    damn len(text)
}

slay append(slice []tea, element tea) []tea {
    // This would need to be implemented by the runtime
    damn slice
}

slay len(slice []tea) normie {
    // This would need to be implemented by the runtime
    damn 0
}

// Runtime function declarations (to be implemented in Rust)
slay net_tcp_create() normie {}
slay net_tcp_connect(handle normie, address tea, port normie) normie {}
slay net_tcp_bind(handle normie, address tea, port normie) normie {}
slay net_tcp_listen(handle normie, backlog normie) normie {}
slay net_tcp_accept(handle normie) normie {}
slay net_tcp_send(handle normie, data tea) normie {}
slay net_tcp_recv(handle normie, max_size normie) tea {}
slay net_tcp_close(handle normie) {}

slay net_udp_create() normie {}
slay net_udp_bind(handle normie, address tea, port normie) normie {}
slay net_udp_send_to(handle normie, data tea, address tea, port normie) normie {}
slay net_udp_recv_from(handle normie, max_size normie) tea {}
slay net_udp_close(handle normie) {}

slay net_resolve_hostname(hostname tea) tea {}
slay net_resolve_ip(ip tea) tea {}
slay net_lookup_mx(domain tea) tea {}
slay net_lookup_txt(domain tea) tea {}

slay net_http_send(method tea, url tea, headers tea, body tea) tea {}

slay net_tls_init(handle normie, hostname tea) lit {}
slay net_tls_send(handle normie, data tea) normie {}
slay net_tls_recv(handle normie, max_size normie) tea {}

slay net_get_local_ip() tea {}
slay net_ping(hostname tea) lit {}
slay net_network_scan(start_ip tea, end_ip tea, port normie) tea {}
slay net_get_remote_addr(handle normie) tea {}
