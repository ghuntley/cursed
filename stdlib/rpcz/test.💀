# RPC Framework Comprehensive Test Suite
# Tests core RPC functionality, client/server communication, middleware,
# authentication, error handling, and performance scenarios

yeet "testz"
yeet "timez"
yeet "stringz"
yeet "concurrenz"
yeet "jsonz"
yeet "./core"
yeet "./client"  
yeet "./server"
yeet "./middleware"

# Test RPC Core Functionality
slay test_rpc_core() {
    vibez.spill("=== Testing RPC Core Functionality ===")
    
    # Test RPC request parsing
    sus json_request tea = "{\"jsonrpc\":\"2.0\",\"method\":\"add\",\"params\":{\"a\":5,\"b\":3},\"id\":\"1\"}"
    sus request RpcRequest = parse_rpc_request(json_request) fam {
        when _ -> {
            testz.fail("Failed to parse valid RPC request")
            damn
        }
    }
    
    testz.assert_eq_string(request.jsonrpc, "2.0", "JSON-RPC version")
    testz.assert_eq_string(request.method, "add", "Method name")
    testz.assert_eq_string(request.id, "1", "Request ID")
    
    # Test invalid request parsing
    sus invalid_json tea = "{\"jsonrpc\":\"1.0\",\"method\":\"add\"}"
    sus invalid_request RpcRequest = parse_rpc_request(invalid_json) fam {
        when _ -> {
            testz.pass("Correctly rejected invalid JSON-RPC version")
            damn
        }
    }
    testz.fail("Should have rejected invalid JSON-RPC version")
    
    # Test response creation
    sus success_response RpcResponse = create_success_response("8", "1")
    testz.assert_eq_string(success_response.jsonrpc, "2.0", "Response JSON-RPC version")
    testz.assert_eq_string(success_response.result, "8", "Response result")
    testz.assert_eq_string(success_response.id, "1", "Response ID")
    
    sus error_response RpcResponse = create_error_response(RPC_ERROR_METHOD_NOT_FOUND, "Method not found", "", "1")
    testz.assert_eq_string(error_response.jsonrpc, "2.0", "Error response JSON-RPC version")
    testz.assert_true(error_response.error != "", "Error response has error field")
    
    vibez.spill("✅ RPC Core tests passed")
}

# Test method registration and dispatch
slay test_method_registry() {
    vibez.spill("=== Testing Method Registry ===")
    
    sus registry RpcRegistry = new_rpc_registry()
    
    # Create test handler
    sus add_handler RpcHandler = slay(params tea) yikes<tea> {
        sus params_obj map<tea, drip> = jsonz.parse(params) fam {
            when _ -> yikes "invalid_params"
        }
        
        sus a drip = params_obj.get("a") fam {
            when _ -> yikes "invalid_params"
        }
        sus b drip = params_obj.get("b") fam {
            when _ -> yikes "invalid_params"
        }
        
        damn string_from_int(a + b)
    }
    
    # Register method
    register_method(&registry, "add", add_handler) fam {
        when _ -> {
            testz.fail("Failed to register method")
            damn
        }
    }
    
    # Test successful method call
    sus request RpcRequest = RpcRequest{
        jsonrpc: "2.0",
        method: "add", 
        params: "{\"a\":5,\"b\":3}",
        id: "1"
    }
    
    sus response RpcResponse = process_rpc_request(&registry, request) fam {
        when _ -> {
            testz.fail("Failed to process valid RPC request")
            damn
        }
    }
    
    testz.assert_eq_string(response.result, "8", "Addition result")
    testz.assert_eq_string(response.error, "", "No error in successful call")
    
    # Test method not found
    sus invalid_request RpcRequest = RpcRequest{
        jsonrpc: "2.0",
        method: "subtract",
        params: "{\"a\":5,\"b\":3}",
        id: "2"
    }
    
    sus invalid_response RpcResponse = process_rpc_request(&registry, invalid_request) fam {
        when _ -> {
            testz.fail("Failed to handle method not found")
            damn
        }
    }
    
    testz.assert_true(invalid_response.error != "", "Error response for method not found")
    
    vibez.spill("✅ Method Registry tests passed")
}

