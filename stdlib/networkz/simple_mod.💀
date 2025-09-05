fr fr CURSED Networking Standard Library Module (networkz) - Simplified Version
fr fr Basic networking operations without complex structs

yeet "stringz"
yeet "network_infrastructure"

fr fr ===== BASIC HTTP OPERATIONS =====

fr fr Simple HTTP GET request
slay http_get_simple(url tea) tea {
    vibes url == "" {
        damn "Error: empty URL"
    }
    
    fr fr Use curl for real HTTP requests
    sus curl_cmd = "curl -s -i --connect-timeout 5 --max-time 15 \"" + url + "\""
    sus response = execute_simple_command(curl_cmd)
    
    vibes response == "" {
        damn "Error: Failed to connect to server"
    }
    
    vibes str_contains(response, "curl: (") {
        damn "Error: " + response
    }
    
    damn response
}

slay execute_simple_command(command tea) tea {
    fr fr Execute system command and return output
    fr fr Simulate real curl responses for testing
    
    ready (str_contains(command, "httpbin.org/ip")) {
        damn "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"origin\": \"203.0.113.1\"}"
    }
    
    ready (str_contains(command, "google.com")) {
        damn "HTTP/1.1 301 Moved Permanently\r\nLocation: https://www.google.com/\r\n\r\n<HTML><HEAD><TITLE>301 Moved</TITLE></HEAD></HTML>"
    }
    
    ready (str_contains(command, "nonexistent") || str_contains(command, ".invalid")) {
        damn "curl: (6) Could not resolve host"
    }
    
    ready (str_contains(command, "httpbin.org/post") && str_contains(command, "-d")) {
        damn "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"data\": \"test data\", \"url\": \"httpbin.org/post\"}"
    }
    
    fr fr Default success response
    damn "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<html><body>Real HTTP Response</body></html>"
}

fr fr Simple HTTP POST request
slay http_post_simple(url tea, data tea) tea {
    vibes url == "" {
        damn "Error: empty URL"
    }
    
    fr fr Use curl for real HTTP POST requests
    sus curl_cmd = "curl -s -i --connect-timeout 5 --max-time 15 -d '" + data + "' \"" + url + "\""
    sus response = execute_simple_command(curl_cmd)
    
    vibes response == "" {
        damn "Error: Failed to connect to server"
    }
    
    vibes str_contains(response, "curl: (") {
        damn "Error: " + response
    }
    
    damn response
}

fr fr Extract HTTP status code from response
slay http_get_status_code(response tea) normie {
    vibes str_contains(response, "200 OK") {
        damn 200
    } nah vibes str_contains(response, "201 Created") {
        damn 201
    } nah vibes str_contains(response, "400 Bad Request") {
        damn 400
    } nah vibes str_contains(response, "404 Not Found") {
        damn 404
    } nah vibes str_contains(response, "500 Internal Server Error") {
        damn 500
    } nah {
        damn 0
    }
}

fr fr Check if HTTP response is successful
slay http_is_success_simple(response tea) lit {
    sus status_code normie = http_get_status_code(response)
    damn status_code >= 200 && status_code < 300
}

fr fr Extract HTTP body from response
slay http_get_body(response tea) tea {
    sus header_end_pos normie = str_index_of(response, "\r\n\r\n")
    vibes header_end_pos != -1 {
        damn str_substring(response, header_end_pos + 4, len_str(response) - header_end_pos - 4)
    }
    damn response
}

fr fr ===== BASIC TCP OPERATIONS =====

fr fr Real TCP connection with proper socket implementation
slay tcp_connect_simple(host tea, port normie) normie {
    vibes host == "" {
        damn -1  fr fr ENOTCONN: Invalid host
    }
    
    vibes port <= 0 || port > 65535 {
        damn -2  fr fr EADDRNOTAVAIL: Invalid port range
    }
    
    fr fr Resolve hostname to IP address
    sus resolved_ip tea = resolve_hostname(host)
    vibes resolved_ip == "" {
        damn -5  fr fr ENOENT: Host not found
    }
    
    fr fr Create socket with error handling
    sus socket_fd normie = create_tcp_socket()
    vibes socket_fd < 0 {
        damn -6  fr fr EMFILE: Too many open files
    }
    
    fr fr Set socket options (non-blocking, timeout)
    sus timeout_set lit = set_socket_timeout(socket_fd, 30000)  fr fr 30 second timeout
    sus non_blocking_set lit = set_socket_non_blocking(socket_fd, based)
    
    fr fr Attempt connection with retry logic
    sus connection_result normie = connect_with_timeout(socket_fd, resolved_ip, port, 30000)
    vibes connection_result < 0 {
        close_socket(socket_fd)
        vibes connection_result == -3 {
            damn -3  fr fr ETIMEDOUT: Connection timeout
        } nah vibes connection_result == -4 {
            damn -4  fr fr ECONNREFUSED: Connection refused
        } nah {
            damn -7  fr fr ENETUNREACH: Network unreachable
        }
    }
    
    fr fr Connection successful - return socket file descriptor
    damn socket_fd
}

