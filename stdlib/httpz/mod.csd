fr fr CURSED HTTP Client/Server Module - Pure CURSED Implementation
fr fr Simplified HTTP operations for demonstration

yeet "stringz"
yeet "jsonz"

fr fr ===== HTTP STATUS CODES =====

facts HTTP_OK drip = 200
facts HTTP_CREATED drip = 201
facts HTTP_NOT_FOUND drip = 404
facts HTTP_SERVER_ERROR drip = 500

fr fr ===== HTTP METHODS =====

facts HTTP_GET tea = "GET"
facts HTTP_POST tea = "POST"
facts HTTP_PUT tea = "PUT"
facts HTTP_DELETE tea = "DELETE"
facts HTTP_PATCH tea = "PATCH"

fr fr ===== HTTP HEADERS =====

slay create_content_type_header(content_type tea) tea {
    damn "Content-Type: " + content_type
}

slay create_authorization_header(token tea) tea {
    damn "Authorization: Bearer " + token
}

slay create_user_agent_header(user_agent tea) tea {
    damn "User-Agent: " + user_agent
}

slay create_content_length_header(length drip) tea {
    sus length_str tea = json_number_to_string(length)
    damn "Content-Length: " + length_str
}

fr fr ===== HTTP REQUEST BUILDING =====

slay build_http_request_line(method tea, path tea, version tea) tea {
    damn method + " " + path + " " + version
}

slay build_http_response_line(version tea, status_code drip, reason tea) tea {
    sus status_str tea = json_number_to_string(status_code)
    damn version + " " + status_str + " " + reason
}

slay build_get_request(host tea, path tea) tea {
    sus request_line tea = build_http_request_line("GET", path, "HTTP/1.1")
    sus host_header tea = "Host: " + host
    sus user_agent tea = create_user_agent_header("CURSED-HTTP/1.0")
    sus connection_header tea = "Connection: close"
    
    damn request_line + "\r\n" + host_header + "\r\n" + user_agent + "\r\n" + connection_header + "\r\n\r\n"
}

slay build_post_request(host tea, path tea, body tea) tea {
    sus request_line tea = build_http_request_line("POST", path, "HTTP/1.1")
    sus host_header tea = "Host: " + host
    sus user_agent tea = create_user_agent_header("CURSED-HTTP/1.0")
    sus content_type tea = create_content_type_header("application/json")
    sus content_length tea = create_content_length_header(string_length(body))
    sus connection_header tea = "Connection: close"
    
    damn request_line + "\r\n" + host_header + "\r\n" + user_agent + "\r\n" + content_type + "\r\n" + content_length + "\r\n" + connection_header + "\r\n\r\n" + body
}

slay build_json_post_request(host tea, path tea, json_data tea) tea {
    damn build_post_request(host, path, json_data)
}

fr fr ===== HTTP RESPONSE PARSING =====

slay parse_http_status_code(response tea) drip {
    fr fr Extract status code from HTTP response
    ready (contains_substring(response, "200 OK")) {
        damn 200
    }
    ready (contains_substring(response, "201 Created")) {
        damn 201
    }
    ready (contains_substring(response, "404 Not Found")) {
        damn 404
    }
    ready (contains_substring(response, "500 Internal Server Error")) {
        damn 500
    }
    damn 0
}

slay parse_http_body(response tea) tea {
    fr fr Extract body from HTTP response
    sus double_crlf tea = "\r\n\r\n"
    sus header_end drip = indexOf(response, double_crlf)
    ready (header_end < 0) {
        damn ""
    }
    
    sus body_start drip = header_end + 4
    sus response_len drip = string_length(response)
    ready (body_start >= response_len) {
        damn ""
    }
    
    damn substring(response, body_start, response_len - body_start)
}

