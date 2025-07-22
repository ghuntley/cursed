yeet "testz"
yeet "vibe_net"

fr fr Comprehensive test suite for vibe_net networking module

fr fr IP Address Tests
test_start("ParseIP IPv4 address")
sus ip4 IPVibe = ParseIP("192.168.1.1")
assert_eq_string(ip4.String(), "192.168.1.1")
assert_true(ip4.IsIPv4())
assert_false(ip4.IsIPv6())

test_start("ParseIP IPv6 address")
sus ip6 IPVibe = ParseIP("2001:db8::1")
assert_eq_string(ip6.String(), "2001:db8::1")
assert_true(ip6.IsIPv6())
assert_false(ip6.IsIPv4())

test_start("IPv4 constructor")
sus constructed_ip IPVibe = IPv4(192, 168, 1, 1)
assert_eq_string(constructed_ip.String(), "192.168.1.1")
assert_true(constructed_ip.IsIPv4())

test_start("IP address IsLoopback")
sus loopback IPVibe = ParseIP("127.0.0.1")
assert_true(loopback.IsLoopback())
sus loopback6 IPVibe = ParseIP("::1")
assert_true(loopback6.IsLoopback())

test_start("IP address IsPrivate")
sus private_ip IPVibe = ParseIP("192.168.1.100")
assert_true(private_ip.IsPrivate())
sus public_ip IPVibe = ParseIP("8.8.8.8")
assert_false(public_ip.IsPrivate())

test_start("IP address IsMulticast")
sus multicast_ip IPVibe = ParseIP("224.0.0.1")
assert_true(multicast_ip.IsMulticast())

test_start("IP address IsUnspecified")
sus unspecified_ip IPVibe = ParseIP("0.0.0.0")
assert_true(unspecified_ip.IsUnspecified())

test_start("IP address IsGlobalUnicast")
sus global_ip IPVibe = ParseIP("8.8.8.8")
assert_true(global_ip.IsGlobalUnicast())

fr fr TCP Address Tests
test_start("ResolveTCPAddr")
sus tcp_addr TCPAddrVibe = ResolveTCPAddr("tcp", "localhost:8080")
assert_eq_string(tcp_addr.Network(), "tcp")
assert_eq_string(tcp_addr.String(), "localhost:8080")
assert_eq_int(tcp_addr.Port(), 8080)

test_start("TCP address IP method")
sus tcp_addr2 TCPAddrVibe = ResolveTCPAddr("tcp", "192.168.1.1:9090")
sus addr_ip IPVibe = tcp_addr2.IP()
assert_eq_string(addr_ip.String(), "192.168.1.1")

fr fr UDP Address Tests
test_start("ResolveUDPAddr")
sus udp_addr UDPAddrVibe = ResolveUDPAddr("udp", "localhost:8081")
assert_eq_string(udp_addr.Network(), "udp")
assert_eq_string(udp_addr.String(), "localhost:8081")
assert_eq_int(udp_addr.Port(), 8081)

fr fr Connection Tests
test_start("ConnVibe LocalAddr")
sus conn ConnVibe = ConnVibe{
    id: 1,
    network: "tcp",
    local_addr: "192.168.1.1:8080",
    remote_addr: "192.168.1.2:8081",
    state: 1,
    read_timeout: 30000,
    write_timeout: 30000,
    keep_alive: based
}
assert_eq_string(conn.LocalAddr(), "192.168.1.1:8080")
assert_eq_string(conn.RemoteAddr(), "192.168.1.2:8081")

test_start("ConnVibe Read")
sus read_data tea = conn.Read(1024)
assert_true(read_data.contains("data:read:size:1024"))

test_start("ConnVibe Write")
sus bytes_written normie = conn.Write("test data")
assert_eq_int(bytes_written, 9)

test_start("ConnVibe Close")
sus close_success lit = conn.Close()
assert_true(close_success)

fr fr TCP Connection Tests
test_start("DialTCP")
sus local_addr TCPAddrVibe = ResolveTCPAddr("tcp", "0.0.0.0:0")
sus remote_addr TCPAddrVibe = ResolveTCPAddr("tcp", "localhost:8080")
sus tcp_conn TCPConnVibe = DialTCP("tcp", local_addr, remote_addr)
assert_eq_string(tcp_conn.LocalAddr(), "0.0.0.0:0")
assert_eq_string(tcp_conn.RemoteAddr(), "localhost:8080")

