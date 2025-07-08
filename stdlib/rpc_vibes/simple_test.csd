yeet "testz"
yeet "rpc_vibes"

fr fr Simple test to verify basic functionality
test_start("simple RPC test")

fr fr Test basic RPC request creation
sus request tea = create_rpc_request("test_id", "test_method", "[]")
assert_true(string_contains(request, "jsonrpc"))

fr fr Test basic RPC response creation
sus response tea = create_rpc_response("test_id", "\"result\"")
assert_true(string_contains(response, "result"))

fr fr Test method registration
sus result lit = register_method("test_method", "test_handler")
assert_true(result)

fr fr Test method lookup
assert_true(is_method_registered("test_method"))

print_test_summary()