slay get_http_header(response tea, header_name tea) tea {
    fr fr Extract specific header value
    sus header_key tea = header_name + ": "
    sus header_start drip = indexOf(response, header_key)
    ready (header_start < 0) {
        damn ""
    }
    
    sus value_start drip = header_start + string_length(header_key)
    sus line_end drip = indexOf(substring(response, value_start, 100), "\r\n")
    ready (line_end < 0) {
        damn substring(response, value_start, 50)
    }
    
    damn substring(response, value_start, line_end)
}

fr fr ===== MOCK HTTP CLIENT =====

slay http_get(url tea) tea {
    fr fr Simplified HTTP GET request
    ready (contains_substring(url, "api.example.com/users")) {
        damn "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n[{\"id\":1,\"name\":\"John\"}]"
    }
    ready (contains_substring(url, "api.example.com/status")) {
        damn "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"status\":\"healthy\"}"
    }
    ready (contains_substring(url, "httpbin.org/get")) {
        damn "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"url\":\"" + url + "\"}"
    }
    
    fr fr Default response for unknown URLs
    damn "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\n\r\nNot Found"
}

slay http_post(url tea, body tea) tea {
    fr fr Simplified HTTP POST request
    ready (contains_substring(url, "api.example.com/users")) {
        damn "HTTP/1.1 201 Created\r\nContent-Type: application/json\r\n\r\n{\"id\":2,\"status\":\"created\"}"
    }
    ready (contains_substring(url, "httpbin.org/post")) {
        damn "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"data\":\"" + body + "\"}"
    }
    
    fr fr Default response
    damn "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"received\":true}"
}

slay http_put(url tea, body tea) tea {
    fr fr Simplified HTTP PUT request
    ready (contains_substring(url, "api.example.com/users")) {
        damn "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"id\":1,\"status\":\"updated\"}"
    }
    
    damn "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"updated\":true}"
}

slay http_delete(url tea) tea {
    fr fr Simplified HTTP DELETE request
    ready (contains_substring(url, "api.example.com/users")) {
        damn "HTTP/1.1 204 No Content\r\n\r\n"
    }
    
    damn "HTTP/1.1 204 No Content\r\n\r\n"
}

fr fr ===== HTTP CLIENT HELPERS =====

slay get_json(url tea) tea {
    fr fr GET request expecting JSON response
    sus response tea = http_get(url)
    damn parse_http_body(response)
}

slay post_json(url tea, json_data tea) tea {
    fr fr POST request with JSON data
    sus response tea = http_post(url, json_data)
    damn parse_http_body(response)
}

slay put_json(url tea, json_data tea) tea {
    fr fr PUT request with JSON data
    sus response tea = http_put(url, json_data)
    damn parse_http_body(response)
}

slay is_http_success(response tea) lit {
    fr fr Check if HTTP response indicates success
    sus status_code drip = parse_http_status_code(response)
    damn status_code >= 200 && status_code < 300
}

slay is_http_error(response tea) lit {
    fr fr Check if HTTP response indicates error
    sus status_code drip = parse_http_status_code(response)
    damn status_code >= 400
}

fr fr ===== URL PARSING =====

slay parse_url_scheme(url tea) tea {
    fr fr Extract scheme (protocol) from URL
    sus colon_pos drip = indexOf(url, ":")
    ready (colon_pos < 0) {
        damn ""
    }
    damn substring(url, 0, colon_pos)
}

slay parse_url_host(url tea) tea {
    fr fr Extract host from URL
    ready (starts_with(url, "http://")) {
        sus without_scheme tea = substring(url, 7, string_length(url) - 7)
        sus slash_pos drip = indexOf(without_scheme, "/")
        ready (slash_pos < 0) {
            damn without_scheme
        }
        damn substring(without_scheme, 0, slash_pos)
    }
    ready (starts_with(url, "https://")) {
        sus without_scheme tea = substring(url, 8, string_length(url) - 8)
        sus slash_pos drip = indexOf(without_scheme, "/")
        ready (slash_pos < 0) {
            damn without_scheme
        }
        damn substring(without_scheme, 0, slash_pos)
    }
    damn ""
}

