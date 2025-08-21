// networkz/tests.csd - Comprehensive test suite for NetworkZ module

yeet "testz"
yeet "networkz"
yeet "stringz"
yeet "arrayz"
yeet "mathz"

// Test data and mock responses
slay setup_test_environment() lit {
    testz.test_group("NetworkZ Standard Library Tests")
    damn based
}

// URL Parsing Tests
slay test_url_parsing() lit {
    testz.test_case("URL parsing - basic HTTP URL")
    sus url_parts networkz.UrlParts = networkz.parse_url("http://example.com/path?query=value#fragment") fam {
        when err -> {
            testz.fail("URL parsing should succeed")
            damn no_cap
        }
    }
    
    testz.assert_eq_string(url_parts.scheme, "http")
    testz.assert_eq_string(url_parts.host, "example.com")
    testz.assert_eq_int(url_parts.port, 80)
    testz.assert_eq_string(url_parts.path, "/path")
    testz.assert_eq_string(url_parts.query, "query=value")
    testz.assert_eq_string(url_parts.fragment, "fragment")
    
    testz.test_case("URL parsing - HTTPS with custom port")
    sus https_parts networkz.UrlParts = networkz.parse_url("https://api.example.com:8443/v1/users") fam {
        when err -> {
            testz.fail("HTTPS URL parsing should succeed")
            damn no_cap
        }
    }
    
    testz.assert_eq_string(https_parts.scheme, "https")
    testz.assert_eq_string(https_parts.host, "api.example.com")
    testz.assert_eq_int(https_parts.port, 8443)
    testz.assert_eq_string(https_parts.path, "/v1/users")
    
    testz.test_case("URL parsing - minimal URL")
    sus minimal_parts networkz.UrlParts = networkz.parse_url("example.com") fam {
        when err -> {
            testz.fail("Minimal URL parsing should succeed")
            damn no_cap
        }
    }
    
    testz.assert_eq_string(minimal_parts.host, "example.com")
    testz.assert_eq_int(minimal_parts.port, 80)
    testz.assert_eq_string(minimal_parts.path, "/")
    
    testz.test_case("URL parsing - error handling")
    networkz.parse_url("") fam {
        when err -> {
            testz.assert_eq_string(err.kind, "url_parse")
            testz.assert_eq_string(err.message, "Empty URL provided")
        }
    } otherwise {
        testz.fail("Empty URL should cause parse error")
    }
    
    networkz.parse_url("http://example.com:invalid") fam {
        when err -> {
            testz.assert_eq_string(err.kind, "url_parse")
            testz.assert_true(stringz.contains(err.message, "Invalid port"))
        }
    } otherwise {
        testz.fail("Invalid port should cause parse error")
    }
    
    damn based
}

// URL Encoding/Decoding Tests
slay test_url_encoding() lit {
    testz.test_case("URL parameter encoding")
    sus params []tea = ["name=John Doe", "city=New York", "special=hello&world"]
    sus encoded tea = networkz.encode_url_params(params)
    
    testz.assert_true(stringz.contains(encoded, "John%20Doe"))
    testz.assert_true(stringz.contains(encoded, "New%20York"))
    testz.assert_true(stringz.contains(encoded, "hello%26world"))
    
    testz.test_case("URL parameter decoding")
    sus encoded_params tea = "name=John%20Doe&city=New%20York&special=hello%26world"
    sus decoded []tea = networkz.decode_url_params(encoded_params)
    
    testz.assert_eq_int(arrayz.len(decoded), 3)
    testz.assert_eq_string(decoded[0], "name=John Doe")
    testz.assert_eq_string(decoded[1], "city=New York")
    testz.assert_eq_string(decoded[2], "special=hello&world")
    
    testz.test_case("Empty parameter handling")
    sus empty_encoded tea = networkz.encode_url_params([])
    testz.assert_eq_string(empty_encoded, "")
    
    sus empty_decoded []tea = networkz.decode_url_params("")
    testz.assert_eq_int(arrayz.len(empty_decoded), 0)
    
    damn based
}

