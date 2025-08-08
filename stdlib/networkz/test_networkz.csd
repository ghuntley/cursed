fr fr CURSED Networking Module Test Suite
fr fr Simple testing with basic functionality

yeet "networkz"

fr fr Simple testing framework
sus total_tests normie = 0
sus passed_tests normie = 0

slay test_assert(condition lit, test_name tea) {
    total_tests = total_tests + 1
    vibes condition {
        passed_tests = passed_tests + 1
        vibez.spill("✅ PASS: " + test_name)
    } nah {
        vibez.spill("❌ FAIL: " + test_name)
    }
}

slay print_test_summary() {
    vibez.spill("\n📊 Test Summary:")
    vibez.spill("Total tests: " + int_to_str(total_tests))
    vibez.spill("Passed: " + int_to_str(passed_tests))
    vibez.spill("Failed: " + int_to_str(total_tests - passed_tests))
    
    vibes passed_tests == total_tests {
        vibez.spill("🎉 All tests passed!")
    } nah {
        vibez.spill("⚠️ Some tests failed")
    }
}

slay int_to_str(num normie) tea {
    vibes num == 0 { damn "0" }
    vibes num == 1 { damn "1" }
    vibes num == 2 { damn "2" }
    vibes num == 3 { damn "3" }
    vibes num == 4 { damn "4" }
    vibes num == 5 { damn "5" }
    vibes num == 10 { damn "10" }
    vibes num == 11 { damn "11" }
    vibes num == 20 { damn "20" }
    vibes num == 80 { damn "80" }
    vibes num == 200 { damn "200" }
    vibes num == 404 { damn "404" }
    vibes num == 500 { damn "500" }
    vibes num == 1001 { damn "1001" }
    damn "unknown"
}

vibez.spill("🧪 CURSED Networking Module (networkz) Test Suite")
vibez.spill("")

fr fr ===== HTTP CLIENT TESTS =====

vibez.spill("📡 HTTP Client Tests")

fr fr Test HTTP GET - successful request
sus get_response tea = http_get("http://example.com")
sus get_status normie = http_get_status_code(get_response)
test_assert(get_status == 200, "HTTP GET returns 200 status")
test_assert(str_contains(get_response, "Generic response"), "HTTP GET response contains expected text")

fr fr Test HTTP GET - localhost request
sus local_response tea = http_get("http://localhost:8080")
sus local_status normie = http_get_status_code(local_response)
test_assert(local_status == 200, "HTTP GET localhost returns 200")
test_assert(str_contains(local_response, "Local server"), "Localhost response contains local server text")

fr fr Test HTTP GET - 404 error
sus not_found_response tea = http_get("http://example.com/404")
sus not_found_status normie = http_get_status_code(not_found_response)
test_assert(not_found_status == 404, "HTTP GET 404 returns correct status")
test_assert(str_contains(not_found_response, "Not Found"), "404 response contains Not Found text")

fr fr Test HTTP GET - server error
sus error_response tea = http_get("http://error.example.com")
sus error_status normie = http_get_status_code(error_response)
test_assert(error_status == 500, "HTTP GET error returns 500 status")
test_assert(str_contains(error_response, "Internal Server Error"), "Error response contains expected text")

fr fr Test HTTP GET - empty URL
sus empty_response tea = http_get("")
test_assert(str_contains(empty_response, "Error"), "Empty URL returns error message")

fr fr Test HTTP POST - successful request
sus post_response tea = http_post("http://httpbin.org/post", "name=test&value=123")
sus post_status normie = http_get_status_code(post_response)
test_assert(post_status == 200, "HTTP POST returns 200 status")
test_assert(str_contains(post_response, "name=test&value=123"), "POST response contains submitted data")

fr fr Test HTTP POST - JSON request
sus json_response tea = http_post_json("http://api.example.com", "{\"name\":\"test\"}")
sus json_status normie = http_get_status_code(json_response)
test_assert(json_status == 201, "HTTP POST JSON returns 201 status")

fr fr Test HTTP PUT request
sus put_response tea = http_put("http://api.example.com/resource/1", "name=updated")
sus put_status normie = http_get_status_code(put_response)
test_assert(put_status == 200, "HTTP PUT returns 200 status")

fr fr Test HTTP DELETE request
sus delete_response tea = http_delete("http://api.example.com/resource/1")
sus delete_status normie = http_get_status_code(delete_response)
test_assert(delete_status == 204, "HTTP DELETE returns 204 status")

fr fr ===== HTTP UTILITY FUNCTION TESTS =====

vibez.spill("\n🔧 HTTP Utility Tests")

fr fr Test success detection
sus success_resp tea = http_get("http://example.com")
test_assert(http_is_success(success_resp), "Success response detected correctly")

sus error_resp tea = http_get("http://example.com/404")
test_assert(!http_is_success(error_resp), "Error response detected correctly")

fr fr Test client error detection
test_assert(http_is_client_error(error_resp), "Client error (404) detected correctly")
test_assert(!http_is_client_error(success_resp), "Success response not flagged as client error")

fr fr Test server error detection
sus server_error_resp tea = http_get("http://error.example.com")
test_assert(http_is_server_error(server_error_resp), "Server error (500) detected correctly")
test_assert(!http_is_server_error(success_resp), "Success response not flagged as server error")

fr fr Test status text function
test_assert(http_status_text(200) == "OK", "Status text for 200")
test_assert(http_status_text(404) == "Not Found", "Status text for 404")
test_assert(http_status_text(500) == "Internal Server Error", "Status text for 500")

fr fr Test body extraction
sus body_text tea = http_get_body(success_resp)
test_assert(len_str(body_text) > 0, "Body extraction returns non-empty content")

