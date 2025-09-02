# RPC Client Module
# Provides JSON-RPC 2.0 client implementation with connection pooling,
# retry logic, authentication, and concurrent request support

yeet "networkz"
yeet "jsonz"
yeet "stringz"
yeet "timez"
yeet "concurrenz"
yeet "./core"

# RPC Client Configuration
squad RpcClientConfig {
    endpoint tea,               # RPC server endpoint (e.g., "http://localhost:8080/rpc")
    timeout drip,               # Request timeout in milliseconds
    max_retries drip,           # Maximum retry attempts
    retry_delay_ms drip,        # Delay between retries in milliseconds
    auth_token tea,             # Authentication token (Bearer)
    user_agent tea,             # HTTP User-Agent header
    max_connections drip,       # Connection pool size
    keep_alive lit              # Keep connections alive
}

# RPC Client State
squad RpcClient {
    config RpcClientConfig,
    http_client networkz.HttpClient,
    connection_pool chan<networkz.Connection>,
    request_counter drip,       # For generating request IDs
    pending_requests map<tea, chan<RpcResponse>>
}

# Create default client configuration
slay default_client_config(endpoint tea) RpcClientConfig {
    damn RpcClientConfig{
        endpoint: endpoint,
        timeout: 30000,
        max_retries: 3,
        retry_delay_ms: 1000,
        auth_token: "",
        user_agent: "CURSED-RPC-Client/1.0",
        max_connections: 10,
        keep_alive: based
    }
}

# Create new RPC client
slay new_rpc_client(config RpcClientConfig) yikes<RpcClient> {
    sus client RpcClient = RpcClient{
        config: config,
        request_counter: 0,
        pending_requests: make_map()
    }
    
    # Initialize HTTP client
    client.http_client = networkz.new_http_client() fam {
        when _ -> yikes "Failed to create HTTP client"
    }
    
    # Set client timeout
    networkz.set_timeout(client.http_client, config.timeout)
    
    # Initialize connection pool
    client.connection_pool = make_channel_buffered(config.max_connections)
    
    damn client
}

# Generate unique request ID
slay generate_request_id(client &RpcClient) tea {
    client.request_counter += 1
    damn "req_" + string_from_int(client.request_counter) + "_" + string_from_int(timez.now_millis())
}

# Make synchronous RPC call
slay call_method(client &RpcClient, method tea, params tea) yikes<tea> {
    sus request_id tea = generate_request_id(client)
    
    # Create RPC request
    sus rpc_request RpcRequest = RpcRequest{
        jsonrpc: "2.0",
        method: method,
        params: params,
        id: request_id
    }
    
    # Execute request
    sus response RpcResponse = execute_request(client, rpc_request)
    
    # Check for errors
    ready (response.error != "") {
        yikes "RPC error: " + response.error
    }
    
    damn response.result
}

# Make asynchronous RPC call
slay call_method_async(client &RpcClient, method tea, params tea) yikes<chan<tea>> {
    sus request_id tea = generate_request_id(client)
    sus result_chan chan<tea> = make_channel()
    
    # Create RPC request
    sus rpc_request RpcRequest = RpcRequest{
        jsonrpc: "2.0",
        method: method,
        params: params,
        id: request_id
    }
    
    # Execute asynchronously
    go {
        sus response RpcResponse = execute_request(client, rpc_request) fam {
            when _ -> {
                result_chan <- ""
                damn
            }
        }
        
        ready (response.error != "") {
            result_chan <- "ERROR: " + response.error
        } otherwise {
            result_chan <- response.result
        }
    }
    
    damn result_chan
}

# Send notification (no response expected)
slay send_notification(client &RpcClient, method tea, params tea) yikes<tea> {
    # Create RPC notification (no ID)
    sus rpc_request RpcRequest = RpcRequest{
        jsonrpc: "2.0",
        method: method,
        params: params,
        id: ""
    }
    
    # Execute request (ignore response)
    execute_request(client, rpc_request) fam {
        when _ -> yikes "Failed to send notification"
    }
    
    damn "Notification sent successfully"
}

# Execute RPC request with retry logic
slay execute_request(client &RpcClient, rpc_request RpcRequest) yikes<RpcResponse> {
    sus last_error tea = ""
    
    # Retry loop
    bestie (attempt in range(client.config.max_retries + 1)) {
        sus response RpcResponse = execute_single_request(client, rpc_request) fam {
            when error -> {
                last_error = error
                
                # Wait before retry (except last attempt)
                ready (attempt < client.config.max_retries) {
                    timez.sleep_ms(client.config.retry_delay_ms * (attempt + 1))
                }
                continue
            }
        }
        
        damn response
    }
    
    yikes "RPC request failed after retries: " + last_error
}

# Execute single RPC request
slay execute_single_request(client &RpcClient, rpc_request RpcRequest) yikes<RpcResponse> {
    # Serialize request
    sus request_json tea = jsonz.encode_object(rpc_request) fam {
        when _ -> yikes "Failed to serialize request"
    }
    
    # Create HTTP request
    sus http_request networkz.HttpRequest = networkz.create_http_request("POST", client.config.endpoint, request_json)
    
    # Set headers
    networkz.set_header(http_request, "Content-Type", "application/json")
    networkz.set_header(http_request, "User-Agent", client.config.user_agent)
    
    # Add authentication if configured
    ready (client.config.auth_token != "") {
        networkz.set_header(http_request, "Authorization", "Bearer " + client.config.auth_token)
    }
    
    # Execute HTTP request
    sus http_response networkz.HttpResponse = networkz.execute_request(client.http_client, http_request) fam {
        when _ -> yikes "HTTP request failed"
    }
    
    # Check HTTP status
    ready (http_response.status_code != 200) {
        yikes "HTTP error: " + string_from_int(http_response.status_code) + " " + http_response.status_text
    }
    
    # Parse RPC response
    sus rpc_response RpcResponse = parse_rpc_response(http_response.body) fam {
        when _ -> yikes "Failed to parse RPC response"
    }
    
    damn rpc_response
}

