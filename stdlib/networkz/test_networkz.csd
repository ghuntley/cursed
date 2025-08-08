yeet "testz"
yeet "networkz"

fr fr Networking Module Test Suite

test_start("DNS Resolution Tests")

fr fr Test hostname resolution
sus (ip, err) = resolve_hostname("localhost")
assert_eq_string(ip, "127.0.0.1")
assert_eq_string(err, "")

sus (ip2, err2) = resolve_hostname("example.com")
assert_eq_string(ip2, "93.184.216.34")
assert_eq_string(err2, "")

fr fr Test invalid hostname
sus (ip3, err3) = resolve_hostname("")
assert_eq_string(ip3, "")
assert_true(len(err3) > 0)

test_start("Network Validation Functions")

fr fr Test IP validation
assert_true(is_valid_ip("127.0.0.1"))
assert_true(is_valid_ip("192.168.1.1"))
assert_false(is_valid_ip(""))
assert_false(is_valid_ip("invalid"))

fr fr Test hostname validation
assert_true(is_valid_hostname("example.com"))
assert_true(is_valid_hostname("localhost"))
assert_false(is_valid_hostname(""))

test_start("TCP Connection Tests")

fr fr Test TCP connection to localhost
sus tcp_result NetworkResult = tcp_connect("localhost", 8080)
assert_true(tcp_result.success)
assert_eq_string(tcp_result.error, "")
assert_true(tcp_result.socket_id > 0)

fr fr Test invalid port
sus tcp_result2 NetworkResult = tcp_connect("localhost", 0)
assert_false(tcp_result2.success)
assert_true(len(tcp_result2.error) > 0)

fr fr Test empty host
sus tcp_result3 NetworkResult = tcp_connect("", 8080)
assert_false(tcp_result3.success)
assert_true(len(tcp_result3.error) > 0)

test_start("Data Transfer Tests")

fr fr Test TCP send
sus send_result TransferResult = tcp_send(12345, "Hello, Server!")
assert_eq_string(send_result.error, "")
assert_true(send_result.bytes_transferred > 0)

fr fr Test invalid socket for send
sus send_result2 TransferResult = tcp_send(0, "data")
assert_true(len(send_result2.error) > 0)
assert_eq_int(send_result2.bytes_transferred, 0)

fr fr Test TCP receive
sus recv_result TransferResult = tcp_receive(12345, 1024)
assert_eq_string(recv_result.error, "")
assert_true(recv_result.bytes_transferred > 0)
assert_true(len(recv_result.data) > 0)

test_start("UDP Operations Tests")

fr fr Test UDP socket creation
sus udp_result NetworkResult = udp_socket()
assert_true(udp_result.success)
assert_eq_string(udp_result.error, "")
assert_true(udp_result.socket_id > 0)

fr fr Test UDP send
sus udp_send_result TransferResult = udp_send_to(54321, "localhost", 9999, "UDP message")
assert_eq_string(udp_send_result.error, "")
assert_true(udp_send_result.bytes_transferred > 0)

test_start("Utility Functions Tests")

fr fr Test string contains function
assert_true(contains_str("hello world", "world"))
assert_true(contains_str("test string", "test"))
assert_false(contains_str("hello", "world"))
assert_false(contains_str("short", "longer"))

fr fr Test local IP retrieval
sus local_ip tea = get_local_ip()
assert_eq_string(local_ip, "127.0.0.1")

print_test_summary()
