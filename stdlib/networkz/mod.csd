// mod.csd - CURSED NetworkZ Module Entry Point
// Provides unified HTTP/1.1 and HTTP/2 networking interface
// Automatically selects optimal protocol based on server capabilities

yeet "networkz/networkz"
yeet "networkz/enhanced_networkz" 
yeet "networkz/http2"
yeet "networkz/http2_advanced"
yeet "stringz"
yeet "arrayz"

// ==== UNIFIED HTTP CLIENT INTERFACE ====

squad HttpClientConfig {
    sus prefer_http2 lit           // Prefer HTTP/2 when available
    sus force_http2 lit            // Force HTTP/2 or fail
    sus fallback_http1 lit         // Allow fallback to HTTP/1.1
    sus max_concurrent_streams drip // For HTTP/2 multiplexing
    sus connection_timeout drip    // Connection timeout in seconds
    sus request_timeout drip       // Individual request timeout
    sus retry_count drip           // Number of retries
    sus user_agent tea            // Custom user agent string
}

sus default_client_config HttpClientConfig = HttpClientConfig{
    prefer_http2: based,
    force_http2: no_cap,
    fallback_http1: based,
    max_concurrent_streams: 100,
    connection_timeout: 30,
    request_timeout: 60,
    retry_count: 3,
    user_agent: "CURSED-NetworkZ/2.0"
}

// ==== SMART HTTP CLIENT FUNCTIONS ====

slay http_get_smart(url tea) yikes<HttpResponse> {
    damn http_request_smart("GET", url, [], "", default_client_config)
}

slay http_post_smart(url tea, body tea, content_type tea) yikes<HttpResponse> {
    sus headers []tea = []
    ready (stringz.len(content_type) > 0) {
        headers = arrayz.push(headers, stringz.concat(["Content-Type: ", content_type]))
    }
    
    damn http_request_smart("POST", url, headers, body, default_client_config)
}

slay http_request_smart(method tea, url tea, headers []tea, body tea, config HttpClientConfig) yikes<HttpResponse> {
    // Parse URL to determine protocol capabilities
    sus url_parts UrlParts = parse_url(url) fam {
        when err -> yikes err
    }
    
    // Check if URL suggests HTTP/2 support
    sus likely_http2 lit = stringz.contains(url_parts.host, "http2") || 
                          stringz.equals(url_parts.scheme, "https") ||
                          url_parts.port == 443
    
    // Attempt HTTP/2 if preferred and likely supported
    ready (config.prefer_http2 && likely_http2) {
        ready (stringz.equals(method, "GET")) {
            sus http2_response HttpResponse = http2_get(url) fam {
                when err -> {
                    ready (config.fallback_http1) {
                        // Fallback to HTTP/1.1
                        damn http_request_advanced(method, url, headers, body, config.request_timeout)
                    }
                    yikes err
                }
            }
            damn http2_response
        } otherwise ready (stringz.equals(method, "POST")) {
            sus content_type tea = ""
            sus i drip = 0
            bestie (i < arrayz.len(headers)) {
                ready (stringz.starts_with(stringz.to_lower(headers[i]), "content-type:")) {
                    sus colon_pos drip = stringz.find(headers[i], ":")
                    ready (colon_pos != -1) {
                        content_type = stringz.trim(stringz.substring(headers[i], colon_pos + 1, stringz.len(headers[i])))
                    }
                    bestie based
                }
                i = i + 1
            }
            
            sus http2_response HttpResponse = http2_post(url, body, content_type) fam {
                when err -> {
                    ready (config.fallback_http1) {
                        // Fallback to HTTP/1.1
                        damn http_request_advanced(method, url, headers, body, config.request_timeout)
                    }
                    yikes err
                }
            }
            damn http2_response
        } otherwise {
            // For other methods, use HTTP/1.1
            damn http_request_advanced(method, url, headers, body, config.request_timeout)
        }
    }
    
    // Force HTTP/2 mode
    ready (config.force_http2) {
        ready (stringz.equals(method, "GET")) {
            damn http2_get(url)
        } otherwise ready (stringz.equals(method, "POST")) {
            sus content_type tea = "application/octet-stream"  // Default content type
            sus j drip = 0
            bestie (j < arrayz.len(headers)) {
                ready (stringz.starts_with(stringz.to_lower(headers[j]), "content-type:")) {
                    sus colon_pos drip = stringz.find(headers[j], ":")
                    ready (colon_pos != -1) {
                        content_type = stringz.trim(stringz.substring(headers[j], colon_pos + 1, stringz.len(headers[j])))
                    }
                    bestie based
                }
                j = j + 1
            }
            damn http2_post(url, body, content_type)
        } otherwise {
            yikes create_network_error_advanced("http_smart", "HTTP/2 forced but method not supported", HTTP2_PROTOCOL_ERROR, method)
        }
    }
    
    // Default to HTTP/1.1
    damn http_request_advanced(method, url, headers, body, config.request_timeout)
}