fr fr Test header extraction
sus content_type tea = http_get_content_type(success_resp)
test_assert(len_str(content_type) > 0, "Content-Type header extracted")

fr fr ===== TCP SOCKET TESTS =====

vibez.spill("\n🔌 TCP Socket Tests")

fr fr Test TCP connection - successful
sus conn_id normie = tcp_connect("localhost", 8080)
test_assert(conn_id > 0, "TCP connection to localhost successful")
test_assert(tcp_is_connected(conn_id), "TCP connection status check works")

fr fr Test TCP connection - IP address
sus ip_conn_id normie = tcp_connect("127.0.0.1", 80)
test_assert(ip_conn_id > 0, "TCP connection to IP address successful")

fr fr Test TCP connection - invalid host
sus invalid_conn normie = tcp_connect("", 80)
test_assert(invalid_conn < 0, "Invalid host handled correctly")

fr fr Test TCP connection - invalid port
sus bad_port_conn normie = tcp_connect("localhost", -1)
test_assert(bad_port_conn < 0, "Invalid port handled correctly")

fr fr Test TCP connection - timeout simulation
sus timeout_conn normie = tcp_connect("timeout.example.com", 80)
test_assert(timeout_conn < 0, "Connection timeout handled correctly")

fr fr Test TCP send
sus bytes_sent normie = tcp_send(conn_id, "Hello, TCP!")
test_assert(bytes_sent == 11, "TCP send returns correct byte count")

fr fr Test TCP send - invalid socket
sus failed_send normie = tcp_send(-1, "test")
test_assert(failed_send == -1, "TCP send to invalid socket fails correctly")

fr fr Test TCP receive
sus received_data tea = tcp_receive(conn_id, 1024)
test_assert(len_str(received_data) > 0, "TCP receive returns data")
test_assert(str_contains(received_data, "Local server"), "TCP receive content is correct")

fr fr Test TCP close
test_assert(tcp_close(conn_id), "TCP connection closes successfully")

fr fr ===== URL PARSING TESTS =====

vibez.spill("\n🔗 URL Parsing Tests")

fr fr Test URL validation
test_assert(is_valid_url("http://example.com"), "Valid HTTP URL recognized")
test_assert(is_valid_url("https://secure.example.com"), "Valid HTTPS URL recognized")
test_assert(!is_valid_url(""), "Empty URL rejected")
test_assert(!is_valid_url("not-a-url"), "Invalid URL rejected")

fr fr Test scheme extraction
test_assert(url_get_scheme("http://example.com") == "http", "HTTP scheme extracted correctly")
test_assert(url_get_scheme("https://secure.com") == "https", "HTTPS scheme extracted correctly")

fr fr Test host extraction
test_assert(url_get_host("http://example.com/path") == "example.com", "Host extracted correctly")

fr fr Test path extraction
test_assert(url_get_path("http://example.com/api/v1") == "/api/v1", "Path extracted correctly")
test_assert(url_get_path("http://example.com") == "/", "Default path for URL without path")

fr fr ===== NETWORK UTILITY TESTS =====

vibez.spill("\n🌐 Network Utility Tests")

fr fr Test IP validation
test_assert(is_valid_ip("127.0.0.1"), "Localhost IP recognized as valid")
test_assert(is_valid_ip("192.168.1.1"), "Private IP recognized as valid")
test_assert(!is_valid_ip(""), "Empty IP rejected")

fr fr Test port validation
test_assert(is_valid_port(80), "Port 80 is valid")
test_assert(is_valid_port(443), "Port 443 is valid")
test_assert(is_valid_port(65535), "Port 65535 is valid")
test_assert(!is_valid_port(0), "Port 0 is invalid")
test_assert(!is_valid_port(-1), "Negative port is invalid")

fr fr Test well-known ports
test_assert(is_well_known_port(80), "Port 80 is well-known")
test_assert(is_well_known_port(443), "Port 443 is well-known")
test_assert(is_well_known_port(22), "Port 22 is well-known")
test_assert(!is_well_known_port(8080), "Port 8080 is not well-known")

fr fr Test default ports
test_assert(get_default_port("http") == 80, "HTTP default port is 80")
test_assert(get_default_port("https") == 443, "HTTPS default port is 443")
test_assert(get_default_port("ftp") == 21, "FTP default port is 21")

fr fr Test scheme from port
test_assert(get_scheme_from_port(80) == "http", "Port 80 maps to HTTP")
test_assert(get_scheme_from_port(443) == "https", "Port 443 maps to HTTPS")
test_assert(get_scheme_from_port(9999) == "unknown", "Unknown port maps to unknown")

fr fr ===== STRING UTILITY TESTS =====

vibez.spill("\n📝 String Utility Tests")

fr fr Test string contains
test_assert(str_contains("hello world", "world"), "String contains substring")
test_assert(!str_contains("hello", "xyz"), "String does not contain non-existent substring")

fr fr Test string starts with
test_assert(str_starts_with("hello world", "hello"), "String starts with prefix")
test_assert(!str_starts_with("hello", "world"), "String does not start with non-matching prefix")

fr fr Test string index of
test_assert(str_index_of("hello world", "world") == 6, "Substring index found correctly")
test_assert(str_index_of("hello", "xyz") == -1, "Non-existent substring returns -1")

fr fr Test string substring
sus substr tea = str_substring("hello world", 6, 5)
test_assert(substr == "world", "Substring extraction works correctly")

fr fr Test string length
test_assert(len_str("hello") == 5, "String length calculation correct")
test_assert(len_str("") == 0, "Empty string length is 0")

vibez.spill("")
print_test_summary()
vibez.spill("")
vibez.spill("🚀 CURSED Networking Module (networkz) testing complete!")