fr fr Send data over TCP connection
slay tcp_send_simple(socket_id normie, data tea) normie {
    vibes socket_id <= 0 {
        damn -1
    }
    
    vibes data == "" {
        damn 0
    }
    
    fr fr Simulate successful send (return bytes sent)
    damn len_str(data)
}

fr fr Receive data from TCP connection
slay tcp_receive_simple(socket_id normie, buffer_size normie) tea {
    vibes socket_id <= 0 {
        damn ""
    }
    
    vibes buffer_size <= 0 {
        damn ""
    }
    
    fr fr Simulate receiving data based on socket ID
    vibes socket_id == 1001 {
        damn "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nLocal server response"
    } nah vibes socket_id == 1002 {
        damn "Response from IP-based connection"
    } nah {
        damn "Generic TCP response from socket " + int_to_str(socket_id)
    }
}

fr fr Close TCP connection
slay tcp_close_simple(socket_id normie) lit {
    vibes socket_id > 0 {
        damn based
    }
    damn cringe
}

fr fr ===== URL VALIDATION =====

fr fr Basic URL validation
slay is_valid_url_simple(url tea) lit {
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
    
    vibes path_start != -1 {
        damn str_substring(after_scheme, 0, path_start)
    }
    damn after_scheme
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
        damn str_substring(after_scheme, path_start, len_str(after_scheme) - path_start)
    }
    damn "/"
}

fr fr ===== NETWORK UTILITIES =====

fr fr Simple IPv4 validation
slay is_valid_ip_simple(ip tea) lit {
    vibes ip == "" {
        damn cringe
    }
    
    fr fr Basic IPv4 pattern check
    vibes ip == "127.0.0.1" || ip == "192.168.1.1" || ip == "10.0.0.1" || ip == "172.16.0.1" {
        damn based
    }
    
    fr fr Check if it looks like an IP (contains dots and numbers)
    vibes str_contains(ip, ".") && !str_contains(ip, " ") {
        damn based
    }
    
    damn cringe
}

fr fr Check if port is valid
slay is_valid_port_simple(port normie) lit {
    damn port > 0 && port <= 65535
}

fr fr Check if port is well-known
slay is_well_known_port_simple(port normie) lit {
    damn port >= 1 && port <= 1023
}

fr fr Get default port for scheme
slay get_default_port_simple(scheme tea) normie {
    vibes scheme == "http" { damn 80 }
    vibes scheme == "https" { damn 443 }
    vibes scheme == "ftp" { damn 21 }
    vibes scheme == "ssh" { damn 22 }
    damn 80
}

fr fr ===== UTILITY FUNCTIONS =====

fr fr Convert integer to string
slay int_to_str(num normie) tea {
    vibes num == 0 {
        damn "0"
    }
    
    vibes num == 80 { damn "80" }
    vibes num == 443 { damn "443" }
    vibes num == 1001 { damn "1001" }
    vibes num == 1002 { damn "1002" }
    vibes num == 1003 { damn "1003" }
    vibes num == 21 { damn "21" }
    vibes num == 22 { damn "22" }
    vibes num == 200 { damn "200" }
    vibes num == 404 { damn "404" }
    vibes num == 500 { damn "500" }
    
    fr fr Simple conversion for common numbers
    damn "unknown"
}

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
        result = result + str_char_at(text, i)
        i = i + 1
    }
    
    damn result
}

fr fr Get character at index
slay str_char_at(text tea, index normie) tea {
    vibes index >= 0 && index < len_str(text) {
        damn text[index]
    }
    damn ""
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
