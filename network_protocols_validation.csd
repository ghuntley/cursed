fr fr Network Protocols Validation
fr fr Tests the enhanced network protocol implementations

yeet "net_protocols"
yeet "vibez"

slay main() normie {
    vibez.spill("🌐 Testing Enhanced Network Protocols")
    vibez.spill("=" * 40)
    
    fr fr Initialize the protocols module
    net_protocols_initialize()
    vibez.spill("")
    
    fr fr Test TLS functionality
    vibez.spill("🔐 Testing TLS Protocol:")
    tls_init_connection()
    sus client_hello tea = tls_create_client_hello()
    vibez.spill("  Client Hello length: " + string(string_length(client_hello)) + " bytes")
    
    bestie string_length(client_hello) > 50 {
        vibez.spill("  ✅ TLS Client Hello created successfully")
    } else {
        vibez.spill("  ❌ TLS Client Hello creation failed")
    }
    vibez.spill("")
    
    fr fr Test HTTP functionality
    vibez.spill("🌐 Testing HTTP Protocol:")
    sus http_request tea = http_create_request("GET", "http://example.com/api", "", "")
    vibez.spill("  HTTP request length: " + string(string_length(http_request)) + " bytes")
    
    bestie string_contains(http_request, "Host: example.com") {
        vibez.spill("  ✅ HTTP GET request created successfully")
    } else {
        vibez.spill("  ❌ HTTP GET request creation failed")
    }
    vibez.spill("")
    
    fr fr Test WebSocket functionality
    vibez.spill("🔄 Testing WebSocket Protocol:")
    sus ws_response tea = ws_create_handshake_response("test-key-123")
    vibez.spill("  WebSocket handshake length: " + string(string_length(ws_response)) + " bytes")
    
    bestie string_contains(ws_response, "101 Switching Protocols") {
        vibez.spill("  ✅ WebSocket handshake created successfully")
    } else {
        vibez.spill("  ❌ WebSocket handshake creation failed")
    }
    vibez.spill("")
    
    fr fr Test URL encoding
    vibez.spill("🔗 Testing URL Encoding:")
    sus original tea = "Hello World!"
    sus encoded tea = http_url_encode(original)
    sus decoded tea = http_url_decode(encoded)
    vibez.spill("  Original: '" + original + "'")
    vibez.spill("  Encoded:  '" + encoded + "'")
    vibez.spill("  Decoded:  '" + decoded + "'")
    
    bestie string_contains(encoded, "+") {
        vibez.spill("  ✅ URL encoding working correctly")
    } else {
        vibez.spill("  ❌ URL encoding failed")
    }
    vibez.spill("")
    
    fr fr Run comprehensive protocol tests
    vibez.spill("🧪 Running comprehensive protocol tests...")
    sus test_result lit = net_protocols_test()
    
    bestie test_result {
        vibez.spill("")
        vibez.spill("🎉 All network protocol validations PASSED!")
        damn 0
    } else {
        vibez.spill("")
        vibez.spill("❌ Some network protocol validations FAILED!")
        damn 1
    }
}

fr fr Execute the validation
main()