# Test RPC client functionality  
slay test_rpc_client() {
    vibez.spill("=== Testing RPC Client ===")
    
    sus config RpcClientConfig = default_client_config("http://localhost:8080/rpc")
    config.timeout = 5000
    config.max_retries = 2
    
    sus client RpcClient = new_rpc_client(config) fam {
        when _ -> {
            testz.fail("Failed to create RPC client")
            damn
        }
    }
    
    # Test request ID generation
    sus id1 tea = generate_request_id(&client)
    sus id2 tea = generate_request_id(&client) 
    testz.assert_true(id1 != id2, "Request IDs should be unique")
    testz.assert_true(stringz.contains(id1, "req_"), "Request ID should have proper prefix")
    
    # Test request creation
    sus rpc_request RpcRequest = RpcRequest{
        jsonrpc: "2.0",
        method: "test_method",
        params: "{\"test\":true}",
        id: id1
    }
    
    sus request_json tea = jsonz.encode_object(rpc_request) fam {
        when _ -> {
            testz.fail("Failed to encode RPC request")
            damn
        }
    }
    
    testz.assert_true(stringz.contains(request_json, "test_method"), "Request JSON contains method name")
    
    vibez.spill("✅ RPC Client tests passed")
}

# Test RPC server configuration and setup
slay test_rpc_server() {
    vibez.spill("=== Testing RPC Server ===")
    
    sus config RpcServerConfig = default_server_config()
    config.port = 8081  # Use different port for testing
    config.max_connections = 50
    config.rate_limit_per_minute = 30
    
    sus server RpcServer = new_rpc_server(config) fam {
        when _ -> {
            testz.fail("Failed to create RPC server")
            damn
        }
    }
    
    # Test method registration on server
    sus multiply_handler RpcHandler = slay(params tea) yikes<tea> {
        sus params_obj map<tea, drip> = jsonz.parse(params) fam {
            when _ -> yikes "invalid_params"
        }
        
        sus a drip = params_obj.get("a") fam {
            when _ -> yikes "invalid_params"
        }
        sus b drip = params_obj.get("b") fam {
            when _ -> yikes "invalid_params"
        }
        
        damn string_from_int(a * b)
    }
    
    server_register_method(&server, "multiply", multiply_handler) fam {
        when _ -> {
            testz.fail("Failed to register method on server")
            damn
        }
    }
    
    # Test server configuration
    testz.assert_eq_int(server.config.port, 8081, "Server port configuration")
    testz.assert_eq_int(server.config.max_connections, 50, "Max connections configuration")
    testz.assert_false(server.is_running, "Server should not be running initially")
    
    vibez.spill("✅ RPC Server tests passed")
}

# Test middleware functionality
slay test_middleware() {
    vibez.spill("=== Testing RPC Middleware ===")
    
    # Test logging middleware
    sus logging LoggingMiddleware = new_logging_middleware(based, based)
    
    sus test_request RpcRequest = RpcRequest{
        jsonrpc: "2.0",
        method: "test_method",
        params: "{\"test\":true}",
        id: "test_1"
    }
    
    logging_before_call(&logging, test_request) fam {
        when _ -> {
            testz.fail("Logging middleware before_call failed")
            damn
        }
    }
    
    sus test_response RpcResponse = create_success_response("success", "test_1")
    logging_after_call(&logging, test_request, test_response) fam {
        when _ -> {
            testz.fail("Logging middleware after_call failed")
            damn
        }
    }
    
    # Test metrics middleware
    sus metrics MetricsMiddleware = new_metrics_middleware()
    
    metrics_before_call(&metrics, test_request) fam {
        when _ -> {
            testz.fail("Metrics middleware before_call failed")
            damn
        }
    }
    
    testz.assert_eq_int(metrics.total_requests, 1, "Total requests counter")
    
    metrics_after_call(&metrics, test_request, test_response) fam {
        when _ -> {
            testz.fail("Metrics middleware after_call failed")
            damn
        }
    }
    
    testz.assert_eq_int(metrics.successful_requests, 1, "Successful requests counter")
    testz.assert_eq_int(metrics.failed_requests, 0, "Failed requests counter")
    
    # Test validation middleware
    sus validation ValidationMiddleware = new_validation_middleware(based)
    
    validation_before_call(&validation, test_request) fam {
        when _ -> {
            testz.fail("Validation middleware should accept valid request")
            damn
        }
    }
    
    # Test invalid request
    sus invalid_request RpcRequest = RpcRequest{
        jsonrpc: "2.0",
        method: "",  # Empty method name
        params: "",
        id: "test_2"
    }
    
    validation_before_call(&validation, invalid_request) fam {
        when _ -> {
            testz.pass("Validation middleware correctly rejected invalid request")
            damn
        }
    }
    testz.fail("Validation middleware should have rejected empty method name")
    
    vibez.spill("✅ Middleware tests passed")
}

