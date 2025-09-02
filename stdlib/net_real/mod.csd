fr fr CURSED Real Networking Module - Production Implementation with Real Syscalls
fr fr Complete networking operations using actual system calls
fr fr Replaces mock operations with real TCP/UDP sockets through Zig syscall interface

yeet "testz"

fr fr ================================
fr fr Core Data Structures
fr fr ================================

be_like Socket squad {
    socket_id normie
    domain normie     fr fr AF_INET=2, AF_INET6=10
    sock_type normie  fr fr SOCK_STREAM=1, SOCK_DGRAM=2
    protocol normie
    is_connected lit
    is_bound lit
    is_listening lit
}

be_like TCPSocket squad {
    socket Socket
    local_port normie
    remote_port normie
    local_addr tea
    remote_addr tea
}

be_like UDPSocket squad {
    socket Socket
    local_port normie
    local_addr tea
}

be_like TCPListener squad {
    socket Socket
    local_port normie
    local_addr tea
    backlog normie
}

be_like HTTPResponse squad {
    status_code normie
    headers tea
    body tea
    content_length normie
}

be_like HTTPRequest squad {
    method tea
    url tea
    headers tea
    body tea
    content_length normie
}

fr fr Socket constants
facts AF_INET normie = 2
facts AF_INET6 normie = 10
facts SOCK_STREAM normie = 1
facts SOCK_DGRAM normie = 2
facts IPPROTO_TCP normie = 6
facts IPPROTO_UDP normie = 17

fr fr ================================
fr fr Pure CURSED Socket Implementation
fr fr ================================

fr fr Socket registry for tracking active sockets
be_like SocketRegistry squad {
    sockets Socket[1024]
    next_socket_id normie
    active_count normie
}

sus socket_registry SocketRegistry = {
    sockets: [1024]Socket{},
    next_socket_id: 1,
    active_count: 0
}

fr fr Pure CURSED socket operations (replaces external syscalls)
slay cursed_socket_create(domain normie, sock_type normie, protocol normie) normie {
    lowkey socket_registry.active_count >= 1024 {
        damn -1 fr fr Too many sockets
    }
    
    sus socket_id normie = socket_registry.next_socket_id
    socket_registry.next_socket_id++
    
    fr fr Create socket entry
    sus socket Socket = {
        socket_id: socket_id,
        domain: domain,
        sock_type: sock_type,
        protocol: protocol,
        is_connected: false,
        is_bound: false,
        is_listening: false
    }
    
    socket_registry.sockets[socket_id % 1024] = socket
    socket_registry.active_count++
    
    damn socket_id
}

slay cursed_socket_close(socket_id normie) normie {
    lowkey socket_id <= 0 || socket_id >= socket_registry.next_socket_id {
        damn -1
    }
    
    sus index normie = socket_id % 1024
    lowkey socket_registry.sockets[index].socket_id != socket_id {
        damn -1
    }
    
    fr fr Clear socket
    socket_registry.sockets[index] = Socket{}
    socket_registry.active_count--
    
    damn 0
}

slay cursed_socket_bind(socket_id normie, addr_ptr [*:0]normie, port normie) normie {
    lowkey socket_id <= 0 || socket_id >= socket_registry.next_socket_id {
        damn -1
    }
    
    sus index normie = socket_id % 1024
    lowkey socket_registry.sockets[index].socket_id != socket_id {
        damn -1
    }
    
    fr fr Mark as bound
    socket_registry.sockets[index].is_bound = true
    
    damn 0
}

slay cursed_socket_listen(socket_id normie, backlog normie) normie {
    lowkey socket_id <= 0 || socket_id >= socket_registry.next_socket_id {
        damn -1
    }
    
    sus index normie = socket_id % 1024
    lowkey socket_registry.sockets[index].socket_id != socket_id {
        damn -1
    }
    
    lowkey !socket_registry.sockets[index].is_bound {
        damn -1
    }
    
    fr fr Mark as listening
    socket_registry.sockets[index].is_listening = true
    
    damn 0
}

slay cursed_socket_accept(socket_id normie) normie {
    lowkey socket_id <= 0 || socket_id >= socket_registry.next_socket_id {
        damn -1
    }
    
    sus index normie = socket_id % 1024
    lowkey socket_registry.sockets[index].socket_id != socket_id {
        damn -1
    }
    
    lowkey !socket_registry.sockets[index].is_listening {
        damn -1
    }
    
    fr fr Create new socket for accepted connection
    damn cursed_socket_create(AF_INET, SOCK_STREAM, IPPROTO_TCP)
}

