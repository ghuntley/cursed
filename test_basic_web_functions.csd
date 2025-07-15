# Basic web functions test
vibez.spill("Testing basic web functions...")

# Test basic string operations
sus url tea = "https://example.com"
lowkey url.contains("https") {
    vibez.spill("✅ URL contains https")
} else {
    vibez.spill("❌ URL test failed")
}

# Test status code mapping
slay status_code_text(code normie) tea {
    lowkey code == 200 {
        damn "OK"
    } elif code == 404 {
        damn "Not Found"
    } else {
        damn "Unknown Status"
    }
}

sus result := status_code_text(200)
lowkey result == "OK" {
    vibez.spill("✅ Status code mapping works")
} else {
    vibez.spill("❌ Status code mapping failed")
}

# Test method validation
slay validate_method(method tea) lit {
    lowkey method == "GET" || method == "POST" {
        damn based
    }
    damn cap
}

lowkey validate_method("GET") {
    vibez.spill("✅ Method validation works")
} else {
    vibez.spill("❌ Method validation failed")
}

vibez.spill("Basic web functions test completed!")