// ==== CONCURRENT REQUEST SUPPORT ====

squad ConcurrentRequest {
    sus id tea                     // Request identifier
    sus method tea                 // HTTP method
    sus url tea                    // Request URL
    sus headers []tea              // Request headers
    sus body tea                   // Request body
    sus timeout drip               // Request timeout
    sus priority drip              // Request priority (1-256)
}

squad ConcurrentResponse {
    sus id tea                     // Corresponding request ID
    sus response HttpResponse      // HTTP response
    sus duration drip              // Request duration in milliseconds
    sus protocol tea               // "HTTP/1.1" or "HTTP/2"
    sus error NetworkError         // Error if request failed
}

slay http_concurrent_requests(requests []ConcurrentRequest, config HttpClientConfig) yikes<[]ConcurrentResponse> {
    sus responses []ConcurrentResponse = []
    sus request_count drip = arrayz.len(requests)
    
    ready (request_count == 0) {
        damn responses
    }
    
    ready (request_count == 1) {
        // Single request - no need for concurrency
        sus req ConcurrentRequest = requests[0]
        sus start_time drip = timez.now()
        
        sus response HttpResponse = http_request_smart(req.method, req.url, req.headers, req.body, config) fam {
            when err -> {
                sus error_response ConcurrentResponse = ConcurrentResponse{
                    id: req.id,
                    response: HttpResponse{status_code: 0, headers: [], body: "", content_length: 0},
                    duration: 0,
                    protocol: "unknown",
                    error: err
                }
                responses = arrayz.push(responses, error_response)
                damn responses
            }
        }
        
        sus end_time drip = timez.now()
        sus success_response ConcurrentResponse = ConcurrentResponse{
            id: req.id,
            response: response,
            duration: end_time - start_time,
            protocol: "HTTP/1.1",  // Default assumption
            error: NetworkError{kind: "", message: "", code: 0, underlying_error: "", timestamp: 0, retry_count: 0}
        }
        responses = arrayz.push(responses, success_response)
        damn responses
    }
    
    // Multiple requests - check if we can use HTTP/2 multiplexing
    ready (config.prefer_http2 && request_count <= config.max_concurrent_streams) {
        // Group requests by host for HTTP/2 multiplexing
        sus host_groups squad {
            sus hosts [10]tea
            sus host_requests [10][]ConcurrentRequest
            sus host_count drip
        }
        
        host_groups.host_count = 0
        
        sus k drip = 0
        bestie (k < request_count) {
            sus req ConcurrentRequest = requests[k]
            sus url_parts UrlParts = parse_url(req.url) fam {
                when err -> {
                    // Skip invalid URLs
                    k = k + 1
                    bestie based
                }
            }
            
            // Find or create host group
            sus host_found lit = no_cap
            sus host_index drip = 0
            
            sus m drip = 0
            bestie (m < host_groups.host_count) {
                ready (stringz.equals(host_groups.hosts[m], url_parts.host)) {
                    host_found = based
                    host_index = m
                    bestie based
                }
                m = m + 1
            }
            
            ready (!host_found && host_groups.host_count < 10) {
                host_groups.hosts[host_groups.host_count] = url_parts.host
                host_index = host_groups.host_count
                host_groups.host_count = host_groups.host_count + 1
            }
            
            ready (host_found || host_index < host_groups.host_count) {
                host_groups.host_requests[host_index] = arrayz.push(host_groups.host_requests[host_index], req)
            }
            
            k = k + 1
        }
        
        // Process each host group with HTTP/2 if possible
        sus n drip = 0
        bestie (n < host_groups.host_count) {
            sus host tea = host_groups.hosts[n]
            sus host_requests []ConcurrentRequest = host_groups.host_requests[n]
            
            ready (arrayz.len(host_requests) > 1) {
                // Multiple requests to same host - try HTTP/2 multiplexing
                sus host_responses []ConcurrentResponse = process_host_requests_http2(host, host_requests, config) fam {
                    when err -> {
                        // Fallback to sequential HTTP/1.1 requests
                        host_responses = process_host_requests_http1(host_requests, config)
                    }
                }
                
                sus p drip = 0
                bestie (p < arrayz.len(host_responses)) {
                    responses = arrayz.push(responses, host_responses[p])
                    p = p + 1
                }
            } otherwise {
                // Single request - use smart client
                sus req ConcurrentRequest = host_requests[0]
                sus start_time drip = timez.now()
                
                sus response HttpResponse = http_request_smart(req.method, req.url, req.headers, req.body, config) fam {
                    when err -> {
                        sus error_response ConcurrentResponse = ConcurrentResponse{
                            id: req.id,
                            response: HttpResponse{status_code: 0, headers: [], body: "", content_length: 0},
                            duration: 0,
                            protocol: "unknown",
                            error: err
                        }
                        responses = arrayz.push(responses, error_response)
                        n = n + 1
                        bestie based
                    }
                }
                
                sus end_time drip = timez.now()
                sus success_response ConcurrentResponse = ConcurrentResponse{
                    id: req.id,
                    response: response,
                    duration: end_time - start_time,
                    protocol: "HTTP/1.1",
                    error: NetworkError{kind: "", message: "", code: 0, underlying_error: "", timestamp: 0, retry_count: 0}
                }
                responses = arrayz.push(responses, success_response)
            }
            
            n = n + 1
        }
        
        damn responses
    }
    
    // Fallback to sequential HTTP/1.1 requests
    damn process_host_requests_http1(requests, config)
}

