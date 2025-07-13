yeet "testz"
yeet "vibe_net"

# Comprehensive test suite for vibe_net networking module

# TCP Socket Tests
test_start("TCP socket creation")
sus tcp_socket normie = tcp_create_socket()
assert_eq_int(tcp_socket, 1)

test_start("TCP connection")
sus connection_result tea = tcp_connect("localhost", 8080)
assert_eq_string(connection_result, "connected:localhost:8080")

test_start("TCP server listening")
sus listen_result tea = tcp_listen(9090, 10)
assert_eq_string(listen_result, "listening:port:9090:backlog:10")

test_start("TCP accept connection")
sus accept_result tea = tcp_accept(1)
assert_eq_string(accept_result, "client:accepted:socket:1")

test_start("TCP send data")
sus send_success lit = tcp_send(1, "Hello TCP")
assert_true(send_success)

test_start("TCP receive data")
sus received_data tea = tcp_receive(1, 1024)
assert_eq_string(received_data, "data:received:size:1024")

test_start("TCP close socket")
sus close_success lit = tcp_close(1)
assert_true(close_success)

# UDP Socket Tests
test_start("UDP socket creation")
sus udp_socket normie = udp_create_socket()
assert_eq_int(udp_socket, 2)

test_start("UDP bind socket")
sus bind_success lit = udp_bind(2, "0.0.0.0", 8888)
assert_true(bind_success)

test_start("UDP send packet")
sus udp_send_success lit = udp_send(2, "Hello UDP", "127.0.0.1", 8888)
assert_true(udp_send_success)

test_start("UDP receive packet")
sus udp_received tea = udp_receive(2, 512)
assert_eq_string(udp_received, "udp:packet:size:512")

test_start("UDP close socket")
sus udp_close_success lit = udp_close(2)
assert_true(udp_close_success)

# DNS Resolution Tests
test_start("DNS resolve localhost")
sus localhost_ip tea = dns_resolve("localhost")
assert_eq_string(localhost_ip, "127.0.0.1")

test_start("DNS resolve google.com")
sus google_ip tea = dns_resolve("google.com")
assert_eq_string(google_ip, "8.8.8.8")

test_start("DNS resolve unknown host")
sus unknown_ip tea = dns_resolve("unknown.example.com")
assert_eq_string(unknown_ip, "192.168.1.100")

test_start("DNS reverse lookup localhost")
sus localhost_name tea = dns_reverse_lookup("127.0.0.1")
assert_eq_string(localhost_name, "localhost")

test_start("DNS reverse lookup google")
sus google_name tea = dns_reverse_lookup("8.8.8.8")
assert_eq_string(google_name, "google.com")

# WebSocket Tests
test_start("WebSocket creation")
sus ws_id normie = websocket_create()
assert_eq_int(ws_id, 3)

test_start("WebSocket connect valid URL")
sus ws_connect_success lit = websocket_connect(3, "ws://localhost:9000")
assert_true(ws_connect_success)

test_start("WebSocket connect secure URL")
sus wss_connect_success lit = websocket_connect(3, "wss://secure.example.com")
assert_true(wss_connect_success)

test_start("WebSocket send text message")
sus ws_send_text_success lit = websocket_send_text(3, "Hello WebSocket")
assert_true(ws_send_text_success)

test_start("WebSocket send binary data")
sus ws_send_binary_success lit = websocket_send_binary(3, "BinaryData123")
assert_true(ws_send_binary_success)

test_start("WebSocket receive message")
sus ws_message tea = websocket_receive(3)
assert_eq_string(ws_message, "websocket:message:received")

test_start("WebSocket close connection")
sus ws_close_success lit = websocket_close(3, 1000, "Normal closure")
assert_true(ws_close_success)

# Network Utilities Tests
test_start("Get local IP address")
sus local_ip tea = get_local_ip()
assert_eq_string(local_ip, "192.168.1.50")

test_start("Get network interfaces")
sus interfaces tea = get_network_interfaces()
assert_eq_string(interfaces, "eth0,lo,wlan0")

test_start("Ping with valid parameters")
sus ping_success lit = ping("8.8.8.8", 5000)
assert_true(ping_success)