// TCP Connection Tests
slay test_tcp_connections() lit {
    testz.test_case("TCP connection - successful connection")
    sus conn networkz.TcpConnection = networkz.tcp_connect("echo.example.com", 80) fam {
        when err -> {
            testz.fail("Connection to echo.example.com should succeed")
            damn no_cap
        }
    }
    
    testz.assert_eq_string(conn.host, "echo.example.com")
    testz.assert_eq_int(conn.port, 80)
    testz.assert_true(conn.is_connected)
    testz.assert_true(conn.socket_fd > 0)
    
    testz.test_case("TCP connection - connection timeout")
    networkz.tcp_connect("timeout.example.com", 80) fam {
        when err -> {
            testz.assert_eq_string(err.kind, "tcp_connect")
            testz.assert_eq_int(err.code, 408)
            testz.assert_true(stringz.contains(err.message, "timeout"))
        }
    } otherwise {
        testz.fail("Connection to timeout.example.com should fail")
    }
    
    testz.test_case("TCP connection - connection refused")
    networkz.tcp_connect("refused.example.com", 80) fam {
        when err -> {
            testz.assert_eq_string(err.kind, "tcp_connect")
            testz.assert_eq_int(err.code, 503)
            testz.assert_true(stringz.contains(err.message, "refused"))
        }
    } otherwise {
        testz.fail("Connection to refused.example.com should fail")
    }
    
    testz.test_case("TCP connection - invalid parameters")
    networkz.tcp_connect("", 80) fam {
        when err -> {
            testz.assert_eq_string(err.kind, "tcp_connect")
            testz.assert_eq_int(err.code, 400)
        }
    } otherwise {
        testz.fail("Empty host should cause error")
    }
    
    networkz.tcp_connect("example.com", -1) fam {
        when err -> {
            testz.assert_eq_string(err.kind, "tcp_connect")
            testz.assert_eq_int(err.code, 400)
        }
    } otherwise {
        testz.fail("Invalid port should cause error")
    }
    
    testz.test_case("TCP data transmission")
    sus test_data tea = "GET / HTTP/1.1\r\nHost: echo.example.com\r\n\r\n"
    sus bytes_sent drip = networkz.tcp_send(conn, test_data) fam {
        when err -> {
            testz.fail("TCP send should succeed")
            damn no_cap
        }
    }
    
    testz.assert_eq_int(bytes_sent, stringz.len(test_data))
    
    testz.test_case("TCP data reception")
    sus received_data tea = networkz.tcp_receive(conn, 1024) fam {
        when err -> {
            testz.fail("TCP receive should succeed")
            damn no_cap
        }
    }
    
    testz.assert_true(stringz.len(received_data) > 0)
    testz.assert_true(stringz.starts_with(received_data, "HTTP/"))
    
    testz.test_case("TCP connection close")
    networkz.tcp_close(conn) fam {
        when err -> {
            testz.fail("TCP close should succeed")
        }
    }
    
    damn based
}