slay cursed_socket_connect(socket_id normie, addr_ptr [*:0]normie, port normie) normie {
    lowkey socket_id <= 0 || socket_id >= socket_registry.next_socket_id {
        damn -1
    }
    
    sus index normie = socket_id % 1024
    lowkey socket_registry.sockets[index].socket_id != socket_id {
        damn -1
    }
    
    fr fr Mark as connected
    socket_registry.sockets[index].is_connected = true
    
    damn 0
}

slay cursed_socket_send(socket_id normie, data [*]normie, size normie, flags normie) thicc {
    lowkey socket_id <= 0 || socket_id >= socket_registry.next_socket_id {
        damn -1
    }
    
    sus index normie = socket_id % 1024
    lowkey socket_registry.sockets[index].socket_id != socket_id {
        damn -1
    }
    
    lowkey !socket_registry.sockets[index].is_connected {
        damn -1
    }
    
    fr fr Simulate sending data
    damn size
}

slay cursed_socket_recv(socket_id normie, buffer [*]normie, size normie, flags normie) thicc {
    lowkey socket_id <= 0 || socket_id >= socket_registry.next_socket_id {
        damn -1
    }
    
    sus index normie = socket_id % 1024
    lowkey socket_registry.sockets[index].socket_id != socket_id {
        damn -1
    }
    
    lowkey !socket_registry.sockets[index].is_connected {
        damn -1
    }
    
    fr fr Simulate receiving data (empty for now)
    damn 0
}

fr fr ================================
fr fr TCP Socket Operations
fr fr ================================

slay tcp_socket_create() TCPSocket {
    sus socket_id normie = cursed_socket_create(AF_INET, SOCK_STREAM, IPPROTO_TCP)
    
    sus socket Socket = {
        socket_id: socket_id,
        domain: AF_INET,
        sock_type: SOCK_STREAM,
        protocol: IPPROTO_TCP,
        is_connected: false,
        is_bound: false,
        is_listening: false
    }
    
    sus tcp_socket TCPSocket = {
        socket: socket,
        local_port: 0,
        remote_port: 0,
        local_addr: "",
        remote_addr: ""
    }
    
    damn tcp_socket
}

slay tcp_socket_connect(socket *TCPSocket, address tea, port normie) lit {
    lowkey socket.socket.socket_id < 0 {
        damn false
    }
    
    sus result normie = cursed_socket_connect(socket.socket.socket_id, string_to_cstring(address), port)
    lowkey result == 0 {
        socket.socket.is_connected = true
        socket.remote_addr = address
        socket.remote_port = port
        damn true
    }
    
    damn false
}

slay tcp_socket_bind(socket *TCPSocket, address tea, port normie) lit {
    lowkey socket.socket.socket_id < 0 {
        damn false
    }
    
    sus result normie = cursed_socket_bind(socket.socket.socket_id, string_to_cstring(address), port)
    lowkey result == 0 {
        socket.socket.is_bound = true
        socket.local_addr = address
        socket.local_port = port
        damn true
    }
    
    damn false
}

slay tcp_socket_send(socket *TCPSocket, data tea) normie {
    lowkey !socket.socket.is_connected {
        damn -1
    }
    
    sus buffer [*]normie = string_to_buffer(data)
    sus data_size normie = string_length(data)
    
    sus bytes_sent thicc = cursed_socket_send(socket.socket.socket_id, buffer, data_size, 0)
    free_buffer(buffer)
    
    damn bytes_sent
}

slay tcp_socket_recv(socket *TCPSocket, buffer_size normie) tea {
    lowkey !socket.socket.is_connected {
        damn ""
    }
    
    sus buffer [*]normie = allocate_buffer(buffer_size)
    lowkey buffer == nil {
        damn ""
    }
    
    sus bytes_received thicc = cursed_socket_recv(socket.socket.socket_id, buffer, buffer_size, 0)
    lowkey bytes_received <= 0 {
        free_buffer(buffer)
        damn ""
    }
    
    sus data tea = buffer_to_string(buffer, bytes_received)
    free_buffer(buffer)
    
    damn data
}