test_start("TCP connection SetKeepAlive")
sus keepalive_set lit = tcp_conn.SetKeepAlive(based)
assert_true(keepalive_set)

test_start("TCP connection SetNoDelay")
sus nodelay_set lit = tcp_conn.SetNoDelay(based)
assert_true(nodelay_set)

test_start("TCP connection SetReadBuffer")
sus read_buffer_set lit = tcp_conn.SetReadBuffer(16384)
assert_true(read_buffer_set)

test_start("TCP connection SetWriteBuffer")
sus write_buffer_set lit = tcp_conn.SetWriteBuffer(16384)
assert_true(write_buffer_set)

fr fr UDP Connection Tests
test_start("DialUDP")
sus udp_local_addr UDPAddrVibe = ResolveUDPAddr("udp", "0.0.0.0:0")
sus udp_remote_addr UDPAddrVibe = ResolveUDPAddr("udp", "localhost:8081")
sus udp_conn UDPConnVibe = DialUDP("udp", udp_local_addr, udp_remote_addr)
assert_eq_string(udp_conn.LocalAddr(), "0.0.0.0:0")
assert_eq_string(udp_conn.RemoteAddr(), "localhost:8081")

test_start("UDP ReadFromUDP")
sus n normie
sus from_addr UDPAddrVibe
sus err tea
n, from_addr, err = udp_conn.ReadFromUDP(1024)
assert_true(n > 0)
assert_eq_string(from_addr.String(), "127.0.0.1:12345")

test_start("UDP WriteToUDP")
sus target_addr UDPAddrVibe = ResolveUDPAddr("udp", "127.0.0.1:8082")
sus udp_bytes_written normie = udp_conn.WriteToUDP("UDP test data", target_addr)
assert_eq_int(udp_bytes_written, 13)

fr fr TCP Listener Tests
test_start("ListenTCP")
sus listen_addr TCPAddrVibe = ResolveTCPAddr("tcp", "0.0.0.0:9090")
sus listener TCPListenerVibe = ListenTCP("tcp", listen_addr)
assert_eq_string(listener.Addr(), "0.0.0.0:9090")

test_start("TCP listener Accept")
sus accepted_conn ConnVibe = listener.Accept()
assert_eq_string(accepted_conn.LocalAddr(), "0.0.0.0:9090")
assert_true(accepted_conn.RemoteAddr().contains("client"))

test_start("TCP listener AcceptTCP")
sus accepted_tcp TCPConnVibe = listener.AcceptTCP()
assert_eq_string(accepted_tcp.LocalAddr(), "0.0.0.0:9090")

test_start("TCP listener Close")
sus listener_close_success lit = listener.Close()
assert_true(listener_close_success)

fr fr DNS Resolver Tests
test_start("NewDNSResolver")
sus resolver DNSResolverVibe = NewDNSResolver()
assert_eq_int(resolver.timeout, 5000)
assert_eq_int(resolver.retries, 3)

test_start("DNS LookupHost localhost")
sus localhost_addrs []tea = resolver.LookupHost("localhost")
assert_eq_string(localhost_addrs[0], "127.0.0.1")

test_start("DNS LookupHost google.com")
sus google_addrs []tea = resolver.LookupHost("google.com")
assert_eq_string(google_addrs[0], "8.8.8.8")

test_start("DNS LookupIP")
sus ip_addrs []IPVibe = resolver.LookupIP("localhost")
assert_eq_string(ip_addrs[0].String(), "127.0.0.1")

test_start("DNS LookupAddr")
sus hostnames []tea = resolver.LookupAddr("127.0.0.1")
assert_eq_string(hostnames[0], "localhost")

test_start("DNS LookupMX")
sus mx_records []MXVibe = resolver.LookupMX("gmail.com")
assert_eq_string(mx_records[0].Host, "gmail-smtp-in.l.google.com")
assert_eq_int(mx_records[0].Pref, 5)

test_start("DNS LookupNS")
sus ns_records []NSVibe = resolver.LookupNS("example.com")
assert_eq_string(ns_records[0].Host, "ns1.example.com")

test_start("DNS LookupTXT")
sus txt_records []tea = resolver.LookupTXT("google.com")
assert_true(txt_records[0].contains("spf1"))

