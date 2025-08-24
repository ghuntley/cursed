// Test encoding utilities (encodingz package)
yeet "encodingz"
yeet "vibez"

vibez.spill("=== Testing Encodingz Encoding Utilities ===")

// Test Base64 encoding
sus original_data tea = "Hello, CURSED encoding world!"
sus base64_encoded tea = base64_encode(original_data)
sus base64_decoded tea = base64_decode(base64_encoded)

ready (base64_decoded == original_data) {
    vibez.spill("✅ Base64 encoding/decoding: PASSED")
} otherwise {
    vibez.spill("❌ Base64 encoding/decoding: FAILED")
}

// Test hex encoding
sus hex_encoded tea = hex_encode(original_data)
sus hex_decoded tea = hex_decode(hex_encoded)

ready (hex_decoded == original_data) {
    vibez.spill("✅ Hex encoding/decoding: PASSED")
} otherwise {
    vibez.spill("❌ Hex encoding/decoding: FAILED")
}

// Test URL encoding
sus url_data tea = "hello world & special chars!@#$%"
sus url_encoded tea = url_encode(url_data)
sus url_decoded tea = url_decode(url_encoded)

ready (url_decoded == url_data) {
    vibez.spill("✅ URL encoding/decoding: PASSED")
} otherwise {
    vibez.spill("❌ URL encoding/decoding: FAILED")
}

// Test HTML entity encoding
sus html_data tea = "<script>alert('XSS')</script>"
sus html_encoded tea = html_escape(html_data)
sus html_decoded tea = html_unescape(html_encoded)

ready (html_decoded == html_data) {
    vibez.spill("✅ HTML encoding/decoding: PASSED")
} otherwise {
    vibez.spill("❌ HTML encoding/decoding: FAILED")
}

// Test Unicode encoding
sus unicode_data tea = "Hello 🌍 世界 🚀"
sus utf8_bytes []drip = utf8_encode(unicode_data)
sus utf8_decoded tea = utf8_decode(utf8_bytes)

ready (utf8_decoded == unicode_data) {
    vibez.spill("✅ UTF-8 encoding/decoding: PASSED")
} otherwise {
    vibez.spill("❌ UTF-8 encoding/decoding: FAILED")
}

vibez.spill("Encoded examples:")
vibez.spill("- Base64:", base64_encoded)
vibez.spill("- Hex:", hex_encoded)
vibez.spill("- URL:", url_encoded)
vibez.spill("- HTML:", html_encoded)

vibez.spill("=== Encodingz Testing Complete ===")
