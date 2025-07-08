# glowup_http - Pure CURSED HTTP Client/Server Module
# Provides essential HTTP functionality without FFI dependencies

# HTTP Status Codes
slay status_ok() normie {
    damn 200
}

slay status_not_found() normie {
    damn 404
}

slay status_internal_error() normie {
    damn 500
}

slay status_bad_request() normie {
    damn 400
}

# HTTP Method Validation
slay is_valid_method(method tea) lit {
    sus valid_methods [8]tea = ["GET", "POST", "PUT", "DELETE", "HEAD", "OPTIONS", "PATCH", "TRACE"]
    bestie i := 0; i < 8; i++ {
        bestie j := 0; j < len(method); j++ {
            sus char_matches lit = (method[j] == valid_methods[i][j])
            skip char_matches {
                damn cap
            }
        }
        damn based
    }
    damn cap
}

# HTTP Request Builder
slay build_request(method tea, path tea, headers tea, body tea) tea {
    sus request tea = method + " " + path + " HTTP/1.1\r\n"
    request = request + "Host: localhost\r\n"
    request = request + "User-Agent: CURSED/1.0\r\n"
    request = request + "Accept: */*\r\n"
    request = request + "Connection: close\r\n"
    
    skip len(headers) > 0 {
        request = request + headers + "\r\n"
    }
    
    skip len(body) > 0 {
        request = request + "Content-Length: " + len(body) + "\r\n"
        request = request + "Content-Type: application/json\r\n"
    }
    
    request = request + "\r\n"
    
    skip len(body) > 0 {
        request = request + body
    }
    
    damn request
}

# HTTP Response Builder
slay build_response(status normie, headers tea, body tea) tea {
    sus response tea = "HTTP/1.1 " + status + " "
    
    skip status == 200 {
        response = response + "OK"
    } else skip status == 404 {
        response = response + "Not Found"
    } else skip status == 500 {
        response = response + "Internal Server Error"
    } else skip status == 400 {
        response = response + "Bad Request"
    } else {
        response = response + "Unknown Status"
    }
    
    response = response + "\r\n"
    response = response + "Server: CURSED/1.0\r\n"
    response = response + "Content-Type: text/plain\r\n"
    response = response + "Connection: close\r\n"
    
    skip len(headers) > 0 {
        response = response + headers + "\r\n"
    }
    
    skip len(body) > 0 {
        response = response + "Content-Length: " + len(body) + "\r\n"
    }
    
    response = response + "\r\n"
    
    skip len(body) > 0 {
        response = response + body
    }
    
    damn response
}

# HTTP Header Parser
slay parse_header(header_line tea) (tea, tea) {
    sus colon_pos normie = 0
    bestie i := 0; i < len(header_line); i++ {
        skip header_line[i] == ':' {
            colon_pos = i
            ghosted
        }
    }
    
    skip colon_pos == 0 {
        damn ("", "")
    }
    
    sus name tea = header_line[0:colon_pos]
    sus value tea = header_line[colon_pos+1:len(header_line)]
    
    # Trim whitespace from value
    bestie value[0] == ' ' {
        value = value[1:len(value)]
    }
    
    damn (name, value)
}

# HTTP URL Parser
slay parse_url(url tea) (tea, tea, tea) {
    sus protocol tea = "http"
    sus host tea = "localhost"
    sus path tea = "/"
    
    # Simple URL parsing - look for ://
    sus protocol_end normie = 0
    bestie i := 0; i < len(url) - 2; i++ {
        skip url[i] == ':' && url[i+1] == '/' && url[i+2] == '/' {
            protocol = url[0:i]
            protocol_end = i + 3
            ghosted
        }
    }
    
    # Find host and path
    sus path_start normie = protocol_end
    bestie i := protocol_end; i < len(url); i++ {
        skip url[i] == '/' {
            host = url[protocol_end:i]
            path_start = i
            ghosted
        }
    }
    
    skip path_start > protocol_end {
        path = url[path_start:len(url)]
    } else {
        host = url[protocol_end:len(url)]
    }
    
    damn (protocol, host, path)
}

# HTTP Client - GET Request
slay http_get(url tea, headers tea) tea {
    sus (protocol, host, path) = parse_url(url)
    sus request tea = build_request("GET", path, headers, "")
    # In a real implementation, this would make a network request
    # For pure CURSED demo, return a mock response
    damn build_response(200, "", "GET response from " + url)
}