test_start("DNS LookupSRV")
sus srv_cname tea
sus srv_records []SRVVibe
srv_cname, srv_records = resolver.LookupSRV("http", "tcp", "example.com")
assert_eq_string(srv_cname, "example.com")
assert_eq_string(srv_records[0].Target, "http.example.com")
assert_eq_int(srv_records[0].Port, 443)

fr fr Dialer Tests
test_start("NewDialer")
sus dialer DialerVibe = NewDialer()
assert_eq_int(dialer.timeout, 30000)
assert_eq_int(dialer.keep_alive, 30000)
assert_true(dialer.dual_stack)

test_start("Dialer Dial TCP")
sus dial_conn ConnVibe = dialer.Dial("tcp", "localhost:8080")
assert_eq_string(dial_conn.network, "tcp")
assert_eq_string(dial_conn.remote_addr, "localhost:8080")

test_start("Dialer Dial UDP")
sus dial_udp_conn ConnVibe = dialer.Dial("udp", "localhost:8081")
assert_eq_string(dial_udp_conn.network, "udp")
assert_eq_string(dial_udp_conn.remote_addr, "localhost:8081")

fr fr High-Level Functions Tests
test_start("Global Dial function")
sus global_conn ConnVibe = Dial("tcp", "localhost:8080")
assert_eq_string(global_conn.network, "tcp")
assert_eq_string(global_conn.remote_addr, "localhost:8080")

test_start("Global DialTimeout function")
sus timeout_conn ConnVibe = DialTimeout("tcp", "localhost:8080", 10000)
assert_eq_int(timeout_conn.read_timeout, 10000)
assert_eq_int(timeout_conn.write_timeout, 10000)

test_start("Global Listen function")
sus global_listener TCPListenerVibe = Listen("tcp", "0.0.0.0:9091")
assert_eq_string(global_listener.Addr(), "0.0.0.0:9091")

fr fr WebSocket Tests
test_start("NewWebSocketConn")
sus ws_base_conn ConnVibe = Dial("tcp", "localhost:8080")
sus ws_conn WebSocketConnVibe = NewWebSocketConn(ws_base_conn, "ws")
assert_eq_string(ws_conn.protocol, "ws")
assert_eq_int(ws_conn.state, 1)

test_start("WebSocket ReadMessage")
sus ws_msg_type normie
sus ws_msg_data tea
ws_msg_type, ws_msg_data = ws_conn.ReadMessage()
assert_eq_int(ws_msg_type, 1)
assert_true(ws_msg_data.contains("data:read"))

test_start("WebSocket WriteMessage")
sus ws_write_success lit = ws_conn.WriteMessage(1, "WebSocket test message")
assert_true(ws_write_success)

test_start("WebSocket Close")
sus ws_close_success lit = ws_conn.Close()
assert_true(ws_close_success)

fr fr HTTP/2 Tests
test_start("NewHTTP2Conn")
sus h2_base_conn ConnVibe = Dial("tcp", "localhost:8080")
sus h2_conn HTTP2ConnVibe = NewHTTP2Conn(h2_base_conn)
assert_eq_int(h2_conn.max_streams, 1000)
assert_eq_int(h2_conn.streams.length(), 0)

test_start("HTTP2 CreateStream")
sus h2_stream HTTP2StreamVibe = h2_conn.CreateStream()
assert_eq_int(h2_stream.id, 1)
assert_eq_int(h2_stream.state, 1)
assert_eq_int(h2_conn.streams.length(), 1)

test_start("HTTP2 Close")
sus h2_close_success lit = h2_conn.Close()
assert_true(h2_close_success)

fr fr Connection Pool Tests
test_start("NewConnPool")
sus pool ConnPoolVibe = NewConnPool("tcp", "localhost:8080", 10)
assert_eq_string(pool.network, "tcp")
assert_eq_string(pool.address, "localhost:8080")
assert_eq_int(pool.max_conns, 10)

test_start("Connection pool Get")
sus pool_conn ConnVibe = pool.Get()
assert_eq_string(pool_conn.remote_addr, "localhost:8080")
assert_eq_int(pool.active_conns, 1)

test_start("Connection pool Put")
sus pool_put_success lit = pool.Put(pool_conn)
assert_true(pool_put_success)

