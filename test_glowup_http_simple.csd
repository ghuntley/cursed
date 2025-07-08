# glowup_http Simple Test Suite
# Essential HTTP functionality tests

vibez.spill("Starting glowup_http tests...")

# HTTP Status Codes
slay status_ok() normie {
    damn 200
}

slay status_not_found() normie {
    damn 404
}

# Test status codes
sus ok_status = status_ok()
sus not_found_status = status_not_found()

vibez.spill("Status OK: " + ok_status)
vibez.spill("Status Not Found: " + not_found_status)

# HTTP Method Validation
slay is_valid_method(method tea) lit {
    skip method == "GET" {
        damn based
    }
    skip method == "POST" {
        damn based
    }
    skip method == "PUT" {
        damn based
    }
    skip method == "DELETE" {
        damn based
    }
    damn cap
}

# Test method validation
sus get_valid = is_valid_method("GET")
sus post_valid = is_valid_method("POST")
sus invalid_method = is_valid_method("INVALID")

vibez.spill("GET valid: " + get_valid)
vibez.spill("POST valid: " + post_valid)  
vibez.spill("INVALID method: " + invalid_method)

# HTTP Request Builder
slay build_simple_request(method tea, path tea) tea {
    sus request tea = method + " " + path + " HTTP/1.1"
    damn request
}

# Test request building
sus get_request = build_simple_request("GET", "/test")
sus post_request = build_simple_request("POST", "/api/data")

vibez.spill("GET request: " + get_request)
vibez.spill("POST request: " + post_request)

# HTTP Response Builder
slay build_simple_response(status normie, body tea) tea {
    sus response tea = "HTTP/1.1 " + status + " OK"
    skip len(body) > 0 {
        response = response + "\r\n\r\n" + body
    }
    damn response
}

# Test response building
sus ok_response = build_simple_response(200, "Hello World")
sus not_found_response = build_simple_response(404, "Not Found")

vibez.spill("OK response: " + ok_response)
vibez.spill("404 response: " + not_found_response)

# Content Type Utilities
slay content_type_json() tea {
    damn "application/json"
}

slay content_type_html() tea {
    damn "text/html"
}

# Test content types
sus json_type = content_type_json()
sus html_type = content_type_html()

vibez.spill("JSON content type: " + json_type)
vibez.spill("HTML content type: " + html_type)

# HTTP Client Mock
slay http_get_mock(url tea) tea {
    damn "GET response from " + url
}

slay http_post_mock(url tea, body tea) tea {
    damn "POST response from " + url + " with body: " + body
}

# Test HTTP client
sus get_response = http_get_mock("http://example.com/api")
sus post_response_test = http_post_mock("http://example.com/api", "{\"test\":true}")

vibez.spill("GET response: " + get_response)
vibez.spill("POST response: " + post_response_test)

# HTTP Status Checker
slay is_success_status(status normie) lit {
    damn status >= 200 && status < 300
}

slay is_client_error(status normie) lit {
    damn status >= 400 && status < 500
}

# Test status checking
sus success_200 = is_success_status(200)
sus success_404 = is_success_status(404)
sus client_error_400 = is_client_error(400)
sus client_error_200 = is_client_error(200)

vibez.spill("200 is success: " + success_200)
vibez.spill("404 is success: " + success_404)
vibez.spill("400 is client error: " + client_error_400)
vibez.spill("200 is client error: " + client_error_200)

vibez.spill("All glowup_http tests completed successfully!")
