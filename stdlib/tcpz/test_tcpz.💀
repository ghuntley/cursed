yeet "testz"
yeet "tcpz"

test_start("TCP Module Tests")

fr fr Test TCP connection to localhost
test_start("tcp_connect - localhost success")
sus result TCPResult = tcp_connect("localhost", 8080)
assert_true(result.success)
assert_true(result.error == "")
assert_true(result.connection.is_connected)
assert_eq_string(result.connection.remote_addr, "localhost:8080")
test_pass("TCP connection to localhost successful")

fr fr Test TCP connection with invalid port
test_start("tcp_connect - invalid port")
sus invalid_result TCPResult = tcp_connect("localhost", 0)
assert_false(invalid_result.success)
assert_true(invalid_result.error != "")
assert_true(str_contains(invalid_result.error, "invalid port"))
test_pass("TCP connection handles invalid port")

fr fr Test TCP connection with port out of range
test_start("tcp_connect - port out of range")
sus range_result TCPResult = tcp_connect("localhost", 70000)
assert_false(range_result.success)
assert_true(str_contains(range_result.error, "invalid port"))
test_pass("TCP connection handles port out of range")

fr fr Test TCP connection to unreachable host
test_start("tcp_connect - unreachable host")
sus unreachable_result TCPResult = tcp_connect("unreachable.example.com", 80)
assert_false(unreachable_result.success)
assert_true(str_contains(unreachable_result.error, "unreachable"))
test_pass("TCP connection handles unreachable host")

fr fr Test TCP connection timeout
test_start("tcp_connect - timeout")
sus timeout_result TCPResult = tcp_connect("timeout.example.com", 80)
assert_false(timeout_result.success)
assert_true(str_contains(timeout_result.error, "timeout"))
test_pass("TCP connection handles timeout")

fr fr Test TCP connection with empty host
test_start("tcp_connect - empty host")
sus empty_result TCPResult = tcp_connect("", 80)
assert_false(empty_result.success)
assert_true(str_contains(empty_result.error, "empty host"))
test_pass("TCP connection handles empty host")

fr fr Test TCP server listening
test_start("tcp_listen - successful bind")
sus server_result TCPServerResult = tcp_listen(8080)
assert_true(server_result.success)
assert_true(server_result.error == "")
assert_true(server_result.server.is_listening)
assert_eq_int(server_result.server.port, 8080)
test_pass("TCP server listening successful")

fr fr Test TCP server with invalid port
test_start("tcp_listen - invalid port")
sus invalid_server TCPServerResult = tcp_listen(0)
assert_false(invalid_server.success)
assert_true(str_contains(invalid_server.error, "invalid port"))
test_pass("TCP server handles invalid port")

fr fr Test TCP server with privileged port
test_start("tcp_listen - privileged port")
sus priv_server TCPServerResult = tcp_listen(80)
assert_false(priv_server.success)
assert_true(str_contains(priv_server.error, "privileged port"))
test_pass("TCP server handles privileged port")

fr fr Test TCP send data
test_start("tcp_send - successful send")
sus conn_result TCPResult = tcp_connect("localhost", 8080)
assert_true(conn_result.success)
sus bytes_sent, send_error = tcp_send(conn_result.connection, "Hello, TCP!")
assert_true(send_error == "")
assert_true(bytes_sent > 0)
test_pass("TCP send data successful")

fr fr Test TCP send with disconnected connection
test_start("tcp_send - disconnected connection")
sus disconnected_conn TCPConnection
disconnected_conn.is_connected = cap
sus disc_bytes, disc_error = tcp_send(disconnected_conn, "test")
assert_eq_int(disc_bytes, 0)
assert_true(str_contains(disc_error, "not established"))
test_pass("TCP send handles disconnected connection")

fr fr Test TCP send with empty data
test_start("tcp_send - empty data")
sus empty_conn_result TCPResult = tcp_connect("localhost", 8080)
sus empty_bytes, empty_error = tcp_send(empty_conn_result.connection, "")
assert_eq_int(empty_bytes, 0)
assert_true(str_contains(empty_error, "no data"))
test_pass("TCP send handles empty data")

fr fr Test TCP receive data
test_start("tcp_receive - successful receive")
sus recv_conn_result TCPResult = tcp_connect("localhost", 8080)
assert_true(recv_conn_result.success)
sus received_data, recv_error = tcp_receive(recv_conn_result.connection)
assert_true(recv_error == "")
assert_true(received_data != "")
assert_true(str_contains(received_data, "TCP response"))
test_pass("TCP receive data successful")

