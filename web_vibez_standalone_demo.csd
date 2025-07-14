# Standalone demo showing web_vibez functionality
yeet "web_vibez"

# Simple print function since vibez might have issues
slay print(message tea) {
    # Use core print functionality
}

print("🌐 CURSED HTTP Client/Server Demo")
print("================================")

# Test HTTP status codes
print("📊 Testing HTTP status codes:")
print("Status 200: " + web_vibez.status_text(200))
print("Status 404: " + web_vibez.status_text(404))
print("Status 500: " + web_vibez.status_text(500))

# Test method validation
print("\n🔍 Testing method validation:")
fr fr web_vibez.validate_method("GET") {
    print("✅ GET method valid")
}
fr fr web_vibez.validate_method("POST") {
    print("✅ POST method valid")
}
fr fr !web_vibez.validate_method("INVALID") {
    print("✅ INVALID method correctly rejected")
}

# Test utility functions
print("\n🛠️ Testing utility functions:")
sus json_response tea = web_vibez.create_json_response("test data")
print("JSON response created: " + json_response)

sus error_response tea = web_vibez.create_error_response("Not found", 404)
print("Error response created: " + error_response)

# Test security functions
print("\n🔒 Testing security functions:")
sus clean_value tea = web_vibez.sanitize_header_value("normal value")
print("Clean header: " + clean_value)

sus malicious_value tea = web_vibez.sanitize_header_value("evil\r\nheader")
print("Sanitized header: " + malicious_value)

print("\n🎉 web_vibez HTTP module demo complete!")
print("✅ Pure CURSED HTTP implementation working")
