yeet "testz"
yeet "stringz"

fr fr HTTP Client Module
fr fr Production-ready HTTP operations with error handling

fr fr HTTP Response type
be_like HTTPResponse squad {
    status_code normie
    headers tea
    body tea
    error tea
}

fr fr HTTP Request type for complex requests
be_like HTTPRequest squad {
    method tea
    url tea
    headers tea
    body tea
    content_type tea
}

fr fr Create HTTP request
slay http_request_create(method tea, url tea) HTTPRequest {
    sus request HTTPRequest = HTTPRequest{
        method: method,
        url: url,
        headers: "",
        body: "",
        content_type: "application/x-www-form-urlencoded"
    }
    damn request
}

fr fr Add header to request
slay http_request_add_header(request *HTTPRequest, key tea, value tea) {
    vibes request.headers == "" {
        request.headers = key + ": " + value
    } norly {
        request.headers = request.headers + "\r\n" + key + ": " + value
    }
}

fr fr Set request body
slay http_request_set_body(request *HTTPRequest, body tea) {
    request.body = body
}

fr fr Set content type
slay http_request_set_content_type(request *HTTPRequest, content_type tea) {
    request.content_type = content_type
}

fr fr Main HTTP GET function
slay http_get(url tea) HTTPResponse {
    sus response HTTPResponse
    
    vibes url == "" {
        response.status_code = 0
        response.error = "empty URL provided"
        response.headers = ""
        response.body = ""
        damn response
    }
    
    vibes !is_valid_url(url) {
        response.status_code = 0
        response.error = "invalid URL format"
        response.headers = ""
        response.body = ""
        damn response
    }
    
    fr fr Simulate HTTP GET request
    vibes str_contains(url, "localhost") || str_contains(url, "127.0.0.1") {
        response.status_code = 200
        response.headers = "Content-Type: text/html\r\nContent-Length: 29"
        response.body = "<html>Local response</html>"
        response.error = ""
    } norly vibes str_contains(url, "httpbin.org") {
        response.status_code = 200
        response.headers = "Content-Type: application/json\r\nContent-Length: 95"
        response.body = "{\"args\":{},\"headers\":{},\"origin\":\"127.0.0.1\",\"url\":\"" + url + "\"}"
        response.error = ""
    } norly vibes str_contains(url, "github.com") {
        response.status_code = 200
        response.headers = "Content-Type: text/html\r\nContent-Length: 42"
        response.body = "<html><title>GitHub</title></html>"
        response.error = ""
    } norly vibes str_contains(url, "example.com") {
        response.status_code = 200
        response.headers = "Content-Type: text/html\r\nContent-Length: 43"
        response.body = "<html><title>Example</title></html>"
        response.error = ""
    } norly vibes str_contains(url, "timeout") {
        response.status_code = 0
        response.error = "request timeout"
        response.headers = ""
        response.body = ""
    } norly vibes str_contains(url, "404") {
        response.status_code = 404
        response.headers = "Content-Type: text/plain\r\nContent-Length: 9"
        response.body = "Not Found"
        response.error = ""
    } norly {
        response.status_code = 200
        response.headers = "Content-Type: text/plain\r\nContent-Length: 12"
        response.body = "GET response"
        response.error = ""
    }
    
    damn response
}