slay process_host_requests_http2(host tea, requests []ConcurrentRequest, config HttpClientConfig) yikes<[]ConcurrentResponse> {
    sus responses []ConcurrentResponse = []
    
    // Convert to HTTP/2 concurrent request format
    sus http2_requests []Http2ConcurrentRequest = []
    sus q drip = 0
    
    bestie (q < arrayz.len(requests)) {
        sus req ConcurrentRequest = requests[q]
        sus http2_req Http2ConcurrentRequest = Http2ConcurrentRequest{
            method: req.method,
            url: req.url,
            headers: req.headers,
            body: req.body,
            priority: req.priority,
            timeout: req.timeout
        }
        http2_requests = arrayz.push(http2_requests, http2_req)
        q = q + 1
    }
    
    // Create HTTP/2 multiplexed connection
    sus socket Socket = tcp_connect(host, 443) fam {  // Assume HTTPS for HTTP/2
        when err -> yikes err
    }
    
    sus mux_conn Http2MultiplexedConnection = create_multiplexed_connection(socket, no_cap) fam {
        when err -> {
            tcp_close(socket)
            yikes err
        }
    }
    
    // Send connection preface
    send_connection_preface(mux_conn.connection) fam {
        when err -> {
            tcp_close(socket)
            yikes err
        }
    }
    
    // Send concurrent requests
    sus http2_responses []HttpResponse = multiplex_send_concurrent(mux_conn, http2_requests) fam {
        when err -> {
            tcp_close(socket)
            yikes err
        }
    }
    
    tcp_close(socket)
    
    // Convert HTTP/2 responses back to concurrent response format
    sus r drip = 0
    bestie (r < arrayz.len(requests) && r < arrayz.len(http2_responses)) {
        sus req ConcurrentRequest = requests[r]
        sus resp HttpResponse = http2_responses[r]
        
        sus concurrent_resp ConcurrentResponse = ConcurrentResponse{
            id: req.id,
            response: resp,
            duration: 0,  // TODO: Calculate actual duration
            protocol: "HTTP/2",
            error: NetworkError{kind: "", message: "", code: 0, underlying_error: "", timestamp: 0, retry_count: 0}
        }
        responses = arrayz.push(responses, concurrent_resp)
        r = r + 1
    }
    
    damn responses
}