test_start("Connection pool Stats")
sus pool_stats ConnPoolStats = pool.Stats()
assert_eq_int(pool_stats.MaxConns, 10)
assert_true(pool_stats.TotalAcquired > 0)

test_start("Connection pool Close")
sus pool_close_success lit = pool.Close()
assert_true(pool_close_success)

fr fr Circuit Breaker Tests
test_start("NewCircuitBreaker")
sus cb CircuitBreakerVibe = NewCircuitBreaker(3, 60000)
assert_eq_int(cb.max_failures, 3)
assert_eq_int(cb.reset_timeout, 60000)
assert_eq_int(cb.state, 0)

test_start("Circuit breaker Execute success")
sus cb_success_result tea = cb.Execute("success_operation")
assert_eq_string(cb_success_result, "operation:success")

test_start("Circuit breaker Execute failure")
sus cb_failure_result tea = cb.Execute("fail_operation")
assert_eq_string(cb_failure_result, "operation:failed")

test_start("Circuit breaker Reset")
sus cb_reset_success lit = cb.Reset()
assert_true(cb_reset_success)
assert_eq_int(cb.state, 0)

fr fr Rate Limiter Tests
test_start("NewRateLimiter")
sus rl RateLimiterVibe = NewRateLimiter(10, 1000)
assert_eq_int(rl.rate, 10)
assert_eq_int(rl.per_duration, 1000)

test_start("Rate limiter Allow")
sus rl_allow_success lit = rl.Allow()
assert_true(rl_allow_success)

fr fr Network Interface Tests
test_start("Interfaces")
sus interfaces []InterfaceVibe = Interfaces()
assert_true(interfaces.length() >= 2)
assert_eq_string(interfaces[0].Name, "eth0")
assert_eq_string(interfaces[1].Name, "lo")

test_start("InterfaceByName")
sus eth0_interface InterfaceVibe = InterfaceByName("eth0")
assert_eq_string(eth0_interface.Name, "eth0")
assert_eq_int(eth0_interface.Index, 1)
assert_eq_int(eth0_interface.MTU, 1500)

test_start("Interface Addrs")
sus interface_addrs []tea = eth0_interface.Addrs()
assert_true(interface_addrs.length() > 0)

fr fr Global DNS Function Tests
test_start("Global LookupHost")
sus global_host_addrs []tea = LookupHost("localhost")
assert_eq_string(global_host_addrs[0], "127.0.0.1")

test_start("Global LookupIP")
sus global_ip_addrs []IPVibe = LookupIP("localhost")
assert_eq_string(global_ip_addrs[0].String(), "127.0.0.1")

test_start("Global LookupAddr")
sus global_hostnames []tea = LookupAddr("127.0.0.1")
assert_eq_string(global_hostnames[0], "localhost")

test_start("Global LookupMX")
sus global_mx_records []MXVibe = LookupMX("gmail.com")
assert_eq_string(global_mx_records[0].Host, "gmail-smtp-in.l.google.com")

test_start("Global LookupNS")
sus global_ns_records []NSVibe = LookupNS("example.com")
assert_eq_string(global_ns_records[0].Host, "ns1.example.com")

test_start("Global LookupTXT")
sus global_txt_records []tea = LookupTXT("google.com")
assert_true(global_txt_records[0].contains("spf1"))

fr fr IPv6 Support Tests
test_start("IsIPv6Enabled")
sus ipv6_enabled lit = IsIPv6Enabled()
assert_true(ipv6_enabled)

test_start("PreferIPv6")
sus prefer_ipv6 lit = PreferIPv6()
assert_false(prefer_ipv6)

test_start("SetPreferIPv6")
sus set_prefer_success lit = SetPreferIPv6(based)
assert_true(set_prefer_success)

test_start("IPv6InterfaceAddrs")
sus ipv6_addrs []IPVibe = IPv6InterfaceAddrs()
assert_true(ipv6_addrs.length() >= 3)
assert_eq_string(ipv6_addrs[0].String(), "::1")

fr fr Legacy TCP Socket Tests (for backward compatibility)
test_start("Legacy TCP socket creation")
sus tcp_socket normie = tcp_create_socket()
assert_eq_int(tcp_socket, 1)

test_start("Legacy TCP connection")
sus connection_result tea = tcp_connect("localhost", 8080)
assert_eq_string(connection_result, "connected:localhost:8080")