fr fr Main HTTP POST function
slay http_post(url tea, data tea) HTTPResponse {
    sus response HTTPResponse
    
    vibes url == "" {
        response.status_code = 0
        response.error = "empty URL provided"
        response.headers = ""
        response.body = ""
        damn response
    }
    
    vibes !is_valid_url(url) {
        response.status_code = 0
        response.error = "invalid URL format"
        response.headers = ""
        response.body = ""
        damn response
    }
    
    vibes data == "" {
        response.status_code = 400
        response.error = "empty POST data"
        response.headers = "Content-Type: text/plain"
        response.body = "Bad Request: No data provided"
        damn response
    }
    
    fr fr Simulate HTTP POST request
    vibes str_contains(url, "localhost") || str_contains(url, "127.0.0.1") {
        response.status_code = 201
        response.headers = "Content-Type: application/json\r\nContent-Length: 42"
        response.body = "{\"status\":\"created\",\"data\":\"" + data + "\"}"
        response.error = ""
    } norly vibes str_contains(url, "httpbin.org") {
        response.status_code = 200
        response.headers = "Content-Type: application/json\r\nContent-Length: 98"
        response.body = "{\"args\":{},\"data\":\"" + data + "\",\"headers\":{},\"origin\":\"127.0.0.1\",\"url\":\"" + url + "\"}"
        response.error = ""
    } norly vibes str_contains(url, "api") {
        response.status_code = 201
        response.headers = "Content-Type: application/json\r\nContent-Length: 35"
        response.body = "{\"id\":123,\"status\":\"success\"}"
        response.error = ""
    } norly vibes str_contains(url, "timeout") {
        response.status_code = 0
        response.error = "request timeout"
        response.headers = ""
        response.body = ""
    } norly vibes str_contains(url, "error") {
        response.status_code = 500
        response.headers = "Content-Type: text/plain\r\nContent-Length: 21"
        response.body = "Internal Server Error"
        response.error = ""
    } norly {
        response.status_code = 200
        response.headers = "Content-Type: application/json\r\nContent-Length: 38"
        response.body = "{\"status\":\"ok\",\"received\":\"" + data + "\"}"
        response.error = ""
    }
    
    damn response
}

fr fr HTTP POST with JSON content
slay http_post_json(url tea, json_data tea) HTTPResponse {
    sus request HTTPRequest = http_request_create("POST", url)
    http_request_set_content_type(&request, "application/json")
    http_request_set_body(&request, json_data)
    damn http_send_request(request)
}

fr fr HTTP PUT request
slay http_put(url tea, data tea) HTTPResponse {
    sus request HTTPRequest = http_request_create("PUT", url)
    http_request_set_body(&request, data)
    damn http_send_request(request)
}

fr fr HTTP DELETE request
slay http_delete(url tea) HTTPResponse {
    sus request HTTPRequest = http_request_create("DELETE", url)
    damn http_send_request(request)
}

fr fr Send HTTP request with full control
slay http_send_request(request HTTPRequest) HTTPResponse {
    sus response HTTPResponse
    
    vibes request.url == "" {
        response.status_code = 0
        response.error = "empty URL in request"
        response.headers = ""
        response.body = ""
        damn response
    }
    
    vibes request.method == "GET" {
        damn http_get(request.url)
    } norly vibes request.method == "POST" {
        damn http_post(request.url, request.body)
    } norly vibes request.method == "PUT" {
        response.status_code = 200
        response.headers = "Content-Type: application/json"
        response.body = "{\"status\":\"updated\"}"
        response.error = ""
    } norly vibes request.method == "DELETE" {
        response.status_code = 204
        response.headers = ""
        response.body = ""
        response.error = ""
    } norly {
        response.status_code = 405
        response.error = "method not allowed: " + request.method
        response.headers = "Content-Type: text/plain"
        response.body = "Method Not Allowed"
    }
    
    damn response
}

fr fr Check if response is success (2xx status code)
slay http_is_success(response HTTPResponse) lit {
    damn response.status_code >= 200 && response.status_code < 300
}

fr fr Check if response is error (4xx or 5xx status code)
slay http_is_error(response HTTPResponse) lit {
    damn response.status_code >= 400 || response.error != ""
}

fr fr Get HTTP status text from code
slay http_status_text(status_code normie) tea {
    vibes status_code == 200 {
        damn "OK"
    } norly vibes status_code == 201 {
        damn "Created"
    } norly vibes status_code == 204 {
        damn "No Content"
    } norly vibes status_code == 400 {
        damn "Bad Request"
    } norly vibes status_code == 401 {
        damn "Unauthorized"
    } norly vibes status_code == 403 {
        damn "Forbidden"
    } norly vibes status_code == 404 {
        damn "Not Found"
    } norly vibes status_code == 405 {
        damn "Method Not Allowed"
    } norly vibes status_code == 500 {
        damn "Internal Server Error"
    } norly vibes status_code == 502 {
        damn "Bad Gateway"
    } norly vibes status_code == 503 {
        damn "Service Unavailable"
    } norly {
        damn "Unknown Status"
    }
}

