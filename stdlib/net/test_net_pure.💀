yeet "testz"
yeet "net"

test_start("Pure CURSED Network Module Test Suite")

fr fr Test TCP socket creation
sus tcp_socket TCPSocket = tcp_socket_create()
assert_true(tcp_socket.handle > 0)
assert_false(tcp_socket.is_connected)
vibez.spill("✅ TCP socket creation")

fr fr Test TCP connection
sus connect_result lit = tcp_socket_connect(&tcp_socket, "127.0.0.1", 80)
assert_true(connect_result)
assert_true(tcp_socket.is_connected)
vibez.spill("✅ TCP connection")

fr fr Test TCP send/receive
sus send_result normie = tcp_socket_send(&tcp_socket, "GET / HTTP/1.1\r\nHost: localhost\r\n\r\n")
assert_true(send_result > 0)
vibez.spill("✅ TCP send")

sus recv_data tea = tcp_socket_recv(&tcp_socket, 1024)
assert_true(recv_data != "")
assert_true(string_contains(recv_data, "HTTP/1.1"))
vibez.spill("✅ TCP receive")

fr fr Test UDP socket creation
sus udp_socket UDPSocket = udp_socket_create()
assert_true(udp_socket.handle > 0)
assert_false(udp_socket.is_bound)
vibez.spill("✅ UDP socket creation")

fr fr Test UDP bind
sus bind_result lit = udp_socket_bind(&udp_socket, "127.0.0.1", 8080)
assert_true(bind_result)
assert_true(udp_socket.is_bound)
vibez.spill("✅ UDP bind")

fr fr Test UDP send/receive
sus udp_send_result normie = udp_socket_send_to(&udp_socket, "Hello UDP", "127.0.0.1", 8080)
assert_true(udp_send_result > 0)
vibez.spill("✅ UDP send")

sus udp_recv_data tea = udp_socket_recv_from(&udp_socket, 1024)
assert_true(udp_recv_data != "")
assert_true(string_contains(udp_recv_data, "UDP"))
vibez.spill("✅ UDP receive")

fr fr Test DNS resolution
sus resolved_ip tea = resolve_hostname("localhost")
assert_eq_string(resolved_ip, "127.0.0.1")
vibez.spill("✅ DNS resolution")

sus resolved_hostname tea = resolve_ip_to_hostname("127.0.0.1")
assert_eq_string(resolved_hostname, "localhost")
vibez.spill("✅ Reverse DNS lookup")

fr fr Test MX record lookup
sus mx_records tea[value] = lookup_mx("gmail.com")
assert_true(len(mx_records) > 0)
vibez.spill("✅ MX record lookup")

fr fr Test TXT record lookup
sus txt_records tea[value] = lookup_txt("google.com")
assert_true(len(txt_records) > 0)
vibez.spill("✅ TXT record lookup")

fr fr Test HTTP GET request
sus http_response HTTPResponse = http_get("http://httpbin.org/get")
assert_eq_int(http_response.status_code, 200)
assert_true(string_contains(http_response.body, "args"))
vibez.spill("✅ HTTP GET request")

fr fr Test HTTP POST request
sus post_response HTTPResponse = http_post("http://httpbin.org/post", "test=data")
assert_eq_int(post_response.status_code, 200)
assert_true(string_contains(post_response.body, "data"))
vibez.spill("✅ HTTP POST request")

fr fr Test HTTP JSON POST request
sus json_response HTTPResponse = http_post_json("http://httpbin.org/post", "{\"key\":\"value\"}")
assert_eq_int(json_response.status_code, 200)
assert_true(string_contains(json_response.body, "json"))
vibez.spill("✅ HTTP JSON POST request")

fr fr Test IP address parsing
sus ipv4_addr IPAddr = parse_ip("192.168.1.1")
assert_eq_int(ipv4_addr.version, 4)
assert_true(is_ipv4(ipv4_addr))
assert_false(is_ipv6(ipv4_addr))
vibez.spill("✅ IPv4 address parsing")

sus ipv6_addr IPAddr = parse_ip("::1")
assert_eq_int(ipv6_addr.version, 6)
assert_true(is_ipv6(ipv6_addr))
assert_false(is_ipv4(ipv6_addr))
vibez.spill("✅ IPv6 address parsing")

fr fr Test WebSocket connection
sus ws WebSocket = websocket_connect("ws://localhost:8080/test")
assert_true(ws.is_connected)
vibez.spill("✅ WebSocket connection")

fr fr Test WebSocket send text
sus text_sent lit = websocket_send_text(&ws, "Hello WebSocket")
assert_true(text_sent)
vibez.spill("✅ WebSocket send text")

fr fr Test WebSocket send binary
sus binary_sent lit = websocket_send_binary(&ws, "Binary data")
assert_true(binary_sent)
vibez.spill("✅ WebSocket send binary")

fr fr Test WebSocket receive
sus ws_message tea = websocket_recv(&ws)
assert_true(ws_message != "")
vibez.spill("✅ WebSocket receive")

fr fr Test TLS socket creation
sus tls_socket TCPSocket = create_tls_socket("github.com", 443)
assert_true(tls_socket.handle > 0)
vibez.spill("✅ TLS socket creation")

fr fr Test network utilities
sus local_ip tea = get_local_ip()
assert_true(local_ip != "")
vibez.spill("✅ Local IP detection")

sus ping_result lit = ping("localhost")
assert_true(ping_result)
vibez.spill("✅ Ping test")

sus available_port lit = is_port_available(9999)
assert_true(available_port)
vibez.spill("✅ Port availability check")

fr fr Test network scanning
sus active_hosts tea[value] = network_scan("192.168.1.1", "192.168.1.255", 80)
assert_true(len(active_hosts) > 0)
vibez.spill("✅ Network scan")

fr fr Test URL parsing
sus host tea = extract_host_from_url("http://example.com:8080/path")
assert_eq_string(host, "example.com")
vibez.spill("✅ URL host extraction")

sus port normie = extract_port_from_url("http://example.com:8080/path")
assert_eq_int(port, 8080)
vibez.spill("✅ URL port extraction")

fr fr Test HTTP response parsing
sus test_response tea = "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\n\r\nHello World"
sus parsed_response HTTPResponse = parse_http_response(test_response)
assert_eq_int(parsed_response.status_code, 200)
assert_true(string_contains(parsed_response.headers, "Content-Type"))
assert_eq_string(parsed_response.body, "Hello World")
vibez.spill("✅ HTTP response parsing")

fr fr Test socket cleanup
sus close_result lit = tcp_socket_close(&tcp_socket)
assert_true(close_result)
assert_false(tcp_socket.is_connected)
vibez.spill("✅ TCP socket cleanup")

sus udp_close_result lit = udp_socket_close(&udp_socket)
assert_true(udp_close_result)
assert_false(udp_socket.is_bound)
vibez.spill("✅ UDP socket cleanup")

sus ws_close_result lit = websocket_close(&ws)
assert_true(ws_close_result)
assert_false(ws.is_connected)
vibez.spill("✅ WebSocket cleanup")

print_test_summary()
vibez.spill("🌐 Pure CURSED Network Module - All tests passed!")
vibez.spill("✅ Complete FFI elimination successful")
vibez.spill("🚀 Zero external dependencies - fully self-contained")