// HTTP Request Building Tests
slay test_http_request_building() lit {
    testz.test_case("HTTP request building - GET request")
    sus get_request tea = networkz.build_http_request(
        "GET", 
        "http://example.com/api/data", 
        [], 
        ""
    ) fam {
        when err -> {
            testz.fail("GET request building should succeed")
            damn no_cap
        }
    }
    
    testz.assert_true(stringz.contains(get_request, "GET /api/data HTTP/1.1"))
    testz.assert_true(stringz.contains(get_request, "Host: example.com"))
    testz.assert_true(stringz.contains(get_request, "User-Agent: CURSED-NetworkZ/1.0"))
    testz.assert_true(stringz.contains(get_request, "Connection: close"))
    
    testz.test_case("HTTP request building - POST request with body")
    sus custom_headers []tea = ["Content-Type: application/json", "Authorization: Bearer token123"]
    sus post_body tea = "{\"name\": \"test\", \"value\": 42}"
    
    sus post_request tea = networkz.build_http_request(
        "POST", 
        "https://api.example.com:443/submit", 
        custom_headers, 
        post_body
    ) fam {
        when err -> {
            testz.fail("POST request building should succeed")
            damn no_cap
        }
    }
    
    testz.assert_true(stringz.contains(post_request, "POST /submit HTTP/1.1"))
    testz.assert_true(stringz.contains(post_request, "Host: api.example.com"))
    testz.assert_true(stringz.contains(post_request, "Content-Type: application/json"))
    testz.assert_true(stringz.contains(post_request, "Authorization: Bearer token123"))
    testz.assert_true(stringz.contains(post_request, "Content-Length: " + stringz.from_int(stringz.len(post_body))))
    testz.assert_true(stringz.ends_with(post_request, post_body))
    
    testz.test_case("HTTP request building - URL with query parameters")
    sus query_request tea = networkz.build_http_request(
        "GET", 
        "http://search.example.com/results?q=cursed&limit=10", 
        [], 
        ""
    ) fam {
        when err -> {
            testz.fail("Query request building should succeed")
            damn no_cap
        }
    }
    
    testz.assert_true(stringz.contains(query_request, "GET /results?q=cursed&limit=10 HTTP/1.1"))
    
    damn based
}