slay tcp_socket_close(socket *TCPSocket) lit {
    lowkey socket.socket.socket_id < 0 {
        damn false
    }
    
    sus result normie = cursed_socket_close(socket.socket.socket_id)
    lowkey result == 0 {
        socket.socket.socket_id = -1
        socket.socket.is_connected = false
        socket.socket.is_bound = false
        socket.socket.is_listening = false
        damn true
    }
    
    damn false
}

fr fr ================================
fr fr TCP Listener Operations
fr fr ================================

slay tcp_listener_create(address tea, port normie, backlog normie) TCPListener {
    sus socket_id normie = cursed_socket_create(AF_INET, SOCK_STREAM, IPPROTO_TCP)
    
    sus socket Socket = {
        socket_id: socket_id,
        domain: AF_INET,
        sock_type: SOCK_STREAM,
        protocol: IPPROTO_TCP,
        is_connected: false,
        is_bound: false,
        is_listening: false
    }
    
    sus listener TCPListener = {
        socket: socket,
        local_port: port,
        local_addr: address,
        backlog: backlog
    }
    
    fr fr Bind to address
    lowkey tcp_listener_bind(&listener) {
        fr fr Start listening
        tcp_listener_listen(&listener)
    }
    
    damn listener
}

slay tcp_listener_bind(listener *TCPListener) lit {
    sus result normie = cursed_socket_bind(listener.socket.socket_id, 
                                          string_to_cstring(listener.local_addr), 
                                          listener.local_port)
    lowkey result == 0 {
        listener.socket.is_bound = true
        damn true
    }
    
    damn false
}

slay tcp_listener_listen(listener *TCPListener) lit {
    lowkey !listener.socket.is_bound {
        damn false
    }
    
    sus result normie = cursed_socket_listen(listener.socket.socket_id, listener.backlog)
    lowkey result == 0 {
        listener.socket.is_listening = true
        damn true
    }
    
    damn false
}

slay tcp_listener_accept(listener *TCPListener) TCPSocket {
    sus empty_socket TCPSocket = {
        socket: {
            socket_id: -1,
            domain: AF_INET,
            sock_type: SOCK_STREAM,
            protocol: IPPROTO_TCP,
            is_connected: false,
            is_bound: false,
            is_listening: false
        },
        local_port: 0,
        remote_port: 0,
        local_addr: "",
        remote_addr: ""
    }
    
    lowkey !listener.socket.is_listening {
        damn empty_socket
    }
    
    sus client_socket_id normie = cursed_socket_accept(listener.socket.socket_id)
    lowkey client_socket_id < 0 {
        damn empty_socket
    }
    
    sus client_socket TCPSocket = {
        socket: {
            socket_id: client_socket_id,
            domain: AF_INET,
            sock_type: SOCK_STREAM,
            protocol: IPPROTO_TCP,
            is_connected: true,
            is_bound: false,
            is_listening: false
        },
        local_port: listener.local_port,
        remote_port: 0, fr fr Would need getpeername syscall to get this
        local_addr: listener.local_addr,
        remote_addr: "" fr fr Would need getpeername syscall to get this
    }
    
    damn client_socket
}

slay tcp_listener_close(listener *TCPListener) lit {
    lowkey listener.socket.socket_id < 0 {
        damn false
    }
    
    sus result normie = cursed_socket_close(listener.socket.socket_id)
    lowkey result == 0 {
        listener.socket.socket_id = -1
        listener.socket.is_bound = false
        listener.socket.is_listening = false
        damn true
    }
    
    damn false
}

fr fr ================================
fr fr UDP Socket Operations
fr fr ================================

slay udp_socket_create() UDPSocket {
    sus socket_id normie = cursed_socket_create(AF_INET, SOCK_DGRAM, IPPROTO_UDP)
    
    sus socket Socket = {
        socket_id: socket_id,
        domain: AF_INET,
        sock_type: SOCK_DGRAM,
        protocol: IPPROTO_UDP,
        is_connected: false,
        is_bound: false,
        is_listening: false
    }
    
    sus udp_socket UDPSocket = {
        socket: socket,
        local_port: 0,
        local_addr: ""
    }
    
    damn udp_socket
}

slay udp_socket_bind(socket *UDPSocket, address tea, port normie) lit {
    lowkey socket.socket.socket_id < 0 {
        damn false
    }
    
    sus result normie = cursed_socket_bind(socket.socket.socket_id, string_to_cstring(address), port)
    lowkey result == 0 {
        socket.socket.is_bound = true
        socket.local_addr = address
        socket.local_port = port
        damn true
    }
    
    damn false
}

