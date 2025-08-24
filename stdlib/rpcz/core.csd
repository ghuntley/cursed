# RPC Core Module
# Implements JSON-RPC 2.0 protocol core functionality
# Provides method registration, request/response handling, and error management

yeet "jsonz"
yeet "networkz" 
yeet "stringz"
yeet "reflectz"

# RPC Error Codes (JSON-RPC 2.0 Standard)
sus RPC_ERROR_PARSE drip = -32700
sus RPC_ERROR_INVALID_REQUEST drip = -32600
sus RPC_ERROR_METHOD_NOT_FOUND drip = -32601
sus RPC_ERROR_INVALID_PARAMS drip = -32602
sus RPC_ERROR_INTERNAL_ERROR drip = -32603
sus RPC_ERROR_SERVER_ERROR_MIN drip = -32099
sus RPC_ERROR_SERVER_ERROR_MAX drip = -32000

# RPC Request Structure
squad RpcRequest {
    jsonrpc tea,        # Must be "2.0"
    method tea,         # Method name to call
    params tea,         # Parameters (JSON string or null)
    id tea              # Request identifier (string, number, or null)
}

# RPC Response Structure  
squad RpcResponse {
    jsonrpc tea,        # Must be "2.0"
    result tea,         # Success result (null on error)
    error tea,          # Error object (null on success)  
    id tea              # Request identifier (from request)
}

# RPC Error Structure
squad RpcError {
    code drip,          # Error code
    message tea,        # Error message
    data tea            # Additional error data (optional)
}

# RPC Method Handler Function Type
collab RpcHandler {
    slay call(params tea) yikes<tea>
}

# RPC Method Registry
squad RpcRegistry {
    methods map<tea, RpcHandler>,
    middleware []RpcMiddleware
}

# RPC Middleware Interface
collab RpcMiddleware {
    slay before_call(request RpcRequest) yikes<lit>
    slay after_call(request RpcRequest, response RpcResponse) yikes<RpcResponse>
}

# Create new RPC registry
slay new_rpc_registry() RpcRegistry {
    damn RpcRegistry{
        methods: make_map(),
        middleware: []
    }
}

# Register RPC method
slay register_method(registry &RpcRegistry, method_name tea, handler RpcHandler) yikes<tea> {
    ready (method_name == "") {
        yikes "Method name cannot be empty"
    }
    
    registry.methods[method_name] = handler
    damn "Method registered successfully"
}

# Parse RPC request from JSON
slay parse_rpc_request(json_data tea) yikes<RpcRequest> {
    sus request RpcRequest = RpcRequest{
        jsonrpc: "",
        method: "",
        params: "",
        id: ""
    }
    
    # Parse JSON
    sus json_obj map<tea, tea> = jsonz.parse(json_data) fam {
        when _ -> yikes "Invalid JSON format"
    }
    
    # Validate JSON-RPC version
    sus version tea = json_obj.get("jsonrpc") fam {
        when _ -> yikes "Missing jsonrpc field"
    }
    
    ready (version != "2.0") {
        yikes "Invalid JSON-RPC version, must be 2.0"
    }
    
    request.jsonrpc = version
    
    # Extract method name
    request.method = json_obj.get("method") fam {
        when _ -> yikes "Missing method field"
    }
    
    ready (request.method == "") {
        yikes "Method name cannot be empty"
    }
    
    # Extract parameters (optional)
    request.params = json_obj.get("params") fam {
        when _ -> ""
    }
    
    # Extract ID (optional for notifications)
    request.id = json_obj.get("id") fam {
        when _ -> ""
    }
    
    damn request
}

# Create RPC response for success
slay create_success_response(result tea, id tea) RpcResponse {
    damn RpcResponse{
        jsonrpc: "2.0",
        result: result,
        error: "",
        id: id
    }
}