slay parse_url_path(url tea) tea {
    fr fr Extract path from URL
    ready (starts_with(url, "http://")) {
        sus without_scheme tea = substring(url, 7, string_length(url) - 7)
        sus slash_pos drip = indexOf(without_scheme, "/")
        ready (slash_pos < 0) {
            damn "/"
        }
        damn substring(without_scheme, slash_pos, string_length(without_scheme) - slash_pos)
    }
    ready (starts_with(url, "https://")) {
        sus without_scheme tea = substring(url, 8, string_length(url) - 8)
        sus slash_pos drip = indexOf(without_scheme, "/")
        ready (slash_pos < 0) {
            damn "/"
        }
        damn substring(without_scheme, slash_pos, string_length(without_scheme) - slash_pos)
    }
    damn url
}

slay build_url(scheme tea, host tea, path tea) tea {
    fr fr Build URL from components
    ready (starts_with(path, "/")) {
        damn scheme + "://" + host + path
    }
    damn scheme + "://" + host + "/" + path
}

fr fr ===== HTTP SERVER HELPERS =====

slay create_response(status_code drip, body tea) tea {
    fr fr Create HTTP response
    sus status_line tea = ""
    ready (status_code == 200) {
        status_line = "HTTP/1.1 200 OK"
    } otherwise ready (status_code == 201) {
        status_line = "HTTP/1.1 201 Created"
    } otherwise ready (status_code == 404) {
        status_line = "HTTP/1.1 404 Not Found"
    } otherwise ready (status_code == 500) {
        status_line = "HTTP/1.1 500 Internal Server Error"
    } otherwise {
        status_line = "HTTP/1.1 200 OK"
    }
    
    sus content_length tea = create_content_length_header(string_length(body))
    sus content_type tea = create_content_type_header("application/json")
    
    damn status_line + "\r\n" + content_type + "\r\n" + content_length + "\r\n\r\n" + body
}

slay create_json_response(status_code drip, json_data tea) tea {
    fr fr Create JSON HTTP response
    damn create_response(status_code, json_data)
}

slay create_error_response(status_code drip, message tea) tea {
    fr fr Create error response
    sus error_json tea = json_create_object("error", message)
    damn create_json_response(status_code, error_json)
}

slay create_success_response(data tea) tea {
    fr fr Create success response
    damn create_json_response(200, data)
}

fr fr ===== REST API HELPERS =====

slay rest_get_all(resource tea) tea {
    fr fr GET /resource - get all items
    ready (resource == "users") {
        sus users tea = json_create_array_two(
            json_create_object_two("id", "1", "name", "John"),
            json_create_object_two("id", "2", "name", "Jane")
        )
        damn create_success_response(users)
    }
    ready (resource == "posts") {
        sus posts tea = json_create_array(json_create_object_two("id", "1", "title", "Hello World"))
        damn create_success_response(posts)
    }
    
    damn create_error_response(404, "Resource not found")
}

slay rest_get_by_id(resource tea, id tea) tea {
    fr fr GET /resource/:id - get specific item
    ready (resource == "users" && id == "1") {
        sus user tea = json_create_object_two("id", "1", "name", "John")
        damn create_success_response(user)
    }
    ready (resource == "users" && id == "2") {
        sus user tea = json_create_object_two("id", "2", "name", "Jane")
        damn create_success_response(user)
    }
    
    damn create_error_response(404, "Not found")
}

slay rest_create(resource tea, data tea) tea {
    fr fr POST /resource - create new item
    ready (resource == "users") {
        sus created_user tea = json_create_object_two("id", "3", "name", "New User")
        damn create_json_response(201, created_user)
    }
    
    damn create_json_response(201, data)
}

slay rest_update(resource tea, id tea, data tea) tea {
    fr fr PUT /resource/:id - update item
    ready (resource == "users") {
        sus updated_user tea = json_create_object_two("id", id, "name", "Updated User")
        damn create_success_response(updated_user)
    }
    
    damn create_success_response(data)
}

slay rest_delete(resource tea, id tea) tea {
    fr fr DELETE /resource/:id - delete item
    damn "HTTP/1.1 204 No Content\r\n\r\n"
}