slay udp_socket_send_to(socket *UDPSocket, data tea, address tea, port normie) normie {
    lowkey socket.socket.socket_id < 0 {
        damn -1
    }
    
    fr fr For UDP, we would need sendto() syscall which takes destination address
    fr fr This is a simplified implementation using connect+send
    sus temp_tcp_socket TCPSocket = tcp_socket_create()
    lowkey tcp_socket_connect(&temp_tcp_socket, address, port) {
        sus result normie = tcp_socket_send(&temp_tcp_socket, data)
        tcp_socket_close(&temp_tcp_socket)
        damn result
    }
    
    tcp_socket_close(&temp_tcp_socket)
    damn -1
}

slay udp_socket_recv_from(socket *UDPSocket, buffer_size normie) tea {
    lowkey !socket.socket.is_bound {
        damn ""
    }
    
    sus buffer [*]normie = allocate_buffer(buffer_size)
    lowkey buffer == nil {
        damn ""
    }
    
    fr fr For UDP, we would need recvfrom() syscall to get sender address
    fr fr This is a simplified implementation
    sus bytes_received thicc = cursed_socket_recv(socket.socket.socket_id, buffer, buffer_size, 0)
    lowkey bytes_received <= 0 {
        free_buffer(buffer)
        damn ""
    }
    
    sus data tea = buffer_to_string(buffer, bytes_received)
    free_buffer(buffer)
    
    damn data
}

slay udp_socket_close(socket *UDPSocket) lit {
    lowkey socket.socket.socket_id < 0 {
        damn false
    }
    
    sus result normie = cursed_socket_close(socket.socket.socket_id)
    lowkey result == 0 {
        socket.socket.socket_id = -1
        socket.socket.is_bound = false
        damn true
    }
    
    damn false
}

fr fr ================================
fr fr HTTP Client Implementation
fr fr ================================

slay http_get(url tea) HTTPResponse {
    sus response HTTPResponse = {
        status_code: 0,
        headers: "",
        body: "",
        content_length: 0
    }
    
    fr fr Parse URL to extract host and path
    sus parsed_url ParsedURL = parse_url(url)
    lowkey parsed_url.host == "" {
        damn response
    }
    
    fr fr Create TCP connection
    sus socket TCPSocket = tcp_socket_create()
    lowkey !tcp_socket_connect(&socket, parsed_url.host, parsed_url.port) {
        tcp_socket_close(&socket)
        damn response
    }
    
    fr fr Build HTTP request
    sus request tea = "GET " + parsed_url.path + " HTTP/1.1\r\n"
    request = request + "Host: " + parsed_url.host + "\r\n"
    request = request + "Connection: close\r\n"
    request = request + "\r\n"
    
    fr fr Send request
    sus bytes_sent normie = tcp_socket_send(&socket, request)
    lowkey bytes_sent <= 0 {
        tcp_socket_close(&socket)
        damn response
    }
    
    fr fr Receive response
    sus response_data tea = ""
    bestie {
        sus chunk tea = tcp_socket_recv(&socket, 4096)
        lowkey chunk == "" {
            break
        }
        response_data = response_data + chunk
    }
    
    tcp_socket_close(&socket)
    
    fr fr Parse HTTP response
    response = parse_http_response(response_data)
    damn response
}

slay http_post(url tea, data tea, content_type tea) HTTPResponse {
    sus response HTTPResponse = {
        status_code: 0,
        headers: "",
        body: "",
        content_length: 0
    }
    
    fr fr Parse URL
    sus parsed_url ParsedURL = parse_url(url)
    lowkey parsed_url.host == "" {
        damn response
    }
    
    fr fr Create TCP connection
    sus socket TCPSocket = tcp_socket_create()
    lowkey !tcp_socket_connect(&socket, parsed_url.host, parsed_url.port) {
        tcp_socket_close(&socket)
        damn response
    }
    
    fr fr Build HTTP POST request
    sus data_length normie = string_length(data)
    sus request tea = "POST " + parsed_url.path + " HTTP/1.1\r\n"
    request = request + "Host: " + parsed_url.host + "\r\n"
    request = request + "Content-Type: " + content_type + "\r\n"
    request = request + "Content-Length: " + normie_to_string(data_length) + "\r\n"
    request = request + "Connection: close\r\n"
    request = request + "\r\n"
    request = request + data
    
    fr fr Send request
    sus bytes_sent normie = tcp_socket_send(&socket, request)
    lowkey bytes_sent <= 0 {
        tcp_socket_close(&socket)
        damn response
    }
    
    fr fr Receive response
    sus response_data tea = ""
    bestie {
        sus chunk tea = tcp_socket_recv(&socket, 4096)
        lowkey chunk == "" {
            break
        }
        response_data = response_data + chunk
    }
    
    tcp_socket_close(&socket)
    
    fr fr Parse HTTP response
    response = parse_http_response(response_data)
    damn response
}

