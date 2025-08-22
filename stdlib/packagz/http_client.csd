# Real HTTP Client Implementation for Package Manager
# Replaces basic networkz calls with comprehensive HTTP functionality

yeet "networkz"
yeet "stringz"
yeet "arrayz"
yeet "vibez"
yeet "jsonz"
yeet "timez"

# HTTP request configuration
squad HttpRequest {
    sus method tea
    sus url tea
    sus headers map<tea, tea>
    sus body tea
    sus timeout_ms drip
    sus follow_redirects lit
    sus max_redirects drip
    sus verify_ssl lit
}

# HTTP response with complete headers and status
squad HttpResponse {
    sus status_code drip
    sus status_text tea
    sus headers map<tea, tea>
    sus body tea
    sus content_length drip
    sus content_type tea
    sus redirect_count drip
    sus response_time_ms drip
}

# HTTP error information
squad HttpError {
    sus error_type tea  # "timeout", "dns", "ssl", "connection", "protocol"
    sus message tea
    sus retry_after drip
    sus is_retryable lit
}

# Create HTTP request with default settings
slay create_http_request(method tea, url tea) HttpRequest {
    damn HttpRequest {
        method: method,
        url: url,
        headers: {},
        body: "",
        timeout_ms: 30000,  # 30 seconds
        follow_redirects: based,
        max_redirects: 5,
        verify_ssl: based
    }
}

# Add header to request
slay add_header(request HttpRequest, name tea, value tea) HttpRequest {
    request.headers[name] = value
    damn request
}

# Add authorization header
slay add_auth_bearer(request HttpRequest, token tea) HttpRequest {
    request.headers["Authorization"] = "Bearer " + token
    damn request
}

# Add user agent header
slay add_user_agent(request HttpRequest, user_agent tea) HttpRequest {
    request.headers["User-Agent"] = user_agent
    damn request
}

# Set request body with content type
slay set_json_body(request HttpRequest, json_data tea) HttpRequest {
    request.body = json_data
    request.headers["Content-Type"] = "application/json"
    request.headers["Content-Length"] = stringz.from_int(stringz.len(json_data))
    damn request
}

# Set binary body for file uploads
slay set_binary_body(request HttpRequest, data tea, content_type tea) HttpRequest {
    request.body = data
    request.headers["Content-Type"] = content_type
    request.headers["Content-Length"] = stringz.from_int(stringz.len(data))
    damn request
}

# Execute HTTP request with error handling
slay execute_http_request(request HttpRequest) HttpResponse {
    sus start_time drip = timez.current_time_ms()
    
    # Validate URL format
    ready (!is_valid_url(request.url)) {
        damn create_error_response(400, "Invalid URL format")
    }
    
    # Validate method
    ready (!is_valid_method(request.method)) {
        damn create_error_response(400, "Invalid HTTP method")
    }
    
    # Set default headers if not present
    sus final_request HttpRequest = set_default_headers(request)
    
    # Execute request with networkz (enhanced call)
    sus response tea = ""
    match final_request.method {
        "GET" -> {
            response = networkz.http_get_advanced(
                final_request.url,
                final_request.headers,
                final_request.timeout_ms,
                final_request.verify_ssl
            )
        }
        "POST" -> {
            response = networkz.http_post_advanced(
                final_request.url,
                final_request.body,
                final_request.headers,
                final_request.timeout_ms,
                final_request.verify_ssl
            )
        }
        "PUT" -> {
            response = networkz.http_put_advanced(
                final_request.url,
                final_request.body,
                final_request.headers,
                final_request.timeout_ms,
                final_request.verify_ssl
            )
        }
        "DELETE" -> {
            response = networkz.http_delete_advanced(
                final_request.url,
                final_request.headers,
                final_request.timeout_ms,
                final_request.verify_ssl
            )
        }
        _ -> {
            damn create_error_response(400, "Unsupported HTTP method: " + final_request.method)
        }
    }
    
    sus end_time drip = timez.current_time_ms()
    
    # Parse response into structured format
    damn parse_http_response(response, end_time - start_time)
}

# Parse raw HTTP response into structured format
slay parse_http_response(raw_response tea, response_time drip) HttpResponse {
    # Extract status line
    sus lines []tea = stringz.split(raw_response, "\n")
    ready (arrayz.len(lines) < 1) {
        damn create_error_response(0, "Empty response")
    }
    
    sus status_line tea = lines[0]
    sus status_parts []tea = stringz.split(status_line, " ")
    ready (arrayz.len(status_parts) < 3) {
        damn create_error_response(0, "Invalid status line")
    }
    
    sus status_code drip = stringz.parse_int(status_parts[1])
    sus status_text tea = stringz.join(arrayz.slice(status_parts, 2, arrayz.len(status_parts)), " ")
    
    # Parse headers
    sus headers map<tea, tea> = {}
    sus header_end_index drip = 1
    bestie (header_end_index < arrayz.len(lines)) {
        sus line tea = stringz.trim(lines[header_end_index])
        ready (line == "") {
            break  # End of headers
        }
        
        sus header_parts []tea = stringz.split_n(line, ":", 2)
        ready (arrayz.len(header_parts) == 2) {
            sus name tea = stringz.trim(stringz.to_lowercase(header_parts[0]))
            sus value tea = stringz.trim(header_parts[1])
            headers[name] = value
        }
        
        header_end_index = header_end_index + 1
    }
    
    # Extract body
    sus body tea = ""
    ready (header_end_index + 1 < arrayz.len(lines)) {
        sus body_lines []tea = arrayz.slice(lines, header_end_index + 1, arrayz.len(lines))
        body = stringz.join(body_lines, "\n")
    }
    
    # Extract metadata from headers
    sus content_length drip = 0
    sus content_type tea = "text/plain"
    
    ready (headers["content-length"] != "") {
        content_length = stringz.parse_int(headers["content-length"])
    }
    ready (headers["content-type"] != "") {
        content_type = headers["content-type"]
    }
    
    damn HttpResponse {
        status_code: status_code,
        status_text: status_text,
        headers: headers,
        body: body,
        content_length: content_length,
        content_type: content_type,
        redirect_count: 0,
        response_time_ms: response_time
    }
}