# Test error handling scenarios
slay test_error_handling() {
    vibez.spill("=== Testing Error Handling ===")
    
    # Test parse errors
    sus invalid_json tea = "invalid json {"
    parse_rpc_request(invalid_json) fam {
        when _ -> {
            testz.pass("Correctly handled parse error")
            damn
        }
    }
    testz.fail("Should have failed to parse invalid JSON")
    
    # Test method not found error
    sus registry RpcRegistry = new_rpc_registry()
    
    sus request RpcRequest = RpcRequest{
        jsonrpc: "2.0",
        method: "nonexistent_method",
        params: "{}",
        id: "1"
    }
    
    sus response RpcResponse = process_rpc_request(&registry, request) fam {
        when _ -> {
            testz.fail("Should have handled method not found gracefully")
            damn
        }
    }
    
    testz.assert_true(response.error != "", "Should have error for method not found")
    testz.assert_true(stringz.contains(response.error, "Method not found"), "Error message should be descriptive")
    
    # Test parameter validation error
    sus divide_handler RpcHandler = slay(params tea) yikes<tea> {
        sus params_obj map<tea, drip> = jsonz.parse(params) fam {
            when _ -> yikes "invalid_params"
        }
        
        sus a drip = params_obj.get("a") fam {
            when _ -> yikes "invalid_params"
        }
        sus b drip = params_obj.get("b") fam {
            when _ -> yikes "invalid_params"
        }
        
        ready (b == 0) {
            yikes "internal_error"  # Division by zero
        }
        
        damn string_from_int(a / b)
    }
    
    register_method(&registry, "divide", divide_handler) fam {
        when _ -> {
            testz.fail("Failed to register divide method")
            damn
        }
    }
    
    sus divide_request RpcRequest = RpcRequest{
        jsonrpc: "2.0",
        method: "divide",
        params: "{\"a\":10,\"b\":0}",  # Division by zero
        id: "2"
    }
    
    sus divide_response RpcResponse = process_rpc_request(&registry, divide_request) fam {
        when _ -> {
            testz.fail("Should have handled division by zero")
            damn
        }
    }
    
    testz.assert_true(divide_response.error != "", "Should have error for division by zero")
    
    vibez.spill("✅ Error Handling tests passed")
}

# Test batch request processing
slay test_batch_requests() {
    vibez.spill("=== Testing Batch Requests ===")
    
    sus registry RpcRegistry = new_rpc_registry()
    
    # Register test methods
    sus add_handler RpcHandler = slay(params tea) yikes<tea> {
        sus params_obj map<tea, drip> = jsonz.parse(params) fam {
            when _ -> yikes "invalid_params"
        }
        
        sus a drip = params_obj.get("a") fam {
            when _ -> yikes "invalid_params"
        }
        sus b drip = params_obj.get("b") fam {
            when _ -> yikes "invalid_params"
        }
        
        damn string_from_int(a + b)
    }
    
    sus multiply_handler RpcHandler = slay(params tea) yikes<tea> {
        sus params_obj map<tea, drip> = jsonz.parse(params) fam {
            when _ -> yikes "invalid_params"
        }
        
        sus a drip = params_obj.get("a") fam {
            when _ -> yikes "invalid_params"
        }
        sus b drip = params_obj.get("b") fam {
            when _ -> yikes "invalid_params"
        }
        
        damn string_from_int(a * b)
    }
    
    register_method(&registry, "add", add_handler) fam {
        when _ -> testz.fail("Failed to register add method")
    }
    register_method(&registry, "multiply", multiply_handler) fam {
        when _ -> testz.fail("Failed to register multiply method")
    }
    
    # Test batch request
    sus batch_json tea = "[" +
        "{\"jsonrpc\":\"2.0\",\"method\":\"add\",\"params\":{\"a\":2,\"b\":3},\"id\":\"1\"}," +
        "{\"jsonrpc\":\"2.0\",\"method\":\"multiply\",\"params\":{\"a\":4,\"b\":5},\"id\":\"2\"}," +
        "{\"jsonrpc\":\"2.0\",\"method\":\"nonexistent\",\"params\":{},\"id\":\"3\"}" +
        "]"
    
    sus batch_response_json tea = process_batch_request(&registry, batch_json) fam {
        when _ -> {
            testz.fail("Failed to process batch request")
            damn
        }
    }
    
    testz.assert_true(stringz.contains(batch_response_json, "\"result\":\"5\""), "First batch result")
    testz.assert_true(stringz.contains(batch_response_json, "\"result\":\"20\""), "Second batch result")  
    testz.assert_true(stringz.contains(batch_response_json, "Method not found"), "Third batch error")
    
    vibez.spill("✅ Batch Request tests passed")
}

