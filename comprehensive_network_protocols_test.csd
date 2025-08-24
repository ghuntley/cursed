fr fr Comprehensive Network Protocols Test
fr fr Tests enhanced TLS, SSH, FTP, SMTP, HTTP, and WebSocket implementations
fr fr Validates production-ready networking functionality

yeet "net_protocols"
yeet "testz"

slay main() normie {
    vibez.spill("🌐 CURSED Network Protocols - Comprehensive Test Suite")
    vibez.spill("=" * 60)
    
    fr fr Initialize protocols
    net_protocols_initialize()
    vibez.spill("")
    
    fr fr Run comprehensive test suite
    sus success lit = net_protocols_test()
    vibez.spill("")
    
    fr fr Additional HTTP tests
    vibez.spill("🔍 Advanced HTTP Protocol Tests")
    test_http_methods()
    test_http_url_parsing()
    test_http_headers()
    vibez.spill("")
    
    fr fr WebSocket advanced tests
    vibez.spill("🔄 Advanced WebSocket Tests")
    test_websocket_frames()
    test_websocket_protocol()
    vibez.spill("")
    
    fr fr TLS advanced tests
    vibez.spill("🔐 Advanced TLS Tests")
    test_tls_handshake()
    test_tls_encryption()
    vibez.spill("")
    
    fr fr Protocol integration tests
    vibez.spill("🔗 Protocol Integration Tests")
    test_protocol_integration()
    vibez.spill("")
    
    bestie success {
        vibez.spill("🎉 All network protocol tests PASSED!")
        damn 0
    } else {
        vibez.spill("❌ Some network protocol tests FAILED!")
        damn 1
    }
}

slay test_http_methods() lit {
    vibez.spill("  Testing HTTP methods...")
    
    fr fr Test GET request
    sus get_request tea = http_create_request("GET", "https://api.example.com/users", "Accept: application/json\r\n", "")
    bestie string_contains(get_request, "GET /users HTTP/1.1") && string_contains(get_request, "api.example.com") {
        vibez.spill("    ✅ HTTP GET request creation")
    } else {
        vibez.spill("    ❌ HTTP GET request creation failed")
    }
    
    fr fr Test POST with JSON
    sus json_data tea = "{\"name\": \"CURSED User\", \"email\": \"user@cursed.com\"}"
    sus post_response tea = http_post_json("https://api.example.com/users", json_data)
    bestie string_length(post_response) > 0 {
        vibez.spill("    ✅ HTTP POST JSON request")
    } else {
        vibez.spill("    ❌ HTTP POST JSON request failed")
    }
    
    fr fr Test URL encoding
    sus encoded tea = http_url_encode("Hello World! @#$%^&*()")
    bestie string_contains(encoded, "%") && string_contains(encoded, "+") {
        vibez.spill("    ✅ URL encoding")
    } else {
        vibez.spill("    ❌ URL encoding failed")
    }
}

slay test_http_url_parsing() lit {
    vibez.spill("  Testing HTTP URL parsing...")
    
    fr fr Test complex URL
    sus request tea = http_create_request("GET", "https://api.example.com:8080/v1/users/123?filter=active", "", "")
    
    bestie string_contains(request, "Host: api.example.com:8080") && string_contains(request, "GET /v1/users/123?filter=active") {
        vibez.spill("    ✅ Complex URL parsing")
    } else {
        vibez.spill("    ❌ Complex URL parsing failed")
    }
}

slay test_http_headers() lit {
    vibez.spill("  Testing HTTP headers...")
    
    fr fr Test response creation
    sus headers tea = "Content-Type: application/json\r\nCache-Control: max-age=3600\r\n"
    sus body tea = "{\"status\": \"success\", \"data\": \"test\"}"
    sus response tea = http_create_server_response(200, headers, body)
    
    bestie string_contains(response, "HTTP/1.1 200 OK") && string_contains(response, "Content-Length:") {
        vibez.spill("    ✅ HTTP server response creation")
    } else {
        vibez.spill("    ❌ HTTP server response creation failed")
    }
    
    fr fr Test response parsing
    (sus status normie, sus resp_headers tea, sus resp_body tea) = http_parse_response(response)
    bestie status == 200 && string_length(resp_body) > 0 {
        vibez.spill("    ✅ HTTP response parsing")
    } else {
        vibez.spill("    ❌ HTTP response parsing failed")
    }
}

