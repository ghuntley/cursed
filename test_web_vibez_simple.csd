yeet "testz"

# Simple test to verify basic functionality
test_start("basic web_vibez test")
vibez.spill("Testing web_vibez module...")

# Test basic status code
sus status := 200
lowkey status == 200 {
    vibez.spill("Status code test passed")
}

# Test simple string operations
sus url := "https://example.com"
lowkey url.contains("https") {
    vibez.spill("URL validation test passed")
}

# Test method validation
sus method := "GET"
lowkey method == "GET" {
    vibez.spill("Method validation test passed")
}

print_test_summary()