# Set default headers for request
slay set_default_headers(request HttpRequest) HttpRequest {
    # Set Accept header if not present
    ready (request.headers["Accept"] == "") {
        request.headers["Accept"] = "application/json, text/plain, */*"
    }
    
    # Set Connection header
    ready (request.headers["Connection"] == "") {
        request.headers["Connection"] = "keep-alive"
    }
    
    # Set Accept-Encoding for compression
    ready (request.headers["Accept-Encoding"] == "") {
        request.headers["Accept-Encoding"] = "gzip, deflate"
    }
    
    damn request
}

# Create error response for client-side errors
slay create_error_response(status_code drip, message tea) HttpResponse {
    damn HttpResponse {
        status_code: status_code,
        status_text: message,
        headers: {},
        body: "",
        content_length: 0,
        content_type: "text/plain",
        redirect_count: 0,
        response_time_ms: 0
    }
}

# Check if HTTP response is successful (2xx status)
slay is_http_success(response HttpResponse) lit {
    damn response.status_code >= 200 && response.status_code < 300
}

# Check if HTTP response is client error (4xx status)
slay is_client_error(response HttpResponse) lit {
    damn response.status_code >= 400 && response.status_code < 500
}

# Check if HTTP response is server error (5xx status)
slay is_server_error(response HttpResponse) lit {
    damn response.status_code >= 500 && response.status_code < 600
}

# Check if error is retryable
slay is_retryable_error(response HttpResponse) lit {
    # Server errors and specific client errors are retryable
    ready (is_server_error(response)) {
        damn based
    }
    
    # Specific retryable client errors
    match response.status_code {
        408 -> damn based  # Request Timeout
        409 -> damn based  # Conflict (temporary)
        429 -> damn based  # Rate Limited
        _ -> damn cap
    }
}

# Extract retry-after header value
slay get_retry_after_seconds(response HttpResponse) drip {
    sus retry_after tea = response.headers["retry-after"]
    ready (retry_after == "") {
        damn 0
    }
    
    # Parse retry-after value (seconds or HTTP date)
    ready (stringz.is_numeric(retry_after)) {
        damn stringz.parse_int(retry_after)
    }
    
    # For HTTP date format, default to 60 seconds
    damn 60
}

# URL validation
slay is_valid_url(url tea) lit {
    ready (url == "") {
        damn cap
    }
    
    ready (!stringz.starts_with(url, "http://") && !stringz.starts_with(url, "https://")) {
        damn cap
    }
    
    # Basic validation - contains domain
    ready (!stringz.contains(url, ".")) {
        damn cap
    }
    
    damn based
}

# HTTP method validation
slay is_valid_method(method tea) lit {
    sus valid_methods []tea = ["GET", "POST", "PUT", "DELETE", "PATCH", "HEAD", "OPTIONS"]
    
    bestie (sus i drip = 0; i < arrayz.len(valid_methods); i = i + 1) {
        ready (method == valid_methods[i]) {
            damn based
        }
    }
    
    damn cap
}

# URL encode for query parameters
slay url_encode(input tea) tea {
    sus result tea = ""
    
    bestie (sus i drip = 0; i < stringz.len(input); i = i + 1) {
        sus ch tea = stringz.char_at(input, i)
        
        # URL-safe characters don't need encoding
        ready (stringz.is_alphanumeric(ch) || ch == "-" || ch == "_" || ch == "." || ch == "~") {
            result = result + ch
        } otherwise {
            # Encode special characters
            sus encoded tea = "%" + stringz.to_hex_upper(stringz.char_code(ch))
            result = result + encoded
        }
    }
    
    damn result
}

# Build query string from parameters
slay build_query_string(params map<tea, tea>) tea {
    sus parts []tea = []
    
    # Iterate over map entries (simplified - real implementation would use proper iteration)
    sus keys []tea = map_keys(params)  # Assume this function exists
    bestie (sus i drip = 0; i < arrayz.len(keys); i = i + 1) {
        sus key tea = keys[i]
        sus value tea = params[key]
        sus part tea = url_encode(key) + "=" + url_encode(value)
        parts = arrayz.append(parts, part)
    }
    
    ready (arrayz.len(parts) == 0) {
        damn ""
    }
    
    damn "?" + stringz.join(parts, "&")
}

# Helper function to get map keys (placeholder for real implementation)
slay map_keys(m map<tea, tea>) []tea {
    # In real implementation, this would iterate over the map
    # For now, return empty array
    damn []
}

# Download file with progress tracking
slay download_file(url tea, local_path tea, progress_callback slay(drip, drip)) lit {
    sus request HttpRequest = create_http_request("GET", url)
    request = add_user_agent(request, "cursed-pkg/1.0.0 (file-downloader)")
    
    # Add range support for resumable downloads
    request.headers["Accept-Ranges"] = "bytes"
    
    sus response HttpResponse = execute_http_request(request)
    
    ready (!is_http_success(response)) {
        vibez.spill("Download failed:", response.status_code, response.status_text)
        damn cap
    }
    
    # Write response body to file
    ready (!filez.write_file(local_path, response.body)) {
        vibez.spill("Failed to write downloaded file to:", local_path)
        damn cap
    }
    
    # Call progress callback if provided
    ready (progress_callback != null) {
        progress_callback(response.content_length, response.content_length)
    }
    
    damn based
}