slay process_host_requests_http1(requests []ConcurrentRequest, config HttpClientConfig) []ConcurrentResponse {
    sus responses []ConcurrentResponse = []
    
    sus s drip = 0
    bestie (s < arrayz.len(requests)) {
        sus req ConcurrentRequest = requests[s]
        sus start_time drip = timez.now()
        
        sus response HttpResponse = http_request_smart(req.method, req.url, req.headers, req.body, config) fam {
            when err -> {
                sus error_response ConcurrentResponse = ConcurrentResponse{
                    id: req.id,
                    response: HttpResponse{status_code: 0, headers: [], body: "", content_length: 0},
                    duration: 0,
                    protocol: "unknown",
                    error: err
                }
                responses = arrayz.push(responses, error_response)
                s = s + 1
                bestie based
            }
        }
        
        sus end_time drip = timez.now()
        sus success_response ConcurrentResponse = ConcurrentResponse{
            id: req.id,
            response: response,
            duration: end_time - start_time,
            protocol: "HTTP/1.1",
            error: NetworkError{kind: "", message: "", code: 0, underlying_error: "", timestamp: 0, retry_count: 0}
        }
        responses = arrayz.push(responses, success_response)
        s = s + 1
    }
    
    damn responses
}

// ==== CONVENIENCE FUNCTIONS FOR COMMON USE CASES ====

slay json_get_smart(url tea) yikes<HttpResponse> {
    sus headers []tea = ["Accept: application/json"]
    sus config HttpClientConfig = default_client_config
    config.prefer_http2 = based
    
    damn http_request_smart("GET", url, headers, "", config)
}

slay json_post_smart(url tea, json_body tea) yikes<HttpResponse> {
    sus headers []tea = [
        "Content-Type: application/json",
        "Accept: application/json"
    ]
    sus config HttpClientConfig = default_client_config
    config.prefer_http2 = based
    
    damn http_request_smart("POST", url, headers, json_body, config)
}

slay form_post_smart(url tea, form_data []tea) yikes<HttpResponse> {
    sus body tea = encode_url_params(form_data)
    sus headers []tea = ["Content-Type: application/x-www-form-urlencoded"]
    sus config HttpClientConfig = default_client_config
    config.prefer_http2 = based
    
    damn http_request_smart("POST", url, headers, body, config)
}

// ==== PROTOCOL DETECTION AND CAPABILITIES ====

slay detect_http_version(url tea) yikes<tea> {
    // Parse URL
    sus url_parts UrlParts = parse_url(url) fam {
        when err -> yikes err
    }
    
    // Strong indicators for HTTP/2
    ready (stringz.contains(url_parts.host, "http2") || 
          stringz.contains(url_parts.host, "h2") ||
          url_parts.port == 443 && stringz.equals(url_parts.scheme, "https")) {
        damn "HTTP/2"
    }
    
    // HTTPS usually supports HTTP/2
    ready (stringz.equals(url_parts.scheme, "https")) {
        damn "HTTP/2-capable"
    }
    
    // Default to HTTP/1.1
    damn "HTTP/1.1"
}

slay get_optimal_config(use_case tea) HttpClientConfig {
    sus config HttpClientConfig = default_client_config
    
    sick use_case {
        when "api" -> {
            config.prefer_http2 = based
            config.max_concurrent_streams = 50
            config.request_timeout = 30
        }
        when "download" -> {
            config.prefer_http2 = based
            config.max_concurrent_streams = 10
            config.request_timeout = 300
        }
        when "batch" -> {
            config.prefer_http2 = based
            config.max_concurrent_streams = 100
            config.request_timeout = 60
        }
        when "realtime" -> {
            config.prefer_http2 = based
            config.force_http2 = based
            config.fallback_http1 = no_cap
            config.request_timeout = 10
        }
        when "legacy" -> {
            config.prefer_http2 = no_cap
            config.force_http2 = no_cap
            config.fallback_http1 = based
        }
    }
    
    damn config
}

// ==== BACKWARD COMPATIBILITY ALIASES ====

// Maintain compatibility with existing HTTP/1.1 code
slay http_get(url tea) yikes<HttpResponse> {
    damn http_get_smart(url)
}

slay http_post(url tea, body tea, content_type tea) yikes<HttpResponse> {
    damn http_post_smart(url, body, content_type)
}

slay json_get(url tea) yikes<HttpResponse> {
    damn json_get_smart(url)
}

slay json_post(url tea, json_body tea) yikes<HttpResponse> {
    damn json_post_smart(url, json_body)
}

slay form_post(url tea, form_data []tea) yikes<HttpResponse> {
    damn form_post_smart(url, form_data)
}