# Test concurrent request handling
slay test_concurrent_requests() {
    vibez.spill("=== Testing Concurrent Requests ===")
    
    sus registry RpcRegistry = new_rpc_registry()
    
    # Register slow method for testing concurrency
    sus slow_handler RpcHandler = slay(params tea) yikes<tea> {
        timez.sleep_ms(100)  # Simulate slow processing
        damn "completed"
    }
    
    register_method(&registry, "slow_method", slow_handler) fam {
        when _ -> {
            testz.fail("Failed to register slow method")
            damn
        }
    }
    
    # Test concurrent execution
    sus start_time drip = timez.now_millis()
    sus results chan<tea> = make_channel_buffered(3)
    
    # Launch 3 concurrent requests
    bestie (i in range(3)) {
        go {
            sus request RpcRequest = RpcRequest{
                jsonrpc: "2.0",
                method: "slow_method",
                params: "{}",
                id: "concurrent_" + string_from_int(i)
            }
            
            sus response RpcResponse = process_rpc_request(&registry, request) fam {
                when _ -> {
                    results <- "error"
                    damn
                }
            }
            
            results <- response.result
        }
    }
    
    # Collect results
    sus completed_count drip = 0
    bestie (completed_count < 3) {
        sus result tea = <-results
        testz.assert_eq_string(result, "completed", "Concurrent request result")
        completed_count += 1
    }
    
    sus total_time drip = timez.now_millis() - start_time
    testz.assert_true(total_time < 300, "Concurrent requests should complete faster than sequential")
    
    vibez.spill("✅ Concurrent Request tests passed")
}

# Test authentication scenarios
slay test_authentication() {
    vibez.spill("=== Testing Authentication ===")
    
    # Test simple auth provider
    sus auth SimpleAuthProvider = new_simple_auth_provider()
    add_auth_token(&auth, "valid_token_123", "user1")
    add_auth_token(&auth, "admin_token_456", "admin")
    
    # Test valid authentication
    simple_auth_authenticate(&auth, "valid_token_123") fam {
        when _ -> {
            testz.fail("Should have authenticated valid token")
            damn
        }
    }
    
    sus user_info tea = simple_auth_get_user_info(&auth, "valid_token_123") fam {
        when _ -> {
            testz.fail("Should have returned user info for valid token")
            damn
        }
    }
    testz.assert_eq_string(user_info, "user1", "User info retrieval")
    
    # Test invalid authentication
    simple_auth_authenticate(&auth, "invalid_token") fam {
        when _ -> {
            testz.pass("Correctly rejected invalid token")
            damn
        }
    }
    testz.fail("Should have rejected invalid token")
    
    vibez.spill("✅ Authentication tests passed")
}

# Performance and load testing
slay test_performance() {
    vibez.spill("=== Testing Performance ===")
    
    sus registry RpcRegistry = new_rpc_registry()
    
    # Register fast method
    sus fast_handler RpcHandler = slay(params tea) yikes<tea> {
        damn "fast_response"
    }
    
    register_method(&registry, "fast_method", fast_handler) fam {
        when _ -> {
            testz.fail("Failed to register fast method")
            damn
        }
    }
    
    # Measure processing time for many requests
    sus request_count drip = 1000
    sus start_time drip = timez.now_millis()
    
    bestie (i in range(request_count)) {
        sus request RpcRequest = RpcRequest{
            jsonrpc: "2.0",
            method: "fast_method",
            params: "{}",
            id: "perf_" + string_from_int(i)
        }
        
        process_rpc_request(&registry, request) fam {
            when _ -> testz.fail("Performance test request failed")
        }
    }
    
    sus total_time drip = timez.now_millis() - start_time
    sus avg_time drip = total_time / request_count
    
    vibez.spill("Processed", string_from_int(request_count), "requests in", string_from_int(total_time), "ms")
    vibez.spill("Average time per request:", string_from_int(avg_time), "ms")
    
    testz.assert_true(avg_time < 10, "Average request time should be under 10ms")
    
    vibez.spill("✅ Performance tests passed")
}

# Main test runner
slay run_all_rpc_tests() {
    vibez.spill("🚀 Starting RPC Framework Test Suite")
    vibez.spill("=====================================")
    
    test_rpc_core()
    test_method_registry()
    test_rpc_client()
    test_rpc_server()
    test_middleware()
    test_error_handling()
    test_batch_requests()
    test_concurrent_requests()
    test_authentication()
    test_performance()
    
    vibez.spill("=====================================")
    vibez.spill("🎉 All RPC Framework tests completed!")
    testz.print_test_summary()
}

# Run tests when module is executed
run_all_rpc_tests()
