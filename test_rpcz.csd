# RPC Framework Implementation Test
yeet "rpcz"
yeet "vibez"

vibez.spill("Testing RPC framework functionality...")

# Test RPC server creation
sus server = rpc_server_create("localhost", 8080)
ready (server.is_error()) {
    vibez.spill("RPC server creation failed:", server.error())
    yikes "RPC server creation failed"
}

vibez.spill("✅ RPC server created successfully")

# Test RPC method registration
slay add_numbers(a drip, b drip) drip {
    damn a + b
}

sus result = server.register_method("add", add_numbers)
ready (result.is_error()) {
    vibez.spill("RPC method registration failed:", result.error())
    yikes "RPC method registration failed"
}

vibez.spill("✅ RPC method registration working")

# Test RPC client creation
sus client = rpc_client_create("localhost", 8080)
ready (client.is_error()) {
    vibez.spill("RPC client creation failed:", client.error())
    yikes "RPC client creation failed"
}

vibez.spill("✅ RPC client created successfully")

# Test RPC call (simulated)
sus call_result = client.call("add", [5, 3])
ready (call_result.is_error()) {
    vibez.spill("RPC call failed:", call_result.error())
    yikes "RPC call failed"
}

vibez.spill("✅ RPC call framework working")
vibez.spill("✅ All RPC framework tests passed")