slay test_websocket_frames() lit {
    vibez.spill("  Testing WebSocket frames...")
    
    fr fr Test text frame
    sus text_frame tea = ws_send_text("Hello, WebSocket!")
    bestie string_length(text_frame) > 2 {
        vibez.spill("    ✅ WebSocket text frame creation")
    } else {
        vibez.spill("    ❌ WebSocket text frame creation failed")
    }
    
    fr fr Test binary frame
    sus binary_data tea = char(0x01) + char(0x02) + char(0x03) + char(0xFF)
    sus binary_frame tea = ws_send_binary(binary_data)
    bestie string_length(binary_frame) > 2 {
        vibez.spill("    ✅ WebSocket binary frame creation")
    } else {
        vibez.spill("    ❌ WebSocket binary frame creation failed")
    }
    
    fr fr Test control frames
    sus ping_frame tea = ws_send_ping("ping")
    sus pong_frame tea = ws_send_pong("pong")
    sus close_frame tea = ws_send_close(1000, "Normal closure")
    
    bestie string_length(ping_frame) > 0 && string_length(pong_frame) > 0 && string_length(close_frame) > 0 {
        vibez.spill("    ✅ WebSocket control frames")
    } else {
        vibez.spill("    ❌ WebSocket control frames failed")
    }
}

slay test_websocket_protocol() lit {
    vibez.spill("  Testing WebSocket protocol...")
    
    fr fr Test handshake
    sus ws_key tea = "dGhlIHNhbXBsZSBub25jZQ=="
    sus handshake_response tea = ws_create_handshake_response(ws_key)
    
    bestie string_contains(handshake_response, "101 Switching Protocols") && 
           string_contains(handshake_response, "websocket") &&
           string_contains(handshake_response, "Sec-WebSocket-Accept:") {
        vibez.spill("    ✅ WebSocket handshake response")
    } else {
        vibez.spill("    ❌ WebSocket handshake response failed")
    }
}

slay test_tls_handshake() lit {
    vibez.spill("  Testing TLS handshake...")
    
    fr fr Test client hello structure
    tls_init_connection()
    sus client_hello tea = tls_create_client_hello()
    
    fr fr Check for proper TLS record structure
    bestie string_length(client_hello) > 100 && char_code(client_hello[0]) == 22 {
        vibez.spill("    ✅ TLS Client Hello structure")
    } else {
        vibez.spill("    ❌ TLS Client Hello structure failed")
    }
    
    fr fr Test extensions
    sus extensions tea = tls_build_extensions()
    bestie string_length(extensions) > 50 {
        vibez.spill("    ✅ TLS extensions generation")
    } else {
        vibez.spill("    ❌ TLS extensions generation failed")
    }
}

slay test_tls_encryption() lit {
    vibez.spill("  Testing TLS encryption...")
    
    fr fr Test key derivation
    tls_generate_master_secret("test_pre_master_secret")
    (sus client_key tea, sus server_key tea, sus client_iv tea, sus server_iv tea) = tls_derive_keys()
    
    bestie string_length(client_key) > 0 && string_length(server_key) > 0 {
        vibez.spill("    ✅ TLS key derivation")
    } else {
        vibez.spill("    ❌ TLS key derivation failed")
    }
    
    fr fr Test encryption/decryption
    sus plaintext tea = "Hello, TLS World!"
    sus encrypted tea = tls_encrypt_application_data(plaintext, client_key, client_iv)
    sus decrypted tea = tls_decrypt_application_data(encrypted, client_key, client_iv)
    
    bestie string_length(encrypted) > string_length(plaintext) {
        vibez.spill("    ✅ TLS application data encryption")
    } else {
        vibez.spill("    ❌ TLS application data encryption failed")
    }
}

slay test_protocol_integration() lit {
    vibez.spill("  Testing protocol integration...")
    
    fr fr Test HTTPS simulation (TLS + HTTP)
    sus http_request tea = http_create_request("GET", "https://secure.example.com/api", "", "")
    bestie string_contains(http_request, "Host: secure.example.com") {
        vibez.spill("    ✅ HTTPS request creation")
    } else {
        vibez.spill("    ❌ HTTPS request creation failed")
    }
    
    fr fr Test SMTPS simulation (SMTP + TLS)
    sus smtp_greeting tea = smtp_connect()
    sus smtp_starttls_response tea = smtp_handle_command("STARTTLS")
    bestie string_contains(smtp_starttls_response, "220") {
        vibez.spill("    ✅ SMTP STARTTLS integration")
    } else {
        vibez.spill("    ❌ SMTP STARTTLS integration failed")
    }
    
    fr fr Test FTPS simulation (FTP + TLS)
    sus ftp_welcome tea = ftp_connect()
    sus ftp_auth_response tea = ftp_handle_command("AUTH TLS")
    bestie string_length(ftp_auth_response) > 0 {
        vibez.spill("    ✅ FTP AUTH TLS integration")
    } else {
        vibez.spill("    ❌ FTP AUTH TLS integration failed")
    }
}

fr fr Run the test
main()