fr fr Test TCP receive from disconnected connection
test_start("tcp_receive - disconnected connection")
sus disconnected_recv TCPConnection
disconnected_recv.is_connected = cap
sus disc_data, disc_recv_error = tcp_receive(disconnected_recv)
assert_eq_string(disc_data, "")
assert_true(str_contains(disc_recv_error, "not established"))
test_pass("TCP receive handles disconnected connection")

fr fr Test TCP accept connection
test_start("tcp_accept - successful accept")
sus accept_server_result TCPServerResult = tcp_listen(9000)
assert_true(accept_server_result.success)
sus client_result TCPResult = tcp_accept(accept_server_result.server)
assert_true(client_result.success)
assert_true(client_result.connection.is_connected)
assert_true(str_contains(client_result.connection.remote_addr, "client:"))
test_pass("TCP accept connection successful")

fr fr Test TCP accept on non-listening server
test_start("tcp_accept - non-listening server")
sus non_listening_server TCPServer
non_listening_server.is_listening = cap
sus no_accept_result TCPResult = tcp_accept(non_listening_server)
assert_false(no_accept_result.success)
assert_true(str_contains(no_accept_result.error, "not listening"))
test_pass("TCP accept handles non-listening server")

fr fr Test TCP connection close
test_start("tcp_close - successful close")
sus close_conn_result TCPResult = tcp_connect("localhost", 8080)
assert_true(close_conn_result.success)
sus closed lit = tcp_close(&close_conn_result.connection)
assert_true(closed)
assert_false(close_conn_result.connection.is_connected)
test_pass("TCP connection close successful")

fr fr Test TCP server close
test_start("tcp_server_close - successful close")
sus close_server_result TCPServerResult = tcp_listen(9001)
assert_true(close_server_result.success)
sus server_closed lit = tcp_server_close(&close_server_result.server)
assert_true(server_closed)
assert_false(close_server_result.server.is_listening)
test_pass("TCP server close successful")

fr fr Test TCP timeout settings
test_start("tcp_set_timeout - valid timeouts")
sus timeout_conn_result TCPResult = tcp_connect("localhost", 8080)
sus timeout_set lit = tcp_set_timeout(&timeout_conn_result.connection, 5000, 5000)
assert_true(timeout_set)
assert_eq_int(timeout_conn_result.connection.read_timeout, 5000)
assert_eq_int(timeout_conn_result.connection.write_timeout, 5000)
test_pass("TCP timeout setting successful")

fr fr Test TCP address functions
test_start("tcp_get_remote_addr")
sus addr_conn_result TCPResult = tcp_connect("github.com", 80)
assert_true(addr_conn_result.success)
sus remote_addr tea = tcp_get_remote_addr(addr_conn_result.connection)
assert_eq_string(remote_addr, "github.com:80")
test_pass("TCP remote address retrieval successful")

fr fr Test TCP connection status
test_start("tcp_is_connected")
sus status_conn_result TCPResult = tcp_connect("localhost", 8080)
assert_true(tcp_is_connected(status_conn_result.connection))
tcp_close(&status_conn_result.connection)
assert_false(tcp_is_connected(status_conn_result.connection))
test_pass("TCP connection status check successful")

fr fr Test hostname validation
test_start("is_valid_hostname")
assert_true(is_valid_hostname("localhost"))
assert_true(is_valid_hostname("example.com"))
assert_true(is_valid_hostname("sub.domain.com"))
assert_false(is_valid_hostname(""))
assert_false(is_valid_hostname("host with spaces"))
test_pass("Hostname validation works correctly")

fr fr Test port availability check
test_start("is_port_available")
assert_false(is_port_available(80))  fr fr Common port likely in use
assert_false(is_port_available(443)) fr fr HTTPS port likely in use
assert_true(is_port_available(8500))  fr fr Development port likely available
test_pass("Port availability check works")

fr fr Test TCP connection pool
test_start("tcp_pool_create")
sus pool TCPConnectionPool = tcp_pool_create("localhost", 8080, 5)
assert_eq_string(pool.host, "localhost")
assert_eq_int(pool.port, 8080)
assert_eq_int(pool.max_size, 5)
assert_eq_int(pool.active_count, 0)
test_pass("TCP connection pool creation successful")

fr fr Test error code messages
test_start("tcp_error_code_to_message")
assert_eq_string(tcp_error_code_to_message(1), "Connection refused")
assert_eq_string(tcp_error_code_to_message(2), "Connection timeout")
assert_eq_string(tcp_error_code_to_message(7), "Address already in use")
assert_eq_string(tcp_error_code_to_message(99), "Unknown error")
test_pass("TCP error code messages work correctly")

print_test_summary()
