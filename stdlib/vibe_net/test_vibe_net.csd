yeet "testz"
yeet "vibe_net"

# TCP Connection Tests
test_start("TCP connect function")
assert_true(vibe_net.tcp_connect("localhost", 8080))
assert_true(vibe_net.tcp_connect("127.0.0.1", 9000))
assert_false(vibe_net.tcp_connect("", 8080))
assert_false(vibe_net.tcp_connect("localhost", 0))
assert_false(vibe_net.tcp_connect("localhost", 70000))

test_start("TCP listen function")
assert_true(vibe_net.tcp_listen("0.0.0.0", 8080))
assert_true(vibe_net.tcp_listen("localhost", 3000))
assert_false(vibe_net.tcp_listen("", 8080))
assert_false(vibe_net.tcp_listen("localhost", -1))

# HTTP Client Tests
test_start("HTTP GET function")
sus get_response tea = vibe_net.http_get("http://example.com")
assert_true(get_response != "")
assert_true(get_response == "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello, World!")

sus invalid_get tea = vibe_net.http_get("http://invalid.url")
assert_true(invalid_get == "HTTP/1.1 404 Not Found\r\nContent-Length: 9\r\n\r\nNot Found")

test_start("HTTP POST function")
sus post_response tea = vibe_net.http_post("http://example.com/api", "test data")
assert_true(post_response != "")
assert_true(post_response == "HTTP/1.1 201 Created\r\nContent-Length: 7\r\n\r\nCreated")

sus invalid_post tea = vibe_net.http_post("", "test data")
assert_true(invalid_post == "HTTP/1.1 400 Bad Request\r\nContent-Length: 11\r\n\r\nBad Request")

# Network Utility Tests
test_start("Network availability function")
assert_true(vibe_net.network_available())

test_start("Hostname resolution function")
sus localhost_ip tea = vibe_net.resolve_hostname("localhost")
assert_eq_string(localhost_ip, "127.0.0.1")

sus example_ip tea = vibe_net.resolve_hostname("example.com")
assert_eq_string(example_ip, "93.184.216.34")

sus github_ip tea = vibe_net.resolve_hostname("github.com")
assert_eq_string(github_ip, "140.82.114.4")

sus unknown_ip tea = vibe_net.resolve_hostname("unknown.domain")
assert_eq_string(unknown_ip, "0.0.0.0")

# Additional Utility Tests
test_start("Local IP function")
sus local_ip tea = vibe_net.get_local_ip()
assert_eq_string(local_ip, "192.168.1.100")

test_start("Ping host function")
assert_true(vibe_net.ping_host("localhost"))
assert_true(vibe_net.ping_host("example.com"))
assert_false(vibe_net.ping_host("unreachable.host"))

test_start("HTTP header parsing function")
sus headers tea = vibe_net.parse_http_headers("HTTP/1.1 200 OK")
assert_eq_string(headers, "Content-Type: text/plain")

sus empty_headers tea = vibe_net.parse_http_headers("")
assert_eq_string(empty_headers, "")

test_start("HTTP request building function")
sus request tea = vibe_net.build_http_request("GET", "/api/data", "Host: example.com")
sus expected tea = "GET /api/data HTTP/1.1\r\nHost: example.com\r\n\r\n"
assert_eq_string(request, expected)

print_test_summary()