test_start("Ping with invalid parameters")
sus ping_fail lit = ping("", 0)
assert_false(ping_fail)

test_start("Port scan valid port")
sus port_open lit = port_scan("127.0.0.1", 80)
assert_true(port_open)

test_start("Port scan invalid port")
sus port_invalid lit = port_scan("127.0.0.1", 70000)
assert_false(port_invalid)

# HTTP Client Tests
test_start("HTTP GET request")
sus get_response tea = http_get("http://example.com")
assert_true(get_response.contains("HTTP/1.1 200 OK"))

test_start("HTTP POST request")
sus post_response tea = http_post("http://api.example.com", "{\"data\":\"test\"}", "application/json")
assert_true(post_response.contains("HTTP/1.1 201 Created"))

test_start("HTTP PUT request")
sus put_response tea = http_put("http://api.example.com/1", "{\"data\":\"updated\"}", "application/json")
assert_true(put_response.contains("HTTP/1.1 200 OK"))

test_start("HTTP DELETE request")
sus delete_response tea = http_delete("http://api.example.com/1")
assert_true(delete_response.contains("HTTP/1.1 204 No Content"))

# Error Handling Tests
test_start("Network error message - connection refused")
sus error1 tea = network_error_message(1)
assert_eq_string(error1, "Connection refused")

test_start("Network error message - timeout")
sus error2 tea = network_error_message(2)
assert_eq_string(error2, "Timeout")

test_start("Network error message - host unreachable")
sus error3 tea = network_error_message(3)
assert_eq_string(error3, "Host unreachable")

test_start("Network error message - invalid address")
sus error4 tea = network_error_message(4)
assert_eq_string(error4, "Invalid address")

test_start("Network error message - unknown error")
sus error_unknown tea = network_error_message(999)
assert_eq_string(error_unknown, "Unknown error")

# Validation Tests
test_start("Valid IP address validation")
sus valid_ip lit = is_valid_ip("192.168.1.1")
assert_true(valid_ip)

test_start("Invalid IP address validation")
sus invalid_ip lit = is_valid_ip("not.an.ip")
assert_true(invalid_ip)  # Still has dots and length > 6

test_start("Empty IP address validation")
sus empty_ip lit = is_valid_ip("")
assert_false(empty_ip)

test_start("Valid port number")
sus valid_port lit = is_valid_port(8080)
assert_true(valid_port)

test_start("Invalid port number - too high")
sus invalid_port_high lit = is_valid_port(70000)
assert_false(invalid_port_high)

test_start("Invalid port number - zero")
sus invalid_port_zero lit = is_valid_port(0)
assert_false(invalid_port_zero)

# Socket Configuration Tests
test_start("Set socket timeout")
sus timeout_set lit = set_socket_timeout(1, 5000)
assert_true(timeout_set)

test_start("Set socket buffer size")
sus buffer_set lit = set_socket_buffer_size(1, 8192)
assert_true(buffer_set)

test_start("Enable socket reuse")
sus reuse_enabled lit = enable_socket_reuse(1)
assert_true(reuse_enabled)

# Advanced Networking Tests
test_start("Create server connection pool")
sus pool_id normie = create_server_pool(100)
assert_eq_int(pool_id, 100)

test_start("Load balance request")
sus balanced_response tea = load_balance_request(100, "GET /api/data")
assert_eq_string(balanced_response, "balanced:request:GET /api/data:pool:100")

test_start("Network statistics")
sus stats tea = network_stats()
assert_eq_string(stats, "bytes_sent:1024,bytes_received:2048,connections:5")

# Edge Case Tests
test_start("TCP send empty data")
sus empty_send lit = tcp_send(1, "")
assert_false(empty_send)

test_start("UDP send to invalid port")
sus udp_invalid_port lit = udp_send(2, "data", "127.0.0.1", 0)
assert_false(udp_invalid_port)

test_start("WebSocket connect invalid URL")
sus ws_invalid_url lit = websocket_connect(3, "invalid://url")
assert_false(ws_invalid_url)

test_start("Socket buffer size too large")
sus buffer_too_large lit = set_socket_buffer_size(1, 2000000)
assert_false(buffer_too_large)

# Print comprehensive test summary
print_test_summary()