# Parse RPC response from JSON
slay parse_rpc_response(json_data tea) yikes<RpcResponse> {
    sus response RpcResponse = RpcResponse{
        jsonrpc: "",
        result: "",
        error: "",
        id: ""
    }
    
    # Parse JSON
    sus json_obj map<tea, tea> = jsonz.parse(json_data) fam {
        when _ -> yikes "Invalid JSON response"
    }
    
    # Validate JSON-RPC version
    response.jsonrpc = json_obj.get("jsonrpc") fam {
        when _ -> yikes "Missing jsonrpc field"
    }
    
    ready (response.jsonrpc != "2.0") {
        yikes "Invalid JSON-RPC version"
    }
    
    # Extract result or error
    response.result = json_obj.get("result") fam {
        when _ -> ""
    }
    
    response.error = json_obj.get("error") fam {
        when _ -> ""
    }
    
    # Extract ID
    response.id = json_obj.get("id") fam {
        when _ -> ""
    }
    
    damn response
}

# Make batch RPC call
slay call_batch(client &RpcClient, requests RpcRequest[value]) yikes<RpcResponse[value]> {
    # Validate batch requests
    ready (len(requests) == 0) {
        yikes "Batch cannot be empty"
    }
    
    # Ensure all requests have IDs (except notifications)
    bestie (request in requests) {
        ready (request.id == "" && request.method != "") {
            # Generate ID for non-notification requests
            request.id = generate_request_id(client)
        }
    }
    
    # Serialize batch request
    sus batch_json tea = jsonz.encode_array(requests) fam {
        when _ -> yikes "Failed to serialize batch request"
    }
    
    # Create HTTP request
    sus http_request networkz.HttpRequest = networkz.create_http_request("POST", client.config.endpoint, batch_json)
    
    # Set headers
    networkz.set_header(http_request, "Content-Type", "application/json")
    networkz.set_header(http_request, "User-Agent", client.config.user_agent)
    
    ready (client.config.auth_token != "") {
        networkz.set_header(http_request, "Authorization", "Bearer " + client.config.auth_token)
    }
    
    # Execute HTTP request
    sus http_response networkz.HttpResponse = networkz.execute_request(client.http_client, http_request) fam {
        when _ -> yikes "Batch HTTP request failed"
    }
    
    ready (http_response.status_code != 200) {
        yikes "Batch HTTP error: " + string_from_int(http_response.status_code)
    }
    
    # Parse batch response
    sus responses RpcResponse[value] = parse_batch_response(http_response.body) fam {
        when _ -> yikes "Failed to parse batch response"
    }
    
    damn responses
}

# Parse batch RPC response
slay parse_batch_response(json_data tea) yikes<RpcResponse[value]> {
    sus responses RpcResponse[value] = []
    
    # Parse JSON array
    sus json_array map[value]<tea, tea> = jsonz.parse_array(json_data) fam {
        when _ -> yikes "Invalid batch response format"
    }
    
    # Parse each response
    bestie (json_obj in json_array) {
        sus response_json tea = jsonz.encode_object(json_obj) fam {
            when _ -> continue
        }
        
        sus response RpcResponse = parse_rpc_response(response_json) fam {
            when _ -> continue
        }
        
        responses = append(responses, response)
    }
    
    damn responses
}

# Set authentication token
slay set_auth_token(client &RpcClient, token tea) {
    client.config.auth_token = token
}

# Close RPC client and cleanup resources
slay close_client(client &RpcClient) yikes<tea> {
    # Close HTTP client
    networkz.close_client(client.http_client) fam {
        when _ -> yikes "Failed to close HTTP client"
    }
    
    # Close connection pool
    close_channel(client.connection_pool)
    
    damn "RPC client closed successfully"
}

# Get client statistics
slay get_client_stats(client &RpcClient) tea {
    sus stats map<tea, tea> = make_map()
    stats["endpoint"] = client.config.endpoint
    stats["timeout"] = string_from_int(client.config.timeout)
    stats["max_retries"] = string_from_int(client.config.max_retries)
    stats["request_counter"] = string_from_int(client.request_counter)
    stats["pending_requests"] = string_from_int(len(client.pending_requests))
    
    damn jsonz.encode_object(stats) fam {
        when _ -> "{}"
    }
}

# Connection pooling utilities
slay get_pooled_connection(client &RpcClient) yikes<networkz.Connection> {
    # Try to get existing connection
    select {
        ready connection := <-client.connection_pool -> {
            damn connection
        }
        ready after timez.after(100) -> {
            # Create new connection if pool is empty
            damn networkz.create_connection(client.config.endpoint)
        }
    }
}

slay return_pooled_connection(client &RpcClient, conn networkz.Connection) {
    # Return connection to pool if not full
    select {
        ready client.connection_pool <- conn -> {}
        ready -> {
            # Pool is full, close connection
            networkz.close_connection(conn)
        }
    }
}

# Simple RPC client factory
slay create_simple_client(endpoint tea) yikes<RpcClient> {
    sus config RpcClientConfig = default_client_config(endpoint)
    damn new_rpc_client(config)
}

# Quick method call utility
slay quick_call(endpoint tea, method tea, params tea) yikes<tea> {
    sus client RpcClient = create_simple_client(endpoint) fam {
        when _ -> yikes "Failed to create client"
    }
    
    defer {
        close_client(&client) fam {
            when _ -> {}
        }
    }
    
    damn call_method(&client, method, params)
}