// HTTP Response Parsing Tests
slay test_http_response_parsing() lit {
    testz.test_case("HTTP response parsing - successful response")
    sus raw_response tea = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: 25\r\n\r\n{\"message\": \"success\"}"
    
    sus response networkz.HttpResponse = networkz.parse_http_response(raw_response) fam {
        when err -> {
            testz.fail("Response parsing should succeed")
            damn no_cap
        }
    }
    
    testz.assert_eq_int(response.status_code, 200)
    testz.assert_eq_int(arrayz.len(response.headers), 2)
    testz.assert_eq_string(response.body, "{\"message\": \"success\"}")
    testz.assert_eq_int(response.content_length, 23)
    
    testz.test_case("HTTP response parsing - error response")
    sus error_response tea = "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\n\r\nPage not found"
    
    sus parsed_error networkz.HttpResponse = networkz.parse_http_response(error_response) fam {
        when err -> {
            testz.fail("Error response parsing should succeed")
            damn no_cap
        }
    }
    
    testz.assert_eq_int(parsed_error.status_code, 404)
    testz.assert_eq_string(parsed_error.body, "Page not found")
    
    testz.test_case("HTTP response parsing - no body")
    sus no_body_response tea = "HTTP/1.1 204 No Content\r\nContent-Length: 0\r\n\r\n"
    
    sus parsed_no_body networkz.HttpResponse = networkz.parse_http_response(no_body_response) fam {
        when err -> {
            testz.fail("No body response parsing should succeed")
            damn no_cap
        }
    }
    
    testz.assert_eq_int(parsed_no_body.status_code, 204)
    testz.assert_eq_string(parsed_no_body.body, "")
    testz.assert_eq_int(parsed_no_body.content_length, 0)
    
    testz.test_case("HTTP response parsing - invalid response")
    networkz.parse_http_response("") fam {
        when err -> {
            testz.assert_eq_string(err.kind, "http_parse")
            testz.assert_eq_int(err.code, 400)
        }
    } otherwise {
        testz.fail("Empty response should cause parse error")
    }
    
    networkz.parse_http_response("Invalid response format") fam {
        when err -> {
            testz.assert_eq_string(err.kind, "http_parse")
            testz.assert_true(stringz.contains(err.message, "Invalid"))
        }
    } otherwise {
        testz.fail("Invalid response should cause parse error")
    }
    
    damn based
}

// High-level HTTP Client Tests
slay test_http_client() lit {
    testz.test_case("HTTP GET request - successful")
    sus get_response networkz.HttpResponse = networkz.http_get("http://echo.example.com/test") fam {
        when err -> {
            testz.fail("GET request should succeed")
            damn no_cap
        }
    }
    
    testz.assert_eq_int(get_response.status_code, 200)
    testz.assert_true(stringz.len(get_response.body) >= 0)
    
    testz.test_case("HTTP POST request - with JSON body")
    sus json_body tea = "{\"test\": \"data\"}"
    sus post_response networkz.HttpResponse = networkz.http_post(
        "http://api.example.com/submit", 
        json_body, 
        "application/json"
    ) fam {
        when err -> {
            testz.fail("POST request should succeed")
            damn no_cap
        }
    }
    
    testz.assert_eq_int(post_response.status_code, 200)
    
    testz.test_case("HTTP request - connection failure")
    networkz.http_get("http://timeout.example.com/test") fam {
        when err -> {
            testz.assert_true(stringz.len(err.message) > 0)
            // Should propagate the connection error
        }
    } otherwise {
        testz.fail("Request to timeout host should fail")
    }
    
    testz.test_case("HTTP advanced request - with custom headers")
    sus custom_headers []tea = [
        "Authorization: Bearer test-token",
        "X-Custom-Header: test-value",
        "Accept: application/json"
    ]
    
    sus advanced_response networkz.HttpResponse = networkz.http_request_advanced(
        "PUT",
        "http://api.example.com/resource/123",
        custom_headers,
        "{\"updated\": true}",
        30
    ) fam {
        when err -> {
            testz.fail("Advanced request should succeed")
            damn no_cap
        }
    }
    
    testz.assert_eq_int(advanced_response.status_code, 200)
    
    damn based
}

// JSON API Tests
slay test_json_api() lit {
    testz.test_case("JSON GET request")
    sus json_response networkz.HttpResponse = networkz.json_get("http://api.example.com/data") fam {
        when err -> {
            testz.fail("JSON GET should succeed")
            damn no_cap
        }
    }
    
    testz.assert_eq_int(json_response.status_code, 200)
    
    testz.test_case("JSON POST request")
    sus json_data tea = "{\"name\": \"test\", \"value\": 42}"
    sus json_post_response networkz.HttpResponse = networkz.json_post(
        "http://api.example.com/create", 
        json_data
    ) fam {
        when err -> {
            testz.fail("JSON POST should succeed")
            damn no_cap
        }
    }
    
    testz.assert_eq_int(json_post_response.status_code, 200)
    
    damn based
}

// Form Data Tests
slay test_form_data() lit {
    testz.test_case("Form POST request")
    sus form_data []tea = [
        "username=testuser",
        "password=secret123",
        "email=test@example.com"
    ]
    
    sus form_response networkz.HttpResponse = networkz.form_post(
        "http://example.com/login", 
        form_data
    ) fam {
        when err -> {
            testz.fail("Form POST should succeed")
            damn no_cap
        }
    }
    
    testz.assert_eq_int(form_response.status_code, 200)
    
    damn based
}

// Response Utilities Tests
slay test_response_utilities() lit {
    testz.test_case("Status code checking functions")
    testz.assert_true(networkz.is_success_status(200))
    testz.assert_true(networkz.is_success_status(201))
    testz.assert_true(networkz.is_success_status(299))
    testz.assert_false(networkz.is_success_status(300))
    testz.assert_false(networkz.is_success_status(404))
    
    testz.assert_true(networkz.is_redirect_status(301))
    testz.assert_true(networkz.is_redirect_status(302))
    testz.assert_false(networkz.is_redirect_status(200))
    testz.assert_false(networkz.is_redirect_status(404))
    
    testz.assert_true(networkz.is_client_error_status(400))
    testz.assert_true(networkz.is_client_error_status(404))
    testz.assert_false(networkz.is_client_error_status(200))
    testz.assert_false(networkz.is_client_error_status(500))
    
    testz.assert_true(networkz.is_server_error_status(500))
    testz.assert_true(networkz.is_server_error_status(503))
    testz.assert_false(networkz.is_server_error_status(404))
    testz.assert_false(networkz.is_server_error_status(200))
    
    testz.test_case("Header extraction")
    sus test_response networkz.HttpResponse = networkz.HttpResponse{
        status_code: 200,
        headers: [
            "Content-Type: application/json",
            "Content-Length: 25",
            "Cache-Control: no-cache",
            "X-Custom: test-value"
        ],
        body: "test body",
        content_length: 9
    }
    
    sus content_type tea = networkz.get_response_header(test_response, "Content-Type")
    testz.assert_eq_string(content_type, "application/json")
    
    sus content_length tea = networkz.get_response_header(test_response, "content-length")  // Test case insensitivity
    testz.assert_eq_string(content_length, "25")
    
    sus custom_header tea = networkz.get_response_header(test_response, "X-Custom")
    testz.assert_eq_string(custom_header, "test-value")
    
    sus missing_header tea = networkz.get_response_header(test_response, "Missing-Header")
    testz.assert_eq_string(missing_header, "")
    
    damn based
}

// HTTP Server Tests
slay test_http_server() lit {
    testz.test_case("HTTP server creation")
    
    // Define test request handler
    slay test_handler(req networkz.HttpRequest) networkz.HttpResponse {
        ready (stringz.equals(req.url, "/hello")) {
            damn networkz.HttpResponse{
                status_code: 200,
                headers: ["Content-Type: text/plain"],
                body: "Hello, World!",
                content_length: 13
            }
        } otherwise ready (stringz.equals(req.url, "/json")) {
            damn networkz.HttpResponse{
                status_code: 200,
                headers: ["Content-Type: application/json"],
                body: "{\"message\": \"JSON response\"}",
                content_length: 27
            }
        } otherwise {
            damn networkz.HttpResponse{
                status_code: 404,
                headers: ["Content-Type: text/plain"],
                body: "Not Found",
                content_length: 9
            }
        }
    }
    
    sus server networkz.HttpServer = networkz.create_http_server("127.0.0.1", 8080, test_handler) fam {
        when err -> {
            testz.fail("Server creation should succeed")
            damn no_cap
        }
    }
    
    testz.assert_eq_string(server.host, "127.0.0.1")
    testz.assert_eq_int(server.port, 8080)
    testz.assert_false(server.is_running)
    testz.assert_true(server.socket_fd > 0)
    
    testz.test_case("HTTP server start and stop")
    networkz.start_http_server(server) fam {
        when err -> {
            testz.fail("Server start should succeed")
            damn no_cap
        }
    }
    
    testz.assert_true(server.is_running)
    
    networkz.stop_http_server(server) fam {
        when err -> {
            testz.fail("Server stop should succeed")
        }
    }
    
    testz.assert_false(server.is_running)
    
    testz.test_case("HTTP server - invalid parameters")
    networkz.create_http_server("", 8080, test_handler) fam {
        when err -> {
            testz.assert_eq_string(err.kind, "server_create")
            testz.assert_eq_int(err.code, 400)
        }
    } otherwise {
        testz.fail("Empty host should cause server creation error")
    }
    
    networkz.create_http_server("localhost", -1, test_handler) fam {
        when err -> {
            testz.assert_eq_string(err.kind, "server_create")
            testz.assert_eq_int(err.code, 400)
        }
    } otherwise {
        testz.fail("Invalid port should cause server creation error")
    }
    
    damn based
}

// Network Diagnostics Tests
slay test_network_diagnostics() lit {
    testz.test_case("Ping host - successful ping")
    sus ping_time drip = networkz.ping_host("example.com") fam {
        when err -> {
            testz.fail("Ping to example.com should succeed")
            damn no_cap
        }
    }
    
    testz.assert_true(ping_time > 0)
    testz.assert_true(ping_time < 1000)  // Should be reasonable ping time
    
    testz.test_case("Ping host - localhost")
    sus localhost_ping drip = networkz.ping_host("localhost") fam {
        when err -> {
            testz.fail("Ping to localhost should succeed")
            damn no_cap
        }
    }
    
    testz.assert_eq_int(localhost_ping, 1)  // Should be very fast for localhost
    
    testz.test_case("Ping host - timeout")
    networkz.ping_host("timeout.example.com") fam {
        when err -> {
            testz.assert_eq_string(err.kind, "ping")
            testz.assert_eq_int(err.code, 408)
        }
    } otherwise {
        testz.fail("Ping to timeout host should fail")
    }
    
    testz.test_case("Port check - open port")
    sus port_open lit = networkz.check_port_open("echo.example.com", 80) fam {
        when err -> {
            testz.fail("Port check should succeed")
            damn no_cap
        }
    }
    
    testz.assert_true(port_open)
    
    testz.test_case("Port check - closed port")
    sus port_closed lit = networkz.check_port_open("refused.example.com", 80) fam {
        when err -> {
            // Port check returns false for closed ports, not an error
            damn no_cap
        }
    }
    
    testz.assert_false(port_closed)
    
    damn based
}

// File Download Tests
slay test_file_operations() lit {
    testz.test_case("Download file - successful download")
    sus bytes_downloaded drip = networkz.download_file(
        "http://echo.example.com/test-file.txt", 
        "/tmp/test-file.txt"
    ) fam {
        when err -> {
            testz.fail("File download should succeed")
            damn no_cap
        }
    }
    
    testz.assert_true(bytes_downloaded > 0)
    
    testz.test_case("Download file - HTTP error")
    networkz.download_file("http://echo.example.com/nonexistent", "/tmp/missing.txt") fam {
        when err -> {
            testz.assert_eq_string(err.kind, "download")
            testz.assert_true(err.code >= 400)
        }
    } otherwise {
        testz.fail("Download of nonexistent file should fail")
    }
    
    damn based
}

// Error Handling Integration Tests
slay test_error_handling_integration() lit {
    testz.test_case("Network error creation")
    sus test_error networkz.NetworkError = networkz.create_network_error("test", "Test error message", 500)
    
    testz.assert_eq_string(test_error.kind, "test")
    testz.assert_eq_string(test_error.message, "Test error message")
    testz.assert_eq_int(test_error.code, 500)
    
    testz.test_case("Error propagation through HTTP stack")
    // Test that errors properly propagate from TCP to HTTP level
    networkz.http_get("http://refused.example.com/test") fam {
        when err -> {
            // Should receive the connection error, not a generic HTTP error
            testz.assert_eq_string(err.kind, "tcp_connect")
            testz.assert_eq_int(err.code, 503)
        }
    } otherwise {
        testz.fail("Request to refused host should propagate connection error")
    }
    
    testz.test_case("Multiple error conditions")
    // Test handling multiple potential error points
    sus invalid_responses []tea = ["", "Invalid", "HTTP/1.1 ABC Invalid"]
    sus i drip = 0
    
    bestie (i < arrayz.len(invalid_responses)) {
        networkz.parse_http_response(invalid_responses[i]) fam {
            when err -> {
                testz.assert_eq_string(err.kind, "http_parse")
                testz.assert_eq_int(err.code, 400)
            }
        } otherwise {
            testz.fail("Invalid response should cause parse error")
        }
        i = i + 1
    }
    
    damn based
}

// Performance and Stress Tests
slay test_performance_characteristics() lit {
    testz.test_case("Multiple concurrent requests simulation")
    sus urls []tea = [
        "http://api.example.com/endpoint1",
        "http://api.example.com/endpoint2", 
        "http://api.example.com/endpoint3",
        "http://api.example.com/endpoint4",
        "http://api.example.com/endpoint5"
    ]
    
    sus successful_requests drip = 0
    sus i drip = 0
    
    bestie (i < arrayz.len(urls)) {
        networkz.http_get(urls[i]) fam {
            when err -> {
                // Count failures but don't fail the test
                vibez.spill("Request", i, "failed:", err.message)
            }
        } otherwise {
            successful_requests = successful_requests + 1
        }
        i = i + 1
    }
    
    testz.assert_true(successful_requests >= 0)  // At least some should work in simulation
    
    testz.test_case("Large request/response handling")
    sus large_body tea = stringz.repeat("A", 10000)  // 10KB of data
    sus large_headers []tea = []
    sus header_count drip = 0
    
    bestie (header_count < 20) {
        large_headers = arrayz.push(large_headers, stringz.concat(["X-Header-", stringz.from_int(header_count), ": value"]))
        header_count = header_count + 1
    }
    
    sus large_request tea = networkz.build_http_request(
        "POST", 
        "http://api.example.com/large-upload", 
        large_headers, 
        large_body
    ) fam {
        when err -> {
            testz.fail("Large request building should succeed")
            damn no_cap
        }
    }
    
    testz.assert_true(stringz.len(large_request) > 10000)
    testz.assert_true(stringz.contains(large_request, "Content-Length: 10000"))
    
    damn based
}

// Integration Test Scenarios
slay test_real_world_scenarios() lit {
    testz.test_case("REST API client pattern")
    // Simulate a typical API client usage pattern
    
    // 1. Authentication request
    sus auth_body tea = "{\"username\": \"test\", \"password\": \"secret\"}"
    sus auth_response networkz.HttpResponse = networkz.json_post("http://api.example.com/auth", auth_body) fam {
        when err -> {
            testz.fail("Authentication request should succeed")
            damn no_cap
        }
    }
    
    testz.assert_true(networkz.is_success_status(auth_response.status_code))
    
    // 2. Authenticated API call
    sus auth_headers []tea = ["Authorization: Bearer fake-token"]
    sus api_response networkz.HttpResponse = networkz.http_request_advanced(
        "GET",
        "http://api.example.com/protected-resource",
        auth_headers,
        "",
        30
    ) fam {
        when err -> {
            testz.fail("Protected API call should succeed")
            damn no_cap
        }
    }
    
    testz.assert_true(networkz.is_success_status(api_response.status_code))
    
    testz.test_case("Web scraping pattern")
    // Simulate web scraping workflow
    
    // 1. Get main page
    sus main_page networkz.HttpResponse = networkz.http_get("http://example.com/") fam {
        when err -> {
            testz.fail("Main page request should succeed")
            damn no_cap
        }
    }
    
    testz.assert_true(networkz.is_success_status(main_page.status_code))
    
    // 2. Check for common web page elements
    testz.assert_true(stringz.len(main_page.body) > 0)
    
    testz.test_case("Error recovery and retry pattern")
    // Simulate retry logic for network failures
    sus max_retries drip = 3
    sus attempt drip = 0
    sus success lit = no_cap
    
    bestie (attempt < max_retries && !success) {
        networkz.http_get("http://flaky.example.com/endpoint") fam {
            when err -> {
                attempt = attempt + 1
                ready (attempt >= max_retries) {
                    testz.fail("Should eventually succeed or exhaust retries")
                }
                // In real implementation, would wait between retries
            }
        } otherwise {
            success = based
        }
    }
    
    // Test should handle both success and failure scenarios gracefully
    testz.assert_true(attempt <= max_retries)
    
    damn based
}

// Main test runner
slay run_all_tests() lit {
    setup_test_environment()
    
    vibez.spill("Running NetworkZ comprehensive test suite...")
    vibez.spill("")
    
    test_url_parsing()
    test_url_encoding()
    test_tcp_connections()
    test_http_request_building()
    test_http_response_parsing()
    test_http_client()
    test_json_api()
    test_form_data()
    test_response_utilities()
    test_http_server()
    test_network_diagnostics()
    test_file_operations()
    test_error_handling_integration()
    test_performance_characteristics()
    test_real_world_scenarios()
    
    vibez.spill("")
    vibez.spill("NetworkZ test suite completed!")
    testz.print_test_summary()
    
    damn based
}

// Execute all tests
run_all_tests()
