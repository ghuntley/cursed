# Test web functionality without full build system
vibez.spill("Testing web functionality...")

# Basic status code function
slay status_code_text(code normie) tea {
    lowkey code == 200 {
        damn "OK"
    } elif code == 404 {
        damn "Not Found"
    } else {
        damn "Unknown Status"
    }
}

# Test the function
sus result := status_code_text(200)
vibez.spill("Status 200: " + result)

sus result2 := status_code_text(404)
vibez.spill("Status 404: " + result2)

# Method validation function
slay validate_method(method tea) lit {
    lowkey method == "GET" || method == "POST" {
        damn based
    }
    damn cap
}

# Test method validation
lowkey validate_method("GET") {
    vibez.spill("✅ GET is valid")
} else {
    vibez.spill("❌ GET validation failed")
}

lowkey validate_method("INVALID") {
    vibez.spill("❌ INVALID should be invalid")
} else {
    vibez.spill("✅ INVALID correctly rejected")
}

# Simple response builder
slay build_response(status normie, body tea) tea {
    sus response tea = "HTTP/1.1 " + status.to_string() + " " + status_code_text(status) + "\r\n"
    response = response + "Content-Type: text/plain\r\n"
    response = response + "Content-Length: " + body.length().to_string() + "\r\n"
    response = response + "\r\n"
    response = response + body
    damn response
}

# Test response building
sus response := build_response(200, "Hello, World!")
vibez.spill("Response built successfully")
vibez.spill("Response length: " + response.length().to_string())

vibez.spill("✅ Web functionality test completed successfully!")