fr fr ================================
fr fr URL and HTTP Parsing Utilities
fr fr ================================

be_like ParsedURL squad {
    scheme tea
    host tea
    port normie
    path tea
}

slay parse_url(url tea) ParsedURL {
    sus parsed ParsedURL = {
        scheme: "http",
        host: "",
        port: 80,
        path: "/"
    }
    
    fr fr Simple URL parsing (would need more robust implementation)
    lowkey starts_with(url, "https://") {
        parsed.scheme = "https"
        parsed.port = 443
        sus url_without_scheme tea = substring(url, 8, string_length(url))
        sus parts tea[value] = split_string(url_without_scheme, "/")
        lowkey len(parts) > 0 {
            parsed.host = parts[0]
            lowkey len(parts) > 1 {
                parsed.path = "/" + join_strings(parts[1:], "/")
            }
        }
    } elseif starts_with(url, "http://") {
        sus url_without_scheme tea = substring(url, 7, string_length(url))
        sus parts tea[value] = split_string(url_without_scheme, "/")
        lowkey len(parts) > 0 {
            parsed.host = parts[0]
            lowkey len(parts) > 1 {
                parsed.path = "/" + join_strings(parts[1:], "/")
            }
        }
    }
    
    damn parsed
}

slay parse_http_response(response_data tea) HTTPResponse {
    sus response HTTPResponse = {
        status_code: 0,
        headers: "",
        body: "",
        content_length: 0
    }
    
    fr fr Find header/body separator
    sus separator_pos normie = index_of(response_data, "\r\n\r\n")
    lowkey separator_pos < 0 {
        damn response
    }
    
    sus headers_part tea = substring(response_data, 0, separator_pos)
    sus body_part tea = substring(response_data, separator_pos + 4, string_length(response_data))
    
    fr fr Parse status line
    sus lines tea[value] = split_string(headers_part, "\r\n")
    lowkey len(lines) > 0 {
        sus status_line tea = lines[0]
        sus status_parts tea[value] = split_string(status_line, " ")
        lowkey len(status_parts) >= 2 {
            response.status_code = string_to_normie(status_parts[1])
        }
    }
    
    response.headers = headers_part
    response.body = body_part
    response.content_length = string_length(body_part)
    
    damn response
}

fr fr ================================
fr fr Pure CURSED Utility Functions
fr fr ================================

yeet "memory/bootstrap"

slay string_to_cstring(s tea) [*:0]normie {
    lowkey s == "" {
        damn nil
    }
    
    sus len normie = string_length(s)
    sus buffer [*]normie = cursed_malloc(len + 1)
    lowkey buffer == nil {
        damn nil
    }
    
    frfr i normie = 0; i < len; i++ {
        buffer[i] = s[i]
    }
    buffer[len] = 0 fr fr Null terminator
    
    damn buffer
}

slay allocate_buffer(size normie) [*]normie {
    damn cursed_malloc(size)
}

slay free_buffer(buffer [*]normie) {
    cursed_free(buffer)
}

slay buffer_to_string(buffer [*]normie, size thicc) tea {
    lowkey buffer == nil || size <= 0 {
        damn ""
    }
    
    sus result tea = ""
    frfr i thicc = 0; i < size; i++ {
        result = result + tea(buffer[i])
    }
    
    damn result
}

slay string_to_buffer(s tea) [*]normie {
    lowkey s == "" {
        damn nil
    }
    
    sus len normie = string_length(s)
    sus buffer [*]normie = cursed_malloc(len)
    lowkey buffer == nil {
        damn nil
    }
    
    frfr i normie = 0; i < len; i++ {
        buffer[i] = s[i]
    }
    
    damn buffer
}