# Create RPC response for error
slay create_error_response(code drip, message tea, data tea, id tea) RpcResponse {
    sus error_obj RpcError = RpcError{
        code: code,
        message: message,
        data: data
    }
    
    sus error_json tea = jsonz.encode_object(error_obj) fam {
        when _ -> "{\"code\":" + string_from_int(code) + ",\"message\":\"" + message + "\"}"
    }
    
    damn RpcResponse{
        jsonrpc: "2.0",
        result: "",
        error: error_json,
        id: id
    }
}

# Serialize RPC response to JSON
slay serialize_rpc_response(response RpcResponse) yikes<tea> {
    sus response_json tea = jsonz.encode_object(response) fam {
        when _ -> yikes "Failed to serialize response"
    }
    
    damn response_json
}

# Process RPC request
slay process_rpc_request(registry &RpcRegistry, request RpcRequest) yikes<RpcResponse> {
    # Run pre-processing middleware
    bestie (middleware in registry.middleware) {
        middleware.before_call(request) fam {
            when _ -> damn create_error_response(RPC_ERROR_INTERNAL_ERROR, "Middleware error", "", request.id)
        }
    }
    
    # Find method handler
    sus handler RpcHandler = registry.methods.get(request.method) fam {
        when _ -> damn create_error_response(RPC_ERROR_METHOD_NOT_FOUND, "Method not found: " + request.method, "", request.id)
    }
    
    # Execute method handler
    sus result tea = handler.call(request.params) fam {
        when "invalid_params" -> damn create_error_response(RPC_ERROR_INVALID_PARAMS, "Invalid method parameters", "", request.id)
        when "internal_error" -> damn create_error_response(RPC_ERROR_INTERNAL_ERROR, "Internal server error", "", request.id)
        when _ -> damn create_error_response(RPC_ERROR_SERVER_ERROR_MIN, "Server error", "", request.id)
    }
    
    sus response RpcResponse = create_success_response(result, request.id)
    
    # Run post-processing middleware
    bestie (middleware in registry.middleware) {
        response = middleware.after_call(request, response) fam {
            when _ -> damn create_error_response(RPC_ERROR_INTERNAL_ERROR, "Middleware error", "", request.id)
        }
    }
    
    damn response
}

# Validate RPC request structure
slay validate_rpc_request(request RpcRequest) yikes<lit> {
    ready (request.jsonrpc != "2.0") {
        yikes "Invalid JSON-RPC version"
    }
    
    ready (request.method == "") {
        yikes "Method name is required"
    }
    
    # Method names starting with "rpc." are reserved
    ready (stringz.starts_with(request.method, "rpc.")) {
        yikes "Method names starting with 'rpc.' are reserved"
    }
    
    damn based
}

# Check if request is a notification (no ID)
slay is_notification(request RpcRequest) lit {
    damn request.id == ""
}

# Create batch response for multiple requests
slay process_batch_request(registry &RpcRegistry, json_data tea) yikes<tea> {
    sus requests []RpcRequest = []
    sus responses []RpcResponse = []
    
    # Parse batch request
    sus json_array []map<tea, tea> = jsonz.parse_array(json_data) fam {
        when _ -> yikes "Invalid batch request format"
    }
    
    ready (len(json_array) == 0) {
        yikes "Batch request cannot be empty"
    }
    
    # Process each request in batch
    bestie (json_obj in json_array) {
        sus request_json tea = jsonz.encode_object(json_obj) fam {
            when _ -> continue
        }
        
        sus request RpcRequest = parse_rpc_request(request_json) fam {
            when _ -> {
                sus error_response RpcResponse = create_error_response(RPC_ERROR_INVALID_REQUEST, "Invalid request", "", "")
                responses = append(responses, error_response)
                continue
            }
        }
        
        sus response RpcResponse = process_rpc_request(registry, request) fam {
            when _ -> create_error_response(RPC_ERROR_INTERNAL_ERROR, "Processing error", "", request.id)
        }
        
        # Skip notifications in batch response
        ready (!is_notification(request)) {
            responses = append(responses, response)
        }
    }
    
    # Return batch response
    sus batch_json tea = jsonz.encode_array(responses) fam {
        when _ -> yikes "Failed to serialize batch response"
    }
    
    damn batch_json
}