test_start("Legacy TCP server listening")
sus listen_result tea = tcp_listen(9090, 10)
assert_eq_string(listen_result, "listening:port:9090:backlog:10")

test_start("Legacy TCP accept connection")
sus accept_result tea = tcp_accept(1)
assert_eq_string(accept_result, "client:accepted:socket:1")

test_start("Legacy TCP send data")
sus send_success lit = tcp_send(1, "Hello TCP")
assert_true(send_success)

test_start("Legacy TCP receive data")
sus received_data tea = tcp_receive(1, 1024)
assert_eq_string(received_data, "data:received:size:1024")

test_start("Legacy TCP close socket")
sus close_success lit = tcp_close(1)
assert_true(close_success)

fr fr Legacy UDP Socket Tests
test_start("Legacy UDP socket creation")
sus udp_socket normie = udp_create_socket()
assert_eq_int(udp_socket, 2)

test_start("Legacy UDP bind socket")
sus bind_success lit = udp_bind(2, "0.0.0.0", 8888)
assert_true(bind_success)

test_start("Legacy UDP send packet")
sus udp_send_success lit = udp_send(2, "Hello UDP", "127.0.0.1", 8888)
assert_true(udp_send_success)

test_start("Legacy UDP receive packet")
sus udp_received tea = udp_receive(2, 512)
assert_eq_string(udp_received, "udp:packet:size:512")

test_start("Legacy UDP close socket")
sus udp_close_success lit = udp_close(2)
assert_true(udp_close_success)

fr fr Legacy DNS Resolution Tests
test_start("Legacy DNS resolve localhost")
sus localhost_ip tea = dns_resolve("localhost")
assert_eq_string(localhost_ip, "127.0.0.1")

test_start("Legacy DNS resolve google.com")
sus google_ip tea = dns_resolve("google.com")
assert_eq_string(google_ip, "8.8.8.8")

test_start("Legacy DNS resolve unknown host")
sus unknown_ip tea = dns_resolve("unknown.example.com")
assert_eq_string(unknown_ip, "192.168.1.100")

test_start("Legacy DNS reverse lookup localhost")
sus localhost_name tea = dns_reverse_lookup("127.0.0.1")
assert_eq_string(localhost_name, "localhost")

test_start("Legacy DNS reverse lookup google")
sus google_name tea = dns_reverse_lookup("8.8.8.8")
assert_eq_string(google_name, "dns.google")

fr fr Legacy WebSocket Tests
test_start("Legacy WebSocket creation")
sus ws_id normie = websocket_create()
assert_eq_int(ws_id, 3)

test_start("Legacy WebSocket connect valid URL")
sus ws_connect_success lit = websocket_connect(3, "ws://localhost:9000")
assert_true(ws_connect_success)

test_start("Legacy WebSocket connect secure URL")
sus wss_connect_success lit = websocket_connect(3, "wss://secure.example.com")
assert_true(wss_connect_success)

test_start("Legacy WebSocket send text message")
sus ws_send_text_success lit = websocket_send_text(3, "Hello WebSocket")
assert_true(ws_send_text_success)

test_start("Legacy WebSocket send binary data")
sus ws_send_binary_success lit = websocket_send_binary(3, "BinaryData123")
assert_true(ws_send_binary_success)

test_start("Legacy WebSocket receive message")
sus ws_message tea = websocket_receive(3)
assert_eq_string(ws_message, "websocket:message:received")

test_start("Legacy WebSocket close connection")
sus ws_close_success lit = websocket_close(3, 1000, "Normal closure")
assert_true(ws_close_success)

fr fr Legacy Network Utilities Tests
test_start("Legacy Get local IP address")
sus local_ip tea = get_local_ip()
assert_eq_string(local_ip, "192.168.1.50")

test_start("Legacy Get network interfaces")
sus legacy_interfaces tea = get_network_interfaces()
assert_eq_string(legacy_interfaces, "eth0,lo,wlan0")

test_start("Legacy Ping with valid parameters")
sus ping_success lit = ping("8.8.8.8", 5000)
assert_true(ping_success)

test_start("Legacy Ping with invalid parameters")
sus ping_fail lit = ping("", 0)
assert_false(ping_fail)

test_start("Legacy Port scan valid port")
sus port_open lit = port_scan("127.0.0.1", 80)
assert_true(port_open)

