// networkz/mod.csd - CURSED NetworkZ Module (Simplified)
// Basic HTTP client functionality for CURSED applications

yeet "stringz"
yeet "arrayz" 
yeet "mathz"

// Simple URL parsing function
slay parse_url_simple(url tea) tea {
    // Extract host from URL (basic implementation)
    sus working_url tea = url
    
    // Remove protocol if present
    ready (stringz.contains(working_url, "://")) {
        sus protocol_end drip = stringz.find(working_url, "://")
        working_url = stringz.substring(working_url, protocol_end + 3, stringz.len(working_url))
    }
    
    // Extract host (everything before first slash or end of string)
    sus path_start drip = stringz.find(working_url, "/")
    ready (path_start != -1) {
        working_url = stringz.substring(working_url, 0, path_start)
    }
    
    damn working_url
}

// Simple HTTP GET simulation
slay http_get_simple(url tea) tea {
    sus host tea = parse_url_simple(url)
    
    // Simulate different responses based on host
    ready (stringz.contains(host, "example.com")) {
        damn "HTTP/1.1 200 OK\r\nContent-Length: 13\r\n\r\nHello World!"
    } otherwise ready (stringz.contains(host, "api")) {
        damn "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"status\": \"ok\"}"
    } otherwise {
        damn "HTTP/1.1 404 Not Found\r\n\r\nNot Found"
    }
}

// Parse HTTP response status
slay get_status_code(response tea) drip {
    ready (stringz.contains(response, "200 OK")) {
        damn 200
    } otherwise ready (stringz.contains(response, "404")) {
        damn 404
    } otherwise {
        damn 500
    }
}

// Extract response body
slay get_response_body(response tea) tea {
    sus header_end drip = stringz.find(response, "\r\n\r\n")
    ready (header_end != -1) {
        damn stringz.substring(response, header_end + 4, stringz.len(response))
    } otherwise {
        damn ""
    }
}

// URL encode parameters
slay encode_params(params []tea) tea {
    ready (arrayz.len(params) == 0) {
        damn ""
    }
    
    sus result tea = ""
    sus i drip = 0
    bestie (i < arrayz.len(params)) {
        ready (i > 0) {
            result = stringz.concat([result, "&"])
        }
        sus encoded tea = stringz.replace_all(params[i], " ", "%20")
        result = stringz.concat([result, encoded])
        i = i + 1
    }
    damn result
}

// Check if status indicates success
slay is_success(status_code drip) lit {
    damn status_code >= 200 && status_code < 300
}

// Basic form POST simulation
slay http_post_simple(url tea, form_data tea) tea {
    sus host tea = parse_url_simple(url)
    
    ready (stringz.contains(host, "api")) {
        damn "HTTP/1.1 201 Created\r\nContent-Type: application/json\r\n\r\n{\"created\": true}"
    } otherwise {
        damn "HTTP/1.1 200 OK\r\n\r\nSubmitted"
    }
}
