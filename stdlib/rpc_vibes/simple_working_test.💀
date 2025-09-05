fr fr Simple working RPC test without arrays

fr fr Simple RPC functions without arrays
slay create_simple_request(id tea, method_name tea) tea {
    damn "{\"jsonrpc\":\"2.0\",\"id\":\"" + id + "\",\"method\":\"" + method_name + "\"}"
}

slay create_simple_response(id tea, result tea) tea {
    damn "{\"jsonrpc\":\"2.0\",\"id\":\"" + id + "\",\"result\":\"" + result + "\"}"
}

slay simple_execute_method(method_name tea) tea {
    if method_name == "ping" {
        damn "pong"
    } else if method_name == "add" {
        damn "42"
    } else if method_name == "echo" {
        damn "hello"
    }
    damn "error"
}

sus registered_method tea = ""

slay simple_register_method(method_name tea) {
    registered_method = method_name
}

slay simple_is_method_registered(method_name tea) lit {
    if registered_method == method_name {
        damn based
    }
    damn cap
}

sus request_count normie = 0

slay simple_init_stats() {
    request_count = 0
}

slay simple_increment_requests() {
    request_count = request_count + 1
}

slay simple_get_request_count() normie {
    damn request_count
}

fr fr Test execution
vibez.spill("🚀 Simple RPC Vibes Test")
vibez.spill("========================")

fr fr Test 1: Request creation
vibez.spill("Test 1: Request Creation")
sus request tea = create_simple_request("test_id", "ping")
vibez.spill("Request: " + request)

fr fr Test 2: Response creation
vibez.spill("Test 2: Response Creation")
sus response tea = create_simple_response("test_id", "pong")
vibez.spill("Response: " + response)

fr fr Test 3: Method execution
vibez.spill("Test 3: Method Execution")
sus result tea = simple_execute_method("ping")
vibez.spill("Result: " + result)

fr fr Test 4: Method registration
vibez.spill("Test 4: Method Registration")
simple_register_method("ping")
if simple_is_method_registered("ping") {
    vibez.spill("✅ Method registration successful")
} else {
    vibez.spill("❌ Method registration failed")
}

fr fr Test 5: Statistics
vibez.spill("Test 5: Statistics")
simple_init_stats()
simple_increment_requests()
sus count normie = simple_get_request_count()
if count == 1 {
    vibez.spill("✅ Statistics working")
} else {
    vibez.spill("❌ Statistics failed")
}

vibez.spill("========================")
vibez.spill("🎉 All simple tests completed!")