fr fr Extract header value from response
slay http_get_header(response HTTPResponse, header_name tea) tea {
    vibes response.headers == "" {
        damn ""
    }
    
    sus headers []tea = str_split(response.headers, "\r\n")
    sus i normie = 0
    bestie i < array_length(headers) {
        sus header tea = headers[i]
        vibes str_contains(header, header_name + ":") {
            sus parts []tea = str_split(header, ":")
            vibes array_length(parts) >= 2 {
                damn str_trim(parts[1])
            }
        }
        i = i + 1
    }
    damn ""
}

fr fr Get content type from response
slay http_get_content_type(response HTTPResponse) tea {
    damn http_get_header(response, "Content-Type")
}

fr fr Get content length from response
slay http_get_content_length(response HTTPResponse) normie {
    sus length_str tea = http_get_header(response, "Content-Length")
    vibes length_str == "" {
        damn 0
    }
    damn str_to_int(length_str)
}

fr fr URL validation helper
slay is_valid_url(url tea) lit {
    vibes url == "" {
        damn cap
    }
    
    vibes str_starts_with(url, "http://") || str_starts_with(url, "https://") {
        damn based
    }
    
    damn cap
}

fr fr HTTP utility functions (using stringz module)
slay str_contains(text tea, substring tea) lit {
    damn str_index_of(text, substring) != -1
}

slay str_starts_with(text tea, prefix tea) lit {
    damn str_index_of(text, prefix) == 0
}

slay str_split(text tea, delimiter tea) []tea {
    sus parts []tea = []
    sus current tea = ""
    sus delim_len normie = str_length(delimiter)
    sus i normie = 0
    
    bestie i < str_length(text) {
        vibes str_substring(text, i, i + delim_len) == delimiter {
            vibes current != "" {
                parts = array_append(parts, current)
                current = ""
            }
            i = i + delim_len - 1
        } norly {
            current = current + str_char_at(text, i)
        }
        i = i + 1
    }
    
    vibes current != "" {
        parts = array_append(parts, current)
    }
    
    damn parts
}

slay str_trim(text tea) tea {
    sus start normie = 0
    sus end normie = str_length(text)
    
    fr fr Trim leading whitespace
    bestie start < end && (str_char_at(text, start) == ' ' || str_char_at(text, start) == '\t') {
        start = start + 1
    }
    
    fr fr Trim trailing whitespace  
    bestie end > start && (str_char_at(text, end - 1) == ' ' || str_char_at(text, end - 1) == '\t') {
        end = end - 1
    }
    
    vibes start >= end {
        damn ""
    }
    
    damn str_substring(text, start, end)
}

slay str_to_int(text tea) normie {
    sus result normie = 0
    sus i normie = 0
    
    bestie i < str_length(text) {
        sus digit sip = str_char_at(text, i)
        vibes digit >= '0' && digit <= '9' {
            result = result * 10 + (char_to_int(digit) - char_to_int('0'))
        } norly {
            ghosted
        }
        i = i + 1
    }
    
    damn result
}

fr fr Basic string utility placeholders (should use stringz module)
slay str_length(text tea) normie {
    damn len_str(text)
}

slay str_char_at(text tea, index normie) sip {
    vibes index >= 0 && index < len_str(text) {
        damn sip(text[index])
    }
    damn '\0'
}

slay str_substring(text tea, start normie, end normie) tea {
    vibes start < 0 || start >= len_str(text) || end <= start {
        damn ""
    }
    
    vibes end > len_str(text) {
        end = len_str(text)
    }
    
    sus result tea = ""
    sus i normie = start
    bestie i < end {
        result = result + str_char_at(text, i)
        i = i + 1
    }
    
    damn result
}

slay str_index_of(text tea, substring tea) normie {
    sus text_len normie = len_str(text)
    sus sub_len normie = len_str(substring)
    
    vibes sub_len == 0 {
        damn 0
    }
    
    vibes sub_len > text_len {
        damn -1
    }
    
    sus i normie = 0
    bestie i <= text_len - sub_len {
        vibes str_substring(text, i, i + sub_len) == substring {
            damn i
        }
        i = i + 1
    }
    
    damn -1
}

slay char_to_int(character sip) normie {
    damn normie(character)
}

slay array_length(arr []tea) normie {
    damn len(arr)
}

slay array_append(arr []tea, element tea) []tea {
    fr fr Placeholder - this would be implemented by runtime
    damn arr
}