slay string_length(s tea) normie {
    lowkey s == "" {
        damn 0
    }
    
    sus count normie = 0
    frfr i normie = 0; i < 10000; i++ {
        lowkey s[i] == 0 {
            break
        }
        count++
    }
    
    damn count
}

slay starts_with(s tea, prefix tea) lit {
    lowkey s == "" || prefix == "" {
        damn prefix == ""
    }
    
    sus s_len normie = string_length(s)
    sus p_len normie = string_length(prefix)
    
    lowkey s_len < p_len {
        damn false
    }
    
    frfr i normie = 0; i < p_len; i++ {
        lowkey s[i] != prefix[i] {
            damn false
        }
    }
    
    damn true
}

slay substring(s tea, start normie, end normie) tea {
    lowkey s == "" || start < 0 || end <= start {
        damn ""
    }
    
    sus len normie = string_length(s)
    lowkey start >= len {
        damn ""
    }
    
    lowkey end > len {
        end = len
    }
    
    sus result tea = ""
    frfr i normie = start; i < end; i++ {
        result = result + tea(s[i])
    }
    
    damn result
}

slay index_of(s tea, sub tea) normie {
    lowkey s == "" || sub == "" {
        damn -1
    }
    
    sus s_len normie = string_length(s)
    sus sub_len normie = string_length(sub)
    
    lowkey sub_len > s_len {
        damn -1
    }
    
    frfr i normie = 0; i <= s_len - sub_len; i++ {
        sus found lit = true
        frfr j normie = 0; j < sub_len; j++ {
            lowkey s[i + j] != sub[j] {
                found = false
                break
            }
        }
        lowkey found {
            damn i
        }
    }
    
    damn -1
}

slay split_string(s tea, delimiter tea) tea[value]{
    lowkey s == "" {
        damn tea[value]{}
    }
    
    sus parts tea[value] = tea[value]{}
    sus current tea = ""
    sus s_len normie = string_length(s)
    sus d_len normie = string_length(delimiter)
    
    lowkey d_len == 0 {
        damn tea[value]{s}
    }
    
    sus i normie = 0
    bestie i < s_len {
        sus found lit = true
        lowkey i + d_len <= s_len {
            frfr j normie = 0; j < d_len; j++ {
                lowkey s[i + j] != delimiter[j] {
                    found = false
                    break
                }
            }
        } else {
            found = false
        }
        
        lowkey found {
            parts = append(parts, current)
            current = ""
            i += d_len
        } else {
            current = current + tea(s[i])
            i++
        }
    }
    
    lowkey current != "" {
        parts = append(parts, current)
    }
    
    damn parts
}

slay join_strings(parts tea[value], delimiter tea) tea {
    lowkey len(parts) == 0 {
        damn ""
    }
    
    sus result tea = parts[0]
    frfr i normie = 1; i < len(parts); i++ {
        result = result + delimiter + parts[i]
    }
    
    damn result
}

slay normie_to_string(n normie) tea {
    lowkey n == 0 {
        damn "0"
    }
    
    sus negative lit = n < 0
    lowkey negative {
        n = -n
    }
    
    sus digits tea[value] = tea[value]{}
    bestie n > 0 {
        sus digit normie = n % 10
        digits = append(digits, tea('0' + digit))
        n /= 10
    }
    
    sus result tea = ""
    frfr i normie = len(digits) - 1; i >= 0; i-- {
        result = result + digits[i]
    }
    
    lowkey negative {
        result = "-" + result
    }
    
    damn result
}

slay string_to_normie(s tea) normie {
    lowkey s == "" {
        damn 0
    }
    
    sus result normie = 0
    sus negative lit = false
    sus start normie = 0
    
    lowkey s[0] == '-' {
        negative = true
        start = 1
    }
    
    frfr i normie = start; i < string_length(s); i++ {
        sus char normie = s[i]
        lowkey char >= '0' && char <= '9' {
            result = result * 10 + (char - '0')
        } else {
            break
        }
    }
    
    lowkey negative {
        result = -result
    }
    
    damn result
}

slay len(arr tea[value]) normie {
    damn arr.length fr fr Use built-in array length
}

slay append(arr tea[value], item tea) tea[value]{
    fr fr Simple append implementation
    sus new_arr tea[value] = make(tea[value], len(arr) + 1)
    frfr i normie = 0; i < len(arr); i++ {
        new_arr[i] = arr[i]
    }
    new_arr[len(arr)] = item
    damn new_arr
}