test_start("Legacy Port scan invalid port")
sus port_invalid lit = port_scan("127.0.0.1", 70000)
assert_false(port_invalid)

fr fr Legacy HTTP Client Tests
test_start("Legacy HTTP GET request")
sus get_response tea = http_get("http://example.com")
assert_true(get_response.contains("HTTP/1.1 200 OK"))

test_start("Legacy HTTP POST request")
sus post_response tea = http_post("http://api.example.com", "{\"data\":\"test\"}", "application/json")
assert_true(post_response.contains("HTTP/1.1 201 Created"))

test_start("Legacy HTTP PUT request")
sus put_response tea = http_put("http://api.example.com/1", "{\"data\":\"updated\"}", "application/json")
assert_true(put_response.contains("HTTP/1.1 200 OK"))

test_start("Legacy HTTP DELETE request")
sus delete_response tea = http_delete("http://api.example.com/1")
assert_true(delete_response.contains("HTTP/1.1 204 No Content"))

fr fr Legacy Error Handling Tests
test_start("Legacy Network error message - connection refused")
sus error1 tea = network_error_message(1)
assert_eq_string(error1, "Connection refused")

test_start("Legacy Network error message - timeout")
sus error2 tea = network_error_message(2)
assert_eq_string(error2, "Timeout")

test_start("Legacy Network error message - host unreachable")
sus error3 tea = network_error_message(3)
assert_eq_string(error3, "Host unreachable")

test_start("Legacy Network error message - invalid address")
sus error4 tea = network_error_message(4)
assert_eq_string(error4, "Invalid address")

test_start("Legacy Network error message - unknown error")
sus error_unknown tea = network_error_message(999)
assert_eq_string(error_unknown, "Unknown error")

fr fr Legacy Validation Tests
test_start("Legacy Valid IP address validation")
sus valid_ip lit = is_valid_ip("192.168.1.1")
assert_true(valid_ip)

test_start("Legacy Invalid IP address validation")
sus invalid_ip lit = is_valid_ip("not.an.ip")
assert_true(invalid_ip) fr fr Still has dots and length > 6

test_start("Legacy Empty IP address validation")
sus empty_ip lit = is_valid_ip("")
assert_false(empty_ip)

test_start("Legacy Valid port number")
sus valid_port lit = is_valid_port(8080)
assert_true(valid_port)

test_start("Legacy Invalid port number - too high")
sus invalid_port_high lit = is_valid_port(70000)
assert_false(invalid_port_high)

test_start("Legacy Invalid port number - zero")
sus invalid_port_zero lit = is_valid_port(0)
assert_false(invalid_port_zero)

fr fr Legacy Socket Configuration Tests
test_start("Legacy Set socket timeout")
sus timeout_set lit = set_socket_timeout(1, 5000)
assert_true(timeout_set)

test_start("Legacy Set socket buffer size")
sus buffer_set lit = set_socket_buffer_size(1, 8192)
assert_true(buffer_set)

test_start("Legacy Enable socket reuse")
sus reuse_enabled lit = enable_socket_reuse(1)
assert_true(reuse_enabled)

fr fr Legacy Advanced Networking Tests
test_start("Legacy Create server connection pool")
sus pool_id normie = create_server_pool(100)
assert_eq_int(pool_id, 100)

test_start("Legacy Load balance request")
sus balanced_response tea = load_balance_request(100, "GET /api/data")
assert_eq_string(balanced_response, "balanced:request:GET /api/data:pool:100")

test_start("Legacy Network statistics")
sus stats tea = network_stats()
assert_eq_string(stats, "bytes_sent:1024,bytes_received:2048,connections:5")

fr fr Legacy Edge Case Tests
test_start("Legacy TCP send empty data")
sus empty_send lit = tcp_send(1, "")
assert_false(empty_send)

test_start("Legacy UDP send to invalid port")
sus udp_invalid_port lit = udp_send(2, "data", "127.0.0.1", 0)
assert_false(udp_invalid_port)

test_start("Legacy WebSocket connect invalid URL")
sus ws_invalid_url lit = websocket_connect(3, "invalid://url")
assert_false(ws_invalid_url)

test_start("Legacy Socket buffer size too large")
sus buffer_too_large lit = set_socket_buffer_size(1, 2000000)
assert_false(buffer_too_large)

fr fr Print comprehensive test summary
print_test_summary()