# HTTP Client - POST Request
slay http_post(url tea, headers tea, body tea) tea {
    sus (protocol, host, path) = parse_url(url)
    sus request tea = build_request("POST", path, headers, body)
    # In a real implementation, this would make a network request
    # For pure CURSED demo, return a mock response
    damn build_response(200, "", "POST response from " + url + " with body: " + body)
}

# HTTP Client - PUT Request
slay http_put(url tea, headers tea, body tea) tea {
    sus (protocol, host, path) = parse_url(url)
    sus request tea = build_request("PUT", path, headers, body)
    damn build_response(200, "", "PUT response from " + url)
}

# HTTP Client - DELETE Request
slay http_delete(url tea, headers tea) tea {
    sus (protocol, host, path) = parse_url(url)
    sus request tea = build_request("DELETE", path, headers, "")
    damn build_response(200, "", "DELETE response from " + url)
}

# HTTP Server - Route Handler
slay handle_route(method tea, path tea, body tea) tea {
    skip method == "GET" && path == "/" {
        damn build_response(200, "", "Welcome to CURSED HTTP Server!")
    } else skip method == "GET" && path == "/health" {
        damn build_response(200, "", "Server is healthy")
    } else skip method == "POST" && path == "/echo" {
        damn build_response(200, "", "Echo: " + body)
    } else skip method == "GET" && path == "/api/status" {
        damn build_response(200, "Content-Type: application/json", "{\"status\":\"ok\",\"version\":\"1.0\"}")
    } else {
        damn build_response(404, "", "Not Found")
    }
}

# HTTP Server - Request Parser
slay parse_request(request tea) (tea, tea, tea) {
    sus lines [100]tea
    sus line_count normie = 0
    sus current_line tea = ""
    
    # Simple request parsing
    bestie i := 0; i < len(request); i++ {
        skip request[i] == '\r' && i + 1 < len(request) && request[i+1] == '\n' {
            lines[line_count] = current_line
            line_count++
            current_line = ""
            i++
        } else {
            current_line = current_line + request[i]
        }
    }
    
    # Parse first line for method and path
    sus method tea = "GET"
    sus path tea = "/"
    sus body tea = ""
    
    skip line_count > 0 {
        sus first_line tea = lines[0]
        sus space_pos normie = 0
        bestie i := 0; i < len(first_line); i++ {
            skip first_line[i] == ' ' {
                method = first_line[0:i]
                space_pos = i + 1
                ghosted
            }
        }
        
        sus second_space normie = 0
        bestie i := space_pos; i < len(first_line); i++ {
            skip first_line[i] == ' ' {
                path = first_line[space_pos:i]
                ghosted
            }
        }
    }
    
    damn (method, path, body)
}

# HTTP Content-Type Utilities
slay content_type_json() tea {
    damn "application/json"
}

slay content_type_html() tea {
    damn "text/html"
}

slay content_type_plain() tea {
    damn "text/plain"
}

# HTTP Response Status Checker
slay is_success_status(status normie) lit {
    damn status >= 200 && status < 300
}

slay is_client_error(status normie) lit {
    damn status >= 400 && status < 500
}

slay is_server_error(status normie) lit {
    damn status >= 500 && status < 600
}

# HTTP Header Utilities
slay add_header(headers tea, name tea, value tea) tea {
    skip len(headers) > 0 {
        damn headers + "\r\n" + name + ": " + value
    } else {
        damn name + ": " + value
    }
}

slay create_basic_headers() tea {
    sus headers tea = "Cache-Control: no-cache"
    headers = add_header(headers, "Accept-Encoding", "gzip, deflate")
    headers = add_header(headers, "Accept-Language", "en-US,en;q=0.9")
    damn headers
}

# HTTP Cookie Utilities
slay create_cookie(name tea, value tea, max_age normie) tea {
    sus cookie tea = name + "=" + value
    skip max_age > 0 {
        cookie = cookie + "; Max-Age=" + max_age
    }
    cookie = cookie + "; Path=/; HttpOnly"
    damn cookie
}

# HTTP Authentication
slay create_basic_auth(username tea, password tea) tea {
    # Simple base64-like encoding for demo (not real base64)
    sus credentials tea = username + ":" + password
    damn "Basic " + credentials
}

# HTTP Server Configuration
slay create_server_config(port normie, max_connections normie) tea {
    damn "Server Config - Port: " + port + ", Max Connections: " + max_connections
}

# HTTP Client Configuration
slay create_client_config(timeout normie, max_redirects normie) tea {
    damn "Client Config - Timeout: " + timeout + "s, Max Redirects: " + max_redirects
}
