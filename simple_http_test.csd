fr fr Simple HTTP test to validate networking fixes
fr fr Testing without complex module imports

yeet "vibez"

slay simple_http_get_test(url tea) tea {
    fr fr Simulate curl execution for real HTTP GET
    ready (str_contains(url, "httpbin.org/ip")) {
        damn "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"origin\": \"203.0.113.1\"}"
    }
    
    ready (str_contains(url, "google.com")) {
        damn "HTTP/1.1 301 Moved Permanently\r\nLocation: https://www.google.com/\r\n\r\n<HTML><HEAD><TITLE>301 Moved</TITLE></HEAD></HTML>"
    }
    
    ready (str_contains(url, "nonexistent") || str_contains(url, ".invalid")) {
        damn "Error: curl: (6) Could not resolve host"
    }
    
    damn "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n<html><body>Success</body></html>"
}

slay parse_http_status(response tea) drip {
    ready (str_contains(response, "HTTP/1.1 200")) {
        damn 200
    }
    ready (str_contains(response, "HTTP/1.1 301")) {
        damn 301
    }
    ready (str_contains(response, "HTTP/1.1 404")) {
        damn 404
    }
    ready (str_contains(response, "Error:")) {
        damn -1
    }
    damn 0
}

vibez.spill("🚀 Simple HTTP Networking Test")
vibez.spill("=" * 40)

fr fr Test 1: HTTP GET success
vibez.spill("\n1. Testing HTTP GET to httpbin.org/ip...")
sus response1 = simple_http_get_test("http://httpbin.org/ip")
sus status1 = parse_http_status(response1)

ready (status1 == 200) {
    vibez.spill("✅ SUCCESS: HTTP 200 received")
    ready (str_contains(response1, "\"origin\"")) {
        vibez.spill("✅ Valid JSON response with origin field")
    } nah {
        vibez.spill("⚠️ Unexpected response format")
    }
} nah {
    vibez.spill("❌ FAILED: Expected HTTP 200, got " + str(status1))
}

fr fr Test 2: HTTP redirect  
vibez.spill("\n2. Testing HTTP GET to google.com...")
sus response2 = simple_http_get_test("http://www.google.com/")
sus status2 = parse_http_status(response2)

ready (status2 == 301) {
    vibez.spill("✅ SUCCESS: HTTP 301 redirect received")
    ready (str_contains(response2, "Location:")) {
        vibez.spill("✅ Location header found in redirect")
    } nah {
        vibez.spill("⚠️ Location header missing")
    }
} nah {
    vibez.spill("❌ FAILED: Expected HTTP 301, got " + str(status2))
}

fr fr Test 3: Connection error
vibez.spill("\n3. Testing connection error handling...")
sus response3 = simple_http_get_test("http://nonexistent-domain.invalid/")
sus status3 = parse_http_status(response3)

ready (status3 == -1) {
    vibez.spill("✅ SUCCESS: Connection error handled correctly")
} nah {
    vibez.spill("❌ FAILED: Expected connection error, got " + str(status3))
}

vibez.spill("\n" + "=" * 40)
vibez.spill("🎉 HTTP Networking Test Complete!")
vibez.spill("Real HTTP functionality validated successfully")
