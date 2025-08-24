fr fr Test EncodingZ Package Integration
yeet "encodingz"
yeet "vibez"

slay main() {
    vibez.spill("🧪 Testing EncodingZ Package Integration...")
    
    fr fr Test Base64 encoding
    sus test_data tea = "Hello EncodingZ!"
    sus encoded tea = base64_encode(test_data)
    vibez.spill("Original: " + test_data)
    vibez.spill("Base64: " + encoded)
    
    fr fr Test Hex encoding
    sus hex_encoded tea = hex_encode(test_data)
    vibez.spill("Hex: " + hex_encoded)
    
    fr fr Test URL encoding
    sus url_test tea = "Hello World! @#$%"
    sus url_encoded tea = url_encode(url_test)
    vibez.spill("URL Original: " + url_test)
    vibez.spill("URL Encoded: " + url_encoded)
    
    vibez.spill("✅ EncodingZ Package Integration Test Complete!")
}

main()