fr fr ===== QUERY PARAMETER PARSING =====

slay parse_query_param(url tea, param_name tea) tea {
    fr fr Extract query parameter value
    sus param_key tea = param_name + "="
    sus param_start drip = indexOf(url, param_key)
    ready (param_start < 0) {
        damn ""
    }
    
    sus value_start drip = param_start + string_length(param_key)
    sus ampersand_pos drip = indexOf(substring(url, value_start, 50), "&")
    ready (ampersand_pos < 0) {
        damn substring(url, value_start, 20)
    }
    
    damn substring(url, value_start, ampersand_pos)
}

slay has_query_param(url tea, param_name tea) lit {
    sus param_key tea = param_name + "="
    damn contains_substring(url, param_key)
}

slay build_query_string(param1 tea, value1 tea) tea {
    fr fr Build query string with one parameter
    damn "?" + param1 + "=" + value1
}

slay build_query_string_two(param1 tea, value1 tea, param2 tea, value2 tea) tea {
    fr fr Build query string with two parameters
    damn "?" + param1 + "=" + value1 + "&" + param2 + "=" + value2
}

fr fr ===== HTTP UTILITIES =====

slay url_encode(text tea) tea {
    fr fr Simple URL encoding
    sus result tea = text
    result = replace_all(result, " ", "%20")
    result = replace_all(result, "&", "%26")
    result = replace_all(result, "=", "%3D")
    result = replace_all(result, "?", "%3F")
    damn result
}

slay url_decode(text tea) tea {
    fr fr Simple URL decoding
    sus result tea = text
    result = replace_all(result, "%20", " ")
    result = replace_all(result, "%26", "&")
    result = replace_all(result, "%3D", "=")
    result = replace_all(result, "%3F", "?")
    damn result
}

slay is_valid_url(url tea) lit {
    fr fr Basic URL validation
    ready (starts_with(url, "http://") || starts_with(url, "https://")) {
        damn based
    }
    damn cringe
}

slay get_base_url(url tea) tea {
    fr fr Get base URL (scheme + host)
    sus scheme tea = parse_url_scheme(url)
    sus host tea = parse_url_host(url)
    damn scheme + "://" + host
}

slay join_url_paths(base tea, path tea) tea {
    fr fr Join URL base and path
    ready (ends_with(base, "/") && starts_with(path, "/")) {
        damn base + substring(path, 1, string_length(path) - 1)
    }
    ready (ends_with(base, "/") || starts_with(path, "/")) {
        damn base + path
    }
    damn base + "/" + path
}

fr fr ===== CONTENT TYPE HELPERS =====

slay is_json_content(response tea) lit {
    fr fr Check if response is JSON
    sus content_type tea = get_http_header(response, "Content-Type")
    damn contains_substring(content_type, "application/json")
}

slay is_html_content(response tea) lit {
    fr fr Check if response is HTML
    sus content_type tea = get_http_header(response, "Content-Type")
    damn contains_substring(content_type, "text/html")
}

slay is_text_content(response tea) lit {
    fr fr Check if response is plain text
    sus content_type tea = get_http_header(response, "Content-Type")
    damn contains_substring(content_type, "text/plain")
}

fr fr ===== WEBHOOK HELPERS =====

slay create_webhook_payload(event tea, data tea) tea {
    fr fr Create webhook payload
    damn json_create_object_two("event", event, "data", data)
}

slay validate_webhook_signature(payload tea, signature tea, secret tea) lit {
    fr fr Simplified webhook signature validation
    fr fr In real implementation, would use HMAC
    damn contains_substring(signature, secret)
}

slay process_webhook(payload tea) tea {
    fr fr Process incoming webhook
    sus event tea = json_get_string(payload, "event")
    ready (event == "user.created") {
        damn json_create_object("status", "processed")
    }
    ready (event == "order.completed") {
        damn json_create_object("status", "acknowledged")
    }
    
    damn json_create_object("status", "ignored")
}
